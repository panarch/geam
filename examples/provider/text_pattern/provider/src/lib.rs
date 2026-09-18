use geam::provider::{ExternalPayload, StringValue};
use regex::Regex;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[geam::provider(
    package = "example_text_pattern",
    modules = [text_pattern],
)]
pub struct Component;

#[geam::module(path = "example_text_pattern")]
mod text_pattern {
    use super::{DefaultHasher, ExternalPayload, Hash, Hasher, Regex, StringValue};

    #[geam::external(name = "Pattern", manual)]
    struct Pattern {
        source: StringValue,
        regex: Regex,
    }

    impl ExternalPayload for Pattern {
        fn source_equal(&self, other: &Self) -> bool {
            self.source == other.source
        }

        fn source_hash(&self) -> u64 {
            let mut hasher = DefaultHasher::new();
            self.source.hash(&mut hasher);
            hasher.finish()
        }

        fn inspect(&self) -> geam::provider::EcoString {
            format!("Pattern({:?})", self.source).into()
        }
    }

    #[geam::custom]
    enum CompileError {
        CompileError { message: StringValue },
    }

    #[geam::function]
    fn compile(source: StringValue) -> Result<Pattern, CompileError> {
        match Regex::new(source.as_str()) {
            Ok(regex) => Ok(Pattern { source, regex }),
            Err(error) => Err(CompileError::CompileError {
                message: error.to_string().into(),
            }),
        }
    }

    #[geam::function]
    fn is_match(pattern: &Pattern, text: StringValue) -> bool {
        pattern.regex.is_match(text.as_str())
    }

    #[geam::function]
    fn find_all(pattern: &Pattern, text: StringValue) -> Vec<StringValue> {
        pattern
            .regex
            .find_iter(text.as_str())
            .map(|matched| text.slice(matched.start()..matched.end()))
            .collect()
    }

    #[geam::function]
    fn replace_all(pattern: &Pattern, text: StringValue, replacement: StringValue) -> StringValue {
        pattern
            .regex
            .replace_all(text.as_str(), replacement.as_str())
            .as_ref()
            .into()
    }
}
