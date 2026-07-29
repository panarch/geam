use super::{HostArgument, HostCallArguments, HostParameter, HostParameterLayout};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostNilArgumentSlot(pub(super) usize);

#[cfg(test)]
impl HostNilArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostArgument for () {
    type Slot = HostNilArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostNilArgumentSlot(layout.next_nil);
        layout.next_nil += 1;
        layout.parameters.push(HostParameter::Nil(slot));
        slot
    }

    fn read(arguments: &dyn HostCallArguments, slot: Self::Slot) -> Self {
        arguments.nil(slot)
    }
}

#[cfg(test)]
mod tests {
    use super::{HostArgument, HostNilArgumentSlot};
    use crate::host::function::argument::CallArguments;

    #[test]
    fn reads_nil_arguments_from_typed_slots() {
        let arguments = CallArguments::new(Vec::new(), Vec::new()).with_scalar_values(
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            2,
        );

        assert_eq!(
            <() as HostArgument>::read(&arguments, HostNilArgumentSlot(1)),
            (),
        );
    }
}
