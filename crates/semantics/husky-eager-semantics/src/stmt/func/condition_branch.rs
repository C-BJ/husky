use husky_file::FileItd;
use husky_text::TextRange;

use crate::*;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FuncConditionFlowBranch {
    pub variant: FuncConditionFlowBranchVariant,
    pub stmts: Arc<Vec<Arc<FuncStmt>>>,
    pub range: TextRange,
    pub file: FileItd,
    pub idx: u8,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FuncConditionFlowBranchVariant {
    If { condition: Arc<EagerExpr> },
    Elif { condition: Arc<EagerExpr> },
    Else,
}
