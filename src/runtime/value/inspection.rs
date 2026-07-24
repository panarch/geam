use std::fmt::{self, Display, Formatter};

use num_bigint::BigInt;

use super::list::ListValueKind;
use super::{BitArrayValue, CustomValue, FunctionValue, ListValue, Value};

pub struct ValueInspection<'value> {
    value: &'value Value,
}

impl<'value> ValueInspection<'value> {
    pub(super) fn new(value: &'value Value) -> Self {
        Self { value }
    }
}

impl Display for ValueInspection<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        write_value(&mut output, self.value);
        formatter.write_str(&output)
    }
}

fn write_value(output: &mut String, value: &Value) {
    match value {
        Value::Int(value) => output.push_str(&value.to_string()),
        Value::Float(value) => output.push_str(&format!("{value:?}")),
        Value::String(value) => output.push_str(&format!("{value:?}")),
        Value::BitArray(value) => write_bit_array(output, value),
        Value::UtfCodepoint(value) => output.push_str(&format!("{value:?}")),
        Value::Custom(value) => write_custom(output, value),
        Value::Bool(true) => output.push_str("True"),
        Value::Bool(false) => output.push_str("False"),
        Value::Nil => output.push_str("Nil"),
        Value::Tuple(values) => {
            output.push_str("#(");
            write_values(output, values);
            output.push(')');
        }
        Value::List(value) => write_list(output, value),
        Value::Function(value) => write_function(output, value),
    }
}

fn write_bit_array(output: &mut String, value: &BitArrayValue) {
    output.push_str("<<");
    let full_bytes = value.bit_len() / 8;
    let remaining_bits = value.bit_len() % 8;
    let mut separator = "";

    for byte in value.bytes().iter().take(full_bytes) {
        output.push_str(separator);
        output.push_str(&byte.to_string());
        separator = ", ";
    }

    if remaining_bits != 0 {
        output.push_str(separator);
        let remaining = value.bytes()[full_bytes] >> (8 - remaining_bits);
        output.push_str(&remaining.to_string());
        output.push_str(":size(");
        output.push_str(&remaining_bits.to_string());
        output.push(')');
    }

    output.push_str(">>");
}

fn write_custom(output: &mut String, value: &CustomValue) {
    output.push_str(value.constructor_name());
    if value.fields().is_empty() {
        return;
    }

    output.push('(');
    let mut separator = "";
    for field in value.fields() {
        output.push_str(separator);
        if let Some(label) = field.label() {
            output.push_str(label);
            output.push_str(": ");
        }
        write_value(output, field.value());
        separator = ", ";
    }
    output.push(')');
}

fn write_list(output: &mut String, value: &ListValue) {
    if let ListValueKind::Int(values) = value.kind()
        && let Some(chars) = printable_charlist(values)
    {
        output.push_str("charlist.from_string(\"");
        output.push_str(&chars);
        output.push_str("\")");
        return;
    }

    match value.kind() {
        ListValueKind::Parameter(_) => output.push_str("[]"),
        ListValueKind::Int(values) => write_list_items(output, values, |output, value| {
            output.push_str(&value.to_string());
        }),
        ListValueKind::String(values) => write_list_items(output, values, |output, value| {
            output.push_str(&format!("{value:?}"));
        }),
        ListValueKind::BitArray(values) => write_list_items(output, values, write_bit_array),
        ListValueKind::UtfCodepoint(values) => {
            write_list_items(output, values, |output, value| {
                output.push_str(&format!("{value:?}"));
            });
        }
        ListValueKind::Custom { values, .. } => write_list_items(output, values, write_custom),
        ListValueKind::Float(values) => write_list_items(output, values, |output, value| {
            output.push_str(&format!("{value:?}"));
        }),
        ListValueKind::Bool(values) => write_list_items(output, values, |output, value| {
            output.push_str(if *value { "True" } else { "False" });
        }),
        ListValueKind::Nil(len) => {
            output.push('[');
            for index in 0..*len {
                if index != 0 {
                    output.push_str(", ");
                }
                output.push_str("Nil");
            }
            output.push(']');
        }
        ListValueKind::Tuple { values, .. } => {
            write_list_items(output, values, |output, values| {
                output.push_str("#(");
                write_values(output, values);
                output.push(')');
            });
        }
        ListValueKind::List { values, .. } => write_list_items(output, values, write_list),
        ListValueKind::Function { values, .. } => {
            write_list_items(output, values, write_function);
        }
    }
}

fn printable_charlist(values: &[BigInt]) -> Option<String> {
    if values.is_empty() {
        return None;
    }

    let mut chars = String::with_capacity(values.len());
    for value in values {
        match u8::try_from(value) {
            Ok(byte @ 32..=126) => chars.push(char::from(byte)),
            Ok(_) | Err(_) => return None,
        }
    }
    Some(chars)
}

fn write_list_items<T>(
    output: &mut String,
    values: &[T],
    mut write_item: impl FnMut(&mut String, &T),
) {
    output.push('[');
    let mut separator = "";
    for value in values {
        output.push_str(separator);
        write_item(output, value);
        separator = ", ";
    }
    output.push(']');
}

fn write_values(output: &mut String, values: &[Value]) {
    let mut separator = "";
    for value in values {
        output.push_str(separator);
        write_value(output, value);
        separator = ", ";
    }
}

fn write_function(output: &mut String, value: &FunctionValue) {
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
    use super::super::{BitArrayValue, CustomFieldValue, CustomValue, ListValue, Value};
    use crate::plan::execution::function::{IntFunctionId, RuntimeFunctionId};
    use crate::plan::{CustomType, CustomTypeName, FunctionType, TypeParameterId, ValueType};
    use crate::runtime::{FunctionValue, run_main};

    #[test]
    fn inspects_primitive_values() {
        let cases = [
            (Value::Int((-12).into()), "-12"),
            (Value::Float(1.0), "1.0"),
            (Value::Float(-0.0), "-0.0"),
            (Value::Float(f64::INFINITY), "inf"),
            (Value::Float(f64::NEG_INFINITY), "-inf"),
            (Value::Float(f64::NAN), "NaN"),
            (
                Value::String("one\n\"two\"\\".into()),
                "\"one\\n\\\"two\\\"\\\\\"",
            ),
            (Value::UtfCodepoint('A'), "'A'"),
            (Value::UtfCodepoint('\n'), "'\\n'"),
            (Value::UtfCodepoint('\u{10ffff}'), "'\\u{10ffff}'"),
            (Value::Bool(true), "True"),
            (Value::Bool(false), "False"),
            (Value::Nil, "Nil"),
        ];

        for (value, expected) in cases {
            assert_eq!(value.inspect().to_string(), expected);
        }
    }

    #[test]
    fn inspects_aligned_and_unaligned_bit_arrays() {
        let cases = [
            (BitArrayValue::from_bytes(Vec::new()), "<<>>"),
            (BitArrayValue::from_bytes(vec![1, 2, 3]), "<<1, 2, 3>>"),
            (
                BitArrayValue::try_from_parts(vec![1, 2, 0b1100_0000], 18)
                    .expect("eighteen supplied bits should be valid"),
                "<<1, 2, 3:size(2)>>",
            ),
        ];

        for (value, expected) in cases {
            assert_eq!(Value::BitArray(value).inspect().to_string(), expected);
        }
    }

    #[test]
    fn inspects_tuples_and_custom_values() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Person".into()),
            Vec::new(),
        );
        let person = CustomValue::from_evaluated(
            custom_type.clone(),
            "Person".into(),
            0,
            vec![
                CustomFieldValue::from_evaluated(Some("name".into()), Value::String("Kim".into())),
                CustomFieldValue::from_evaluated(None, Value::Int(42.into())),
            ],
        );
        let ready = CustomValue::from_evaluated(custom_type, "Ready".into(), 1, Vec::new());

        assert_eq!(
            Value::Tuple(vec![Value::Int(1.into()), Value::String("one".into())])
                .inspect()
                .to_string(),
            "#(1, \"one\")",
        );
        assert_eq!(
            Value::Custom(person).inspect().to_string(),
            "Person(name: \"Kim\", 42)",
        );
        assert_eq!(Value::Custom(ready).inspect().to_string(), "Ready");
    }

    #[test]
    fn inspects_every_list_storage_family() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let custom =
            CustomValue::from_evaluated(custom_type.clone(), "Boxed".into(), 0, Vec::new());
        let function = sample_function(Vec::new());
        let cases = [
            (
                ListValue::empty(ValueType::Parameter(TypeParameterId(0))),
                "[]",
            ),
            (ListValue::int(Vec::new()), "[]"),
            (
                ListValue::int(vec![72.into(), 105.into(), 33.into()]),
                "charlist.from_string(\"Hi!\")",
            ),
            (
                ListValue::int(vec![31.into(), 126.into(), 256.into()]),
                "[31, 126, 256]",
            ),
            (ListValue::string(vec!["one".into()]), "[\"one\"]"),
            (
                ListValue::bit_array(vec![BitArrayValue::from_bytes(vec![1])]),
                "[<<1>>]",
            ),
            (ListValue::utf_codepoint(vec!['A']), "['A']"),
            (
                ListValue::from_evaluated_custom(custom_type, vec![custom]),
                "[Boxed]",
            ),
            (ListValue::float(vec![1.5]), "[1.5]"),
            (ListValue::bool(vec![true, false]), "[True, False]"),
            (ListValue::nil(2), "[Nil, Nil]"),
            (
                ListValue::from_evaluated_tuple(
                    vec![ValueType::Int],
                    vec![vec![Value::Int(1.into())]],
                ),
                "[#(1)]",
            ),
            (
                ListValue::from_evaluated_list(
                    ValueType::Int,
                    vec![ListValue::int(vec![1.into()])],
                ),
                "[[1]]",
            ),
            (
                ListValue::from_evaluated_function(function.type_(), vec![function]),
                "[//fn() { ... }]",
            ),
        ];

        for (value, expected) in cases {
            assert_eq!(Value::List(value).inspect().to_string(), expected);
        }
    }

    #[test]
    fn inspects_functions_by_arity_without_runtime_identity_or_captures() {
        let function = sample_function(vec![ValueType::Int, ValueType::String]);
        let other_runtime_target = FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(99)),
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
        let captured = run_main(&crate::ExecutionPlan::from_module_plan(plan))
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
    fn inspects_function_argument_names_beyond_the_alphabet() {
        let function = sample_function(vec![ValueType::Int; 27]);

        assert_eq!(
            Value::Function(function).inspect().to_string(),
            "//fn(a, b, c, d, e, f, g, h, i, j, k, l, m, n, o, p, q, r, s, t, u, v, w, x, y, z, aa) { ... }",
        );
    }

    fn sample_function(arguments: Vec<ValueType>) -> FunctionValue {
        FunctionValue::new(
            RuntimeFunctionId::Int(IntFunctionId(0)),
            Vec::new(),
            FunctionType::new(arguments, ValueType::Int),
        )
    }
}
