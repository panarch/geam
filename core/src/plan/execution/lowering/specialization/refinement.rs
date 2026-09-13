use super::{RepresentationContext, is_result};
use crate::plan::execution::type_::custom::FieldRefinement;
use crate::plan::{CustomConstructor, CustomTypeParameterId, CustomTypeTemplate};
use std::collections::HashMap;

impl RepresentationContext {
    pub(super) fn constructor_refinements(
        &self,
        constructor: &CustomConstructor,
    ) -> Vec<FieldRefinement> {
        if is_result(constructor.type_().type_name()) {
            return vec![FieldRefinement::Argument(constructor.index())];
        }
        let definition = &self.custom_types[constructor.type_().type_name()];
        let parameters = definition
            .parameters()
            .iter()
            .copied()
            .enumerate()
            .map(|(index, parameter)| (parameter, index))
            .collect();
        definition.constructors()[constructor.index()]
            .fields()
            .iter()
            .map(|field| field_refinement(field.type_(), &parameters))
            .collect()
    }
}

fn field_refinement(
    type_: &CustomTypeTemplate,
    parameters: &HashMap<CustomTypeParameterId, usize>,
) -> FieldRefinement {
    match type_ {
        CustomTypeTemplate::Parameter(parameter) => {
            FieldRefinement::Argument(parameters[parameter])
        }
        CustomTypeTemplate::Int
        | CustomTypeTemplate::Float
        | CustomTypeTemplate::String
        | CustomTypeTemplate::BitArray
        | CustomTypeTemplate::UtfCodepoint
        | CustomTypeTemplate::Bool
        | CustomTypeTemplate::Nil
        | CustomTypeTemplate::External { .. } => FieldRefinement::Value,
        CustomTypeTemplate::Tuple(elements) => FieldRefinement::Tuple(
            elements
                .iter()
                .map(|element| field_refinement(element, parameters))
                .collect(),
        ),
        CustomTypeTemplate::List(item) => {
            FieldRefinement::List(Box::new(field_refinement(item, parameters)).into())
        }
        CustomTypeTemplate::Function { arguments, return_ } => FieldRefinement::Function {
            arguments: arguments
                .iter()
                .map(|argument| field_refinement(argument, parameters))
                .collect(),
            return_: Box::new(field_refinement(return_, parameters)).into(),
        },
        CustomTypeTemplate::Custom { name: _, arguments } => FieldRefinement::Custom(
            arguments
                .iter()
                .map(|argument| field_refinement(argument, parameters))
                .collect(),
        ),
    }
}
