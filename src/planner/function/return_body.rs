mod function_value;
mod primitive;

use crate::plan::{
    Expr, ExprKind, ListExpr, ListFunctionId, ReturnExpr, RuntimeFunctionId, ValueType,
};
use crate::planner::error::{InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError};
use ecow::EcoString;

pub(super) fn function_return_expr(
    name: &EcoString,
    expected: &ValueType,
    runtime_id: &RuntimeFunctionId,
    actual: Expr,
) -> Result<ReturnExpr, PlanError> {
    match (expected, runtime_id, actual.into_kind()) {
        (ValueType::Int, RuntimeFunctionId::Int(runtime_id), ExprKind::Int(actual)) => Ok(
            ReturnExpr::int_body(*runtime_id, primitive::int_return(actual)),
        ),
        (ValueType::String, RuntimeFunctionId::String(runtime_id), ExprKind::String(actual)) => Ok(
            ReturnExpr::string_body(*runtime_id, primitive::string_return(actual)),
        ),
        (
            ValueType::BitArray,
            RuntimeFunctionId::BitArray(runtime_id),
            ExprKind::BitArray(actual),
        ) => Ok(ReturnExpr::bit_array_body(
            *runtime_id,
            primitive::bit_array_return(actual),
        )),
        (
            ValueType::UtfCodepoint,
            RuntimeFunctionId::UtfCodepoint(runtime_id),
            ExprKind::UtfCodepoint(actual),
        ) => Ok(ReturnExpr::utf_codepoint_body(
            *runtime_id,
            primitive::utf_codepoint_return(actual),
        )),
        (
            ValueType::Custom(expected),
            RuntimeFunctionId::Custom(runtime_id),
            ExprKind::Custom(actual),
        ) if expected == actual.type_() && expected == runtime_id.return_type() => Ok(
            ReturnExpr::custom_body(runtime_id.index(), primitive::custom_return(actual)),
        ),
        (ValueType::Float, RuntimeFunctionId::Float(runtime_id), ExprKind::Float(actual)) => Ok(
            ReturnExpr::float_body(*runtime_id, primitive::float_return(actual)),
        ),
        (ValueType::Bool, RuntimeFunctionId::Bool(runtime_id), ExprKind::Bool(actual)) => Ok(
            ReturnExpr::bool_body(*runtime_id, primitive::bool_return(actual)),
        ),
        (ValueType::Nil, RuntimeFunctionId::Nil(runtime_id), ExprKind::Nil(actual)) => Ok(
            ReturnExpr::nil_body(*runtime_id, primitive::nil_return(actual)),
        ),
        (
            ValueType::Tuple(expected),
            RuntimeFunctionId::Tuple { id, return_type },
            ExprKind::Tuple(actual),
        ) if expected == actual.type_() && expected == return_type => Ok(ReturnExpr::tuple_body(
            *id,
            expected.clone(),
            primitive::tuple_return(actual),
        )),
        (
            ValueType::List(expected),
            RuntimeFunctionId::List(ListFunctionId::Int(runtime_id)),
            ExprKind::List(ListExpr::Int(actual)),
        ) if expected.as_ref() == &ValueType::Int => Ok(ReturnExpr::int_list_body(
            *runtime_id,
            primitive::typed_list_return_body(actual),
        )),
        (
            ValueType::List(expected),
            RuntimeFunctionId::List(ListFunctionId::String(runtime_id)),
            ExprKind::List(ListExpr::String(actual)),
        ) if expected.as_ref() == &ValueType::String => Ok(ReturnExpr::string_list_body(
            *runtime_id,
            primitive::typed_list_return_body(actual),
        )),
        (
            ValueType::List(expected),
            RuntimeFunctionId::List(ListFunctionId::BitArray(runtime_id)),
            ExprKind::List(ListExpr::BitArray(actual)),
        ) if expected.as_ref() == &ValueType::BitArray => Ok(ReturnExpr::bit_array_list_body(
            *runtime_id,
            primitive::typed_list_return_body(actual),
        )),
        (
            ValueType::List(expected),
            RuntimeFunctionId::List(ListFunctionId::UtfCodepoint(runtime_id)),
            ExprKind::List(ListExpr::UtfCodepoint(actual)),
        ) if expected.as_ref() == &ValueType::UtfCodepoint => {
            Ok(ReturnExpr::utf_codepoint_list_body(
                *runtime_id,
                primitive::typed_list_return_body(actual),
            ))
        }
        (
            ValueType::List(expected),
            RuntimeFunctionId::List(ListFunctionId::Custom {
                id: runtime_id,
                item_type,
            }),
            ExprKind::List(ListExpr::Custom(actual)),
        ) if expected.as_ref() == &ValueType::Custom(item_type.clone())
            && item_type == &actual.item().item_type() =>
        {
            Ok(ReturnExpr::custom_list_body(
                *runtime_id,
                item_type.clone(),
                primitive::typed_list_return_body(actual),
            ))
        }
        (
            ValueType::List(expected),
            RuntimeFunctionId::List(ListFunctionId::Float(runtime_id)),
            ExprKind::List(ListExpr::Float(actual)),
        ) if expected.as_ref() == &ValueType::Float => Ok(ReturnExpr::float_list_body(
            *runtime_id,
            primitive::typed_list_return_body(actual),
        )),
        (
            ValueType::List(expected),
            RuntimeFunctionId::List(ListFunctionId::Bool(runtime_id)),
            ExprKind::List(ListExpr::Bool(actual)),
        ) if expected.as_ref() == &ValueType::Bool => Ok(ReturnExpr::bool_list_body(
            *runtime_id,
            primitive::typed_list_return_body(actual),
        )),
        (
            ValueType::List(expected),
            RuntimeFunctionId::List(ListFunctionId::Nil(runtime_id)),
            ExprKind::List(ListExpr::Nil(actual)),
        ) if expected.as_ref() == &ValueType::Nil => Ok(ReturnExpr::nil_list_body(
            *runtime_id,
            primitive::typed_list_return_body(actual),
        )),
        (
            ValueType::List(expected),
            RuntimeFunctionId::List(ListFunctionId::Tuple {
                id: runtime_id,
                item_type,
            }),
            ExprKind::List(ListExpr::Tuple(actual)),
        ) if expected.as_ref() == &ValueType::Tuple(item_type.clone())
            && item_type == actual.item().item_type().as_slice() =>
        {
            Ok(ReturnExpr::tuple_list_body(
                *runtime_id,
                item_type.clone(),
                primitive::typed_list_return_body(actual),
            ))
        }
        (
            ValueType::List(expected),
            RuntimeFunctionId::List(ListFunctionId::List {
                id: runtime_id,
                item_type,
            }),
            ExprKind::List(ListExpr::List(actual)),
        ) if expected.as_ref() == &ValueType::List(item_type.clone())
            && item_type.as_ref() == actual.item().item_type().as_ref() =>
        {
            Ok(ReturnExpr::list_list_body(
                *runtime_id,
                item_type.clone(),
                primitive::typed_list_return_body(actual),
            ))
        }
        (
            ValueType::List(expected),
            RuntimeFunctionId::List(ListFunctionId::Function {
                id: runtime_id,
                item_type,
            }),
            ExprKind::List(ListExpr::Function(actual)),
        ) if expected.as_ref() == &ValueType::Function(Box::new(item_type.clone()))
            && item_type == &actual.item().item_type() =>
        {
            Ok(ReturnExpr::function_list_body(
                *runtime_id,
                item_type.clone(),
                primitive::typed_list_return_body(actual),
            ))
        }
        (
            ValueType::Function(expected),
            RuntimeFunctionId::Function { id, return_type },
            ExprKind::Function(actual),
        ) if *expected.as_ref() == actual.type_() && expected.as_ref() == return_type => {
            function_value::function_returning_function_expr(name, id.clone(), actual)
        }
        _ => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: name.clone(),
                reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::function_return_expr;
    use crate::plan::{
        BitArrayListFunctionId, BoolListFunctionId, Expr, FloatExpr, FloatListFunctionId,
        FunctionExpr, FunctionFunctionId, FunctionListFunctionId, FunctionType, IntFunctionExpr,
        IntFunctionFunctionId, IntFunctionId, IntFunctionReference, IntListFunctionId, IntLocalId,
        ListExpr, ListFunctionId, ListListFunctionId, NilListFunctionId, ParamLocal, ReturnBody,
        ReturnExpr, RuntimeFunctionId, StringFunctionFunctionId, StringListFunctionId,
        TupleListFunctionId, UtfCodepointListFunctionId, ValueType,
    };
    use crate::planner::{InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError};

    #[test]
    fn reject_margin_function_return_family_mismatch() {
        assert_eq!(
            function_return_expr(
                &"main".into(),
                &ValueType::Float,
                &RuntimeFunctionId::Int(IntFunctionId(0)),
                Expr::float(FloatExpr::value(1.0)),
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
                &ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
                &RuntimeFunctionId::Function {
                    id: FunctionFunctionId::String(StringFunctionFunctionId(0)),
                    return_type: FunctionType::new(Vec::new(), ValueType::Int),
                },
                Expr::function(FunctionExpr::int(IntFunctionExpr::reference(
                    IntFunctionReference::new(IntFunctionId(0), Vec::new()),
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
                &ValueType::Function(Box::new(expected.clone())),
                &RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: expected,
                },
                Expr::function(FunctionExpr::int(IntFunctionExpr::reference(
                    IntFunctionReference::new(
                        IntFunctionId(0),
                        vec![ParamLocal::int(IntLocalId(0))]
                    ),
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
                &ValueType::Function(Box::new(expected)),
                &RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                },
                Expr::function(FunctionExpr::int(IntFunctionExpr::reference(
                    IntFunctionReference::new(IntFunctionId(0), Vec::new()),
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
    fn plan_list_return_preserves_every_item_family() {
        let string = ListExpr::value(Vec::new(), ValueType::String);
        assert_eq!(
            function_return_expr(
                &"strings".into(),
                &ValueType::List(Box::new(ValueType::String)),
                &RuntimeFunctionId::List(ListFunctionId::String(StringListFunctionId(0))),
                Expr::list(string.clone()),
            ),
            Ok(ReturnExpr::string_list_body(
                StringListFunctionId(0),
                ReturnBody::expr(
                    string
                        .into_string()
                        .expect("expression should be List(String)"),
                ),
            )),
        );

        let bit_array = ListExpr::value(Vec::new(), ValueType::BitArray);
        assert_eq!(
            function_return_expr(
                &"bit_arrays".into(),
                &ValueType::List(Box::new(ValueType::BitArray)),
                &RuntimeFunctionId::List(ListFunctionId::BitArray(BitArrayListFunctionId(0))),
                Expr::list(bit_array.clone()),
            ),
            Ok(ReturnExpr::bit_array_list_body(
                BitArrayListFunctionId(0),
                ReturnBody::expr(
                    bit_array
                        .into_bit_array()
                        .expect("expression should be List(BitArray)"),
                ),
            )),
        );

        let utf_codepoint = ListExpr::value(Vec::new(), ValueType::UtfCodepoint);
        assert_eq!(
            function_return_expr(
                &"utf_codepoints".into(),
                &ValueType::List(Box::new(ValueType::UtfCodepoint)),
                &RuntimeFunctionId::List(ListFunctionId::UtfCodepoint(UtfCodepointListFunctionId(
                    0
                ),)),
                Expr::list(utf_codepoint.clone()),
            ),
            Ok(ReturnExpr::utf_codepoint_list_body(
                UtfCodepointListFunctionId(0),
                ReturnBody::expr(
                    utf_codepoint
                        .into_utf_codepoint()
                        .expect("expression should be List(UtfCodepoint)"),
                ),
            )),
        );

        let float = ListExpr::value(Vec::new(), ValueType::Float);
        assert_eq!(
            function_return_expr(
                &"floats".into(),
                &ValueType::List(Box::new(ValueType::Float)),
                &RuntimeFunctionId::List(ListFunctionId::Float(FloatListFunctionId(0))),
                Expr::list(float.clone()),
            ),
            Ok(ReturnExpr::float_list_body(
                FloatListFunctionId(0),
                ReturnBody::expr(
                    float
                        .into_float()
                        .expect("expression should be List(Float)")
                ),
            )),
        );

        let bool_ = ListExpr::value(Vec::new(), ValueType::Bool);
        assert_eq!(
            function_return_expr(
                &"bools".into(),
                &ValueType::List(Box::new(ValueType::Bool)),
                &RuntimeFunctionId::List(ListFunctionId::Bool(BoolListFunctionId(0))),
                Expr::list(bool_.clone()),
            ),
            Ok(ReturnExpr::bool_list_body(
                BoolListFunctionId(0),
                ReturnBody::expr(bool_.into_bool().expect("expression should be List(Bool)")),
            )),
        );

        let nil = ListExpr::value(Vec::new(), ValueType::Nil);
        assert_eq!(
            function_return_expr(
                &"nils".into(),
                &ValueType::List(Box::new(ValueType::Nil)),
                &RuntimeFunctionId::List(ListFunctionId::Nil(NilListFunctionId(0))),
                Expr::list(nil.clone()),
            ),
            Ok(ReturnExpr::nil_list_body(
                NilListFunctionId(0),
                ReturnBody::expr(nil.into_nil().expect("expression should be List(Nil)")),
            )),
        );

        let tuple_item = vec![ValueType::Int];
        let tuple = ListExpr::value(Vec::new(), ValueType::Tuple(tuple_item.clone()));
        assert_eq!(
            function_return_expr(
                &"tuples".into(),
                &ValueType::List(Box::new(ValueType::Tuple(tuple_item.clone()))),
                &RuntimeFunctionId::List(ListFunctionId::Tuple {
                    id: TupleListFunctionId(0),
                    item_type: tuple_item.clone(),
                }),
                Expr::list(tuple.clone()),
            ),
            Ok(ReturnExpr::tuple_list_body(
                TupleListFunctionId(0),
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
                &ValueType::List(Box::new(ValueType::List(list_item.clone()))),
                &RuntimeFunctionId::List(ListFunctionId::List {
                    id: ListListFunctionId(0),
                    item_type: list_item.clone(),
                }),
                Expr::list(list.clone()),
            ),
            Ok(ReturnExpr::list_list_body(
                ListListFunctionId(0),
                list_item,
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
                &ValueType::List(Box::new(ValueType::Function(Box::new(
                    function_item.clone(),
                )))),
                &RuntimeFunctionId::List(ListFunctionId::Function {
                    id: FunctionListFunctionId(0),
                    item_type: function_item.clone(),
                }),
                Expr::list(functions.clone()),
            ),
            Ok(ReturnExpr::function_list_body(
                FunctionListFunctionId(0),
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
                &ValueType::List(Box::new(ValueType::Int)),
                &RuntimeFunctionId::List(ListFunctionId::Int(IntListFunctionId(0))),
                Expr::list(int.clone()),
            ),
            Ok(ReturnExpr::int_list_body(
                IntListFunctionId(0),
                ReturnBody::expr(int.into_int().expect("expression should be List(Int)")),
            )),
        );
    }
}
