use crate::plan::Step;
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use crate::planner::expression::plan_expr;
use ecow::EcoString;
use gleam_core::ast::{AssignmentKind, Pattern, Statement, TypedAssignment, TypedPattern};

pub(super) fn plan_step(
    statement: gleam_core::ast::TypedStatement,
    context: &mut PlanContext<'_>,
) -> Result<Step, PlanError> {
    match statement {
        Statement::Expression(expression) => Ok(Step::Evaluate(plan_expr(expression, context)?)),
        Statement::Assignment(assignment) => plan_assignment(*assignment, context),
        Statement::Use(_) => Err(PlanError::UnsupportedStatement { kind: "use" }),
        Statement::Assert(_) => Err(PlanError::UnsupportedStatement { kind: "assert" }),
    }
}

fn plan_assignment(
    assignment: TypedAssignment,
    context: &mut PlanContext<'_>,
) -> Result<Step, PlanError> {
    match assignment.kind {
        AssignmentKind::Let => {}
        AssignmentKind::Generated => {
            return Err(PlanError::UnsupportedAssignment { kind: "generated" });
        }
        AssignmentKind::Assert { .. } => {
            return Err(PlanError::UnsupportedAssignment { kind: "let assert" });
        }
    }

    let name = plan_variable_pattern(assignment.pattern)?;
    let value = plan_expr(assignment.value, context)?;
    let local = context.define_local(name.clone());

    Ok(Step::Let { local, name, value })
}

fn plan_variable_pattern(pattern: TypedPattern) -> Result<EcoString, PlanError> {
    match pattern {
        Pattern::Variable { name, .. } => Ok(name),
        Pattern::Int { .. } => Err(PlanError::UnsupportedPattern { kind: "int" }),
        Pattern::Float { .. } => Err(PlanError::UnsupportedPattern { kind: "float" }),
        Pattern::String { .. } => Err(PlanError::UnsupportedPattern { kind: "string" }),
        Pattern::BitArraySize(_) => Err(PlanError::UnsupportedPattern {
            kind: "bit array size",
        }),
        Pattern::Assign { .. } => Err(PlanError::UnsupportedPattern { kind: "assign" }),
        Pattern::Discard { .. } => Err(PlanError::UnsupportedPattern { kind: "discard" }),
        Pattern::List { .. } => Err(PlanError::UnsupportedPattern { kind: "list" }),
        Pattern::Constructor { .. } => Err(PlanError::UnsupportedPattern {
            kind: "constructor",
        }),
        Pattern::Tuple { .. } => Err(PlanError::UnsupportedPattern { kind: "tuple" }),
        Pattern::BitArray { .. } => Err(PlanError::UnsupportedPattern { kind: "bit array" }),
        Pattern::StringPrefix { .. } => Err(PlanError::UnsupportedPattern {
            kind: "string prefix",
        }),
        Pattern::Invalid { .. } => Err(PlanError::UnsupportedPattern { kind: "invalid" }),
    }
}

#[cfg(test)]
mod tests {
    use super::plan_variable_pattern;
    use crate::planner::PlanError;
    use crate::planner::dsl::{function, int, local, module};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use gleam_core::analyse::Inferred;
    use gleam_core::ast::{
        AssignName, AssignmentKind, BitArraySize, Pattern, Statement, TypedAssignment, TypedExpr,
        TypedPattern,
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
        let expected = module("main")
            .function(
                function("main")
                    .let_("x", int(1))
                    .return_(local("x").add_int(int(2))),
            )
            .build();

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
        let expected = module("main")
            .function(function("main").evaluate(int(1)).return_(int(2)))
            .build();

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
            PlanError::UnsupportedStatement { kind: "assert" },
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
            PlanError::UnsupportedPattern { kind: "tuple" },
        );
    }

    #[test]
    fn reject_margin_statement_positions() {
        let mut final_assignment = compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        );
        let assignment = final_assignment.definitions.functions[0].body.remove(0);
        final_assignment.definitions.functions[0].body = vec![assignment];
        assert_eq!(
            plan_module(final_assignment),
            Err(PlanError::UnsupportedStatement {
                kind: "assignment as final statement",
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
            Err(PlanError::UnsupportedStatement {
                kind: "use as final statement",
            }),
        );

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
            Err(PlanError::UnsupportedStatement { kind: "use" }),
        );

        let mut final_assert = compile(
            r#"
pub fn main() {
  assert True
  1
}
"#,
        );
        let assert_statement = final_assert.definitions.functions[0].body.remove(0);
        final_assert.definitions.functions[0].body = vec![assert_statement];
        assert_eq!(
            plan_module(final_assert),
            Err(PlanError::UnsupportedStatement {
                kind: "assert as final statement",
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
            Err(PlanError::UnsupportedAssignment { kind: "generated" }),
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
            PlanError::UnsupportedAssignment { kind: "let assert" },
        );
    }

    #[test]
    fn reject_margin_non_variable_pattern_shapes() {
        let variable = |name: &str| Pattern::Variable {
            location: dummy_span(),
            name: name.into(),
            type_: type_::int(),
            origin: VariableOrigin::generated(),
        };

        assert_eq!(plan_variable_pattern(variable("x")), Ok("x".into()));

        let cases: Vec<(TypedPattern, &str)> = vec![
            (
                Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: BigInt::from(1),
                },
                "int",
            ),
            (
                Pattern::Float {
                    location: dummy_span(),
                    value: "1.0".into(),
                    float_value: LiteralFloatValue::ONE,
                },
                "float",
            ),
            (
                Pattern::String {
                    location: dummy_span(),
                    value: "a".into(),
                },
                "string",
            ),
            (
                Pattern::BitArraySize(BitArraySize::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: BigInt::from(1),
                }),
                "bit array size",
            ),
            (
                Pattern::Assign {
                    name: "x".into(),
                    location: dummy_span(),
                    pattern: Box::new(variable("inner")),
                },
                "assign",
            ),
            (
                Pattern::Discard {
                    name: "_".into(),
                    location: dummy_span(),
                    type_: type_::int(),
                },
                "discard",
            ),
            (
                Pattern::List {
                    location: dummy_span(),
                    elements: vec![variable("x")],
                    tail: None,
                    type_: type_::list(type_::int()),
                },
                "list",
            ),
            (
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
                "constructor",
            ),
            (
                Pattern::Tuple {
                    location: dummy_span(),
                    elements: vec![variable("x")],
                },
                "tuple",
            ),
            (
                Pattern::BitArray {
                    location: dummy_span(),
                    segments: Vec::new(),
                },
                "bit array",
            ),
            (
                Pattern::StringPrefix {
                    location: dummy_span(),
                    left_location: dummy_span(),
                    left_side_assignment: None,
                    right_location: dummy_span(),
                    left_side_string: "pre".into(),
                    right_side_assignment: AssignName::Variable("rest".into()),
                },
                "string prefix",
            ),
            (
                Pattern::Invalid {
                    location: dummy_span(),
                    type_: type_::int(),
                },
                "invalid",
            ),
        ];

        for (pattern, kind) in cases {
            assert_eq!(
                plan_variable_pattern(pattern),
                Err(PlanError::UnsupportedPattern { kind }),
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
