use super::shape::CompiledEdge;
use super::{Code, FunctionCodegen, ProgressOutput, Rust, tuple};
use crate::plan::execution::compiled::CompiledCheckpoint;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    BoolInstruction, BoolLocalId, CustomLocalId, IntInstruction, IntLocalId, IntegerLiteral, Match,
    MatchEdgeArgument, MatchPattern, ParamLocal, ProfiledInstruction, ProfiledInstructionKind,
};
use crate::plan::execution::type_::{CustomConstructorId, CustomTypeTable, ValueType};

pub(super) enum CustomField {
    Integer {
        output: IntLocalId,
        source: CustomLocalId,
        index: usize,
    },
    Boolean {
        output: BoolLocalId,
        source: CustomLocalId,
        index: usize,
    },
}

/// A preparation-only view of a flat constructor pattern. Field families come
/// from the admitted constructor, including fields whose patterns discard.
pub(super) struct CustomMatch<'graph> {
    pub matcher: &'graph Match,
    pub subject: CustomLocalId,
    pub constructor: Option<CustomConstructorId>,
    pub fields: Vec<CustomFieldPattern<'graph>>,
    pub aliases: Vec<usize>,
}

pub(super) struct CustomFieldPattern<'graph> {
    kind: ScalarKind,
    literal: Option<ScalarLiteral<'graph>>,
    pub bindings: Vec<usize>,
}

#[derive(Clone, Copy)]
enum ScalarKind {
    Integer,
    Boolean,
}

enum ScalarLiteral<'graph> {
    Integer(&'graph IntegerLiteral),
    Boolean(bool),
}

impl CustomField {
    pub(super) fn inspect<Graph: ExecutionGraphProfile>(
        instruction: &ProfiledInstruction<Graph>,
    ) -> Option<Self> {
        let ProfiledInstruction::Value(value) = instruction else {
            return None;
        };
        Some(match (value.kind(), value.output().local()) {
            (
                ProfiledInstructionKind::Int(IntInstruction::CustomField { source, index }),
                ParamLocal::Int(output),
            ) => Self::Integer {
                output: *output,
                source: source.id(),
                index: *index,
            },
            (
                ProfiledInstructionKind::Bool(BoolInstruction::CustomField { source, index }),
                ParamLocal::Bool(output),
            ) => Self::Boolean {
                output: *output,
                source: source.id(),
                index: *index,
            },
            _ => return None,
        })
    }
}

impl<'graph> CustomMatch<'graph> {
    pub(super) fn inspect(matcher: &'graph Match, types: &CustomTypeTable) -> Option<Self> {
        let ParamLocal::Custom(subject) = matcher.subject() else {
            return None;
        };
        let mut view = Self {
            matcher,
            subject: subject.id(),
            constructor: None,
            fields: Vec::new(),
            aliases: Vec::new(),
        };
        view.pattern(matcher.pattern(), types)?;
        Some(view)
    }

    pub(super) fn selected(&self, index: usize) -> bool {
        self.matcher.success().args().iter().any(
            |argument| matches!(argument, MatchEdgeArgument::Binding(binding) if *binding == index),
        )
    }

    fn pattern(&mut self, pattern: &'graph MatchPattern, types: &CustomTypeTable) -> Option<()> {
        match pattern {
            MatchPattern::Bind(binding) => self.aliases.push(binding.index()),
            MatchPattern::Discard => {}
            MatchPattern::Alias { pattern, binding } => {
                self.pattern(pattern, types)?;
                self.aliases.push(binding.index());
            }
            MatchPattern::Custom {
                constructor,
                fields,
            } => {
                let descriptor = types.constructor(*constructor);
                if descriptor.fields().len() != fields.len() {
                    return None;
                }
                self.constructor = Some(*constructor);
                for (field, pattern) in descriptor.fields().iter().zip(fields.iter()) {
                    let kind = match field.type_() {
                        ValueType::Int => ScalarKind::Integer,
                        ValueType::Bool => ScalarKind::Boolean,
                        _ => return None,
                    };
                    let mut field = CustomFieldPattern {
                        kind,
                        literal: None,
                        bindings: Vec::new(),
                    };
                    field.pattern(pattern)?;
                    self.fields.push(field);
                }
            }
            _ => return None,
        }
        Some(())
    }
}

impl<'graph> CustomFieldPattern<'graph> {
    fn pattern(&mut self, pattern: &'graph MatchPattern) -> Option<()> {
        match pattern {
            MatchPattern::Int(literal)
                if matches!(self.kind, ScalarKind::Integer)
                    && i64::try_from(literal.materialize()).is_ok() =>
            {
                self.literal = Some(ScalarLiteral::Integer(literal));
            }
            MatchPattern::Bool(literal) if matches!(self.kind, ScalarKind::Boolean) => {
                self.literal = Some(ScalarLiteral::Boolean(*literal));
            }
            MatchPattern::Bind(binding) => self.bindings.push(binding.index()),
            MatchPattern::Discard => {}
            MatchPattern::Alias { pattern, binding } => {
                self.pattern(pattern)?;
                self.bindings.push(binding.index());
            }
            _ => return None,
        }
        Some(())
    }
}

impl<Graph: ExecutionGraphProfile> FunctionCodegen<'_, '_, Graph> {
    pub(super) fn custom_preflight(
        &self,
        source: &mut Code,
        index: usize,
        field: &CustomField,
        output: ProgressOutput<'_>,
    ) {
        let point = self.shape.checkpoints[index];
        let (local, custom, field_index, method) = match field {
            CustomField::Integer {
                output,
                source,
                index,
            } => (
                format!("b{}_i{}", point.block.0, output.0),
                source.0,
                index,
                "integer",
            ),
            CustomField::Boolean {
                output,
                source,
                index,
            } => (
                format!("b{}_v{}", point.block.0, output.0),
                source.0,
                index,
                "boolean",
            ),
        };
        source.open(&format!(
            "let {local} = match b{}_c{custom}.{method}({field_index}) {{\n",
            point.block.0
        ));
        source.push_str("Some(value) => value,\n");
        source.open("None => {\n");
        self.interpreted(source, index, true, output);
        source.close("}\n");
        source.close("};\n");
    }

    pub(super) fn custom_preflight_terminator(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        view: &CustomMatch<'_>,
    ) {
        let subject = format!("b{}_c{}", point.block.0, view.subject.0);
        source.push_str("let _matched = ");
        if let Some(constructor) = view.constructor {
            source.open(&format!(
                "if {subject}.matches_constructor({}) {{\n",
                Rust::expression(&constructor)
            ));
        } else {
            source.open("{\n");
        }
        source.open("'pattern: {\n");
        let mut bindings = Vec::new();
        for (index, field) in view.fields.iter().enumerate() {
            let method = match field.kind {
                ScalarKind::Integer => "integer",
                ScalarKind::Boolean => "boolean",
            };
            // The canonical matcher checks every field family before its
            // pattern, even a discard. Stage all reads before charging or
            // exposing bindings to the success edge.
            source.open(&format!(
                "let Some(_field{index}) = {subject}.{method}({index}) else {{\n"
            ));
            source.push_str("break 'pattern Err(());\n");
            source.close("};\n");
            if let Some(literal) = &field.literal {
                let condition = match literal {
                    ScalarLiteral::Integer(literal) => format!("_field{index} != {literal}_i128"),
                    ScalarLiteral::Boolean(true) => format!("!_field{index}"),
                    ScalarLiteral::Boolean(false) => format!("_field{index}"),
                };
                source.open(&format!("if {condition} {{\n"));
                source.push_str("break 'pattern Ok(None);\n");
                source.close("}\n");
            }
            for &binding in &field.bindings {
                if view.selected(binding) {
                    source.push_str(&format!("let m{binding} = _field{index};\n"));
                    bindings.push(format!("m{binding}"));
                }
            }
        }
        source.push_str(&format!("break 'pattern Ok(Some({}));\n", tuple(bindings)));
        source.close("}\n");
        if view.constructor.is_some() {
            source.alternative("} else {\n");
            source.push_str("Ok(None)\n");
        }
        source.close("};\n");
    }

    pub(super) fn custom_match_branch(
        &self,
        source: &mut Code,
        point: CompiledCheckpoint,
        view: &CustomMatch<'_>,
        returning: bool,
        output: ProgressOutput<'_>,
        emit_edge: &mut impl FnMut(&mut Code, CompiledEdge<'_>),
    ) {
        let bindings = tuple(
            view.fields
                .iter()
                .flat_map(|field| field.bindings.iter())
                .filter(|&&index| view.selected(index))
                .map(|index| format!("m{index}")),
        );
        source.open("match _matched {\n");
        source.open(&format!("Ok(Some({bindings})) => {{\n"));
        source.push_str("*budget -= 1;\n");
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

#[cfg(test)]
mod tests {
    use super::{Code, FunctionCodegen, Rust};
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::prepared::codegen::shape::{CompiledShape, CompiledTerminator};
    use crate::{ExecutionPlan, Value, compile_typed_module, plan_module};

    #[test]
    fn constructor_preflight_stages_selected_integer_and_boolean_bindings() {
        let source = r#"
type Item { Item(value: Int, enabled: Bool) Other }
fn score(input: Item) {
  case input {
    Item(value, enabled) -> case enabled { True -> value False -> 0 }
    Other -> -1
  }
}
pub fn main() { score(Item(7, True)) }
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            Value::Int(7.into())
        );
        let function = FunctionCodegen {
            name: "score",
            shape: &CompiledShape::inspect_callback(
                plan.int_function(IntFunctionId(1)).body(),
                &plan.program.common.custom_types,
            )
            .unwrap(),
        };
        let point = function.shape.checkpoints[function.entry()];
        let views = function
            .shape
            .blocks
            .values()
            .filter_map(|block| match &block.terminator {
                CompiledTerminator::Custom(view) => Some(view),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(views.len(), 1);
        let view = views[0];
        let mut code = Code::default();
        function.custom_preflight_terminator(&mut code, point, view);
        assert_eq!(
            code.as_str(),
            r#"let _matched = if b0_c0.matches_constructor(data::type_::CustomConstructorId {
    type_id: data::type_::CustomTypeId(0),
    index: 0,
}) {
    'pattern: {
        let Some(_field0) = b0_c0.integer(0) else {
            break 'pattern Err(());
        };
        let m0 = _field0;
        let Some(_field1) = b0_c0.boolean(1) else {
            break 'pattern Err(());
        };
        let m1 = _field1;
        break 'pattern Ok(Some((m0, m1,)));
    }
} else {
    Ok(None)
};
"#
        );
    }

    #[test]
    fn constructor_preflight_reads_typed_literals_in_order_without_charging_or_committing() {
        let source = r#"
type Item { Item(Int, Bool) Other }
fn score(input: Item) {
  case input {
    Item(7, True) -> 9
    _ -> 0
  }
}
pub fn main() { score(Item(7, True)) }
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        assert_eq!(
            crate::runtime::run_main(&plan, &mut Vec::new()).unwrap(),
            Value::Int(9.into())
        );
        let function = FunctionCodegen {
            name: "score",
            shape: &CompiledShape::inspect_callback(
                plan.int_function(IntFunctionId(1)).body(),
                &plan.program.common.custom_types,
            )
            .unwrap(),
        };
        let mut target = Code::default();
        function.write_target(&mut target, &Rust::expression(&IntFunctionId(1)));
        assert_eq!(target.as_str(), "");
        let entry = function.shape.graph.entry();
        assert!(function.resumes_next());
        let views = function
            .shape
            .blocks
            .values()
            .filter_map(|block| match &block.terminator {
                CompiledTerminator::Custom(view) => Some(view),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(views.len(), 1);
        let view = views[0];
        let mut code = Code::default();
        function.custom_preflight_terminator(
            &mut code,
            function.shape.checkpoints[function.shape.start(entry)],
            view,
        );
        assert_eq!(
            code.as_str(),
            r#"let _matched = if b0_c0.matches_constructor(data::type_::CustomConstructorId {
    type_id: data::type_::CustomTypeId(0),
    index: 0,
}) {
    'pattern: {
        let Some(_field0) = b0_c0.integer(0) else {
            break 'pattern Err(());
        };
        if _field0 != 7_i128 {
            break 'pattern Ok(None);
        }
        let Some(_field1) = b0_c0.boolean(1) else {
            break 'pattern Err(());
        };
        if !_field1 {
            break 'pattern Ok(None);
        }
        break 'pattern Ok(Some(()));
    }
} else {
    Ok(None)
};
"#
        );
    }

    #[test]
    fn single_scalar_projection_does_not_require_intermediate_resumption() {
        let source = r#"
type Item { Item(value: Int) }
fn read(input: Item) { input.value }
pub fn main() { read(Item(7)) }
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        assert_eq!(
            crate::runtime::run_main(&plan, &mut Vec::new()).unwrap(),
            Value::Int(7.into())
        );
        let function = FunctionCodegen {
            name: "read",
            shape: &CompiledShape::inspect_callback(
                plan.int_function(IntFunctionId(1)).body(),
                &plan.program.common.custom_types,
            )
            .unwrap(),
        };
        assert_eq!(function.shape.checkpoints.len(), 2);
        assert!(!function.resumes_next());
    }

    #[test]
    fn boolean_projection_saves_the_original_custom_before_an_uncharged_stop() {
        use super::{CustomField, ProgressOutput};
        use crate::plan::execution::function::BoolFunctionId;

        let source = r#"
type Item { Item(value: Int, enabled: Bool) }
fn read(item: Item) { item.enabled }
pub fn main() { read(Item(7, True)) }
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            Value::Bool(true)
        );
        let body = plan.bool_function(BoolFunctionId(1)).body();
        let function = FunctionCodegen {
            name: "read",
            shape: &CompiledShape::inspect_callback(body, &plan.program.common.custom_types)
                .unwrap(),
        };
        let entry = function.shape.graph.entry();
        let field =
            CustomField::inspect(&body.block_graph().block(entry).instructions()[0]).unwrap();
        let mut code = Code::default();
        function.custom_preflight(&mut code, function.entry(), &field, ProgressOutput::Direct);
        assert_eq!(
            code.as_str(),
            r#"let b0_v0 = match b0_c0.boolean(1) {
    Some(value) => value,
    None => {

        values.ints.clear();
        values.ints.extend_from_slice(&[]);
        values.bools.clear();
        values.bools.extend_from_slice(&[]);
        values.customs.clear();
        values.customs.extend([b0_c0]);
        return data::compiled::custom_loop::CallbackProgress::Stopped(data::compiled::custom_loop::CallbackStop::Interpreted(0));
    }
};
"#
        );
    }
    #[test]
    fn flat_pattern_views_keep_whole_and_field_aliases_and_reject_other_grammars() {
        use super::CustomMatch;
        use crate::plan::execution::graph::{IntegerLiteral, MatchPattern, MatchPatternBinding};

        let source = r#"
type Item { Item(Int, Bool) Other }
fn score(item: Item) { case item { Item(value, _) -> value Other -> 0 } }
pub fn main() { score(Item(7, True)) }
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            Value::Int(7.into())
        );
        let shape = CompiledShape::inspect_callback(
            plan.int_function(IntFunctionId(1)).body(),
            &plan.program.common.custom_types,
        )
        .unwrap();
        let function = FunctionCodegen {
            name: "score",
            shape: &shape,
        };
        let views = function
            .shape
            .blocks
            .values()
            .filter_map(|block| match &block.terminator {
                CompiledTerminator::Custom(view) => Some(view),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(views.len(), 1);
        let original = views[0];
        let constructor = original.constructor.unwrap();
        let mut matcher = original.matcher.clone();
        // These are preparation-only pattern grammar inputs. No changed graph
        // or metadata is admitted or executed by this owner example.
        for (pattern, aliases) in [
            (MatchPattern::Bind(MatchPatternBinding::new(5)), vec![5]),
            (MatchPattern::Discard, vec![]),
            (
                MatchPattern::Alias {
                    pattern: Box::new(MatchPattern::Custom {
                        constructor,
                        fields: vec![
                            MatchPattern::Alias {
                                pattern: Box::new(MatchPattern::Int(IntegerLiteral::from(
                                    num_bigint::BigInt::from(7),
                                )))
                                .into(),
                                binding: MatchPatternBinding::new(1),
                            },
                            MatchPattern::Alias {
                                pattern: Box::new(MatchPattern::Bool(false)).into(),
                                binding: MatchPatternBinding::new(2),
                            },
                        ]
                        .into(),
                    })
                    .into(),
                    binding: MatchPatternBinding::new(3),
                },
                vec![3],
            ),
        ] {
            matcher.pattern = pattern;
            let view = CustomMatch::inspect(&matcher, &plan.program.common.custom_types).unwrap();
            assert_eq!(view.aliases, aliases);
            let mut code = Code::default();
            let entry = function.shape.graph.entry();
            function.custom_preflight_terminator(
                &mut code,
                function.shape.checkpoints[function.shape.start(entry)],
                &view,
            );
            if !view.fields.is_empty() {
                assert_eq!(view.fields.len(), 2);
                assert_eq!(view.fields[0].bindings, vec![1]);
                assert_eq!(view.fields[1].bindings, vec![2]);
                assert_eq!(
                    code.as_str(),
                    r#"let _matched = if b0_c0.matches_constructor(data::type_::CustomConstructorId {
    type_id: data::type_::CustomTypeId(0),
    index: 0,
}) {
    'pattern: {
        let Some(_field0) = b0_c0.integer(0) else {
            break 'pattern Err(());
        };
        if _field0 != 7_i128 {
            break 'pattern Ok(None);
        }
        let Some(_field1) = b0_c0.boolean(1) else {
            break 'pattern Err(());
        };
        if _field1 {
            break 'pattern Ok(None);
        }
        break 'pattern Ok(Some(()));
    }
} else {
    Ok(None)
};
"#
                );
            } else {
                assert_eq!(
                    code.as_str(),
                    r#"let _matched = {
    'pattern: {
        break 'pattern Ok(Some(()));
    }
};
"#
                );
            }
        }
        for fields in [
            vec![],
            vec![MatchPattern::Bool(true), MatchPattern::Bool(false)],
            vec![
                MatchPattern::Int(IntegerLiteral::from(num_bigint::BigInt::from(7))),
                MatchPattern::Int(IntegerLiteral::from(num_bigint::BigInt::from(1))),
            ],
        ] {
            matcher.pattern = MatchPattern::Custom {
                constructor,
                fields: fields.into(),
            };
            assert!(CustomMatch::inspect(&matcher, &plan.program.common.custom_types).is_none());
        }
        for pattern in [
            MatchPattern::Alias {
                pattern: Box::new(MatchPattern::Int(IntegerLiteral::from(
                    num_bigint::BigInt::from(7),
                )))
                .into(),
                binding: MatchPatternBinding::new(0),
            },
            MatchPattern::Custom {
                constructor,
                fields: vec![
                    MatchPattern::Alias {
                        pattern: Box::new(MatchPattern::Bool(true)).into(),
                        binding: MatchPatternBinding::new(1),
                    },
                    MatchPattern::Discard,
                ]
                .into(),
            },
        ] {
            matcher.pattern = pattern;
            assert!(CustomMatch::inspect(&matcher, &plan.program.common.custom_types).is_none());
        }
        matcher.pattern = MatchPattern::Int(IntegerLiteral::from(num_bigint::BigInt::from(7)));
        assert!(CustomMatch::inspect(&matcher, &plan.program.common.custom_types).is_none());
    }
    #[test]
    fn arithmetic_regions_are_neither_custom_fields_nor_loop_calls() {
        use super::CustomField;
        use crate::plan::execution::prepared::codegen::custom_loop::CustomLoopInstruction;
        let source = r#"
fn walk(n: Int, total: Int) {
  case n { 0 -> total _ -> walk(n - 1, total + 1) }
}
pub fn main() { walk(3, 0) }
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            Value::Int(3.into())
        );
        let graph = plan
            .int_function(IntFunctionId(1))
            .body()
            .block_graph()
            .as_view();
        let instructions = graph
            .blocks()
            .flat_map(|block| block.instructions())
            .collect::<Vec<_>>();
        assert_eq!(instructions.len(), 1);
        for instruction in instructions {
            assert!(CustomField::inspect(instruction).is_none());
            assert!(
                CustomLoopInstruction::inspect(instruction, graph.block(graph.entry()).params())
                    .is_none()
            );
        }
    }
    #[test]
    fn a_selected_whole_subject_alias_uses_the_original_custom_owner() {
        use super::super::shape::CompiledEdge;
        let source = r#"
type Item { Item(Int, Bool) Other }
fn score(item: Item) {
  let assert Item(7, _) as original = item
  let assert Item(value, _) = original
  value
}
pub fn main() { score(Item(7, True)) }
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            Value::Int(7.into())
        );
        let shape = CompiledShape::inspect_callback(
            plan.int_function(IntFunctionId(1)).body(),
            &plan.program.common.custom_types,
        )
        .unwrap();
        let function = FunctionCodegen {
            name: "score",
            shape: &shape,
        };
        let mut selected = Vec::new();
        for (&index, block) in &function.shape.blocks {
            if let CompiledTerminator::Custom(view) = &block.terminator
                && !view.aliases.is_empty()
            {
                let mut code = Code::default();
                let inputs = function.edge_inputs(
                    &mut code,
                    crate::plan::execution::graph::BlockId(index),
                    CompiledEdge::Match(view.matcher.success()),
                );
                selected.push((code.text, inputs));
            }
        }
        assert_eq!(selected, vec![(String::new(), "(b0_c0,)".into())]);
    }
}
