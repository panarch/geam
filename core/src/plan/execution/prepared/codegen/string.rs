use super::shape::CompiledEdge;
use super::{Code, FunctionCodegen, Rust};
use crate::plan::execution::compiled::CompiledCheckpoint;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    BlockId, Match, MatchEdgeArgument, MatchPattern, ParamLocal, StringInstruction, StringLocalId,
};

/// Preparation-local string operations borrow the canonical graph's literals.
pub(super) enum StringOperation<'graph> {
    Literal(&'graph str),
    DropPrefix { value: StringLocalId, bytes: usize },
}

pub(super) enum StringTest<'graph> {
    Prefix {
        value: StringLocalId,
        prefix: &'graph str,
    },
    Equal {
        left: StringLocalId,
        right: StringLocalId,
        negate: bool,
    },
}

/// Let-assert retains its canonical refutable Match and binding indices.
/// Only a scalar string pattern and its whole-value aliases are admitted.
pub(super) struct StringMatch<'graph> {
    pub matcher: &'graph Match,
    pub subject: StringLocalId,
    pattern: StringMatchPattern<'graph>,
    bindings: Vec<(usize, StringBinding<'graph>)>,
}

enum StringMatchPattern<'graph> {
    Literal(&'graph str),
    Prefix(&'graph str),
}

enum StringBinding<'graph> {
    Whole,
    Prefix(&'graph str),
    Suffix(usize),
}

impl<'graph> StringMatch<'graph> {
    pub(super) fn inspect(matcher: &'graph Match) -> Option<Self> {
        let ParamLocal::String(subject) = matcher.subject() else {
            return None;
        };
        let mut pattern = matcher.pattern();
        let mut bindings = Vec::new();
        while let MatchPattern::Alias {
            pattern: inner,
            binding,
        } = pattern
        {
            bindings.push((binding.index(), StringBinding::Whole));
            pattern = inner;
        }
        let pattern = match pattern {
            MatchPattern::String(text) => StringMatchPattern::Literal(text.as_str()),
            MatchPattern::StringPrefix {
                prefix,
                left,
                right,
            } => {
                if let Some(binding) = left {
                    bindings.push((binding.index(), StringBinding::Prefix(prefix.as_str())));
                }
                if let Some(binding) = right {
                    bindings.push((binding.index(), StringBinding::Suffix(prefix.len())));
                }
                StringMatchPattern::Prefix(prefix.as_str())
            }
            _ => return None,
        };
        Some(Self {
            matcher,
            subject: *subject,
            pattern,
            bindings,
        })
    }
}

impl<'graph> StringOperation<'graph> {
    pub(super) fn inspect(instruction: &'graph StringInstruction) -> Option<Self> {
        Some(match instruction {
            StringInstruction::Value(text) => Self::Literal(text.as_str()),
            StringInstruction::DropPrefix { value, prefix } => Self::DropPrefix {
                value: *value,
                bytes: prefix.len(),
            },
            _ => return None,
        })
    }
}

impl<Graph: ExecutionGraphProfile> FunctionCodegen<'_, Graph> {
    pub(super) fn string_match(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        view: &StringMatch<'_>,
        mut emit_edge: impl FnMut(&mut Code, CompiledEdge<'_>),
    ) {
        let subject = format!("b{}_s{}", point.block.0, view.subject.0);
        let condition = match view.pattern {
            StringMatchPattern::Literal(text) => {
                literal_condition(&format!("values.text({subject})"), text)
            }
            StringMatchPattern::Prefix(text) => format!(
                "values.text({subject}).starts_with({})",
                Rust::expression(text)
            ),
        };
        source.open(&format!("if {condition} {{\n"));
        for (index, binding) in &view.bindings {
            let selected = view.matcher.success().args().iter().any(|argument| {
                matches!(argument, MatchEdgeArgument::Binding(selected) if selected == index)
            });
            if !selected {
                continue;
            }
            let expression = match binding {
                StringBinding::Whole => subject.clone(),
                StringBinding::Prefix(text) => format!(
                    "data::compiled::string::StringRange::literal({})",
                    Rust::expression(text)
                ),
                StringBinding::Suffix(bytes) => format!("{subject}.drop_prefix({bytes})"),
            };
            source.push_str(&format!("let m{index} = {expression};\n"));
        }
        emit_edge(source, CompiledEdge::Match(view.matcher.success()));
        source.alternative("} else {\n");
        emit_edge(source, CompiledEdge::Ordinary(view.matcher.failure()));
        source.close("}\n");
    }

    pub(super) fn string_instruction(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        output: StringLocalId,
        instruction: &StringOperation<'_>,
    ) {
        let expression = match instruction {
            StringOperation::Literal(text) => format!(
                "data::compiled::string::StringRange::literal({})",
                Rust::expression(text)
            ),
            StringOperation::DropPrefix { value, bytes } => {
                format!("b{}_s{}.drop_prefix({bytes})", point.block.0, value.0)
            }
        };
        source.push_str(&format!(
            "let b{}_s{} = {expression};\n",
            point.block.0, output.0
        ));
    }
}

pub(super) fn literal_condition(subject: &str, text: &str) -> String {
    if text.is_empty() {
        format!("{subject}.is_empty()")
    } else {
        format!("{subject} == {}", Rust::expression(text))
    }
}

pub(super) fn test_expression(block: BlockId, test: &StringTest<'_>) -> String {
    match test {
        StringTest::Prefix { value, prefix } => format!(
            "values.text(b{}_s{}).starts_with({})",
            block.0,
            value.0,
            Rust::expression(prefix)
        ),
        StringTest::Equal {
            left,
            right,
            negate,
        } => format!(
            "values.text(b{}_s{}) {} values.text(b{}_s{})",
            block.0,
            left.0,
            if *negate { "!=" } else { "==" },
            block.0,
            right.0
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::super::shape::CompiledTerminator;
    use super::super::{Code, CompiledShape, FunctionCodegen};
    use super::{StringMatch, StringOperation, StringTest, literal_condition, test_expression};
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::{
        BlockId, MatchPattern, StringInstruction, StringLocalId, Terminator,
    };
    use crate::{ExecutionPlan, compile_typed_module, plan_module};

    #[test]
    fn literal_conditions_emit_empty_checks_and_exact_escaped_comparisons() {
        assert_eq!(
            literal_condition("values.text(b3_s2)", ""),
            "values.text(b3_s2).is_empty()"
        );
        assert_eq!(
            literal_condition("values.text(b3_s2)", "\n\"\\λ"),
            "values.text(b3_s2) == \"\\n\\\"\\\\λ\""
        );
    }

    fn string_match<'view, 'graph>(
        terminator: &'view CompiledTerminator<'graph>,
    ) -> &'view StringMatch<'graph> {
        match terminator {
            CompiledTerminator::StringMatch(view) => view,
            _ => panic!("source should lower one string match"),
        }
    }

    #[test]
    #[should_panic(expected = "source should lower one string match")]
    fn string_match_fixture_guard_rejects_a_non_match_terminator() {
        string_match(&CompiledTerminator::Interpreted);
    }

    #[test]
    fn prefix_assertions_emit_only_bindings_used_by_the_success_edge() {
        for (source, expected) in [
            (
                r#"
fn check(text: String) { let assert "λ" <> _ = text 1 }
pub fn main() { check("λtail") }
"#,
                r#"if values.text(b0_s0).starts_with("λ") {
    ()
} else {
    (b0_s0,)
}
"#,
            ),
            (
                r#"
fn check(text: String) {
  let assert "λ" as unused <> rest = text
  case rest { "tail" -> 1 _ -> 2 }
}
pub fn main() { check("λtail") }
"#,
                r#"if values.text(b0_s0).starts_with("λ") {
    let m1 = b0_s0.drop_prefix(2);
    (m1,)
} else {
    (b0_s0,)
}
"#,
            ),
        ] {
            let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
            let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
            let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
            let point = shape.checkpoints[shape.start(shape.graph.entry())];
            let function = FunctionCodegen {
                name: "string_int_1".into(),
                shape,
            };
            let view = string_match(&function.shape.block(point.block).terminator);
            let mut code = Code::default();
            function.string_match(&mut code, point, view, |code, edge| {
                let inputs = function.edge_inputs(code, point.block, edge);
                code.push_str(&format!("{inputs}\n"));
            });
            assert_eq!(code.as_str(), expected);
        }
    }

    #[test]
    fn bit_array_assertions_stay_outside_the_string_match_owner() {
        let source = r#"
fn read(bytes: BitArray) {
  let assert <<head:8, _:bits>> = bytes
  head
}
pub fn main() { read(<<7, 8>>) }
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        let matches = plan
            .int_function(IntFunctionId(1))
            .body()
            .block_graph()
            .blocks()
            .filter_map(|block| match block.terminator() {
                Terminator::Match(matcher) => Some(matcher),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(matches.len(), 1);
        assert!(StringMatch::inspect(matches[0]).is_none());
    }

    #[test]
    fn universal_string_patterns_remain_outside_the_refutable_match_contract() {
        // The frontend removes irrefutable assertions. The Match owner still
        // has a valid universal String pattern, outside this leaf selector.
        let source = r#"
fn check(text: String) { let assert "λ" <> _ = text 1 }
pub fn main() { check("λtail") }
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        let view = string_match(&shape.block(shape.graph.entry()).terminator);
        let mut matcher = view.matcher.clone();
        matcher.pattern = MatchPattern::Discard;
        assert!(StringMatch::inspect(&matcher).is_none());
    }

    #[test]
    fn string_tests_emit_exact_escaped_prefix_and_borrowed_variable_comparisons() {
        for (test, expected) in [
            (
                StringTest::Prefix {
                    value: StringLocalId(2),
                    prefix: "\n\"\\한",
                },
                "values.text(b3_s2).starts_with(\"\\n\\\"\\\\한\")",
            ),
            (
                StringTest::Equal {
                    left: StringLocalId(2),
                    right: StringLocalId(5),
                    negate: false,
                },
                "values.text(b3_s2) == values.text(b3_s5)",
            ),
            (
                StringTest::Equal {
                    left: StringLocalId(2),
                    right: StringLocalId(5),
                    negate: true,
                },
                "values.text(b3_s2) != values.text(b3_s5)",
            ),
        ] {
            assert_eq!(test_expression(BlockId(3), &test), expected);
        }
    }

    #[test]
    fn literal_and_prefix_operations_emit_ranges_and_leave_concatenation_interpreted() {
        let source = r#"pub fn main() { case "λ" { "" -> 1 _ -> 2 } }"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(0)).body()).unwrap();
        let point = shape.checkpoints[shape.start(shape.graph.entry())];
        let function = FunctionCodegen {
            name: "string_int_0".into(),
            shape,
        };
        for (instruction, expected) in [
            (
                StringInstruction::Value("\n\"\\한".into()),
                "let b0_s0 = data::compiled::string::StringRange::literal(\"\\n\\\"\\\\한\");\n",
            ),
            (
                StringInstruction::DropPrefix {
                    value: StringLocalId(0),
                    prefix: "한".into(),
                },
                "let b0_s0 = b0_s0.drop_prefix(3);\n",
            ),
        ] {
            let inspected = StringOperation::inspect(&instruction).unwrap();
            let mut output = Code::default();
            function.string_instruction(&mut output, point, StringLocalId(0), &inspected);
            assert_eq!(output.as_str(), expected);
        }
        assert!(
            StringOperation::inspect(&StringInstruction::Concatenate {
                left: StringLocalId(0),
                right: StringLocalId(1)
            })
            .is_none()
        );
    }

    #[test]
    fn string_let_assert_emits_exact_prefix_bindings_and_literal_conditions() {
        let source = r#"
fn examine(text: String) {
  let assert "λ" as prefix <> rest as whole = text as "required"
  case whole == prefix { True -> 1 False -> case rest == "tail" { True -> 2 False -> 3 } }
}
pub fn main() { examine("λtail") }
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(1)).body()).unwrap();
        let point = shape.checkpoints[shape.start(shape.graph.entry())];
        let function = FunctionCodegen {
            name: "string_int_1".into(),
            shape,
        };
        let view = string_match(&function.shape.block(point.block).terminator);
        let mut code = Code::default();
        function.string_match(&mut code, point, view, |code, edge| {
            let inputs = function.edge_inputs(code, point.block, edge);
            code.push_str(&format!("b{} {inputs}\n", edge.target().0));
        });
        assert_eq!(
            code.as_str(),
            r#"if values.text(b0_s0).starts_with("λ") {
    let m2 = b0_s0;
    let m0 = data::compiled::string::StringRange::literal("λ");
    let m1 = b0_s0.drop_prefix(2);
    b1 (m0, m1, m2,)
} else {
    b6 (b0_s0,)
}
"#
        );

        let source = r#"pub fn main() { let assert "λ" as whole = "λ" as "required" case whole { "λ" -> 3 _ -> 4 } }"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        let shape = CompiledShape::inspect(plan.int_function(IntFunctionId(0)).body()).unwrap();
        let point = shape.checkpoints[shape.start(shape.graph.entry()) + 1];
        let function = FunctionCodegen {
            name: "string_int_0".into(),
            shape,
        };
        let view = string_match(&function.shape.block(point.block).terminator);
        let mut code = Code::default();
        function.string_match(&mut code, point, view, |code, edge| {
            let inputs = function.edge_inputs(code, point.block, edge);
            code.push_str(&format!("{inputs}\n"));
        });
        assert_eq!(
            code.as_str(),
            r#"if values.text(b0_s0) == "λ" {
    let m0 = b0_s0;
    (m0,)
} else {
    (b0_s0,)
}
"#
        );
    }
}
