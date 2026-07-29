use super::{HostArgument, HostCallArguments, HostParameter, HostParameterLayout};
use crate::BitArrayValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostBitArrayArgumentSlot(pub(super) usize);

impl HostBitArrayArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostArgument for BitArrayValue {
    type Slot = HostBitArrayArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostBitArrayArgumentSlot(layout.next_bit_array);
        layout.next_bit_array += 1;
        layout.parameters.push(HostParameter::BitArray(slot));
        slot
    }

    fn read(arguments: &dyn HostCallArguments, slot: Self::Slot) -> Self {
        arguments.bit_array(slot)
    }
}

#[cfg(test)]
mod tests {
    use super::{HostArgument, HostBitArrayArgumentSlot};
    use crate::BitArrayValue;
    use crate::host::function::argument::CallArguments;

    #[test]
    fn reads_bit_array_arguments_from_typed_slots() {
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            Vec::new(),
            vec![
                BitArrayValue::from_bytes(vec![1]),
                BitArrayValue::from_bytes(vec![2]),
            ],
            Vec::new(),
            0,
        );

        assert_eq!(
            <BitArrayValue as HostArgument>::read(&arguments, HostBitArrayArgumentSlot(1)),
            BitArrayValue::from_bytes(vec![2]),
        );
    }
}
