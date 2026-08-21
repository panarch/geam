use crate::result::{GleamError, GleamOk, GleamResult};
use num_bigint::BigInt;

pub(super) type ParseResult = GleamResult<BigInt, ()>;
pub(super) type ParseOk = GleamOk<BigInt, ()>;
pub(super) type ParseError = GleamError<BigInt, ()>;
