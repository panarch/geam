use crate::plan::{FunctionId, FunctionPlan, Param};
use crate::planner::context::{FunctionInfo, PlanContext};
use crate::planner::error::PlanError;
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
            reason: "external functions are not executable by the Geam runtime",
        });
    }

    let mut context = PlanContext::new(module_name, functions);
    let params = plan_params(&mut context, name.clone(), function.arguments)?;
    let mut body = function.body;
    let Some(last_statement) = body.pop() else {
        return Err(PlanError::UnsupportedFunction {
            name,
            reason: "empty function bodies are not supported",
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
                kind: "assignment as final statement",
            });
        }
        Statement::Use(_) => {
            return Err(PlanError::UnsupportedStatement {
                kind: "use as final statement",
            });
        }
        Statement::Assert(_) => {
            return Err(PlanError::UnsupportedStatement {
                kind: "assert as final statement",
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
        .ok_or(PlanError::UnsupportedFunction {
            name: "<anonymous>".into(),
            reason: "anonymous functions are not module functions",
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
                    reason: "discard arguments are not supported",
                });
            };

            if !matches!(argument.names, ArgNames::Named { .. }) {
                return Err(PlanError::UnsupportedArgument {
                    function: function_name.clone(),
                    reason: "labelled arguments are not supported",
                });
            }

            let local = context.define_local(argument_name.clone());
            Ok(Param {
                local,
                name: argument_name,
            })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::planner::PlanError;
    use crate::planner::dsl::{call, function, int, local, module};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use gleam_core::ast::ArgNames;

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
                reason: "labelled arguments are not supported",
            },
        );
    }

    #[test]
    fn reject_margin_function_shapes() {
        let mut external = compile_minimal_module();
        external.definitions.functions[0].external_erlang =
            Some(("module".into(), "function".into(), dummy_span()));
        assert_eq!(
            plan_module(external),
            Err(PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: "external functions are not executable by the Geam runtime",
            }),
        );

        let mut discard_arg = compile(
            r#"
fn helper(value: Int) {
  value
}

pub fn main() {
  helper(1)
}
"#,
        );
        discard_arg.definitions.functions[0].arguments[0].names = ArgNames::Discard {
            name: "_".into(),
            location: dummy_span(),
        };
        assert_eq!(
            plan_module(discard_arg),
            Err(PlanError::UnsupportedArgument {
                function: "helper".into(),
                reason: "discard arguments are not supported",
            }),
        );

        let mut empty_body = compile_minimal_module();
        empty_body.definitions.functions[0].body = Vec::new();
        assert_eq!(
            plan_module(empty_body),
            Err(PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: "empty function bodies are not supported",
            }),
        );
    }
}
