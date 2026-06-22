use crate::plan::LocalId;
use thiserror::Error;

#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum RuntimeError {
    #[error("unbound local: {local:?}")]
    UnboundLocal { local: LocalId },
}
