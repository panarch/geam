use super::super::{call::Target, local::Locals, operand::Operand};
use super::local_flow;
use super::{InstructionError, Instructions, read};
use crate::plan::execution::constant::ConstantValue;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{
    ListInstruction, ParamSlot, ParameterListInstruction, TypedListInstruction,
};
use crate::plan::execution::type_::{
    ListStorageTypeId, ValueShapeDescriptor, ValueShapeId, ValueType,
};

impl<'data, Graph: ExecutionGraphProfile> Instructions<'_, 'data, Graph> {
    pub(super) fn list(
        &self,
        instruction: &ListInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        match instruction {
            ListInstruction::Parameter(id, instruction) => {
                self.list_output(ListStorageTypeId::Parameter(*id), output)?;
                self.parameter_list(instruction, output, locals)
            }
            ListInstruction::ParameterList(id, instruction) => self.typed_list(
                ListStorageTypeId::ParameterList(*id),
                instruction,
                output,
                locals,
            ),
            ListInstruction::Int(id, instruction) => {
                self.typed_list(ListStorageTypeId::Int(*id), instruction, output, locals)
            }
            ListInstruction::String(id, instruction) => {
                self.typed_list(ListStorageTypeId::String(*id), instruction, output, locals)
            }
            ListInstruction::BitArray(id, instruction) => self.typed_list(
                ListStorageTypeId::BitArray(*id),
                instruction,
                output,
                locals,
            ),
            ListInstruction::UtfCodepoint(id, instruction) => self.typed_list(
                ListStorageTypeId::UtfCodepoint(*id),
                instruction,
                output,
                locals,
            ),
            ListInstruction::Custom(id, instruction) => {
                self.typed_list(ListStorageTypeId::Custom(*id), instruction, output, locals)
            }
            ListInstruction::Float(id, instruction) => {
                self.typed_list(ListStorageTypeId::Float(*id), instruction, output, locals)
            }
            ListInstruction::Bool(id, instruction) => {
                self.typed_list(ListStorageTypeId::Bool(*id), instruction, output, locals)
            }
            ListInstruction::Nil(id, instruction) => {
                self.typed_list(ListStorageTypeId::Nil(*id), instruction, output, locals)
            }
            ListInstruction::Tuple(id, instruction) => {
                self.typed_list(ListStorageTypeId::Tuple(*id), instruction, output, locals)
            }
            ListInstruction::List(id, instruction) => {
                self.typed_list(ListStorageTypeId::List(*id), instruction, output, locals)
            }
            ListInstruction::Function(id, instruction) => self.typed_list(
                ListStorageTypeId::Function(*id),
                instruction,
                output,
                locals,
            ),
        }
    }

    fn list_output(
        &self,
        storage: ListStorageTypeId,
        output: &ParamSlot,
    ) -> Result<ValueShapeId, InstructionError> {
        let id = self
            .types
            .list_storage(storage)
            .map_err(InstructionError::Type)?;
        let admitted = self.types.slot(output).map_err(InstructionError::Type)?;
        match admitted.descriptor {
            ValueShapeDescriptor::List(item) if admitted.type_ == &ValueType::List(id) => Ok(*item),
            _ => Err(InstructionError::OutputType),
        }
    }

    fn parameter_list(
        &self,
        instruction: &ParameterListInstruction,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError> {
        match instruction {
            ParameterListInstruction::Empty => Ok(()),
            ParameterListInstruction::Constant(id) => self.constant(*id, output),
            ParameterListInstruction::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            ParameterListInstruction::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            ParameterListInstruction::TupleIndex { tuple, index } => {
                self.tuple_index(tuple, *index, output, locals)
            }
            ParameterListInstruction::CustomField { source, index } => {
                self.custom_field(source, *index, output, locals)
            }
            ParameterListInstruction::ListIndex { list, index: _ } => {
                self.list_index(list, output, locals)
            }
        }
    }

    pub(super) fn typed_list<Element, Local, Function, FunctionLocal>(
        &self,
        storage: ListStorageTypeId,
        instruction: &TypedListInstruction<Element, Local, Function, FunctionLocal>,
        output: &ParamSlot,
        locals: &Locals<'data>,
    ) -> Result<(), InstructionError>
    where
        Element: Operand + 'static,
        Local: Operand + ConstantValue,
        Function: Target,
        FunctionLocal: Operand,
    {
        let item = self.list_output(storage, output)?;
        match instruction {
            TypedListInstruction::Value(elements) => {
                for element in elements.iter() {
                    local_flow(self.types, read(element, locals)?, item, locals)?;
                }
                Ok(())
            }
            TypedListInstruction::Spread { elements, tail } => {
                for element in elements.iter() {
                    local_flow(self.types, read(element, locals)?, item, locals)?;
                }
                local_flow(self.types, read(tail, locals)?, output.shape, locals)
            }
            TypedListInstruction::Constant(id) => self.constant(*id, output),
            TypedListInstruction::Call {
                function,
                args,
                site,
            } => self.call(function, args, site, output, locals),
            TypedListInstruction::FunctionCall {
                function,
                args,
                site,
            } => self.indirect(function, args, site, output, locals),
            TypedListInstruction::TupleIndex { tuple, index } => {
                self.tuple_index(tuple, *index, output, locals)
            }
            TypedListInstruction::CustomField { source, index } => {
                self.custom_field(source, *index, output, locals)
            }
            TypedListInstruction::ListIndex { list, index: _ } => {
                self.list_index(list, output, locals)
            }
            TypedListInstruction::DropFirst { list, count: _ } => {
                local_flow(self.types, read(list, locals)?, output.shape, locals)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::{catalog::Catalog, source::Sources, type_::Types};
    use super::{
        InstructionError, Instructions, ListInstruction, Locals, ParamSlot,
        ParameterListInstruction, TypedListInstruction, ValueType,
    };
    use crate::plan::execution::graph::{IntLocalId, ParamLocal, ProfiledInstructionKind};
    use crate::plan::execution::type_::ValueShapeId;
    use std::collections::BTreeSet;

    #[test]
    fn all_list_families_preserve_elements_spreads_calls_and_projections() {
        let mut families = BTreeSet::new();
        for item in [
            "42",
            "1.5",
            r#""text""#,
            "True",
            "Nil",
            "<<42>>",
            "point",
            "Box(42)",
            "#(42, True)",
            "[]",
            "[42]",
            "fn(x: Int) { x + 1 }",
        ] {
            let source = format!(
                r#"
pub type Box(a) {{ Box(items: a) }}
const no_items = []
fn pass(value) {{ value }}
pub fn main() {{
  let assert <<point:utf8_codepoint>> = <<65>>
  let item = {item}
  let values = pass([item])
  let tuple = pass(#(values))
  let boxed = pass(Box(values))
  let nested = pass([values])
  let callback = pass(fn() {{ values }})
  let first = case nested {{ [first, ..] if first == first -> first _ -> [] }}
  let assert [_, ..tail] = values
  echo #([item, ..values], tuple.0, boxed.items, first, tail, callback(), no_items)
  Nil
}}
"#
            );
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", &source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            let common = &plan.program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &plan.program.functions, &types)
                    .unwrap();
            let sources = Sources::admit(common.root, &common.modules).unwrap();
            let context = Instructions {
                types: &types,
                catalog: &catalog,
                sources: &sources,
                constants: &common.constants,
            };
            let wrong_output = ParamSlot::new(
                ParamLocal::Int(IntLocalId(0)),
                ValueShapeId(
                    types
                        .shape_types()
                        .iter()
                        .position(|value| value == &ValueType::Int)
                        .unwrap(),
                ),
            );
            for function in plan.program.functions.value_returns.nil_functions.iter() {
                for block in function.body().block_graph().blocks() {
                    let mut locals = Locals::default();
                    for parameter in block.params() {
                        locals.define(parameter, &types).unwrap();
                    }
                    for instruction in block.instructions() {
                        if let ProfiledInstructionKind::List(value) = &instruction.kind {
                            assert_eq!(
                                context.list(value, &instruction.output, &locals),
                                Ok(()),
                                "{item}"
                            );
                            assert_eq!(
                                context.list(value, &wrong_output, &locals),
                                Err(InstructionError::OutputType),
                                "{item}"
                            );
                            families.insert(match value {
                                ListInstruction::Parameter(..) => "parameter",
                                ListInstruction::ParameterList(..) => "parameter list",
                                ListInstruction::Int(..) => "int",
                                ListInstruction::String(..) => "string",
                                ListInstruction::BitArray(..) => "bit array",
                                ListInstruction::UtfCodepoint(..) => "codepoint",
                                ListInstruction::Custom(..) => "custom",
                                ListInstruction::Float(..) => "float",
                                ListInstruction::Bool(..) => "bool",
                                ListInstruction::Nil(..) => "nil",
                                ListInstruction::Tuple(..) => "tuple",
                                ListInstruction::List(..) => "list",
                                ListInstruction::Function(..) => "function",
                            });
                        }
                        locals.define(&instruction.output, &types).unwrap();
                    }
                }
            }
            assert_eq!(
                crate::run_main(&plan, &mut Vec::new()).unwrap(),
                crate::Value::Nil
            );
        }
        assert_eq!(
            families,
            [
                "parameter",
                "parameter list",
                "int",
                "string",
                "bit array",
                "codepoint",
                "custom",
                "float",
                "bool",
                "nil",
                "tuple",
                "list",
                "function"
            ]
            .into_iter()
            .collect()
        );
    }

    #[test]
    fn parameter_list_projections_preserve_the_uninhabited_element_family() {
        let source = r#"
pub type Box(a) { Box(items: a) }
fn pass(value) { value }
fn invoke(callback) { callback() }
pub fn main() {
  let empty = pass([])
  let tuple = pass(#(empty))
  let boxed = pass(Box(empty))
  let nested = pass([empty])
  let first = case nested { [first, ..] -> first [] -> empty }
  echo #(tuple.0, boxed.items, first, invoke(fn() { empty }))
  Nil
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        assert_eq!(
            super::super::super::functions::all(
                &plan.program.functions,
                &context,
                &super::super::super::functions::InfallibleHosts
            ),
            Ok(())
        );
        let mut seen = [false; 3];
        for function in plan.program.functions.value_returns.nil_functions.iter() {
            for block in function.body().block_graph().blocks() {
                let mut locals = Locals::default();
                for parameter in block.params() {
                    locals.define(parameter, &types).unwrap();
                }
                for instruction in block.instructions() {
                    if let ProfiledInstructionKind::List(ListInstruction::Parameter(_, value)) =
                        &instruction.kind
                    {
                        assert_eq!(
                            context.parameter_list(value, &instruction.output, &locals),
                            Ok(())
                        );
                        match value {
                            ParameterListInstruction::TupleIndex { .. } => seen[0] = true,
                            ParameterListInstruction::CustomField { .. } => seen[1] = true,
                            ParameterListInstruction::ListIndex { .. } => seen[2] = true,
                            _ => {}
                        }
                    }
                    locals.define(&instruction.output, &types).unwrap();
                }
            }
        }
        assert_eq!(seen, [true; 3]);
    }

    #[test]
    fn list_elements_and_tails_preserve_tuple_types_and_checked_local_links() {
        use super::super::super::local::LocalError;
        use crate::plan::execution::graph::{TupleListLocalId, TupleLocalId};

        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "pub fn main() { #([#(42, True)], [#(True, 42)]) }",
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let block = plan.program.functions.value_returns.tuple_functions[0]
            .body()
            .block_graph()
            .blocks()
            .next()
            .unwrap();
        let mut locals = Locals::default();
        for instruction in block.instructions() {
            locals.define(&instruction.output, &types).unwrap();
        }
        let lists = block
            .instructions()
            .iter()
            .filter_map(|instruction| match &instruction.kind {
                ProfiledInstructionKind::List(ListInstruction::Tuple(storage, _)) => {
                    Some((*storage, &instruction.output))
                }
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(lists.len(), 2);
        let (storage, output) = lists[0];
        for (element, expected) in [
            (TupleLocalId(0), Ok(())),
            (TupleLocalId(1), Err(InstructionError::Flow)),
            (
                TupleLocalId(99),
                Err(InstructionError::Local(LocalError::Missing(
                    TupleLocalId(99).into(),
                ))),
            ),
        ] {
            for instruction in [
                TypedListInstruction::Value(vec![element].into()),
                TypedListInstruction::Spread {
                    elements: vec![element].into(),
                    tail: TupleListLocalId(0),
                },
            ] {
                assert_eq!(
                    context.list(
                        &ListInstruction::Tuple(storage, instruction),
                        output,
                        &locals
                    ),
                    expected
                );
            }
        }
        for (tail, expected) in [
            (TupleListLocalId(0), Ok(())),
            (TupleListLocalId(1), Err(InstructionError::Flow)),
            (
                TupleListLocalId(99),
                Err(InstructionError::Local(LocalError::Missing(
                    TupleListLocalId(99).into(),
                ))),
            ),
        ] {
            for instruction in [
                TypedListInstruction::Spread {
                    elements: vec![TupleLocalId(0)].into(),
                    tail,
                },
                TypedListInstruction::DropFirst {
                    list: tail,
                    count: 1,
                },
            ] {
                assert_eq!(
                    context.list(
                        &ListInstruction::Tuple(storage, instruction),
                        output,
                        &locals
                    ),
                    expected
                );
            }
        }
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new())
                .unwrap()
                .inspect()
                .to_string(),
            "#([#(42, True)], [#(True, 42)])"
        );
    }

    #[test]
    fn element_and_tail_references_are_checked_before_list_construction() {
        use super::super::super::local::{Address, LocalError};
        use crate::plan::execution::graph::IntListLocalId;
        use crate::plan::execution::type_::{IntListTypeId, ListTypeId};
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "pub fn main() { echo [42] Nil }",
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let block = plan.program.functions.value_returns.nil_functions[0]
            .body()
            .block_graph()
            .blocks()
            .next()
            .unwrap();
        let mut locals = Locals::default();
        for instruction in block.instructions() {
            locals.define(&instruction.output, &types).unwrap();
        }
        let (output, storage) = block
            .instructions()
            .iter()
            .find_map(|instruction| match &instruction.kind {
                ProfiledInstructionKind::List(ListInstruction::Int(id, _)) => {
                    Some((&instruction.output, *id))
                }
                _ => None,
            })
            .unwrap();
        let tail = IntListLocalId(99);
        for (instruction, address) in [
            (
                TypedListInstruction::Value(vec![IntLocalId(99)].into()),
                Address::from(IntLocalId(99)),
            ),
            (
                TypedListInstruction::Spread {
                    elements: vec![IntLocalId(99)].into(),
                    tail,
                },
                Address::from(IntLocalId(99)),
            ),
            (
                TypedListInstruction::Spread {
                    elements: vec![].into(),
                    tail,
                },
                Address::from(tail),
            ),
            (
                TypedListInstruction::DropFirst {
                    list: tail,
                    count: 1,
                },
                Address::from(tail),
            ),
        ] {
            assert_eq!(
                context.list(&ListInstruction::Int(storage, instruction), output, &locals),
                Err(InstructionError::Local(LocalError::Missing(address)))
            );
        }
        assert_eq!(
            context.list(
                &ListInstruction::Int(
                    IntListTypeId::new(ListTypeId(99)),
                    TypedListInstruction::Value(vec![].into())
                ),
                output,
                &locals
            ),
            Err(InstructionError::Type(
                super::super::super::type_::TypeError::MissingList { index: 99 }
            ))
        );
        let wrong = ParamSlot {
            shape: ValueShapeId(999),
            local: output.local.clone(),
        };
        assert_eq!(
            context.list(
                &ListInstruction::Int(storage, TypedListInstruction::Value(vec![].into())),
                &wrong,
                &locals
            ),
            Err(InstructionError::Type(
                super::super::super::type_::TypeError::MissingShape { index: 999 }
            ))
        );
    }
}
