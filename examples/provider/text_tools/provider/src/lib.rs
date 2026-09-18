use geam::provider::StringValue;

#[geam::provider(
    package = "example_text_tools",
    modules = [text_tools, casing, checks],
)]
pub struct Component;

#[geam::module(path = "example_text_tools")]
mod text_tools {
    use super::StringValue;

    #[geam::function]
    fn join(left: StringValue, separator: StringValue, right: StringValue) -> StringValue {
        format!("{left}{separator}{right}").into()
    }

    #[geam::function]
    fn surround(value: StringValue, left: StringValue, right: StringValue) -> StringValue {
        format!("{left}{value}{right}").into()
    }
}

#[geam::module(path = "example_text_tools/casing")]
mod casing {
    use super::StringValue;

    #[geam::function]
    fn upper(value: StringValue) -> StringValue {
        value.to_uppercase().into()
    }

    #[geam::function]
    fn lower(value: StringValue) -> StringValue {
        value.to_lowercase().into()
    }
}

#[geam::module(path = "example_text_tools/checks")]
mod checks {
    use super::StringValue;

    #[geam::function]
    fn starts_with(value: StringValue, prefix: StringValue) -> bool {
        value.starts_with(prefix.as_str())
    }

    #[geam::function]
    fn ends_with(value: StringValue, suffix: StringValue) -> bool {
        value.ends_with(suffix.as_str())
    }
}
