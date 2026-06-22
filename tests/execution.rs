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

execution_case!(integer_return, "integer_return.gleam");
execution_case!(integer_arithmetic, "integer_arithmetic.gleam");
execution_case!(integer_comparison, "integer_comparison.gleam");
execution_case!(integer_division, "integer_division.gleam");
execution_case!(let_binding, "let_binding.gleam");
execution_case!(local_function_call, "local_function_call.gleam");
execution_case!(string_concatenation, "string_concatenation.gleam");
execution_case!(bool_value, "bool_value.gleam");
execution_case!(nil_value, "nil_value.gleam");

fn run_fixture(file_name: &str) {
    let path = format!("tests/fixtures/execution/{file_name}");
    let src = std::fs::read_to_string(&path).expect("fixture should be readable");
    let expected = parse_expected_value(&src);
    let module = compile_typed_module("main", path, &src).expect("fixture should compile");
    let plan = plan_module(module).expect("fixture should plan");
    let actual = run_main(&plan).expect("fixture should run");

    assert_eq!(actual, expected);
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
