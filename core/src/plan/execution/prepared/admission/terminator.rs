use super::block::Blocks;
use super::call::{self, CallError, Target};
use super::edge::{self, EdgeError};
use super::instruction::Instructions;
use super::local::{LocalError, Locals};
use super::operand::Operand;
use super::pattern::{Bindings, PatternError};
use super::source::SourceError;
use crate::plan::execution::function::ExecutionGraphProfile;
use crate::plan::execution::graph::{BlockGraphExitId, NeverCallTarget, Terminator};

#[derive(Debug, PartialEq, Eq)]
pub(super) enum TerminatorError {
    Integer(super::literal::IntegerError),
    Local(LocalError),
    Edge(EdgeError),
    Pattern(PatternError),
    Source(SourceError),
    Call(CallError),
    Transfer(super::transfer::TransferError),
}

pub(super) fn check<'data, Graph: ExecutionGraphProfile>(
    terminator: &'data Terminator,
    blocks: &Blocks<'data, Graph>,
    locals: &Locals<'data>,
    context: &Instructions<'_, 'data, Graph>,
) -> Result<Option<BlockGraphExitId>, TerminatorError> {
    let types = context.types;
    let edge = |edge| edge::regular(edge, blocks, locals, types).map_err(TerminatorError::Edge);
    let message = |value: &Option<crate::plan::execution::graph::StringLocalId>| {
        if let Some(value) = value {
            value.read(locals).map_err(TerminatorError::Local)?;
        }
        Ok(())
    };
    match terminator {
        Terminator::Jump(value) => edge(&value.edge)?,
        Terminator::BoolBranch(value) => {
            value.subject.read(locals).map_err(TerminatorError::Local)?;
            edge(&value.true_)?;
            edge(&value.false_)?;
        }
        Terminator::IntSwitch(value) => {
            value.subject.read(locals).map_err(TerminatorError::Local)?;
            for (value, branch) in value.clauses.iter() {
                super::literal::integer(value).map_err(TerminatorError::Integer)?;
                edge(branch)?;
            }
            edge(&value.fallback)?;
        }
        Terminator::FloatSwitch(value) => {
            value.subject.read(locals).map_err(TerminatorError::Local)?;
            for (_, branch) in value.clauses.iter() {
                edge(branch)?;
            }
            edge(&value.fallback)?;
        }
        Terminator::StringSwitch(value) => {
            value.subject.read(locals).map_err(TerminatorError::Local)?;
            for (_, branch) in value.clauses.iter() {
                edge(branch)?;
            }
            edge(&value.fallback)?;
        }
        Terminator::Match(value) => {
            let subject = value.subject.read(locals).map_err(TerminatorError::Local)?;
            let bindings = Bindings::admit(&value.pattern, subject.shape, locals, types)
                .map_err(TerminatorError::Pattern)?;
            edge::matched(&value.success, &bindings, blocks, locals, types)
                .map_err(TerminatorError::Edge)?;
            edge(&value.failure)?;
        }
        Terminator::Echo(value) => {
            value.subject.read(locals).map_err(TerminatorError::Local)?;
            message(&value.message)?;
            context
                .sources
                .span(value.site.module(), value.site.span())
                .map_err(TerminatorError::Source)?;
            edge(&value.next)?;
        }
        Terminator::Exit(exit) => return Ok(Some(*exit)),
        Terminator::SourceStop(value) => {
            message(&value.message)?;
            context
                .sources
                .span(value.site.module(), value.site.span())
                .map_err(TerminatorError::Source)?;
        }
        Terminator::LetAssertPanic(value) => {
            value.subject.read(locals).map_err(TerminatorError::Local)?;
            message(&value.message)?;
            context
                .sources
                .span(value.site.module(), value.site.span())
                .map_err(TerminatorError::Source)?;
            context
                .sources
                .span(value.site.module(), value.pattern_span)
                .map_err(TerminatorError::Source)?;
        }
        Terminator::NeverCall(value) => {
            context
                .sources
                .span(value.site.module(), value.site.span())
                .map_err(TerminatorError::Source)?;
            match &value.function {
                NeverCallTarget::Direct(function) => function
                    .resolve(context.catalog, types)
                    .map_err(TerminatorError::Call)?
                    .arguments(&value.args, locals, types)
                    .map_err(TerminatorError::Call)?,
                NeverCallTarget::Value(function) => {
                    call::indirect_arguments(function, &value.args, locals, types)
                        .map_err(TerminatorError::Call)?;
                }
            }
            super::transfer::arguments(&value.transfer, &value.args, locals)
                .map_err(TerminatorError::Transfer)?;
        }
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::{
        Blocks, EdgeError, Instructions, LocalError, Locals, SourceError, Terminator,
        TerminatorError, check,
    };
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockHeader, BlockId, BoolBranch, BoolLocalId, Echo, Edge,
        FamilyTransfer, FloatLocalId, FloatSwitch, IntLocalId, IntSwitch, IntegerLiteral, Jump,
        LetAssertPanic, Match, MatchEdge, MatchPattern, ParamLocal, ParamSlot, ProfiledBlockGraph,
        SourceStop, SourceStopKind, StorageFamily, StringLocalId, StringSwitch, Transfer,
    };
    use crate::plan::execution::prepared::admission::{
        block::BlockError, catalog::Catalog, literal::IntegerError, local::Address,
        pattern::PatternError, source::Sources, type_::Types,
    };
    use crate::plan::execution::storage::Table;
    use crate::plan::execution::type_::{ValueShapeId, ValueType};
    use crate::plan::{EchoSite, PanicSite, SourceSpan};
    use num_bigint::{BigInt, Sign};
    use std::convert::Infallible;

    #[test]
    fn branch_and_match_terminators_validate_every_subject_and_destination() {
        let source = "pub fn main() { #(42, 1.5, \"text\", True) }";
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
        let slots = [
            (ParamLocal::Int(IntLocalId(0)), ValueType::Int),
            (ParamLocal::Float(FloatLocalId(0)), ValueType::Float),
            (ParamLocal::String(StringLocalId(0)), ValueType::String),
            (ParamLocal::Bool(BoolLocalId(0)), ValueType::Bool),
        ]
        .into_iter()
        .map(|(local, type_)| {
            ParamSlot::new(
                local,
                ValueShapeId(
                    types
                        .shape_types()
                        .iter()
                        .position(|value| value == &type_)
                        .unwrap(),
                ),
            )
        })
        .collect::<Vec<_>>();
        let mut locals = Locals::default();
        for slot in &slots {
            locals.define(slot, &types).unwrap();
        }
        let raw = ProfiledBlockGraph::<Infallible> {
            entry: BlockId(0),
            blocks: vec![BlockHeader {
                params: 0..0,
                instructions: 0..0,
                terminator: Terminator::Exit(BlockGraphExitId(0)),
            }]
            .into(),
            params: Table::Static(&[]),
            instructions: Table::Static(&[]),
        };
        let blocks = Blocks::admit(&raw).unwrap();
        let edge = |index| {
            Edge::new(
                BlockId(index),
                Vec::new(),
                Transfer {
                    families: [
                        StorageFamily::Int,
                        StorageFamily::Float,
                        StorageFamily::String,
                        StorageFamily::Bool,
                    ]
                    .map(|family| FamilyTransfer {
                        family,
                        positions: Table::Static(&[]),
                    })
                    .to_vec()
                    .into(),
                },
            )
        };
        let bad_edge =
            || TerminatorError::Edge(EdgeError::Block(BlockError::Missing { index: 99 }));
        let missing = |local| TerminatorError::Local(LocalError::Missing(local));
        for (target, expected) in [(0, Ok(None)), (99, Err(bad_edge()))] {
            assert_eq!(
                check(
                    &Terminator::Jump(Jump { edge: edge(target) }),
                    &blocks,
                    &locals,
                    &context
                ),
                expected
            );
        }
        for (subject, yes, no, expected) in [
            (0, 0, 0, Ok(None)),
            (99, 0, 0, Err(missing(Address::from(BoolLocalId(99))))),
            (0, 99, 0, Err(bad_edge())),
            (0, 0, 99, Err(bad_edge())),
        ] {
            assert_eq!(
                check(
                    &Terminator::BoolBranch(BoolBranch {
                        subject: BoolLocalId(subject),
                        true_: edge(yes),
                        false_: edge(no)
                    }),
                    &blocks,
                    &locals,
                    &context
                ),
                expected
            );
        }
        for (subject, clause, fallback, missing_subject) in [
            (0, 0, 0, false),
            (99, 0, 0, true),
            (0, 99, 0, false),
            (0, 0, 99, false),
        ] {
            let cases = [
                (
                    Terminator::IntSwitch(IntSwitch {
                        subject: IntLocalId(subject),
                        clauses: vec![(BigInt::from(42).into(), edge(clause))].into(),
                        fallback: edge(fallback),
                    }),
                    Address::from(IntLocalId(99)),
                ),
                (
                    Terminator::FloatSwitch(FloatSwitch {
                        subject: FloatLocalId(subject),
                        clauses: vec![(1.5, edge(clause))].into(),
                        fallback: edge(fallback),
                    }),
                    Address::from(FloatLocalId(99)),
                ),
                (
                    Terminator::StringSwitch(StringSwitch {
                        subject: StringLocalId(subject),
                        clauses: vec![("text".into(), edge(clause))].into(),
                        fallback: edge(fallback),
                    }),
                    Address::from(StringLocalId(99)),
                ),
            ];
            for (value, address) in cases {
                let expected = if missing_subject {
                    Err(missing(address))
                } else if clause == 99 || fallback == 99 {
                    Err(bad_edge())
                } else {
                    Ok(None)
                };
                assert_eq!(check(&value, &blocks, &locals, &context), expected);
            }
        }
        let malformed = Terminator::IntSwitch(IntSwitch {
            subject: IntLocalId(0),
            clauses: vec![(
                IntegerLiteral {
                    sign: Sign::Plus,
                    digits: Table::Static(&[]),
                },
                edge(0),
            )]
            .into(),
            fallback: edge(0),
        });
        assert_eq!(
            check(&malformed, &blocks, &locals, &context),
            Err(TerminatorError::Integer(IntegerError::EmptyMagnitude))
        );
        for (subject, pattern, success, failure, expected) in [
            (0, MatchPattern::Discard, 0, 0, Ok(None)),
            (
                99,
                MatchPattern::Discard,
                0,
                0,
                Err(missing(Address::from(IntLocalId(99)))),
            ),
            (
                0,
                MatchPattern::String("text".into()),
                0,
                0,
                Err(TerminatorError::Pattern(PatternError::SubjectType)),
            ),
            (0, MatchPattern::Discard, 99, 0, Err(bad_edge())),
            (0, MatchPattern::Discard, 0, 99, Err(bad_edge())),
        ] {
            let value = Terminator::Match(Match {
                subject: ParamLocal::Int(IntLocalId(subject)),
                pattern,
                success: MatchEdge::new(BlockId(success), Vec::new(), Vec::new(), edge(0).transfer),
                failure: edge(failure),
            });
            assert_eq!(check(&value, &blocks, &locals, &context), expected);
        }
        assert_eq!(
            check(
                &Terminator::Exit(BlockGraphExitId(3)),
                &blocks,
                &locals,
                &context
            ),
            Ok(Some(BlockGraphExitId(3)))
        );
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Tuple(vec![
                crate::Value::Int(42.into()),
                crate::Value::Float(1.5),
                crate::Value::String("text".into()),
                crate::Value::Bool(true)
            ])
        );
    }

    #[test]
    fn never_calls_validate_direct_and_retained_targets_without_executing_them() {
        use super::CallError;
        use crate::plan::HostCallSite;
        use crate::plan::execution::function::{
            FunctionBodyOwner, FunctionTableFamily, NeverFunctionId,
        };
        use crate::plan::execution::graph::{NeverCall, NeverCallTarget, NeverFunctionLocalId};
        use crate::plan::execution::prepared::admission::catalog::CatalogError;
        let source = r#"
fn stop(message: String) -> a { panic as message }
pub fn main() { #(stop, "message", 42) }
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
        let raw = plan.program.functions.value_returns.tuple_functions[0]
            .body()
            .function_body()
            .block_graph();
        let blocks = Blocks::admit(raw).unwrap();
        let mut locals = Locals::default();
        let block = raw.blocks().next().unwrap();
        for instruction in block.instructions() {
            locals.define(&instruction.output, &types).unwrap();
        }
        let callbacks = block
            .instructions()
            .iter()
            .filter_map(|instruction| match &instruction.output.local {
                ParamLocal::NeverFunction(local) => Some(local.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(callbacks.len(), 1);
        assert_eq!(
            plan.program.functions.value_returns.never_functions.len(),
            1
        );
        let direct = NeverCallTarget::Direct(NeverFunctionId(0));
        let indirect = NeverCallTarget::Value(callbacks[0].clone());
        for target in [direct, indirect] {
            for (arguments, expected) in [
                (vec![ParamLocal::String(StringLocalId(0))], Ok(None)),
                (
                    vec![],
                    Err(TerminatorError::Call(CallError::ArgumentCount {
                        expected: 1,
                        found: 0,
                    })),
                ),
                (
                    vec![ParamLocal::Int(IntLocalId(0))],
                    Err(TerminatorError::Call(CallError::ArgumentType { index: 0 })),
                ),
            ] {
                let value = Terminator::NeverCall(NeverCall {
                    function: target.clone(),
                    args: arguments.into(),
                    transfer: Transfer {
                        families: vec![
                            FamilyTransfer {
                                family: StorageFamily::Int,
                                positions: Table::Static(&[]),
                            },
                            FamilyTransfer {
                                family: StorageFamily::String,
                                positions: Table::Static(&[0]),
                            },
                            FamilyTransfer {
                                family: StorageFamily::Tuple,
                                positions: Table::Static(&[]),
                            },
                            FamilyTransfer {
                                family: StorageFamily::NeverFunction,
                                positions: Table::Static(&[]),
                            },
                        ]
                        .into(),
                    },
                    site: HostCallSite::new("example".into(), "main".into(), SourceSpan::new(0, 1)),
                });
                assert_eq!(check(&value, &blocks, &locals, &context), expected);
            }
        }
        let malformed = Terminator::NeverCall(NeverCall {
            function: NeverCallTarget::Direct(NeverFunctionId(0)),
            args: vec![ParamLocal::String(StringLocalId(0))].into(),
            transfer: Transfer {
                families: Table::Static(&[]),
            },
            site: HostCallSite::new("example".into(), "main".into(), SourceSpan::new(0, 1)),
        });
        assert_eq!(
            check(&malformed, &blocks, &locals, &context),
            Err(TerminatorError::Transfer(
                super::super::transfer::TransferError::Families
            )),
        );
        let mut missing_callback = callbacks[0].clone();
        missing_callback.id = NeverFunctionLocalId(99);
        for (target, module, expected) in [
            (
                NeverCallTarget::Direct(NeverFunctionId(99)),
                "example",
                TerminatorError::Call(CallError::Catalog(CatalogError::MissingFunction {
                    family: FunctionTableFamily::Never,
                    index: 99,
                })),
            ),
            (
                NeverCallTarget::Value(missing_callback),
                "example",
                TerminatorError::Call(CallError::Local(LocalError::Missing(Address::from(
                    NeverFunctionLocalId(99),
                )))),
            ),
            (
                NeverCallTarget::Direct(NeverFunctionId(0)),
                "missing",
                TerminatorError::Source(SourceError::MissingModule("missing".into())),
            ),
        ] {
            let value = Terminator::NeverCall(NeverCall {
                function: target,
                args: vec![ParamLocal::String(StringLocalId(0))].into(),
                transfer: Transfer {
                    families: Table::Static(&[]),
                },
                site: HostCallSite::new(module.into(), "main".into(), SourceSpan::new(0, 1)),
            });
            assert_eq!(check(&value, &blocks, &locals, &context), Err(expected));
        }
    }

    #[test]
    fn diagnostic_terminators_check_values_messages_sites_and_pattern_spans() {
        let source = "pub fn main() { #(42, \"message\") }";
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
        let slots = [
            (ParamLocal::Int(IntLocalId(0)), ValueType::Int),
            (ParamLocal::String(StringLocalId(0)), ValueType::String),
        ]
        .into_iter()
        .map(|(local, type_)| {
            ParamSlot::new(
                local,
                ValueShapeId(
                    types
                        .shape_types()
                        .iter()
                        .position(|value| value == &type_)
                        .unwrap(),
                ),
            )
        })
        .collect::<Vec<_>>();
        let mut locals = Locals::default();
        for slot in &slots {
            locals.define(slot, &types).unwrap();
        }
        let raw = ProfiledBlockGraph::<Infallible> {
            entry: BlockId(0),
            blocks: vec![BlockHeader {
                params: 0..0,
                instructions: 0..0,
                terminator: Terminator::Exit(BlockGraphExitId(0)),
            }]
            .into(),
            params: Table::Static(&[]),
            instructions: Table::Static(&[]),
        };
        let blocks = Blocks::admit(&raw).unwrap();
        let span = SourceSpan::new(0, 1);
        for (message, module, expected) in [
            (None, "example", Ok(None)),
            (Some(StringLocalId(0)), "example", Ok(None)),
            (
                Some(StringLocalId(99)),
                "example",
                Err(TerminatorError::Local(LocalError::Missing(Address::from(
                    StringLocalId(99),
                )))),
            ),
            (
                None,
                "missing",
                Err(TerminatorError::Source(SourceError::MissingModule(
                    "missing".into(),
                ))),
            ),
        ] {
            let cases = [
                Terminator::Echo(Echo {
                    subject: ParamLocal::Int(IntLocalId(0)),
                    message,
                    site: EchoSite::new(module.into(), "main".into(), span),
                    next: Edge::new(
                        BlockId(0),
                        Vec::new(),
                        Transfer {
                            families: [StorageFamily::Int, StorageFamily::String]
                                .map(|family| FamilyTransfer {
                                    family,
                                    positions: Table::Static(&[]),
                                })
                                .to_vec()
                                .into(),
                        },
                    ),
                }),
                Terminator::SourceStop(SourceStop {
                    kind: SourceStopKind::Panic,
                    message,
                    site: PanicSite::new(module.into(), "main".into(), span),
                }),
                Terminator::LetAssertPanic(LetAssertPanic {
                    subject: ParamLocal::Int(IntLocalId(0)),
                    message,
                    site: PanicSite::new(module.into(), "main".into(), span),
                    pattern_span: span,
                }),
            ];
            for value in cases {
                assert_eq!(check(&value, &blocks, &locals, &context), expected);
            }
        }
        for value in [
            Terminator::Echo(Echo {
                subject: ParamLocal::Int(IntLocalId(99)),
                message: None,
                site: EchoSite::new("example".into(), "main".into(), span),
                next: Edge::new(
                    BlockId(0),
                    Vec::new(),
                    Transfer {
                        families: Table::Static(&[]),
                    },
                ),
            }),
            Terminator::LetAssertPanic(LetAssertPanic {
                subject: ParamLocal::Int(IntLocalId(99)),
                message: None,
                site: PanicSite::new("example".into(), "main".into(), span),
                pattern_span: span,
            }),
        ] {
            assert_eq!(
                check(&value, &blocks, &locals, &context),
                Err(TerminatorError::Local(LocalError::Missing(Address::from(
                    IntLocalId(99)
                ))))
            );
        }
        let value = Terminator::Echo(Echo {
            subject: ParamLocal::Int(IntLocalId(0)),
            message: None,
            site: EchoSite::new("example".into(), "main".into(), span),
            next: Edge::new(
                BlockId(99),
                Vec::new(),
                Transfer {
                    families: Table::Static(&[]),
                },
            ),
        });
        assert_eq!(
            check(&value, &blocks, &locals, &context),
            Err(TerminatorError::Edge(EdgeError::Block(
                BlockError::Missing { index: 99 }
            )))
        );
        let invalid_span = SourceSpan::new(1, 0);
        let value = Terminator::LetAssertPanic(LetAssertPanic {
            subject: ParamLocal::Int(IntLocalId(0)),
            message: None,
            site: PanicSite::new("example".into(), "main".into(), span),
            pattern_span: invalid_span,
        });
        assert_eq!(
            check(&value, &blocks, &locals, &context),
            Err(TerminatorError::Source(SourceError::SpanOrder(
                invalid_span
            )))
        );
    }
}
