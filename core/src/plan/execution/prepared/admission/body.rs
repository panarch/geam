mod contract;
pub(super) use contract::Contract;

use super::block::{BlockError, Blocks};
use super::call::{CallError, Target};
use super::catalog::{Catalog, Function};
use super::instruction::{InstructionError, Instructions};
use super::local::{LocalError, Locals};
use super::operand::Operand;
use super::pattern::BindingValue;
use super::source::SourceError;
use super::terminator::{self, TerminatorError};
use super::type_::{TypeError, Types};
use crate::plan::FunctionCallTarget;
use crate::plan::execution::function::{
    ExecutionGraphProfile, FunctionEntry, FunctionExit, FunctionTableFamily,
};
use crate::plan::execution::graph::{BlockGraphExitId, ExternalListInstructionView, ParamLocal};
use crate::plan::execution::type_::ValueShapeId;

#[derive(Debug, PartialEq, Eq)]
pub(super) enum BodyError {
    Contract(contract::ContractError),
    Type(TypeError),
    Block(BlockError),
    Local {
        block: usize,
        error: LocalError,
    },
    Instruction {
        block: usize,
        index: usize,
        error: InstructionError,
    },
    Guard {
        block: usize,
        index: usize,
        error: super::guard::GuardError,
    },
    Terminator {
        block: usize,
        error: TerminatorError,
    },
    Exit {
        block: usize,
        error: ExitError,
    },
    UnclaimedExit {
        index: usize,
    },
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum ExitError {
    Missing { index: usize },
    Local(LocalError),
    Call(CallError),
    Source(SourceError),
    ReturnType,
    Transfer(super::transfer::TransferError),
}

pub(super) trait Tail {
    fn check<'data, Graph: ExecutionGraphProfile>(
        &self,
        family: FunctionTableFamily,
        args: &[ParamLocal],
        return_: ValueShapeId,
        locals: &Locals<'data>,
        context: &Instructions<'_, 'data, Graph>,
    ) -> Result<(), ExitError>;
}

pub(super) trait TailTarget {
    fn function<'data>(
        &self,
        family: FunctionTableFamily,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError>;
}

impl<T: Target> TailTarget for T {
    fn function<'data>(
        &self,
        _family: FunctionTableFamily,
        catalog: &Catalog<'data>,
        types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        self.resolve(catalog, types)
    }
}

impl TailTarget for usize {
    fn function<'data>(
        &self,
        family: FunctionTableFamily,
        catalog: &Catalog<'data>,
        _types: &Types<'data>,
    ) -> Result<Function<'data>, CallError> {
        catalog.function(family, *self).map_err(CallError::Catalog)
    }
}

impl<T: TailTarget> Tail for FunctionCallTarget<T> {
    fn check<'data, Graph: ExecutionGraphProfile>(
        &self,
        family: FunctionTableFamily,
        args: &[ParamLocal],
        return_: ValueShapeId,
        locals: &Locals<'data>,
        context: &Instructions<'_, 'data, Graph>,
    ) -> Result<(), ExitError> {
        context
            .sources
            .span(self.site().module(), self.site().span())
            .map_err(ExitError::Source)?;
        let function = self
            .function()
            .function(family, context.catalog, context.types)
            .map_err(ExitError::Call)?;
        function
            .arguments(args, locals, context.types)
            .map_err(ExitError::Call)?;
        return_value(
            BindingValue::source(function.return_),
            return_,
            context.types,
        )
    }
}

pub(super) fn function<'data, Body: Contract>(
    owner: &'data Body,
    entry: &FunctionEntry,
    family: FunctionTableFamily,
    contract: &Function<'data>,
    context: &Instructions<'_, 'data, Body::Graph>,
) -> Result<(), BodyError>
where
    Body::Return: Operand,
    Body::TailCall: Tail,
    <Body::Graph as ExecutionGraphProfile>::ExternalFunctionId: Target,
    <Body::Graph as ExecutionGraphProfile>::ExternalListFunctionId: Target,
    <<Body::Graph as ExecutionGraphProfile>::ExternalListInstruction as ExternalListInstructionView>::FunctionLocal: Operand,
{
    owner
        .check_contract(contract.return_, context.types)
        .map_err(BodyError::Contract)?;
    let body = owner.function_body();
    let blocks = Blocks::admit(&body.block_graph).map_err(BodyError::Block)?;
    blocks
        .entry_contract(entry, contract, context.types)
        .map_err(BodyError::Block)?;
    let mut used = vec![false; body.exits.len()];
    graph(&blocks, context, &mut |exit, locals| {
        let value = body.exits.get(exit.index()).ok_or(ExitError::Missing {
            index: exit.index(),
        })?;
        used[exit.index()] = true;
        match value {
            FunctionExit::Return(value) => {
                let value = value.read(locals).map_err(ExitError::Local)?;
                return_value(locals.value(value), contract.return_, context.types)
            }
            FunctionExit::TailCall {
                function,
                args,
                transfer,
            } => {
                function.check(family, args, contract.return_, locals, context)?;
                super::transfer::arguments(transfer, args, locals).map_err(ExitError::Transfer)
            }
        }
    })?;
    claimed(&used)
}

pub(super) fn graph<'data, Graph: ExecutionGraphProfile>(
    blocks: &Blocks<'data, Graph>,
    context: &Instructions<'_, 'data, Graph>,
    exit: &mut dyn FnMut(BlockGraphExitId, &Locals<'data>) -> Result<(), ExitError>,
) -> Result<(), BodyError>
where
    Graph::ExternalFunctionId: Target,
    Graph::ExternalListFunctionId: Target,
    <Graph::ExternalListInstruction as ExternalListInstructionView>::FunctionLocal: Operand,
{
    let control = super::control::Control {
        blocks,
        types: context.types,
    };
    let guards = super::guard::Guards { blocks };
    for (block_index, block) in blocks.iter().enumerate() {
        let mut locals = Locals::default();
        for parameter in block.params() {
            locals
                .define(parameter, context.types)
                .map_err(|error| BodyError::Local {
                    block: block_index,
                    error,
                })?;
            control.refine(
                crate::plan::execution::graph::BlockId(block_index),
                parameter,
                &mut locals,
            );
        }
        for (index, instruction) in block.instructions().iter().enumerate() {
            control.refine_projection(
                crate::plan::execution::graph::BlockId(block_index),
                &instruction.output,
                &mut locals,
            );
            context
                .check(instruction, &locals)
                .map_err(|error| BodyError::Instruction {
                    block: block_index,
                    index,
                    error,
                })?;
            guards
                .check(
                    crate::plan::execution::graph::BlockId(block_index),
                    instruction,
                )
                .map_err(|error| BodyError::Guard {
                    block: block_index,
                    index,
                    error,
                })?;
            locals
                .define(&instruction.output, context.types)
                .map_err(|error| BodyError::Local {
                    block: block_index,
                    error,
                })?;
            control.refine(
                crate::plan::execution::graph::BlockId(block_index),
                &instruction.output,
                &mut locals,
            );
        }
        if let Some(id) =
            terminator::check(block.terminator(), blocks, &locals, context).map_err(|error| {
                BodyError::Terminator {
                    block: block_index,
                    error,
                }
            })?
        {
            exit(id, &locals).map_err(|error| BodyError::Exit {
                block: block_index,
                error,
            })?;
        }
    }
    Ok(())
}

pub(super) fn claimed(used: &[bool]) -> Result<(), BodyError> {
    if let Some(index) = used.iter().position(|used| !used) {
        return Err(BodyError::UnclaimedExit { index });
    }
    Ok(())
}

pub(super) fn return_value(
    source: BindingValue,
    target: ValueShapeId,
    types: &Types<'_>,
) -> Result<(), ExitError> {
    if !source.flows_to(target, types) {
        return Err(ExitError::ReturnType);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::super::source::Sources;
    use super::{Catalog, FunctionTableFamily, Instructions, Types, function};

    #[test]
    fn nominal_body_disagreement_is_rejected_before_reading_the_graph() {
        use super::{BodyError, contract::ContractError};
        use crate::plan::execution::prepared::admission::{tests::owned_mut, type_::TypeError};
        use crate::plan::execution::type_::CustomValueShapeId;

        let source = "pub type Box { Box(Int) } pub fn main() { Box(42) }";
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let mut plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
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
        let declaration = catalog.function(FunctionTableFamily::Custom, 0).unwrap();
        let entry = &mut owned_mut(
            &mut owned_mut(&mut plan.program.functions)
                .value_returns
                .custom_functions,
        )[0];
        assert_eq!(
            function(
                &entry.body,
                &entry.entry,
                FunctionTableFamily::Custom,
                &declaration,
                &context
            ),
            Ok(())
        );
        use super::super::block::BlockError;
        use crate::plan::execution::graph::{BlockGraphExitId, BlockId, Terminator};
        entry.body.body.block_graph.entry = BlockId(999);
        assert_eq!(
            function(
                &entry.body,
                &entry.entry,
                FunctionTableFamily::Custom,
                &declaration,
                &context
            ),
            Err(BodyError::Block(BlockError::Missing { index: 999 }))
        );
        entry.body.body.block_graph.entry = BlockId(0);
        entry.entry.parameter_count = 1;
        assert_eq!(
            function(
                &entry.body,
                &entry.entry,
                FunctionTableFamily::Custom,
                &declaration,
                &context
            ),
            Err(BodyError::Block(BlockError::ParameterCount {
                expected: 0,
                found: 1
            }))
        );
        entry.entry.parameter_count = 0;
        owned_mut(&mut entry.body.body.block_graph.blocks)[0].terminator =
            Terminator::Exit(BlockGraphExitId(999));
        assert_eq!(
            function(
                &entry.body,
                &entry.entry,
                FunctionTableFamily::Custom,
                &declaration,
                &context
            ),
            Err(BodyError::Exit {
                block: 0,
                error: super::ExitError::Missing { index: 999 }
            })
        );
        entry.body._signature_shape.shape_id = CustomValueShapeId(999);
        assert_eq!(
            function(
                &entry.body,
                &entry.entry,
                FunctionTableFamily::Custom,
                &declaration,
                &context
            ),
            Err(BodyError::Contract(ContractError::Type(
                TypeError::MissingCustomShape { index: 999 },
            )))
        );
    }

    #[test]
    fn function_admission_reports_the_body_stage_and_exact_block_or_exit() {
        use super::super::{
            block::BlockError, edge::EdgeError, instruction::InstructionError, local::LocalError,
            terminator::TerminatorError, type_::TypeError,
        };
        use super::{BodyError, ExitError};
        use crate::plan::execution::function::{
            FunctionEntry, FunctionExit, IntFunctionId, ProfiledFunctionBody,
        };
        use crate::plan::execution::graph::{
            BlockGraphExitId, BlockId, Edge, IntInstruction, IntLocalId, Jump, ParamLocal,
            ParamSlot, ProfiledBlock, ProfiledBlockGraph, ProfiledInstruction,
            ProfiledInstructionKind, Terminator,
        };
        use crate::plan::execution::type_::ValueShapeId;
        use std::convert::Infallible;

        let typed =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { 42 }")
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
        let declaration = catalog.function(FunctionTableFamily::Int, 0).unwrap();
        let int = |shape, value| ProfiledInstruction::<Infallible> {
            output: ParamSlot::new(ParamLocal::Int(IntLocalId(0)), shape),
            kind: ProfiledInstructionKind::Int(value),
        };
        let literal = || {
            int(
                declaration.return_,
                IntInstruction::Value(num_bigint::BigInt::from(42).into()),
            )
        };
        let cases = [
            (
                vec![literal()],
                Terminator::Exit(BlockGraphExitId(0)),
                vec![FunctionExit::Return(IntLocalId(0))],
                Ok(()),
            ),
            (
                vec![literal(), literal()],
                Terminator::Exit(BlockGraphExitId(0)),
                vec![FunctionExit::Return(IntLocalId(0))],
                Err(BodyError::Local {
                    block: 0,
                    error: LocalError::DefinitionOrder {
                        address: IntLocalId(0).into(),
                        expected: 1,
                    },
                }),
            ),
            (
                vec![int(
                    ValueShapeId(99),
                    IntInstruction::Value(num_bigint::BigInt::from(42).into()),
                )],
                Terminator::Exit(BlockGraphExitId(0)),
                vec![FunctionExit::Return(IntLocalId(0))],
                Err(BodyError::Instruction {
                    block: 0,
                    index: 0,
                    error: InstructionError::Type(TypeError::MissingShape { index: 99 }),
                }),
            ),
            (
                vec![int(
                    declaration.return_,
                    IntInstruction::Negate(IntLocalId(99)),
                )],
                Terminator::Exit(BlockGraphExitId(0)),
                vec![FunctionExit::Return(IntLocalId(0))],
                Err(BodyError::Instruction {
                    block: 0,
                    index: 0,
                    error: InstructionError::Local(LocalError::Missing(IntLocalId(99).into())),
                }),
            ),
            (
                Vec::new(),
                Terminator::Exit(BlockGraphExitId(0)),
                vec![FunctionExit::Return(IntLocalId(0))],
                Err(BodyError::Exit {
                    block: 0,
                    error: ExitError::Local(LocalError::Missing(IntLocalId(0).into())),
                }),
            ),
            (
                Vec::new(),
                Terminator::Exit(BlockGraphExitId(0)),
                vec![FunctionExit::TailCall {
                    function: FunctionCallTarget::new(
                        IntFunctionId(99),
                        HostCallSite::new("example".into(), "main".into(), SourceSpan::new(16, 18)),
                    ),
                    args: Vec::new().into(),
                    transfer: crate::plan::execution::graph::Transfer {
                        families: crate::plan::execution::storage::Table::Static(&[]),
                    },
                }],
                Err(BodyError::Exit {
                    block: 0,
                    error: ExitError::Call(CallError::Catalog(CatalogError::MissingFunction {
                        family: FunctionTableFamily::Int,
                        index: 99,
                    })),
                }),
            ),
            (
                vec![literal()],
                Terminator::Exit(BlockGraphExitId(99)),
                vec![FunctionExit::Return(IntLocalId(0))],
                Err(BodyError::Exit {
                    block: 0,
                    error: ExitError::Missing { index: 99 },
                }),
            ),
            (
                vec![literal()],
                Terminator::Exit(BlockGraphExitId(0)),
                vec![
                    FunctionExit::Return(IntLocalId(0)),
                    FunctionExit::Return(IntLocalId(0)),
                ],
                Err(BodyError::UnclaimedExit { index: 1 }),
            ),
            (
                vec![literal()],
                Terminator::Jump(Jump {
                    edge: Edge::new(
                        BlockId(99),
                        Vec::new(),
                        crate::plan::execution::graph::Transfer {
                            families: crate::plan::execution::storage::Table::Static(&[]),
                        },
                    ),
                }),
                vec![FunctionExit::Return(IntLocalId(0))],
                Err(BodyError::Terminator {
                    block: 0,
                    error: TerminatorError::Edge(EdgeError::Block(BlockError::Missing {
                        index: 99,
                    })),
                }),
            ),
        ];
        for (instructions, terminator, exits, expected) in cases {
            let body = ProfiledFunctionBody::<
                IntLocalId,
                crate::plan::FunctionCallTarget<IntFunctionId>,
                Infallible,
            > {
                block_graph: ProfiledBlockGraph::from_parts(
                    BlockId(0),
                    vec![ProfiledBlock::new(Vec::new(), instructions, terminator)],
                ),
                exits: exits.into(),
            };
            assert_eq!(
                function(
                    &body,
                    &FunctionEntry::new(0),
                    FunctionTableFamily::Int,
                    &declaration,
                    &context
                ),
                expected
            );
            assert_eq!(
                function(
                    &body,
                    &FunctionEntry::new(1),
                    FunctionTableFamily::Int,
                    &declaration,
                    &context,
                ),
                Err(BodyError::Block(BlockError::ParameterCount {
                    expected: 0,
                    found: 1
                })),
            );
        }
        use super::super::{
            call::CallError, catalog::CatalogError, local::Locals, pattern::BindingValue,
            source::SourceError,
        };
        use super::{Tail, return_value};
        use crate::plan::{FunctionCallTarget, HostCallSite, SourceSpan};
        let span = SourceSpan::new(16, 18);
        for (module, index, arguments, expected) in [
            (
                "missing",
                0,
                vec![],
                Err(ExitError::Source(SourceError::MissingModule(
                    "missing".into(),
                ))),
            ),
            (
                "example",
                99,
                vec![],
                Err(ExitError::Call(CallError::Catalog(
                    CatalogError::MissingFunction {
                        family: FunctionTableFamily::Int,
                        index: 99,
                    },
                ))),
            ),
            (
                "example",
                0,
                vec![ParamLocal::Int(IntLocalId(0))],
                Err(ExitError::Call(CallError::ArgumentCount {
                    expected: 0,
                    found: 1,
                })),
            ),
            ("example", 0, vec![], Ok(())),
        ] {
            let target = FunctionCallTarget::new(
                IntFunctionId(index),
                HostCallSite::new(module.into(), "main".into(), span),
            );
            assert_eq!(
                target.check(
                    FunctionTableFamily::Int,
                    &arguments,
                    declaration.return_,
                    &Locals::default(),
                    &context
                ),
                expected
            );
        }
        assert_eq!(
            return_value(
                BindingValue::Scalar(&crate::plan::execution::type_::ValueType::Bool),
                declaration.return_,
                &types
            ),
            Err(ExitError::ReturnType),
        );
    }

    #[test]
    fn graph_admission_reports_invalid_parameters_and_unguarded_reads() {
        use super::super::{block::Blocks, guard::GuardError, local::LocalError};
        use super::{BodyError, graph};
        use crate::plan::execution::graph::{
            BlockGraphExitId, BlockId, ParamLocal, ParamSlot, ProfiledBlock, ProfiledBlockGraph,
            ProfiledInstruction, ProfiledInstructionKind, StringInstruction, StringLocalId,
            Terminator,
        };
        use crate::plan::execution::type_::{ValueShapeDescriptor, ValueShapeId};
        use std::convert::Infallible;

        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "pub fn main() { \"abc\" }",
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
            constants: &common.constants,
            sources: &sources,
        };
        let shape = ValueShapeId(
            common
                .value_shapes
                .shapes
                .iter()
                .position(|shape| shape == &ValueShapeDescriptor::String)
                .unwrap(),
        );
        for (index, instructions, expected) in [
            (
                1,
                vec![],
                Err(BodyError::Local {
                    block: 0,
                    error: LocalError::DefinitionOrder {
                        address: StringLocalId(1).into(),
                        expected: 0,
                    },
                }),
            ),
            (
                0,
                vec![ProfiledInstruction::<Infallible> {
                    output: ParamSlot::new(ParamLocal::String(StringLocalId(1)), shape),
                    kind: ProfiledInstructionKind::String(StringInstruction::DropPrefix {
                        value: StringLocalId(0),
                        prefix: "a".into(),
                    }),
                }],
                Err(BodyError::Guard {
                    block: 0,
                    index: 0,
                    error: GuardError::Unproved {
                        local: StringLocalId(0).into(),
                        requirement: "string prefix \"a\"".into(),
                    },
                }),
            ),
            (0, vec![], Ok(())),
        ] {
            let raw = ProfiledBlockGraph::from_parts(
                BlockId(0),
                vec![ProfiledBlock::new(
                    vec![ParamSlot::new(
                        ParamLocal::String(StringLocalId(index)),
                        shape,
                    )],
                    instructions,
                    Terminator::Exit(BlockGraphExitId(0)),
                )],
            );
            let blocks = Blocks::admit(&raw).unwrap();
            let mut exits = Vec::new();
            assert_eq!(
                graph(&blocks, &context, &mut |id, _| {
                    exits.push(id);
                    Ok(())
                }),
                expected
            );
            assert_eq!(
                exits,
                if expected.is_ok() {
                    vec![BlockGraphExitId(0)]
                } else {
                    vec![]
                }
            );
        }
    }

    #[test]
    fn rejects_a_tail_exit_with_a_missing_transfer_family() {
        use super::super::{tests::owned_mut, transfer::TransferError};
        use super::{BodyError, ExitError, FunctionExit};
        use crate::plan::execution::storage::Table;

        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "fn done(n) { n } pub fn main() { done(42) }",
        )
        .unwrap();
        let mut plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let mut changed = Vec::new();
        let tables = owned_mut(&mut plan.program.functions);
        for (index, value) in owned_mut(&mut tables.value_returns.int_functions)
            .iter_mut()
            .enumerate()
        {
            for exit in owned_mut(&mut value.body.exits) {
                if let FunctionExit::TailCall { transfer, .. } = exit {
                    transfer.families = Table::Static(&[]);
                    changed.push(index);
                }
            }
        }
        assert_eq!(changed.len(), 1);
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
            constants: &common.constants,
            sources: &sources,
        };
        let index = changed[0];
        let value = &plan.program.functions.value_returns.int_functions[index];
        assert_eq!(
            function(
                value.body(),
                value.entry(),
                FunctionTableFamily::Int,
                &catalog.function(FunctionTableFamily::Int, index).unwrap(),
                &context,
            ),
            Err(BodyError::Exit {
                block: 0,
                error: ExitError::Transfer(TransferError::Families),
            }),
        );
    }

    #[test]
    fn checks_calls_matches_branches_and_exits_of_real_programs() {
        for source in [
            "fn go(n) { case n { 0 -> 42 _ -> go(n - 1) } } pub fn main() { go(3) }",
            "fn choose(b, x) { case b { True -> x False -> x + 1 } } pub fn main() { choose(True, 42) }",
            "pub type Box(a) { Box(a) } fn read(b) { let Box(n) = b n } pub fn main() { read(Box(42)) }",
            "fn read(xs) { let assert [x, ..] = xs x } pub fn main() { read([42]) }",
            "fn read(bits) { let assert <<size, value:size(size)>> = bits value } pub fn main() { read(<<8, 42>>) }",
            "fn read(s) { let assert \"a\" <> tail = s case tail { \"b\" -> 42 _ -> 0 } } pub fn main() { read(\"ab\") }",
            "pub fn main() { echo 42 }",
        ] {
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
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
            for (index, value) in plan
                .program
                .functions
                .value_returns
                .int_functions
                .iter()
                .enumerate()
            {
                assert_eq!(
                    function(
                        value.body(),
                        value.entry(),
                        FunctionTableFamily::Int,
                        &catalog.function(FunctionTableFamily::Int, index).unwrap(),
                        &context,
                    ),
                    Ok(()),
                    "{source}"
                );
            }
        }
    }
}
