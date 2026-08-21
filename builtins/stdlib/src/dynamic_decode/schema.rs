use crate::dict::DictOf;
use crate::dynamic::Dynamic;
use crate::option::{GleamNone, GleamOption, GleamSome};
use crate::result::{GleamError, GleamOk, GleamResult};
use crate::{
    BitArrayValue, HostCustomConstructorAt, HostCustomConstructorDefinition,
    HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList,
    HostCustomFieldListEnd, HostCustomIndex0, HostCustomSchema, HostCustomType, HostFunctionType,
    HostListType, HostTupleType, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
    HostTypeParameter,
};
use ecow::EcoString;
use geam_core::provider_support::HostOpaqueFunctionType;
use num_bigint::BigInt;

pub struct DecodeErrorSchema;

pub struct DecodeErrorDefinition;

pub struct ExpectedField;

pub struct FoundField;

pub struct PathField;

pub(super) type DynamicDict = DictOf<Dynamic, Dynamic>;
pub(super) type DynamicOption = GleamOption<Dynamic>;
pub(super) type DynamicSome = GleamSome<Dynamic>;
pub(super) type DynamicNone = GleamNone<Dynamic>;
pub(super) type IndexKey = HostTypeParameter<0>;
pub(super) type IndexResult = GleamResult<DynamicOption, EcoString>;
pub(super) type IndexOk = GleamOk<DynamicOption, EcoString>;
pub(super) type IndexError = GleamError<DynamicOption, EcoString>;
pub(super) type BareIndexConstructions =
    HostTypeList<Dynamic, HostTypeList<DynamicOption, HostTypeListEnd>>;
pub(super) type BareIndexDynamicIndex = HostTypeIndex0;
pub(super) type BareIndexOptionIndex = HostTypeIndexNext<BareIndexDynamicIndex>;

pub(super) type StringResult = GleamResult<EcoString, EcoString>;
pub(super) type StringOk = GleamOk<EcoString, EcoString>;
pub(super) type StringError = GleamError<EcoString, EcoString>;
pub(super) type IntResult = GleamResult<BigInt, BigInt>;
pub(super) type IntOk = GleamOk<BigInt, BigInt>;
pub(super) type IntError = GleamError<BigInt, BigInt>;
pub(super) type FloatResult = GleamResult<f64, f64>;
pub(super) type FloatOk = GleamOk<f64, f64>;
pub(super) type FloatError = GleamError<f64, f64>;
pub(super) type BitArrayResult = GleamResult<BitArrayValue, BitArrayValue>;
pub(super) type BitArrayOk = GleamOk<BitArrayValue, BitArrayValue>;
pub(super) type BitArrayError = GleamError<BitArrayValue, BitArrayValue>;

pub type DecodeError = HostCustomType<DecodeErrorSchema>;
pub(super) type DecodeErrorConstructor =
    HostCustomConstructorAt<DecodeError, HostCustomIndex0, DecodeErrorDefinition>;
pub(super) type DecodeErrors = HostListType<DecodeError>;

pub(super) type DecodedItem = HostTypeParameter<0>;
pub(super) type PathKey = HostTypeParameter<1>;
pub(super) type DecodedItems = HostListType<DecodedItem>;
pub(super) type ItemDecodeLayerElements =
    HostTypeList<DecodedItem, HostTypeList<DecodeErrors, HostTypeListEnd>>;
pub(super) type ItemDecodeLayer = HostTupleType<ItemDecodeLayerElements>;
pub(super) type DecodeListResultElements =
    HostTypeList<DecodedItems, HostTypeList<DecodeErrors, HostTypeListEnd>>;
pub(super) type DecodeListResult = HostTupleType<DecodeListResultElements>;
pub(super) type DecodeListConstructions = HostTypeList<
    HostListType<EcoString>,
    HostTypeList<
        DecodeError,
        HostTypeList<DecodedItems, HostTypeList<DecodeErrors, HostTypeListEnd>>,
    >,
>;
pub(super) type DecodeListPathIndex = HostTypeIndex0;
pub(super) type DecodeListErrorIndex = HostTypeIndexNext<DecodeListPathIndex>;
pub(super) type DecodeListValuesIndex = HostTypeIndexNext<DecodeListErrorIndex>;
pub(super) type DecodeListErrorsIndex = HostTypeIndexNext<DecodeListValuesIndex>;
pub(super) type ItemDecoderArguments = HostTypeList<Dynamic, HostTypeListEnd>;
pub(super) type ItemDecoder = HostFunctionType<ItemDecoderArguments, ItemDecodeLayer>;
pub(super) type PushPathArguments =
    HostTypeList<ItemDecodeLayer, HostTypeList<PathKey, HostTypeListEnd>>;
pub(super) type PushPath = HostOpaqueFunctionType<PushPathArguments, ItemDecodeLayer>;

pub(super) type DictResult = GleamResult<DynamicDict, ()>;
pub(super) type DictOk = GleamOk<DynamicDict, ()>;
pub(super) type DictError = GleamError<DynamicDict, ()>;

pub(super) type CastValue = HostTypeParameter<0>;

impl HostCustomField for ExpectedField {
    const LABEL: Option<&'static str> = Some("expected");

    type Type = EcoString;
}

impl HostCustomField for FoundField {
    const LABEL: Option<&'static str> = Some("found");

    type Type = EcoString;
}

impl HostCustomField for PathField {
    const LABEL: Option<&'static str> = Some("path");

    type Type = HostListType<EcoString>;
}

impl HostCustomConstructorDefinition for DecodeErrorDefinition {
    const NAME: &'static str = "DecodeError";

    type Fields = HostCustomFieldList<
        ExpectedField,
        HostCustomFieldList<FoundField, HostCustomFieldList<PathField, HostCustomFieldListEnd>>,
    >;
}

impl HostCustomSchema for DecodeErrorSchema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/dynamic/decode";
    const NAME: &'static str = "DecodeError";
    const PARAMETER_COUNT: usize = 0;

    type Constructors =
        HostCustomConstructorList<DecodeErrorDefinition, HostCustomConstructorListEnd>;
}

#[cfg(test)]
mod tests {
    use super::DecodeErrorSchema;
    use crate::{
        HostCustomConstructorSchema, HostCustomFieldSchema, HostCustomTypeSchema, HostSchemaType,
    };

    #[test]
    fn describes_the_exact_official_decode_error_schema() {
        assert_eq!(
            HostCustomTypeSchema::of::<DecodeErrorSchema>(),
            HostCustomTypeSchema::new(
                "gleam_stdlib",
                "gleam/dynamic/decode",
                "DecodeError",
                0,
                [HostCustomConstructorSchema::new(
                    "DecodeError",
                    [
                        HostCustomFieldSchema::new(Some("expected"), HostSchemaType::String),
                        HostCustomFieldSchema::new(Some("found"), HostSchemaType::String),
                        HostCustomFieldSchema::new(
                            Some("path"),
                            HostSchemaType::List(Box::new(HostSchemaType::String)),
                        ),
                    ],
                )],
            ),
        );
    }
}
