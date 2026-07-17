use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use num_bigint::BigInt;

use super::{
    eval_bool_expr, eval_custom_field, eval_float_expr, eval_int_expr, eval_panic_expr,
    eval_string_expr, eval_utf_codepoint_expr, project_bit_array_list_expr, project_tuple_expr,
};
use crate::plan::ValueType;
use crate::plan::execution::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayExpr, BitArrayExprKind, BitArraySegment,
    Endianness, ExecutionPlan, FloatBitSize, StringEncoding,
};
use crate::runtime::evaluated::{EvaluatedBitArray, EvaluatedValue};
use crate::runtime::frame::Frame;
use crate::runtime::state::RuntimeState;
use crate::runtime::{BitArraySegmentPanicReason, ExecutionError, function};

pub(in crate::runtime) fn eval_bit_array_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &BitArrayExpr,
) -> Result<EvaluatedBitArray, ExecutionError> {
    match expression.kind() {
        BitArrayExprKind::Value(segments) => {
            let mut bits = BitVec::<u8, Msb0>::new();
            for segment in segments {
                append_segment(plan, state, frame, &mut bits, segment)?;
            }
            Ok(EvaluatedBitArray::new(bits))
        }
        BitArrayExprKind::LocalGet { local } => Ok(frame.get_bit_array(*local)),
        BitArrayExprKind::Call { function, args } => {
            function::run_bit_array_call(plan, state, *function, args, frame)
        }
        BitArrayExprKind::FunctionCall { function, args } => {
            function::run_bit_array_function_call(plan, state, function, args, frame)
        }
        BitArrayExprKind::TupleIndex { tuple, index } => {
            match project_tuple_expr(plan, state, frame, tuple, *index, ValueType::BitArray)? {
                EvaluatedValue::BitArray(value) => Ok(value),
                other => Err(ExecutionError::TupleIndexFamilyMismatch {
                    expected: ValueType::BitArray,
                    actual: other.value_type(plan),
                }),
            }
        }
        BitArrayExprKind::CustomField(access) => {
            let (constructor, value) = eval_custom_field(plan, state, frame, access)?;
            match value {
                EvaluatedValue::BitArray(value) => Ok(value),
                other => {
                    let descriptor = plan.custom_constructor(constructor);
                    Err(ExecutionError::CustomFieldFamilyMismatch {
                        custom_type: plan.custom_value_type(constructor.type_id()),
                        constructor: descriptor.name().clone(),
                        field_index: access.index(),
                        expected: ValueType::BitArray,
                        actual: other.value_type(plan),
                    })
                }
            }
        }
        BitArrayExprKind::ListIndex { list, index } => {
            project_bit_array_list_expr(plan, state, frame, list, *index)
        }
        BitArrayExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        BitArrayExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_bit_array_expr(plan, state, frame, true_)
            } else {
                eval_bit_array_expr(plan, state, frame, false_)
            }
        }
        BitArrayExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bit_array_expr(plan, state, frame, branch);
                }
            }
            eval_bit_array_expr(plan, state, frame, fallback)
        }
        BitArrayExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bit_array_expr(plan, state, frame, branch);
                }
            }
            eval_bit_array_expr(plan, state, frame, fallback)
        }
        BitArrayExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_bit_array_expr(plan, state, frame, branch);
                }
            }
            eval_bit_array_expr(plan, state, frame, fallback)
        }
        BitArrayExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_bit_array_expr(plan, state, frame, return_)
        }
    }
}

fn append_segment(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    output: &mut BitVec<u8, Msb0>,
    segment: &BitArraySegment,
) -> Result<(), ExecutionError> {
    match segment {
        BitArraySegment::Int {
            value,
            bit_size,
            endianness,
        } => {
            let value = eval_int_expr(plan, state, frame, value)?;
            append_integer(output, &value, *bit_size, *endianness);
        }
        BitArraySegment::EvaluatedInt {
            value,
            size,
            endianness,
            site,
        } => {
            let value = eval_int_expr(plan, state, frame, value)?;
            let bit_size = eval_bit_size(plan, state, frame, size, site)?;
            append_integer(output, &value, bit_size, *endianness);
        }
        BitArraySegment::Float {
            value,
            bit_size,
            endianness,
        } => {
            let value = eval_float_expr(plan, state, frame, value)?;
            append_float(output, value, *bit_size, *endianness);
        }
        BitArraySegment::EvaluatedFloat {
            value,
            size,
            endianness,
            site,
        } => {
            let value = eval_float_expr(plan, state, frame, value)?;
            let bit_size = eval_bit_size(plan, state, frame, size, site)?;
            let bit_size = match bit_size {
                16 => FloatBitSize::Sixteen,
                32 => FloatBitSize::ThirtyTwo,
                64 => FloatBitSize::SixtyFour,
                bit_size => {
                    return Err(ExecutionError::bit_array_segment_panic(
                        plan.source_context(),
                        BitArraySegmentPanicReason::InvalidFloatSize {
                            bit_size: BigInt::from(bit_size),
                        },
                        site.clone(),
                    ));
                }
            };
            append_float(output, value, bit_size, *endianness);
        }
        BitArraySegment::String { value, encoding } => {
            let value = eval_string_expr(plan, state, frame, value)?;
            append_string(output, value.as_str(), *encoding);
        }
        BitArraySegment::UtfCodepoint { value, encoding } => {
            let value = eval_utf_codepoint_expr(plan, state, frame, value)?;
            append_utf_codepoint(output, value, *encoding);
        }
        BitArraySegment::Bits(value) => {
            let value = eval_bit_array_expr(plan, state, frame, value)?;
            output.extend_from_bitslice(value.bits());
        }
        BitArraySegment::SizedBits { value, size, site } => {
            let value = eval_bit_array_expr(plan, state, frame, value)?;
            let bit_size = match size {
                BitArrayBitsSize::Fixed(bit_size) => *bit_size,
                BitArrayBitsSize::Evaluated(size) => eval_bit_size(plan, state, frame, size, site)?,
            };
            let Some(bits) = value.bits().get(..bit_size) else {
                return Err(ExecutionError::bit_array_segment_panic(
                    plan.source_context(),
                    BitArraySegmentPanicReason::InsufficientBits {
                        requested: bit_size,
                        available: value.bits().len(),
                    },
                    site.clone(),
                ));
            };
            output.extend_from_bitslice(bits);
        }
    }
    Ok(())
}

fn eval_bit_size(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    size: &BitArrayEvaluatedSize,
    site: &crate::plan::PanicSite,
) -> Result<usize, ExecutionError> {
    let value = eval_int_expr(plan, state, frame, size.value())?;
    let bit_size = if value < BigInt::from(0) {
        BigInt::from(0)
    } else {
        value * BigInt::from(size.unit())
    };
    usize::try_from(bit_size.clone()).map_err(|_| {
        ExecutionError::bit_array_segment_panic(
            plan.source_context(),
            BitArraySegmentPanicReason::SizeOutOfRange { bit_size },
            site.clone(),
        )
    })
}

fn append_integer(
    output: &mut BitVec<u8, Msb0>,
    value: &BigInt,
    bit_size: usize,
    endianness: Endianness,
) {
    if bit_size == 0 {
        return;
    }
    let mask = (BigInt::from(1u8) << bit_size) - BigInt::from(1u8);
    let truncated = value & mask;
    match endianness {
        Endianness::Big => {
            let bytes = truncated.to_bytes_be().1;
            append_low_bits(output, &bytes, bit_size);
        }
        Endianness::Little => {
            let bytes = truncated.to_bytes_le().1;
            let full_bytes = bit_size / 8;
            for index in 0..full_bytes {
                output.extend_from_bitslice(
                    bytes.get(index).copied().unwrap_or(0).view_bits::<Msb0>(),
                );
            }
            let remaining = bit_size % 8;
            if remaining > 0 {
                let byte = bytes.get(full_bytes).copied().unwrap_or(0);
                output.extend_from_bitslice(&byte.view_bits::<Msb0>()[8 - remaining..]);
            }
        }
    }
}

fn append_low_bits(output: &mut BitVec<u8, Msb0>, bytes: &[u8], bit_size: usize) {
    let byte_len = bit_size.div_ceil(8);
    let padding = byte_len.saturating_sub(bytes.len());
    let mut padded = vec![0; padding];
    padded.extend_from_slice(bytes);
    let leading = byte_len * 8 - bit_size;
    output.extend_from_bitslice(&padded.view_bits::<Msb0>()[leading..]);
}

fn append_float(
    output: &mut BitVec<u8, Msb0>,
    value: f64,
    bit_size: FloatBitSize,
    endianness: Endianness,
) {
    let bytes = match (bit_size, endianness) {
        (FloatBitSize::Sixteen, Endianness::Big) => {
            half::f16::from_f64(value).to_be_bytes().to_vec()
        }
        (FloatBitSize::Sixteen, Endianness::Little) => {
            half::f16::from_f64(value).to_le_bytes().to_vec()
        }
        (FloatBitSize::ThirtyTwo, Endianness::Big) => (value as f32).to_be_bytes().to_vec(),
        (FloatBitSize::ThirtyTwo, Endianness::Little) => (value as f32).to_le_bytes().to_vec(),
        (FloatBitSize::SixtyFour, Endianness::Big) => value.to_be_bytes().to_vec(),
        (FloatBitSize::SixtyFour, Endianness::Little) => value.to_le_bytes().to_vec(),
    };
    output.extend_from_bitslice(bytes.view_bits::<Msb0>());
}

fn append_string(output: &mut BitVec<u8, Msb0>, value: &str, encoding: StringEncoding) {
    match encoding {
        StringEncoding::Utf8 => output.extend_from_bitslice(value.as_bytes().view_bits::<Msb0>()),
        StringEncoding::Utf16(endianness) => {
            for code in value.encode_utf16() {
                let bytes = match endianness {
                    Endianness::Big => code.to_be_bytes(),
                    Endianness::Little => code.to_le_bytes(),
                };
                output.extend_from_bitslice(bytes.view_bits::<Msb0>());
            }
        }
        StringEncoding::Utf32(endianness) => {
            for character in value.chars() {
                let code = u32::from(character);
                let bytes = match endianness {
                    Endianness::Big => code.to_be_bytes(),
                    Endianness::Little => code.to_le_bytes(),
                };
                output.extend_from_bitslice(bytes.view_bits::<Msb0>());
            }
        }
    }
}

fn append_utf_codepoint(output: &mut BitVec<u8, Msb0>, value: char, encoding: StringEncoding) {
    match encoding {
        StringEncoding::Utf8 => {
            let mut buffer = [0; 4];
            let encoded = value.encode_utf8(&mut buffer);
            output.extend_from_bitslice(encoded.as_bytes().view_bits::<Msb0>());
        }
        StringEncoding::Utf16(endianness) => {
            let mut buffer = [0; 2];
            for code in value.encode_utf16(&mut buffer) {
                let bytes = match endianness {
                    Endianness::Big => code.to_be_bytes(),
                    Endianness::Little => code.to_le_bytes(),
                };
                output.extend_from_bitslice(bytes.view_bits::<Msb0>());
            }
        }
        StringEncoding::Utf32(endianness) => {
            let code = u32::from(value);
            let bytes = match endianness {
                Endianness::Big => code.to_be_bytes(),
                Endianness::Little => code.to_le_bytes(),
            };
            output.extend_from_bitslice(bytes.view_bits::<Msb0>());
        }
    }
}

use bitvec::view::BitView;

#[cfg(test)]
mod tests {
    use bitvec::order::Msb0;
    use bitvec::vec::BitVec;
    use num_bigint::BigInt;

    use crate::plan::{
        BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayExpr, BitArrayFunctionId, BitArraySegment,
        BoolExpr, Endianness, Expr, FloatBitSize, FloatExpr, FunctionTemplate, FunctionTemplateId,
        IntExpr, ListExpr, ModulePlan, PanicExpr, PanicSite, ReturnExpr, SourceSpan, Step,
        StringEncoding, StringExpr, TupleExpr, UtfCodepointExpr, ValueType,
    };
    use crate::runtime::BitArraySegmentPanicReason;
    use crate::runtime::{BitArrayValue, ExecutionError, ListValue, Value, run_main};

    #[test]
    fn evaluated_segment_failures_preserve_exact_panic_reasons() {
        for (source, segment, reason) in [
            (
                "pub fn main() { let size = 24 <<1.5:float-size(size)>> }",
                "1.5:float-size(size)",
                BitArraySegmentPanicReason::InvalidFloatSize {
                    bit_size: 24.into(),
                },
            ),
            (
                "pub fn main() { let bits = <<1>> <<bits:bits-size(9)>> }",
                "bits:bits-size(9)",
                BitArraySegmentPanicReason::InsufficientBits {
                    requested: 9,
                    available: 8,
                },
            ),
            (
                "pub fn main() { let size = 99999999999999999999999999999999999999 <<1:size(size)>> }",
                "1:size(size)",
                BitArraySegmentPanicReason::SizeOutOfRange {
                    bit_size: BigInt::parse_bytes(b"99999999999999999999999999999999999999", 10)
                        .expect("integer"),
                },
            ),
        ] {
            let start = source.find(segment).expect("segment should exist");
            assert_eq!(
                crate::runtime::run_src_error(source),
                ExecutionError::bit_array_segment_panic(
                    None,
                    reason,
                    PanicSite::new(
                        "main".into(),
                        "main".into(),
                        SourceSpan::new(start, start + segment.len()),
                    ),
                ),
            );
        }
    }

    #[test]
    fn module_expression_errors_propagate_through_bit_array_wrappers() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let fallback = || BitArrayExpr::value(Vec::new());
        let expressions = [
            BitArrayExpr::value(vec![BitArraySegment::Int {
                value: IntExpr::panic(panic()),
                bit_size: 8,
                endianness: Endianness::Big,
            }]),
            BitArrayExpr::value(vec![BitArraySegment::Float {
                value: FloatExpr::panic(panic()),
                bit_size: FloatBitSize::SixtyFour,
                endianness: Endianness::Big,
            }]),
            BitArrayExpr::value(vec![BitArraySegment::String {
                value: StringExpr::panic(panic()),
                encoding: StringEncoding::Utf8,
            }]),
            BitArrayExpr::value(vec![BitArraySegment::UtfCodepoint {
                value: UtfCodepointExpr::panic(panic()),
                encoding: StringEncoding::Utf8,
            }]),
            BitArrayExpr::value(vec![BitArraySegment::Bits(BitArrayExpr::panic(panic()))]),
            BitArrayExpr::value(vec![BitArraySegment::EvaluatedInt {
                value: IntExpr::panic(panic()),
                size: BitArrayEvaluatedSize::new(IntExpr::value(8.into()), 1),
                endianness: Endianness::Big,
                site: PanicSite::unknown(),
            }]),
            BitArrayExpr::value(vec![BitArraySegment::EvaluatedInt {
                value: IntExpr::value(1.into()),
                size: BitArrayEvaluatedSize::new(IntExpr::panic(panic()), 1),
                endianness: Endianness::Big,
                site: PanicSite::unknown(),
            }]),
            BitArrayExpr::value(vec![BitArraySegment::EvaluatedFloat {
                value: FloatExpr::panic(panic()),
                size: BitArrayEvaluatedSize::new(IntExpr::value(16.into()), 1),
                endianness: Endianness::Big,
                site: PanicSite::unknown(),
            }]),
            BitArrayExpr::value(vec![BitArraySegment::EvaluatedFloat {
                value: FloatExpr::value(1.5),
                size: BitArrayEvaluatedSize::new(IntExpr::panic(panic()), 1),
                endianness: Endianness::Big,
                site: PanicSite::unknown(),
            }]),
            BitArrayExpr::value(vec![BitArraySegment::SizedBits {
                value: BitArrayExpr::panic(panic()),
                size: BitArrayBitsSize::Fixed(0),
                site: PanicSite::unknown(),
            }]),
            BitArrayExpr::value(vec![BitArraySegment::SizedBits {
                value: BitArrayExpr::value(Vec::new()),
                size: BitArrayBitsSize::Evaluated(BitArrayEvaluatedSize::new(
                    IntExpr::panic(panic()),
                    1,
                )),
                site: PanicSite::unknown(),
            }]),
            BitArrayExpr::tuple_index(TupleExpr::panic(panic(), vec![ValueType::BitArray]), 0),
            BitArrayExpr::list_index(
                ListExpr::panic(panic(), ValueType::BitArray)
                    .into_bit_array()
                    .expect("bit array list"),
                0,
            ),
            BitArrayExpr::bool_case(BoolExpr::panic(panic()), fallback(), fallback()),
            BitArrayExpr::int_case(IntExpr::panic(panic()), Vec::new(), fallback()),
            BitArrayExpr::string_case(StringExpr::panic(panic()), Vec::new(), fallback()),
            BitArrayExpr::float_case(FloatExpr::panic(panic()), Vec::new(), fallback()),
            BitArrayExpr::block(
                vec![Step::evaluate(Expr::bool(BoolExpr::panic(panic())))],
                fallback(),
            ),
        ];

        for expression in expressions {
            assert_eq!(
                run_module_bit_array_expression(expression).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    #[test]
    fn bit_array_tuple_projection_reports_direct_mutated_family_mismatch() {
        let expression = BitArrayExpr::tuple_index(
            TupleExpr::value(
                vec![Expr::int(IntExpr::value(1.into()))],
                vec![ValueType::BitArray],
            ),
            0,
        );

        assert_eq!(
            run_module_bit_array_expression(expression),
            ExecutionError::TupleIndexFamilyMismatch {
                expected: ValueType::BitArray,
                actual: ValueType::Int,
            },
        );
    }

    #[test]
    fn source_aligned_little_endian_integer_preserves_byte_order() {
        assert_eq!(
            crate::runtime::run_src("pub fn main() { <<0x1234:little-size(16)>> }"),
            Value::BitArray(BitArrayValue::from_bytes(vec![0x34, 0x12])),
        );
    }

    #[test]
    fn source_expression_paths_preserve_bit_array_values() {
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/values/bit_array_expression_paths.gleam"
            )),
            Value::Tuple(
                [
                    1, 2, 3, 11, 12, 3, 4, 5, 6, 7, 8, 9, 10, 14, 15, 16, 17, 18, 19, 20, 21, 13
                ]
                .into_iter()
                .map(|byte| Value::BitArray(BitArrayValue::from_bytes(vec![byte])))
                .collect(),
            ),
        );
    }

    #[test]
    fn source_expression_segments_preserve_bit_array_encodings() {
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/values/bit_array_segments.gleam"
            )),
            expected_segment_values(),
        );
    }

    #[test]
    fn source_constant_segments_preserve_bit_array_encodings() {
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/module_items/constant_bit_array_segments.gleam"
            )),
            expected_segment_values(),
        );
    }

    #[test]
    fn source_list_function_paths_preserve_bit_array_values() {
        let bit_array = |byte| Value::BitArray(BitArrayValue::from_bytes(vec![byte]));

        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/values/bit_array_list_function_paths.gleam"
            )),
            Value::Tuple(vec![
                bit_array(1),
                bit_array(1),
                bit_array(1),
                bit_array(1),
                bit_array(1),
            ]),
        );
    }

    #[test]
    fn source_let_assert_tail_preserves_bit_array_values() {
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/bindings/let_assert_bit_array_list.gleam"
            )),
            Value::Tuple(vec![
                Value::BitArray(BitArrayValue::from_bytes(vec![1])),
                Value::List(ListValue::bit_array(vec![BitArrayValue::from_bytes(vec![
                    2
                ])])),
            ]),
        );
    }

    #[test]
    fn source_tail_calls_preserve_bit_array_return_families() {
        assert_eq!(
            crate::runtime::run_src(include_str!(
                "../../../tests/fixtures/execution/functions/tail_call/bit_array_return_families.gleam"
            )),
            Value::Tuple(vec![
                Value::BitArray(BitArrayValue::from_bytes(vec![1])),
                Value::BitArray(BitArrayValue::from_bytes(vec![2])),
            ]),
        );
    }

    fn expected_segment_values() -> Value {
        let bit_array = |bytes, bit_len| {
            let mut bits = BitVec::<u8, Msb0>::from_vec(bytes);
            bits.truncate(bit_len);
            Value::BitArray(BitArrayValue::from_evaluated(&bits))
        };

        Value::Tuple(vec![
            bit_array(vec![], 0),
            bit_array(vec![], 0),
            bit_array(vec![0x23, 0x40], 12),
            bit_array(vec![0x34, 0x20], 12),
            bit_array(vec![0xf0], 4),
            bit_array(vec![0x01], 8),
            bit_array(vec![0x3e, 0x00], 16),
            bit_array(vec![0x00, 0x3e], 16),
            bit_array(vec![0x3f, 0xc0, 0x00, 0x00], 32),
            bit_array(vec![0x00, 0x00, 0xc0, 0x3f], 32),
            bit_array(vec![0x3f, 0xf8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00], 64),
            bit_array(vec![0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0xf8, 0x3f], 64),
            bit_array(vec![0xec, 0x95, 0x88], 24),
            bit_array(vec![0xec, 0x95, 0x88], 24),
            bit_array(vec![0xc5, 0x48], 16),
            bit_array(vec![0x48, 0xc5], 16),
            bit_array(vec![0x00, 0x00, 0x00, 0x41], 32),
            bit_array(vec![0x41, 0x00, 0x00, 0x00], 32),
            bit_array(vec![0x12], 8),
        ])
    }

    fn run_module_bit_array_expression(expression: BitArrayExpr) -> ExecutionError {
        let main = FunctionTemplate::new(
            FunctionTemplateId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::bit_array(BitArrayFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
