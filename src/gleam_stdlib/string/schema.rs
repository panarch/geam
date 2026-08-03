use super::super::result::{GleamError, GleamOk, GleamResult};
use crate::{
    HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
    HostCustomFieldListEnd, HostCustomSchema, HostCustomType, HostListType, HostTupleType,
    HostTypeList, HostTypeListEnd, HostTypeParameter,
};
use ecow::EcoString;

pub(super) struct DirectionSchema;

pub(super) struct LeadingDefinition;

pub(super) struct TrailingDefinition;

pub(super) type Direction = HostCustomType<DirectionSchema>;
pub(super) type StringList = HostListType<EcoString>;
pub(super) type UtfCodepointList = HostListType<char>;
pub(super) type StringPairElements =
    HostTypeList<EcoString, HostTypeList<EcoString, HostTypeListEnd>>;
pub(super) type StringPair = HostTupleType<StringPairElements>;
pub(super) type PopResult = GleamResult<StringPair, ()>;
pub(super) type PopOk = GleamOk<StringPair, ()>;
pub(super) type PopError = GleamError<StringPair, ()>;
pub(super) type InspectValue = HostTypeParameter<0>;

impl HostCustomConstructorDefinition for LeadingDefinition {
    const NAME: &'static str = "Leading";

    type Fields = HostCustomFieldListEnd;
}

impl HostCustomConstructorDefinition for TrailingDefinition {
    const NAME: &'static str = "Trailing";

    type Fields = HostCustomFieldListEnd;
}

impl HostCustomSchema for DirectionSchema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/string";
    const NAME: &'static str = "Direction";
    const PARAMETER_COUNT: usize = 0;

    type Constructors = HostCustomConstructorList<
        LeadingDefinition,
        HostCustomConstructorList<TrailingDefinition, HostCustomConstructorListEnd>,
    >;
}

#[cfg(test)]
mod tests {
    use super::DirectionSchema;
    use crate::{HostCustomConstructorSchema, HostCustomTypeSchema};

    #[test]
    fn describes_the_exact_private_trim_direction_schema() {
        assert_eq!(
            HostCustomTypeSchema::of::<DirectionSchema>(),
            HostCustomTypeSchema::new(
                "gleam_stdlib",
                "gleam/string",
                "Direction",
                0,
                [
                    HostCustomConstructorSchema::new("Leading", Vec::new()),
                    HostCustomConstructorSchema::new("Trailing", Vec::new()),
                ],
            ),
        );
    }
}
