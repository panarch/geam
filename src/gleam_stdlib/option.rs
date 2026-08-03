use crate::{
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostTypeIndex0, HostTypeList, HostTypeListEnd,
};

pub(in crate::gleam_stdlib) struct OptionSchema;

pub(in crate::gleam_stdlib) struct SomeDefinition;

pub(in crate::gleam_stdlib) struct NoneDefinition;

pub(in crate::gleam_stdlib) struct SomeField;

type Arguments<Value> = HostTypeList<Value, HostTypeListEnd>;
pub(in crate::gleam_stdlib) type GleamOption<Value> =
    HostCustomType<OptionSchema, Arguments<Value>>;
pub(in crate::gleam_stdlib) type GleamSome<Value> =
    HostCustomConstructorAt<GleamOption<Value>, HostCustomIndex0, SomeDefinition>;
pub(in crate::gleam_stdlib) type GleamNone<Value> = HostCustomConstructorAt<
    GleamOption<Value>,
    HostCustomIndexNext<HostCustomIndex0>,
    NoneDefinition,
>;

impl HostCustomField for SomeField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomTypeArgument<HostTypeIndex0>;
}

impl HostCustomConstructorDefinition for SomeDefinition {
    const NAME: &'static str = "Some";

    type Fields = HostCustomFieldList<SomeField, HostCustomFieldListEnd>;
}

impl HostCustomConstructorDefinition for NoneDefinition {
    const NAME: &'static str = "None";

    type Fields = HostCustomFieldListEnd;
}

impl HostCustomSchema for OptionSchema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/option";
    const NAME: &'static str = "Option";
    const PARAMETER_COUNT: usize = 1;

    type Constructors = HostCustomConstructorList<
        SomeDefinition,
        HostCustomConstructorList<NoneDefinition, HostCustomConstructorListEnd>,
    >;
}

#[cfg(test)]
mod tests {
    use super::OptionSchema;
    use crate::{
        HostCustomConstructorSchema, HostCustomFieldSchema, HostCustomTypeSchema, HostSchemaType,
    };

    #[test]
    fn describes_the_exact_official_option_schema() {
        assert_eq!(
            HostCustomTypeSchema::of::<OptionSchema>(),
            HostCustomTypeSchema::new(
                "gleam_stdlib",
                "gleam/option",
                "Option",
                1,
                [
                    HostCustomConstructorSchema::new(
                        "Some",
                        [HostCustomFieldSchema::new(
                            None::<&str>,
                            HostSchemaType::parameter(0),
                        )],
                    ),
                    HostCustomConstructorSchema::new("None", Vec::new()),
                ],
            ),
        );
    }
}
