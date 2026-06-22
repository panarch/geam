use crate::plan::{Expr, FunctionId, FunctionPlan, LocalId, ModulePlan, Step, Value};
use crate::runtime::error::RuntimeError;
use crate::runtime::expression::{
    eval_bool_expr, eval_expr, eval_int_expr, eval_nil_expr, eval_string_expr,
};
use crate::runtime::frame::Frame;
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn run_function(
    plan: &ModulePlan,
    function: FunctionId,
    args: &[Expr],
    caller_frame: &mut Frame,
) -> Result<Value, RuntimeError> {
    let function = &plan.functions[function.0];
    let mut frame = bind_arguments(plan, function, args, caller_frame)?;
    execute_function(plan, function, &mut frame)
}

pub(super) fn run_int_call(
    plan: &ModulePlan,
    function: FunctionId,
    args: &[Expr],
    caller_frame: &mut Frame,
) -> Result<BigInt, RuntimeError> {
    let function = &plan.functions[function.0];
    let mut frame = bind_arguments(plan, function, args, caller_frame)?;
    execute_steps(plan, function, &mut frame)?;
    let Expr::Int(return_) = &function.return_ else {
        invalid_return_shape()
    };
    eval_int_expr(plan, &mut frame, return_)
}

pub(super) fn run_string_call(
    plan: &ModulePlan,
    function: FunctionId,
    args: &[Expr],
    caller_frame: &mut Frame,
) -> Result<EcoString, RuntimeError> {
    let function = &plan.functions[function.0];
    let mut frame = bind_arguments(plan, function, args, caller_frame)?;
    execute_steps(plan, function, &mut frame)?;
    let Expr::String(return_) = &function.return_ else {
        invalid_return_shape()
    };
    eval_string_expr(plan, &mut frame, return_)
}

pub(super) fn run_bool_call(
    plan: &ModulePlan,
    function: FunctionId,
    args: &[Expr],
    caller_frame: &mut Frame,
) -> Result<bool, RuntimeError> {
    let function = &plan.functions[function.0];
    let mut frame = bind_arguments(plan, function, args, caller_frame)?;
    execute_steps(plan, function, &mut frame)?;
    let Expr::Bool(return_) = &function.return_ else {
        invalid_return_shape()
    };
    eval_bool_expr(plan, &mut frame, return_)
}

pub(super) fn run_nil_call(
    plan: &ModulePlan,
    function: FunctionId,
    args: &[Expr],
    caller_frame: &mut Frame,
) -> Result<(), RuntimeError> {
    let function = &plan.functions[function.0];
    let mut frame = bind_arguments(plan, function, args, caller_frame)?;
    execute_steps(plan, function, &mut frame)?;
    let Expr::Nil(return_) = &function.return_ else {
        invalid_return_shape()
    };
    eval_nil_expr(plan, &mut frame, return_)
}

fn execute_function(
    plan: &ModulePlan,
    function: &FunctionPlan,
    frame: &mut Frame,
) -> Result<Value, RuntimeError> {
    execute_steps(plan, function, frame)?;
    eval_expr(plan, frame, &function.return_)
}

fn execute_steps(
    plan: &ModulePlan,
    function: &FunctionPlan,
    frame: &mut Frame,
) -> Result<(), RuntimeError> {
    for step in &function.steps {
        match step {
            Step::LetInt { local, value, .. } => {
                let value = eval_int_expr(plan, frame, value)?;
                frame.set_int(*local, value);
            }
            Step::LetString { local, value, .. } => {
                let value = eval_string_expr(plan, frame, value)?;
                frame.set_string(*local, value);
            }
            Step::LetBool { local, value, .. } => {
                let value = eval_bool_expr(plan, frame, value)?;
                frame.set_bool(*local, value);
            }
            Step::LetNil { local, value, .. } => {
                eval_nil_expr(plan, frame, value)?;
                frame.set_nil(*local);
            }
            Step::Evaluate(expression) => {
                let _ = eval_expr(plan, frame, expression)?;
            }
        }
    }

    Ok(())
}

fn bind_arguments(
    plan: &ModulePlan,
    function: &FunctionPlan,
    args: &[Expr],
    caller_frame: &mut Frame,
) -> Result<Frame, RuntimeError> {
    let mut frame = Frame::default();
    for (param, arg) in function.params.iter().zip(args) {
        match (param.local, arg) {
            (LocalId::Int(local), Expr::Int(arg)) => {
                let value = eval_int_expr(plan, caller_frame, arg)?;
                frame.set_int(local, value);
            }
            (LocalId::String(local), Expr::String(arg)) => {
                let value = eval_string_expr(plan, caller_frame, arg)?;
                frame.set_string(local, value);
            }
            (LocalId::Bool(local), Expr::Bool(arg)) => {
                let value = eval_bool_expr(plan, caller_frame, arg)?;
                frame.set_bool(local, value);
            }
            (LocalId::Nil(local), Expr::Nil(arg)) => {
                eval_nil_expr(plan, caller_frame, arg)?;
                frame.set_nil(local);
            }
            _ => invalid_argument_shape(),
        }
    }
    Ok(frame)
}

fn invalid_argument_shape() -> ! {
    panic!("typed ModulePlan call argument shape mismatch")
}

fn invalid_return_shape() -> ! {
    panic!("typed ModulePlan call return shape mismatch")
}

#[cfg(test)]
mod tests {
    use super::super::{Value, int, run_src};
    use super::{run_bool_call, run_int_call, run_nil_call, run_string_call};
    use crate::plan::{
        BoolExpr, Expr, FunctionId, FunctionPlan, IntExpr, IntLocalId, LocalId, ModulePlan, Param,
        StringExpr,
    };
    use crate::runtime::frame::Frame;

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
pub fn main() {
  1 == 1
  5
}
"#,
            ),
            int(5),
        );
    }

    #[test]
    fn execute_function_with_arguments() {
        assert_eq!(
            run_src(
                r#"
fn double(value: Int) {
  value * 2
}

pub fn main() {
  double(3)
}
"#,
            ),
            int(6),
        );
    }

    #[test]
    fn execute_typed_function_arguments() {
        assert_eq!(
            run_src(
                r#"
fn string_id(value: String) {
  value
}

pub fn main() {
  string_id("geam")
}
"#,
            ),
            Value::String("geam".into()),
        );
        assert_eq!(
            run_src(
                r#"
fn invert(value: Bool) {
  !value
}

pub fn main() {
  invert(False)
}
"#,
            ),
            Value::Bool(true),
        );
        assert_eq!(
            run_src(
                r#"
fn identity(value: Nil) {
  value
}

pub fn main() {
  identity(Nil)
}
"#,
            ),
            Value::Nil,
        );
    }

    #[test]
    fn execute_typed_let_bindings() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let name = "ge"
  name <> "am"
}
"#,
            ),
            Value::String("geam".into()),
        );
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let flag = False
  !flag
}
"#,
            ),
            Value::Bool(true),
        );
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let nothing = Nil
  nothing
}
"#,
            ),
            Value::Nil,
        );
    }

    #[test]
    #[should_panic(expected = "typed ModulePlan call argument shape mismatch")]
    fn run_int_call_panics_on_invalid_argument_shape() {
        let plan = ModulePlan {
            module: "main".into(),
            main: FunctionId(0),
            functions: vec![FunctionPlan {
                id: FunctionId(0),
                name: "main".into(),
                params: vec![Param {
                    local: LocalId::Int(IntLocalId(0)),
                    name: "value".into(),
                }],
                steps: vec![],
                return_: Expr::Int(IntExpr::LocalGet {
                    local: IntLocalId(0),
                    name: "value".into(),
                }),
            }],
        };
        let mut frame = Frame::default();

        let _ = run_int_call(
            &plan,
            FunctionId(0),
            &[Expr::String(StringExpr::Value("bad".into()))],
            &mut frame,
        );
    }

    #[test]
    #[should_panic(expected = "typed ModulePlan call return shape mismatch")]
    fn run_int_call_panics_on_invalid_return_shape() {
        let plan = ModulePlan {
            module: "main".into(),
            main: FunctionId(0),
            functions: vec![FunctionPlan {
                id: FunctionId(0),
                name: "main".into(),
                params: vec![],
                steps: vec![],
                return_: Expr::Bool(BoolExpr::Value(true)),
            }],
        };
        let mut frame = Frame::default();

        let _ = run_int_call(&plan, FunctionId(0), &[], &mut frame);
    }

    #[test]
    #[should_panic(expected = "typed ModulePlan call return shape mismatch")]
    fn run_string_call_panics_on_invalid_return_shape() {
        let plan = invalid_int_return_plan();
        let mut frame = Frame::default();

        let _ = run_string_call(&plan, FunctionId(0), &[], &mut frame);
    }

    #[test]
    #[should_panic(expected = "typed ModulePlan call return shape mismatch")]
    fn run_bool_call_panics_on_invalid_return_shape() {
        let plan = invalid_int_return_plan();
        let mut frame = Frame::default();

        let _ = run_bool_call(&plan, FunctionId(0), &[], &mut frame);
    }

    #[test]
    #[should_panic(expected = "typed ModulePlan call return shape mismatch")]
    fn run_nil_call_panics_on_invalid_return_shape() {
        let plan = invalid_int_return_plan();
        let mut frame = Frame::default();

        let _ = run_nil_call(&plan, FunctionId(0), &[], &mut frame);
    }

    fn invalid_int_return_plan() -> ModulePlan {
        ModulePlan {
            module: "main".into(),
            main: FunctionId(0),
            functions: vec![FunctionPlan {
                id: FunctionId(0),
                name: "main".into(),
                params: vec![],
                steps: vec![],
                return_: Expr::Int(IntExpr::Value(1.into())),
            }],
        }
    }
}
