use super::CaptureSubstitution;
use crate::plan::{
    BitArrayExpr, BoolExpr, CallArg, Expr, FloatExpr, FunctionExpr, FunctionFunctionExpr, IntExpr,
    ListExpr, ListFunctionExpr, NilExpr, RuntimeFunctionId, StringExpr, TupleExpr,
    TupleFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr, ValueType,
};
use crate::planner::context::{FunctionInfo, FunctionParam, PlanContext};
use crate::planner::error::{InvalidCallShapeReason, InvalidTypedAstReason, PlanError};
use gleam_core::ast::{CallArg as GleamCallArg, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan_direct_function_call(
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
    let function_return_type = function.return_type();
    let function_id = function.runtime_id;
    let return_type = ValueType::from_gleam(type_.as_ref()).ok_or(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CallShape {
            reason: InvalidCallShapeReason::LocalFunctionCallUnsupportedReturnType,
        },
    })?;
    if return_type != function_return_type {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LocalFunctionCallReturnTypeMismatch,
            },
        });
    }
    validate_argument_labels(&arguments, &function.params)?;
    let args = super::argument::plan_call_args(arguments, &function.params, context, capture)?;

    Ok(call_expr(function_id, args))
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

fn call_expr(function: RuntimeFunctionId, args: Vec<CallArg>) -> Expr {
    match function {
        RuntimeFunctionId::Int(function) => Expr::int(IntExpr::call(function, args)),
        RuntimeFunctionId::String(function) => Expr::string(StringExpr::call(function, args)),
        RuntimeFunctionId::BitArray(function) => {
            Expr::bit_array(BitArrayExpr::call(function, args))
        }
        RuntimeFunctionId::UtfCodepoint(function) => {
            Expr::utf_codepoint(UtfCodepointExpr::call(function, args))
        }
        RuntimeFunctionId::Float(function) => Expr::float(FloatExpr::call(function, args)),
        RuntimeFunctionId::Bool(function) => Expr::bool(BoolExpr::call(function, args)),
        RuntimeFunctionId::Nil(function) => Expr::nil(NilExpr::call(function, args)),
        RuntimeFunctionId::Tuple { id, return_type } => {
            Expr::tuple(TupleExpr::call(id, args, return_type))
        }
        RuntimeFunctionId::List(id) => Expr::list(ListExpr::call(id, args)),
        RuntimeFunctionId::Function { id, return_type } => {
            function_returning_function_call_expr(id, args, return_type)
        }
    }
}

fn function_returning_function_call_expr(
    function: crate::plan::FunctionFunctionId,
    args: Vec<CallArg>,
    return_type: crate::plan::FunctionType,
) -> Expr {
    match function {
        crate::plan::FunctionFunctionId::Int(function) => Expr::function(FunctionExpr::int(
            crate::plan::IntFunctionExpr::call(function, args, return_type),
        )),
        crate::plan::FunctionFunctionId::String(function) => Expr::function(FunctionExpr::string(
            crate::plan::StringFunctionExpr::call(function, args, return_type),
        )),
        crate::plan::FunctionFunctionId::BitArray(function) => {
            Expr::function(FunctionExpr::bit_array(
                crate::plan::BitArrayFunctionExpr::call(function, args, return_type),
            ))
        }
        crate::plan::FunctionFunctionId::UtfCodepoint(function) => {
            Expr::function(FunctionExpr::utf_codepoint(UtfCodepointFunctionExpr::call(
                function,
                args,
                return_type,
            )))
        }
        crate::plan::FunctionFunctionId::Float(function) => Expr::function(FunctionExpr::float(
            crate::plan::FloatFunctionExpr::call(function, args, return_type),
        )),
        crate::plan::FunctionFunctionId::Bool(function) => Expr::function(FunctionExpr::bool(
            crate::plan::BoolFunctionExpr::call(function, args, return_type),
        )),
        crate::plan::FunctionFunctionId::Nil(function) => Expr::function(FunctionExpr::nil(
            crate::plan::NilFunctionExpr::call(function, args, return_type),
        )),
        crate::plan::FunctionFunctionId::Tuple(function) => Expr::function(FunctionExpr::tuple(
            TupleFunctionExpr::call(function, args, return_type),
        )),
        crate::plan::FunctionFunctionId::List(function) => {
            Expr::function(FunctionExpr::list(ListFunctionExpr::call(function, args)))
        }
        crate::plan::FunctionFunctionId::Function(function) => Expr::function(
            FunctionExpr::function(FunctionFunctionExpr::call(function, args, return_type)),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::function_returning_function_call_expr;
    use crate::plan::{
        BoolFunctionFunctionId, FloatFunctionFunctionId, FunctionFunctionFunctionId,
        FunctionFunctionId, FunctionType, IntFunctionFunctionId, IntLocalId,
        ListFunctionFunctionId, LocalId, NilFunctionFunctionId, StringFunctionFunctionId,
        TupleFunctionFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        call_float, call_int_function, call_list, float, float_arg, function, int, int_arg,
        int_function_arg, int_function_call_arg, int_function_ref, int_return_tail_call,
        let_float_step, let_int_function_step, let_list_step, list, list_return_expr, local_float,
        local_int, local_int_function, local_list, module, return_list,
    };
    use crate::planner::expression::call::support::expect_call_statement_mut;
    use crate::planner::expression::{typed_int_expr, typed_string_expr};
    use crate::planner::plan_module;
    use crate::planner::support::compile;
    use crate::planner::{
        InvalidCallShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    };
    use gleam_core::type_;

    #[test]
    fn plan_function_value_argument_direct_call() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn apply(function: fn(Int) -> Int, value: Int) {
  function(value)
}

pub fn main() {
  apply(add_one, 41)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_tail_call(
                    2,
                    [
                        int_function_arg(0, int_function_ref(1, [LocalId::Int(IntLocalId(0))])),
                        int_arg(0, int(41)),
                    ],
                ),
            ),
            [
                function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value"),
                function(
                    "apply",
                    call_int_function(
                        local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                        [int_function_call_arg(0, local_int(0, "value"))],
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
        let actual = plan_module(compile(
            r#"
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
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_tail_call(
                    2,
                    [
                        int_function_arg(
                            0,
                            local_int_function(0, "add", [LocalId::Int(IntLocalId(0))]),
                        ),
                        int_arg(0, int(41)),
                    ],
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
                    call_int_function(
                        local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                        [int_function_call_arg(0, local_int(0, "value"))],
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
        let actual = plan_module(compile(
            r#"
fn add(to base: Int, value amount: Int) {
  base + amount
}

pub fn main() {
  add(value: 2, to: 40)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_tail_call(1, [int_arg(0, int(40)), int_arg(1, int(2))]),
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
                call_float(0, [float_arg(0, float(3.0))]),
            ))
            .step(let_list_step(
                0,
                "values",
                call_list(1, [int_arg(0, int(1))], ValueType::Int),
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

        let mut unsupported_return_type_call = compile(
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
            &mut unsupported_return_type_call.definitions.functions[1].body[0],
        );
        *type_ = type_::result(type_::int(), type_::nil());
        assert_eq!(
            plan_module(unsupported_return_type_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionCallUnsupportedReturnType,
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
            InvalidExpressionType::Int,
            InvalidExpressionType::String,
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
            InvalidExpressionType::String,
            InvalidExpressionType::Int,
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
            InvalidExpressionType::Bool,
            InvalidExpressionType::Int,
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
            InvalidExpressionType::Nil,
            InvalidExpressionType::Int,
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
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Function,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    fn assert_call_argument_type_mismatch(
        src: &str,
        value: gleam_core::ast::TypedExpr,
        expected: InvalidExpressionType,
        actual: InvalidExpressionType,
    ) {
        let mut module = compile(src);
        let (_, _, arguments) =
            expect_call_statement_mut(&mut module.definitions.functions[1].body[0]);
        arguments[0].value = value;

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType { expected, actual },
            }),
        );
    }

    #[test]
    fn function_returning_function_call_expr_preserves_return_family() {
        let cases = [
            (
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            ),
            (
                FunctionFunctionId::String(StringFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::String], ValueType::String),
            ),
            (
                FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::Float], ValueType::Float),
            ),
            (
                FunctionFunctionId::Bool(BoolFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
            ),
            (
                FunctionFunctionId::Nil(NilFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
            ),
            (
                FunctionFunctionId::Tuple(TupleFunctionFunctionId(0)),
                FunctionType::new(
                    vec![ValueType::Tuple(vec![ValueType::Int])],
                    ValueType::Tuple(vec![ValueType::Int]),
                ),
            ),
            (
                FunctionFunctionId::List(ListFunctionFunctionId::from_item_type(
                    0,
                    FunctionType::new(
                        vec![ValueType::List(Box::new(ValueType::Int))],
                        ValueType::List(Box::new(ValueType::Int)),
                    ),
                    ValueType::Int,
                )),
                FunctionType::new(
                    vec![ValueType::List(Box::new(ValueType::Int))],
                    ValueType::List(Box::new(ValueType::Int)),
                ),
            ),
            (
                FunctionFunctionId::Function(FunctionFunctionFunctionId(0)),
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ),
            ),
        ];

        for (function, returned_function_type) in cases {
            assert_eq!(
                function_returning_function_call_expr(
                    function,
                    Vec::new(),
                    returned_function_type.clone(),
                )
                .value_type(),
                ValueType::Function(Box::new(returned_function_type)),
            );
        }
    }
}
