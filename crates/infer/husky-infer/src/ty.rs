use crate::*;
use husky_expr_syntax::{RawAtom, RawExprIdx, RawExprRange};
use husky_primitive_literal_syntax::RawLiteralData;
use husky_print_utils::p;
use husky_symbol_syntax::SymbolKind;
use husky_term::Ty;

impl<'a> InferContext<'a> {
    pub(crate) fn infer(&mut self) -> InferResult<Ty> {
        match self.normalized_expr() {
            NormalizedExpr::Atom(atom) => self.infer_atom(atom),
            NormalizedExpr::Opn { opn_kind, opds } => self.infer_opn(opn_kind, opds),
        }
    }

    fn infer_subexpr(&mut self, subexpr: RawExprIdx) -> InferResult<Ty> {
        self.subexpr_context(subexpr).infer()
    }

    fn infer_atom(&self, atom: &RawAtom) -> InferResult<Ty> {
        match atom {
            RawAtom::Literal(literal) => Ok(self.infer_literal(literal)),
            RawAtom::Symbol(symbol) => match symbol.kind {
                SymbolKind::EntityPath(_) => todo!(),
                SymbolKind::LocalVariable { init_range } => todo!(),
                SymbolKind::FrameVariable { init_range } => todo!(),
                SymbolKind::Unrecognized => Err(InferError::IdentUnrecognized),
                SymbolKind::ThisValue => todo!(),
                SymbolKind::ThisMethod => todo!(),
                SymbolKind::ThisField => todo!(),
            },
            RawAtom::Uncertain => todo!(),
        }
    }

    fn infer_opn(&mut self, opn_kind: NormalizedOpnKind, opds: RawExprRange) -> InferResult<Ty> {
        match opn_kind {
            NormalizedOpnKind::ApplyMethod {
                opt_trait_entity,
                method_ident,
            } => todo!(),
            NormalizedOpnKind::ScopeResolution => todo!(),
        }
        let this_ty = self.infer_subexpr(opds.start);
        p!(this_ty);
        todo!()
    }

    fn infer_literal(&self, literal: &RawLiteralData) -> Ty {
        let term_menu = self.term_menu();
        match literal {
            RawLiteralData::Void => todo!(),
            RawLiteralData::Integer(_) => term_menu.i32(),
            RawLiteralData::I32(_) => todo!(),
            RawLiteralData::I64(_) => todo!(),
            RawLiteralData::Float(_) => todo!(),
            RawLiteralData::F32(_) => todo!(),
            RawLiteralData::F64(_) => todo!(),
            RawLiteralData::Bits(_) => todo!(),
            RawLiteralData::B32(_) => todo!(),
            RawLiteralData::B64(_) => todo!(),
            RawLiteralData::Bool(_) => todo!(),
        }
    }
}
