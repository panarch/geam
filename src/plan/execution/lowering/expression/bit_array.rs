use super::super::specialization::Representability;
use super::{
    bit_array_function_expr, bit_array_list_expr, custom_field_access, float_expr, int_expr,
    panic_expr, string_expr, tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn bit_array_expr(
    expression: &module::BitArrayExpr,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::BitArrayExpr> {
    use execution::BitArrayExprKind as E;
    use module::BitArrayExprKind as M;

    let kind = match expression.kind() {
        M::Value(segments) => Representability::collect(
            segments
                .iter()
                .map(|segment| bit_array_segment(segment, context)),
        )
        .map(E::Value),
        M::Constant(reference) => context.bit_array_constant(reference).map(E::Constant),
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::BitArrayLocalId(
                context.mapped_local(super::super::frame::LocalKind::BitArray, local.0),
            ),
        }),
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.bit_array_function_id(function)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| bit_array_function_expr(function, context),
            |context| super::function::evaluated_bit_array_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
            tuple: Box::new(tuple),
            index: *index,
        }),
        M::CustomField(access) => custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => {
            bit_array_list_expr(list, context).map(|list| E::ListIndex {
                list: Box::new(list),
                index: *index,
            })
        }
        M::Panic(value) => panic_expr(value, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case_into(
            subject,
            context,
            |context| bit_array_expr(true_, context),
            |context| bit_array_expr(false_, context),
            execution::BitArrayExpr::into_kind,
            |subject, true_, false_| E::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        ),
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                bit_array_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                bit_array_expr(fallback, context).map(|fallback| E::IntCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                bit_array_expr(branch, context).map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                bit_array_expr(fallback, context).map(|fallback| E::StringCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                bit_array_expr(branch, context).map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                bit_array_expr(fallback, context).map(|fallback| E::FloatCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                bit_array_expr(return_, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    };
    kind.map(execution::BitArrayExpr::from_kind)
}

fn bit_array_segment(
    segment: &module::BitArraySegment,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::BitArraySegment> {
    match segment {
        module::BitArraySegment::Int {
            value,
            bit_size,
            endianness,
        } => int_expr(value, context).map(|value| execution::BitArraySegment::Int {
            value,
            bit_size: *bit_size,
            endianness: lower_endianness(*endianness),
        }),
        module::BitArraySegment::EvaluatedInt {
            value,
            size,
            endianness,
            site,
        } => {
            int_expr(value, context).zip_with(lower_evaluated_size(size, context), |value, size| {
                execution::BitArraySegment::EvaluatedInt {
                    value,
                    size,
                    endianness: lower_endianness(*endianness),
                    site: site.clone(),
                }
            })
        }
        module::BitArraySegment::Float {
            value,
            bit_size,
            endianness,
        } => float_expr(value, context).map(|value| execution::BitArraySegment::Float {
            value,
            bit_size: lower_float_bit_size(*bit_size),
            endianness: lower_endianness(*endianness),
        }),
        module::BitArraySegment::EvaluatedFloat {
            value,
            size,
            endianness,
            site,
        } => float_expr(value, context).zip_with(
            lower_evaluated_size(size, context),
            |value, size| execution::BitArraySegment::EvaluatedFloat {
                value,
                size,
                endianness: lower_endianness(*endianness),
                site: site.clone(),
            },
        ),
        module::BitArraySegment::String { value, encoding } => {
            string_expr(value, context).map(|value| execution::BitArraySegment::String {
                value,
                encoding: lower_string_encoding(*encoding),
            })
        }
        module::BitArraySegment::UtfCodepoint { value, encoding } => {
            super::utf_codepoint_expr(value, context).map(|value| {
                execution::BitArraySegment::UtfCodepoint {
                    value,
                    encoding: (*encoding).into(),
                }
            })
        }
        module::BitArraySegment::Bits(value) => {
            bit_array_expr(value, context).map(execution::BitArraySegment::Bits)
        }
        module::BitArraySegment::SizedBits { value, size, site } => bit_array_expr(value, context)
            .zip_with(lower_bits_size(size, context), |value, size| {
                execution::BitArraySegment::SizedBits {
                    value,
                    size,
                    site: site.clone(),
                }
            }),
    }
}

fn lower_bits_size(
    size: &module::BitArrayBitsSize,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::BitArrayBitsSize> {
    match size {
        module::BitArrayBitsSize::Fixed(size) => {
            Representability::Inhabited(execution::BitArrayBitsSize::Fixed(*size))
        }
        module::BitArrayBitsSize::Evaluated(size) => {
            lower_evaluated_size(size, context).map(execution::BitArrayBitsSize::Evaluated)
        }
    }
}

fn lower_string_encoding(value: module::StringEncoding) -> execution::StringEncoding {
    match value {
        module::StringEncoding::Utf8 => execution::StringEncoding::Utf8,
        module::StringEncoding::Utf16(endianness) => {
            execution::StringEncoding::Utf16(lower_endianness(endianness))
        }
        module::StringEncoding::Utf32(endianness) => {
            execution::StringEncoding::Utf32(lower_endianness(endianness))
        }
    }
}

fn lower_evaluated_size(
    size: &module::BitArrayEvaluatedSize,
    context: &mut super::super::LoweringContext,
) -> Representability<execution::BitArrayEvaluatedSize> {
    int_expr(size.value(), context)
        .map(|value| execution::BitArrayEvaluatedSize::new(value, size.unit()))
}

fn lower_float_bit_size(value: module::FloatBitSize) -> execution::FloatBitSize {
    match value {
        module::FloatBitSize::Sixteen => execution::FloatBitSize::Sixteen,
        module::FloatBitSize::ThirtyTwo => execution::FloatBitSize::ThirtyTwo,
        module::FloatBitSize::SixtyFour => execution::FloatBitSize::SixtyFour,
    }
}

fn lower_endianness(value: module::Endianness) -> execution::Endianness {
    match value {
        module::Endianness::Big => execution::Endianness::Big,
        module::Endianness::Little => execution::Endianness::Little,
    }
}

#[cfg(test)]
mod tests {
    use super::{execution, lower_float_bit_size, module};

    #[test]
    fn lowering_preserves_float_bit_sizes() {
        assert_eq!(
            [
                lower_float_bit_size(module::FloatBitSize::Sixteen),
                lower_float_bit_size(module::FloatBitSize::ThirtyTwo),
                lower_float_bit_size(module::FloatBitSize::SixtyFour),
            ],
            [
                execution::FloatBitSize::Sixteen,
                execution::FloatBitSize::ThirtyTwo,
                execution::FloatBitSize::SixtyFour,
            ],
        );
    }
}
