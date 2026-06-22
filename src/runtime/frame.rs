use crate::plan::{BoolLocalId, IntLocalId, NilLocalId, StringLocalId};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Default)]
pub(super) struct Frame {
    ints: Vec<BigInt>,
    strings: Vec<EcoString>,
    bools: Vec<bool>,
    nils: usize,
}

impl Frame {
    pub(super) fn set_int(&mut self, local: IntLocalId, value: BigInt) {
        set_slot(&mut self.ints, local.0, value);
    }

    pub(super) fn get_int(&self, local: IntLocalId) -> BigInt {
        self.ints[local.0].clone()
    }

    pub(super) fn set_string(&mut self, local: StringLocalId, value: EcoString) {
        set_slot(&mut self.strings, local.0, value);
    }

    pub(super) fn get_string(&self, local: StringLocalId) -> EcoString {
        self.strings[local.0].clone()
    }

    pub(super) fn set_bool(&mut self, local: BoolLocalId, value: bool) {
        set_slot(&mut self.bools, local.0, value);
    }

    pub(super) fn get_bool(&self, local: BoolLocalId) -> bool {
        self.bools[local.0]
    }

    pub(super) fn set_nil(&mut self, local: NilLocalId) {
        if local.0 == self.nils {
            self.nils += 1;
        }
    }

    pub(super) fn get_nil(&self, _local: NilLocalId) {}
}

fn set_slot<T>(slots: &mut Vec<T>, index: usize, value: T) {
    if index == slots.len() {
        slots.push(value);
    } else {
        slots[index] = value;
    }
}

#[cfg(test)]
mod tests {
    use super::Frame;
    use crate::plan::{BoolLocalId, IntLocalId, NilLocalId, StringLocalId};
    use num_bigint::BigInt;

    #[test]
    fn frame_set_and_get_local() {
        let mut frame = Frame::default();

        frame.set_int(IntLocalId(0), int(1));
        frame.set_string(StringLocalId(0), "geam".into());
        frame.set_bool(BoolLocalId(0), true);
        frame.set_nil(NilLocalId(0));

        assert_eq!(frame.get_int(IntLocalId(0)), int(1));
        assert_eq!(frame.get_string(StringLocalId(0)), "geam");
        assert!(frame.get_bool(BoolLocalId(0)));
        assert_eq!(frame.get_nil(NilLocalId(0)), ());
    }

    #[test]
    fn frame_set_overwrites_local() {
        let mut frame = Frame::default();

        frame.set_int(IntLocalId(0), int(1));
        frame.set_int(IntLocalId(0), int(2));

        assert_eq!(frame.get_int(IntLocalId(0)), int(2));
    }

    fn int(value: i64) -> BigInt {
        BigInt::from(value)
    }
}
