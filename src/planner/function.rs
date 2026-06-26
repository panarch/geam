use crate::plan::{
    Expr, ExprKind, FunctionPlan, FunctionType, Param, ReturnExpr, RuntimeFunctionId, ValueType,
};
use crate::planner::context::{FunctionInfo, FunctionParam, PlanContext};
use crate::planner::error::{
    InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedFunctionReason,
};
use crate::planner::statement::plan_steps_and_return;
use ecow::EcoString;
use gleam_core::ast::TypedFunction;
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
    validate_function_param_types(&name, &info.type_, &info.params)?;
    let params = info
        .params
        .iter()
        .map(|param| {
            context.define_existing_local(param.name.clone(), param.local, param.type_.clone());
            Param::new(param.local, param.name.clone())
        })
        .collect();
    let planned = plan_steps_and_return(
        function.body,
        &mut context,
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: name.clone(),
                reason: InvalidFunctionShapeReason::EmptyBody,
            },
        },
    )?;
    let return_ = function_return_expr(&name, &info.return_type, planned.return_)?;

    Ok(FunctionPlan::new(
        info.id,
        name,
        params,
        planned.steps,
        return_,
    ))
}

fn function_return_expr(
    name: &EcoString,
    expected: &ValueType,
    actual: Expr,
) -> Result<ReturnExpr, PlanError> {
    match (expected, actual.into_kind()) {
        (ValueType::Int, ExprKind::Int(actual)) => Ok(ReturnExpr::int(actual)),
        (ValueType::String, ExprKind::String(actual)) => Ok(ReturnExpr::string(actual)),
        (ValueType::Bool, ExprKind::Bool(actual)) => Ok(ReturnExpr::bool(actual)),
        (ValueType::Nil, ExprKind::Nil(actual)) => Ok(ReturnExpr::nil(actual)),
        _ => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: name.clone(),
                reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
            },
        }),
    }
}

pub(in crate::planner) fn validate_function_param_types(
    name: &EcoString,
    expected: &FunctionType,
    params: &[FunctionParam],
) -> Result<(), PlanError> {
    if expected.arguments().len() != params.len() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: name.clone(),
                reason: InvalidFunctionShapeReason::ArityMismatch,
            },
        });
    }

    for (expected, param) in expected.arguments().iter().zip(params) {
        if expected != &param.type_ {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: name.clone(),
                    reason: InvalidFunctionShapeReason::ArgumentTypeMismatch,
                },
            });
        }
    }

    Ok(())
}

pub(in crate::planner) fn validate_function_runtime_id(
    name: &EcoString,
    expected: &FunctionType,
    runtime_id: RuntimeFunctionId,
) -> Result<(), PlanError> {
    let matches = matches!(
        (runtime_id, expected.return_()),
        (RuntimeFunctionId::Int(_), ValueType::Int)
            | (RuntimeFunctionId::String(_), ValueType::String)
            | (RuntimeFunctionId::Bool(_), ValueType::Bool)
            | (RuntimeFunctionId::Nil(_), ValueType::Nil)
    );

    if matches {
        return Ok(());
    }

    Err(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::FunctionShape {
            name: name.clone(),
            reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
        },
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

#[cfg(test)]
mod tests {
    use super::{validate_function_param_types, validate_function_runtime_id};
    use crate::plan::{
        FunctionType, IntFunctionId, IntLocalId, LocalId, RuntimeFunctionId, StringFunctionId,
        StringLocalId, ValueType,
    };
    use crate::planner::context::FunctionParam;
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

        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  [1]
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
            },
        );

        assert_eq!(
            expect_plan_error(
                r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
}
"#,
            ),
            PlanError::UnsupportedFunction {
                name: "main".into(),
                reason: UnsupportedFunctionReason::UnsupportedReturnType,
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

        let mut return_type_mismatch = compile_minimal_module();
        return_type_mismatch.definitions.functions[0].return_type = gleam_core::type_::bool();
        assert_eq!(
            plan_module(return_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_function_param_type_validation() {
        let name = "main".into();
        let type_ = FunctionType::new(vec![ValueType::Int], ValueType::Int);

        assert_eq!(
            validate_function_param_types(&name, &type_, &[]),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ArityMismatch,
                },
            }),
        );

        assert_eq!(
            validate_function_param_types(
                &name,
                &type_,
                &[FunctionParam {
                    local: LocalId::String(StringLocalId(0)),
                    name: "value".into(),
                    type_: ValueType::String,
                }],
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ArgumentTypeMismatch,
                },
            }),
        );

        assert_eq!(
            validate_function_param_types(
                &name,
                &type_,
                &[FunctionParam {
                    local: LocalId::Int(IntLocalId(0)),
                    name: "value".into(),
                    type_: ValueType::Int,
                }],
            ),
            Ok(()),
        );
    }

    #[test]
    fn reject_margin_function_runtime_id_validation() {
        let name = "main".into();
        let type_ = FunctionType::new(Vec::new(), ValueType::String);

        assert_eq!(
            validate_function_runtime_id(&name, &type_, RuntimeFunctionId::Int(IntFunctionId(0)),),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );

        assert_eq!(
            validate_function_runtime_id(
                &name,
                &type_,
                RuntimeFunctionId::String(StringFunctionId(0)),
            ),
            Ok(()),
        );
    }
}
