use super::*;

pub(crate) fn record_signature(
    generics: Vec<GenericArgument>,
    children: AstIter,
) -> InferResultArc<TySignature> {
    let mut memb_vars = VecMap::default();
    let mut memb_features = VecMap::default();
    for subitem in children {
        let subast = subitem.value.as_ref()?;
        match subast.kind {
            AstKind::MembVar {
                ident,
                signature: MembAccessSignature { contract, ty },
            } => memb_vars.insert_new(ident, MembAccessSignature { contract, ty }),
            AstKind::MembFeatureDecl { ident, ty } => memb_features.insert_new(ident, ty),
            _ => panic!(),
        }
    }
    Ok(Arc::new(TySignature {
        generics: Default::default(),
        members: Default::default(),
        kind: TySignatureKind::Record {
            memb_vars,
            memb_features,
        },
        traits: todo!(),
    }))
}
