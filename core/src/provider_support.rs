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
