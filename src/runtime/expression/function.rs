mod bool;
mod int;
mod nil;
mod returning_function;
mod string;

use crate::plan::{ExecutionPlan, FunctionExpr, FunctionExprKind, FunctionValue};
use crate::runtime::frame::Frame;

pub(in crate::runtime) use self::{
    bool::eval_bool_function_expr, int::eval_int_function_expr, nil::eval_nil_function_expr,
    returning_function::eval_function_function_expr, string::eval_string_function_expr,
};

pub(in crate::runtime) fn eval_function_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FunctionExpr,
) -> FunctionValue {
    match expression.kind() {
        FunctionExprKind::Int(expression) => eval_int_function_expr(plan, frame, expression).into(),
        FunctionExprKind::String(expression) => {
            eval_string_function_expr(plan, frame, expression).into()
        }
        FunctionExprKind::Bool(expression) => {
            eval_bool_function_expr(plan, frame, expression).into()
        }
        FunctionExprKind::Nil(expression) => eval_nil_function_expr(plan, frame, expression).into(),
        FunctionExprKind::Function(expression) => {
            eval_function_function_expr(plan, frame, expression).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::eval_function_expr;
    use crate::plan::{
        BoolFunctionId, FunctionExpr, FunctionPlan, FunctionType, FunctionValue, IntExpr,
        IntFunctionId, IntLocalId, NilFunctionId, NilLocalId, ParamLocal, RuntimeFunctionId,
        StringFunctionId, StringLocalId, ValueType,
    };
    use crate::runtime::frame::Frame;

    #[test]
    fn eval_function_value() {
        let plan = plan();
        let mut frame = Frame::default();
        let function =
            eval_function_expr(&plan, &mut frame, &FunctionExpr::value(function_value()));

        assert_int_function(function);
    }

    #[test]
    fn eval_function_value_return_families() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_function_expr(
                &plan,
                &mut frame,
                &FunctionExpr::value(string_function_value())
            )
            .type_()
            .return_(),
            &ValueType::String,
        );
        assert_eq!(
            eval_function_expr(
                &plan,
                &mut frame,
                &FunctionExpr::value(bool_function_value())
            )
            .type_()
            .return_(),
            &ValueType::Bool,
        );
        assert_eq!(
            eval_function_expr(
                &plan,
                &mut frame,
                &FunctionExpr::value(nil_function_value())
            )
            .type_()
            .return_(),
            &ValueType::Nil,
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
                crate::plan::ReturnExpr::int(IntExpr::value(1.into())),
            ),
            Vec::new(),
        )
    }

    fn assert_int_function(function: FunctionValue) {
        let type_ = function.type_();

        assert_eq!(
            type_,
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        assert_eq!(type_.return_(), &ValueType::Int);
    }

    fn function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            vec![ParamLocal::int(IntLocalId(0))],
        )
    }

    fn string_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::String(StringFunctionId(0)),
            vec![ParamLocal::string(StringLocalId(0))],
        )
    }

    fn bool_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Bool(BoolFunctionId(0)),
            vec![ParamLocal::bool(crate::plan::BoolLocalId(0))],
        )
    }

    fn nil_function_value() -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Nil(NilFunctionId(0)),
            vec![ParamLocal::nil(NilLocalId(0))],
        )
    }
}
