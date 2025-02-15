use husky_entity_route::{EntityRoute, EntityRoutePtr, EntityRouteVariant, SpatialArgument};
use husky_entity_syntax::EntitySyntaxSalsaQueryGroup;
use husky_print_utils::p;
use husky_word::CustomIdentifier;
use thin_vec::thin_vec;

pub struct ImplementationContext<'a> {
    db: &'a dyn EntitySyntaxSalsaQueryGroup,
    this_ty: EntityRoutePtr,
    member_impls: &'a [(CustomIdentifier, SpatialArgument)],
}

impl<'a> ImplementationContext<'a> {
    pub fn new(
        db: &'a dyn EntitySyntaxSalsaQueryGroup,
        this_ty: EntityRoutePtr,
        member_impls: &'a [(CustomIdentifier, SpatialArgument)],
    ) -> Self {
        Self {
            db,
            this_ty,
            member_impls,
        }
    }

    pub fn opt_spatial_argument(&self, ident0: CustomIdentifier) -> Option<SpatialArgument> {
        self.member_impls
            .iter()
            .find_map(|(ident, generic_argument)| {
                if *ident == ident0 {
                    Some(*generic_argument)
                } else {
                    None
                }
            })
    }

    pub fn spatial_argument(&self, ident: CustomIdentifier) -> SpatialArgument {
        self.opt_spatial_argument(ident).unwrap()
    }
}

impl<'a> std::fmt::Debug for ImplementationContext<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Implementor")
            .field("this_ty", &self.this_ty)
            .finish()
    }
}

pub trait Implementable {
    type Target;

    fn implement(&self, ctx: &ImplementationContext) -> Self::Target;
}

impl Implementable for EntityRoutePtr {
    type Target = SpatialArgument;

    fn implement(&self, ctx: &ImplementationContext) -> Self::Target {
        let (variant, mut spatial_arguments) = match self.variant {
            EntityRouteVariant::Root { ident } => {
                (EntityRouteVariant::Root { ident }, Default::default())
            }
            EntityRouteVariant::Package { main, ident } => (
                EntityRouteVariant::Package { main, ident },
                Default::default(),
            ),
            EntityRouteVariant::Child { parent, ident } => match parent.variant {
                EntityRouteVariant::ThisType { .. } => {
                    let route = ctx.spatial_argument(ident).take_entity_route();
                    assert!(route.is_implemented());
                    (route.variant.clone(), route.spatial_arguments.clone())
                }
                _ => {
                    p!(parent);
                    todo!()
                }
            },
            EntityRouteVariant::TargetInputValue => todo!(),
            EntityRouteVariant::Any { .. } => todo!(),
            EntityRouteVariant::ThisType { .. } => {
                assert!(!ctx.this_ty.is_self_ty_alias());
                let mut spatial_arguments = ctx.this_ty.spatial_arguments.clone();
                spatial_arguments.extend(self.spatial_arguments.clone());
                (ctx.this_ty.variant.clone(), spatial_arguments)
            }
            EntityRouteVariant::TypeAsTraitMember { ty, trai, ident } => match ty.variant {
                EntityRouteVariant::ThisType { .. } => {
                    if let Some(spatial_argument) = ctx.opt_spatial_argument(ident) {
                        match spatial_argument {
                            SpatialArgument::EntityRoute(_) => todo!(),
                            SpatialArgument::Const(_) => todo!(),
                        }
                    } else {
                        (
                            EntityRouteVariant::TypeAsTraitMember {
                                ty: ctx.this_ty,
                                trai,
                                ident,
                            },
                            thin_vec![],
                        )
                    }
                    // (route.kind, route.spatial_arguments.clone())
                }
                _ => {
                    p!(ty);
                    todo!()
                }
            },
            EntityRouteVariant::TargetOutputType => todo!(),
        };
        for spatial_argument in self.spatial_arguments.iter() {
            spatial_arguments.push(spatial_argument.implement(ctx))
        }
        let route = EntityRoute {
            variant,
            temporal_arguments: thin_vec![],
            spatial_arguments,
        };
        assert!(route.is_implemented());
        SpatialArgument::EntityRoute(ctx.db.intern_entity_route(route))
    }
}

impl Implementable for SpatialArgument {
    type Target = Self;

    fn implement(&self, implementor: &ImplementationContext) -> Self::Target {
        match self {
            SpatialArgument::Const(value) => SpatialArgument::Const(*value),
            SpatialArgument::EntityRoute(entity_route) => entity_route.implement(implementor),
        }
    }
}
