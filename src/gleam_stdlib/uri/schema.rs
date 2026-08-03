use super::super::result::{GleamError, GleamOk, GleamResult};
use crate::{HostListType, HostTupleType, HostTypeList, HostTypeListEnd};
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) type CodeunitPairElements =
    HostTypeList<BigInt, HostTypeList<EcoString, HostTypeListEnd>>;
pub(super) type CodeunitPair = HostTupleType<CodeunitPairElements>;

pub(super) type QueryPairElements =
    HostTypeList<EcoString, HostTypeList<EcoString, HostTypeListEnd>>;
pub(super) type QueryPair = HostTupleType<QueryPairElements>;
pub(super) type QueryPairs = HostListType<QueryPair>;
pub(super) type QueryResult = GleamResult<QueryPairs, ()>;
pub(super) type QueryOk = GleamOk<QueryPairs, ()>;
pub(super) type QueryError = GleamError<QueryPairs, ()>;

pub(super) type PercentDecodeResult = GleamResult<EcoString, ()>;
pub(super) type PercentDecodeOk = GleamOk<EcoString, ()>;
pub(super) type PercentDecodeError = GleamError<EcoString, ()>;
