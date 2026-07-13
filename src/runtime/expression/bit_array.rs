use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use num_bigint::BigInt;

use super::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_bit_array_list_expr, project_tuple_expr,
};
use crate::plan::ValueType;
use crate::plan::execution::{
    BitArrayExpr, BitArrayExprKind, BitArraySegment, Endianness, ExecutionPlan, FloatBitSize,
    StringEncoding,
};
use crate::runtime::evaluated::{EvaluatedBitArray, EvaluatedValue};
use crate::runtime::frame::Frame;
use crate::runtime::state::RuntimeState;
use crate::runtime::{ExecutionError, function};

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
        BitArraySegment::Float {
            value,
            bit_size,
            endianness,
        } => {
            let value = eval_float_expr(plan, state, frame, value)?;
            append_float(output, value, *bit_size, *endianness);
        }
        BitArraySegment::String { value, encoding } => {
            let value = eval_string_expr(plan, state, frame, value)?;
            append_string(output, value.as_str(), *encoding);
        }
        BitArraySegment::Bits(value) => {
            let value = eval_bit_array_expr(plan, state, frame, value)?;
            output.extend_from_bitslice(value.bits());
        }
    }
    Ok(())
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

use bitvec::view::BitView;

#[cfg(test)]
mod tests {
    use crate::plan::{
        BitArrayExpr, BitArrayFunctionId, BoolExpr, Expr, FloatExpr, FunctionId, FunctionPlan,
        IntExpr, ListExpr, ModulePlan, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr,
        TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn module_expression_errors_propagate_through_bit_array_wrappers() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let fallback = || BitArrayExpr::value(Vec::new());
        let expressions = [
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

    fn run_module_bit_array_expression(expression: BitArrayExpr) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
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
