use super::super::specialization::{
    FunctionRepresentation, RepresentationContext, SpecializedCustomValueShape,
    SpecializedFunctionShape, SpecializedTypeSubstitution, SpecializedValueShape,
};
use crate::host::{HostCustomTypeSchema, HostSchemaType, HostTypeDescriptor};
use crate::plan::{
    CustomConstructorRefinement, CustomTypeName, FunctionType, HostFunctionTemplate,
};
use ecow::EcoString;
use std::collections::{HashMap, HashSet};

type CustomIdentity = (EcoString, EcoString, EcoString);

pub(super) fn first_uninhabited_callback(
    template: &HostFunctionTemplate,
    substitution: &SpecializedTypeSubstitution,
    representations: &RepresentationContext,
    include_return: bool,
) -> Option<FunctionType> {
    let schemas = template
        .custom_schemas()
        .iter()
        .map(|schema| (identity(schema), schema))
        .collect();
    let mut search = CallbackSearch {
        substitution,
        representations,
        schemas,
        visiting: HashSet::new(),
    };

    for parameter in template.parameters() {
        if let Some(callback) = search.find(parameter) {
            return Some(callback);
        }
    }
    if include_return {
        return search.find(template.return_type());
    }
    None
}

struct CallbackSearch<'a> {
    substitution: &'a SpecializedTypeSubstitution,
    representations: &'a RepresentationContext,
    schemas: HashMap<CustomIdentity, &'a HostCustomTypeSchema>,
    visiting: HashSet<SpecializedCustomValueShape>,
}

impl CallbackSearch<'_> {
    fn find(&mut self, descriptor: &HostTypeDescriptor) -> Option<FunctionType> {
        match descriptor {
            HostTypeDescriptor::Parameter(_)
            | HostTypeDescriptor::Int
            | HostTypeDescriptor::Float
            | HostTypeDescriptor::String
            | HostTypeDescriptor::BitArray
            | HostTypeDescriptor::UtfCodepoint
            | HostTypeDescriptor::Bool
            | HostTypeDescriptor::Nil => None,
            HostTypeDescriptor::List(item) => {
                let item_shape =
                    SpecializedValueShape::instantiate(&item.value_shape(), self.substitution);
                self.representations
                    .is_inhabited(&item_shape)
                    .then(|| self.find(item))
                    .flatten()
            }
            HostTypeDescriptor::Tuple(elements) => {
                for element in elements {
                    if let Some(callback) = self.find(element) {
                        return Some(callback);
                    }
                }
                None
            }
            HostTypeDescriptor::Function { arguments, return_ } => {
                let function = SpecializedFunctionShape::new(
                    arguments
                        .iter()
                        .map(|argument| {
                            SpecializedValueShape::instantiate(
                                &argument.value_shape(),
                                self.substitution,
                            )
                        })
                        .collect(),
                    SpecializedValueShape::instantiate(&return_.value_shape(), self.substitution),
                );
                match function.representation(self.representations) {
                    FunctionRepresentation::Symbolic => Some(function.to_module_shape().type_()),
                    FunctionRepresentation::Never(_) => None,
                    FunctionRepresentation::Executable(_) => self.find(return_),
                }
            }
            HostTypeDescriptor::Custom { schema, arguments } => self.find_custom(schema, arguments),
        }
    }

    fn find_custom(
        &mut self,
        schema: &HostCustomTypeSchema,
        arguments: &[HostTypeDescriptor],
    ) -> Option<FunctionType> {
        let concrete = SpecializedCustomValueShape::new(
            CustomTypeName::new(
                schema.package().clone(),
                schema.module().clone(),
                schema.name().clone(),
            ),
            arguments
                .iter()
                .map(|argument| {
                    SpecializedValueShape::instantiate(&argument.value_shape(), self.substitution)
                })
                .collect(),
            CustomConstructorRefinement::Any,
        );
        if !self.visiting.insert(concrete.clone()) {
            return None;
        }

        for constructor in schema.constructors() {
            let fields = constructor
                .fields()
                .iter()
                .map(|field| self.descriptor(field.type_(), arguments))
                .collect::<Vec<_>>();
            if fields.iter().all(|field| {
                let shape =
                    SpecializedValueShape::instantiate(&field.value_shape(), self.substitution);
                self.representations.is_inhabited(&shape)
            }) {
                for field in &fields {
                    if let Some(callback) = self.find(field) {
                        self.visiting.remove(&concrete);
                        return Some(callback);
                    }
                }
            }
        }

        self.visiting.remove(&concrete);
        None
    }

    fn descriptor(
        &self,
        type_: &HostSchemaType,
        arguments: &[HostTypeDescriptor],
    ) -> HostTypeDescriptor {
        match type_ {
            HostSchemaType::Parameter(index) => arguments[*index].clone(),
            HostSchemaType::Int => HostTypeDescriptor::Int,
            HostSchemaType::Float => HostTypeDescriptor::Float,
            HostSchemaType::String => HostTypeDescriptor::String,
            HostSchemaType::BitArray => HostTypeDescriptor::BitArray,
            HostSchemaType::UtfCodepoint => HostTypeDescriptor::UtfCodepoint,
            HostSchemaType::Bool => HostTypeDescriptor::Bool,
            HostSchemaType::Nil => HostTypeDescriptor::Nil,
            HostSchemaType::List(item) => {
                HostTypeDescriptor::List(Box::new(self.descriptor(item, arguments)))
            }
            HostSchemaType::Tuple(elements) => HostTypeDescriptor::Tuple(
                elements
                    .iter()
                    .map(|element| self.descriptor(element, arguments))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ),
            HostSchemaType::Function {
                arguments: function_arguments,
                return_,
            } => HostTypeDescriptor::Function {
                arguments: function_arguments
                    .iter()
                    .map(|argument| self.descriptor(argument, arguments))
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
                return_: Box::new(self.descriptor(return_, arguments)),
            },
            HostSchemaType::Custom {
                package,
                module,
                name,
                arguments: custom_arguments,
            } => {
                let identity = (package.clone(), module.clone(), name.clone());
                HostTypeDescriptor::Custom {
                    schema: self.schemas[&identity].clone(),
                    arguments: custom_arguments
                        .iter()
                        .map(|argument| self.descriptor(argument, arguments))
                        .collect::<Vec<_>>()
                        .into_boxed_slice(),
                }
            }
        }
    }
}

fn identity(schema: &HostCustomTypeSchema) -> CustomIdentity {
    (
        schema.package().clone(),
        schema.module().clone(),
        schema.name().clone(),
    )
}
