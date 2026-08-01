use super::super::FunctionValue;

pub(super) fn write(output: &mut String, value: &FunctionValue) {
    output.push_str("//fn(");
    for index in 0..value.type_().argument_types().len() {
        if index != 0 {
            output.push_str(", ");
        }
        write_argument_name(output, index);
    }
    output.push_str(") { ... }");
}

fn write_argument_name(output: &mut String, index: usize) {
    let mut digits = [0; size_of::<usize>() * 2];
    let mut cursor = digits.len();
    let mut value = index + 1;

    while value != 0 {
        value -= 1;
        cursor -= 1;
        digits[cursor] = b'a' + (value % 26) as u8;
        value /= 26;
    }

    for digit in &digits[cursor..] {
        output.push(char::from(*digit));
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::{FunctionValue, Value};
    use crate::plan::execution::function::{
        CoreRuntimeFunctionId, IntFunctionId, RuntimeFunctionId,
    };
    use crate::plan::{FunctionType, ValueType};
    use crate::runtime::run_main;

    #[test]
    fn writes_arity_without_runtime_identity_or_captures() {
        let function = FunctionValue::new(
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Int(IntFunctionId(0))),
            Vec::new(),
            FunctionType::new(vec![ValueType::Int, ValueType::String], ValueType::Int),
        );
        let other_runtime_target = FunctionValue::new(
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Int(IntFunctionId(99))),
            Vec::new(),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        let module = crate::compile_typed_module(
            "main",
            "main.gleam",
            r#"
pub fn main() {
  let captured = 1
  fn(argument) { argument + captured }
}
"#,
        )
        .expect("source should compile");
        let plan = crate::plan_module(module).expect("module should plan");
        let captured = run_main(
            &crate::ExecutionPlan::from_module_plan(plan),
            &mut Vec::new(),
        )
        .expect("main should return its closure");

        assert_eq!(
            Value::Function(function).inspect().to_string(),
            "//fn(a, b) { ... }",
        );
        assert_eq!(
            Value::Function(other_runtime_target).inspect().to_string(),
            "//fn(a) { ... }",
        );
        assert_eq!(captured.inspect().to_string(), "//fn(a) { ... }");
    }

    #[test]
    fn writes_argument_names_beyond_the_alphabet() {
        let function = FunctionValue::new(
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Int(IntFunctionId(0))),
            Vec::new(),
            FunctionType::new(vec![ValueType::Int; 27], ValueType::Int),
        );

        assert_eq!(
            Value::Function(function).inspect().to_string(),
            "//fn(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa) { ... }",
        );
    }
}
