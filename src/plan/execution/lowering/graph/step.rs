use super::expression::{
    bit_array_expr, bool_expr, custom_expr, float_expr, function, generic, int_expr, list,
    nil_expr, string_expr, tuple_expr, utf_codepoint_expr,
};
use super::pattern::{
    DraftBitArrayBindingPattern, DraftBitArrayPattern, DraftBitArrayPatternSegment,
    DraftBitArrayPatternSize, DraftBitArrayPatternSizeExpr, DraftBitArrayPatternValue,
    DraftBitArrayStringPattern, DraftMatchListTail, DraftMatchPattern, DraftMatchPatternBinding,
};
use super::{
    DraftCursor, DraftCustom, DraftFlow, DraftGraph, DraftGraphValue, DraftString, DraftTuple,
    DraftValueRef,
};
use crate::plan::execution::lowering::specialization::{
    CustomConstructorMatch, FunctionRepresentation, Representability, StoredValueShape,
};
use crate::plan::{execution, module};
use std::collections::HashMap;

type Lowered<T> = Representability<DraftFlow<T>>;

pub(super) fn steps(
    steps: &[module::Step],
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<()> {
    steps.iter().fold(
        Representability::Inhabited(DraftFlow::value(cursor, ())),
        |lowered, step| {
            lowered.and_then(|flow| {
                flow.and_then(|cursor, ()| lower_step(step, cursor, graph, context))
            })
        },
    )
}

fn bind_value<Value: DraftGraphValue>(
    key: super::super::local::LocalKey,
    lowered: Lowered<Value>,
) -> Lowered<()> {
    lowered.map(|flow| {
        flow.map_cursor(|cursor, value| {
            cursor.scope_mut().insert(key, value.erase());
        })
    })
}

fn lower_step(
    step: &module::Step,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<()> {
    use module::StepKind as S;

    match step.kind() {
        S::LetGeneric { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(
                super::super::local::LocalKind::Generic,
                local.id().0,
            ),
            generic::generic_expr(value, cursor, graph, context),
        ),
        S::LetInt { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::Int, local.0),
            int_expr(value, cursor, graph, context),
        ),
        S::LetFloat { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::Float, local.0),
            float_expr(value, cursor, graph, context),
        ),
        S::LetString { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::String, local.0),
            string_expr(value, cursor, graph, context),
        ),
        S::LetBitArray { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::BitArray, local.0),
            bit_array_expr(value, cursor, graph, context),
        ),
        S::LetUtfCodepoint { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(
                super::super::local::LocalKind::UtfCodepoint,
                local.0,
            ),
            utf_codepoint_expr(value, cursor, graph, context),
        ),
        S::LetCustom { binding, .. } => bind_value(
            super::super::local::LocalKey::new(
                super::super::local::LocalKind::Custom,
                binding.local().id().0,
            ),
            custom_expr(binding.value(), cursor, graph, context),
        ),
        S::LetBool { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::Bool, local.0),
            bool_expr(value, cursor, graph, context),
        ),
        S::LetNil { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::Nil, local.0),
            nil_expr(value, cursor, graph, context),
        ),
        S::LetTuple { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::Tuple, local.0),
            tuple_expr(value, cursor, graph, context),
        ),
        S::LetList { value, .. } => list_binding(value, cursor, graph, context),
        S::LetIntFunction { local, value, .. } => {
            let key = super::super::local::LocalKey::new(
                super::super::local::LocalKind::IntFunction,
                local.0,
            );
            let shape = context.concrete_function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                FunctionRepresentation::Symbolic
            ) {
                bind_value(
                    key,
                    function::symbolic_int_function_expr(
                        value.expression(),
                        &shape,
                        cursor,
                        graph,
                        context,
                    ),
                )
            } else {
                bind_value(
                    key,
                    function::int_function_expr(value.expression(), cursor, graph, context),
                )
            }
        }
        S::LetFloatFunction { local, value, .. } => {
            let key = super::super::local::LocalKey::new(
                super::super::local::LocalKind::FloatFunction,
                local.0,
            );
            let shape = context.concrete_function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                FunctionRepresentation::Symbolic
            ) {
                bind_value(
                    key,
                    function::symbolic_float_function_expr(
                        value.expression(),
                        &shape,
                        cursor,
                        graph,
                        context,
                    ),
                )
            } else {
                bind_value(
                    key,
                    function::float_function_expr(value.expression(), cursor, graph, context),
                )
            }
        }
        S::LetStringFunction { local, value, .. } => {
            let key = super::super::local::LocalKey::new(
                super::super::local::LocalKind::StringFunction,
                local.0,
            );
            let shape = context.concrete_function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                FunctionRepresentation::Symbolic
            ) {
                bind_value(
                    key,
                    function::symbolic_string_function_expr(
                        value.expression(),
                        &shape,
                        cursor,
                        graph,
                        context,
                    ),
                )
            } else {
                bind_value(
                    key,
                    function::string_function_expr(value.expression(), cursor, graph, context),
                )
            }
        }
        S::LetBitArrayFunction { local, value, .. } => {
            let key = super::super::local::LocalKey::new(
                super::super::local::LocalKind::BitArrayFunction,
                local.0,
            );
            let shape = context.concrete_function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                FunctionRepresentation::Symbolic
            ) {
                bind_value(
                    key,
                    function::symbolic_bit_array_function_expr(
                        value.expression(),
                        &shape,
                        cursor,
                        graph,
                        context,
                    ),
                )
            } else {
                bind_value(
                    key,
                    function::bit_array_function_expr(value.expression(), cursor, graph, context),
                )
            }
        }
        S::LetUtfCodepointFunction { local, value, .. } => {
            let key = super::super::local::LocalKey::new(
                super::super::local::LocalKind::UtfCodepointFunction,
                local.0,
            );
            let shape = context.concrete_function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                FunctionRepresentation::Symbolic
            ) {
                bind_value(
                    key,
                    function::symbolic_utf_codepoint_function_expr(
                        value.expression(),
                        &shape,
                        cursor,
                        graph,
                        context,
                    ),
                )
            } else {
                bind_value(
                    key,
                    function::utf_codepoint_function_expr(
                        value.expression(),
                        cursor,
                        graph,
                        context,
                    ),
                )
            }
        }
        S::LetCustomFunction { local, value, .. } => {
            let key = super::super::local::LocalKey::new(
                super::super::local::LocalKind::CustomFunction,
                local.id().0,
            );
            let shape = context.concrete_function_shape(value.shape());
            match context.function_representation(&shape) {
                FunctionRepresentation::Symbolic => bind_value(
                    key,
                    function::symbolic_custom_function_expr_kind(
                        value.expression().kind(),
                        &shape,
                        cursor,
                        graph,
                        context,
                    ),
                ),
                FunctionRepresentation::Never(_) => bind_value(
                    key,
                    function::custom_never_function_expr(
                        value.expression(),
                        cursor,
                        graph,
                        context,
                    ),
                ),
                FunctionRepresentation::Executable(_) => bind_value(
                    key,
                    function::custom_function_expr(value.expression(), cursor, graph, context),
                ),
            }
        }
        S::LetBoolFunction { local, value, .. } => {
            let key = super::super::local::LocalKey::new(
                super::super::local::LocalKind::BoolFunction,
                local.0,
            );
            let shape = context.concrete_function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                FunctionRepresentation::Symbolic
            ) {
                bind_value(
                    key,
                    function::symbolic_bool_function_expr(
                        value.expression(),
                        &shape,
                        cursor,
                        graph,
                        context,
                    ),
                )
            } else {
                bind_value(
                    key,
                    function::bool_function_expr(value.expression(), cursor, graph, context),
                )
            }
        }
        S::LetNilFunction { local, value, .. } => {
            let key = super::super::local::LocalKey::new(
                super::super::local::LocalKind::NilFunction,
                local.0,
            );
            let shape = context.concrete_function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                FunctionRepresentation::Symbolic
            ) {
                bind_value(
                    key,
                    function::symbolic_nil_function_expr(
                        value.expression(),
                        &shape,
                        cursor,
                        graph,
                        context,
                    ),
                )
            } else {
                bind_value(
                    key,
                    function::nil_function_expr(value.expression(), cursor, graph, context),
                )
            }
        }
        S::LetTupleFunction { local, value, .. } => {
            let key = super::super::local::LocalKey::new(
                super::super::local::LocalKind::TupleFunction,
                local.0,
            );
            let shape = context.concrete_function_shape(value.shape());
            match context.function_representation(&shape) {
                FunctionRepresentation::Symbolic => bind_value(
                    key,
                    function::symbolic_tuple_function_expr(
                        value.expression(),
                        &shape,
                        cursor,
                        graph,
                        context,
                    ),
                ),
                FunctionRepresentation::Never(_) => bind_value(
                    key,
                    function::tuple_never_function_expr(value.expression(), cursor, graph, context),
                ),
                FunctionRepresentation::Executable(_) => bind_value(
                    key,
                    function::tuple_function_expr(value.expression(), cursor, graph, context),
                ),
            }
        }
        S::LetListFunction { local, value, .. } => {
            let key = super::super::local::list_function_local_key(local);
            let shape = context.concrete_function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                FunctionRepresentation::Symbolic
            ) {
                bind_value(
                    key,
                    function::symbolic_list_function_expr(
                        value.expression(),
                        &shape,
                        cursor,
                        graph,
                        context,
                    ),
                )
            } else {
                bind_value(
                    key,
                    function::list_function_expr(value.expression(), cursor, graph, context),
                )
            }
        }
        S::LetFunctionFunction { local, value, .. } => {
            let key = super::super::local::LocalKey::new(
                super::super::local::LocalKind::FunctionFunction,
                local.id().0,
            );
            let shape = context.concrete_function_shape(value.shape());
            if matches!(
                context.function_representation(&shape),
                FunctionRepresentation::Symbolic
            ) {
                bind_value(
                    key,
                    function::symbolic_function_function_expr_kind(
                        value.expression().kind(),
                        &shape,
                        cursor,
                        graph,
                        context,
                    ),
                )
            } else {
                bind_value(
                    key,
                    function::function_function_expr(value.expression(), cursor, graph, context),
                )
            }
        }
        S::LetGenericFunction { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(
                super::super::local::LocalKind::GenericFunction,
                local.id().0,
            ),
            function::generic_function_expr(value.expression(), cursor, graph, context),
        ),
        S::AssertPattern {
            subject,
            pattern,
            message,
            site,
            pattern_span,
        } => assert_pattern(
            subject,
            pattern,
            LetAssertFailure {
                message: message.as_ref(),
                site,
                pattern_span: *pattern_span,
            },
            cursor,
            graph,
            context,
        ),
        S::BindCustomFields { local, pattern } => {
            let source = cursor.scope().custom(super::super::local::LocalKey::new(
                super::super::local::LocalKind::Custom,
                local.id().0,
            ));
            bind_custom_fields(pattern, source, cursor, graph, context)
                .map(|cursor| DraftFlow::value(cursor, ()))
        }
        S::AssertBool {
            condition,
            message,
            site,
        } => assert_bool(condition, message.as_ref(), site, cursor, graph, context),
        S::Evaluate(value) => {
            super::expression::expr(value, cursor, graph, context).map(|flow| flow.map(|_| ()))
        }
    }
}

fn list_binding(
    binding: &module::ListLocalExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<()> {
    match binding {
        module::ListLocalExpr::Generic { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(
                super::super::local::LocalKind::GenericList,
                local.0,
            ),
            list::generic_list_expr(value, cursor, graph, context),
        ),
        module::ListLocalExpr::ParameterList { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::ListList, local.0),
            list::parameter_list_list_expr(value, cursor, graph, context),
        ),
        module::ListLocalExpr::Int { local, value } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::IntList, local.0),
            list::int_list_expr(value, cursor, graph, context),
        ),
        module::ListLocalExpr::String { local, value } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::StringList, local.0),
            list::string_list_expr(value, cursor, graph, context),
        ),
        module::ListLocalExpr::BitArray { local, value } => bind_value(
            super::super::local::LocalKey::new(
                super::super::local::LocalKind::BitArrayList,
                local.0,
            ),
            list::bit_array_list_expr(value, cursor, graph, context),
        ),
        module::ListLocalExpr::UtfCodepoint { local, value } => bind_value(
            super::super::local::LocalKey::new(
                super::super::local::LocalKind::UtfCodepointList,
                local.0,
            ),
            list::utf_codepoint_list_expr(value, cursor, graph, context),
        ),
        module::ListLocalExpr::Custom { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::CustomList, local.0),
            list::custom_list_expr(value, cursor, graph, context),
        ),
        module::ListLocalExpr::Float { local, value } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::FloatList, local.0),
            list::float_list_expr(value, cursor, graph, context),
        ),
        module::ListLocalExpr::Bool { local, value } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::BoolList, local.0),
            list::bool_list_expr(value, cursor, graph, context),
        ),
        module::ListLocalExpr::Nil { local, value } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::NilList, local.0),
            list::nil_list_expr(value, cursor, graph, context),
        ),
        module::ListLocalExpr::Tuple { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::TupleList, local.0),
            list::tuple_list_expr(value, cursor, graph, context),
        ),
        module::ListLocalExpr::List { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(super::super::local::LocalKind::ListList, local.0),
            list::list_list_expr(value, cursor, graph, context),
        ),
        module::ListLocalExpr::Function { local, value, .. } => bind_value(
            super::super::local::LocalKey::new(
                super::super::local::LocalKind::FunctionList,
                local.0,
            ),
            list::function_list_expr(value, cursor, graph, context),
        ),
    }
}

fn assert_subject(subject: &module::AssertSubject, cursor: &DraftCursor) -> DraftValueRef {
    let scope = cursor.scope();
    match subject {
        module::AssertSubject::Int(local) => scope
            .int(super::super::local::LocalKey::new(
                super::super::local::LocalKind::Int,
                local.0,
            ))
            .erase(),
        module::AssertSubject::Float(local) => scope
            .float(super::super::local::LocalKey::new(
                super::super::local::LocalKind::Float,
                local.0,
            ))
            .erase(),
        module::AssertSubject::String(local) => scope
            .string(super::super::local::LocalKey::new(
                super::super::local::LocalKind::String,
                local.0,
            ))
            .erase(),
        module::AssertSubject::BitArray(local) => scope
            .bit_array(super::super::local::LocalKey::new(
                super::super::local::LocalKind::BitArray,
                local.0,
            ))
            .erase(),
        module::AssertSubject::Custom(local) => scope
            .custom(super::super::local::LocalKey::new(
                super::super::local::LocalKind::Custom,
                local.id().0,
            ))
            .erase(),
        module::AssertSubject::Bool(local) => scope
            .bool(super::super::local::LocalKey::new(
                super::super::local::LocalKind::Bool,
                local.0,
            ))
            .erase(),
        module::AssertSubject::Nil(local) => scope
            .nil(super::super::local::LocalKey::new(
                super::super::local::LocalKind::Nil,
                local.0,
            ))
            .erase(),
        module::AssertSubject::Tuple(local) => scope
            .tuple(super::super::local::LocalKey::new(
                super::super::local::LocalKind::Tuple,
                local.0,
            ))
            .erase(),
        module::AssertSubject::List(local) => scope
            .list(super::super::local::list_local_key(local))
            .erase(),
    }
}

struct LetAssertFailure<'a> {
    message: Option<&'a module::StringExpr>,
    site: &'a crate::plan::PanicSite,
    pattern_span: crate::plan::SourceSpan,
}

fn assert_pattern(
    subject_local: &module::AssertSubject,
    pattern: &module::AssertPattern,
    failure: LetAssertFailure<'_>,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<()> {
    let subject = assert_subject(subject_local, &cursor);
    let failure_scope = cursor.scope().clone();
    let mut planner = MatchPlanner::new(cursor.scope().clone(), graph, context);
    let pattern = planner.assert_pattern(pattern);
    let Representability::Inhabited(pattern) = pattern else {
        return finish_let_assert_failure(cursor, subject, failure, graph, context)
            .map(|()| DraftFlow::Diverged);
    };
    let (success_scope, bindings) = planner.finish();
    let success = graph.block(
        success_scope,
        bindings
            .iter()
            .map(|binding| binding.value.clone())
            .collect(),
    );
    let failure_cursor = graph.empty_block(failure_scope);
    graph.finish_match(
        cursor,
        subject.clone(),
        pattern,
        success.id(),
        bindings.len(),
        failure_cursor.id(),
    );
    finish_let_assert_failure(failure_cursor, subject, failure, graph, context)
        .map(|()| DraftFlow::value(success, ()))
}

fn finish_let_assert_failure(
    cursor: DraftCursor,
    subject: DraftValueRef,
    failure: LetAssertFailure<'_>,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<()> {
    lower_message(failure.message, cursor, graph, context).map(|flow| {
        flow.fold((), |cursor, message| {
            graph.finish_let_assert_panic(
                cursor,
                subject,
                message,
                failure.site.clone(),
                failure.pattern_span,
            );
        })
    })
}

fn lower_message(
    message: Option<&module::StringExpr>,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<Option<DraftString>> {
    match message {
        Some(message) => string_expr(message, cursor, graph, context).map(|flow| flow.map(Some)),
        None => Representability::Inhabited(DraftFlow::value(cursor, None)),
    }
}

fn assert_bool(
    condition: &module::BoolExpr,
    message: Option<&module::StringExpr>,
    site: &crate::plan::PanicSite,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Lowered<()> {
    lower_message(message, cursor, graph, context).and_then(|flow| {
        flow.and_then(|cursor, message| {
            super::expression::bool::bool_paths(condition, cursor, graph, context).map(|paths| {
                use super::expression::bool::BoolPaths;
                match paths {
                    BoolPaths::Diverged => DraftFlow::Diverged,
                    BoolPaths::True(cursor) => DraftFlow::value(cursor, ()),
                    BoolPaths::False(cursor) => {
                        finish_assert_failure(cursor, message, site, graph);
                        DraftFlow::Diverged
                    }
                    BoolPaths::Both { true_, false_ } => {
                        finish_assert_failure(false_, message, site, graph);
                        DraftFlow::value(true_, ())
                    }
                }
            })
        })
    })
}

fn finish_assert_failure(
    cursor: DraftCursor,
    message: Option<DraftString>,
    site: &crate::plan::PanicSite,
    graph: &mut DraftGraph,
) {
    graph.finish_source_stop(
        cursor,
        execution::graph::SourceStopKind::Assert,
        message,
        site.clone(),
    );
}

pub(super) fn bit_array_match_paths(
    value: &module::BitArrayExpr,
    pattern: &module::BitArrayPattern,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<super::expression::bool::BoolPaths> {
    bit_array_expr(value, cursor, graph, context).map(|flow| {
        flow.fold(
            super::expression::bool::BoolPaths::Diverged,
            |cursor, value| {
                match_paths(
                    value.erase(),
                    cursor,
                    graph,
                    context,
                    MatchPattern::BitArray(pattern),
                )
            },
        )
    })
}

pub(super) fn custom_match_paths(
    value: &module::CustomExpr,
    pattern: &module::CustomPattern,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<super::expression::bool::BoolPaths> {
    let source_shape = context.concrete_custom_value_shape(value.shape());
    let total = total_custom_match(pattern);
    let constructor_match = total.as_ref().map(|total| {
        context
            .representations
            .custom_constructor_match(&source_shape, total.constructor.index())
    });
    custom_expr(value, cursor, graph, context).and_then(|flow| {
        flow.fold(
            Representability::Inhabited(super::expression::bool::BoolPaths::Diverged),
            |cursor, value| match (constructor_match, total) {
                (Some(CustomConstructorMatch::Certain), Some(total)) => {
                    bind_certain_custom_match(total, value, cursor, graph, context)
                        .map(super::expression::bool::BoolPaths::True)
                }
                _ => Representability::Inhabited(match_paths(
                    value.erase(),
                    cursor,
                    graph,
                    context,
                    MatchPattern::Custom(pattern),
                )),
            },
        )
    })
}

struct TotalCustomMatch<'a> {
    constructor: &'a crate::plan::CustomConstructor,
    fields: &'a [module::TotalBindingPattern],
}

fn total_custom_match(pattern: &module::CustomPattern) -> Option<TotalCustomMatch<'_>> {
    pattern.total_fields().map(|fields| TotalCustomMatch {
        constructor: pattern.constructor(),
        fields,
    })
}

fn bind_certain_custom_match(
    total: TotalCustomMatch<'_>,
    source: DraftCustom,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftCursor> {
    total.fields.iter().enumerate().fold(
        Representability::Inhabited(cursor),
        |cursor, (index, field)| {
            cursor.and_then(|cursor| {
                bind_total_custom_field(field, source.clone(), index, cursor, graph, context)
            })
        },
    )
}

fn match_paths(
    subject: DraftValueRef,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
    pattern: MatchPattern<'_>,
) -> super::expression::bool::BoolPaths {
    let failure_scope = cursor.scope().clone();
    let mut planner = MatchPlanner::new(cursor.scope().clone(), graph, context);
    let pattern = match pattern {
        MatchPattern::Custom(pattern) => planner.custom_pattern(pattern),
        MatchPattern::BitArray(pattern) => Representability::Inhabited(
            DraftMatchPattern::BitArray(planner.bit_array_pattern(pattern)),
        ),
    };
    let Representability::Inhabited(pattern) = pattern else {
        return super::expression::bool::BoolPaths::False(cursor);
    };
    let (success_scope, bindings) = planner.finish();
    let success = graph.block(
        success_scope,
        bindings
            .iter()
            .map(|binding| binding.value.clone())
            .collect(),
    );
    let failure = graph.empty_block(failure_scope);
    graph.finish_match(
        cursor,
        subject,
        pattern,
        success.id(),
        bindings.len(),
        failure.id(),
    );
    super::expression::bool::BoolPaths::Both {
        true_: success,
        false_: failure,
    }
}

enum MatchPattern<'a> {
    Custom(&'a module::CustomPattern),
    BitArray(&'a module::BitArrayPattern),
}

struct MatchPlanner<'a> {
    scope: super::DraftScope,
    bindings: Vec<DraftMatchPatternBinding>,
    binding_indices: HashMap<super::super::local::LocalKey, usize>,
    graph: &'a mut DraftGraph,
    context: &'a mut super::super::LoweringContext,
}

impl<'a> MatchPlanner<'a> {
    fn new(
        scope: super::DraftScope,
        graph: &'a mut DraftGraph,
        context: &'a mut super::super::LoweringContext,
    ) -> Self {
        Self {
            scope,
            bindings: Vec::new(),
            binding_indices: HashMap::new(),
            graph,
            context,
        }
    }

    fn finish(self) -> (super::DraftScope, Vec<DraftMatchPatternBinding>) {
        (self.scope, self.bindings)
    }

    fn bind(
        &mut self,
        key: super::super::local::LocalKey,
        shape: StoredValueShape,
    ) -> DraftMatchPatternBinding {
        let index = self.bindings.len();
        let value = self.graph.value_ref(shape);
        self.scope.insert(key, value.clone());
        self.binding_indices.insert(key, index);
        let binding = DraftMatchPatternBinding { value, index };
        self.bindings.push(binding.clone());
        binding
    }

    fn assert_binding(
        &mut self,
        binding: &module::AssertBinding,
    ) -> Representability<DraftMatchPatternBinding> {
        let shape = self.context.concrete_value_shape(binding.slot().shape());
        self.context
            .representations
            .inhabitation(&shape)
            .into_representability()
            .map(|shape| {
                self.bind(
                    super::super::local::param_local_key(binding.slot().local()),
                    shape,
                )
            })
    }

    fn assert_pattern(
        &mut self,
        pattern: &module::AssertPattern,
    ) -> Representability<DraftMatchPattern> {
        match pattern {
            module::AssertPattern::Bind(binding) => {
                self.assert_binding(binding).map(DraftMatchPattern::Bind)
            }
            module::AssertPattern::Discard => {
                Representability::Inhabited(DraftMatchPattern::Discard)
            }
            module::AssertPattern::Int(value) => {
                Representability::Inhabited(DraftMatchPattern::Int(value.clone()))
            }
            module::AssertPattern::Float(value) => {
                Representability::Inhabited(DraftMatchPattern::Float(*value))
            }
            module::AssertPattern::String(value) => {
                Representability::Inhabited(DraftMatchPattern::String(value.clone()))
            }
            module::AssertPattern::Bool(value) => {
                Representability::Inhabited(DraftMatchPattern::Bool(*value))
            }
            module::AssertPattern::Nil => Representability::Inhabited(DraftMatchPattern::Nil),
            module::AssertPattern::Tuple(elements) => Representability::collect(
                elements.iter().map(|element| self.assert_pattern(element)),
            )
            .map(DraftMatchPattern::Tuple),
            module::AssertPattern::List(pattern) => {
                let item =
                    self.context
                        .concrete_value_shape(&crate::plan::ValueShape::from_value_type(
                            pattern.element_type().clone(),
                        ));
                if !pattern.elements().is_empty()
                    && !self.context.representations.is_inhabited(&item)
                {
                    return Representability::Uninhabited;
                }
                Representability::collect(
                    pattern
                        .elements()
                        .iter()
                        .map(|element| self.assert_pattern(element)),
                )
                .map(|elements| DraftMatchPattern::List {
                    elements,
                    tail: pattern.tail().map(|tail| self.list_tail(tail)),
                })
            }
            module::AssertPattern::BitArray(pattern) => Representability::Inhabited(
                DraftMatchPattern::BitArray(self.bit_array_pattern(pattern)),
            ),
            module::AssertPattern::Custom(pattern) => self.custom_pattern(pattern),
            module::AssertPattern::StringPrefix {
                prefix,
                left,
                right,
            } => Representability::Inhabited(DraftMatchPattern::StringPrefix {
                prefix: prefix.clone(),
                left: left.as_ref().map(|binding| {
                    self.bind(
                        super::super::local::LocalKey::new(
                            super::super::local::LocalKind::String,
                            binding.local().0,
                        ),
                        StoredValueShape::String,
                    )
                }),
                right: right.as_ref().map(|binding| {
                    self.bind(
                        super::super::local::LocalKey::new(
                            super::super::local::LocalKind::String,
                            binding.local().0,
                        ),
                        StoredValueShape::String,
                    )
                }),
            }),
            module::AssertPattern::Alias { pattern, binding } => self
                .assert_pattern(pattern)
                .zip_with(self.assert_binding(binding), |pattern, binding| {
                    DraftMatchPattern::Alias {
                        pattern: Box::new(pattern),
                        binding,
                    }
                }),
        }
    }

    fn custom_pattern(
        &mut self,
        pattern: &module::CustomPattern,
    ) -> Representability<DraftMatchPattern> {
        let source = self
            .context
            .concrete_custom_value_shape(&crate::plan::CustomValueShape::any(
                pattern.constructor().type_().clone(),
            ));
        if self
            .context
            .representations
            .custom_constructor_match(&source, pattern.constructor().index())
            == CustomConstructorMatch::Impossible
        {
            return Representability::Uninhabited;
        }
        Representability::collect(
            pattern
                .fields()
                .iter()
                .map(|field| self.assert_pattern(field)),
        )
        .map(|fields| DraftMatchPattern::Custom {
            constructor: self
                .context
                .custom_constructor(pattern.constructor().clone()),
            fields,
        })
    }

    fn list_tail(&mut self, tail: &module::ListAssertTail) -> DraftMatchListTail {
        match tail {
            module::ListAssertTail::Ignore => DraftMatchListTail::Ignore,
            module::ListAssertTail::Bind(binding) => {
                let item =
                    self.context
                        .concrete_value_shape(&crate::plan::ValueShape::from_value_type(
                            binding.local().item_type(),
                        ));
                DraftMatchListTail::Bind(self.bind(
                    super::super::local::list_local_key(binding.local()),
                    StoredValueShape::List(Box::new(item)),
                ))
            }
        }
    }

    fn bit_array_pattern(&mut self, pattern: &module::BitArrayPattern) -> DraftBitArrayPattern {
        DraftBitArrayPattern {
            segments: pattern
                .segments()
                .iter()
                .map(|segment| self.bit_array_segment(segment))
                .collect(),
        }
    }

    fn bit_array_segment(
        &mut self,
        segment: &module::BitArrayPatternSegment,
    ) -> DraftBitArrayPatternSegment {
        use module::BitArrayPatternSegment as S;

        match segment {
            S::Int {
                pattern,
                size,
                endianness,
                signedness,
            } => DraftBitArrayPatternSegment::Int {
                pattern: self.bit_array_value(
                    pattern,
                    super::super::local::LocalKind::Int,
                    StoredValueShape::Int,
                ),
                size: self.bit_array_size(size),
                endianness: lower_endianness(*endianness),
                signedness: lower_signedness(*signedness),
            },
            S::Float {
                pattern,
                size,
                endianness,
            } => DraftBitArrayPatternSegment::Float {
                pattern: self.bit_array_value(
                    pattern,
                    super::super::local::LocalKind::Float,
                    StoredValueShape::Float,
                ),
                size: self.bit_array_size(size),
                endianness: lower_endianness(*endianness),
            },
            S::Bits {
                pattern,
                size,
                unit,
            } => DraftBitArrayPatternSegment::Bits {
                pattern: self.bit_array_binding(
                    pattern,
                    super::super::local::LocalKind::BitArray,
                    StoredValueShape::BitArray,
                ),
                size: size.as_ref().map(|size| self.bit_array_size(size)),
                unit: *unit,
            },
            S::String { pattern, encoding } => DraftBitArrayPatternSegment::String {
                pattern: match pattern {
                    module::BitArrayStringPattern::Literal(value) => {
                        DraftBitArrayStringPattern::Literal(value.clone())
                    }
                    module::BitArrayStringPattern::Discard => DraftBitArrayStringPattern::Discard,
                },
                encoding: lower_string_encoding(*encoding),
            },
            S::UtfCodepoint { pattern, encoding } => DraftBitArrayPatternSegment::UtfCodepoint {
                pattern: self.bit_array_binding(
                    pattern,
                    super::super::local::LocalKind::UtfCodepoint,
                    StoredValueShape::UtfCodepoint,
                ),
                encoding: lower_string_encoding(*encoding),
            },
        }
    }

    fn bit_array_value<Value: Clone, Local: Copy + IntoLocalIndex>(
        &mut self,
        pattern: &module::BitArrayPatternValue<Value, Local>,
        kind: super::super::local::LocalKind,
        shape: StoredValueShape,
    ) -> DraftBitArrayPatternValue<Value> {
        match pattern {
            module::BitArrayPatternValue::Literal(value) => {
                DraftBitArrayPatternValue::Literal(value.clone())
            }
            module::BitArrayPatternValue::Bind(binding) => {
                DraftBitArrayPatternValue::Bind(self.bit_array_known_binding(binding, kind, shape))
            }
            module::BitArrayPatternValue::Discard => DraftBitArrayPatternValue::Discard,
            module::BitArrayPatternValue::Alias { pattern, binding } => {
                DraftBitArrayPatternValue::Alias {
                    pattern: Box::new(self.bit_array_value(pattern, kind, shape.clone())),
                    binding: self.bit_array_known_binding(binding, kind, shape),
                }
            }
        }
    }

    fn bit_array_binding<Local: Copy + IntoLocalIndex>(
        &mut self,
        pattern: &module::BitArrayBindingPattern<Local>,
        kind: super::super::local::LocalKind,
        shape: StoredValueShape,
    ) -> DraftBitArrayBindingPattern {
        match pattern {
            module::BitArrayBindingPattern::Bind(binding) => {
                DraftBitArrayBindingPattern::Bind(self.bind(
                    super::super::local::LocalKey::new(kind, binding.local().index()),
                    shape,
                ))
            }
            module::BitArrayBindingPattern::Discard => DraftBitArrayBindingPattern::Discard,
            module::BitArrayBindingPattern::Alias { pattern, binding } => {
                DraftBitArrayBindingPattern::Alias {
                    pattern: Box::new(self.bit_array_binding(pattern, kind, shape.clone())),
                    binding: self.bind(
                        super::super::local::LocalKey::new(kind, binding.local().index()),
                        shape,
                    ),
                }
            }
        }
    }

    fn bit_array_known_binding<Local: Copy + IntoLocalIndex>(
        &mut self,
        binding: &module::PatternBinding<Local>,
        kind: super::super::local::LocalKind,
        shape: StoredValueShape,
    ) -> DraftMatchPatternBinding {
        self.bind(
            super::super::local::LocalKey::new(kind, binding.local().index()),
            shape,
        )
    }

    fn bit_array_size(&self, size: &module::BitArrayPatternSize) -> DraftBitArrayPatternSize {
        DraftBitArrayPatternSize {
            value: self.bit_array_size_expr(size.value()),
            unit: size.unit(),
        }
    }

    fn bit_array_size_expr(
        &self,
        value: &module::BitArrayPatternSizeExpr,
    ) -> DraftBitArrayPatternSizeExpr {
        use module::BitArrayPatternSizeExpr as E;

        match value {
            E::Value(value) => DraftBitArrayPatternSizeExpr::Value(value.clone()),
            E::LocalGet { local, .. } => {
                let key = super::super::local::LocalKey::new(
                    super::super::local::LocalKind::Int,
                    local.0,
                );
                match self.binding_indices.get(&key) {
                    Some(index) => DraftBitArrayPatternSizeExpr::Binding(*index),
                    None => DraftBitArrayPatternSizeExpr::Local(self.scope.int(key)),
                }
            }
            E::Add { left, right } => DraftBitArrayPatternSizeExpr::Add {
                left: Box::new(self.bit_array_size_expr(left)),
                right: Box::new(self.bit_array_size_expr(right)),
            },
            E::Subtract { left, right } => DraftBitArrayPatternSizeExpr::Subtract {
                left: Box::new(self.bit_array_size_expr(left)),
                right: Box::new(self.bit_array_size_expr(right)),
            },
            E::Multiply { left, right } => DraftBitArrayPatternSizeExpr::Multiply {
                left: Box::new(self.bit_array_size_expr(left)),
                right: Box::new(self.bit_array_size_expr(right)),
            },
            E::Divide { left, right } => DraftBitArrayPatternSizeExpr::Divide {
                left: Box::new(self.bit_array_size_expr(left)),
                right: Box::new(self.bit_array_size_expr(right)),
            },
            E::Remainder { left, right } => DraftBitArrayPatternSizeExpr::Remainder {
                left: Box::new(self.bit_array_size_expr(left)),
                right: Box::new(self.bit_array_size_expr(right)),
            },
        }
    }
}

trait IntoLocalIndex {
    fn index(self) -> usize;
}

impl IntoLocalIndex for crate::plan::IntLocalId {
    fn index(self) -> usize {
        self.0
    }
}

impl IntoLocalIndex for crate::plan::FloatLocalId {
    fn index(self) -> usize {
        self.0
    }
}

impl IntoLocalIndex for crate::plan::BitArrayLocalId {
    fn index(self) -> usize {
        self.0
    }
}

impl IntoLocalIndex for crate::plan::UtfCodepointLocalId {
    fn index(self) -> usize {
        self.0
    }
}

fn bind_custom_fields(
    pattern: &module::CustomBindingPattern,
    source: DraftCustom,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftCursor> {
    pattern.fields().iter().enumerate().fold(
        Representability::Inhabited(cursor),
        |cursor, (index, pattern)| {
            cursor.and_then(|cursor| {
                bind_total_custom_field(pattern, source.clone(), index, cursor, graph, context)
            })
        },
    )
}

fn bind_total_custom_field(
    pattern: &module::TotalBindingPattern,
    source: DraftCustom,
    index: usize,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftCursor> {
    if !total_pattern_requires_value(pattern) {
        return Representability::Inhabited(cursor);
    }
    stored_pattern_shape(pattern, context).and_then(|shape| {
        let (cursor, value) = generic::custom_field(&shape, source, index, cursor, graph, context);
        bind_total_pattern(pattern, value, cursor, graph, context)
    })
}

fn bind_total_pattern(
    pattern: &module::TotalBindingPattern,
    source: DraftValueRef,
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::LoweringContext,
) -> Representability<DraftCursor> {
    use module::TotalBindingPatternKind as P;

    match pattern.kind() {
        P::Bind(binding) => stored_pattern_shape(pattern, context).map(|_| {
            cursor.scope_mut().insert(
                super::super::local::param_local_key(binding.slot().local()),
                source,
            );
            cursor
        }),
        P::Discard => Representability::Inhabited(cursor),
        P::Tuple(elements) => {
            let tuple = DraftTuple::from_ref(&source);
            elements.iter().enumerate().fold(
                Representability::Inhabited(cursor),
                |cursor, (index, element)| {
                    cursor.and_then(|cursor| {
                        if !total_pattern_requires_value(element) {
                            return Representability::Inhabited(cursor);
                        }
                        stored_pattern_shape(element, context).and_then(|shape| {
                            let (cursor, value) = generic::tuple_index(
                                &shape,
                                tuple.clone(),
                                index,
                                cursor,
                                graph,
                                context,
                            );
                            bind_total_pattern(element, value, cursor, graph, context)
                        })
                    })
                },
            )
        }
        P::List(tail) => {
            if let module::ListAssertTail::Bind(binding) = tail {
                cursor
                    .scope_mut()
                    .insert(super::super::local::list_local_key(binding.local()), source);
            }
            Representability::Inhabited(cursor)
        }
        P::Custom(pattern) => bind_custom_fields(
            pattern,
            DraftCustom::from_ref(&source),
            cursor,
            graph,
            context,
        ),
        P::Alias { pattern, binding } => {
            bind_total_pattern(pattern, source.clone(), cursor, graph, context).and_then(
                |mut cursor| {
                    stored_pattern_shape(pattern, context).map(|_| {
                        cursor.scope_mut().insert(
                            super::super::local::param_local_key(binding.slot().local()),
                            source,
                        );
                        cursor
                    })
                },
            )
        }
    }
}

fn lower_endianness(value: module::Endianness) -> crate::plan::execution::Endianness {
    match value {
        module::Endianness::Big => crate::plan::execution::Endianness::Big,
        module::Endianness::Little => crate::plan::execution::Endianness::Little,
    }
}

fn lower_signedness(value: module::Signedness) -> crate::plan::execution::Signedness {
    match value {
        module::Signedness::Signed => crate::plan::execution::Signedness::Signed,
        module::Signedness::Unsigned => crate::plan::execution::Signedness::Unsigned,
    }
}

fn lower_string_encoding(value: module::StringEncoding) -> crate::plan::execution::StringEncoding {
    match value {
        module::StringEncoding::Utf8 => crate::plan::execution::StringEncoding::Utf8,
        module::StringEncoding::Utf16(endianness) => {
            crate::plan::execution::StringEncoding::Utf16(lower_endianness(endianness))
        }
        module::StringEncoding::Utf32(endianness) => {
            crate::plan::execution::StringEncoding::Utf32(lower_endianness(endianness))
        }
    }
}

fn stored_pattern_shape(
    pattern: &module::TotalBindingPattern,
    context: &super::super::LoweringContext,
) -> Representability<StoredValueShape> {
    let shape = context.concrete_value_shape(&crate::plan::ValueShape::from_value_type(
        pattern.type_().clone(),
    ));
    context
        .representations
        .representation(&shape)
        .into_representability()
}

fn total_pattern_requires_value(pattern: &module::TotalBindingPattern) -> bool {
    match pattern.kind() {
        module::TotalBindingPatternKind::Bind(_) => true,
        module::TotalBindingPatternKind::Discard => false,
        module::TotalBindingPatternKind::Tuple(elements) => {
            elements.iter().any(total_pattern_requires_value)
        }
        module::TotalBindingPatternKind::List(tail) => {
            matches!(tail, module::ListAssertTail::Bind(_))
        }
        module::TotalBindingPatternKind::Custom(pattern) => {
            pattern.fields().iter().any(total_pattern_requires_value)
        }
        module::TotalBindingPatternKind::Alias { .. } => true,
    }
}

#[cfg(test)]
mod tests {
    use super::super::{DraftCustom, DraftGraphBuilder, DraftValueRef};
    use crate::plan::execution::lowering::specialization::{
        Representability, SpecializedValueShape, StoredValueShape,
    };
    use crate::plan::{
        AssertBinding, BoolExpr, CustomBindingPattern, CustomConstructor,
        CustomConstructorDefinition, CustomConstructorField, CustomFieldDefinition, CustomPattern,
        CustomType, CustomTypeDefinition, CustomTypeName, CustomTypeParameterId,
        CustomTypePublicity, CustomTypeTemplate, CustomValueShape, GenericLocal, GenericLocalId,
        IntListLocalId, ListAssertTail, ListLocal, PanicExpr, PanicSite, ParamLocal,
        TotalBindingPattern, TypeParameterId, ValueShape, ValueType,
    };

    #[test]
    fn bool_assertion_preserves_a_diverging_condition() {
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let condition = BoolExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown()));
        let lowered = super::assert_bool(
            &condition,
            None,
            &PanicSite::unknown(),
            cursor,
            &mut graph,
            &mut context,
        )
        .map(|flow| flow.fold(false, |_, ()| true));

        assert_eq!(lowered, Representability::Inhabited(false));
    }

    #[test]
    fn total_custom_match_accepts_only_complete_custom_patterns() {
        let name = CustomTypeName::new("geam".into(), "main".into(), "Empty".into());
        let type_ = CustomType::new(name, Vec::new());
        let constructor = CustomConstructor::new(type_.clone(), "Empty".into(), 0, Vec::new());
        let custom = CustomPattern::new(constructor, Vec::new(), Some(Vec::new()));
        assert_eq!(
            super::total_custom_match(&custom)
                .map(|total| (total.constructor.index(), total.fields.len())),
            Some((0, 0)),
        );
        assert_eq!(
            super::total_custom_match(&CustomPattern::new(
                CustomConstructor::new(type_, "Empty".into(), 0, Vec::new()),
                Vec::new(),
                None,
            ))
            .map(|_| ()),
            None,
        );
    }

    #[test]
    fn uninhabited_total_bindings_stop_custom_and_tuple_lowering() {
        let parameter = TypeParameterId(0);
        let name = CustomTypeName::new("geam".into(), "main".into(), "Choice".into());
        let definition = CustomTypeDefinition::new(
            name.clone(),
            CustomTypePublicity::Private,
            false,
            vec![CustomTypeParameterId(0)],
            vec![
                CustomConstructorDefinition::new("Empty".into(), 0, Vec::new()),
                CustomConstructorDefinition::new(
                    "Filled".into(),
                    1,
                    vec![CustomFieldDefinition::new(
                        None,
                        CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                    )],
                ),
            ],
        );
        let type_ = CustomType::new(name.clone(), vec![ValueType::Parameter(parameter)]);
        let source_shape = CustomValueShape::any(type_.clone());
        let filled = CustomConstructor::new(
            type_.clone(),
            "Filled".into(),
            1,
            vec![CustomConstructorField::new(
                None,
                ValueType::Parameter(parameter),
            )],
        );
        let field_binding = AssertBinding::new(
            ParamLocal::generic(GenericLocal::new(GenericLocalId(0), parameter)),
            "field".into(),
            ValueShape::Parameter(parameter),
        );
        let field_pattern = TotalBindingPattern::bind(field_binding.clone());
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(vec![definition]);
        let stored_custom =
            StoredValueShape::Custom(context.concrete_custom_value_shape(&source_shape));

        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let source = DraftCustom::from_owned(graph.value_ref(stored_custom.clone()));
        assert_eq!(
            super::bind_certain_custom_match(
                super::TotalCustomMatch {
                    constructor: &filled,
                    fields: std::slice::from_ref(&field_pattern),
                },
                source,
                cursor,
                &mut graph,
                &mut context,
            )
            .map(|_| ()),
            Representability::Uninhabited,
        );

        let custom_pattern =
            CustomBindingPattern::exact(source_shape, filled, vec![field_pattern.clone()]);
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let source = DraftCustom::from_owned(graph.value_ref(stored_custom));
        assert_eq!(
            super::bind_custom_fields(&custom_pattern, source, cursor, &mut graph, &mut context,)
                .map(|_| ()),
            Representability::Uninhabited,
        );

        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let mut planner =
            super::MatchPlanner::new(cursor.scope().clone(), &mut graph, &mut context);
        assert_eq!(
            planner.assert_binding(&field_binding).map(|_| ()),
            Representability::Uninhabited,
        );
        assert_eq!(
            super::stored_pattern_shape(&field_pattern, &context).map(|_| ()),
            Representability::Uninhabited,
        );

        let tuple_pattern = TotalBindingPattern::tuple(vec![field_pattern]);
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let tuple = graph.value_ref(StoredValueShape::Tuple(
            vec![SpecializedValueShape::Int].into_boxed_slice(),
        ));
        assert_eq!(
            super::bind_total_pattern(&tuple_pattern, tuple, cursor, &mut graph, &mut context,)
                .map(|_| ()),
            Representability::Uninhabited,
        );
    }

    #[test]
    fn total_list_bindings_preserve_ignored_and_bound_tails() {
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let ignored = TotalBindingPattern::list(ValueType::Int, ListAssertTail::Ignore);
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let list = graph.value_ref(StoredValueShape::List(Box::new(SpecializedValueShape::Int)));
        assert_eq!(
            super::bind_total_pattern(&ignored, list, cursor, &mut graph, &mut context).map(|_| ()),
            Representability::Inhabited(()),
        );

        let local = ListLocal::int(IntListLocalId(0));
        let bound = TotalBindingPattern::list(
            ValueType::Int,
            ListAssertTail::bind(local.clone(), "rest".into()),
        );
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let list = graph.value_ref(StoredValueShape::List(Box::new(SpecializedValueShape::Int)));
        assert_eq!(
            super::bind_total_pattern(&bound, list, cursor, &mut graph, &mut context).map(
                |cursor| {
                    cursor
                        .scope()
                        .list(super::super::super::local::list_local_key(&local))
                        .shape()
                        .clone()
                }
            ),
            Representability::Inhabited(StoredValueShape::List(Box::new(
                SpecializedValueShape::Int,
            ))),
        );
    }
}
