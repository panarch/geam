use crate::function::provider;
use crate::{HostCustomType, HostListType};
use geam_core::provider::{
    ProviderConstruction, ProviderConstructionIndex0, ProviderConstructionIndexNext,
    ProviderConstructionList, ProviderNoConstructions,
};
use geam_stdlib::provider_support::{DictOf, Dynamic, GleamError, GleamOk, GleamResult};

#[cfg(test)]
pub(super) type JsonSchema = provider::__GeamExternalSchema0;
pub(super) type DecodeErrorSchema = provider::__GeamCustomSchema0;

pub(super) type DecodeError = HostCustomType<DecodeErrorSchema>;
pub(super) type UnexpectedEndOfInput = provider::__GeamCustom0Constructor0;
pub(super) type UnexpectedByte = provider::__GeamCustom0Constructor1;
pub(super) type UnexpectedSequence = provider::__GeamCustom0Constructor2;

pub(super) type JsonDynamicResult = GleamResult<Dynamic, DecodeError>;
pub(super) type JsonDynamicOk = GleamOk<Dynamic, DecodeError>;
pub(super) type JsonDynamicError = GleamError<Dynamic, DecodeError>;

pub(super) type DynamicList = HostListType<Dynamic>;
pub(super) type DynamicDict = DictOf<Dynamic, Dynamic>;

pub(super) type DecodeRequirements = ProviderConstructionList<
    ProviderConstruction<Dynamic>,
    ProviderConstructionList<
        ProviderConstruction<DynamicList>,
        ProviderConstructionList<
            ProviderConstruction<DynamicDict>,
            ProviderConstructionList<ProviderConstruction<DecodeError>, ProviderNoConstructions>,
        >,
    >,
>;
pub(super) type DecodeDynamicIndex = ProviderConstructionIndex0;
pub(super) type DecodeListIndex = ProviderConstructionIndexNext<DecodeDynamicIndex>;
pub(super) type DecodeDictIndex = ProviderConstructionIndexNext<DecodeListIndex>;
pub(super) type DecodeErrorIndex = ProviderConstructionIndexNext<DecodeDictIndex>;

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
