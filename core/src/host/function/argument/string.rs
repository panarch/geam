use super::{HostArgument, HostCallArguments, HostParameter, HostParameterLayout};
use ecow::EcoString;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostStringArgumentSlot(pub(super) usize);

impl HostStringArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostArgument for EcoString {
    type Slot = HostStringArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostStringArgumentSlot(layout.next_string);
        layout.next_string += 1;
        layout.parameters.push(HostParameter::String(slot));
        slot
    }

    fn read(arguments: &dyn HostCallArguments, slot: Self::Slot) -> Self {
        arguments.string(slot)
    }
}

#[cfg(test)]
mod tests {
    use super::{HostArgument, HostStringArgumentSlot};
    use crate::host::function::argument::CallArguments;
    use ecow::EcoString;

    #[test]
    fn reads_string_arguments_from_typed_slots() {
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            vec![EcoString::from("first"), EcoString::from("second")],
            Vec::new(),
            Vec::new(),
            0,
        );

        assert_eq!(
            <EcoString as HostArgument>::read(&arguments, HostStringArgumentSlot(1)),
            "second",
        );
    }
}
