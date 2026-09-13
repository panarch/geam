use super::Types;
use crate::plan::execution::type_::custom::FieldRefinement;
use crate::plan::execution::type_::{
    CustomConstructorRefinement, ValueShapeDescriptor, ValueShapeId,
};
use std::collections::HashSet;

#[derive(Clone, Copy)]
enum Direction {
    Into,
    Out,
}

impl Types<'_> {
    pub(in crate::plan::execution::prepared::admission) fn is_nominal_shape(
        &self,
        root: ValueShapeId,
    ) -> bool {
        // The shape walk or function catalog has already admitted these links.
        let mut pending = vec![root];
        let mut complete = HashSet::new();
        while let Some(shape) = pending.pop() {
            if !complete.insert(shape) {
                continue;
            }
            match &self.shapes.shapes[shape.index()] {
                ValueShapeDescriptor::Custom(id) => {
                    let custom = &self.shapes.custom_shapes[id.0];
                    if custom.constructor != CustomConstructorRefinement::Any {
                        return false;
                    }
                    pending.extend(custom.arguments.iter().copied());
                }
                ValueShapeDescriptor::Tuple(elements) => pending.extend(elements.iter().copied()),
                ValueShapeDescriptor::List(item) => pending.push(*item),
                ValueShapeDescriptor::Function { arguments, return_ } => {
                    pending.extend(arguments.iter().copied());
                    pending.push(*return_);
                }
                ValueShapeDescriptor::Parameter(_)
                | ValueShapeDescriptor::Int
                | ValueShapeDescriptor::Float
                | ValueShapeDescriptor::String
                | ValueShapeDescriptor::BitArray
                | ValueShapeDescriptor::UtfCodepoint
                | ValueShapeDescriptor::Bool
                | ValueShapeDescriptor::Nil
                | ValueShapeDescriptor::External(_) => {}
            }
        }
        true
    }

    pub(in crate::plan::execution::prepared::admission) fn can_enter_field(
        &self,
        source: ValueShapeId,
        rule: &FieldRefinement,
        arguments: &[ValueShapeId],
    ) -> bool {
        self.field_flow(rule, arguments, source, Direction::Into)
    }

    pub(in crate::plan::execution::prepared::admission) fn can_leave_field(
        &self,
        rule: &FieldRefinement,
        arguments: &[ValueShapeId],
        target: ValueShapeId,
    ) -> bool {
        self.field_flow(rule, arguments, target, Direction::Out)
    }

    fn field_flow(
        &self,
        root: &FieldRefinement,
        arguments: &[ValueShapeId],
        actual: ValueShapeId,
        direction: Direction,
    ) -> bool {
        // Rules, argument links and source slots come from admitted type data.
        let mut pending = vec![(root, actual, direction)];
        while let Some((rule, actual, direction)) = pending.pop() {
            let shape = &self.shapes.shapes[actual.index()];
            match (rule, shape) {
                (FieldRefinement::Argument(index), _) => {
                    let expected = arguments[*index];
                    let matches = match direction {
                        Direction::Into => self.flow(actual, expected),
                        Direction::Out => self.flow(expected, actual),
                    };
                    if !matches {
                        return false;
                    }
                }
                (
                    FieldRefinement::Value,
                    ValueShapeDescriptor::Int
                    | ValueShapeDescriptor::Float
                    | ValueShapeDescriptor::String
                    | ValueShapeDescriptor::BitArray
                    | ValueShapeDescriptor::UtfCodepoint
                    | ValueShapeDescriptor::Bool
                    | ValueShapeDescriptor::Nil
                    | ValueShapeDescriptor::External(_),
                ) => {}
                (FieldRefinement::Tuple(rules), ValueShapeDescriptor::Tuple(values)) => {
                    if rules.len() != values.len() {
                        return false;
                    }
                    pending.extend(
                        rules
                            .iter()
                            .zip(values.iter())
                            .map(|(rule, value)| (rule, *value, direction)),
                    );
                }
                (FieldRefinement::List(rule), ValueShapeDescriptor::List(value)) => {
                    pending.push((rule, *value, direction))
                }
                (
                    FieldRefinement::Function {
                        arguments: rules,
                        return_: rule,
                    },
                    ValueShapeDescriptor::Function {
                        arguments: values,
                        return_: value,
                    },
                ) => {
                    if rules.len() != values.len() {
                        return false;
                    }
                    let reversed = match direction {
                        Direction::Into => Direction::Out,
                        Direction::Out => Direction::Into,
                    };
                    pending.extend(
                        rules
                            .iter()
                            .zip(values.iter())
                            .map(|(rule, value)| (rule, *value, reversed)),
                    );
                    pending.push((rule, *value, direction));
                }
                (FieldRefinement::Custom(rules), ValueShapeDescriptor::Custom(id)) => {
                    let shape = &self.shapes.custom_shapes[id.0];
                    if rules.len() != shape.arguments.len()
                        || matches!(direction, Direction::Out)
                            && shape.constructor != CustomConstructorRefinement::Any
                    {
                        return false;
                    }
                    pending.extend(
                        rules
                            .iter()
                            .zip(shape.arguments.iter())
                            .map(|(rule, value)| (rule, *value, direction)),
                    );
                }
                _ => return false,
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CustomConstructorRefinement, FieldRefinement, Types, ValueShapeDescriptor, ValueShapeId,
    };

    #[test]
    fn preserves_generic_constructor_refinements_in_both_directions() {
        let typed = crate::compile_typed_module("example", "src/example.gleam", "pub type Choice { First Second } pub type Box(a) { Box(value: a) } pub fn main() { #(Box(First), Box(Second)) }").unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let choices = common
            .value_shapes
            .shapes
            .iter()
            .enumerate()
            .filter_map(|(index, shape)| {
                let ValueShapeDescriptor::Custom(id) = shape else {
                    return None;
                };
                let shape = &common.value_shapes.custom_shapes[id.0];
                (common.custom_types.types[shape.type_id.index()]
                    .type_
                    .name
                    .as_ref()
                    == "Choice")
                    .then_some((ValueShapeId(index), shape.constructor))
            })
            .collect::<Vec<_>>();
        let first = choices
            .iter()
            .find(|(_, constructor)| *constructor == CustomConstructorRefinement::Exact(0))
            .unwrap()
            .0;
        let second = choices
            .iter()
            .find(|(_, constructor)| *constructor == CustomConstructorRefinement::Exact(1))
            .unwrap()
            .0;
        let rule = FieldRefinement::Argument(0);
        assert!(types.can_enter_field(first, &rule, &[first]));
        assert!(!types.can_enter_field(second, &rule, &[first]));
        assert!(types.can_leave_field(&rule, &[first], first));
        assert!(!types.can_leave_field(&rule, &[first], second));
    }

    #[test]
    fn nested_fields_preserve_arity_nominal_outputs_and_function_variance() {
        let source = r#"
pub type Choice { First Second }
pub type Box(a) { Box(a) }
pub type Envelope(a) {
  Envelope(pair: #(a, Int), items: List(a), callback: fn(a) -> Int, boxed: Box(a))
}
fn choice(value: Choice) { value }
fn boxed(value: Box(Choice)) { value }
fn make(value: Choice) { Envelope(#(value, 1), [value], fn(_) { 1 }, Box(value)) }
pub fn main() {
  let value = choice(First)
  let _ = #(make(value), boxed(Box(value)), First, #(value), #(value, 1),
    fn(_a: Choice, _b: Int) { 1 }, fn(_a: Choice) { 1 })
  42
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
        let nominal = |name: &str, constructor| {
            let index = common
                .value_shapes
                .custom_shapes
                .iter()
                .position(|shape| {
                    common.custom_types.types[shape.type_id.index()]
                        .type_
                        .name
                        .as_str()
                        == name
                        && shape.constructor == constructor
                })
                .unwrap();
            ValueShapeId(
                common
                    .value_shapes
                    .shapes
                    .iter()
                    .position(|shape| {
                        shape
                            == &ValueShapeDescriptor::Custom(
                                crate::plan::execution::type_::CustomValueShapeId(index),
                            )
                    })
                    .unwrap(),
            )
        };
        let choice = nominal("Choice", CustomConstructorRefinement::Any);
        let first = nominal("Choice", CustomConstructorRefinement::Exact(0));
        let boxed = nominal("Box", CustomConstructorRefinement::Any);
        let exact_box = nominal("Box", CustomConstructorRefinement::Exact(0));
        let envelope = common
            .custom_types
            .types
            .iter()
            .find(|descriptor| descriptor.type_.name.as_str() == "Envelope")
            .unwrap();
        let fields = &envelope.constructors[0].fields;
        assert_eq!(fields.len(), 4);
        let pair = &fields[0].refinement;
        let callback = &fields[2].refinement;
        let boxed_rule = &fields[3].refinement;
        assert!(types.is_nominal_shape(choice));
        assert!(!types.is_nominal_shape(first));
        let tuple = common
            .value_shapes
            .shapes
            .iter()
            .position(|shape| shape == &ValueShapeDescriptor::Tuple(vec![choice].into()))
            .unwrap();
        assert!(!types.can_enter_field(ValueShapeId(tuple), pair, &[choice]));
        assert!(types.can_enter_field(fields[0].shape, pair, &[choice]));
        assert!(types.can_leave_field(pair, &[choice], fields[0].shape));
        let unary = fields[2].shape;
        let binary = common
            .value_shapes
            .shapes
            .iter()
            .position(|shape| match shape {
                ValueShapeDescriptor::Function { arguments, .. } => arguments.len() == 2,
                _ => false,
            })
            .unwrap();
        assert!(!types.can_enter_field(ValueShapeId(binary), callback, &[choice]));
        assert!(types.can_enter_field(unary, callback, &[choice]));
        assert!(types.can_leave_field(callback, &[choice], unary));
        assert!(!types.can_leave_field(callback, &[first], unary));
        assert!(!types.can_enter_field(choice, boxed_rule, &[choice]));
        assert!(types.can_enter_field(exact_box, boxed_rule, &[choice]));
        assert!(types.can_leave_field(boxed_rule, &[choice], boxed));
        assert!(!types.can_leave_field(boxed_rule, &[choice], exact_box));
        assert!(!types.can_enter_field(choice, pair, &[choice]));
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()),
            Ok(crate::Value::Int(42.into()))
        );
    }
}
