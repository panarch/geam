use super::{HostArgument, HostCallArguments, HostParameter, HostParameterLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostFloatArgumentSlot(pub(super) usize);

impl HostFloatArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostArgument for f64 {
    type Slot = HostFloatArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostFloatArgumentSlot(layout.next_float);
        layout.next_float += 1;
        layout.parameters.push(HostParameter::Float(slot));
        slot
    }

    fn read(arguments: &dyn HostCallArguments, slot: Self::Slot) -> Self {
        arguments.float(slot)
    }
}

#[cfg(test)]
mod tests {
    use super::{HostArgument, HostFloatArgumentSlot};
    use crate::host::function::argument::CallArguments;

    #[test]
    fn reads_float_arguments_from_typed_slots() {
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            vec![1.0, 2.0],
            Vec::new(),
            Vec::new(),
            Vec::new(),
            0,
        );

        assert_eq!(
            <f64 as HostArgument>::read(&arguments, HostFloatArgumentSlot(1)),
            2.0,
        );
    }
}
