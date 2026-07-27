mod bool;
mod int;

use super::HostValueType;
use num_bigint::BigInt;

pub(crate) use bool::HostBoolArgumentSlot;
pub(crate) use int::HostIntArgumentSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostParameter {
    Int(HostIntArgumentSlot),
    Bool(HostBoolArgumentSlot),
}

pub(crate) trait HostCallArguments {
    fn int(&self, slot: HostIntArgumentSlot) -> BigInt;
    fn bool(&self, slot: HostBoolArgumentSlot) -> bool;
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
    next_bool: usize,
}

impl HostParameter {
    pub(crate) fn type_(self) -> HostValueType {
        match self {
            Self::Int(_) => HostValueType::Int,
            Self::Bool(_) => HostValueType::Bool,
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
    bools: Vec<bool>,
}

#[cfg(test)]
impl CallArguments {
    pub(in crate::host) fn new(ints: Vec<BigInt>, bools: Vec<bool>) -> Self {
        Self { ints, bools }
    }
}

#[cfg(test)]
impl HostCallArguments for CallArguments {
    fn int(&self, slot: HostIntArgumentSlot) -> BigInt {
        self.ints[slot.index()].clone()
    }

    fn bool(&self, slot: HostBoolArgumentSlot) -> bool {
        self.bools[slot.index()]
    }
}

#[cfg(test)]
mod tests {
    use super::{HostParameter, HostParameterLayout};
    use num_bigint::BigInt;

    #[test]
    fn allocates_family_local_slots_in_source_order() {
        let mut layout = HostParameterLayout::default();
        let first_int = layout.register::<BigInt>();
        let first_bool = layout.register::<bool>();
        let second_int = layout.register::<BigInt>();
        let second_bool = layout.register::<bool>();

        assert_eq!(first_int.index(), 0);
        assert_eq!(first_bool.index(), 0);
        assert_eq!(second_int.index(), 1);
        assert_eq!(second_bool.index(), 1);
        assert_eq!(
            layout.finish().as_ref(),
            [
                HostParameter::Int(first_int),
                HostParameter::Bool(first_bool),
                HostParameter::Int(second_int),
                HostParameter::Bool(second_bool),
            ],
        );
    }
}
