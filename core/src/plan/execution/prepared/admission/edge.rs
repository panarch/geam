use super::block::{BlockError, Blocks};
use super::local::{Address, LocalError, Locals};
use super::operand::Operand;
use super::pattern::Bindings;
use super::type_::{Slot, TypeError, Types};
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{Edge, MatchEdge, MatchEdgeArgument, ParamSlot};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum EdgeError {
    Block(BlockError),
    Local(LocalError),
    Type(TypeError),
    Arity { expected: usize, found: usize },
    Argument { index: usize },
    Binding { index: usize },
}

pub(super) fn matched<'data, Graph: ExecutionGraphProfile>(
    edge: &MatchEdge,
    bindings: &Bindings,
    blocks: &Blocks<'data, Graph>,
    locals: &Locals<'data>,
    types: &Types<'data>,
) -> Result<(), EdgeError> {
    let target = blocks.block(edge.target).map_err(EdgeError::Block)?;
    let parameters = target.params();
    if edge.args.len() != parameters.len() {
        return Err(EdgeError::Arity {
            expected: parameters.len(),
            found: edge.args.len(),
        });
    }
    for (index, (argument, expected)) in edge.args.iter().zip(parameters).enumerate() {
        match argument {
            MatchEdgeArgument::Value(value) => {
                let source = value.read(locals).map_err(EdgeError::Local)?;
                transfer(source, expected, index, locals, types)?;
            }
            MatchEdgeArgument::Binding(binding) => {
                types.slot(expected).map_err(EdgeError::Type)?;
                let source = bindings
                    .values
                    .get(*binding)
                    .ok_or(EdgeError::Binding { index: *binding })?;
                if !source.flows_to(expected.shape, types) {
                    return Err(EdgeError::Argument { index });
                }
            }
        }
    }
    Ok(())
}

pub(super) fn regular<'data, Graph: ExecutionGraphProfile>(
    edge: &Edge,
    blocks: &Blocks<'data, Graph>,
    locals: &Locals<'data>,
    types: &Types<'data>,
) -> Result<(), EdgeError> {
    let target = blocks.block(edge.target).map_err(EdgeError::Block)?;
    let parameters = target.params();
    if edge.args.len() != parameters.len() {
        return Err(EdgeError::Arity {
            expected: parameters.len(),
            found: edge.args.len(),
        });
    }
    for (index, (argument, expected)) in edge.args.iter().zip(parameters).enumerate() {
        let source = argument.read(locals).map_err(EdgeError::Local)?;
        transfer(source, expected, index, locals, types)?;
    }
    Ok(())
}

fn transfer(
    source: Slot<'_, '_>,
    target: &ParamSlot,
    index: usize,
    locals: &Locals<'_>,
    types: &Types<'_>,
) -> Result<(), EdgeError> {
    types.slot(target).map_err(EdgeError::Type)?;
    if !Address::of(&source.local).same_family(Address::of(&target.local))
        || !locals.value(source).flows_to(target.shape, types)
    {
        return Err(EdgeError::Argument { index });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{BlockError, Blocks, Edge, EdgeError, Locals, Types, regular};
    use crate::plan::execution::graph::{BlockId, Terminator};

    #[test]
    fn verifies_real_branch_arguments_before_entering_a_block() {
        let typed = crate::compile_typed_module("example", "src/example.gleam", "fn choose(flag, x, text) { case flag { True -> #(x, text) False -> #(x + 1, text) } } pub fn main() { choose(True, 42, \"text\") }").unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let mut count = 0;
        for entry in plan.program.functions.value_returns.tuple_functions.iter() {
            let blocks = Blocks::admit(entry.body().block_graph()).unwrap();
            for block in blocks.iter() {
                let mut locals = Locals::default();
                for slot in block.params() {
                    locals.define(slot, &types).unwrap();
                }
                for instruction in block.instructions() {
                    locals.define(instruction.output(), &types).unwrap();
                }
                if let Terminator::BoolBranch(branch) = block.terminator() {
                    for edge in [&branch.true_, &branch.false_] {
                        regular(edge, &blocks, &locals, &types).unwrap();
                        let missing = Edge {
                            target: BlockId(usize::MAX),
                            args: edge.args.clone(),
                        };
                        assert_eq!(
                            regular(&missing, &blocks, &locals, &types),
                            Err(EdgeError::Block(BlockError::Missing { index: usize::MAX }))
                        );
                        let empty = Edge {
                            target: edge.target,
                            args: Vec::new().into(),
                        };
                        assert_eq!(
                            regular(&empty, &blocks, &locals, &types),
                            Err(EdgeError::Arity {
                                expected: edge.args.len(),
                                found: 0
                            })
                        );
                        count += 1;
                    }
                }
            }
        }
        assert_eq!(count, 2);
    }

    #[test]
    fn match_edges_distinguish_missing_bindings_locals_and_wrong_target_slots() {
        use super::{Address, Bindings, LocalError, TypeError, matched};
        use crate::plan::execution::graph::{
            BlockGraphExitId, BoolLocalId, IntLocalId, MatchEdge, MatchEdgeArgument, MatchPattern,
            MatchPatternBinding, ParamLocal, ParamSlot, ProfiledBlock, ProfiledBlockGraph,
        };
        use crate::plan::execution::type_::{ValueShapeDescriptor, ValueShapeId};
        use std::convert::Infallible;

        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "pub fn main() { #(42, True) }",
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
        let int = ValueShapeId(
            common
                .value_shapes
                .shapes
                .iter()
                .position(|shape| matches!(shape, ValueShapeDescriptor::Int))
                .unwrap(),
        );
        let bool_ = ValueShapeId(
            common
                .value_shapes
                .shapes
                .iter()
                .position(|shape| matches!(shape, ValueShapeDescriptor::Bool))
                .unwrap(),
        );
        let inputs = [
            ParamSlot {
                local: ParamLocal::Int(IntLocalId(0)),
                shape: int,
            },
            ParamSlot {
                local: ParamLocal::Bool(BoolLocalId(0)),
                shape: bool_,
            },
        ];
        let mut locals = Locals::default();
        for input in &inputs {
            locals.define(input, &types).unwrap();
        }
        let pattern = MatchPattern::Bind(MatchPatternBinding { index: 0 });
        let bindings = Bindings::admit(&pattern, int, &locals, &types).unwrap();
        let bool_bindings = Bindings::admit(&pattern, bool_, &locals, &types).unwrap();
        let raw = ProfiledBlockGraph::<Infallible>::from_parts(
            BlockId(0),
            vec![ProfiledBlock::new(
                vec![inputs[0].clone()],
                Vec::new(),
                Terminator::Exit(BlockGraphExitId(0)),
            )],
        );
        let blocks = Blocks::admit(&raw).unwrap();
        for argument in [
            MatchEdgeArgument::Binding(0),
            MatchEdgeArgument::Value(inputs[0].local.clone()),
        ] {
            assert_eq!(
                matched(
                    &MatchEdge {
                        target: BlockId(0),
                        args: vec![argument].into()
                    },
                    &bindings,
                    &blocks,
                    &locals,
                    &types
                ),
                Ok(())
            );
        }
        let missing = ParamLocal::Int(IntLocalId(99));
        let cases = [
            (
                MatchEdge {
                    target: BlockId(99),
                    args: vec![].into(),
                },
                EdgeError::Block(BlockError::Missing { index: 99 }),
            ),
            (
                MatchEdge {
                    target: BlockId(0),
                    args: vec![].into(),
                },
                EdgeError::Arity {
                    expected: 1,
                    found: 0,
                },
            ),
            (
                MatchEdge {
                    target: BlockId(0),
                    args: vec![MatchEdgeArgument::Binding(99)].into(),
                },
                EdgeError::Binding { index: 99 },
            ),
            (
                MatchEdge {
                    target: BlockId(0),
                    args: vec![MatchEdgeArgument::Value(missing.clone())].into(),
                },
                EdgeError::Local(LocalError::Missing(Address::of(&missing))),
            ),
            (
                MatchEdge {
                    target: BlockId(0),
                    args: vec![MatchEdgeArgument::Value(inputs[1].local.clone())].into(),
                },
                EdgeError::Argument { index: 0 },
            ),
        ];
        for (edge, error) in cases {
            assert_eq!(
                matched(&edge, &bindings, &blocks, &locals, &types),
                Err(error)
            );
        }
        assert_eq!(
            matched(
                &MatchEdge {
                    target: BlockId(0),
                    args: vec![MatchEdgeArgument::Binding(0)].into()
                },
                &bool_bindings,
                &blocks,
                &locals,
                &types
            ),
            Err(EdgeError::Argument { index: 0 })
        );
        assert_eq!(
            regular(
                &Edge {
                    target: BlockId(0),
                    args: vec![missing.clone()].into()
                },
                &blocks,
                &locals,
                &types
            ),
            Err(EdgeError::Local(LocalError::Missing(Address::of(&missing))))
        );
        assert_eq!(
            regular(
                &Edge {
                    target: BlockId(0),
                    args: vec![inputs[1].local.clone()].into()
                },
                &blocks,
                &locals,
                &types
            ),
            Err(EdgeError::Argument { index: 0 })
        );

        for (shape, error) in [
            (ValueShapeId(99), TypeError::MissingShape { index: 99 }),
            (bool_, TypeError::LocalTypeMismatch),
        ] {
            let invalid = ProfiledBlockGraph::<Infallible>::from_parts(
                BlockId(0),
                vec![ProfiledBlock::new(
                    vec![ParamSlot {
                        local: inputs[0].local.clone(),
                        shape,
                    }],
                    Vec::new(),
                    Terminator::Exit(BlockGraphExitId(0)),
                )],
            );
            let blocks = Blocks::admit(&invalid).unwrap();
            let expected = EdgeError::Type(error);
            for result in [
                regular(
                    &Edge {
                        target: BlockId(0),
                        args: vec![inputs[0].local.clone()].into(),
                    },
                    &blocks,
                    &locals,
                    &types,
                ),
                matched(
                    &MatchEdge {
                        target: BlockId(0),
                        args: vec![MatchEdgeArgument::Binding(0)].into(),
                    },
                    &bindings,
                    &blocks,
                    &locals,
                    &types,
                ),
            ] {
                assert_eq!(result.as_ref(), Err(&expected));
            }
        }
    }
}
