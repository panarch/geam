use super::{InstructionError, Instructions, read};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{CustomInstruction, ParamSlot};
use crate::plan::execution::prepared::admission::local::Locals;
use crate::plan::execution::type_::{CustomConstructorRefinement, ValueShapeDescriptor};

impl<'data, Graph: ExecutionGraphProfile> Instructions<'_, 'data, Graph> {
    pub(super) fn custom(
        &self,
        instruction: &CustomInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        let admitted = self.types.slot(output).map_err(InstructionError::Type)?;
        let ValueShapeDescriptor::Custom(id) = admitted.descriptor else {
            return Err(InstructionError::OutputType);
        };
        let shape = &self.types.shapes.custom_shapes[id.0];
        match instruction {
            CustomInstruction::Construct {
                constructor,
                fields,
            } => {
                if constructor.type_id != shape.type_id
                    || matches!(shape.constructor, CustomConstructorRefinement::Exact(index) if index != constructor.index)
                {
                    return Err(InstructionError::OutputType);
                }
                let descriptor = self
                    .types
                    .constructor_descriptor(*constructor)
                    .map_err(InstructionError::Type)?;
                if fields.len() != descriptor.fields.len() {
                    return Err(InstructionError::Arity {
                        expected: descriptor.fields.len(),
                        found: fields.len(),
                    });
                }
                for (source, field) in fields.iter().zip(descriptor.fields.iter()) {
                    let source = read(source, locals)?;
                    if source.type_ != &field.type_ {
                        return Err(InstructionError::OperandType);
                    }
                    if !self.types.can_enter_field(
                        source.shape,
                        &field.refinement,
                        &shape.arguments,
                    ) {
                        return Err(InstructionError::Flow);
                    }
                }
                Ok(())
            }
            CustomInstruction::Constant(id) => self.constant(*id, output),
            CustomInstruction::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            CustomInstruction::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            CustomInstruction::TupleIndex { tuple, index } => {
                self.tuple_index(tuple, *index, output, locals)
            }
            CustomInstruction::CustomField { source, index } => {
                self.custom_field(source, *index, output, locals)
            }
            CustomInstruction::ListIndex { list, index: _ } => {
                self.list_index(list, output, locals)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{CustomInstruction, InstructionError, Instructions, Locals};
    use crate::plan::execution::graph::ProfiledInstructionKind;
    use crate::plan::execution::graph::{CustomLocal, CustomLocalId, ParamLocal, ParamSlot};
    use crate::plan::execution::prepared::admission::{
        catalog::Catalog, source::Sources, type_::Types,
    };
    use crate::plan::execution::prepared::admission::{local::LocalError, type_::TypeError};
    use crate::plan::execution::type_::ValueShapeId;

    #[test]
    fn checks_generic_construction_and_projection_without_losing_refinements() {
        let source = r#"
pub type Choice { First Second }
pub type Box(a) { Box(value: a) }
pub type Pair(a) { Pair(values: #(a, List(a)), nested: Box(a), mapper: fn(a) -> a) }
fn keep(x) { x }
fn first(values, fallback) {
  case values {
    [value, ..] -> value
    _ -> fallback
  }
}
pub fn main() {
  let pair = Pair(#(First, [First]), Box(First), keep)
  let held = Box(pair)
  let nested = held.value.nested
  first([nested], nested)
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
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let mut constructors = 0;
        let mut projections = 0;
        let mut list_projections = 0;
        for entry in plan.program.functions.value_returns.custom_functions.iter() {
            for block in entry.body().function_body().block_graph().blocks() {
                let mut locals = Locals::default();
                for slot in block.params() {
                    locals.define(slot, &types).unwrap();
                }
                for instruction in block.instructions() {
                    if let ProfiledInstructionKind::Custom(value) = instruction.kind() {
                        context
                            .custom(value, instruction.output(), &locals)
                            .unwrap();
                        if let CustomInstruction::Construct {
                            constructor,
                            fields,
                        } = value
                        {
                            constructors += 1;
                            let mut extra = fields.iter().cloned().collect::<Vec<_>>();
                            extra.push(instruction.output().local.clone());
                            assert_eq!(
                                context.custom(
                                    &CustomInstruction::Construct {
                                        constructor: *constructor,
                                        fields: extra.into()
                                    },
                                    instruction.output(),
                                    &locals
                                ),
                                Err(InstructionError::Arity {
                                    expected: fields.len(),
                                    found: fields.len() + 1
                                })
                            );
                        }
                        if let CustomInstruction::CustomField { source, index } = value {
                            projections += 1;
                            assert_eq!(
                                context.custom_field(source, 999, instruction.output(), &locals),
                                Err(InstructionError::CustomField { index: 999 }),
                            );
                            assert_eq!(
                                context.custom_field(
                                    &CustomLocal {
                                        id: CustomLocalId(999),
                                        ..*source
                                    },
                                    *index,
                                    instruction.output(),
                                    &locals,
                                ),
                                Err(InstructionError::Local(LocalError::Missing(
                                    CustomLocalId(999).into()
                                ))),
                            );
                            let missing = ParamSlot {
                                shape: ValueShapeId(999),
                                ..instruction.output().clone()
                            };
                            assert_eq!(
                                context.custom_field(source, *index, &missing, &locals),
                                Err(InstructionError::Type(TypeError::MissingShape {
                                    index: 999
                                })),
                            );
                            locals.restrict_constructors(&ParamLocal::Custom(*source), vec![]);
                            assert_eq!(
                                context.custom_field(source, *index, instruction.output(), &locals),
                                Err(InstructionError::CustomField { index: *index }),
                            );
                            locals.restrict_constructors(
                                &ParamLocal::Custom(*source),
                                (0..common.custom_types.types[source.shape.type_id.index()]
                                    .constructor_count)
                                    .collect(),
                            );
                        }
                        if matches!(value, CustomInstruction::ListIndex { .. }) {
                            list_projections += 1;
                        }
                    }
                    locals.define(instruction.output(), &types).unwrap();
                }
            }
        }
        assert_eq!((constructors, projections, list_projections), (6, 2, 1));
    }

    #[test]
    fn constructor_operands_must_match_the_nominal_type_and_generic_refinements() {
        use crate::plan::execution::graph::{
            BoolLocalId, CustomLocal, CustomLocalId, IntLocalId, ParamLocal, ParamSlot,
        };
        use crate::plan::execution::prepared::admission::{local::LocalError, type_::TypeError};
        use crate::plan::execution::type_::{
            CustomConstructorId, CustomConstructorRefinement, CustomTypeId, CustomValueShape,
            CustomValueShapeId, ValueShapeDescriptor, ValueShapeId,
        };

        let source = r#"
pub type Choice { First(Int) Second(String) }
pub type Box(a) { Box(a) }
fn widen(value: Choice) { value }
pub fn main() { #(Box(First(42)), Second("text"), widen(First(42)), True, 42) }
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
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let custom_slot = |name: &str, constructor, local| {
            let (index, shape) = common
                .value_shapes
                .custom_shapes
                .iter()
                .enumerate()
                .find(|(_, shape)| {
                    common.custom_types.types[shape.type_id.index()]
                        .type_
                        .name
                        .as_str()
                        == name
                        && shape.constructor == constructor
                })
                .unwrap();
            let shape_id = CustomValueShapeId(index);
            let value = common
                .value_shapes
                .shapes
                .iter()
                .position(|shape| shape == &ValueShapeDescriptor::Custom(shape_id))
                .unwrap();
            let local = CustomLocal {
                id: CustomLocalId(local),
                shape: CustomValueShape {
                    type_id: shape.type_id,
                    shape_id,
                },
            };
            (
                ParamSlot {
                    local: ParamLocal::Custom(local),
                    shape: ValueShapeId(value),
                },
                local,
            )
        };
        let (first, first_local) = custom_slot("Choice", CustomConstructorRefinement::Exact(0), 0);
        let (second, _) = custom_slot("Choice", CustomConstructorRefinement::Exact(1), 1);
        let (any, _) = custom_slot("Choice", CustomConstructorRefinement::Any, 2);
        let (boxed, boxed_local) = custom_slot("Box", CustomConstructorRefinement::Exact(0), 3);
        let boolean = ParamSlot {
            local: ParamLocal::Bool(BoolLocalId(0)),
            shape: ValueShapeId(
                common
                    .value_shapes
                    .shapes
                    .iter()
                    .position(|shape| shape == &ValueShapeDescriptor::Bool)
                    .unwrap(),
            ),
        };
        let integer = ParamSlot {
            local: ParamLocal::Int(IntLocalId(0)),
            shape: ValueShapeId(
                common
                    .value_shapes
                    .shapes
                    .iter()
                    .position(|shape| shape == &ValueShapeDescriptor::Int)
                    .unwrap(),
            ),
        };
        let box_type = common
            .custom_types
            .types
            .iter()
            .position(|type_| type_.type_.name.as_str() == "Box")
            .unwrap();
        let choice_type = common
            .custom_types
            .types
            .iter()
            .position(|type_| type_.type_.name.as_str() == "Choice")
            .unwrap();
        let constructor = CustomConstructorId {
            type_id: CustomTypeId(box_type),
            index: 0,
        };
        let valid = CustomInstruction::Construct {
            constructor,
            fields: vec![first.local.clone()].into(),
        };
        let mut locals = Locals::default();
        for slot in [&first, &second, &any, &boolean, &integer] {
            locals.define(slot, &types).unwrap();
        }
        assert_eq!(context.custom(&valid, &boxed, &locals), Ok(()));
        assert_eq!(
            context.custom_field(&first_local, 0, &integer, &locals),
            Ok(())
        );
        locals.define(&boxed, &types).unwrap();
        assert_eq!(
            context.custom_field(&boxed_local, 0, &first, &locals),
            Ok(())
        );
        assert_eq!(
            context.custom_field(&boxed_local, 0, &second, &locals),
            Err(InstructionError::Flow)
        );
        assert_eq!(
            context.custom(&valid, &boolean, &locals),
            Err(InstructionError::OutputType)
        );
        let missing_shape = ParamSlot {
            shape: ValueShapeId(999),
            ..boxed.clone()
        };
        assert_eq!(
            context.custom(&valid, &missing_shape, &locals),
            Err(InstructionError::Type(TypeError::MissingShape {
                index: 999
            }))
        );
        for (fields, expected) in [
            (vec![second.local.clone()], InstructionError::Flow),
            (vec![boolean.local.clone()], InstructionError::OperandType),
            (
                vec![ParamLocal::Int(IntLocalId(999))],
                InstructionError::Local(LocalError::Missing(IntLocalId(999).into())),
            ),
        ] {
            assert_eq!(
                context.custom(
                    &CustomInstruction::Construct {
                        constructor,
                        fields: fields.into()
                    },
                    &boxed,
                    &locals
                ),
                Err(expected)
            );
        }
        for constructor in [
            CustomConstructorId {
                type_id: CustomTypeId(choice_type),
                index: 0,
            },
            CustomConstructorId {
                type_id: CustomTypeId(box_type),
                index: 1,
            },
        ] {
            assert_eq!(
                context.custom(
                    &CustomInstruction::Construct {
                        constructor,
                        fields: vec![first.local.clone()].into()
                    },
                    &boxed,
                    &locals
                ),
                Err(InstructionError::OutputType)
            );
        }
        assert_eq!(
            context.custom(
                &CustomInstruction::Construct {
                    constructor: CustomConstructorId {
                        type_id: CustomTypeId(choice_type),
                        index: 99
                    },
                    fields: Vec::new().into()
                },
                &any,
                &locals
            ),
            Err(InstructionError::Type(TypeError::MissingConstructor {
                type_index: choice_type,
                index: 99
            }))
        );
    }
}
