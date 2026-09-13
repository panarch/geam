use crate::plan;
use crate::plan::Text;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::{Node, Table};
use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum TypeMetadata {
    Parameter(plan::TypeParameterId),
    Int,
    Float,
    String,
    BitArray,
    UtfCodepoint,
    Bool,
    Nil,
    Tuple(Table<Self>),
    List(Node<Self>),
    Function(FunctionMetadata),
    Custom(NominalTypeMetadata),
    External(NominalTypeMetadata),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FunctionMetadata {
    pub arguments: Table<TypeMetadata>,
    pub return_: Node<TypeMetadata>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NominalTypeMetadata {
    pub package: Text,
    pub module: Text,
    pub name: Text,
    pub arguments: Table<TypeMetadata>,
}

impl TypeMetadata {
    pub(crate) fn compare(&self, type_: &plan::ValueType) -> Ordering {
        match (self, type_) {
            (Self::Parameter(left), plan::ValueType::Parameter(right)) => left.cmp(right),
            (Self::Int, plan::ValueType::Int)
            | (Self::Float, plan::ValueType::Float)
            | (Self::String, plan::ValueType::String)
            | (Self::BitArray, plan::ValueType::BitArray)
            | (Self::UtfCodepoint, plan::ValueType::UtfCodepoint)
            | (Self::Bool, plan::ValueType::Bool)
            | (Self::Nil, plan::ValueType::Nil) => Ordering::Equal,
            (Self::Tuple(left), plan::ValueType::Tuple(right)) => compare_arguments(left, right),
            (Self::List(left), plan::ValueType::List(right)) => left.compare(right),
            (Self::Function(left), plan::ValueType::Function(right)) => {
                compare_arguments(&left.arguments, right.argument_types())
                    .then_with(|| left.return_.compare(right.return_()))
            }
            (Self::Custom(left), plan::ValueType::Custom(right)) => {
                let name = right.type_name();
                left.compare(
                    name.package(),
                    name.module(),
                    name.name(),
                    right.arguments(),
                )
            }
            (Self::External(left), plan::ValueType::External(right)) => {
                let name = right.type_name();
                left.compare(
                    name.package(),
                    name.module(),
                    name.name(),
                    right.arguments(),
                )
            }
            _ => self.rank().cmp(&public_rank(type_)),
        }
    }

    fn rank(&self) -> u8 {
        match self {
            Self::Parameter(_) => 0,
            Self::Int => 1,
            Self::Float => 2,
            Self::String => 3,
            Self::BitArray => 4,
            Self::UtfCodepoint => 5,
            Self::Bool => 6,
            Self::Nil => 7,
            Self::Tuple(_) => 8,
            Self::List(_) => 9,
            Self::Function(_) => 10,
            Self::Custom(_) => 11,
            Self::External(_) => 12,
        }
    }

    pub(in crate::plan::execution) fn from_public(type_: &plan::ValueType) -> Self {
        match type_ {
            plan::ValueType::Parameter(parameter) => Self::Parameter(*parameter),
            plan::ValueType::Int => Self::Int,
            plan::ValueType::Float => Self::Float,
            plan::ValueType::String => Self::String,
            plan::ValueType::BitArray => Self::BitArray,
            plan::ValueType::UtfCodepoint => Self::UtfCodepoint,
            plan::ValueType::Bool => Self::Bool,
            plan::ValueType::Nil => Self::Nil,
            plan::ValueType::Tuple(elements) => {
                Self::Tuple(elements.iter().map(Self::from_public).collect())
            }
            plan::ValueType::List(item) => Self::List(Box::new(Self::from_public(item)).into()),
            plan::ValueType::Function(type_) => {
                Self::Function(FunctionMetadata::from_public(type_))
            }
            plan::ValueType::Custom(type_) => Self::Custom(NominalTypeMetadata::from_custom(type_)),
            plan::ValueType::External(type_) => {
                Self::External(NominalTypeMetadata::from_external(type_))
            }
        }
    }

    pub(crate) fn materialize(&self) -> plan::ValueType {
        match self {
            Self::Parameter(parameter) => plan::ValueType::Parameter(*parameter),
            Self::Int => plan::ValueType::Int,
            Self::Float => plan::ValueType::Float,
            Self::String => plan::ValueType::String,
            Self::BitArray => plan::ValueType::BitArray,
            Self::UtfCodepoint => plan::ValueType::UtfCodepoint,
            Self::Bool => plan::ValueType::Bool,
            Self::Nil => plan::ValueType::Nil,
            Self::Tuple(elements) => {
                plan::ValueType::Tuple(elements.iter().map(Self::materialize).collect())
            }
            Self::List(item) => plan::ValueType::List(Box::new(item.materialize())),
            Self::Function(type_) => plan::ValueType::Function(Box::new(type_.materialize())),
            Self::Custom(type_) => plan::ValueType::Custom(type_.custom_type()),
            Self::External(type_) => plan::ValueType::External(type_.external_type()),
        }
    }
}

impl FunctionMetadata {
    pub(in crate::plan::execution) fn from_public(type_: &plan::FunctionType) -> Self {
        Self {
            arguments: type_
                .argument_types()
                .iter()
                .map(TypeMetadata::from_public)
                .collect(),
            return_: Box::new(TypeMetadata::from_public(type_.return_())).into(),
        }
    }

    pub(crate) fn materialize(&self) -> plan::FunctionType {
        plan::FunctionType::new(
            self.arguments
                .iter()
                .map(TypeMetadata::materialize)
                .collect(),
            self.return_.materialize(),
        )
    }
}

impl NominalTypeMetadata {
    fn compare(
        &self,
        package: &str,
        module: &str,
        name: &str,
        arguments: &[plan::ValueType],
    ) -> Ordering {
        self.package
            .as_str()
            .cmp(package)
            .then_with(|| self.module.as_str().cmp(module))
            .then_with(|| self.name.as_str().cmp(name))
            .then_with(|| compare_arguments(&self.arguments, arguments))
    }

    pub(in crate::plan::execution) fn from_custom(type_: &plan::CustomType) -> Self {
        let name = type_.type_name();
        Self {
            package: name.package().clone().into(),
            module: name.module().clone().into(),
            name: name.name().clone().into(),
            arguments: type_
                .arguments()
                .iter()
                .map(TypeMetadata::from_public)
                .collect(),
        }
    }

    pub(in crate::plan::execution) fn from_external(type_: &plan::ExternalType) -> Self {
        let name = type_.type_name();
        Self {
            package: name.package().clone().into(),
            module: name.module().clone().into(),
            name: name.name().clone().into(),
            arguments: type_
                .arguments()
                .iter()
                .map(TypeMetadata::from_public)
                .collect(),
        }
    }

    pub(crate) fn custom_type(&self) -> plan::CustomType {
        plan::CustomType::new(
            plan::CustomTypeName::new(
                self.package.materialize(),
                self.module.materialize(),
                self.name.materialize(),
            ),
            self.arguments
                .iter()
                .map(TypeMetadata::materialize)
                .collect(),
        )
    }

    pub(crate) fn external_type(&self) -> plan::ExternalType {
        plan::ExternalType::new(
            plan::ExternalTypeName::new(
                self.package.materialize(),
                self.module.materialize(),
                self.name.materialize(),
            ),
            self.arguments
                .iter()
                .map(TypeMetadata::materialize)
                .collect(),
        )
    }
}

fn compare_arguments(left: &[TypeMetadata], right: &[plan::ValueType]) -> Ordering {
    left.iter()
        .zip(right)
        .map(|(left, right)| left.compare(right))
        .find(|ordering| !ordering.is_eq())
        .unwrap_or_else(|| left.len().cmp(&right.len()))
}

fn public_rank(type_: &plan::ValueType) -> u8 {
    match type_ {
        plan::ValueType::Parameter(_) => 0,
        plan::ValueType::Int => 1,
        plan::ValueType::Float => 2,
        plan::ValueType::String => 3,
        plan::ValueType::BitArray => 4,
        plan::ValueType::UtfCodepoint => 5,
        plan::ValueType::Bool => 6,
        plan::ValueType::Nil => 7,
        plan::ValueType::Tuple(_) => 8,
        plan::ValueType::List(_) => 9,
        plan::ValueType::Function(_) => 10,
        plan::ValueType::Custom(_) => 11,
        plan::ValueType::External(_) => 12,
    }
}

impl Emit for TypeMetadata {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Parameter(field_0) => output.call("type_::TypeMetadata::Parameter", &[field_0]),
            Self::Int => output.path("type_::TypeMetadata::Int"),
            Self::Float => output.path("type_::TypeMetadata::Float"),
            Self::String => output.path("type_::TypeMetadata::String"),
            Self::BitArray => output.path("type_::TypeMetadata::BitArray"),
            Self::UtfCodepoint => output.path("type_::TypeMetadata::UtfCodepoint"),
            Self::Bool => output.path("type_::TypeMetadata::Bool"),
            Self::Nil => output.path("type_::TypeMetadata::Nil"),
            Self::Tuple(field_0) => output.call("type_::TypeMetadata::Tuple", &[field_0]),
            Self::List(field_0) => output.call("type_::TypeMetadata::List", &[field_0]),
            Self::Function(field_0) => output.call("type_::TypeMetadata::Function", &[field_0]),
            Self::Custom(field_0) => output.call("type_::TypeMetadata::Custom", &[field_0]),
            Self::External(field_0) => output.call("type_::TypeMetadata::External", &[field_0]),
        }
    }
}

impl Emit for FunctionMetadata {
    fn emit(&self, output: &mut Rust) {
        let Self { arguments, return_ } = self;
        output.structure(
            "type_::FunctionMetadata",
            &[("arguments", arguments), ("return_", return_)],
        );
    }
}

impl Emit for NominalTypeMetadata {
    fn emit(&self, output: &mut Rust) {
        let Self {
            package,
            module,
            name,
            arguments,
        } = self;
        output.structure(
            "type_::NominalTypeMetadata",
            &[
                ("package", package),
                ("module", module),
                ("name", name),
                ("arguments", arguments),
            ],
        );
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionMetadata, NominalTypeMetadata, TypeMetadata};
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::{self, Text};
    use std::hash::{Hash, Hasher};

    #[test]
    fn emits_all_metadata_families_and_their_nested_fields() {
        use crate::plan::execution::prepared::rust::Rust;
        let cases = [
            (
                TypeMetadata::Parameter(plan::TypeParameterId(2)),
                "data::type_::TypeMetadata::Parameter(data::type_::parameter_id(2,),)",
            ),
            (TypeMetadata::Int, "data::type_::TypeMetadata::Int"),
            (TypeMetadata::Float, "data::type_::TypeMetadata::Float"),
            (TypeMetadata::String, "data::type_::TypeMetadata::String"),
            (
                TypeMetadata::BitArray,
                "data::type_::TypeMetadata::BitArray",
            ),
            (
                TypeMetadata::UtfCodepoint,
                "data::type_::TypeMetadata::UtfCodepoint",
            ),
            (TypeMetadata::Bool, "data::type_::TypeMetadata::Bool"),
            (TypeMetadata::Nil, "data::type_::TypeMetadata::Nil"),
            (
                TypeMetadata::Tuple(Table::Static(&[TypeMetadata::Int])),
                "data::type_::TypeMetadata::Tuple(data::Storage::Static(&[data::type_::TypeMetadata::Int,]),)",
            ),
            (
                TypeMetadata::List(Node::Static(&TypeMetadata::Bool)),
                "data::type_::TypeMetadata::List(data::Storage::Static(&data::type_::TypeMetadata::Bool),)",
            ),
            (
                TypeMetadata::Function(FunctionMetadata {
                    arguments: Table::Static(&[TypeMetadata::Int]),
                    return_: Node::Static(&TypeMetadata::Bool),
                }),
                "data::type_::TypeMetadata::Function(data::type_::FunctionMetadata {arguments: data::Storage::Static(&[data::type_::TypeMetadata::Int,]),return_: data::Storage::Static(&data::type_::TypeMetadata::Bool),},)",
            ),
            (
                TypeMetadata::Custom(NominalTypeMetadata {
                    package: Text::Static("app"),
                    module: Text::Static("app/types"),
                    name: Text::Static("Box"),
                    arguments: Table::Static(&[TypeMetadata::Int]),
                }),
                "data::type_::TypeMetadata::Custom(data::type_::NominalTypeMetadata {package: data::Text::Static(\"app\",),module: data::Text::Static(\"app/types\",),name: data::Text::Static(\"Box\",),arguments: data::Storage::Static(&[data::type_::TypeMetadata::Int,]),},)",
            ),
            (
                TypeMetadata::External(NominalTypeMetadata {
                    package: Text::Static("host"),
                    module: Text::Static("host/key"),
                    name: Text::Static("Key"),
                    arguments: Table::Static(&[TypeMetadata::String]),
                }),
                "data::type_::TypeMetadata::External(data::type_::NominalTypeMetadata {package: data::Text::Static(\"host\",),module: data::Text::Static(\"host/key\",),name: data::Text::Static(\"Key\",),arguments: data::Storage::Static(&[data::type_::TypeMetadata::String,]),},)",
            ),
        ];
        for (metadata, expected) in cases {
            assert_eq!(Rust::expression(&metadata), expected);
        }
    }

    #[test]
    fn borrowed_comparisons_follow_the_complete_structural_type_order() {
        use plan::ValueType as V;
        let custom = |package: &str, module: &str, name: &str, arguments| {
            V::Custom(plan::CustomType::new(
                plan::CustomTypeName::new(package.into(), module.into(), name.into()),
                arguments,
            ))
        };
        let external = |package: &str, module: &str, name: &str, arguments| {
            V::External(plan::ExternalType::new(
                plan::ExternalTypeName::new(package.into(), module.into(), name.into()),
                arguments,
            ))
        };
        let types = vec![
            V::Parameter(plan::TypeParameterId(0)),
            V::Parameter(plan::TypeParameterId(2)),
            V::Int,
            V::Float,
            V::String,
            V::BitArray,
            V::UtfCodepoint,
            V::Bool,
            V::Nil,
            V::Tuple(vec![]),
            V::Tuple(vec![V::Int]),
            V::Tuple(vec![V::Int, V::String]),
            V::Tuple(vec![V::Float]),
            V::List(Box::new(V::Int)),
            V::List(Box::new(V::Bool)),
            V::Function(Box::new(plan::FunctionType::new(vec![], V::Nil))),
            V::Function(Box::new(plan::FunctionType::new(vec![V::Int], V::Int))),
            V::Function(Box::new(plan::FunctionType::new(vec![V::Int], V::Float))),
            V::Function(Box::new(plan::FunctionType::new(vec![V::String], V::Int))),
            custom("a", "a/types", "A", vec![]),
            custom("a", "a/types", "A", vec![V::Int]),
            custom("a", "a/types", "A", vec![V::String]),
            custom("a", "a/types", "B", vec![]),
            custom("a", "b/types", "A", vec![]),
            custom("b", "a/types", "A", vec![]),
            external("a", "a/types", "A", vec![]),
            external("a", "a/types", "A", vec![V::Int]),
            external("a", "a/types", "A", vec![V::String]),
            external("a", "a/types", "B", vec![]),
            external("a", "b/types", "A", vec![]),
            external("b", "a/types", "A", vec![]),
        ];
        for (left_index, left) in types.iter().enumerate() {
            let metadata = TypeMetadata::from_public(left);
            for (right_index, right) in types.iter().enumerate() {
                let expected = left_index.cmp(&right_index);
                assert_eq!(metadata.compare(right), expected, "{left:?}, {right:?}");
                assert_eq!(metadata.cmp(&TypeMetadata::from_public(right)), expected);
            }
        }
    }

    #[test]
    fn nominal_metadata_preserves_every_public_type_family() {
        let scalar_types = vec![
            plan::ValueType::Parameter(plan::TypeParameterId(3)),
            plan::ValueType::Int,
            plan::ValueType::Float,
            plan::ValueType::String,
            plan::ValueType::BitArray,
            plan::ValueType::UtfCodepoint,
            plan::ValueType::Bool,
            plan::ValueType::Nil,
        ];
        let type_ = plan::ValueType::Tuple(vec![
            plan::ValueType::Tuple(scalar_types),
            plan::ValueType::List(Box::new(plan::ValueType::Function(Box::new(
                plan::FunctionType::new(
                    vec![plan::ValueType::Custom(plan::CustomType::new(
                        plan::CustomTypeName::new(
                            "domain".into(),
                            "domain/result".into(),
                            "Result".into(),
                        ),
                        vec![plan::ValueType::String],
                    ))],
                    plan::ValueType::External(plan::ExternalType::new(
                        plan::ExternalTypeName::new(
                            "native".into(),
                            "native/key".into(),
                            "Key".into(),
                        ),
                        vec![plan::ValueType::Parameter(plan::TypeParameterId(3))],
                    )),
                ),
            )))),
        ]);
        assert_eq!(TypeMetadata::from_public(&type_).materialize(), type_);
    }

    #[test]
    fn recursive_static_metadata_borrows_nodes_and_preserves_identity() {
        static ARGUMENTS: [TypeMetadata; 1] = [TypeMetadata::Int];
        static RESULT: TypeMetadata = TypeMetadata::Bool;
        static TYPES: [TypeMetadata; 3] = [
            TypeMetadata::Function(FunctionMetadata {
                arguments: Table::Static(&ARGUMENTS),
                return_: Node::Static(&RESULT),
            }),
            TypeMetadata::Custom(NominalTypeMetadata {
                package: Text::Static("app"),
                module: Text::Static("app/types"),
                name: Text::Static("Value"),
                arguments: Table::Static(&ARGUMENTS),
            }),
            TypeMetadata::External(NominalTypeMetadata {
                package: Text::Static("app"),
                module: Text::Static("app/types"),
                name: Text::Static("Handle"),
                arguments: Table::Static(&ARGUMENTS),
            }),
        ];
        let static_type = TypeMetadata::Tuple(Table::Static(&TYPES));
        let expected = plan::ValueType::Tuple(vec![
            plan::ValueType::Function(Box::new(plan::FunctionType::new(
                vec![plan::ValueType::Int],
                plan::ValueType::Bool,
            ))),
            plan::ValueType::Custom(plan::CustomType::new(
                plan::CustomTypeName::new("app".into(), "app/types".into(), "Value".into()),
                vec![plan::ValueType::Int],
            )),
            plan::ValueType::External(plan::ExternalType::new(
                plan::ExternalTypeName::new("app".into(), "app/types".into(), "Handle".into()),
                vec![plan::ValueType::Int],
            )),
        ]);
        assert_eq!(static_type.materialize(), expected);
        let owned = TypeMetadata::from_public(&expected);
        assert_eq!(static_type, owned);
        assert_eq!(hash(&static_type), hash(&owned));
        let cloned_type = static_type.clone();
        let (cloned, function) = tuple_function_metadata(&cloned_type);
        assert!(std::ptr::eq(cloned.as_ptr(), TYPES.as_ptr()));
        assert!(std::ptr::eq(
            function.arguments.as_ptr(),
            ARGUMENTS.as_ptr()
        ));
        assert!(std::ptr::eq(function.return_.as_ref(), &RESULT));
        assert_eq!(
            format!("{:?}", &cloned[1]),
            "Custom(NominalTypeMetadata { package: \"app\", module: \"app/types\", name: \"Value\", arguments: [Int] })"
        );
        assert_eq!(
            format!("{:?}", &cloned[0]),
            "Function(FunctionMetadata { arguments: [Int], return_: Bool })"
        );
    }

    fn tuple_function_metadata(value: &TypeMetadata) -> (&[TypeMetadata], &FunctionMetadata) {
        let TypeMetadata::Tuple(elements) = value else {
            panic!("tuple metadata expected")
        };
        let TypeMetadata::Function(function) = &elements[0] else {
            panic!("function metadata expected")
        };
        (elements, function)
    }

    #[test]
    #[should_panic(expected = "tuple metadata expected")]
    fn tuple_function_fixture_rejects_a_scalar() {
        tuple_function_metadata(&TypeMetadata::Int);
    }

    #[test]
    #[should_panic(expected = "function metadata expected")]
    fn tuple_function_fixture_rejects_a_scalar_item() {
        tuple_function_metadata(&TypeMetadata::Tuple(Table::Static(&[TypeMetadata::Int])));
    }

    fn hash(value: &TypeMetadata) -> u64 {
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        value.hash(&mut hasher);
        hasher.finish()
    }
}
