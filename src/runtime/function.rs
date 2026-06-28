use crate::plan::{
    BoolFunctionFunctionId, BoolFunctionId, CallArg, CallArgKind, ExecutionPlan, FrameLayout,
    FunctionFunctionFunctionId, FunctionFunctionValue, FunctionValue, IntFunctionFunctionId,
    IntFunctionId, NilFunctionFunctionId, NilFunctionId, RuntimeFunctionId, StepKind,
    StringFunctionFunctionId, StringFunctionId, Value,
};
use crate::runtime::expression::{
    eval_bool_expr, eval_bool_function_expr, eval_expr, eval_function_function_expr, eval_int_expr,
    eval_int_function_expr, eval_nil_expr, eval_nil_function_expr, eval_string_expr,
    eval_string_function_expr,
};
use crate::runtime::frame::Frame;
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn run_main(plan: &ExecutionPlan) -> Value {
    let mut caller_frame = Frame::default();
    match plan.main_runtime() {
        RuntimeFunctionId::Int(function) => {
            Value::Int(run_int_call(plan, function, &[], &mut caller_frame))
        }
        RuntimeFunctionId::String(function) => {
            Value::String(run_string_call(plan, function, &[], &mut caller_frame))
        }
        RuntimeFunctionId::Bool(function) => {
            Value::Bool(run_bool_call(plan, function, &[], &mut caller_frame))
        }
        RuntimeFunctionId::Nil(function) => {
            run_nil_call(plan, function, &[], &mut caller_frame);
            Value::Nil
        }
        RuntimeFunctionId::Function { id, .. } => Value::Function(
            run_function_returning_function_call(plan, id, &[], &mut caller_frame),
        ),
    }
}

pub(super) fn run_int_call(
    plan: &ExecutionPlan,
    function: IntFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> BigInt {
    let function = plan.int_function(function);
    let mut frame = bind_arguments(plan, args, caller_frame, function.frame_layout());
    execute_steps(plan, function.steps(), &mut frame);
    eval_int_expr(plan, &mut frame, function.return_())
}

pub(super) fn run_string_call(
    plan: &ExecutionPlan,
    function: StringFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> EcoString {
    let function = plan.string_function(function);
    let mut frame = bind_arguments(plan, args, caller_frame, function.frame_layout());
    execute_steps(plan, function.steps(), &mut frame);
    eval_string_expr(plan, &mut frame, function.return_())
}

pub(super) fn run_bool_call(
    plan: &ExecutionPlan,
    function: BoolFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> bool {
    let function = plan.bool_function(function);
    let mut frame = bind_arguments(plan, args, caller_frame, function.frame_layout());
    execute_steps(plan, function.steps(), &mut frame);
    eval_bool_expr(plan, &mut frame, function.return_())
}

pub(super) fn run_nil_call(
    plan: &ExecutionPlan,
    function: NilFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) {
    let function = plan.nil_function(function);
    let mut frame = bind_arguments(plan, args, caller_frame, function.frame_layout());
    execute_steps(plan, function.steps(), &mut frame);
    eval_nil_expr(plan, &mut frame, function.return_());
}

pub(in crate::runtime) fn execute_steps(
    plan: &ExecutionPlan,
    steps: &[crate::plan::Step],
    frame: &mut Frame,
) {
    for step in steps {
        match step.kind() {
            StepKind::LetInt { local, value, .. } => {
                let value = eval_int_expr(plan, frame, value);
                frame.set_int(*local, value);
            }
            StepKind::LetString { local, value, .. } => {
                let value = eval_string_expr(plan, frame, value);
                frame.set_string(*local, value);
            }
            StepKind::LetBool { local, value, .. } => {
                let value = eval_bool_expr(plan, frame, value);
                frame.set_bool(*local, value);
            }
            StepKind::LetNil { local, value, .. } => {
                eval_nil_expr(plan, frame, value);
                frame.set_nil(*local);
            }
            StepKind::LetIntFunction { local, value, .. } => {
                let value = eval_int_function_expr(plan, frame, value);
                frame.set_int_function(*local, value);
            }
            StepKind::LetStringFunction { local, value, .. } => {
                let value = eval_string_function_expr(plan, frame, value);
                frame.set_string_function(*local, value);
            }
            StepKind::LetBoolFunction { local, value, .. } => {
                let value = eval_bool_function_expr(plan, frame, value);
                frame.set_bool_function(*local, value);
            }
            StepKind::LetNilFunction { local, value, .. } => {
                let value = eval_nil_function_expr(plan, frame, value);
                frame.set_nil_function(*local, value);
            }
            StepKind::LetFunctionFunction { local, value, .. } => {
                let value = eval_function_function_expr(plan, frame, value);
                frame.set_function_function(*local, value);
            }
            StepKind::Evaluate(expression) => {
                let _ = eval_expr(plan, frame, expression);
            }
        }
    }
}

fn bind_arguments(
    plan: &ExecutionPlan,
    args: &[CallArg],
    caller_frame: &mut Frame,
    frame_layout: FrameLayout,
) -> Frame {
    let mut frame = Frame::new(frame_layout);

    for arg in args {
        match arg.kind() {
            CallArgKind::Int { local, value } => {
                let value = eval_int_expr(plan, caller_frame, value);
                frame.set_int(*local, value);
            }
            CallArgKind::String { local, value } => {
                let value = eval_string_expr(plan, caller_frame, value);
                frame.set_string(*local, value);
            }
            CallArgKind::Bool { local, value } => {
                let value = eval_bool_expr(plan, caller_frame, value);
                frame.set_bool(*local, value);
            }
            CallArgKind::Nil { local, value } => {
                eval_nil_expr(plan, caller_frame, value);
                frame.set_nil(*local);
            }
            CallArgKind::IntFunction { local, value } => {
                let value = eval_int_function_expr(plan, caller_frame, value);
                frame.set_int_function(*local, value);
            }
            CallArgKind::StringFunction { local, value } => {
                let value = eval_string_function_expr(plan, caller_frame, value);
                frame.set_string_function(*local, value);
            }
            CallArgKind::BoolFunction { local, value } => {
                let value = eval_bool_function_expr(plan, caller_frame, value);
                frame.set_bool_function(*local, value);
            }
            CallArgKind::NilFunction { local, value } => {
                let value = eval_nil_function_expr(plan, caller_frame, value);
                frame.set_nil_function(*local, value);
            }
            CallArgKind::FunctionFunction { local, value } => {
                let value = eval_function_function_expr(plan, caller_frame, value);
                frame.set_function_function(*local, value);
            }
        }
    }

    frame
}

pub(in crate::runtime) fn run_int_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::IntFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> BigInt {
    let function = eval_int_function_expr(plan, caller_frame, function);
    let runtime_function = plan.int_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let mut frame = bind_arguments(plan, args, caller_frame, frame_layout);
    execute_steps(plan, runtime_function.steps(), &mut frame);
    eval_int_expr(plan, &mut frame, runtime_function.return_())
}

pub(in crate::runtime) fn run_string_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::StringFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> EcoString {
    let function = eval_string_function_expr(plan, caller_frame, function);
    let runtime_function = plan.string_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let mut frame = bind_arguments(plan, args, caller_frame, frame_layout);
    execute_steps(plan, runtime_function.steps(), &mut frame);
    eval_string_expr(plan, &mut frame, runtime_function.return_())
}

pub(in crate::runtime) fn run_bool_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::BoolFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> bool {
    let function = eval_bool_function_expr(plan, caller_frame, function);
    let runtime_function = plan.bool_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let mut frame = bind_arguments(plan, args, caller_frame, frame_layout);
    execute_steps(plan, runtime_function.steps(), &mut frame);
    eval_bool_expr(plan, &mut frame, runtime_function.return_())
}

pub(in crate::runtime) fn run_nil_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::NilFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) {
    let function = eval_nil_function_expr(plan, caller_frame, function);
    let runtime_function = plan.nil_function(function.runtime_id());
    let frame_layout = runtime_function.frame_layout();
    let mut frame = bind_arguments(plan, args, caller_frame, frame_layout);
    execute_steps(plan, runtime_function.steps(), &mut frame);
    eval_nil_expr(plan, &mut frame, runtime_function.return_());
}

pub(in crate::runtime) fn run_int_function_returning_function_call(
    plan: &ExecutionPlan,
    function: IntFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> crate::plan::IntFunctionValue {
    let function = plan.int_function_function(function);
    let mut frame = bind_arguments(plan, args, caller_frame, function.frame_layout());
    execute_steps(plan, function.steps(), &mut frame);
    eval_int_function_expr(plan, &mut frame, function.return_())
}

pub(in crate::runtime) fn run_string_function_returning_function_call(
    plan: &ExecutionPlan,
    function: StringFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> crate::plan::StringFunctionValue {
    let function = plan.string_function_function(function);
    let mut frame = bind_arguments(plan, args, caller_frame, function.frame_layout());
    execute_steps(plan, function.steps(), &mut frame);
    eval_string_function_expr(plan, &mut frame, function.return_())
}

pub(in crate::runtime) fn run_bool_function_returning_function_call(
    plan: &ExecutionPlan,
    function: BoolFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> crate::plan::BoolFunctionValue {
    let function = plan.bool_function_function(function);
    let mut frame = bind_arguments(plan, args, caller_frame, function.frame_layout());
    execute_steps(plan, function.steps(), &mut frame);
    eval_bool_function_expr(plan, &mut frame, function.return_())
}

pub(in crate::runtime) fn run_nil_function_returning_function_call(
    plan: &ExecutionPlan,
    function: NilFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> crate::plan::NilFunctionValue {
    let function = plan.nil_function_function(function);
    let mut frame = bind_arguments(plan, args, caller_frame, function.frame_layout());
    execute_steps(plan, function.steps(), &mut frame);
    eval_nil_function_expr(plan, &mut frame, function.return_())
}

pub(in crate::runtime) fn run_function_function_returning_function_call(
    plan: &ExecutionPlan,
    function: FunctionFunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> FunctionFunctionValue {
    let function = plan.function_function_function(function);
    let mut frame = bind_arguments(plan, args, caller_frame, function.frame_layout());
    execute_steps(plan, function.steps(), &mut frame);
    eval_function_function_expr(plan, &mut frame, function.return_())
}

pub(in crate::runtime) fn run_int_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> crate::plan::IntFunctionValue {
    let function = eval_function_function_expr(plan, caller_frame, function);
    let function_id = function.runtime_id().int();
    let runtime_function = plan.int_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let mut frame = bind_arguments(plan, args, caller_frame, frame_layout);
    execute_steps(plan, runtime_function.steps(), &mut frame);
    eval_int_function_expr(plan, &mut frame, runtime_function.return_())
}

pub(in crate::runtime) fn run_string_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> crate::plan::StringFunctionValue {
    let function = eval_function_function_expr(plan, caller_frame, function);
    let function_id = function.runtime_id().string();
    let runtime_function = plan.string_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let mut frame = bind_arguments(plan, args, caller_frame, frame_layout);
    execute_steps(plan, runtime_function.steps(), &mut frame);
    eval_string_function_expr(plan, &mut frame, runtime_function.return_())
}

pub(in crate::runtime) fn run_bool_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> crate::plan::BoolFunctionValue {
    let function = eval_function_function_expr(plan, caller_frame, function);
    let function_id = function.runtime_id().bool();
    let runtime_function = plan.bool_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let mut frame = bind_arguments(plan, args, caller_frame, frame_layout);
    execute_steps(plan, runtime_function.steps(), &mut frame);
    eval_bool_function_expr(plan, &mut frame, runtime_function.return_())
}

pub(in crate::runtime) fn run_nil_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> crate::plan::NilFunctionValue {
    let function = eval_function_function_expr(plan, caller_frame, function);
    let function_id = function.runtime_id().nil();
    let runtime_function = plan.nil_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let mut frame = bind_arguments(plan, args, caller_frame, frame_layout);
    execute_steps(plan, runtime_function.steps(), &mut frame);
    eval_nil_function_expr(plan, &mut frame, runtime_function.return_())
}

pub(in crate::runtime) fn run_function_function_function_call(
    plan: &ExecutionPlan,
    function: &crate::plan::FunctionFunctionExpr,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> FunctionFunctionValue {
    let function = eval_function_function_expr(plan, caller_frame, function);
    let function_id = function.runtime_id().function();
    let runtime_function = plan.function_function_function(function_id);
    let frame_layout = runtime_function.frame_layout();
    let mut frame = bind_arguments(plan, args, caller_frame, frame_layout);
    execute_steps(plan, runtime_function.steps(), &mut frame);
    eval_function_function_expr(plan, &mut frame, runtime_function.return_())
}

fn run_function_returning_function_call(
    plan: &ExecutionPlan,
    function: crate::plan::FunctionFunctionId,
    args: &[CallArg],
    caller_frame: &mut Frame,
) -> FunctionValue {
    match function {
        crate::plan::FunctionFunctionId::Int(function) => {
            run_int_function_returning_function_call(plan, function, args, caller_frame).into()
        }
        crate::plan::FunctionFunctionId::String(function) => {
            run_string_function_returning_function_call(plan, function, args, caller_frame).into()
        }
        crate::plan::FunctionFunctionId::Bool(function) => {
            run_bool_function_returning_function_call(plan, function, args, caller_frame).into()
        }
        crate::plan::FunctionFunctionId::Nil(function) => {
            run_nil_function_returning_function_call(plan, function, args, caller_frame).into()
        }
        crate::plan::FunctionFunctionId::Function(function) => {
            run_function_function_returning_function_call(plan, function, args, caller_frame).into()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{Value, int, run_src};
    use crate::plan::{FunctionType, ValueType};

    #[test]
    fn execute_let_binding() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let x = 1
  x + 2
}
"#,
            ),
            int(3),
        );
    }

    #[test]
    fn execute_expression_steps() {
        assert_eq!(
            run_src(
                r#"
fn identity(value: Int) {
  value
}

fn get_identity() {
  identity
}

pub fn main() {
  "side"
  True
  Nil
  1 == 1
  identity
  get_identity
  5
}
"#,
            ),
            int(5),
        );
    }

    #[test]
    fn execute_function_value_alias_call() {
        assert_eq!(
            run_src(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let add = add_one
  add(1)
}
"#,
            ),
            int(2),
        );

        assert_eq!(
            run_src(
                r#"
fn identity(value: String) {
  value
}

pub fn main() {
  let function = identity
  function("geam")
}
"#,
            ),
            Value::String("geam".into()),
        );

        let expected = Value::Bool(true);
        assert_eq!(
            run_src(
                r#"
fn identity(value: Bool) {
  value
}

pub fn main() {
  let function = identity
  function(True)
}
"#,
            ),
            expected,
        );

        let expected = Value::Nil;
        assert_eq!(
            run_src(
                r#"
fn identity(value: Nil) {
  value
}

pub fn main() {
  let function = identity
  function(Nil)
}
"#,
            ),
            expected,
        );
    }

    #[test]
    fn execute_main_returning_function_value() {
        assert_returned_function_type(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
}
"#,
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );

        assert_returned_function_type(
            r#"
fn identity(value: String) {
  value
}

pub fn main() {
  identity
}
"#,
            FunctionType::new(vec![ValueType::String], ValueType::String),
        );

        assert_returned_function_type(
            r#"
fn identity(value: Bool) {
  value
}

pub fn main() {
  identity
}
"#,
            FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
        );

        assert_returned_function_type(
            r#"
fn identity(value: Nil) {
  value
}

pub fn main() {
  identity
}
"#,
            FunctionType::new(vec![ValueType::Nil], ValueType::Nil),
        );

        assert_returned_function_type(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

pub fn main() {
  get
}
"#,
            FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Int,
                ))),
            ),
        );
    }

    #[test]
    fn execute_typed_let_steps() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let text = "geam"
  let flag = True
  let none = Nil
  text
}
"#,
            ),
            Value::String("geam".into()),
        );
    }

    #[test]
    fn execute_typed_function_calls() {
        assert_eq!(
            run_src(
                r#"
fn add(a: Int, b: Int) {
  a + b
}

pub fn main() {
  add(1, 2)
}
"#,
            ),
            int(3),
        );

        assert_eq!(
            run_src(
                r#"
fn join(a: String, b: String) {
  a <> b
}

pub fn main() {
  join("ge", "am")
}
"#,
            ),
            Value::String("geam".into()),
        );

        let expected = Value::Bool(true);
        assert_eq!(
            run_src(
                r#"
fn id(value: Bool) {
  value
}

pub fn main() {
  id(True)
}
"#,
            ),
            expected,
        );

        let expected = Value::Nil;
        assert_eq!(
            run_src(
                r#"
fn id(value: Nil) {
  value
}

pub fn main() {
  id(Nil)
}
"#,
            ),
            expected,
        );
    }

    #[test]
    fn execute_sparse_bool_local_after_skipped_block() {
        let expected = Value::Bool(true);
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  False && {
    let x = True
    x
  }

  let y = True
  y
}
"#,
            ),
            expected,
        );
    }

    #[test]
    fn execute_sparse_bool_local_after_untaken_case_block() {
        let expected = Value::Bool(true);
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case False {
    True -> {
      let x = True
      x
    }
    False -> False
  }

  let y = True
  y
}
"#,
            ),
            expected,
        );
    }

    #[test]
    #[should_panic]
    fn assert_returned_function_type_panics_on_non_function() {
        assert_returned_function_type(
            r#"
pub fn main() {
  1
}
"#,
            FunctionType::new(Vec::new(), ValueType::Int),
        );
    }

    fn assert_returned_function_type(src: &str, expected: FunctionType) {
        let mut actual_type = None;

        if let Value::Function(function) = run_src(src) {
            actual_type = Some(function.type_());
        }

        assert_eq!(actual_type, Some(expected));
    }

}
