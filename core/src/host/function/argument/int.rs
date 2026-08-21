use super::{HostArgument, HostCallArguments, HostParameter, HostParameterLayout};
use num_bigint::BigInt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostIntArgumentSlot(pub(super) usize);

impl HostIntArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
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

#[cfg(test)]
mod tests {
    use super::{HostArgument, HostIntArgumentSlot};
    use crate::host::function::argument::CallArguments;
    use num_bigint::BigInt;

    #[test]
    fn reads_int_arguments_from_typed_slots() {
        let arguments = CallArguments::new(vec![BigInt::from(10), BigInt::from(20)], Vec::new());

        assert_eq!(
            <BigInt as HostArgument>::read(&arguments, HostIntArgumentSlot(1)),
            BigInt::from(20),
        );
    }
}
