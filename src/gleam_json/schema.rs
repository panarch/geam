use crate::gleam_stdlib::{DictOf, Dynamic, DynamicDecodeError, GleamError, GleamOk, GleamResult};
use crate::{
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType, HostExternalSchema,
    HostExternalType, HostListType, HostTupleType, HostTypeList, HostTypeListEnd,
};
use ecow::EcoString;

pub(super) struct JsonSchema;

pub(super) struct DecodeErrorSchema;
pub(super) struct UnexpectedEndOfInputDefinition;
pub(super) struct UnexpectedByteDefinition;
pub(super) struct UnexpectedSequenceDefinition;
pub(super) struct UnableToDecodeDefinition;
pub(super) struct UnexpectedByteField;
pub(super) struct UnexpectedSequenceField;
pub(super) struct UnableToDecodeField;

pub(super) type Json = HostExternalType<JsonSchema>;
pub(super) type JsonList = HostListType<Json>;

pub(super) type DecodeError = HostCustomType<DecodeErrorSchema>;
pub(super) type UnexpectedEndOfInput =
    HostCustomConstructorAt<DecodeError, HostCustomIndex0, UnexpectedEndOfInputDefinition>;
pub(super) type UnexpectedByte = HostCustomConstructorAt<
    DecodeError,
    HostCustomIndexNext<HostCustomIndex0>,
    UnexpectedByteDefinition,
>;
pub(super) type UnexpectedSequence = HostCustomConstructorAt<
    DecodeError,
    HostCustomIndexNext<HostCustomIndexNext<HostCustomIndex0>>,
    UnexpectedSequenceDefinition,
>;
pub(super) type JsonDynamicResult = GleamResult<Dynamic, DecodeError>;
pub(super) type JsonDynamicOk = GleamOk<Dynamic, DecodeError>;
pub(super) type JsonDynamicError = GleamError<Dynamic, DecodeError>;

pub(super) type DynamicList = HostListType<Dynamic>;
pub(super) type DynamicDict = DictOf<EcoString, Dynamic>;
pub(super) type DecodeConstructions =
    HostTypeList<DynamicList, HostTypeList<DynamicDict, HostTypeListEnd>>;

pub(super) type ObjectEntryElements = HostTypeList<EcoString, HostTypeList<Json, HostTypeListEnd>>;
pub(super) type ObjectEntry = HostTupleType<ObjectEntryElements>;
pub(super) type ObjectEntries = HostListType<ObjectEntry>;

impl HostExternalSchema for JsonSchema {
    const PACKAGE: &'static str = "gleam_json";
    const MODULE: &'static str = "gleam/json";
    const NAME: &'static str = "Json";
    const PARAMETER_COUNT: usize = 0;
}

impl HostCustomConstructorDefinition for UnexpectedEndOfInputDefinition {
    const NAME: &'static str = "UnexpectedEndOfInput";

    type Fields = HostCustomFieldListEnd;
}

impl HostCustomField for UnexpectedByteField {
    const LABEL: Option<&'static str> = None;

    type Type = EcoString;
}

impl HostCustomConstructorDefinition for UnexpectedByteDefinition {
    const NAME: &'static str = "UnexpectedByte";

    type Fields = HostCustomFieldList<UnexpectedByteField, HostCustomFieldListEnd>;
}

impl HostCustomField for UnexpectedSequenceField {
    const LABEL: Option<&'static str> = None;

    type Type = EcoString;
}

impl HostCustomConstructorDefinition for UnexpectedSequenceDefinition {
    const NAME: &'static str = "UnexpectedSequence";

    type Fields = HostCustomFieldList<UnexpectedSequenceField, HostCustomFieldListEnd>;
}

impl HostCustomField for UnableToDecodeField {
    const LABEL: Option<&'static str> = None;

    type Type = HostListType<DynamicDecodeError>;
}

impl HostCustomConstructorDefinition for UnableToDecodeDefinition {
    const NAME: &'static str = "UnableToDecode";

    type Fields = HostCustomFieldList<UnableToDecodeField, HostCustomFieldListEnd>;
}

impl HostCustomSchema for DecodeErrorSchema {
    const PACKAGE: &'static str = "gleam_json";
    const MODULE: &'static str = "gleam/json";
    const NAME: &'static str = "DecodeError";
    const PARAMETER_COUNT: usize = 0;

    type Constructors = HostCustomConstructorList<
        UnexpectedEndOfInputDefinition,
        HostCustomConstructorList<
            UnexpectedByteDefinition,
            HostCustomConstructorList<
                UnexpectedSequenceDefinition,
                HostCustomConstructorList<UnableToDecodeDefinition, HostCustomConstructorListEnd>,
            >,
        >,
    >;
}

#[cfg(test)]
mod tests {
    use super::{DecodeErrorSchema, JsonSchema};
    use crate::{
        HostCustomConstructorSchema, HostCustomFieldSchema, HostCustomTypeSchema,
        HostExternalTypeSchema, HostSchemaType,
    };

    #[test]
    fn describes_the_exact_official_json_and_decode_error_schemas() {
        assert_eq!(
            HostExternalTypeSchema::of::<JsonSchema>(),
            HostExternalTypeSchema::new("gleam_json", "gleam/json", "Json", 0),
        );
        assert_eq!(
            HostCustomTypeSchema::of::<DecodeErrorSchema>(),
            HostCustomTypeSchema::new(
                "gleam_json",
                "gleam/json",
                "DecodeError",
                0,
                [
                    HostCustomConstructorSchema::new(
                        "UnexpectedEndOfInput",
                        Vec::<HostCustomFieldSchema>::new(),
                    ),
                    HostCustomConstructorSchema::new(
                        "UnexpectedByte",
                        [HostCustomFieldSchema::new(
                            None::<&str>,
                            HostSchemaType::String,
                        )],
                    ),
                    HostCustomConstructorSchema::new(
                        "UnexpectedSequence",
                        [HostCustomFieldSchema::new(
                            None::<&str>,
                            HostSchemaType::String,
                        )],
                    ),
                    HostCustomConstructorSchema::new(
                        "UnableToDecode",
                        [HostCustomFieldSchema::new(
                            None::<&str>,
                            HostSchemaType::list(HostSchemaType::custom(
                                "gleam_stdlib",
                                "gleam/dynamic/decode",
                                "DecodeError",
                                Vec::new(),
                            )),
                        )],
                    ),
                ],
            ),
        );
    }
}
