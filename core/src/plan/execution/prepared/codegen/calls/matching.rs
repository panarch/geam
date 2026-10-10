use super::local::{field_assignment, local_expression, local_name, local_type};
use super::nullary::{CallTypes, CustomLocalShape};
use super::shape::{CallFunction, CallLocal};
use super::{CallGroupCodegen, Code, ExecutionGraphProfile, Rust, canonical, function_state};
use crate::plan::execution::graph::{
    BlockId, Edge, Match, MatchEdgeArgument, MatchPattern, ParamLocal, ParamSlot,
};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::{CustomConstructorId, CustomTypeId, ValueType};
use std::collections::BTreeMap;

/// A preparation-local view of the admitted pattern. No second pattern program
/// is retained at run time: the renderer emits the original ordered walk.
pub(super) struct CompoundMatch<'graph> {
    original: &'graph Match,
    subject: CallLocal,
    pattern: CompoundPattern,
    arguments: Vec<CompoundArgument>,
    bindings: BTreeMap<usize, CompoundBinding>,
}

enum CompoundArgument {
    Binding(usize),
    Value(CallLocal),
}

struct CompoundBinding {
    local: CallLocal,
    read: FieldRead,
}

pub(super) enum FieldRead {
    Int,
    Float,
    Bool,
    Nil,
    UtfCodepoint,
    String,
    BitArray,
    Custom(CustomTypeId),
    Nullary(CustomLocalShape),
    Tuple(Table<ValueType>),
}

impl FieldRead {
    pub(super) fn inspect(local: &CallLocal) -> Option<Self> {
        Some(match local {
            CallLocal::Int(_) => Self::Int,
            CallLocal::Float(_) => Self::Float,
            CallLocal::Bool(_) => Self::Bool,
            CallLocal::Nil(_) => Self::Nil,
            CallLocal::UtfCodepoint(_) => Self::UtfCodepoint,
            CallLocal::String(_) => Self::String,
            CallLocal::BitArray(_) => Self::BitArray,
            CallLocal::Custom(local) => Self::Custom(local.local.shape.type_id),
            CallLocal::Nullary(local) => Self::Nullary(local.clone()),
            CallLocal::Tuple { type_, .. } => Self::Tuple(type_.clone()),
            _ => return None,
        })
    }
    pub(super) fn value_type(&self) -> ValueType {
        match self {
            Self::Int => ValueType::Int,
            Self::Float => ValueType::Float,
            Self::Bool => ValueType::Bool,
            Self::Nil => ValueType::Nil,
            Self::UtfCodepoint => ValueType::UtfCodepoint,
            Self::String => ValueType::String,
            Self::BitArray => ValueType::BitArray,
            Self::Custom(id) => ValueType::Custom(*id),
            Self::Nullary(local) => ValueType::Custom(local.local.shape.type_id),
            Self::Tuple(elements) => ValueType::Tuple(elements.clone()),
        }
    }
    pub(super) fn expression(&self, value: &str) -> String {
        let method = match self {
            Self::Int => "integer",
            Self::Float => "float",
            Self::Bool => "boolean",
            Self::Nil => "nil",
            Self::UtfCodepoint => "utf_codepoint",
            Self::String => "string",
            Self::BitArray => "bit_array",
            Self::Custom(_) => "custom",
            Self::Tuple(_) => "tuple",
            Self::Nullary(local) => {
                return format!(
                    "{value}.nullary(&{})",
                    Rust::expression(local.constructors.as_slice())
                );
            }
        };
        format!("{value}.{method}()")
    }
}

enum CompoundPattern {
    Bind(usize),
    Discard,
    Int(i64),
    Float(f64),
    String(String),
    Bool(bool),
    Nil,
    Tuple(Vec<Self>),
    Custom {
        constructor: CustomConstructorId,
        fields: Vec<(ValueType, Self)>,
    },
    Alias {
        pattern: Box<Self>,
        binding: usize,
    },
}

impl<'graph> CompoundMatch<'graph> {
    pub(super) fn enters(&self, block: BlockId) -> bool {
        self.original.success().target() == block || self.original.failure().target() == block
    }

    pub(super) fn accepts_edges(&self, mut accepts: impl FnMut(&Edge) -> bool) -> bool {
        accepts(self.original.failure())
    }

    pub(super) fn inspect(
        original: &'graph Match,
        types: &CallTypes<'_>,
        locals: &[CallLocal],
        parameters: &[ParamSlot],
    ) -> Option<Self> {
        let subject = types.local(original.subject())?;
        let expected = match original.subject() {
            ParamLocal::Custom(local) => ValueType::Custom(local.shape.type_id),
            ParamLocal::Tuple { type_, .. } => ValueType::Tuple(type_.clone()),
            _ => return None,
        };
        if !supported_type(&expected) {
            return None;
        }
        // Resolve every success column once, including the binding read.
        // CallShape checks ordinary failure edges alongside other transfers.
        let mut bindings = BTreeMap::new();
        let mut arguments = Vec::new();
        for (argument, parameter) in original.success().args().iter().zip(parameters) {
            let local = types.local(parameter.local())?;
            match argument {
                MatchEdgeArgument::Binding(index) => {
                    bindings.insert(
                        *index,
                        CompoundBinding {
                            read: FieldRead::inspect(&local)?,
                            local,
                        },
                    );
                    arguments.push(CompoundArgument::Binding(*index));
                }
                MatchEdgeArgument::Value(value) => {
                    // Admission identifies exactly one existing source local;
                    // this complete prefix has already classified it.
                    arguments.extend(
                        locals
                            .iter()
                            .filter(|local| local.canonical() == *value)
                            .cloned()
                            .map(CompoundArgument::Value),
                    );
                }
            }
        }
        let pattern = CompoundPattern::inspect(original.pattern(), types)?;
        Some(Self {
            original,
            subject,
            pattern,
            arguments,
            bindings,
        })
    }
}

impl CompoundPattern {
    // Pattern admission already proves subject/leaf types, nominal identity
    // and arity. This preparation-local pass classifies the supported reads;
    // it does not admit a second, weaker copy of the graph contract.
    fn inspect(pattern: &MatchPattern, types: &CallTypes<'_>) -> Option<Self> {
        Some(match pattern {
            MatchPattern::Bind(binding) => Self::Bind(binding.index),
            MatchPattern::Discard => Self::Discard,
            MatchPattern::Int(value) => Self::Int(i64::try_from(value.materialize()).ok()?),
            MatchPattern::Float(value) => Self::Float(*value),
            MatchPattern::String(value) => Self::String(value.as_str().to_owned()),
            MatchPattern::Bool(value) => Self::Bool(*value),
            MatchPattern::Nil => Self::Nil,
            MatchPattern::Tuple(patterns) => Self::Tuple(
                patterns
                    .iter()
                    .map(|pattern| Self::inspect(pattern, types))
                    .collect::<Option<Vec<_>>>()?,
            ),
            MatchPattern::Custom {
                constructor,
                fields,
            } => {
                let descriptor = types.custom_types.constructor(*constructor);
                Self::Custom {
                    constructor: *constructor,
                    fields: fields
                        .iter()
                        .zip(descriptor.fields.iter())
                        .map(|(pattern, field)| {
                            let expected = field.type_();
                            if !supported_type(expected) {
                                return None;
                            }
                            Some((expected.clone(), Self::inspect(pattern, types)?))
                        })
                        .collect::<Option<Vec<_>>>()?,
                }
            }
            MatchPattern::Alias { pattern, binding } => Self::Alias {
                pattern: Box::new(Self::inspect(pattern, types)?),
                binding: binding.index,
            },
            _ => return None,
        })
    }
}

fn supported_type(type_: &ValueType) -> bool {
    match type_ {
        ValueType::Int
        | ValueType::Float
        | ValueType::Bool
        | ValueType::Nil
        | ValueType::UtfCodepoint
        | ValueType::String
        | ValueType::BitArray
        | ValueType::Custom(_) => true,
        ValueType::Tuple(elements) => elements.iter().all(supported_type),
        _ => false,
    }
}

impl<Graph: ExecutionGraphProfile> CallGroupCodegen<'_, '_, Graph> {
    pub(super) fn write_compound_match(
        &self,
        source: &mut Code,
        function: &CallFunction<'_, Graph>,
        point: usize,
        matcher: &CompoundMatch<'_>,
    ) {
        let result_type = super::tuple(
            matcher
                .bindings
                .values()
                .map(|local| local_type(&local.local).to_owned()),
        );
        source.open(&format!(
            "let matched = (|| -> Option<Option<{result_type}>> {{\n"
        ));
        write_pattern(
            source,
            &matcher.pattern,
            &local_name(&matcher.subject),
            Some(&matcher.subject),
            &matcher.bindings,
            &mut 0,
        );
        let bindings = super::tuple(
            matcher
                .bindings
                .keys()
                .map(|index| format!("binding{index}")),
        );
        source.push_str(&format!("Some(Some({bindings}))\n"));
        source.close("})();\n");
        source.push_str(&format!(
            "let Some(matched) = matched else {{ return {}; }};\n*budget -= 1;\n",
            canonical(
                function.target,
                function.shape.checkpoints[point],
                &function.shape.locals[point]
            )
        ));
        source.open("match matched {\n");
        let edge = matcher.original.success();
        let destination = function.shape.starts[&edge.target().index()];
        let fields = success_fields(&function.shape.locals[destination], &matcher.arguments);
        source.push_str(&format!(
            "Some({bindings}) => {}State::Point{destination} {{ {fields} }},\nNone => {},\n",
            function_state(function.target),
            self.edge(function, matcher.original.failure())
        ));
        source.close("}\n");
    }
}

fn success_fields(parameters: &[CallLocal], arguments: &[CompoundArgument]) -> String {
    let mut remaining_bindings = BTreeMap::<usize, usize>::new();
    for argument in arguments {
        if let CompoundArgument::Binding(index) = argument {
            *remaining_bindings.entry(*index).or_default() += 1;
        }
    }
    parameters
        .iter()
        .zip(arguments)
        .map(|(parameter, argument)| match argument {
            CompoundArgument::Binding(index) => {
                let remaining = remaining_bindings.entry(*index).or_default();
                *remaining -= 1;
                let value = match parameter {
                    CallLocal::Int(_)
                    | CallLocal::Float(_)
                    | CallLocal::Bool(_)
                    | CallLocal::Nil(_)
                    | CallLocal::UtfCodepoint(_)
                    | CallLocal::Nullary(_) => format!("binding{index}"),
                    _ if *remaining > 0 => format!("binding{index}.clone()"),
                    _ => format!("binding{index}"),
                };
                format!("{}: {value}", local_name(parameter))
            }
            CompoundArgument::Value(argument) => field_assignment(parameter, argument),
        })
        .collect::<Vec<_>>()
        .join(", ")
}

fn write_pattern(
    source: &mut Code,
    pattern: &CompoundPattern,
    value: &str,
    root: Option<&CallLocal>,
    bindings: &BTreeMap<usize, CompoundBinding>,
    next_field: &mut usize,
) {
    match pattern {
        CompoundPattern::Bind(index) => write_binding(source, *index, value, root, bindings),
        CompoundPattern::Discard => {}
        CompoundPattern::Int(expected) => source.push_str(&format!(
            "if {value}.integer()? != {expected}_i128 {{ return Some(None); }}\n"
        )),
        CompoundPattern::Float(expected) => source.push_str(&format!(
            "if {value}.float()? != {} {{ return Some(None); }}\n",
            Rust::expression(expected)
        )),
        CompoundPattern::String(expected) => source.push_str(&format!(
            "if {value}.string()?.as_bytes() != {}.as_bytes() {{ return Some(None); }}\n",
            Rust::expression(expected.as_str())
        )),
        CompoundPattern::Bool(true) => source.push_str(&format!(
            "if !{value}.boolean()? {{ return Some(None); }}\n"
        )),
        CompoundPattern::Bool(false) => {
            source.push_str(&format!("if {value}.boolean()? {{ return Some(None); }}\n"))
        }
        CompoundPattern::Nil => source.push_str(&format!("{value}.nil()?;\n")),
        CompoundPattern::Tuple(patterns) => {
            let len = if root.is_some() {
                format!("{value}.len()")
            } else {
                format!("{value}.tuple_len()?")
            };
            source.push_str(&format!(
                "if {len} != {} {{ return Some(None); }}\n",
                patterns.len()
            ));
            for (index, pattern) in patterns.iter().enumerate() {
                if matches!(pattern, CompoundPattern::Discard) {
                    continue;
                }
                let field = *next_field;
                *next_field += 1;
                source.push_str(&format!("let field{field} = {value}.field({index})?;\n"));
                write_pattern(
                    source,
                    pattern,
                    &format!("field{field}"),
                    None,
                    bindings,
                    next_field,
                );
            }
        }
        CompoundPattern::Custom {
            constructor,
            fields,
        } => {
            source.push_str(&format!(
                "if !{value}.matches_constructor({}) {{ return Some(None); }}\n",
                Rust::expression(constructor)
            ));
            for (index, (expected, pattern)) in fields.iter().enumerate() {
                let field = *next_field;
                *next_field += 1;
                source.push_str(&format!("let field{field} = {value}.field({index})?;\nif !field{field}.matches_type(&{}) {{ return None; }}\n", Rust::expression(expected)));
                write_pattern(
                    source,
                    pattern,
                    &format!("field{field}"),
                    None,
                    bindings,
                    next_field,
                );
            }
        }
        CompoundPattern::Alias { pattern, binding } => {
            write_pattern(source, pattern, value, root, bindings, next_field);
            write_binding(source, *binding, value, root, bindings);
        }
    }
}

fn write_binding(
    source: &mut Code,
    index: usize,
    value: &str,
    root: Option<&CallLocal>,
    bindings: &BTreeMap<usize, CompoundBinding>,
) {
    let Some(binding) = bindings.get(&index) else {
        return;
    };
    let local = &binding.local;
    let expression = if let Some(root) = root {
        match (root, local) {
            (CallLocal::Custom(_), CallLocal::Nullary(local)) => format!(
                "{value}.nullary(&{})?",
                Rust::expression(local.constructors.as_slice())
            ),
            (CallLocal::Nullary(_), CallLocal::Custom(_)) => {
                format!("{}.into()", local_expression(root, true))
            }
            _ => local_expression(root, true),
        }
    } else {
        format!("{}?", binding.read.expression(value))
    };
    source.push_str(&format!("let binding{index} = {expression};\n"));
}

#[cfg(test)]
mod tests {
    use super::{Code, CompoundBinding, CompoundPattern, FieldRead, supported_type, write_pattern};
    use crate::plan::execution::graph::{
        BitArrayLocalId, BlockId, BoolLocalId, CustomLocal, CustomLocalId, FloatLocalId,
        IntFunctionLocalId, IntLocalId, MatchPattern, NilLocalId, StringLocalId, TupleLocalId,
        UtfCodepointLocalId,
    };
    use crate::plan::execution::prepared::codegen::calls::nullary::{CallTypes, CustomLocalShape};
    use crate::plan::execution::prepared::codegen::calls::shape::CallLocal;
    use crate::plan::execution::type_::{
        CustomConstructorId, CustomTypeId, CustomTypeTable, CustomValueShape, CustomValueShapeId,
        FunctionType, ValueShapeTable, ValueType,
    };
    use num_bigint::BigInt;
    use std::collections::BTreeMap;

    #[test]
    fn literal_pattern_classification_preserves_values_and_declines_wide_integers() {
        use super::CompoundPattern;
        let empty = crate::ExecutionPlan::from_module_plan(
            crate::plan_module(
                crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { 0 }")
                    .unwrap(),
            )
            .unwrap(),
        );
        let types = CallTypes {
            custom_types: &empty.program.common.custom_types,
            value_shapes: &empty.program.common.value_shapes,
        };
        for (pattern, expected) in [
            (MatchPattern::Discard, ""),
            (MatchPattern::Nil, "field.nil()?;\n"),
            (
                MatchPattern::Bool(false),
                "if field.boolean()? { return Some(None); }\n",
            ),
            (
                MatchPattern::Bool(true),
                "if !field.boolean()? { return Some(None); }\n",
            ),
            (
                MatchPattern::Int(BigInt::from(42).into()),
                "if field.integer()? != 42_i128 { return Some(None); }\n",
            ),
            (
                MatchPattern::Float(-0.0),
                "if field.float()? != f64::from_bits(9223372036854775808) { return Some(None); }\n",
            ),
            (
                MatchPattern::String("kept".into()),
                "if field.string()?.as_bytes() != \"kept\".as_bytes() { return Some(None); }\n",
            ),
        ] {
            let classified = CompoundPattern::inspect(&pattern, &types).unwrap();
            let mut rendered = Code::default();
            write_pattern(
                &mut rendered,
                &classified,
                "field",
                None,
                &BTreeMap::new(),
                &mut 0,
            );
            assert_eq!(rendered.as_str(), expected);
        }
        let wide: BigInt = BigInt::from(1_u8) << 130_usize;
        assert!(CompoundPattern::inspect(&MatchPattern::Int(wide.into()), &types).is_none());
        assert!(
            CompoundPattern::inspect(
                &MatchPattern::StringPrefix {
                    prefix: "tag:".into(),
                    left: None,
                    right: None
                },
                &types,
            )
            .is_none()
        );
    }

    #[test]
    fn source_compound_match_classification_preserves_unsupported_payloads_and_transfers() {
        use super::super::shape::CallTerminator;
        use super::CompoundMatch;
        use crate::plan::execution::function::{ExecutionFunctionEntry, ExecutionFunctionRef};
        use crate::plan::execution::graph::Terminator;
        use crate::{
            HostProviderModule, HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile,
        };
        for (name, source, expected, result) in [
            (
                "tuple_list",
                r#"
pub type Wrapped { Wrapped(String) }
fn choose(value: #(Wrapped, List(Int))) {
  case value { #(Wrapped("x"), [42]) -> 42 _ -> 0 }
}
pub fn main() { choose(#(Wrapped("x"), [42])) }
"#,
                // Tuple/list projection precedes the supported Custom match.
                &[true] as &[bool],
                42,
            ),
            (
                "callback_binding",
                r#"
pub type Wrapped { Wrapped(Int, fn(Int) -> Int) }
fn choose(value: Wrapped) { case value { Wrapped(1, f) -> f(42) _ -> 0 } }
pub fn main() { choose(Wrapped(1, fn(value) { value })) }
"#,
                &[false],
                42,
            ),
            (
                "list_field",
                r#"
pub type Wrapped { Wrapped(List(Int)) }
fn choose(value: Wrapped) { case value { Wrapped([42]) -> 42 _ -> 0 } }
pub fn main() { choose(Wrapped([42])) }
"#,
                &[false],
                42,
            ),
            (
                "large_integer",
                r#"
pub type Wrapped { Wrapped(#(Int, Int)) }
fn choose(value: Wrapped) {
  case value { Wrapped(#(9223372036854775808, _)) -> 1 _ -> 0 }
}
pub fn main() { choose(Wrapped(#(0, 0))) }
"#,
                &[false],
                0,
            ),
            (
                "large_alias",
                r#"
pub type Wrapped { Wrapped(#(Int, Int)) }
fn choose(value: Wrapped) {
  case value { Wrapped(#(9223372036854775808, _) as pair) -> pair.1 _ -> 0 }
}
pub fn main() { choose(Wrapped(#(0, 0))) }
"#,
                &[false],
                0,
            ),
            (
                "unused_binding",
                r#"
pub type Wrapped { Wrapped(Int, Int) }
fn choose(value: Wrapped) { case value { Wrapped(7, unused) -> 42 _ -> 0 } }
pub fn main() { choose(Wrapped(7, 9)) }
"#,
                &[true],
                42,
            ),
            (
                "sparse_binding",
                r#"
pub type Choice(a) { Filled(a) Empty }
pub type Wrapped { Wrapped(Int, Choice(Int)) }
fn choose(value: Wrapped) {
  case value { Wrapped(1, inside) -> case inside { Empty -> 42 _ -> 0 } _ -> 1 }
}
pub fn main() { choose(Wrapped(1, Empty)) }
"#,
                &[false, false],
                42,
            ),
            (
                "native_transfer",
                r#"
@external(erlang, "example", "identity")
fn identity(value: Int) -> Int
pub type Wrapped { Wrapped(String) }
fn choose(value: Wrapped, keep: Int) { case value { Wrapped("x") -> identity(keep) _ -> 0 } }
pub fn main() { choose(Wrapped("x"), 42) }
"#,
                &[true],
                42,
            ),
            (
                "unsupported_transfer",
                r#"
pub type Wrapped { Wrapped(String) }
fn choose(value: Wrapped, keep: List(Wrapped)) {
  case value {
    Wrapped("x") -> case keep { [Wrapped("y")] -> 42 _ -> 1 }
    _ -> case keep { [Wrapped("y")] -> 7 _ -> 0 }
  }
}
pub fn main() { choose(Wrapped("x"), [Wrapped("y")]) }
"#,
                &[false, true, true],
                42,
            ),
        ] {
            let typed = crate::compile_typed_host_program(
                "example",
                "example",
                [PackageSource::new(
                    "example",
                    Vec::<String>::new(),
                    [ModuleSource::new("example", "src/example.gleam", source)],
                )],
                if name == "native_transfer" {
                    HostProviderSet::<StatelessHostProfile>::from_providers([
                        HostProviderModule::new("example", "example")
                            .unwrap()
                            .with_function::<(BigInt,), BigInt, _>("identity", |value: BigInt| {
                                value
                            })
                            .unwrap(),
                    ])
                    .unwrap()
                } else {
                    HostProviderSet::<StatelessHostProfile>::new([]).unwrap()
                },
            )
            .unwrap();
            let mut execution = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let plan = &execution.execution;
            let types = CallTypes {
                custom_types: &plan.program.common.custom_types,
                value_shapes: &plan.program.common.value_shapes,
            };
            let mut decisions = Vec::new();
            for function in plan.program.functions.value_returns.int_functions.iter() {
                if let ExecutionFunctionRef::Graph(function) = function.as_ref() {
                    let graph = function.body().block_graph().as_view();
                    for block in graph.blocks() {
                        if let Terminator::Match(matcher) = block.terminator() {
                            let parameters = graph.block(matcher.success().target()).params();
                            let locals = block
                                .params()
                                .iter()
                                .map(|slot| slot.local())
                                .chain(block.instructions().iter().flat_map(|instruction| {
                                    instruction.outputs().map(|slot| slot.local())
                                }))
                                .filter_map(|local| types.local(local))
                                .collect::<Vec<_>>();
                            let compound =
                                CompoundMatch::inspect(matcher, &types, &locals, parameters);
                            decisions.push(compound.is_some());
                            if let Some(compound) = compound {
                                let terminator = CallTerminator::Match(Box::new(compound));
                                for index in 0..graph.blocks().count() {
                                    let block = BlockId(index);
                                    assert_eq!(
                                        terminator.enters(block),
                                        block == matcher.success().target()
                                            || block == matcher.failure().target(),
                                        "{name}: block {index}"
                                    );
                                }
                            }
                        }
                    }
                }
            }
            assert_eq!(decisions, expected, "{name}");
            let mut echo = Vec::new();
            assert_eq!(
                crate::execution_fixture::run(&mut execution, &mut (), &mut echo).unwrap(),
                crate::Value::Int(result.into()),
                "{name}"
            );
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn success_columns_clone_owned_bindings_only_before_the_last_use() {
        use super::{CompoundArgument, success_fields};
        let repeated = [CompoundArgument::Binding(3), CompoundArgument::Binding(3)];
        assert_eq!(
            success_fields(
                &[
                    CallLocal::String(StringLocalId(0)),
                    CallLocal::String(StringLocalId(1))
                ],
                &repeated,
            ),
            "string0: binding3.clone(), string1: binding3",
        );
        assert_eq!(
            success_fields(
                &[CallLocal::Int(IntLocalId(0)), CallLocal::Int(IntLocalId(1))],
                &repeated
            ),
            "int0: binding3, int1: binding3",
        );
        assert_eq!(
            success_fields(
                &[CallLocal::String(StringLocalId(0))],
                &[CompoundArgument::Value(CallLocal::String(StringLocalId(4)))],
            ),
            "string0: string4.clone()",
        );
    }

    #[test]
    fn scalar_subjects_and_tuple_list_leaves_remain_outside_compound_match_grammar() {
        use super::CompoundMatch;
        use crate::plan::execution::graph::{Edge, Match, MatchEdge, ParamLocal, Transfer};
        use crate::plan::execution::type_::ListTypeId;
        let types = CustomTypeTable {
            types: Vec::new().into(),
            definitions: Vec::new().into(),
        };
        let shapes = ValueShapeTable {
            shapes: Vec::new().into(),
            shape_types: Vec::new().into(),
            custom_shapes: Vec::new().into(),
        };
        for subject in [
            ParamLocal::Int(IntLocalId(0)),
            ParamLocal::Tuple {
                local: TupleLocalId(0),
                type_: vec![ValueType::List(ListTypeId(0))].into(),
            },
        ] {
            let grammar = Match::new(
                subject,
                MatchPattern::Discard,
                MatchEdge::new(
                    BlockId(1),
                    Vec::new(),
                    Vec::new(),
                    Transfer {
                        families: Vec::new().into(),
                    },
                ),
                Edge::new(
                    BlockId(2),
                    Vec::new(),
                    Transfer {
                        families: Vec::new().into(),
                    },
                ),
            );
            assert!(
                CompoundMatch::inspect(
                    &grammar,
                    &CallTypes {
                        custom_types: &types,
                        value_shapes: &shapes
                    },
                    &[],
                    &[],
                )
                .is_none()
            );
        }
    }

    #[test]
    fn field_read_emission_keeps_each_concrete_family_and_exact_nominal_type() {
        assert!(
            FieldRead::inspect(&CallLocal::IntFunction {
                local: IntFunctionLocalId(4),
                type_: FunctionType::new(Vec::new(), ValueType::Int),
            })
            .is_none(),
            "callable fields keep their canonical owner"
        );
        for (local, type_, expression) in [
            (
                CallLocal::Int(IntLocalId(4)),
                ValueType::Int,
                "field.integer()",
            ),
            (
                CallLocal::Float(FloatLocalId(4)),
                ValueType::Float,
                "field.float()",
            ),
            (
                CallLocal::Bool(BoolLocalId(4)),
                ValueType::Bool,
                "field.boolean()",
            ),
            (CallLocal::Nil(NilLocalId(4)), ValueType::Nil, "field.nil()"),
            (
                CallLocal::UtfCodepoint(UtfCodepointLocalId(4)),
                ValueType::UtfCodepoint,
                "field.utf_codepoint()",
            ),
            (
                CallLocal::String(StringLocalId(4)),
                ValueType::String,
                "field.string()",
            ),
            (
                CallLocal::BitArray(BitArrayLocalId(4)),
                ValueType::BitArray,
                "field.bit_array()",
            ),
            (
                CallLocal::Tuple {
                    local: TupleLocalId(4),
                    type_: vec![ValueType::Int, ValueType::Bool].into(),
                },
                ValueType::Tuple(vec![ValueType::Int, ValueType::Bool].into()),
                "field.tuple()",
            ),
            (
                CallLocal::Custom(CustomLocalShape {
                    local: CustomLocal {
                        id: CustomLocalId(4),
                        shape: CustomValueShape {
                            type_id: CustomTypeId(2),
                            shape_id: CustomValueShapeId(3),
                        },
                    },
                    arguments: vec![],
                    constructors: vec![CustomConstructorId {
                        type_id: CustomTypeId(2),
                        index: 1,
                    }],
                }),
                ValueType::Custom(CustomTypeId(2)),
                "field.custom()",
            ),
        ] {
            let read = FieldRead::inspect(&local).unwrap();
            assert_eq!(read.value_type(), type_);
            assert_eq!(read.expression("field"), expression);
            assert!(supported_type(&type_));
        }
        let fieldless = CallLocal::Nullary(CustomLocalShape {
            local: CustomLocal {
                id: CustomLocalId(4),
                shape: CustomValueShape {
                    type_id: CustomTypeId(2),
                    shape_id: CustomValueShapeId(3),
                },
            },
            arguments: vec![],
            constructors: vec![CustomConstructorId {
                type_id: CustomTypeId(2),
                index: 1,
            }],
        });
        let read = FieldRead::inspect(&fieldless).unwrap();
        assert_eq!(read.value_type(), ValueType::Custom(CustomTypeId(2)));
        assert_eq!(
            read.expression("field"),
            r#"field.nullary(&[
    data::type_::CustomConstructorId {
        type_id: data::type_::CustomTypeId(2),
        index: 1,
    },
])"#
        );
    }

    #[test]
    fn root_bindings_project_a_fieldless_refinement_and_restore_its_nominal_carrier() {
        let constructor = CustomConstructorId {
            type_id: CustomTypeId(2),
            index: 1,
        };
        let shape = CustomLocalShape {
            local: CustomLocal {
                id: CustomLocalId(4),
                shape: CustomValueShape {
                    type_id: CustomTypeId(2),
                    shape_id: CustomValueShapeId(3),
                },
            },
            arguments: Vec::new(),
            constructors: vec![constructor],
        };
        let nullary = CallLocal::Nullary(shape.clone());
        let mut unrestricted = shape;
        unrestricted.local.shape.shape_id = CustomValueShapeId(4);
        unrestricted.constructors.push(CustomConstructorId {
            type_id: CustomTypeId(2),
            index: 0,
        });
        let custom = CallLocal::Custom(unrestricted);
        for (root, local, name, expected) in [
            (
                &custom,
                &nullary,
                "custom4",
                "let binding0 = custom4.nullary(&[\n    data::type_::CustomConstructorId {\n        type_id: data::type_::CustomTypeId(2),\n        index: 1,\n    },\n])?;\n",
            ),
            (
                &nullary,
                &custom,
                "nullary4",
                "let binding0 = nullary4.into();\n",
            ),
        ] {
            let bindings = BTreeMap::from([(
                0,
                CompoundBinding {
                    local: local.clone(),
                    read: FieldRead::inspect(local).unwrap(),
                },
            )]);
            let mut source = Code::default();
            write_pattern(
                &mut source,
                &CompoundPattern::Bind(0),
                name,
                Some(root),
                &bindings,
                &mut 0,
            );
            assert_eq!(source.as_str(), expected);
        }
    }

    #[test]
    fn literal_pattern_emission_distinguishes_refutable_mismatch_from_unsupported_value() {
        for (pattern, expected) in [
            (
                CompoundPattern::Int(-7),
                "if field.integer()? != -7_i128 { return Some(None); }\n",
            ),
            (
                CompoundPattern::Float(-0.0),
                "if field.float()? != f64::from_bits(9223372036854775808) { return Some(None); }\n",
            ),
            (
                CompoundPattern::Bool(false),
                "if field.boolean()? { return Some(None); }\n",
            ),
            (
                CompoundPattern::String("한\"글".into()),
                "if field.string()?.as_bytes() != \"한\\\"글\".as_bytes() { return Some(None); }\n",
            ),
            (CompoundPattern::Nil, "field.nil()?;\n"),
            (CompoundPattern::Discard, ""),
            // An unused binding has no destination read or owned escape.
            (CompoundPattern::Bind(0), ""),
        ] {
            let mut source = Code::default();
            write_pattern(
                &mut source,
                &pattern,
                "field",
                None,
                &BTreeMap::new(),
                &mut 0,
            );
            assert_eq!(source.as_str(), expected);
        }
    }

    #[test]
    fn nested_tuple_emission_skips_discarded_fields_and_binds_alias_only_after_success() {
        let local = CallLocal::Tuple {
            local: TupleLocalId(2),
            type_: vec![
                ValueType::Nil,
                ValueType::Tuple(vec![ValueType::Bool, ValueType::String].into()),
            ]
            .into(),
        };
        let bindings = BTreeMap::from([
            (
                0,
                CompoundBinding {
                    local: local.clone(),
                    read: FieldRead::inspect(&local).unwrap(),
                },
            ),
            (
                1,
                CompoundBinding {
                    local: CallLocal::String(StringLocalId(3)),
                    read: FieldRead::String,
                },
            ),
        ]);
        let pattern = CompoundPattern::Alias {
            pattern: Box::new(CompoundPattern::Tuple(vec![
                CompoundPattern::Discard,
                CompoundPattern::Tuple(vec![CompoundPattern::Bool(true), CompoundPattern::Bind(1)]),
            ])),
            binding: 0,
        };
        let mut source = Code::default();
        write_pattern(
            &mut source,
            &pattern,
            "tuple2",
            Some(&local),
            &bindings,
            &mut 0,
        );
        assert_eq!(
            source.as_str(),
            "if tuple2.len() != 2 { return Some(None); }\n\
let field0 = tuple2.field(1)?;\n\
if field0.tuple_len()? != 2 { return Some(None); }\n\
let field1 = field0.field(0)?;\n\
if !field1.boolean()? { return Some(None); }\n\
let field2 = field0.field(1)?;\n\
let binding1 = field2.string()?;\n\
let binding0 = tuple2.clone();\n"
        );
    }
}
