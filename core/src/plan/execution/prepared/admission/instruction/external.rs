use super::{InstructionError, Instructions};
use crate::plan::execution::function::{ExecutionGraphProfile, FunctionReturnFamily};
use crate::plan::execution::graph::{
    ExternalFunctionInstruction, ExternalFunctionInstructionKind, ExternalFunctionTarget,
    ExternalInstructionRef, FunctionCapture, ParamSlot,
};
use crate::plan::execution::prepared::admission::{call::Target, local::Locals};
use crate::plan::execution::type_::ValueType;

impl<'data, Graph: ExecutionGraphProfile> Instructions<'_, 'data, Graph> {
    pub(super) fn external<Function: Target>(
        &self,
        instruction: ExternalInstructionRef<'_, Function>,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        let admitted = self.types.slot(output).map_err(InstructionError::Type)?;
        if !matches!(admitted.type_, ValueType::External(_)) {
            return Err(InstructionError::OutputType);
        }
        match instruction {
            ExternalInstructionRef::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            ExternalInstructionRef::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            ExternalInstructionRef::TupleIndex { tuple, index } => {
                self.tuple_index(&tuple, index, output, locals)
            }
            ExternalInstructionRef::CustomField { source, index } => {
                self.custom_field(source, index, output, locals)
            }
            ExternalInstructionRef::ListIndex { list, index: _ } => {
                self.list_index(&list, output, locals)
            }
        }
    }

    pub(super) fn external_function(
        &self,
        instruction: &ExternalFunctionInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        super::function::header(self.types, &instruction.type_, instruction.family, output)?;
        match &instruction.kind {
            ExternalFunctionInstructionKind::Reference(target) => {
                self.external_function_value(target, &[], instruction.family, output, locals)
            }
            ExternalFunctionInstructionKind::Closure { target, captures } => {
                self.external_function_value(target, captures, instruction.family, output, locals)
            }
            ExternalFunctionInstructionKind::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            ExternalFunctionInstructionKind::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
        }
    }

    fn external_function_value(
        &self,
        target: &ExternalFunctionTarget,
        captures: &[FunctionCapture],
        family: FunctionReturnFamily,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        let actual = match target {
            ExternalFunctionTarget::Value(_) => FunctionReturnFamily::External,
            ExternalFunctionTarget::List(_) => FunctionReturnFamily::List,
            ExternalFunctionTarget::Function(_) | ExternalFunctionTarget::ListFunction { .. } => {
                FunctionReturnFamily::Function
            }
        };
        if actual != family {
            return Err(InstructionError::FunctionFamily);
        }
        self.bound_function(
            target
                .resolve(self.catalog, self.types)
                .map_err(InstructionError::Call)?,
            captures,
            output,
            locals,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{InstructionError, Instructions};
    use crate::plan::execution::function::FunctionReturnFamily;
    use crate::plan::execution::function::{ExternalFunctionId, FunctionTableFamily};
    use crate::plan::execution::graph::{
        ExternalFunctionInstruction, ExternalFunctionInstructionKind, ExternalFunctionTarget,
        ExternalInstructionRef, TupleLocalId,
    };
    use crate::plan::execution::prepared::admission::{
        call::CallError,
        catalog::{Catalog, CatalogError},
        local::Locals,
        source::Sources,
        type_::{TypeError, Types},
    };
    use crate::plan::execution::type_::{ExternalTypeId, FunctionType, ValueShapeId, ValueType};

    #[test]
    fn external_instruction_headers_reject_non_external_outputs_and_missing_targets() {
        use crate::plan::execution::prepared::admission::{
            functions,
            hosts::NativeFunctions,
            tests::{lowered_native, native_hosts},
        };
        let source = r#"
pub type Key
@external(erlang, "native", "key") fn key() -> Key
fn first(values: List(Key)) {
  case values { [value, ..] if value == value -> value _ -> key() }
}
pub fn main() {
  let _ = #(first([key()]), fn() { key() }, fn() { [key()] })
  fn() { 42 }
}
"#;
        let (program, values, nevers) = lowered_native(source);
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let function = &program.functions.function_returns.int_function_functions[0];
        let body = crate::plan::execution::prepared::admission::tests::graph_body(function);
        let output = &body
            .function_body()
            .block_graph()
            .instructions
            .iter()
            .find(|instruction| {
                matches!(
                    instruction.kind(),
                    crate::plan::execution::graph::ProfiledInstructionKind::Function(_)
                )
            })
            .unwrap()
            .output;
        let linked = NativeFunctions::new(&values, &nevers, native_hosts()).unwrap();
        assert_eq!(
            functions::all(&program.functions, &context, &linked),
            Ok(())
        );
        let locals = Locals::default();
        assert_eq!(
            context.external::<ExternalFunctionId>(
                ExternalInstructionRef::TupleIndex {
                    tuple: TupleLocalId(0),
                    index: 0
                },
                output,
                &locals
            ),
            Err(InstructionError::OutputType)
        );
        let mut missing = output.clone();
        missing.shape = ValueShapeId(99);
        assert_eq!(
            context.external::<ExternalFunctionId>(
                ExternalInstructionRef::TupleIndex {
                    tuple: TupleLocalId(0),
                    index: 0
                },
                &missing,
                &locals
            ),
            Err(InstructionError::Type(TypeError::MissingShape {
                index: 99
            }))
        );
        let target = ExternalFunctionTarget::Value(ExternalFunctionId {
            index: 99,
            return_type: ExternalTypeId(0),
        });
        assert_eq!(
            context.external_function_value(
                &target,
                &[],
                FunctionReturnFamily::Function,
                output,
                &locals
            ),
            Err(InstructionError::FunctionFamily)
        );
        assert_eq!(
            context.external_function_value(
                &target,
                &[],
                FunctionReturnFamily::External,
                output,
                &locals
            ),
            Err(InstructionError::Call(CallError::Catalog(
                CatalogError::MissingFunction {
                    family: FunctionTableFamily::External,
                    index: 99
                }
            )))
        );
        let instruction = ExternalFunctionInstruction {
            type_: FunctionType::new(Vec::new(), ValueType::Int),
            family: FunctionReturnFamily::Int,
            kind: ExternalFunctionInstructionKind::Reference(target),
        };
        assert_eq!(
            context.external_function(&instruction, output, &locals),
            Err(InstructionError::FunctionFamily)
        );
        assert_eq!(
            context.external_function(&instruction, &missing, &locals),
            Err(InstructionError::Type(TypeError::MissingShape {
                index: 99
            }))
        );
    }
}
