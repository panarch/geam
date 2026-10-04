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
    pub fn text(&self, value: StringRange) -> &str {
        let text = match value.origin {
            StringOrigin::Input(index) => self.inputs[index].as_str(),
            StringOrigin::Literal(text) => text,
        };
        &text[value.start..value.end]
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
                StringOrigin::Literal(text) => StringValue::from(&text[range]),
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

    /// Canonical DropPrefix follows the successful UTF-8 prefix condition.
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
        assert_eq!(values.text(values.strings[0]), "");
        assert_eq!(values.text(values.strings[1]), "unused".repeat(32));
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
        let pointer = visible.as_str().as_ptr();
        let mut inputs = vec![visible, StringValue::from("other:".repeat(10))];
        let mut values = StringValues::default();
        values.load_strings(&mut inputs);
        assert!(inputs.is_empty());
        let whole = values.strings[0];
        let suffix = whole.drop_prefix("가나다".len());
        let other = values.strings[1].drop_prefix(6);
        let literal = StringRange::literal("\n\"가나").drop_prefix(2);
        assert_eq!(values.text(whole), "가나다abcdefghijklmnopqrstuvwxyz");
        assert_eq!(values.text(suffix), "abcdefghijklmnopqrstuvwxyz");
        assert_eq!(values.text(other), "other:".repeat(9));
        assert_eq!(values.text(literal), "가나");
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
            inputs.iter().map(StringValue::as_str).collect::<Vec<_>>(),
            [
                "abcdefghijklmnopqrstuvwxyz",
                "가나다abcdefghijklmnopqrstuvwxyz",
                "abcdefghijklmnopqrstuvwxyz",
                "other:other:other:other:other:other:other:other:other:",
                "가나",
                "",
            ]
        );
        assert_eq!(inputs[0].as_str().as_ptr(), pointer.wrapping_add(9));
        assert_eq!(inputs[1].as_str().as_ptr(), pointer);
        assert_eq!(inputs[2].as_str().as_ptr(), inputs[0].as_str().as_ptr());
        assert!(values.inputs.is_empty());
        assert!(values.strings.is_empty());
        assert!(values.uses.is_empty());
        drop(original);
        assert_eq!(inputs[0].as_str(), "abcdefghijklmnopqrstuvwxyz");
    }

    #[test]
    fn long_repetition_retains_only_inputs_and_restores_inline_empty_or_no_outputs() {
        let mut inputs = vec![StringValue::from("한".repeat(20_000)), "unused".into()];
        let mut values = StringValues::default();
        values.load_strings(&mut inputs);
        let mut suffix = values.strings[0];
        for _ in 0..19_999 {
            assert!(values.text(suffix).starts_with("한"));
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
}
