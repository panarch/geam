mod local;
mod slot;
mod type_;

pub(super) use local::ExplainLocal;
pub(super) use slot::{write_locals, write_slot, write_slots};
pub(super) use type_::write_type;

pub(super) fn write_list<Value>(
    output: &mut String,
    values: &[Value],
    mut write_value: impl FnMut(&mut String, &Value),
) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        write_value(output, value);
    }
    output.push(']');
}

#[cfg(test)]
mod tests {
    #[test]
    fn writes_empty_and_separated_value_lists() {
        fn write_usize(output: &mut String, value: &usize) {
            output.push_str(&value.to_string());
        }

        let mut output = String::new();
        super::write_list(&mut output, &[] as &[usize], write_usize);
        assert_eq!(output, "[]");

        output.clear();
        super::write_list(&mut output, &[1, 2, 3], write_usize);
        assert_eq!(output, "[1, 2, 3]");
    }
}
