use super::{write_call, write_function_call, write_projection};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::ExternalFunctionId;
use crate::plan::execution::graph::{
    CustomLocal, ExternalFunctionLocal, ExternalListLocalId, ParamLocal, TupleLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternalInstruction {
    Call {
        function: ExternalFunctionId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: ExternalFunctionLocal,
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
        list: ExternalListLocalId,
        index: usize,
    },
}

pub trait ExternalInstructionView {
    type Function;

    fn instruction_ref(&self) -> ExternalInstructionRef<'_, Self::Function>;
}

#[derive(Debug, PartialEq, Eq)]
pub enum ExternalInstructionRef<'instruction, Function> {
    Call {
        function: &'instruction Function,
        args: &'instruction [ParamLocal],
        site: &'instruction crate::plan::HostCallSite,
    },
    FunctionCall {
        function: &'instruction ExternalFunctionLocal,
        args: &'instruction [ParamLocal],
        site: &'instruction crate::plan::HostCallSite,
    },
    TupleIndex {
        tuple: TupleLocalId,
        index: usize,
    },
    CustomField {
        source: &'instruction CustomLocal,
        index: usize,
    },
    ListIndex {
        list: ExternalListLocalId,
        index: usize,
    },
}

impl ExternalInstructionView for ExternalInstruction {
    type Function = ExternalFunctionId;

    fn instruction_ref(&self) -> ExternalInstructionRef<'_, Self::Function> {
        match self {
            Self::Call {
                function,
                args,
                site,
            } => ExternalInstructionRef::Call {
                function,
                args,
                site,
            },
            Self::FunctionCall {
                function,
                args,
                site,
            } => ExternalInstructionRef::FunctionCall {
                function,
                args,
                site,
            },
            Self::TupleIndex { tuple, index } => ExternalInstructionRef::TupleIndex {
                tuple: *tuple,
                index: *index,
            },
            Self::CustomField { source, index } => ExternalInstructionRef::CustomField {
                source,
                index: *index,
            },
            Self::ListIndex { list, index } => ExternalInstructionRef::ListIndex {
                list: *list,
                index: *index,
            },
        }
    }
}

impl ExternalInstructionView for std::convert::Infallible {
    type Function = std::convert::Infallible;

    fn instruction_ref(&self) -> ExternalInstructionRef<'_, Self::Function> {
        match *self {}
    }
}

impl Explain for ExternalInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            Self::Call { function, args, .. } => {
                write_call(output, "external.call", function, args);
            }
            Self::FunctionCall { function, args, .. } => {
                write_function_call(output, "external.function_call", function, args);
            }
            Self::TupleIndex { tuple, index } => {
                write_projection(output, "external.tuple_index", tuple, *index);
            }
            Self::CustomField { source, index } => {
                write_projection(output, "external.custom_field", source, *index);
            }
            Self::ListIndex { list, index } => {
                write_projection(output, "external.list_index", list, *index);
            }
        }
    }
}

impl Emit for ExternalInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::ExternalInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::ExternalInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::ExternalInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::ExternalInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::ExternalInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::ExternalInstruction;
    use crate::plan::execution::function::ExternalFunctionId;
    use crate::plan::execution::graph::ExternalFunctionLocal;
    use crate::plan::execution::graph::{
        CustomLocal, CustomLocalId, ExternalFunctionLocalId, ExternalListLocalId, IntLocalId,
        ParamLocal, TupleLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{
        CustomTypeId, CustomValueShape, CustomValueShapeId, ExternalFunctionType, ExternalTypeId,
        FunctionType, ValueType,
    };
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn emits_every_external_instruction_with_its_operands_and_source_site() {
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let cases = [
            (
                ExternalInstruction::Call {
                    function: ExternalFunctionId::new(2, ExternalTypeId(3)),
                    args: vec![ParamLocal::Int(IntLocalId(5))].into(),
                    site: site.clone(),
                },
                r#"
data::graph::ExternalInstruction::Call {
    function: data::function::ExternalFunctionId {
        index: 2,
        return_type: data::type_::ExternalTypeId(3),
    },
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                ExternalInstruction::FunctionCall {
                    function: ExternalFunctionLocal::new(
                        ExternalFunctionLocalId(2),
                        ExternalFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::External(ExternalTypeId(3))),
                            Vec::new(),
                            ExternalTypeId(3),
                        ),
                    ),
                    args: vec![ParamLocal::Int(IntLocalId(5))].into(),
                    site,
                },
                r#"
data::graph::ExternalInstruction::FunctionCall {
    function: data::graph::ExternalFunctionLocal {
        id: data::graph::ExternalFunctionLocalId(2),
        type_: data::type_::ExternalFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(3))),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::ExternalTypeId(3),
        },
    },
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                ExternalInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                r#"
data::graph::ExternalInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                ExternalInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                r#"
data::graph::ExternalInstruction::CustomField {
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
                ExternalInstruction::ListIndex {
                    list: ExternalListLocalId(2),
                    index: 1,
                },
                r#"
data::graph::ExternalInstruction::ListIndex {
    list: data::graph::ExternalListLocalId(2),
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
mod view_tests {
    use super::{ExternalInstruction, ExternalInstructionRef, ExternalInstructionView, Table};
    use crate::plan::execution::function::ExternalFunctionId;
    use crate::plan::execution::graph::{
        CustomLocal, CustomLocalId, ExternalFunctionLocal, ExternalFunctionLocalId,
        ExternalListLocalId, IntLocalId, ParamLocal, TupleLocalId,
    };
    use crate::plan::execution::type_::{
        CustomTypeId, CustomValueShape, CustomValueShapeId, ExternalFunctionType, ExternalTypeId,
        FunctionType, ValueType,
    };

    #[test]
    fn exposes_every_external_instruction_variant() {
        let external_type = ExternalTypeId::new(0);
        let call_function = ExternalFunctionId::new(1, external_type);
        let call_args: Table<ParamLocal> = vec![ParamLocal::Int(IntLocalId(0))].into();
        let call_site = crate::plan::HostCallSite::unknown();
        let call = ExternalInstruction::Call {
            function: call_function,
            args: call_args.clone(),
            site: call_site.clone(),
        };
        assert_eq!(
            call.instruction_ref(),
            ExternalInstructionRef::Call {
                function: &call_function,
                args: &call_args,
                site: &call_site,
            },
        );

        let function_local = ExternalFunctionLocal::new(
            ExternalFunctionLocalId(2),
            ExternalFunctionType::from_shapes(
                FunctionType::new(Vec::new(), ValueType::External(external_type)),
                Vec::new(),
                external_type,
            ),
        );
        let function_args: Table<ParamLocal> = Vec::new().into();
        let function_site = crate::plan::HostCallSite::unknown();
        let function_call = ExternalInstruction::FunctionCall {
            function: function_local.clone(),
            args: function_args.clone(),
            site: function_site.clone(),
        };
        assert_eq!(
            function_call.instruction_ref(),
            ExternalInstructionRef::FunctionCall {
                function: &function_local,
                args: &function_args,
                site: &function_site,
            },
        );

        let tuple_index = ExternalInstruction::TupleIndex {
            tuple: TupleLocalId(3),
            index: 4,
        };
        assert_eq!(
            tuple_index.instruction_ref(),
            ExternalInstructionRef::TupleIndex {
                tuple: TupleLocalId(3),
                index: 4,
            },
        );

        let custom_source = CustomLocal::new(
            CustomLocalId(5),
            CustomValueShape::new(CustomTypeId::new(0), CustomValueShapeId::new(0)),
        );
        let custom_field = ExternalInstruction::CustomField {
            source: custom_source,
            index: 6,
        };
        assert_eq!(
            custom_field.instruction_ref(),
            ExternalInstructionRef::CustomField {
                source: &custom_source,
                index: 6,
            },
        );

        let list_index = ExternalInstruction::ListIndex {
            list: ExternalListLocalId(7),
            index: 8,
        };
        assert_eq!(
            list_index.instruction_ref(),
            ExternalInstructionRef::ListIndex {
                list: ExternalListLocalId(7),
                index: 8,
            },
        );
    }

    #[test]
    fn plain_external_instruction_view_is_uninhabited() {
        fn assert_view<View>()
        where
            View: ExternalInstructionView<Function = std::convert::Infallible>,
        {
        }

        assert_view::<std::convert::Infallible>();
    }
}

#[cfg(test)]
mod explain_tests {
    use super::ExternalInstruction;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::ExternalFunctionId;
    use crate::plan::execution::graph::{
        CustomLocal, CustomLocalId, ExternalFunctionLocal, ExternalFunctionLocalId,
        ExternalListLocalId, IntLocalId, ParamLocal, TupleLocalId,
    };
    use crate::plan::execution::type_::{
        CustomTypeId, CustomValueShape, CustomValueShapeId, ExternalFunctionType, ExternalTypeId,
        FunctionType, ValueType,
    };

    #[test]
    fn writes_external_call() {
        let external_type = ExternalTypeId::new(0);
        let instruction = ExternalInstruction::Call {
            function: ExternalFunctionId::new(1, external_type),
            args: vec![ParamLocal::Int(IntLocalId(0))].into(),
            site: crate::plan::HostCallSite::unknown(),
        };
        let expected = "external.call external#1 args=[%int#0]";

        assert_explanation(&instruction, expected);
    }

    #[test]
    fn writes_external_function_call() {
        let external_type = ExternalTypeId::new(0);
        let function_local = ExternalFunctionLocal::new(
            ExternalFunctionLocalId(2),
            ExternalFunctionType::from_shapes(
                FunctionType::new(Vec::new(), ValueType::External(external_type)),
                Vec::new(),
                external_type,
            ),
        );
        let instruction = ExternalInstruction::FunctionCall {
            function: function_local,
            args: Vec::new().into(),
            site: crate::plan::HostCallSite::unknown(),
        };
        let expected = "external.function_call %function.external#2 args=[]";

        assert_explanation(&instruction, expected);
    }

    #[test]
    fn writes_external_tuple_index() {
        let instruction = ExternalInstruction::TupleIndex {
            tuple: TupleLocalId(3),
            index: 4,
        };
        let expected = "external.tuple_index %tuple#3 index=4";

        assert_explanation(&instruction, expected);
    }

    #[test]
    fn writes_external_custom_field() {
        let custom = CustomLocal::new(
            CustomLocalId(5),
            CustomValueShape::new(CustomTypeId::new(0), CustomValueShapeId::new(0)),
        );
        let instruction = ExternalInstruction::CustomField {
            source: custom,
            index: 6,
        };
        let expected = "external.custom_field %custom#5 index=6";

        assert_explanation(&instruction, expected);
    }

    #[test]
    fn writes_external_list_index() {
        let instruction = ExternalInstruction::ListIndex {
            list: ExternalListLocalId(7),
            index: 8,
        };
        let expected = "external.list_index %list.external#7 index=8";

        assert_explanation(&instruction, expected);
    }

    fn assert_explanation(instruction: &ExternalInstruction, expected: &str) {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            let mut actual = String::new();
            let mut context = explain::ExplainContext::new(plan, &mut actual);
            context.write(instruction);
            assert_eq!(actual, expected);
        });
    }
}
