use super::super::super::value::write_local_labels;
use super::{write_call, write_constant, write_function_call, write_projection};
use crate::plan::execution::constant::ConstantId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::TupleFunctionId;
use crate::plan::execution::graph::{
    CustomLocal, ParamLocal, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub enum TupleInstruction {
    Value(Table<ParamLocal>),
    Constant(ConstantId<TupleLocalId>),
    Call {
        function: TupleFunctionId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: TupleFunctionLocalId,
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
        list: TupleListLocalId,
        index: usize,
    },
}

impl Explain for TupleInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            TupleInstruction::Value(elements) => {
                output.push_str("tuple.value elements=");
                write_local_labels(output, elements);
            }
            TupleInstruction::Constant(id) => write_constant(output, "tuple", *id),
            TupleInstruction::Call { function, args, .. } => {
                write_call(output, "tuple.call", function, args);
            }
            TupleInstruction::FunctionCall { function, args, .. } => {
                write_function_call(output, "tuple.function_call", function, args);
            }
            TupleInstruction::TupleIndex { tuple, index } => {
                write_projection(output, "tuple.tuple_index", tuple, *index);
            }
            TupleInstruction::CustomField { source, index } => {
                write_projection(output, "tuple.custom_field", source, *index);
            }
            TupleInstruction::ListIndex { list, index } => {
                write_projection(output, "tuple.list_index", list, *index);
            }
        }
    }
}

impl Emit for TupleInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value(field_0) => output.call("graph::TupleInstruction::Value", &[field_0]),
            Self::Constant(field_0) => output.call("graph::TupleInstruction::Constant", &[field_0]),
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::TupleInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::TupleInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::TupleInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::TupleInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::TupleInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::TupleInstruction;
    use crate::plan::execution::constant::ConstantId;
    use crate::plan::execution::function::TupleFunctionId;
    use crate::plan::execution::graph::{
        CustomLocal, CustomLocalId, IntLocalId, ParamLocal, TupleFunctionLocalId, TupleListLocalId,
        TupleLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{CustomTypeId, CustomValueShape, CustomValueShapeId};
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn emits_every_tuple_instruction_with_its_operands_and_source_site() {
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let cases = [
            (
                TupleInstruction::Value(vec![ParamLocal::Int(IntLocalId(7))].into()),
                r#"
data::graph::TupleInstruction::Value(data::Storage::Static(&[
    data::graph::ParamLocal::Int(data::graph::IntLocalId(7)),
]))"#.trim_start_matches('\n'),
            ),
            (
                TupleInstruction::Constant(ConstantId::new(3)),
                r#"
data::graph::TupleInstruction::Constant(data::constant::ConstantId {
    index: 3,
    value: ::core::marker::PhantomData,
})"#.trim_start_matches('\n'),
            ),
            (
                TupleInstruction::Call {
                    function: TupleFunctionId(2),
                    args: vec![ParamLocal::Int(IntLocalId(5))].into(),
                    site: site.clone(),
                },
                r#"
data::graph::TupleInstruction::Call {
    function: data::function::TupleFunctionId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                TupleInstruction::FunctionCall {
                    function: TupleFunctionLocalId(2),
                    args: vec![ParamLocal::Int(IntLocalId(5))].into(),
                    site,
                },
                r#"
data::graph::TupleInstruction::FunctionCall {
    function: data::graph::TupleFunctionLocalId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                TupleInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                r#"
data::graph::TupleInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                TupleInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                r#"
data::graph::TupleInstruction::CustomField {
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
                TupleInstruction::ListIndex {
                    list: TupleListLocalId(2),
                    index: 1,
                },
                r#"
data::graph::TupleInstruction::ListIndex {
    list: data::graph::TupleListLocalId(2),
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
    use crate::plan::execution::function::TupleFunctionId;
    use crate::plan::execution::graph::ProfiledInstructionKind;

    #[test]
    fn writes_tuple_construction() {
        let source = "pub fn main() { #(1, True) }";
        let expected = "tuple.value elements=[%int#0, %bool#0]";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_tuple_constants_calls_and_projections() {
        let source = r#"
const saved = #(0)

pub type Holder {
  Holder(value: #(Int))
}

fn tuple_value(value: #(Int)) { value }
fn tuple_values(values: List(#(Int))) { values }

pub fn main() {
  let function = tuple_value
  let values = tuple_values([#(1)])
  let selected = case values {
    [value, ..] -> value
    _ -> #(2)
  }
  let nested = #(#(3))
  let holder = Holder(#(4))
  let ignored = #(
    saved,
    tuple_value(#(5)),
    function(#(6)),
    nested.0,
    holder.value,
  )
  selected
}
"#;
        let expected = concat!(
            "tuple.value elements=[%int#0] | tuple.list_index %list.tuple#0 index=0 | ",
            "tuple.value elements=[%int#0] | tuple.value elements=[%tuple#1] | ",
            "tuple.value elements=[%int#1] | constant.tuple#0 | ",
            "tuple.value elements=[%int#2] | tuple.call tuple#1 args=[%tuple#5] | ",
            "tuple.value elements=[%int#3] | ",
            "tuple.function_call %function.tuple#0 args=[%tuple#7] | ",
            "tuple.tuple_index %tuple#2 index=0 | tuple.custom_field %custom#0 index=0 | ",
            "tuple.value elements=[%tuple#4, %tuple#6, %tuple#8, %tuple#9, %tuple#10] | ",
            "tuple.value elements=[%int#0]",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan.tuple_function(TupleFunctionId(0)).body().block_graph();
            let mut first = true;
            for instruction in graph.blocks().flat_map(|block| block.instructions()) {
                if let ProfiledInstructionKind::Tuple(instruction) = instruction.kind() {
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
