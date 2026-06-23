use crate::plan::{ExprKind, Step};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidTypedAstReason, PlanError, UnsupportedAssignmentKind, UnsupportedPatternKind,
    UnsupportedStatementKind,
};
use crate::planner::expression::plan_expr;
use ecow::EcoString;
use gleam_core::ast::{AssignmentKind, Pattern, Statement, TypedAssignment, TypedPattern};
use vec1::Vec1;

pub(super) struct PlannedStatements {
    pub(super) steps: Vec<Step>,
    pub(super) return_: crate::plan::Expr,
}

pub(super) fn plan_steps_and_return(
    mut statements: Vec<gleam_core::ast::TypedStatement>,
    context: &mut PlanContext<'_>,
    empty_error: PlanError,
) -> Result<PlannedStatements, PlanError> {
    let Some(last_statement) = statements.pop() else {
        return Err(empty_error);
    };

    plan_ordered_steps_and_return(statements, last_statement, context)
}

pub(super) fn plan_non_empty_steps_and_return(
    statements: Vec1<gleam_core::ast::TypedStatement>,
    context: &mut PlanContext<'_>,
) -> Result<PlannedStatements, PlanError> {
    let (statements, last_statement) = statements.split_off_last();

    plan_ordered_steps_and_return(statements, last_statement, context)
}

fn plan_ordered_steps_and_return(
    statements: Vec<gleam_core::ast::TypedStatement>,
    last_statement: gleam_core::ast::TypedStatement,
    context: &mut PlanContext<'_>,
) -> Result<PlannedStatements, PlanError> {
    let steps = statements
        .into_iter()
        .map(|statement| plan_step(statement, context))
        .collect::<Result<Vec<_>, _>>()?;

    let return_ = match last_statement {
        Statement::Expression(expression) => plan_expr(expression, context)?,
        Statement::Assignment(_) => {
            return Err(PlanError::UnsupportedStatement {
                kind: UnsupportedStatementKind::AssignmentAsFinalStatement,
            });
        }
        Statement::Use(_) => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UseStatement,
            });
        }
        Statement::Assert(_) => {
            return Err(PlanError::UnsupportedStatement {
                kind: UnsupportedStatementKind::AssertAsFinalStatement,
            });
        }
    };

    Ok(PlannedStatements { steps, return_ })
}

pub(super) fn plan_step(
    statement: gleam_core::ast::TypedStatement,
    context: &mut PlanContext<'_>,
) -> Result<Step, PlanError> {
    match statement {
        Statement::Expression(expression) => Ok(Step::evaluate(plan_expr(expression, context)?)),
        Statement::Assignment(assignment) => plan_assignment(*assignment, context),
        Statement::Use(_) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::UseStatement,
        }),
        Statement::Assert(_) => Err(PlanError::UnsupportedStatement {
            kind: UnsupportedStatementKind::Assert,
        }),
    }
}

fn plan_assignment(
    assignment: TypedAssignment,
    context: &mut PlanContext<'_>,
) -> Result<Step, PlanError> {
    match assignment.kind {
        AssignmentKind::Let => {}
        AssignmentKind::Generated => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::GeneratedAssignment,
            });
        }
        AssignmentKind::Assert { .. } => {
            return Err(PlanError::UnsupportedAssignment {
                kind: UnsupportedAssignmentKind::LetAssert,
            });
        }
    }

    let name = plan_variable_pattern(assignment.pattern)?;
    let value = plan_expr(assignment.value, context)?;
    Ok(plan_variable_step(name, value, context))
}

pub(in crate::planner) fn plan_variable_step(
    name: EcoString,
    value: crate::plan::Expr,
    context: &mut PlanContext<'_>,
) -> Step {
    match value.into_kind() {
        ExprKind::Int(value) => {
            let local = context.define_int_local(name.clone());
            Step::let_int(local, name, value)
        }
        ExprKind::String(value) => {
            let local = context.define_string_local(name.clone());
            Step::let_string(local, name, value)
        }
        ExprKind::Bool(value) => {
            let local = context.define_bool_local(name.clone());
            Step::let_bool(local, name, value)
        }
        ExprKind::Nil(value) => {
            let local = context.define_nil_local(name.clone());
            Step::let_nil(local, name, value)
        }
        ExprKind::Function(value) => {
            let type_ = value.type_().clone();
            let local = context.define_function_local(name.clone(), type_);
            Step::let_function(local, name, value)
        }
    }
}

fn plan_variable_pattern(pattern: TypedPattern) -> Result<EcoString, PlanError> {
    match pattern {
        Pattern::Variable { name, .. } => Ok(name),
        Pattern::Assign { .. } => Err(PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::Assign,
        }),
        Pattern::Discard { .. } => Err(PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::Discard,
        }),
        Pattern::Tuple { .. } => Err(PlanError::UnsupportedPattern {
            kind: UnsupportedPatternKind::Tuple,
        }),
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::InvalidPattern,
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::plan_variable_pattern;
    use crate::planner::dsl::{function, int, local_int, module};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidTypedAstReason, PlanError, UnsupportedAssignmentKind, UnsupportedPatternKind,
        UnsupportedStatementKind,
    };
    use gleam_core::analyse::Inferred;
    use gleam_core::ast::{
        AssignName, AssignmentKind, BitArraySize, Pattern, Statement, TypedAssignment, TypedExpr,
    };
    use gleam_core::exhaustiveness::CompiledCase;
    use gleam_core::parse::LiteralFloatValue;
    use gleam_core::type_::{self, error::VariableOrigin};
    use num_bigint::BigInt;

    #[test]
    fn plan_let_and_integer_binop() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let x = 1
  x + 2
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "x").add_int(int(2))).let_int(0, "x", int(1)),
            [],
        );

        assert_eq!(actual, expected);
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
    fn reject_profile_tuple_pattern() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  let #(a, b) = #(1, 2)
  a
}
"#,
            ),
            PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::Tuple,
            },
        );
    }

    #[test]
    fn reject_profile_final_statement_positions() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  let x = 1
}
"#,
            ),
            PlanError::UnsupportedStatement {
                kind: UnsupportedStatementKind::AssignmentAsFinalStatement,
            },
        );

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
    fn reject_profile_use_syntax() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  use <- pair
  1
}

fn pair(callback: fn() -> Int) {
  callback()
}
"#,
            ),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UseStatement,
            },
        );
    }

    #[test]
    fn reject_margin_use_statement_shapes() {
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

        let mut final_use = compile_minimal_module();
        final_use.definitions.functions[0].body = vec![Statement::Use(gleam_core::ast::Use {
            call: Box::new(typed_int_expr(1)),
            location: dummy_span(),
            right_hand_side_location: dummy_span(),
            assignments_location: dummy_span(),
            assignments: Vec::new(),
        })];
        assert_eq!(
            plan_module(final_use),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UseStatement,
            }),
        );
    }

    #[test]
    fn reject_margin_generated_assignment() {
        let mut generated = compile_minimal_module();
        generated.definitions.functions[0].body = vec![
            Statement::Assignment(Box::new(TypedAssignment {
                location: dummy_span(),
                value: typed_int_expr(1),
                pattern: Pattern::Variable {
                    location: dummy_span(),
                    name: "x".into(),
                    type_: type_::int(),
                    origin: VariableOrigin::generated(),
                },
                kind: AssignmentKind::Generated,
                compiled_case: CompiledCase::simple_variable_assignment("x".into(), type_::int()),
                annotation: None,
            })),
            Statement::Expression(typed_int_expr(1)),
        ];
        assert_eq!(
            plan_module(generated),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::GeneratedAssignment,
            }),
        );
    }

    #[test]
    fn reject_profile_let_assert_assignment() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  let assert x = 1
  x
}
"#,
            ),
            PlanError::UnsupportedAssignment {
                kind: UnsupportedAssignmentKind::LetAssert,
            },
        );
    }

    #[test]
    fn reject_profile_non_variable_pattern_shapes() {
        let cases = [
            (
                r#"
pub fn main() {
  let _ = 1
  1
}
"#,
                PlanError::UnsupportedPattern {
                    kind: UnsupportedPatternKind::Discard,
                },
            ),
            (
                r#"
pub fn main() {
  let #(a, b) = #(1, 2)
  a
}
"#,
                PlanError::UnsupportedPattern {
                    kind: UnsupportedPatternKind::Tuple,
                },
            ),
            (
                r#"
pub fn main() {
  let value as alias = 1
  alias
}
"#,
                PlanError::UnsupportedPattern {
                    kind: UnsupportedPatternKind::Assign,
                },
            ),
        ];

        for (src, expected) in cases {
            assert_eq!(expect_plan_error(src), expected);
        }
    }

    #[test]
    fn reject_margin_invalid_pattern_shapes() {
        let variable = |name: &str| Pattern::Variable {
            location: dummy_span(),
            name: name.into(),
            type_: type_::int(),
            origin: VariableOrigin::generated(),
        };

        assert_eq!(plan_variable_pattern(variable("x")), Ok("x".into()));

        let patterns = vec![
            Pattern::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: BigInt::from(1),
            },
            Pattern::Float {
                location: dummy_span(),
                value: "1.0".into(),
                float_value: LiteralFloatValue::ONE,
            },
            Pattern::String {
                location: dummy_span(),
                value: "a".into(),
            },
            Pattern::BitArraySize(BitArraySize::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: BigInt::from(1),
            }),
            Pattern::List {
                location: dummy_span(),
                elements: vec![variable("x")],
                tail: None,
                type_: type_::list(type_::int()),
            },
            Pattern::Constructor {
                location: dummy_span(),
                name_location: dummy_span(),
                name: "Boxed".into(),
                arguments: Vec::new(),
                module: None,
                constructor: Inferred::Unknown,
                spread: None,
                type_: type_::int(),
            },
            Pattern::BitArray {
                location: dummy_span(),
                segments: Vec::new(),
            },
            Pattern::StringPrefix {
                location: dummy_span(),
                left_location: dummy_span(),
                left_side_assignment: None,
                right_location: dummy_span(),
                left_side_string: "pre".into(),
                right_side_assignment: AssignName::Variable("rest".into()),
            },
            Pattern::Invalid {
                location: dummy_span(),
                type_: type_::int(),
            },
        ];

        for pattern in patterns {
            assert_eq!(
                plan_variable_pattern(pattern),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::InvalidPattern,
                }),
            );
        }
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
