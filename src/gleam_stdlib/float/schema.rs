use crate::gleam_stdlib::result::{GleamError, GleamOk, GleamResult};

pub(super) type ParseResult = GleamResult<f64, ()>;
pub(super) type ParseOk = GleamOk<f64, ()>;
pub(super) type ParseError = GleamError<f64, ()>;
