use husky_ast::{RawBoundary, RawLoopKind};
use husky_pattern_syntax::{RawPattern, RawPatternVariant};
use husky_primitive_literal_syntax::RawLiteralData;

use super::*;

impl<'a> EntityRouteSheetBuilder<'a> {
    pub(super) fn infer_stmts(&mut self, ast_iter: AstIter, opt_output_ty: Option<EntityRoutePtr>) {
        self.enter_block();
        for item in ast_iter {
            if let Ok(ref value) = item.value {
                match value.variant {
                    AstVariant::Stmt(ref stmt) => match stmt.variant {
                        RawStmtVariant::Match { match_expr, .. } => {
                            let opt_match_expr_ty = self.infer_expr(match_expr, None);
                            if let Some(children) = item.opt_children {
                                self.infer_match_branches(
                                    children,
                                    opt_output_ty,
                                    opt_match_expr_ty,
                                )
                            }
                        }
                        _ => {
                            self.infer_stmt(stmt, opt_output_ty);
                            if let Some(children) = item.opt_children {
                                self.infer_stmts(children, opt_output_ty)
                            }
                        }
                    },
                    _ => todo!(),
                }
            } else {
                if let Some(children) = item.opt_children {
                    self.infer_stmts(children, opt_output_ty)
                }
            }
        }
        self.exit_block()
    }

    fn infer_stmt(&mut self, stmt: &RawStmt, opt_output_ty: Option<EntityRoutePtr>) {
        match stmt.variant {
            RawStmtVariant::Loop(raw_loop_kind) => match raw_loop_kind {
                RawLoopKind::For {
                    frame_var,
                    initial_boundary,
                    final_boundary,
                    ..
                } => {
                    should!(self
                        .entity_route_sheet
                        .variable_tys
                        .insert(
                            (frame_var.ident, frame_var.range),
                            RootBuiltinIdentifier::I32.into()
                        )
                        .is_none());
                    self.infer_loop_bound(initial_boundary);
                    self.infer_loop_bound(final_boundary);
                }
                RawLoopKind::ForExt { final_boundary, .. } => self.infer_loop_bound(final_boundary),
                RawLoopKind::While { condition } => self.infer_condition(condition),
                RawLoopKind::DoWhile { condition } => self.infer_condition(condition),
            },
            RawStmtVariant::IfElseBranch {
                condition_branch_kind,
            } => match condition_branch_kind {
                RawConditionBranchKind::If { condition } => self.infer_condition(condition),
                RawConditionBranchKind::Elif { condition } => self.infer_condition(condition),
                RawConditionBranchKind::Else => (),
            },
            RawStmtVariant::Require { condition, .. } => self.infer_condition(condition),
            RawStmtVariant::MatchBranch {
                ref pattern_branch_variant,
            } => match pattern_branch_variant {
                RawPatternBranchVariant::Case { .. } => todo!(),
                RawPatternBranchVariant::Default => todo!(),
            },
            RawStmtVariant::Exec { expr, discard } => {
                if let Some(ty) = self.infer_expr(
                    expr,
                    if discard {
                        None
                    } else {
                        Some(RootBuiltinIdentifier::Void.into())
                    },
                ) {
                    match (ty, discard) {
                        (EntityRoutePtr::Root(RootBuiltinIdentifier::Void), true) => {
                            self.entity_route_sheet.extra_errors.push(error!(
                                format!("obsolete discard because the output is of type `void`"),
                                self.arena[expr].range
                            ))
                        }
                        _ => (),
                    }
                }
            }
            RawStmtVariant::Init {
                varname,
                initial_value,
                ..
            } => {
                if let Some(ty) = self.infer_expr(initial_value, None) {
                    should!(self
                        .entity_route_sheet
                        .variable_tys
                        .insert((varname.ident, varname.range), ty)
                        .is_none())
                }
            }
            RawStmtVariant::Return { result, .. } => {
                self.infer_expr(result, opt_output_ty);
            }
            RawStmtVariant::Assert(condition) => self.infer_condition(condition),
            RawStmtVariant::Break => msg_once!("ensure break is inside a loop"),
            RawStmtVariant::Match { .. } => unreachable!(),
            RawStmtVariant::ReturnXml(ref xml_expr) => match xml_expr.variant {
                RawXmlExprVariant::Value(raw_expr_idx) => {
                    self.infer_expr(raw_expr_idx, None);
                }
                RawXmlExprVariant::Tag { ref props, .. } => {
                    props.iter().for_each(|(_, argument)| {
                        self.infer_expr(*argument, None);
                    })
                }
            },
        }
    }

    fn infer_match_branches(
        &mut self,
        branch_ast_iter: AstIter,
        opt_output_ty: Option<EntityRoutePtr>,
        opt_match_expr_ty: Option<EntityRoutePtr>,
    ) {
        for item in branch_ast_iter {
            if let Ok(ref ast) = item.value.as_ref() {
                match ast.variant {
                    AstVariant::Stmt(RawStmt {
                        variant:
                            RawStmtVariant::MatchBranch {
                                pattern_branch_variant:
                                    RawPatternBranchVariant::Case { ref pattern },
                            },
                        ..
                    }) => {
                        opt_match_expr_ty
                            .map(|match_expr_ty| self.infer_pattern(match_expr_ty, pattern));
                    }
                    AstVariant::Stmt(RawStmt {
                        variant:
                            RawStmtVariant::MatchBranch {
                                pattern_branch_variant: RawPatternBranchVariant::Default,
                            },
                        ..
                    }) => (),
                    _ => {
                        p!(ast.variant);
                        panic!()
                    }
                }
            }
            if let Some(children) = item.opt_children {
                self.infer_stmts(children, opt_output_ty)
            }
        }
    }

    fn infer_pattern(&mut self, expectation: EntityRoutePtr, pattern: &RawPattern) {
        // in pattern matching, we don't use expectation.intrinsic
        // because there is no implicit conversion
        let ty: EntityRoutePtr = match pattern.variant {
            RawPatternVariant::PrimitiveLiteral(value) => match value {
                RawLiteralData::Void => todo!(),
                RawLiteralData::Integer(_) => match expectation {
                    EntityRoutePtr::Root(
                        RootBuiltinIdentifier::I32
                        | RootBuiltinIdentifier::I64
                        | RootBuiltinIdentifier::B32
                        | RootBuiltinIdentifier::B64,
                    ) => return,
                    _ => RootBuiltinIdentifier::I32.into(),
                },
                RawLiteralData::I32(_) => RootBuiltinIdentifier::I32.into(),
                RawLiteralData::I64(_) => RootBuiltinIdentifier::I64.into(),
                RawLiteralData::Float(_) => todo!(),
                RawLiteralData::F32(_) => RootBuiltinIdentifier::F32.into(),
                RawLiteralData::F64(_) => RootBuiltinIdentifier::F64.into(),
                RawLiteralData::Bits(_) => todo!(),
                RawLiteralData::B32(_) => RootBuiltinIdentifier::B32.into(),
                RawLiteralData::B64(_) => RootBuiltinIdentifier::B64.into(),
                RawLiteralData::Bool(_) => todo!(),
            },
            RawPatternVariant::OneOf { ref subpatterns } => {
                for subpattern in subpatterns {
                    self.infer_pattern(expectation, subpattern)
                }
                return;
            }
            RawPatternVariant::EnumLiteral(value) => value.parent(),
            RawPatternVariant::Some => todo!(),
            RawPatternVariant::None => todo!(),
        };
        if ty != expectation {
            todo!()
        }
        // if let Some(match_expr_ty) = opt_match_expr_ty {
        //     if match_expr_ty != pattern.ty {
        //         self.entity_route_sheet.extra_errors.push(InferError {
        //      variant: InferErrorVariant::Original {
        //          message: format!(
        //              "match expression is of type `{:?}` but case pattern is of type `{:?}` instead",
        //              match_expr_ty, pattern.ty
        //          ),
        //          range: pattern.range,
        //      },
        //      dev_src: dev_src!(),
        //  })
        //     }
        // }
    }

    fn infer_loop_bound(&mut self, boundary: RawBoundary) {
        if let Some(bound) = boundary.opt_bound {
            self.infer_expr(bound, Some(RootBuiltinIdentifier::I32.into()));
        }
    }

    fn infer_condition(&mut self, condition: RawExprIdx) {
        self.infer_expr(condition, Some(RootBuiltinIdentifier::Bool.into()));
    }
}
