use thin_vec::thin_vec;

use crate::*;

#[test]
fn test_canonicalize() {
    let interner = EntityRouteInterner::default();
    let opt_f32_ty = interner.route_call(
        RootBuiltinIdentifier::Option.into(),
        thin_vec![SpatialArgument::EntityRoute(
            RootBuiltinIdentifier::F32.into()
        )],
    );
    assert_eq!(
        opt_f32_ty.canonicalize(),
        (CanonicalTy::new(
            1,
            CanonicalQualifier::Intrinsic,
            RootBuiltinIdentifier::F32.into()
        ))
    );
    let ref_opt_f32_ty = interner.route_call(
        RootBuiltinIdentifier::Ref.into(),
        thin_vec![SpatialArgument::EntityRoute(opt_f32_ty)],
    );
    assert_eq!(
        ref_opt_f32_ty.canonicalize(),
        (CanonicalTy::new(
            1,
            CanonicalQualifier::EvalRef,
            RootBuiltinIdentifier::F32.into()
        ))
    );
    let opt_opt_f32_ty = interner.route_call(
        RootBuiltinIdentifier::Option.into(),
        thin_vec![SpatialArgument::EntityRoute(opt_f32_ty)],
    );
    assert_eq!(
        opt_opt_f32_ty.canonicalize(),
        (CanonicalTy::new(
            2,
            CanonicalQualifier::Intrinsic,
            RootBuiltinIdentifier::F32.into()
        ))
    );
    let opt_ref_opt_f32_ty = interner.route_call(
        RootBuiltinIdentifier::Option.into(),
        thin_vec![SpatialArgument::EntityRoute(ref_opt_f32_ty)],
    );
    assert_eq!(
        opt_ref_opt_f32_ty.canonicalize(),
        (CanonicalTy::new(
            2,
            CanonicalQualifier::EvalRef,
            RootBuiltinIdentifier::F32.into()
        ))
    );
}
