use crate::{
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostExternalSchema, HostExternalType, HostFunctionType, HostStoredType,
    HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd, HostTypeParameter,
};

pub(super) struct DictSchema;

pub(super) struct TransientDictSchema;

pub(super) struct ResultSchema;

pub(super) struct ResultOkField;

pub(super) struct ResultOkDefinition;

pub(super) struct ResultErrorField;

pub(super) struct ResultErrorDefinition;

pub(super) type KeyIndex = HostTypeIndex0;
pub(super) type ItemIndex = HostTypeIndexNext<KeyIndex>;
pub(super) type StoredKey = HostStoredType<KeyIndex>;
pub(super) type StoredValue = HostStoredType<ItemIndex>;

// Function parameters follow the planner's return-first canonical order.
pub(super) type Key = HostTypeParameter<0>;
pub(super) type Item = HostTypeParameter<1>;
type DictArguments = HostTypeList<Key, HostTypeList<Item, HostTypeListEnd>>;
pub(super) type Dict = HostExternalType<DictSchema, DictArguments>;
pub(super) type TransientDict = HostExternalType<TransientDictSchema, DictArguments>;
pub(super) type UpdateFunctionArguments = HostTypeList<Item, HostTypeListEnd>;
pub(super) type UpdateFunction = HostFunctionType<UpdateFunctionArguments, Item>;

type GetItem = HostTypeParameter<0>;
pub(super) type GetKey = HostTypeParameter<1>;
type GetDictArguments = HostTypeList<GetKey, HostTypeList<GetItem, HostTypeListEnd>>;
pub(super) type GetDict = HostExternalType<DictSchema, GetDictArguments>;
type GetResultArguments = HostTypeList<GetItem, HostTypeList<(), HostTypeListEnd>>;
pub(super) type GetResult = HostCustomType<ResultSchema, GetResultArguments>;
pub(super) type GetOk = HostCustomConstructorAt<GetResult, HostCustomIndex0, ResultOkDefinition>;
pub(super) type GetError = HostCustomConstructorAt<
    GetResult,
    HostCustomIndexNext<HostCustomIndex0>,
    ResultErrorDefinition,
>;

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

impl HostCustomField for ResultOkField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomTypeArgument<HostTypeIndex0>;
}

impl HostCustomConstructorDefinition for ResultOkDefinition {
    const NAME: &'static str = "Ok";

    type Fields = HostCustomFieldList<ResultOkField, HostCustomFieldListEnd>;
}

impl HostCustomField for ResultErrorField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomTypeArgument<HostTypeIndexNext<HostTypeIndex0>>;
}

impl HostCustomConstructorDefinition for ResultErrorDefinition {
    const NAME: &'static str = "Error";

    type Fields = HostCustomFieldList<ResultErrorField, HostCustomFieldListEnd>;
}

impl HostCustomSchema for ResultSchema {
    const PACKAGE: &'static str = "";
    const MODULE: &'static str = "gleam";
    const NAME: &'static str = "Result";
    const PARAMETER_COUNT: usize = 2;

    type Constructors = HostCustomConstructorList<
        ResultOkDefinition,
        HostCustomConstructorList<ResultErrorDefinition, HostCustomConstructorListEnd>,
    >;
}

#[cfg(test)]
mod tests {
    use super::{DictSchema, ResultSchema, TransientDictSchema};
    use crate::{
        HostCustomConstructorSchema, HostCustomFieldSchema, HostCustomTypeSchema,
        HostExternalTypeSchema, HostSchemaType,
    };

    #[test]
    fn describes_the_exact_dict_transient_and_result_schemas() {
        assert_eq!(
            HostExternalTypeSchema::of::<DictSchema>(),
            HostExternalTypeSchema::new("gleam_stdlib", "gleam/dict", "Dict", 2),
        );
        assert_eq!(
            HostExternalTypeSchema::of::<TransientDictSchema>(),
            HostExternalTypeSchema::new("gleam_stdlib", "gleam/dict", "TransientDict", 2),
        );
        assert_eq!(
            HostCustomTypeSchema::of::<ResultSchema>(),
            HostCustomTypeSchema::new(
                "",
                "gleam",
                "Result",
                2,
                [
                    HostCustomConstructorSchema::new(
                        "Ok",
                        [HostCustomFieldSchema::new(
                            None::<&str>,
                            HostSchemaType::parameter(0),
                        )],
                    ),
                    HostCustomConstructorSchema::new(
                        "Error",
                        [HostCustomFieldSchema::new(
                            None::<&str>,
                            HostSchemaType::parameter(1),
                        )],
                    ),
                ],
            ),
        );
    }
}
