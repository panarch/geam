use super::super::CustomValue;
use super::write_value;

pub(super) fn write(output: &mut String, value: &CustomValue) {
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

#[cfg(test)]
mod tests {
    use super::super::super::{CustomFieldValue, CustomValue, Value};
    use crate::plan::{CustomType, CustomTypeName};

    #[test]
    fn writes_labelled_unlabelled_and_empty_custom_fields() {
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
            Value::Custom(person).inspect().to_string(),
            r#"Person(name: "Kim", 42)"#,
        );
        assert_eq!(Value::Custom(ready).inspect().to_string(), "Ready");
    }
}
