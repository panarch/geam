use super::shape::{CompiledEdge, CompiledInstruction, CompiledTerminator};
use super::{Code, FunctionCodegen, ProgressOutput, Rust, length_expression, tuple};
use crate::plan::execution::compiled::CompiledCheckpoint;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    BlockId, BoolTest, IntListLocalId, IntLocalId, IntegerLiteral, ListLocal, Match,
    MatchEdgeArgument, MatchPattern, MatchPatternListTail, ParamLocal,
};
use crate::plan::execution::type_::IntListTypeId;

pub(super) enum IntListInstruction<'graph> {
    Value {
        output: IntListLocalId,
        type_id: IntListTypeId,
        elements: &'graph [IntLocalId],
    },
    Spread {
        output: IntListLocalId,
        type_id: IntListTypeId,
        elements: &'graph [IntLocalId],
        tail: IntListLocalId,
    },
    Index {
        output: IntLocalId,
        list: IntListLocalId,
        index: usize,
    },
    Tail {
        output: IntListLocalId,
        type_id: IntListTypeId,
        list: IntListLocalId,
        count: usize,
    },
}

pub(super) enum IntListTest {
    Length {
        list: IntListLocalId,
        length: usize,
        at_least: bool,
    },
    Equal {
        left: IntListLocalId,
        right: IntListLocalId,
        negate: bool,
    },
}

/// A preparation-local view of the supported canonical pattern and its
/// selected bindings. It is never retained in an execution program.
pub(super) struct IntListMatch<'graph> {
    pub matcher: &'graph Match,
    pub subject: IntListLocalId,
    pub type_id: IntListTypeId,
    pub length: Option<(usize, bool)>,
    pub elements: Vec<IntListElement<'graph>>,
    pub lists: Vec<(usize, Option<usize>)>,
}

pub(super) struct IntListElement<'graph> {
    pub literal: Option<&'graph IntegerLiteral>,
    pub bindings: Vec<usize>,
}

impl IntListTest {
    pub(super) fn inspect(test: &BoolTest) -> Option<Self> {
        Some(match test {
            BoolTest::ListLengthEquals {
                value: ListLocal::Int { local, .. },
                length,
            } => Self::Length {
                list: *local,
                length: *length,
                at_least: false,
            },
            BoolTest::ListLengthAtLeast {
                value: ListLocal::Int { local, .. },
                length,
            } => Self::Length {
                list: *local,
                length: *length,
                at_least: true,
            },
            BoolTest::Equal {
                left: ParamLocal::List(ListLocal::Int { local: left, .. }),
                right: ParamLocal::List(ListLocal::Int { local: right, .. }),
            } => Self::Equal {
                left: *left,
                right: *right,
                negate: false,
            },
            BoolTest::NotEqual {
                left: ParamLocal::List(ListLocal::Int { local: left, .. }),
                right: ParamLocal::List(ListLocal::Int { local: right, .. }),
            } => Self::Equal {
                left: *left,
                right: *right,
                negate: true,
            },
            _ => return None,
        })
    }
}

impl<'graph> IntListMatch<'graph> {
    pub(super) fn inspect(matcher: &'graph Match) -> Option<Self> {
        let ParamLocal::List(ListLocal::Int { local, type_id }) = matcher.subject() else {
            return None;
        };
        let mut view = Self {
            matcher,
            subject: *local,
            type_id: *type_id,
            length: None,
            elements: Vec::new(),
            lists: Vec::new(),
        };
        view.pattern(matcher.pattern())?;
        Some(view)
    }

    pub(super) fn selected(&self, index: usize) -> bool {
        self.matcher.success().args().iter().any(
            |argument| matches!(argument, MatchEdgeArgument::Binding(binding) if *binding == index),
        )
    }

    fn pattern(&mut self, pattern: &'graph MatchPattern) -> Option<()> {
        match pattern {
            MatchPattern::Bind(binding) => self.lists.push((binding.index(), None)),
            MatchPattern::Discard => {}
            MatchPattern::Alias { pattern, binding } => {
                self.pattern(pattern)?;
                self.lists.push((binding.index(), None));
            }
            MatchPattern::List(pattern) => {
                self.length = Some((pattern.elements().len(), pattern.tail().is_some()));
                for pattern in pattern.elements() {
                    self.elements.push(IntListElement::inspect(pattern)?);
                }
                if let Some(MatchPatternListTail::Bind(binding)) = pattern.tail() {
                    self.lists
                        .push((binding.index(), Some(pattern.elements().len())));
                }
            }
            _ => return None,
        }
        Some(())
    }
}

impl<'graph> IntListElement<'graph> {
    fn inspect(pattern: &'graph MatchPattern) -> Option<Self> {
        let mut element = Self {
            literal: None,
            bindings: Vec::new(),
        };
        element.pattern(pattern)?;
        Some(element)
    }

    fn pattern(&mut self, pattern: &'graph MatchPattern) -> Option<()> {
        match pattern {
            MatchPattern::Int(literal) => self.literal = Some(literal),
            MatchPattern::Discard => {}
            MatchPattern::Bind(binding) => self.bindings.push(binding.index()),
            MatchPattern::Alias { pattern, binding } => {
                self.pattern(pattern)?;
                self.bindings.push(binding.index());
            }
            _ => return None,
        }
        Some(())
    }
}

impl<Graph: ExecutionGraphProfile> FunctionCodegen<'_, Graph> {
    pub(super) fn list_preflight(
        &self,
        source: &mut Code,
        index: usize,
        instruction: &CompiledInstruction<'_>,
        output: ProgressOutput<'_>,
    ) {
        let CompiledInstruction::IntList(IntListInstruction::Index {
            output: local,
            list,
            index: element,
        }) = instruction
        else {
            return;
        };
        let point = self.shape.checkpoints[index];
        source.open(&format!(
            "let b{}_i{} = match _lists.index(&b{}_l{}, {element}) {{\n",
            point.block.0, local.0, point.block.0, list.0
        ));
        source.push_str("Some(value) => value,\n");
        source.open("None => {\n");
        self.interpreted(source, index, true, output);
        source.close("}\n");
        source.close("};\n");
    }

    pub(super) fn list_instruction(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        instruction: &IntListInstruction<'_>,
    ) {
        match instruction {
            IntListInstruction::Value {
                output,
                type_id,
                elements,
            } => source.push_str(&format!(
                "let b{}_l{} = _lists.value({}, &[{}]);\n",
                point.block.0,
                output.0,
                Rust::expression(type_id),
                list_elements(point.block, elements),
            )),
            IntListInstruction::Spread {
                output,
                type_id,
                elements,
                tail,
            } => source.push_str(&format!(
                "let b{}_l{} = _lists.prepend({}, &[{}], &b{}_l{});\n",
                point.block.0,
                output.0,
                Rust::expression(type_id),
                list_elements(point.block, elements),
                point.block.0,
                tail.0,
            )),
            // Index is read before charging its step so an unrepresentable
            // head returns to the unexecuted canonical instruction.
            IntListInstruction::Index { .. } => {}
            IntListInstruction::Tail {
                output,
                type_id,
                list,
                count,
            } => source.push_str(&format!(
                "let b{}_l{} = _lists.tail(&b{}_l{}, {}, {count});\n",
                point.block.0,
                output.0,
                point.block.0,
                list.0,
                Rust::expression(type_id)
            )),
        }
    }

    pub(super) fn list_preflight_terminator(
        &self,
        source: &mut Code,
        index: usize,
        terminator: &CompiledTerminator<'_>,
    ) {
        let CompiledTerminator::Match(view) = terminator else {
            return;
        };
        let point = self.shape.checkpoints[index];
        let subject = format!("b{}_l{}", point.block.0, view.subject.0);
        let length_guard = view
            .length
            .filter(|&(length, at_least)| length != 0 || !at_least);
        source.push_str("let _matched = ");
        if let Some((length, at_least)) = length_guard {
            source.open(&format!(
                "if {} {{\n",
                length_expression(&subject, length, at_least)
            ));
        } else {
            source.open("{\n");
        }
        source.open("'pattern: {\n");
        let reads = view
            .elements
            .iter()
            .rposition(|element| {
                element.literal.is_some()
                    || element.bindings.iter().any(|&index| view.selected(index))
            })
            .map_or(0, |index| index + 1);
        if reads != 0 {
            source.push_str(&format!(
                "let mut _reader = _lists.prefix(&{subject}, {reads});\n"
            ));
        }
        let mut outputs = Vec::new();
        for (element_index, element) in view.elements.iter().take(reads).enumerate() {
            source.open(&format!(
                "let Some(_head{element_index}) = _reader.next() else {{\n"
            ));
            source.push_str("break 'pattern Ok(None);\n");
            source.close("};\n");
            if let Some(literal) = element.literal {
                source.open(&format!(
                    "if !_head{element_index}.matches_literal(&{}) {{\n",
                    Rust::expression(literal)
                ));
                source.push_str("break 'pattern Ok(None);\n");
                source.close("}\n");
            }
            if element.bindings.iter().any(|&index| view.selected(index)) {
                source.open(&format!(
                    "let Some(_small{element_index}) = _head{element_index}.small() else {{\n"
                ));
                source.push_str("break 'pattern Err(());\n");
                source.close("};\n");
                for &binding in &element.bindings {
                    if view.selected(binding) {
                        source.push_str(&format!("let m{binding} = _small{element_index};\n"));
                        outputs.push(format!("m{binding}"));
                    }
                }
            }
        }
        source.push_str(&format!("break 'pattern Ok(Some({}));\n", tuple(outputs)));
        source.close("}\n");
        if length_guard.is_some() {
            source.alternative("} else {\n");
            source.push_str("Ok(None)\n");
        }
        source.close("};\n");
    }

    pub(super) fn match_branch(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        view: &IntListMatch<'_>,
        returning: bool,
        output: ProgressOutput<'_>,
        emit_edge: &mut impl FnMut(&mut Code, CompiledEdge<'_>),
    ) {
        let bindings = tuple(
            view.elements
                .iter()
                .flat_map(|element| element.bindings.iter().copied())
                .filter(|&index| view.selected(index))
                .map(|index| format!("m{index}")),
        );
        source.open("match _matched {\n");
        source.open(&format!("Ok(Some({bindings})) => {{\n"));
        source.push_str("*budget -= 1;\n");
        for &(index, tail) in &view.lists {
            if !view.selected(index) {
                continue;
            }
            let subject = format!("b{}_l{}", point.block.0, view.subject.0);
            let expression = match tail {
                // Alias bindings name the same source handle in edge_inputs.
                // Only additional retained edge uses clone that handle.
                None => continue,
                Some(count) => format!(
                    "_lists.tail(&{subject}, {}, {count})",
                    Rust::expression(&view.type_id)
                ),
            };
            source.push_str(&format!("let m{index} = {expression};\n"));
        }
        emit_edge(source, CompiledEdge::Match(view.matcher.success()));
        source.close("},\n");
        source.open("Ok(None) => {\n");
        source.push_str("*budget -= 1;\n");
        emit_edge(source, CompiledEdge::Ordinary(view.matcher.failure()));
        source.close("},\n");
        source.open("Err(()) => {\n");
        self.interpreted(
            source,
            self.shape.start(point.block) + point.instruction,
            returning,
            output,
        );
        source.close("}\n");
        source.close("}\n");
    }
}

fn list_elements(block: BlockId, elements: &[IntLocalId]) -> String {
    elements
        .iter()
        .map(|element| format!("b{}_i{} as i64", block.0, element.0))
        .collect::<Vec<_>>()
        .join(", ")
}

pub(super) fn test_expression(block: BlockId, test: &IntListTest) -> String {
    match test {
        IntListTest::Length {
            list,
            length,
            at_least,
        } => length_expression(&format!("b{}_l{}", block.0, list.0), *length, *at_least),
        IntListTest::Equal {
            left,
            right,
            negate,
        } => {
            let negate = if *negate { "!" } else { "" };
            format!(
                "{negate}_lists.equal(&b{}_l{}, &b{}_l{})",
                block.0, left.0, block.0, right.0
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::{CompiledEdge, CompiledShape};
    use super::{
        Code, CompiledTerminator, FunctionCodegen, IntListMatch, IntListTest, ProgressOutput, Rust,
        test_expression,
    };
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::{
        BlockId, Edge, IntListLocalId, IntLocalId, IntegerLiteral, ListLocal, Match, MatchEdge,
        MatchEdgeArgument, MatchPattern, MatchPatternBinding, MatchPatternList,
        MatchPatternListTail, ParamLocal, Transfer,
    };
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::execution::type_::{IntListTypeId, ListTypeId};
    use num_bigint::BigInt;

    #[test]
    fn list_constructors_emit_the_exact_small_prefix_and_existing_storage_calls() {
        let typed = crate::compile_typed_module("example", "src/example.gleam",
            "fn create(first: Int, second: Int, tail: List(Int)) { let empty: List(Int) = [] let pair = [first, second] [first, second, ..tail] } pub fn main() { create(7, -9, []) }",
        ).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let id = plan.int_list_function_id(1);
        let function = FunctionCodegen {
            name: "int_list_int_list_1".to_owned(),
            shape: CompiledShape::inspect(plan.int_list_function(id).body()).unwrap(),
        };
        let mut source = Code::default();
        let block = function.shape.block(BlockId(0));
        for (index, instruction) in block.instructions.iter().enumerate() {
            function.instruction(
                &mut source,
                function.shape.checkpoints[index],
                instruction,
                ProgressOutput::Direct,
            );
        }
        assert_eq!(
            source.as_str(),
            r#"
let b0_l1 = _lists.value(data::type_::IntListTypeId {
    list_type: data::type_::ListTypeId(0),
}, &[]);
let b0_l2 = _lists.value(data::type_::IntListTypeId {
    list_type: data::type_::ListTypeId(0),
}, &[b0_i0 as i64, b0_i1 as i64]);
let b0_l3 = _lists.prepend(data::type_::IntListTypeId {
    list_type: data::type_::ListTypeId(0),
}, &[b0_i0 as i64, b0_i1 as i64], &b0_l0);
"#
            .trim_start_matches('\n')
        );
        let mut source = Code::default();
        function.write_target(&mut source, &Rust::expression(&id));
        assert_eq!(source.as_str(), r#"
data::compiled::CompiledFunction {
    function: data::function::IntListFunctionId {
        index: 1,
        type_id: data::type_::IntListTypeId {
            list_type: data::type_::ListTypeId(0),
        },
    },
    implementation: data::compiled::CompiledImplementation::IntList(data::compiled::IntListImplementation {
        entry: 0,
        checkpoints: data::Storage::Static(&[
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(0),
                instruction: 0,
                ints: 2,
                bools: 0,
                bit_arrays: 0,
                int_lists: 1,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(0),
                instruction: 1,
                ints: 2,
                bools: 0,
                bit_arrays: 0,
                int_lists: 2,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(0),
                instruction: 2,
                ints: 2,
                bools: 0,
                bit_arrays: 0,
                int_lists: 3,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
            data::compiled::CompiledCheckpoint {
                block: data::graph::BlockId(0),
                instruction: 3,
                ints: 2,
                bools: 0,
                bit_arrays: 0,
                int_lists: 4,
                strings: 0,
                customs: 0,
                custom_lists: 0,
                int_functions: 0,
                bool_functions: 0,
            },
        ]),
        run: int_list_int_list_1,
    }),
},
"#.trim_start_matches('\n'));
    }

    #[test]
    fn pattern_views_keep_aliases_selected_bindings_and_the_actual_read_prefix() {
        let type_id = IntListTypeId {
            list_type: ListTypeId(0),
        };
        let subject = ParamLocal::List(ListLocal::Int {
            local: IntListLocalId(0),
            type_id,
        });
        let matcher = Match {
            subject: subject.clone(),
            pattern: MatchPattern::Alias {
                binding: MatchPatternBinding::new(3),
                pattern: Box::new(MatchPattern::List(MatchPatternList::new(
                    vec![
                        MatchPattern::Alias {
                            binding: MatchPatternBinding::new(1),
                            pattern: Box::new(MatchPattern::Alias {
                                binding: MatchPatternBinding::new(0),
                                pattern: Box::new(MatchPattern::Int(IntegerLiteral::from(
                                    BigInt::from(1),
                                )))
                                .into(),
                            })
                            .into(),
                        },
                        MatchPattern::Discard,
                    ],
                    Some(MatchPatternListTail::Bind(MatchPatternBinding::new(2))),
                )))
                .into(),
            },
            success: MatchEdge::new(
                BlockId(1),
                vec![
                    MatchEdgeArgument::Binding(0),
                    MatchEdgeArgument::Binding(3),
                    MatchEdgeArgument::Value(subject),
                ],
                vec![0, 3],
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
        let view = IntListMatch::inspect(&matcher).unwrap();
        assert_eq!(view.subject, IntListLocalId(0));
        assert_eq!(view.type_id, type_id);
        assert_eq!(view.length, Some((2, true)));
        assert_eq!(view.lists, [(2, Some(2)), (3, None)]);
        assert_eq!(view.elements[0].bindings, [0, 1]);
        assert!(view.elements[1].bindings.is_empty());
        assert_eq!(
            view.elements[0].literal.unwrap().materialize(),
            BigInt::from(1)
        );
        assert!(!view.selected(1));
        assert!(!view.selected(2));

        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "fn head(values: List(Int)) { let assert [first, ..] = values first } pub fn main() { head([1]) }",
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        let point = shape.checkpoints[shape.start(shape.graph.entry())];
        assert_eq!((point.ints, point.bools, point.int_lists), (0, 0, 1));
        let function = FunctionCodegen {
            name: "head".into(),
            shape,
        };
        let terminator = CompiledTerminator::Match(view);
        let mut source = Code::default();
        function.preflight_terminator(&mut source, function.shape.start(BlockId(0)), &terminator);
        assert_eq!(
            source.as_str(),
            r#"
let _matched = if b0_l0.len() >= 2 {
    'pattern: {
        let mut _reader = _lists.prefix(&b0_l0, 1);
        let Some(_head0) = _reader.next() else {
            break 'pattern Ok(None);
        };
        if !_head0.matches_literal(&data::graph::IntegerLiteral {
            sign: data::Sign::Plus,
            digits: data::Storage::Static(&[
                1,
            ]),
        }) {
            break 'pattern Ok(None);
        }
        let Some(_small0) = _head0.small() else {
            break 'pattern Err(());
        };
        let m0 = _small0;
        break 'pattern Ok(Some((m0,)));
    }
} else {
    Ok(None)
};
"#
            .trim_start_matches('\n')
        );
        let view = IntListMatch::inspect(&matcher).unwrap();
        let mut branch = Code::default();
        function.match_branch(
            &mut branch,
            point,
            &view,
            false,
            ProgressOutput::Direct,
            &mut |source, _| source.push_str("edge;\n"),
        );
        assert_eq!(
            branch.as_str(),
            r#"
match _matched {
    Ok(Some((m0,))) => {
        *budget -= 1;
        edge;
    },
    Ok(None) => {
        *budget -= 1;
        edge;
    },
    Err(()) => {

        values.ints.clear();
        values.ints.extend_from_slice(&[]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        values.int_lists.clear();
        values.int_lists.extend([b0_l0]);
        data::compiled::CompiledProgress::Interpreted(0)
    }
}
"#
            .trim_start_matches('\n')
        );
    }

    #[test]
    fn irrefutable_whole_list_patterns_need_no_length_guard_or_element_reader() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "fn head(values: List(Int)) { let assert [first, ..] = values first } pub fn main() { head([1]) }",
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        let function = FunctionCodegen {
            name: "head".into(),
            shape,
        };
        for (pattern, expected_lists, expected_length) in [
            (
                MatchPattern::Bind(MatchPatternBinding::new(0)),
                vec![(0, None)],
                None,
            ),
            (MatchPattern::Discard, vec![], None),
            (
                MatchPattern::List(MatchPatternList::new(
                    vec![],
                    Some(MatchPatternListTail::Ignore),
                )),
                vec![],
                Some((0, true)),
            ),
        ] {
            let matcher = Match {
                subject: ParamLocal::List(ListLocal::Int {
                    local: IntListLocalId(0),
                    type_id: IntListTypeId {
                        list_type: ListTypeId(0),
                    },
                }),
                pattern,
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
            let view = IntListMatch::inspect(&matcher).unwrap();
            assert_eq!(view.length, expected_length);
            assert_eq!(view.elements.len(), 0);
            assert_eq!(view.lists, expected_lists);
            let mut source = Code::default();
            function.preflight_terminator(
                &mut source,
                function.shape.start(BlockId(0)),
                &CompiledTerminator::Match(view),
            );
            assert_eq!(
                source.as_str(),
                r#"
let _matched = {
    'pattern: {
        break 'pattern Ok(Some(()));
    }
};
"#
                .trim_start_matches('\n')
            );
        }
    }

    #[test]
    fn pattern_classification_rejects_other_subjects_and_unsupported_pattern_kinds() {
        let subject = ParamLocal::List(ListLocal::Int {
            local: IntListLocalId(0),
            type_id: IntListTypeId {
                list_type: ListTypeId(0),
            },
        });
        for (subject, pattern) in [
            (
                ParamLocal::Int(IntLocalId(0)),
                MatchPattern::Int(IntegerLiteral::from(BigInt::from(1))),
            ),
            (
                subject.clone(),
                MatchPattern::Int(IntegerLiteral::from(BigInt::from(1))),
            ),
            (
                subject.clone(),
                MatchPattern::Alias {
                    pattern: Box::new(MatchPattern::Float(1.0)).into(),
                    binding: MatchPatternBinding::new(0),
                },
            ),
            (
                subject,
                MatchPattern::List(MatchPatternList::new(
                    vec![MatchPattern::Alias {
                        pattern: Box::new(MatchPattern::Float(1.0)).into(),
                        binding: MatchPatternBinding::new(0),
                    }],
                    None,
                )),
            ),
        ] {
            let matcher = Match {
                subject,
                pattern,
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
            assert!(IntListMatch::inspect(&matcher).is_none());
        }
    }

    #[test]
    fn list_conditions_emit_exact_length_and_borrowed_equality_expressions() {
        let block = BlockId(3);
        for (length, at_least, expected) in [
            (4, false, "b3_l2.len() == 4"),
            (4, true, "b3_l2.len() >= 4"),
            (0, false, "b3_l2.is_empty()"),
            (0, true, "true"),
            (1, true, "!b3_l2.is_empty()"),
        ] {
            assert_eq!(
                test_expression(
                    block,
                    &IntListTest::Length {
                        list: IntListLocalId(2),
                        length,
                        at_least,
                    }
                ),
                expected
            );
        }
        for (negate, expected) in [
            (false, "_lists.equal(&b3_l2, &b3_l5)"),
            (true, "!_lists.equal(&b3_l2, &b3_l5)"),
        ] {
            assert_eq!(
                test_expression(
                    block,
                    &IntListTest::Equal {
                        left: IntListLocalId(2),
                        right: IntListLocalId(5),
                        negate,
                    }
                ),
                expected
            );
        }
    }

    #[test]
    fn list_conditions_have_exact_views_and_non_list_conditions_stay_outside_the_protocol() {
        use crate::plan::execution::graph::{BoolLocalId, BoolTest};
        let left = ListLocal::Int {
            local: IntListLocalId(0),
            type_id: IntListTypeId {
                list_type: ListTypeId(0),
            },
        };
        let right = ListLocal::Int {
            local: IntListLocalId(1),
            type_id: IntListTypeId {
                list_type: ListTypeId(0),
            },
        };
        for (test, expected) in [
            (
                BoolTest::ListLengthEquals {
                    value: left.clone(),
                    length: 3,
                },
                ("length", 0, 3, false),
            ),
            (
                BoolTest::ListLengthAtLeast {
                    value: left.clone(),
                    length: 2,
                },
                ("length", 0, 2, true),
            ),
            (
                BoolTest::Equal {
                    left: ParamLocal::List(left.clone()),
                    right: ParamLocal::List(right.clone()),
                },
                ("equal", 0, 1, false),
            ),
            (
                BoolTest::NotEqual {
                    left: ParamLocal::List(left),
                    right: ParamLocal::List(right),
                },
                ("equal", 0, 1, true),
            ),
        ] {
            let actual = IntListTest::inspect(&test).map(|view| match view {
                IntListTest::Length {
                    list,
                    length,
                    at_least,
                } => ("length", list.0, length, at_least),
                IntListTest::Equal {
                    left,
                    right,
                    negate,
                } => ("equal", left.0, right.0, negate),
            });
            assert_eq!(actual, Some(expected));
        }
        assert!(IntListTest::inspect(&BoolTest::Not(BoolLocalId(0))).is_none());
    }

    #[test]
    fn skipped_heads_selected_tails_and_whole_list_aliases_emit_exactly_their_work() {
        let typed = crate::compile_typed_module("example", "src/example.gleam",
            "fn head(values: List(Int)) { let assert [first, ..] = values first } pub fn main() { head([1]) }").unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let function = FunctionCodegen {
            name: "head".into(),
            shape: CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap(),
        };
        let type_id = IntListTypeId {
            list_type: ListTypeId(0),
        };
        let mut emit_edge = |source: &mut Code, _: CompiledEdge<'_>| source.push_str("edge;\n");
        for (selected, expected) in [
            (
                vec![1, 3],
                r#"
match _matched {
    Ok(Some((m1,))) => {
        *budget -= 1;
        edge;
    },
    Ok(None) => {
        *budget -= 1;
        edge;
    },
    Err(()) => {

        values.ints.clear();
        values.ints.extend_from_slice(&[]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        values.int_lists.clear();
        values.int_lists.extend([b0_l0]);
        data::compiled::CompiledProgress::Interpreted(0)
    }
}
"#,
            ),
            (
                vec![1, 2, 3],
                r#"
match _matched {
    Ok(Some((m1,))) => {
        *budget -= 1;
        let m2 = _lists.tail(&b0_l0, data::type_::IntListTypeId {
            list_type: data::type_::ListTypeId(0),
        }, 2);
        edge;
    },
    Ok(None) => {
        *budget -= 1;
        edge;
    },
    Err(()) => {

        values.ints.clear();
        values.ints.extend_from_slice(&[]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        values.int_lists.clear();
        values.int_lists.extend([b0_l0]);
        data::compiled::CompiledProgress::Interpreted(0)
    }
}
"#,
            ),
        ] {
            let matcher = Match {
                subject: ParamLocal::List(ListLocal::Int {
                    local: IntListLocalId(0),
                    type_id,
                }),
                pattern: MatchPattern::Alias {
                    binding: MatchPatternBinding::new(3),
                    pattern: Box::new(MatchPattern::List(MatchPatternList::new(
                        vec![
                            MatchPattern::Discard,
                            MatchPattern::Alias {
                                binding: MatchPatternBinding::new(4),
                                pattern: Box::new(MatchPattern::Bind(MatchPatternBinding::new(1)))
                                    .into(),
                            },
                        ],
                        Some(MatchPatternListTail::Bind(MatchPatternBinding::new(2))),
                    )))
                    .into(),
                },
                success: MatchEdge::new(
                    BlockId(1),
                    selected
                        .iter()
                        .copied()
                        .map(MatchEdgeArgument::Binding)
                        .collect::<Vec<_>>(),
                    selected,
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
            let view = IntListMatch::inspect(&matcher).unwrap();
            let mut code = Code::default();
            function.preflight_terminator(&mut code, 0, &CompiledTerminator::Match(view));
            assert_eq!(
                code.as_str(),
                r#"
let _matched = if b0_l0.len() >= 2 {
    'pattern: {
        let mut _reader = _lists.prefix(&b0_l0, 2);
        let Some(_head0) = _reader.next() else {
            break 'pattern Ok(None);
        };
        let Some(_head1) = _reader.next() else {
            break 'pattern Ok(None);
        };
        let Some(_small1) = _head1.small() else {
            break 'pattern Err(());
        };
        let m1 = _small1;
        break 'pattern Ok(Some((m1,)));
    }
} else {
    Ok(None)
};
"#
                .trim_start_matches('\n')
            );
            let view = IntListMatch::inspect(&matcher).unwrap();
            let mut code = Code::default();
            function.match_branch(
                &mut code,
                function.shape.checkpoints[0],
                &view,
                false,
                ProgressOutput::Direct,
                &mut emit_edge,
            );
            assert_eq!(code.as_str(), expected.trim_start_matches('\n'));
        }
    }
}
