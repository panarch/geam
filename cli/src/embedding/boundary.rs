use super::identifier::RustIdentifier;
use crate::error::CliError;
use geam_core::TypedProgram;
use gleam_core::ast::Publicity;
use gleam_core::type_::{Type, collapse_links};
use std::collections::HashSet;
use std::sync::Arc;

mod named;
use named::NamedTypes;
pub(super) use named::{ClosedType, NamedKind, NamedType};

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
    Future(Box<DataType>),
    Named(usize),
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
    pub(super) named_types: Vec<NamedType>,
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
        let mut named = NamedTypes::new(program);
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

            let arguments = DataType::from_types(
                &mut named,
                function
                    .arguments
                    .iter()
                    .enumerate()
                    .map(|(index, argument)| {
                        (
                            &argument.type_,
                            format!("public function `{name}` argument {}", index + 1),
                        )
                    }),
            );
            let return_type = DataType::from_type(
                &mut named,
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
            named_types: named.finish(),
        })
    }

    pub(super) fn functions(&self) -> impl Iterator<Item = &FunctionBinding> {
        std::iter::once(&self.first).chain(self.remaining.iter())
    }

    pub(super) fn has_future(&self) -> bool {
        self.functions().any(|function| {
            function
                .arguments
                .iter()
                .chain(std::iter::once(&function.return_type))
                .any(DataType::has_future)
        })
    }

    pub(super) fn needs_scope(&self) -> bool {
        !self.named_types.is_empty() || self.has_future()
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
    fn has_future(&self) -> bool {
        match self {
            Self::Future(_) => true,
            Self::Tuple(items) => items.iter().any(Self::has_future),
            Self::Result(ok, error) => ok.has_future() || error.has_future(),
            Self::Option(item) | Self::List(item) => item.has_future(),
            Self::Int
            | Self::Float
            | Self::String
            | Self::BitArray
            | Self::UtfCodepoint
            | Self::Bool
            | Self::Nil
            | Self::Named(_) => false,
        }
    }

    fn from_type(
        named: &mut NamedTypes<'_>,
        type_: &Arc<Type>,
        position: &str,
    ) -> Result<Self, Vec<String>> {
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
                Type::Tuple { elements } if (1..=7).contains(&elements.len()) => Self::from_types(
                    named,
                    elements.iter().enumerate().map(|(index, element)| {
                        (
                            element,
                            format!("{position} -> Tuple element {}", index + 1),
                        )
                    }),
                )
                .map(Self::Tuple),
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
                            Self::from_type(named, item, &format!("{position} -> List item"))
                                .map(|item| Self::List(Box::new(item)))
                        }
                        ("gleam_stdlib", "gleam/option", "Option", [item]) => {
                            Self::from_type(named, item, &format!("{position} -> Option value"))
                                .map(|item| Self::Option(Box::new(item)))
                        }
                        ("geam", "geam/future", "Future", [item]) => Self::from_type(
                            named,
                            item,
                            &format!("{position} -> Future completion"),
                        )
                        .map(|item| Self::Future(Box::new(item))),
                        ("", "gleam", "Result", [ok, error]) => {
                            let ok =
                                Self::from_type(named, ok, &format!("{position} -> Result Ok"));
                            let error = Self::from_type(
                                named,
                                error,
                                &format!("{position} -> Result Error"),
                            );
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
                        _ => named
                            .register(package, module, name, arguments, position)
                            .map(Self::Named),
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
        named: &mut NamedTypes<'_>,
        types: impl Iterator<Item = (&'a Arc<Type>, String)>,
    ) -> Result<Vec<Self>, Vec<String>> {
        let mut values = Vec::new();
        let mut failures = Vec::new();
        for (type_, position) in types {
            match Self::from_type(named, type_, &position) {
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
    use super::{
        ClosedType, DataType, NamedKind, NamedType, PlainBindings, unique_rust_identifier,
    };
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
    fn preserves_canonical_future_identity_and_recursive_source_positions() {
        let program = compile_typed_package_program(
            "application",
            "boundary",
            [
                PackageSource::new(
                    "geam",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "geam/future",
                        "future.gleam",
                        "pub type Future(value)",
                    )],
                ),
                PackageSource::new(
                    "application",
                    ["geam"],
                    [ModuleSource::new(
                        "boundary",
                        "boundary.gleam",
                        r#"
import geam/future.{type Future}
pub type Nested = #(List(Future(Int)), Future(List(Result(String, Future(Bool)))))
pub fn keep(value: Nested) { value }
pub fn double(value: Int) { value * 2 }
pub fn again(value: Future(Future(Int))) { value }
"#,
                    )],
                ),
            ],
        )
        .expect("ordinary nominal source");
        let bindings =
            PlainBindings::from_program(RustIdentifier::parse("runtime").expect("alias"), &program)
                .expect("recursive Future boundary");
        let expected = DataType::Tuple(vec![
            DataType::List(Box::new(DataType::Future(Box::new(DataType::Int)))),
            DataType::Future(Box::new(DataType::List(Box::new(DataType::Result(
                Box::new(DataType::String),
                Box::new(DataType::Future(Box::new(DataType::Bool))),
            ))))),
        ]);
        assert_eq!(
            bindings.first.arguments.as_slice(),
            std::slice::from_ref(&expected)
        );
        assert_eq!(bindings.first.return_type, expected);
        assert_eq!(bindings.remaining[0].arguments, [DataType::Int]);
        assert_eq!(bindings.remaining[0].return_type, DataType::Int);
        assert_eq!(
            bindings.remaining[1].return_type,
            DataType::Future(Box::new(DataType::Future(Box::new(DataType::Int))))
        );
    }

    #[test]
    fn retains_lookalike_futures_without_granting_observation_and_rejects_callable_completion() {
        for package in ["other", "geam"] {
            let program = compile_typed_package_program("application", "boundary", [
                PackageSource::new(package, Vec::<String>::new(), [ModuleSource::new(
                    "geam/future", "future.gleam", "pub type Future(value)",
                )]),
                PackageSource::new("application", [package], [ModuleSource::new(
                    "boundary", "boundary.gleam", "import geam/future.{type Future}\npub fn work(_value: Future(List(fn() -> Int))) { 42 }",
                )]),
            ]).expect("valid nominal source");
            let result = PlainBindings::from_program(
                RustIdentifier::parse("runtime").expect("alias"),
                &program,
            );
            if package == "other" {
                let bindings = result.expect("ordinary external retention");
                assert!(!bindings.has_future());
                assert!(bindings.needs_scope());
                assert_eq!(bindings.first.arguments, [DataType::Named(0)]);
                assert_eq!(
                    bindings.named_types,
                    [NamedType {
                        package: "other".into(),
                        module: "geam/future".into(),
                        name: "Future".into(),
                        kind: NamedKind::External,
                        arguments: vec![ClosedType::List(Box::new(ClosedType::Function(
                            Vec::new(),
                            Box::new(ClosedType::Int)
                        )))],
                    }]
                );
                continue;
            }
            let error = result.expect_err("callable work results are not Rust function bindings");
            assert!(
                matches!(error, CliError::InvalidEmbeddingBoundary { module, reason } if module == "boundary" && reason == "public function `work` argument 1 -> Future completion -> List item has an unsupported function type")
            );
        }
    }

    #[test]
    fn retains_lookalike_standard_types_and_rejects_nested_option_callable_values() {
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
            let result = PlainBindings::from_program(
                RustIdentifier::parse("runtime").expect("fixture alias"),
                &program,
            );
            if package == "other_package" {
                let bindings = result.expect("nominal custom retention");
                assert_eq!(bindings.first.arguments, [DataType::Named(0)]);
                assert_eq!(
                    bindings.named_types,
                    [NamedType {
                        package: "other_package".into(),
                        module: "gleam/option".into(),
                        name: "Option".into(),
                        kind: NamedKind::Custom,
                        arguments: vec![ClosedType::List(Box::new(ClosedType::Function(
                            Vec::new(),
                            Box::new(ClosedType::Int)
                        )))],
                    }]
                );
                continue;
            }
            let error = result.expect_err("standard Option decodes its exposed content");
            assert!(
                matches!(error, CliError::InvalidEmbeddingBoundary { reason, .. } if reason == "public function `optional` argument 1 -> Option value -> List item has an unsupported function type")
            );
        }
        let bindings = bindings("pub type Result(a, b) { Ok(a) Error(b) }\npub fn local(value: Result(Int, String)) { value }")
            .expect("local Result retains its original identity");
        assert_eq!(bindings.first.arguments, [DataType::Named(0)]);
        assert_eq!(bindings.first.return_type, DataType::Named(0));
        assert_eq!(
            bindings.named_types,
            [NamedType {
                package: "geam".into(),
                module: "boundary".into(),
                name: "Result".into(),
                kind: NamedKind::Custom,
                arguments: vec![ClosedType::Int, ClosedType::String],
            }]
        );
    }

    #[test]
    fn preserves_named_types_and_aliases_without_exposing_private_data() {
        let bindings = bindings(
            r#"
pub opaque type Boxed(a) { Boxed(a, fn() -> Int) }
pub type External(a)
pub type Alias = Boxed(Int)
pub fn boxed(value: Alias) { value }
pub fn same(value: Boxed(Int)) { value }
pub fn different(value: Boxed(String)) { value }
pub fn resource(value: External(fn(List(Int)) -> Result(Int, String))) { value }
"#,
        )
        .expect("exact named boundaries");
        assert!(bindings.needs_scope());
        assert!(!bindings.has_future());
        assert_eq!(bindings.first.arguments, [DataType::Named(0)]);
        assert_eq!(bindings.first.return_type, DataType::Named(0));
        assert_eq!(bindings.remaining[0].arguments, [DataType::Named(0)]);
        assert_eq!(bindings.remaining[1].return_type, DataType::Named(1));
        assert_eq!(bindings.remaining[2].return_type, DataType::Named(2));
        assert_eq!(
            bindings.named_types,
            [
                NamedType {
                    package: "geam".into(),
                    module: "boundary".into(),
                    name: "Boxed".into(),
                    kind: NamedKind::Custom,
                    arguments: vec![ClosedType::Int]
                },
                NamedType {
                    package: "geam".into(),
                    module: "boundary".into(),
                    name: "Boxed".into(),
                    kind: NamedKind::Custom,
                    arguments: vec![ClosedType::String]
                },
                NamedType {
                    package: "geam".into(),
                    module: "boundary".into(),
                    name: "External".into(),
                    kind: NamedKind::External,
                    arguments: vec![ClosedType::Function(
                        vec![ClosedType::List(Box::new(ClosedType::Int))],
                        Box::new(ClosedType::Named(NamedType {
                            package: "".into(),
                            module: "gleam".into(),
                            name: "Result".into(),
                            kind: NamedKind::Custom,
                            arguments: vec![ClosedType::Int, ClosedType::String]
                        })),
                    )]
                },
            ]
        );
        for (source, expected) in [
            (
                "pub type Boxed { Boxed(Int) }\npub fn keep(value: Boxed) { value }",
                NamedKind::Custom,
            ),
            (
                "pub type External\npub fn keep(value: External) { value }",
                NamedKind::External,
            ),
        ] {
            let program = compile_typed_program(
                "boundary",
                [ModuleSource::new("boundary", "boundary.gleam", source)],
            )
            .expect("source");
            let boundary = PlainBindings::from_program(
                RustIdentifier::parse("runtime").expect("alias"),
                &program,
            )
            .expect("retention");
            assert_eq!(boundary.named_types[0].kind, expected);
            assert!(boundary.named_types[0].arguments.is_empty());
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
