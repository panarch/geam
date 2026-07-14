use super::{
    bit_array_function_expr, bit_array_list_expr, bool_expr, call_args, float_expr, int_expr,
    panic_expr, string_expr, tuple_expr,
};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn bit_array_expr(
    expression: module::BitArrayExpr,
    context: &mut super::super::LoweringContext,
) -> execution::BitArrayExpr {
    use execution::BitArrayExprKind as E;
    use module::BitArrayExprKind as M;

    execution::BitArrayExpr::from_kind(match expression.into_kind() {
        M::Value(segments) => E::Value(
            segments
                .into_iter()
                .map(|segment| bit_array_segment(segment, context))
                .collect(),
        ),
        M::LocalGet { local, name: _ } => E::LocalGet {
            local: execution::BitArrayLocalId(local.0),
        },
        M::Call { function, args } => E::Call {
            function: execution::BitArrayFunctionId(function.0),
            args: call_args(args, context),
        },
        M::FunctionCall { function, args } => E::FunctionCall {
            function: Box::new(bit_array_function_expr(*function, context)),
            args: call_args(args, context),
        },
        M::TupleIndex { tuple, index } => E::TupleIndex {
            tuple: Box::new(tuple_expr(*tuple, context)),
            index,
        },
        M::ListIndex { list, index } => E::ListIndex {
            list: Box::new(bit_array_list_expr(*list, context)),
            index,
        },
        M::Panic(value) => E::Panic(panic_expr(value, context)),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: Box::new(bool_expr(*subject, context)),
            true_: Box::new(bit_array_expr(*true_, context)),
            false_: Box::new(bit_array_expr(*false_, context)),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: Box::new(int_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bit_array_expr(branch, context)))
                .collect(),
            fallback: Box::new(bit_array_expr(*fallback, context)),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: Box::new(string_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bit_array_expr(branch, context)))
                .collect(),
            fallback: Box::new(bit_array_expr(*fallback, context)),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: Box::new(float_expr(*subject, context)),
            clauses: clauses
                .into_iter()
                .map(|(pattern, branch)| (pattern, bit_array_expr(branch, context)))
                .collect(),
            fallback: Box::new(bit_array_expr(*fallback, context)),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::super::step::steps(steps, context),
            return_: Box::new(bit_array_expr(*return_, context)),
        },
    })
}

fn bit_array_segment(
    segment: module::BitArraySegment,
    context: &mut super::super::LoweringContext,
) -> execution::BitArraySegment {
    match segment {
        module::BitArraySegment::Int {
            value,
            bit_size,
            endianness,
        } => execution::BitArraySegment::Int {
            value: int_expr(value, context),
            bit_size,
            endianness: lower_endianness(endianness),
        },
        module::BitArraySegment::Float {
            value,
            bit_size,
            endianness,
        } => execution::BitArraySegment::Float {
            value: float_expr(value, context),
            bit_size: lower_float_bit_size(bit_size),
            endianness: lower_endianness(endianness),
        },
        module::BitArraySegment::String { value, encoding } => execution::BitArraySegment::String {
            value: string_expr(value, context),
            encoding: match encoding {
                module::StringEncoding::Utf8 => execution::StringEncoding::Utf8,
                module::StringEncoding::Utf16(endianness) => {
                    execution::StringEncoding::Utf16(lower_endianness(endianness))
                }
                module::StringEncoding::Utf32(endianness) => {
                    execution::StringEncoding::Utf32(lower_endianness(endianness))
                }
            },
        },
        module::BitArraySegment::UtfCodepoint { value, encoding } => {
            execution::BitArraySegment::UtfCodepoint {
                value: super::utf_codepoint_expr(value, context),
                encoding: encoding.into(),
            }
        }
        module::BitArraySegment::Bits(value) => {
            execution::BitArraySegment::Bits(bit_array_expr(value, context))
        }
    }
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
