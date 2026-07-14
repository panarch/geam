use super::CaptureSubstitution;
use crate::plan::{CallArg, Expr, FunctionExpr, FunctionFunctionExpr, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidCallShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
};
use gleam_core::ast::{CallArg as GleamCallArg, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan_function_value_call(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Expr, PlanError> {
    if arguments.iter().any(|argument| argument.label.is_some()) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LabelledArguments,
            },
        });
    }

    let function = {
        let expression = super::super::plan_expr(fun, context)?;
        let actual = super::super::expression_type(&expression);
        match expression.into_function() {
            Some(function) => function,
            None => {
                return Err(super::super::invalid_expression_type(
                    InvalidExpressionType::Function,
                    actual,
                ));
            }
        }
    };
    let function_type = function.type_().clone();
    let return_type = ValueType::from_gleam(type_.as_ref()).ok_or(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CallShape {
            reason: InvalidCallShapeReason::FunctionCallUnsupportedReturnType,
        },
    })?;
    if &return_type != function_type.return_() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
            },
        });
    }
    if arguments.len() != function_type.argument_types().len() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::FunctionCallArityMismatch,
            },
        });
    }

    let args = super::argument::plan_function_call_args(
        arguments,
        function_type.argument_types(),
        context,
        capture,
    )?;

    function_call_expr(function, args, return_type)
}

fn function_call_expr(
    function: FunctionExpr,
    args: Vec<CallArg>,
    return_type: ValueType,
) -> Result<Expr, PlanError> {
    match return_type {
        ValueType::Int => match function.into_int() {
            Some(function) => Ok(Expr::int(crate::plan::IntExpr::function_call(
                function, args,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueType::String => match function.into_string() {
            Some(function) => Ok(Expr::string(crate::plan::StringExpr::function_call(
                function, args,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueType::BitArray => match function.into_bit_array() {
            Some(function) => Ok(Expr::bit_array(crate::plan::BitArrayExpr::function_call(
                function, args,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueType::UtfCodepoint => match function.into_utf_codepoint() {
            Some(function) => Ok(Expr::utf_codepoint(
                crate::plan::UtfCodepointExpr::function_call(function, args),
            )),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueType::Custom(return_type) => match function.into_custom() {
            Some(function) => Ok(Expr::custom(crate::plan::CustomExpr::function_call(
                function,
                args,
                return_type,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueType::Float => match function.into_float() {
            Some(function) => Ok(Expr::float(crate::plan::FloatExpr::function_call(
                function, args,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueType::Bool => match function.into_bool() {
            Some(function) => Ok(Expr::bool(crate::plan::BoolExpr::function_call(
                function, args,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueType::Nil => match function.into_nil() {
            Some(function) => Ok(Expr::nil(crate::plan::NilExpr::function_call(
                function, args,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueType::Tuple(return_type) => match function.into_tuple() {
            Some(function) => Ok(Expr::tuple(crate::plan::TupleExpr::function_call(
                function,
                args,
                return_type,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueType::List(_) => match function.into_list() {
            Some(function) => Ok(Expr::list(crate::plan::ListExpr::function_call(
                function, args,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueType::Function(return_type) => match function.into_function() {
            Some(function) => Ok(function_returning_function_value_call_expr(
                function,
                args,
                *return_type,
            )),
            None => Err(function_call_return_type_mismatch()),
        },
    }
}

fn function_call_return_type_mismatch() -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CallShape {
            reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
        },
    }
}

fn function_returning_function_value_call_expr(
    function: FunctionFunctionExpr,
    args: Vec<CallArg>,
    return_type: crate::plan::FunctionType,
) -> Expr {
    match return_type.return_().clone() {
        ValueType::Int => Expr::function(FunctionExpr::int(
            crate::plan::IntFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::String => Expr::function(FunctionExpr::string(
            crate::plan::StringFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::BitArray => Expr::function(FunctionExpr::bit_array(
            crate::plan::BitArrayFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::UtfCodepoint => Expr::function(FunctionExpr::utf_codepoint(
            crate::plan::UtfCodepointFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::Custom(_) => Expr::function(FunctionExpr::custom(
            crate::plan::CustomFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::Float => Expr::function(FunctionExpr::float(
            crate::plan::FloatFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::Bool => Expr::function(FunctionExpr::bool(
            crate::plan::BoolFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::Nil => Expr::function(FunctionExpr::nil(
            crate::plan::NilFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::Tuple(_) => Expr::function(FunctionExpr::tuple(
            crate::plan::TupleFunctionExpr::function_call(function, args, return_type),
        )),
        ValueType::List(item_type) => Expr::function(FunctionExpr::list(
            crate::plan::ListFunctionExpr::function_call(function, args, return_type, *item_type),
        )),
        ValueType::Function(_) => Expr::function(FunctionExpr::function(
            FunctionFunctionExpr::function_call(function, args, return_type),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        function_call_expr, function_call_return_type_mismatch,
        function_returning_function_value_call_expr,
    };
    use crate::plan::{
        BitArrayFunctionFunctionId, BitArrayFunctionId, BoolFunctionFunctionId, BoolFunctionId,
        CustomType, CustomTypeName, Expr, FloatFunctionFunctionId, FloatFunctionId, FunctionExpr,
        FunctionFunctionExpr, FunctionFunctionFunctionId, FunctionFunctionId, FunctionType,
        IntFunctionFunctionId, IntLocalId, LocalId, NilFunctionFunctionId, NilFunctionId,
        ParamLocal, RuntimeFunctionId, StringFunctionFunctionId, StringFunctionId,
        TupleFunctionFunctionId, TupleFunctionId, TupleLocalId, UtfCodepointFunctionFunctionId,
        UtfCodepointFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        block_int_function, bool_, bool_case_int_function, call_int_function, function,
        function_function_ref, function_ref, int, int_case_int_function, int_function_call_arg,
        int_function_ref, let_int_function_step, local_int, local_int_function, local_tuple,
        module, module_with_anonymous, string, tuple, tuple_arg, tuple_function_ref,
    };
    use crate::planner::expression::call::support::expect_call_statement_mut;
    use crate::planner::expression::{typed_int_expr, typed_string_expr};
    use crate::planner::plan_module;
    use crate::planner::support::compile;
    use crate::planner::{
        InvalidCallShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedExpressionKind,
    };
    use gleam_core::type_;

    #[test]
    fn plan_immediate_anonymous_function_call() {
        let actual = plan_module(compile(r#"pub fn main() { fn(x) { x + 1 }(41) }"#))
            .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function(
                    int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(0, int(41))],
                ),
            ),
            [],
            [function("<anonymous:0>", local_int(0, "x").add_int(int(1))).param_int(0, "x")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_function_call_argument_expression() {
        assert_eq!(
            plan_module(compile(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(echo 1)
}
"#,
            )),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            }),
        );
    }

    #[test]
    fn plan_function_value_assignment_before_call() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                call_int_function(
                    local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(0, int(1))],
                ),
            )
            .step(let_int_function_step(
                0,
                "function",
                int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
            )),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_value_and_primitive_shadowing_bindings() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = 1
  let function = add_one
  function(1)
}

pub fn primitive_shadow() {
  let function = add_one
  let function = 1
  function + 1
}
"#,
        ))
        .expect("source should plan");
        let add_one =
            function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value");
        let expected = module(
            "main",
            function(
                "main",
                call_int_function(
                    local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(0, int(1))],
                ),
            )
            .let_int(0, "function", int(1))
            .step(let_int_function_step(
                0,
                "function",
                int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
            )),
            [
                add_one,
                function("primitive_shadow", local_int(0, "function").add_int(int(1)))
                    .step(let_int_function_step(
                        0,
                        "function",
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                    ))
                    .let_int(0, "function", int(1)),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_valued_block_callee() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  { add_one }(1)
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                call_int_function(
                    block_int_function([], int_function_ref(1, [LocalId::Int(IntLocalId(0))])),
                    [int_function_call_arg(0, int(1))],
                ),
            ),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_valued_case_callee() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn add_ten(value: Int) {
  value + 10
}

pub fn main() {
  let bool_result = case True {
    True -> add_one
    False -> add_ten
  }(1)
  let int_result = case 0 {
    0 -> add_ten
    _ -> add_one
  }(1)
  bool_result + int_result
}
"#,
        ))
        .expect("source should plan");
        let add_one =
            function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value");
        let add_ten =
            function("add_ten", local_int(0, "value").add_int(int(10))).param_int(0, "value");
        let expected = module(
            "main",
            function(
                "main",
                local_int(0, "bool_result").add_int(local_int(1, "int_result")),
            )
            .let_int(
                0,
                "bool_result",
                call_int_function(
                    bool_case_int_function(
                        bool_(true),
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                        int_function_ref(2, [LocalId::Int(IntLocalId(0))]),
                    ),
                    [int_function_call_arg(0, int(1))],
                ),
            )
            .let_int(
                1,
                "int_result",
                call_int_function(
                    int_case_int_function(
                        int(0),
                        [(0, int_function_ref(2, [LocalId::Int(IntLocalId(0))]))],
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                    ),
                    [int_function_call_arg(0, int(1))],
                ),
            ),
            [add_one, add_ten],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_value_call_tuple_argument() {
        let actual = plan_module(compile(
            r#"
fn tuple_score(pair: #(Int, String)) {
  pair.0
}

pub fn main() {
  let function = tuple_score
  function(#(41, "ok"))
}
"#,
        ))
        .expect("source should plan");
        let pair_type = vec![ValueType::Int, ValueType::String];
        let pair_param = ParamLocal::tuple(TupleLocalId(0), pair_type.clone());
        let expected = module(
            "main",
            function(
                "main",
                call_int_function(
                    local_int_function(0, "function", [ValueType::Tuple(pair_type.clone())]),
                    [tuple_arg(
                        0,
                        tuple([Expr::from(int(41)), Expr::from(string("ok"))]),
                    )],
                ),
            )
            .step(let_int_function_step(
                0,
                "function",
                int_function_ref(1, [pair_param.clone()]),
            )),
            [function(
                "tuple_score",
                local_tuple(0, "pair", pair_type.clone()).index_int(0),
            )
            .param_tuple(0, "pair", pair_type)],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_function_value_call_shapes() {
        let mut arity_mismatch_case_call = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case True {
    True -> add_one
    False -> add_one
  }(1)
}
"#,
        );
        let (_, _, arguments) = expect_call_statement_mut(
            &mut arity_mismatch_case_call.definitions.functions[1].body[0],
        );
        let mut extra_argument = arguments[0].clone();
        extra_argument.value = typed_int_expr(2);
        arguments.push(extra_argument);
        assert_eq!(arguments.len(), 2);
        assert_eq!(
            plan_module(arity_mismatch_case_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArityMismatch,
                },
            }),
        );

        let mut non_function_callee = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        );
        let (_, fun, _) =
            expect_call_statement_mut(&mut non_function_callee.definitions.functions[1].body[1]);
        *fun = typed_int_expr(1);
        assert_eq!(
            plan_module(non_function_callee),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Function,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );

        let mut custom_return_type_mismatch_call = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        );
        let (type_, _, _) = expect_call_statement_mut(
            &mut custom_return_type_mismatch_call.definitions.functions[1].body[1],
        );
        *type_ = type_::result(type_::int(), type_::nil());
        assert_eq!(
            plan_module(custom_return_type_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            }),
        );

        let mut unsupported_return_type_call = compile(
            r#"
fn add_one(value: Int) { value + 1 }
pub fn main() {
  let function = add_one
  function(1)
}
"#,
        );
        let (type_, _, _) = expect_call_statement_mut(
            &mut unsupported_return_type_call.definitions.functions[1].body[1],
        );
        *type_ = type_::generic_var(0);
        assert_eq!(
            plan_module(unsupported_return_type_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallUnsupportedReturnType,
                },
            }),
        );

        let mut return_type_mismatch_call = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        );
        let (type_, _, _) = expect_call_statement_mut(
            &mut return_type_mismatch_call.definitions.functions[1].body[1],
        );
        *type_ = type_::bool();
        assert_eq!(
            plan_module(return_type_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            }),
        );

        let mut argument_type_mismatch_call = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        );
        let (_, _, arguments) = expect_call_statement_mut(
            &mut argument_type_mismatch_call.definitions.functions[1].body[1],
        );
        arguments[0].value = typed_string_expr("wrong");
        assert_eq!(
            plan_module(argument_type_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::String,
                },
            }),
        );

        let mut arity_mismatch_call = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut arity_mismatch_call.definitions.functions[1].body[1]);
        arguments.clear();
        assert_eq!(
            plan_module(arity_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArityMismatch,
                },
            }),
        );

        let mut labelled_argument_call = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut labelled_argument_call.definitions.functions[1].body[1]);
        arguments[0].label = Some("value".into());
        assert_eq!(
            plan_module(labelled_argument_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LabelledArguments,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_function_value_call_argument_type_shapes() {
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
  let apply_value = apply
  apply_value(add_one)
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
            expect_call_statement_mut(&mut function_mismatch_call.definitions.functions[4].body[2]);
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
  let apply_value = apply
  apply_value(add_one)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut non_function_call.definitions.functions[2].body[1]);
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

        assert_function_value_argument_type_mismatch(
            r#"
fn identity(value: String) {
  value
}

pub fn main() {
  let function = identity
  function("ok")
}
"#,
            typed_int_expr(1),
            InvalidExpressionType::String,
            InvalidExpressionType::Int,
        );
        assert_function_value_argument_type_mismatch(
            r#"
fn identity(value: Bool) {
  value
}

pub fn main() {
  let function = identity
  function(True)
}
"#,
            typed_int_expr(1),
            InvalidExpressionType::Bool,
            InvalidExpressionType::Int,
        );
        assert_function_value_argument_type_mismatch(
            r#"
fn identity(value: Nil) {
  value
}

pub fn main() {
  let function = identity
  function(Nil)
}
"#,
            typed_int_expr(1),
            InvalidExpressionType::Nil,
            InvalidExpressionType::Int,
        );
    }

    #[test]
    fn function_call_expr_preserves_return_family() {
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::String,
            )
            .expect("string function call")
            .value_type(),
            ValueType::String,
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::BitArray(BitArrayFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::BitArray,
            )
            .expect("bit array function call")
            .value_type(),
            ValueType::BitArray,
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::UtfCodepoint(UtfCodepointFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::UtfCodepoint,
            )
            .expect("utf codepoint function call")
            .value_type(),
            ValueType::UtfCodepoint,
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Float(FloatFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::Float,
            )
            .expect("float function call")
            .value_type(),
            ValueType::Float,
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Bool(BoolFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::Bool,
            )
            .expect("bool function call")
            .value_type(),
            ValueType::Bool,
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Nil(NilFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::Nil,
            )
            .expect("nil function call")
            .value_type(),
            ValueType::Nil,
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::Tuple {
                        id: TupleFunctionId(0),
                        return_type: vec![ValueType::Int],
                    },
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::Tuple(vec![ValueType::Int]),
            )
            .expect("tuple function call")
            .value_type(),
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::List(crate::plan::ListFunctionId::from_item_type(
                        0,
                        crate::plan::ValueType::Int
                    )),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::List(Box::new(ValueType::Int)),
            )
            .expect("list function call")
            .value_type(),
            ValueType::List(Box::new(ValueType::Int)),
        );

        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_function_ref(
                    FunctionFunctionId::Function(FunctionFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type.clone(),
                )),
                Vec::new(),
                ValueType::Function(Box::new(returned_function_type.clone())),
            )
            .expect("function-returning function call")
            .value_type(),
            ValueType::Function(Box::new(returned_function_type)),
        );
    }

    #[test]
    fn function_returning_function_value_call_expr_preserves_return_family() {
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
                FunctionFunctionId::BitArray(BitArrayFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray),
            ),
            (
                FunctionFunctionId::UtfCodepoint(UtfCodepointFunctionFunctionId(0)),
                FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint),
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
                FunctionFunctionId::List(crate::plan::ListFunctionFunctionId::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int)),
                    ),
                    crate::plan::ValueType::Int,
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

        for (runtime_id, returned_function_type) in cases {
            let function = FunctionFunctionExpr::from(function_function_ref(
                runtime_id,
                Vec::<ParamLocal>::new(),
                returned_function_type.clone(),
            ));

            assert_eq!(
                function_returning_function_value_call_expr(
                    function,
                    Vec::new(),
                    returned_function_type.clone(),
                )
                .value_type(),
                ValueType::Function(Box::new(returned_function_type)),
            );
        }
    }

    #[test]
    fn reject_margin_function_call_expr_return_family_mismatch() {
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueType::Int,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::String,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::BitArray,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::UtfCodepoint,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::Custom(CustomType::new(
                    CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                    Vec::new(),
                )),
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::Float,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::Bool,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::Nil,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::Tuple(vec![ValueType::Int]),
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(tuple_function_ref(
                    0,
                    Vec::<ParamLocal>::new(),
                    [ValueType::Int],
                )),
                Vec::new(),
                ValueType::Int,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueType::List(Box::new(ValueType::Int)),
            ),
            Err(function_call_return_type_mismatch()),
        );
    }

    fn assert_function_value_argument_type_mismatch(
        src: &str,
        value: gleam_core::ast::TypedExpr,
        expected: InvalidExpressionType,
        actual: InvalidExpressionType,
    ) {
        let mut module = compile(src);
        let (_, _, arguments) =
            expect_call_statement_mut(&mut module.definitions.functions[1].body[1]);
        arguments[0].value = value;

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType { expected, actual },
            }),
        );
    }
}
