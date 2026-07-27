mod bit_array;
mod bool;
mod float;
mod int;
mod nil;
mod string;
mod utf_codepoint;

use super::HostValueType;
use crate::BitArrayValue;
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) use bit_array::HostBitArrayArgumentSlot;
pub(crate) use bool::HostBoolArgumentSlot;
pub(crate) use float::HostFloatArgumentSlot;
pub(crate) use int::HostIntArgumentSlot;
pub(crate) use nil::HostNilArgumentSlot;
pub(crate) use string::HostStringArgumentSlot;
pub(crate) use utf_codepoint::HostUtfCodepointArgumentSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostParameter {
    Int(HostIntArgumentSlot),
    Float(HostFloatArgumentSlot),
    String(HostStringArgumentSlot),
    BitArray(HostBitArrayArgumentSlot),
    UtfCodepoint(HostUtfCodepointArgumentSlot),
    Bool(HostBoolArgumentSlot),
    Nil(HostNilArgumentSlot),
}

pub(crate) trait HostCallArguments {
    fn int(&self, slot: HostIntArgumentSlot) -> BigInt;
    fn float(&self, slot: HostFloatArgumentSlot) -> f64;
    fn string(&self, slot: HostStringArgumentSlot) -> EcoString;
    fn bit_array(&self, slot: HostBitArrayArgumentSlot) -> BitArrayValue;
    fn utf_codepoint(&self, slot: HostUtfCodepointArgumentSlot) -> char;
    fn bool(&self, slot: HostBoolArgumentSlot) -> bool;
    fn nil(&self, slot: HostNilArgumentSlot);
}

pub(super) trait HostArgument: Sized {
    type Slot: Copy + Send + Sync + 'static;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot;
    fn read(arguments: &dyn HostCallArguments, slot: Self::Slot) -> Self;
}

#[derive(Default)]
pub(super) struct HostParameterLayout {
    parameters: Vec<HostParameter>,
    next_int: usize,
    next_float: usize,
    next_string: usize,
    next_bit_array: usize,
    next_utf_codepoint: usize,
    next_bool: usize,
    next_nil: usize,
}

impl HostParameter {
    pub(crate) fn type_(self) -> HostValueType {
        match self {
            Self::Int(_) => HostValueType::Int,
            Self::Float(_) => HostValueType::Float,
            Self::String(_) => HostValueType::String,
            Self::BitArray(_) => HostValueType::BitArray,
            Self::UtfCodepoint(_) => HostValueType::UtfCodepoint,
            Self::Bool(_) => HostValueType::Bool,
            Self::Nil(_) => HostValueType::Nil,
        }
    }
}

impl HostParameterLayout {
    pub(super) fn register<Argument: HostArgument>(&mut self) -> Argument::Slot {
        Argument::register(self)
    }

    pub(super) fn finish(self) -> Box<[HostParameter]> {
        self.parameters.into_boxed_slice()
    }
}

#[cfg(test)]
pub(in crate::host) struct CallArguments {
    ints: Vec<BigInt>,
    floats: Vec<f64>,
    strings: Vec<EcoString>,
    bit_arrays: Vec<BitArrayValue>,
    utf_codepoints: Vec<char>,
    bools: Vec<bool>,
    nils: Vec<()>,
}

#[cfg(test)]
impl CallArguments {
    pub(in crate::host) fn new(ints: Vec<BigInt>, bools: Vec<bool>) -> Self {
        Self {
            ints,
            floats: Vec::new(),
            strings: Vec::new(),
            bit_arrays: Vec::new(),
            utf_codepoints: Vec::new(),
            bools,
            nils: Vec::new(),
        }
    }

    pub(in crate::host) fn with_scalar_values(
        mut self,
        floats: Vec<f64>,
        strings: Vec<EcoString>,
        bit_arrays: Vec<BitArrayValue>,
        utf_codepoints: Vec<char>,
        nils: usize,
    ) -> Self {
        self.floats = floats;
        self.strings = strings;
        self.bit_arrays = bit_arrays;
        self.utf_codepoints = utf_codepoints;
        self.nils = vec![(); nils];
        self
    }
}

#[cfg(test)]
impl HostCallArguments for CallArguments {
    fn int(&self, slot: HostIntArgumentSlot) -> BigInt {
        self.ints[slot.index()].clone()
    }

    fn float(&self, slot: HostFloatArgumentSlot) -> f64 {
        self.floats[slot.index()]
    }

    fn string(&self, slot: HostStringArgumentSlot) -> EcoString {
        self.strings[slot.index()].clone()
    }

    fn bit_array(&self, slot: HostBitArrayArgumentSlot) -> BitArrayValue {
        self.bit_arrays[slot.index()].clone()
    }

    fn utf_codepoint(&self, slot: HostUtfCodepointArgumentSlot) -> char {
        self.utf_codepoints[slot.index()]
    }

    fn bool(&self, slot: HostBoolArgumentSlot) -> bool {
        self.bools[slot.index()]
    }

    fn nil(&self, slot: HostNilArgumentSlot) {
        self.nils[slot.index()]
    }
}

#[cfg(test)]
mod tests {
    use super::{HostParameter, HostParameterLayout};
    use crate::BitArrayValue;
    use ecow::EcoString;
    use num_bigint::BigInt;

    #[test]
    fn allocates_every_family_local_slot_in_source_order() {
        let mut layout = HostParameterLayout::default();
        let first_int = layout.register::<BigInt>();
        let first_bool = layout.register::<bool>();
        let first_float = layout.register::<f64>();
        let first_string = layout.register::<EcoString>();
        let first_bit_array = layout.register::<BitArrayValue>();
        let first_utf_codepoint = layout.register::<char>();
        let first_nil = layout.register::<()>();
        let second_int = layout.register::<BigInt>();
        let second_bool = layout.register::<bool>();
        let second_float = layout.register::<f64>();
        let second_string = layout.register::<EcoString>();
        let second_bit_array = layout.register::<BitArrayValue>();
        let second_utf_codepoint = layout.register::<char>();
        let second_nil = layout.register::<()>();

        assert_eq!(first_int.index(), 0);
        assert_eq!(first_bool.index(), 0);
        assert_eq!(first_float.index(), 0);
        assert_eq!(first_string.index(), 0);
        assert_eq!(first_bit_array.index(), 0);
        assert_eq!(first_utf_codepoint.index(), 0);
        assert_eq!(first_nil.index(), 0);
        assert_eq!(second_int.index(), 1);
        assert_eq!(second_bool.index(), 1);
        assert_eq!(second_float.index(), 1);
        assert_eq!(second_string.index(), 1);
        assert_eq!(second_bit_array.index(), 1);
        assert_eq!(second_utf_codepoint.index(), 1);
        assert_eq!(second_nil.index(), 1);
        assert_eq!(
            layout.finish().as_ref(),
            [
                HostParameter::Int(first_int),
                HostParameter::Bool(first_bool),
                HostParameter::Float(first_float),
                HostParameter::String(first_string),
                HostParameter::BitArray(first_bit_array),
                HostParameter::UtfCodepoint(first_utf_codepoint),
                HostParameter::Nil(first_nil),
                HostParameter::Int(second_int),
                HostParameter::Bool(second_bool),
                HostParameter::Float(second_float),
                HostParameter::String(second_string),
                HostParameter::BitArray(second_bit_array),
                HostParameter::UtfCodepoint(second_utf_codepoint),
                HostParameter::Nil(second_nil),
            ],
        );
    }
}
