use super::identifier::RustIdentifier;
use crate::error::CliError;
use geam_core::TypedProgram;
use gleam_core::ast::Publicity;
use gleam_core::type_::{Type, collapse_links};
use std::collections::HashSet;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum DataType {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Vec<DataType>),
    Result(Box<DataType>, Box<DataType>),
    Option(Box<DataType>),
    List(Box<DataType>),
}

#[derive(Debug)]
pub(super) struct FunctionBinding {
    pub(super) gleam_name: String,
    pub(super) rust_name: RustIdentifier,
    pub(super) arguments: Vec<DataType>,
    pub(super) return_type: DataType,
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

            let arguments = DataType::from_types(function.arguments.iter().enumerate().map(
                |(index, argument)| {
                    (
                        &argument.type_,
                        format!("public function `{name}` argument {}", index + 1),
                    )
                },
            ));
            let return_type = DataType::from_type(
                &function.return_type,
                &format!("public function `{name}` return value"),
            );
            match (arguments, return_type) {
                (Ok(arguments), Ok(return_type)) => bindings.push(FunctionBinding {
                    gleam_name: name,
                    rust_name,
                    arguments,
                    return_type,
                }),
                (Err(arguments), Err(return_type)) => {
                    failures.extend(arguments);
                    failures.extend(return_type);
                }
                (Err(errors), Ok(_)) | (Ok(_), Err(errors)) => failures.extend(errors),
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

impl DataType {
    fn from_type(type_: &Arc<Type>, position: &str) -> Result<Self, Vec<String>> {
        if type_.is_int() {
            Ok(Self::Int)
        } else if type_.is_float() {
            Ok(Self::Float)
        } else if type_.is_string() {
            Ok(Self::String)
        } else if type_.is_bit_array() {
            Ok(Self::BitArray)
        } else if type_.is_utf_codepoint() {
            Ok(Self::UtfCodepoint)
        } else if type_.is_bool() {
            Ok(Self::Bool)
        } else if type_.is_nil() {
            Ok(Self::Nil)
        } else {
            let type_ = collapse_links(type_.clone());
            match type_.as_ref() {
                Type::Tuple { elements } if (1..=7).contains(&elements.len()) => {
                    Self::from_types(elements.iter().enumerate().map(|(index, element)| {
                        (
                            element,
                            format!("{position} -> Tuple element {}", index + 1),
                        )
                    }))
                    .map(Self::Tuple)
                }
                Type::Tuple { elements } => Err(vec![format!(
                    "{position} has Tuple arity {}, but embedding supports Tuple arity 1..=7",
                    elements.len(),
                )]),
                Type::Named {
                    package,
                    module,
                    name,
                    arguments,
                    ..
                } => {
                    match (
                        package.as_str(),
                        module.as_str(),
                        name.as_str(),
                        arguments.as_slice(),
                    ) {
                        ("", "gleam", "List", [item]) => {
                            Self::from_type(item, &format!("{position} -> List item"))
                                .map(|item| Self::List(Box::new(item)))
                        }
                        ("gleam_stdlib", "gleam/option", "Option", [item]) => {
                            Self::from_type(item, &format!("{position} -> Option value"))
                                .map(|item| Self::Option(Box::new(item)))
                        }
                        ("", "gleam", "Result", [ok, error]) => {
                            let ok = Self::from_type(ok, &format!("{position} -> Result Ok"));
                            let error =
                                Self::from_type(error, &format!("{position} -> Result Error"));
                            match (ok, error) {
                                (Ok(ok), Ok(error)) => {
                                    Ok(Self::Result(Box::new(ok), Box::new(error)))
                                }
                                (Err(mut errors), Err(rest)) => {
                                    errors.extend(rest);
                                    Err(errors)
                                }
                                (Err(errors), Ok(_)) | (Ok(_), Err(errors)) => Err(errors),
                            }
                        }
                        _ => Err(vec![format!(
                            "{position} has an unsupported named type `{package}:{module}.{name}`",
                        )]),
                    }
                }
                Type::Fn { .. } => {
                    Err(vec![format!("{position} has an unsupported function type")])
                }
                Type::Var { .. } => {
                    Err(vec![format!("{position} has an unsupported generic type")])
                }
            }
        }
    }

    fn from_types<'a>(
        types: impl Iterator<Item = (&'a Arc<Type>, String)>,
    ) -> Result<Vec<Self>, Vec<String>> {
        let mut values = Vec::new();
        let mut failures = Vec::new();
        for (type_, position) in types {
            match Self::from_type(type_, &position) {
                Ok(value) => values.push(value),
                Err(errors) => failures.extend(errors),
            }
        }
        if failures.is_empty() {
            Ok(values)
        } else {
            Err(failures)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DataType, PlainBindings, unique_rust_identifier};
    use crate::embedding::identifier::RustIdentifier;
    use crate::error::CliError;
    use geam_core::{
        ModuleSource, PackageSource, compile_typed_package_program, compile_typed_program,
    };
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
        assert_eq!(bindings.first.return_type, DataType::Nil);
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
                DataType::Int,
                DataType::Float,
                DataType::String,
                DataType::BitArray,
                DataType::UtfCodepoint,
                DataType::Bool,
                DataType::Nil,
            ],
        );
        assert_eq!(bindings.remaining[1].return_type, DataType::String);
    }

    #[test]
    fn rejects_every_unsupported_public_signature_family() {
        for (source, expected) in [
            (
                "pub fn unsupported(value) { value }",
                "public function `unsupported` argument 1 has an unsupported generic type; public function `unsupported` return value has an unsupported generic type",
            ),
            (
                "pub fn unsupported(_value: List(fn(Int) -> Int)) -> Int { 1 }",
                "public function `unsupported` argument 1 -> List item has an unsupported function type",
            ),
            (
                "pub fn unsupported(value: #(Int, Int, Int, Int, Int, Int, Int, Int)) { value }",
                "public function `unsupported` argument 1 has Tuple arity 8, but embedding supports Tuple arity 1..=7; public function `unsupported` return value has Tuple arity 8, but embedding supports Tuple arity 1..=7",
            ),
            (
                "pub type Boxed { Boxed(Int) }\npub fn unsupported(value: Boxed) { value }",
                "public function `unsupported` argument 1 has an unsupported named type `geam:boundary.Boxed`; public function `unsupported` return value has an unsupported named type `geam:boundary.Boxed`",
            ),
            (
                "pub type External\npub fn unsupported(value: External) { value }",
                "public function `unsupported` argument 1 has an unsupported named type `geam:boundary.External`; public function `unsupported` return value has an unsupported named type `geam:boundary.External`",
            ),
            (
                "pub fn unsupported() { fn(value: Int) { value } }",
                "public function `unsupported` return value has an unsupported function type",
            ),
            (
                "pub fn unsupported(_value: #(a, Result(fn() -> Int, b))) { 1 }",
                "public function `unsupported` argument 1 -> Tuple element 1 has an unsupported generic type; public function `unsupported` argument 1 -> Tuple element 2 -> Result Ok has an unsupported function type; public function `unsupported` argument 1 -> Tuple element 2 -> Result Error has an unsupported generic type",
            ),
            (
                "pub fn first(_value: Result(Int, a)) { 1 }\npub fn second(_value: Result(a, Int)) { 2 }",
                "public function `first` argument 1 -> Result Error has an unsupported generic type; public function `second` argument 1 -> Result Ok has an unsupported generic type",
            ),
        ] {
            let error = bindings(source).expect_err("unsupported boundary should fail");
            assert!(
                matches!(
                    &error,
                    CliError::InvalidEmbeddingBoundary { module, reason }
                        if module == "boundary"
                            && reason == expected
                ),
                "unexpected diagnostic: {error:?}"
            );
        }
    }

    #[test]
    fn accepts_recursive_standard_types_and_resolved_aliases() {
        let program = compile_typed_package_program(
            "application",
            "boundary",
            [
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/option",
                        "option.gleam",
                        "pub type Option(a) { Some(a) None }",
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["gleam_stdlib"],
                    [ModuleSource::new(
                        "boundary",
                        "boundary.gleam",
                        r#"
import gleam/option.{type Option}
pub type Row = #(String, Int)
pub type Rows = List(Result(Row, Option(String)))
pub fn rows(value: Rows) -> Rows { value }
pub fn nested(value: List(List(String))) { value }
pub fn one(value: #(Int)) { value }
pub fn seven(value: #(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil)) { value }
"#,
                    )],
                ),
            ],
        )
        .expect("recursive boundary source should compile");
        let bindings = PlainBindings::from_program(
            RustIdentifier::parse("runtime").expect("fixture alias"),
            &program,
        )
        .expect("recursive ordinary data should be accepted");
        let rows = DataType::List(Box::new(DataType::Result(
            Box::new(DataType::Tuple(vec![DataType::String, DataType::Int])),
            Box::new(DataType::Option(Box::new(DataType::String))),
        )));
        assert_eq!(bindings.first.arguments, std::slice::from_ref(&rows));
        assert_eq!(bindings.first.return_type, rows);
        assert_eq!(
            bindings.remaining[0].arguments,
            [DataType::List(Box::new(DataType::List(Box::new(
                DataType::String
            )),))]
        );
        assert_eq!(
            bindings.remaining[1].arguments,
            [DataType::Tuple(vec![DataType::Int])]
        );
        assert_eq!(
            bindings.remaining[2].return_type,
            DataType::Tuple(vec![
                DataType::Int,
                DataType::Float,
                DataType::String,
                DataType::BitArray,
                DataType::UtfCodepoint,
                DataType::Bool,
                DataType::Nil,
            ])
        );
    }

    #[test]
    fn rejects_lookalike_standard_types_and_nested_option_failures() {
        for package in ["other_package", "gleam_stdlib"] {
            let program = compile_typed_package_program(
                "application",
                "boundary",
                [
                    PackageSource::new(
                        package,
                        Vec::<String>::new(),
                        [ModuleSource::new(
                            "gleam/option",
                            "option.gleam",
                            "pub type Option(a) { Some(a) None }",
                        )],
                    ),
                    PackageSource::new(
                        "application",
                        [package],
                        [ModuleSource::new(
                            "boundary",
                            "boundary.gleam",
                            r#"
import gleam/option.{type Option}
pub fn optional(_value: Option(List(fn() -> Int))) { 1 }
"#,
                        )],
                    ),
                ],
            )
            .expect("Option source should compile");
            let error = PlainBindings::from_program(
                RustIdentifier::parse("runtime").expect("fixture alias"),
                &program,
            )
            .expect_err("nonstandard or unsupported Option should fail");
            let expected = if package == "gleam_stdlib" {
                "public function `optional` argument 1 -> Option value -> List item has an unsupported function type"
            } else {
                "public function `optional` argument 1 has an unsupported named type `other_package:gleam/option.Option`"
            };
            assert!(
                matches!(error, CliError::InvalidEmbeddingBoundary { reason, .. } if reason == expected)
            );
        }
        let error = bindings("pub type Result(a, b) { Ok(a) Error(b) }\npub fn local(value: Result(Int, String)) { value }")
            .expect_err("local Result identity must not become a standard result");
        assert!(
            matches!(error, CliError::InvalidEmbeddingBoundary { reason, .. }
            if reason == "public function `local` argument 1 has an unsupported named type `geam:boundary.Result`; public function `local` return value has an unsupported named type `geam:boundary.Result`")
        );
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
