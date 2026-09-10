use geam_core::TypedProgram;
use gleam_core::type_::{Type, collapse_links};
use std::sync::Arc;

pub(super) struct NamedTypes<'program> {
    program: &'program TypedProgram,
    types: Vec<NamedType>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::embedding) struct NamedType {
    pub(in crate::embedding) package: String,
    pub(in crate::embedding) module: String,
    pub(in crate::embedding) name: String,
    pub(in crate::embedding) kind: NamedKind,
    pub(in crate::embedding) arguments: Vec<ClosedType>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::embedding) enum NamedKind {
    Custom,
    External,
}

// These are concrete type arguments, not values exposed to Rust callers. A
// function or wide tuple can be retained inside an otherwise opaque value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::embedding) enum ClosedType {
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    List(Box<ClosedType>),
    Tuple(Vec<ClosedType>),
    Function(Vec<ClosedType>, Box<ClosedType>),
    Named(NamedType),
}

impl<'program> NamedTypes<'program> {
    pub(super) fn new(program: &'program TypedProgram) -> Self {
        Self {
            program,
            types: Vec::new(),
        }
    }

    pub(super) fn register(
        &mut self,
        package: &str,
        module: &str,
        name: &str,
        arguments: &[Arc<Type>],
        position: &str,
    ) -> Result<usize, Vec<String>> {
        let type_ = self.named(package, module, name, arguments, position)?;
        if let Some(index) = self
            .types
            .iter()
            .position(|registered| registered == &type_)
        {
            return Ok(index);
        }
        let index = self.types.len();
        self.types.push(type_);
        Ok(index)
    }

    pub(super) fn finish(self) -> Vec<NamedType> {
        self.types
    }

    fn named(
        &self,
        package: &str,
        module: &str,
        name: &str,
        arguments: &[Arc<Type>],
        position: &str,
    ) -> Result<NamedType, Vec<String>> {
        let kind = if (package, module, name) == ("", "gleam", "Result") {
            NamedKind::Custom
        } else {
            let declaration = self
                .program
                .modules()
                .find(|source| source.type_info.package == package && source.name == module)
                .and_then(|source| {
                    source
                        .definitions
                        .custom_types
                        .iter()
                        .find(|type_| type_.name == name)
                });
            let Some(declaration) = declaration else {
                return Err(vec![format!(
                    "{position} has an unknown named type `{package}:{module}.{name}`"
                )]);
            };
            if declaration.constructors.is_empty() {
                NamedKind::External
            } else {
                NamedKind::Custom
            }
        };
        let arguments = arguments
            .iter()
            .enumerate()
            .map(|(index, argument)| {
                self.closed(
                    argument,
                    &format!("{position} -> {name} type argument {}", index + 1),
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        Ok(NamedType {
            package: package.to_owned(),
            module: module.to_owned(),
            name: name.to_owned(),
            kind,
            arguments,
        })
    }

    fn closed(&self, type_: &Arc<Type>, position: &str) -> Result<ClosedType, Vec<String>> {
        if type_.is_int() {
            return Ok(ClosedType::Int);
        }
        if type_.is_float() {
            return Ok(ClosedType::Float);
        }
        if type_.is_string() {
            return Ok(ClosedType::String);
        }
        if type_.is_bit_array() {
            return Ok(ClosedType::BitArray);
        }
        if type_.is_utf_codepoint() {
            return Ok(ClosedType::UtfCodepoint);
        }
        if type_.is_bool() {
            return Ok(ClosedType::Bool);
        }
        if type_.is_nil() {
            return Ok(ClosedType::Nil);
        }
        let type_ = collapse_links(type_.clone());
        match type_.as_ref() {
            Type::Tuple { elements } => elements
                .iter()
                .map(|type_| self.closed(type_, position))
                .collect::<Result<Vec<_>, _>>()
                .map(ClosedType::Tuple),
            Type::Fn { arguments, return_ } => Ok(ClosedType::Function(
                arguments
                    .iter()
                    .map(|type_| self.closed(type_, position))
                    .collect::<Result<Vec<_>, _>>()?,
                Box::new(self.closed(return_, position)?),
            )),
            Type::Named {
                package,
                module,
                name,
                arguments,
                ..
            } => {
                if let ("", "gleam", "List", [item]) = (
                    package.as_str(),
                    module.as_str(),
                    name.as_str(),
                    arguments.as_slice(),
                ) {
                    return self
                        .closed(item, position)
                        .map(|item| ClosedType::List(Box::new(item)));
                }
                self.named(package, module, name, arguments, position)
                    .map(ClosedType::Named)
            }
            Type::Var { .. } => Err(vec![format!("{position} has an unsupported generic type")]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ClosedType, NamedKind, NamedType, NamedTypes};
    use geam_core::{ModuleSource, compile_typed_program};

    #[test]
    fn preserves_all_scalar_arguments_inside_an_opaque_type() {
        let program = compile_typed_program(
            "library",
            [ModuleSource::new(
                "library",
                "library.gleam",
                r#"
pub type Resource(a)
pub fn keep(value: Resource(#(Int, Float, String, BitArray, UtfCodepoint, Bool, Nil))) {
  value
}
"#,
            )],
        )
        .expect("valid closed source types");
        let named = NamedTypes::new(&program);
        let return_type = &program.root_typed_module().definitions.functions[0].return_type;
        assert_eq!(
            named.closed(return_type, "return value"),
            Ok(ClosedType::Named(NamedType {
                package: "geam".into(),
                module: "library".into(),
                name: "Resource".into(),
                kind: NamedKind::External,
                arguments: vec![ClosedType::Tuple(vec![
                    ClosedType::Int,
                    ClosedType::Float,
                    ClosedType::String,
                    ClosedType::BitArray,
                    ClosedType::UtfCodepoint,
                    ClosedType::Bool,
                    ClosedType::Nil,
                ])],
            })),
        );
    }

    #[test]
    fn rejects_generic_arguments_with_their_source_position() {
        for source in [
            "pub type Boxed(a) { Boxed(a) }\npub fn keep(value: Boxed(a)) { value }",
            "pub type Boxed(a) { Boxed(a) }\npub fn keep(value: Boxed(fn(a) -> Int)) { value }",
            "pub type Boxed(a) { Boxed(a) }\npub fn keep(value: Boxed(fn() -> a)) { value }",
            "pub type Boxed(a) { Boxed(a) }\npub fn keep(value: Boxed(List(a))) { value }",
            "pub type Boxed(a) { Boxed(a) }\npub fn keep(value: Boxed(#(Int, a))) { value }",
        ] {
            let program = compile_typed_program(
                "library",
                [ModuleSource::new("library", "library.gleam", source)],
            )
            .expect("valid generic source type");
            let named = NamedTypes::new(&program);
            let return_type = &program.root_typed_module().definitions.functions[0].return_type;
            assert_eq!(
                named.closed(return_type, "return value"),
                Err(vec![
                    "return value -> Boxed type argument 1 has an unsupported generic type".into()
                ]),
            );
        }
    }

    #[test]
    fn rejects_nominal_metadata_without_a_matching_declaration() {
        let program = compile_typed_program(
            "library",
            [ModuleSource::new(
                "library",
                "library.gleam",
                "pub type Resource\npub fn keep(value: Resource) { value }",
            )],
        )
        .expect("valid nominal source");
        for (package, module, name, expected) in [
            (
                "another_package",
                "library",
                "Resource",
                "argument 1 has an unknown named type `another_package:library.Resource`",
            ),
            (
                "geam",
                "another_module",
                "Resource",
                "argument 1 has an unknown named type `geam:another_module.Resource`",
            ),
            (
                "geam",
                "library",
                "AnotherType",
                "argument 1 has an unknown named type `geam:library.AnotherType`",
            ),
        ] {
            let mut named = NamedTypes::new(&program);
            assert_eq!(
                named.register(package, module, name, &[], "argument 1"),
                Err(vec![expected.to_owned()]),
            );
            assert!(named.finish().is_empty());
        }
    }
}
