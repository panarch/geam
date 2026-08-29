use super::identifier::RustIdentifier;
use crate::error::CliError;
use geam_core::TypedProgram;
use gleam_core::ast::Publicity;
use gleam_core::type_::Type;
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum Scalar {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
}

#[derive(Debug)]
pub(super) struct FunctionBinding {
    pub(super) gleam_name: String,
    pub(super) rust_name: RustIdentifier,
    pub(super) arguments: Vec<Scalar>,
    pub(super) return_type: Scalar,
}

#[derive(Debug)]
pub(super) struct PlainBindings {
    pub(super) geam_alias: RustIdentifier,
    pub(super) root_module: String,
    pub(super) first: FunctionBinding,
    pub(super) remaining: Vec<FunctionBinding>,
}

impl PlainBindings {
    pub(super) fn from_program(
        geam_alias: RustIdentifier,
        program: &TypedProgram,
    ) -> Result<Self, CliError> {
        let root = program.root_typed_module();
        let module = root.name.to_string();
        let public_constants = root
            .definitions
            .constants
            .iter()
            .filter(|constant| constant.publicity == Publicity::Public)
            .map(|constant| constant.name.to_string())
            .collect::<Vec<_>>();
        if !public_constants.is_empty() {
            return Err(CliError::InvalidEmbeddingBoundary {
                module,
                reason: format!(
                    "public constants are not supported: {}",
                    public_constants.join(", ")
                ),
            });
        }

        let mut functions = root
            .definitions
            .functions
            .iter()
            .filter(|function| function.publicity == Publicity::Public)
            .filter_map(|function| {
                function
                    .name
                    .as_ref()
                    .map(|(_, name)| (function, name.to_string()))
            })
            .collect::<Vec<_>>();
        functions.sort_by_key(|(function, _)| function.location.start);

        let mut bindings = Vec::new();
        let mut identifiers = HashSet::new();
        let mut failures = Vec::new();
        for (function, name) in functions {
            if function.arguments.len() > 7 {
                failures.push(format!(
                    "public function `{name}` has arity {}, but embedding supports arity 0..=7",
                    function.arguments.len()
                ));
                continue;
            }
            let rust_name = match unique_rust_identifier(&name, &mut identifiers) {
                Ok(identifier) => identifier,
                Err(reason) => {
                    failures.push(format!("public function `{name}`: {reason}"));
                    continue;
                }
            };

            let mut arguments = Vec::new();
            let mut supported = true;
            for (index, argument) in function.arguments.iter().enumerate() {
                match Scalar::from_type(&argument.type_) {
                    Some(type_) => arguments.push(type_),
                    None => {
                        failures.push(format!(
                            "public function `{name}` argument {} has an unsupported type",
                            index + 1
                        ));
                        supported = false;
                    }
                }
            }
            let Some(return_type) = Scalar::from_type(&function.return_type) else {
                failures.push(format!(
                    "public function `{name}` return value has an unsupported type"
                ));
                continue;
            };
            if supported {
                bindings.push(FunctionBinding {
                    gleam_name: name,
                    rust_name,
                    arguments,
                    return_type,
                });
            }
        }

        if !failures.is_empty() {
            return Err(CliError::InvalidEmbeddingBoundary {
                module,
                reason: failures.join("; "),
            });
        }
        let mut bindings = bindings.into_iter();
        let Some(first) = bindings.next() else {
            return Err(CliError::InvalidEmbeddingBoundary {
                module,
                reason: "the selected module has no public functions".to_owned(),
            });
        };
        Ok(Self {
            geam_alias,
            root_module: root.name.to_string(),
            first,
            remaining: bindings.collect(),
        })
    }

    pub(super) fn functions(&self) -> impl Iterator<Item = &FunctionBinding> {
        std::iter::once(&self.first).chain(self.remaining.iter())
    }
}

fn unique_rust_identifier(
    name: &str,
    identifiers: &mut HashSet<String>,
) -> Result<RustIdentifier, String> {
    let identifier = RustIdentifier::parse(name)?;
    if identifiers.insert(identifier.as_str().to_owned()) {
        Ok(identifier)
    } else {
        Err("the generated Rust field collides with another public function".to_owned())
    }
}

impl Scalar {
    fn from_type(type_: &Arc<Type>) -> Option<Self> {
        if type_.is_int() {
            Some(Self::Int)
        } else if type_.is_float() {
            Some(Self::Float)
        } else if type_.is_string() {
            Some(Self::String)
        } else if type_.is_bit_array() {
            Some(Self::BitArray)
        } else if type_.is_utf_codepoint() {
            Some(Self::UtfCodepoint)
        } else if type_.is_bool() {
            Some(Self::Bool)
        } else if type_.is_nil() {
            Some(Self::Nil)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{PlainBindings, Scalar, unique_rust_identifier};
    use crate::embedding::identifier::RustIdentifier;
    use crate::error::CliError;
    use geam_core::{ModuleSource, compile_typed_program};
    use std::collections::HashSet;

    #[test]
    fn preserves_source_order_keywords_scalars_and_supported_arities() {
        let bindings = bindings(
            r#"
pub fn zero() -> Nil { Nil }

pub fn async(value: Int) -> Int { value }

pub fn seven(
  integer: Int,
  float: Float,
  string: String,
  bits: BitArray,
  codepoint: UtfCodepoint,
  boolean: Bool,
  nil: Nil,
) -> String {
  string
}

pub fn float_value(value: Float) -> Float { value }
pub fn bits_value(value: BitArray) -> BitArray { value }
pub fn codepoint_value(value: UtfCodepoint) -> UtfCodepoint { value }
pub fn bool_value(value: Bool) -> Bool { value }
"#,
        )
        .expect("supported scalar boundary should be accepted");
        assert_eq!(bindings.root_module, "boundary");
        assert_eq!(bindings.first.gleam_name, "zero");
        assert_eq!(bindings.first.arguments, []);
        assert_eq!(bindings.first.return_type, Scalar::Nil);
        assert_eq!(bindings.remaining[0].gleam_name, "async");
        assert_eq!(bindings.remaining[0].rust_name.as_str(), "r#async");
        assert_eq!(
            bindings
                .functions()
                .map(|function| function.gleam_name.as_str())
                .collect::<Vec<_>>(),
            [
                "zero",
                "async",
                "seven",
                "float_value",
                "bits_value",
                "codepoint_value",
                "bool_value",
            ],
        );
        assert_eq!(
            bindings.remaining[1].arguments,
            [
                Scalar::Int,
                Scalar::Float,
                Scalar::String,
                Scalar::BitArray,
                Scalar::UtfCodepoint,
                Scalar::Bool,
                Scalar::Nil,
            ],
        );
        assert_eq!(bindings.remaining[1].return_type, Scalar::String);
    }

    #[test]
    fn rejects_every_unsupported_public_signature_family() {
        for (source, expected) in [
            ("pub fn unsupported(value) { value }", "argument 1"),
            (
                "pub fn unsupported(_value: List(Int)) -> Int { 1 }",
                "argument 1",
            ),
            (
                "pub fn unsupported(value: #(Int, Int)) { value }",
                "argument 1",
            ),
            (
                "pub type Boxed { Boxed(Int) }\npub fn unsupported(value: Boxed) { value }",
                "argument 1",
            ),
            (
                "pub fn unsupported(value: fn(Int) -> Int) { value }",
                "argument 1",
            ),
            ("pub fn unsupported() -> List(Int) { [] }", "return value"),
        ] {
            let error = bindings(source).expect_err("unsupported boundary should fail");
            assert!(matches!(
                error,
                CliError::InvalidEmbeddingBoundary { module, reason }
                    if module == "boundary"
                        && reason.contains("unsupported type")
                        && reason.contains(expected)
            ));
        }
    }

    #[test]
    fn rejects_constants_empty_boundaries_and_excessive_arity() {
        let error = bindings("pub const answer = 42\npub fn value() { answer }")
            .expect_err("public constant should fail");
        assert!(matches!(
            error,
            CliError::InvalidEmbeddingBoundary { reason, .. }
                if reason == "public constants are not supported: answer"
        ));

        let error = bindings("fn private() { 1 }").expect_err("empty boundary should fail");
        assert!(matches!(
            error,
            CliError::InvalidEmbeddingBoundary { reason, .. }
                if reason == "the selected module has no public functions"
        ));

        let error = bindings(
            "pub fn too_many(a: Int, b: Int, c: Int, d: Int, e: Int, f: Int, g: Int, h: Int) { a }",
        )
        .expect_err("arity eight should fail");
        assert!(matches!(
            error,
            CliError::InvalidEmbeddingBoundary { reason, .. }
                if reason.contains("arity 8") && reason.contains("0..=7")
        ));
    }

    #[test]
    fn rejects_duplicate_and_unrepresentable_generated_fields() {
        let mut identifiers = HashSet::new();
        assert_eq!(
            unique_rust_identifier("selected", &mut identifiers)
                .expect("first field should be accepted")
                .as_str(),
            "selected",
        );
        assert!(unique_rust_identifier("selected", &mut identifiers).is_err());

        let error =
            bindings("pub fn self() { 1 }").expect_err("unrepresentable Rust field should fail");
        assert!(matches!(
            error,
            CliError::InvalidEmbeddingBoundary { reason, .. }
                if reason.contains("cannot be used as a Rust raw identifier")
        ));
    }

    fn bindings(source: &str) -> Result<PlainBindings, CliError> {
        let program = compile_typed_program(
            "boundary",
            [ModuleSource::new("boundary", "boundary.gleam", source)],
        )
        .expect("boundary fixture should compile");
        PlainBindings::from_program(
            RustIdentifier::parse("runtime").expect("fixture alias should be valid"),
            &program,
        )
    }
}
