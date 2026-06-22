use crate::plan::{FunctionPlan, Param};
use crate::planner::context::{FunctionInfo, PlanContext};
use crate::planner::error::{
    InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedFunctionReason,
    UnsupportedStatementKind,
};
use crate::planner::expression::plan_expr;
use crate::planner::statement::plan_step;
use ecow::EcoString;
use gleam_core::ast::{Statement, TypedFunction};
use std::collections::HashMap;

pub(super) fn plan_function(
    info: FunctionInfo,
    module_name: &EcoString,
    functions: &HashMap<EcoString, FunctionInfo>,
    function: TypedFunction,
) -> Result<FunctionPlan, PlanError> {
    let name = function_name(&function)?;

    if function.external_erlang.is_some() || function.external_javascript.is_some() {
        return Err(PlanError::UnsupportedFunction {
            name,
            reason: UnsupportedFunctionReason::External,
        });
    }

    let mut context = PlanContext::new(module_name, functions);
    let params = info
        .params
        .iter()
        .map(|param| {
            context.define_existing_local(param.name.clone(), param.local);
            Param::new(param.local, param.name.clone())
        })
        .collect();
    let mut body = function.body;
    let Some(last_statement) = body.pop() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name,
                reason: InvalidFunctionShapeReason::EmptyBody,
            },
        });
    };

    let steps = body
        .into_iter()
        .map(|statement| plan_step(statement, &mut context))
        .collect::<Result<Vec<_>, _>>()?;

    let return_ = match last_statement {
        Statement::Expression(expression) => plan_expr(expression, &mut context)?,
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

    Ok(FunctionPlan::new(info.id, name, params, steps, return_))
}

pub(super) fn function_name(function: &TypedFunction) -> Result<EcoString, PlanError> {
    function
        .name
        .as_ref()
        .map(|(_, name)| name.clone())
        .ok_or_else(|| PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: "<anonymous>".into(),
                reason: InvalidFunctionShapeReason::Anonymous,
            },
        })
}

#[cfg(test)]
mod tests {
    use crate::planner::dsl::{
        bool_, bool_arg, call_bool, call_int, call_nil, call_string, function, int, int_arg,
        local_bool, local_int, local_nil, local_string, module, nil, nil_arg, string, string_arg,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, expect_plan_error};
    use crate::planner::{
        InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedArgumentReason,
        UnsupportedFunctionReason,
    };

    #[test]
    fn plan_local_function_call() {
        let actual = plan_module(compile(
            r#"
fn add(a: Int, b: Int) {
  a + b
}

pub fn main() {
  add(1, 2)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                call_int(1, [int_arg(0, int(1)), int_arg(1, int(2))]),
            ),
            [
                function("add", local_int(0, "a").add_int(local_int(1, "b")))
                    .param_int(0, "a")
                    .param_int(1, "b"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_main_as_local_function_call() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1
}

pub fn helper() {
  main()
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1)),
            [function("helper", call_int(0, []))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_typed_local_function_calls() {
        let actual = plan_module(compile(
            r#"
pub fn string_id(value: String) {
  value
}

pub fn bool_id(value: Bool) {
  value
}

pub fn nil_id(value: Nil) {
  value
}

pub fn main() {
  string_id("geam")
}

pub fn bool_main() {
  bool_id(True)
}

pub fn nil_main() {
  nil_id(Nil)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", call_string(1, [string_arg(0, string("geam"))])),
            [
                function("string_id", local_string(0, "value")).param_string(0, "value"),
                function("bool_id", local_bool(0, "value")).param_bool(0, "value"),
                function("nil_id", local_nil(0, "value")).param_nil(0, "value"),
                function("bool_main", call_bool(0, [bool_arg(0, bool_(true))])),
                function("nil_main", call_nil(0, [nil_arg(0, nil())])),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_labelled_arguments() {
        assert_eq!(
            expect_plan_error(
                r#"
fn identity(value value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
            ),
            PlanError::UnsupportedArgument {
                function: "identity".into(),
                reason: UnsupportedArgumentReason::Labelled,
            },
        );
    }

    #[test]
    fn reject_profile_function_shapes() {
        assert_eq!(
            expect_plan_error(
                r#"
@external(erlang, "one", "two")
fn main() -> Int
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: UnsupportedFunctionReason::External,
            },
        );

        assert_eq!(
            expect_plan_error(
                r#"
fn helper(_: Int) {
  1
}

pub fn main() {
  helper(1)
}
"#,
            ),
            PlanError::UnsupportedArgument {
                function: "helper".into(),
                reason: UnsupportedArgumentReason::Discard,
            },
        );
    }

    #[test]
    fn reject_margin_function_shapes() {
        let mut empty_body = compile_minimal_module();
        empty_body.definitions.functions[0].body = Vec::new();
        assert_eq!(
            plan_module(empty_body),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::EmptyBody,
                },
            }),
        );

        let mut anonymous = compile_minimal_module();
        anonymous.definitions.functions[0].name = None;
        assert_eq!(
            plan_module(anonymous),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous>".into(),
                    reason: InvalidFunctionShapeReason::Anonymous,
                },
            }),
        );
    }
}
