use super::super::{call_args, custom, list, lower_function_call, tuple};
use super::{
    DraftCursor, DraftFlow, DraftGraph, FunctionTarget, Representability, SpecializedFunctionShape,
    closure, function_function_expr, reference, source_stop,
};
use crate::plan::execution::lowering::graph::{
    DraftBitArrayFunction, DraftBoolFunction, DraftFloatFunction, DraftFunction,
    DraftGenericFunction, DraftGraphValue, DraftIntFunction, DraftNilFunction, DraftStringFunction,
    DraftTupleFunction, DraftUtfCodepointFunction,
};
use crate::plan::execution::lowering::specialization::StoredValueShape;
use crate::plan::{execution, module};

pub(in crate::plan::execution::lowering) fn executable_function_expr(
    expression: &module::GenericFunctionExpr,
    shape: &SpecializedFunctionShape,
    return_: &StoredValueShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunction>> {
    use StoredValueShape as S;

    match return_ {
        S::Int => generic_int_function_expr(expression, shape, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        S::Float => generic_float_function_expr(expression, shape, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        S::String => generic_string_function_expr(expression, shape, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        S::BitArray => generic_bit_array_function_expr(expression, shape, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        S::UtfCodepoint => {
            generic_utf_codepoint_function_expr(expression, shape, cursor, graph, context)
                .map(|flow| flow.map(|value| value.value().clone()))
        }
        S::Custom(return_) => super::custom::generic_custom_function_expr(
            expression, return_, shape, cursor, graph, context,
        )
        .map(|flow| flow.map(|value| value.value().clone())),
        S::Bool => generic_bool_function_expr(expression, shape, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        S::Nil => generic_nil_function_expr(expression, shape, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        S::Tuple(_) => generic_tuple_function_expr(expression, shape, cursor, graph, context)
            .map(|flow| flow.map(|value| value.value().clone())),
        S::List(item) => {
            super::list::generic_list_function_expr(expression, item, shape, cursor, graph, context)
                .map(|flow| flow.map(|value| value.value().clone()))
        }
        S::Function(return_) => super::returning_function::generic_function_function_expr(
            expression, return_, shape, cursor, graph, context,
        )
        .map(|flow| flow.map(|value| value.value().clone())),
    }
}

pub(super) struct ExecutableKindLowering<Make, Constant, Target, Direct, Branch> {
    make: Make,
    constant: Constant,
    target: Target,
    direct: Direct,
    branch: Branch,
}

pub(super) fn executable_kind_lowering<Value, Make, Constant, Target, Direct, Branch>(
    make: Make,
    constant: Constant,
    target: Target,
    direct: Direct,
    branch: Branch,
) -> ExecutableKindLowering<Make, Constant, Target, Direct, Branch>
where
    Value: DraftGraphValue,
    Make: Copy + Fn(DraftFunction) -> Value,
    Constant: Copy
        + Fn(
            &module::ConstantGenericFunctionInstantiation,
            &mut super::super::super::LoweringContext,
        ) -> Representability<usize>,
    Target: Copy
        + Fn(
            &module::FunctionInstantiation,
            &mut super::super::super::LoweringContext,
        ) -> Representability<FunctionTarget>,
    Direct: Copy
        + Fn(
            &module::FunctionInstantiation,
            &mut super::super::super::LoweringContext,
        ) -> Representability<execution::function::FunctionFunctionId>,
    Branch: Copy
        + Fn(
            &module::GenericFunctionExprKind,
            DraftCursor,
            &mut DraftGraph,
            &mut super::super::super::LoweringContext,
        ) -> Representability<DraftFlow<Value>>,
{
    ExecutableKindLowering {
        make,
        constant,
        target,
        direct,
        branch,
    }
}

pub(super) fn lower_executable_kind<Value, Make, Constant, Target, Direct, Branch>(
    kind: &module::GenericFunctionExprKind,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
    lowering: ExecutableKindLowering<Make, Constant, Target, Direct, Branch>,
) -> Representability<DraftFlow<Value>>
where
    Value: DraftGraphValue,
    Make: Copy + Fn(DraftFunction) -> Value,
    Constant: Copy
        + Fn(
            &module::ConstantGenericFunctionInstantiation,
            &mut super::super::super::LoweringContext,
        ) -> Representability<usize>,
    Target: Copy
        + Fn(
            &module::FunctionInstantiation,
            &mut super::super::super::LoweringContext,
        ) -> Representability<FunctionTarget>,
    Direct: Copy
        + Fn(
            &module::FunctionInstantiation,
            &mut super::super::super::LoweringContext,
        ) -> Representability<execution::function::FunctionFunctionId>,
    Branch: Copy
        + Fn(
            &module::GenericFunctionExprKind,
            DraftCursor,
            &mut DraftGraph,
            &mut super::super::super::LoweringContext,
        ) -> Representability<DraftFlow<Value>>,
{
    use super::super::super::instruction::DraftFunctionInstruction as I;
    use module::GenericFunctionExprKind as E;

    let ExecutableKindLowering {
        make,
        constant,
        target,
        direct,
        branch,
    } = lowering;
    let stored = StoredValueShape::Function(Box::new(shape.clone()));
    match kind {
        E::Constant(value) => constant(value, context).map(|index| {
            let mut cursor = cursor;
            let value = graph.function_instruction(
                &mut cursor,
                shape.clone(),
                I::Constant(execution::constant::ConstantId::new(index)),
            );
            DraftFlow::value(cursor, make(value))
        }),
        E::Reference(value) => target(value.instantiation(), context)
            .map(|target| reference(shape.clone(), target, cursor, graph).map(make)),
        E::Closure { function, captures } => target(function, context).and_then(|target| {
            closure(
                function,
                captures,
                shape.clone(),
                target,
                cursor,
                graph,
                context,
            )
            .map(|flow| flow.map(make))
        }),
        E::LocalGet { local, name: _ } => {
            let value = cursor
                .scope()
                .function(super::super::super::local::LocalKey::new(
                    super::super::super::local::LocalKind::GenericFunction,
                    local.id().0,
                ));
            Representability::Inhabited(DraftFlow::value(cursor, make(value)))
        }
        E::Call {
            function,
            args,
            site,
        } => call_args(args, cursor, graph, context).and_then(|flow| {
            flow.and_then(|mut cursor, args| {
                direct(function, context).map(|function| {
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::Call {
                            function,
                            args,
                            site: site.clone(),
                        },
                    );
                    DraftFlow::value(cursor, make(value))
                })
            })
        }),
        E::FunctionCall {
            function,
            args,
            site,
        } => lower_function_call(
            args,
            cursor,
            graph,
            context,
            |cursor, graph, context| function_function_expr(function, cursor, graph, context),
            |cursor, graph, context| {
                super::evaluated_function_function_expr(function, cursor, graph, context)
            },
            |mut cursor, function, args, graph, _| {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::FunctionCall {
                        function: function.value().clone(),
                        args,
                        site: site.clone(),
                    },
                );
                DraftFlow::value(cursor, make(value))
            },
        ),
        E::TupleIndex {
            tuple: source,
            index,
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| {
            flow.map_cursor(|cursor, tuple| {
                let value = graph.function_instruction(
                    cursor,
                    shape.clone(),
                    I::TupleIndex {
                        tuple,
                        index: *index,
                    },
                );
                make(value)
            })
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| {
                flow.map_cursor(|cursor, source| {
                    let value = graph.function_instruction(
                        cursor,
                        shape.clone(),
                        I::CustomField {
                            source,
                            index: access.index(),
                        },
                    );
                    make(value)
                })
            })
        }
        E::ListIndex {
            list: source,
            index,
        } => list::function_list_expr(source, cursor, graph, context).map(|flow| {
            flow.map_cursor(|cursor, list| {
                let value = graph.function_instruction(
                    cursor,
                    shape.clone(),
                    I::ListIndex {
                        list: list.value().clone(),
                        index: *index,
                    },
                );
                make(value)
            })
        }),
        E::Panic(value) => source_stop(value, cursor, graph, context).map(|flow| flow.map(make)),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case(
            subject,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |cursor, graph, context| branch(true_.kind(), cursor, graph, context),
            |cursor, graph, context| branch(false_.kind(), cursor, graph, context),
            |value| make(DraftFunction::from_ref(value)),
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => super::super::int_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch_kind, cursor, graph, context| {
                branch(branch_kind.kind(), cursor, graph, context)
            },
            |value| make(DraftFunction::from_ref(value)),
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => super::super::string_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch_kind, cursor, graph, context| {
                branch(branch_kind.kind(), cursor, graph, context)
            },
            |value| make(DraftFunction::from_ref(value)),
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::super::float_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch_kind, cursor, graph, context| {
                branch(branch_kind.kind(), cursor, graph, context)
            },
            |value| make(DraftFunction::from_ref(value)),
        ),
        E::Block { steps, return_ } => {
            super::super::super::step::steps(steps, cursor, graph, context).and_then(|flow| {
                flow.and_then(|cursor, ()| branch(return_.kind(), cursor, graph, context))
            })
        }
    }
}

macro_rules! generic_primitive_function {
    (
        $name:ident,
        $value:ty,
        $local_kind:ident,
        $constant:ident,
        $function_id:ident,
        $target:ident,
        $function_function_id:ident
    ) => {
        pub(in crate::plan::execution::lowering) fn $name(
            expression: &module::GenericFunctionExpr,
            shape: &SpecializedFunctionShape,
            cursor: DraftCursor,
            graph: &mut DraftGraph,
            context: &mut super::super::super::LoweringContext,
        ) -> Representability<DraftFlow<$value>> {
            fn lower_kind(
                kind: &module::GenericFunctionExprKind,
                shape: &SpecializedFunctionShape,
                cursor: DraftCursor,
                graph: &mut DraftGraph,
                context: &mut super::super::super::LoweringContext,
            ) -> Representability<DraftFlow<$value>> {
                lower_executable_kind(
                    kind,
                    shape,
                    cursor,
                    graph,
                    context,
                    executable_kind_lowering(
                        <$value>::new,
                        |value, context| context.$constant(value, shape).map(|id| id.index()),
                        |function, context| {
                            context.$function_id(function).map(FunctionTarget::$target)
                        },
                        |function, context| {
                            context
                                .$function_function_id(function)
                                .map(execution::function::FunctionFunctionId::$target)
                        },
                        |branch, cursor, graph, context| {
                            lower_kind(branch, shape, cursor, graph, context)
                        },
                    ),
                )
            }

            lower_kind(expression.kind(), shape, cursor, graph, context)
        }
    };
}

generic_primitive_function!(
    generic_int_function_expr,
    DraftIntFunction,
    IntFunction,
    generic_int_function_constant,
    int_function_id,
    Int,
    int_function_function_id
);
generic_primitive_function!(
    generic_float_function_expr,
    DraftFloatFunction,
    FloatFunction,
    generic_float_function_constant,
    float_function_id,
    Float,
    float_function_function_id
);
generic_primitive_function!(
    generic_string_function_expr,
    DraftStringFunction,
    StringFunction,
    generic_string_function_constant,
    string_function_id,
    String,
    string_function_function_id
);
generic_primitive_function!(
    generic_bit_array_function_expr,
    DraftBitArrayFunction,
    BitArrayFunction,
    generic_bit_array_function_constant,
    bit_array_function_id,
    BitArray,
    bit_array_function_function_id
);
generic_primitive_function!(
    generic_utf_codepoint_function_expr,
    DraftUtfCodepointFunction,
    UtfCodepointFunction,
    generic_utf_codepoint_function_constant,
    utf_codepoint_function_id,
    UtfCodepoint,
    utf_codepoint_function_function_id
);
generic_primitive_function!(
    generic_bool_function_expr,
    DraftBoolFunction,
    BoolFunction,
    generic_bool_function_constant,
    bool_function_id,
    Bool,
    bool_function_function_id
);
generic_primitive_function!(
    generic_nil_function_expr,
    DraftNilFunction,
    NilFunction,
    generic_nil_function_constant,
    nil_function_id,
    Nil,
    nil_function_function_id
);
generic_primitive_function!(
    generic_tuple_function_expr,
    DraftTupleFunction,
    TupleFunction,
    generic_tuple_function_constant,
    tuple_function_id,
    Tuple,
    tuple_function_function_id
);

macro_rules! define_symbolic_fixed_function {
    ($name:ident, $module_kind:ident, $constant:ident, $local_kind:ident) => {
        fn $name(
            kind: &module::$module_kind,
            shape: &SpecializedFunctionShape,
            cursor: DraftCursor,
            graph: &mut DraftGraph,
            context: &mut super::super::super::LoweringContext,
        ) -> Representability<DraftFlow<DraftGenericFunction>> {
            use super::super::super::instruction::DraftFunctionInstruction as I;
            use module::$module_kind as E;

            let stored = StoredValueShape::Function(Box::new(shape.clone()));
            match kind {
                E::Constant(value) => context.$constant(value, shape).map(|id| {
                    let mut cursor = cursor;
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::Constant(execution::constant::ConstantId::new(id.index())),
                    );
                    DraftFlow::value(cursor, DraftGenericFunction::new(value))
                }),
                E::Reference(value) => {
                    let target =
                        FunctionTarget::Generic(context.generic_callable_id(value.instantiation()));
                    Representability::Inhabited(
                        reference(shape.clone(), target, cursor, graph)
                            .map(DraftGenericFunction::new),
                    )
                }
                E::Closure {
                    function, captures, ..
                } => {
                    let target = FunctionTarget::Generic(context.generic_callable_id(function));
                    closure(
                        function,
                        captures,
                        shape.clone(),
                        target,
                        cursor,
                        graph,
                        context,
                    )
                    .map(|flow| flow.map(DraftGenericFunction::new))
                }
                E::LocalGet { local, name: _ } => {
                    let value = cursor
                        .scope()
                        .function(super::super::super::local::LocalKey::new(
                            super::super::super::local::LocalKind::$local_kind,
                            local.0,
                        ));
                    Representability::Inhabited(DraftFlow::value(
                        cursor,
                        DraftGenericFunction::new(value),
                    ))
                }
                E::Call {
                    function,
                    args,
                    site,
                    ..
                } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value {
                        mut cursor,
                        value: args,
                    } => {
                        let type_ = context.generic_function_type(shape);
                        context
                            .generic_function_function_id(function, type_)
                            .map(|function| {
                                let function =
                                    execution::function::FunctionFunctionId::Generic(function);
                                let value = graph.function_instruction(
                                    &mut cursor,
                                    shape.clone(),
                                    I::Call {
                                        function,
                                        args,
                                        site: site.clone(),
                                    },
                                );
                                DraftFlow::value(cursor, DraftGenericFunction::new(value))
                            })
                    }
                }),
                E::FunctionCall {
                    function,
                    args,
                    site,
                    ..
                } => lower_function_call(
                    args,
                    cursor,
                    graph,
                    context,
                    |cursor, graph, context| {
                        function_function_expr(function, cursor, graph, context)
                    },
                    |cursor, graph, context| {
                        super::evaluated_function_function_expr(function, cursor, graph, context)
                    },
                    |mut cursor, function, args, graph, _| {
                        let value = graph.function_instruction(
                            &mut cursor,
                            shape.clone(),
                            I::FunctionCall {
                                function: function.value().clone(),
                                args,
                                site: site.clone(),
                            },
                        );
                        DraftFlow::value(cursor, DraftGenericFunction::new(value))
                    },
                ),
                E::TupleIndex {
                    tuple: source,
                    index,
                    ..
                } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| match flow {
                    DraftFlow::Diverged => DraftFlow::Diverged,
                    DraftFlow::Value {
                        mut cursor,
                        value: tuple,
                    } => {
                        let value = graph.function_instruction(
                            &mut cursor,
                            shape.clone(),
                            I::TupleIndex {
                                tuple,
                                index: *index,
                            },
                        );
                        DraftFlow::value(cursor, DraftGenericFunction::new(value))
                    }
                }),
                E::CustomField(access) => {
                    custom::custom_expr(access.source(), cursor, graph, context).map(|flow| {
                        flow.map_cursor(|cursor, source| {
                            let value = graph.function_instruction(
                                cursor,
                                shape.clone(),
                                I::CustomField {
                                    source,
                                    index: access.index(),
                                },
                            );
                            DraftGenericFunction::new(value)
                        })
                    })
                }
                E::ListIndex {
                    list: source,
                    index,
                    ..
                } => list::function_list_expr(source, cursor, graph, context).map(|flow| {
                    flow.map_cursor(|cursor, list| {
                        let value = graph.function_instruction(
                            cursor,
                            shape.clone(),
                            I::ListIndex {
                                list: list.value().clone(),
                                index: *index,
                            },
                        );
                        DraftGenericFunction::new(value)
                    })
                }),
                E::Panic(value) => source_stop(value, cursor, graph, context)
                    .map(|flow| flow.map(DraftGenericFunction::new)),
                E::BoolCase {
                    subject,
                    true_,
                    false_,
                } => super::super::bool_case(
                    subject,
                    cursor,
                    super::super::case_lowering(graph, context, stored),
                    |cursor, graph, context| $name(true_.kind(), shape, cursor, graph, context),
                    |cursor, graph, context| $name(false_.kind(), shape, cursor, graph, context),
                    DraftGenericFunction::from_ref,
                ),
                E::IntCase {
                    subject,
                    clauses,
                    fallback,
                } => super::super::int_case(
                    subject,
                    clauses,
                    fallback,
                    cursor,
                    super::super::case_lowering(graph, context, stored),
                    |branch, cursor, graph, context| {
                        $name(branch.kind(), shape, cursor, graph, context)
                    },
                    DraftGenericFunction::from_ref,
                ),
                E::StringCase {
                    subject,
                    clauses,
                    fallback,
                } => super::super::string_case(
                    subject,
                    clauses,
                    fallback,
                    cursor,
                    super::super::case_lowering(graph, context, stored),
                    |branch, cursor, graph, context| {
                        $name(branch.kind(), shape, cursor, graph, context)
                    },
                    DraftGenericFunction::from_ref,
                ),
                E::FloatCase {
                    subject,
                    clauses,
                    fallback,
                } => super::super::float_case(
                    subject,
                    clauses,
                    fallback,
                    cursor,
                    super::super::case_lowering(graph, context, stored),
                    |branch, cursor, graph, context| {
                        $name(branch.kind(), shape, cursor, graph, context)
                    },
                    DraftGenericFunction::from_ref,
                ),
                E::Block { steps, return_ } => super::super::super::step::steps(
                    steps, cursor, graph, context,
                )
                .and_then(|flow| match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value { cursor, value: () } => {
                        $name(return_.kind(), shape, cursor, graph, context)
                    }
                }),
            }
        }
    };
}

define_symbolic_fixed_function!(
    symbolic_int_kind,
    IntFunctionExprKind,
    symbolic_int_function_constant,
    IntFunction
);
define_symbolic_fixed_function!(
    symbolic_float_kind,
    FloatFunctionExprKind,
    symbolic_float_function_constant,
    FloatFunction
);
define_symbolic_fixed_function!(
    symbolic_string_kind,
    StringFunctionExprKind,
    symbolic_string_function_constant,
    StringFunction
);
define_symbolic_fixed_function!(
    symbolic_bit_array_kind,
    BitArrayFunctionExprKind,
    symbolic_bit_array_function_constant,
    BitArrayFunction
);
define_symbolic_fixed_function!(
    symbolic_utf_codepoint_kind,
    UtfCodepointFunctionExprKind,
    symbolic_utf_codepoint_function_constant,
    UtfCodepointFunction
);
define_symbolic_fixed_function!(
    symbolic_bool_kind,
    BoolFunctionExprKind,
    symbolic_bool_function_constant,
    BoolFunction
);
define_symbolic_fixed_function!(
    symbolic_nil_kind,
    NilFunctionExprKind,
    symbolic_nil_function_constant,
    NilFunction
);
define_symbolic_fixed_function!(
    symbolic_tuple_kind,
    TupleFunctionExprKind,
    symbolic_tuple_function_constant,
    TupleFunction
);

fn symbolic_custom_kind(
    kind: &module::CustomFunctionExprKind,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    use super::super::super::instruction::DraftFunctionInstruction as I;
    use module::CustomFunctionExprKind as E;

    let stored = StoredValueShape::Function(Box::new(shape.clone()));
    match kind {
        E::Constant(value) => context
            .symbolic_custom_function_constant(value, shape)
            .map(|id| {
                let mut cursor = cursor;
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::Constant(execution::constant::ConstantId::new(id.index())),
                );
                DraftFlow::value(cursor, DraftGenericFunction::new(value))
            }),
        E::Constructor(constructor) => {
            let mut cursor = cursor;
            let value = graph.function_instruction(
                &mut cursor,
                shape.clone(),
                I::Closure {
                    target: FunctionTarget::Generic(
                        context.generic_constructor_callable_id(constructor.clone()),
                    ),
                    captures: Vec::new(),
                },
            );
            Representability::Inhabited(DraftFlow::value(cursor, DraftGenericFunction::new(value)))
        }
        E::Reference(value) => {
            let target =
                FunctionTarget::Generic(context.generic_callable_id(value.instantiation()));
            Representability::Inhabited(
                reference(shape.clone(), target, cursor, graph).map(DraftGenericFunction::new),
            )
        }
        E::Closure { function, captures } => {
            let target = FunctionTarget::Generic(context.generic_callable_id(function));
            closure(
                function,
                captures,
                shape.clone(),
                target,
                cursor,
                graph,
                context,
            )
            .map(|flow| flow.map(DraftGenericFunction::new))
        }
        E::LocalGet { local, name: _ } => {
            let value = cursor
                .scope()
                .function(super::super::super::local::LocalKey::new(
                    super::super::super::local::LocalKind::CustomFunction,
                    local.id().0,
                ));
            Representability::Inhabited(DraftFlow::value(cursor, DraftGenericFunction::new(value)))
        }
        E::Call {
            function,
            args,
            site,
        } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value {
                mut cursor,
                value: args,
            } => {
                let type_ = context.generic_function_type(shape);
                context
                    .generic_function_function_id(function, type_)
                    .map(|function| {
                        let value = graph.function_instruction(
                            &mut cursor,
                            shape.clone(),
                            I::Call {
                                function: execution::function::FunctionFunctionId::Generic(
                                    function,
                                ),
                                args,
                                site: site.clone(),
                            },
                        );
                        DraftFlow::value(cursor, DraftGenericFunction::new(value))
                    })
            }
        }),
        E::FunctionCall {
            function,
            args,
            site,
        } => lower_function_call(
            args,
            cursor,
            graph,
            context,
            |cursor, graph, context| function_function_expr(function, cursor, graph, context),
            |cursor, graph, context| {
                super::evaluated_function_function_expr(function, cursor, graph, context)
            },
            |mut cursor, function, args, graph, _| {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::FunctionCall {
                        function: function.value().clone(),
                        args,
                        site: site.clone(),
                    },
                );
                DraftFlow::value(cursor, DraftGenericFunction::new(value))
            },
        ),
        E::TupleIndex {
            tuple: source,
            index,
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: tuple,
            } => {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::TupleIndex {
                        tuple,
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, DraftGenericFunction::new(value))
            }
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: source,
                } => {
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::CustomField {
                            source,
                            index: access.index(),
                        },
                    );
                    DraftFlow::value(cursor, DraftGenericFunction::new(value))
                }
            })
        }
        E::ListIndex {
            list: source,
            index,
        } => list::function_list_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: list,
            } => {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::ListIndex {
                        list: list.value().clone(),
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, DraftGenericFunction::new(value))
            }
        }),
        E::Panic(value) => source_stop(value, cursor, graph, context)
            .map(|flow| flow.map(DraftGenericFunction::new)),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case(
            subject,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |cursor, graph, context| symbolic_custom_kind(true_, shape, cursor, graph, context),
            |cursor, graph, context| symbolic_custom_kind(false_, shape, cursor, graph, context),
            DraftGenericFunction::from_ref,
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => super::super::int_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch, cursor, graph, context| {
                symbolic_custom_kind(branch, shape, cursor, graph, context)
            },
            DraftGenericFunction::from_ref,
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => super::super::string_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch, cursor, graph, context| {
                symbolic_custom_kind(branch, shape, cursor, graph, context)
            },
            DraftGenericFunction::from_ref,
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::super::float_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch, cursor, graph, context| {
                symbolic_custom_kind(branch, shape, cursor, graph, context)
            },
            DraftGenericFunction::from_ref,
        ),
        E::Block { steps, return_ } => {
            super::super::super::step::steps(steps, cursor, graph, context).and_then(|flow| {
                match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value { cursor, value: () } => {
                        symbolic_custom_kind(return_, shape, cursor, graph, context)
                    }
                }
            })
        }
    }
}

fn symbolic_list_kind(
    kind: &module::ListFunctionExprKind,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    use super::super::super::instruction::DraftFunctionInstruction as I;
    use module::ListFunctionExprKind as E;

    let stored = StoredValueShape::Function(Box::new(shape.clone()));
    match kind {
        E::Constant(value) => context
            .symbolic_list_function_constant(value, shape)
            .map(|id| {
                let mut cursor = cursor;
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::Constant(execution::constant::ConstantId::new(id.index())),
                );
                DraftFlow::value(cursor, DraftGenericFunction::new(value))
            }),
        E::Reference(value) => {
            let target =
                FunctionTarget::Generic(context.generic_callable_id(value.instantiation()));
            Representability::Inhabited(
                reference(shape.clone(), target, cursor, graph).map(DraftGenericFunction::new),
            )
        }
        E::Closure { function, captures } => {
            let target = FunctionTarget::Generic(context.generic_callable_id(function));
            closure(
                function,
                captures,
                shape.clone(),
                target,
                cursor,
                graph,
                context,
            )
            .map(|flow| flow.map(DraftGenericFunction::new))
        }
        E::LocalGet { local, name: _ } => {
            let value = cursor
                .scope()
                .function(super::super::super::local::list_function_local_key(local));
            Representability::Inhabited(DraftFlow::value(cursor, DraftGenericFunction::new(value)))
        }
        E::Call {
            function,
            args,
            site,
            ..
        } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value {
                mut cursor,
                value: args,
            } => {
                let type_ = context.generic_function_type(shape);
                context
                    .generic_function_function_id(function, type_)
                    .map(|function| {
                        let value = graph.function_instruction(
                            &mut cursor,
                            shape.clone(),
                            I::Call {
                                function: execution::function::FunctionFunctionId::Generic(
                                    function,
                                ),
                                args,
                                site: site.clone(),
                            },
                        );
                        DraftFlow::value(cursor, DraftGenericFunction::new(value))
                    })
            }
        }),
        E::FunctionCall {
            function,
            args,
            site,
            ..
        } => lower_function_call(
            args,
            cursor,
            graph,
            context,
            |cursor, graph, context| function_function_expr(function, cursor, graph, context),
            |cursor, graph, context| {
                super::evaluated_function_function_expr(function, cursor, graph, context)
            },
            |mut cursor, function, args, graph, _| {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::FunctionCall {
                        function: function.value().clone(),
                        args,
                        site: site.clone(),
                    },
                );
                DraftFlow::value(cursor, DraftGenericFunction::new(value))
            },
        ),
        E::TupleIndex {
            tuple: source,
            index,
            ..
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: tuple,
            } => {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::TupleIndex {
                        tuple,
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, DraftGenericFunction::new(value))
            }
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: source,
                } => {
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::CustomField {
                            source,
                            index: access.index(),
                        },
                    );
                    DraftFlow::value(cursor, DraftGenericFunction::new(value))
                }
            })
        }
        E::ListIndex {
            list: source,
            index,
            ..
        } => list::function_list_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: list,
            } => {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::ListIndex {
                        list: list.value().clone(),
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, DraftGenericFunction::new(value))
            }
        }),
        E::Panic(value) => source_stop(value, cursor, graph, context)
            .map(|flow| flow.map(DraftGenericFunction::new)),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case(
            subject,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |cursor, graph, context| {
                symbolic_list_kind(true_.kind(), shape, cursor, graph, context)
            },
            |cursor, graph, context| {
                symbolic_list_kind(false_.kind(), shape, cursor, graph, context)
            },
            DraftGenericFunction::from_ref,
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => super::super::int_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch, cursor, graph, context| {
                symbolic_list_kind(branch.kind(), shape, cursor, graph, context)
            },
            DraftGenericFunction::from_ref,
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => super::super::string_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch, cursor, graph, context| {
                symbolic_list_kind(branch.kind(), shape, cursor, graph, context)
            },
            DraftGenericFunction::from_ref,
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::super::float_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch, cursor, graph, context| {
                symbolic_list_kind(branch.kind(), shape, cursor, graph, context)
            },
            DraftGenericFunction::from_ref,
        ),
        E::Block { steps, return_ } => {
            super::super::super::step::steps(steps, cursor, graph, context).and_then(|flow| {
                match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value { cursor, value: () } => {
                        symbolic_list_kind(return_.kind(), shape, cursor, graph, context)
                    }
                }
            })
        }
    }
}

fn symbolic_returning_function_kind(
    kind: &module::FunctionFunctionExprKind,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    use super::super::super::instruction::DraftFunctionInstruction as I;
    use module::FunctionFunctionExprKind as E;

    let stored = StoredValueShape::Function(Box::new(shape.clone()));
    match kind {
        E::Constant(value) => context
            .symbolic_function_function_constant(value, shape)
            .map(|id| {
                let mut cursor = cursor;
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::Constant(execution::constant::ConstantId::new(id.index())),
                );
                DraftFlow::value(cursor, DraftGenericFunction::new(value))
            }),
        E::Reference(value) => {
            let target =
                FunctionTarget::Generic(context.generic_callable_id(value.instantiation()));
            Representability::Inhabited(
                reference(shape.clone(), target, cursor, graph).map(DraftGenericFunction::new),
            )
        }
        E::Closure { function, captures } => {
            let target = FunctionTarget::Generic(context.generic_callable_id(function));
            closure(
                function,
                captures,
                shape.clone(),
                target,
                cursor,
                graph,
                context,
            )
            .map(|flow| flow.map(DraftGenericFunction::new))
        }
        E::LocalGet { local, name: _ } => {
            let value = cursor
                .scope()
                .function(super::super::super::local::LocalKey::new(
                    super::super::super::local::LocalKind::FunctionFunction,
                    local.id().0,
                ));
            Representability::Inhabited(DraftFlow::value(cursor, DraftGenericFunction::new(value)))
        }
        E::Call {
            function,
            args,
            site,
        } => call_args(args, cursor, graph, context).and_then(|flow| match flow {
            DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
            DraftFlow::Value {
                mut cursor,
                value: args,
            } => {
                let type_ = context.generic_function_type(shape);
                context
                    .generic_function_function_id(function, type_)
                    .map(|function| {
                        let value = graph.function_instruction(
                            &mut cursor,
                            shape.clone(),
                            I::Call {
                                function: execution::function::FunctionFunctionId::Generic(
                                    function,
                                ),
                                args,
                                site: site.clone(),
                            },
                        );
                        DraftFlow::value(cursor, DraftGenericFunction::new(value))
                    })
            }
        }),
        E::FunctionCall {
            function,
            args,
            site,
        } => lower_function_call(
            args,
            cursor,
            graph,
            context,
            |cursor, graph, context| function_function_expr(function, cursor, graph, context),
            |cursor, graph, context| {
                super::evaluated_function_function_expr(function, cursor, graph, context)
            },
            |mut cursor, function, args, graph, _| {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::FunctionCall {
                        function: function.value().clone(),
                        args,
                        site: site.clone(),
                    },
                );
                DraftFlow::value(cursor, DraftGenericFunction::new(value))
            },
        ),
        E::TupleIndex {
            tuple: source,
            index,
        } => tuple::tuple_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: tuple,
            } => {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::TupleIndex {
                        tuple,
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, DraftGenericFunction::new(value))
            }
        }),
        E::CustomField(access) => {
            custom::custom_expr(access.source(), cursor, graph, context).map(|flow| match flow {
                DraftFlow::Diverged => DraftFlow::Diverged,
                DraftFlow::Value {
                    mut cursor,
                    value: source,
                } => {
                    let value = graph.function_instruction(
                        &mut cursor,
                        shape.clone(),
                        I::CustomField {
                            source,
                            index: access.index(),
                        },
                    );
                    DraftFlow::value(cursor, DraftGenericFunction::new(value))
                }
            })
        }
        E::ListIndex {
            list: source,
            index,
        } => list::function_list_expr(source, cursor, graph, context).map(|flow| match flow {
            DraftFlow::Diverged => DraftFlow::Diverged,
            DraftFlow::Value {
                mut cursor,
                value: list,
            } => {
                let value = graph.function_instruction(
                    &mut cursor,
                    shape.clone(),
                    I::ListIndex {
                        list: list.value().clone(),
                        index: *index,
                    },
                );
                DraftFlow::value(cursor, DraftGenericFunction::new(value))
            }
        }),
        E::Panic(value) => source_stop(value, cursor, graph, context)
            .map(|flow| flow.map(DraftGenericFunction::new)),
        E::BoolCase {
            subject,
            true_,
            false_,
        } => super::super::bool_case(
            subject,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |cursor, graph, context| {
                symbolic_returning_function_kind(true_, shape, cursor, graph, context)
            },
            |cursor, graph, context| {
                symbolic_returning_function_kind(false_, shape, cursor, graph, context)
            },
            DraftGenericFunction::from_ref,
        ),
        E::IntCase {
            subject,
            clauses,
            fallback,
        } => super::super::int_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch, cursor, graph, context| {
                symbolic_returning_function_kind(branch, shape, cursor, graph, context)
            },
            DraftGenericFunction::from_ref,
        ),
        E::StringCase {
            subject,
            clauses,
            fallback,
        } => super::super::string_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch, cursor, graph, context| {
                symbolic_returning_function_kind(branch, shape, cursor, graph, context)
            },
            DraftGenericFunction::from_ref,
        ),
        E::FloatCase {
            subject,
            clauses,
            fallback,
        } => super::super::float_case(
            subject,
            clauses,
            fallback,
            cursor,
            super::super::case_lowering(graph, context, stored),
            |branch, cursor, graph, context| {
                symbolic_returning_function_kind(branch, shape, cursor, graph, context)
            },
            DraftGenericFunction::from_ref,
        ),
        E::Block { steps, return_ } => {
            super::super::super::step::steps(steps, cursor, graph, context).and_then(|flow| {
                match flow {
                    DraftFlow::Diverged => Representability::Inhabited(DraftFlow::Diverged),
                    DraftFlow::Value { cursor, value: () } => {
                        symbolic_returning_function_kind(return_, shape, cursor, graph, context)
                    }
                }
            })
        }
    }
}

pub(super) fn symbolic_function_expr(
    expression: &module::FunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunction>> {
    symbolic_kind_from_function(expression, shape, cursor, graph, context)
        .map(|flow| flow.map(|value| value.value().clone()))
}

pub(in crate::plan::execution::lowering) fn symbolic_int_function_expr(
    expression: &module::IntFunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    symbolic_int_kind(expression.kind(), shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn symbolic_float_function_expr(
    expression: &module::FloatFunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    symbolic_float_kind(expression.kind(), shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn symbolic_string_function_expr(
    expression: &module::StringFunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    symbolic_string_kind(expression.kind(), shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn symbolic_bit_array_function_expr(
    expression: &module::BitArrayFunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    symbolic_bit_array_kind(expression.kind(), shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn symbolic_utf_codepoint_function_expr(
    expression: &module::UtfCodepointFunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    symbolic_utf_codepoint_kind(expression.kind(), shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn symbolic_custom_function_expr_kind(
    kind: &module::CustomFunctionExprKind,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    symbolic_custom_kind(kind, shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn symbolic_bool_function_expr(
    expression: &module::BoolFunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    symbolic_bool_kind(expression.kind(), shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn symbolic_nil_function_expr(
    expression: &module::NilFunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    symbolic_nil_kind(expression.kind(), shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn symbolic_tuple_function_expr(
    expression: &module::TupleFunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    symbolic_tuple_kind(expression.kind(), shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn symbolic_list_function_expr(
    expression: &module::ListFunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    symbolic_list_kind(expression.kind(), shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn symbolic_function_function_expr_kind(
    kind: &module::FunctionFunctionExprKind,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    symbolic_returning_function_kind(kind, shape, cursor, graph, context)
}

pub(in crate::plan::execution::lowering) fn symbolic_generic_function_expr(
    expression: &module::GenericFunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunction>> {
    symbolic_generic_kind(expression.kind(), shape, cursor, graph, context)
        .map(|flow| flow.map(|value| value.value().clone()))
}

fn symbolic_kind_from_function(
    expression: &module::FunctionExpr,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    match expression.kind() {
        module::FunctionExprKind::Generic(value) => {
            symbolic_generic_kind(value.kind(), shape, cursor, graph, context)
        }
        module::FunctionExprKind::Int(value) => {
            symbolic_int_kind(value.kind(), shape, cursor, graph, context)
        }
        module::FunctionExprKind::Float(value) => {
            symbolic_float_kind(value.kind(), shape, cursor, graph, context)
        }
        module::FunctionExprKind::String(value) => {
            symbolic_string_kind(value.kind(), shape, cursor, graph, context)
        }
        module::FunctionExprKind::BitArray(value) => {
            symbolic_bit_array_kind(value.kind(), shape, cursor, graph, context)
        }
        module::FunctionExprKind::UtfCodepoint(value) => {
            symbolic_utf_codepoint_kind(value.kind(), shape, cursor, graph, context)
        }
        module::FunctionExprKind::Custom(value) => {
            symbolic_custom_kind(value.kind(), shape, cursor, graph, context)
        }
        module::FunctionExprKind::Bool(value) => {
            symbolic_bool_kind(value.kind(), shape, cursor, graph, context)
        }
        module::FunctionExprKind::Nil(value) => {
            symbolic_nil_kind(value.kind(), shape, cursor, graph, context)
        }
        module::FunctionExprKind::Tuple(value) => {
            symbolic_tuple_kind(value.kind(), shape, cursor, graph, context)
        }
        module::FunctionExprKind::List(value) => {
            symbolic_list_kind(value.kind(), shape, cursor, graph, context)
        }
        module::FunctionExprKind::Function(value) => {
            symbolic_returning_function_kind(value.kind(), shape, cursor, graph, context)
        }
    }
}

fn symbolic_generic_kind(
    kind: &module::GenericFunctionExprKind,
    shape: &SpecializedFunctionShape,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftGenericFunction>> {
    lower_executable_kind(
        kind,
        shape,
        cursor,
        graph,
        context,
        executable_kind_lowering(
            DraftGenericFunction::new,
            |value, context| {
                context
                    .generic_function_constant(value, shape)
                    .map(|id| id.index())
            },
            |function, context| {
                Representability::Inhabited(FunctionTarget::Generic(
                    context.generic_callable_id(function),
                ))
            },
            |function, context| {
                let type_ = context.generic_function_type(shape);
                context
                    .generic_function_function_id(function, type_)
                    .map(execution::function::FunctionFunctionId::Generic)
            },
            |branch, cursor, graph, context| {
                symbolic_generic_kind(branch, shape, cursor, graph, context)
            },
        ),
    )
}

#[cfg(test)]
mod tests {
    use super::{
        generic_int_function_expr, symbolic_bit_array_kind, symbolic_bool_kind,
        symbolic_custom_kind, symbolic_float_kind, symbolic_int_kind, symbolic_list_kind,
        symbolic_nil_kind, symbolic_returning_function_kind, symbolic_string_kind,
        symbolic_tuple_kind, symbolic_utf_codepoint_kind,
    };
    use crate::plan::execution::lowering::graph::draft::DraftGraphBuilder;
    use crate::plan::execution::lowering::graph::{
        DraftFlow, DraftGenericFunction, DraftGraph, DraftValueRef,
    };
    use crate::plan::execution::lowering::specialization::{
        Representability, SpecializedFunctionShape,
    };
    use crate::plan::{
        self, BitArrayFunctionExpr, BoolFunctionExpr, CustomConstructorRefinement,
        CustomFieldAccess, CustomFunctionExpr, CustomFunctionType, CustomType,
        CustomTypeDefinition, CustomTypeName, CustomTypeParameterId, CustomTypePublicity,
        CustomTypeTemplate, CustomValueShape, FloatFunctionExpr, FunctionFunctionExpr,
        FunctionFunctionType, FunctionShape, FunctionType, GenericFunctionExpr,
        GenericFunctionType, IntFunctionExpr, ListExpr, ListFunctionExpr, NilFunctionExpr,
        PanicExpr, PanicSite, StringFunctionExpr, TupleFunctionExpr, TypeParameterId,
        UtfCodepointFunctionExpr, ValueShape, ValueType,
    };

    #[test]
    fn symbolic_callable_handoffs_execute_every_return_family() {
        let source = include_str!(
            "../../../../../../../../tests/fixtures/execution/functions/generic_symbolic_handoffs.gleam"
        );
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("symbolic callable fixture should compile");
        let module = crate::plan_module(typed).expect("symbolic callable fixture should plan");
        let execution = crate::ExecutionPlan::from_module_plan(module);

        assert_eq!(
            crate::run_main(&execution, &mut Vec::new()),
            Ok(crate::Value::Tuple(vec![crate::Value::Bool(true); 48])),
        );
    }

    #[test]
    fn symbolic_callable_projections_stop_with_their_custom_or_list_source() {
        let holder_name = CustomTypeName::new("geam".into(), "main".into(), "Holder".into());
        let marker_name = CustomTypeName::new("geam".into(), "main".into(), "Marker".into());
        let holder = CustomTypeDefinition::new(
            holder_name.clone(),
            CustomTypePublicity::Private,
            false,
            vec![CustomTypeParameterId(0)],
            vec![plan::CustomConstructorDefinition::new(
                "Holder".into(),
                0,
                vec![plan::CustomFieldDefinition::new(
                    Some("selected".into()),
                    CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                )],
            )],
        );
        let marker = CustomTypeDefinition::new(
            marker_name.clone(),
            CustomTypePublicity::Private,
            false,
            Vec::new(),
            vec![plan::CustomConstructorDefinition::new(
                "Marker".into(),
                0,
                Vec::new(),
            )],
        );
        let mut context =
            crate::plan::execution::lowering::test_support::lowering_context(vec![holder, marker]);
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let parameter = TypeParameterId(0);
        let generic_type = GenericFunctionType::new(Vec::new(), parameter);
        let marker_type = CustomType::new(marker_name.clone(), Vec::new());
        let custom_type = CustomFunctionType::new(Vec::new(), marker_type.clone());
        let list_type = FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int)));
        let returning_type =
            FunctionFunctionType::new(Vec::new(), FunctionType::new(Vec::new(), ValueType::Int));
        let target_types = [
            FunctionType::new(Vec::new(), ValueType::Int),
            custom_type.to_function_type(),
            list_type.clone(),
            returning_type.to_function_type(),
        ];

        let accesses = target_types.clone().map(|type_| {
            let shape = CustomValueShape::new(
                holder_name.clone(),
                vec![ValueShape::Function(Box::new(
                    FunctionShape::from_function_type(type_),
                ))],
                CustomConstructorRefinement::Exact(0),
            );
            CustomFieldAccess::new(plan::CustomExpr::panic_shape(panic(), shape), 0, None)
        });
        let lists = target_types.map(|type_| {
            ListExpr::panic(panic(), ValueType::Function(Box::new(type_)))
                .into_function()
                .expect("a function item type should create a function list")
        });
        let shapes = [
            FunctionShape::from_function_type(FunctionType::new(Vec::new(), ValueType::Int)),
            FunctionShape::from_function_type(custom_type.to_function_type()),
            FunctionShape::from_function_type(list_type.clone()),
            FunctionShape::from_function_type(returning_type.to_function_type()),
        ]
        .map(|shape| context.concrete_function_shape(&shape));
        let expressions = [
            GenericFunctionExpr::custom_field(accesses[0].clone(), generic_type.clone()),
            GenericFunctionExpr::list_index(lists[0].clone(), 0, generic_type),
        ];

        let (mut graph, _) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        for expression in expressions {
            let cursor = graph.empty_block(Default::default());
            assert_diverged(
                generic_int_function_expr(
                    &expression,
                    &shapes[0],
                    cursor,
                    &mut graph,
                    &mut context,
                )
                .map(|flow| flow.map(|_| ())),
            );
        }

        let custom_expressions = [
            CustomFunctionExpr::custom_field(accesses[1].clone(), custom_type.clone()),
            CustomFunctionExpr::list_index(lists[1].clone(), 0, custom_type.clone()),
        ];
        for expression in custom_expressions {
            let cursor = graph.empty_block(Default::default());
            assert_diverged(
                symbolic_custom_kind(
                    expression.kind(),
                    &shapes[1],
                    cursor,
                    &mut graph,
                    &mut context,
                )
                .map(|flow| flow.map(|_| ())),
            );
        }

        let list_expressions = [
            ListFunctionExpr::custom_field(accesses[2].clone(), list_type.clone(), ValueType::Int),
            ListFunctionExpr::list_index(lists[2].clone(), 0, list_type.clone(), ValueType::Int),
        ];
        for expression in list_expressions {
            let cursor = graph.empty_block(Default::default());
            assert_diverged(
                symbolic_list_kind(
                    expression.kind(),
                    &shapes[2],
                    cursor,
                    &mut graph,
                    &mut context,
                )
                .map(|flow| flow.map(|_| ())),
            );
        }

        let returning_expressions = [
            FunctionFunctionExpr::custom_field(accesses[3].clone(), returning_type.clone()),
            FunctionFunctionExpr::list_index(lists[3].clone(), 0, returning_type),
        ];
        for expression in returning_expressions {
            let cursor = graph.empty_block(Default::default());
            assert_diverged(
                symbolic_returning_function_kind(
                    expression.kind(),
                    &shapes[3],
                    cursor,
                    &mut graph,
                    &mut context,
                )
                .map(|flow| flow.map(|_| ())),
            );
        }

        let fixed_types = [
            FunctionType::new(Vec::new(), ValueType::Int),
            FunctionType::new(Vec::new(), ValueType::Float),
            FunctionType::new(Vec::new(), ValueType::String),
            FunctionType::new(Vec::new(), ValueType::BitArray),
            FunctionType::new(Vec::new(), ValueType::UtfCodepoint),
            FunctionType::new(Vec::new(), ValueType::Bool),
            FunctionType::new(Vec::new(), ValueType::Nil),
            FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
        ];
        let fixed_accesses = fixed_types.clone().map(|type_| {
            let shape = CustomValueShape::new(
                holder_name.clone(),
                vec![ValueShape::Function(Box::new(
                    FunctionShape::from_function_type(type_),
                ))],
                CustomConstructorRefinement::Exact(0),
            );
            CustomFieldAccess::new(plan::CustomExpr::panic_shape(panic(), shape), 0, None)
        });
        let fixed_lists = fixed_types.clone().map(|type_| {
            ListExpr::panic(panic(), ValueType::Function(Box::new(type_)))
                .into_function()
                .expect("a function item type should create a function list")
        });
        let fixed_shapes = fixed_types.clone().map(|type_| {
            context.concrete_function_shape(&FunctionShape::from_function_type(type_))
        });

        let int = [
            IntFunctionExpr::custom_field(fixed_accesses[0].clone(), fixed_types[0].clone()),
            IntFunctionExpr::list_index(fixed_lists[0].clone(), 0, fixed_types[0].clone()),
        ];
        assert_projection_sources_diverge(
            [&int[0], &int[1]],
            &fixed_shapes[0],
            &mut graph,
            &mut context,
            |expression, shape, cursor, graph, context| {
                symbolic_int_kind(expression.kind(), shape, cursor, graph, context)
            },
        );

        let float = [
            FloatFunctionExpr::custom_field(fixed_accesses[1].clone(), fixed_types[1].clone()),
            FloatFunctionExpr::list_index(fixed_lists[1].clone(), 0, fixed_types[1].clone()),
        ];
        assert_projection_sources_diverge(
            [&float[0], &float[1]],
            &fixed_shapes[1],
            &mut graph,
            &mut context,
            |expression, shape, cursor, graph, context| {
                symbolic_float_kind(expression.kind(), shape, cursor, graph, context)
            },
        );

        let string = [
            StringFunctionExpr::custom_field(fixed_accesses[2].clone(), fixed_types[2].clone()),
            StringFunctionExpr::list_index(fixed_lists[2].clone(), 0, fixed_types[2].clone()),
        ];
        assert_projection_sources_diverge(
            [&string[0], &string[1]],
            &fixed_shapes[2],
            &mut graph,
            &mut context,
            |expression, shape, cursor, graph, context| {
                symbolic_string_kind(expression.kind(), shape, cursor, graph, context)
            },
        );

        let bit_array = [
            BitArrayFunctionExpr::custom_field(fixed_accesses[3].clone(), fixed_types[3].clone()),
            BitArrayFunctionExpr::list_index(fixed_lists[3].clone(), 0, fixed_types[3].clone()),
        ];
        assert_projection_sources_diverge(
            [&bit_array[0], &bit_array[1]],
            &fixed_shapes[3],
            &mut graph,
            &mut context,
            |expression, shape, cursor, graph, context| {
                symbolic_bit_array_kind(expression.kind(), shape, cursor, graph, context)
            },
        );

        let utf_codepoint = [
            UtfCodepointFunctionExpr::custom_field(
                fixed_accesses[4].clone(),
                fixed_types[4].clone(),
            ),
            UtfCodepointFunctionExpr::list_index(fixed_lists[4].clone(), 0, fixed_types[4].clone()),
        ];
        assert_projection_sources_diverge(
            [&utf_codepoint[0], &utf_codepoint[1]],
            &fixed_shapes[4],
            &mut graph,
            &mut context,
            |expression, shape, cursor, graph, context| {
                symbolic_utf_codepoint_kind(expression.kind(), shape, cursor, graph, context)
            },
        );

        let bool_ = [
            BoolFunctionExpr::custom_field(fixed_accesses[5].clone(), fixed_types[5].clone()),
            BoolFunctionExpr::list_index(fixed_lists[5].clone(), 0, fixed_types[5].clone()),
        ];
        assert_projection_sources_diverge(
            [&bool_[0], &bool_[1]],
            &fixed_shapes[5],
            &mut graph,
            &mut context,
            |expression, shape, cursor, graph, context| {
                symbolic_bool_kind(expression.kind(), shape, cursor, graph, context)
            },
        );

        let nil = [
            NilFunctionExpr::custom_field(fixed_accesses[6].clone(), fixed_types[6].clone()),
            NilFunctionExpr::list_index(fixed_lists[6].clone(), 0, fixed_types[6].clone()),
        ];
        assert_projection_sources_diverge(
            [&nil[0], &nil[1]],
            &fixed_shapes[6],
            &mut graph,
            &mut context,
            |expression, shape, cursor, graph, context| {
                symbolic_nil_kind(expression.kind(), shape, cursor, graph, context)
            },
        );

        let tuple = [
            TupleFunctionExpr::custom_field(fixed_accesses[7].clone(), fixed_types[7].clone()),
            TupleFunctionExpr::list_index(fixed_lists[7].clone(), 0, fixed_types[7].clone()),
        ];
        assert_projection_sources_diverge(
            [&tuple[0], &tuple[1]],
            &fixed_shapes[7],
            &mut graph,
            &mut context,
            |expression, shape, cursor, graph, context| {
                symbolic_tuple_kind(expression.kind(), shape, cursor, graph, context)
            },
        );
    }

    fn assert_projection_sources_diverge<Expression>(
        expressions: [&Expression; 2],
        shape: &SpecializedFunctionShape,
        graph: &mut DraftGraph,
        context: &mut crate::plan::execution::lowering::LoweringContext,
        lower: impl Copy
        + Fn(
            &Expression,
            &SpecializedFunctionShape,
            crate::plan::execution::lowering::graph::DraftCursor,
            &mut DraftGraph,
            &mut crate::plan::execution::lowering::LoweringContext,
        ) -> Representability<DraftFlow<DraftGenericFunction>>,
    ) {
        for expression in expressions {
            let cursor = graph.empty_block(Default::default());
            assert_diverged(
                lower(expression, shape, cursor, graph, context).map(|flow| flow.map(|_| ())),
            );
        }
    }

    fn assert_diverged(flow: Representability<DraftFlow<()>>) {
        if let Representability::Inhabited(DraftFlow::Diverged) = flow {
            return;
        }
        panic!("expected the source projection to diverge");
    }

    #[test]
    #[should_panic(expected = "expected the source projection to diverge")]
    fn projection_divergence_assertion_rejects_an_uninhabited_result() {
        assert_diverged(Representability::Uninhabited);
    }
}
