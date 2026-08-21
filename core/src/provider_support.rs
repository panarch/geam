use crate::{
    BitArrayValue, HostCall, HostCustom, HostCustomConstructor, HostProfile, HostProvider,
    HostStoredDynamic, HostType, HostTypeSequence, ValueType,
};
use bitvec::order::Msb0;
use bitvec::slice::BitSlice;
use bitvec::vec::BitVec;
use std::marker::PhantomData;

pub struct HostOpaqueFunctionType<Arguments, Return>(PhantomData<(Arguments, Return)>);

#[allow(private_bounds)]
pub trait SoleHostCustomConstructor:
    private::SoleHostCustomConstructor + HostCustomConstructor
{
}

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

pub fn stored_value_type(value: &HostStoredDynamic) -> &ValueType {
    value.value_type()
}

pub fn stored_value_family(value: &HostStoredDynamic) -> HostStoredValueFamily {
    value.value_family()
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

pub fn sole_custom_fields<'call, Profile, Provider, Return, Constructor>(
    call: &mut HostCall<'call, Profile, Provider, Return>,
    value: HostCustom<'call, Constructor::Custom>,
) -> <Constructor::Fields as HostTypeSequence>::Values<'call>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Constructor: SoleHostCustomConstructor,
{
    call.sole_custom_fields::<Constructor>(value)
}

pub(crate) mod private {
    pub trait SoleHostCustomConstructor {}
}
