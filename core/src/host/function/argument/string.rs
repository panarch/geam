use super::{HostArgument, HostCallArguments, HostParameter, HostParameterLayout};
use crate::StringValue;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostStringArgumentSlot(pub(super) usize);

impl HostStringArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostArgument for StringValue {
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
    use crate::StringValue;
    use crate::host::function::argument::CallArguments;

    #[test]
    fn reads_string_arguments_from_typed_slots() {
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            vec![StringValue::from("first"), StringValue::from("second")],
            Vec::new(),
            Vec::new(),
            0,
        );

        assert_eq!(
            <StringValue as HostArgument>::read(&arguments, HostStringArgumentSlot(1)),
            "second",
        );
    }
}
