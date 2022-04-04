mod call;
mod feature;
mod global;
mod memb;
mod traits;
mod ty;

pub use call::*;
pub use memb::*;
pub use traits::*;
pub use ty::*;

use ast::*;
use entity_syntax::RawTyKind;
use feature::*;
use file::FilePtr;
use fold::FoldStorage;
use global::*;
use infer_error::*;
use scope::*;
use scope_query::*;
use std::sync::Arc;
use vm::Compiled;
use word::CustomIdentifier;

#[salsa::query_group(InferSignatureQueryGroupStorage)]
pub trait InferSignatureQueryGroup: ScopeQueryGroup + ast::AstQueryGroup {
    fn call_signature(&self, scope: ScopePtr) -> InferResultArc<CallSignature>;
    fn ty_signature(&self, scope: ScopePtr) -> InferResultArc<TySignature>;
    fn feature_signature(&self, scope: ScopePtr) -> InferResultArc<FeatureSignature>;
    fn global_input_ty(&self, main_file: FilePtr) -> InferResult<ScopePtr>;
    fn global_output_ty(&self, main_file: FilePtr) -> InferResult<ScopePtr>;
    fn vec_signature_template(&self) -> Arc<TySignature>;
}
