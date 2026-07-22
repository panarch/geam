mod bit_array;
pub(super) mod bool;
mod custom;
mod float;
pub(in crate::plan::execution::lowering) mod function;
pub(super) mod generic;
mod int;
pub(super) mod list;
mod nil;
mod string;
mod tuple;
mod utf_codepoint;

use super::{DraftCursor, DraftFlow, DraftGraph, DraftGraphValue, DraftScope, DraftValueRef};
use crate::plan::execution::lowering::specialization::{
    CompoundInhabitation, Representability, StoredValueShape, UninhabitedCustomValueShape,
    UninhabitedTupleValueShape,
};
use crate::plan::module;

pub(in crate::plan::execution::lowering) use bit_array::bit_array_expr;
pub(in crate::plan::execution::lowering) use bool::bool_expr;
pub(in crate::plan::execution::lowering) use custom::{custom_expr, custom_expr_kind};
pub(in crate::plan::execution::lowering) use float::float_expr;
pub(in crate::plan::execution::lowering) use function::function_expr;
pub(in crate::plan::execution::lowering) use generic::{
    custom_never_expr_kind, generic_expr, never_expr, tuple_never_expr,
};
pub(in crate::plan::execution::lowering) use int::int_expr;
pub(in crate::plan::execution::lowering) use list::{
    bit_array_list_expr, bool_list_expr, custom_list_expr, float_list_expr, function_list_expr,
    generic_list_expr, int_list_expr, list_expr, list_list_expr, nil_list_expr,
    parameter_list_list_expr, string_list_expr, tuple_list_expr, utf_codepoint_list_expr,
};
pub(in crate::plan::execution::lowering) use nil::nil_expr;
pub(in crate::plan::execution::lowering) use string::string_expr;
pub(in crate::plan::execution::lowering) use tuple::tuple_expr;
pub(in crate::plan::execution::lowering) use utf_codepoint::utf_codepoint_expr;

type Lowered<T> = Representability<DraftFlow<T>>;

pub(super) fn expr(
    expression: &module::Expr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftValueRef> {
    use module::ExprKind as E;

    match expression.kind() {
        E::Generic(value) => generic_expr(value, cursor, graph, context),
        E::Int(value) => {
            int_expr(value, cursor, graph, context).map(|flow| flow.map(|value| value.erase()))
        }
        E::Float(value) => {
            float_expr(value, cursor, graph, context).map(|flow| flow.map(|value| value.erase()))
        }
        E::String(value) => {
            string_expr(value, cursor, graph, context).map(|flow| flow.map(|value| value.erase()))
        }
        E::BitArray(value) => bit_array_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| value.erase())),
        E::UtfCodepoint(value) => utf_codepoint_expr(value, cursor, graph, context)
            .map(|flow| flow.map(|value| value.erase())),
        E::Custom(value) => {
            custom_expr(value, cursor, graph, context).map(|flow| flow.map(|value| value.erase()))
        }
        E::Bool(value) => {
            bool_expr(value, cursor, graph, context).map(|flow| flow.map(|value| value.erase()))
        }
        E::Nil(value) => {
            nil_expr(value, cursor, graph, context).map(|flow| flow.map(|value| value.erase()))
        }
        E::Tuple(value) => {
            tuple_expr(value, cursor, graph, context).map(|flow| flow.map(|value| value.erase()))
        }
        E::List(value) => {
            list_expr(value, cursor, graph, context).map(|flow| flow.map(|value| value.erase()))
        }
        E::Function(value) => {
            function_expr(value, cursor, graph, context).map(|flow| flow.map(|value| value.erase()))
        }
    }
}

pub(super) fn call_args(
    args: &[module::CallArg],
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<Vec<DraftValueRef>> {
    args.iter().fold(
        Representability::Inhabited(DraftFlow::value(cursor, Vec::with_capacity(args.len()))),
        |lowered, arg| {
            lowered.and_then(|flow| {
                flow.and_then(|cursor, mut values| {
                    expr(arg.value(), cursor, graph, context).map(|flow| {
                        flow.map(|value| {
                            values.push(value);
                            values
                        })
                    })
                })
            })
        },
    )
}

#[cfg_attr(test, derive(Debug, PartialEq))]
enum CallArguments<'a> {
    Complete,
    Diverging(DivergingCallArguments<'a>),
}

#[cfg_attr(test, derive(Debug, PartialEq))]
struct DivergingCallArguments<'a> {
    prefix: &'a [module::CallArg],
    value: DivergingCallArgument<'a>,
}

#[cfg_attr(test, derive(Debug, PartialEq))]
enum DivergingCallArgument<'a> {
    Generic(&'a module::GenericExpr),
    Tuple {
        expression: &'a module::TupleExpr,
        proof: UninhabitedTupleValueShape,
    },
    Custom {
        expression: &'a module::CustomExpr,
        proof: UninhabitedCustomValueShape,
    },
}

fn call_arguments<'a>(
    args: &'a [module::CallArg],
    context: &super::super::LoweringContext,
) -> CallArguments<'a> {
    for (index, arg) in args.iter().enumerate() {
        let value = match arg.storage() {
            module::CallArgStorage::Stored => continue,
            module::CallArgStorage::PotentiallyUninhabited(value) => value,
        };
        let value = match value {
            module::PotentiallyUninhabitedCallArg::Generic(expression) => {
                let shape = context.concrete_parameter(expression.parameter());
                match context
                    .representations
                    .inhabitation(&shape)
                    .into_representability()
                {
                    Representability::Inhabited(_) => continue,
                    Representability::Uninhabited => DivergingCallArgument::Generic(expression),
                }
            }
            module::PotentiallyUninhabitedCallArg::Tuple(expression) => {
                let elements = expression
                    .shape()
                    .iter()
                    .map(|shape| context.concrete_value_shape(shape))
                    .collect::<Vec<_>>();
                match context.representations.tuple_inhabitation(&elements) {
                    CompoundInhabitation::Inhabited => continue,
                    CompoundInhabitation::Uninhabited(proof) => {
                        DivergingCallArgument::Tuple { expression, proof }
                    }
                }
            }
            module::PotentiallyUninhabitedCallArg::Custom(expression) => {
                let shape = context.concrete_custom_value_shape(expression.shape());
                match context.representations.custom_inhabitation(&shape) {
                    CompoundInhabitation::Inhabited => continue,
                    CompoundInhabitation::Uninhabited(proof) => {
                        DivergingCallArgument::Custom { expression, proof }
                    }
                }
            }
        };
        return CallArguments::Diverging(DivergingCallArguments {
            prefix: &args[..index],
            value,
        });
    }
    CallArguments::Complete
}

fn diverging_call_arguments(
    arguments: DivergingCallArguments<'_>,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    let DivergingCallArguments { prefix, value } = arguments;
    prefix
        .iter()
        .fold(
            Representability::Inhabited(DraftFlow::value(cursor, ())),
            |lowered, arg| {
                lowered.and_then(|flow| {
                    flow.and_then(|cursor, ()| {
                        expr(arg.value(), cursor, graph, context).map(|flow| flow.map(|_| ()))
                    })
                })
            },
        )
        .and_then(|flow| {
            flow.fold(Representability::Inhabited(()), |cursor, ()| match value {
                DivergingCallArgument::Generic(expression) => {
                    generic::never_expr(expression, cursor, graph, context)
                }
                DivergingCallArgument::Tuple { expression, proof } => {
                    generic::tuple_never_expr(expression, &proof, cursor, graph, context)
                }
                DivergingCallArgument::Custom { expression, proof } => {
                    generic::custom_never_expr(expression, &proof, cursor, graph, context)
                }
            })
        })
}

pub(super) fn lower_function_call<Function, Output>(
    args: &[module::CallArg],
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
    executable: impl FnOnce(
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<DraftFlow<Function>>,
    evaluated: impl FnOnce(
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Representability<DraftFlow<()>>,
    emit: impl FnOnce(
        DraftCursor,
        Function,
        Vec<DraftValueRef>,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> DraftFlow<Output>,
) -> Representability<DraftFlow<Output>>
where
{
    match call_arguments(args, context) {
        CallArguments::Complete => executable(cursor, graph, context).and_then(|flow| {
            flow.and_then(|cursor, function| {
                call_args(args, cursor, graph, context).and_then(|flow| {
                    flow.and_then(|cursor, args| {
                        Representability::Inhabited(emit(cursor, function, args, graph, context))
                    })
                })
            })
        }),
        CallArguments::Diverging(arguments) => evaluated(cursor, graph, context).and_then(|flow| {
            flow.and_then(|cursor, ()| {
                diverging_call_arguments(arguments, cursor, graph, context)
                    .map(|()| DraftFlow::Diverged)
            })
        }),
    }
}

pub(super) fn capture_args(
    function: &module::FunctionInstantiation,
    args: &[module::CaptureArg],
    cursor: &DraftCursor,
    context: &mut super::super::LoweringContext,
) -> Vec<super::instruction::DraftFunctionCapture> {
    let mut values = Vec::with_capacity(args.len());
    for (index, arg) in args.iter().enumerate() {
        let source = cursor
            .scope()
            .get(super::super::local::param_local_key(arg.local()));
        let target = context.target_capture_local(
            function,
            module::CapturePosition::new(index),
            source.shape().clone(),
        );
        let target =
            super::super::local::stored_value_local_at(target.shape(), target.index(), context);
        values.push(super::instruction::DraftFunctionCapture { target, source });
    }
    values
}

pub(super) fn panic_expr(
    expression: &module::PanicExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<DraftValueRef> {
    let kind = match expression.kind() {
        module::PanicExprKind::Panic { .. } => crate::plan::execution::graph::SourceStopKind::Panic,
        module::PanicExprKind::Todo { .. } => crate::plan::execution::graph::SourceStopKind::Todo,
        module::PanicExprKind::EmptyFunction => {
            crate::plan::execution::graph::SourceStopKind::EmptyFunction
        }
        module::PanicExprKind::EmptyBlock => {
            crate::plan::execution::graph::SourceStopKind::EmptyBlock
        }
        module::PanicExprKind::IncompleteUse => {
            crate::plan::execution::graph::SourceStopKind::IncompleteUse
        }
    };
    match expression.message() {
        Some(message) => string_expr(message, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Value { cursor, value } => {
                graph.finish_source_stop(cursor, kind, Some(value), expression.site());
                DraftFlow::Diverged
            }
            DraftFlow::Diverged => DraftFlow::Diverged,
        }),
        None => {
            graph.finish_source_stop(cursor, kind, None, expression.site());
            Representability::Inhabited(DraftFlow::Diverged)
        }
    }
}

pub(super) fn join_branches<Value>(
    scope: DraftScope,
    result_shape: StoredValueShape,
    branches: Vec<DraftFlow<Value>>,
    graph: &mut DraftGraph,
    result: impl FnOnce(&DraftValueRef) -> Value,
) -> DraftFlow<Value>
where
    Value: DraftGraphValue,
{
    let branches = branches
        .into_iter()
        .map(|branch| branch.map(|value| value.erase()))
        .collect();
    join_value_branches(scope, result_shape, branches, graph).map(|value| result(&value))
}

fn join_value_branches(
    scope: DraftScope,
    result_shape: StoredValueShape,
    mut branches: Vec<DraftFlow<DraftValueRef>>,
    graph: &mut DraftGraph,
) -> DraftFlow<DraftValueRef> {
    if branches
        .iter()
        .all(|branch| matches!(branch, DraftFlow::Diverged))
    {
        return DraftFlow::Diverged;
    }

    let result_ref = graph.value_ref(result_shape);
    let result = result_ref.clone();
    let merge = graph.block(scope, vec![result_ref]);
    let merge_id = merge.id();
    for branch in branches.drain(..) {
        if let DraftFlow::Value { cursor, value } = branch {
            graph.finish_jump(cursor, merge_id, vec![value]);
        }
    }
    DraftFlow::value(merge, result)
}

pub(super) struct CaseLowering<'a> {
    graph: &'a mut DraftGraph,
    context: &'a mut super::super::LoweringContext,
    result_shape: StoredValueShape,
}

pub(super) fn case_lowering<'a>(
    graph: &'a mut DraftGraph,
    context: &'a mut super::super::LoweringContext,
    result_shape: StoredValueShape,
) -> CaseLowering<'a> {
    CaseLowering {
        graph,
        context,
        result_shape,
    }
}

pub(super) fn bool_case<Value>(
    subject: &module::BoolExpr,
    cursor: DraftCursor,
    case: CaseLowering<'_>,
    lower_true: impl FnOnce(
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Lowered<Value>,
    lower_false: impl FnOnce(
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Lowered<Value>,
    result: impl FnOnce(&DraftValueRef) -> Value,
) -> Lowered<Value>
where
    Value: DraftGraphValue,
{
    let CaseLowering {
        graph,
        context,
        result_shape,
    } = case;
    bool::bool_paths(subject, cursor, graph, context).and_then(|paths| match paths {
        bool::BoolPaths::Diverged => Representability::Inhabited(DraftFlow::Diverged),
        bool::BoolPaths::True(cursor) => lower_true(cursor, graph, context),
        bool::BoolPaths::False(cursor) => lower_false(cursor, graph, context),
        bool::BoolPaths::Both { true_, false_ } => {
            let scope = true_.scope().clone();
            lower_true(true_, graph, context).and_then(|true_| {
                lower_false(false_, graph, context).map(|false_| {
                    join_branches(scope, result_shape, vec![true_, false_], graph, result)
                })
            })
        }
    })
}

pub(super) fn int_case<Branch, Value>(
    subject: &module::IntExpr,
    clauses: &[(num_bigint::BigInt, Branch)],
    fallback: &Branch,
    cursor: DraftCursor,
    case: CaseLowering<'_>,
    mut lower: impl FnMut(
        &Branch,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Lowered<Value>,
    result: impl FnOnce(&DraftValueRef) -> Value,
) -> Lowered<Value>
where
    Value: DraftGraphValue,
{
    let CaseLowering {
        graph,
        context,
        result_shape,
    } = case;
    int_expr(subject, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
        DraftFlow::Value { cursor, value } => {
            let scope = cursor.scope().clone();
            let clause_cursors = clauses
                .iter()
                .map(|_| graph.empty_block(scope.clone()))
                .collect::<Vec<_>>();
            let fallback_cursor = graph.empty_block(scope.clone());
            graph.finish_int_switch(
                cursor,
                value,
                clauses
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, _))| (pattern.clone(), clause_cursors[index].id()))
                    .collect(),
                fallback_cursor.id(),
            );
            clause_cursors
                .into_iter()
                .enumerate()
                .fold(
                    Representability::Inhabited(Vec::with_capacity(clauses.len() + 1)),
                    |lowered, (index, cursor)| {
                        let branch = &clauses[index].1;
                        lowered.and_then(|mut branches| {
                            lower(branch, cursor, graph, context).map(|flow| {
                                branches.push(flow);
                                branches
                            })
                        })
                    },
                )
                .and_then(|mut branches| {
                    lower(fallback, fallback_cursor, graph, context).map(|flow| {
                        branches.push(flow);
                        branches
                    })
                })
                .map(|branches| join_branches(scope, result_shape, branches, graph, result))
        }
    })
}

pub(super) fn float_case<Branch, Value>(
    subject: &module::FloatExpr,
    clauses: &[(f64, Branch)],
    fallback: &Branch,
    cursor: DraftCursor,
    case: CaseLowering<'_>,
    mut lower: impl FnMut(
        &Branch,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Lowered<Value>,
    result: impl FnOnce(&DraftValueRef) -> Value,
) -> Lowered<Value>
where
    Value: DraftGraphValue,
{
    let CaseLowering {
        graph,
        context,
        result_shape,
    } = case;
    float_expr(subject, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
        DraftFlow::Value { cursor, value } => {
            let scope = cursor.scope().clone();
            let clause_cursors = clauses
                .iter()
                .map(|_| graph.empty_block(scope.clone()))
                .collect::<Vec<_>>();
            let fallback_cursor = graph.empty_block(scope.clone());
            graph.finish_float_switch(
                cursor,
                value,
                clauses
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, _))| (*pattern, clause_cursors[index].id()))
                    .collect(),
                fallback_cursor.id(),
            );
            clause_cursors
                .into_iter()
                .enumerate()
                .fold(
                    Representability::Inhabited(Vec::with_capacity(clauses.len() + 1)),
                    |lowered, (index, cursor)| {
                        let branch = &clauses[index].1;
                        lowered.and_then(|mut branches| {
                            lower(branch, cursor, graph, context).map(|flow| {
                                branches.push(flow);
                                branches
                            })
                        })
                    },
                )
                .and_then(|mut branches| {
                    lower(fallback, fallback_cursor, graph, context).map(|flow| {
                        branches.push(flow);
                        branches
                    })
                })
                .map(|branches| join_branches(scope, result_shape, branches, graph, result))
        }
    })
}

pub(super) fn string_case<Branch, Value>(
    subject: &module::StringExpr,
    clauses: &[(ecow::EcoString, Branch)],
    fallback: &Branch,
    cursor: DraftCursor,
    case: CaseLowering<'_>,
    mut lower: impl FnMut(
        &Branch,
        DraftCursor,
        &mut DraftGraph,
        &mut super::super::LoweringContext,
    ) -> Lowered<Value>,
    result: impl FnOnce(&DraftValueRef) -> Value,
) -> Lowered<Value>
where
    Value: DraftGraphValue,
{
    let CaseLowering {
        graph,
        context,
        result_shape,
    } = case;
    string_expr(subject, cursor, graph, context).and_then(|flow| match flow {
        DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
        DraftFlow::Value { cursor, value } => {
            let scope = cursor.scope().clone();
            let clause_cursors = clauses
                .iter()
                .map(|_| graph.empty_block(scope.clone()))
                .collect::<Vec<_>>();
            let fallback_cursor = graph.empty_block(scope.clone());
            graph.finish_string_switch(
                cursor,
                value,
                clauses
                    .iter()
                    .enumerate()
                    .map(|(index, (pattern, _))| (pattern.clone(), clause_cursors[index].id()))
                    .collect(),
                fallback_cursor.id(),
            );
            clause_cursors
                .into_iter()
                .enumerate()
                .fold(
                    Representability::Inhabited(Vec::with_capacity(clauses.len() + 1)),
                    |lowered, (index, cursor)| {
                        let branch = &clauses[index].1;
                        lowered.and_then(|mut branches| {
                            lower(branch, cursor, graph, context).map(|flow| {
                                branches.push(flow);
                                branches
                            })
                        })
                    },
                )
                .and_then(|mut branches| {
                    lower(fallback, fallback_cursor, graph, context).map(|flow| {
                        branches.push(flow);
                        branches
                    })
                })
                .map(|branches| join_branches(scope, result_shape, branches, graph, result))
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::Value;
    use crate::plan::execution::graph::{SourceStopKind, Terminator};
    use crate::plan::execution::lowering::graph::{
        DraftCursor, DraftFlow, DraftGraph, DraftGraphBuilder, DraftNeverReturn, DraftScope,
        DraftValueRef,
    };
    use crate::plan::execution::lowering::specialization::{Representability, StoredValueShape};
    use crate::plan::{
        BoolExpr, CallArg, CaptureArg, CustomConstructorDefinition, CustomConstructorRefinement,
        CustomExpr, CustomFieldDefinition, CustomLocal, CustomLocalId, CustomTypeDefinition,
        CustomTypeName, CustomTypeParameterId, CustomTypePublicity, CustomTypeTemplate,
        CustomValueShape, Expr, FloatExpr, FunctionExpr, FunctionShape, FunctionTemplateId,
        FunctionTemplateSignature, FunctionType, GenericExpr, GenericLocal, GenericLocalId,
        IntExpr, IntFunctionExpr, ListExpr, NilExpr, PanicExpr, PanicSite, Step, StringExpr,
        TupleExpr, TypeParameterId, TypeScheme, UtfCodepointExpr, ValueShape, ValueType,
    };

    #[derive(Debug, PartialEq, Eq)]
    enum FlowOutcome {
        Uninhabited,
        Diverged,
        Value,
    }

    fn flow_outcome<T>(flow: &Representability<DraftFlow<T>>) -> FlowOutcome {
        match flow {
            Representability::Uninhabited => FlowOutcome::Uninhabited,
            Representability::Inhabited(DraftFlow::Diverged) => FlowOutcome::Diverged,
            Representability::Inhabited(DraftFlow::Value { .. }) => FlowOutcome::Value,
        }
    }

    fn unit_flow(
        cursor: DraftCursor,
        _graph: &mut DraftGraph,
        _context: &mut crate::plan::execution::lowering::LoweringContext,
    ) -> Representability<DraftFlow<()>> {
        Representability::Inhabited(DraftFlow::value(cursor, ()))
    }

    fn diverged_flow(
        _cursor: DraftCursor,
        _graph: &mut DraftGraph,
        _context: &mut crate::plan::execution::lowering::LoweringContext,
    ) -> Representability<DraftFlow<()>> {
        Representability::Inhabited(DraftFlow::Diverged)
    }

    fn uninhabited_flow(
        _cursor: DraftCursor,
        _graph: &mut DraftGraph,
        _context: &mut crate::plan::execution::lowering::LoweringContext,
    ) -> Representability<DraftFlow<()>> {
        Representability::Uninhabited
    }

    fn uninhabited_branch<Branch>(
        _branch: &Branch,
        _cursor: DraftCursor,
        _graph: &mut DraftGraph,
        _context: &mut crate::plan::execution::lowering::LoweringContext,
    ) -> Representability<DraftFlow<DraftValueRef>> {
        Representability::Uninhabited
    }

    fn emit_unit(
        cursor: DraftCursor,
        (): (),
        _args: Vec<DraftValueRef>,
        _graph: &mut DraftGraph,
        _context: &mut crate::plan::execution::lowering::LoweringContext,
    ) -> DraftFlow<()> {
        DraftFlow::value(cursor, ())
    }

    #[test]
    fn shared_expression_control_flow_propagates_terminal_branches() {
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let message_stop = PanicExpr::panic_at(
            Some(StringExpr::panic(PanicExpr::panic_at(
                None,
                PanicSite::unknown(),
            ))),
            PanicSite::unknown(),
        );

        assert_eq!(
            flow_outcome(&super::panic_expr(
                &message_stop,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Diverged,
        );
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(&super::expr(
                &Expr::int(IntExpr::value(1.into())),
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Value,
        );

        let all_diverged = super::join_branches(
            Default::default(),
            StoredValueShape::Int,
            vec![DraftFlow::Diverged, DraftFlow::Diverged],
            &mut graph,
            DraftValueRef::clone,
        );
        assert_eq!(
            flow_outcome(&Representability::Inhabited(all_diverged)),
            FlowOutcome::Diverged
        );

        for clause in [true, false] {
            let clauses = if clause {
                vec![(1.into(), ())]
            } else {
                Vec::new()
            };
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                flow_outcome(&super::int_case(
                    &IntExpr::value(1.into()),
                    &clauses,
                    &(),
                    cursor,
                    super::case_lowering(&mut graph, &mut context, StoredValueShape::Int),
                    uninhabited_branch,
                    DraftValueRef::clone,
                )),
                FlowOutcome::Uninhabited,
            );
        }
        for clause in [true, false] {
            let clauses = if clause { vec![(1.0, ())] } else { Vec::new() };
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                flow_outcome(&super::float_case(
                    &FloatExpr::value(1.0),
                    &clauses,
                    &(),
                    cursor,
                    super::case_lowering(&mut graph, &mut context, StoredValueShape::Float),
                    uninhabited_branch,
                    DraftValueRef::clone,
                )),
                FlowOutcome::Uninhabited,
            );
        }
        for clause in [true, false] {
            let clauses = if clause {
                vec![("one".into(), ())]
            } else {
                Vec::new()
            };
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                flow_outcome(&super::string_case(
                    &StringExpr::value("one".into()),
                    &clauses,
                    &(),
                    cursor,
                    super::case_lowering(&mut graph, &mut context, StoredValueShape::String),
                    uninhabited_branch,
                    DraftValueRef::clone,
                )),
                FlowOutcome::Uninhabited,
            );
        }
    }

    #[test]
    fn panic_expression_lowers_every_source_stop_kind() {
        let expressions = [
            (
                PanicExpr::panic_at(None, PanicSite::unknown()),
                SourceStopKind::Panic,
            ),
            (
                PanicExpr::todo_at(None, PanicSite::unknown()),
                SourceStopKind::Todo,
            ),
            (
                PanicExpr::empty_function_at(PanicSite::unknown()),
                SourceStopKind::EmptyFunction,
            ),
            (
                PanicExpr::empty_block_at(PanicSite::unknown()),
                SourceStopKind::EmptyBlock,
            ),
            (
                PanicExpr::incomplete_use_at(PanicSite::unknown()),
                SourceStopKind::IncompleteUse,
            ),
        ];

        for (expression, expected) in expressions {
            let mut context =
                crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
            let (mut graph, cursor) =
                DraftGraphBuilder::<DraftNeverReturn, ()>::new(Vec::new(), Vec::new());
            assert_eq!(
                flow_outcome(&super::panic_expr(
                    &expression,
                    cursor,
                    &mut graph,
                    &mut context,
                )),
                FlowOutcome::Diverged,
            );

            let lowered = super::super::freeze::freeze(graph, &mut context);
            assert_eq!(lowered.body.blocks().len(), 1);
            assert_eq!(source_stop_kind(&lowered.body), (expected, None));
        }

        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftNeverReturn, ()>::new(Vec::new(), Vec::new());
        let expression = PanicExpr::todo_at(
            Some(StringExpr::value("unfinished".into())),
            PanicSite::unknown(),
        );
        assert_eq!(
            flow_outcome(&super::panic_expr(
                &expression,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Diverged,
        );

        let lowered = super::super::freeze::freeze(graph, &mut context);
        assert_eq!(
            lowered
                .body
                .block(lowered.body.entry())
                .instructions()
                .len(),
            1,
        );
        assert_eq!(
            source_stop_kind(&lowered.body),
            (
                SourceStopKind::Todo,
                Some(crate::plan::execution::StringLocalId(0)),
            ),
        );
    }

    #[test]
    #[should_panic(expected = "fixture should contain a source stop")]
    fn source_stop_kind_rejects_the_wrong_fixture_shape() {
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftNeverReturn, ()>::new(Vec::new(), Vec::new());
        graph.finish_tail_call(cursor, (), Vec::new());
        let lowered = super::super::freeze::freeze(graph, &mut context);
        source_stop_kind(&lowered.body);
    }

    fn source_stop_kind(
        graph: &crate::plan::execution::graph::FunctionGraph<
            crate::plan::execution::graph::NeverReturn,
            (),
        >,
    ) -> (
        SourceStopKind,
        Option<crate::plan::execution::StringLocalId>,
    ) {
        match graph.block(graph.entry()).terminator() {
            Terminator::SourceStop { kind, message, .. } => (*kind, *message),
            _ => panic!("fixture should contain a source stop"),
        }
    }

    fn uninhabited_custom_proof(
        context: &crate::plan::execution::lowering::LoweringContext,
        shape: &crate::plan::execution::lowering::specialization::SpecializedCustomValueShape,
    ) -> crate::plan::execution::lowering::specialization::UninhabitedCustomValueShape {
        match context.representations.custom_inhabitation(shape) {
            crate::plan::execution::lowering::specialization::CompoundInhabitation::Uninhabited(
                proof,
            ) => proof,
            crate::plan::execution::lowering::specialization::CompoundInhabitation::Inhabited => {
                panic!("expected an uninhabited custom value")
            }
        }
    }

    #[test]
    fn call_argument_classification_preserves_compound_inhabitation() {
        let parameter = TypeParameterId(0);
        let custom_name = CustomTypeName::new("geam".into(), "main".into(), "Boxed".into());
        let custom_definition = CustomTypeDefinition::new(
            custom_name.clone(),
            CustomTypePublicity::Private,
            false,
            vec![CustomTypeParameterId(0)],
            vec![CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![CustomFieldDefinition::new(
                    None,
                    CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                )],
            )],
        );
        let context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            custom_definition,
        ]);
        let inhabited_tuple = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );
        assert_eq!(
            super::call_arguments(&[CallArg::new(Expr::tuple(inhabited_tuple))], &context,),
            super::CallArguments::Complete,
        );

        let uninhabited_shape = CustomValueShape::new(
            custom_name.clone(),
            vec![ValueShape::Parameter(parameter)],
            CustomConstructorRefinement::Exact(0),
        );
        let uninhabited_custom = CustomExpr::local_get(
            CustomLocal::from_shape(CustomLocalId(0), uninhabited_shape.clone()),
            "boxed".into(),
        );
        let concrete_shape = context.concrete_custom_value_shape(&uninhabited_shape);
        let custom_proof = uninhabited_custom_proof(&context, &concrete_shape);
        let args = vec![CallArg::new(Expr::custom(uninhabited_custom.clone()))];
        assert_eq!(
            super::call_arguments(&args, &context),
            super::CallArguments::Diverging(super::DivergingCallArguments {
                prefix: &[],
                value: super::DivergingCallArgument::Custom {
                    expression: &uninhabited_custom,
                    proof: custom_proof,
                },
            }),
        );

        let inhabited_custom = CustomExpr::local_get(
            CustomLocal::from_shape(
                CustomLocalId(0),
                CustomValueShape::new(
                    custom_name,
                    vec![ValueShape::Int],
                    CustomConstructorRefinement::Exact(0),
                ),
            ),
            "boxed".into(),
        );
        assert_eq!(
            super::call_arguments(&[CallArg::new(Expr::custom(inhabited_custom))], &context,),
            super::CallArguments::Complete,
        );
    }

    #[test]
    #[should_panic(expected = "expected an uninhabited custom value")]
    fn uninhabited_custom_proof_rejects_inhabited_shape() {
        let custom_name = CustomTypeName::new("geam".into(), "main".into(), "Marker".into());
        let definition = CustomTypeDefinition::new(
            custom_name.clone(),
            CustomTypePublicity::Private,
            false,
            Vec::new(),
            vec![CustomConstructorDefinition::new(
                "Marker".into(),
                0,
                Vec::new(),
            )],
        );
        let context =
            crate::plan::execution::lowering::test_support::lowering_context(vec![definition]);
        let shape = CustomValueShape::new(
            custom_name,
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        uninhabited_custom_proof(&context, &context.concrete_custom_value_shape(&shape));
    }

    #[test]
    fn diverging_call_arguments_preserve_function_and_prefix_evaluation_order() {
        let parameter = TypeParameterId(0);
        let custom_name = CustomTypeName::new("geam".into(), "main".into(), "Boxed".into());
        let custom_definition = CustomTypeDefinition::new(
            custom_name.clone(),
            CustomTypePublicity::Private,
            false,
            vec![CustomTypeParameterId(0)],
            vec![CustomConstructorDefinition::new(
                "Boxed".into(),
                0,
                vec![CustomFieldDefinition::new(
                    None,
                    CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                )],
            )],
        );
        let mut context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            custom_definition,
        ]);
        let local = GenericExpr::local_get(
            GenericLocal::new(GenericLocalId(0), parameter),
            "value".into(),
        );
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let generic_panic = || GenericExpr::panic(parameter, panic());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        let diverging_value = generic_panic();
        let prefix = [CallArg::new(Expr::int(IntExpr::value(1.into())))];
        assert_eq!(
            super::diverging_call_arguments(
                super::DivergingCallArguments {
                    prefix: &prefix,
                    value: super::DivergingCallArgument::Generic(&diverging_value),
                },
                cursor,
                &mut graph,
                &mut context,
            ),
            Representability::Inhabited(()),
        );

        let cursor = graph.empty_block(Default::default());
        let prefix = [CallArg::new(Expr::int(IntExpr::panic(panic())))];
        assert_eq!(
            super::diverging_call_arguments(
                super::DivergingCallArguments {
                    prefix: &prefix,
                    value: super::DivergingCallArgument::Generic(&local),
                },
                cursor,
                &mut graph,
                &mut context,
            ),
            Representability::Inhabited(()),
        );

        let cursor = graph.empty_block(Default::default());
        let prefix = [CallArg::new(Expr::generic(local.clone()))];
        assert_eq!(
            super::diverging_call_arguments(
                super::DivergingCallArguments {
                    prefix: &prefix,
                    value: super::DivergingCallArgument::Generic(&diverging_value),
                },
                cursor,
                &mut graph,
                &mut context,
            ),
            Representability::Uninhabited,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(&super::lower_function_call(
                &[],
                cursor,
                &mut graph,
                &mut context,
                unit_flow,
                unit_flow,
                emit_unit,
            )),
            FlowOutcome::Value,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(&super::lower_function_call(
                &[],
                cursor,
                &mut graph,
                &mut context,
                uninhabited_flow,
                unit_flow,
                emit_unit,
            )),
            FlowOutcome::Uninhabited,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(&super::lower_function_call(
                &[],
                cursor,
                &mut graph,
                &mut context,
                diverged_flow,
                unit_flow,
                emit_unit,
            )),
            FlowOutcome::Diverged,
        );

        let cursor = graph.empty_block(Default::default());
        let args = vec![CallArg::new(Expr::generic(local.clone()))];
        assert_eq!(
            flow_outcome(&super::lower_function_call(
                &args,
                cursor,
                &mut graph,
                &mut context,
                unit_flow,
                diverged_flow,
                emit_unit,
            )),
            FlowOutcome::Diverged,
        );

        let cursor = graph.empty_block(Default::default());
        let args = vec![
            CallArg::new(Expr::int(IntExpr::value(1.into()))),
            CallArg::new(Expr::generic(generic_panic())),
        ];
        assert_eq!(
            flow_outcome(&super::lower_function_call(
                &args,
                cursor,
                &mut graph,
                &mut context,
                unit_flow,
                unit_flow,
                emit_unit,
            )),
            FlowOutcome::Diverged,
        );

        let cursor = graph.empty_block(Default::default());
        let args = vec![
            CallArg::new(Expr::int(IntExpr::panic(panic()))),
            CallArg::new(Expr::generic(local.clone())),
        ];
        assert_eq!(
            flow_outcome(&super::lower_function_call(
                &args,
                cursor,
                &mut graph,
                &mut context,
                unit_flow,
                unit_flow,
                emit_unit,
            )),
            FlowOutcome::Diverged,
        );

        let cursor = graph.empty_block(Default::default());
        let uninhabited_prefix = TupleExpr::block(
            vec![Step::evaluate(Expr::generic(local.clone()))],
            TupleExpr::value(
                vec![Expr::int(IntExpr::value(1.into()))],
                vec![ValueType::Int],
            ),
        );
        let args = vec![
            CallArg::new(Expr::tuple(uninhabited_prefix)),
            CallArg::new(Expr::generic(local.clone())),
        ];
        assert_eq!(
            flow_outcome(&super::lower_function_call(
                &args,
                cursor,
                &mut graph,
                &mut context,
                unit_flow,
                unit_flow,
                emit_unit,
            )),
            FlowOutcome::Uninhabited,
        );

        let uninhabited_tuple = TupleExpr::local_get(
            crate::plan::TupleLocalId(0),
            "tuple".into(),
            vec![ValueType::Parameter(parameter)],
        );
        for (tuple, expected) in [
            (uninhabited_tuple.clone(), FlowOutcome::Uninhabited),
            (
                TupleExpr::block(
                    vec![Step::evaluate(Expr::generic(generic_panic()))],
                    uninhabited_tuple,
                ),
                FlowOutcome::Diverged,
            ),
        ] {
            let cursor = graph.empty_block(Default::default());
            let args = vec![CallArg::new(Expr::tuple(tuple))];
            assert_eq!(
                flow_outcome(&super::lower_function_call(
                    &args,
                    cursor,
                    &mut graph,
                    &mut context,
                    unit_flow,
                    unit_flow,
                    emit_unit,
                )),
                expected,
            );
        }

        let custom_shape = CustomValueShape::new(
            custom_name,
            vec![ValueShape::Parameter(parameter)],
            CustomConstructorRefinement::Exact(0),
        );
        let uninhabited_custom = CustomExpr::local_get(
            CustomLocal::from_shape(CustomLocalId(0), custom_shape),
            "boxed".into(),
        );
        for (custom, expected) in [
            (uninhabited_custom.clone(), FlowOutcome::Uninhabited),
            (
                CustomExpr::block(
                    vec![Step::evaluate(Expr::generic(generic_panic()))],
                    uninhabited_custom,
                ),
                FlowOutcome::Diverged,
            ),
        ] {
            let cursor = graph.empty_block(Default::default());
            let args = vec![CallArg::new(Expr::custom(custom))];
            assert_eq!(
                flow_outcome(&super::lower_function_call(
                    &args,
                    cursor,
                    &mut graph,
                    &mut context,
                    unit_flow,
                    unit_flow,
                    emit_unit,
                )),
                expected,
            );
        }
    }

    #[test]
    fn capture_arguments_preserve_typed_source_and_entry_destination() {
        let parameter = TypeParameterId(0);
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::new(1),
            TypeScheme::new(1),
            FunctionShape::new(vec![ValueShape::Parameter(parameter)], ValueShape::Int),
        );
        let concrete = signature
            .try_instantiate(vec![ValueShape::Int])
            .expect("one concrete argument should instantiate the capture target");
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, _) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let source_local = crate::plan::ParamLocal::int(crate::plan::IntLocalId(0));
        let source = graph.value_ref(StoredValueShape::Int);
        let mut scope = DraftScope::default();
        scope.insert(
            crate::plan::execution::lowering::local::param_local_key(&source_local),
            source.clone(),
        );
        let cursor = graph.empty_block(scope);

        let captures = super::capture_args(
            &concrete,
            &[CaptureArg::new(source_local)],
            &cursor,
            &mut context,
        );
        assert_eq!(captures.len(), 1);
        assert_eq!(
            captures[0].target,
            crate::plan::execution::ParamLocal::Int(crate::plan::execution::IntLocalId(1)),
        );
        assert_eq!(captures[0].source, source);
    }

    struct ValueFamily {
        type_: &'static str,
        value: &'static str,
        constant: Option<&'static str>,
        assertion: &'static str,
    }

    const VALUE_FAMILIES: &[ValueFamily] = &[
        ValueFamily {
            type_: "Int",
            value: "1",
            constant: Some("1"),
            assertion: "selected == 1",
        },
        ValueFamily {
            type_: "Float",
            value: "1.5",
            constant: Some("1.5"),
            assertion: "selected == 1.5",
        },
        ValueFamily {
            type_: "String",
            value: "\"one\"",
            constant: Some("\"one\""),
            assertion: "selected == \"one\"",
        },
        ValueFamily {
            type_: "BitArray",
            value: "<<1>>",
            constant: Some("<<1>>"),
            assertion: "selected == <<1>>",
        },
        ValueFamily {
            type_: "UtfCodepoint",
            value: "codepoint()",
            constant: None,
            assertion: "selected == codepoint()",
        },
        ValueFamily {
            type_: "Marker",
            value: "Marker(1)",
            constant: Some("Marker(1)"),
            assertion: "selected == Marker(1)",
        },
        ValueFamily {
            type_: "Bool",
            value: "True",
            constant: Some("True"),
            assertion: "selected == True",
        },
        ValueFamily {
            type_: "Nil",
            value: "Nil",
            constant: Some("Nil"),
            assertion: "selected == Nil",
        },
        ValueFamily {
            type_: "#(Int)",
            value: "#(1)",
            constant: Some("#(1)"),
            assertion: "selected == #(1)",
        },
        ValueFamily {
            type_: "List(Int)",
            value: "[1]",
            constant: Some("[1]"),
            assertion: "selected == [1]",
        },
        ValueFamily {
            type_: "fn() -> Int",
            value: "int_value",
            constant: Some("int_value"),
            assertion: "selected() == 1",
        },
    ];

    #[test]
    fn every_value_family_lowers_each_expression_owner() {
        for family in VALUE_FAMILIES {
            let mut expressions = vec![
                family.value.to_owned(),
                format!("{{ let local = {} local }}", family.value),
                "provider()".to_owned(),
                "{ let callable = provider callable() }".to_owned(),
                format!("#({}).0", family.value),
                format!("Holder(selected: {}).selected", family.value),
                format!(
                    "case [{value}] {{ [selected] -> selected _ -> {value} }}",
                    value = family.value,
                ),
                format!(
                    "case True {{ True -> {value} False -> {value} }}",
                    value = family.value,
                ),
                format!(
                    "case 1 {{ 1 -> {value} _ -> {value} }}",
                    value = family.value,
                ),
                format!(
                    "case \"selected\" {{ \"selected\" -> {value} _ -> {value} }}",
                    value = family.value,
                ),
                format!(
                    "case 1.0 {{ 1.0 -> {value} _ -> {value} }}",
                    value = family.value,
                ),
                format!("{{ let _ = Nil {} }}", family.value),
            ];
            if family.constant.is_some() {
                expressions.push("selected_constant".to_owned());
            }

            for expression in expressions {
                let source = source(family, &expression);
                assert_eq!(
                    crate::run_main(&execution_plan(&source)),
                    Ok(Value::Bool(true)),
                    "failed value family {} expression {expression}",
                    family.type_,
                );
            }
        }
    }

    #[test]
    fn every_value_family_preserves_its_source_stop() {
        for family in VALUE_FAMILIES {
            let source = format!(
                r#"
pub type Marker {{ Marker(Int) }}

fn codepoint() -> UtfCodepoint {{
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}}

fn selected() -> {type_} {{ panic as "selected" }}

pub fn main() {{
  let selected = selected()
  {assertion}
}}
"#,
                type_ = family.type_,
                assertion = family.assertion,
            );
            let error = crate::run_main(&execution_plan(&source)).unwrap_err();
            assert_eq!(error.to_string(), "panic: selected");
        }
    }

    #[test]
    fn every_value_family_stops_when_an_owner_source_diverges() {
        for family in VALUE_FAMILIES {
            let expressions = [
                "provider(panic as \"source\")".to_owned(),
                "{ panic as \"source\" }()".to_owned(),
                format!(
                    "{{ let callable: fn() -> {} = panic as \"source\" callable() }}",
                    family.type_,
                ),
                format!("#(panic as \"source\", {}).1", family.value),
                "Holder(selected: panic as \"source\").selected".to_owned(),
                "{ panic as \"source\" }[0]".to_owned(),
                format!(
                    "case [panic as \"source\"] {{ [selected] -> selected _ -> {} }}",
                    family.value,
                ),
                format!(
                    "case panic as \"source\" {{ True -> {value} False -> {value} }}",
                    value = family.value,
                ),
                format!(
                    "case panic as \"source\" {{ 1 -> {value} _ -> {value} }}",
                    value = family.value,
                ),
                format!(
                    "case panic as \"source\" {{ \"selected\" -> {value} _ -> {value} }}",
                    value = family.value,
                ),
                format!(
                    "case panic as \"source\" {{ 1.0 -> {value} _ -> {value} }}",
                    value = family.value,
                ),
                format!(
                    "{{ let failed: Int = panic as \"source\" let _ = failed {} }}",
                    family.value,
                ),
            ];

            for expression in expressions {
                let source = diverging_source(family, &expression);
                let error = crate::run_main(&execution_plan(&source)).unwrap_err();
                assert_eq!(
                    error.to_string(),
                    "panic: source",
                    "failed value family {} expression {expression}",
                    family.type_,
                );
            }
        }
    }

    #[test]
    fn planner_generated_list_projections_stop_before_every_value_family_output() {
        let custom_name = CustomTypeName::new("geam".into(), "main".into(), "Marker".into());
        let custom_type = crate::plan::CustomType::new(custom_name.clone(), Vec::new());
        let custom_shape = CustomValueShape::new(
            custom_name.clone(),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        let custom_definition = CustomTypeDefinition::new(
            custom_name,
            CustomTypePublicity::Private,
            false,
            Vec::new(),
            vec![CustomConstructorDefinition::new(
                "Marker".into(),
                0,
                Vec::new(),
            )],
        );
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let expressions = vec![
            Expr::int(IntExpr::list_index(
                ListExpr::panic(panic(), ValueType::Int)
                    .into_int()
                    .expect("an Int item type should create an Int list"),
                0,
            )),
            Expr::float(FloatExpr::list_index(
                ListExpr::panic(panic(), ValueType::Float)
                    .into_float()
                    .expect("a Float item type should create a Float list"),
                0,
            )),
            Expr::string(StringExpr::list_index(
                ListExpr::panic(panic(), ValueType::String)
                    .into_string()
                    .expect("a String item type should create a String list"),
                0,
            )),
            Expr::bit_array(crate::plan::BitArrayExpr::list_index(
                ListExpr::panic(panic(), ValueType::BitArray)
                    .into_bit_array()
                    .expect("a BitArray item type should create a BitArray list"),
                0,
            )),
            Expr::utf_codepoint(UtfCodepointExpr::list_index(
                ListExpr::panic(panic(), ValueType::UtfCodepoint)
                    .into_utf_codepoint()
                    .expect("a UtfCodepoint item type should create a UtfCodepoint list"),
                0,
            )),
            Expr::custom(crate::plan::CustomExpr::list_index_shape(
                ListExpr::panic(panic(), ValueType::Custom(custom_type.clone()))
                    .into_custom()
                    .expect("a custom item type should create a custom list"),
                0,
                custom_shape,
            )),
            Expr::bool(BoolExpr::list_index(
                ListExpr::panic(panic(), ValueType::Bool)
                    .into_bool()
                    .expect("a Bool item type should create a Bool list"),
                0,
            )),
            Expr::nil(NilExpr::list_index(
                ListExpr::panic(panic(), ValueType::Nil)
                    .into_nil()
                    .expect("a Nil item type should create a Nil list"),
                0,
            )),
            Expr::tuple(TupleExpr::list_index(
                ListExpr::panic(panic(), ValueType::Tuple(vec![ValueType::Int]))
                    .into_tuple()
                    .expect("a tuple item type should create a tuple list"),
                0,
                vec![ValueType::Int],
            )),
            Expr::list(ListExpr::list_index(
                ListExpr::panic(panic(), ValueType::List(Box::new(ValueType::Int)))
                    .into_list()
                    .expect("a list item type should create a nested list"),
                0,
            )),
            Expr::function(FunctionExpr::int(IntFunctionExpr::list_index(
                ListExpr::panic(
                    panic(),
                    ValueType::Function(Box::new(function_type.clone())),
                )
                .into_function()
                .expect("a function item type should create a function list"),
                0,
                function_type,
            ))),
        ];
        let mut context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            custom_definition,
        ]);
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        assert_eq!(
            flow_outcome(&super::expr(
                &Expr::int(IntExpr::value(1.into())),
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Value,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(&super::expr(
                &Expr::generic(GenericExpr::local_get(
                    GenericLocal::new(GenericLocalId(0), TypeParameterId(0)),
                    "value".into(),
                )),
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Uninhabited,
        );

        for expression in expressions {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                flow_outcome(&super::expr(&expression, cursor, &mut graph, &mut context,)),
                FlowOutcome::Diverged,
            );
        }
    }

    fn source(family: &ValueFamily, expression: &str) -> String {
        let constant = match family.constant {
            Some(value) => format!("const selected_constant = {value}"),
            None => String::new(),
        };
        format!(
            r#"
pub type Marker {{ Marker(Int) }}
pub type Holder(value) {{ Holder(selected: value) }}

fn codepoint() -> UtfCodepoint {{
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}}

fn int_value() -> Int {{ 1 }}
fn provider() -> {type_} {{ {value} }}
{constant}

pub fn main() {{
  let selected: {type_} = {expression}
  {assertion}
}}
"#,
            type_ = family.type_,
            value = family.value,
            assertion = family.assertion,
        )
    }

    fn diverging_source(family: &ValueFamily, expression: &str) -> String {
        format!(
            r#"
pub type Marker {{ Marker(Int) }}
pub type Holder(value) {{ Holder(selected: value) }}

fn codepoint() -> UtfCodepoint {{
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}}

fn int_value() -> Int {{ 1 }}
fn provider(_value: Int) -> {type_} {{ {value} }}

pub fn main() {{
  let selected: {type_} = {expression}
  {assertion}
}}
"#,
            type_ = family.type_,
            value = family.value,
            assertion = family.assertion,
        )
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module)
    }
}
