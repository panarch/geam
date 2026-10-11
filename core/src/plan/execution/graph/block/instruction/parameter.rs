use super::{write_call, write_function_call, write_projection};
use crate::plan::HostCallSite;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::ParameterFunctionId;
use crate::plan::execution::graph::{
    CustomLocal, ParamLocal, ParameterFunctionLocalId, ParameterListLocalId, TupleLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub enum ParameterInstruction {
    Call {
        function: ParameterFunctionId,
        args: Table<ParamLocal>,
        site: HostCallSite,
    },
    FunctionCall {
        function: ParameterFunctionLocalId,
        args: Table<ParamLocal>,
        site: HostCallSite,
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
        list: ParameterListLocalId,
        index: usize,
    },
}

impl Explain for ParameterInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            ParameterInstruction::Call { function, args, .. } => {
                write_call(output, "parameter.call", function, args);
            }
            ParameterInstruction::FunctionCall { function, args, .. } => {
                write_function_call(output, "parameter.function_call", function, args);
            }
            ParameterInstruction::TupleIndex { tuple, index } => {
                write_projection(output, "parameter.tuple_index", tuple, *index);
            }
            ParameterInstruction::CustomField { source, index } => {
                write_projection(output, "parameter.custom_field", source, *index);
            }
            ParameterInstruction::ListIndex { list, index } => {
                write_projection(output, "parameter.list_index", list, *index);
            }
        }
    }
}

impl Emit for ParameterInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::ParameterInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::ParameterInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::ParameterInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::ParameterInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::ParameterInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ExplainContext, ParameterInstruction, Rust};
    use crate::plan::execution::function::ParameterFunctionId;
    use crate::plan::execution::graph::{
        CustomLocal, CustomLocalId, IntLocalId, ParamLocal, ParameterFunctionLocalId,
        ParameterListLocalId, TupleLocalId,
    };
    use crate::plan::execution::type_::{CustomTypeId, CustomValueShape, CustomValueShapeId};
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn parameter_instruction_formats_preserve_targets_operands_and_call_sites() {
        let module =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { Nil }")
                .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(module).unwrap());
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let cases = [
            (
                ParameterInstruction::Call {
                    function: ParameterFunctionId(2),
                    args: vec![ParamLocal::Int(IntLocalId(5))].into(),
                    site: site.clone(),
                },
                "parameter.call parameter#2 args=[%int#5]",
                r#"
data::graph::ParameterInstruction::Call {
    function: data::function::ParameterFunctionId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#,
            ),
            (
                ParameterInstruction::FunctionCall {
                    function: ParameterFunctionLocalId(2),
                    args: vec![ParamLocal::Int(IntLocalId(5))].into(),
                    site,
                },
                "parameter.function_call %function.parameter#2 args=[%int#5]",
                r#"
data::graph::ParameterInstruction::FunctionCall {
    function: data::graph::ParameterFunctionLocalId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#,
            ),
            (
                ParameterInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                "parameter.tuple_index %tuple#2 index=1",
                r#"
data::graph::ParameterInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#,
            ),
            (
                ParameterInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                "parameter.custom_field %custom#2 index=1",
                r#"
data::graph::ParameterInstruction::CustomField {
    source: data::graph::CustomLocal {
        id: data::graph::CustomLocalId(2),
        shape: data::type_::CustomValueShape {
            type_id: data::type_::CustomTypeId(3),
            shape_id: data::type_::CustomValueShapeId(4),
        },
    },
    index: 1,
}"#,
            ),
            (
                ParameterInstruction::ListIndex {
                    list: ParameterListLocalId(2),
                    index: 1,
                },
                "parameter.list_index %list.parameter#2 index=1",
                r#"
data::graph::ParameterInstruction::ListIndex {
    list: data::graph::ParameterListLocalId(2),
    index: 1,
}"#,
            ),
        ];
        for (instruction, explanation, emission) in cases {
            let mut output = String::new();
            ExplainContext::new(&plan, &mut output).write(&instruction);
            assert_eq!(output, explanation);
            assert_eq!(
                Rust::expression(&instruction),
                emission.trim_start_matches('\n')
            );
        }
    }
}
