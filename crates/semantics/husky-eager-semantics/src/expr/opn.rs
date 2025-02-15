use crate::*;
use husky_entity_kind::FieldKind;
use husky_entity_route::{EntityRoutePtr, RangedEntityRoute};
use husky_text::RangedCustomIdentifier;
use husky_vm::Binding;
use infer_decl::TyDecl;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EagerOpnVariant {
    Binary {
        opr: BinaryOpr,
    },
    Prefix {
        opr: PrefixOpr,
    },
    Suffix {
        opr: EagerSuffixOpr,
    },
    RoutineCall(RangedEntityRoute),
    ValueCall,
    TypeCall {
        ranged_ty: RangedEntityRoute,
        ty_decl: Arc<TyDecl>,
    },
    NewVecFromList,
    Field {
        this_ty: EntityRoutePtr,
        field_ident: RangedCustomIdentifier,
        field_liason: MemberModifier,
        field_binding: Binding,
        field_kind: FieldKind,
    },
    MethodCall {
        method_ident: RangedCustomIdentifier,
        this_ty_decl: Arc<TyDecl>,
        method_route: EntityRoutePtr,
        output_binding: Binding,
    },
    Index {
        element_binding: Binding,
    },
}
