use num_bigint::BigInt;

use super::super::list::{ListValue, ListValueKind};
use super::{bit_array, custom, function, tuple};

pub(super) fn write(output: &mut String, value: &ListValue) {
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
        ListValueKind::Int(values) => write_items(output, values, |output, value| {
            output.push_str(&value.to_string());
        }),
        ListValueKind::String(values) => write_items(output, values, |output, value| {
            output.push_str(&format!("{value:?}"));
        }),
        ListValueKind::BitArray(values) => write_items(output, values, bit_array::write),
        ListValueKind::UtfCodepoint(values) => {
            write_items(output, values, |output, value| {
                output.push_str(&format!("{value:?}"));
            });
        }
        ListValueKind::Custom { values, .. } => write_items(output, values, custom::write),
        ListValueKind::External { values, .. } => {
            write_items(output, values, |output, value| {
                output.push_str(value.inspection());
            });
        }
        ListValueKind::Float(values) => write_items(output, values, |output, value| {
            output.push_str(&format!("{value:?}"));
        }),
        ListValueKind::Bool(values) => write_items(output, values, |output, value| {
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
        ListValueKind::Tuple { values, .. } => write_items(output, values, |output, values| {
            tuple::write(output, values)
        }),
        ListValueKind::List { values, .. } => write_items(output, values, write),
        ListValueKind::Function { values, .. } => write_items(output, values, function::write),
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

fn write_items<T>(output: &mut String, values: &[T], mut write_item: impl FnMut(&mut String, &T)) {
    output.push('[');
    let mut separator = "";
    for value in values {
        output.push_str(separator);
        write_item(output, value);
        separator = ", ";
    }
    output.push(']');
}

#[cfg(test)]
mod tests {
    use super::super::super::{BitArrayValue, CustomValue, FunctionValue, ListValue, Value};
    use crate::host::HostExternalStore;
    use crate::plan::execution::function::{
        CoreRuntimeFunctionId, IntFunctionId, RuntimeFunctionId,
    };
    use crate::plan::{
        CustomType, CustomTypeName, ExternalType, ExternalTypeName, FunctionType, TypeParameterId,
        ValueType,
    };
    use crate::runtime::ExternalValue;

    #[test]
    fn writes_empty_printable_and_non_printable_int_lists() {
        let cases = [
            (
                ListValue::empty(ValueType::Parameter(TypeParameterId(0))),
                "[]",
            ),
            (ListValue::int(Vec::new()), "[]"),
            (
                ListValue::int(vec![72.into(), 105.into(), 33.into()]),
                r#"charlist.from_string("Hi!")"#,
            ),
            (
                ListValue::int(vec![31.into(), 126.into(), 256.into()]),
                "[31, 126, 256]",
            ),
        ];

        for (value, expected) in cases {
            assert_eq!(Value::List(value).inspect().to_string(), expected);
        }
    }

    #[test]
    fn writes_every_non_int_list_storage_family() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let custom =
            CustomValue::from_evaluated(custom_type.clone(), "Boxed".into(), 0, Vec::new());
        let function = FunctionValue::new(
            RuntimeFunctionId::Core(CoreRuntimeFunctionId::Int(IntFunctionId(0))),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        );
        let external_type = ExternalType::new(
            ExternalTypeName::new("application".into(), "main".into(), "Resource".into()),
            Vec::new(),
        );
        let store = HostExternalStore::default();
        let source_equal =
            |context: &crate::host::HostExternalEquality<'_>,
             left: &crate::host::HostStoredValue<num_bigint::BigInt>,
             right: &crate::host::HostStoredValue<num_bigint::BigInt>| {
                context.stored_values_equal(left, right)
            };
        let first = store.insert(
            crate::host::HostStoredValue::new(crate::runtime::StoredRuntimeValue::test_int(
                7.into(),
            )),
            source_equal,
            7,
            "Resource(7)".into(),
        );
        let equal = store.insert(
            crate::host::HostStoredValue::new(crate::runtime::StoredRuntimeValue::test_int(
                7.into(),
            )),
            source_equal,
            7,
            "Resource(7)".into(),
        );
        let stored_equal =
            |left: &crate::runtime::StoredRuntimeValue,
             right: &crate::runtime::StoredRuntimeValue| left.value() == right.value();
        let equality = crate::host::HostExternalEquality::new(&stored_equal);
        assert!(first.source_equal(&equality, &equal));
        let external = ExternalValue::from_evaluated(external_type.clone(), first);
        let cases = [
            (ListValue::string(vec!["one".into()]), r#"["one"]"#),
            (
                ListValue::bit_array(vec![BitArrayValue::from_bytes(vec![1])]),
                "[<<1>>]",
            ),
            (ListValue::utf_codepoint(vec!['A']), "['A']"),
            (
                ListValue::from_evaluated_custom(custom_type, vec![custom]),
                "[Boxed]",
            ),
            (
                ListValue::from_evaluated_external(external_type, vec![external]),
                "[Resource(7)]",
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
}
