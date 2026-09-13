use super::{write_call, write_function_call, write_projection};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::UtfCodepointFunctionId;
use crate::plan::execution::graph::{
    CustomLocal, ParamLocal, TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointListLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub enum UtfCodepointInstruction {
    Call {
        function: UtfCodepointFunctionId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: UtfCodepointFunctionLocalId,
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
        list: UtfCodepointListLocalId,
        index: usize,
    },
}

impl Explain for UtfCodepointInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            UtfCodepointInstruction::Call { function, args, .. } => {
                write_call(output, "utf_codepoint.call", function, args);
            }
            UtfCodepointInstruction::FunctionCall { function, args, .. } => {
                write_function_call(output, "utf_codepoint.function_call", function, args);
            }
            UtfCodepointInstruction::TupleIndex { tuple, index } => {
                write_projection(output, "utf_codepoint.tuple_index", tuple, *index);
            }
            UtfCodepointInstruction::CustomField { source, index } => {
                write_projection(output, "utf_codepoint.custom_field", source, *index);
            }
            UtfCodepointInstruction::ListIndex { list, index } => {
                write_projection(output, "utf_codepoint.list_index", list, *index);
            }
        }
    }
}

impl Emit for UtfCodepointInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::UtfCodepointInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::UtfCodepointInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::UtfCodepointInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::UtfCodepointInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::UtfCodepointInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::UtfCodepointInstruction;
    use crate::plan::execution::function::UtfCodepointFunctionId;
    use crate::plan::execution::graph::{
        CustomLocal, CustomLocalId, ParamLocal, TupleLocalId, UtfCodepointFunctionLocalId,
        UtfCodepointListLocalId, UtfCodepointLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{CustomTypeId, CustomValueShape, CustomValueShapeId};
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn emits_every_utf_codepoint_instruction_with_its_operands_and_source_site() {
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let cases = [
            (
                UtfCodepointInstruction::Call {
                    function: UtfCodepointFunctionId(2),
                    args: vec![ParamLocal::UtfCodepoint(UtfCodepointLocalId(5))].into(),
                    site: site.clone(),
                },
                concat!(
                    "data::graph::UtfCodepointInstruction::Call {function: data::function::UtfCodepointFunctionId(2,),",
                    "args: data::Storage::Static(&[data::graph::ParamLocal::UtfCodepoint(data::graph::UtfCodepointLocalId(5,),),]),",
                    "site: data::source::HostCallSite::from_static(\"example\",\"main\",",
                    "data::source::SourceSpan::new(3,8,),),}"
                ),
            ),
            (
                UtfCodepointInstruction::FunctionCall {
                    function: UtfCodepointFunctionLocalId(2),
                    args: vec![ParamLocal::UtfCodepoint(UtfCodepointLocalId(5))].into(),
                    site,
                },
                concat!(
                    "data::graph::UtfCodepointInstruction::FunctionCall {function: data::graph::UtfCodepointFunctionLocalId(2,),",
                    "args: data::Storage::Static(&[data::graph::ParamLocal::UtfCodepoint(data::graph::UtfCodepointLocalId(5,),),]),",
                    "site: data::source::HostCallSite::from_static(\"example\",\"main\",",
                    "data::source::SourceSpan::new(3,8,),),}"
                ),
            ),
            (
                UtfCodepointInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                "data::graph::UtfCodepointInstruction::TupleIndex {tuple: data::graph::TupleLocalId(2,),index: 1,}",
            ),
            (
                UtfCodepointInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                concat!(
                    "data::graph::UtfCodepointInstruction::CustomField {source: data::graph::CustomLocal {",
                    "id: data::graph::CustomLocalId(2,),shape: data::type_::CustomValueShape {",
                    "type_id: data::type_::CustomTypeId(3,),shape_id: data::type_::CustomValueShapeId(4,),},},index: 1,}"
                ),
            ),
            (
                UtfCodepointInstruction::ListIndex {
                    list: UtfCodepointListLocalId(2),
                    index: 1,
                },
                "data::graph::UtfCodepointInstruction::ListIndex {list: data::graph::UtfCodepointListLocalId(2,),index: 1,}",
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
    use crate::plan::execution::function::TupleFunctionId;
    use crate::plan::execution::graph::ProfiledInstructionKind;

    #[test]
    fn writes_utf_codepoint_calls_and_projections() {
        let source = r#"
pub type Holder {
  Holder(value: UtfCodepoint)
}

fn point() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn point_value(value: UtfCodepoint) { value }
fn point_values(values: List(UtfCodepoint)) { values }

pub fn main() {
  let scalar = point()
  let function = point_value
  let values = point_values([scalar])
  let selected = case values {
    [value, ..] -> value
    _ -> scalar
  }
  let tuple = #(scalar)
  let holder = Holder(scalar)
  #(
    point_value(scalar),
    function(scalar),
    tuple.0,
    holder.value,
    selected,
  )
}
"#;
        let expected = concat!(
            "utf_codepoint.call utf_codepoint#0 args=[] | ",
            "utf_codepoint.list_index %list.utf_codepoint#0 index=0 | ",
            "utf_codepoint.call utf_codepoint#1 args=[%utf_codepoint#1] | ",
            "utf_codepoint.function_call %function.utf_codepoint#0 ",
            "args=[%utf_codepoint#1] | ",
            "utf_codepoint.tuple_index %tuple#0 index=0 | ",
            "utf_codepoint.custom_field %custom#0 index=0",
        );

        assert_explanation(source, expected);
    }

    fn write_separator(output: &mut String, first: &mut bool) {
        if *first {
            *first = false;
        } else {
            output.push_str(" | ");
        }
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan.tuple_function(TupleFunctionId(0)).body().block_graph();
            let mut first = true;
            for instruction in graph.blocks().flat_map(|block| block.instructions()) {
                if let ProfiledInstructionKind::UtfCodepoint(instruction) = instruction.kind() {
                    write_separator(output, &mut first);
                    let mut context = explain::ExplainContext::new(plan, output);
                    context.write(instruction);
                }
            }
        });
    }
}
