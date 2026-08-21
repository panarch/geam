use crate::{
    HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
    HostCustomFieldListEnd, HostCustomSchema, HostCustomType, HostExternalSchema, HostExternalType,
    HostListType, HostTypeIndex0, HostTypeList, HostTypeListEnd,
};
use ecow::EcoString;

pub struct StringTreeSchema;

pub(super) struct DirectionSchema;

pub(super) struct AllDefinition;

pub type StringTree = HostExternalType<StringTreeSchema>;
pub(super) type StringTreeList = HostListType<StringTree>;
pub(super) type StringList = HostListType<EcoString>;
pub(super) type Direction = HostCustomType<DirectionSchema>;
pub(super) type SplitConstructions = HostTypeList<StringTree, HostTypeListEnd>;
pub(super) type SplitStringTreeIndex = HostTypeIndex0;

impl HostExternalSchema for StringTreeSchema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/string_tree";
    const NAME: &'static str = "StringTree";
    const PARAMETER_COUNT: usize = 0;
}

impl HostCustomConstructorDefinition for AllDefinition {
    const NAME: &'static str = "All";

    type Fields = HostCustomFieldListEnd;
}

impl HostCustomSchema for DirectionSchema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/string_tree";
    const NAME: &'static str = "Direction";
    const PARAMETER_COUNT: usize = 0;

    type Constructors = HostCustomConstructorList<AllDefinition, HostCustomConstructorListEnd>;
}

#[cfg(test)]
mod tests {
    use super::{DirectionSchema, StringTreeSchema};
    use crate::{HostCustomConstructorSchema, HostCustomTypeSchema, HostExternalTypeSchema};

    #[test]
    fn describes_the_exact_string_tree_and_private_direction_schemas() {
        assert_eq!(
            HostExternalTypeSchema::of::<StringTreeSchema>(),
            HostExternalTypeSchema::new("gleam_stdlib", "gleam/string_tree", "StringTree", 0),
        );
        assert_eq!(
            HostCustomTypeSchema::of::<DirectionSchema>(),
            HostCustomTypeSchema::new(
                "gleam_stdlib",
                "gleam/string_tree",
                "Direction",
                0,
                [HostCustomConstructorSchema::new("All", Vec::new(),)],
            ),
        );
    }
}
