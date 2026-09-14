use super::super::local::Locals;
use super::{InstructionError, Instructions, read};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction, BitArraySegment, ParamSlot,
};
use crate::plan::execution::type_::ValueType;

impl<'data, Graph: ExecutionGraphProfile> Instructions<'_, 'data, Graph> {
    pub(super) fn bit_array(
        &self,
        instruction: &BitArrayInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        self.output_type(output, &ValueType::BitArray)?;
        match instruction {
            BitArrayInstruction::Value(segments) => {
                for segment in segments.iter() {
                    self.bit_segment(segment, locals)?;
                }
                Ok(())
            }
            BitArrayInstruction::Constant(id) => self.constant(*id, output),
            BitArrayInstruction::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            BitArrayInstruction::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            BitArrayInstruction::TupleIndex { tuple, index } => {
                self.tuple_index(tuple, *index, output, locals)
            }
            BitArrayInstruction::CustomField { source, index } => {
                self.custom_field(source, *index, output, locals)
            }
            BitArrayInstruction::ListIndex { list, index: _ } => {
                self.list_index(list, output, locals)
            }
        }
    }

    fn bit_segment(
        &self,
        segment: &BitArraySegment,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        match segment {
            BitArraySegment::Int {
                value,
                bit_size: _,
                endianness: _,
            } => {
                read(value, locals)?;
            }
            BitArraySegment::EvaluatedInt {
                value,
                size,
                endianness: _,
                site,
            } => {
                read(value, locals)?;
                self.bit_size(size, locals)?;
                self.sources
                    .span(site.module(), site.span())
                    .map_err(InstructionError::Source)?;
            }
            BitArraySegment::Float {
                value,
                bit_size: _,
                endianness: _,
            } => {
                read(value, locals)?;
            }
            BitArraySegment::EvaluatedFloat {
                value,
                size,
                endianness: _,
                site,
            } => {
                read(value, locals)?;
                self.bit_size(size, locals)?;
                self.sources
                    .span(site.module(), site.span())
                    .map_err(InstructionError::Source)?;
            }
            BitArraySegment::String { value, encoding: _ } => {
                read(value, locals)?;
            }
            BitArraySegment::UtfCodepoint { value, encoding: _ } => {
                read(value, locals)?;
            }
            BitArraySegment::Bits(value) => {
                read(value, locals)?;
            }
            BitArraySegment::SizedBits { value, size, site } => {
                read(value, locals)?;
                match size {
                    BitArrayBitsSize::Fixed(_) => {}
                    BitArrayBitsSize::Evaluated(size) => self.bit_size(size, locals)?,
                }
                self.sources
                    .span(site.module(), site.span())
                    .map_err(InstructionError::Source)?;
            }
        }
        Ok(())
    }

    fn bit_size(
        &self,
        size: &BitArrayEvaluatedSize,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        read(&size.value, locals)?;
        if size.unit == 0 {
            return Err(InstructionError::ZeroBitUnit);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::{
        catalog::Catalog,
        local::LocalError,
        source::{SourceError, Sources},
        type_::Types,
    };
    use super::{
        BitArrayBitsSize, BitArrayEvaluatedSize, BitArrayInstruction, BitArraySegment,
        InstructionError, Instructions, Locals, ParamSlot, ValueType,
    };
    use crate::plan::execution::graph::{
        BitArrayLocalId, Endianness, FloatBitSize, FloatLocalId, IntLocalId, ParamLocal,
        ProfiledInstructionKind, StringEncoding, StringLocalId, UtfCodepointLocalId,
    };
    use crate::plan::{PanicSite, SourceSpan};

    #[test]
    fn every_bit_array_instruction_reads_the_declared_target_and_projection() {
        let source = r#"
pub type Box { Box(BitArray) }
const fixed = <<42>>
fn tuple(value: #(BitArray)) { value.0 }
fn custom(value: Box) { let Box(bits) = value bits }
fn first(values: List(BitArray)) { case values { [bits, ..] -> bits _ -> <<>> } }
fn apply(callback: fn() -> BitArray) { callback() }
fn build(n: Int, size: Int, decimal: Float, text: String, bits: BitArray, point: UtfCodepoint) {
  <<n, n:size(size), decimal:float, decimal:float-size(size), text:utf8,
    bits:bits, bits:bits-size(8), bits:bits-size(size), point:utf8_codepoint>>
}
pub fn main() {
  let assert <<point:utf8_codepoint>> = <<65>>
  let value = build(42, 64, 1.0, "x", fixed, point)
  <<tuple(#(value)):bits, custom(Box(value)):bits, first([value]):bits,
    apply(fn() { fixed }):bits>>
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
        let mut seen = [false; 7];
        for function in plan
            .program
            .functions
            .value_returns
            .bit_array_functions
            .iter()
        {
            for block in function.body().block_graph().blocks() {
                let mut locals = Locals::default();
                for slot in block.params() {
                    locals.define(slot, &types).unwrap();
                }
                for instruction in block.instructions() {
                    if let ProfiledInstructionKind::BitArray(value) = instruction.kind() {
                        let index = match value {
                            BitArrayInstruction::Value(_) => 0,
                            BitArrayInstruction::Constant(_) => 1,
                            BitArrayInstruction::Call { .. } => 2,
                            BitArrayInstruction::FunctionCall { .. } => 3,
                            BitArrayInstruction::TupleIndex { .. } => 4,
                            BitArrayInstruction::CustomField { .. } => 5,
                            BitArrayInstruction::ListIndex { .. } => 6,
                        };
                        assert_eq!(
                            context.bit_array(value, instruction.output(), &locals),
                            Ok(())
                        );
                        seen[index] = true;
                    }
                    locals.define(instruction.output(), &types).unwrap();
                }
            }
        }
        assert_eq!(seen, [true; 7]);
    }

    #[test]
    fn segments_reject_missing_values_bad_sizes_and_invalid_source_sites() {
        let source = r#"
pub fn main() {
  let assert <<point:utf8_codepoint>> = <<65>>
  #(42, 1.0, "x", <<42>>, point)
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
        let slots: Vec<_> = [
            (ParamLocal::Int(IntLocalId(0)), ValueType::Int),
            (ParamLocal::Float(FloatLocalId(0)), ValueType::Float),
            (ParamLocal::String(StringLocalId(0)), ValueType::String),
            (
                ParamLocal::BitArray(BitArrayLocalId(0)),
                ValueType::BitArray,
            ),
            (
                ParamLocal::UtfCodepoint(UtfCodepointLocalId(0)),
                ValueType::UtfCodepoint,
            ),
        ]
        .into_iter()
        .map(|(local, type_)| ParamSlot {
            local,
            shape: crate::plan::execution::type_::ValueShapeId(
                common
                    .value_shapes
                    .shape_types
                    .iter()
                    .position(|value| value == &type_)
                    .unwrap(),
            ),
        })
        .collect();
        let mut locals = Locals::default();
        for slot in &slots {
            locals.define(slot, &types).unwrap();
        }
        let site = PanicSite::new("example".into(), "main".into(), SourceSpan::new(0, 1));
        let size = BitArrayEvaluatedSize {
            value: IntLocalId(0),
            unit: 1,
        };
        let segments = [
            BitArraySegment::Int {
                value: IntLocalId(0),
                bit_size: 8,
                endianness: Endianness::Big,
            },
            BitArraySegment::EvaluatedInt {
                value: IntLocalId(0),
                size: size.clone(),
                endianness: Endianness::Big,
                site: site.clone(),
            },
            BitArraySegment::Float {
                value: FloatLocalId(0),
                bit_size: FloatBitSize::SixtyFour,
                endianness: Endianness::Big,
            },
            BitArraySegment::EvaluatedFloat {
                value: FloatLocalId(0),
                size: size.clone(),
                endianness: Endianness::Big,
                site: site.clone(),
            },
            BitArraySegment::String {
                value: StringLocalId(0),
                encoding: StringEncoding::Utf8,
            },
            BitArraySegment::UtfCodepoint {
                value: UtfCodepointLocalId(0),
                encoding: StringEncoding::Utf8,
            },
            BitArraySegment::Bits(BitArrayLocalId(0)),
            BitArraySegment::SizedBits {
                value: BitArrayLocalId(0),
                size: BitArrayBitsSize::Fixed(8),
                site: site.clone(),
            },
            BitArraySegment::SizedBits {
                value: BitArrayLocalId(0),
                size: BitArrayBitsSize::Evaluated(size),
                site: site.clone(),
            },
        ];
        for segment in segments {
            assert_eq!(context.bit_segment(&segment, &locals), Ok(()));
            let mut missing = segment.clone();
            let address = match &mut missing {
                BitArraySegment::Int { value, .. }
                | BitArraySegment::EvaluatedInt { value, .. } => {
                    value.0 = 99;
                    (*value).into()
                }
                BitArraySegment::Float { value, .. }
                | BitArraySegment::EvaluatedFloat { value, .. } => {
                    value.0 = 99;
                    (*value).into()
                }
                BitArraySegment::String { value, .. } => {
                    value.0 = 99;
                    (*value).into()
                }
                BitArraySegment::UtfCodepoint { value, .. } => {
                    value.0 = 99;
                    (*value).into()
                }
                BitArraySegment::Bits(value) | BitArraySegment::SizedBits { value, .. } => {
                    value.0 = 99;
                    (*value).into()
                }
            };
            assert_eq!(
                context.bit_segment(&missing, &locals),
                Err(InstructionError::Local(LocalError::Missing(address)))
            );
            assert_eq!(
                context.bit_array(
                    &BitArrayInstruction::Value(vec![missing].into()),
                    &slots[3],
                    &locals,
                ),
                Err(InstructionError::Local(LocalError::Missing(address)))
            );
        }
        assert_eq!(
            context.bit_array(
                &BitArrayInstruction::Value(Vec::new().into()),
                &slots[0],
                &locals
            ),
            Err(InstructionError::OutputType)
        );
        let bad_site = PanicSite::new("missing".into(), "main".into(), SourceSpan::new(0, 1));
        for source in [site, bad_site] {
            for unit in [0, 1] {
                for value in [IntLocalId(0), IntLocalId(99)] {
                    let size = BitArrayEvaluatedSize { value, unit };
                    let cases = [
                        BitArraySegment::EvaluatedInt {
                            value: IntLocalId(0),
                            size: size.clone(),
                            endianness: Endianness::Big,
                            site: source.clone(),
                        },
                        BitArraySegment::EvaluatedFloat {
                            value: FloatLocalId(0),
                            size: size.clone(),
                            endianness: Endianness::Big,
                            site: source.clone(),
                        },
                        BitArraySegment::SizedBits {
                            value: BitArrayLocalId(0),
                            size: BitArrayBitsSize::Evaluated(size),
                            site: source.clone(),
                        },
                    ];
                    for segment in cases {
                        let expected = if value.0 == 99 {
                            Err(InstructionError::Local(LocalError::Missing(value.into())))
                        } else if unit == 0 {
                            Err(InstructionError::ZeroBitUnit)
                        } else if source.module() == "missing" {
                            Err(InstructionError::Source(SourceError::MissingModule(
                                "missing".into(),
                            )))
                        } else {
                            Ok(())
                        };
                        assert_eq!(context.bit_segment(&segment, &locals), expected);
                    }
                }
            }
        }
    }
}
