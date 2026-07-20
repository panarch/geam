mod bit_array;
mod bool;
mod custom;
mod float;
mod generic;
mod int;
mod list;
mod never;
mod nil;
mod returning_function;
mod string;
mod tuple;
mod utf_codepoint;

use crate::plan::ValueType;
use crate::plan::execution::{CustomConstructorId, CustomFieldAccess, ExecutionPlan};
use crate::plan::execution::{FunctionExpr, FunctionExprKind};
use crate::runtime::frame::Frame;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedCustomFunction, EvaluatedFunction, EvaluatedFunctionValue, EvaluatedValue,
    ExecutionError, InvariantError,
};

pub(in crate::runtime) use self::{
    bit_array::eval_bit_array_function_expr,
    bool::eval_bool_function_expr,
    custom::{eval_custom_function_expr, eval_custom_function_expr_kind},
    float::eval_float_function_expr,
    generic::{eval_generic_function_expr, eval_generic_function_expr_kind},
    int::eval_int_function_expr,
    list::eval_list_function_expr,
    never::{eval_never_function_expr, eval_never_function_expr_kind},
    nil::eval_nil_function_expr,
    returning_function::{eval_function_function_expr, eval_function_function_expr_kind},
    string::eval_string_function_expr,
    tuple::eval_tuple_function_expr,
    utf_codepoint::eval_utf_codepoint_function_expr,
};

pub(in crate::runtime) fn eval_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &FunctionExpr,
) -> Result<EvaluatedFunctionValue, ExecutionError> {
    let value: EvaluatedFunctionValue = match expression.kind() {
        FunctionExprKind::Generic(expression) => {
            eval_generic_function_expr(plan, state, frame, expression)?.into()
        }
        FunctionExprKind::Never(expression) => {
            eval_never_function_expr(plan, state, frame, expression)?.into()
        }
        FunctionExprKind::Int(expression) => {
            eval_int_function_expr(plan, state, frame, expression)?.into()
        }
        FunctionExprKind::String(expression) => {
            eval_string_function_expr(plan, state, frame, expression)?.into()
        }
        FunctionExprKind::BitArray(expression) => {
            eval_bit_array_function_expr(plan, state, frame, expression)?.into()
        }
        FunctionExprKind::UtfCodepoint(expression) => {
            eval_utf_codepoint_function_expr(plan, state, frame, expression)?.into()
        }
        FunctionExprKind::Custom(expression) => {
            eval_custom_function_expr(plan, state, frame, expression)?.into()
        }
        FunctionExprKind::Float(expression) => {
            eval_float_function_expr(plan, state, frame, expression)?.into()
        }
        FunctionExprKind::Bool(expression) => {
            eval_bool_function_expr(plan, state, frame, expression)?.into()
        }
        FunctionExprKind::Nil(expression) => {
            eval_nil_function_expr(plan, state, frame, expression)?.into()
        }
        FunctionExprKind::Tuple(expression) => {
            eval_tuple_function_expr(plan, state, frame, expression)?.into()
        }
        FunctionExprKind::List(expression) => {
            eval_list_function_expr(plan, state, frame, expression)?.into()
        }
        FunctionExprKind::Function(expression) => {
            eval_function_function_expr(plan, state, frame, expression)?.into()
        }
    };
    Ok(value.with_type(expression.shape().type_().clone()))
}

pub(in crate::runtime) fn eval_typed_function_expr<Expression, Id: Clone>(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &crate::plan::execution::TypedFunctionExpr<Expression>,
    eval: impl FnOnce(
        &ExecutionPlan,
        &mut RuntimeState,
        &mut Frame,
        &Expression,
    ) -> Result<EvaluatedFunction<Id>, ExecutionError>,
) -> Result<EvaluatedFunction<Id>, ExecutionError> {
    let value = eval(plan, state, frame, expression.expression())?;
    Ok(value.with_type(expression.shape().type_().clone()))
}

pub(in crate::runtime) fn eval_typed_custom_function_expr<Expression>(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &crate::plan::execution::TypedFunctionExpr<Expression>,
    eval: impl FnOnce(
        &ExecutionPlan,
        &mut RuntimeState,
        &mut Frame,
        &Expression,
    ) -> Result<EvaluatedCustomFunction, ExecutionError>,
) -> Result<EvaluatedCustomFunction, ExecutionError> {
    let value = eval(plan, state, frame, expression.expression())?;
    Ok(value.with_type(expression.shape().type_().clone()))
}

fn eval_custom_field_function(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    access: &CustomFieldAccess,
) -> Result<(CustomConstructorId, ValueType, EvaluatedFunctionValue), ExecutionError> {
    let (constructor, value) = super::eval_custom_field(plan, state, frame, access)?;
    let descriptor = plan.custom_constructor(constructor);
    let expected = plan.value_type(descriptor.fields()[access.index()].type_());
    match value {
        EvaluatedValue::Function(value) => Ok((constructor, expected, value)),
        other => Err(ExecutionError::Invariant(
            InvariantError::CustomFieldFamilyMismatch {
                custom_type: plan.custom_value_type(constructor.type_id()),
                constructor: descriptor.name().clone(),
                field_index: access.index(),
                expected,
                actual: other.value_type(plan),
            },
        )),
    }
}

#[cfg(test)]
fn expect_function_list(expression: crate::plan::ListExpr) -> crate::plan::FunctionListExpr {
    match expression {
        crate::plan::ListExpr::Function(expression) => expression,
        _ => panic!("expected a function-valued list expression"),
    }
}

#[cfg(test)]
mod tests {
    use crate::runtime::run_main;

    #[test]
    fn function_expression_propagates_never_function_errors() {
        let plan = crate::runtime::plan_src(
            r#"
fn diverge(_value: Int) -> value { panic }

pub fn main() {
  [case False {
    True -> diverge
    False -> panic as "function"
  }]
}
"#,
        );

        assert_eq!(
            run_main(&plan)
                .expect_err("function list element should propagate its panic")
                .to_string(),
            "panic: function",
        );
    }

    #[test]
    fn compound_function_tuple_projections_propagate_tuple_errors() {
        let sources = [
            "fn provider() -> #(fn() -> List(Int)) { panic } pub fn main() { provider().0 }",
            "fn provider() -> #(fn() -> #(Int)) { panic } pub fn main() { provider().0 }",
        ];

        for source in sources {
            let plan = crate::runtime::plan_src(source);
            let error = run_main(&plan).expect_err("tuple provider panic should propagate");

            assert_eq!(error.to_string(), "panic: `panic` expression evaluated.");
        }
    }

    #[test]
    #[should_panic(expected = "expected a function-valued list expression")]
    fn function_list_shape_guard_rejects_int_lists() {
        let expression = crate::plan::ListExpr::panic(
            crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
            crate::plan::ValueType::Int,
        );

        let _ = super::expect_function_list(expression);
    }
}
