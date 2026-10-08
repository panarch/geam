use super::CompiledProgress;
use crate::StringValue;
use std::mem;

pub type StringKernel = fn(usize, &mut StringValues, &mut usize) -> CompiledProgress;

/// One bounded generated run owns the current input strings. Generated locals
/// carry ranges into those owners or static literals, never cloned suffixes.
#[derive(Default)]
pub struct StringValues {
    pub ints: Vec<i128>,
    pub bools: Vec<bool>,
    pub strings: Vec<StringRange>,
    inputs: Vec<StringValue>,
    uses: Vec<usize>,
}

/// A run-local view. Its private origin cannot escape with a borrowed input.
#[derive(Clone, Copy)]
pub struct StringRange {
    origin: StringOrigin,
    start: usize,
    end: usize,
}

#[derive(Clone, Copy)]
enum StringOrigin {
    Input(usize),
    Literal(&'static str),
}

impl StringValues {
    pub fn bytes(&self, value: StringRange) -> &[u8] {
        let bytes = match value.origin {
            StringOrigin::Input(index) => self.inputs[index].as_bytes(),
            StringOrigin::Literal(text) => text.as_bytes(),
        };
        &bytes[value.start..value.end]
    }

    pub(in crate::runtime) fn load_owned(
        &mut self,
        ints: &[i128],
        bools: &[bool],
        strings: impl IntoIterator<Item = StringValue>,
    ) {
        self.ints.clear();
        self.ints.extend_from_slice(ints);
        self.bools.clear();
        self.bools.extend_from_slice(bools);
        self.inputs.clear();
        self.inputs.extend(strings);
        self.strings.clear();
        self.strings.extend(
            self.inputs
                .iter()
                .enumerate()
                .map(|(index, input)| StringRange {
                    origin: StringOrigin::Input(index),
                    start: 0,
                    end: input.len(),
                }),
        );
    }

    /// Materialize only live checkpoint columns before another call or yield.
    pub fn take_strings(&mut self) -> Vec<StringValue> {
        let mut strings = Vec::with_capacity(self.strings.len());
        self.restore_strings(&mut strings);
        strings
    }

    /// Completion consumes just the actual result without checkpoint columns.
    pub fn finish(&mut self, value: StringRange) -> StringValue {
        let range = value.start..value.end;
        let result = match value.origin {
            StringOrigin::Input(index) => mem::take(&mut self.inputs[index]).into_slice(range),
            StringOrigin::Literal(text) => StringValue::from(text).into_slice(range),
        };
        self.release_inputs();
        result
    }

    pub fn release_inputs(&mut self) {
        self.strings.clear();
        self.inputs.clear();
        self.uses.clear();
    }

    pub(in crate::runtime) fn load_strings(&mut self, strings: &mut Vec<StringValue>) {
        self.strings.clear();
        self.inputs.clear();
        mem::swap(&mut self.inputs, strings);
        self.strings.extend(
            self.inputs
                .iter()
                .enumerate()
                .map(|(index, input)| StringRange {
                    origin: StringOrigin::Input(index),
                    start: 0,
                    end: input.len(),
                }),
        );
    }

    pub(in crate::runtime) fn restore_strings(&mut self, strings: &mut Vec<StringValue>) {
        self.uses.clear();
        // A checkpoint without strings only releases the roots. It needs no
        // alias counters, including the first return of an empty input.
        if !self.strings.is_empty() {
            self.uses.resize(self.inputs.len(), 0);
        }
        for value in &self.strings {
            if let StringOrigin::Input(index) = value.origin {
                self.uses[index] += 1;
            }
        }
        strings.clear();
        for value in self.strings.drain(..) {
            let range = value.start..value.end;
            strings.push(match value.origin {
                StringOrigin::Input(index) => {
                    self.uses[index] -= 1;
                    if self.uses[index] == 0 {
                        mem::take(&mut self.inputs[index]).into_slice(range)
                    } else {
                        self.inputs[index].slice(range)
                    }
                }
                StringOrigin::Literal(text) => StringValue::from(text).into_slice(range),
            });
        }
        // Capacity is reusable, but abandoned input owners never survive a run.
        self.inputs.clear();
        self.uses.clear();
    }
}

impl StringRange {
    pub fn literal(text: &'static str) -> Self {
        Self {
            origin: StringOrigin::Literal(text),
            start: 0,
            end: text.len(),
        }
    }

    /// Canonical DropPrefix follows the successful byte prefix condition.
    pub fn drop_prefix(self, bytes: usize) -> Self {
        Self {
            start: self.start + bytes,
            ..self
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{StringRange, StringValues};
    use crate::StringValue;

    #[test]
    fn input_container_moves_and_an_empty_checkpoint_needs_no_alias_counter_allocation() {
        let mut inputs = vec![StringValue::new(), StringValue::from("unused".repeat(32))];
        let pointer = inputs.as_ptr();
        let capacity = inputs.capacity();
        let mut values = StringValues::default();
        values.load_strings(&mut inputs);
        assert!(inputs.is_empty());
        assert_eq!(values.inputs.as_ptr(), pointer);
        assert_eq!(values.inputs.capacity(), capacity);
        assert_eq!(values.bytes(values.strings[0]), "".as_bytes());
        assert_eq!(
            values.bytes(values.strings[1]),
            "unused".repeat(32).as_bytes()
        );
        values.strings.clear();
        values.restore_strings(&mut inputs);
        assert!(inputs.is_empty());
        assert!(values.inputs.is_empty());
        assert!(values.strings.is_empty());
        assert_eq!(values.uses.capacity(), 0);
    }

    #[test]
    fn ranges_keep_distinct_inputs_literals_aliases_and_utf8_without_cloning_owners() {
        let original = StringValue::from("hidden:가나다abcdefghijklmnopqrstuvwxyz:end");
        let visible = original.slice(7..original.len() - 4);
        let pointer = visible.as_ptr();
        let mut inputs = vec![visible, StringValue::from("other:".repeat(10))];
        let mut values = StringValues::default();
        values.load_strings(&mut inputs);
        assert!(inputs.is_empty());
        let whole = values.strings[0];
        let suffix = whole.drop_prefix("가나다".len());
        let other = values.strings[1].drop_prefix(6);
        let literal = StringRange::literal("\n\"가나").drop_prefix(2);
        assert_eq!(
            values.bytes(whole),
            "가나다abcdefghijklmnopqrstuvwxyz".as_bytes()
        );
        assert_eq!(
            values.bytes(suffix),
            "abcdefghijklmnopqrstuvwxyz".as_bytes()
        );
        assert_eq!(values.bytes(other), "other:".repeat(9).as_bytes());
        assert_eq!(values.bytes(literal), "가나".as_bytes());
        values.strings = vec![
            suffix,
            whole,
            suffix,
            other,
            literal,
            StringRange::literal(""),
        ];
        values.restore_strings(&mut inputs);
        assert_eq!(
            inputs
                .iter()
                .map(|value| value.as_str().unwrap())
                .collect::<Vec<_>>(),
            [
                "abcdefghijklmnopqrstuvwxyz",
                "가나다abcdefghijklmnopqrstuvwxyz",
                "abcdefghijklmnopqrstuvwxyz",
                "other:other:other:other:other:other:other:other:other:",
                "가나",
                "",
            ]
        );
        assert_eq!(inputs[0].as_ptr(), pointer.wrapping_add(9));
        assert_eq!(inputs[1].as_ptr(), pointer);
        assert_eq!(inputs[2].as_ptr(), inputs[0].as_ptr());
        assert!(values.inputs.is_empty());
        assert!(values.strings.is_empty());
        assert!(values.uses.is_empty());
        drop(original);
        assert_eq!(inputs[0].as_str().unwrap(), "abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn long_repetition_retains_only_inputs_and_restores_inline_empty_or_no_outputs() {
        let mut inputs = vec![StringValue::from("한".repeat(20_000)), "unused".into()];
        let mut values = StringValues::default();
        values.load_strings(&mut inputs);
        let mut suffix = values.strings[0];
        for _ in 0..19_999 {
            assert!(values.bytes(suffix).starts_with("한".as_bytes()));
            suffix = suffix.drop_prefix(3);
        }
        assert_eq!(values.inputs.len(), 2);
        assert_eq!(values.strings.len(), 2);
        values.strings = vec![suffix, suffix.drop_prefix(3)];
        values.restore_strings(&mut inputs);
        assert_eq!(inputs, [StringValue::from("한"), StringValue::new()]);
        values.load_strings(&mut inputs);
        values.strings.clear();
        values.restore_strings(&mut inputs);
        assert!(inputs.is_empty());
        assert!(values.inputs.is_empty());
        assert!(values.uses.is_empty());
    }
    #[test]
    fn raw_ranges_restore_aliases_last_owners_and_partial_literal_codepoints() {
        let raw = StringValue::from_bytes(["λ".as_bytes(), &[0xff; 64]].concat());
        let pointer = raw.as_ptr();
        let mut inputs = vec![raw];
        let mut values = StringValues::default();
        values.load_strings(&mut inputs);
        let original = values.strings[0];
        let suffix = original.drop_prefix(2);
        assert_eq!(values.bytes(suffix), &[0xff; 64]);
        values.strings = vec![
            original,
            suffix,
            suffix,
            StringRange::literal("é").drop_prefix(1),
        ];
        values.restore_strings(&mut inputs);
        assert_eq!(inputs[0].as_ptr(), pointer);
        assert_eq!(inputs[1].as_ptr(), pointer.wrapping_add(2));
        assert_eq!(inputs[2].as_ptr(), inputs[1].as_ptr());
        assert_eq!(inputs[3].as_bytes(), &[0xa9]);
        assert!(inputs[3].as_str().is_err());
        let suffix = inputs.remove(2);
        drop(inputs);
        assert_eq!(suffix.as_bytes(), &[0xff; 64]);
        let mut inputs = vec![suffix];
        values.load_strings(&mut inputs);
        values.strings[0] = values.strings[0].drop_prefix(1);
        values.restore_strings(&mut inputs);
        assert_eq!(inputs[0].as_ptr(), pointer.wrapping_add(3));
        assert_eq!(inputs[0].as_bytes(), &[0xff; 63]);
    }
    #[test]
    fn owned_call_checkpoints_and_completion_release_roots_and_keep_raw_slices() {
        let raw = StringValue::from_bytes([b"tag:".as_slice(), &[0xff; 64]].concat());
        let pointer = raw.as_ptr();
        let mut values = StringValues::default();
        values.load_owned(&[7], &[true], [raw, "unused root".into()]);
        let suffix = values.strings[0].drop_prefix(4);
        values.strings = vec![suffix, suffix, StringRange::literal("é").drop_prefix(1)];
        let ranges_capacity = values.strings.capacity();
        let roots_capacity = values.inputs.capacity();
        let checkpoint = values.take_strings();
        assert_eq!(checkpoint[0].as_ptr(), pointer.wrapping_add(4));
        assert_eq!(checkpoint[1].as_ptr(), checkpoint[0].as_ptr());
        assert_eq!(checkpoint[2].as_bytes(), &[0xa9]);
        assert_eq!(values.ints, [7]);
        assert_eq!(values.bools, [true]);
        assert!(values.inputs.is_empty());
        assert!(values.strings.is_empty());
        assert_eq!(values.strings.capacity(), ranges_capacity);
        assert_eq!(values.inputs.capacity(), roots_capacity);
        values.load_owned(&[9], &[false], checkpoint);
        let result = values.finish(values.strings[1].drop_prefix(1));
        assert_eq!(result.as_ptr(), pointer.wrapping_add(5));
        assert_eq!(result.as_bytes(), &[0xff; 63]);
        assert!(values.inputs.is_empty());
        assert!(values.strings.is_empty());
        assert!(values.uses.is_empty());
        assert_eq!(values.ints, [9]);
        assert_eq!(values.bools, [false]);
        assert_eq!(
            values
                .finish(StringRange::literal("é").drop_prefix(1))
                .as_bytes(),
            &[0xa9]
        );
    }
}
