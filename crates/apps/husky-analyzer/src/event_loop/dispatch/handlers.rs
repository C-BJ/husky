use crate::{convert::from_lsp_types, lsp_ext::PositionOrRange, *};

type HuskyComptimeSnapshot = salsa::Snapshot<husky_comptime::HuskyComptime>;

use husky_comptime::*;
use husky_hover::HoverResult;
use husky_text::{FilePosition, FileRange, TextPosition, TextRange};
use husky_token::AbsSemanticToken;
use lsp_types::{
    CallHierarchyIncomingCall, CallHierarchyIncomingCallsParams, CallHierarchyItem,
    CallHierarchyOutgoingCall, CallHierarchyOutgoingCallsParams, CallHierarchyPrepareParams,
    CodeLens, CompletionItem, DocumentFormattingParams, FoldingRange, FoldingRangeParams, Location,
    Position, PrepareRenameResponse, RenameParams, SemanticTokens, SemanticTokensDeltaParams,
    SemanticTokensFullDeltaResult, SemanticTokensParams, SemanticTokensRangeParams,
    SemanticTokensRangeResult, SemanticTokensResult, SymbolInformation, WorkspaceEdit,
};

use crate::lsp_ext::{self, InlayHint, InlayHintsParams, WorkspaceSymbolParams};

pub(crate) fn handle_selection_range(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::SelectionRangeParams,
) -> Result<Option<Vec<lsp_types::SelectionRange>>> {
    msg_once!("todo!");
    Ok(None)
}

pub(crate) fn handle_matching_brace(
    _comptime: HuskyComptimeSnapshot,
    _params: lsp_ext::MatchingBraceParams,
) -> Result<Vec<Position>> {
    msg_once!("todo!");
    Ok(Vec::new())
}

pub(crate) fn handle_on_enter(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<Vec<lsp_ext::SnippetTextEdit>>> {
    msg_once!("todo!");
    Ok(None)
}

pub(crate) fn handle_on_type_formatting(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::DocumentOnTypeFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    msg_once!("TODO: handle_on_type_formatting");
    Ok(None)
}

pub(crate) fn handle_document_symbol(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::DocumentSymbolParams,
) -> Result<Option<lsp_types::DocumentSymbolResponse>> {
    msg_once!("todo handle_document_symbol");
    Ok(None)
}

pub(crate) fn handle_workspace_symbol(
    _snapshot: HuskyComptimeSnapshot,
    _params: WorkspaceSymbolParams,
) -> Result<Option<Vec<SymbolInformation>>> {
    msg_once!("todo handle workspace symbol!");
    Ok(None)
}

pub(crate) fn handle_will_rename_files(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::RenameFilesParams,
) -> Result<Option<lsp_types::WorkspaceEdit>> {
    msg_once!("todo handle will rename files!");
    Ok(None)
}

pub(crate) fn handle_goto_definition(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::GotoDefinitionParams,
) -> Result<Option<lsp_types::GotoDefinitionResponse>> {
    msg_once!("todo goto definition!");
    Ok(None)
}

pub(crate) fn handle_goto_declaration(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::request::GotoDeclarationParams,
) -> Result<Option<lsp_types::request::GotoDeclarationResponse>> {
    msg_once!("todo goto declaration!");
    Ok(None)
}

pub(crate) fn handle_goto_type_definition(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::request::GotoTypeDefinitionParams,
) -> Result<Option<lsp_types::request::GotoTypeDefinitionResponse>> {
    msg_once!("todo goto type definition!");
    Ok(None)
}

pub(crate) fn handle_parent_module(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<lsp_types::GotoDefinitionResponse>> {
    msg_once!("todo handle parent module!");
    Ok(None)
}

pub(crate) fn handle_completion(
    comptime: HuskyComptimeSnapshot,
    params: lsp_types::CompletionParams,
) -> Result<Option<lsp_types::CompletionResponse>> {
    let position = FilePosition::from_proto(&*comptime, &params.text_document_position);
    let completion_trigger_character = params.context.and_then(|ctx| ctx.trigger_character);
    Ok(comptime.completion(position, completion_trigger_character))
}

pub(crate) fn handle_completion_resolve(
    _snapshot: HuskyComptimeSnapshot,
    mut _original_completion: CompletionItem,
) -> Result<CompletionItem> {
    msg_once!("todo handle completion resolve");
    Ok(CompletionItem::default())
}

pub(crate) fn handle_folding_range(
    snapshot: HuskyComptimeSnapshot,
    params: FoldingRangeParams,
) -> Result<Option<Vec<FoldingRange>>> {
    use husky_token::*;
    if let Ok(path) = from_lsp_types::path_from_url(&params.text_document.uri) {
        let file = snapshot.intern_file(path);
        if let Ok(tokenized_text) = snapshot.tokenized_text(file) {
            return Ok(Some(tokenized_text.folding_ranges()));
        }
    }
    Ok(None)
}

pub(crate) fn handle_decl_help(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::SignatureHelpParams,
) -> Result<Option<lsp_types::SignatureHelp>> {
    msg_once!("todo handle signature help!");
    Ok(None)
}

pub(crate) fn handle_hover(
    comptime: HuskyComptimeSnapshot,
    params: lsp_ext::HoverParams,
) -> Result<Option<HoverResult>> {
    let file = comptime.it_url(&params.text_document.uri).expect("todo");
    let range = match params.position {
        PositionOrRange::Position(position) => lsp_types::Range::new(position, position),
        PositionOrRange::Range(range) => range,
    };
    let range: TextRange = range.into();
    Ok(comptime.opt_hover_result(FileRange::new(file, range)))
}

pub(crate) fn handle_prepare_rename(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<PrepareRenameResponse>> {
    msg_once!("todo handle prepare rename!");
    Ok(None)
}

pub(crate) fn handle_rename(
    _snapshot: HuskyComptimeSnapshot,
    _params: RenameParams,
) -> Result<Option<WorkspaceEdit>> {
    msg_once!("todo handle rename!");
    Ok(None)
}

pub(crate) fn handle_references(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::ReferenceParams,
) -> Result<Option<Vec<Location>>> {
    msg_once!("todo handle references!");
    Ok(None)
}

pub(crate) fn handle_formatting(
    _snapshot: HuskyComptimeSnapshot,
    _params: DocumentFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    msg_once!("todo handle formatting!");
    Ok(None)
}

pub(crate) fn handle_range_formatting(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::DocumentRangeFormattingParams,
) -> Result<Option<Vec<lsp_types::TextEdit>>> {
    msg_once!("todo handle range formatting!");
    Ok(None)
}

pub(crate) fn handle_code_action(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::CodeActionParams,
) -> Result<Option<Vec<lsp_ext::CodeAction>>> {
    msg_once!("todo handle code action!");
    Ok(None)
}

pub(crate) fn handle_code_action_resolve(
    _snapshot: HuskyComptimeSnapshot,
    mut _code_action: lsp_ext::CodeAction,
) -> Result<lsp_ext::CodeAction> {
    msg_once!("todo handle code action resolve!");
    Ok(lsp_ext::CodeAction::default())
}

pub(crate) fn handle_code_lens(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::CodeLensParams,
) -> Result<Option<Vec<CodeLens>>> {
    Ok(None)
}

pub(crate) fn handle_code_lens_resolve(
    _snapshot: HuskyComptimeSnapshot,
    _code_lens: CodeLens,
) -> Result<CodeLens> {
    msg_once!("todo handle code lens resolve!");
    todo!()
}

pub(crate) fn handle_document_highlight(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::DocumentHighlightParams,
) -> Result<Option<Vec<lsp_types::DocumentHighlight>>> {
    msg_once!("todo handle document highlight!");
    Ok(None)
}

pub(crate) fn handle_ssr(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_ext::SsrParams,
) -> Result<lsp_types::WorkspaceEdit> {
    msg_once!("todo handle ssr");
    todo!()
}

pub(crate) fn handle_inlay_hints(
    _snapshot: HuskyComptimeSnapshot,
    _params: InlayHintsParams,
) -> Result<Vec<InlayHint>> {
    msg_once!("todo handle inlay hints");
    todo!()
}

pub(crate) fn handle_call_hierarchy_prepare(
    _snapshot: HuskyComptimeSnapshot,
    _params: CallHierarchyPrepareParams,
) -> Result<Option<Vec<CallHierarchyItem>>> {
    msg_once!("todo handle call hierarchy prepare");
    Ok(None)
}

pub(crate) fn handle_call_hierarchy_incoming(
    _snapshot: HuskyComptimeSnapshot,
    _params: CallHierarchyIncomingCallsParams,
) -> Result<Option<Vec<CallHierarchyIncomingCall>>> {
    msg_once!("todo handle call hierarchy incoming");
    Ok(None)
}

pub(crate) fn handle_call_hierarchy_outgoing(
    _snapshot: HuskyComptimeSnapshot,
    _params: CallHierarchyOutgoingCallsParams,
) -> Result<Option<Vec<CallHierarchyOutgoingCall>>> {
    msg_once!("todo handle call hierarchy outgoing");
    Ok(None)
}

pub(crate) fn handle_semantic_tokens_full(
    snapshot: HuskyComptimeSnapshot,
    params: SemanticTokensParams,
) -> Result<Option<SemanticTokensResult>> {
    let file = snapshot.intern_file(convert::from_lsp_types::path_from_url(
        &params.text_document.uri,
    )?);
    let ast_text = match snapshot.ast_text(file) {
        Ok(ast_text) => ast_text,
        Err(_) => return Ok(None),
    };
    let data = AbsSemanticToken::to_semantic_tokens(&ast_text.semantic_tokens);
    Ok(Some(SemanticTokensResult::Tokens(SemanticTokens {
        result_id: None,
        data,
    })))
}

pub(crate) fn handle_semantic_tokens_full_delta(
    snapshot: HuskyComptimeSnapshot,
    params: SemanticTokensDeltaParams,
) -> Result<Option<SemanticTokensFullDeltaResult>> {
    msg_once!("todo handle semantic tokens full delta");
    let file = snapshot.intern_file(convert::from_lsp_types::path_from_url(
        &params.text_document.uri,
    )?);
    let ast_text = match snapshot.ast_text(file) {
        Ok(ast_text) => ast_text,
        Err(_) => return Ok(None),
    };
    Ok(Some(SemanticTokensFullDeltaResult::Tokens(
        SemanticTokens {
            result_id: None,
            data: AbsSemanticToken::to_semantic_tokens(&ast_text.semantic_tokens),
        },
    )))
}

pub(crate) fn handle_semantic_tokens_range(
    snapshot: HuskyComptimeSnapshot,
    params: SemanticTokensRangeParams,
) -> Result<Option<SemanticTokensRangeResult>> {
    let file = snapshot.intern_file(convert::from_lsp_types::path_from_url(
        &params.text_document.uri,
    )?);
    let ast_text = match snapshot.ast_text(file) {
        Ok(ast_text) => ast_text,
        Err(_) => return Ok(None),
    };
    Ok(Some(SemanticTokensRangeResult::Tokens(SemanticTokens {
        result_id: None,
        data: AbsSemanticToken::to_semantic_tokens(&ast_text.semantic_tokens),
    })))
}

pub(crate) fn handle_open_docs(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_types::TextDocumentPositionParams,
) -> Result<Option<lsp_types::Url>> {
    msg_once!("todo!");
    Ok(None)
}

pub(crate) fn handle_move_item(
    _snapshot: HuskyComptimeSnapshot,
    _params: lsp_ext::MoveItemParams,
) -> Result<Vec<lsp_ext::SnippetTextEdit>> {
    msg_once!("todo handle move item");
    todo!()
}
