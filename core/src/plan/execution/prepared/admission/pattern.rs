mod bits;

use super::local::{LocalError, Locals};
use super::type_::{TypeError, Types};
use crate::plan::execution::graph::{MatchPattern, MatchPatternBinding, MatchPatternListTail};
use crate::plan::execution::type_::custom::FieldRefinement;
use crate::plan::execution::type_::{
    CustomConstructorRefinement, ValueShapeDescriptor, ValueShapeId, ValueType,
};
use std::collections::HashSet;

pub(super) struct Bindings {
    pub(super) values: Vec<BindingValue>,
    ints: HashSet<usize>,
}

#[derive(Clone, Copy)]
pub(super) enum BindingValue {
    Source {
        shape: ValueShapeId,
        constructor: Option<usize>,
    },
    Scalar(&'static ValueType),
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum PatternError {
    Integer(super::literal::IntegerError),
    Type(TypeError),
    Local(LocalError),
    SubjectType,
    FieldCount { expected: usize, found: usize },
    BindingOrder { expected: usize, found: usize },
    IntBinding { index: usize },
    RecursivePattern,
    RecursiveSize,
    ZeroUnit,
}

enum Visit<'data> {
    Enter(&'data MatchPattern, ValueShapeId),
    Leave(*const MatchPattern),
    Bind(&'data MatchPatternBinding, BindingValue),
}

impl Bindings {
    pub(super) fn admit<'data>(
        pattern: &'data MatchPattern,
        subject: ValueShapeId,
        locals: &Locals<'data>,
        types: &Types<'data>,
    ) -> Result<Self, PatternError> {
        types.shape(subject).map_err(PatternError::Type)?;
        let mut bindings = Self {
            values: Vec::new(),
            ints: HashSet::new(),
        };
        let mut active = HashSet::new();
        let mut pending = vec![Visit::Enter(pattern, subject)];
        while let Some(visit) = pending.pop() {
            let (pattern, value) = match visit {
                Visit::Enter(pattern, value) => (pattern, value),
                Visit::Leave(key) => {
                    active.remove(&key);
                    continue;
                }
                Visit::Bind(binding, value) => {
                    bindings.bind(binding, value)?;
                    continue;
                }
            };
            let key = pattern as *const MatchPattern;
            if !active.insert(key) {
                return Err(PatternError::RecursivePattern);
            }
            pending.push(Visit::Leave(key));
            match pattern {
                MatchPattern::Bind(binding) => {
                    bindings.bind(binding, BindingValue::source(value))?;
                }
                MatchPattern::Discard => {}
                MatchPattern::Int(literal) => {
                    super::literal::integer(literal).map_err(PatternError::Integer)?;
                    Self::require(value, &ValueType::Int, types)?;
                }
                MatchPattern::Float(_) => Self::require(value, &ValueType::Float, types)?,
                MatchPattern::String(_) => Self::require(value, &ValueType::String, types)?,
                MatchPattern::Bool(_) => Self::require(value, &ValueType::Bool, types)?,
                MatchPattern::Nil => Self::require(value, &ValueType::Nil, types)?,
                MatchPattern::Tuple(patterns) => {
                    let ValueShapeDescriptor::Tuple(elements) = &types.shapes.shapes[value.index()]
                    else {
                        return Err(PatternError::SubjectType);
                    };
                    if patterns.len() != elements.len() {
                        return Err(PatternError::FieldCount {
                            expected: elements.len(),
                            found: patterns.len(),
                        });
                    }
                    pending.extend(
                        patterns
                            .iter()
                            .zip(elements.iter())
                            .rev()
                            .map(|(pattern, shape)| Visit::Enter(pattern, *shape)),
                    );
                }
                MatchPattern::List(pattern) => {
                    let ValueShapeDescriptor::List(item) = &types.shapes.shapes[value.index()]
                    else {
                        return Err(PatternError::SubjectType);
                    };
                    if let Some(MatchPatternListTail::Bind(binding)) = &pattern.tail {
                        pending.push(Visit::Bind(binding, BindingValue::source(value)));
                    }
                    pending.extend(
                        pattern
                            .elements
                            .iter()
                            .rev()
                            .map(|pattern| Visit::Enter(pattern, *item)),
                    );
                }
                MatchPattern::Custom {
                    constructor,
                    fields,
                } => {
                    let ValueShapeDescriptor::Custom(id) = &types.shapes.shapes[value.index()]
                    else {
                        return Err(PatternError::SubjectType);
                    };
                    let shape = &types.shapes.custom_shapes[id.0];
                    if shape.type_id != constructor.type_id {
                        return Err(PatternError::SubjectType);
                    }
                    let descriptor = types
                        .constructor_descriptor(*constructor)
                        .map_err(PatternError::Type)?;
                    if fields.len() != descriptor.fields.len() {
                        return Err(PatternError::FieldCount {
                            expected: descriptor.fields.len(),
                            found: fields.len(),
                        });
                    }
                    for (pattern, field) in fields.iter().zip(descriptor.fields.iter()).rev() {
                        let source = match &field.refinement {
                            FieldRefinement::Argument(index) => shape.arguments[*index],
                            _ => field.shape,
                        };
                        pending.push(Visit::Enter(pattern, source));
                    }
                }
                MatchPattern::StringPrefix {
                    prefix: _,
                    left,
                    right,
                } => {
                    Self::require(value, &ValueType::String, types)?;
                    if let Some(binding) = left {
                        bindings.bind(binding, BindingValue::Scalar(&ValueType::String))?;
                    }
                    if let Some(binding) = right {
                        bindings.bind(binding, BindingValue::Scalar(&ValueType::String))?;
                    }
                }
                MatchPattern::Alias { pattern, binding } => {
                    let mut constructor = None;
                    let mut inner = pattern.as_ref();
                    let mut aliases = HashSet::new();
                    while let MatchPattern::Alias { pattern, .. } = inner {
                        if !aliases.insert(inner as *const MatchPattern) {
                            return Err(PatternError::RecursivePattern);
                        }
                        inner = pattern;
                    }
                    if let MatchPattern::Custom {
                        constructor: id, ..
                    } = inner
                    {
                        constructor = Some(id.index);
                    }
                    pending.push(Visit::Bind(
                        binding,
                        BindingValue::Source {
                            shape: value,
                            constructor,
                        },
                    ));
                    pending.push(Visit::Enter(pattern, value));
                }
                MatchPattern::BitArray(pattern) => {
                    Self::require(value, &ValueType::BitArray, types)?;
                    bindings.bits(pattern, locals)?;
                }
            }
        }
        Ok(bindings)
    }

    fn require(
        value: ValueShapeId,
        expected: &ValueType,
        types: &Types<'_>,
    ) -> Result<(), PatternError> {
        if &types.shape_types()[value.index()] != expected {
            return Err(PatternError::SubjectType);
        }
        Ok(())
    }

    fn bind(
        &mut self,
        binding: &MatchPatternBinding,
        value: BindingValue,
    ) -> Result<(), PatternError> {
        if binding.index != self.values.len() {
            return Err(PatternError::BindingOrder {
                expected: self.values.len(),
                found: binding.index,
            });
        }
        self.values.push(value);
        Ok(())
    }
}

impl BindingValue {
    pub(super) fn source(shape: ValueShapeId) -> Self {
        Self::Source {
            shape,
            constructor: None,
        }
    }

    pub(super) fn flows_to(self, target: ValueShapeId, types: &Types<'_>) -> bool {
        // Both the binding and its destination come from admitted slots or type tables.
        match self {
            Self::Scalar(type_) => &types.shape_types()[target.index()] == type_,
            Self::Source {
                shape,
                constructor: None,
            } => types.flow(shape, target),
            Self::Source {
                shape,
                constructor: Some(index),
            } => {
                let (ValueShapeDescriptor::Custom(source), ValueShapeDescriptor::Custom(target)) = (
                    &types.shapes.shapes[shape.index()],
                    &types.shapes.shapes[target.index()],
                ) else {
                    return false;
                };
                let source = &types.shapes.custom_shapes[source.0];
                let target = &types.shapes.custom_shapes[target.0];
                if source.type_id != target.type_id
                    || matches!(target.constructor, CustomConstructorRefinement::Exact(target) if target != index)
                {
                    return false;
                }
                for (source, target) in source.arguments.iter().zip(target.arguments.iter()) {
                    if !types.flow(*source, *target) {
                        return false;
                    }
                }
                true
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        BindingValue, Bindings, Locals, MatchPattern, MatchPatternBinding, PatternError, Types,
        ValueShapeDescriptor, ValueShapeId, ValueType,
    };
    use crate::plan::execution::graph::{
        BitArrayBindingPattern, BitArrayPattern, BitArrayPatternSegment, IntegerLiteral,
        MatchPatternList, MatchPatternListTail,
    };
    use crate::plan::execution::storage::{Node, Table};

    #[test]
    fn scalar_and_compound_patterns_validate_their_subject_and_binding_order() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub fn main() { #(42, 1.5, "text", True, Nil, <<42>>, #(42, True), [42]) }
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
        let shape = |type_: &ValueType| {
            ValueShapeId(
                types
                    .shape_types()
                    .iter()
                    .position(|value| value == type_)
                    .unwrap(),
            )
        };
        let int = shape(&ValueType::Int);
        let boolean = shape(&ValueType::Bool);
        let string = shape(&ValueType::String);
        let tuple = ValueShapeId(common.value_shapes.shapes.iter().position(|value| matches!(value, ValueShapeDescriptor::Tuple(fields) if fields.len() == 2)).unwrap());
        let list = ValueShapeId(
            common
                .value_shapes
                .shapes
                .iter()
                .position(|value| matches!(value, ValueShapeDescriptor::List(_)))
                .unwrap(),
        );
        let bits = shape(&ValueType::BitArray);
        let locals = Locals::default();
        for (pattern, subject) in [
            (
                MatchPattern::Int(IntegerLiteral::from(num_bigint::BigInt::from(42))),
                int,
            ),
            (MatchPattern::Float(1.5), shape(&ValueType::Float)),
            (MatchPattern::String("text".into()), string),
            (MatchPattern::Bool(true), boolean),
            (MatchPattern::Nil, shape(&ValueType::Nil)),
            (
                MatchPattern::Tuple(vec![MatchPattern::Discard, MatchPattern::Discard].into()),
                tuple,
            ),
            (
                MatchPattern::List(MatchPatternList {
                    elements: Table::Static(&[]),
                    tail: None,
                }),
                list,
            ),
            (
                MatchPattern::StringPrefix {
                    prefix: "pre".into(),
                    left: None,
                    right: None,
                },
                string,
            ),
            (
                MatchPattern::BitArray(BitArrayPattern {
                    segments: Table::Static(&[]),
                }),
                bits,
            ),
        ] {
            assert_eq!(
                Bindings::admit(&pattern, subject, &locals, &types)
                    .unwrap()
                    .values
                    .len(),
                0
            );
            let wrong = if subject == int { boolean } else { int };
            assert_eq!(
                Bindings::admit(&pattern, wrong, &locals, &types).err(),
                Some(PatternError::SubjectType)
            );
            assert_eq!(
                Bindings::admit(&pattern, ValueShapeId(99_999), &locals, &types).err(),
                Some(PatternError::Type(
                    super::super::type_::TypeError::MissingShape { index: 99_999 }
                ))
            );
        }
        let wrong_count = MatchPattern::Tuple(vec![MatchPattern::Discard].into());
        assert_eq!(
            Bindings::admit(&wrong_count, tuple, &locals, &types).err(),
            Some(PatternError::FieldCount {
                expected: 2,
                found: 1
            })
        );
        let invalid_integer = MatchPattern::Int(IntegerLiteral {
            sign: num_bigint::Sign::Plus,
            digits: Table::Static(&[]),
        });
        assert_eq!(
            Bindings::admit(&invalid_integer, int, &locals, &types).err(),
            Some(PatternError::Integer(
                super::super::literal::IntegerError::EmptyMagnitude
            ))
        );

        let pattern = MatchPattern::List(MatchPatternList {
            elements: vec![MatchPattern::Bind(MatchPatternBinding { index: 0 })].into(),
            tail: Some(MatchPatternListTail::Bind(MatchPatternBinding { index: 1 })),
        });
        let values = Bindings::admit(&pattern, list, &locals, &types)
            .unwrap()
            .values;
        assert_eq!(values.len(), 2);
        assert!(values[0].flows_to(int, &types));
        assert!(values[1].flows_to(list, &types));
        assert!(!values[0].flows_to(boolean, &types));
        for pattern in [
            MatchPattern::Bind(MatchPatternBinding { index: 1 }),
            MatchPattern::Alias {
                pattern: Node::Static(&MatchPattern::Discard),
                binding: MatchPatternBinding { index: 1 },
            },
            MatchPattern::List(MatchPatternList {
                elements: Table::Static(&[]),
                tail: Some(MatchPatternListTail::Bind(MatchPatternBinding { index: 1 })),
            }),
        ] {
            assert_eq!(
                Bindings::admit(&pattern, list, &locals, &types).err(),
                Some(PatternError::BindingOrder {
                    expected: 0,
                    found: 1
                })
            );
        }
        for (left, right, expected) in [
            (Some(0), Some(1), None),
            (Some(1), None, Some(1)),
            (None, Some(1), Some(1)),
        ] {
            let pattern = MatchPattern::StringPrefix {
                prefix: "pre".into(),
                left: left.map(|index| MatchPatternBinding { index }),
                right: right.map(|index| MatchPatternBinding { index }),
            };
            let result = Bindings::admit(&pattern, string, &locals, &types);
            if let Some(found) = expected {
                assert_eq!(
                    result.err(),
                    Some(PatternError::BindingOrder { expected: 0, found })
                );
            } else {
                let values = result.unwrap().values;
                assert_eq!(values.len(), 2);
                for value in values {
                    assert!(value.flows_to(string, &types));
                    assert!(!value.flows_to(int, &types));
                }
            }
        }
        let invalid_bits = MatchPattern::BitArray(BitArrayPattern {
            segments: vec![BitArrayPatternSegment::Bits {
                pattern: BitArrayBindingPattern::Discard,
                size: None,
                unit: 0,
            }]
            .into(),
        });
        assert_eq!(
            Bindings::admit(&invalid_bits, bits, &locals, &types).err(),
            Some(PatternError::ZeroUnit)
        );
    }

    #[test]
    fn shared_pattern_nodes_preserve_each_binding_but_cycles_are_rejected() {
        let typed =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { [42] }")
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
        let list = ValueShapeId(
            common
                .value_shapes
                .shapes
                .iter()
                .position(|value| matches!(value, ValueShapeDescriptor::List(_)))
                .unwrap(),
        );
        static DISCARD: MatchPattern = MatchPattern::Discard;
        let shared = MatchPattern::List(MatchPatternList {
            elements: vec![
                MatchPattern::Alias {
                    pattern: Node::Static(&DISCARD),
                    binding: MatchPatternBinding { index: 0 },
                },
                MatchPattern::Alias {
                    pattern: Node::Static(&DISCARD),
                    binding: MatchPatternBinding { index: 1 },
                },
            ]
            .into(),
            tail: None,
        });
        let locals = Locals::default();
        let values = Bindings::admit(&shared, list, &locals, &types)
            .unwrap()
            .values;
        assert_eq!(values.len(), 2);
        let int = ValueShapeId(
            types
                .shape_types()
                .iter()
                .position(|value| value == &ValueType::Int)
                .unwrap(),
        );
        assert!(values[0].flows_to(int, &types));
        assert!(values[1].flows_to(int, &types));
        static ALIAS_CYCLE: MatchPattern = MatchPattern::Alias {
            pattern: Node::Static(&ALIAS_CYCLE),
            binding: MatchPatternBinding { index: 0 },
        };
        static LIST_CYCLE: MatchPattern = MatchPattern::List(MatchPatternList {
            elements: Table::Static(&[MatchPattern::Alias {
                pattern: Node::Static(&LIST_CYCLE),
                binding: MatchPatternBinding { index: 0 },
            }]),
            tail: None,
        });
        for pattern in [&ALIAS_CYCLE, &LIST_CYCLE] {
            assert_eq!(
                Bindings::admit(pattern, list, &locals, &types).err(),
                Some(PatternError::RecursivePattern)
            );
        }
    }

    #[test]
    fn refined_aliases_preserve_the_constructor_and_nested_generic_argument() {
        use crate::plan::execution::type_::{CustomConstructorId, CustomConstructorRefinement};

        let source = r#"
pub type Choice { First Second }
pub type Box(a) { Box(a) }
pub fn main() { #(First, Second, Box(First), Box(Second)) }
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
        let body = plan.program.functions.value_returns.tuple_functions[0].body();
        let tuple = types
            .tuple_slot(&body.block_graph().instructions.last().unwrap().output)
            .unwrap();
        let elements = tuple.elements;
        let customs = common
            .value_shapes
            .shapes
            .iter()
            .enumerate()
            .filter_map(|(index, shape)| match shape {
                ValueShapeDescriptor::Custom(custom) => Some((
                    ValueShapeId(index),
                    &common.value_shapes.custom_shapes[custom.0],
                )),
                _ => None,
            })
            .collect::<std::collections::HashMap<_, _>>();
        assert_eq!(elements.len(), 4);
        for (source, target) in [(elements[0], elements[1]), (elements[2], elements[3])] {
            let custom = customs[&source];
            assert_eq!(custom.constructor, CustomConstructorRefinement::Exact(0));
            let fields = types
                .constructor_descriptor(CustomConstructorId {
                    type_id: custom.type_id,
                    index: 0,
                })
                .unwrap()
                .fields
                .iter()
                .map(|_| MatchPattern::Discard)
                .collect::<Vec<_>>();
            let pattern = MatchPattern::Alias {
                pattern: Node::Owned(Box::new(MatchPattern::Custom {
                    constructor: CustomConstructorId {
                        type_id: custom.type_id,
                        index: 0,
                    },
                    fields: fields.into(),
                })),
                binding: MatchPatternBinding { index: 0 },
            };
            let bindings = Bindings::admit(&pattern, source, &Locals::default(), &types).unwrap();
            assert!(bindings.values[0].flows_to(source, &types));
            assert!(!bindings.values[0].flows_to(target, &types));
        }
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new())
                .unwrap()
                .inspect()
                .to_string(),
            "#(First, Second, Box(First), Box(Second))"
        );
    }

    #[test]
    fn nominal_patterns_retain_generic_fields_and_refined_aliases() {
        use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            r#"
pub type Box(a) { Box(a) }
pub type Other { Other(Int) }
pub fn main() { #(Box(42), Box("text"), Other(42)) }
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
        let int = ValueShapeId(
            types
                .shape_types()
                .iter()
                .position(|value| value == &ValueType::Int)
                .unwrap(),
        );
        let custom = CustomTypeId(
            common
                .custom_types
                .types
                .iter()
                .position(|value| {
                    value.type_.name.as_str() == "Box"
                        && value.type_.arguments.as_ref()
                            == [crate::plan::execution::type_::TypeMetadata::Int]
                })
                .unwrap(),
        );
        let shape = ValueShapeId(common.value_shapes.shapes.iter().position(|value| matches!(value, ValueShapeDescriptor::Custom(id) if types.custom_shape_descriptor(*id).unwrap().type_id == custom)).unwrap());
        let constructor = CustomConstructorId {
            type_id: custom,
            index: 0,
        };
        let pattern = MatchPattern::Alias {
            pattern: Node::Owned(Box::new(MatchPattern::Alias {
                pattern: Node::Owned(Box::new(MatchPattern::Custom {
                    constructor,
                    fields: vec![MatchPattern::Bind(MatchPatternBinding { index: 0 })].into(),
                })),
                binding: MatchPatternBinding { index: 1 },
            })),
            binding: MatchPatternBinding { index: 2 },
        };
        let locals = Locals::default();
        let values = Bindings::admit(&pattern, shape, &locals, &types)
            .unwrap()
            .values;
        assert_eq!(values.len(), 3);
        assert!(values[0].flows_to(int, &types));
        for value in &values[1..] {
            assert!(value.flows_to(shape, &types));
            assert!(!value.flows_to(int, &types));
            for (index, other) in common.value_shapes.shape_types.iter().enumerate() {
                if let ValueType::Custom(other) = other {
                    assert_eq!(
                        value.flows_to(ValueShapeId(index), &types),
                        *other == custom
                    );
                }
            }
        }
        for (pattern, subject, error) in [
            (
                MatchPattern::Custom {
                    constructor,
                    fields: Table::Static(&[]),
                },
                ValueShapeId(99_999),
                PatternError::Type(super::super::type_::TypeError::MissingShape { index: 99_999 }),
            ),
            (
                MatchPattern::Custom {
                    constructor,
                    fields: Table::Static(&[]),
                },
                shape,
                PatternError::FieldCount {
                    expected: 1,
                    found: 0,
                },
            ),
            (
                MatchPattern::Custom {
                    constructor,
                    fields: Table::Static(&[]),
                },
                int,
                PatternError::SubjectType,
            ),
            (
                MatchPattern::Custom {
                    constructor: CustomConstructorId {
                        type_id: CustomTypeId(99),
                        index: 0,
                    },
                    fields: Table::Static(&[]),
                },
                shape,
                PatternError::SubjectType,
            ),
            (
                MatchPattern::Custom {
                    constructor: CustomConstructorId {
                        index: 1,
                        ..constructor
                    },
                    fields: Table::Static(&[]),
                },
                shape,
                PatternError::Type(super::super::type_::TypeError::MissingConstructor {
                    type_index: custom.index(),
                    index: 1,
                }),
            ),
        ] {
            assert_eq!(
                Bindings::admit(&pattern, subject, &locals, &types).err(),
                Some(error)
            );
        }
        assert!(!BindingValue::source(int).flows_to(shape, &types));
        assert_eq!(
            Bindings::admit(
                &MatchPattern::Bind(MatchPatternBinding { index: 0 }),
                ValueShapeId(99_999),
                &locals,
                &types
            )
            .err(),
            Some(PatternError::Type(
                super::super::type_::TypeError::MissingShape { index: 99_999 }
            ))
        );
    }
}
