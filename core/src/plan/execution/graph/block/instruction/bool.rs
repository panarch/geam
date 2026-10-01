use super::{write_call, write_constant, write_function_call, write_literal, write_projection};
use crate::plan::execution::constant::ConstantId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::BoolFunctionId;
use crate::plan::execution::graph::{
    BoolFunctionLocalId, BoolListLocalId, BoolLocalId, BoolTest, CustomLocal, ParamLocal,
    TupleLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub enum BoolInstruction {
    Value(bool),
    Constant(ConstantId<BoolLocalId>),
    Call {
        function: BoolFunctionId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: BoolFunctionLocalId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    TupleIndex {
        tuple: TupleLocalId,
        index: usize,
    },
    CustomField {
        source: CustomLocal,
        index: usize,
    },
    ListIndex {
        list: BoolListLocalId,
        index: usize,
    },
    Test(BoolTest),
}

impl Explain for BoolInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            BoolInstruction::Value(value) => {
                write_literal(output, "bool.value", if *value { "True" } else { "False" });
            }
            BoolInstruction::Constant(id) => write_constant(output, "bool", *id),
            BoolInstruction::Call { function, args, .. } => {
                write_call(output, "bool.call", function, args);
            }
            BoolInstruction::FunctionCall { function, args, .. } => {
                write_function_call(output, "bool.function_call", function, args);
            }
            BoolInstruction::TupleIndex { tuple, index } => {
                write_projection(output, "bool.tuple_index", tuple, *index);
            }
            BoolInstruction::CustomField { source, index } => {
                write_projection(output, "bool.custom_field", source, *index);
            }
            BoolInstruction::ListIndex { list, index } => {
                write_projection(output, "bool.list_index", list, *index);
            }
            BoolInstruction::Test(test) => test.write_explanation(context),
        }
    }
}

impl Emit for BoolInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value(field_0) => output.call("graph::BoolInstruction::Value", &[field_0]),
            Self::Constant(field_0) => output.call("graph::BoolInstruction::Constant", &[field_0]),
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::BoolInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::BoolInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::BoolInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::BoolInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::BoolInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
            Self::Test(test) => output.call("graph::BoolInstruction::Test", &[test]),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::BoolInstruction;
    use crate::plan::execution::constant::ConstantId;
    use crate::plan::execution::function::BoolFunctionId;
    use crate::plan::execution::graph::{
        BoolFunctionLocalId, BoolListLocalId, BoolLocalId, BoolTest, CustomLocal, CustomLocalId,
        ParamLocal, TupleLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{CustomTypeId, CustomValueShape, CustomValueShapeId};
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn emits_every_boolean_instruction_with_its_operands_and_source_site() {
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let cases = [
            (
                BoolInstruction::Value(true),
                "data::graph::BoolInstruction::Value(true)",
            ),
            (
                BoolInstruction::Constant(ConstantId::new(3)),
                r#"
data::graph::BoolInstruction::Constant(data::constant::ConstantId {
    index: 3,
    value: ::core::marker::PhantomData,
})"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::Call {
                    function: BoolFunctionId(2),
                    args: vec![ParamLocal::Bool(BoolLocalId(5))].into(),
                    site: site.clone(),
                },
                r#"
data::graph::BoolInstruction::Call {
    function: data::function::BoolFunctionId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::FunctionCall {
                    function: BoolFunctionLocalId(2),
                    args: vec![ParamLocal::Bool(BoolLocalId(5))].into(),
                    site,
                },
                r#"
data::graph::BoolInstruction::FunctionCall {
    function: data::graph::BoolFunctionLocalId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                r#"
data::graph::BoolInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                r#"
data::graph::BoolInstruction::CustomField {
    source: data::graph::CustomLocal {
        id: data::graph::CustomLocalId(2),
        shape: data::type_::CustomValueShape {
            type_id: data::type_::CustomTypeId(3),
            shape_id: data::type_::CustomValueShapeId(4),
        },
    },
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::ListIndex {
                    list: BoolListLocalId(2),
                    index: 1,
                },
                r#"
data::graph::BoolInstruction::ListIndex {
    list: data::graph::BoolListLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::Test(BoolTest::Not(BoolLocalId(2))),
                "data::graph::BoolInstruction::Test(data::graph::BoolTest::Not(data::graph::BoolLocalId(2)))",
            ),
        ];
        for (instruction, expected) in cases {
            assert_eq!(Rust::expression(&instruction), expected);
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::BoolFunctionId;
    use crate::plan::execution::graph::{ProfiledInstructionKind, Terminator};
    #[test]
    fn writes_bool_constants_calls_projections_and_pattern_checks() {
        let source = r#"
const saved = True

pub type Flag {
  Flag(value: Bool)
}

fn bool_value(value: Bool) { value }
fn bool_values(values: List(Bool)) { values }
fn string_value(value: String) { value }

pub fn main() {
  let function = bool_value
  let values = bool_values([True])
  let selected = case values {
    [value, ..] -> value
    _ -> False
  }
  let exact = case values {
    [value] -> value
    _ -> False
  }
  let tuple = #(True)
  let record = Flag(True)
  let text = string_value("prefix-tail")
  let prefix = case text {
    "prefix-" <> _ -> True
    _ -> False
  }

  saved
  && bool_value(True)
  && function(True)
  && tuple.0
  && record.value
  && selected
  && exact
  && prefix
}
"#;
        let expected = concat!(
            "bool.value True | bool.list_length_at_least %list.bool#1 length=1 | ",
            "bool.list_index %list.bool#0 index=0 | bool.value True | bool.value True | ",
            "bool.list_length_equals %list.bool#0 length=1 | ",
            "bool.list_index %list.bool#0 index=0 | bool.value True | bool.value True | ",
            "bool.value True | bool.value True | ",
            "bool.string_starts_with %string#1 prefix=\"prefix-\" | bool.value True | ",
            "bool.value True | constant.bool#0 | bool.value True | ",
            "bool.call bool#1 args=[%bool#3] | bool.value True | ",
            "bool.function_call %function.bool#0 args=[%bool#3] | ",
            "bool.tuple_index %tuple#0 index=0 | bool.custom_field %custom#0 index=0 | ",
            "bool.value True | bool.value False | bool.value False | bool.value False | ",
            "bool.value False | bool.value False | bool.value False | bool.value False | ",
            "bool.value False | bool.value False",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan.bool_function(BoolFunctionId(0)).body().block_graph();
            let mut parts = Vec::new();
            for block in graph.blocks() {
                for instruction in block.instructions() {
                    if let ProfiledInstructionKind::Bool(instruction) = instruction.kind() {
                        let mut part = String::new();
                        let mut context = explain::ExplainContext::new(plan, &mut part);
                        context.write(instruction);
                        parts.push(part);
                    }
                }
                if let Terminator::TestBranch(branch) = block.terminator() {
                    let mut part = String::new();
                    let mut context = explain::ExplainContext::new(plan, &mut part);
                    context.write(&branch.test);
                    parts.push(part);
                }
            }
            output.push_str(&parts.join(" | "));
        });
    }
}
