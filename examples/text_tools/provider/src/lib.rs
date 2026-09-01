use geam::provider::EcoString;

#[geam::provider(
    package = "example_text_tools",
    modules = [text_tools, casing, checks],
)]
pub struct Component;

#[geam::module(path = "example_text_tools")]
mod text_tools {
    use super::EcoString;

    #[geam::function]
    fn join(left: EcoString, separator: EcoString, right: EcoString) -> EcoString {
        format!("{left}{separator}{right}").into()
    }

    #[geam::function]
    fn surround(value: EcoString, left: EcoString, right: EcoString) -> EcoString {
        format!("{left}{value}{right}").into()
    }
}

#[geam::module(path = "example_text_tools/casing")]
mod casing {
    use super::EcoString;

    #[geam::function]
    fn upper(value: EcoString) -> EcoString {
        value.to_uppercase()
    }

    #[geam::function]
    fn lower(value: EcoString) -> EcoString {
        value.to_lowercase()
    }
}

#[geam::module(path = "example_text_tools/checks")]
mod checks {
    use super::EcoString;

    #[geam::function]
    fn starts_with(value: EcoString, prefix: EcoString) -> bool {
        value.starts_with(prefix.as_str())
    }

    #[geam::function]
    fn ends_with(value: EcoString, suffix: EcoString) -> bool {
        value.ends_with(suffix.as_str())
    }
}
