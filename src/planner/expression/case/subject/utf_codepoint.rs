use super::super::super::{
    conversion::expect_expression, plan_expr_with_expected_source_stop_type,
};
use super::{CaseClause, OrderedCaseClauseInput};
use crate::plan::{BoolExpr, Expr, Step, UtfCodepointExpr, UtfCodepointLocalId, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use ecow::EcoString;
use gleam_core::ast::{Pattern, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject =
        plan_expr_with_expected_source_stop_type(subject, ValueType::UtfCodepoint, context)?;
    let return_shape = context.value_shape(type_.as_ref());
    let subject: UtfCodepointExpr = expect_expression(subject)?;
    let (subject_step, subject) = bind_case_subject(subject, context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let (pattern, reachable, exhaustive_remainder) = pattern.into_parts();
            let pattern = plan_case_pattern_with_context(pattern, context)?;
            let bindings = super::branch_bindings(pattern.bound_names(), subject.clone());
            let is_total = clause.guard.is_none();
            ordered_clauses.push(super::plan_ordered_case_clause(
                OrderedCaseClauseInput {
                    case_type: type_.as_ref(),
                    return_shape: &return_shape,
                    then: clause.then.clone(),
                    branch_bindings: bindings,
                    guard: clause.guard.clone(),
                    match_condition: BoolExpr::value(true),
                    is_total,
                    reachable,
                    exhaustive_remainder,
                },
                context,
            )?);
        }
    }

    super::ordered_case_expr(ordered_clauses)
        .map(|case| super::case_subject_block(subject_step, case))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct UtfCodepointCasePattern {
    bound_names: Vec<EcoString>,
}

impl UtfCodepointCasePattern {
    fn bound_names(&self) -> &[EcoString] {
        &self.bound_names
    }

    fn add_bound_name(&mut self, name: EcoString) {
        self.bound_names.push(name);
    }
}

fn plan_case_pattern_with_context(
    pattern: Pattern<Arc<Type>>,
    context: &PlanContext<'_>,
) -> Result<UtfCodepointCasePattern, PlanError> {
    match pattern {
        ref pattern @ Pattern::Variable { ref name, .. } => {
            crate::planner::pattern::validate_pattern(
                pattern,
                &crate::plan::ValueShape::UtfCodepoint,
                context,
            )?;
            Ok(UtfCodepointCasePattern {
                bound_names: vec![name.clone()],
            })
        }
        ref pattern @ Pattern::Discard { .. } => {
            crate::planner::pattern::validate_pattern(
                pattern,
                &crate::plan::ValueShape::UtfCodepoint,
                context,
            )?;
            Ok(UtfCodepointCasePattern {
                bound_names: Vec::new(),
            })
        }
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_case_pattern_with_context(*pattern, context)?;
            pattern.add_bound_name(name);
            Ok(pattern)
        }
        pattern @ (Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. }) => Err(crate::planner::pattern::unexpected_pattern(
            &pattern,
            &crate::plan::ValueShape::UtfCodepoint,
            context,
        )),
    }
}

#[cfg(test)]
fn plan_case_pattern(pattern: Pattern<Arc<Type>>) -> Result<UtfCodepointCasePattern, PlanError> {
    let module_name = EcoString::from("main");
    let functions = std::collections::HashMap::new();
    let mut anonymous = crate::planner::context::AnonymousFunctions::default();
    let context = PlanContext::new(&module_name, &functions, &mut anonymous);
    plan_case_pattern_with_context(pattern, &context)
}

fn bind_case_subject(subject: UtfCodepointExpr, context: &mut PlanContext<'_>) -> (Step, Expr) {
    let local = context.define_internal_utf_codepoint_local();
    let name = internal_case_subject_name(local);
    (
        Step::let_utf_codepoint(local, name.clone(), subject),
        Expr::utf_codepoint(UtfCodepointExpr::local_get(local, name)),
    )
}

fn internal_case_subject_name(local: UtfCodepointLocalId) -> EcoString {
    format!("<case:utf_codepoint:{}>", local.0).into()
}

#[cfg(test)]
mod tests {
    use crate::plan::ValueType;
    use crate::planner::dsl::{
        function, int, let_utf_codepoint_step, local_utf_codepoint, module,
        utf_codepoint_return_block, utf_codepoint_return_expr,
    };
    use crate::planner::plan_module;
    use crate::planner::support::dummy_span;
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    };
    use gleam_core::type_::error::VariableOrigin;

    #[test]
    fn plan_utf_codepoint_subject_binds_internal_subject_once() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
fn identity(value: UtfCodepoint) -> UtfCodepoint {
  case value {
    bound -> bound
  }
}

pub fn main() {
  0
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(0)),
            [function(
                "identity",
                utf_codepoint_return_block(
                    [let_utf_codepoint_step(
                        1,
                        "<case:utf_codepoint:1>",
                        local_utf_codepoint(0, "value"),
                    )],
                    utf_codepoint_return_block(
                        [let_utf_codepoint_step(
                            2,
                            "bound",
                            local_utf_codepoint(1, "<case:utf_codepoint:1>"),
                        )],
                        utf_codepoint_return_expr(local_utf_codepoint(2, "bound")),
                    ),
                ),
            )
            .param_utf_codepoint(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_utf_codepoint_case_pattern_preserves_binding_order() {
        let type_ = utf_codepoint_type();
        let pattern = gleam_core::ast::Pattern::Assign {
            location: dummy_span(),
            name: "alias".into(),
            pattern: Box::new(gleam_core::ast::Pattern::Variable {
                location: dummy_span(),
                name: "value".into(),
                type_,
                origin: VariableOrigin::generated(),
            }),
        };

        let planned = super::plan_case_pattern(pattern).expect("pattern should plan");
        assert_eq!(planned.bound_names(), &["value", "alias"]);
    }

    #[test]
    fn reject_margin_utf_codepoint_case_pattern_shapes() {
        let type_ = utf_codepoint_type();
        assert_eq!(
            super::plan_case_pattern(gleam_core::ast::Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: type_.clone(),
            })
            .expect("discard should plan")
            .bound_names(),
            Vec::<ecow::EcoString>::new().as_slice(),
        );
        assert_eq!(
            super::plan_case_pattern(gleam_core::ast::Pattern::Variable {
                location: dummy_span(),
                name: "value".into(),
                type_: gleam_core::type_::int(),
                origin: VariableOrigin::generated(),
            }),
            Err(super::super::pattern_type_mismatch(
                ValueType::UtfCodepoint,
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_case_pattern(gleam_core::ast::Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: gleam_core::type_::int(),
            }),
            Err(super::super::pattern_type_mismatch(
                ValueType::UtfCodepoint,
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_case_pattern(gleam_core::ast::Pattern::Assign {
                location: dummy_span(),
                name: "alias".into(),
                pattern: Box::new(gleam_core::ast::Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }),
            }),
            Err(super::super::pattern_type_mismatch(
                ValueType::UtfCodepoint,
                ValueType::Int,
            )),
        );
        assert_eq!(
            super::plan_case_pattern(gleam_core::ast::Pattern::Invalid {
                location: dummy_span(),
                type_,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );
        assert_eq!(
            super::plan_case_pattern(gleam_core::ast::Pattern::Tuple {
                location: dummy_span(),
                elements: Vec::new(),
            }),
            Err(super::super::pattern_type_mismatch(
                ValueType::UtfCodepoint,
                ValueType::Tuple(Vec::new()),
            )),
        );
    }

    #[test]
    fn reject_margin_utf_codepoint_subject_expression_family_mismatch() {
        let mut module = crate::planner::support::compile(
            r#"
fn identity(value: UtfCodepoint) -> UtfCodepoint {
  case value {
    bound -> bound
  }
}

pub fn main() {
  0
}
"#,
        );
        let (_, subjects, _) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        subjects[0] = gleam_core::ast::TypedExpr::Int {
            location: dummy_span(),
            type_: utf_codepoint_type(),
            value: "1".into(),
            int_value: 1.into(),
        };

        assert_eq!(
            plan_module(module),
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::UtfCodepoint,
                InvalidExpressionType::Int,
            )),
        );
    }

    #[test]
    fn reject_profile_utf_codepoint_case_subject_and_branch_errors_propagate() {
        for source in [
            r#"
fn identity(value: UtfCodepoint) -> UtfCodepoint {
  case { <<1:native>> value } { bound -> bound }
}
pub fn main() { 0 }
"#,
            r#"
fn identity(value: UtfCodepoint) -> UtfCodepoint {
  case value { bound -> { <<1:native>> bound } }
}
pub fn main() { 0 }
"#,
        ] {
            assert_eq!(
                plan_module(crate::planner::support::compile(source)),
                Err(PlanError::UnsupportedBitArraySegment {
                    reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
                }),
            );
        }
    }

    #[test]
    fn reject_margin_utf_codepoint_case_return_type_and_nested_pattern_errors_propagate() {
        let mut invalid_return_type = crate::planner::support::compile(
            r#"
fn identity(value: UtfCodepoint) -> UtfCodepoint {
  case value { bound -> bound }
}
pub fn main() { 0 }
"#,
        );
        let (type_, _, _) = super::super::super::expect_case_statement_mut(
            &mut invalid_return_type.definitions.functions[0].body[0],
        );
        *type_ = super::super::mismatched_generic_case_return_type();
        assert_eq!(
            plan_module(invalid_return_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );

        let mut module = crate::planner::support::compile(
            r#"
fn identity(value: UtfCodepoint) -> UtfCodepoint {
  case value { bound -> bound }
}
pub fn main() { 0 }
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut module.definitions.functions[0].body[0],
        );
        clauses[0].pattern[0] = gleam_core::ast::Pattern::Invalid {
            location: dummy_span(),
            type_: utf_codepoint_type(),
        };
        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
                },
            }),
        );
    }

    fn utf_codepoint_type() -> std::sync::Arc<gleam_core::type_::Type> {
        let module = crate::planner::support::compile(
            r#"
fn identity(value: UtfCodepoint) -> UtfCodepoint {
  value
}

pub fn main() {
  0
}
"#,
        );
        module.definitions.functions[0].arguments[0].type_.clone()
    }
}
