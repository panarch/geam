use geam::provider::{EcoString, ExternalPayload};
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
    use super::{DefaultHasher, EcoString, ExternalPayload, Hash, Hasher, Regex};

    #[geam::external(name = "Pattern", manual)]
    struct Pattern {
        source: EcoString,
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

        fn inspect(&self) -> EcoString {
            format!("Pattern({:?})", self.source).into()
        }
    }

    #[geam::custom]
    enum CompileError {
        CompileError { message: EcoString },
    }

    #[geam::function]
    fn compile(source: EcoString) -> Result<Pattern, CompileError> {
        match Regex::new(source.as_str()) {
            Ok(regex) => Ok(Pattern { source, regex }),
            Err(error) => Err(CompileError::CompileError {
                message: error.to_string().into(),
            }),
        }
    }

    #[geam::function]
    fn is_match(pattern: &Pattern, text: EcoString) -> bool {
        pattern.regex.is_match(text.as_str())
    }

    #[geam::function]
    fn find_all(pattern: &Pattern, text: EcoString) -> Vec<EcoString> {
        pattern
            .regex
            .find_iter(text.as_str())
            .map(|matched| EcoString::from(matched.as_str()))
            .collect()
    }

    #[geam::function]
    fn replace_all(pattern: &Pattern, text: EcoString, replacement: EcoString) -> EcoString {
        pattern
            .regex
            .replace_all(text.as_str(), replacement.as_str())
            .as_ref()
            .into()
    }
}
