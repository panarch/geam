use super::{HostArgument, HostCallArguments, HostParameter, HostParameterLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostUtfCodepointArgumentSlot(pub(super) usize);

impl HostUtfCodepointArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostArgument for char {
    type Slot = HostUtfCodepointArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostUtfCodepointArgumentSlot(layout.next_utf_codepoint);
        layout.next_utf_codepoint += 1;
        layout.parameters.push(HostParameter::UtfCodepoint(slot));
        slot
    }

    fn read(arguments: &dyn HostCallArguments, slot: Self::Slot) -> Self {
        arguments.utf_codepoint(slot)
    }
}

#[cfg(test)]
mod tests {
    use super::{HostArgument, HostUtfCodepointArgumentSlot};
    use crate::host::function::argument::CallArguments;

    #[test]
    fn reads_utf_codepoint_arguments_from_typed_slots() {
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            vec!['a', 'b'],
            0,
        );

        assert_eq!(
            <char as HostArgument>::read(&arguments, HostUtfCodepointArgumentSlot(1)),
            'b',
        );
    }
}
