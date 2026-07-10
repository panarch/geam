mod bool;
mod float;
mod function;
mod int;
mod list;
mod nil;
mod string;
mod tuple;

use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{Expr, ExprKind, PanicExpr, PanicExprKind};
use crate::runtime::frame::Frame;
use crate::runtime::{ExecutionError, PanicKind, Value};
use std::convert::Infallible;

pub(super) use self::{
    bool::eval_bool_expr,
    float::eval_float_expr,
    function::{
        eval_bool_function_expr, eval_float_function_expr, eval_function_expr,
        eval_function_function_expr, eval_int_function_expr, eval_list_function_expr,
        eval_nil_function_expr, eval_string_function_expr, eval_tuple_function_expr,
    },
    int::eval_int_expr,
    list::{
        eval_bool_list_expr, eval_float_list_expr, eval_function_list_expr, eval_int_list_expr,
        eval_list_expr, eval_list_list_expr, eval_nil_list_expr, eval_string_list_expr,
        eval_tuple_list_expr, get_list_value, project_bool_list_expr, project_float_list_expr,
        project_function_list_expr, project_int_list_expr, project_nil_list_expr,
        project_string_list_expr, project_tuple_list_expr,
    },
    nil::eval_nil_expr,
    string::eval_string_expr,
    tuple::{eval_tuple_expr, project_tuple_expr},
};

pub(super) fn eval_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &Expr,
) -> Result<Value, ExecutionError> {
    match expression.kind() {
        ExprKind::Int(expression) => Ok(Value::Int(eval_int_expr(plan, frame, expression)?)),
        ExprKind::String(expression) => {
            Ok(Value::String(eval_string_expr(plan, frame, expression)?))
        }
        ExprKind::Float(expression) => Ok(Value::Float(eval_float_expr(plan, frame, expression)?)),
        ExprKind::Bool(expression) => Ok(Value::Bool(eval_bool_expr(plan, frame, expression)?)),
        ExprKind::Nil(expression) => {
            eval_nil_expr(plan, frame, expression)?;
            Ok(Value::Nil)
        }
        ExprKind::Tuple(expression) => Ok(Value::Tuple(eval_tuple_expr(plan, frame, expression)?)),
        ExprKind::List(expression) => Ok(Value::List(eval_list_expr(plan, frame, expression)?)),
        ExprKind::Function(expression) => {
            let value = eval_function_expr(plan, frame, expression)?;
            Ok(Value::Function(value))
        }
    }
}

pub(super) fn eval_panic_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &PanicExpr,
) -> Result<Infallible, ExecutionError> {
    let (kind, message) = match expression.kind() {
        PanicExprKind::Panic { message } => (
            PanicKind::Panic,
            eval_panic_message(plan, frame, message.as_deref())?,
        ),
        PanicExprKind::Todo { message } => (
            PanicKind::Todo,
            eval_panic_message(plan, frame, message.as_deref())?,
        ),
        PanicExprKind::EmptyFunction => (PanicKind::EmptyFunction, None),
        PanicExprKind::EmptyBlock => (PanicKind::EmptyBlock, None),
        PanicExprKind::IncompleteUse => (PanicKind::IncompleteUse, None),
    };

    Err(ExecutionError::source_panic(
        plan.source_context(),
        kind,
        message,
        expression.site().clone(),
    ))
}

fn eval_panic_message(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    message: Option<&crate::plan::execution::StringExpr>,
) -> Result<Option<ecow::EcoString>, ExecutionError> {
    match message {
        Some(message) => Ok(Some(eval_string_expr(plan, frame, message)?)),
        None => Ok(None),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        eval_bool_expr, eval_bool_function_expr, eval_bool_list_expr, eval_float_expr,
        eval_float_function_expr, eval_float_list_expr, eval_function_function_expr,
        eval_function_list_expr, eval_int_expr, eval_int_function_expr, eval_int_list_expr,
        eval_list_function_expr, eval_list_list_expr, eval_nil_expr, eval_nil_function_expr,
        eval_nil_list_expr, eval_string_expr, eval_string_function_expr, eval_string_list_expr,
        eval_tuple_expr, eval_tuple_function_expr, eval_tuple_list_expr,
    };
    use crate::plan::execution::{
        BoolFunctionFunctionId, BoolFunctionId, BoolListFunctionId, FloatFunctionFunctionId,
        FloatFunctionId, FloatListFunctionId, FunctionFunctionFunctionId, IntFunctionFunctionId,
        IntFunctionId, IntListFunctionId, NilFunctionFunctionId, NilFunctionId, NilListFunctionId,
        ReturnBody, ReturnBodyKind, RuntimeFunctionId, StringFunctionFunctionId, StringFunctionId,
        StringListFunctionId, TupleFunctionFunctionId, TupleFunctionId, TupleListFunctionId,
        TupleLocalId,
    };
    use crate::plan::{FunctionType, ValueType};
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, FunctionValue, ListValue, Value};

    #[test]
    fn panic_and_todo_message_errors_propagate() {
        for source in [
            r#"
fn fail_message() -> String { panic as "message" }
pub fn main() -> Int { panic as fail_message() }
"#,
            r#"
fn fail_message() -> String { panic as "message" }
pub fn main() -> Int { todo as fail_message() }
"#,
        ] {
            assert_eq!(
                crate::runtime::run_src_error(source).to_string(),
                "panic: message",
            );
        }
    }

    #[test]
    fn empty_expression_panics_preserve_their_source_kind() {
        let cases = [
            (
                include_str!(
                    "../../tests/fixtures/execution_errors/expressions/empty_function.gleam"
                ),
                "empty_function: Function body is empty.",
            ),
            (
                include_str!("../../tests/fixtures/execution_errors/expressions/empty_block.gleam"),
                "empty_block: Block is empty.",
            ),
            (
                include_str!(
                    "../../tests/fixtures/execution_errors/functions/use/incomplete_use.gleam"
                ),
                "incomplete_use: Use callback is incomplete.",
            ),
        ];

        for (source, expected) in cases {
            assert_eq!(crate::runtime::run_src_error(source).to_string(), expected);
        }
    }

    #[test]
    fn tuple_projection_invariants_are_preserved_for_every_return_family() {
        let plan = crate::runtime::plan_src(
            r#"
fn int_value(value: #(Int)) { value.0 }
fn string_value(value: #(String)) { value.0 }
fn float_value(value: #(Float)) { value.0 }
fn bool_value(value: #(Bool)) { value.0 }
fn nil_value(value: #(Nil)) { value.0 }
fn tuple_value(value: #(#(Int))) { value.0 }

fn int_list(value: #(List(Int))) { value.0 }
fn string_list(value: #(List(String))) { value.0 }
fn float_list(value: #(List(Float))) { value.0 }
fn bool_list(value: #(List(Bool))) { value.0 }
fn nil_list(value: #(List(Nil))) { value.0 }
fn tuple_list(value: #(List(#(Int)))) { value.0 }
fn list_list(value: #(List(List(Int)))) { value.0 }
fn function_list(value: #(List(fn() -> Int))) { value.0 }

fn int_function(value: #(fn() -> Int)) { value.0 }
fn string_function(value: #(fn() -> String)) { value.0 }
fn float_function(value: #(fn() -> Float)) { value.0 }
fn bool_function(value: #(fn() -> Bool)) { value.0 }
fn nil_function(value: #(fn() -> Nil)) { value.0 }
fn tuple_function(value: #(fn() -> #(Int))) { value.0 }
fn list_function(value: #(fn() -> List(Int))) { value.0 }
fn function_function(value: #(fn() -> fn() -> Int)) { value.0 }

pub fn main() { Nil }
"#,
        );
        let actual = ValueType::Tuple(Vec::new());
        let wrong_tuple = vec![Value::Tuple(Vec::new())];

        let function = plan.int_function(IntFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_int_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Int,
                actual.clone(),
            )),
        );
        frame.set_tuple(TupleLocalId(0), Vec::new());
        assert_eq!(
            eval_int_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Int,
                ValueType::Tuple(Vec::new()),
            )),
        );

        let function = plan.string_function(StringFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_string_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::String,
                actual.clone(),
            )),
        );

        let function = plan.float_function(FloatFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_float_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Float,
                actual.clone(),
            )),
        );

        let function = plan.bool_function(BoolFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_bool_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Bool,
                actual.clone(),
            )),
        );

        let function = plan.nil_function(NilFunctionId(1));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_nil_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Nil,
                actual.clone(),
            )),
        );

        let function = plan.tuple_function(TupleFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), vec![Value::String("wrong".into())]);
        assert_eq!(
            eval_tuple_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Tuple(vec![ValueType::Int]),
                ValueType::String,
            )),
        );

        let function = plan.int_list_function(IntListFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_int_list_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::Int)),
                actual.clone(),
            )),
        );
        frame.set_tuple(
            TupleLocalId(0),
            vec![Value::List(ListValue::string(vec!["wrong".into()]))],
        );
        assert_eq!(
            eval_int_list_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::Int)),
                ValueType::List(Box::new(ValueType::String)),
            )),
        );

        let function = plan.string_list_function(StringListFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_string_list_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::String)),
                actual.clone(),
            )),
        );

        let function = plan.float_list_function(FloatListFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_float_list_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::Float)),
                actual.clone(),
            )),
        );

        let function = plan.bool_list_function(BoolListFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_bool_list_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::Bool)),
                actual.clone(),
            )),
        );

        let function = plan.nil_list_function(NilListFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_nil_list_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::Nil)),
                actual.clone(),
            )),
        );

        let function = plan.tuple_list_function(TupleListFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_tuple_list_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
                actual.clone(),
            )),
        );

        let function = plan.list_list_function(crate::plan::execution::ListListFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_list_list_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
                actual.clone(),
            )),
        );

        let function =
            plan.function_list_function(crate::plan::execution::FunctionListFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        let int_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            eval_function_list_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::List(Box::new(ValueType::Function(Box::new(
                    int_function_type.clone(),
                )))),
                actual.clone(),
            )),
        );

        let expected_function_types = [
            FunctionType::new(Vec::new(), ValueType::Int),
            FunctionType::new(Vec::new(), ValueType::String),
            FunctionType::new(Vec::new(), ValueType::Float),
            FunctionType::new(Vec::new(), ValueType::Bool),
            FunctionType::new(Vec::new(), ValueType::Nil),
            FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
            FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int))),
            FunctionType::new(Vec::new(), ValueType::Function(Box::new(int_function_type))),
        ];

        let function = plan.int_function_function(IntFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_int_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[0].clone())),
                actual.clone(),
            )),
        );

        let function = plan.string_function_function(StringFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_string_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[1].clone())),
                actual.clone(),
            )),
        );

        let function = plan.float_function_function(FloatFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_float_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[2].clone())),
                actual.clone(),
            )),
        );

        let function = plan.bool_function_function(BoolFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_bool_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[3].clone())),
                actual.clone(),
            )),
        );

        let function = plan.nil_function_function(NilFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_nil_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[4].clone())),
                actual.clone(),
            )),
        );

        let function = plan.tuple_function_function(TupleFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_tuple_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[5].clone())),
                actual.clone(),
            )),
        );

        let function =
            plan.list_function_function(&crate::plan::execution::ListFunctionFunctionId::Int {
                id: crate::plan::execution::IntListFunctionFunctionId(0),
                type_: expected_function_types[6].clone(),
            });
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_list_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[6].clone())),
                actual.clone(),
            )),
        );

        let function = plan.function_function_function(FunctionFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), wrong_tuple.clone());
        assert_eq!(
            eval_function_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[7].clone())),
                actual.clone(),
            )),
        );

        let wrong_string_function =
            FunctionValue::new(RuntimeFunctionId::String(StringFunctionId(0)), Vec::new());
        let wrong_int_function =
            FunctionValue::new(RuntimeFunctionId::Int(IntFunctionId(0)), Vec::new());
        let wrong_string_type = ValueType::Function(Box::new(wrong_string_function.type_()));
        let wrong_int_type = ValueType::Function(Box::new(wrong_int_function.type_()));

        let function = plan.int_function_function(IntFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(
            TupleLocalId(0),
            vec![Value::Function(wrong_string_function)],
        );
        assert_eq!(
            eval_int_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[0].clone())),
                wrong_string_type,
            )),
        );

        let function = plan.string_function_function(StringFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(
            TupleLocalId(0),
            vec![Value::Function(wrong_int_function.clone())],
        );
        assert_eq!(
            eval_string_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[1].clone())),
                wrong_int_type.clone(),
            )),
        );

        let function = plan.float_function_function(FloatFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(
            TupleLocalId(0),
            vec![Value::Function(wrong_int_function.clone())],
        );
        assert_eq!(
            eval_float_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[2].clone())),
                wrong_int_type.clone(),
            )),
        );

        let function = plan.bool_function_function(BoolFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(
            TupleLocalId(0),
            vec![Value::Function(wrong_int_function.clone())],
        );
        assert_eq!(
            eval_bool_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[3].clone())),
                wrong_int_type.clone(),
            )),
        );

        let function = plan.nil_function_function(NilFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(
            TupleLocalId(0),
            vec![Value::Function(wrong_int_function.clone())],
        );
        assert_eq!(
            eval_nil_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[4].clone())),
                wrong_int_type.clone(),
            )),
        );

        let function = plan.tuple_function_function(TupleFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(
            TupleLocalId(0),
            vec![Value::Function(wrong_int_function.clone())],
        );
        assert_eq!(
            eval_tuple_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[5].clone())),
                wrong_int_type.clone(),
            )),
        );

        let function =
            plan.list_function_function(&crate::plan::execution::ListFunctionFunctionId::Int {
                id: crate::plan::execution::IntListFunctionFunctionId(0),
                type_: expected_function_types[6].clone(),
            });
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(
            TupleLocalId(0),
            vec![Value::Function(wrong_int_function.clone())],
        );
        assert_eq!(
            eval_list_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[6].clone())),
                wrong_int_type.clone(),
            )),
        );

        let function = plan.function_function_function(FunctionFunctionFunctionId(0));
        let expression = expression_return(function.return_())
            .expect("source function should have an expression return body");
        let mut frame = Frame::new(function.frame_layout());
        frame.set_tuple(TupleLocalId(0), vec![Value::Function(wrong_int_function)]);
        assert_eq!(
            eval_function_function_expr(&plan, &mut frame, expression),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Function(Box::new(expected_function_types[7].clone())),
                wrong_int_type,
            )),
        );
    }

    #[test]
    fn expression_return_shape_guard_rejects_tail_calls_for_every_return_family() {
        let plan = crate::runtime::plan_src(
            r#"
fn int_value() -> Int { int_value() }
fn string_value() -> String { string_value() }
fn float_value() -> Float { float_value() }
fn bool_value() -> Bool { bool_value() }
fn nil_value() -> Nil { nil_value() }
fn tuple_value() -> #(Int) { tuple_value() }

fn int_list() -> List(Int) { int_list() }
fn string_list() -> List(String) { string_list() }
fn float_list() -> List(Float) { float_list() }
fn bool_list() -> List(Bool) { bool_list() }
fn nil_list() -> List(Nil) { nil_list() }
fn tuple_list() -> List(#(Int)) { tuple_list() }
fn list_list() -> List(List(Int)) { list_list() }
fn function_list() -> List(fn() -> Int) { function_list() }

fn int_function() -> fn() -> Int { int_function() }
fn string_function() -> fn() -> String { string_function() }
fn float_function() -> fn() -> Float { float_function() }
fn bool_function() -> fn() -> Bool { bool_function() }
fn nil_function() -> fn() -> Nil { nil_function() }
fn tuple_function() -> fn() -> #(Int) { tuple_function() }
fn list_function() -> fn() -> List(Int) { list_function() }
fn function_function() -> fn() -> fn() -> Int { function_function() }

pub fn main() { Nil }
"#,
        );

        assert_eq!(
            expression_return(plan.int_function(IntFunctionId(0)).return_()).map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(plan.string_function(StringFunctionId(0)).return_()).map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(plan.float_function(FloatFunctionId(0)).return_()).map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(plan.bool_function(BoolFunctionId(0)).return_()).map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(plan.nil_function(NilFunctionId(1)).return_()).map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(plan.tuple_function(TupleFunctionId(0)).return_()).map(|_| ()),
            None,
        );

        assert_eq!(
            expression_return(plan.int_list_function(IntListFunctionId(0)).return_()).map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(plan.string_list_function(StringListFunctionId(0)).return_())
                .map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(plan.float_list_function(FloatListFunctionId(0)).return_())
                .map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(plan.bool_list_function(BoolListFunctionId(0)).return_()).map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(plan.nil_list_function(NilListFunctionId(0)).return_()).map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(plan.tuple_list_function(TupleListFunctionId(0)).return_())
                .map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(
                plan.list_list_function(crate::plan::execution::ListListFunctionId(0))
                    .return_(),
            )
            .map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(
                plan.function_list_function(crate::plan::execution::FunctionListFunctionId(0))
                    .return_(),
            )
            .map(|_| ()),
            None,
        );

        assert_eq!(
            expression_return(
                plan.int_function_function(IntFunctionFunctionId(0))
                    .return_(),
            )
            .map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(
                plan.string_function_function(StringFunctionFunctionId(0))
                    .return_(),
            )
            .map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(
                plan.float_function_function(FloatFunctionFunctionId(0))
                    .return_(),
            )
            .map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(
                plan.bool_function_function(BoolFunctionFunctionId(0))
                    .return_(),
            )
            .map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(
                plan.nil_function_function(NilFunctionFunctionId(0))
                    .return_(),
            )
            .map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(
                plan.tuple_function_function(TupleFunctionFunctionId(0))
                    .return_(),
            )
            .map(|_| ()),
            None,
        );
        let list_function_type =
            FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int)));
        assert_eq!(
            expression_return(
                plan.list_function_function(&crate::plan::execution::ListFunctionFunctionId::Int {
                    id: crate::plan::execution::IntListFunctionFunctionId(0),
                    type_: list_function_type,
                },)
                    .return_(),
            )
            .map(|_| ()),
            None,
        );
        assert_eq!(
            expression_return(
                plan.function_function_function(FunctionFunctionFunctionId(0))
                    .return_(),
            )
            .map(|_| ()),
            None,
        );
    }

    fn expression_return<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> Option<&Expression> {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => Some(expression),
            _ => None,
        }
    }
}
