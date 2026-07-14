use super::super::super::plan_expr_with_expected_source_stop_type;
use super::super::invalid_case_shape;
use super::{CaseClause, OrderedCaseClauseInput, case_return_type};
use crate::plan::{
    BoolExpr, Expr, ExprKind, Step, UtfCodepointExpr, UtfCodepointLocalId, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError};
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
    let return_type = case_return_type(type_.as_ref())?;
    let ExprKind::UtfCodepoint(subject) = subject.into_kind() else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    };
    let (subject_step, subject) = bind_case_subject(subject, context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let pattern = plan_case_pattern(pattern)?;
            let bindings = super::branch_bindings(pattern.bound_names(), subject.clone());
            let is_total = clause.guard.is_none();
            ordered_clauses.push(super::plan_ordered_case_clause(
                OrderedCaseClauseInput {
                    case_type: type_.as_ref(),
                    return_type: &return_type,
                    then: clause.then.clone(),
                    branch_bindings: bindings,
                    guard: clause.guard.clone(),
                    match_condition: BoolExpr::value(true),
                    is_total,
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

fn plan_case_pattern(pattern: Pattern<Arc<Type>>) -> Result<UtfCodepointCasePattern, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. }
            if ValueType::from_gleam(type_.as_ref()) == Some(ValueType::UtfCodepoint) =>
        {
            Ok(UtfCodepointCasePattern {
                bound_names: vec![name],
            })
        }
        Pattern::Discard { type_, .. }
            if ValueType::from_gleam(type_.as_ref()) == Some(ValueType::UtfCodepoint) =>
        {
            Ok(UtfCodepointCasePattern {
                bound_names: Vec::new(),
            })
        }
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_case_pattern(*pattern)?;
            pattern.add_bound_name(name);
            Ok(pattern)
        }
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        Pattern::Variable { .. }
        | Pattern::Discard { .. }
        | Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
    }
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
    use crate::planner::dsl::{
        function, int, let_utf_codepoint_step, local_utf_codepoint, module,
        utf_codepoint_return_block, utf_codepoint_return_expr,
    };
    use crate::planner::plan_module;
    use crate::planner::support::dummy_span;
    use crate::planner::{
        InvalidCaseShapeReason, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
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
            Err(pattern_type_mismatch()),
        );
        assert_eq!(
            super::plan_case_pattern(gleam_core::ast::Pattern::Discard {
                location: dummy_span(),
                name: "_".into(),
                type_: gleam_core::type_::int(),
            }),
            Err(pattern_type_mismatch()),
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
            Err(pattern_type_mismatch()),
        );
        assert_eq!(
            super::plan_case_pattern(gleam_core::ast::Pattern::Invalid {
                location: dummy_span(),
                type_,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );
        assert_eq!(
            super::plan_case_pattern(gleam_core::ast::Pattern::Tuple {
                location: dummy_span(),
                elements: Vec::new(),
            }),
            Err(pattern_type_mismatch()),
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

        assert_eq!(plan_module(module), Err(pattern_type_mismatch()));
    }

    #[test]
    fn reject_profile_utf_codepoint_case_subject_and_branch_errors_propagate() {
        for source in [
            r#"
fn identity(value: UtfCodepoint) -> UtfCodepoint {
  case echo value { bound -> bound }
}
pub fn main() { 0 }
"#,
            r#"
fn identity(value: UtfCodepoint) -> UtfCodepoint {
  case value { bound -> echo bound }
}
pub fn main() { 0 }
"#,
        ] {
            assert_eq!(
                plan_module(crate::planner::support::compile(source)),
                Err(PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::Echo,
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
        *type_ = super::super::unsupported_case_return_type();
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
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
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

    fn pattern_type_mismatch() -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CaseShape {
                reason: InvalidCaseShapeReason::PatternTypeMismatch,
            },
        }
    }
}
