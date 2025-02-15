use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct FeatureDecl {
    pub ty: EntityRoutePtr,
}

pub(crate) fn feature_decl(
    db: &dyn DeclQueryGroup,
    entity_route: EntityRoutePtr,
) -> InferResultArc<FeatureDecl> {
    let source = db.entity_source(entity_route)?;
    match source {
        EntitySource::StaticModuleItem(data) => todo!(),
        EntitySource::WithinBuiltinModule => todo!(),
        EntitySource::WithinModule {
            file,
            token_group_index,
        } => {
            let ast_text = db.ast_text(file)?;
            let item = ast_text
                .folded_results
                .iter_from(token_group_index)
                .next()
                .unwrap();
            let ast = item.value.as_ref()?;
            match ast.variant {
                AstVariant::FeatureDefnHead { return_ty, .. } => Ok(Arc::new(FeatureDecl {
                    ty: return_ty.route,
                })),
                _ => todo!(),
            }
        }
        EntitySource::Module { .. } => todo!(),
        EntitySource::TargetInput {} => Ok(Arc::new(FeatureDecl {
            ty: db.target_input_ty()?,
        })),
        EntitySource::StaticTypeMember(_) => todo!(),
        EntitySource::StaticTraitMember(_) => todo!(),
        EntitySource::StaticTypeAsTraitMember => todo!(),
        EntitySource::Any { .. } => todo!(),
        EntitySource::StaticEnumVariant(_) => todo!(),
        EntitySource::ThisType { .. } => todo!(),
    }
}
