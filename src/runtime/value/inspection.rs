mod bit_array;
mod custom;
mod function;
mod list;
mod tuple;

use std::fmt::{self, Display, Formatter};

use super::Value;

pub struct ValueInspection<'value> {
    value: &'value Value,
}

impl<'value> ValueInspection<'value> {
    pub(super) fn new(value: &'value Value) -> Self {
        Self { value }
    }

    pub(crate) fn write_to(&self, output: &mut String) {
        write_value(output, self.value);
    }
}

impl Display for ValueInspection<'_> {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        let mut output = String::new();
        self.write_to(&mut output);
        formatter.write_str(&output)
    }
}

fn write_value(output: &mut String, value: &Value) {
    match value {
        Value::Int(value) => output.push_str(&value.to_string()),
        Value::Float(value) => output.push_str(&format!("{value:?}")),
        Value::String(value) => output.push_str(&format!("{value:?}")),
        Value::BitArray(value) => bit_array::write(output, value),
        Value::UtfCodepoint(value) => output.push_str(&format!("{value:?}")),
        Value::Custom(value) => custom::write(output, value),
        Value::Bool(true) => output.push_str("True"),
        Value::Bool(false) => output.push_str("False"),
        Value::Nil => output.push_str("Nil"),
        Value::Tuple(values) => tuple::write(output, values),
        Value::List(value) => list::write(output, value),
        Value::Function(value) => function::write(output, value),
    }
}

#[cfg(test)]
mod tests {
    use super::super::Value;

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
                r#""one\n\"two\"\\""#,
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
}
