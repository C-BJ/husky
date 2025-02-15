use crate::*;

impl EntityRoute {
    pub fn is_self_ty_alias(&self) -> bool {
        matches!(self.variant, EntityRouteVariant::ThisType { .. })
    }

    pub fn is_implemented(&self) -> bool {
        for spatial_argument in self.spatial_arguments.iter() {
            match spatial_argument {
                SpatialArgument::Const(_) => todo!(),
                SpatialArgument::EntityRoute(route) => {
                    if !route.is_implemented() {
                        return false;
                    }
                }
            }
        }
        match self.variant {
            EntityRouteVariant::Root { .. } => true,
            EntityRouteVariant::Package { .. } => todo!(),
            EntityRouteVariant::Child { parent, .. } => parent.is_implemented(),
            EntityRouteVariant::TypeAsTraitMember { ty, .. } => ty.is_implemented(),
            EntityRouteVariant::TargetInputValue => todo!(),
            EntityRouteVariant::TargetOutputType => todo!(),
            EntityRouteVariant::Any { .. } => true,
            EntityRouteVariant::ThisType { .. } => false,
        }
    }
}
