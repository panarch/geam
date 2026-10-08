use crate::plan::execution::type_::custom::CustomDefinition;
use crate::plan::execution::type_::metadata::substitute;
use crate::plan::execution::type_::{NominalTypeMetadata, TypeMetadata};
use std::collections::{HashMap, HashSet};

/// Expands the finite structural closure of an explicit retained view grant.
/// Non-regular custom recursion remains exact transport, rather than inventing
/// an arbitrary expansion limit or specializing more types at runtime.
pub(in crate::plan::execution) fn retained_types<'definition>(
    roots: impl IntoIterator<Item = TypeMetadata>,
    definition: impl Fn(&NominalTypeMetadata) -> Option<&'definition CustomDefinition>,
) -> Vec<TypeMetadata> {
    type Identity = (String, String, String);
    struct Walk<Definition> {
        definition: Definition,
        seen: HashSet<TypeMetadata>,
        active: HashMap<Identity, NominalTypeMetadata>,
        non_regular: HashSet<Identity>,
        types: Vec<TypeMetadata>,
    }
    fn identity(nominal: &NominalTypeMetadata) -> Identity {
        (
            nominal.package.to_string(),
            nominal.module.to_string(),
            nominal.name.to_string(),
        )
    }
    impl<'definition, Definition: Fn(&NominalTypeMetadata) -> Option<&'definition CustomDefinition>>
        Walk<Definition>
    {
        fn visit(&mut self, type_: TypeMetadata) {
            if let TypeMetadata::Custom(nominal) = &type_ {
                let id = identity(nominal);
                if let Some(active) = self.active.get(&id) {
                    if active != nominal {
                        self.non_regular.insert(id);
                    }
                    return;
                }
            }
            if !self.seen.insert(type_.clone()) {
                return;
            }
            self.types.push(type_.clone());
            match type_ {
                TypeMetadata::Tuple(items) => {
                    for item in items.iter() {
                        self.visit(item.clone());
                    }
                }
                TypeMetadata::List(item) => self.visit(item.as_ref().clone()),
                TypeMetadata::Function(function) => {
                    for argument in function.arguments.iter() {
                        self.visit(argument.clone());
                    }
                    self.visit(function.return_.as_ref().clone());
                }
                TypeMetadata::Custom(nominal) => {
                    let id = identity(&nominal);
                    self.active.insert(id.clone(), nominal.clone());
                    if let Some(definition) = (self.definition)(&nominal)
                        && definition.native_visible()
                    {
                        for argument in nominal.arguments.iter() {
                            self.visit(argument.clone());
                        }
                        for constructor in definition.constructors.iter() {
                            for field in constructor.fields.iter() {
                                self.visit(substitute(&field.type_, &nominal.arguments));
                            }
                        }
                    }
                    self.active.remove(&id);
                }
                TypeMetadata::External(nominal) => {
                    for argument in nominal.arguments.iter() {
                        self.visit(argument.clone());
                    }
                }
                TypeMetadata::Parameter(_)
                | TypeMetadata::Int
                | TypeMetadata::Float
                | TypeMetadata::String
                | TypeMetadata::BitArray
                | TypeMetadata::UtfCodepoint
                | TypeMetadata::Bool
                | TypeMetadata::Nil => {}
            }
        }
    }
    let mut walk = Walk {
        definition,
        seen: HashSet::new(),
        active: HashMap::new(),
        non_regular: HashSet::new(),
        types: Vec::new(),
    };
    for root in roots {
        walk.visit(root);
    }
    walk.types.into_iter().filter(|type_| {
        !matches!(type_, TypeMetadata::Custom(nominal) if walk.non_regular.contains(&identity(nominal)))
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::retained_types;
    use crate::plan::execution::type_::{NominalTypeMetadata, TypeMetadata};

    #[test]
    fn structural_grants_expand_regular_fields_and_keep_non_regular_and_unknown_types_exact() {
        use crate::plan::execution::storage::Node;
        use crate::plan::execution::type_::FunctionMetadata;
        use crate::{HostProviderSet, StatelessHostProfile};
        let source = r#"
pub type Tree(a) { Leaf(a) Branch(List(Tree(a))) }
pub type Grow(a) { Stop Grow(Grow(List(a))) }
pub fn main() {
  let growing: Grow(Int) = Grow(Stop)
  #(Leaf(fn(value: Int) { value }), growing)
}
"#;
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            HostProviderSet::<StatelessHostProfile>::from_providers([]).unwrap(),
        )
        .unwrap();
        let (program, _) = crate::plan::execution::lowering::lower_hosted(
            crate::plan_host_program(typed).unwrap(),
        )
        .unwrap();
        let custom = |name: &'static str, argument| {
            TypeMetadata::Custom(NominalTypeMetadata {
                package: "app".into(),
                module: "main".into(),
                name: name.into(),
                arguments: vec![argument].into(),
            })
        };
        let callback = TypeMetadata::Function(FunctionMetadata {
            arguments: vec![TypeMetadata::Int].into(),
            return_: Box::new(TypeMetadata::Int).into(),
        });
        let tree = custom("Tree", callback.clone());
        let growing = custom("Grow", TypeMetadata::Int);
        let opaque = custom("Opaque", TypeMetadata::Int);
        let external = TypeMetadata::External(NominalTypeMetadata {
            package: "provider".into(),
            module: "schema".into(),
            name: "External".into(),
            arguments: vec![TypeMetadata::Int].into(),
        });
        let scalars = vec![
            TypeMetadata::Float,
            TypeMetadata::String,
            TypeMetadata::BitArray,
            TypeMetadata::UtfCodepoint,
            TypeMetadata::Bool,
            TypeMetadata::Nil,
            TypeMetadata::Parameter(crate::plan::TypeParameterId(0)),
        ];
        let root = TypeMetadata::Tuple(
            [
                callback.clone(),
                tree.clone(),
                growing,
                opaque.clone(),
                external.clone(),
            ]
            .into_iter()
            .chain(scalars.clone())
            .collect(),
        );
        let definitions = &program.common.custom_types.definitions;
        assert_eq!(
            retained_types([root.clone(), callback.clone()], |nominal| definitions
                .iter()
                .find(|definition| definition.identity()
                    == (
                        nominal.package.as_str(),
                        nominal.module.as_str(),
                        nominal.name.as_str()
                    ))),
            [
                root,
                callback,
                TypeMetadata::Int,
                tree.clone(),
                TypeMetadata::List(Node::Owned(Box::new(tree))),
                opaque,
                external
            ]
            .into_iter()
            .chain(scalars)
            .collect::<Vec<_>>()
        );
    }

    #[test]
    fn an_opaque_nominal_retains_its_arguments_without_inventing_fields() {
        let opaque = TypeMetadata::Custom(NominalTypeMetadata {
            package: "provider".into(),
            module: "schema".into(),
            name: "Opaque".into(),
            arguments: vec![TypeMetadata::Int].into(),
        });
        assert_eq!(
            retained_types([opaque.clone(), TypeMetadata::Int], |_| None),
            [opaque, TypeMetadata::Int]
        );
    }

    #[test]
    fn external_arguments_are_part_of_the_structural_grant() {
        let external = TypeMetadata::External(NominalTypeMetadata {
            package: "provider".into(),
            module: "schema".into(),
            name: "External".into(),
            arguments: vec![TypeMetadata::Int].into(),
        });
        assert_eq!(
            retained_types([external.clone()], |_| None),
            [external, TypeMetadata::Int]
        );
    }
}
