use super::super::draft::pattern::{
    DraftBitArrayBindingPattern, DraftBitArrayPattern, DraftBitArrayPatternSegment,
    DraftBitArrayPatternSize, DraftBitArrayPatternSizeExpr, DraftBitArrayPatternValue,
    DraftBitArrayStringPattern, DraftMatchListTail, DraftMatchPattern, DraftMatchPatternBinding,
};
use super::value::BlockValues;
use crate::plan::execution;

pub(super) fn freeze(
    pattern: DraftMatchPattern,
    values: &BlockValues,
) -> execution::graph::MatchPattern {
    use execution::graph::MatchPattern as E;

    match pattern {
        DraftMatchPattern::Bind(binding) => E::Bind(freeze_binding(binding)),
        DraftMatchPattern::Discard => E::Discard,
        DraftMatchPattern::Int(value) => E::Int(value),
        DraftMatchPattern::Float(value) => E::Float(value),
        DraftMatchPattern::String(value) => E::String(value),
        DraftMatchPattern::Bool(value) => E::Bool(value),
        DraftMatchPattern::Nil => E::Nil,
        DraftMatchPattern::Tuple(elements) => E::Tuple(
            elements
                .into_iter()
                .map(|element| freeze(element, values))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        ),
        DraftMatchPattern::List { elements, tail } => {
            E::List(execution::graph::MatchPatternList::new(
                elements
                    .into_iter()
                    .map(|element| freeze(element, values))
                    .collect(),
                tail.map(freeze_list_tail),
            ))
        }
        DraftMatchPattern::BitArray(pattern) => E::BitArray(freeze_bit_array(pattern, values)),
        DraftMatchPattern::Custom {
            constructor,
            fields,
        } => E::Custom {
            constructor,
            fields: fields
                .into_iter()
                .map(|field| freeze(field, values))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        },
        DraftMatchPattern::StringPrefix {
            prefix,
            left,
            right,
        } => E::StringPrefix {
            prefix,
            left: left.map(freeze_binding),
            right: right.map(freeze_binding),
        },
        DraftMatchPattern::Alias { pattern, binding } => E::Alias {
            pattern: Box::new(freeze(*pattern, values)),
            binding: freeze_binding(binding),
        },
    }
}

fn freeze_binding(binding: DraftMatchPatternBinding) -> execution::graph::MatchPatternBinding {
    execution::graph::MatchPatternBinding::new(binding.index)
}

fn freeze_list_tail(tail: DraftMatchListTail) -> execution::graph::MatchPatternListTail {
    match tail {
        DraftMatchListTail::Ignore => execution::graph::MatchPatternListTail::Ignore,
        DraftMatchListTail::Bind(binding) => {
            execution::graph::MatchPatternListTail::Bind(freeze_binding(binding))
        }
    }
}

fn freeze_bit_array(
    pattern: DraftBitArrayPattern,
    values: &BlockValues,
) -> execution::graph::BitArrayPattern {
    execution::graph::BitArrayPattern::new(
        pattern
            .segments
            .into_iter()
            .map(|segment| freeze_bit_array_segment(segment, values))
            .collect(),
    )
}

fn freeze_bit_array_segment(
    segment: DraftBitArrayPatternSegment,
    values: &BlockValues,
) -> execution::graph::BitArrayPatternSegment {
    use execution::graph::BitArrayPatternSegment as E;

    match segment {
        DraftBitArrayPatternSegment::Int {
            pattern,
            size,
            endianness,
            signedness,
        } => E::Int {
            pattern: freeze_bit_array_value(pattern),
            size: freeze_bit_array_size(size, values),
            endianness,
            signedness,
        },
        DraftBitArrayPatternSegment::Float {
            pattern,
            size,
            endianness,
        } => E::Float {
            pattern: freeze_bit_array_value(pattern),
            size: freeze_bit_array_size(size, values),
            endianness,
        },
        DraftBitArrayPatternSegment::Bits {
            pattern,
            size,
            unit,
        } => E::Bits {
            pattern: freeze_bit_array_binding(pattern),
            size: size.map(|size| freeze_bit_array_size(size, values)),
            unit,
        },
        DraftBitArrayPatternSegment::String { pattern, encoding } => E::String {
            pattern: match pattern {
                DraftBitArrayStringPattern::Literal(value) => {
                    execution::graph::BitArrayStringPattern::Literal(value)
                }
                DraftBitArrayStringPattern::Discard => {
                    execution::graph::BitArrayStringPattern::Discard
                }
            },
            encoding,
        },
        DraftBitArrayPatternSegment::UtfCodepoint { pattern, encoding } => E::UtfCodepoint {
            pattern: freeze_bit_array_binding(pattern),
            encoding,
        },
    }
}

fn freeze_bit_array_size(
    size: DraftBitArrayPatternSize,
    values: &BlockValues,
) -> execution::graph::BitArrayPatternSize {
    execution::graph::BitArrayPatternSize::new(
        freeze_bit_array_size_expr(size.value, values),
        size.unit,
    )
}

fn freeze_bit_array_size_expr(
    expression: DraftBitArrayPatternSizeExpr,
    values: &BlockValues,
) -> execution::graph::BitArrayPatternSizeExpr {
    use execution::graph::BitArrayPatternSizeExpr as E;

    match expression {
        DraftBitArrayPatternSizeExpr::Value(value) => E::Value(value),
        DraftBitArrayPatternSizeExpr::Local(value) => E::Local(values.int(&value)),
        DraftBitArrayPatternSizeExpr::Binding(index) => {
            E::Binding(execution::graph::MatchIntBindingId::new(index))
        }
        DraftBitArrayPatternSizeExpr::Add { left, right } => E::Add {
            left: Box::new(freeze_bit_array_size_expr(*left, values)),
            right: Box::new(freeze_bit_array_size_expr(*right, values)),
        },
        DraftBitArrayPatternSizeExpr::Subtract { left, right } => E::Subtract {
            left: Box::new(freeze_bit_array_size_expr(*left, values)),
            right: Box::new(freeze_bit_array_size_expr(*right, values)),
        },
        DraftBitArrayPatternSizeExpr::Multiply { left, right } => E::Multiply {
            left: Box::new(freeze_bit_array_size_expr(*left, values)),
            right: Box::new(freeze_bit_array_size_expr(*right, values)),
        },
        DraftBitArrayPatternSizeExpr::Divide { left, right } => E::Divide {
            left: Box::new(freeze_bit_array_size_expr(*left, values)),
            right: Box::new(freeze_bit_array_size_expr(*right, values)),
        },
        DraftBitArrayPatternSizeExpr::Remainder { left, right } => E::Remainder {
            left: Box::new(freeze_bit_array_size_expr(*left, values)),
            right: Box::new(freeze_bit_array_size_expr(*right, values)),
        },
    }
}

fn freeze_bit_array_value<Value>(
    pattern: DraftBitArrayPatternValue<Value>,
) -> execution::graph::BitArrayPatternValue<Value> {
    match pattern {
        DraftBitArrayPatternValue::Literal(value) => {
            execution::graph::BitArrayPatternValue::Literal(value)
        }
        DraftBitArrayPatternValue::Bind(binding) => {
            execution::graph::BitArrayPatternValue::Bind(freeze_binding(binding))
        }
        DraftBitArrayPatternValue::Discard => execution::graph::BitArrayPatternValue::Discard,
        DraftBitArrayPatternValue::Alias { pattern, binding } => {
            execution::graph::BitArrayPatternValue::Alias {
                pattern: Box::new(freeze_bit_array_value(*pattern)),
                binding: freeze_binding(binding),
            }
        }
    }
}

fn freeze_bit_array_binding(
    pattern: DraftBitArrayBindingPattern,
) -> execution::graph::BitArrayBindingPattern {
    match pattern {
        DraftBitArrayBindingPattern::Bind(binding) => {
            execution::graph::BitArrayBindingPattern::Bind(freeze_binding(binding))
        }
        DraftBitArrayBindingPattern::Discard => execution::graph::BitArrayBindingPattern::Discard,
        DraftBitArrayBindingPattern::Alias { pattern, binding } => {
            execution::graph::BitArrayBindingPattern::Alias {
                pattern: Box::new(freeze_bit_array_binding(*pattern)),
                binding: freeze_binding(binding),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::draft::pattern as draft_pattern;
    use super::super::super::draft::{DraftGraphBuilder, DraftInt};
    use super::super::value::BlockValues;
    use super::freeze;
    use crate::plan::execution::ExecutionPlan;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph as execution_graph;
    use crate::plan::execution::graph::{
        BlockGraphExitId, IntInstruction, IntLocalId, ListLocal, MatchEdge, MatchEdgeArgument,
        MatchPattern, MatchPatternList, ParamLocal, ProfiledInstruction, ProfiledInstructionKind,
        Terminator,
    };
    use crate::plan::execution::lowering::specialization::StoredValueShape;
    use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId};
    use std::convert::Infallible;

    type Instruction = ProfiledInstruction<Infallible>;
    type InstructionKind = ProfiledInstructionKind<Infallible>;

    #[test]
    fn freezes_every_recursive_pattern_variant_with_exact_metadata() {
        use draft_pattern::DraftMatchPattern as D;

        let (mut draft, _) = DraftGraphBuilder::<DraftInt, usize>::new(Vec::new(), Vec::new());
        let binding_value = draft.value_ref(StoredValueShape::Int);
        let local_value = draft.value_ref(StoredValueShape::Int);
        let local = DraftInt::from_ref(&local_value);
        let mut values = BlockValues::default();
        values.allocate(
            &local_value,
            &mut crate::plan::execution::lowering::test_support::lowering_context(Vec::new()),
        );
        let constructor = CustomConstructorId::new(CustomTypeId::new(3), 4);
        let expected = recursive_pattern(&binding_value, local.clone(), constructor);
        let actual = freeze(
            recursive_pattern(&binding_value, local, constructor),
            &values,
        );
        assert!(pattern_matches(&actual, &expected, &values));
        assert!(!pattern_matches(
            &execution_graph::MatchPattern::Discard,
            &D::Nil,
            &values,
        ));
        assert!(list_tail_matches(None, None));
        assert!(!list_tail_matches(
            None,
            Some(&draft_pattern::DraftMatchListTail::Ignore),
        ));
        let actual_binding = execution_graph::MatchPatternBinding::new(0);
        assert!(!optional_binding_matches(Some(&actual_binding), None));

        let actual_bits = execution_graph::BitArrayPatternSegment::Bits {
            pattern: execution_graph::BitArrayBindingPattern::Discard,
            size: None,
            unit: 1,
        };
        let expected_bits = draft_pattern::DraftBitArrayPatternSegment::Bits {
            pattern: draft_pattern::DraftBitArrayBindingPattern::Discard,
            size: Some(draft_pattern::DraftBitArrayPatternSize {
                value: draft_pattern::DraftBitArrayPatternSizeExpr::Value(1.into()),
                unit: 1,
            }),
            unit: 1,
        };
        assert!(!bit_array_segment_matches(
            &actual_bits,
            &expected_bits,
            &values,
        ));
        assert!(!bit_array_segment_matches(
            &execution_graph::BitArrayPatternSegment::String {
                pattern: execution_graph::BitArrayStringPattern::Discard,
                encoding: execution_graph::StringEncoding::Utf8,
            },
            &draft_pattern::DraftBitArrayPatternSegment::UtfCodepoint {
                pattern: draft_pattern::DraftBitArrayBindingPattern::Discard,
                encoding: execution_graph::StringEncoding::Utf8,
            },
            &values,
        ));
        assert!(!bit_array_value_matches(
            &execution_graph::BitArrayPatternValue::<num_bigint::BigInt>::Discard,
            &draft_pattern::DraftBitArrayPatternValue::Literal(0.into()),
        ));
        assert!(!bit_array_binding_matches(
            &execution_graph::BitArrayBindingPattern::Discard,
            &draft_pattern::DraftBitArrayBindingPattern::Bind(binding(&binding_value, 20)),
        ));
        assert!(!bit_array_string_matches(
            &execution_graph::BitArrayStringPattern::Discard,
            &draft_pattern::DraftBitArrayStringPattern::Literal("mismatch".into()),
        ));
        assert!(!bit_array_size_expr_matches(
            &execution_graph::BitArrayPatternSizeExpr::Value(1.into()),
            &draft_pattern::DraftBitArrayPatternSizeExpr::Binding(0),
            &values,
        ));
    }

    fn recursive_pattern(
        binding_value: &super::super::super::draft::DraftValueRef,
        local: DraftInt,
        constructor: CustomConstructorId,
    ) -> draft_pattern::DraftMatchPattern {
        use draft_pattern::DraftMatchPattern as D;

        D::Tuple(vec![
            D::Bind(binding(binding_value, 0)),
            D::Discard,
            D::Int(1.into()),
            D::Float(2.5),
            D::String("text".into()),
            D::Bool(true),
            D::Nil,
            D::Tuple(vec![D::Int(2.into())]),
            D::List {
                elements: vec![D::Bind(binding(binding_value, 1))],
                tail: Some(draft_pattern::DraftMatchListTail::Bind(binding(
                    binding_value,
                    2,
                ))),
            },
            D::List {
                elements: Vec::new(),
                tail: Some(draft_pattern::DraftMatchListTail::Ignore),
            },
            D::BitArray(bit_array_pattern(binding_value, local)),
            D::Custom {
                constructor,
                fields: vec![D::Discard, D::Bind(binding(binding_value, 3))],
            },
            D::StringPrefix {
                prefix: "pre".into(),
                left: Some(binding(binding_value, 4)),
                right: None,
            },
            D::Alias {
                pattern: Box::new(D::Bool(false)),
                binding: binding(binding_value, 5),
            },
        ])
    }

    fn binding(
        value: &super::super::super::draft::DraftValueRef,
        index: usize,
    ) -> draft_pattern::DraftMatchPatternBinding {
        draft_pattern::DraftMatchPatternBinding {
            value: value.clone(),
            index,
        }
    }

    fn bit_array_pattern(
        binding_value: &super::super::super::draft::DraftValueRef,
        local: DraftInt,
    ) -> draft_pattern::DraftBitArrayPattern {
        use draft_pattern::{
            DraftBitArrayBindingPattern as B, DraftBitArrayPatternSegment as S,
            DraftBitArrayPatternValue as V, DraftBitArrayStringPattern as T,
        };

        let value_size = || draft_pattern::DraftBitArrayPatternSize {
            value: draft_pattern::DraftBitArrayPatternSizeExpr::Value(8.into()),
            unit: 1,
        };
        let recursive_size = draft_pattern::DraftBitArrayPatternSize {
            value: draft_pattern::DraftBitArrayPatternSizeExpr::Add {
                left: Box::new(draft_pattern::DraftBitArrayPatternSizeExpr::Subtract {
                    left: Box::new(draft_pattern::DraftBitArrayPatternSizeExpr::Local(local)),
                    right: Box::new(draft_pattern::DraftBitArrayPatternSizeExpr::Binding(6)),
                }),
                right: Box::new(draft_pattern::DraftBitArrayPatternSizeExpr::Multiply {
                    left: Box::new(draft_pattern::DraftBitArrayPatternSizeExpr::Value(3.into())),
                    right: Box::new(draft_pattern::DraftBitArrayPatternSizeExpr::Divide {
                        left: Box::new(draft_pattern::DraftBitArrayPatternSizeExpr::Value(
                            10.into(),
                        )),
                        right: Box::new(draft_pattern::DraftBitArrayPatternSizeExpr::Remainder {
                            left: Box::new(draft_pattern::DraftBitArrayPatternSizeExpr::Value(
                                5.into(),
                            )),
                            right: Box::new(draft_pattern::DraftBitArrayPatternSizeExpr::Value(
                                2.into(),
                            )),
                        }),
                    }),
                }),
            },
            unit: 2,
        };

        draft_pattern::DraftBitArrayPattern {
            segments: vec![
                S::Int {
                    pattern: V::Literal(1.into()),
                    size: recursive_size,
                    endianness: execution_graph::Endianness::Big,
                    signedness: execution_graph::Signedness::Signed,
                },
                S::Int {
                    pattern: V::Bind(binding(binding_value, 7)),
                    size: value_size(),
                    endianness: execution_graph::Endianness::Little,
                    signedness: execution_graph::Signedness::Unsigned,
                },
                S::Int {
                    pattern: V::Discard,
                    size: value_size(),
                    endianness: execution_graph::Endianness::Big,
                    signedness: execution_graph::Signedness::Unsigned,
                },
                S::Int {
                    pattern: V::Alias {
                        pattern: Box::new(V::Literal(4.into())),
                        binding: binding(binding_value, 8),
                    },
                    size: value_size(),
                    endianness: execution_graph::Endianness::Little,
                    signedness: execution_graph::Signedness::Signed,
                },
                S::Float {
                    pattern: V::Literal(1.5),
                    size: value_size(),
                    endianness: execution_graph::Endianness::Big,
                },
                S::Float {
                    pattern: V::Bind(binding(binding_value, 9)),
                    size: value_size(),
                    endianness: execution_graph::Endianness::Little,
                },
                S::Float {
                    pattern: V::Discard,
                    size: value_size(),
                    endianness: execution_graph::Endianness::Big,
                },
                S::Float {
                    pattern: V::Alias {
                        pattern: Box::new(V::Literal(2.5)),
                        binding: binding(binding_value, 10),
                    },
                    size: value_size(),
                    endianness: execution_graph::Endianness::Little,
                },
                S::Bits {
                    pattern: B::Bind(binding(binding_value, 11)),
                    size: Some(value_size()),
                    unit: 2,
                },
                S::Bits {
                    pattern: B::Discard,
                    size: None,
                    unit: 3,
                },
                S::Bits {
                    pattern: B::Alias {
                        pattern: Box::new(B::Bind(binding(binding_value, 12))),
                        binding: binding(binding_value, 13),
                    },
                    size: Some(value_size()),
                    unit: 4,
                },
                S::String {
                    pattern: T::Literal("value".into()),
                    encoding: execution_graph::StringEncoding::Utf8,
                },
                S::String {
                    pattern: T::Discard,
                    encoding: execution_graph::StringEncoding::Utf16(
                        execution_graph::Endianness::Little,
                    ),
                },
                S::UtfCodepoint {
                    pattern: B::Bind(binding(binding_value, 14)),
                    encoding: execution_graph::StringEncoding::Utf32(
                        execution_graph::Endianness::Big,
                    ),
                },
            ],
        }
    }

    fn pattern_matches(
        actual: &execution_graph::MatchPattern,
        expected: &draft_pattern::DraftMatchPattern,
        values: &BlockValues,
    ) -> bool {
        use draft_pattern::DraftMatchPattern as D;
        use execution_graph::MatchPattern as E;

        match (actual, expected) {
            (E::Bind(actual), D::Bind(expected)) => actual.index() == expected.index,
            (E::Discard, D::Discard) => true,
            (E::Int(actual), D::Int(expected)) => actual == expected,
            (E::Float(actual), D::Float(expected)) => actual == expected,
            (E::String(actual), D::String(expected)) => actual == expected,
            (E::Bool(actual), D::Bool(expected)) => actual == expected,
            (E::Nil, D::Nil) => true,
            (E::Tuple(actual), D::Tuple(expected)) => {
                actual.len() == expected.len()
                    && actual
                        .iter()
                        .zip(expected)
                        .all(|(actual, expected)| pattern_matches(actual, expected, values))
            }
            (E::List(actual), D::List { elements, tail }) => {
                actual.elements().len() == elements.len()
                    && actual
                        .elements()
                        .iter()
                        .zip(elements)
                        .all(|(actual, expected)| pattern_matches(actual, expected, values))
                    && list_tail_matches(actual.tail(), tail.as_ref())
            }
            (E::BitArray(actual), D::BitArray(expected)) => {
                actual.segments().len() == expected.segments.len()
                    && actual
                        .segments()
                        .iter()
                        .zip(&expected.segments)
                        .all(|(actual, expected)| {
                            bit_array_segment_matches(actual, expected, values)
                        })
            }
            (
                E::Custom {
                    constructor: actual_constructor,
                    fields: actual_fields,
                },
                D::Custom {
                    constructor: expected_constructor,
                    fields: expected_fields,
                },
            ) => {
                actual_constructor == expected_constructor
                    && actual_fields.len() == expected_fields.len()
                    && actual_fields
                        .iter()
                        .zip(expected_fields)
                        .all(|(actual, expected)| pattern_matches(actual, expected, values))
            }
            (
                E::StringPrefix {
                    prefix: actual_prefix,
                    left: actual_left,
                    right: actual_right,
                },
                D::StringPrefix {
                    prefix: expected_prefix,
                    left: expected_left,
                    right: expected_right,
                },
            ) => {
                actual_prefix == expected_prefix
                    && optional_binding_matches(actual_left.as_ref(), expected_left.as_ref())
                    && optional_binding_matches(actual_right.as_ref(), expected_right.as_ref())
            }
            (
                E::Alias {
                    pattern: actual_pattern,
                    binding: actual_binding,
                },
                D::Alias {
                    pattern: expected_pattern,
                    binding: expected_binding,
                },
            ) => {
                actual_binding.index() == expected_binding.index
                    && pattern_matches(actual_pattern, expected_pattern, values)
            }
            _ => false,
        }
    }

    fn list_tail_matches(
        actual: Option<&execution_graph::MatchPatternListTail>,
        expected: Option<&draft_pattern::DraftMatchListTail>,
    ) -> bool {
        match (actual, expected) {
            (None, None) => true,
            (
                Some(execution_graph::MatchPatternListTail::Ignore),
                Some(draft_pattern::DraftMatchListTail::Ignore),
            ) => true,
            (
                Some(execution_graph::MatchPatternListTail::Bind(actual)),
                Some(draft_pattern::DraftMatchListTail::Bind(expected)),
            ) => actual.index() == expected.index,
            _ => false,
        }
    }

    fn optional_binding_matches(
        actual: Option<&execution_graph::MatchPatternBinding>,
        expected: Option<&draft_pattern::DraftMatchPatternBinding>,
    ) -> bool {
        match (actual, expected) {
            (None, None) => true,
            (Some(actual), Some(expected)) => actual.index() == expected.index,
            _ => false,
        }
    }

    fn bit_array_segment_matches(
        actual: &execution_graph::BitArrayPatternSegment,
        expected: &draft_pattern::DraftBitArrayPatternSegment,
        values: &BlockValues,
    ) -> bool {
        use draft_pattern::DraftBitArrayPatternSegment as D;
        use execution_graph::BitArrayPatternSegment as E;

        match (actual, expected) {
            (
                E::Int {
                    pattern: actual_pattern,
                    size: actual_size,
                    endianness: actual_endianness,
                    signedness: actual_signedness,
                },
                D::Int {
                    pattern: expected_pattern,
                    size: expected_size,
                    endianness: expected_endianness,
                    signedness: expected_signedness,
                },
            ) => {
                bit_array_value_matches(actual_pattern, expected_pattern)
                    && bit_array_size_matches(actual_size, expected_size, values)
                    && actual_endianness == expected_endianness
                    && actual_signedness == expected_signedness
            }
            (
                E::Float {
                    pattern: actual_pattern,
                    size: actual_size,
                    endianness: actual_endianness,
                },
                D::Float {
                    pattern: expected_pattern,
                    size: expected_size,
                    endianness: expected_endianness,
                },
            ) => {
                bit_array_value_matches(actual_pattern, expected_pattern)
                    && bit_array_size_matches(actual_size, expected_size, values)
                    && actual_endianness == expected_endianness
            }
            (
                E::Bits {
                    pattern: actual_pattern,
                    size: actual_size,
                    unit: actual_unit,
                },
                D::Bits {
                    pattern: expected_pattern,
                    size: expected_size,
                    unit: expected_unit,
                },
            ) => {
                bit_array_binding_matches(actual_pattern, expected_pattern)
                    && match (actual_size, expected_size) {
                        (None, None) => true,
                        (Some(actual), Some(expected)) => {
                            bit_array_size_matches(actual, expected, values)
                        }
                        _ => false,
                    }
                    && actual_unit == expected_unit
            }
            (
                E::String {
                    pattern: actual_pattern,
                    encoding: actual_encoding,
                },
                D::String {
                    pattern: expected_pattern,
                    encoding: expected_encoding,
                },
            ) => {
                bit_array_string_matches(actual_pattern, expected_pattern)
                    && actual_encoding == expected_encoding
            }
            (
                E::UtfCodepoint {
                    pattern: actual_pattern,
                    encoding: actual_encoding,
                },
                D::UtfCodepoint {
                    pattern: expected_pattern,
                    encoding: expected_encoding,
                },
            ) => {
                bit_array_binding_matches(actual_pattern, expected_pattern)
                    && actual_encoding == expected_encoding
            }
            _ => false,
        }
    }

    fn bit_array_value_matches<Value: PartialEq>(
        actual: &execution_graph::BitArrayPatternValue<Value>,
        expected: &draft_pattern::DraftBitArrayPatternValue<Value>,
    ) -> bool {
        use draft_pattern::DraftBitArrayPatternValue as D;
        use execution_graph::BitArrayPatternValue as E;

        match (actual, expected) {
            (E::Literal(actual), D::Literal(expected)) => actual == expected,
            (E::Bind(actual), D::Bind(expected)) => actual.index() == expected.index,
            (E::Discard, D::Discard) => true,
            (
                E::Alias {
                    pattern: actual_pattern,
                    binding: actual_binding,
                },
                D::Alias {
                    pattern: expected_pattern,
                    binding: expected_binding,
                },
            ) => {
                actual_binding.index() == expected_binding.index
                    && bit_array_value_matches(actual_pattern, expected_pattern)
            }
            _ => false,
        }
    }

    fn bit_array_binding_matches(
        actual: &execution_graph::BitArrayBindingPattern,
        expected: &draft_pattern::DraftBitArrayBindingPattern,
    ) -> bool {
        use draft_pattern::DraftBitArrayBindingPattern as D;
        use execution_graph::BitArrayBindingPattern as E;

        match (actual, expected) {
            (E::Bind(actual), D::Bind(expected)) => actual.index() == expected.index,
            (E::Discard, D::Discard) => true,
            (
                E::Alias {
                    pattern: actual_pattern,
                    binding: actual_binding,
                },
                D::Alias {
                    pattern: expected_pattern,
                    binding: expected_binding,
                },
            ) => {
                actual_binding.index() == expected_binding.index
                    && bit_array_binding_matches(actual_pattern, expected_pattern)
            }
            _ => false,
        }
    }

    fn bit_array_string_matches(
        actual: &execution_graph::BitArrayStringPattern,
        expected: &draft_pattern::DraftBitArrayStringPattern,
    ) -> bool {
        match (actual, expected) {
            (
                execution_graph::BitArrayStringPattern::Literal(actual),
                draft_pattern::DraftBitArrayStringPattern::Literal(expected),
            ) => actual == expected,
            (
                execution_graph::BitArrayStringPattern::Discard,
                draft_pattern::DraftBitArrayStringPattern::Discard,
            ) => true,
            _ => false,
        }
    }

    fn bit_array_size_matches(
        actual: &execution_graph::BitArrayPatternSize,
        expected: &draft_pattern::DraftBitArrayPatternSize,
        values: &BlockValues,
    ) -> bool {
        actual.unit() == expected.unit
            && bit_array_size_expr_matches(actual.value(), &expected.value, values)
    }

    fn bit_array_size_expr_matches(
        actual: &execution_graph::BitArrayPatternSizeExpr,
        expected: &draft_pattern::DraftBitArrayPatternSizeExpr,
        values: &BlockValues,
    ) -> bool {
        use draft_pattern::DraftBitArrayPatternSizeExpr as D;
        use execution_graph::BitArrayPatternSizeExpr as E;

        match (actual, expected) {
            (E::Value(actual), D::Value(expected)) => actual == expected,
            (E::Local(actual), D::Local(expected)) => *actual == values.int(expected),
            (E::Binding(actual), D::Binding(expected)) => actual.index() == *expected,
            (
                E::Add {
                    left: actual_left,
                    right: actual_right,
                },
                D::Add {
                    left: expected_left,
                    right: expected_right,
                },
            ) => {
                bit_array_size_expr_matches(actual_left, expected_left, values)
                    && bit_array_size_expr_matches(actual_right, expected_right, values)
            }
            (
                E::Subtract {
                    left: actual_left,
                    right: actual_right,
                },
                D::Subtract {
                    left: expected_left,
                    right: expected_right,
                },
            ) => {
                bit_array_size_expr_matches(actual_left, expected_left, values)
                    && bit_array_size_expr_matches(actual_right, expected_right, values)
            }
            (
                E::Multiply {
                    left: actual_left,
                    right: actual_right,
                },
                D::Multiply {
                    left: expected_left,
                    right: expected_right,
                },
            ) => {
                bit_array_size_expr_matches(actual_left, expected_left, values)
                    && bit_array_size_expr_matches(actual_right, expected_right, values)
            }
            (
                E::Divide {
                    left: actual_left,
                    right: actual_right,
                },
                D::Divide {
                    left: expected_left,
                    right: expected_right,
                },
            ) => {
                bit_array_size_expr_matches(actual_left, expected_left, values)
                    && bit_array_size_expr_matches(actual_right, expected_right, values)
            }
            (
                E::Remainder {
                    left: actual_left,
                    right: actual_right,
                },
                D::Remainder {
                    left: expected_left,
                    right: expected_right,
                },
            ) => {
                bit_array_size_expr_matches(actual_left, expected_left, values)
                    && bit_array_size_expr_matches(actual_right, expected_right, values)
            }
            _ => false,
        }
    }

    #[test]
    fn freezes_match_bindings_only_into_the_success_edge() {
        let plan = execution_plan(
            r#"
pub fn main() {
  let assert [first, second] = [1, 2]
  first + second
}
"#,
        );
        let body = plan.int_function(IntFunctionId(0)).body();
        let block_graph = body.block_graph();

        assert_eq!(block_graph.blocks().len(), 3);
        let entry = block_graph.block(crate::plan::execution::graph::BlockId::new(0));
        let (pattern, success, failure) = match_terminator(entry.terminator());
        let list = list_pattern(pattern);
        assert_eq!(list.elements().len(), 2);
        assert_binding_pattern(&list.elements()[0]);
        assert_binding_pattern(&list.elements()[1]);
        assert!(list.tail().is_none());
        assert_eq!(
            success.target(),
            crate::plan::execution::graph::BlockId::new(1)
        );
        assert_eq!(success.args().len(), 2);
        assert_eq!(binding_edge_argument(&success.args()[0]), 0);
        assert_eq!(binding_edge_argument(&success.args()[1]), 1);
        assert_eq!(
            failure.target(),
            crate::plan::execution::graph::BlockId::new(2)
        );
        assert_eq!(failure.args().len(), 1);
        let failure_subject = list_local(&failure.args()[0]);

        let success_block = block_graph.block(success.target());
        assert_eq!(
            success_block
                .params()
                .iter()
                .map(|slot| slot.local())
                .collect::<Vec<_>>(),
            vec![
                &ParamLocal::Int(IntLocalId(0)),
                &ParamLocal::Int(IntLocalId(1)),
            ],
        );
        assert_eq!(success_block.instructions().len(), 1);
        assert_eq!(
            int_add_operands(&success_block.instructions()[0]),
            (IntLocalId(0), IntLocalId(1)),
        );

        let failure_block = block_graph.block(failure.target());
        assert_eq!(failure_block.params().len(), 1);
        assert_eq!(
            failure_block.params()[0].local(),
            &ParamLocal::List(failure_subject.clone()),
        );
        let (panic_subject, message) = let_assert_panic(failure_block.terminator());
        assert_eq!(panic_subject, failure_block.params()[0].local());
        assert_eq!(message, None);
    }

    #[test]
    fn freezes_only_live_match_bindings_into_success_parameters() {
        let plan = execution_plan(
            r#"
pub fn main() {
  let assert [first, second] = [1, 2]
  first
}
"#,
        );
        let body = plan.int_function(IntFunctionId(0)).body();
        let block_graph = body.block_graph();
        let entry = block_graph.block(crate::plan::execution::graph::BlockId::new(0));
        let (pattern, success, _) = match_terminator(entry.terminator());
        let list = list_pattern(pattern);
        assert_eq!(list.elements().len(), 2);
        assert_binding_pattern(&list.elements()[0]);
        assert_binding_pattern(&list.elements()[1]);
        assert_eq!(success.args().len(), 1);
        assert_eq!(binding_edge_argument(&success.args()[0]), 0);

        let success_block = block_graph.block(success.target());
        assert_eq!(success_block.params().len(), 1);
        assert_eq!(
            success_block.params()[0].local(),
            &ParamLocal::Int(IntLocalId(0)),
        );
        assert_eq!(
            exit_id(success_block.terminator()),
            BlockGraphExitId::new(0)
        );
    }

    #[test]
    #[should_panic(expected = "fixture should contain a match terminator")]
    fn match_terminator_guard_rejects_an_exit() {
        match_terminator(&Terminator::Exit(BlockGraphExitId::new(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain a List pattern")]
    fn list_pattern_guard_rejects_discard() {
        list_pattern(&MatchPattern::Discard);
    }

    #[test]
    #[should_panic(expected = "fixture should contain a binding pattern")]
    fn binding_pattern_guard_rejects_discard() {
        assert_binding_pattern(&MatchPattern::Discard);
    }

    #[test]
    #[should_panic(expected = "fixture should export a match binding")]
    fn binding_argument_guard_rejects_a_value() {
        binding_edge_argument(&MatchEdgeArgument::Value(ParamLocal::Int(IntLocalId(0))));
    }

    #[test]
    #[should_panic(expected = "fixture should carry a List local")]
    fn list_local_guard_rejects_an_int() {
        list_local(&ParamLocal::Int(IntLocalId(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain an Int add instruction")]
    fn int_add_guard_rejects_a_value() {
        let plan = execution_plan("pub fn main() { 1 }");
        let graph = plan.int_function(IntFunctionId(0)).body().block_graph();
        int_add_operands(&graph.block(graph.entry()).instructions()[0]);
    }

    #[test]
    #[should_panic(expected = "fixture should contain a let-assert panic")]
    fn let_assert_guard_rejects_an_exit() {
        let_assert_panic(&Terminator::Exit(BlockGraphExitId::new(0)));
    }

    #[test]
    #[should_panic(expected = "fixture should contain an exit terminator")]
    fn exit_guard_rejects_a_match() {
        let plan = execution_plan(
            r#"
pub fn main() {
  let assert [first] = [1]
  first
}
"#,
        );
        let graph = plan.int_function(IntFunctionId(0)).body().block_graph();
        exit_id(graph.block(graph.entry()).terminator());
    }

    fn match_terminator(
        terminator: &Terminator,
    ) -> (
        &MatchPattern,
        &MatchEdge,
        &crate::plan::execution::graph::Edge,
    ) {
        match terminator {
            Terminator::Match(matcher) => (matcher.pattern(), matcher.success(), matcher.failure()),
            _ => panic!("fixture should contain a match terminator"),
        }
    }

    fn list_pattern(pattern: &MatchPattern) -> &MatchPatternList {
        match pattern {
            MatchPattern::List(pattern) => pattern,
            _ => panic!("fixture should contain a List pattern"),
        }
    }

    fn assert_binding_pattern(pattern: &MatchPattern) {
        match pattern {
            MatchPattern::Bind(_) => {}
            _ => panic!("fixture should contain a binding pattern"),
        }
    }

    fn binding_edge_argument(argument: &MatchEdgeArgument) -> usize {
        match argument {
            MatchEdgeArgument::Binding(index) => *index,
            MatchEdgeArgument::Value(_) => panic!("fixture should export a match binding"),
        }
    }

    fn list_local(local: &ParamLocal) -> &ListLocal {
        match local {
            ParamLocal::List(local) => local,
            _ => panic!("fixture should carry a List local"),
        }
    }

    fn int_add_operands(instruction: &Instruction) -> (IntLocalId, IntLocalId) {
        match instruction.kind() {
            InstructionKind::Int(IntInstruction::Add { left, right }) => (*left, *right),
            _ => panic!("fixture should contain an Int add instruction"),
        }
    }

    fn let_assert_panic(
        terminator: &Terminator,
    ) -> (
        &ParamLocal,
        Option<crate::plan::execution::graph::StringLocalId>,
    ) {
        match terminator {
            Terminator::LetAssertPanic(panic) => (panic.subject(), panic.message()),
            _ => panic!("fixture should contain a let-assert panic"),
        }
    }

    fn exit_id(terminator: &Terminator) -> BlockGraphExitId {
        match terminator {
            Terminator::Exit(exit) => *exit,
            _ => panic!("fixture should contain an exit terminator"),
        }
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }
}
