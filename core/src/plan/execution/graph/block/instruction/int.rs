use super::{
    write_binary, write_call, write_constant, write_function_call, write_literal, write_projection,
    write_unary,
};
use crate::plan::execution::constant::ConstantId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::IntFunctionId;
use crate::plan::execution::graph::IntegerLiteral;
use crate::plan::execution::graph::{
    CustomLocal, IntFunctionLocalId, IntListLocalId, IntLocalId, ParamLocal, TupleLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub enum IntInstruction {
    Value(IntegerLiteral),
    Constant(ConstantId<IntLocalId>),
    Call {
        function: IntFunctionId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: IntFunctionLocalId,
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
        list: IntListLocalId,
        index: usize,
    },
    Add {
        left: IntLocalId,
        right: IntLocalId,
    },
    Sub {
        left: IntLocalId,
        right: IntLocalId,
    },
    Mult {
        left: IntLocalId,
        right: IntLocalId,
    },
    Div {
        left: IntLocalId,
        right: IntLocalId,
    },
    Remainder {
        left: IntLocalId,
        right: IntLocalId,
    },
    Negate(IntLocalId),
}

impl Explain for IntInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            IntInstruction::Value(value) => write_literal(output, "int.value", &value.to_string()),
            IntInstruction::Constant(id) => write_constant(output, "int", *id),
            IntInstruction::Call { function, args, .. } => {
                write_call(output, "int.call", function, args)
            }
            IntInstruction::FunctionCall { function, args, .. } => {
                write_function_call(output, "int.function_call", function, args);
            }
            IntInstruction::TupleIndex { tuple, index } => {
                write_projection(output, "int.tuple_index", tuple, *index);
            }
            IntInstruction::CustomField { source, index } => {
                write_projection(output, "int.custom_field", source, *index);
            }
            IntInstruction::ListIndex { list, index } => {
                write_projection(output, "int.list_index", list, *index);
            }
            IntInstruction::Add { left, right } => write_binary(output, "int.add", left, right),
            IntInstruction::Sub { left, right } => write_binary(output, "int.sub", left, right),
            IntInstruction::Mult { left, right } => write_binary(output, "int.mult", left, right),
            IntInstruction::Div { left, right } => write_binary(output, "int.div", left, right),
            IntInstruction::Remainder { left, right } => {
                write_binary(output, "int.remainder", left, right);
            }
            IntInstruction::Negate(value) => write_unary(output, "int.negate", value),
        }
    }
}

impl Emit for IntInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value(field_0) => output.call("graph::IntInstruction::Value", &[field_0]),
            Self::Constant(field_0) => output.call("graph::IntInstruction::Constant", &[field_0]),
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::IntInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::IntInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::IntInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::IntInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::IntInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
            Self::Add { left, right } => output.structure(
                "graph::IntInstruction::Add",
                &[("left", left), ("right", right)],
            ),
            Self::Sub { left, right } => output.structure(
                "graph::IntInstruction::Sub",
                &[("left", left), ("right", right)],
            ),
            Self::Mult { left, right } => output.structure(
                "graph::IntInstruction::Mult",
                &[("left", left), ("right", right)],
            ),
            Self::Div { left, right } => output.structure(
                "graph::IntInstruction::Div",
                &[("left", left), ("right", right)],
            ),
            Self::Remainder { left, right } => output.structure(
                "graph::IntInstruction::Remainder",
                &[("left", left), ("right", right)],
            ),
            Self::Negate(field_0) => output.call("graph::IntInstruction::Negate", &[field_0]),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::IntInstruction;
    use crate::plan::execution::constant::ConstantId;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::{
        CustomLocal, CustomLocalId, IntFunctionLocalId, IntListLocalId, IntLocalId, ParamLocal,
        TupleLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{CustomTypeId, CustomValueShape, CustomValueShapeId};
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn emits_every_integer_instruction_with_its_operands_and_source_site() {
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let cases = [
            (
                IntInstruction::Value(num_bigint::BigInt::from(-42).into()),
                concat!(
                    "data::graph::IntInstruction::Value(data::graph::IntegerLiteral {",
                    "sign: data::Sign::Minus,digits: data::Storage::Static(&[42,]),},)"
                ),
            ),
            (
                IntInstruction::Constant(ConstantId::new(3)),
                concat!(
                    "data::graph::IntInstruction::Constant(data::constant::ConstantId {",
                    "index: 3,value: ::core::marker::PhantomData,},)"
                ),
            ),
            (
                IntInstruction::Call {
                    function: IntFunctionId(2),
                    args: vec![ParamLocal::Int(IntLocalId(5))].into(),
                    site: site.clone(),
                },
                concat!(
                    "data::graph::IntInstruction::Call {function: data::function::IntFunctionId(2,),",
                    "args: data::Storage::Static(&[data::graph::ParamLocal::Int(data::graph::IntLocalId(5,),),]),",
                    "site: data::source::HostCallSite::from_static(\"example\",\"main\",",
                    "data::source::SourceSpan::new(3,8,),),}"
                ),
            ),
            (
                IntInstruction::FunctionCall {
                    function: IntFunctionLocalId(2),
                    args: vec![ParamLocal::Int(IntLocalId(5))].into(),
                    site,
                },
                concat!(
                    "data::graph::IntInstruction::FunctionCall {function: data::graph::IntFunctionLocalId(2,),",
                    "args: data::Storage::Static(&[data::graph::ParamLocal::Int(data::graph::IntLocalId(5,),),]),",
                    "site: data::source::HostCallSite::from_static(\"example\",\"main\",",
                    "data::source::SourceSpan::new(3,8,),),}"
                ),
            ),
            (
                IntInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                "data::graph::IntInstruction::TupleIndex {tuple: data::graph::TupleLocalId(2,),index: 1,}",
            ),
            (
                IntInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                concat!(
                    "data::graph::IntInstruction::CustomField {source: data::graph::CustomLocal {",
                    "id: data::graph::CustomLocalId(2,),shape: data::type_::CustomValueShape {",
                    "type_id: data::type_::CustomTypeId(3,),shape_id: data::type_::CustomValueShapeId(4,),},},index: 1,}"
                ),
            ),
            (
                IntInstruction::ListIndex {
                    list: IntListLocalId(2),
                    index: 1,
                },
                "data::graph::IntInstruction::ListIndex {list: data::graph::IntListLocalId(2,),index: 1,}",
            ),
            (
                IntInstruction::Add {
                    left: IntLocalId(2),
                    right: IntLocalId(5),
                },
                "data::graph::IntInstruction::Add {left: data::graph::IntLocalId(2,),right: data::graph::IntLocalId(5,),}",
            ),
            (
                IntInstruction::Sub {
                    left: IntLocalId(2),
                    right: IntLocalId(5),
                },
                "data::graph::IntInstruction::Sub {left: data::graph::IntLocalId(2,),right: data::graph::IntLocalId(5,),}",
            ),
            (
                IntInstruction::Mult {
                    left: IntLocalId(2),
                    right: IntLocalId(5),
                },
                "data::graph::IntInstruction::Mult {left: data::graph::IntLocalId(2,),right: data::graph::IntLocalId(5,),}",
            ),
            (
                IntInstruction::Div {
                    left: IntLocalId(2),
                    right: IntLocalId(5),
                },
                "data::graph::IntInstruction::Div {left: data::graph::IntLocalId(2,),right: data::graph::IntLocalId(5,),}",
            ),
            (
                IntInstruction::Remainder {
                    left: IntLocalId(2),
                    right: IntLocalId(5),
                },
                "data::graph::IntInstruction::Remainder {left: data::graph::IntLocalId(2,),right: data::graph::IntLocalId(5,),}",
            ),
            (
                IntInstruction::Negate(IntLocalId(2)),
                "data::graph::IntInstruction::Negate(data::graph::IntLocalId(2,),)",
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
    fn writes_int_arithmetic() {
        let source = r#"
pub fn main() {
  let value = 6
  #(
    value + 2,
    value - 2,
    value * 2,
    value / 2,
    value % 2,
    -value,
  )
}
"#;
        let expected = concat!(
            "int.value 6 | int.value 2 | int.add %int#0 %int#1 | ",
            "int.value 2 | int.sub %int#0 %int#3 | ",
            "int.value 2 | int.mult %int#0 %int#5 | ",
            "int.value 2 | int.div %int#0 %int#7 | ",
            "int.value 2 | int.remainder %int#0 %int#9 | int.negate %int#0",
        );

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_int_constants_calls_and_projections() {
        let source = r#"
const saved = 1

pub type Holder {
  Holder(value: Int)
}

fn int_value(value: Int) { value }
fn int_values(values: List(Int)) { values }

pub fn main() {
  let function = int_value
  let values = int_values([2])
  let selected = case values {
    [value, ..] -> value
    _ -> 0
  }
  let tuple = #(3)
  let holder = Holder(4)
  #(
    saved,
    int_value(5),
    function(6),
    tuple.0,
    holder.value,
    selected,
  )
}
"#;
        let expected = concat!(
            "int.value 2 | int.list_index %list.int#0 index=0 | int.value 3 | ",
            "int.value 4 | constant.int#0 | int.value 5 | ",
            "int.call int#0 args=[%int#4] | int.value 6 | ",
            "int.function_call %function.int#0 args=[%int#6] | ",
            "int.tuple_index %tuple#0 index=0 | int.custom_field %custom#0 index=0 | ",
            "int.value 0",
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
                if let ProfiledInstructionKind::Int(instruction) = instruction.kind() {
                    write_separator(output, &mut first);
                    let mut context = explain::ExplainContext::new(plan, output);
                    context.write(instruction);
                }
            }
        });
    }
}
