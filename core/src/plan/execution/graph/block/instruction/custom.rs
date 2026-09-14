use super::super::super::value::write_local_labels;
use super::{write_call, write_constant, write_function_call, write_projection};
use crate::plan::execution::constant::ConstantId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::CustomFunctionId;
use crate::plan::execution::graph::{
    CustomFunctionLocal, CustomListLocalId, CustomLocal, ParamLocal, TupleLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::CustomConstructorId;

#[derive(Clone)]
pub enum CustomInstruction {
    Construct {
        constructor: CustomConstructorId,
        fields: Table<ParamLocal>,
    },
    Constant(ConstantId<CustomLocal>),
    Call {
        function: CustomFunctionId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: CustomFunctionLocal,
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
        list: CustomListLocalId,
        index: usize,
    },
}

impl Explain for CustomInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            CustomInstruction::Construct {
                constructor,
                fields,
            } => {
                output.push_str("custom.construct custom_type#");
                output.push_str(&constructor.type_id().index().to_string());
                output.push_str(".constructor#");
                output.push_str(&constructor.index().to_string());
                output.push_str(" fields=");
                write_local_labels(output, fields);
            }
            CustomInstruction::Constant(id) => write_constant(output, "custom", *id),
            CustomInstruction::Call { function, args, .. } => {
                write_call(output, "custom.call", function, args);
            }
            CustomInstruction::FunctionCall { function, args, .. } => {
                write_function_call(output, "custom.function_call", function, args);
            }
            CustomInstruction::TupleIndex { tuple, index } => {
                write_projection(output, "custom.tuple_index", tuple, *index);
            }
            CustomInstruction::CustomField { source, index } => {
                write_projection(output, "custom.custom_field", source, *index);
            }
            CustomInstruction::ListIndex { list, index } => {
                write_projection(output, "custom.list_index", list, *index);
            }
        }
    }
}

impl Emit for CustomInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Construct {
                constructor,
                fields,
            } => output.structure(
                "graph::CustomInstruction::Construct",
                &[("constructor", constructor), ("fields", fields)],
            ),
            Self::Constant(field_0) => {
                output.call("graph::CustomInstruction::Constant", &[field_0])
            }
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::CustomInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::CustomInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::CustomInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::CustomInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::CustomInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::CustomInstruction;
    use crate::plan::execution::constant::ConstantId;
    use crate::plan::execution::function::CustomFunctionId;
    use crate::plan::execution::graph::CustomFunctionLocal;
    use crate::plan::execution::graph::{
        CustomFunctionLocalId, CustomListLocalId, CustomLocal, CustomLocalId, IntLocalId,
        ParamLocal, TupleLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{
        CustomConstructorId, CustomFunctionType, CustomTypeId, CustomValueShape,
        CustomValueShapeId, FunctionType, ValueType,
    };
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn emits_every_custom_instruction_with_its_operands_and_source_site() {
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let shape = CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4));
        let cases = [
            (
                CustomInstruction::Construct {
                    constructor: CustomConstructorId::new(CustomTypeId(3), 1),
                    fields: vec![ParamLocal::Int(IntLocalId(5))].into(),
                },
                r#"
data::graph::CustomInstruction::Construct {
    constructor: data::type_::CustomConstructorId {
        type_id: data::type_::CustomTypeId(3),
        index: 1,
    },
    fields: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
    ]),
}"#.trim_start_matches('\n'),
            ),
            (
                CustomInstruction::Constant(ConstantId::new(3)),
                r#"
data::graph::CustomInstruction::Constant(data::constant::ConstantId {
    index: 3,
    value: ::core::marker::PhantomData,
})"#.trim_start_matches('\n'),
            ),
            (
                CustomInstruction::Call {
                    function: CustomFunctionId::new(2, shape),
                    args: vec![ParamLocal::Int(IntLocalId(5))].into(),
                    site: site.clone(),
                },
                r#"
data::graph::CustomInstruction::Call {
    function: data::function::CustomFunctionId {
        index: 2,
        return_shape: data::type_::CustomValueShape {
            type_id: data::type_::CustomTypeId(3),
            shape_id: data::type_::CustomValueShapeId(4),
        },
    },
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                CustomInstruction::FunctionCall {
                    function: CustomFunctionLocal::new(
                        CustomFunctionLocalId(2),
                        CustomFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::Custom(CustomTypeId(3))),
                            Vec::new(),
                            shape,
                        ),
                    ),
                    args: vec![ParamLocal::Int(IntLocalId(5))].into(),
                    site,
                },
                r#"
data::graph::CustomInstruction::FunctionCall {
    function: data::graph::CustomFunctionLocal {
        id: data::graph::CustomFunctionLocalId(2),
        type_: data::type_::CustomFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(3))),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::CustomValueShape {
                type_id: data::type_::CustomTypeId(3),
                shape_id: data::type_::CustomValueShapeId(4),
            },
        },
    },
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                CustomInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                r#"
data::graph::CustomInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                CustomInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                r#"
data::graph::CustomInstruction::CustomField {
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
                CustomInstruction::ListIndex {
                    list: CustomListLocalId(2),
                    index: 1,
                },
                r#"
data::graph::CustomInstruction::ListIndex {
    list: data::graph::CustomListLocalId(2),
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
    use crate::plan::execution::graph::ProfiledInstructionKind;

    #[test]
    fn writes_custom_construction() {
        let source = r#"
pub type Boxed { Boxed(Int) }
pub fn main() { Boxed(1) }
"#;
        let expected = "custom.construct custom_type#0.constructor#0 fields=[%int#0]";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_custom_constants_calls_and_projections() {
        let source = r#"
pub type Boxed {
  Boxed(Int)
}

pub type Holder {
  Holder(value: Boxed)
}

const saved = Boxed(0)

fn custom_value(value: Boxed) { value }
fn custom_values(values: List(Boxed)) { values }

pub fn main() {
  let function = custom_value
  let values = custom_values([Boxed(1)])
  let selected = case values {
    [value, ..] -> value
    _ -> Boxed(2)
  }
  let tuple = #(Boxed(3))
  let holder = Holder(Boxed(4))
  let ignored = #(
    saved,
    custom_value(Boxed(5)),
    function(Boxed(6)),
    tuple.0,
    holder.value,
  )
  selected
}
"#;
        let expected = concat!(
            "custom.construct custom_type#0.constructor#0 fields=[%int#0] | ",
            "custom.list_index %list.custom#0 index=0 | ",
            "custom.construct custom_type#0.constructor#0 fields=[%int#0] | ",
            "custom.construct custom_type#0.constructor#0 fields=[%int#1] | ",
            "custom.construct custom_type#1.constructor#0 fields=[%custom#2] | ",
            "constant.custom#0 | ",
            "custom.construct custom_type#0.constructor#0 fields=[%int#2] | ",
            "custom.call custom#1 args=[%custom#5] | ",
            "custom.construct custom_type#0.constructor#0 fields=[%int#3] | ",
            "custom.function_call %function.custom#0 args=[%custom#7] | ",
            "custom.tuple_index %tuple#0 index=0 | ",
            "custom.custom_field %custom#3 index=0 | ",
            "custom.construct custom_type#0.constructor#0 fields=[%int#0]",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let function = plan.custom_function(plan.custom_function_id(0));
            let graph = function.body().function_body().block_graph();
            let mut first = true;
            for instruction in graph.blocks().flat_map(|block| block.instructions()) {
                if let ProfiledInstructionKind::Custom(instruction) = instruction.kind() {
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
