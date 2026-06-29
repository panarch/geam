use geam::{Value, compile_typed_module, plan_module, run_main};
use num_bigint::BigInt;

macro_rules! execution_case {
    ($name:ident, $fixture:literal) => {
        #[test]
        fn $name() {
            run_fixture($fixture);
        }
    };
}

macro_rules! rejection_case {
    ($name:ident, $fixture:literal) => {
        #[test]
        fn $name() {
            reject_fixture($fixture);
        }
    };
}

execution_case!(integer_return, "integer_return.gleam");
execution_case!(integer_arithmetic, "integer_arithmetic.gleam");
execution_case!(integer_comparison, "integer_comparison.gleam");
execution_case!(integer_division, "integer_division.gleam");
execution_case!(let_binding, "let_binding.gleam");
execution_case!(local_function_call, "local_function_call.gleam");
execution_case!(string_concatenation, "string_concatenation.gleam");
execution_case!(bool_value, "bool_value.gleam");
execution_case!(bool_operators, "bool_operators.gleam");
execution_case!(bool_case, "bool_case.gleam");
execution_case!(bool_case_fallback, "bool_case_fallback.gleam");
execution_case!(int_case, "int_case.gleam");
execution_case!(block_expression, "block_expression.gleam");
execution_case!(pipeline, "pipeline.gleam");
execution_case!(function_after_main, "function_after_main.gleam");
execution_case!(function_value_local, "function_value_local.gleam");
execution_case!(
    function_value_argument_callback,
    "function_value_argument_callback.gleam"
);
execution_case!(
    function_value_argument_higher_order_alias,
    "function_value_argument_higher_order_alias.gleam"
);
execution_case!(
    function_value_argument_higher_order_return_shapes,
    "function_value_argument_higher_order_return_shapes.gleam"
);
execution_case!(
    function_value_argument_input_shapes,
    "function_value_argument_input_shapes.gleam"
);
execution_case!(
    function_value_argument_local_value,
    "function_value_argument_local_value.gleam"
);
execution_case!(
    function_value_argument_multi_arity,
    "function_value_argument_multi_arity.gleam"
);
execution_case!(
    function_value_argument_return_shapes,
    "function_value_argument_return_shapes.gleam"
);
execution_case!(
    function_returning_function_argument,
    "function_returning_function_argument.gleam"
);
execution_case!(
    function_returning_function_deep,
    "function_returning_function_deep.gleam"
);
execution_case!(
    function_returning_function_direct_shapes,
    "function_returning_function_direct_shapes.gleam"
);
execution_case!(
    function_returning_function_recursive,
    "function_returning_function_recursive.gleam"
);
execution_case!(function_value_shadowing, "function_value_shadowing.gleam");
execution_case!(
    function_value_block_callee,
    "function_value_block_callee.gleam"
);
execution_case!(
    function_value_case_callee,
    "function_value_case_callee.gleam"
);
execution_case!(nil_value, "nil_value.gleam");

rejection_case!(reject_top_level_import, "top_level_import.gleam");
rejection_case!(reject_top_level_constant, "top_level_constant.gleam");
rejection_case!(reject_top_level_custom_type, "top_level_custom_type.gleam");
rejection_case!(reject_top_level_type_alias, "top_level_type_alias.gleam");
rejection_case!(reject_missing_main, "missing_main.gleam");
rejection_case!(reject_main_with_arguments, "main_with_arguments.gleam");
rejection_case!(
    reject_function_unsupported_return_type,
    "function_unsupported_return_type.gleam"
);
rejection_case!(reject_argument_discard, "argument_discard.gleam");
rejection_case!(reject_argument_labelled, "argument_labelled.gleam");
rejection_case!(
    reject_argument_unsupported_type,
    "argument_unsupported_type.gleam"
);
rejection_case!(
    reject_function_before_main_unsupported_body,
    "function_before_main_unsupported_body.gleam"
);
rejection_case!(reject_main_unsupported_body, "main_unsupported_body.gleam");
rejection_case!(
    reject_function_after_main_unsupported_body,
    "function_after_main_unsupported_body.gleam"
);

fn run_fixture(file_name: &str) {
    let path = format!("tests/fixtures/execution/{file_name}");
    let src = std::fs::read_to_string(&path).expect("fixture should be readable");
    let expected = parse_expected_value(&src);
    let module = compile_typed_module("main", path, &src).expect("fixture should compile");
    let plan = plan_module(module).expect("fixture should plan");
    let actual = run_main(&plan);

    assert_eq!(actual, expected);
}

fn reject_fixture(file_name: &str) {
    let path = format!("tests/fixtures/rejection/{file_name}");
    let src = std::fs::read_to_string(&path).expect("fixture should be readable");
    let module = compile_typed_module("main", path, &src).expect("fixture should compile");

    assert!(plan_module(module).is_err());
}

fn parse_expected_value(src: &str) -> Value {
    let line = src
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .expect("fixture should not be empty")
        .trim();
    let Some(value) = line.strip_prefix("// geam:expect ") else {
        panic!("last non-empty fixture line must start with `// geam:expect `");
    };

    if let Some(value) = value.strip_prefix("Int(").and_then(|s| s.strip_suffix(')')) {
        return Value::Int(value.parse::<BigInt>().expect("valid Int expectation"));
    }

    if let Some(value) = value
        .strip_prefix("String(\"")
        .and_then(|s| s.strip_suffix("\")"))
    {
        return Value::String(value.into());
    }

    if let Some(value) = value
        .strip_prefix("Bool(")
        .and_then(|s| s.strip_suffix(')'))
    {
        return Value::Bool(value.parse::<bool>().expect("valid Bool expectation"));
    }

    if value == "Nil" {
        return Value::Nil;
    }

    panic!("unsupported expectation: {value}");
}
