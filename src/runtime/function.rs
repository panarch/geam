use crate::plan::{
    BoolFunctionId, CallArg, CallArgKind, ExecutionPlan, FrameLayout, IntFunctionId, NilFunctionId,
    RuntimeFunctionId, StepKind, StringFunctionId, Value,
};
use crate::runtime::expression::{
    eval_bool_expr, eval_expr, eval_function_expr, eval_int_expr, eval_nil_expr, eval_string_expr,
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
            StepKind::LetFunction { value, .. } => {
                let _ = eval_function_expr(plan, frame, value);
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
        }
    }

    frame
}

#[cfg(test)]
mod tests {
    use super::super::{Value, int, run_src};

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

pub fn main() {
  "side"
  True
  Nil
  1 == 1
  identity
  5
}
"#,
            ),
            int(5),
        );
    }

    #[test]
    fn execute_function_value_let_step() {
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
            Value::Bool(true),
        );

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
            Value::Nil,
        );
    }

    #[test]
    fn execute_sparse_bool_local_after_skipped_block() {
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
            Value::Bool(true),
        );
    }

    #[test]
    fn execute_sparse_bool_local_after_untaken_case_block() {
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
            Value::Bool(true),
        );
    }
}
