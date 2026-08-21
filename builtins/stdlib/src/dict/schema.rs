use crate::result::{GleamError, GleamOk, GleamResult};
use crate::{
    HostExternalSchema, HostExternalType, HostFunctionType, HostStoredType, HostTypeIndex0,
    HostTypeIndexNext, HostTypeList, HostTypeListEnd, HostTypeParameter,
};

pub struct DictSchema;

pub(super) struct TransientDictSchema;

pub(super) type KeyIndex = HostTypeIndex0;
pub(super) type ItemIndex = HostTypeIndexNext<KeyIndex>;
pub(super) type StoredKey = HostStoredType<KeyIndex>;
pub(super) type StoredValue = HostStoredType<ItemIndex>;

// Function parameters follow the planner's return-first canonical order.
pub(super) type Key = HostTypeParameter<0>;
pub(super) type Item = HostTypeParameter<1>;
pub type DictOf<Key, Item> =
    HostExternalType<DictSchema, HostTypeList<Key, HostTypeList<Item, HostTypeListEnd>>>;
pub(super) type Dict = DictOf<Key, Item>;
pub(super) type TransientDict =
    HostExternalType<TransientDictSchema, HostTypeList<Key, HostTypeList<Item, HostTypeListEnd>>>;
pub(super) type UpdateFunctionArguments = HostTypeList<Item, HostTypeListEnd>;
pub(super) type UpdateFunction = HostFunctionType<UpdateFunctionArguments, Item>;

type GetItem = HostTypeParameter<0>;
pub(super) type GetKey = HostTypeParameter<1>;
type GetDictArguments = HostTypeList<GetKey, HostTypeList<GetItem, HostTypeListEnd>>;
pub(super) type GetDict = HostExternalType<DictSchema, GetDictArguments>;
pub(super) type GetResult = GleamResult<GetItem, ()>;
pub(super) type GetOk = GleamOk<GetItem, ()>;
pub(super) type GetError = GleamError<GetItem, ()>;

type MapKey = HostTypeParameter<0>;
pub(super) type MapOutput = HostTypeParameter<1>;
type MapInput = HostTypeParameter<2>;
type MapInputDictArguments = HostTypeList<MapKey, HostTypeList<MapInput, HostTypeListEnd>>;
pub(super) type MapInputDict = HostExternalType<DictSchema, MapInputDictArguments>;
type MapOutputDictArguments = HostTypeList<MapKey, HostTypeList<MapOutput, HostTypeListEnd>>;
pub(super) type MapOutputDict = HostExternalType<DictSchema, MapOutputDictArguments>;
pub(super) type MapFunctionArguments =
    HostTypeList<MapKey, HostTypeList<MapInput, HostTypeListEnd>>;
pub(super) type MapFunction = HostFunctionType<MapFunctionArguments, MapOutput>;

pub(super) type FoldAccumulator = HostTypeParameter<0>;
type FoldKey = HostTypeParameter<1>;
type FoldValue = HostTypeParameter<2>;
type FoldDictArguments = HostTypeList<FoldKey, HostTypeList<FoldValue, HostTypeListEnd>>;
pub(super) type FoldDict = HostExternalType<DictSchema, FoldDictArguments>;
pub(super) type FoldFunctionArguments =
    HostTypeList<FoldKey, HostTypeList<FoldValue, HostTypeList<FoldAccumulator, HostTypeListEnd>>>;
pub(super) type FoldFunction = HostFunctionType<FoldFunctionArguments, FoldAccumulator>;

impl HostExternalSchema for DictSchema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/dict";
    const NAME: &'static str = "Dict";
    const PARAMETER_COUNT: usize = 2;
}

impl HostExternalSchema for TransientDictSchema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/dict";
    const NAME: &'static str = "TransientDict";
    const PARAMETER_COUNT: usize = 2;
}

#[cfg(test)]
mod tests {
    use super::{DictSchema, TransientDictSchema};
    use crate::HostExternalTypeSchema;

    #[test]
    fn describes_the_exact_dict_and_transient_schemas() {
        assert_eq!(
            HostExternalTypeSchema::of::<DictSchema>(),
            HostExternalTypeSchema::new("gleam_stdlib", "gleam/dict", "Dict", 2),
        );
        assert_eq!(
            HostExternalTypeSchema::of::<TransientDictSchema>(),
            HostExternalTypeSchema::new("gleam_stdlib", "gleam/dict", "TransientDict", 2),
        );
    }
}
