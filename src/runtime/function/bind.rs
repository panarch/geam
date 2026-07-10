use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{CallArg, CallArgKind, CaptureArg, CaptureArgKind, FrameLayout};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_expr, eval_bool_function_expr, eval_bool_list_expr, eval_float_expr,
    eval_float_function_expr, eval_float_list_expr, eval_function_function_expr,
    eval_function_list_expr, eval_int_expr, eval_int_function_expr, eval_int_list_expr,
    eval_list_function_expr, eval_list_list_expr, eval_nil_expr, eval_nil_function_expr,
    eval_nil_list_expr, eval_string_expr, eval_string_function_expr, eval_string_list_expr,
    eval_tuple_expr, eval_tuple_function_expr, eval_tuple_list_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::{CaptureValue, CaptureValueKind, ListLocalValue};

pub(super) fn bind_arguments(
    plan: &ExecutionPlan,
    args: &[CallArg],
    caller_frame: &mut Frame,
    frame_layout: &FrameLayout,
) -> ExecutionResult<Frame> {
    let mut frame = Frame::new(frame_layout);
    bind_arguments_into(plan, args, caller_frame, &mut frame)?;
    Ok(frame)
}

pub(super) fn bind_function_value_arguments(
    plan: &ExecutionPlan,
    args: &[CallArg],
    caller_frame: &mut Frame,
    frame_layout: &FrameLayout,
    captures: &[CaptureValue],
) -> ExecutionResult<Frame> {
    let mut frame = Frame::new(frame_layout);
    bind_captures(&mut frame, captures);
    bind_arguments_into(plan, args, caller_frame, &mut frame)?;
    Ok(frame)
}

fn bind_arguments_into(
    plan: &ExecutionPlan,
    args: &[CallArg],
    caller_frame: &mut Frame,
    frame: &mut Frame,
) -> ExecutionResult<()> {
    for arg in args {
        match arg.kind() {
            CallArgKind::Int { local, value } => {
                let value = eval_int_expr(plan, caller_frame, value)?;
                frame.set_int(*local, value);
            }
            CallArgKind::String { local, value } => {
                let value = eval_string_expr(plan, caller_frame, value)?;
                frame.set_string(*local, value);
            }
            CallArgKind::Float { local, value } => {
                let value = eval_float_expr(plan, caller_frame, value)?;
                frame.set_float(*local, value);
            }
            CallArgKind::Bool { local, value } => {
                let value = eval_bool_expr(plan, caller_frame, value)?;
                frame.set_bool(*local, value);
            }
            CallArgKind::Nil { local, value } => {
                eval_nil_expr(plan, caller_frame, value)?;
                frame.set_nil(*local);
            }
            CallArgKind::Tuple { local, value } => {
                let value = eval_tuple_expr(plan, caller_frame, value)?;
                frame.set_tuple(*local, value);
            }
            CallArgKind::List(value) => bind_list_argument(plan, caller_frame, frame, value)?,
            CallArgKind::IntFunction { local, value } => {
                let value = eval_int_function_expr(plan, caller_frame, value)?;
                frame.set_int_function(*local, value);
            }
            CallArgKind::StringFunction { local, value } => {
                let value = eval_string_function_expr(plan, caller_frame, value)?;
                frame.set_string_function(*local, value);
            }
            CallArgKind::FloatFunction { local, value } => {
                let value = eval_float_function_expr(plan, caller_frame, value)?;
                frame.set_float_function(*local, value);
            }
            CallArgKind::BoolFunction { local, value } => {
                let value = eval_bool_function_expr(plan, caller_frame, value)?;
                frame.set_bool_function(*local, value);
            }
            CallArgKind::NilFunction { local, value } => {
                let value = eval_nil_function_expr(plan, caller_frame, value)?;
                frame.set_nil_function(*local, value);
            }
            CallArgKind::TupleFunction { local, value } => {
                let value = eval_tuple_function_expr(plan, caller_frame, value)?;
                frame.set_tuple_function(*local, value);
            }
            CallArgKind::ListFunction { local, value } => {
                let value = eval_list_function_expr(plan, caller_frame, value)?;
                frame.set_list_function(local.clone(), value);
            }
            CallArgKind::FunctionFunction { local, value } => {
                let value = eval_function_function_expr(plan, caller_frame, value)?;
                frame.set_function_function(*local, value);
            }
        }
    }

    Ok(())
}

pub(in crate::runtime) fn eval_capture_args(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    args: &[CaptureArg],
) -> ExecutionResult<Vec<CaptureValue>> {
    let mut captures = Vec::with_capacity(args.len());
    for arg in args {
        captures.push(match arg.kind() {
            CaptureArgKind::Int { local, value } => {
                CaptureValue::int(*local, eval_int_expr(plan, frame, value)?)
            }
            CaptureArgKind::String { local, value } => {
                CaptureValue::string(*local, eval_string_expr(plan, frame, value)?)
            }
            CaptureArgKind::Float { local, value } => {
                CaptureValue::float(*local, eval_float_expr(plan, frame, value)?)
            }
            CaptureArgKind::Bool { local, value } => {
                CaptureValue::bool(*local, eval_bool_expr(plan, frame, value)?)
            }
            CaptureArgKind::Nil { local, value } => {
                eval_nil_expr(plan, frame, value)?;
                CaptureValue::nil(*local)
            }
            CaptureArgKind::Tuple { local, value } => {
                CaptureValue::tuple(*local, eval_tuple_expr(plan, frame, value)?)
            }
            CaptureArgKind::List(value) => eval_list_capture(plan, frame, value)?,
            CaptureArgKind::IntFunction { local, value } => {
                CaptureValue::int_function(*local, eval_int_function_expr(plan, frame, value)?)
            }
            CaptureArgKind::StringFunction { local, value } => CaptureValue::string_function(
                *local,
                eval_string_function_expr(plan, frame, value)?,
            ),
            CaptureArgKind::FloatFunction { local, value } => {
                CaptureValue::float_function(*local, eval_float_function_expr(plan, frame, value)?)
            }
            CaptureArgKind::BoolFunction { local, value } => {
                CaptureValue::bool_function(*local, eval_bool_function_expr(plan, frame, value)?)
            }
            CaptureArgKind::NilFunction { local, value } => {
                CaptureValue::nil_function(*local, eval_nil_function_expr(plan, frame, value)?)
            }
            CaptureArgKind::TupleFunction { local, value } => {
                CaptureValue::tuple_function(*local, eval_tuple_function_expr(plan, frame, value)?)
            }
            CaptureArgKind::ListFunction { local, value } => CaptureValue::list_function(
                local.clone(),
                eval_list_function_expr(plan, frame, value)?,
            ),
            CaptureArgKind::FunctionFunction { local, value } => CaptureValue::function_function(
                *local,
                eval_function_function_expr(plan, frame, value)?,
            ),
        });
    }

    Ok(captures)
}

fn bind_captures(frame: &mut Frame, captures: &[CaptureValue]) {
    for capture in captures {
        match capture.kind() {
            CaptureValueKind::Int { local, value } => frame.set_int(*local, value.clone()),
            CaptureValueKind::String { local, value } => frame.set_string(*local, value.clone()),
            CaptureValueKind::Float { local, value } => frame.set_float(*local, *value),
            CaptureValueKind::Bool { local, value } => frame.set_bool(*local, *value),
            CaptureValueKind::Nil { local } => frame.set_nil(*local),
            CaptureValueKind::Tuple { local, value } => frame.set_tuple(*local, value.clone()),
            CaptureValueKind::List(value) => bind_list_capture(frame, value),
            CaptureValueKind::IntFunction { local, value } => {
                frame.set_int_function(*local, value.clone());
            }
            CaptureValueKind::StringFunction { local, value } => {
                frame.set_string_function(*local, value.clone());
            }
            CaptureValueKind::FloatFunction { local, value } => {
                frame.set_float_function(*local, value.clone());
            }
            CaptureValueKind::BoolFunction { local, value } => {
                frame.set_bool_function(*local, value.clone());
            }
            CaptureValueKind::NilFunction { local, value } => {
                frame.set_nil_function(*local, value.clone());
            }
            CaptureValueKind::TupleFunction { local, value } => {
                frame.set_tuple_function(*local, value.clone());
            }
            CaptureValueKind::ListFunction { local, value } => {
                frame.set_list_function(local.clone(), value.clone());
            }
            CaptureValueKind::FunctionFunction { local, value } => {
                frame.set_function_function(*local, value.clone());
            }
        }
    }
}

fn bind_list_argument(
    plan: &ExecutionPlan,
    caller_frame: &mut Frame,
    frame: &mut Frame,
    value: &crate::plan::execution::ListLocalExpr,
) -> ExecutionResult<()> {
    match value {
        crate::plan::execution::ListLocalExpr::Int { local, value } => {
            let value = eval_int_list_expr(plan, caller_frame, value)?;
            frame.set_int_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::String { local, value } => {
            let value = eval_string_list_expr(plan, caller_frame, value)?;
            frame.set_string_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Float { local, value } => {
            let value = eval_float_list_expr(plan, caller_frame, value)?;
            frame.set_float_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Bool { local, value } => {
            let value = eval_bool_list_expr(plan, caller_frame, value)?;
            frame.set_bool_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Nil { local, value } => {
            let value = eval_nil_list_expr(plan, caller_frame, value)?;
            frame.set_nil_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Tuple { local, value, .. } => {
            let value = eval_tuple_list_expr(plan, caller_frame, value)?;
            frame.set_tuple_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::List { local, value, .. } => {
            let value = eval_list_list_expr(plan, caller_frame, value)?;
            frame.set_list_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Function { local, value, .. } => {
            let value = eval_function_list_expr(plan, caller_frame, value)?;
            frame.set_function_list(*local, value);
        }
    }
    Ok(())
}

fn eval_list_capture(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    value: &crate::plan::execution::ListLocalExpr,
) -> ExecutionResult<CaptureValue> {
    Ok(match value {
        crate::plan::execution::ListLocalExpr::Int { local, value } => {
            CaptureValue::list(ListLocalValue::Int {
                local: *local,
                value: eval_int_list_expr(plan, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::String { local, value } => {
            CaptureValue::list(ListLocalValue::String {
                local: *local,
                value: eval_string_list_expr(plan, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::Float { local, value } => {
            CaptureValue::list(ListLocalValue::Float {
                local: *local,
                value: eval_float_list_expr(plan, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::Bool { local, value } => {
            CaptureValue::list(ListLocalValue::Bool {
                local: *local,
                value: eval_bool_list_expr(plan, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::Nil { local, value } => {
            CaptureValue::list(ListLocalValue::Nil {
                local: *local,
                len: eval_nil_list_expr(plan, frame, value)?,
            })
        }
        crate::plan::execution::ListLocalExpr::Tuple {
            local,
            item_type,
            value,
        } => CaptureValue::list(ListLocalValue::Tuple {
            local: *local,
            item_type: item_type.clone(),
            value: eval_tuple_list_expr(plan, frame, value)?,
        }),
        crate::plan::execution::ListLocalExpr::List {
            local,
            item_type,
            value,
        } => CaptureValue::list(ListLocalValue::List {
            local: *local,
            item_type: item_type.clone(),
            value: eval_list_list_expr(plan, frame, value)?,
        }),
        crate::plan::execution::ListLocalExpr::Function {
            local,
            item_type,
            value,
        } => CaptureValue::list(ListLocalValue::Function {
            local: *local,
            item_type: item_type.clone(),
            value: eval_function_list_expr(plan, frame, value)?,
        }),
    })
}

fn bind_list_capture(frame: &mut Frame, value: &ListLocalValue) {
    match value {
        ListLocalValue::Int { local, value } => {
            frame.set_int_list(*local, value.clone());
        }
        ListLocalValue::String { local, value } => {
            frame.set_string_list(*local, value.clone());
        }
        ListLocalValue::Float { local, value } => {
            frame.set_float_list(*local, value.clone());
        }
        ListLocalValue::Bool { local, value } => {
            frame.set_bool_list(*local, value.clone());
        }
        ListLocalValue::Nil { local, len } => {
            frame.set_nil_list(*local, *len);
        }
        ListLocalValue::Tuple { local, value, .. } => {
            frame.set_tuple_list(*local, value.clone());
        }
        ListLocalValue::List { local, value, .. } => {
            frame.set_list_list(*local, value.clone());
        }
        ListLocalValue::Function { local, value, .. } => {
            frame.set_function_list(*local, value.clone());
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, BoolFunctionLocalId, BoolListExpr, BoolListLocalId, CaptureArg,
        FloatExpr, FloatFunctionExpr, FloatFunctionLocalId, FloatListExpr, FloatListLocalId,
        FunctionFunctionExpr, FunctionFunctionLocalId, FunctionId, FunctionListLocalId,
        FunctionPlan, FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId, IntFunctionId,
        IntFunctionLocalId, IntListExpr, IntListLocalId, IntLocalId, ListExpr, ListFunctionExpr,
        ListFunctionLocal, ListListExpr, ListListLocalId, ListLocalExpr, ModulePlan, NilExpr,
        NilFunctionExpr, NilFunctionLocalId, NilListExpr, NilListLocalId, NilLocalId, PanicExpr,
        PanicSite, ReturnExpr, StringExpr, StringFunctionExpr, StringFunctionLocalId,
        StringListExpr, StringListLocalId, StringLocalId, TupleExpr, TupleFunctionExpr,
        TupleFunctionLocalId, TupleListExpr, TupleListLocalId, TupleLocalId, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_arguments_and_captures_preserve_every_value_family() {
        let cases = [
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/argument/list_boundary_item_families.gleam"
                ),
                crate::runtime::Value::Int(1.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/anonymous/capturing_closure_value_families.gleam"
                ),
                crate::runtime::Value::Int(42.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/anonymous/capturing_closure_return_shapes.gleam"
                ),
                crate::runtime::Value::Int(42.into()),
            ),
        ];

        for (source, expected) in cases {
            assert_eq!(crate::runtime::run_src(source), expected);
        }

        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/functions/anonymous/capturing_closure_list_function.gleam"
            )),
            crate::runtime::Value::List(crate::runtime::ListValue::int(vec![1.into(), 2.into()])),
        );

        assert_eq!(
            crate::runtime::run_src(
                r#"
fn int_value() { 1 }
fn string_value() { "one" }
fn float_value() { 1.0 }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(1) }
fn list_value() { [1] }
fn function_value() { int_value }

pub fn main() {
  let int = 1
  let string = "one"
  let float = 1.0
  let bool = True
  let nil = Nil
  let tuple = #(1)
  let int_list = [1]
  let string_list = ["one"]
  let float_list = [1.0]
  let bool_list = [True]
  let nil_list = [Nil]
  let tuple_list = [#(1)]
  let list_list = [[1]]
  let function_list = [int_value]
  let int_function = int_value
  let string_function = string_value
  let float_function = float_value
  let bool_function = bool_value
  let nil_function = nil_value
  let tuple_function = tuple_value
  let list_function = list_value
  let function_function = function_value

  let closure = fn() {
    assert int == 1
    assert string == "one"
    assert float == 1.0
    assert bool
    nil
    assert tuple == #(1)
    assert int_list == [1]
    assert string_list == ["one"]
    assert float_list == [1.0]
    assert bool_list == [True]
    assert nil_list == [Nil]
    assert tuple_list == [#(1)]
    assert list_list == [[1]]
    assert case function_list { [function] -> function() == 1 _ -> False }
    assert int_function() == 1
    assert string_function() == "one"
    assert float_function() == 1.0
    assert bool_function()
    nil_function()
    assert tuple_function() == #(1)
    assert list_function() == [1]
    assert function_function()() == 1
    42
  }

  closure()
}
"#,
            ),
            crate::runtime::Value::Int(42.into()),
        );

        assert_eq!(
            crate::runtime::run_src(
                r#"
fn int_value() { 1 }
fn string_value() { "one" }
fn float_value() { 1.0 }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(1) }
fn list_value() { [1] }
fn function_value() { int_value }

fn accept_int(function: fn() -> Int) { function() }
fn accept_string(function: fn() -> String) { function() }
fn accept_float(function: fn() -> Float) { function() }
fn accept_bool(function: fn() -> Bool) { function() }
fn accept_nil(function: fn() -> Nil) { function() }
fn accept_tuple(function: fn() -> #(Int)) { function() }
fn accept_list(function: fn() -> List(Int)) { function() }
fn accept_function(function: fn() -> fn() -> Int) { function()() }

pub fn main() {
  assert accept_int(int_value) == 1
  assert accept_string(string_value) == "one"
  assert accept_float(float_value) == 1.0
  assert accept_bool(bool_value)
  accept_nil(nil_value)
  assert accept_tuple(tuple_value) == #(1)
  assert accept_list(list_value) == [1]
  assert accept_function(function_value) == 1
  42
}
"#,
            ),
            crate::runtime::Value::Int(42.into()),
        );
    }

    #[test]
    fn source_argument_errors_propagate_for_every_value_family() {
        let parameter_types = [
            "Int",
            "String",
            "Float",
            "Bool",
            "Nil",
            "#(Int)",
            "List(Int)",
            "List(String)",
            "List(Float)",
            "List(Bool)",
            "List(Nil)",
            "List(#(Int))",
            "List(List(Int))",
            "List(fn() -> Int)",
            "fn() -> Int",
            "fn() -> String",
            "fn() -> Float",
            "fn() -> Bool",
            "fn() -> Nil",
            "fn() -> #(Int)",
            "fn() -> List(Int)",
            "fn() -> fn() -> Int",
        ];

        for parameter_type in parameter_types {
            let source = format!(
                "fn callee(value: {parameter_type}) -> Nil {{ Nil }} pub fn main() {{ callee(panic as \"argument\") }}",
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: argument",
            );
        }
    }

    #[test]
    fn module_capture_errors_propagate_for_every_value_family() {
        let panic = || {
            PanicExpr::panic_at(
                Some(StringExpr::value("capture".into())),
                PanicSite::unknown(),
            )
        };
        let function_type = |return_: ValueType| FunctionType::new(Vec::new(), return_);
        let int_function_type = function_type(ValueType::Int);
        let list_function_type = function_type(ValueType::List(Box::new(ValueType::Int)));
        let function_function_type =
            function_type(ValueType::Function(Box::new(int_function_type.clone())));
        let captures = [
            CaptureArg::int(IntLocalId(0), IntExpr::panic(panic())),
            CaptureArg::string(StringLocalId(0), StringExpr::panic(panic())),
            CaptureArg::float(crate::plan::FloatLocalId(0), FloatExpr::panic(panic())),
            CaptureArg::bool(crate::plan::BoolLocalId(0), BoolExpr::panic(panic())),
            CaptureArg::nil(NilLocalId(0), NilExpr::panic(panic())),
            CaptureArg::tuple(
                TupleLocalId(0),
                TupleExpr::panic(panic(), vec![ValueType::Int]),
            ),
            CaptureArg::list(ListLocalExpr::Int {
                local: IntListLocalId(0),
                value: IntListExpr::from(ListExpr::panic(panic(), ValueType::Int)),
            }),
            CaptureArg::list(ListLocalExpr::String {
                local: StringListLocalId(0),
                value: StringListExpr::from(ListExpr::panic(panic(), ValueType::String)),
            }),
            CaptureArg::list(ListLocalExpr::Float {
                local: FloatListLocalId(0),
                value: FloatListExpr::from(ListExpr::panic(panic(), ValueType::Float)),
            }),
            CaptureArg::list(ListLocalExpr::Bool {
                local: BoolListLocalId(0),
                value: BoolListExpr::from(ListExpr::panic(panic(), ValueType::Bool)),
            }),
            CaptureArg::list(ListLocalExpr::Nil {
                local: NilListLocalId(0),
                value: NilListExpr::from(ListExpr::panic(panic(), ValueType::Nil)),
            }),
            CaptureArg::list(ListLocalExpr::Tuple {
                local: TupleListLocalId(0),
                item_type: vec![ValueType::Int],
                value: TupleListExpr::from(ListExpr::panic(
                    panic(),
                    ValueType::Tuple(vec![ValueType::Int]),
                )),
            }),
            CaptureArg::list(ListLocalExpr::List {
                local: ListListLocalId(0),
                item_type: Box::new(ValueType::Int),
                value: ListListExpr::from(ListExpr::panic(
                    panic(),
                    ValueType::List(Box::new(ValueType::Int)),
                )),
            }),
            CaptureArg::list(ListLocalExpr::Function {
                local: FunctionListLocalId(0),
                item_type: int_function_type.clone(),
                value: crate::plan::FunctionListExpr::from(ListExpr::panic(
                    panic(),
                    ValueType::Function(Box::new(int_function_type.clone())),
                )),
            }),
            CaptureArg::int_function(
                IntFunctionLocalId(0),
                IntFunctionExpr::panic(panic(), int_function_type.clone()),
            ),
            CaptureArg::string_function(
                StringFunctionLocalId(0),
                StringFunctionExpr::panic(panic(), function_type(ValueType::String)),
            ),
            CaptureArg::float_function(
                FloatFunctionLocalId(0),
                FloatFunctionExpr::panic(panic(), function_type(ValueType::Float)),
            ),
            CaptureArg::bool_function(
                BoolFunctionLocalId(0),
                BoolFunctionExpr::panic(panic(), function_type(ValueType::Bool)),
            ),
            CaptureArg::nil_function(
                NilFunctionLocalId(0),
                NilFunctionExpr::panic(panic(), function_type(ValueType::Nil)),
            ),
            CaptureArg::tuple_function(
                TupleFunctionLocalId(0),
                TupleFunctionExpr::panic(
                    panic(),
                    function_type(ValueType::Tuple(vec![ValueType::Int])),
                ),
            ),
            CaptureArg::list_function(
                ListFunctionLocal::from_item_type(0, list_function_type.clone(), ValueType::Int),
                ListFunctionExpr::panic(panic(), list_function_type, ValueType::Int),
            ),
            CaptureArg::function_function(
                FunctionFunctionLocalId(0),
                FunctionFunctionExpr::panic(panic(), function_function_type),
            ),
        ];

        for capture in captures {
            assert_eq!(run_module_capture(capture).to_string(), "panic: capture",);
        }
    }

    fn run_module_capture(capture: CaptureArg) -> ExecutionError {
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let expression =
            IntFunctionExpr::closure(IntFunctionId(1), Vec::new(), vec![capture], function_type);
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int_function(IntFunctionFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("capture expression should fail at runtime")
    }
}
