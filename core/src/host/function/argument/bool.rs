use super::{HostArgument, HostCallArguments, HostParameter, HostParameterLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostBoolArgumentSlot(pub(super) usize);

impl HostBoolArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
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
    use super::{HostArgument, HostBoolArgumentSlot};
    use crate::host::function::argument::CallArguments;

    #[test]
    fn reads_bool_arguments_from_typed_slots() {
        let arguments = CallArguments::new(Vec::new(), vec![false, true]);

        assert!(<bool as HostArgument>::read(
            &arguments,
            HostBoolArgumentSlot(1),
        ));
    }
}
