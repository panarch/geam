use super::{write_call, write_function_call, write_projection};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::ExternalFunctionId;
use crate::plan::execution::graph::{
    CustomLocal, ExternalFunctionLocal, ExternalListLocalId, ParamLocal, TupleLocalId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExternalInstruction {
    Call {
        function: ExternalFunctionId,
        args: Box<[ParamLocal]>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: ExternalFunctionLocal,
        args: Box<[ParamLocal]>,
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

pub(crate) trait ExternalInstructionView {
    type Function;

    fn instruction_ref(&self) -> ExternalInstructionRef<'_, Self::Function>;
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum ExternalInstructionRef<'instruction, Function> {
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

#[cfg(test)]
mod view_tests {
    use super::{ExternalInstruction, ExternalInstructionRef, ExternalInstructionView};
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
        let call_args = vec![ParamLocal::Int(IntLocalId(0))].into_boxed_slice();
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
        let function_args: Box<[ParamLocal]> = Box::new([]);
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
            args: vec![ParamLocal::Int(IntLocalId(0))].into_boxed_slice(),
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
            args: Box::new([]),
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
