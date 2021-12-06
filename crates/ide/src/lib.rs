//! ide crate provides "ide-centric" APIs for the husky-lang-server. That is,
//! it generally operates with files and text ranges, and returns results as
//! Strings, suitable for displaying to the human.
//!
//! What powers this API are the `RootDatabase` struct, which defines a `salsa`
//! database, and the `hir` crate, where majority of the analysis happens.
//! However, IDE specific bits of the analysis (most notably completion) happen
//! in this crate.

// For proving that RootDatabase is RefUnwindSafe.
#![recursion_limit = "128"]
#![allow(dead_code, unused)]
#[allow(unused)]
macro_rules! eprintln {
    ($($tt:tt)*) => { stdx::eprintln!($($tt)*) };
}

mod markup;
mod navigation_target;
mod prime_caches;

mod annotations;
mod call_hierarchy;
mod call_info;
mod doc_links;
mod extend_selection;
mod file_structure;
mod fn_references;
mod folding_ranges;
mod goto_declaration;
mod goto_definition;
mod goto_implementation;
mod goto_type_definition;
mod highlight_related;
mod hover;
mod join_lines;
mod markdown_remove;
mod matching_brace;
mod move_item;
mod parent_module;
mod references;
mod rename;
mod ssr;
mod static_index;
mod status;
mod syntax_highlighting;
mod syntax_tree;
mod typing;
mod view_hir;
mod view_item_tree;

use std::sync::Arc;

use common::*;

use ide_db::{
    base_db::{
        salsa::{self, ParallelDatabase},
        Env, FileLoader, FilePathIdTable, SourceDatabase, VfsPath,
    },
    symbol_index::{self, FileSymbol},
    LineIndexDatabase,
};
use syntax::SingleFileParseTree;

use hir::db::DiagDatabase;

use crate::navigation_target::{ToNav, TryToNav};

pub use crate::{
    annotations::{Annotation, AnnotationConfig, AnnotationKind},
    call_hierarchy::CallItem,
    call_info::CallInfo,
    file_structure::{StructureNode, StructureNodeKind},
    folding_ranges::{Fold, FoldKind},
    highlight_related::{HighlightRelatedConfig, HighlightedRange},
    hover::{HoverAction, HoverConfig, HoverDocFormat, HoverGotoTypeData, HoverResult},
    join_lines::JoinLinesConfig,
    markup::Markup,
    move_item::Direction,
    navigation_target::NavigationTarget,
    prime_caches::PrimeCachesProgress,
    references::ReferenceSearchResult,
    rename::RenameError,
    static_index::{StaticIndex, StaticIndexedFile, TokenId, TokenStaticData},
    syntax_highlighting::{
        tags::{Highlight, HlMod, HlMods, HlOperator, HlPunct, HlTag},
        HlRange,
    },
};
pub use hir::{Documentation, Semantics};
pub use ide_assists::{
    Assist, AssistConfig, AssistId, AssistKind, AssistResolveStrategy, SingleResolve,
};
pub use ide_completion::{
    CompletionConfig, CompletionItem, CompletionItemKind, CompletionRelevance, ImportEdit, Snippet,
    SnippetScope,
};
pub use ide_db::{
    base_db::{
        Cancelled, Change, CrateId, Edition, FileID, FilePosition, FileRange, SourceRoot,
        SourceRootId,
    },
    label::Label,
    line_index::{LineCol, LineColUtf16, LineIndex},
    search::{ReferenceCategory, SearchScope},
    source_change::{FileSystemEdit, SourceChange},
    symbol_index::Query,
    IdeDatabase, SymbolKind,
};
pub use ide_ssr::SsrError;
pub use text_edit::{Indel, TextEdit};

pub type Cancellable<T> = Result<T, Cancelled>;

/// Info associated with a text range.
#[derive(Debug)]
pub struct RangeInfo<T> {
    pub range: TextRange,
    pub info: T,
}

impl<T> RangeInfo<T> {
    pub fn new(range: TextRange, info: T) -> RangeInfo<T> {
        RangeInfo { range, info }
    }
}

/// `AnalysisHost` stores the current state of the world.
#[derive(Debug)]
pub struct AnalysisHost {
    db: IdeDatabase,
}

impl AnalysisHost {
    pub fn new(lru_capacity: Option<usize>) -> AnalysisHost {
        AnalysisHost {
            db: IdeDatabase::new(lru_capacity),
        }
    }

    pub fn update_lru_capacity(&mut self, lru_capacity: Option<usize>) {
        self.db.update_lru_capacity(lru_capacity);
    }

    /// Returns a snapshot of the current state, which you can query for
    /// semantic information.
    pub fn analysis(&self) -> DatabaseProxy {
        DatabaseProxy {
            db: self.db.snapshot(),
        }
    }

    /// Applies changes to the current state of the world. If there are
    /// outstanding snapshots, they will be canceled.
    pub fn apply_change(&mut self, change: Change) {
        self.db.apply_change(change)
    }

    /// NB: this clears the database
    pub fn per_query_memory_usage(&mut self) -> Vec<(String, profile::Bytes)> {
        todo!()
    }
    pub fn request_cancellation(&mut self) {
        self.db.request_cancellation();
    }
    pub fn raw_database(&self) -> &IdeDatabase {
        &self.db
    }
    pub fn raw_database_mut(&mut self) -> &mut IdeDatabase {
        &mut self.db
    }
}

impl Default for AnalysisHost {
    fn default() -> AnalysisHost {
        AnalysisHost::new(None)
    }
}

/// Analysis is a snapshot of a world state at a moment in time. It is the main
/// entry point for asking semantic information about the world. When the world
/// state is advanced using `AnalysisHost::apply_change` method, all existing
/// `Analysis` are canceled (most method return `Err(Canceled)`).
#[derive(Debug)]
pub struct DatabaseProxy {
    db: salsa::Snapshot<IdeDatabase>,
}

// As a general design guideline, `Analysis` API are intended to be independent
// from the language server protocol. That is, when exposing some functionality
// we should think in terms of "what API makes most sense" and not in terms of
// "what types LSP uses". Although currently LSP is the only consumer of the
// API, the API should in theory be usable as a library, or via a different
// protocol.
impl DatabaseProxy {
    // Creates an analysis instance for a single file, without any extenal
    // dependencies, stdlib support or ability to apply changes. See
    // `AnalysisHost` for creating a fully-featured analysis.
    pub fn from_single_file(text: String) -> (DatabaseProxy, FileID) {
        todo!()
    }

    /// Debug info about the current state of the analysis.
    pub fn status(&self, file_id: Option<FileID>) -> Cancellable<String> {
        self.try_db_query(|db| status::status(&*db, file_id))
    }

    pub fn prime_caches<F>(&self, cb: F) -> Cancellable<()>
    where
        F: Fn(PrimeCachesProgress) + Sync + std::panic::UnwindSafe,
    {
        self.try_db_query(move |db| prime_caches::prime_caches(db, &cb))
    }

    /// Gets the text of the source file.
    pub fn file_text(&self, file_id: FileID) -> Cancellable<Arc<String>> {
        self.try_db_query(|db| db.file_text(file_id))
    }

    /// Gets the syntax tree of the file.
    pub fn parse(&self, file_id: FileID) -> Cancellable<SingleFileParseTree> {
        todo!()
    }

    /// Returns true if this file belongs to an immutable library.
    pub fn is_library_file(&self, file_id: FileID) -> Cancellable<bool> {
        use ide_db::base_db::SourceDatabaseExt;
        self.try_db_query(|db| db.source_root(db.file_source_root(file_id)).is_library)
    }

    /// Gets the file's `LineIndex`: data structure to convert between absolute
    /// offsets and line/column representation.
    pub fn file_line_index(&self, file_id: FileID) -> Cancellable<Arc<LineIndex>> {
        self.try_db_query(|db| db.line_index(file_id))
    }

    /// Selects the next syntactic nodes encompassing the range.
    pub fn extend_selection(&self, frange: FileRange) -> Cancellable<TextRange> {
        self.try_db_query(|db| extend_selection::extend_selection(db, frange))
    }

    /// Returns position of the matching brace (all types of braces are
    /// supported).
    pub fn matching_brace(&self, position: FilePosition) -> Cancellable<Option<TextSize>> {
        todo!()
    }

    /// Returns a syntax tree represented as `String`, for debug purposes.
    // FIXME: use a better name here.
    pub fn syntax_tree(
        &self,
        file_id: FileID,
        text_range: Option<TextRange>,
    ) -> Cancellable<String> {
        self.try_db_query(|db| syntax_tree::syntax_tree(db, file_id, text_range))
    }

    pub fn view_hir(&self, position: FilePosition) -> Cancellable<String> {
        self.try_db_query(|db| view_hir::view_hir(db, position))
    }

    pub fn view_item_tree(&self, file_id: FileID) -> Cancellable<String> {
        self.try_db_query(|db| view_item_tree::view_item_tree(db, file_id))
    }

    /// Returns an edit to remove all newlines in the range, cleaning up minor
    /// stuff like trailing commas.
    pub fn join_lines(&self, config: &JoinLinesConfig, frange: FileRange) -> Cancellable<TextEdit> {
        todo!()
    }

    /// Returns an edit which should be applied when opening a new line, fixing
    /// up minor stuff like continuing the comment.
    /// The edit will be a snippet (with `$0`).
    pub fn on_enter(&self, position: FilePosition) -> Cancellable<Option<TextEdit>> {
        self.try_db_query(|db| typing::on_enter(db, position))
    }

    /// Returns an edit which should be applied after a character was typed.
    ///
    /// This is useful for some on-the-fly fixups, like adding `;` to `let =`
    /// automatically.
    pub fn on_char_typed(
        &self,
        position: FilePosition,
        char_typed: char,
    ) -> Cancellable<Option<SourceChange>> {
        // Fast path to not even parse the file.
        if !typing::TRIGGER_CHARS.contains(char_typed) {
            return Ok(None);
        }
        self.try_db_query(|db| typing::on_char_typed(db, position, char_typed))
    }

    /// Returns a tree representation of symbols in the file. Useful to draw a
    /// file outline.
    pub fn file_structure(&self, file_id: FileID) -> Cancellable<Vec<StructureNode>> {
        eprintln!("TODO: tree representation of symbols in the file");
        Ok(vec![])
    }

    /// Returns the set of folding ranges.
    pub fn folding_ranges(&self, file_id: FileID) -> Cancellable<Vec<Fold>> {
        eprintln!("TODO: folding_ranges");
        Ok(vec![])
    }

    /// Fuzzy searches for a symbol.
    pub fn symbol_search(&self, query: Query) -> Cancellable<Vec<NavigationTarget>> {
        self.try_db_query(|db| {
            symbol_index::world_symbols(db, query)
                .into_iter()
                .map(|s| s.to_nav(db))
                .collect::<Vec<_>>()
        })
    }

    /// Returns the definitions from the symbol at `position`.
    pub fn goto_definition(
        &self,
        position: FilePosition,
    ) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>> {
        self.try_db_query(|db| goto_definition::goto_definition(db, position))
    }

    /// Returns the declaration from the symbol at `position`.
    pub fn goto_declaration(
        &self,
        position: FilePosition,
    ) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>> {
        self.try_db_query(|db| goto_declaration::goto_declaration(db, position))
    }

    /// Returns the impls from the symbol at `position`.
    pub fn goto_implementation(
        &self,
        position: FilePosition,
    ) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>> {
        self.try_db_query(|db| goto_implementation::goto_implementation(db, position))
    }

    /// Returns the type definitions for the symbol at `position`.
    pub fn goto_type_definition(
        &self,
        position: FilePosition,
    ) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>> {
        self.try_db_query(|db| goto_type_definition::goto_type_definition(db, position))
    }

    /// Finds all usages of the reference at point.
    pub fn find_all_refs(
        &self,
        position: FilePosition,
        search_scope: Option<SearchScope>,
    ) -> Cancellable<Option<Vec<ReferenceSearchResult>>> {
        todo!()
    }

    /// Finds all methods and free functions for the file. Does not return tests!
    pub fn find_all_methods(&self, file_id: FileID) -> Cancellable<Vec<FileRange>> {
        self.try_db_query(|db| fn_references::find_all_methods(db, file_id))
    }

    /// Returns a short text describing element at position.
    pub fn hover(
        &self,
        config: &HoverConfig,
        range: FileRange,
    ) -> Cancellable<Option<RangeInfo<HoverResult>>> {
        self.try_db_query(|db| hover::hover(db, range, config))
    }

    /// Return URL(s) for the documentation of the symbol under the cursor.
    pub fn external_docs(
        &self,
        position: FilePosition,
    ) -> Cancellable<Option<doc_links::DocumentationLink>> {
        self.try_db_query(|db| doc_links::external_docs(db, &position))
    }

    /// Computes parameter information for the given call expression.
    pub fn call_info(&self, position: FilePosition) -> Cancellable<Option<CallInfo>> {
        self.try_db_query(|db| call_info::call_info(db, position))
    }

    /// Computes call hierarchy candidates for the given file position.
    pub fn call_hierarchy(
        &self,
        position: FilePosition,
    ) -> Cancellable<Option<RangeInfo<Vec<NavigationTarget>>>> {
        self.try_db_query(|db| call_hierarchy::call_hierarchy(db, position))
    }

    /// Computes incoming calls for the given file position.
    pub fn incoming_calls(&self, position: FilePosition) -> Cancellable<Option<Vec<CallItem>>> {
        self.try_db_query(|db| call_hierarchy::incoming_calls(db, position))
    }

    /// Computes outgoing calls for the given file position.
    pub fn outgoing_calls(&self, position: FilePosition) -> Cancellable<Option<Vec<CallItem>>> {
        self.try_db_query(|db| call_hierarchy::outgoing_calls(db, position))
    }

    /// Returns a `mod name;` declaration which created the current module.
    pub fn parent_module(&self, position: FilePosition) -> Cancellable<Vec<NavigationTarget>> {
        self.try_db_query(|db| parent_module::parent_module(db, position))
    }

    /// Returns crates this file belongs too.
    pub fn crate_for(&self, file_id: FileID) -> Cancellable<Vec<CrateId>> {
        self.try_db_query(|db| parent_module::crate_for(db, file_id))
    }

    /// Returns the edition of the given crate.
    pub fn crate_edition(&self, crate_id: CrateId) -> Cancellable<Edition> {
        todo!()
    }

    /// Returns the root file of the given crate.
    pub fn crate_root(&self, crate_id: CrateId) -> Cancellable<FileID> {
        todo!()
    }

    /// Computes syntax highlighting for the given file
    pub fn highlight(&self, file_id: FileID) -> Cancellable<Vec<HlRange>> {
        self.try_db_query(|db| syntax_highlighting::highlight(db, file_id, None, false))
    }

    /// Computes all ranges to highlight for a given item in a file.
    pub fn highlight_related(
        &self,
        config: HighlightRelatedConfig,
        position: FilePosition,
    ) -> Cancellable<Option<Vec<HighlightedRange>>> {
        eprintln!("TODO: all ranges to highlight for a given item in a file");
        Ok(None)
    }

    /// Computes syntax highlighting for the given file range.
    pub fn highlight_range(&self, frange: FileRange) -> Cancellable<Vec<HlRange>> {
        self.try_db_query(|db| {
            syntax_highlighting::highlight(db, frange.file_id, Some(frange.range), false)
        })
    }

    /// Computes syntax highlighting for the given file.
    pub fn highlight_as_html(&self, file_id: FileID, rainbow: bool) -> Cancellable<String> {
        todo!()
    }

    /// Computes completions at the given position.
    pub fn completions(
        &self,
        config: &CompletionConfig,
        position: FilePosition,
    ) -> Cancellable<Option<Vec<CompletionItem>>> {
        self.try_db_query(|db| ide_completion::completions(db, config, position).map(Into::into))
    }

    /// Resolves additional completion data at the position given.
    pub fn resolve_completion_edits(
        &self,
        config: &CompletionConfig,
        position: FilePosition,
        imports: impl IntoIterator<Item = (String, String)> + std::panic::UnwindSafe,
    ) -> Cancellable<Vec<TextEdit>> {
        Ok(self
            .try_db_query(|db| {
                ide_completion::resolve_completion_edits(db, config, position, imports)
            })?
            .unwrap_or_default())
    }

    /// Computes assists (aka code actions aka intentions) for the given
    /// position. If `resolve == false`, computes enough info to show the
    /// lightbulb list in the editor, but doesn't compute actual edits, to
    /// improve performance.
    pub fn assists(
        &self,
        config: &AssistConfig,
        resolve: AssistResolveStrategy,
        frange: FileRange,
    ) -> Cancellable<Vec<Assist>> {
        self.try_db_query(|db| {
            let ssr_assists = ssr::ssr_assists(db, &resolve, frange);
            let mut acc = ide_assists::assists(db, config, resolve, frange);
            acc.extend(ssr_assists.into_iter());
            acc
        })
    }

    /// Computes the set of diagnostics for the given file.
    pub fn diagnostics(&self, file_id: FileID) -> Cancellable<Vec<hir::Diagnostic>> {
        self.try_db_query(|db| db.diagnostics(file_id))
    }

    /// Returns the edit required to rename reference at the position to the new
    /// name.
    pub fn rename(
        &self,
        position: FilePosition,
        new_name: &str,
    ) -> Cancellable<Result<SourceChange, RenameError>> {
        self.try_db_query(|db| rename::rename(db, position, new_name))
    }

    pub fn prepare_rename(
        &self,
        position: FilePosition,
    ) -> Cancellable<Result<RangeInfo<()>, RenameError>> {
        self.try_db_query(|db| rename::prepare_rename(db, position))
    }

    pub fn will_rename_file(
        &self,
        file_id: FileID,
        new_name_stem: &str,
    ) -> Cancellable<Option<SourceChange>> {
        self.try_db_query(|db| rename::will_rename_file(db, file_id, new_name_stem))
    }

    pub fn structural_search_replace(
        &self,
        query: &str,
        parse_only: bool,
        resolve_context: FilePosition,
        selections: Vec<FileRange>,
    ) -> Cancellable<Result<SourceChange, SsrError>> {
        self.try_db_query(|db| {
            let rule: ide_ssr::SsrRule = query.parse()?;
            let mut match_finder =
                ide_ssr::MatchFinder::in_context(db, resolve_context, selections);
            match_finder.add_rule(rule)?;
            let edits = if parse_only {
                Default::default()
            } else {
                match_finder.edits()
            };
            Ok(SourceChange::from(edits))
        })
    }

    pub fn annotations(
        &self,
        config: &AnnotationConfig,
        file_id: FileID,
    ) -> Cancellable<Vec<Annotation>> {
        self.try_db_query(|db| annotations::annotations(db, config, file_id))
    }

    pub fn resolve_annotation(&self, annotation: Annotation) -> Cancellable<Annotation> {
        self.try_db_query(|db| annotations::resolve_annotation(db, annotation))
    }

    pub fn move_item(
        &self,
        range: FileRange,
        direction: Direction,
    ) -> Cancellable<Option<TextEdit>> {
        self.try_db_query(|db| move_item::move_item(db, range, direction))
    }

    /// Performs an operation on the database that may be canceled.
    ///
    /// husky-lang-server needs to be able to answer semantic questions about the
    /// code while the code is being modified. A common problem is that a
    /// long-running query is being calculated when a new change arrives.
    ///
    /// We can't just apply the change immediately: this will cause the pending
    /// query to see inconsistent state (it will observe an absence of
    /// repeatable read). So what we do is we **cancel** all pending queries
    /// before applying the change.
    ///
    /// Salsa implements cancelation by unwinding with a special value and
    /// catching it on the API boundary.
    fn try_db_query<F, T>(&self, f: F) -> Cancellable<T>
    where
        F: FnOnce(&IdeDatabase) -> T + std::panic::UnwindSafe,
    {
        Cancelled::catch(|| f(&self.db))
    }
}
