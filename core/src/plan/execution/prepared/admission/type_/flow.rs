use super::{TypeError, Types, ValueShapeDescriptor};
use crate::plan::execution::type_::{CustomConstructorRefinement, CustomValueShape, ValueShapeId};
use std::collections::HashSet;

impl Types<'_> {
    pub(in crate::plan::execution::prepared::admission) fn equivalent(
        &self,
        left: ValueShapeId,
        right: ValueShapeId,
    ) -> Result<bool, TypeError> {
        self.shape_type(left)?;
        self.shape_type(right)?;
        Ok(self.flow(left, right) && self.flow(right, left))
    }

    // This is the source ValueShape flow relation over admitted, borrowed IDs.
    // Function inputs are contravariant; all other stored children are covariant.
    pub(in crate::plan::execution::prepared::admission) fn can_flow(
        &self,
        source: ValueShapeId,
        target: ValueShapeId,
    ) -> Result<bool, TypeError> {
        self.shape_type(source)?;
        self.shape_type(target)?;
        Ok(self.flow(source, target))
    }

    pub(in crate::plan::execution::prepared::admission) fn can_flow_custom(
        &self,
        source: CustomValueShape,
        target: CustomValueShape,
    ) -> Result<bool, TypeError> {
        let source = self.custom_value_shape(source)?;
        let target = self.custom_value_shape(target)?;
        if source.type_id != target.type_id
            || matches!(target.constructor, CustomConstructorRefinement::Exact(index) if source.constructor != CustomConstructorRefinement::Exact(index))
        {
            return Ok(false);
        }
        Ok(source
            .arguments
            .iter()
            .zip(target.arguments.iter())
            .all(|(source, target)| self.flow(*source, *target)))
    }

    // Admission has already validated every child ID in the immutable type tables.
    pub(in crate::plan::execution::prepared::admission) fn flow(
        &self,
        source: ValueShapeId,
        target: ValueShapeId,
    ) -> bool {
        let mut pending = vec![(source, target)];
        let mut checked = HashSet::new();
        while let Some((source, target)) = pending.pop() {
            if source == target || !checked.insert((source, target)) {
                continue;
            }
            match (
                &self.shapes.shapes[source.index()],
                &self.shapes.shapes[target.index()],
            ) {
                (
                    ValueShapeDescriptor::Parameter(source),
                    ValueShapeDescriptor::Parameter(target),
                ) => {
                    if source != target {
                        return false;
                    }
                }
                (ValueShapeDescriptor::Int, ValueShapeDescriptor::Int)
                | (ValueShapeDescriptor::Float, ValueShapeDescriptor::Float)
                | (ValueShapeDescriptor::String, ValueShapeDescriptor::String)
                | (ValueShapeDescriptor::BitArray, ValueShapeDescriptor::BitArray)
                | (ValueShapeDescriptor::UtfCodepoint, ValueShapeDescriptor::UtfCodepoint)
                | (ValueShapeDescriptor::Bool, ValueShapeDescriptor::Bool)
                | (ValueShapeDescriptor::Nil, ValueShapeDescriptor::Nil) => {}
                (
                    ValueShapeDescriptor::External(source),
                    ValueShapeDescriptor::External(target),
                ) => {
                    if source != target {
                        return false;
                    }
                }
                (ValueShapeDescriptor::Tuple(source), ValueShapeDescriptor::Tuple(target)) => {
                    if source.len() != target.len() {
                        return false;
                    }
                    pending.extend(source.iter().copied().zip(target.iter().copied()));
                }
                (
                    ValueShapeDescriptor::List(source_item),
                    ValueShapeDescriptor::List(target_item),
                ) => {
                    // Typed runtime handles retain this identity through projection.
                    if self.shapes.shape_types[source.index()]
                        != self.shapes.shape_types[target.index()]
                    {
                        return false;
                    }
                    pending.push((*source_item, *target_item));
                }
                (
                    ValueShapeDescriptor::Function {
                        arguments: source_args,
                        return_: source_return,
                    },
                    ValueShapeDescriptor::Function {
                        arguments: target_args,
                        return_: target_return,
                    },
                ) => {
                    if source_args.len() != target_args.len() {
                        return false;
                    }
                    pending.extend(target_args.iter().copied().zip(source_args.iter().copied()));
                    pending.push((*source_return, *target_return));
                }
                (ValueShapeDescriptor::Custom(source), ValueShapeDescriptor::Custom(target)) => {
                    let source = &self.shapes.custom_shapes[source.0];
                    let target = &self.shapes.custom_shapes[target.0];
                    if source.type_id != target.type_id {
                        return false;
                    }
                    if let CustomConstructorRefinement::Exact(index) = target.constructor
                        && source.constructor != CustomConstructorRefinement::Exact(index)
                    {
                        return false;
                    }
                    pending.extend(
                        source
                            .arguments
                            .iter()
                            .copied()
                            .zip(target.arguments.iter().copied()),
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
        CustomConstructorRefinement, CustomValueShape, TypeError, Types, ValueShapeDescriptor,
        ValueShapeId,
    };
    use crate::plan::Text;
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::{
        CustomConstructorDescriptor, CustomConstructorId, CustomTypeDescriptor, CustomTypeId,
        CustomTypeTable, CustomValueShapeDescriptor, CustomValueShapeId, ExternalTypeTable,
        FunctionType, ListStorageTypeId, ListTypeId, ListTypeTable, NominalTypeMetadata,
        ValueShapeTable, ValueType,
    };

    #[test]
    fn list_flow_preserves_storage_identity_between_equivalent_item_shapes() {
        use crate::plan::execution::type_::IntListTypeId;

        let lists = ListTypeTable {
            types: [0, 1]
                .into_iter()
                .map(|index| {
                    ListStorageTypeId::Int(IntListTypeId {
                        list_type: ListTypeId(index),
                    })
                })
                .collect(),
            tuple_items: Table::Static(&[]),
            function_items: Table::Static(&[]),
        };
        let customs = CustomTypeTable::new(Vec::new(), Vec::new());
        let externals = ExternalTypeTable {
            types: Table::Static(&[]),
        };
        let shapes = ValueShapeTable {
            shapes: Table::Static(&[
                ValueShapeDescriptor::Int,
                ValueShapeDescriptor::List(ValueShapeId(0)),
                ValueShapeDescriptor::List(ValueShapeId(0)),
                ValueShapeDescriptor::List(ValueShapeId(0)),
            ]),
            shape_types: Table::Static(&[
                ValueType::Int,
                ValueType::List(ListTypeId(0)),
                ValueType::List(ListTypeId(0)),
                ValueType::List(ListTypeId(1)),
            ]),
            custom_shapes: Table::Static(&[]),
        };
        let types = Types::admit(&lists, &customs, &externals, &shapes).unwrap();
        for (source, target, expected) in [(1, 2, true), (2, 1, true), (1, 3, false), (3, 1, false)]
        {
            assert_eq!(
                types.can_flow(ValueShapeId(source), ValueShapeId(target)),
                Ok(expected)
            );
            assert_eq!(
                types.equivalent(ValueShapeId(source), ValueShapeId(target)),
                Ok(expected)
            );
        }
    }

    #[test]
    fn preserves_constructor_widening_and_function_variance_over_borrowed_shapes() {
        let custom = CustomTypeId(0);
        let lists = ListTypeTable {
            types: vec![ListStorageTypeId::Custom(
                crate::plan::execution::type_::CustomListTypeId {
                    list_type: ListTypeId(0),
                    item_type: custom,
                },
            )]
            .into(),
            tuple_items: Table::Static(&[]),
            function_items: Table::Static(&[]),
        };
        let customs = CustomTypeTable {
            definitions: vec![crate::plan::execution::type_::custom::CustomDefinition {
                package: Text::Static("example"),
                module: Text::Static("example"),
                name: Text::Static("Choice"),
                publicity: crate::plan::CustomTypePublicity::Public,
                opaque: false,
                parameters: 0,
                constructors: ["First", "Second"]
                    .into_iter()
                    .map(
                        |name| crate::plan::execution::type_::custom::ConstructorDefinition {
                            name: Text::Static(name),
                            fields: Table::Static(&[]),
                        },
                    )
                    .collect(),
            }]
            .into(),
            types: vec![CustomTypeDescriptor {
                constructor_count: 2,
                type_: NominalTypeMetadata {
                    package: Text::Static("example"),
                    module: Text::Static("example"),
                    name: Text::Static("Choice"),
                    arguments: Table::Static(&[]),
                },
                constructors: ["First", "Second"]
                    .into_iter()
                    .enumerate()
                    .map(|(index, name)| CustomConstructorDescriptor {
                        id: CustomConstructorId {
                            type_id: custom,
                            index,
                        },
                        name: Text::Static(name),
                        native_tag: Text::from(gleam_compiler_core::strings::to_snake_case(name)),
                        fields: Table::Static(&[]),
                    })
                    .collect(),
            }]
            .into(),
        };
        let externals = ExternalTypeTable {
            types: Table::Static(&[]),
        };
        let mut descriptors = vec![
            ValueShapeDescriptor::Custom(CustomValueShapeId(0)),
            ValueShapeDescriptor::Custom(CustomValueShapeId(1)),
            ValueShapeDescriptor::Custom(CustomValueShapeId(2)),
            ValueShapeDescriptor::List(ValueShapeId(0)),
            ValueShapeDescriptor::List(ValueShapeId(1)),
        ];
        let mut nominal = vec![ValueType::Custom(custom); 3];
        nominal.extend([
            ValueType::List(ListTypeId(0)),
            ValueType::List(ListTypeId(0)),
        ]);
        for (argument, return_) in [(0, 0), (1, 0), (0, 1), (1, 1)] {
            descriptors.push(ValueShapeDescriptor::Function {
                arguments: vec![ValueShapeId(argument)].into(),
                return_: ValueShapeId(return_),
            });
            nominal.push(ValueType::Function(FunctionType {
                arguments: vec![ValueType::Custom(custom)].into(),
                return_: Box::new(ValueType::Custom(custom)).into(),
            }));
        }
        for shape in [0, 1] {
            descriptors.push(ValueShapeDescriptor::Tuple(
                vec![ValueShapeId(shape); 2].into(),
            ));
            nominal.push(ValueType::Tuple(vec![ValueType::Custom(custom); 2].into()));
        }
        descriptors.extend([
            ValueShapeDescriptor::Int,
            ValueShapeDescriptor::Bool,
            ValueShapeDescriptor::Tuple(Table::Static(&[])),
            ValueShapeDescriptor::Function {
                arguments: Table::Static(&[]),
                return_: ValueShapeId(0),
            },
        ]);
        nominal.extend([
            ValueType::Int,
            ValueType::Bool,
            ValueType::Tuple(Table::Static(&[])),
            ValueType::Function(FunctionType {
                arguments: Table::Static(&[]),
                return_: Box::new(ValueType::Custom(custom)).into(),
            }),
        ]);
        let shapes = ValueShapeTable {
            shapes: descriptors.into(),
            shape_types: nominal.into(),
            custom_shapes: [
                CustomConstructorRefinement::Any,
                CustomConstructorRefinement::Exact(0),
                CustomConstructorRefinement::Exact(1),
            ]
            .into_iter()
            .map(|constructor| CustomValueShapeDescriptor {
                type_id: custom,
                arguments: Table::Static(&[]),
                constructor,
            })
            .collect(),
        };
        let types = Types::admit(&lists, &customs, &externals, &shapes).unwrap();
        for (source, target, expected) in [(1, 0, true), (0, 1, false), (1, 2, false), (2, 0, true)]
        {
            assert_eq!(
                types.can_flow_custom(
                    CustomValueShape {
                        type_id: custom,
                        shape_id: CustomValueShapeId(source)
                    },
                    CustomValueShape {
                        type_id: custom,
                        shape_id: CustomValueShapeId(target)
                    },
                ),
                Ok(expected)
            );
        }
        let valid = CustomValueShape {
            type_id: custom,
            shape_id: CustomValueShapeId(0),
        };
        let missing = CustomValueShape {
            type_id: custom,
            shape_id: CustomValueShapeId(99),
        };
        for (source, target) in [(missing, valid), (valid, missing)] {
            assert_eq!(
                types.can_flow_custom(source, target),
                Err(TypeError::MissingCustomShape { index: 99 })
            );
        }
        for (source, target, expected) in [
            (1, 0, true),
            (0, 1, false),
            (1, 2, false),
            (2, 0, true),
            (4, 3, true),
            (3, 4, false),
            (5, 6, true),
            (6, 5, false),
            (7, 5, true),
            (5, 7, false),
            (7, 6, true),
            (8, 5, false),
            (10, 9, true),
            (9, 10, false),
            (11, 12, false),
            (11, 11, true),
            (9, 13, false),
            (5, 14, false),
        ] {
            assert_eq!(
                types.can_flow(ValueShapeId(source), ValueShapeId(target)),
                Ok(expected),
                "{source} -> {target}"
            );
        }
        assert_eq!(
            types.equivalent(ValueShapeId(0), ValueShapeId(1)),
            Ok(false)
        );
        assert_eq!(types.equivalent(ValueShapeId(1), ValueShapeId(1)), Ok(true));
        assert_eq!(
            types.equivalent(ValueShapeId(1), ValueShapeId(0)),
            Ok(false)
        );
        assert_eq!(
            types.can_flow(ValueShapeId(99), ValueShapeId(99)),
            Err(TypeError::MissingShape { index: 99 })
        );
        assert_eq!(
            types.can_flow(ValueShapeId(0), ValueShapeId(99)),
            Err(TypeError::MissingShape { index: 99 })
        );
        assert_eq!(
            types.equivalent(ValueShapeId(99), ValueShapeId(0)),
            Err(TypeError::MissingShape { index: 99 })
        );
        assert_eq!(
            types.equivalent(ValueShapeId(0), ValueShapeId(99)),
            Err(TypeError::MissingShape { index: 99 })
        );
    }

    #[test]
    fn equivalent_scalar_rows_and_nominal_external_rows_do_not_depend_on_shape_ids() {
        use crate::plan::{TypeParameterId, execution::type_::ExternalTypeId};
        let lists = ListTypeTable::default();
        let customs = CustomTypeTable::new(Vec::new(), Vec::new());
        let externals = ExternalTypeTable {
            types: vec![
                NominalTypeMetadata {
                    package: "app".into(),
                    module: "native".into(),
                    name: "First".into(),
                    arguments: Table::Static(&[]),
                },
                NominalTypeMetadata {
                    package: "app".into(),
                    module: "native".into(),
                    name: "Second".into(),
                    arguments: Table::Static(&[]),
                },
            ]
            .into(),
        };
        let rows = [
            (ValueShapeDescriptor::Int, ValueType::Int),
            (ValueShapeDescriptor::Float, ValueType::Float),
            (ValueShapeDescriptor::String, ValueType::String),
            (ValueShapeDescriptor::BitArray, ValueType::BitArray),
            (ValueShapeDescriptor::UtfCodepoint, ValueType::UtfCodepoint),
            (ValueShapeDescriptor::Bool, ValueType::Bool),
            (ValueShapeDescriptor::Nil, ValueType::Nil),
            (
                ValueShapeDescriptor::Parameter(TypeParameterId(0)),
                ValueType::Parameter(TypeParameterId(0)),
            ),
            (
                ValueShapeDescriptor::Parameter(TypeParameterId(1)),
                ValueType::Parameter(TypeParameterId(1)),
            ),
            (
                ValueShapeDescriptor::External(ExternalTypeId(0)),
                ValueType::External(ExternalTypeId(0)),
            ),
            (
                ValueShapeDescriptor::External(ExternalTypeId(1)),
                ValueType::External(ExternalTypeId(1)),
            ),
        ];
        let shapes = ValueShapeTable {
            shapes: rows
                .iter()
                .chain(&rows)
                .map(|(shape, _)| shape.clone())
                .collect(),
            shape_types: rows
                .iter()
                .chain(&rows)
                .map(|(_, type_)| type_.clone())
                .collect(),
            custom_shapes: Table::Static(&[]),
        };
        let types = Types::admit(&lists, &customs, &externals, &shapes).unwrap();
        for source in 0..rows.len() {
            for target in 0..rows.len() {
                assert_eq!(
                    types.can_flow(ValueShapeId(source), ValueShapeId(target + rows.len())),
                    Ok(source == target)
                );
                assert_eq!(
                    types.equivalent(ValueShapeId(source), ValueShapeId(target + rows.len())),
                    Ok(source == target)
                );
            }
        }
    }

    #[test]
    fn custom_shape_flow_preserves_nominal_identity_and_refined_generic_arguments() {
        let source = r#"
pub type Choice { First Second }
pub type Box(a) { Box(a) }
pub type Other { Other }
fn widen(value: Choice) { value }
pub fn main() { #(Box(First), Box(widen(First)), Box(Second), Other) }
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
        let custom = |name: &str, constructor, arguments: &[ValueShapeId]| {
            let (index, descriptor) = common
                .value_shapes
                .custom_shapes
                .iter()
                .enumerate()
                .find(|(_, value)| {
                    common.custom_types.types[value.type_id.index()]
                        .type_
                        .name
                        .as_str()
                        == name
                        && value.constructor == constructor
                        && value.arguments.as_ref() == arguments
                })
                .unwrap();
            CustomValueShape {
                type_id: descriptor.type_id,
                shape_id: CustomValueShapeId(index),
            }
        };
        let shape = |value: CustomValueShape| {
            ValueShapeId(
                common
                    .value_shapes
                    .shapes
                    .iter()
                    .position(|shape| shape == &ValueShapeDescriptor::Custom(value.shape_id))
                    .unwrap(),
            )
        };
        let first = custom("Choice", CustomConstructorRefinement::Exact(0), &[]);
        let second = custom("Choice", CustomConstructorRefinement::Exact(1), &[]);
        let any = custom("Choice", CustomConstructorRefinement::Any, &[]);
        let first = custom(
            "Box",
            CustomConstructorRefinement::Exact(0),
            &[shape(first)],
        );
        let second = custom(
            "Box",
            CustomConstructorRefinement::Exact(0),
            &[shape(second)],
        );
        let any = custom("Box", CustomConstructorRefinement::Exact(0), &[shape(any)]);
        let other = custom("Other", CustomConstructorRefinement::Exact(0), &[]);
        for (source, target, expected) in [
            (first, any, true),
            (any, first, false),
            (first, second, false),
            (second, any, true),
            (first, other, false),
        ] {
            assert_eq!(types.can_flow_custom(source, target), Ok(expected));
            assert_eq!(types.can_flow(shape(source), shape(target)), Ok(expected));
        }
    }
}
