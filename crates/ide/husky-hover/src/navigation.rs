use husky_documentation::Documentation;
use husky_file::FileItd;
use husky_text::TextRange;

use crate::*;

/// `NavigationTarget` represents an element in the editor's UI which you can
/// click on to navigate to a particular piece of code.
///
/// Typically, a `NavigationTarget` corresponds to some element in the source
/// code, like a function or a struct, but this is not strictly required.
#[derive(Clone, PartialEq, Eq, Hash)]
pub struct NavigationTarget {
    pub file: FileItd,
    /// Range which encompasses the whole element.
    ///
    /// Should include body, doc comments, attributes, etc.
    ///
    /// Clients should use this range to answer "is the cursor inside the
    /// element?" question.
    pub full_range: TextRange,
    /// A "most interesting" range within the `full_range`.
    ///
    /// Typically, `full_range` is the whole syntax node, including doc
    /// comments, and `focus_range` is the range of the identifier.
    ///
    /// Clients should place the cursor on this range when navigating to this target.
    pub focus_range: Option<TextRange>,
    pub name: String,
    pub kind: Option<SymbolKind>,
    pub container_name: Option<String>,
    pub description: Option<String>,
    pub docs: Option<Documentation>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum SymbolKind {
    Attribute,
    BuiltinAttr,
    Const,
    ConstParam,
    Derive,
    DeriveHelper,
    Enum,
    Field,
    Function,
    Impl,
    Label,
    LifetimeParam,
    Local,
    Macro,
    Module,
    SelfParam,
    SelfType,
    Static,
    Struct,
    ToolModule,
    Trait,
    TypeAlias,
    TypeParam,
    Union,
    ValueParam,
    Variant,
}
