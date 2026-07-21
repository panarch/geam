use super::{call_args, custom, function, int_expr, list, panic_expr, tuple};
use crate::plan::execution::lowering::graph::{
    DraftBitArray, DraftBitArrayBitsSize, DraftBitArrayEvaluatedSize, DraftBitArraySegment,
    DraftCursor, DraftFlow, DraftGraph,
};
use crate::plan::execution::lowering::specialization::{Representability, StoredValueShape};
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn bit_array_expr(
    expression: &module::BitArrayExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftBitArray>> {
    use super::super::instruction::DraftBitArrayInstruction as I;
    use module::BitArrayExprKind as E;

    match expression.kind() {
        E::Value(segments) => {
            lower_segments(segments, cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: segments,
                } => {
                    let value = graph.bit_array_instruction(&mut cursor, I::Value(segments));
                    DraftFlow::value(cursor, value)
                }
            })
        }
        E::Constant(reference) => context.bit_array_constant(reference).map(|id| {
            let mut cursor = cursor;
            let value = graph.bit_array_instruction(
                &mut cursor,
                I::Constant(execution::ConstantId::new(id.index())),
            );
            DraftFlow::value(cursor, value)
        }),
        E::LocalGet { local, name: _ } => {
            let value = cursor
                .scope()
                .bit_array(super::super::super::local::LocalKey::new(
                    super::super::super::local::LocalKind::BitArray,
                    local.0,
                ));
            Representability::Inhabited(DraftFlow::value(cursor, value))
        }
        E::Call { function, args } => {
            call_args(args, cursor, graph, context).and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value {
                    mut cursor,
                    value: args,
                } => context.bit_array_function_id(function).map(|function| {
                    let value =
                        graph.bit_array_instruction(&mut cursor, I::Call { function, args });
                    DraftFlow::value(cursor, value)
                }),
            })
        }
        E::FunctionCall {
            function: value,
            args,
        } => function::bit_array_function_expr(value, cursor, graph, context).and_then(|flow| {
            match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value {
                    cursor,
                    value: function,
                } => call_args(args, cursor, graph, context).map(|flow| match flow {
                    DraftFlow::Diverged => DraftFlow::Diverged,
                    DraftFlow::Value {
                        mut cursor,
                        value: args,
                    } => {
                        let value = graph.bit_array_instruction(
                            &mut cursor,
                            I::FunctionCall {
                                function: function.value().clone(),
                                args,
                            },
                        );
                        DraftFlow::value(cursor, value)
                    }
                }),
            }
        }),
        E::TupleIndex {
            tuple: source,
            index,
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: tuple,
            } => {
                let value = graph.bit_array_instruction(
                    &mut cursor,
                    I::TupleIndex {
                        tuple,
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, value)
            }
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: source,
                } => {
                    let value = graph.bit_array_instruction(
                        &mut cursor,
                        I::CustomField {
                            source,
                            index: access.index(),
                        },
                    );
                    DraftFlow::value(cursor, value)
                }
            })
        }
        E::ListIndex {
            list: source,
            index,
        } => list::bit_array_list_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: list,
            } => {
                let value = graph.bit_array_instruction(
                    &mut cursor,
                    I::ListIndex {
                        list: list.value().clone(),
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, value)
            }
        }),
        E::Panic(value) => panic_expr(value, cursor, graph, context).map(|_| DraftFlow::Diverged),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            cursor,
            super::case_lowering(graph, context, StoredValueShape::BitArray),
            |cursor, graph, context| bit_array_expr(true_, cursor, graph, context),
            |cursor, graph, context| bit_array_expr(false_, cursor, graph, context),
            DraftBitArray::from_ref,
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => super::int_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, StoredValueShape::BitArray),
            bit_array_expr,
            DraftBitArray::from_ref,
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => super::string_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, StoredValueShape::BitArray),
            bit_array_expr,
            DraftBitArray::from_ref,
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::float_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::case_lowering(graph, context, StoredValueShape::BitArray),
            bit_array_expr,
            DraftBitArray::from_ref,
        ),
        E::Block { steps, return_ } => super::super::step::steps(steps, cursor, graph, context)
            .and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value { cursor, value: () } => {
                    bit_array_expr(return_, cursor, graph, context)
                }
            }),
    }
}

fn lower_segments(
    segments: &[module::BitArraySegment],
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<Vec<DraftBitArraySegment>>> {
    let mut lowered = Vec::with_capacity(segments.len());
    for segment in segments {
        match lower_segment(segment, cursor, graph, context) {
            Representability::Uninhabited => return Representability::Uninhabited,
            Representability::Inhabited(DraftFlow::Diverged) => {
                return Representability::Inhabited(DraftFlow::Diverged);
            }
            Representability::Inhabited(DraftFlow::Value {
                cursor: next,
                value,
            }) => {
                cursor = next;
                lowered.push(value);
            }
        }
    }
    Representability::Inhabited(DraftFlow::value(cursor, lowered))
}

fn lower_segment(
    segment: &module::BitArraySegment,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftBitArraySegment>> {
    match segment {
        module::BitArraySegment::Int {
            value,
            bit_size,
            endianness,
        } => int_expr(value, cursor, graph, context).map(|flow| {
            flow.map(|value| DraftBitArraySegment::Int {
                value,
                bit_size: *bit_size,
                endianness: lower_endianness(*endianness),
            })
        }),
        module::BitArraySegment::EvaluatedInt {
            value,
            size,
            endianness,
            site,
        } => int_expr(value, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value { cursor, value } => {
                lower_evaluated_size(size, cursor, graph, context).map(|flow| {
                    flow.map(|size| DraftBitArraySegment::EvaluatedInt {
                        value,
                        size,
                        endianness: lower_endianness(*endianness),
                        site: site.clone(),
                    })
                })
            }
        }),
        module::BitArraySegment::Float {
            value,
            bit_size,
            endianness,
        } => super::float_expr(value, cursor, graph, context).map(|flow| {
            flow.map(|value| DraftBitArraySegment::Float {
                value,
                bit_size: lower_float_bit_size(*bit_size),
                endianness: lower_endianness(*endianness),
            })
        }),
        module::BitArraySegment::EvaluatedFloat {
            value,
            size,
            endianness,
            site,
        } => super::float_expr(value, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value { cursor, value } => {
                lower_evaluated_size(size, cursor, graph, context).map(|flow| {
                    flow.map(|size| DraftBitArraySegment::EvaluatedFloat {
                        value,
                        size,
                        endianness: lower_endianness(*endianness),
                        site: site.clone(),
                    })
                })
            }
        }),
        module::BitArraySegment::String { value, encoding } => {
            super::string_expr(value, cursor, graph, context).map(|flow| {
                flow.map(|value| DraftBitArraySegment::String {
                    value,
                    encoding: lower_string_encoding(*encoding),
                })
            })
        }
        module::BitArraySegment::UtfCodepoint { value, encoding } => {
            super::utf_codepoint_expr(value, cursor, graph, context).map(|flow| {
                flow.map(|value| DraftBitArraySegment::UtfCodepoint {
                    value,
                    encoding: lower_string_encoding(*encoding),
                })
            })
        }
        module::BitArraySegment::Bits(value) => bit_array_expr(value, cursor, graph, context)
            .map(|flow| flow.map(DraftBitArraySegment::Bits)),
        module::BitArraySegment::SizedBits { value, size, site } => {
            bit_array_expr(value, cursor, graph, context).and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value { cursor, value } => lower_bits_size(size, cursor, graph, context)
                    .map(|flow| {
                        flow.map(|size| DraftBitArraySegment::SizedBits {
                            value,
                            size,
                            site: site.clone(),
                        })
                    }),
            })
        }
    }
}

fn lower_bits_size(
    size: &module::BitArrayBitsSize,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftBitArrayBitsSize>> {
    match size {
        module::BitArrayBitsSize::Fixed(size) => Representability::Inhabited(DraftFlow::value(
            cursor,
            DraftBitArrayBitsSize::Fixed(*size),
        )),
        module::BitArrayBitsSize::Evaluated(size) => {
            lower_evaluated_size(size, cursor, graph, context)
                .map(|flow| flow.map(DraftBitArrayBitsSize::Evaluated))
        }
    }
}

fn lower_evaluated_size(
    size: &module::BitArrayEvaluatedSize,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftBitArrayEvaluatedSize>> {
    int_expr(size.value(), cursor, graph, context).map(|flow| {
        flow.map(|value| DraftBitArrayEvaluatedSize {
            value,
            unit: size.unit(),
        })
    })
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
    use super::bit_array_expr;
    use crate::plan::execution::lowering::graph::{DraftFlow, DraftGraphBuilder, DraftValueRef};
    use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};
    use crate::plan::{
        BitArrayExpr, BitArraySegment, Endianness, FunctionShape, FunctionTemplateId, IntExpr,
        PanicExpr, PanicSite, ValueShape, monomorphic_function_instantiation,
    };

    #[derive(Debug, PartialEq, Eq)]
    enum FlowOutcome {
        Uninhabited,
        Diverged,
        Value,
    }

    fn flow_outcome<T>(flow: Representability<DraftFlow<T>>) -> FlowOutcome {
        match flow {
            Representability::Uninhabited => FlowOutcome::Uninhabited,
            Representability::Inhabited(DraftFlow::Diverged) => FlowOutcome::Diverged,
            Representability::Inhabited(DraftFlow::Value { .. }) => FlowOutcome::Value,
        }
    }

    #[test]
    fn bit_array_construction_propagates_an_erased_segment_specialization() {
        let function =
            monomorphic_function_instantiation(0, FunctionShape::new(Vec::new(), ValueShape::Int));
        let expressions = [
            (IntExpr::value(1.into()), FlowOutcome::Value),
            (
                IntExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown())),
                FlowOutcome::Diverged,
            ),
            (
                IntExpr::call(function, Vec::new()),
                FlowOutcome::Uninhabited,
            ),
        ];
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        context
            .erased_specializations
            .insert(SpecializationKey::monomorphic(FunctionTemplateId::new(0)));
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        for (value, expected) in expressions {
            let expression = BitArrayExpr::value(vec![BitArraySegment::Int {
                value,
                bit_size: 8,
                endianness: Endianness::Big,
            }]);
            let cursor = graph.empty_block(cursor.scope().clone());
            assert_eq!(
                flow_outcome(bit_array_expr(
                    &expression,
                    cursor,
                    &mut graph,
                    &mut context
                )),
                expected,
            );
        }
    }

    #[test]
    fn bit_array_segments_preserve_value_size_and_segment_order() {
        for (expression, expected) in [
            (
                "<<failed_int(\"value\"):size(failed_int(\"size\"))>>",
                "panic: value",
            ),
            ("<<1:size(failed_int(\"size\"))>>", "panic: size"),
            (
                "<<failed_float(\"value\"):float-size(failed_int(\"size\"))>>",
                "panic: value",
            ),
            ("<<1.0:float-size(failed_int(\"size\"))>>", "panic: size"),
            (
                "<<failed_bits(\"value\"):bits-size(failed_int(\"size\"))>>",
                "panic: value",
            ),
            ("<<<<1>>:bits-size(failed_int(\"size\"))>>", "panic: size"),
            ("<<1, failed_int(\"second\")>>", "panic: second"),
        ] {
            assert_eq!(run(expression), expected);
        }
    }

    fn run(expression: &str) -> String {
        let source = format!(
            r#"
fn failed_int(message: String) -> Int {{ panic as message }}
fn failed_float(message: String) -> Float {{ panic as message }}
fn failed_bits(message: String) -> BitArray {{ panic as message }}

pub fn main() {{ {expression} }}
"#,
        );
        let typed = crate::compile_typed_module("main", "main.gleam", source.as_str())
            .expect("source should compile");
        let module = crate::plan_module(typed).expect("source should plan");
        crate::run_main(&crate::ExecutionPlan::from_module_plan(module))
            .unwrap_err()
            .to_string()
    }
}
