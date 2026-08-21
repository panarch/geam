use crate::{
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
};

pub struct ResultSchema;

pub struct OkDefinition;

pub struct ErrorDefinition;

pub struct OkField;

pub struct ErrorField;

type Arguments<Success, Failure> = HostTypeList<Success, HostTypeList<Failure, HostTypeListEnd>>;
pub type GleamResult<Success, Failure> = HostCustomType<ResultSchema, Arguments<Success, Failure>>;
pub type GleamOk<Success, Failure> =
    HostCustomConstructorAt<GleamResult<Success, Failure>, HostCustomIndex0, OkDefinition>;
pub type GleamError<Success, Failure> = HostCustomConstructorAt<
    GleamResult<Success, Failure>,
    HostCustomIndexNext<HostCustomIndex0>,
    ErrorDefinition,
>;

impl HostCustomField for OkField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomTypeArgument<HostTypeIndex0>;
}

impl HostCustomConstructorDefinition for OkDefinition {
    const NAME: &'static str = "Ok";

    type Fields = HostCustomFieldList<OkField, HostCustomFieldListEnd>;
}

impl HostCustomField for ErrorField {
    const LABEL: Option<&'static str> = None;

    type Type = HostCustomTypeArgument<HostTypeIndexNext<HostTypeIndex0>>;
}

impl HostCustomConstructorDefinition for ErrorDefinition {
    const NAME: &'static str = "Error";

    type Fields = HostCustomFieldList<ErrorField, HostCustomFieldListEnd>;
}

impl HostCustomSchema for ResultSchema {
    const PACKAGE: &'static str = "";
    const MODULE: &'static str = "gleam";
    const NAME: &'static str = "Result";
    const PARAMETER_COUNT: usize = 2;

    type Constructors = HostCustomConstructorList<
        OkDefinition,
        HostCustomConstructorList<ErrorDefinition, HostCustomConstructorListEnd>,
    >;
}

#[cfg(test)]
mod tests {
    use super::ResultSchema;
    use crate::{
        HostCustomConstructorSchema, HostCustomFieldSchema, HostCustomTypeSchema, HostSchemaType,
    };

    #[test]
    fn describes_the_exact_prelude_result_schema() {
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
