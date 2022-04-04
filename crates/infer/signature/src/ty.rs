mod enum_ty;
mod impl_instantiate;
mod record;
mod vec;

pub use vec::*;

use crate::*;
use ast::AstIter;
use enum_ty::*;
use record::*;
use scope::BuiltinScopeSignature;
use syntax_types::{MembAccessSignature, MembCallSignature, RawEnumVariantKind};
use vec_map::VecMap;
use vm::{MembAccessContract, VMTySignatureKind};
use word::{IdentMap, WordAllocator};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct TySignature {
    generics: Vec<GenericArgument>,
    traits: Vec<ScopePtr>,
    members: IdentMap<MembSignature>,
    kind: TySignatureKind,
}

impl TySignature {
    fn new(
        generics: Vec<GenericArgument>,
        traits: Vec<ScopePtr>,
        members: IdentMap<MembSignature>,
    ) -> Self {
        todo!()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TySignatureKind {
    Struct {
        memb_vars: IdentMap<MembAccessSignature>,
        memb_routines: IdentMap<MembCallSignature>,
    },
    Enum {
        variants: IdentMap<EnumVariantSignature>,
    },
    Record {
        memb_vars: IdentMap<MembAccessSignature>,
        memb_features: IdentMap<ScopePtr>,
    },
    Vec {
        element_ty: ScopePtr,
    },
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MembAccessKind {
    StructMembVar,
    StructMembFeature,
    RecordMemb,
}

impl TySignature {
    // fn vec(word_allocator: &WordAllocator, element_ty: ScopePtr) -> Self {
    //     let mut members = IdentMap::default();
    //     members.insert_new(
    //         word_allocator.alloc_from_ref("push").custom().unwrap(),
    //         MembSignature {
    //             kind: MembSignatureKind::Routine,
    //         },
    //     );
    //     Self {
    //         members,
    //         kind: TySignatureKind::Vec { element_ty },
    //     }
    // }

    pub fn memb_access_ty_result(&self, ident: CustomIdentifier) -> InferResult<ScopePtr> {
        match self.kind {
            TySignatureKind::Struct { ref memb_vars, .. } => ok_or!(
                memb_vars.get(ident),
                format!("no such member variable {}", ident.0)
            )
            .map(|signature| signature.ty),
            TySignatureKind::Enum { ref variants } => todo!(),
            TySignatureKind::Record {
                ref memb_vars,
                ref memb_features,
            } => {
                if let Some(memb_var) = memb_vars.get(ident) {
                    Ok(memb_var.ty)
                } else if let Some(memb_feature) = memb_features.get(ident) {
                    Ok(*memb_feature)
                } else {
                    todo!()
                }
            }
            TySignatureKind::Vec { element_ty } => todo!(),
        }
    }

    pub fn memb_access_signature(&self, ident: CustomIdentifier) -> MembAccessSignature {
        match self.kind {
            TySignatureKind::Struct { ref memb_vars, .. } => *memb_vars.get(ident).unwrap(),
            TySignatureKind::Enum { ref variants } => todo!(),
            TySignatureKind::Record {
                ref memb_vars,
                ref memb_features,
            } => {
                if let Some(memb_var) = memb_vars.get(ident) {
                    *memb_var
                } else if let Some(memb_feature) = memb_features.get(ident) {
                    MembAccessSignature {
                        contract: MembAccessContract::LazyOwn,
                        ty: *memb_feature,
                    }
                } else {
                    todo!()
                }
            }
            TySignatureKind::Vec { element_ty } => todo!(),
        }
    }

    pub fn memb_access_kind(&self, memb_ident: CustomIdentifier) -> MembAccessKind {
        match self.kind {
            TySignatureKind::Struct {
                ref memb_vars,
                ref memb_routines,
            } => {
                if memb_vars.get(memb_ident).is_some() {
                    MembAccessKind::StructMembVar
                } else {
                    panic!("todo: memb feature of struct")
                }
            }
            TySignatureKind::Enum { ref variants } => todo!(),
            TySignatureKind::Record {
                ref memb_vars,
                ref memb_features,
            } => {
                if memb_vars.get(memb_ident).is_some() {
                    MembAccessKind::RecordMemb
                } else if memb_features.get(memb_ident).is_some() {
                    MembAccessKind::RecordMemb
                } else {
                    todo!()
                }
            }
            TySignatureKind::Vec { element_ty } => todo!(),
        }
    }

    pub fn vm_ty_signature(&self) -> VMTySignatureKind {
        match self.kind {
            TySignatureKind::Struct { ref memb_vars, .. } => {
                let mut vm_memb_vars = IdentMap::<MembAccessContract>::default();
                memb_vars.iter().for_each(|(ident, memb_var_sig)| {
                    vm_memb_vars.insert_new(*ident, memb_var_sig.contract)
                });
                VMTySignatureKind::Struct {
                    memb_vars: vm_memb_vars,
                }
            }
            TySignatureKind::Enum { ref variants } => todo!(),
            TySignatureKind::Record {
                ref memb_vars,
                ref memb_features,
            } => todo!(),
            TySignatureKind::Vec { element_ty } => todo!(),
        }
    }

    pub fn memb_call_signature(&self, ident: CustomIdentifier) -> InferResult<&MembCallSignature> {
        match self.kind {
            TySignatureKind::Struct {
                ref memb_routines, ..
            } => {
                derived_not_none!(memb_routines.get(ident))
            }
            TySignatureKind::Enum { ref variants } => todo!(),
            TySignatureKind::Record {
                ref memb_vars,
                ref memb_features,
            } => todo!(),
            TySignatureKind::Vec { element_ty } => todo!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum EnumVariantSignature {
    Constant,
}

pub(crate) fn ty_signature(
    db: &dyn InferSignatureQueryGroup,
    scope: ScopePtr,
) -> InferResultArc<TySignature> {
    let source = db.scope_source(scope)?;
    match source {
        ScopeSource::Builtin(data) => Ok(Arc::new(match data.signature {
            BuiltinScopeSignature::Func(_) => todo!(),
            BuiltinScopeSignature::Module => todo!(),
            BuiltinScopeSignature::Ty { .. } => todo!(),
            BuiltinScopeSignature::Vec => {
                let vec_signature_template = db.vec_signature_template();
                vec_signature_template.instantiate(&scope.generics)
            }
        })),
        ScopeSource::WithinBuiltinModule => todo!(),
        ScopeSource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let item = ast_text
                .folded_results
                .fold_iter(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.kind {
                AstKind::TypeDecl { kind, .. } => match kind {
                    RawTyKind::Enum => {
                        enum_signature(scope.generics.clone(), derived_not_none!(item.children)?)
                    }
                    RawTyKind::Struct => {
                        struct_signature(scope.generics.clone(), item.children.unwrap())
                    }
                    RawTyKind::Record => {
                        record_signature(scope.generics.clone(), item.children.unwrap())
                    }
                    RawTyKind::Primitive => todo!(),
                    RawTyKind::Vec => todo!(),
                    RawTyKind::Array => todo!(),
                    RawTyKind::Other => todo!(),
                },
                _ => panic!(),
            }
        }
        ScopeSource::Module { file } => todo!(),
        ScopeSource::Contextual { .. } => todo!(),
    }
}

pub(crate) fn struct_signature(
    generics: Vec<GenericArgument>,
    children: AstIter,
) -> InferResultArc<TySignature> {
    let mut memb_vars = VecMap::default();
    let mut memb_routines = VecMap::default();
    for subitem in children {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::MembVar {
                ident,
                signature: MembAccessSignature { contract, ty },
            } => memb_vars.insert_new(ident, MembAccessSignature { contract, ty }),
            AstKind::MembRoutineDecl {
                ref memb_routine_head,
                ..
            } => memb_routines.insert_new(memb_routine_head.routine_name, memb_routine_head.into()),
            _ => panic!(),
        }
    }
    Ok(Arc::new(TySignature {
        generics,
        members: Default::default(),
        traits: Default::default(),
        kind: TySignatureKind::Struct {
            memb_vars,
            memb_routines,
        },
    }))
}
