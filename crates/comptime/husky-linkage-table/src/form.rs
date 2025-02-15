use crate::*;
use husky_instantiate::{Instantiable, InstantiationContext};
use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub enum LinkageForm {
    VecConstructor {
        element_ty: EntityRoutePtr,
    },
    TypeCall {
        ty: EntityRoutePtr,
    },
    Routine {
        routine: EntityRoutePtr,
    },
    Index {
        opd_tys: SmallVec<[EntityRoutePtr; 2]>,
    },
    StructFieldAccess {
        this_ty: EntityRoutePtr,
        field_ident: CustomIdentifier,
    },
}

impl Instantiable for LinkageForm {
    type Target = Self;

    fn instantiate(&self, ctx: &InstantiationContext) -> Self::Target {
        match self {
            LinkageForm::VecConstructor { element_ty } => LinkageForm::VecConstructor {
                element_ty: element_ty.instantiate(ctx).take_entity_route(),
            },
            LinkageForm::TypeCall { ty } => LinkageForm::TypeCall {
                ty: ty.instantiate(ctx).take_entity_route(),
            },
            LinkageForm::Routine { routine } => LinkageForm::Routine {
                routine: routine.instantiate(ctx).take_entity_route(),
            },
            LinkageForm::Index { opd_tys } => LinkageForm::Index {
                opd_tys: opd_tys
                    .iter()
                    .map(|opd_ty| opd_ty.instantiate(ctx).take_entity_route())
                    .collect(),
            },
            LinkageForm::StructFieldAccess {
                this_ty,
                field_ident,
            } => LinkageForm::StructFieldAccess {
                this_ty: this_ty.instantiate(ctx).take_entity_route(),
                field_ident: *field_ident,
            },
        }
    }
}
