use super::{write_call, write_constant, write_function_call, write_projection};
use crate::plan::execution::constant::ConstantId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::NilFunctionId;
use crate::plan::execution::graph::{
    CustomLocal, NilFunctionLocalId, NilListLocalId, NilLocalId, ParamLocal, TupleLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub enum NilInstruction {
    Value,
    Constant(ConstantId<NilLocalId>),
    Call {
        function: NilFunctionId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: NilFunctionLocalId,
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
        list: NilListLocalId,
        index: usize,
    },
}

impl Explain for NilInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            NilInstruction::Value => output.push_str("nil.value"),
            NilInstruction::Constant(id) => write_constant(output, "nil", *id),
            NilInstruction::Call { function, args, .. } => {
                write_call(output, "nil.call", function, args)
            }
            NilInstruction::FunctionCall { function, args, .. } => {
                write_function_call(output, "nil.function_call", function, args);
            }
            NilInstruction::TupleIndex { tuple, index } => {
                write_projection(output, "nil.tuple_index", tuple, *index);
            }
            NilInstruction::CustomField { source, index } => {
                write_projection(output, "nil.custom_field", source, *index);
            }
            NilInstruction::ListIndex { list, index } => {
                write_projection(output, "nil.list_index", list, *index);
            }
        }
    }
}

impl Emit for NilInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value => output.path("graph::NilInstruction::Value"),
            Self::Constant(field_0) => output.call("graph::NilInstruction::Constant", &[field_0]),
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::NilInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::NilInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::NilInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::NilInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::NilInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::NilInstruction;
    use crate::plan::execution::constant::ConstantId;
    use crate::plan::execution::function::NilFunctionId;
    use crate::plan::execution::graph::{
        CustomLocal, CustomLocalId, NilFunctionLocalId, NilListLocalId, NilLocalId, ParamLocal,
        TupleLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{CustomTypeId, CustomValueShape, CustomValueShapeId};
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn emits_every_nil_instruction_with_its_operands_and_source_site() {
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let cases = [
            (NilInstruction::Value, "data::graph::NilInstruction::Value"),
            (
                NilInstruction::Constant(ConstantId::new(3)),
                r#"
data::graph::NilInstruction::Constant(data::constant::ConstantId {
    index: 3,
    value: ::core::marker::PhantomData,
})"#.trim_start_matches('\n'),
            ),
            (
                NilInstruction::Call {
                    function: NilFunctionId(2),
                    args: vec![ParamLocal::Nil(NilLocalId(5))].into(),
                    site: site.clone(),
                },
                r#"
data::graph::NilInstruction::Call {
    function: data::function::NilFunctionId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Nil(data::graph::NilLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                NilInstruction::FunctionCall {
                    function: NilFunctionLocalId(2),
                    args: vec![ParamLocal::Nil(NilLocalId(5))].into(),
                    site,
                },
                r#"
data::graph::NilInstruction::FunctionCall {
    function: data::graph::NilFunctionLocalId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Nil(data::graph::NilLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                NilInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                r#"
data::graph::NilInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                NilInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                r#"
data::graph::NilInstruction::CustomField {
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
                NilInstruction::ListIndex {
                    list: NilListLocalId(2),
                    index: 1,
                },
                r#"
data::graph::NilInstruction::ListIndex {
    list: data::graph::NilListLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
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
    use crate::plan::execution::function::NilFunctionId;
    use crate::plan::execution::graph::ProfiledInstructionKind;

    #[test]
    fn writes_nil_value() {
        let source = "pub fn main() { Nil }";
        let expected = "nil.value";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_nil_constants_calls_and_projections() {
        let source = r#"
const saved = Nil

pub type Holder {
  Holder(value: Nil)
}

fn nil_value(value: Nil) { value }
fn nil_values(values: List(Nil)) { values }

pub fn main() {
  let function = nil_value
  let values = nil_values([Nil])
  let selected = case values {
    [value, ..] -> value
    _ -> Nil
  }
  let tuple = #(Nil)
  let holder = Holder(Nil)
  let ignored = #(
    saved,
    nil_value(Nil),
    function(Nil),
    tuple.0,
    holder.value,
  )
  selected
}
"#;
        let expected = concat!(
            "nil.value | nil.list_index %list.nil#0 index=0 | nil.value | nil.value | ",
            "constant.nil#0 | nil.value | nil.call nil#1 args=[%nil#4] | nil.value | ",
            "nil.function_call %function.nil#0 args=[%nil#6] | ",
            "nil.tuple_index %tuple#0 index=0 | nil.custom_field %custom#0 index=0 | ",
            "nil.value",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan.nil_function(NilFunctionId(0)).body().block_graph();
            let mut first = true;
            for instruction in graph.blocks().flat_map(|block| block.instructions()) {
                if let ProfiledInstructionKind::Nil(instruction) = instruction.kind() {
                    if first {
                        first = false;
                    } else {
                        output.push_str(" | ");
                    }
                    let mut context = explain::ExplainContext::new(plan, output);
                    context.write(instruction);
                }
            }
        });
    }
}
