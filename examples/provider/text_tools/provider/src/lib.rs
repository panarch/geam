use geam::HostFailure;
use geam::provider::{HostResult, StringValue};

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
        left.concat(&separator).concat(&right)
    }

    #[geam::function]
    fn surround(value: StringValue, left: StringValue, right: StringValue) -> StringValue {
        left.concat(&value).concat(&right)
    }
}

#[geam::module(path = "example_text_tools/casing")]
mod casing {
    use super::{HostFailure, HostResult, StringValue};

    #[geam::function]
    fn upper(value: StringValue) -> HostResult<StringValue> {
        Ok(value
            .into_ecostring()
            .map_err(|error| HostFailure::new(error.to_string()))?
            .to_uppercase()
            .into())
    }

    #[geam::function]
    fn lower(value: StringValue) -> HostResult<StringValue> {
        Ok(value
            .into_ecostring()
            .map_err(|error| HostFailure::new(error.to_string()))?
            .to_lowercase()
            .into())
    }
}

#[geam::module(path = "example_text_tools/checks")]
mod checks {
    use super::StringValue;

    #[geam::function]
    fn starts_with(value: StringValue, prefix: StringValue) -> bool {
        value.starts_with(prefix.as_bytes())
    }

    #[geam::function]
    fn ends_with(value: StringValue, suffix: StringValue) -> bool {
        value.ends_with(suffix.as_bytes())
    }
}
