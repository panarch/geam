use super::HostValueType;
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostParameter {
    Int(HostIntArgumentSlot),
    Bool(HostBoolArgumentSlot),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostIntArgumentSlot(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostBoolArgumentSlot(usize);

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

impl HostIntArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostBoolArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
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

impl HostArgument for BigInt {
    type Slot = HostIntArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostIntArgumentSlot(layout.next_int);
        layout.next_int += 1;
        layout.parameters.push(HostParameter::Int(slot));
        slot
    }

    fn read(arguments: &dyn HostCallArguments, slot: Self::Slot) -> Self {
        arguments.int(slot)
    }
}

impl HostArgument for bool {
    type Slot = HostBoolArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostBoolArgumentSlot(layout.next_bool);
        layout.next_bool += 1;
        layout.parameters.push(HostParameter::Bool(slot));
        slot
    }

    fn read(arguments: &dyn HostCallArguments, slot: Self::Slot) -> Self {
        arguments.bool(slot)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostArgument, HostBoolArgumentSlot, HostCallArguments, HostIntArgumentSlot, HostParameter,
        HostParameterLayout,
    };
    use num_bigint::BigInt;

    struct Arguments {
        ints: Vec<BigInt>,
        bools: Vec<bool>,
    }

    impl HostCallArguments for Arguments {
        fn int(&self, slot: HostIntArgumentSlot) -> BigInt {
            self.ints[slot.index()].clone()
        }

        fn bool(&self, slot: HostBoolArgumentSlot) -> bool {
            self.bools[slot.index()]
        }
    }

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

    #[test]
    fn int_and_bool_arguments_read_their_typed_slots() {
        let arguments = Arguments {
            ints: vec![10.into(), 20.into()],
            bools: vec![false, true],
        };

        assert_eq!(
            <BigInt as HostArgument>::read(&arguments, HostIntArgumentSlot(1)),
            BigInt::from(20),
        );
        assert!(<bool as HostArgument>::read(
            &arguments,
            HostBoolArgumentSlot(1),
        ));
    }
}
