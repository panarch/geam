mod bool;
mod float;
mod int;
mod list;
mod nil;
mod returning_function;
mod string;
mod tuple;

use crate::plan::{ExecutionPlan, FunctionExpr, FunctionExprKind, FunctionValue};
use crate::runtime::ExecutionError;
use crate::runtime::frame::Frame;

pub(in crate::runtime) use self::{
    bool::eval_bool_function_expr, float::eval_float_function_expr, int::eval_int_function_expr,
    list::eval_list_function_expr, nil::eval_nil_function_expr,
    returning_function::eval_function_function_expr, string::eval_string_function_expr,
    tuple::eval_tuple_function_expr,
};

pub(in crate::runtime) fn eval_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FunctionExpr,
) -> Result<FunctionValue, ExecutionError> {
    match expression.kind() {
        FunctionExprKind::Int(expression) => {
            Ok(eval_int_function_expr(plan, frame, expression)?.into())
        }
        FunctionExprKind::String(expression) => {
            Ok(eval_string_function_expr(plan, frame, expression)?.into())
        }
        FunctionExprKind::Float(expression) => {
            Ok(eval_float_function_expr(plan, frame, expression)?.into())
        }
        FunctionExprKind::Bool(expression) => {
            Ok(eval_bool_function_expr(plan, frame, expression)?.into())
        }
        FunctionExprKind::Nil(expression) => {
            Ok(eval_nil_function_expr(plan, frame, expression)?.into())
        }
        FunctionExprKind::Tuple(expression) => {
            Ok(eval_tuple_function_expr(plan, frame, expression)?.into())
        }
        FunctionExprKind::List(expression) => {
            Ok(eval_list_function_expr(plan, frame, expression)?.into())
        }
        FunctionExprKind::Function(expression) => {
            Ok(eval_function_function_expr(plan, frame, expression)?.into())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_function_expr;
    use crate::plan::{
        BoolFunctionExpr, BoolFunctionId, BoolLocalId, FloatFunctionExpr, FloatFunctionId,
        FloatLocalId, FunctionExpr, FunctionFunctionExpr, FunctionFunctionId, FunctionPlan,
        FunctionType, FunctionValue, IntExpr, IntFunctionExpr, IntFunctionFunctionId,
        IntFunctionId, IntListLocalId, IntLocalId, ListFunctionExpr, ListFunctionId, ListLocal,
        NilFunctionExpr, NilFunctionId, NilLocalId, ParamLocal, RuntimeFunctionId,
        StringFunctionExpr, StringFunctionId, StringLocalId, TupleExpr, TupleFunctionExpr,
        TupleFunctionId, TupleLocalId, ValueType,
    };
    use crate::runtime::ExecutionError;
    use crate::runtime::frame::Frame;

    #[test]
    fn eval_function_value() {
        let plan = plan();
        let mut frame = Frame::default();
        let function = eval_function_expr(
            &plan,
            &mut frame,
            &FunctionExpr::value(FunctionValue::new(
                RuntimeFunctionId::Int(IntFunctionId(0)),
                vec![ParamLocal::int(IntLocalId(0))],
            )),
        )
        .expect("expression should evaluate");
        let type_ = function.type_();

        assert_eq!(
            type_,
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        assert_eq!(type_.return_(), &ValueType::Int);
    }

    #[test]
    fn eval_function_value_return_families() {
        let plan = plan();
        let mut frame = Frame::default();
        let mut assert_return_type = |value: FunctionValue, expected: ValueType| {
            assert_eq!(
                eval_function_expr(&plan, &mut frame, &FunctionExpr::value(value))
                    .expect("expression should evaluate")
                    .type_()
                    .return_(),
                &expected,
            );
        };

        assert_return_type(
            FunctionValue::new(
                RuntimeFunctionId::String(StringFunctionId(0)),
                vec![ParamLocal::string(StringLocalId(0))],
            ),
            ValueType::String,
        );
        assert_return_type(
            FunctionValue::new(
                RuntimeFunctionId::Float(FloatFunctionId(0)),
                vec![ParamLocal::float(FloatLocalId(0))],
            ),
            ValueType::Float,
        );
        assert_return_type(
            FunctionValue::new(
                RuntimeFunctionId::Bool(BoolFunctionId(0)),
                vec![ParamLocal::bool(BoolLocalId(0))],
            ),
            ValueType::Bool,
        );
        assert_return_type(
            FunctionValue::new(
                RuntimeFunctionId::Nil(NilFunctionId(0)),
                vec![ParamLocal::nil(NilLocalId(0))],
            ),
            ValueType::Nil,
        );
        assert_return_type(
            FunctionValue::new(
                RuntimeFunctionId::Tuple {
                    id: TupleFunctionId(0),
                    return_type: vec![ValueType::Int],
                },
                vec![ParamLocal::tuple(TupleLocalId(0), vec![ValueType::Int])],
            ),
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_return_type(
            FunctionValue::new(
                RuntimeFunctionId::List(ListFunctionId::from_item_type(
                    0,
                    crate::plan::ValueType::Int,
                )),
                vec![ParamLocal::list(ListLocal::int(IntListLocalId(0)))],
            ),
            ValueType::List(Box::new(ValueType::Int)),
        );
        assert_return_type(
            FunctionValue::new(
                RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                },
                Vec::new(),
            ),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Int,
            ))),
        );
    }

    #[test]
    fn eval_function_expr_propagates_family_errors() {
        let plan = plan();
        let mut frame = Frame::default();
        let mut assert_tuple_index_error = |expression: FunctionExpr, type_: FunctionType| {
            assert_eq!(
                eval_function_expr(&plan, &mut frame, &expression),
                Err(ExecutionError::tuple_index_family_mismatch(
                    ValueType::Function(Box::new(type_)),
                    ValueType::Tuple(Vec::new()),
                )),
            );
        };
        let empty_tuple = || TupleExpr::value(Vec::new(), Vec::new());

        let int_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        assert_tuple_index_error(
            FunctionExpr::int(IntFunctionExpr::tuple_index(
                empty_tuple(),
                0,
                int_type.clone(),
            )),
            int_type,
        );

        let string_type = FunctionType::new(vec![ValueType::String], ValueType::String);
        assert_tuple_index_error(
            FunctionExpr::string(StringFunctionExpr::tuple_index(
                empty_tuple(),
                0,
                string_type.clone(),
            )),
            string_type,
        );

        let float_type = FunctionType::new(vec![ValueType::Float], ValueType::Float);
        assert_tuple_index_error(
            FunctionExpr::float(FloatFunctionExpr::tuple_index(
                empty_tuple(),
                0,
                float_type.clone(),
            )),
            float_type,
        );

        let bool_type = FunctionType::new(vec![ValueType::Bool], ValueType::Bool);
        assert_tuple_index_error(
            FunctionExpr::bool(BoolFunctionExpr::tuple_index(
                empty_tuple(),
                0,
                bool_type.clone(),
            )),
            bool_type,
        );

        let nil_type = FunctionType::new(vec![ValueType::Nil], ValueType::Nil);
        assert_tuple_index_error(
            FunctionExpr::nil(NilFunctionExpr::tuple_index(
                empty_tuple(),
                0,
                nil_type.clone(),
            )),
            nil_type,
        );

        let tuple_type = FunctionType::new(
            vec![ValueType::Tuple(vec![ValueType::Int])],
            ValueType::Tuple(vec![ValueType::Int]),
        );
        assert_tuple_index_error(
            FunctionExpr::tuple(TupleFunctionExpr::tuple_index(
                empty_tuple(),
                0,
                tuple_type.clone(),
            )),
            tuple_type,
        );

        let list_type = FunctionType::new(
            vec![ValueType::List(Box::new(ValueType::Int))],
            ValueType::List(Box::new(ValueType::Int)),
        );
        assert_tuple_index_error(
            FunctionExpr::list(ListFunctionExpr::tuple_index(
                empty_tuple(),
                0,
                list_type.clone(),
                ValueType::Int,
            )),
            list_type,
        );

        let function_type = FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Int,
            ))),
        );
        assert_tuple_index_error(
            FunctionExpr::function(FunctionFunctionExpr::tuple_index(
                empty_tuple(),
                0,
                function_type.clone(),
            )),
            function_type,
        );
    }

    fn plan() -> crate::plan::ExecutionPlan {
        crate::plan::ExecutionPlan::new(
            "main".into(),
            FunctionPlan::new(
                crate::plan::FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                crate::plan::ReturnExpr::int(
                    crate::plan::IntFunctionId(0),
                    IntExpr::value(1.into()),
                ),
            ),
            Vec::new(),
        )
    }
}
