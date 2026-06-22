use crate::plan::{BoolLocalId, IntLocalId, LocalId, NilLocalId, StringLocalId};
use crate::runtime::error::RuntimeError;
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Default)]
pub(super) struct Frame {
    ints: Vec<Option<BigInt>>,
    strings: Vec<Option<EcoString>>,
    bools: Vec<Option<bool>>,
    nils: Vec<Option<()>>,
}

impl Frame {
    pub(super) fn set_int(&mut self, local: IntLocalId, value: BigInt) {
        if self.ints.len() <= local.0 {
            self.ints.resize(local.0 + 1, None);
        }
        self.ints[local.0] = Some(value);
    }

    pub(super) fn get_int(&self, local: IntLocalId) -> Result<BigInt, RuntimeError> {
        self.ints
            .get(local.0)
            .and_then(Clone::clone)
            .ok_or(RuntimeError::UnboundLocal {
                local: LocalId::Int(local),
            })
    }

    pub(super) fn set_string(&mut self, local: StringLocalId, value: EcoString) {
        if self.strings.len() <= local.0 {
            self.strings.resize(local.0 + 1, None);
        }
        self.strings[local.0] = Some(value);
    }

    pub(super) fn get_string(&self, local: StringLocalId) -> Result<EcoString, RuntimeError> {
        self.strings
            .get(local.0)
            .and_then(Clone::clone)
            .ok_or(RuntimeError::UnboundLocal {
                local: LocalId::String(local),
            })
    }

    pub(super) fn set_bool(&mut self, local: BoolLocalId, value: bool) {
        if self.bools.len() <= local.0 {
            self.bools.resize(local.0 + 1, None);
        }
        self.bools[local.0] = Some(value);
    }

    pub(super) fn get_bool(&self, local: BoolLocalId) -> Result<bool, RuntimeError> {
        self.bools
            .get(local.0)
            .and_then(Clone::clone)
            .ok_or(RuntimeError::UnboundLocal {
                local: LocalId::Bool(local),
            })
    }

    pub(super) fn set_nil(&mut self, local: NilLocalId) {
        if self.nils.len() <= local.0 {
            self.nils.resize(local.0 + 1, None);
        }
        self.nils[local.0] = Some(());
    }

    pub(super) fn get_nil(&self, local: NilLocalId) -> Result<(), RuntimeError> {
        self.nils
            .get(local.0)
            .and_then(|value| *value)
            .ok_or(RuntimeError::UnboundLocal {
                local: LocalId::Nil(local),
            })
    }
}

#[cfg(test)]
mod tests {
    use super::{Frame, RuntimeError};
    use crate::plan::{BoolLocalId, IntLocalId, LocalId, NilLocalId, StringLocalId};
    use num_bigint::BigInt;

    #[test]
    fn frame_set_and_get_local() {
        let mut frame = Frame::default();

        frame.set_int(IntLocalId(0), int(1));
        frame.set_string(StringLocalId(0), "geam".into());
        frame.set_bool(BoolLocalId(0), true);
        frame.set_nil(NilLocalId(0));

        assert_eq!(frame.get_int(IntLocalId(0)), Ok(int(1)));
        assert_eq!(frame.get_string(StringLocalId(0)), Ok("geam".into()));
        assert_eq!(frame.get_bool(BoolLocalId(0)), Ok(true));
        assert_eq!(frame.get_nil(NilLocalId(0)), Ok(()));
    }

    #[test]
    fn frame_set_sparse_local() {
        let mut frame = Frame::default();

        frame.set_int(IntLocalId(3), int(5));

        assert_eq!(frame.get_int(IntLocalId(3)), Ok(int(5)));
        assert_eq!(
            frame.get_int(IntLocalId(2)),
            Err(RuntimeError::UnboundLocal {
                local: LocalId::Int(IntLocalId(2)),
            }),
        );
    }

    #[test]
    fn frame_set_sparse_typed_locals() {
        let mut frame = Frame::default();

        frame.set_string(StringLocalId(2), "geam".into());
        frame.set_bool(BoolLocalId(2), true);
        frame.set_nil(NilLocalId(2));

        assert_eq!(frame.get_string(StringLocalId(2)), Ok("geam".into()));
        assert_eq!(frame.get_bool(BoolLocalId(2)), Ok(true));
        assert_eq!(frame.get_nil(NilLocalId(2)), Ok(()));
    }

    #[test]
    fn frame_set_overwrites_local() {
        let mut frame = Frame::default();

        frame.set_int(IntLocalId(0), int(1));
        frame.set_int(IntLocalId(0), int(2));

        assert_eq!(frame.get_int(IntLocalId(0)), Ok(int(2)));
    }

    #[test]
    fn frame_get_unbound_local() {
        let frame = Frame::default();

        assert_eq!(
            frame.get_int(IntLocalId(0)),
            Err(RuntimeError::UnboundLocal {
                local: LocalId::Int(IntLocalId(0)),
            }),
        );
        assert_eq!(
            frame.get_string(StringLocalId(0)),
            Err(RuntimeError::UnboundLocal {
                local: LocalId::String(StringLocalId(0)),
            }),
        );
        assert_eq!(
            frame.get_bool(BoolLocalId(0)),
            Err(RuntimeError::UnboundLocal {
                local: LocalId::Bool(BoolLocalId(0)),
            }),
        );
        assert_eq!(
            frame.get_nil(NilLocalId(0)),
            Err(RuntimeError::UnboundLocal {
                local: LocalId::Nil(NilLocalId(0)),
            }),
        );
    }

    fn int(value: i64) -> BigInt {
        BigInt::from(value)
    }
}
