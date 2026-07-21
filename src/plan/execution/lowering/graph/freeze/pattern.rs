use super::super::pattern::{
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
