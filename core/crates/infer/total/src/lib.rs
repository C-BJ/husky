use husky_infer_entity_route::*;
use infer_contract::*;
use infer_qualifier::InferQualifiedTyQueryGroup;

pub trait InferQueryGroup:
    InferContractQueryGroup + InferEntityRouteQueryGroup + InferQualifiedTyQueryGroup
{
}
