use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{
    AssertBinding, AssertPattern, BoolFunctionLocalId, BoolLocalId, FloatFunctionLocalId,
    FloatLocalId, FunctionFunctionLocalId, IntFunctionLocalId, IntLocalId, ListAssertPattern,
    ListAssertTail, ListFunctionLocal, NilFunctionLocalId, NilLocalId, ParamLocal, StepKind,
    StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
};
use crate::runtime::error::ExecutionResult;
use crate::runtime::expression::{
    eval_bool_expr, eval_bool_function_expr, eval_bool_list_expr, eval_expr, eval_float_expr,
    eval_float_function_expr, eval_float_list_expr, eval_function_function_expr,
    eval_function_list_expr, eval_int_expr, eval_int_function_expr, eval_int_list_expr,
    eval_list_function_expr, eval_list_list_expr, eval_nil_expr, eval_nil_function_expr,
    eval_nil_list_expr, eval_string_expr, eval_string_function_expr, eval_string_list_expr,
    eval_tuple_expr, eval_tuple_function_expr, eval_tuple_list_expr, get_list_value,
};
use crate::runtime::frame::Frame;
use crate::runtime::{
    BoolFunctionValue, FloatFunctionValue, FunctionFunctionValue, FunctionValue, FunctionValueKind,
    IntFunctionValue, ListFunctionValue, ListLocalValue, ListValue, NilFunctionValue,
    StringFunctionValue, TupleFunctionValue, Value,
};
use crate::runtime::{ExecutionError, PanicKind};
use ecow::EcoString;
use num_bigint::BigInt;

pub(in crate::runtime) fn execute_steps(
    plan: &ExecutionPlan,
    steps: &[crate::plan::execution::Step],
    frame: &mut Frame,
) -> ExecutionResult<()> {
    for step in steps {
        match step.kind() {
            StepKind::LetInt { local, value, .. } => {
                let value = eval_int_expr(plan, frame, value)?;
                frame.set_int(*local, value);
            }
            StepKind::LetString { local, value, .. } => {
                let value = eval_string_expr(plan, frame, value)?;
                frame.set_string(*local, value);
            }
            StepKind::LetFloat { local, value, .. } => {
                let value = eval_float_expr(plan, frame, value)?;
                frame.set_float(*local, value);
            }
            StepKind::LetBool { local, value, .. } => {
                let value = eval_bool_expr(plan, frame, value)?;
                frame.set_bool(*local, value);
            }
            StepKind::LetNil { local, value, .. } => {
                eval_nil_expr(plan, frame, value)?;
                frame.set_nil(*local);
            }
            StepKind::LetTuple { local, value, .. } => {
                let value = eval_tuple_expr(plan, frame, value)?;
                frame.set_tuple(*local, value);
            }
            StepKind::LetList { value, .. } => execute_let_list(plan, frame, value)?,
            StepKind::LetIntFunction { local, value, .. } => {
                let value = eval_int_function_expr(plan, frame, value)?;
                frame.set_int_function(*local, value);
            }
            StepKind::LetStringFunction { local, value, .. } => {
                let value = eval_string_function_expr(plan, frame, value)?;
                frame.set_string_function(*local, value);
            }
            StepKind::LetFloatFunction { local, value, .. } => {
                let value = eval_float_function_expr(plan, frame, value)?;
                frame.set_float_function(*local, value);
            }
            StepKind::LetBoolFunction { local, value, .. } => {
                let value = eval_bool_function_expr(plan, frame, value)?;
                frame.set_bool_function(*local, value);
            }
            StepKind::LetNilFunction { local, value, .. } => {
                let value = eval_nil_function_expr(plan, frame, value)?;
                frame.set_nil_function(*local, value);
            }
            StepKind::LetTupleFunction { local, value, .. } => {
                let value = eval_tuple_function_expr(plan, frame, value)?;
                frame.set_tuple_function(*local, value);
            }
            StepKind::LetListFunction { local, value, .. } => {
                let value = eval_list_function_expr(plan, frame, value)?;
                frame.set_list_function(local.clone(), value);
            }
            StepKind::LetFunctionFunction { local, value, .. } => {
                let value = eval_function_function_expr(plan, frame, value)?;
                frame.set_function_function(*local, value);
            }
            StepKind::AssertList {
                local,
                pattern,
                message,
                site,
                pattern_span,
            } => {
                let value = get_list_value(frame, local);
                let mut bindings = Vec::new();
                if match_assert_pattern(pattern, &Value::List(value.clone()), &mut bindings)
                    .is_none()
                {
                    let message = match message {
                        Some(message) => Some(eval_string_expr(plan, frame, message)?),
                        None => None,
                    };
                    return Err(ExecutionError::let_assert_panic(
                        plan.source_context(),
                        message,
                        site.clone(),
                        Value::List(value),
                        *pattern_span,
                    ));
                }
                for binding in bindings {
                    frame_set_binding(frame, binding);
                }
            }
            StepKind::AssertBool {
                condition,
                message,
                site,
            } => {
                let message = match message {
                    Some(message) => Some(eval_string_expr(plan, frame, message)?),
                    None => None,
                };
                if !eval_bool_expr(plan, frame, condition)? {
                    return Err(ExecutionError::source_panic(
                        plan.source_context(),
                        PanicKind::Assert,
                        message,
                        site.clone(),
                    ));
                }
            }
            StepKind::Evaluate(expression) => {
                let _ = eval_expr(plan, frame, expression)?;
            }
        }
    }

    Ok(())
}

#[derive(Debug, Clone, PartialEq)]
enum PendingBinding {
    Int(IntLocalId, BigInt),
    Float(FloatLocalId, f64),
    String(StringLocalId, EcoString),
    Bool(BoolLocalId, bool),
    Nil(NilLocalId),
    Tuple(TupleLocalId, Vec<Value>),
    List(ListLocalValue),
    IntFunction(IntFunctionLocalId, IntFunctionValue),
    FloatFunction(FloatFunctionLocalId, FloatFunctionValue),
    StringFunction(StringFunctionLocalId, StringFunctionValue),
    BoolFunction(BoolFunctionLocalId, BoolFunctionValue),
    NilFunction(NilFunctionLocalId, NilFunctionValue),
    TupleFunction(TupleFunctionLocalId, TupleFunctionValue),
    ListFunction(ListFunctionLocal, ListFunctionValue),
    FunctionFunction(FunctionFunctionLocalId, FunctionFunctionValue),
}

fn match_list_assert_pattern(
    pattern: &ListAssertPattern,
    value: &ListValue,
) -> Option<Vec<PendingBinding>> {
    let values = value.to_values();
    if let Some(tail) = pattern.tail() {
        if values.len() < pattern.elements().len() {
            return None;
        }

        let mut bindings = match_prefix_assert_patterns(pattern.elements(), &values)?;
        if let ListAssertTail::Bind(binding) = tail {
            bindings.push(PendingBinding::List(ListLocalValue::try_new(
                binding.local().clone(),
                value.drop_first(pattern.elements().len()),
            )?));
        }
        Some(bindings)
    } else {
        if values.len() != pattern.elements().len() {
            return None;
        }

        match_prefix_assert_patterns(pattern.elements(), &values)
    }
}

fn match_prefix_assert_patterns(
    patterns: &[AssertPattern],
    values: &[Value],
) -> Option<Vec<PendingBinding>> {
    let mut bindings = Vec::new();
    for (pattern, value) in patterns.iter().zip(values) {
        match_assert_pattern(pattern, value, &mut bindings)?;
    }
    Some(bindings)
}

fn match_assert_pattern(
    pattern: &AssertPattern,
    value: &Value,
    bindings: &mut Vec<PendingBinding>,
) -> Option<()> {
    match pattern {
        AssertPattern::Bind(binding) => {
            bindings.push(pending_binding(binding, value)?);
            Some(())
        }
        AssertPattern::Discard => Some(()),
        AssertPattern::Tuple(patterns) => {
            let Value::Tuple(values) = value else {
                return None;
            };
            if patterns.len() != values.len() {
                return None;
            }
            for (pattern, value) in patterns.iter().zip(values) {
                match_assert_pattern(pattern, value, bindings)?;
            }
            Some(())
        }
        AssertPattern::List(pattern) => {
            let Value::List(value) = value else {
                return None;
            };
            bindings.extend(match_list_assert_pattern(pattern, value)?);
            Some(())
        }
        AssertPattern::Alias { pattern, binding } => {
            match_assert_pattern(pattern, value, bindings)?;
            bindings.push(pending_binding(binding, value)?);
            Some(())
        }
    }
}

fn pending_binding(target: &AssertBinding, value: &Value) -> Option<PendingBinding> {
    match (target.local(), value) {
        (ParamLocal::Int(local), Value::Int(value)) => {
            Some(PendingBinding::Int(*local, value.clone()))
        }
        (ParamLocal::Float(local), Value::Float(value)) => {
            Some(PendingBinding::Float(*local, *value))
        }
        (ParamLocal::String(local), Value::String(value)) => {
            Some(PendingBinding::String(*local, value.clone()))
        }
        (ParamLocal::Bool(local), Value::Bool(value)) => Some(PendingBinding::Bool(*local, *value)),
        (ParamLocal::Nil(local), Value::Nil) => Some(PendingBinding::Nil(*local)),
        (ParamLocal::Tuple { local, .. }, Value::Tuple(value))
            if target.local().value_type()
                == ValueType::Tuple(value.iter().map(Value::value_type).collect()) =>
        {
            Some(PendingBinding::Tuple(*local, value.clone()))
        }
        (ParamLocal::List(local), Value::List(value)) => {
            ListLocalValue::try_new(local.clone(), value.clone()).map(PendingBinding::List)
        }
        (_, Value::Function(value)) => pending_function_binding(target.local(), value),
        _ => None,
    }
}

fn pending_function_binding(target: &ParamLocal, value: &FunctionValue) -> Option<PendingBinding> {
    if target.value_type() != ValueType::Function(Box::new(value.type_())) {
        return None;
    }

    match (target, value.kind()) {
        (ParamLocal::IntFunction { local, .. }, FunctionValueKind::Int(value)) => {
            Some(PendingBinding::IntFunction(*local, value.clone()))
        }
        (ParamLocal::FloatFunction { local, .. }, FunctionValueKind::Float(value)) => {
            Some(PendingBinding::FloatFunction(*local, value.clone()))
        }
        (ParamLocal::StringFunction { local, .. }, FunctionValueKind::String(value)) => {
            Some(PendingBinding::StringFunction(*local, value.clone()))
        }
        (ParamLocal::BoolFunction { local, .. }, FunctionValueKind::Bool(value)) => {
            Some(PendingBinding::BoolFunction(*local, value.clone()))
        }
        (ParamLocal::NilFunction { local, .. }, FunctionValueKind::Nil(value)) => {
            Some(PendingBinding::NilFunction(*local, value.clone()))
        }
        (ParamLocal::TupleFunction { local, .. }, FunctionValueKind::Tuple(value)) => {
            Some(PendingBinding::TupleFunction(*local, value.clone()))
        }
        (ParamLocal::ListFunction(local), FunctionValueKind::List(value)) => {
            Some(PendingBinding::ListFunction(local.clone(), value.clone()))
        }
        (ParamLocal::FunctionFunction { local, .. }, FunctionValueKind::Function(value)) => {
            Some(PendingBinding::FunctionFunction(*local, value.clone()))
        }
        _ => None,
    }
}

fn frame_set_binding(frame: &mut Frame, binding: PendingBinding) {
    match binding {
        PendingBinding::Int(local, value) => frame.set_int(local, value),
        PendingBinding::Float(local, value) => frame.set_float(local, value),
        PendingBinding::String(local, value) => frame.set_string(local, value),
        PendingBinding::Bool(local, value) => frame.set_bool(local, value),
        PendingBinding::Nil(local) => frame.set_nil(local),
        PendingBinding::Tuple(local, value) => frame.set_tuple(local, value),
        PendingBinding::List(value) => frame_set_list_binding(frame, value),
        PendingBinding::IntFunction(local, value) => frame.set_int_function(local, value),
        PendingBinding::FloatFunction(local, value) => frame.set_float_function(local, value),
        PendingBinding::StringFunction(local, value) => frame.set_string_function(local, value),
        PendingBinding::BoolFunction(local, value) => frame.set_bool_function(local, value),
        PendingBinding::NilFunction(local, value) => frame.set_nil_function(local, value),
        PendingBinding::TupleFunction(local, value) => frame.set_tuple_function(local, value),
        PendingBinding::ListFunction(local, value) => frame.set_list_function(local, value),
        PendingBinding::FunctionFunction(local, value) => {
            frame.set_function_function(local, value);
        }
    }
}

fn execute_let_list(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    value: &crate::plan::execution::ListLocalExpr,
) -> ExecutionResult<()> {
    match value {
        crate::plan::execution::ListLocalExpr::Int { local, value } => {
            let value = eval_int_list_expr(plan, frame, value)?;
            frame.set_int_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::String { local, value } => {
            let value = eval_string_list_expr(plan, frame, value)?;
            frame.set_string_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Float { local, value } => {
            let value = eval_float_list_expr(plan, frame, value)?;
            frame.set_float_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Bool { local, value } => {
            let value = eval_bool_list_expr(plan, frame, value)?;
            frame.set_bool_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Nil { local, value } => {
            let value = eval_nil_list_expr(plan, frame, value)?;
            frame.set_nil_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Tuple { local, value, .. } => {
            let value = eval_tuple_list_expr(plan, frame, value)?;
            frame.set_tuple_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::List { local, value, .. } => {
            let value = eval_list_list_expr(plan, frame, value)?;
            frame.set_list_list(*local, value);
        }
        crate::plan::execution::ListLocalExpr::Function { local, value, .. } => {
            let value = eval_function_list_expr(plan, frame, value)?;
            frame.set_function_list(*local, value);
        }
    }
    Ok(())
}

fn frame_set_list_binding(frame: &mut Frame, value: ListLocalValue) {
    match value {
        ListLocalValue::Int { local, value } => frame.set_int_list(local, value),
        ListLocalValue::String { local, value } => frame.set_string_list(local, value),
        ListLocalValue::Float { local, value } => frame.set_float_list(local, value),
        ListLocalValue::Bool { local, value } => frame.set_bool_list(local, value),
        ListLocalValue::Nil { local, len } => frame.set_nil_list(local, len),
        ListLocalValue::Tuple { local, value, .. } => frame.set_tuple_list(local, value),
        ListLocalValue::List { local, value, .. } => frame.set_list_list(local, value),
        ListLocalValue::Function { local, value, .. } => frame.set_function_list(local, value),
    }
}

#[cfg(test)]
mod tests {
    use super::{PendingBinding, match_assert_pattern, match_list_assert_pattern};
    use crate::plan::execution::{
        AssertBinding, AssertPattern, FunctionFunctionId, IntFunctionFunctionId, IntFunctionId,
        IntListFunctionId, IntListLocalId, IntLocalId, ListAssertPattern, ListAssertTail,
        ListLocal, ParamLocal, RuntimeFunctionId, StepKind,
    };
    use crate::plan::{FunctionType, ValueType};
    use crate::runtime::{FunctionFunctionValue, FunctionValue, ListLocalValue, ListValue, Value};

    #[test]
    fn source_steps_bind_and_assert_exact_values() {
        let cases = [
            (
                include_str!(
                    "../../../tests/fixtures/execution/values/list_expression_item_families.gleam"
                ),
                crate::runtime::Value::Int(42.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/functions/anonymous/capturing_closure_return_shapes.gleam"
                ),
                crate::runtime::Value::Int(42.into()),
            ),
            (
                include_str!("../../../tests/fixtures/execution/bindings/expression_steps.gleam"),
                crate::runtime::Value::Int(5.into()),
            ),
            (
                include_str!("../../../tests/fixtures/execution/statements/assert_statement.gleam"),
                crate::runtime::Value::Int(1.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/let_assert_list_destructuring.gleam"
                ),
                crate::runtime::Value::Bool(true),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/let_assert_fixed_list.gleam"
                ),
                crate::runtime::Value::Int(3.into()),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/let_assert_empty_list.gleam"
                ),
                crate::runtime::Value::List(ListValue::int(Vec::new())),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/nested_pattern_alias_assignment.gleam"
                ),
                crate::runtime::Value::Bool(true),
            ),
            (
                include_str!(
                    "../../../tests/fixtures/execution/bindings/let_assert_discard_alias.gleam"
                ),
                crate::runtime::Value::Bool(true),
            ),
        ];

        for (source, expected) in cases {
            assert_eq!(crate::runtime::run_src(source), expected);
        }
    }

    #[test]
    fn let_assert_binds_every_function_return_family() {
        let function_shapes = [
            ("Int", "1"),
            ("String", "\"one\""),
            ("Float", "1.0"),
            ("Bool", "True"),
            ("Nil", "Nil"),
            ("#(Int)", "#(1)"),
            ("List(Int)", "[1]"),
            ("fn() -> Int", "fn() { 1 }"),
        ];

        for (return_type, return_value) in function_shapes {
            let source = format!(
                r#"
fn target() -> {return_type} {{ {return_value} }}
pub fn main() {{
  let assert [function] = [target]
  let _ = function()
  42
}}
"#,
            );

            assert_eq!(
                crate::runtime::run_src(&source),
                crate::runtime::Value::Int(42.into()),
            );
        }
    }

    #[test]
    fn source_assert_steps_return_default_and_explicit_panics() {
        let cases = [
            (
                "pub fn main() { let values: List(Int) = [] let assert [first] = values first }",
                "let_assert: Pattern match failed, no pattern matched the value.",
            ),
            (
                "pub fn main() { let values: List(Int) = [] let assert [first] = values as \"missing\" first }",
                "let_assert: missing",
            ),
            (
                "pub fn main() { assert False Nil }",
                "assert: Assertion failed.",
            ),
            (
                "pub fn main() { assert False as \"checked\" Nil }",
                "assert: checked",
            ),
            (
                "fn fail_message() -> String { panic as \"message\" } pub fn main() { assert True as fail_message() Nil }",
                "panic: message",
            ),
            (
                "fn fail_condition() -> Bool { panic as \"condition\" } pub fn main() { assert fail_condition() as \"checked\" Nil }",
                "panic: condition",
            ),
        ];

        for (source, expected) in cases {
            assert_eq!(crate::runtime::run_src_error(source).to_string(), expected);
        }
    }

    #[test]
    fn source_let_errors_propagate_for_every_value_family() {
        let value_types = [
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

        for value_type in value_types {
            let source = format!(
                "pub fn main() {{ let value: {value_type} = panic as \"step\" let _ = value Nil }}",
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: step",
            );
        }
    }

    #[test]
    fn let_assert_message_errors_propagate_after_mismatch() {
        let source = r#"
fn fail_message() -> String { panic as "message" }
pub fn main() {
  let values: List(Int) = []
  let assert [first, ..] = values as fail_message()
  first
}
"#;

        assert_eq!(
            crate::runtime::run_src_error(source).to_string(),
            "panic: message",
        );
    }

    #[test]
    fn let_assert_matcher_rejects_direct_mutated_value_shapes_without_bindings() {
        let tuple_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [#(left, right)] = [#(1, 2)]
  left + right
}
"#,
        );
        let function = tuple_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let tuple_pattern = &expect_list_assert_pattern(pattern).elements()[0];
        let mut bindings = Vec::new();
        assert_eq!(
            match_assert_pattern(tuple_pattern, &Value::Int(1.into()), &mut bindings),
            None,
        );
        assert_eq!(bindings, Vec::new());
        assert_eq!(
            match_assert_pattern(
                tuple_pattern,
                &Value::Tuple(vec![Value::Int(1.into())]),
                &mut bindings,
            ),
            None,
        );
        assert_eq!(bindings, Vec::new());

        let list_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [[value]] = [[1]]
  value
}
"#,
        );
        let function = list_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let nested_pattern = &expect_list_assert_pattern(pattern).elements()[0];
        assert_eq!(
            match_assert_pattern(nested_pattern, &Value::Int(1.into()), &mut bindings),
            None,
        );
        assert_eq!(bindings, Vec::new());

        let binding_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [value] = [1]
  value
}
"#,
        );
        let function = binding_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let binding = &expect_list_assert_pattern(pattern).elements()[0];
        assert_eq!(
            match_assert_pattern(binding, &Value::String("wrong".into()), &mut bindings),
            None,
        );
        assert_eq!(bindings, Vec::new());
    }

    #[test]
    fn let_assert_tail_and_function_bindings_reject_mismatched_families() {
        let tail_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [..rest] = [1]
  rest
}
"#,
        );
        let function = tail_plan.int_list_function(IntListFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let list_pattern = expect_list_assert_pattern(pattern);
        assert_eq!(
            match_list_assert_pattern(list_pattern, &ListValue::string(Vec::new())),
            None,
        );

        let prefix_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [first, ..rest] = [1]
  let _ = rest
  first
}
"#,
        );
        let function = prefix_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let list_pattern = expect_list_assert_pattern(pattern);
        assert_eq!(
            match_list_assert_pattern(list_pattern, &ListValue::string(vec!["wrong".into()]),),
            None,
        );

        let function_plan = crate::runtime::plan_src(
            r#"
fn target() { 1 }
pub fn main() {
  let assert [function] = [target]
  function()
}
"#,
        );
        let function = function_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let binding = &expect_list_assert_pattern(pattern).elements()[0];
        let wrong_type = FunctionValue::new(
            RuntimeFunctionId::String(crate::plan::execution::StringFunctionId(0)),
            Vec::new(),
        );
        let mut bindings = Vec::new();
        assert_eq!(
            match_assert_pattern(binding, &Value::Function(wrong_type), &mut bindings),
            None,
        );
        assert_eq!(bindings, Vec::new());

        let wrong_kind = FunctionValue::from(FunctionFunctionValue::from_evaluated(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        ));
        assert_eq!(
            match_assert_pattern(binding, &Value::Function(wrong_kind), &mut bindings),
            None,
        );
        assert_eq!(bindings, Vec::new());
    }

    #[test]
    fn list_assert_tail_binding_preserves_the_typed_local_and_value() {
        let pattern = ListAssertPattern::new(
            vec![AssertPattern::Bind(AssertBinding::new(ParamLocal::Int(
                IntLocalId(0),
            )))],
            Some(ListAssertTail::bind(ListLocal::Int(IntListLocalId(0)))),
        );
        let ignored_tail = ListAssertPattern::new(
            vec![AssertPattern::Bind(AssertBinding::new(ParamLocal::Int(
                IntLocalId(0),
            )))],
            Some(ListAssertTail::Ignore),
        );
        let value = ListValue::int(vec![1.into(), 2.into(), 3.into()]);

        assert_eq!(
            match_list_assert_pattern(&pattern, &value),
            Some(vec![
                PendingBinding::Int(IntLocalId(0), 1.into()),
                PendingBinding::List(ListLocalValue::Int {
                    local: IntListLocalId(0),
                    value: vec![2.into(), 3.into()],
                }),
            ]),
        );
        assert_eq!(
            match_list_assert_pattern(&ignored_tail, &value),
            Some(vec![PendingBinding::Int(IntLocalId(0), 1.into())]),
        );
    }

    #[test]
    fn nested_and_alias_assert_patterns_propagate_binding_mismatches() {
        let nested_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [#(left, right)] = [#(1, 2)]
  left + right
}
"#,
        );
        let function = nested_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let tuple_pattern = &expect_list_assert_pattern(pattern).elements()[0];
        let mut bindings = Vec::new();
        assert_eq!(
            match_assert_pattern(
                tuple_pattern,
                &Value::Tuple(vec![Value::Int(1.into()), Value::String("wrong".into())]),
                &mut bindings,
            ),
            None,
        );
        assert_eq!(bindings.len(), 1);

        let alias_plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [#(left, right) as pair] = [#(1, 2)]
  pair.0 + left + right
}
"#,
        );
        let function = alias_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let alias_pattern = &expect_list_assert_pattern(pattern).elements()[0];
        bindings.clear();
        assert_eq!(
            match_assert_pattern(alias_pattern, &Value::Int(1.into()), &mut bindings),
            None,
        );
        assert_eq!(bindings, Vec::new());

        let function_alias_plan = crate::runtime::plan_src(
            r#"
fn target() { 1 }
pub fn main() {
  let assert [_ as function] = [target]
  function()
}
"#,
        );
        let function = function_alias_plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let alias_pattern = &expect_list_assert_pattern(pattern).elements()[0];
        let wrong_kind = FunctionValue::from(FunctionFunctionValue::from_evaluated(
            FunctionFunctionId::Int(IntFunctionFunctionId(0)),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        ));
        assert_eq!(
            match_assert_pattern(alias_pattern, &Value::Function(wrong_kind), &mut bindings,),
            None,
        );
        assert_eq!(bindings, Vec::new());
    }

    #[test]
    #[should_panic(expected = "expected a list assert pattern")]
    fn list_assert_pattern_shape_guard_rejects_tuple_patterns() {
        let plan = crate::runtime::plan_src(
            r#"
pub fn main() {
  let assert [#(left, right)] = [#(1, 2)]
  left + right
}
"#,
        );
        let function = plan.int_function(IntFunctionId(0));
        let pattern = function
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::AssertList { pattern, .. } => Some(pattern),
                _ => None,
            })
            .expect("source should lower an assert-list step");
        let tuple_pattern = &expect_list_assert_pattern(pattern).elements()[0];

        let _ = expect_list_assert_pattern(tuple_pattern);
    }

    fn expect_list_assert_pattern(pattern: &AssertPattern) -> &ListAssertPattern {
        match pattern {
            AssertPattern::List(pattern) => pattern,
            _ => panic!("expected a list assert pattern"),
        }
    }
}
