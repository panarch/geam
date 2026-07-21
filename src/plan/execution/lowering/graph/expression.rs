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
    UninhabitedTupleValueShape, ValueInhabitation,
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
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<Vec<DraftValueRef>> {
    let mut values = Vec::with_capacity(args.len());
    for arg in args {
        match expr(arg.value(), cursor, graph, context) {
            Representability::Uninhabited => return Representability::Uninhabited,
            Representability::Inhabited(DraftFlow::Diverged) => {
                return Representability::Inhabited(DraftFlow::Diverged);
            }
            Representability::Inhabited(DraftFlow::Value {
                cursor: next,
                value,
            }) => {
                cursor = next;
                values.push(value);
            }
        }
    }
    Representability::Inhabited(DraftFlow::value(cursor, values))
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
                match context.representations.inhabitation(&shape) {
                    ValueInhabitation::Inhabited(_) => continue,
                    ValueInhabitation::Uninhabited(_) => DivergingCallArgument::Generic(expression),
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
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    for arg in arguments.prefix {
        match expr(arg.value(), cursor, graph, context) {
            Representability::Uninhabited => return Representability::Uninhabited,
            Representability::Inhabited(DraftFlow::Diverged) => {
                return Representability::Inhabited(());
            }
            Representability::Inhabited(DraftFlow::Value { cursor: next, .. }) => {
                cursor = next;
            }
        }
    }
    match arguments.value {
        DivergingCallArgument::Generic(expression) => {
            generic::never_expr(expression, cursor, graph, context)
        }
        DivergingCallArgument::Tuple { expression, proof } => {
            generic::tuple_never_expr(expression, &proof, cursor, graph, context)
        }
        DivergingCallArgument::Custom { expression, proof } => {
            generic::custom_never_expr(expression, &proof, cursor, graph, context)
        }
    }
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
        CallArguments::Complete => executable(cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value {
                cursor,
                value: function,
            } => call_args(args, cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    cursor,
                    value: args,
                } => emit(cursor, function, args, graph, context),
            }),
        }),
        CallArguments::Diverging(arguments) => {
            evaluated(cursor, graph, context).and_then(|flow| match flow {
                DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                DraftFlow::Value { cursor, value: () } => {
                    diverging_call_arguments(arguments, cursor, graph, context)
                        .map(|()| DraftFlow::Diverged)
                }
            })
        }
    }
}

pub(super) fn capture_args(
    function: &module::FunctionInstantiation,
    args: &[module::CaptureArg],
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<Vec<super::instruction::DraftFunctionCapture>> {
    let mut values = Vec::with_capacity(args.len());
    for (index, arg) in args.iter().enumerate() {
        match expr(arg.value(), cursor, graph, context) {
            Representability::Uninhabited => return Representability::Uninhabited,
            Representability::Inhabited(DraftFlow::Diverged) => {
                return Representability::Inhabited(DraftFlow::Diverged);
            }
            Representability::Inhabited(DraftFlow::Value {
                cursor: next,
                value,
            }) => {
                cursor = next;
                let target = match context
                    .target_capture_local(function, module::CapturePosition::new(index))
                {
                    Representability::Uninhabited => return Representability::Uninhabited,
                    Representability::Inhabited(target) => target,
                };
                let target = super::super::local::stored_value_local_at(
                    target.shape(),
                    target.index(),
                    context,
                );
                values.push(super::instruction::DraftFunctionCapture {
                    target,
                    source: value,
                });
            }
        }
    }
    Representability::Inhabited(DraftFlow::value(cursor, values))
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
            let mut branches = Vec::with_capacity(clauses.len() + 1);
            for (index, cursor) in clause_cursors.into_iter().enumerate() {
                let branch = &clauses[index].1;
                match lower(branch, cursor, graph, context) {
                    Representability::Inhabited(flow) => branches.push(flow),
                    Representability::Uninhabited => return Representability::Uninhabited,
                }
            }
            match lower(fallback, fallback_cursor, graph, context) {
                Representability::Inhabited(flow) => branches.push(flow),
                Representability::Uninhabited => return Representability::Uninhabited,
            }
            Representability::Inhabited(join_branches(scope, result_shape, branches, graph, result))
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
            let mut branches = Vec::with_capacity(clauses.len() + 1);
            for (index, cursor) in clause_cursors.into_iter().enumerate() {
                let branch = &clauses[index].1;
                match lower(branch, cursor, graph, context) {
                    Representability::Inhabited(flow) => branches.push(flow),
                    Representability::Uninhabited => return Representability::Uninhabited,
                }
            }
            match lower(fallback, fallback_cursor, graph, context) {
                Representability::Inhabited(flow) => branches.push(flow),
                Representability::Uninhabited => return Representability::Uninhabited,
            }
            Representability::Inhabited(join_branches(scope, result_shape, branches, graph, result))
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
            let mut branches = Vec::with_capacity(clauses.len() + 1);
            for (index, cursor) in clause_cursors.into_iter().enumerate() {
                let branch = &clauses[index].1;
                match lower(branch, cursor, graph, context) {
                    Representability::Inhabited(flow) => branches.push(flow),
                    Representability::Uninhabited => return Representability::Uninhabited,
                }
            }
            match lower(fallback, fallback_cursor, graph, context) {
                Representability::Inhabited(flow) => branches.push(flow),
                Representability::Uninhabited => return Representability::Uninhabited,
            }
            Representability::Inhabited(join_branches(scope, result_shape, branches, graph, result))
        }
    })
}

#[cfg(test)]
mod tests {
    use crate::Value;
    use crate::plan::execution::lowering::graph::{
        DraftCursor, DraftFlow, DraftGraph, DraftGraphBuilder, DraftValueRef,
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

    fn flow_outcome<T>(flow: Representability<DraftFlow<T>>) -> FlowOutcome {
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
            flow_outcome(super::panic_expr(
                &message_stop,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Diverged,
        );
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(super::expr(
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
            flow_outcome(Representability::Inhabited(all_diverged)),
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
                flow_outcome(super::int_case(
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
                flow_outcome(super::float_case(
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
                flow_outcome(super::string_case(
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

    fn uninhabited_tuple_proof(
        context: &crate::plan::execution::lowering::LoweringContext,
        elements: &[crate::plan::execution::lowering::specialization::SpecializedValueShape],
    ) -> crate::plan::execution::lowering::specialization::UninhabitedTupleValueShape {
        match context.representations.tuple_inhabitation(elements) {
            crate::plan::execution::lowering::specialization::CompoundInhabitation::Uninhabited(
                proof,
            ) => proof,
            crate::plan::execution::lowering::specialization::CompoundInhabitation::Inhabited => {
                panic!("expected an uninhabited tuple")
            }
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
        let uninhabited_tuple = TupleExpr::local_get(
            crate::plan::TupleLocalId(0),
            "tuple".into(),
            vec![ValueType::Parameter(parameter)],
        );
        let tuple_elements = uninhabited_tuple
            .shape()
            .iter()
            .map(|shape| context.concrete_value_shape(shape))
            .collect::<Vec<_>>();
        let tuple_proof = uninhabited_tuple_proof(&context, &tuple_elements);
        let args = vec![CallArg::new(Expr::tuple(uninhabited_tuple.clone()))];
        assert_eq!(
            super::call_arguments(&args, &context),
            super::CallArguments::Diverging(super::DivergingCallArguments {
                prefix: &[],
                value: super::DivergingCallArgument::Tuple {
                    expression: &uninhabited_tuple,
                    proof: tuple_proof,
                },
            }),
        );

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
    #[should_panic(expected = "expected an uninhabited tuple")]
    fn uninhabited_tuple_proof_rejects_inhabited_shape() {
        let context = crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        uninhabited_tuple_proof(
            &context,
            &[crate::plan::execution::lowering::specialization::SpecializedValueShape::Int],
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

        assert_eq!(
            flow_outcome(super::lower_function_call(
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
        let args = vec![CallArg::new(Expr::generic(local.clone()))];
        assert_eq!(
            flow_outcome(super::lower_function_call(
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
            flow_outcome(super::lower_function_call(
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
            flow_outcome(super::lower_function_call(
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
            flow_outcome(super::lower_function_call(
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
                flow_outcome(super::lower_function_call(
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
                flow_outcome(super::lower_function_call(
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
    fn capture_arguments_preserve_entry_destinations_and_source_outcomes() {
        let parameter = TypeParameterId(0);
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::new(1),
            TypeScheme::new(1),
            FunctionShape::new(vec![ValueShape::Parameter(parameter)], ValueShape::Int),
        );
        let concrete = signature
            .try_instantiate(vec![ValueShape::Int])
            .expect("one concrete argument should instantiate the capture target");
        let unresolved = signature
            .try_instantiate(vec![ValueShape::Parameter(parameter)])
            .expect("one parameter argument should instantiate the capture target");
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        let captures = vec![CaptureArg::new(Expr::int(IntExpr::value(1.into())))];
        let flow = super::capture_args(&concrete, &captures, cursor, &mut graph, &mut context);
        let captures = captured_values(flow);
        assert_eq!(captures.len(), 1);
        assert_eq!(
            captures[0].target,
            crate::plan::execution::ParamLocal::Int(crate::plan::execution::IntLocalId(1)),
        );
        assert_eq!(
            captures[0].source.shape(),
            &crate::plan::execution::lowering::specialization::StoredValueShape::Int,
        );

        let cursor = graph.empty_block(Default::default());
        let captures = vec![CaptureArg::new(Expr::int(IntExpr::panic(
            PanicExpr::panic_at(None, PanicSite::unknown()),
        )))];
        assert_eq!(
            flow_outcome(super::capture_args(
                &concrete,
                &captures,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Diverged,
        );

        let cursor = graph.empty_block(Default::default());
        let generic = GenericExpr::local_get(
            GenericLocal::new(GenericLocalId(0), parameter),
            "value".into(),
        );
        let captures = vec![CaptureArg::new(Expr::generic(generic))];
        assert_eq!(
            flow_outcome(super::capture_args(
                &unresolved,
                &captures,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Uninhabited,
        );

        let cursor = graph.empty_block(Default::default());
        let captures = vec![CaptureArg::new(Expr::int(IntExpr::value(1.into())))];
        assert_eq!(
            flow_outcome(super::capture_args(
                &unresolved,
                &captures,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Uninhabited,
        );
    }

    fn captured_values(
        flow: super::Lowered<Vec<super::super::instruction::DraftFunctionCapture>>,
    ) -> Vec<super::super::instruction::DraftFunctionCapture> {
        match flow {
            Representability::Inhabited(DraftFlow::Value { value, .. }) => value,
            Representability::Inhabited(DraftFlow::Diverged) => {
                panic!("expected capture values, found a diverged flow")
            }
            Representability::Uninhabited => {
                panic!("expected capture values, found an uninhabited flow")
            }
        }
    }

    #[test]
    #[should_panic(expected = "expected capture values, found a diverged flow")]
    fn capture_value_guard_rejects_diverged_flow() {
        captured_values(Representability::Inhabited(DraftFlow::Diverged));
    }

    #[test]
    #[should_panic(expected = "expected capture values, found an uninhabited flow")]
    fn capture_value_guard_rejects_uninhabited_flow() {
        captured_values(Representability::Uninhabited);
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
            flow_outcome(super::expr(
                &Expr::int(IntExpr::value(1.into())),
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Value,
        );

        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(super::expr(
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
                flow_outcome(super::expr(&expression, cursor, &mut graph, &mut context,)),
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
