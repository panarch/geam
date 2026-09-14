use super::{
    write_binary, write_call, write_constant, write_function_call, write_literal, write_projection,
};
use crate::plan::execution::constant::ConstantId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::FloatFunctionId;
use crate::plan::execution::graph::{
    CustomLocal, FloatFunctionLocalId, FloatListLocalId, FloatLocalId, ParamLocal, TupleLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub enum FloatInstruction {
    Value(f64),
    Constant(ConstantId<FloatLocalId>),
    Call {
        function: FloatFunctionId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: FloatFunctionLocalId,
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
        list: FloatListLocalId,
        index: usize,
    },
    Add {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    Sub {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    Mult {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    Div {
        left: FloatLocalId,
        right: FloatLocalId,
    },
}

impl Explain for FloatInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            FloatInstruction::Value(value) => {
                write_literal(output, "float.value", &format!("{value:?}"));
            }
            FloatInstruction::Constant(id) => write_constant(output, "float", *id),
            FloatInstruction::Call { function, args, .. } => {
                write_call(output, "float.call", function, args);
            }
            FloatInstruction::FunctionCall { function, args, .. } => {
                write_function_call(output, "float.function_call", function, args);
            }
            FloatInstruction::TupleIndex { tuple, index } => {
                write_projection(output, "float.tuple_index", tuple, *index);
            }
            FloatInstruction::CustomField { source, index } => {
                write_projection(output, "float.custom_field", source, *index);
            }
            FloatInstruction::ListIndex { list, index } => {
                write_projection(output, "float.list_index", list, *index);
            }
            FloatInstruction::Add { left, right } => write_binary(output, "float.add", left, right),
            FloatInstruction::Sub { left, right } => write_binary(output, "float.sub", left, right),
            FloatInstruction::Mult { left, right } => {
                write_binary(output, "float.mult", left, right);
            }
            FloatInstruction::Div { left, right } => write_binary(output, "float.div", left, right),
        }
    }
}

impl Emit for FloatInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value(field_0) => output.call("graph::FloatInstruction::Value", &[field_0]),
            Self::Constant(field_0) => output.call("graph::FloatInstruction::Constant", &[field_0]),
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::FloatInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::FloatInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::FloatInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::FloatInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::FloatInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
            Self::Add { left, right } => output.structure(
                "graph::FloatInstruction::Add",
                &[("left", left), ("right", right)],
            ),
            Self::Sub { left, right } => output.structure(
                "graph::FloatInstruction::Sub",
                &[("left", left), ("right", right)],
            ),
            Self::Mult { left, right } => output.structure(
                "graph::FloatInstruction::Mult",
                &[("left", left), ("right", right)],
            ),
            Self::Div { left, right } => output.structure(
                "graph::FloatInstruction::Div",
                &[("left", left), ("right", right)],
            ),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::FloatInstruction;
    use crate::plan::execution::constant::ConstantId;
    use crate::plan::execution::function::FloatFunctionId;
    use crate::plan::execution::graph::{
        CustomLocal, CustomLocalId, FloatFunctionLocalId, FloatListLocalId, FloatLocalId,
        ParamLocal, TupleLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{CustomTypeId, CustomValueShape, CustomValueShapeId};
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn emits_every_float_instruction_with_its_operands_and_source_site() {
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let cases = [
            (
                FloatInstruction::Value(-0.0),
                "data::graph::FloatInstruction::Value(f64::from_bits(9223372036854775808))",
            ),
            (
                FloatInstruction::Constant(ConstantId::new(3)),
                r#"
data::graph::FloatInstruction::Constant(data::constant::ConstantId {
    index: 3,
    value: ::core::marker::PhantomData,
})"#.trim_start_matches('\n'),
            ),
            (
                FloatInstruction::Call {
                    function: FloatFunctionId(2),
                    args: vec![ParamLocal::Float(FloatLocalId(5))].into(),
                    site: site.clone(),
                },
                r#"
data::graph::FloatInstruction::Call {
    function: data::function::FloatFunctionId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Float(data::graph::FloatLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                FloatInstruction::FunctionCall {
                    function: FloatFunctionLocalId(2),
                    args: vec![ParamLocal::Float(FloatLocalId(5))].into(),
                    site,
                },
                r#"
data::graph::FloatInstruction::FunctionCall {
    function: data::graph::FloatFunctionLocalId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Float(data::graph::FloatLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                FloatInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                r#"
data::graph::FloatInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                FloatInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                r#"
data::graph::FloatInstruction::CustomField {
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
                FloatInstruction::ListIndex {
                    list: FloatListLocalId(2),
                    index: 1,
                },
                r#"
data::graph::FloatInstruction::ListIndex {
    list: data::graph::FloatListLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                FloatInstruction::Add {
                    left: FloatLocalId(2),
                    right: FloatLocalId(5),
                },
                r#"
data::graph::FloatInstruction::Add {
    left: data::graph::FloatLocalId(2),
    right: data::graph::FloatLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                FloatInstruction::Sub {
                    left: FloatLocalId(2),
                    right: FloatLocalId(5),
                },
                r#"
data::graph::FloatInstruction::Sub {
    left: data::graph::FloatLocalId(2),
    right: data::graph::FloatLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                FloatInstruction::Mult {
                    left: FloatLocalId(2),
                    right: FloatLocalId(5),
                },
                r#"
data::graph::FloatInstruction::Mult {
    left: data::graph::FloatLocalId(2),
    right: data::graph::FloatLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                FloatInstruction::Div {
                    left: FloatLocalId(2),
                    right: FloatLocalId(5),
                },
                r#"
data::graph::FloatInstruction::Div {
    left: data::graph::FloatLocalId(2),
    right: data::graph::FloatLocalId(5),
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
    fn writes_float_arithmetic() {
        let source = r#"
pub fn main() {
  let value = 6.0
  #(
    value +. 2.0,
    value -. 2.0,
    value *. 2.0,
    value /. 2.0,
  )
}
"#;
        let expected = concat!(
            "float.value 6.0 | float.value 2.0 | float.add %float#0 %float#1 | ",
            "float.value 2.0 | float.sub %float#0 %float#3 | ",
            "float.value 2.0 | float.mult %float#0 %float#5 | ",
            "float.value 2.0 | float.div %float#0 %float#7",
        );

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_float_constants_calls_and_projections() {
        let source = r#"
const saved = 1.0

pub type Holder {
  Holder(value: Float)
}

fn float_value(value: Float) { value }
fn float_values(values: List(Float)) { values }

pub fn main() {
  let function = float_value
  let values = float_values([2.0])
  let selected = case values {
    [value, ..] -> value
    _ -> 0.0
  }
  let tuple = #(3.0)
  let holder = Holder(4.0)
  #(
    saved,
    float_value(5.0),
    function(6.0),
    tuple.0,
    holder.value,
    selected,
  )
}
"#;
        let expected = concat!(
            "float.value 2.0 | float.list_index %list.float#0 index=0 | ",
            "float.value 3.0 | float.value 4.0 | constant.float#0 | ",
            "float.value 5.0 | float.call float#0 args=[%float#4] | ",
            "float.value 6.0 | float.function_call %function.float#0 args=[%float#6] | ",
            "float.tuple_index %tuple#0 index=0 | ",
            "float.custom_field %custom#0 index=0 | float.value 0.0",
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
                if let ProfiledInstructionKind::Float(instruction) = instruction.kind() {
                    write_separator(output, &mut first);
                    let mut context = explain::ExplainContext::new(plan, output);
                    context.write(instruction);
                }
            }
        });
    }
}
