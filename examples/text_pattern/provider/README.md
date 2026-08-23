# geam-example-text-pattern

`geam-example-text-pattern` is the documentation provider for the ordinary
[example_text_pattern Gleam package](https://github.com/panarch/geam/tree/main/examples/text_pattern/project/packages/example_text_pattern).
It compiles patterns with Rust's `regex` crate and exposes the resulting opaque
values through Geam's typed host component contracts.

## Provider Authoring Direction

The current [`src/lib.rs`](src/lib.rs) deliberately spells out Geam's complete
low-level provider contract. It is an executable baseline for the SDK boundary,
not the intended amount of boilerplate for an ordinary provider author to
maintain.

Today the provider author has to declare `Component`, `Stores`, `Provider`, all
schema marker types and type-level lists, storage projection, registration, and
call-scoped construction tokens by hand. Those declarations are useful for
testing Geam's low-level ABI, but they do not carry text-pattern domain meaning.

The scalar component and module shape now has a concrete target. Applying the
same shape to this provider should look roughly like the following:

```rust
use ecow::EcoString;
use geam::provider::{ExternalPayload, GleamResult};
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
    use super::{
        DefaultHasher, EcoString, ExternalPayload, GleamResult, Hash, Hasher,
        Regex,
    };

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
    fn compile(source: EcoString) -> GleamResult<Pattern, CompileError> {
        match Regex::new(source.as_str()) {
            Ok(regex) => GleamResult::Ok(Pattern { source, regex }),
            Err(error) => GleamResult::Error(CompileError::CompileError {
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
            .map(|matched| matched.as_str().into())
            .collect()
    }

    #[geam::function]
    fn replace_all(
        pattern: &Pattern,
        text: EcoString,
        replacement: EcoString,
    ) -> EcoString {
        pattern
            .regex
            .replace_all(text.as_str(), replacement.as_str())
            .as_ref()
            .into()
    }
}
```

`#[geam::provider]`, `#[geam::module]`, scalar `#[geam::function]`, injected
`#[geam::state]` parameters, `ExternalPayload`, and `#[geam::external]` are now
implemented. The external-value slice generates the schema, payload store,
storage binding, and callback adapter for payloads that do not retain Gleam
values. The `Vec<EcoString>` return above now maps to one newly constructed
Gleam List. `GleamResult` and `#[geam::custom]` remain a design sketch, so the
macros still cannot express this complete provider.

Later proc-macro slices should add the remaining custom and sealed-construction
machinery while retaining the established component/package/module shape. The
provider author should still write the Rust payload and behavior, external
equality/hash/inspection semantics, configuration and state initialization when
needed, callback bodies, and explicit failures. The
`[package.metadata.geam.provider]` declaration also remains explicit because it
is the crates.io discovery contract rather than Rust adapter boilerplate.

The macros live in the separate `geam-macros` workspace crate, but `geam`
re-exports them so a provider author still has one Geam dependency.
Generated composition must remain static and typed: no dynamic registry,
runtime reflection, type erasure, hidden configuration, or checked-in generated
bindings.

This example is the before-and-after acceptance case for that work. Replacing
its low-level declarations with the higher-level authoring API must preserve the
same discovery metadata, typed callbacks, opaque escaped values, configuration,
and runtime behavior.

The complete standalone workflow and local development commands are documented
in the [example README](https://github.com/panarch/geam/tree/main/examples/text_pattern).
