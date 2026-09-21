use super::{TypeError, Types};
use crate::plan::execution::type_::custom::CustomDefinition;
use crate::plan::execution::type_::{
    CustomConstructorRefinement, TypeMetadata, ValueShapeDescriptor, ValueShapeId,
};
use std::collections::HashSet;

#[derive(PartialEq, Eq)]
pub(in crate::plan::execution::prepared::admission) enum FunctionRepresentation {
    Symbolic,
    Never,
    Value,
}

#[derive(PartialEq, Eq, Hash, Clone)]
struct CustomKey {
    definition: *const CustomDefinition,
    arguments: Vec<bool>,
    constructor: CustomConstructorRefinement,
}

impl Types<'_> {
    pub(in crate::plan::execution::prepared::admission) fn metadata_inhabited(
        &self,
        type_: &TypeMetadata,
        arguments: &[bool],
    ) -> Result<bool, TypeError> {
        self.metadata(type_)?;
        self.input_inhabited(type_, arguments, &mut HashSet::new(), &mut HashSet::new())
    }

    /// The metadata must already match an admitted value type. Its nominal
    /// references then share the validated definition graph, like stored shapes.
    pub(in crate::plan::execution::prepared::admission) fn matched_metadata_inhabited(
        &self,
        type_: &TypeMetadata,
    ) -> bool {
        self.template_inhabited(type_, &[], &mut HashSet::new(), &mut HashSet::new())
    }

    pub(in crate::plan::execution::prepared::admission) fn inhabited(
        &self,
        shape: &ValueShapeDescriptor,
    ) -> bool {
        self.shape_inhabited(shape, &mut HashSet::new(), &mut HashSet::new())
    }

    fn shape_inhabited(
        &self,
        shape: &ValueShapeDescriptor,
        active: &mut HashSet<CustomKey>,
        known: &mut HashSet<CustomKey>,
    ) -> bool {
        match shape {
            ValueShapeDescriptor::Parameter(_) => false,
            ValueShapeDescriptor::Tuple(items) => {
                for item in items.iter() {
                    if !self.shape_inhabited(&self.shapes.shapes[item.index()], active, known) {
                        return false;
                    }
                }
                true
            }
            ValueShapeDescriptor::Custom(id) => {
                let custom = &self.shapes.custom_shapes[id.0];
                let arguments = custom
                    .arguments
                    .iter()
                    .map(|argument| {
                        self.shape_inhabited(&self.shapes.shapes[argument.index()], active, known)
                    })
                    .collect();
                let nominal = &self.customs.types[custom.type_id.index()].type_;
                let declaration = self.declarations[&(
                    nominal.package.as_str(),
                    nominal.module.as_str(),
                    nominal.name.as_str(),
                )];
                self.custom_inhabited(declaration, arguments, custom.constructor, active, known)
            }
            ValueShapeDescriptor::Int
            | ValueShapeDescriptor::Float
            | ValueShapeDescriptor::String
            | ValueShapeDescriptor::BitArray
            | ValueShapeDescriptor::UtfCodepoint
            | ValueShapeDescriptor::Bool
            | ValueShapeDescriptor::Nil
            | ValueShapeDescriptor::List(_)
            | ValueShapeDescriptor::Function { .. }
            | ValueShapeDescriptor::External(_) => true,
        }
    }

    fn custom_inhabited(
        &self,
        definition: &CustomDefinition,
        arguments: Vec<bool>,
        constructor: CustomConstructorRefinement,
        active: &mut HashSet<CustomKey>,
        known: &mut HashSet<CustomKey>,
    ) -> bool {
        let key = CustomKey {
            definition,
            arguments,
            constructor,
        };
        if known.contains(&key) {
            return true;
        }
        if !active.insert(key.clone()) {
            return false;
        }
        let mut inhabited = false;
        for (index, definition) in definition.constructors.iter().enumerate() {
            if matches!(constructor, CustomConstructorRefinement::Exact(selected) if selected != index)
            {
                continue;
            }
            let mut fields_inhabited = true;
            for field in definition.fields.iter() {
                if !self.template_inhabited(&field.type_, &key.arguments, active, known) {
                    fields_inhabited = false;
                    break;
                }
            }
            if fields_inhabited {
                inhabited = true;
                break;
            }
        }
        active.remove(&key);
        if inhabited {
            known.insert(key);
        }
        inhabited
    }

    fn template_inhabited(
        &self,
        template: &TypeMetadata,
        arguments: &[bool],
        active: &mut HashSet<CustomKey>,
        known: &mut HashSet<CustomKey>,
    ) -> bool {
        match template {
            // Unbound parameters denote symbolic values, not a runtime payload.
            TypeMetadata::Parameter(id) => arguments.get(id.0).copied().unwrap_or(false),
            TypeMetadata::Tuple(items) => {
                for item in items.iter() {
                    if !self.template_inhabited(item, arguments, active, known) {
                        return false;
                    }
                }
                true
            }
            TypeMetadata::Custom(nominal) => {
                let arguments = nominal
                    .arguments
                    .iter()
                    .map(|argument| self.template_inhabited(argument, arguments, active, known))
                    .collect();
                self.custom_inhabited(
                    self.declarations[&(
                        nominal.package.as_str(),
                        nominal.module.as_str(),
                        nominal.name.as_str(),
                    )],
                    arguments,
                    CustomConstructorRefinement::Any,
                    active,
                    known,
                )
            }
            TypeMetadata::Int
            | TypeMetadata::Float
            | TypeMetadata::String
            | TypeMetadata::BitArray
            | TypeMetadata::UtfCodepoint
            | TypeMetadata::Bool
            | TypeMetadata::Nil
            | TypeMetadata::List(_)
            | TypeMetadata::Function(_)
            | TypeMetadata::External(_) => true,
        }
    }

    fn input_inhabited(
        &self,
        input: &TypeMetadata,
        arguments: &[bool],
        active: &mut HashSet<CustomKey>,
        known: &mut HashSet<CustomKey>,
    ) -> Result<bool, TypeError> {
        match input {
            TypeMetadata::Parameter(id) => Ok(arguments.get(id.0).copied().unwrap_or(false)),
            TypeMetadata::Tuple(items) => {
                for item in items.iter() {
                    if !self.input_inhabited(item, arguments, active, known)? {
                        return Ok(false);
                    }
                }
                Ok(true)
            }
            TypeMetadata::Custom(nominal) => {
                let arguments = nominal
                    .arguments
                    .iter()
                    .map(|argument| self.input_inhabited(argument, arguments, active, known))
                    .collect::<Result<Vec<_>, _>>()?;
                let definition = self.definition(nominal)?;
                Ok(self.custom_inhabited(
                    definition,
                    arguments,
                    CustomConstructorRefinement::Any,
                    active,
                    known,
                ))
            }
            TypeMetadata::Int
            | TypeMetadata::Float
            | TypeMetadata::String
            | TypeMetadata::BitArray
            | TypeMetadata::UtfCodepoint
            | TypeMetadata::Bool
            | TypeMetadata::Nil
            | TypeMetadata::List(_)
            | TypeMetadata::Function(_)
            | TypeMetadata::External(_) => Ok(true),
        }
    }

    pub(super) fn function_slot(
        &self,
        slot: &crate::plan::execution::graph::ParamSlot,
        descriptor: &ValueShapeDescriptor,
    ) -> Result<(), TypeError> {
        use crate::plan::execution::graph::ParamLocal;
        let ValueShapeDescriptor::Function { arguments, return_ } = descriptor else {
            return Ok(());
        };
        let representation = self.representation(arguments, *return_);
        if matches!(slot.local, ParamLocal::GenericFunction(_))
            != (representation == FunctionRepresentation::Symbolic)
            || matches!(slot.local, ParamLocal::NeverFunction(_))
                != (representation == FunctionRepresentation::Never)
        {
            return Err(TypeError::FunctionRepresentation);
        }
        Ok(())
    }

    pub(in crate::plan::execution::prepared::admission) fn representation(
        &self,
        arguments: &[ValueShapeId],
        return_: ValueShapeId,
    ) -> FunctionRepresentation {
        let mut active = HashSet::new();
        let mut known = HashSet::new();
        for argument in arguments.iter() {
            if !self.shape_inhabited(
                &self.shapes.shapes[argument.index()],
                &mut active,
                &mut known,
            ) {
                return FunctionRepresentation::Symbolic;
            }
        }
        if self.shape_inhabited(
            &self.shapes.shapes[return_.index()],
            &mut active,
            &mut known,
        ) {
            FunctionRepresentation::Value
        } else {
            FunctionRepresentation::Never
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{TypeError, Types};
    use crate::plan::execution::function::FunctionBodyOwner;
    use crate::plan::execution::graph::{
        GenericFunctionLocal, GenericFunctionLocalId, NeverFunctionLocal, NeverFunctionLocalId,
        ParamLocal,
    };
    use crate::plan::execution::type_::{FunctionShape, GenericFunctionType, ValueType};

    #[test]
    fn original_recursive_definitions_distinguish_symbolic_never_and_callable_slots() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub type Empty { Again(Empty) }
fn identity(value: a) { value }
fn from_empty(value: Empty) -> Int { from_empty(value) }
fn never() -> a { panic as "never" }
fn add(value: Int) { value + 1 }
pub fn main() { #(identity, from_empty, never, add) }
"#,
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let main = plan.program.functions.value_returns.tuple_functions[0]
            .body()
            .function_body()
            .block_graph();
        let functions = main
            .blocks()
            .flat_map(|block| block.instructions())
            .map(|instruction| instruction.output())
            .filter_map(|slot| match types.shape_type(slot.shape).unwrap() {
                ValueType::Function(function) => Some((slot, function)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(functions.len(), 4);
        assert_eq!(
            functions
                .iter()
                .map(|(slot, _)| match slot.local {
                    ParamLocal::GenericFunction(_) => "symbolic",
                    ParamLocal::NeverFunction(_) => "never",
                    _ => "callable",
                })
                .collect::<Vec<_>>(),
            ["symbolic", "symbolic", "never", "callable"]
        );
        assert!(
            common
                .custom_types
                .types
                .iter()
                .any(|type_| type_.type_.name.as_str() == "Empty" && type_.constructors.is_empty())
        );
        for (index, (slot, function)) in functions.into_iter().enumerate() {
            assert_eq!(*types.slot(slot).unwrap(), *slot);
            assert!(types.inhabited(types.shape(slot.shape).unwrap()));
            let representation = GenericFunctionType {
                type_: function.clone(),
                shape: FunctionShape {
                    shape_id: slot.shape,
                    type_: function.clone(),
                },
            };
            let mut forged = slot.clone();
            forged.local = if index < 2 {
                ParamLocal::NeverFunction(NeverFunctionLocal {
                    id: NeverFunctionLocalId(0),
                    type_: representation,
                })
            } else {
                ParamLocal::GenericFunction(GenericFunctionLocal {
                    id: GenericFunctionLocalId(0),
                    type_: representation,
                })
            };
            assert_eq!(types.slot(&forged), Err(TypeError::FunctionRepresentation));
        }
    }

    #[test]
    fn metadata_inhabitation_preserves_generic_choices_shared_children_and_errors() {
        use crate::plan::execution::storage::{Node, Table};
        use crate::plan::execution::type_::{
            FunctionMetadata, TypeMetadata, ValueShapeDescriptor, ValueShapeId,
        };

        let source = r#"
pub type Empty { Again(Empty) }
pub type Box(a) { Box(a) }
pub type Choice(a) { First(a) Last }
pub type Pair(a) { Pair(a, a) }
pub type TupleBox(a) { TupleBox(#(a, Int)) }
fn empty(value: Empty) { value }
fn tuple(value: #(a, Int)) { value }
pub fn main() {
  #(Box(42), First(42), Last, Pair(Box(42), Box(42)), TupleBox(#(42, 0)), empty, tuple)
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let nominal = |name: &str| {
            common
                .custom_types
                .types
                .iter()
                .find(|descriptor| descriptor.type_.name.as_str() == name)
                .unwrap()
                .type_
                .clone()
        };
        let parameter = TypeMetadata::Parameter(crate::plan::TypeParameterId(0));
        let mut boxed = nominal("Box");
        boxed.arguments = vec![parameter.clone()].into();
        let mut choice = nominal("Choice");
        choice.arguments = vec![parameter.clone()].into();
        let mut pair = nominal("Pair");
        pair.arguments = vec![TypeMetadata::Custom(nominal("Box"))].into();
        let mut tuple_box = nominal("TupleBox");
        tuple_box.arguments = vec![parameter.clone()].into();
        for (metadata, arguments, expected) in [
            (parameter.clone(), Vec::new(), false),
            (parameter.clone(), vec![true], true),
            (TypeMetadata::Custom(boxed.clone()), vec![false], false),
            (TypeMetadata::Custom(boxed.clone()), vec![true], true),
            (TypeMetadata::Custom(choice), vec![false], true),
            (TypeMetadata::Custom(pair), Vec::new(), true),
            (TypeMetadata::Custom(tuple_box.clone()), vec![true], true),
            (TypeMetadata::Custom(tuple_box), vec![false], false),
            (TypeMetadata::Custom(nominal("Empty")), Vec::new(), false),
            (
                TypeMetadata::Tuple(vec![TypeMetadata::Int, parameter.clone()].into()),
                vec![false],
                false,
            ),
            (
                TypeMetadata::Tuple(vec![TypeMetadata::Int, TypeMetadata::Bool].into()),
                Vec::new(),
                true,
            ),
            (
                TypeMetadata::List(Box::new(parameter).into()),
                Vec::new(),
                true,
            ),
            (
                TypeMetadata::Function(FunctionMetadata {
                    arguments: Table::Static(&[]),
                    return_: Node::Static(&TypeMetadata::Int),
                }),
                Vec::new(),
                true,
            ),
        ] {
            assert_eq!(
                types.metadata_inhabited(&metadata, &arguments),
                Ok(expected)
            );
        }
        let mut missing = nominal("Empty");
        missing.name = "Missing".into();
        let expected = TypeError::MissingDefinition {
            package: missing.package.to_string(),
            module: missing.module.to_string(),
            name: "Missing".into(),
        };
        let missing = TypeMetadata::Custom(missing);
        boxed.arguments = vec![missing.clone()].into();
        for metadata in [
            missing.clone(),
            TypeMetadata::Custom(boxed),
            TypeMetadata::Tuple(vec![missing].into()),
        ] {
            assert_eq!(
                types.metadata_inhabited(&metadata, &[]).err().as_ref(),
                Some(&expected)
            );
        }
        static RECURSIVE: TypeMetadata = TypeMetadata::List(Node::Static(&RECURSIVE));
        assert_eq!(
            types.metadata_inhabited(&RECURSIVE, &[]),
            Err(TypeError::RecursiveMetadata)
        );
        let symbolic_tuple = common
            .value_shapes
            .shapes
            .iter()
            .position(|shape| match shape {
                ValueShapeDescriptor::Tuple(items) => items.iter().any(|item| {
                    matches!(
                        types.shape(*item).unwrap(),
                        ValueShapeDescriptor::Parameter(_)
                    )
                }),
                _ => false,
            })
            .unwrap();
        assert!(!types.inhabited(types.shape(ValueShapeId(symbolic_tuple)).unwrap()));
        let last =
            common
                .value_shapes
                .shapes
                .iter()
                .enumerate()
                .find_map(|(index, shape)| match shape {
                    ValueShapeDescriptor::Custom(id) => {
                        let custom = &common.value_shapes.custom_shapes[id.0];
                        (common.custom_types.types[custom.type_id.index()].type_.name.as_str()
                        == "Choice"
                        && custom.constructor
                            == crate::plan::execution::type_::CustomConstructorRefinement::Exact(1))
                        .then_some(ValueShapeId(index))
                    }
                    _ => None,
                })
                .unwrap();
        assert!(types.inhabited(types.shape(last).unwrap()));
    }
}
