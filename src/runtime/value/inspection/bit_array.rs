use super::super::BitArrayValue;

pub(super) fn write(output: &mut String, value: &BitArrayValue) {
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

#[cfg(test)]
mod tests {
    use super::super::super::{BitArrayValue, Value};

    #[test]
    fn writes_aligned_and_unaligned_bit_arrays() {
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
}
