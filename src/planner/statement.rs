mod assignment;
mod use_;

pub(in crate::planner) use assignment::plan_variable_runtime_step;

use crate::plan::{Step, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidTypedAstReason, PlanError, UnsupportedStatementKind};
use crate::planner::expression::{plan_expr, plan_expr_with_expected_source_stop_type};
use gleam_core::ast::Statement;
use vec1::Vec1;

pub(super) struct PlannedStatements {
    pub(super) steps: Vec<Step>,
    pub(super) return_: crate::plan::Expr,
}

pub(super) fn plan_steps_and_return(
    mut statements: Vec<gleam_core::ast::TypedStatement>,
    context: &mut PlanContext<'_>,
    empty_error: PlanError,
    expected_return_type: Option<&ValueType>,
) -> Result<PlannedStatements, PlanError> {
    let Some(last_statement) = statements.pop() else {
        return Err(empty_error);
    };

    plan_ordered_steps_and_return(statements, last_statement, context, expected_return_type)
}

pub(super) fn plan_non_empty_steps_and_return(
    statements: Vec1<gleam_core::ast::TypedStatement>,
    context: &mut PlanContext<'_>,
    expected_return_type: Option<&ValueType>,
) -> Result<PlannedStatements, PlanError> {
    let (statements, last_statement) = statements.split_off_last();

    plan_ordered_steps_and_return(statements, last_statement, context, expected_return_type)
}

fn plan_ordered_steps_and_return(
    statements: Vec<gleam_core::ast::TypedStatement>,
    last_statement: gleam_core::ast::TypedStatement,
    context: &mut PlanContext<'_>,
    expected_return_type: Option<&ValueType>,
) -> Result<PlannedStatements, PlanError> {
    let mut steps = Vec::new();
    for statement in statements {
        steps.extend(plan_runtime_steps(statement, context)?);
    }

    let return_ = match last_statement {
        Statement::Expression(expression) => match expected_return_type {
            Some(type_) => {
                plan_expr_with_expected_source_stop_type(expression, type_.clone(), context)?
            }
            None => plan_expr(expression, context)?,
        },
        Statement::Assignment(assignment) => {
            let planned = assignment::plan_final_assignment(*assignment, context)?;
            steps.extend(planned.steps);
            planned.value
        }
        Statement::Use(use_) => use_::plan_use_statement(use_, context)?,
        Statement::Assert(_) => {
            return Err(PlanError::UnsupportedStatement {
                kind: UnsupportedStatementKind::AssertAsFinalStatement,
            });
        }
    };

    Ok(PlannedStatements { steps, return_ })
}

pub(super) fn plan_runtime_steps(
    statement: gleam_core::ast::TypedStatement,
    context: &mut PlanContext<'_>,
) -> Result<Vec<Step>, PlanError> {
    match statement {
        Statement::Expression(expression) => Ok(vec![Step::evaluate(
            plan_expr_with_expected_source_stop_type(expression, ValueType::Nil, context)?,
        )]),
        Statement::Assignment(assignment) => assignment::plan_assignment(*assignment, context),
        Statement::Use(_) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::UseStatement,
        }),
        Statement::Assert(_) => Err(PlanError::UnsupportedStatement {
            kind: UnsupportedStatementKind::Assert,
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::dsl::{function, int, module};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidTypedAstReason, PlanError, UnsupportedExpressionKind, UnsupportedStatementKind,
    };
    use gleam_core::ast::{Statement, TypedExpr};
    use gleam_core::type_;
    use num_bigint::BigInt;
    use std::collections::HashMap;
    use vec1::Vec1;

    #[test]
    fn plan_final_expression_without_expected_return_type_uses_plain_expression_lowering() {
        let mut module = compile("pub fn main() { 1 }");
        let statement = module.definitions.functions[0].body.remove(0);
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        let actual =
            super::plan_non_empty_steps_and_return(Vec1::new(statement), &mut context, None)
                .expect("statement should plan");

        assert_eq!(actual.steps, []);
        assert_eq!(actual.return_, int(1).into());
    }

    #[test]
    fn plan_final_expression_without_expected_return_type_propagates_expression_error() {
        let mut module = compile("pub fn main() { echo 1 }");
        let statement = module.definitions.functions[0].body.remove(0);
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        let actual =
            super::plan_non_empty_steps_and_return(Vec1::new(statement), &mut context, None).err();

        assert_eq!(
            actual,
            Some(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            }),
        );
    }

    #[test]
    fn plan_expression_statement_steps() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1
  2
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main", function("main", int(2)).evaluate(int(1)), []);

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_assert_statement() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  assert True
  1
}
"#,
            ),
            PlanError::UnsupportedStatement {
                kind: UnsupportedStatementKind::Assert,
            },
        );
    }

    #[test]
    fn reject_profile_final_assert_statement() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  assert True
}
"#,
            ),
            PlanError::UnsupportedStatement {
                kind: UnsupportedStatementKind::AssertAsFinalStatement,
            },
        );
    }

    #[test]
    fn reject_margin_step_use_statement_shape() {
        let mut step_use = compile_minimal_module();
        step_use.definitions.functions[0].body = vec![
            Statement::Use(gleam_core::ast::Use {
                call: Box::new(typed_int_expr(1)),
                location: dummy_span(),
                right_hand_side_location: dummy_span(),
                assignments_location: dummy_span(),
                assignments: Vec::new(),
            }),
            Statement::Expression(typed_int_expr(1)),
        ];
        assert_eq!(
            plan_module(step_use),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UseStatement,
            }),
        );
    }

    fn typed_int_expr(value: i64) -> TypedExpr {
        TypedExpr::Int {
            location: dummy_span(),
            type_: type_::int(),
            value: value.to_string().into(),
            int_value: BigInt::from(value),
        }
    }
}
