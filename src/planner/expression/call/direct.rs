use super::CaptureSubstitution;
use crate::plan::{Expr, FunctionShape};
use crate::planner::context::{FunctionInfo, FunctionParam, PlanContext};
use crate::planner::error::{InvalidCallShapeReason, InvalidTypedAstReason, PlanError};
use crate::planner::type_parameter::FunctionInstantiationMismatch;
use gleam_core::ast::{CallArg as GleamCallArg, SrcSpan, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan_direct_function_call(
    location: SrcSpan,
    type_: Arc<Type>,
    function: FunctionInfo,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Expr, PlanError> {
    if function.arity() != arguments.len() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LocalFunctionCallArityMismatch,
            },
        });
    }
    validate_argument_labels(&arguments, &function.params)?;
    let actual_shape = FunctionShape::new(
        arguments
            .iter()
            .map(|argument| context.value_shape(argument.value.type_().as_ref()))
            .collect(),
        context.value_shape(type_.as_ref()),
    );
    let instantiation = function
        .instantiate(&actual_shape)
        .map_err(function_instantiation_mismatch)?;
    let args = super::argument::plan_instantiated_call_args(
        arguments,
        instantiation.shape().argument_shapes(),
        context,
        capture,
    )?;

    Ok(Expr::call_at(
        instantiation,
        args,
        context.host_call_site(location),
    ))
}

fn function_instantiation_mismatch(mismatch: FunctionInstantiationMismatch) -> PlanError {
    let reason = match mismatch {
        FunctionInstantiationMismatch::ArgumentCount => {
            InvalidCallShapeReason::LocalFunctionCallArityMismatch
        }
        FunctionInstantiationMismatch::ArgumentShape => {
            InvalidCallShapeReason::FunctionCallArgumentTypeMismatch
        }
        FunctionInstantiationMismatch::ReturnShape
        | FunctionInstantiationMismatch::UnresolvedParameter => {
            InvalidCallShapeReason::LocalFunctionCallReturnTypeMismatch
        }
    };
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CallShape { reason },
    }
}

fn validate_argument_labels(
    arguments: &[GleamCallArg<TypedExpr>],
    params: &[FunctionParam],
) -> Result<(), PlanError> {
    for (argument, param) in arguments.iter().zip(params) {
        if let Some(label) = &argument.label
            && param.label.as_ref() != Some(label)
        {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LabelledArguments,
                },
            });
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::function_instantiation_mismatch;
    use crate::plan::{Expr, FunctionType, IntLocalId, LocalId, Step, ValueType};
    use crate::planner::dsl::{
        call_float, call_int_function_at, call_list, float, float_arg, function, host_call_site,
        int, int_arg, int_function_arg, int_function_call_arg, int_function_ref,
        int_return_tail_call_at, let_float_step, let_int_function_step, let_list_step, list,
        list_return_expr, local_float, local_int, local_int_function, local_list, module,
        return_list,
    };
    use crate::planner::expression::call::support::expect_call_statement_mut;
    use crate::planner::expression::{typed_int_expr, typed_string_expr};
    use crate::planner::plan_module;
    use crate::planner::support::compile;
    use crate::planner::type_parameter::FunctionInstantiationMismatch;
    use crate::planner::{InvalidCallShapeReason, InvalidTypedAstReason, PlanError};
    use gleam_core::type_;

    #[test]
    fn function_instantiation_mismatch_preserves_each_call_boundary() {
        for (mismatch, reason) in [
            (
                FunctionInstantiationMismatch::ArgumentCount,
                InvalidCallShapeReason::LocalFunctionCallArityMismatch,
            ),
            (
                FunctionInstantiationMismatch::ArgumentShape,
                InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
            ),
            (
                FunctionInstantiationMismatch::ReturnShape,
                InvalidCallShapeReason::LocalFunctionCallReturnTypeMismatch,
            ),
            (
                FunctionInstantiationMismatch::UnresolvedParameter,
                InvalidCallShapeReason::LocalFunctionCallReturnTypeMismatch,
            ),
        ] {
            assert_eq!(
                function_instantiation_mismatch(mismatch),
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape { reason },
                },
            );
        }
    }

    #[test]
    fn plan_unresolved_direct_call_return_outside_current_template() {
        let source = r#"
fn fail() -> value {
  panic
}

pub fn main() {
  let _ = fail()
  1
}
"#;
        let plan = plan_module(compile(source)).expect("unresolved diverging call should plan");

        assert_eq!(plan.main_function().steps().len(), 1);
        assert_eq!(
            plan.main_function().steps()[0],
            Step::evaluate(Expr::call_at(
                plan.functions()[0].signature().identity_instantiation(),
                Vec::new(),
                host_call_site(source, "main", "fail()"),
            )),
        );
    }

    #[test]
    fn plan_function_value_argument_direct_call() {
        let source = r#"
fn add_one(value: Int) {
  value + 1
}

fn apply(function: fn(Int) -> Int, value: Int) {
  function(value)
}

pub fn main() {
  apply(add_one, 41)
}
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_tail_call_at(
                    2,
                    [
                        int_function_arg(int_function_ref(1, [LocalId::Int(IntLocalId(0))])),
                        int_arg(int(41)),
                    ],
                    host_call_site(source, "main", "apply(add_one, 41)"),
                ),
            ),
            [
                function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value"),
                function(
                    "apply",
                    call_int_function_at(
                        local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                        [int_function_call_arg(local_int(0, "value"))],
                        host_call_site(source, "apply", "function(value)"),
                    ),
                )
                .param_int_function(0, "function", [ValueType::Int])
                .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_local_function_value_argument_direct_call() {
        let source = r#"
fn add_one(value: Int) {
  value + 1
}

fn apply(function: fn(Int) -> Int, value: Int) {
  function(value)
}

pub fn main() {
  let add = add_one
  apply(add, 41)
}
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_tail_call_at(
                    2,
                    [
                        int_function_arg(local_int_function(
                            0,
                            "add",
                            [LocalId::Int(IntLocalId(0))],
                        )),
                        int_arg(int(41)),
                    ],
                    host_call_site(source, "main", "apply(add, 41)"),
                ),
            )
            .step(let_int_function_step(
                0,
                "add",
                int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
            )),
            [
                function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value"),
                function(
                    "apply",
                    call_int_function_at(
                        local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                        [int_function_call_arg(local_int(0, "value"))],
                        host_call_site(source, "apply", "function(value)"),
                    ),
                )
                .param_int_function(0, "function", [ValueType::Int])
                .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_labelled_direct_call_uses_function_param_order() {
        let source = r#"
fn add(to base: Int, value amount: Int) {
  base + amount
}

pub fn main() {
  add(value: 2, to: 40)
}
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_tail_call_at(
                    1,
                    [int_arg(int(40)), int_arg(int(2))],
                    host_call_site(source, "main", "add(value: 2, to: 40)"),
                ),
            ),
            [
                function("add", local_int(0, "base").add_int(local_int(1, "amount")))
                    .param_int(0, "base")
                    .param_int(1, "amount"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_float_and_list_direct_call_shapes() {
        let actual = plan_module(compile(
            r#"
fn half(value: Float) {
  value /. 2.0
}

fn singleton(value: Int) {
  [value]
}

pub fn main() {
  let half_value = half(3.0)
  let values = singleton(1)
  values
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                return_list(list_return_expr(local_list(0, "values", ValueType::Int))),
            )
            .step(let_float_step(
                0,
                "half_value",
                call_float(1, [float_arg(float(3.0))]),
            ))
            .step(let_list_step(
                0,
                "values",
                call_list(2, [int_arg(int(1))], ValueType::Int),
            )),
            [
                function("half", local_float(0, "value").div_float(float(2.0)))
                    .param_float(0, "value"),
                function("singleton", list([local_int(0, "value")], ValueType::Int))
                    .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_direct_local_function_call_shapes() {
        let mut arity_mismatch_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut arity_mismatch_call.definitions.functions[1].body[0]);
        arguments.clear();
        assert_eq!(
            plan_module(arity_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionCallArityMismatch,
                },
            }),
        );

        let mut custom_return_type_mismatch_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (type_, _, _) = expect_call_statement_mut(
            &mut custom_return_type_mismatch_call.definitions.functions[1].body[0],
        );
        *type_ = type_::result(type_::int(), type_::nil());
        assert_eq!(
            plan_module(custom_return_type_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionCallReturnTypeMismatch,
                },
            }),
        );

        let mut unsupported_return_type_call = compile(
            r#"
fn identity(value: Int) { value }
pub fn main() { identity(1) }
"#,
        );
        let (type_, _, _) = expect_call_statement_mut(
            &mut unsupported_return_type_call.definitions.functions[1].body[0],
        );
        *type_ = type_::generic_var(0);
        assert_eq!(
            plan_module(unsupported_return_type_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionCallReturnTypeMismatch,
                },
            }),
        );

        let mut return_type_mismatch_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (type_, _, _) = expect_call_statement_mut(
            &mut return_type_mismatch_call.definitions.functions[1].body[0],
        );
        *type_ = type_::bool();
        assert_eq!(
            plan_module(return_type_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionCallReturnTypeMismatch,
                },
            }),
        );

        assert_call_argument_type_mismatch(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
            "#,
            typed_string_expr("wrong"),
        );

        assert_call_argument_type_mismatch(
            r#"
fn identity(value: String) {
  value
}

pub fn main() {
  identity("ok")
}
            "#,
            typed_int_expr(1),
        );

        assert_call_argument_type_mismatch(
            r#"
fn identity(value: Bool) {
  value
}

pub fn main() {
  identity(True)
}
            "#,
            typed_int_expr(1),
        );

        assert_call_argument_type_mismatch(
            r#"
fn identity(value: Nil) {
  value
}

pub fn main() {
  identity(Nil)
}
            "#,
            typed_int_expr(1),
        );

        let mut wrong_label_call = compile(
            r#"
fn add(to base: Int, value amount: Int) {
  base + amount
}

pub fn main() {
  add(to: 1, value: 2)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut wrong_label_call.definitions.functions[1].body[0]);
        arguments[0].label = Some("wrong".into());
        assert_eq!(
            plan_module(wrong_label_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LabelledArguments,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_function_call_argument_function_shapes() {
        let mut function_mismatch_call = compile(
            r#"
fn apply(function: fn(Int) -> Int) {
  function(1)
}

fn add_one(value: Int) {
  value + 1
}

fn string_identity(value: String) {
  value
}

fn accept_string(function: fn(String) -> String) {
  function("ok")
}

pub fn main() {
  accept_string(string_identity)
  apply(add_one)
}
"#,
        );
        let wrong_function = {
            let (_, _, arguments) = expect_call_statement_mut(
                &mut function_mismatch_call.definitions.functions[4].body[0],
            );
            arguments[0].value.clone()
        };
        let (_, _, arguments) =
            expect_call_statement_mut(&mut function_mismatch_call.definitions.functions[4].body[1]);
        arguments[0].value = wrong_function;
        assert_eq!(
            plan_module(function_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
                },
            }),
        );

        let mut non_function_call = compile(
            r#"
fn apply(function: fn(Int) -> Int) {
  function(1)
}

fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  apply(add_one)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut non_function_call.definitions.functions[2].body[0]);
        arguments[0].value = typed_int_expr(1);
        assert_eq!(
            plan_module(non_function_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
                },
            }),
        );
    }

    fn assert_call_argument_type_mismatch(src: &str, value: gleam_core::ast::TypedExpr) {
        let mut module = compile(src);
        let (_, _, arguments) =
            expect_call_statement_mut(&mut module.definitions.functions[1].body[0]);
        arguments[0].value = value;

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArgumentTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn function_returning_function_call_expr_preserves_return_family() {
        let cases = [
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
            FunctionType::new(vec![ValueType::String], ValueType::String),
            FunctionType::new(vec![ValueType::Float], ValueType::Float),
            FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
            FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
            FunctionType::new(
                vec![ValueType::Tuple(vec![ValueType::Int])],
                ValueType::Tuple(vec![ValueType::Int]),
            ),
            FunctionType::new(
                vec![ValueType::List(Box::new(ValueType::Int))],
                ValueType::List(Box::new(ValueType::Int)),
            ),
            FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
            ),
        ];

        for (template, returned_function_type) in cases.into_iter().enumerate() {
            let shape = crate::plan::FunctionShape::new(
                Vec::new(),
                crate::plan::ValueShape::Function(Box::new(
                    crate::plan::FunctionShape::from_function_type(returned_function_type.clone()),
                )),
            );
            assert_eq!(
                crate::plan::Expr::call(
                    crate::plan::monomorphic_function_instantiation(template, shape.clone()),
                    Vec::new(),
                )
                .value_type(),
                ValueType::Function(Box::new(returned_function_type)),
            );
        }
    }
}
