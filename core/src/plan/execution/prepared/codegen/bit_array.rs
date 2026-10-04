use super::shape::CompiledTerminator;
use super::{Code, FunctionCodegen, ProgressOutput, tuple};
use crate::plan::execution::compiled::CompiledCheckpoint;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    BitArrayBindingPattern, BitArrayLocalId, BitArrayPatternSegment, BitArrayPatternSize,
    BitArrayPatternValue, BlockId, BoolLocalId, Edge, Endianness, IntLocalId, IntegerLiteral,
    Match, MatchEdge, MatchEdgeArgument, MatchIntPatternBinding, MatchPattern, ParamLocal,
    Signedness,
};
use crate::plan::execution::prepared::rust::Rust;
use std::collections::BTreeMap;

/// A borrowed, fixed-field match supported by generated BitArray loops.
pub(super) struct BitArrayMatch<'graph> {
    pub subject: BitArrayLocalId,
    pub fields: Vec<BitField<'graph>>,
    pub complete: bool,
    pub root_bindings: Vec<usize>,
    pub arguments: Vec<BitArgument>,
    pub success: &'graph MatchEdge,
    pub failure: &'graph Edge,
    pub bindings: BTreeMap<usize, BitBinding>,
}

pub(super) enum BitField<'graph> {
    Integer {
        pattern: &'graph BitArrayPatternValue<IntegerLiteral, MatchIntPatternBinding>,
        size: u64,
        endianness: Endianness,
        signedness: Signedness,
    },
    Range {
        pattern: &'graph BitArrayBindingPattern,
        size: Option<u64>,
        unit: u8,
    },
}

pub(super) enum BitArgument {
    Binding(usize),
    Integer(IntLocalId),
    Boolean(BoolLocalId),
    BitArray(BitArrayLocalId),
}

#[derive(Clone, Copy)]
pub(super) enum BitBinding {
    Integer { may_be_big: bool },
    BitArray,
}

impl<'graph> BitArrayMatch<'graph> {
    pub(super) fn inspect(matcher: &'graph Match) -> Option<Self> {
        let ParamLocal::BitArray(subject) = matcher.subject else {
            return None;
        };
        let mut bindings = BTreeMap::new();
        let mut fields = Vec::new();
        let mut root_bindings = Vec::new();
        let complete = inspect_pattern(
            &matcher.pattern,
            &mut bindings,
            &mut fields,
            &mut root_bindings,
        )?;
        let arguments = matcher
            .success
            .args()
            .iter()
            .map(|argument| {
                Some(match argument {
                    MatchEdgeArgument::Binding(index) => BitArgument::Binding(*index),
                    MatchEdgeArgument::Value(ParamLocal::Int(local)) => {
                        BitArgument::Integer(*local)
                    }
                    MatchEdgeArgument::Value(ParamLocal::Bool(local)) => {
                        BitArgument::Boolean(*local)
                    }
                    MatchEdgeArgument::Value(ParamLocal::BitArray(local)) => {
                        BitArgument::BitArray(*local)
                    }
                    _ => return None,
                })
            })
            .collect::<Option<Vec<_>>>()?;
        Some(Self {
            subject,
            fields,
            complete,
            root_bindings,
            arguments,
            success: &matcher.success,
            failure: &matcher.failure,
            bindings,
        })
    }
}

fn inspect_pattern<'graph>(
    pattern: &'graph MatchPattern,
    bindings: &mut BTreeMap<usize, BitBinding>,
    fields: &mut Vec<BitField<'graph>>,
    root_bindings: &mut Vec<usize>,
) -> Option<bool> {
    match pattern {
        MatchPattern::Bind(binding) => {
            bindings.insert(binding.index, BitBinding::BitArray);
            root_bindings.push(binding.index);
            Some(false)
        }
        MatchPattern::Discard => Some(false),
        MatchPattern::Alias { pattern, binding } => {
            let complete = inspect_pattern(pattern, bindings, fields, root_bindings)?;
            bindings.insert(binding.index, BitBinding::BitArray);
            root_bindings.push(binding.index);
            Some(complete)
        }
        MatchPattern::BitArray(pattern) => {
            for segment in &pattern.segments {
                match segment {
                    BitArrayPatternSegment::Int {
                        pattern,
                        size: BitArrayPatternSize::Fixed(size),
                        signedness,
                        endianness,
                    } if *size <= 64 => {
                        inspect_integer(
                            pattern,
                            *size == 64 && *signedness == Signedness::Unsigned,
                            bindings,
                        );
                        fields.push(BitField::Integer {
                            pattern,
                            size: *size,
                            signedness: *signedness,
                            endianness: *endianness,
                        });
                    }
                    BitArrayPatternSegment::Bits {
                        pattern,
                        size,
                        unit,
                    } => {
                        inspect_range(pattern, bindings);
                        let size = match size {
                            Some(BitArrayPatternSize::Fixed(size)) => Some(*size),
                            None => None,
                            _ => return None,
                        };
                        fields.push(BitField::Range {
                            pattern,
                            size,
                            unit: *unit,
                        });
                    }
                    _ => return None,
                }
            }
            Some(true)
        }
        _ => None,
    }
}

fn inspect_integer(
    pattern: &BitArrayPatternValue<IntegerLiteral, MatchIntPatternBinding>,
    may_be_big: bool,
    bindings: &mut BTreeMap<usize, BitBinding>,
) {
    match pattern {
        BitArrayPatternValue::Literal(_) | BitArrayPatternValue::Discard => {}
        BitArrayPatternValue::Bind(binding) => {
            bindings.insert(binding.binding.index, BitBinding::Integer { may_be_big });
        }
        BitArrayPatternValue::Alias { pattern, binding } => {
            inspect_integer(pattern, may_be_big, bindings);
            bindings.insert(binding.binding.index, BitBinding::Integer { may_be_big });
        }
    }
}

fn inspect_range(pattern: &BitArrayBindingPattern, bindings: &mut BTreeMap<usize, BitBinding>) {
    match pattern {
        BitArrayBindingPattern::Discard => {}
        BitArrayBindingPattern::Bind(binding) => {
            bindings.insert(binding.index, BitBinding::BitArray);
        }
        BitArrayBindingPattern::Alias { pattern, binding } => {
            inspect_range(pattern, bindings);
            bindings.insert(binding.index, BitBinding::BitArray);
        }
    }
}

impl<Graph: ExecutionGraphProfile> FunctionCodegen<'_, Graph> {
    /// Forward labels preserve shared failure/guard joins without copying blocks
    /// or adding dispatch to the entry loop. Entry back-edges remain `continue`.
    pub(super) fn bit_loop(&self, source: &mut Code) {
        for &block in self.shape.order.iter().skip(1).rev() {
            let point = self.shape.checkpoints[self.shape.start(block)];
            source.open(&format!(
                "let {} = 'block_{}: {{\n",
                self.locals(point, false),
                block.0
            ));
        }
        for (index, &block) in self.shape.order.iter().enumerate() {
            if index > 0 {
                source.close("};\n");
            }
            let body = self.shape.block(block);
            for (instruction_index, instruction) in body.instructions.iter().enumerate() {
                let index = self.shape.start(block) + instruction_index;
                self.tick(source, index, ProgressOutput::Direct);
                self.instruction(
                    source,
                    self.shape.checkpoints[index],
                    instruction,
                    ProgressOutput::Direct,
                );
                self.big_exit(
                    source,
                    self.shape.checkpoints[index],
                    instruction,
                    index + 1,
                    ProgressOutput::Direct,
                );
            }
            let index = self.shape.start(block) + body.instructions.len();
            let point = self.shape.checkpoints[index];
            if !matches!(body.terminator, CompiledTerminator::Interpreted) {
                self.tick(source, index, ProgressOutput::Direct);
            }
            self.branch(
                source,
                point,
                &body.terminator,
                self.shape.repeats || self.shape.order.last() != Some(&block),
                ProgressOutput::Direct,
                &mut |source, target, inputs| {
                    if target == self.shape.graph.entry() {
                        source.push_str(&format!(
                            "{} = {inputs};\ncontinue 'repeat;\n",
                            self.locals(self.shape.checkpoints[self.entry()], false)
                        ));
                    } else if inputs == "()" {
                        source.push_str(&format!("break 'block_{};\n", target.0));
                    } else {
                        source.push_str(&format!("break 'block_{} {inputs};\n", target.0));
                    }
                },
            );
        }
    }

    pub(super) fn bit_match(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        matcher: &BitArrayMatch<'_>,
        output: ProgressOutput<'_>,
        emit_edge: &mut impl FnMut(&mut Code, BlockId, String),
    ) {
        let types = tuple(matcher.bindings.values().map(|binding| match binding {
            BitBinding::Integer { .. } => "i128".to_owned(),
            BitBinding::BitArray => "data::compiled::bit_array::BitArrayRange".to_owned(),
        }));
        let names = tuple(
            matcher
                .bindings
                .keys()
                .map(|index| format!("_matched_{index}")),
        );
        source.open(&format!("let _matched = (|| -> Option<{types}> {{\n"));
        let input = format!("b{}_b{}", point.block.0, matcher.subject.0);
        if matcher.fields.iter().any(|field| matches!(field, BitField::Integer { pattern, .. } if impossible_integer_literal(pattern))) {
            source.push_str("None\n");
        } else {
            bit_pattern(source, matcher, &input);
            source.push_str(&format!("Some({names})\n"));
        }
        source.close("})();\n");
        source.open("match _matched {\n");
        source.open(&format!("Some({names}) => {{\n"));
        let target = matcher.success.target();
        let arguments = matcher
            .arguments
            .iter()
            .map(|argument| match argument {
                BitArgument::Binding(index) => format!("_matched_{index}"),
                BitArgument::Integer(local) => format!("b{}_i{}", point.block.0, local.0),
                BitArgument::Boolean(local) => format!("b{}_v{}", point.block.0, local.0),
                BitArgument::BitArray(local) => format!("b{}_b{}", point.block.0, local.0),
            })
            .collect::<Vec<_>>();
        let params = self.shape.graph.block(target).params();
        let inputs = tuple(
            params
                .iter()
                .zip(&arguments)
                .filter(|(slot, _)| matches!(slot.local(), ParamLocal::Int(_)))
                .map(|(_, input)| input.clone())
                .chain(
                    params
                        .iter()
                        .zip(&arguments)
                        .filter(|(slot, _)| matches!(slot.local(), ParamLocal::Bool(_)))
                        .map(|(_, input)| input.clone()),
                )
                .chain(
                    params
                        .iter()
                        .zip(&arguments)
                        .filter(|(slot, _)| matches!(slot.local(), ParamLocal::BitArray(_)))
                        .map(|(_, input)| input.clone()),
                ),
        );
        // Only selected unsigned 64-bit bindings can introduce a Big value.
        // Late failures have already discarded the entire tentative match.
        let checks = matcher
            .arguments
            .iter()
            .filter_map(|argument| match argument {
                BitArgument::Binding(index)
                    if matches!(
                        matcher.bindings[index],
                        BitBinding::Integer { may_be_big: true }
                    ) =>
                {
                    Some(format!("_matched_{index} > i128::from(i64::MAX)"))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        if !checks.is_empty() {
            source.open(&format!("if {} {{\n", checks.join(" || ")));
            let next = self.shape.start(target);
            let target = self.shape.checkpoints[next];
            source.push_str(&format!("let {} = {inputs};\n", self.locals(target, false)));
            self.save(source, target);
            let progress = output.for_kind(
                format!("data::compiled::CompiledProgress::Interpreted({next})"),
                self.shape.kind,
            );
            source.push_str(&format!("return {progress};\n"));
            source.close("}\n");
        }
        emit_edge(source, target, inputs);
        source.close("},\n");
        source.open("None => {\n");
        emit_edge(
            source,
            matcher.failure.target(),
            edge_inputs(point.block, matcher.failure),
        );
        source.close("},\n");
        source.close("}\n");
    }
}

fn impossible_integer_literal(
    pattern: &BitArrayPatternValue<IntegerLiteral, MatchIntPatternBinding>,
) -> bool {
    match pattern {
        BitArrayPatternValue::Literal(value) => i128::try_from(value.materialize()).is_err(),
        BitArrayPatternValue::Alias { pattern, .. } => impossible_integer_literal(pattern),
        _ => false,
    }
}

fn bit_pattern(source: &mut Code, matcher: &BitArrayMatch<'_>, input: &str) {
    if matcher.complete {
        source.push_str("let mut _offset = 0_usize;\n");
    }
    for (index, field) in matcher.fields.iter().enumerate() {
        match field {
            BitField::Integer {
                pattern,
                size,
                endianness,
                signedness,
            } => {
                source.push_str(&format!(
                    r#"let _field_{index} = values.integer(
    {input},
    _offset,
    {size}_usize,
    {},
    {},
)?;
"#,
                    Rust::expression(endianness),
                    Rust::expression(signedness),
                ));
                integer_pattern(source, pattern, &format!("_field_{index}"));
                source.push_str(&format!("_offset = _offset.checked_add({size}_usize)?;\n"));
            }
            BitField::Range {
                pattern,
                size,
                unit,
            } => {
                let length = match size {
                    Some(size) => format!("usize::try_from({size}_u64).ok()?"),
                    None => format!("{input}.bit_len().checked_sub(_offset)?"),
                };
                source.push_str(&format!("let _length_{index} = {length};\n"));
                if size.is_none() && *unit > 1 {
                    source.push_str(&format!(
                        "if !_length_{index}.is_multiple_of({unit}_usize) {{ return None; }}\n"
                    ));
                }
                source.push_str(&format!(
                    "let _range_{index} = {input}.slice(_offset, _length_{index})?;\n"
                ));
                range_pattern(source, pattern, &format!("_range_{index}"));
                source.push_str(&format!(
                    "_offset = _offset.checked_add(_length_{index})?;\n"
                ));
            }
        }
    }
    if matcher.complete {
        source.push_str(&format!(
            "if _offset != {input}.bit_len() {{ return None; }}\n"
        ));
    }
    for binding in &matcher.root_bindings {
        source.push_str(&format!("let _matched_{binding} = {input};\n"));
    }
}

fn integer_pattern(
    source: &mut Code,
    pattern: &BitArrayPatternValue<IntegerLiteral, MatchIntPatternBinding>,
    value: &str,
) {
    match pattern {
        BitArrayPatternValue::Literal(literal) => source.push_str(&format!(
            "if {value} != {literal}_i128 {{ return None; }}\n"
        )),
        BitArrayPatternValue::Discard => {}
        BitArrayPatternValue::Bind(binding) => source.push_str(&format!(
            "let _matched_{} = {value};\n",
            binding.binding.index
        )),
        BitArrayPatternValue::Alias { pattern, binding } => {
            integer_pattern(source, pattern, value);
            source.push_str(&format!(
                "let _matched_{} = {value};\n",
                binding.binding.index
            ));
        }
    }
}

fn range_pattern(source: &mut Code, pattern: &BitArrayBindingPattern, value: &str) {
    match pattern {
        BitArrayBindingPattern::Discard => {}
        BitArrayBindingPattern::Bind(binding) => {
            source.push_str(&format!("let _matched_{} = {value};\n", binding.index))
        }
        BitArrayBindingPattern::Alias { pattern, binding } => {
            range_pattern(source, pattern, value);
            source.push_str(&format!("let _matched_{} = {value};\n", binding.index));
        }
    }
}

fn edge_inputs(block: BlockId, edge: &Edge) -> String {
    let ints = edge.args().iter().filter_map(|local| match local {
        ParamLocal::Int(local) => Some(format!("b{}_i{}", block.0, local.0)),
        _ => None,
    });
    let bools = edge.args().iter().filter_map(|local| match local {
        ParamLocal::Bool(local) => Some(format!("b{}_v{}", block.0, local.0)),
        _ => None,
    });
    let bits = edge.args().iter().filter_map(|local| match local {
        ParamLocal::BitArray(local) => Some(format!("b{}_b{}", block.0, local.0)),
        _ => None,
    });
    tuple(ints.chain(bools).chain(bits))
}

#[cfg(test)]
mod tests {
    use super::super::shape::CompiledShape;
    use super::super::tests::emit_edge_expression;
    use super::{
        BitArgument, BitArrayMatch, BitBinding, BitField, Code, CompiledTerminator,
        FunctionCodegen, ProgressOutput, bit_pattern, inspect_pattern,
    };
    use crate::embedding::BigInt;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::{
        BitArrayBindingPattern, BitArrayLocalId, BitArrayPattern, BitArrayPatternSegment,
        BitArrayPatternSize, BitArrayPatternSizeExpr, BlockId, Endianness, IntLocalId,
        IntegerLiteral, Match, MatchEdge, MatchIntPatternBinding, MatchPattern,
        MatchPatternBinding, ParamLocal, Signedness, Terminator,
    };
    use crate::plan::execution::graph::{BitArrayPatternValue, Edge, Transfer};
    use crate::runtime::run_main;
    use std::collections::BTreeMap;
    use std::mem::discriminant;

    #[test]
    fn aliases_and_fixed_and_rest_ranges_preserve_their_exact_binding_families() {
        let source = r#"
fn walk(input: BitArray, total: Int) {
  case input {
    <<1 as first, 2:8, middle:bits-size(8), rest:bytes>> as whole ->
      case middle {
        <<value:8>> -> case whole {
          <<_:24, _:bytes>> -> walk(rest, total + first + value)
          _ -> -2
        }
        _ -> -2
      }
    <<>> -> total
    _ -> -1
  }
}
pub fn main() { walk(<<1, 2, 3>>, 0) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let shape =
            CompiledShape::inspect_bits(plan.int_function(IntFunctionId(1)).body()).unwrap();
        let function = FunctionCodegen {
            name: "walk".into(),
            shape,
        };
        let mut code = Code::default();
        function.write_code(&mut code, function.resumes_next());
        assert_eq!(
            code.as_str().split("\nfn walk_entry(").next().unwrap(),
            r#"
fn walk(
    point: usize,
    values: &mut data::compiled::bit_array::BitArrayValues,
    budget: &mut usize,
) -> data::compiled::CompiledProgress {

    const RESUME: [
        fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
        13
    ] = [
        |values, budget| CompiledResume::Exit(walk_entry((values.ints[0], values.bit_arrays[0],), values, budget)),
        walk_resume_1,
        walk_resume_2,
        walk_resume_3,
        walk_resume_4,
        walk_resume_5,
        walk_resume_6,
        walk_resume_7,
        walk_resume_8,
        walk_resume_9,
        walk_resume_10,
        walk_resume_11,
        walk_resume_12,
    ];

    let mut point = point;
    loop {
        match RESUME[point](values, budget) {
            CompiledResume::Next(next) => point = next,
            CompiledResume::Exit(progress) => return progress,
        }
    }
}
"#
        );
        let graph = plan
            .int_function(IntFunctionId(1))
            .body()
            .block_graph()
            .as_view();
        let matcher = graph
            .blocks()
            .filter_map(|block| match block.terminator() {
                Terminator::Match(matcher) => BitArrayMatch::inspect(matcher),
                _ => None,
            })
            .collect::<Vec<_>>()
            .into_iter()
            .find(|matcher| matcher.fields.len() == 4)
            .unwrap();
        assert!(matcher.complete);
        // The source root alias is forwarded as the original BitArray edge
        // argument. The field alias remains an integer match binding.
        assert_eq!(matcher.root_bindings.len(), 0);
        assert_eq!(matcher.bindings.len(), 3);
        assert_eq!(
            matcher
                .bindings
                .values()
                .filter(|binding| matches!(binding, BitBinding::Integer { may_be_big: false }))
                .count(),
            1
        );
        assert_eq!(
            matcher
                .bindings
                .values()
                .filter(|binding| matches!(binding, BitBinding::BitArray))
                .count(),
            2
        );
        let fields = matcher
            .fields
            .iter()
            .map(|field| match field {
                BitField::Integer {
                    size,
                    endianness,
                    signedness,
                    ..
                } => (*size, 0, Some((*endianness, *signedness))),
                BitField::Range { size, unit, .. } => (size.unwrap_or(u64::MAX), *unit, None),
            })
            .collect::<Vec<_>>();
        assert_eq!(
            fields,
            [
                (8, 0, Some((Endianness::Big, Signedness::Unsigned))),
                (8, 0, Some((Endianness::Big, Signedness::Unsigned))),
                (8, 1, None),
                (u64::MAX, 8, None),
            ]
        );
    }

    #[test]
    fn whole_input_and_range_aliases_keep_each_binding_on_the_same_backing() {
        let pattern = MatchPattern::Alias {
            pattern: Box::new(MatchPattern::BitArray(BitArrayPattern::new(vec![
                BitArrayPatternSegment::Bits {
                    pattern: BitArrayBindingPattern::Alias {
                        pattern: Box::new(BitArrayBindingPattern::Bind(MatchPatternBinding::new(
                            0,
                        )))
                        .into(),
                        binding: MatchPatternBinding::new(1),
                    },
                    size: None,
                    unit: 1,
                },
            ])))
            .into(),
            binding: MatchPatternBinding::new(2),
        };
        let mut bindings = BTreeMap::new();
        let mut fields = Vec::new();
        let mut roots = Vec::new();
        assert_eq!(
            inspect_pattern(&pattern, &mut bindings, &mut fields, &mut roots),
            Some(true)
        );
        assert_eq!(bindings.keys().copied().collect::<Vec<_>>(), [0, 1, 2]);
        assert_eq!(roots, [2]);
        assert_eq!(fields.len(), 1);
        assert_eq!(
            bindings.values().map(discriminant).collect::<Vec<_>>(),
            [discriminant(&BitBinding::BitArray); 3]
        );
        fields.clear();
        bindings.clear();
        roots.clear();
        let whole = MatchPattern::Bind(MatchPatternBinding::new(3));
        assert_eq!(
            inspect_pattern(&whole, &mut bindings, &mut fields, &mut roots),
            Some(false)
        );
        assert_eq!(roots, [3]);
        assert_eq!(
            inspect_pattern(
                &MatchPattern::Discard,
                &mut bindings,
                &mut fields,
                &mut roots
            ),
            Some(false)
        );
    }

    #[test]
    fn dynamic_range_sizes_and_other_pattern_families_are_not_generated() {
        let pattern =
            MatchPattern::BitArray(BitArrayPattern::new(vec![BitArrayPatternSegment::Bits {
                pattern: BitArrayBindingPattern::Discard,
                size: Some(BitArrayPatternSize::Dynamic {
                    value: BitArrayPatternSizeExpr::Local(IntLocalId(0)),
                    unit: 1,
                }),
                unit: 1,
            }]));
        assert_eq!(
            inspect_pattern(
                &pattern,
                &mut BTreeMap::new(),
                &mut Vec::new(),
                &mut Vec::new()
            ),
            None
        );
        let alias = MatchPattern::Alias {
            pattern: Box::new(pattern).into(),
            binding: MatchPatternBinding::new(0),
        };
        assert_eq!(
            inspect_pattern(
                &alias,
                &mut BTreeMap::new(),
                &mut Vec::new(),
                &mut Vec::new()
            ),
            None
        );
        assert_eq!(
            inspect_pattern(
                &MatchPattern::Nil,
                &mut BTreeMap::new(),
                &mut Vec::new(),
                &mut Vec::new()
            ),
            None
        );
    }

    #[test]
    fn a_supported_field_cannot_forward_an_unsupported_live_value() {
        let source = r#"
fn choose(input: BitArray, threshold: Float) {
  case input {
    <<value:8>> -> case threshold >. 0.0 { True -> value False -> 0 }
    _ -> 0
  }
}
pub fn main() { choose(<<7>>, 1.0) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let graph = plan
            .int_function(IntFunctionId(1))
            .body()
            .block_graph()
            .as_view();
        let matches = graph
            .blocks()
            .filter_map(|block| match block.terminator() {
                Terminator::Match(matcher) => Some(matcher),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(matches.len(), 1);
        assert!(BitArrayMatch::inspect(matches[0]).is_none());
    }

    #[test]
    fn argument_views_keep_each_supported_family_and_reject_other_subjects() {
        let source = r#"
fn read(input: BitArray, fallback: Int, flag: Bool, other: BitArray) {
  case input {
    <<value:64-signed>> -> case flag {
      True -> value + fallback
      False -> case other { <<_:8>> -> fallback _ -> value }
    }
    _ -> -1
  }
}
pub fn main() { read(<<7:64>>, 3, True, <<1>>) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let graph = plan
            .int_function(IntFunctionId(1))
            .body()
            .block_graph()
            .as_view();
        let matches = graph
            .blocks()
            .filter_map(|block| match block.terminator() {
                Terminator::Match(matcher) => Some(matcher),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(matches.len(), 2);
        let inspected = BitArrayMatch::inspect(matches[0]).unwrap();
        assert_eq!(inspected.bindings.len(), 1);
        assert_eq!(
            discriminant(&inspected.bindings[&0]),
            discriminant(&BitBinding::Integer { may_be_big: false })
        );
        let arguments = inspected
            .arguments
            .iter()
            .map(|argument| match argument {
                BitArgument::Binding(index) => (0, *index),
                BitArgument::Integer(local) => (1, local.0),
                BitArgument::Boolean(local) => (2, local.0),
                BitArgument::BitArray(local) => (3, local.0),
            })
            .collect::<Vec<_>>();
        assert_eq!(arguments, [(1, 0), (0, 0), (2, 0), (3, 1)]);

        let source = "fn read(input: List(Int)) { let assert [value, ..] = input value } pub fn main() { read([7]) }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let graph = plan
            .int_function(IntFunctionId(1))
            .body()
            .block_graph()
            .as_view();
        let matches = graph
            .blocks()
            .filter_map(|block| match block.terminator() {
                Terminator::Match(matcher) => Some(matcher),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(matches.len(), 1);
        assert!(BitArrayMatch::inspect(matches[0]).is_none());
    }

    #[test]
    fn bit_ranges_and_whole_input_aliases_emit_exact_checked_slices_and_bindings() {
        let mut matcher = Match {
            subject: ParamLocal::BitArray(BitArrayLocalId(0)),
            pattern: MatchPattern::Alias {
                pattern: Box::new(MatchPattern::BitArray(BitArrayPattern::new(vec![
                    BitArrayPatternSegment::Bits {
                        pattern: BitArrayBindingPattern::Alias {
                            pattern: Box::new(BitArrayBindingPattern::Bind(
                                MatchPatternBinding::new(0),
                            ))
                            .into(),
                            binding: MatchPatternBinding::new(1),
                        },
                        size: Some(BitArrayPatternSize::Fixed(8)),
                        unit: 1,
                    },
                ])))
                .into(),
                binding: MatchPatternBinding::new(2),
            },
            success: MatchEdge::new(
                BlockId(1),
                vec![],
                vec![],
                Transfer {
                    families: vec![].into(),
                },
            ),
            failure: Edge::new(
                BlockId(2),
                vec![],
                Transfer {
                    families: vec![].into(),
                },
            ),
        };
        let mut code = Code::default();
        bit_pattern(
            &mut code,
            &BitArrayMatch::inspect(&matcher).unwrap(),
            "input",
        );
        assert_eq!(
            code.as_str(),
            concat!(
                "let mut _offset = 0_usize;\n",
                "let _length_0 = usize::try_from(8_u64).ok()?;\n",
                "let _range_0 = input.slice(_offset, _length_0)?;\n",
                "let _matched_0 = _range_0;\n",
                "let _matched_1 = _range_0;\n",
                "_offset = _offset.checked_add(_length_0)?;\n",
                "if _offset != input.bit_len() { return None; }\n",
                "let _matched_2 = input;\n",
            )
        );
        matcher.pattern = MatchPattern::Bind(MatchPatternBinding::new(3));
        let mut code = Code::default();
        bit_pattern(
            &mut code,
            &BitArrayMatch::inspect(&matcher).unwrap(),
            "input",
        );
        assert_eq!(code.as_str(), "let _matched_3 = input;\n");
        matcher.pattern = MatchPattern::BitArray(BitArrayPattern::new(vec![
            BitArrayPatternSegment::Int {
                pattern: BitArrayPatternValue::Literal(IntegerLiteral::from(BigInt::from(1))),
                size: BitArrayPatternSize::Fixed(8),
                endianness: Endianness::Big,
                signedness: Signedness::Unsigned,
            },
            BitArrayPatternSegment::Int {
                pattern: BitArrayPatternValue::Discard,
                size: BitArrayPatternSize::Fixed(8),
                endianness: Endianness::Little,
                signedness: Signedness::Signed,
            },
            BitArrayPatternSegment::Bits {
                pattern: BitArrayBindingPattern::Discard,
                size: None,
                unit: 8,
            },
        ]));
        let mut code = Code::default();
        bit_pattern(
            &mut code,
            &BitArrayMatch::inspect(&matcher).unwrap(),
            "input",
        );
        assert_eq!(
            code.as_str(),
            concat!(
                "let mut _offset = 0_usize;\n",
                "let _field_0 = values.integer(
    input,
    _offset,
    8_usize,
    data::graph::Endianness::Big,
    data::graph::Signedness::Unsigned,
)?;\n",
                "if _field_0 != 1_i128 { return None; }\n",
                "_offset = _offset.checked_add(8_usize)?;\n",
                "let _field_1 = values.integer(
    input,
    _offset,
    8_usize,
    data::graph::Endianness::Little,
    data::graph::Signedness::Signed,
)?;\n",
                "_offset = _offset.checked_add(8_usize)?;\n",
                "let _length_2 = input.bit_len().checked_sub(_offset)?;\n",
                "if !_length_2.is_multiple_of(8_usize) { return None; }\n",
                "let _range_2 = input.slice(_offset, _length_2)?;\n",
                "_offset = _offset.checked_add(_length_2)?;\n",
                "if _offset != input.bit_len() { return None; }\n",
            )
        );
    }

    #[test]
    fn an_impossible_literal_emits_a_miss_without_an_invalid_rust_integer() {
        let source = "fn read(input: BitArray) { case input { <<value:8>> -> value _ -> 0 } } pub fn main() { read(<<7>>) }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        assert_eq!(
            run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(7.into())
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        let shape = CompiledShape::inspect_bits(body).unwrap();
        let mut matches = shape
            .graph
            .blocks()
            .filter_map(|block| match block.terminator() {
                Terminator::Match(matcher) => Some(matcher.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(matches.len(), 1);
        let mut matcher = matches.pop().unwrap();
        // Prepared pattern grammar allows an integer literal outside i128.
        // Keep the original field binding and edges, adding its literal constraint.
        matcher.pattern =
            MatchPattern::BitArray(BitArrayPattern::new(vec![BitArrayPatternSegment::Int {
                pattern: BitArrayPatternValue::Alias {
                    pattern: Box::new(BitArrayPatternValue::Literal(IntegerLiteral::from(
                        BigInt::from(1) << 128_usize,
                    )))
                    .into(),
                    binding: MatchIntPatternBinding {
                        binding: MatchPatternBinding::new(0),
                        size: None,
                    },
                },
                size: BitArrayPatternSize::Fixed(8),
                endianness: Endianness::Big,
                signedness: Signedness::Unsigned,
            }]));
        let point = shape.checkpoints[shape.start(shape.graph.entry())];
        let codegen = FunctionCodegen {
            name: "numeric_int_1".to_owned(),
            shape,
        };
        let mut code = Code::default();
        codegen.branch(
            &mut code,
            point,
            &CompiledTerminator::BitArray(BitArrayMatch::inspect(&matcher).unwrap()),
            false,
            ProgressOutput::Direct,
            &mut emit_edge_expression,
        );
        assert_eq!(
            code.as_str(),
            concat!(
                "let _matched = (|| -> Option<(i128,)> {\n",
                "    None\n",
                "})();\n",
                "match _matched {\n",
                "    Some((_matched_0,)) => {\n",
                "        (_matched_0,)\n",
                "    },\n",
                "    None => {\n",
                "        ()\n",
                "    },\n",
                "}\n",
            )
        );
    }

    #[test]
    fn bit_match_branches_preserve_live_scalar_and_range_arguments_and_terminal_exits() {
        let source = r#"
fn read(input: BitArray, fallback: Int, flag: Bool, other: BitArray) {
  case input {
    <<value:64, rest:bits>> -> case flag {
      True -> case rest { <<next:8, _:bits>> -> value + next _ -> value }
      False -> case other { <<_:8>> -> fallback _ -> -1 }
    }
    _ -> panic as "short input"
  }
}
pub fn main() { read(<<7:64, 2>>, 3, True, <<1>>) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        assert_eq!(
            run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(9.into())
        );
        let body = plan.int_function(IntFunctionId(1)).body();
        let function = FunctionCodegen {
            name: "selected".into(),
            shape: CompiledShape::inspect_bits(body).unwrap(),
        };
        let mut dispatcher = Code::default();
        function.write_code(&mut dispatcher, function.resumes_next());
        assert_eq!(
            dispatcher
                .as_str()
                .split("\nfn selected_entry(")
                .next()
                .unwrap(),
            r#"
fn selected(
    point: usize,
    values: &mut data::compiled::bit_array::BitArrayValues,
    budget: &mut usize,
) -> data::compiled::CompiledProgress {

    const RESUME: [
        fn(&mut data::compiled::bit_array::BitArrayValues, &mut usize) -> CompiledResume;
        11
    ] = [
        |values, budget| CompiledResume::Exit(selected_entry((values.ints[0], values.bools[0], values.bit_arrays[0], values.bit_arrays[1],), values, budget)),
        selected_resume_1,
        selected_resume_2,
        selected_resume_3,
        selected_resume_4,
        selected_resume_5,
        selected_resume_6,
        selected_resume_7,
        selected_resume_8,
        selected_resume_9,
        selected_resume_10,
    ];

    let mut point = point;
    loop {
        match RESUME[point](values, budget) {
            CompiledResume::Next(next) => point = next,
            CompiledResume::Exit(progress) => return progress,
        }
    }
}
"#
        );

        assert_eq!(
            dispatcher.as_str().rsplit("\nfn ").next().unwrap(),
            r#"selected_resume_10(
    values: &mut data::compiled::bit_array::BitArrayValues,
    budget: &mut usize,
) -> CompiledResume {
    let () = ();
    let _ = budget;

    values.ints.clear();
    values.ints.extend_from_slice(&[]);
    values.bools.clear();
    values.bools.extend_from_slice(&[]);
    values.bit_arrays.clear();
    values.bit_arrays.extend_from_slice(&[]);
    CompiledResume::Exit(data::compiled::CompiledProgress::Interpreted(10))
}
"#
        );
        let mut target = Code::default();
        function.write_target(&mut target, "data::function::IntFunctionId(1)");
        assert_eq!(
            target.as_str(),
            r#"data::compiled::CompiledFunction {
    function: data::function::IntFunctionId(1),
    implementation: data::compiled::CompiledImplementation::BitArray(data::compiled::BitArrayImplementation {
        entry: 0,
        checkpoints: data::Storage::Static(&[
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(0),
                instruction: 0,
                ints: 1,
                bools: 1,
                bit_arrays: 2,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(1),
                instruction: 0,
                ints: 2,
                bools: 1,
                bit_arrays: 2,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(2),
                instruction: 0,
                ints: 1,
                bools: 0,
                bit_arrays: 1,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(3),
                instruction: 0,
                ints: 2,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(3),
                instruction: 1,
                ints: 3,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(4),
                instruction: 0,
                ints: 1,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(5),
                instruction: 0,
                ints: 1,
                bools: 0,
                bit_arrays: 1,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(6),
                instruction: 0,
                ints: 1,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(7),
                instruction: 0,
                ints: 0,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(7),
                instruction: 1,
                ints: 1,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(8),
                instruction: 0,
                ints: 0,
                bools: 0,
                bit_arrays: 0,
                int_lists: 0,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
        ]),
        run: selected,
    }),
},
"#
        );
        let expected = [
            (
                0,
                r#"let _matched = (|| -> Option<(i128, data::compiled::bit_array::BitArrayRange,)> {
    let mut _offset = 0_usize;
    let _field_0 = values.integer(
        b0_b0,
        _offset,
        64_usize,
        data::graph::Endianness::Big,
        data::graph::Signedness::Unsigned,
    )?;
    let _matched_0 = _field_0;
    _offset = _offset.checked_add(64_usize)?;
    let _length_1 = b0_b0.bit_len().checked_sub(_offset)?;
    let _range_1 = b0_b0.slice(_offset, _length_1)?;
    let _matched_1 = _range_1;
    _offset = _offset.checked_add(_length_1)?;
    if _offset != b0_b0.bit_len() { return None; }
    Some((_matched_0, _matched_1,))
})();
match _matched {
    Some((_matched_0, _matched_1,)) => {
        if _matched_0 > i128::from(i64::MAX) {
            let (b1_i0, b1_i1, b1_v0, b1_b0, b1_b1,) = (b0_i0, _matched_0, b0_v0, b0_b1, _matched_1,);

            values.ints.clear();
            values.ints.extend_from_slice(&[b1_i0, b1_i1]);
            values.bools.clear();
            values.bools.extend_from_slice(&[b1_v0]);
            values.bit_arrays.clear();
            values.bit_arrays.extend_from_slice(&[b1_b0, b1_b1]);
            return data::compiled::CompiledProgress::Interpreted(1);
        }
        (b0_i0, _matched_0, b0_v0, b0_b1, _matched_1,)
    },
    None => {
        ()
    },
}
"#,
            ),
            (
                8,
                r#"
values.ints.clear();
values.ints.extend_from_slice(&[]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
values.bit_arrays.clear();
values.bit_arrays.extend_from_slice(&[]);
data::compiled::CompiledProgress::Interpreted(10)
"#,
            ),
            (
                1,
                r#"if b1_v0 {
    (b1_i1, b1_b1,)
} else {
    (b1_i0, b1_b0,)
}
"#,
            ),
            (
                5,
                r#"let _matched = (|| -> Option<()> {
    let mut _offset = 0_usize;
    let _field_0 = values.integer(
        b5_b0,
        _offset,
        8_usize,
        data::graph::Endianness::Big,
        data::graph::Signedness::Unsigned,
    )?;
    _offset = _offset.checked_add(8_usize)?;
    if _offset != b5_b0.bit_len() { return None; }
    Some(())
})();
match _matched {
    Some(()) => {
        (b5_i0,)
    },
    None => {
        ()
    },
}
"#,
            ),
            (
                7,
                r#"
values.ints.clear();
values.ints.extend_from_slice(&[b7_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
values.bit_arrays.clear();
values.bit_arrays.extend_from_slice(&[]);
data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(3))
"#,
            ),
            (
                6,
                r#"
values.ints.clear();
values.ints.extend_from_slice(&[b6_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
values.bit_arrays.clear();
values.bit_arrays.extend_from_slice(&[]);
data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(2))
"#,
            ),
            (
                2,
                r#"let _matched = (|| -> Option<(i128,)> {
    let mut _offset = 0_usize;
    let _field_0 = values.integer(
        b2_b0,
        _offset,
        8_usize,
        data::graph::Endianness::Big,
        data::graph::Signedness::Unsigned,
    )?;
    let _matched_0 = _field_0;
    _offset = _offset.checked_add(8_usize)?;
    let _length_1 = b2_b0.bit_len().checked_sub(_offset)?;
    let _range_1 = b2_b0.slice(_offset, _length_1)?;
    _offset = _offset.checked_add(_length_1)?;
    if _offset != b2_b0.bit_len() { return None; }
    Some((_matched_0,))
})();
match _matched {
    Some((_matched_0,)) => {
        (b2_i0, _matched_0,)
    },
    None => {
        (b2_i0,)
    },
}
"#,
            ),
            (
                4,
                r#"
values.ints.clear();
values.ints.extend_from_slice(&[b4_i0]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
values.bit_arrays.clear();
values.bit_arrays.extend_from_slice(&[]);
data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(1))
"#,
            ),
            (
                3,
                r#"
values.ints.clear();
values.ints.extend_from_slice(&[b3_i0, b3_i1, b3_i2]);
values.bools.clear();
values.bools.extend_from_slice(&[]);
values.bit_arrays.clear();
values.bit_arrays.extend_from_slice(&[]);
data::compiled::CompiledProgress::Complete(data::graph::BlockGraphExitId(0))
"#,
            ),
        ];
        assert_eq!(function.shape.order.len(), expected.len());
        for (&block, (expected_block, expected_source)) in function.shape.order.iter().zip(expected)
        {
            assert_eq!(block, BlockId(expected_block));
            let body = function.shape.block(block);
            let point =
                function.shape.checkpoints[function.shape.start(block) + body.instructions.len()];
            let mut output = Code::default();
            function.branch(
                &mut output,
                point,
                &body.terminator,
                false,
                ProgressOutput::Direct,
                &mut emit_edge_expression,
            );
            assert_eq!(output.as_str(), expected_source, "block {block:?}");
        }
    }
}
