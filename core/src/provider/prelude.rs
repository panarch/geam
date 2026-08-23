use crate::host::{
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
};

#[doc(hidden)]
pub struct ProviderResultSchema;

#[doc(hidden)]
pub struct ProviderOkDefinition;

#[doc(hidden)]
pub struct ProviderErrorDefinition;

#[doc(hidden)]
pub struct ProviderOkField;

#[doc(hidden)]
pub struct ProviderErrorField;

#[doc(hidden)]
pub type ProviderResult<Success, Failure> = HostCustomType<
    ProviderResultSchema,
    HostTypeList<Success, HostTypeList<Failure, HostTypeListEnd>>,
>;

#[doc(hidden)]
pub type ProviderOk<Success, Failure> = HostCustomConstructorAt<
    ProviderResult<Success, Failure>,
    HostCustomIndex0,
    ProviderOkDefinition,
>;

#[doc(hidden)]
pub type ProviderError<Success, Failure> = HostCustomConstructorAt<
    ProviderResult<Success, Failure>,
    HostCustomIndexNext<HostCustomIndex0>,
    ProviderErrorDefinition,
>;

impl HostCustomField for ProviderOkField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomTypeArgument<HostTypeIndex0>;
}

impl HostCustomConstructorDefinition for ProviderOkDefinition {
    const NAME: &'static str = "Ok";

    type Fields = HostCustomFieldList<ProviderOkField, HostCustomFieldListEnd>;
}

impl HostCustomField for ProviderErrorField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomTypeArgument<HostTypeIndexNext<HostTypeIndex0>>;
}

impl HostCustomConstructorDefinition for ProviderErrorDefinition {
    const NAME: &'static str = "Error";

    type Fields = HostCustomFieldList<ProviderErrorField, HostCustomFieldListEnd>;
}

impl HostCustomSchema for ProviderResultSchema {
    const PACKAGE: &'static str = "";
    const MODULE: &'static str = "gleam";
    const NAME: &'static str = "Result";
    const PARAMETER_COUNT: usize = 2;

    type Constructors = HostCustomConstructorList<
        ProviderOkDefinition,
        HostCustomConstructorList<ProviderErrorDefinition, HostCustomConstructorListEnd>,
    >;
}

#[doc(hidden)]
pub struct ProviderOptionSchema;

#[doc(hidden)]
pub struct ProviderSomeDefinition;

#[doc(hidden)]
pub struct ProviderNoneDefinition;

#[doc(hidden)]
pub struct ProviderSomeField;

#[doc(hidden)]
pub type ProviderOption<Value> =
    HostCustomType<ProviderOptionSchema, HostTypeList<Value, HostTypeListEnd>>;

#[doc(hidden)]
pub type ProviderSome<Value> =
    HostCustomConstructorAt<ProviderOption<Value>, HostCustomIndex0, ProviderSomeDefinition>;

#[doc(hidden)]
pub type ProviderNone<Value> = HostCustomConstructorAt<
    ProviderOption<Value>,
    HostCustomIndexNext<HostCustomIndex0>,
    ProviderNoneDefinition,
>;

impl HostCustomField for ProviderSomeField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomTypeArgument<HostTypeIndex0>;
}

impl HostCustomConstructorDefinition for ProviderSomeDefinition {
    const NAME: &'static str = "Some";

    type Fields = HostCustomFieldList<ProviderSomeField, HostCustomFieldListEnd>;
}

impl HostCustomConstructorDefinition for ProviderNoneDefinition {
    const NAME: &'static str = "None";

    type Fields = HostCustomFieldListEnd;
}

impl HostCustomSchema for ProviderOptionSchema {
    const PACKAGE: &'static str = "gleam_stdlib";
    const MODULE: &'static str = "gleam/option";
    const NAME: &'static str = "Option";
    const PARAMETER_COUNT: usize = 1;

    type Constructors = HostCustomConstructorList<
        ProviderSomeDefinition,
        HostCustomConstructorList<ProviderNoneDefinition, HostCustomConstructorListEnd>,
    >;
}

#[cfg(test)]
mod tests {
    use super::{ProviderOptionSchema, ProviderResultSchema};
    use crate::{
        HostCustomConstructorSchema, HostCustomFieldSchema, HostCustomTypeSchema, HostSchemaType,
    };

    #[test]
    fn describes_the_exact_prelude_result_and_option_schemas() {
        assert_eq!(
            HostCustomTypeSchema::of::<ProviderResultSchema>(),
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
        assert_eq!(
            HostCustomTypeSchema::of::<ProviderOptionSchema>(),
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
