use crate::BitArrayValue;
use bitvec::order::Msb0;
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;
use std::marker::PhantomData;

pub struct HostOpaqueFunctionType<Arguments, Return>(PhantomData<(Arguments, Return)>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HostStoredValueFamily {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    List,
    Tuple,
    Custom,
    External,
    Function,
}

pub fn bit_array_bits(value: &BitArrayValue) -> &BitSlice<u8, Msb0> {
    value.bits()
}

pub fn bit_array_from_bits(bits: BitVec<u8, Msb0>) -> BitArrayValue {
    BitArrayValue::from_evaluated(bits)
}

pub fn bit_array_byte_slice(
    value: &BitArrayValue,
    start: usize,
    length: usize,
) -> Option<BitArrayValue> {
    value.byte_slice(start, length)
}

pub fn bit_array_pad_to_bytes(value: &BitArrayValue) -> BitArrayValue {
    value.pad_to_bytes()
}

#[cfg(test)]
mod tests {
    use super::{
        bit_array_bits, bit_array_byte_slice, bit_array_from_bits, bit_array_pad_to_bytes,
    };
    use crate::BitArrayValue;
    use bitvec::bitvec;
    use bitvec::order::Msb0;

    #[test]
    fn bit_array_bridges_preserve_bits_slices_and_padding() {
        let bits = bitvec![u8, Msb0; 1, 0, 1, 0];
        let unaligned = bit_array_from_bits(bits.clone());
        let bytes = BitArrayValue::from_bytes(vec![1, 2, 3, 4]);

        assert_eq!(bit_array_bits(&unaligned), bits.as_bitslice());
        assert_eq!(
            bit_array_byte_slice(&bytes, 1, 2)
                .expect("aligned in-bounds byte slice should exist")
                .bytes(),
            &[2, 3],
        );
        assert_eq!(bit_array_byte_slice(&bytes, 3, 2), None);

        let padded = bit_array_pad_to_bytes(&unaligned);
        assert_eq!(padded.bytes(), &[0b1010_0000]);
        assert_eq!(padded.bit_len(), 8);
    }
}
