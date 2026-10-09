use super::CompiledProgress;
use crate::BitArrayValue;
use crate::plan::execution::graph::{Endianness, Signedness};
use crate::runtime::graph::decode_short_integer;

pub type BitArrayKernel = fn(usize, &mut BitArrayValues, &mut usize) -> CompiledProgress;

/// Backings are retained once per generated activation, never per loop edge.
#[derive(Default)]
pub struct BitArrayValues {
    pub ints: Vec<i128>,
    pub bools: Vec<bool>,
    pub bit_arrays: Vec<BitArrayRange>,
    backings: Vec<BitArrayValue>,
}

/// A copyable logical range within one activation's immutable backing table.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BitArrayRange {
    backing: usize,
    start: usize,
    length: usize,
}

impl BitArrayValues {
    pub fn integer(
        &self,
        range: BitArrayRange,
        start: usize,
        length: usize,
        endianness: Endianness,
        signedness: Signedness,
    ) -> Option<i128> {
        if length > 64 {
            return None;
        }
        let selected = range.slice(start, length)?;
        let bits = self.backings[selected.backing].bits();
        Some(decode_short_integer(
            &bits[selected.start..selected.start + selected.length],
            endianness,
            signedness,
        ))
    }

    pub(in crate::runtime) fn load_owned(
        &mut self,
        ints: &[i128],
        bools: &[bool],
        inputs: impl IntoIterator<Item = BitArrayValue>,
    ) {
        self.clear();
        self.ints.extend_from_slice(ints);
        self.bools.extend_from_slice(bools);
        for input in inputs {
            self.push_input(input);
        }
    }

    pub fn take_bit_arrays(&mut self) -> Vec<BitArrayValue> {
        let values = self
            .bit_arrays
            .iter()
            .map(|range| self.materialize(*range))
            .collect();
        self.release_inputs();
        values
    }

    pub fn finish(&mut self, range: BitArrayRange) -> BitArrayValue {
        let value = self.materialize(range);
        self.release_inputs();
        value
    }

    pub fn release_inputs(&mut self) {
        self.bit_arrays.clear();
        self.backings.clear();
    }

    pub(in crate::runtime) fn push_input(&mut self, input: BitArrayValue) {
        self.bit_arrays.push(BitArrayRange {
            backing: self.backings.len(),
            start: 0,
            length: input.bit_len(),
        });
        self.backings.push(input);
    }

    pub(in crate::runtime) fn materialize(&self, range: BitArrayRange) -> BitArrayValue {
        // Only checked subranges of loaded inputs can construct these values.
        self.backings[range.backing].slice_in_bounds(range.start, range.length)
    }

    pub(in crate::runtime) fn clear(&mut self) {
        self.ints.clear();
        self.bools.clear();
        self.bit_arrays.clear();
        self.backings.clear();
    }
}

impl BitArrayRange {
    pub fn bit_len(self) -> usize {
        self.length
    }

    pub fn slice(self, start: usize, length: usize) -> Option<Self> {
        if start.checked_add(length)? > self.length {
            return None;
        }
        Some(Self {
            backing: self.backing,
            // Private ranges stay inside their original backing. The checked
            // relative bounds above also prove this absolute offset fits.
            start: self.start + start,
            length,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{BitArrayValue, BitArrayValues, Endianness, Signedness};

    #[test]
    fn reads_checked_ranges_at_every_short_width_and_alignment() {
        let input = BitArrayValue::from_bytes(vec![
            0x91, 0x36, 0xf2, 0x85, 0xae, 0x63, 0x19, 0xd7, 0x45, 0x82,
        ]);
        let mut values = BitArrayValues::default();
        values.push_input(input.clone());
        for start in [0, 1, 7, 8, 9] {
            let range = values.bit_arrays[0].slice(start, 64).unwrap();
            for width in [0, 1, 7, 8, 9, 31, 32, 63, 64] {
                let bits = &input.bits()[start..start + width];
                for endian in [Endianness::Big, Endianness::Little] {
                    let unsigned = match endian {
                        Endianness::Big => bits
                            .iter()
                            .fold(0_i128, |value, bit| value * 2 + i128::from(*bit)),
                        Endianness::Little => {
                            bits.chunks(8)
                                .enumerate()
                                .fold(0_i128, |value, (index, byte)| {
                                    value
                                        + (byte.iter().fold(0_i128, |value, bit| {
                                            value * 2 + i128::from(*bit)
                                        }) << (index * 8))
                                })
                        }
                    };
                    assert_eq!(
                        values.integer(range, 0, width, endian, Signedness::Unsigned),
                        Some(unsigned)
                    );
                    let signed = if width > 0 && unsigned & (1_i128 << (width - 1)) != 0 {
                        unsigned - (1_i128 << width)
                    } else {
                        unsigned
                    };
                    assert_eq!(
                        values.integer(range, 0, width, endian, Signedness::Signed),
                        Some(signed)
                    );
                }
                let restored = values.materialize(range.slice(0, width).unwrap());
                assert_eq!(restored, input.bit_slice(start, width).unwrap());
            }
            assert_eq!(range.slice(64, 0).unwrap().bit_len(), 0);
            assert!(range.slice(63, 2).is_none());
            assert!(range.slice(usize::MAX, 1).is_none());
            assert!(
                values
                    .integer(range, 0, 65, Endianness::Big, Signedness::Unsigned)
                    .is_none()
            );
            assert!(
                values
                    .integer(range, 1, 64, Endianness::Big, Signedness::Unsigned)
                    .is_none()
            );
        }
    }

    #[test]
    fn separate_input_owners_and_copied_aliases_keep_their_identity_until_clear() {
        let mut values = BitArrayValues::default();
        values.push_input(BitArrayValue::from_bytes(vec![1, 2]));
        values.push_input(BitArrayValue::from_bytes(vec![3, 4]));
        let alias = values.bit_arrays[0].slice(8, 8).unwrap();
        values.bit_arrays.push(alias);
        assert_eq!(values.backings.len(), 2);
        assert_eq!(values.materialize(alias).bytes(), &[2]);
        assert_eq!(values.materialize(values.bit_arrays[1]).bytes(), &[3, 4]);
        values.ints.push(3);
        values.bools.push(true);
        values.clear();
        assert!(values.backings.is_empty());
        assert!(values.bit_arrays.is_empty());
        assert!(values.ints.is_empty());
        assert!(values.bools.is_empty());
        values.push_input(BitArrayValue::from_bytes(vec![9]));
        assert_eq!(values.backings.len(), 1);
        assert_eq!(
            values.integer(
                values.bit_arrays[0],
                0,
                8,
                Endianness::Big,
                Signedness::Unsigned
            ),
            Some(9)
        );
    }
    #[test]
    fn owned_call_checkpoints_and_completion_keep_nonbyte_ranges_and_release_backings() {
        let input = BitArrayValue::try_from_parts(vec![0xE5, 0x58], 13).unwrap();
        let mut values = BitArrayValues::default();
        values.load_owned(
            &[7],
            &[true],
            [input.clone(), BitArrayValue::from_bytes(vec![0])],
        );
        let sliced = values.bit_arrays[0].slice(2, 11).unwrap();
        values.bit_arrays = vec![sliced, sliced];
        let capacity = values.backings.capacity();
        let checkpoint = values.take_bit_arrays();
        assert_eq!(checkpoint, vec![input.bit_slice(2, 11).unwrap(); 2]);
        assert!(values.backings.is_empty());
        assert!(values.bit_arrays.is_empty());
        assert_eq!(values.backings.capacity(), capacity);
        assert_eq!(values.ints, [7]);
        assert_eq!(values.bools, [true]);
        values.load_owned(&[9], &[false], checkpoint);
        let range = values.bit_arrays[1].slice(1, 10).unwrap();
        let result = values.finish(range);
        assert_eq!(result, input.bit_slice(3, 10).unwrap());
        assert!(values.backings.is_empty());
        assert!(values.bit_arrays.is_empty());
        assert_eq!(values.ints, [9]);
        assert_eq!(values.bools, [false]);
    }
}
