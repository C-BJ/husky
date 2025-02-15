use husky_entity_semantics::CallFormSource;

use super::*;

impl<'a> RustCodeGenerator<'a> {
    pub(super) fn gen_parameter(&mut self, parameter: &Parameter) {
        self.write(&parameter.ident());
        self.write(": ");
        match parameter.liason() {
            ParameterModifier::None => {
                if !self.db.is_copyable(parameter.ty()).unwrap() {
                    self.write("&")
                }
            }
            ParameterModifier::EvalRef => self.write("&'eval "),
            ParameterModifier::Owned => (),
            ParameterModifier::TempRefMut => self.write("&mut "),
            ParameterModifier::OwnedMut => todo!(),
            ParameterModifier::MemberAccess => todo!(),
            ParameterModifier::TempRef => todo!(),
        }
        self.gen_entity_route(parameter.ty(), EntityRouteRole::Decl);
    }

    pub(crate) fn gen_call_form_source(&mut self, source: &CallFormSource) {
        match source {
            CallFormSource::Func { stmts } => self.gen_func_stmts(stmts),
            CallFormSource::Proc { stmts } => self.gen_proc_stmts(stmts),
            CallFormSource::Lazy { .. } => todo!(),
            CallFormSource::Static(_) => todo!(),
        }
    }
}
