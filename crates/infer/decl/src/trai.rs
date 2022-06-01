use crate::*;
use atom::{
    context::{AtomContextKind, Symbol},
    AtomContext, AtomContextStandalone,
};
use check_utils::should_eq;
use entity_kind::MemberKind;
use implement::Implementor;
use map_collect::MapCollect;
use vec_map::HasKey;
use word::IdentDict;

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDecl {
    pub trai: EntityRoutePtr,
    pub generic_parameters: IdentDict<SpatialParameter>,
    pub members: IdentDict<TraitMemberDecl>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum TraitMemberDecl {
    Method(Arc<MethodDecl>),
    Type {
        ident: CustomIdentifier,
        traits: Vec<EntityRoutePtr>,
    },
    ConstSize(usize),
    Call {},
}

impl TraitMemberDecl {
    pub fn from_static(
        db: &dyn DeclQueryGroup,
        static_member_defn: &EntityStaticDefn,
        symbol_context: &mut dyn AtomContext,
    ) -> Self {
        match static_member_defn.variant {
            EntityStaticDefnVariant::Method { .. } => TraitMemberDecl::Method(
                MethodDecl::from_static(db, static_member_defn, symbol_context),
            ),
            EntityStaticDefnVariant::TraitAssociatedType { trai, traits } => {
                TraitMemberDecl::Type {
                    ident: db.intern_word(static_member_defn.name).custom(),
                    traits: traits.map(|trai| symbol_context.entity_route_from_str(trai).unwrap()),
                }
            }
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            _ => panic!(),
        }
    }

    pub fn instantiate(&self, instantiator: &Instantiator) -> Self {
        match self {
            TraitMemberDecl::Method(method_decl) => {
                TraitMemberDecl::Method(method_decl.instantiate(instantiator))
            }
            TraitMemberDecl::Type { ident, traits } => TraitMemberDecl::Type {
                ident: *ident,
                traits: traits.map(|trai| {
                    instantiator
                        .instantiate_entity_route(*trai)
                        .take_entity_route()
                }),
            },
            TraitMemberDecl::ConstSize(_) => todo!(),
            TraitMemberDecl::Call {} => todo!(),
        }
    }

    pub fn implement(
        &self,
        db: &dyn DeclQueryGroup,
        implementor: &Implementor,
    ) -> TraitMemberImplDecl {
        match self {
            TraitMemberDecl::Method(method_decl) => {
                TraitMemberImplDecl::Method(method_decl.implement(&implementor))
            }
            TraitMemberDecl::Type { ident, traits } => {
                if traits.len() > 0 {
                    todo!("verify traits are satisfied")
                }
                let ty = implementor.generic_argument(*ident).take_entity_route();
                TraitMemberImplDecl::AssociatedType { ident: *ident, ty }
            }
            TraitMemberDecl::ConstSize(_) => todo!(),
            TraitMemberDecl::Call {} => todo!(),
        }
    }
}

impl HasKey<CustomIdentifier> for TraitMemberDecl {
    fn key(&self) -> CustomIdentifier {
        match self {
            TraitMemberDecl::Method(method_decl) => method_decl.ident,
            TraitMemberDecl::Type { ident, .. } => *ident,
            TraitMemberDecl::ConstSize(_) => todo!(),
            TraitMemberDecl::Call {} => todo!(),
        }
    }
}

impl TraitDecl {
    pub fn from_static(db: &dyn DeclQueryGroup, static_defn: &EntityStaticDefn) -> Arc<Self> {
        match static_defn.variant {
            EntityStaticDefnVariant::Trait {
                base_route,
                ref generic_parameters,
                ref members,
            } => {
                let generic_parameters = db.generic_parameters_from_static(generic_parameters);
                let symbols = db.symbols_from_generic_parameters(&generic_parameters);
                let member_context: Vec<_> = members.map(|member| {
                    (
                        db.intern_word(member.name).custom(),
                        match member.variant {
                            EntityStaticDefnVariant::Method { .. } => MemberKind::Method,
                            EntityStaticDefnVariant::TraitAssociatedType { .. } => {
                                MemberKind::TraitAssociatedType
                            }
                            _ => panic!(),
                        },
                    )
                });
                let mut symbol_context = AtomContextStandalone {
                    opt_package_main: None,
                    db: db.upcast(),
                    opt_this_ty: None,
                    opt_this_contract: None,
                    symbols: symbols.into(),
                    kind: AtomContextKind::Normal,
                };
                let base_route = symbol_context.entity_route_from_str(base_route).unwrap();
                let generic_arguments =
                    db.generic_arguments_from_generic_parameters(&generic_parameters);
                should_eq!(base_route.spatial_arguments.len(), 0);
                let trai = db.intern_entity_route(EntityRoute {
                    kind: base_route.kind,
                    spatial_arguments: generic_arguments,
                });
                symbol_context.kind = AtomContextKind::Trait {
                    this_trai: trai,
                    member_kinds: &member_context,
                };
                Arc::new(TraitDecl {
                    trai,
                    generic_parameters,
                    members: members
                        .iter()
                        .map(|member| TraitMemberDecl::from_static(db, member, &mut symbol_context))
                        .collect(),
                })
            }
            _ => panic!(),
        }
    }

    pub fn instantiate(
        &self,
        db: &dyn DeclQueryGroup,
        dst_generics: &[SpatialArgument],
    ) -> Arc<Self> {
        should_eq!(self.generic_parameters.len(), dst_generics.len());
        let instantiator = Instantiator {
            db: db.upcast(),
            generic_parameters: &self.generic_parameters,
            dst_generics,
        };
        Arc::new(TraitDecl {
            trai: instantiator
                .instantiate_entity_route(self.trai)
                .take_entity_route(),
            generic_parameters: Default::default(),
            members: self
                .members
                .iter()
                .map(|member| member.instantiate(&instantiator))
                .collect(),
        })
    }
}

pub(crate) fn trait_decl(
    db: &dyn DeclQueryGroup,
    entity_route: EntityRoutePtr,
) -> InferResultArc<TraitDecl> {
    let entity_source = db.entity_locus(entity_route).unwrap();
    match entity_source {
        EntityLocus::StaticModuleItem(static_defn) => match static_defn.variant {
            EntityStaticDefnVariant::Routine { .. } => todo!(),
            EntityStaticDefnVariant::Ty { .. } => todo!(),
            EntityStaticDefnVariant::Trait { .. } => {
                let base_decl = TraitDecl::from_static(db, static_defn);
                if entity_route.spatial_arguments.len() > 0 {
                    Ok(base_decl.instantiate(db, &entity_route.spatial_arguments))
                } else {
                    Ok(base_decl)
                }
            }
            EntityStaticDefnVariant::Module => todo!(),
            EntityStaticDefnVariant::Method { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedType { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedConstSize => todo!(),
            EntityStaticDefnVariant::TyField { .. } => todo!(),
            EntityStaticDefnVariant::TraitAssociatedTypeImpl { ty: route } => todo!(),
        },
        EntityLocus::WithinBuiltinModule => todo!(),
        EntityLocus::WithinModule {
            file,
            token_group_index,
        } => todo!(),
        EntityLocus::Module { file } => todo!(),
        EntityLocus::Input { main } => todo!(),
        EntityLocus::StaticTypeMember => todo!(),
        EntityLocus::StaticTypeAsTraitMember => todo!(),
    }
}

pub(crate) fn trait_decl_menu(db: &dyn DeclQueryGroup) -> Arc<TraitDeclMenu> {
    Arc::new(TraitDeclMenu {
        clone_trait: TraitDecl::from_static(db, &CLONE_TRAIT_DEFN),
    })
}

#[derive(Debug, PartialEq, Eq)]
pub struct TraitDeclMenu {
    pub clone_trait: Arc<TraitDecl>,
}
