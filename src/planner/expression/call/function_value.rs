use super::super::conversion::expect_expression;
use super::CaptureSubstitution;
use crate::plan::{CallArg, Expr, FunctionExpr, FunctionFunctionExpr, ValueShape};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCallShapeReason, InvalidTypedAstReason, PlanError};
use gleam_core::ast::{CallArg as GleamCallArg, SrcSpan, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan_function_value_call(
    location: SrcSpan,
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

    let function: FunctionExpr = expect_expression(super::super::plan_expr(fun, context)?)?;
    let function_shape = function.shape().clone();
    let function_type = function_shape.type_();
    let return_shape = context.value_shape(type_.as_ref());
    let return_shape = function_shape
        .return_shape()
        .refine(&return_shape)
        .ok_or_else(function_call_return_type_mismatch)?;
    if arguments.len() != function_type.argument_types().len() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::FunctionCallArityMismatch,
            },
        });
    }

    let args = super::argument::plan_function_call_args(
        arguments,
        function_shape.argument_shapes(),
        context,
        capture,
    )?;

    function_call_expr_at(
        function,
        args,
        return_shape,
        context.host_call_site(location),
    )
}

#[cfg(test)]
fn function_call_expr(
    function: FunctionExpr,
    args: Vec<CallArg>,
    return_shape: ValueShape,
) -> Result<Expr, PlanError> {
    function_call_expr_at(
        function,
        args,
        return_shape,
        crate::plan::HostCallSite::unknown(),
    )
}

fn function_call_expr_at(
    function: FunctionExpr,
    args: Vec<CallArg>,
    return_shape: ValueShape,
    site: crate::plan::HostCallSite,
) -> Result<Expr, PlanError> {
    match return_shape {
        ValueShape::Parameter(_) => match function.into_generic() {
            Some(function) => Ok(Expr::generic(crate::plan::GenericExpr::function_call_at(
                function, args, site,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueShape::Int => match function.into_int() {
            Some(function) => Ok(Expr::int(crate::plan::IntExpr::function_call_at(
                function, args, site,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueShape::String => match function.into_string() {
            Some(function) => Ok(Expr::string(crate::plan::StringExpr::function_call_at(
                function, args, site,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueShape::BitArray => match function.into_bit_array() {
            Some(function) => Ok(Expr::bit_array(
                crate::plan::BitArrayExpr::function_call_at(function, args, site),
            )),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueShape::UtfCodepoint => match function.into_utf_codepoint() {
            Some(function) => Ok(Expr::utf_codepoint(
                crate::plan::UtfCodepointExpr::function_call_at(function, args, site),
            )),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueShape::Custom(_) => match function.into_custom() {
            Some(function) => crate::plan::CustomExpr::try_function_call_at(function, args, site)
                .map(Expr::custom)
                .map_err(|_| PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::FunctionCallArityMismatch,
                    },
                }),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueShape::External(_) => match function.into_external() {
            Some(function) => crate::plan::ExternalExpr::try_function_call_at(function, args, site)
                .map(Expr::external)
                .map_err(|_| PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::FunctionCallArityMismatch,
                    },
                }),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueShape::Float => match function.into_float() {
            Some(function) => Ok(Expr::float(crate::plan::FloatExpr::function_call_at(
                function, args, site,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueShape::Bool => match function.into_bool() {
            Some(function) => Ok(Expr::bool(crate::plan::BoolExpr::function_call_at(
                function, args, site,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueShape::Nil => match function.into_nil() {
            Some(function) => Ok(Expr::nil(crate::plan::NilExpr::function_call_at(
                function, args, site,
            ))),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueShape::Tuple(return_shape) => match function.into_tuple() {
            Some(function) => {
                let return_type = return_shape.iter().map(ValueShape::value_type).collect();
                Ok(Expr::tuple(
                    crate::plan::TupleExpr::function_call_at(function, args, return_type, site)
                        .with_shape(return_shape),
                ))
            }
            None => Err(function_call_return_type_mismatch()),
        },
        ValueShape::List(item_shape) => match function.into_list() {
            Some(function) => Ok(Expr::list(
                crate::plan::ListExpr::function_call_at(function, args, site)
                    .with_item_shape(*item_shape),
            )),
            None => Err(function_call_return_type_mismatch()),
        },
        ValueShape::Function(_) => match function.into_function() {
            Some(function) => function_returning_function_value_call_expr(function, args, site),
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
    site: crate::plan::HostCallSite,
) -> Result<Expr, PlanError> {
    let return_shape = function.function_function_type().return_shape().clone();
    let return_type = return_shape.type_();
    Ok(match return_shape.return_shape().clone() {
        ValueShape::Parameter(parameter) => Expr::function(FunctionExpr::generic_with_shape(
            crate::plan::GenericFunctionExpr::function_call_at(
                function,
                args,
                crate::plan::GenericFunctionType::new(
                    return_shape.argument_shapes().to_vec(),
                    parameter,
                ),
                site,
            ),
            return_shape,
        )),
        ValueShape::Int => Expr::function(FunctionExpr::int_with_shape(
            crate::plan::IntFunctionExpr::function_call_at(function, args, return_type, site),
            return_shape,
        )),
        ValueShape::String => Expr::function(FunctionExpr::string_with_shape(
            crate::plan::StringFunctionExpr::function_call_at(function, args, return_type, site),
            return_shape,
        )),
        ValueShape::BitArray => Expr::function(FunctionExpr::bit_array_with_shape(
            crate::plan::BitArrayFunctionExpr::function_call_at(function, args, return_type, site),
            return_shape,
        )),
        ValueShape::UtfCodepoint => Expr::function(FunctionExpr::utf_codepoint_with_shape(
            crate::plan::UtfCodepointFunctionExpr::function_call_at(
                function,
                args,
                return_type,
                site,
            ),
            return_shape,
        )),
        ValueShape::Custom(_) => {
            return crate::plan::CustomFunctionExpr::try_function_call_at(function, args, site)
                .map(FunctionExpr::custom)
                .map(Expr::function)
                .map_err(function_function_call_mismatch);
        }
        ValueShape::External(_) => {
            return crate::plan::ExternalFunctionExpr::try_function_call_at(function, args, site)
                .map(FunctionExpr::external)
                .map(Expr::function)
                .map_err(function_function_call_mismatch);
        }
        ValueShape::Float => Expr::function(FunctionExpr::float_with_shape(
            crate::plan::FloatFunctionExpr::function_call_at(function, args, return_type, site),
            return_shape,
        )),
        ValueShape::Bool => Expr::function(FunctionExpr::bool_with_shape(
            crate::plan::BoolFunctionExpr::function_call_at(function, args, return_type, site),
            return_shape,
        )),
        ValueShape::Nil => Expr::function(FunctionExpr::nil_with_shape(
            crate::plan::NilFunctionExpr::function_call_at(function, args, return_type, site),
            return_shape,
        )),
        ValueShape::Tuple(_) => Expr::function(FunctionExpr::tuple_with_shape(
            crate::plan::TupleFunctionExpr::function_call_at(function, args, return_type, site),
            return_shape,
        )),
        ValueShape::List(item_shape) => Expr::function(FunctionExpr::list_with_shape(
            crate::plan::ListFunctionExpr::function_call_at(
                function,
                args,
                return_type,
                item_shape.value_type(),
                site,
            ),
            return_shape,
        )),
        ValueShape::Function(_) => {
            return FunctionFunctionExpr::try_function_call_at(function, args, site)
                .map(FunctionExpr::function)
                .map(Expr::function)
                .map_err(function_function_call_mismatch);
        }
    })
}

fn function_function_call_mismatch(
    mismatch: crate::plan::FunctionFunctionCallMismatch,
) -> PlanError {
    let reason = match mismatch {
        crate::plan::FunctionFunctionCallMismatch::ArgumentCount { .. } => {
            InvalidCallShapeReason::FunctionCallArityMismatch
        }
        crate::plan::FunctionFunctionCallMismatch::ReturnFamily => {
            InvalidCallShapeReason::FunctionCallReturnTypeMismatch
        }
    };
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CallShape { reason },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        function_call_expr, function_call_return_type_mismatch, function_function_call_mismatch,
        function_returning_function_value_call_expr,
    };
    use crate::plan::{
        BitArrayFunctionFunctionId, BitArrayFunctionId, BoolFunctionFunctionId, BoolFunctionId,
        CustomConstructor, CustomConstructorField, CustomFunctionExpr, CustomFunctionFunctionId,
        CustomFunctionType, CustomType, CustomTypeName, Expr, ExternalFunctionExpr,
        ExternalFunctionType, ExternalTypeName, ExternalValueShape, FloatFunctionFunctionId,
        FloatFunctionId, FunctionExpr, FunctionFunctionExpr, FunctionFunctionFunctionId,
        FunctionFunctionId, FunctionFunctionType, FunctionShape, FunctionType, GenericExpr,
        GenericFunctionExpr, GenericFunctionType, IntFunctionFunctionId, IntLocalId, LocalId,
        NilFunctionFunctionId, NilFunctionId, PanicExpr, PanicSite, ParamLocal, RuntimeFunctionId,
        StringFunctionFunctionId, StringFunctionId, TupleFunctionFunctionId, TupleFunctionId,
        TupleLocalId, TypeParameterId, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
        ValueShape, ValueType,
    };
    use crate::planner::dsl::{
        block_int_function, bool_, bool_case_int_function, call_int_function_at, function,
        function_function_ref, function_ref, host_call_site, int, int_case_int_function,
        int_function_call_arg, int_function_closure, int_function_ref, let_int_function_step,
        local_int, local_int_function, local_tuple, module, module_with_anonymous, string, tuple,
        tuple_arg, tuple_function_ref,
    };
    use crate::planner::expression::call::support::expect_call_statement_mut;
    use crate::planner::expression::{typed_int_expr, typed_string_expr};
    use crate::planner::plan_module;
    use crate::planner::support::compile;
    use crate::planner::{
        InvalidCallShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedBitArraySegmentReason,
    };
    use gleam_core::type_;

    #[test]
    fn generic_function_value_calls_preserve_parameter_returns() {
        let parameter = TypeParameterId(0);
        let type_ = GenericFunctionType::new(vec![ValueShape::Int], parameter);
        let function = GenericFunctionExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            type_.clone(),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::generic(function.clone()),
                Vec::new(),
                ValueShape::Parameter(parameter),
            ),
            Ok(Expr::generic(GenericExpr::function_call(
                function,
                Vec::new(),
            ))),
        );

        let returned_shape =
            FunctionShape::new(vec![ValueShape::String], ValueShape::Parameter(parameter));
        let function_type = FunctionFunctionType::from_shapes(Vec::new(), returned_shape.clone());
        let provider = FunctionFunctionExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            function_type,
        );
        assert_eq!(
            function_returning_function_value_call_expr(
                provider.clone(),
                Vec::new(),
                crate::plan::HostCallSite::unknown(),
            ),
            Ok(Expr::function(FunctionExpr::generic_with_shape(
                GenericFunctionExpr::function_call(
                    provider,
                    Vec::new(),
                    GenericFunctionType::new(vec![ValueShape::String], parameter),
                ),
                returned_shape,
            ))),
        );
    }

    #[test]
    fn plan_unresolved_function_value_call_return_outside_current_template() {
        use crate::plan::{
            GenericExpr, GenericFunctionExpr, GenericFunctionLocal, GenericFunctionLocalId,
            GenericFunctionType, Step, TypeParameterId,
        };

        let source = r#"
fn fail() -> value {
  panic
}

pub fn main() {
  let function = fail
  let _ = function()
  1
}
"#;
        let plan =
            plan_module(compile(source)).expect("unresolved diverging function call should plan");

        assert_eq!(plan.main_function().steps().len(), 2);
        let function_type = GenericFunctionType::new(Vec::new(), TypeParameterId(0));
        assert_eq!(
            plan.main_function().steps()[1],
            Step::evaluate(Expr::generic(GenericExpr::function_call_at(
                GenericFunctionExpr::local_get(
                    GenericFunctionLocal::new(GenericFunctionLocalId(0), function_type),
                    "function".into(),
                ),
                Vec::new(),
                host_call_site(source, "main", "function()"),
            ))),
        );
    }

    #[test]
    fn plan_immediate_anonymous_function_call() {
        let source = r#"pub fn main() { fn(x) { x + 1 }(41) }"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function_at(
                    int_function_closure(1, [LocalId::Int(IntLocalId(0))], []),
                    [int_function_call_arg(int(41))],
                    host_call_site(source, "main", "fn(x) { x + 1 }(41)"),
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
  function({
    <<1:native>>
    1
  })
}
"#,
            )),
            Err(PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::NativeEndianness,
            }),
        );
    }

    #[test]
    fn plan_function_value_assignment_before_call() {
        let source = r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let function = add_one
  function(1)
}
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                call_int_function_at(
                    local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(int(1))],
                    host_call_site(source, "main", "function(1)"),
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
        let source = r#"
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
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let add_one =
            function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value");
        let expected = module(
            "main",
            function(
                "main",
                call_int_function_at(
                    local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(int(1))],
                    host_call_site(source, "main", "function(1)"),
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
        let source = r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  { add_one }(1)
}
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                call_int_function_at(
                    block_int_function([], int_function_ref(1, [LocalId::Int(IntLocalId(0))])),
                    [int_function_call_arg(int(1))],
                    host_call_site(source, "main", "{ add_one }(1)"),
                ),
            ),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_valued_case_callee() {
        let source = r#"
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
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
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
                call_int_function_at(
                    bool_case_int_function(
                        bool_(true),
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                        int_function_ref(2, [LocalId::Int(IntLocalId(0))]),
                    ),
                    [int_function_call_arg(int(1))],
                    host_call_site(
                        source,
                        "main",
                        "case True {\n    True -> add_one\n    False -> add_ten\n  }(1)",
                    ),
                ),
            )
            .let_int(
                1,
                "int_result",
                call_int_function_at(
                    int_case_int_function(
                        int(0),
                        [(0, int_function_ref(2, [LocalId::Int(IntLocalId(0))]))],
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                    ),
                    [int_function_call_arg(int(1))],
                    host_call_site(
                        source,
                        "main",
                        "case 0 {\n    0 -> add_ten\n    _ -> add_one\n  }(1)",
                    ),
                ),
            ),
            [add_one, add_ten],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_value_call_tuple_argument() {
        let source = r#"
fn tuple_score(pair: #(Int, String)) {
  pair.0
}

pub fn main() {
  let function = tuple_score
  function(#(41, "ok"))
}
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let pair_type = vec![ValueType::Int, ValueType::String];
        let pair_param = ParamLocal::tuple(TupleLocalId(0), pair_type.clone());
        let expected = module(
            "main",
            function(
                "main",
                call_int_function_at(
                    local_int_function(0, "function", [ValueType::Tuple(pair_type.clone())]),
                    [tuple_arg(tuple([
                        Expr::from(int(41)),
                        Expr::from(string("ok")),
                    ]))],
                    host_call_site(source, "main", r#"function(#(41, "ok"))"#),
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
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
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
                reason: InvalidTypedAstReason::ExpressionValueTypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::String,
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
                reason: InvalidTypedAstReason::ExpressionValueTypeMismatch {
                    expected: ValueType::Function(Box::new(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                    actual: ValueType::Int,
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
            ValueType::String,
            ValueType::Int,
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
            ValueType::Bool,
            ValueType::Int,
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
            ValueType::Nil,
            ValueType::Int,
        );
    }

    #[test]
    fn function_call_expr_preserves_return_family() {
        let external_shape = ExternalValueShape::new(
            ExternalTypeName::new("geam".into(), "main".into(), "Token".into()),
            Vec::new(),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::external(ExternalFunctionExpr::panic(
                    PanicExpr::panic_at(None, PanicSite::unknown()),
                    ExternalFunctionType::from_shapes(Vec::new(), external_shape.clone()),
                )),
                Vec::new(),
                ValueShape::External(external_shape),
            )
            .expect("external function call")
            .value_type(),
            ValueType::External(crate::plan::ExternalType::new(
                ExternalTypeName::new("geam".into(), "main".into(), "Token".into()),
                Vec::new(),
            )),
        );

        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueShape::String,
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
                ValueShape::BitArray,
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
                ValueShape::UtfCodepoint,
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
                ValueShape::Float,
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
                ValueShape::Bool,
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
                ValueShape::Nil,
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
                ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
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
                ValueShape::List(Box::new(ValueShape::Int)),
            )
            .expect("list function call")
            .value_type(),
            ValueType::List(Box::new(ValueType::Int)),
        );

        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_function_ref(
                    FunctionFunctionId::Function(FunctionFunctionFunctionId::new(
                        0,
                        crate::plan::FunctionFunctionType::new(
                            Vec::new(),
                            returned_function_type.clone(),
                        ),
                    )),
                    Vec::<ParamLocal>::new(),
                    returned_function_type.clone(),
                )),
                Vec::new(),
                ValueShape::Function(Box::new(crate::plan::FunctionShape::from_function_type(
                    returned_function_type.clone(),
                ))),
            )
            .expect("function-returning function call")
            .value_type(),
            ValueType::Function(Box::new(returned_function_type)),
        );
    }

    #[test]
    fn function_returning_function_value_call_expr_preserves_return_family() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
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
                FunctionFunctionId::Custom(CustomFunctionFunctionId::new(
                    0,
                    CustomFunctionType::new(vec![ValueType::Int], custom_type.clone()),
                )),
                FunctionType::new(vec![ValueType::Int], ValueType::Custom(custom_type.clone())),
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
                FunctionFunctionId::Function(FunctionFunctionFunctionId::new(
                    0,
                    crate::plan::FunctionFunctionType::new(
                        Vec::new(),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    ),
                )),
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
                    crate::plan::HostCallSite::unknown(),
                )
                .expect("function-returning function call")
                .value_type(),
                ValueType::Function(Box::new(returned_function_type)),
            );
        }
    }

    #[test]
    fn reject_margin_function_call_expr_return_family_mismatch() {
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueShape::Parameter(crate::plan::TypeParameterId(0)),
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueShape::External(ExternalValueShape::new(
                    ExternalTypeName::new("geam".into(), "main".into(), "Token".into()),
                    Vec::new(),
                )),
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(function_ref(
                    RuntimeFunctionId::String(StringFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                )),
                Vec::new(),
                ValueShape::Int,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueShape::String,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueShape::BitArray,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueShape::UtfCodepoint,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueShape::from_value_type(ValueType::Custom(CustomType::new(
                    CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
                    Vec::new(),
                ))),
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueShape::Float,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueShape::Bool,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueShape::Nil,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
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
                ValueShape::Int,
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueShape::Function(Box::new(crate::plan::FunctionShape::new(
                    Vec::new(),
                    ValueShape::Int,
                ))),
            ),
            Err(function_call_return_type_mismatch()),
        );
        assert_eq!(
            function_call_expr(
                FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
                Vec::new(),
                ValueShape::List(Box::new(ValueShape::Int)),
            ),
            Err(function_call_return_type_mismatch()),
        );
    }

    #[test]
    fn reject_margin_custom_function_call_argument_count() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let function =
            FunctionExpr::custom(CustomFunctionExpr::constructor(CustomConstructor::new(
                custom_type.clone(),
                "Boxed".into(),
                0,
                vec![CustomConstructorField::new(None, ValueType::Int)],
            )));

        assert_eq!(
            function_call_expr(
                function,
                Vec::new(),
                ValueShape::from_value_type(ValueType::Custom(custom_type)),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArityMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_external_function_call_argument_count() {
        let external_shape = ExternalValueShape::new(
            ExternalTypeName::new("geam".into(), "main".into(), "Token".into()),
            Vec::new(),
        );
        let function = FunctionExpr::external(ExternalFunctionExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
            ExternalFunctionType::from_shapes(vec![ValueShape::Int], external_shape.clone()),
        ));

        assert_eq!(
            function_call_expr(function, Vec::new(), ValueShape::External(external_shape),),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArityMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_function_returning_function_call_shape() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let returned_custom_function =
            FunctionType::new(Vec::new(), ValueType::Custom(custom_type.clone()));
        let function = FunctionFunctionExpr::from(function_function_ref(
            FunctionFunctionId::Custom(CustomFunctionFunctionId::new(
                0,
                CustomFunctionType::new(Vec::new(), custom_type.clone()),
            )),
            vec![ParamLocal::int(IntLocalId(0))],
            returned_custom_function.clone(),
        ));

        assert_eq!(
            function_returning_function_value_call_expr(
                function,
                Vec::new(),
                crate::plan::HostCallSite::unknown(),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArityMismatch,
                },
            }),
        );
        assert_eq!(
            function_function_call_mismatch(
                crate::plan::FunctionFunctionCallMismatch::ReturnFamily,
            ),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallReturnTypeMismatch,
                },
            },
        );
    }

    fn assert_function_value_argument_type_mismatch(
        src: &str,
        value: gleam_core::ast::TypedExpr,
        expected: ValueType,
        actual: ValueType,
    ) {
        let mut module = compile(src);
        let (_, _, arguments) =
            expect_call_statement_mut(&mut module.definitions.functions[1].body[1]);
        arguments[0].value = value;

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionValueTypeMismatch { expected, actual },
            }),
        );
    }
}
