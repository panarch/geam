use super::super::Value;
use super::write_value;

pub(super) fn write(output: &mut String, values: &[Value]) {
    output.push_str("#(");
    let mut separator = "";
    for value in values {
        output.push_str(separator);
        write_value(output, value);
        separator = ", ";
    }
    output.push(')');
}

#[cfg(test)]
mod tests {
    use super::super::super::Value;

    #[test]
    fn writes_empty_and_recursive_tuples() {
        let cases = [
            (Value::Tuple(Vec::new()), "#()"),
            (
                Value::Tuple(vec![
                    Value::Int(1.into()),
                    Value::Tuple(vec![Value::String("one".into())]),
                ]),
                r#"#(1, #("one"))"#,
            ),
        ];

        for (value, expected) in cases {
            assert_eq!(value.inspect().to_string(), expected);
        }
    }
}
