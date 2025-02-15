use super::*;

pub(super) fn entity_feature_repr(
    db: &dyn FeatureGenQueryGroup,
    entity_route: EntityRoutePtr,
) -> FeatureRepr {
    let entity_defn = db.entity_defn(entity_route).unwrap();
    match entity_defn.variant {
        EntityDefnVariant::Feature { ref defn_repr, .. } => {
            FeatureRepr::from_defn(db, None, defn_repr, db.feature_interner())
        }
        EntityDefnVariant::TargetInput => FeatureRepr::input(db),
        _ => todo!(),
    }
}

impl FeatureRepr {
    pub fn input(db: &dyn FeatureGenQueryGroup) -> Self {
        FeatureRepr::TargetInput {
            ty: db.target_input_ty().unwrap(),
            feature: db.feature_interner().intern(Feature::Input {}),
            main_file: db.target_entrance(),
        }
    }
}
