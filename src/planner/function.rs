use crate::plan::ValueType;
use crate::plan::{FunctionId, FunctionPlan, Param};
use crate::planner::context::{FunctionInfo, PlanContext};
use crate::planner::error::{
    InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedArgumentReason,
    UnsupportedFunctionReason, UnsupportedStatementKind,
};
use crate::planner::expression::plan_expr;
use crate::planner::statement::plan_step;
use ecow::EcoString;
use gleam_core::ast::{ArgNames, Statement, TypedFunction};
use std::collections::HashMap;

pub(super) fn plan_function(
    id: FunctionId,
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
    let params = plan_params(&mut context, name.clone(), function.arguments)?;
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

    Ok(FunctionPlan {
        id,
        name,
        params,
        steps,
        return_,
    })
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

fn plan_params(
    context: &mut PlanContext<'_>,
    function_name: EcoString,
    arguments: Vec<gleam_core::ast::TypedArg>,
) -> Result<Vec<Param>, PlanError> {
    arguments
        .into_iter()
        .map(|argument| {
            let Some(argument_name) = argument.names.get_variable_name().cloned() else {
                return Err(PlanError::UnsupportedArgument {
                    function: function_name.clone(),
                    reason: UnsupportedArgumentReason::Discard,
                });
            };

            if !matches!(argument.names, ArgNames::Named { .. }) {
                return Err(PlanError::UnsupportedArgument {
                    function: function_name.clone(),
                    reason: UnsupportedArgumentReason::Labelled,
                });
            }

            let Some(type_) = ValueType::from_gleam(&argument.type_) else {
                return Err(PlanError::UnsupportedArgument {
                    function: function_name.clone(),
                    reason: UnsupportedArgumentReason::UnsupportedType,
                });
            };
            let local = context.define_local(argument_name.clone(), type_);
            Ok(Param {
                local,
                name: argument_name,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::planner::dsl::{bool_, call, function, int, local, module, nil, string};
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
        let expected = module("main")
            .function(
                function("add")
                    .param("a")
                    .param("b")
                    .return_(local("a").add_int(local("b"))),
            )
            .function(function("main").return_(call("add", [int(1), int(2)])))
            .build();

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
        let expected = module("main")
            .function(
                function("string_id")
                    .param_string("value")
                    .return_(local("value")),
            )
            .function(
                function("bool_id")
                    .param_bool("value")
                    .return_(local("value")),
            )
            .function(
                function("nil_id")
                    .param_nil("value")
                    .return_(local("value")),
            )
            .function(function("main").return_(call("string_id", [string("geam")])))
            .function(function("bool_main").return_(call("bool_id", [bool_(true)])))
            .function(function("nil_main").return_(call("nil_id", [nil()])))
            .build();

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
