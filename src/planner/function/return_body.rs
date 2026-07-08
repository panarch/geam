mod function_value;
mod primitive;

use crate::plan::{Expr, ExprKind, ReturnExpr, RuntimeFunctionId, ValueType};
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
        (ValueType::List(expected), RuntimeFunctionId::List(id), ExprKind::List(actual))
            if expected.as_ref() == &actual.element_type()
                && expected.as_ref() == &id.item_type() =>
        {
            Ok(ReturnExpr::list_body(
                id.clone(),
                primitive::list_return(actual),
            ))
        }
        (
            ValueType::Function(expected),
            RuntimeFunctionId::Function { id, return_type },
            ExprKind::Function(actual),
        ) if expected.as_ref() == actual.type_() && expected.as_ref() == return_type => {
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
        Expr, FloatExpr, FunctionExpr, FunctionFunctionId, FunctionType, IntFunctionExpr,
        IntFunctionFunctionId, IntFunctionId, IntFunctionValue, IntLocalId, ParamLocal,
        RuntimeFunctionId, StringFunctionFunctionId, ValueType,
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
                Expr::function(FunctionExpr::int(IntFunctionExpr::value(
                    IntFunctionValue::new(IntFunctionId(0), Vec::new()),
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
                Expr::function(FunctionExpr::int(IntFunctionExpr::value(
                    IntFunctionValue::new(IntFunctionId(0), vec![ParamLocal::int(IntLocalId(0))]),
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
                Expr::function(FunctionExpr::int(IntFunctionExpr::value(
                    IntFunctionValue::new(IntFunctionId(0), Vec::new()),
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
}
