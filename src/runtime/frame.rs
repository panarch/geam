use crate::plan::{LocalId, Value};
use crate::runtime::error::RuntimeError;

#[derive(Default)]
pub(super) struct Frame {
    values: Vec<Option<Value>>,
}

impl Frame {
    pub(super) fn set(&mut self, local: LocalId, value: Value) {
        if self.values.len() <= local.0 {
            self.values.resize_with(local.0 + 1, || None);
        }
        self.values[local.0] = Some(value);
    }

    pub(super) fn get(&self, local: LocalId) -> Result<Value, RuntimeError> {
        self.values
            .get(local.0)
            .and_then(Clone::clone)
            .ok_or(RuntimeError::UnboundLocal { local })
    }
}

#[cfg(test)]
mod tests {
    use super::{Frame, RuntimeError};
    use crate::plan::{LocalId, Value};
    use num_bigint::BigInt;

    #[test]
    fn frame_set_and_get_local() {
        let mut frame = Frame::default();

        frame.set(LocalId(0), int(1));

        assert_eq!(frame.get(LocalId(0)), Ok(int(1)));
    }

    #[test]
    fn frame_set_sparse_local() {
        let mut frame = Frame::default();

        frame.set(LocalId(3), int(5));

        assert_eq!(frame.get(LocalId(3)), Ok(int(5)));
        assert_eq!(
            frame.get(LocalId(2)),
            Err(RuntimeError::UnboundLocal { local: LocalId(2) }),
        );
    }

    #[test]
    fn frame_set_overwrites_local() {
        let mut frame = Frame::default();

        frame.set(LocalId(0), int(1));
        frame.set(LocalId(0), int(2));

        assert_eq!(frame.get(LocalId(0)), Ok(int(2)));
    }

    #[test]
    fn frame_get_unbound_local() {
        let frame = Frame::default();

        assert_eq!(
            frame.get(LocalId(0)),
            Err(RuntimeError::UnboundLocal { local: LocalId(0) }),
        );
    }

    fn int(value: i64) -> Value {
        Value::Int(BigInt::from(value))
    }
}
