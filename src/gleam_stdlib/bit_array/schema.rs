use super::super::result::{GleamError, GleamOk, GleamResult};
use crate::{BitArrayValue, HostListType, HostTupleType, HostTypeList, HostTypeListEnd};
use num_bigint::BigInt;

pub(super) type BitArrayResult = GleamResult<BitArrayValue, ()>;
pub(super) type BitArrayOk = GleamOk<BitArrayValue, ()>;
pub(super) type BitArrayError = GleamError<BitArrayValue, ()>;
pub(super) type BitArrayList = HostListType<BitArrayValue>;
pub(super) type IntPairElements = HostTypeList<BigInt, HostTypeList<BigInt, HostTypeListEnd>>;
pub(super) type IntPair = HostTupleType<IntPairElements>;
