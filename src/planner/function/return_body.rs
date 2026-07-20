mod function_value;
mod primitive;

use crate::plan::{Expr, ExprKind, ListExpr, ReturnExpr, ValueShape};
use crate::planner::error::{InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError};
use ecow::EcoString;

pub(super) fn function_return_expr(
    name: &EcoString,
    expected: &ValueShape,
    actual: Expr,
) -> Result<ReturnExpr, PlanError> {
    let compatible =
        actual.shape().can_flow_to(expected) && actual.value_type() == expected.value_type();
    let return_ = match (actual.into_kind(), expected) {
        (ExprKind::Generic(actual), _) if compatible => Ok(ReturnExpr::generic_body(
            actual.parameter(),
            primitive::generic_return(actual),
        )),
        (ExprKind::Int(actual), _) if compatible => {
            Ok(ReturnExpr::int_body(primitive::int_return(actual)))
        }
        (ExprKind::String(actual), _) if compatible => {
            Ok(ReturnExpr::string_body(primitive::string_return(actual)))
        }
        (ExprKind::BitArray(actual), _) if compatible => Ok(ReturnExpr::bit_array_body(
            primitive::bit_array_return(actual),
        )),
        (ExprKind::UtfCodepoint(actual), _) if compatible => Ok(ReturnExpr::utf_codepoint_body(
            primitive::utf_codepoint_return(actual),
        )),
        (ExprKind::Custom(actual), ValueShape::Custom(signature_shape)) if compatible => Ok(
            ReturnExpr::custom_body(primitive::custom_return(signature_shape.clone(), actual)),
        ),
        (ExprKind::Float(actual), _) if compatible => {
            Ok(ReturnExpr::float_body(primitive::float_return(actual)))
        }
        (ExprKind::Bool(actual), _) if compatible => {
            Ok(ReturnExpr::bool_body(primitive::bool_return(actual)))
        }
        (ExprKind::Nil(actual), _) if compatible => {
            Ok(ReturnExpr::nil_body(primitive::nil_return(actual)))
        }
        (ExprKind::Tuple(actual), _) if compatible => {
            let type_ = actual.type_().to_vec();
            Ok(ReturnExpr::tuple_body(
                type_,
                primitive::tuple_return(actual),
            ))
        }
        (ExprKind::List(actual), _) if compatible => Ok(list_return_expr(actual)),
        (ExprKind::Function(actual), _) if compatible => {
            Ok(function_value::function_returning_function_expr(actual))
        }
        _ => Err(()),
    };

    return_.map_err(|()| PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::FunctionShape {
            name: name.clone(),
            reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
        },
    })
}

fn list_return_expr(actual: ListExpr) -> ReturnExpr {
    match actual {
        ListExpr::Generic(actual) => {
            let parameter = actual.item().parameter();
            ReturnExpr::generic_list_body(parameter, primitive::typed_list_return_body(actual))
        }
        ListExpr::ParameterList(actual) => {
            let parameter = actual.item().parameter();
            ReturnExpr::parameter_list_list_body(
                parameter,
                primitive::typed_list_return_body(actual),
            )
        }
        ListExpr::Int(actual) => {
            ReturnExpr::int_list_body(primitive::typed_list_return_body(actual))
        }
        ListExpr::String(actual) => {
            ReturnExpr::string_list_body(primitive::typed_list_return_body(actual))
        }
        ListExpr::BitArray(actual) => {
            ReturnExpr::bit_array_list_body(primitive::typed_list_return_body(actual))
        }
        ListExpr::UtfCodepoint(actual) => {
            ReturnExpr::utf_codepoint_list_body(primitive::typed_list_return_body(actual))
        }
        ListExpr::Custom(actual) => {
            let item_type = actual.item().item_type();
            ReturnExpr::custom_list_body(item_type, primitive::typed_list_return_body(actual))
        }
        ListExpr::Float(actual) => {
            ReturnExpr::float_list_body(primitive::typed_list_return_body(actual))
        }
        ListExpr::Bool(actual) => {
            ReturnExpr::bool_list_body(primitive::typed_list_return_body(actual))
        }
        ListExpr::Nil(actual) => {
            ReturnExpr::nil_list_body(primitive::typed_list_return_body(actual))
        }
        ListExpr::Tuple(actual) => {
            let item_type = actual.item().item_type();
            ReturnExpr::tuple_list_body(item_type, primitive::typed_list_return_body(actual))
        }
        ListExpr::List(actual) => {
            let item_shape = actual.item().item_shape().clone();
            ReturnExpr::list_list_body(item_shape, primitive::typed_list_return_body(actual))
        }
        ListExpr::Function(actual) => {
            let item_type = actual.item().item_type();
            ReturnExpr::function_list_body(item_type, primitive::typed_list_return_body(actual))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::function_return_expr;
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FunctionExpr, FunctionReference, FunctionShape, FunctionType,
        GenericFunctionExpr, GenericFunctionReference, GenericFunctionType, IntExpr,
        IntFunctionExpr, IntFunctionReference, ListExpr, ReturnBody, ReturnExpr, StringExpr,
        TypeParameterId, ValueShape, ValueType, monomorphic_function_instantiation,
    };
    use crate::planner::{InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError};

    #[test]
    fn reject_margin_function_return_family_mismatch() {
        assert_eq!(
            function_return_expr(
                &"main".into(),
                &ValueShape::Float,
                Expr::int(crate::plan::IntExpr::value(1.into())),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );

        assert_eq!(
            function_return_expr(
                &"main".into(),
                &ValueShape::Function(Box::new(FunctionShape::new(Vec::new(), ValueShape::Int,))),
                Expr::function(FunctionExpr::reference(FunctionReference::new(
                    instantiation(FunctionType::new(Vec::new(), ValueType::String))
                ))),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_function_return_type_metadata_mismatch() {
        let expected = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            function_return_expr(
                &"main".into(),
                &ValueShape::Function(Box::new(FunctionShape::from_function_type(expected))),
                Expr::function(FunctionExpr::int(IntFunctionExpr::reference(
                    IntFunctionReference::new(instantiation(FunctionType::new(
                        vec![ValueType::Int],
                        ValueType::Int,
                    ))),
                ))),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );

        let expected = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        assert_eq!(
            function_return_expr(
                &"main".into(),
                &ValueShape::Function(Box::new(FunctionShape::from_function_type(expected))),
                Expr::function(FunctionExpr::int(IntFunctionExpr::reference(
                    IntFunctionReference::new(instantiation(FunctionType::new(
                        Vec::new(),
                        ValueType::Int
                    ))),
                ))),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn generic_function_returns_preserve_tail_cases_and_block() {
        let parameter = TypeParameterId(0);
        let type_ = GenericFunctionType::new(Vec::new(), parameter);
        let shape = type_.shape();
        let reference = GenericFunctionExpr::reference(
            GenericFunctionReference::new(monomorphic_function_instantiation(0, shape.clone())),
            type_.clone(),
        );
        let call_function = monomorphic_function_instantiation(1, shape.clone());
        let call = GenericFunctionExpr::call(call_function.clone(), Vec::new(), type_.clone());
        let int_case = GenericFunctionExpr::int_case(
            IntExpr::value(0.into()),
            vec![(0.into(), call)],
            reference.clone(),
        )
        .expect("matching generic function types");
        let string_case = GenericFunctionExpr::string_case(
            StringExpr::value("value".into()),
            vec![("value".into(), int_case)],
            reference.clone(),
        )
        .expect("matching generic function types");
        let float_case = GenericFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, string_case)],
            reference.clone(),
        )
        .expect("matching generic function types");
        let bool_case =
            GenericFunctionExpr::bool_case(BoolExpr::value(true), float_case, reference.clone())
                .expect("matching generic function types");

        assert_eq!(
            function_return_expr(
                &"choose".into(),
                &ValueShape::Function(Box::new(shape.clone())),
                Expr::function(FunctionExpr::generic(GenericFunctionExpr::block(
                    Vec::new(),
                    bool_case,
                ))),
            ),
            Ok(ReturnExpr::generic_function_shape_body(
                shape,
                ReturnBody::block(
                    Vec::new(),
                    ReturnBody::bool_case(
                        BoolExpr::value(true),
                        ReturnBody::float_case(
                            FloatExpr::value(1.0),
                            vec![(
                                1.0,
                                ReturnBody::string_case(
                                    StringExpr::value("value".into()),
                                    vec![(
                                        "value".into(),
                                        ReturnBody::int_case(
                                            IntExpr::value(0.into()),
                                            vec![(
                                                0.into(),
                                                ReturnBody::tail_call(call_function, Vec::new(),),
                                            )],
                                            ReturnBody::expr(reference.clone()),
                                        ),
                                    )],
                                    ReturnBody::expr(reference.clone()),
                                ),
                            )],
                            ReturnBody::expr(reference.clone()),
                        ),
                        ReturnBody::expr(reference),
                    ),
                ),
            )),
        );
    }

    #[test]
    fn plan_list_return_preserves_every_item_family() {
        let string = ListExpr::value(Vec::new(), ValueType::String);
        assert_eq!(
            function_return_expr(
                &"strings".into(),
                &ValueShape::List(Box::new(ValueShape::String)),
                Expr::list(string.clone()),
            ),
            Ok(ReturnExpr::string_list_body(ReturnBody::expr(
                string
                    .into_string()
                    .expect("expression should be List(String)"),
            ),)),
        );

        let bit_array = ListExpr::value(Vec::new(), ValueType::BitArray);
        assert_eq!(
            function_return_expr(
                &"bit_arrays".into(),
                &ValueShape::List(Box::new(ValueShape::BitArray)),
                Expr::list(bit_array.clone()),
            ),
            Ok(ReturnExpr::bit_array_list_body(ReturnBody::expr(
                bit_array
                    .into_bit_array()
                    .expect("expression should be List(BitArray)"),
            ),)),
        );

        let utf_codepoint = ListExpr::value(Vec::new(), ValueType::UtfCodepoint);
        assert_eq!(
            function_return_expr(
                &"utf_codepoints".into(),
                &ValueShape::List(Box::new(ValueShape::UtfCodepoint)),
                Expr::list(utf_codepoint.clone()),
            ),
            Ok(ReturnExpr::utf_codepoint_list_body(ReturnBody::expr(
                utf_codepoint
                    .into_utf_codepoint()
                    .expect("expression should be List(UtfCodepoint)"),
            ),)),
        );

        let float = ListExpr::value(Vec::new(), ValueType::Float);
        assert_eq!(
            function_return_expr(
                &"floats".into(),
                &ValueShape::List(Box::new(ValueShape::Float)),
                Expr::list(float.clone()),
            ),
            Ok(ReturnExpr::float_list_body(ReturnBody::expr(
                float
                    .into_float()
                    .expect("expression should be List(Float)")
            ),)),
        );

        let bool_ = ListExpr::value(Vec::new(), ValueType::Bool);
        assert_eq!(
            function_return_expr(
                &"bools".into(),
                &ValueShape::List(Box::new(ValueShape::Bool)),
                Expr::list(bool_.clone()),
            ),
            Ok(ReturnExpr::bool_list_body(ReturnBody::expr(
                bool_.into_bool().expect("expression should be List(Bool)")
            ),)),
        );

        let nil = ListExpr::value(Vec::new(), ValueType::Nil);
        assert_eq!(
            function_return_expr(
                &"nils".into(),
                &ValueShape::List(Box::new(ValueShape::Nil)),
                Expr::list(nil.clone()),
            ),
            Ok(ReturnExpr::nil_list_body(ReturnBody::expr(
                nil.into_nil().expect("expression should be List(Nil)")
            ),)),
        );

        let tuple_item = vec![ValueType::Int];
        let tuple = ListExpr::value(Vec::new(), ValueType::Tuple(tuple_item.clone()));
        assert_eq!(
            function_return_expr(
                &"tuples".into(),
                &ValueShape::List(Box::new(ValueShape::Tuple(
                    vec![ValueShape::Int].into_boxed_slice(),
                ))),
                Expr::list(tuple.clone()),
            ),
            Ok(ReturnExpr::tuple_list_body(
                tuple_item,
                ReturnBody::expr(
                    tuple
                        .into_tuple()
                        .expect("expression should be List(Tuple)"),
                ),
            )),
        );

        let list_item = Box::new(ValueType::Int);
        let list = ListExpr::value(Vec::new(), ValueType::List(list_item.clone()));
        assert_eq!(
            function_return_expr(
                &"lists".into(),
                &ValueShape::List(Box::new(ValueShape::List(Box::new(ValueShape::Int)))),
                Expr::list(list.clone()),
            ),
            Ok(ReturnExpr::list_list_body(
                crate::plan::ValueStorageShape::Int,
                ReturnBody::expr(list.into_list().expect("expression should be List(List)")),
            )),
        );

        let function_item = FunctionType::new(Vec::new(), ValueType::Int);
        let functions = ListExpr::value(
            Vec::new(),
            ValueType::Function(Box::new(function_item.clone())),
        );
        assert_eq!(
            function_return_expr(
                &"functions".into(),
                &ValueShape::List(Box::new(ValueShape::Function(Box::new(
                    FunctionShape::from_function_type(function_item.clone()),
                )))),
                Expr::list(functions.clone()),
            ),
            Ok(ReturnExpr::function_list_body(
                function_item,
                ReturnBody::expr(
                    functions
                        .into_function()
                        .expect("expression should be List(Function)"),
                ),
            )),
        );

        let int = ListExpr::value(Vec::new(), ValueType::Int);
        assert_eq!(
            function_return_expr(
                &"ints".into(),
                &ValueShape::List(Box::new(ValueShape::Int)),
                Expr::list(int.clone()),
            ),
            Ok(ReturnExpr::int_list_body(ReturnBody::expr(
                int.into_int().expect("expression should be List(Int)")
            ),)),
        );
    }

    fn instantiation(type_: FunctionType) -> crate::plan::FunctionInstantiation {
        monomorphic_function_instantiation(0, FunctionShape::from_function_type(type_))
    }
}
