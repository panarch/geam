use super::super::super as execution;
use super::{
    bit_array_expr, bool_expr, custom_expr, float_expr, function_expr, generic_bit_array_expr,
    generic_bool_expr, generic_float_expr, generic_int_expr, generic_nil_expr, generic_string_expr,
    generic_utf_codepoint_expr, int_expr, list_function_expr, panic_expr, string_expr, tuple_expr,
    utf_codepoint_expr,
};
use crate::plan::execution::lowering::LoweringContext;
use crate::plan::execution::lowering::specialization::{
    Representability, SpecializedValueShape, StoredValueShape,
};
use crate::plan::module;
use vec1::Vec1;

trait LowerListItem:
    module::ListItem<Function = module::FunctionInstantiation, IndexSource = module::ListListExpr>
{
    type Execution: execution::ListItem<IndexSource = execution::ListListExpr>;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution;
    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> Representability<<Self::Execution as execution::ListItem>::ElementExpr>;
    fn lower_constant(
        constant: &Self::Constant,
        context: &mut LoweringContext,
    ) -> Representability<<Self::Execution as execution::ListItem>::Constant>;
    fn lower_local(
        local: &Self::Local,
        _context: &mut LoweringContext,
    ) -> <Self::Execution as execution::ListItem>::Local;
    fn lower_function(
        &self,
        function: &Self::Function,
        item: &Self::Execution,
        _context: &mut LoweringContext,
    ) -> Representability<<Self::Execution as execution::ListItem>::Function>;
}

pub(in crate::plan::execution::lowering) fn list_expr(
    expression: &module::ListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::ListExpr> {
    match expression {
        module::ListExpr::Generic(expression) => generic_list_expr(expression, context),
        module::ListExpr::ParameterList(expression) => {
            parameter_list_list_facade(expression, context)
        }
        module::ListExpr::Int(expression) => {
            int_list_expr(expression, context).map(execution::ListExpr::Int)
        }
        module::ListExpr::String(expression) => {
            string_list_expr(expression, context).map(execution::ListExpr::String)
        }
        module::ListExpr::BitArray(expression) => {
            bit_array_list_expr(expression, context).map(execution::ListExpr::BitArray)
        }
        module::ListExpr::UtfCodepoint(expression) => {
            utf_codepoint_list_expr(expression, context).map(execution::ListExpr::UtfCodepoint)
        }
        module::ListExpr::Custom(expression) => {
            custom_list_expr(expression, context).map(execution::ListExpr::Custom)
        }
        module::ListExpr::Float(expression) => {
            float_list_expr(expression, context).map(execution::ListExpr::Float)
        }
        module::ListExpr::Bool(expression) => {
            bool_list_expr(expression, context).map(execution::ListExpr::Bool)
        }
        module::ListExpr::Nil(expression) => {
            nil_list_expr(expression, context).map(execution::ListExpr::Nil)
        }
        module::ListExpr::Tuple(expression) => {
            tuple_list_expr(expression, context).map(execution::ListExpr::Tuple)
        }
        module::ListExpr::List(expression) => {
            list_list_expr(expression, context).map(execution::ListExpr::List)
        }
        module::ListExpr::Function(expression) => {
            function_list_expr(expression, context).map(execution::ListExpr::Function)
        }
    }
}

fn stored_list_expr(
    expression: &module::StoredListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::StoredListExpr> {
    match expression {
        module::StoredListExpr::ParameterList(expression) => {
            parameter_list_list_expr(expression, context)
        }
        module::StoredListExpr::Int(expression) => {
            int_list_expr(expression, context).map(execution::StoredListExpr::Int)
        }
        module::StoredListExpr::String(expression) => {
            string_list_expr(expression, context).map(execution::StoredListExpr::String)
        }
        module::StoredListExpr::BitArray(expression) => {
            bit_array_list_expr(expression, context).map(execution::StoredListExpr::BitArray)
        }
        module::StoredListExpr::UtfCodepoint(expression) => {
            utf_codepoint_list_expr(expression, context)
                .map(execution::StoredListExpr::UtfCodepoint)
        }
        module::StoredListExpr::Custom(expression) => {
            custom_list_expr(expression, context).map(execution::StoredListExpr::Custom)
        }
        module::StoredListExpr::Float(expression) => {
            float_list_expr(expression, context).map(execution::StoredListExpr::Float)
        }
        module::StoredListExpr::Bool(expression) => {
            bool_list_expr(expression, context).map(execution::StoredListExpr::Bool)
        }
        module::StoredListExpr::Nil(expression) => {
            nil_list_expr(expression, context).map(execution::StoredListExpr::Nil)
        }
        module::StoredListExpr::Tuple(expression) => {
            tuple_list_expr(expression, context).map(execution::StoredListExpr::Tuple)
        }
        module::StoredListExpr::List(expression) => {
            list_list_expr(expression, context).map(execution::StoredListExpr::List)
        }
        module::StoredListExpr::Function(expression) => {
            function_list_expr(expression, context).map(execution::StoredListExpr::Function)
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_list_expr(
    expression: &module::GenericListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::ListExpr> {
    match context.concrete_parameter(expression.item().parameter()) {
        super::super::specialization::SpecializedValueShape::Parameter(parameter) => {
            parameter_list_expr(expression, parameter, context).map(execution::ListExpr::Parameter)
        }
        super::super::specialization::SpecializedValueShape::Int => {
            generic_int_list_expr(expression, context).map(execution::ListExpr::Int)
        }
        super::super::specialization::SpecializedValueShape::String => {
            generic_string_list_expr(expression, context).map(execution::ListExpr::String)
        }
        super::super::specialization::SpecializedValueShape::BitArray => {
            generic_bit_array_list_expr(expression, context).map(execution::ListExpr::BitArray)
        }
        super::super::specialization::SpecializedValueShape::UtfCodepoint => {
            generic_utf_codepoint_list_expr(expression, context)
                .map(execution::ListExpr::UtfCodepoint)
        }
        super::super::specialization::SpecializedValueShape::Custom(shape) => {
            generic_custom_list_expr(expression, &shape, context).map(execution::ListExpr::Custom)
        }
        super::super::specialization::SpecializedValueShape::Float => {
            generic_float_list_expr(expression, context).map(execution::ListExpr::Float)
        }
        super::super::specialization::SpecializedValueShape::Bool => {
            generic_bool_list_expr(expression, context).map(execution::ListExpr::Bool)
        }
        super::super::specialization::SpecializedValueShape::Nil => {
            generic_nil_list_expr(expression, context).map(execution::ListExpr::Nil)
        }
        super::super::specialization::SpecializedValueShape::Tuple(elements) => {
            generic_tuple_list_expr(expression, &elements, context).map(execution::ListExpr::Tuple)
        }
        super::super::specialization::SpecializedValueShape::List(item) => {
            generic_nested_list_facade(expression, &item, context)
        }
        super::super::specialization::SpecializedValueShape::Function(function) => {
            generic_function_list_expr(expression, &function, context)
                .map(execution::ListExpr::Function)
        }
    }
}

pub(in crate::plan::execution::lowering) fn int_list_expr(
    expression: &module::IntListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::IntListExpr> {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn string_list_expr(
    expression: &module::StringListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::StringListExpr> {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn bit_array_list_expr(
    expression: &module::BitArrayListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::BitArrayListExpr> {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn utf_codepoint_list_expr(
    expression: &module::UtfCodepointListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::UtfCodepointListExpr> {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn custom_list_expr(
    expression: &module::CustomListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::CustomListExpr> {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn float_list_expr(
    expression: &module::FloatListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::FloatListExpr> {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn bool_list_expr(
    expression: &module::BoolListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::BoolListExpr> {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn nil_list_expr(
    expression: &module::NilListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::NilListExpr> {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn tuple_list_expr(
    expression: &module::TupleListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::TupleListExpr> {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn list_list_expr(
    expression: &module::ListListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::ListListExpr> {
    let item =
        execution::ListListItem::new(context.stored_list_list_type(expression.item().item_shape()));
    lower_representable_list_kind(
        expression.kind(),
        &item,
        RepresentableListLowering {
            element: stored_list_expr,
            constant: |constant: &module::ConstantListListInstantiation,
                       context: &mut LoweringContext| {
                context.list_list_constant(constant)
            },
            local: |local: &module::ListListLocalId, context: &mut LoweringContext| {
                execution::ListListLocalId(
                    context.mapped_local(super::super::frame::LocalKind::ListList, local.0),
                )
            },
            function: |function: &module::FunctionInstantiation,
                       item: &execution::ListListItem,
                       context: &mut LoweringContext| {
                context.list_list_function_id(function, item.type_id())
            },
            index_source: list_list_expr,
        },
        context,
    )
    .map(|kind| execution::TypedListExpr::from_item_and_kind(item, kind))
}

pub(in crate::plan::execution::lowering) fn function_list_expr(
    expression: &module::FunctionListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::FunctionListExpr> {
    typed_list_expr(expression, context)
}

pub(in crate::plan::execution::lowering) fn parameter_list_expr(
    expression: &module::GenericListExpr,
    parameter: crate::plan::TypeParameterId,
    context: &mut LoweringContext,
) -> Representability<execution::ParameterListExpr> {
    let item = execution::ParameterListItem::new(context.parameter_list_type(parameter));
    lower_parameter_list_kind(expression.kind(), parameter, context)
        .map(|kind| execution::ParameterListExpr::from_parts(item, kind))
}

pub(in crate::plan::execution::lowering) fn parameter_list_value_expr(
    expression: &module::GenericExpr,
    parameter: crate::plan::TypeParameterId,
    context: &mut LoweringContext,
) -> Representability<execution::ParameterListExpr> {
    use execution::ParameterListExprKind as E;
    use module::GenericExprKind as M;

    let item = execution::ParameterListItem::new(context.parameter_list_type(parameter));
    let kind = match expression.kind() {
        M::LocalGet { local, name: _ } => {
            context
                .generic_local_index(local.id())
                .map(|index| E::LocalGet {
                    local: execution::ParameterListLocalId(index),
                })
        }
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.parameter_list_function_id(function, parameter)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| {
                super::generic_list_function_expr(
                    function,
                    &SpecializedValueShape::Parameter(parameter),
                    context,
                )
            },
            |context| super::function::evaluated_generic_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
            tuple: Box::new(tuple),
            index: *index,
        }),
        M::CustomField(access) => super::custom_field_access(access, context).map(E::CustomField),
        M::ListIndex { list, index } => generic_parameter_list_list_expr(list, parameter, context)
            .map(|list| E::ListIndex(execution::ParameterListIndexSource::new(list, *index))),
        M::Panic(panic) => panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            context,
            |context| {
                parameter_list_value_expr(true_, parameter, context)
                    .map(execution::ParameterListExpr::into_kind)
            },
            |context| {
                parameter_list_value_expr(false_, parameter, context)
                    .map(execution::ParameterListExpr::into_kind)
            },
            |subject, true_, false_| E::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        ),
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                parameter_list_value_expr(branch, parameter, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                parameter_list_value_expr(fallback, parameter, context).map(|fallback| E::IntCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback.into_kind()),
                })
            })
        }),
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                parameter_list_value_expr(branch, parameter, context)
                    .map(|branch| (pattern.clone(), branch.into_kind()))
            }))
            .and_then(|clauses| {
                parameter_list_value_expr(fallback, parameter, context).map(|fallback| {
                    E::StringCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback.into_kind()),
                    }
                })
            })
        }),
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                parameter_list_value_expr(branch, parameter, context)
                    .map(|branch| (*pattern, branch.into_kind()))
            }))
            .and_then(|clauses| {
                parameter_list_value_expr(fallback, parameter, context).map(|fallback| {
                    E::FloatCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback.into_kind()),
                    }
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                parameter_list_value_expr(return_, parameter, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_.into_kind()),
                })
            })
        }
    };

    kind.map(|kind| execution::ParameterListExpr::from_parts(item, kind))
}

fn lower_parameter_list_kind(
    kind: &module::TypedListExprKind<module::GenericListItem>,
    parameter: crate::plan::TypeParameterId,
    context: &mut LoweringContext,
) -> Representability<execution::ParameterListExprKind> {
    use execution::ParameterListExprKind as E;
    use module::TypedListExprKind as M;

    match kind {
        M::Value(elements) => match elements.as_slice() {
            [] => Representability::Inhabited(E::Value),
            [first, ..] => super::generic::never_expr(first, context).map(E::Never),
        },
        M::Constant(constant) => context
            .generic_parameter_list_constant(constant, parameter)
            .map(E::Constant),
        M::Spread { elements, tail: _ } => {
            super::generic::never_expr(elements.first(), context).map(E::Never)
        }
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: execution::ParameterListLocalId(context.generic_list_local_index(*local)),
        }),
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                context.parameter_list_function_id(function, parameter)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| list_function_expr(function, context),
            |context| super::function::evaluated_list_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
            tuple: Box::new(tuple),
            index: *index,
        }),
        M::CustomField(access) => super::custom_field_access(access, context).map(E::CustomField),
        M::ListIndex(source) => {
            unresolved_parameter_list_list_expr(source.list(), parameter, context).map(|list| {
                E::ListIndex(execution::ParameterListIndexSource::new(
                    list,
                    source.index(),
                ))
            })
        }
        M::DropFirst { list, count: _ } => lower_parameter_list_kind(list, parameter, context),
        M::Panic(panic) => panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            context,
            |context| lower_parameter_list_kind(true_, parameter, context),
            |context| lower_parameter_list_kind(false_, parameter, context),
            |subject, true_, false_| E::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        ),
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                lower_parameter_list_kind(branch, parameter, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                lower_parameter_list_kind(fallback, parameter, context).map(|fallback| E::IntCase {
                    subject: Box::new(subject),
                    clauses,
                    fallback: Box::new(fallback),
                })
            })
        }),
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                lower_parameter_list_kind(branch, parameter, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                lower_parameter_list_kind(fallback, parameter, context).map(|fallback| {
                    E::StringCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    }
                })
            })
        }),
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                lower_parameter_list_kind(branch, parameter, context)
                    .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                lower_parameter_list_kind(fallback, parameter, context).map(|fallback| {
                    E::FloatCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    }
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                lower_parameter_list_kind(return_, parameter, context).map(|return_| E::Block {
                    steps,
                    return_: Box::new(return_),
                })
            })
        }
    }
}

fn parameter_list_list_expr(
    expression: &module::ParameterListListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::StoredListExpr> {
    match context
        .concrete_parameter(expression.item().parameter())
        .storage_representation()
    {
        super::super::specialization::StorageRepresentation::Parameter(parameter) => {
            unresolved_parameter_list_list_expr(expression, parameter, context)
                .map(execution::StoredListExpr::ParameterList)
        }
        super::super::specialization::StorageRepresentation::Stored(shape) => {
            concrete_parameter_list_list_expr(expression, &shape, context)
                .map(execution::StoredListExpr::List)
        }
    }
}

fn parameter_list_list_facade(
    expression: &module::ParameterListListExpr,
    context: &mut LoweringContext,
) -> Representability<execution::ListExpr> {
    match context
        .concrete_parameter(expression.item().parameter())
        .storage_representation()
    {
        super::super::specialization::StorageRepresentation::Parameter(parameter) => {
            unresolved_parameter_list_list_expr(expression, parameter, context)
                .map(execution::ListExpr::ParameterList)
        }
        super::super::specialization::StorageRepresentation::Stored(shape) => {
            concrete_parameter_list_list_expr(expression, &shape, context)
                .map(execution::ListExpr::List)
        }
    }
}

pub(in crate::plan::execution::lowering) fn unresolved_parameter_list_list_expr(
    expression: &module::ParameterListListExpr,
    parameter: crate::plan::TypeParameterId,
    context: &mut LoweringContext,
) -> Representability<execution::ParameterListListExpr> {
    let item = execution::ParameterListListItem::new(context.parameter_list_list_type(parameter));
    lower_representable_list_kind(
        expression.kind(),
        &item,
        RepresentableListLowering {
            element: |element: &module::GenericListExpr, context: &mut LoweringContext| {
                parameter_list_expr(element, parameter, context)
            },
            constant: |constant: &module::ConstantParameterListListInstantiation,
                       context: &mut LoweringContext| {
                context.parameter_list_list_constant(constant, parameter)
            },
            local: |local: &module::ListListLocalId, context: &mut LoweringContext| {
                execution::ParameterListListLocalId(
                    context.mapped_local(super::super::frame::LocalKind::ListList, local.0),
                )
            },
            function: |function: &module::FunctionInstantiation,
                       item: &execution::ParameterListListItem,
                       context: &mut LoweringContext| {
                context.parameter_list_list_function_id(function, item.type_id())
            },
            index_source: list_list_expr,
        },
        context,
    )
    .map(|kind| execution::TypedListExpr::from_item_and_kind(item, kind))
}

pub(in crate::plan::execution::lowering) fn generic_parameter_list_list_expr(
    expression: &module::GenericListExpr,
    parameter: crate::plan::TypeParameterId,
    context: &mut LoweringContext,
) -> Representability<execution::ParameterListListExpr> {
    let item = execution::ParameterListListItem::new(context.parameter_list_list_type(parameter));
    let nested_item = StoredValueShape::List(Box::new(SpecializedValueShape::Parameter(parameter)));
    lower_representable_list_kind(
        expression.kind(),
        &item,
        RepresentableListLowering {
            element: |element: &module::GenericExpr, context: &mut LoweringContext| {
                parameter_list_value_expr(element, parameter, context)
            },
            constant: |constant: &module::ConstantGenericListInstantiation,
                       context: &mut LoweringContext| {
                context.generic_parameter_list_list_constant(constant, parameter)
            },
            local: |local: &module::GenericListLocalId, context: &mut LoweringContext| {
                execution::ParameterListListLocalId(context.generic_list_local_index(*local))
            },
            function: |function: &module::FunctionInstantiation,
                       item: &execution::ParameterListListItem,
                       context: &mut LoweringContext| {
                context.parameter_list_list_function_id(function, item.type_id())
            },
            index_source: |source: &module::ParameterListListExpr,
                           context: &mut LoweringContext| {
                concrete_parameter_list_list_expr(source, &nested_item, context)
            },
        },
        context,
    )
    .map(|kind| execution::TypedListExpr::from_item_and_kind(item, kind))
}

pub(in crate::plan::execution::lowering) fn concrete_parameter_list_list_expr(
    expression: &module::ParameterListListExpr,
    item_shape: &StoredValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::ListListExpr> {
    let item = execution::ListListItem::new(context.specialized_stored_list_list_type(item_shape));
    lower_representable_list_kind(
        expression.kind(),
        &item,
        RepresentableListLowering {
            element: |element: &module::GenericListExpr, context: &mut LoweringContext| {
                generic_stored_list_expr(element, item_shape, context)
            },
            constant: |constant: &module::ConstantParameterListListInstantiation,
                       context: &mut LoweringContext| {
                context.parameter_list_list_as_stored_constant(constant, item_shape)
            },
            local: |local: &module::ListListLocalId, context: &mut LoweringContext| {
                execution::ListListLocalId(
                    context.mapped_local(super::super::frame::LocalKind::ListList, local.0),
                )
            },
            function: |function: &module::FunctionInstantiation,
                       item: &execution::ListListItem,
                       context: &mut LoweringContext| {
                context.list_list_function_id(function, item.type_id())
            },
            index_source: list_list_expr,
        },
        context,
    )
    .map(|kind| execution::TypedListExpr::from_item_and_kind(item, kind))
}

fn generic_stored_list_expr(
    expression: &module::GenericListExpr,
    item_shape: &StoredValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::StoredListExpr> {
    match item_shape {
        StoredValueShape::Int => {
            generic_int_list_expr(expression, context).map(execution::StoredListExpr::Int)
        }
        StoredValueShape::String => {
            generic_string_list_expr(expression, context).map(execution::StoredListExpr::String)
        }
        StoredValueShape::BitArray => generic_bit_array_list_expr(expression, context)
            .map(execution::StoredListExpr::BitArray),
        StoredValueShape::UtfCodepoint => generic_utf_codepoint_list_expr(expression, context)
            .map(execution::StoredListExpr::UtfCodepoint),
        StoredValueShape::Custom(shape) => generic_custom_list_expr(expression, shape, context)
            .map(execution::StoredListExpr::Custom),
        StoredValueShape::Float => {
            generic_float_list_expr(expression, context).map(execution::StoredListExpr::Float)
        }
        StoredValueShape::Bool => {
            generic_bool_list_expr(expression, context).map(execution::StoredListExpr::Bool)
        }
        StoredValueShape::Nil => {
            generic_nil_list_expr(expression, context).map(execution::StoredListExpr::Nil)
        }
        StoredValueShape::Tuple(elements) => generic_tuple_list_expr(expression, elements, context)
            .map(execution::StoredListExpr::Tuple),
        StoredValueShape::List(item) => generic_nested_list_expr(expression, item, context),
        StoredValueShape::Function(function) => {
            generic_function_list_expr(expression, function, context)
                .map(execution::StoredListExpr::Function)
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_stored_nested_list_expr(
    expression: &module::GenericListExpr,
    item_shape: &StoredValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::ListListExpr> {
    let item = execution::ListListItem::new(context.specialized_stored_list_list_type(item_shape));
    lower_representable_list_kind(
        expression.kind(),
        &item,
        RepresentableListLowering {
            element: |element: &module::GenericExpr, context: &mut LoweringContext| {
                super::generic_value_stored_list_expr(element, item_shape, context)
            },
            constant: |constant: &module::ConstantGenericListInstantiation,
                       context: &mut LoweringContext| {
                context.generic_list_list_constant(constant, item_shape)
            },
            local: |local: &module::GenericListLocalId, context: &mut LoweringContext| {
                execution::ListListLocalId(context.generic_list_local_index(*local))
            },
            function: |function: &module::FunctionInstantiation,
                       item: &execution::ListListItem,
                       context: &mut LoweringContext| {
                context.list_list_function_id(function, item.type_id())
            },
            index_source: |source: &module::ParameterListListExpr,
                           context: &mut LoweringContext| {
                concrete_parameter_list_list_expr(source, item_shape, context)
            },
        },
        context,
    )
    .map(|kind| execution::TypedListExpr::from_item_and_kind(item, kind))
}

macro_rules! primitive_generic_list_expr {
    (
        $lower:ident,
        $result:ty,
        $item:ident,
        $stored:ident,
        $type_id:ident,
        $element:ident,
        $constant:ident,
        $local:ident,
        $function:ident
    ) => {
        pub(in crate::plan::execution::lowering) fn $lower(
            expression: &module::GenericListExpr,
            context: &mut LoweringContext,
        ) -> Representability<$result> {
            let item = execution::$item::new(context.$type_id());
            generic_typed_list_expr(
                expression,
                item,
                RepresentableListLowering {
                    element: $element,
                    constant: |constant: &module::ConstantGenericListInstantiation,
                               context: &mut LoweringContext| {
                        context.$constant(constant)
                    },
                    local: |local: &module::GenericListLocalId, context: &mut LoweringContext| {
                        execution::$local(context.generic_list_local_index(*local))
                    },
                    function: |function: &module::FunctionInstantiation,
                               _: &execution::$item,
                               context: &mut LoweringContext| {
                        context.$function(function)
                    },
                    index_source: |source: &module::ParameterListListExpr,
                                   context: &mut LoweringContext| {
                        concrete_parameter_list_list_expr(
                            source,
                            &StoredValueShape::$stored,
                            context,
                        )
                    },
                },
                context,
            )
        }
    };
}

primitive_generic_list_expr!(
    generic_int_list_expr,
    execution::IntListExpr,
    IntListItem,
    Int,
    int_list_type,
    generic_int_expr,
    generic_int_list_constant,
    IntListLocalId,
    int_list_function_id
);
primitive_generic_list_expr!(
    generic_string_list_expr,
    execution::StringListExpr,
    StringListItem,
    String,
    string_list_type,
    generic_string_expr,
    generic_string_list_constant,
    StringListLocalId,
    string_list_function_id
);
primitive_generic_list_expr!(
    generic_bit_array_list_expr,
    execution::BitArrayListExpr,
    BitArrayListItem,
    BitArray,
    bit_array_list_type,
    generic_bit_array_expr,
    generic_bit_array_list_constant,
    BitArrayListLocalId,
    bit_array_list_function_id
);
primitive_generic_list_expr!(
    generic_utf_codepoint_list_expr,
    execution::UtfCodepointListExpr,
    UtfCodepointListItem,
    UtfCodepoint,
    utf_codepoint_list_type,
    generic_utf_codepoint_expr,
    generic_utf_codepoint_list_constant,
    UtfCodepointListLocalId,
    utf_codepoint_list_function_id
);
primitive_generic_list_expr!(
    generic_float_list_expr,
    execution::FloatListExpr,
    FloatListItem,
    Float,
    float_list_type,
    generic_float_expr,
    generic_float_list_constant,
    FloatListLocalId,
    float_list_function_id
);
primitive_generic_list_expr!(
    generic_bool_list_expr,
    execution::BoolListExpr,
    BoolListItem,
    Bool,
    bool_list_type,
    generic_bool_expr,
    generic_bool_list_constant,
    BoolListLocalId,
    bool_list_function_id
);
primitive_generic_list_expr!(
    generic_nil_list_expr,
    execution::NilListExpr,
    NilListItem,
    Nil,
    nil_list_type,
    generic_nil_expr,
    generic_nil_list_constant,
    NilListLocalId,
    nil_list_function_id
);

pub(in crate::plan::execution::lowering) fn generic_tuple_list_expr(
    expression: &module::GenericListExpr,
    elements: &[super::super::specialization::SpecializedValueShape],
    context: &mut LoweringContext,
) -> Representability<execution::TupleListExpr> {
    let stored_item = StoredValueShape::Tuple(elements.to_vec().into_boxed_slice());
    let item = execution::TupleListItem::new(context.specialized_tuple_list_type(elements));
    generic_typed_list_expr(
        expression,
        item,
        RepresentableListLowering {
            element: |element: &module::GenericExpr, context: &mut LoweringContext| {
                super::generic_tuple_expr(element, elements, context)
            },
            constant: |constant: &module::ConstantGenericListInstantiation,
                       context: &mut LoweringContext| {
                context.generic_tuple_list_constant(constant, elements)
            },
            local: |local: &module::GenericListLocalId, context: &mut LoweringContext| {
                execution::TupleListLocalId(context.generic_list_local_index(*local))
            },
            function: |function: &module::FunctionInstantiation,
                       item: &execution::TupleListItem,
                       context: &mut LoweringContext| {
                context.tuple_list_function_id(function, item.type_id())
            },
            index_source: |source: &module::ParameterListListExpr,
                           context: &mut LoweringContext| {
                concrete_parameter_list_list_expr(source, &stored_item, context)
            },
        },
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_custom_list_expr(
    expression: &module::GenericListExpr,
    shape: &super::super::specialization::SpecializedCustomValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::CustomListExpr> {
    let stored_item = StoredValueShape::Custom(shape.clone());
    let item = execution::CustomListItem::new(context.specialized_custom_list_type(shape));
    generic_typed_list_expr(
        expression,
        item,
        RepresentableListLowering {
            element: |element: &module::GenericExpr, context: &mut LoweringContext| {
                super::generic_custom_expr(element, shape, context)
            },
            constant: |constant: &module::ConstantGenericListInstantiation,
                       context: &mut LoweringContext| {
                context.generic_custom_list_constant(constant, shape)
            },
            local: |local: &module::GenericListLocalId, context: &mut LoweringContext| {
                execution::CustomListLocalId(context.generic_list_local_index(*local))
            },
            function: |function: &module::FunctionInstantiation,
                       item: &execution::CustomListItem,
                       context: &mut LoweringContext| {
                context.custom_list_function_id(function, item.type_id())
            },
            index_source: |source: &module::ParameterListListExpr,
                           context: &mut LoweringContext| {
                concrete_parameter_list_list_expr(source, &stored_item, context)
            },
        },
        context,
    )
}

pub(in crate::plan::execution::lowering) fn generic_nested_list_expr(
    expression: &module::GenericListExpr,
    item_shape: &super::super::specialization::SpecializedValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::StoredListExpr> {
    match item_shape.storage_representation() {
        super::super::specialization::StorageRepresentation::Parameter(parameter) => {
            generic_parameter_list_list_expr(expression, parameter, context)
                .map(execution::StoredListExpr::ParameterList)
        }
        super::super::specialization::StorageRepresentation::Stored(item_shape) => {
            generic_stored_nested_list_expr(expression, &item_shape, context)
                .map(execution::StoredListExpr::List)
        }
    }
}

fn generic_nested_list_facade(
    expression: &module::GenericListExpr,
    item_shape: &super::super::specialization::SpecializedValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::ListExpr> {
    match item_shape.storage_representation() {
        super::super::specialization::StorageRepresentation::Parameter(parameter) => {
            generic_parameter_list_list_expr(expression, parameter, context)
                .map(execution::ListExpr::ParameterList)
        }
        super::super::specialization::StorageRepresentation::Stored(item_shape) => {
            generic_stored_nested_list_expr(expression, &item_shape, context)
                .map(execution::ListExpr::List)
        }
    }
}

pub(in crate::plan::execution::lowering) fn generic_function_list_expr(
    expression: &module::GenericListExpr,
    function_shape: &super::super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::FunctionListExpr> {
    let stored_item = StoredValueShape::Function(Box::new(function_shape.clone()));
    let item =
        execution::FunctionListItem::new(context.specialized_function_list_type(function_shape));
    generic_typed_list_expr(
        expression,
        item,
        RepresentableListLowering {
            element: |element: &module::GenericExpr, context: &mut LoweringContext| {
                super::generic_function_value_expr(element, function_shape, context)
            },
            constant: |constant: &module::ConstantGenericListInstantiation,
                       context: &mut LoweringContext| {
                context.generic_function_list_constant(constant, function_shape)
            },
            local: |local: &module::GenericListLocalId, context: &mut LoweringContext| {
                execution::FunctionListLocalId(context.generic_list_local_index(*local))
            },
            function: |function: &module::FunctionInstantiation,
                       item: &execution::FunctionListItem,
                       context: &mut LoweringContext| {
                context.function_list_function_id(function, item.type_id())
            },
            index_source: |source: &module::ParameterListListExpr,
                           context: &mut LoweringContext| {
                concrete_parameter_list_list_expr(source, &stored_item, context)
            },
        },
        context,
    )
}

#[derive(Clone, Copy)]
struct RepresentableListLowering<Element, Constant, Local, Function, IndexSource> {
    element: Element,
    constant: Constant,
    local: Local,
    function: Function,
    index_source: IndexSource,
}

fn generic_typed_list_expr<Item, Element, Constant, Local, Function, IndexSource>(
    expression: &module::GenericListExpr,
    item: Item,
    lowering: RepresentableListLowering<Element, Constant, Local, Function, IndexSource>,
    context: &mut LoweringContext,
) -> Representability<execution::TypedListExpr<Item>>
where
    Item: execution::ListItem,
    Element: Copy
        + Fn(&module::GenericExpr, &mut LoweringContext) -> Representability<Item::ElementExpr>,
    Constant: Copy
        + Fn(
            &module::ConstantGenericListInstantiation,
            &mut LoweringContext,
        ) -> Representability<Item::Constant>,
    Local: Copy + Fn(&module::GenericListLocalId, &mut LoweringContext) -> Item::Local,
    Function: Copy
        + Fn(
            &module::FunctionInstantiation,
            &Item,
            &mut LoweringContext,
        ) -> Representability<Item::Function>,
    IndexSource: Copy
        + Fn(
            &module::ParameterListListExpr,
            &mut LoweringContext,
        ) -> Representability<Item::IndexSource>,
{
    lower_representable_list_kind(expression.kind(), &item, lowering, context)
        .map(|kind| execution::TypedListExpr::from_item_and_kind(item, kind))
}

fn lower_representable_list_kind<
    ModuleItem,
    Item,
    Element,
    Constant,
    Local,
    Function,
    IndexSource,
>(
    kind: &module::TypedListExprKind<ModuleItem>,
    item: &Item,
    lowering: RepresentableListLowering<Element, Constant, Local, Function, IndexSource>,
    context: &mut LoweringContext,
) -> Representability<execution::TypedListExprKind<Item>>
where
    ModuleItem: module::ListItem<Function = module::FunctionInstantiation>,
    Item: execution::ListItem,
    Element: Copy
        + Fn(&ModuleItem::ElementExpr, &mut LoweringContext) -> Representability<Item::ElementExpr>,
    Constant:
        Copy + Fn(&ModuleItem::Constant, &mut LoweringContext) -> Representability<Item::Constant>,
    Local: Copy + Fn(&ModuleItem::Local, &mut LoweringContext) -> Item::Local,
    Function: Copy
        + Fn(
            &module::FunctionInstantiation,
            &Item,
            &mut LoweringContext,
        ) -> Representability<Item::Function>,
    IndexSource: Copy
        + Fn(&ModuleItem::IndexSource, &mut LoweringContext) -> Representability<Item::IndexSource>,
{
    use execution::TypedListExprKind as E;
    use module::TypedListExprKind as M;

    match kind {
        M::Value(elements) => Representability::collect(
            elements
                .iter()
                .map(|element| (lowering.element)(element, context)),
        )
        .map(E::Value),
        M::Constant(constant) => (lowering.constant)(constant, context).map(E::Constant),
        M::Spread { elements, tail } => {
            let first = (lowering.element)(elements.first(), context);
            let rest = Representability::collect(
                elements[1..]
                    .iter()
                    .map(|element| (lowering.element)(element, context)),
            );
            first
                .zip_with(rest, |first, rest| {
                    let mut elements = Vec1::with_capacity(first, rest.len() + 1);
                    elements.extend(rest);
                    elements
                })
                .and_then(|elements| {
                    lower_representable_list_kind(tail, item, lowering, context).map(|tail| {
                        E::Spread {
                            elements,
                            tail: Box::new(tail),
                        }
                    })
                })
        }
        M::LocalGet { local, name: _ } => Representability::Inhabited(E::LocalGet {
            local: (lowering.local)(local, context),
        }),
        M::Call { function, args } => {
            super::direct_call(function, args, context, |function, context| {
                (lowering.function)(function, item, context)
            })
            .map(E::Call)
        }
        M::FunctionCall { function, args } => super::function_call(
            args,
            context,
            |context| list_function_expr(function, context),
            |context| super::function::evaluated_list_function_expr(function, context),
        )
        .map(E::FunctionCall),
        M::TupleIndex { tuple, index } => tuple_expr(tuple, context).map(|tuple| E::TupleIndex {
            tuple: Box::new(tuple),
            index: *index,
        }),
        M::CustomField(access) => super::custom_field_access(access, context).map(E::CustomField),
        M::ListIndex(source) => (lowering.index_source)(source.list(), context)
            .map(|list| E::ListIndex(execution::ListIndexSource::from_parts(list, source.index()))),
        M::DropFirst { list, count } => {
            lower_representable_list_kind(list, item, lowering, context).map(|list| E::DropFirst {
                list: Box::new(list),
                count: *count,
            })
        }
        M::Panic(panic) => panic_expr(panic, context).map(E::Panic),
        M::BoolCase {
            subject,
            true_,
            false_,
        } => super::bool_case(
            subject,
            context,
            |context| lower_representable_list_kind(true_, item, lowering, context),
            |context| lower_representable_list_kind(false_, item, lowering, context),
            |subject, true_, false_| E::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        ),
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => int_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                lower_representable_list_kind(branch, item, lowering, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                lower_representable_list_kind(fallback, item, lowering, context).map(|fallback| {
                    E::IntCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    }
                })
            })
        }),
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => string_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                lower_representable_list_kind(branch, item, lowering, context)
                    .map(|branch| (pattern.clone(), branch))
            }))
            .and_then(|clauses| {
                lower_representable_list_kind(fallback, item, lowering, context).map(|fallback| {
                    E::StringCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    }
                })
            })
        }),
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => float_expr(subject, context).and_then(|subject| {
            Representability::collect(clauses.iter().map(|(pattern, branch)| {
                lower_representable_list_kind(branch, item, lowering, context)
                    .map(|branch| (*pattern, branch))
            }))
            .and_then(|clauses| {
                lower_representable_list_kind(fallback, item, lowering, context).map(|fallback| {
                    E::FloatCase {
                        subject: Box::new(subject),
                        clauses,
                        fallback: Box::new(fallback),
                    }
                })
            })
        }),
        M::Block { steps, return_ } => {
            super::super::step::steps(steps, context).and_then(|steps| {
                lower_representable_list_kind(return_, item, lowering, context).map(|return_| {
                    E::Block {
                        steps,
                        return_: Box::new(return_),
                    }
                })
            })
        }
    }
}

fn typed_list_expr<Item>(
    expression: &module::TypedListExpr<Item>,
    context: &mut LoweringContext,
) -> Representability<execution::TypedListExpr<Item::Execution>>
where
    Item: LowerListItem,
{
    let item = expression.item().lower_item(context);
    lower_representable_list_kind(
        expression.kind(),
        &item,
        RepresentableListLowering {
            element: Item::lower_element,
            constant: Item::lower_constant,
            local: Item::lower_local,
            function: |function: &module::FunctionInstantiation,
                       item: &Item::Execution,
                       context: &mut LoweringContext| {
                expression.item().lower_function(function, item, context)
            },
            index_source: list_list_expr,
        },
        context,
    )
    .map(|kind| execution::TypedListExpr::from_item_and_kind(item, kind))
}

pub(in crate::plan::execution::lowering) fn list_local_expr(
    expression: &module::ListLocalExpr,
    context: &mut LoweringContext,
) -> Representability<execution::ListLocalExpr> {
    let index = context.local_index(super::list_local_expr_key(expression));
    list_local_expr_at(index, expression, context)
}

pub(super) fn list_local_expr_at(
    index: usize,
    expression: &module::ListLocalExpr,
    context: &mut LoweringContext,
) -> Representability<execution::ListLocalExpr> {
    match expression {
        module::ListLocalExpr::Generic { value, .. } => {
            specialized_list_local_expr(index, generic_list_expr(value, context))
        }
        module::ListLocalExpr::ParameterList { value, .. } => {
            specialized_list_local_expr(index, parameter_list_list_facade(value, context))
        }
        module::ListLocalExpr::Int { value, .. } => {
            int_list_expr(value, context).map(|value| execution::ListLocalExpr::Int {
                local: execution::IntListLocalId(index),
                value,
            })
        }
        module::ListLocalExpr::String { value, .. } => {
            string_list_expr(value, context).map(|value| execution::ListLocalExpr::String {
                local: execution::StringListLocalId(index),
                value,
            })
        }
        module::ListLocalExpr::BitArray { value, .. } => {
            bit_array_list_expr(value, context).map(|value| execution::ListLocalExpr::BitArray {
                local: execution::BitArrayListLocalId(index),
                value,
            })
        }
        module::ListLocalExpr::UtfCodepoint { value, .. } => {
            utf_codepoint_list_expr(value, context).map(|value| {
                execution::ListLocalExpr::UtfCodepoint {
                    local: execution::UtfCodepointListLocalId(index),
                    value,
                }
            })
        }
        module::ListLocalExpr::Custom { value, .. } => {
            custom_list_expr(value, context).map(|value| execution::ListLocalExpr::Custom {
                local: execution::CustomListLocalId(index),
                value,
            })
        }
        module::ListLocalExpr::Float { value, .. } => {
            float_list_expr(value, context).map(|value| execution::ListLocalExpr::Float {
                local: execution::FloatListLocalId(index),
                value,
            })
        }
        module::ListLocalExpr::Bool { value, .. } => {
            bool_list_expr(value, context).map(|value| execution::ListLocalExpr::Bool {
                local: execution::BoolListLocalId(index),
                value,
            })
        }
        module::ListLocalExpr::Nil { value, .. } => {
            nil_list_expr(value, context).map(|value| execution::ListLocalExpr::Nil {
                local: execution::NilListLocalId(index),
                value,
            })
        }
        module::ListLocalExpr::Tuple { value, .. } => {
            tuple_list_expr(value, context).map(|value| execution::ListLocalExpr::Tuple {
                local: execution::TupleListLocalId(index),
                value,
            })
        }
        module::ListLocalExpr::List { value, .. } => {
            list_list_expr(value, context).map(|value| execution::ListLocalExpr::List {
                local: execution::ListListLocalId(index),
                value,
            })
        }
        module::ListLocalExpr::Function { value, .. } => {
            function_list_expr(value, context).map(|value| execution::ListLocalExpr::Function {
                local: execution::FunctionListLocalId(index),
                value,
            })
        }
    }
}

pub(super) fn specialized_list_local_expr(
    index: usize,
    value: Representability<execution::ListExpr>,
) -> Representability<execution::ListLocalExpr> {
    value.map(|value| match value {
        execution::ListExpr::Parameter(value) => execution::ListLocalExpr::Parameter {
            local: execution::ParameterListLocalId(index),
            value,
        },
        execution::ListExpr::ParameterList(value) => execution::ListLocalExpr::ParameterList {
            local: execution::ParameterListListLocalId(index),
            value,
        },
        execution::ListExpr::Int(value) => execution::ListLocalExpr::Int {
            local: execution::IntListLocalId(index),
            value,
        },
        execution::ListExpr::String(value) => execution::ListLocalExpr::String {
            local: execution::StringListLocalId(index),
            value,
        },
        execution::ListExpr::BitArray(value) => execution::ListLocalExpr::BitArray {
            local: execution::BitArrayListLocalId(index),
            value,
        },
        execution::ListExpr::UtfCodepoint(value) => execution::ListLocalExpr::UtfCodepoint {
            local: execution::UtfCodepointListLocalId(index),
            value,
        },
        execution::ListExpr::Custom(value) => execution::ListLocalExpr::Custom {
            local: execution::CustomListLocalId(index),
            value,
        },
        execution::ListExpr::Float(value) => execution::ListLocalExpr::Float {
            local: execution::FloatListLocalId(index),
            value,
        },
        execution::ListExpr::Bool(value) => execution::ListLocalExpr::Bool {
            local: execution::BoolListLocalId(index),
            value,
        },
        execution::ListExpr::Nil(value) => execution::ListLocalExpr::Nil {
            local: execution::NilListLocalId(index),
            value,
        },
        execution::ListExpr::Tuple(value) => execution::ListLocalExpr::Tuple {
            local: execution::TupleListLocalId(index),
            value,
        },
        execution::ListExpr::List(value) => execution::ListLocalExpr::List {
            local: execution::ListListLocalId(index),
            value,
        },
        execution::ListExpr::Function(value) => execution::ListLocalExpr::Function {
            local: execution::FunctionListLocalId(index),
            value,
        },
    })
}

impl LowerListItem for module::IntListItem {
    type Execution = execution::IntListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::IntListItem::new(context.int_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> Representability<execution::IntExpr> {
        int_expr(element, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut LoweringContext,
    ) -> Representability<execution::ConstantId<execution::IntListExpr>> {
        context.int_list_constant(constant)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::IntListLocalId {
        execution::IntListLocalId(
            context.mapped_local(super::super::frame::LocalKind::IntList, local.0),
        )
    }

    fn lower_function(
        &self,
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> Representability<execution::IntListFunctionId> {
        context.int_list_function_id(function)
    }
}

impl LowerListItem for module::StringListItem {
    type Execution = execution::StringListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::StringListItem::new(context.string_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> Representability<execution::StringExpr> {
        string_expr(element, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut LoweringContext,
    ) -> Representability<execution::ConstantId<execution::StringListExpr>> {
        context.string_list_constant(constant)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::StringListLocalId {
        execution::StringListLocalId(
            context.mapped_local(super::super::frame::LocalKind::StringList, local.0),
        )
    }

    fn lower_function(
        &self,
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> Representability<execution::StringListFunctionId> {
        context.string_list_function_id(function)
    }
}

impl LowerListItem for module::BitArrayListItem {
    type Execution = execution::BitArrayListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::BitArrayListItem::new(context.bit_array_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> Representability<execution::BitArrayExpr> {
        bit_array_expr(element, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut LoweringContext,
    ) -> Representability<execution::ConstantId<execution::BitArrayListExpr>> {
        context.bit_array_list_constant(constant)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::BitArrayListLocalId {
        execution::BitArrayListLocalId(
            context.mapped_local(super::super::frame::LocalKind::BitArrayList, local.0),
        )
    }

    fn lower_function(
        &self,
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> Representability<execution::BitArrayListFunctionId> {
        context.bit_array_list_function_id(function)
    }
}

impl LowerListItem for module::UtfCodepointListItem {
    type Execution = execution::UtfCodepointListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::UtfCodepointListItem::new(context.utf_codepoint_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> Representability<execution::UtfCodepointExpr> {
        utf_codepoint_expr(element, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut LoweringContext,
    ) -> Representability<execution::ConstantId<execution::UtfCodepointListExpr>> {
        context.utf_codepoint_list_constant(constant)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::UtfCodepointListLocalId {
        execution::UtfCodepointListLocalId(
            context.mapped_local(super::super::frame::LocalKind::UtfCodepointList, local.0),
        )
    }

    fn lower_function(
        &self,
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> Representability<execution::UtfCodepointListFunctionId> {
        context.utf_codepoint_list_function_id(function)
    }
}

impl LowerListItem for module::CustomListItem {
    type Execution = execution::CustomListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::CustomListItem::new(context.custom_list_type(self.item_type()))
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> Representability<execution::CustomExpr> {
        custom_expr(element, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut LoweringContext,
    ) -> Representability<execution::ConstantId<execution::CustomListExpr>> {
        context.custom_list_constant(constant)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::CustomListLocalId {
        execution::CustomListLocalId(
            context.mapped_local(super::super::frame::LocalKind::CustomList, local.0),
        )
    }

    fn lower_function(
        &self,
        function: &Self::Function,
        item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> Representability<execution::CustomListFunctionId> {
        context.custom_list_function_id(function, item.type_id())
    }
}

impl LowerListItem for module::FloatListItem {
    type Execution = execution::FloatListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::FloatListItem::new(context.float_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> Representability<execution::FloatExpr> {
        float_expr(element, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut LoweringContext,
    ) -> Representability<execution::ConstantId<execution::FloatListExpr>> {
        context.float_list_constant(constant)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::FloatListLocalId {
        execution::FloatListLocalId(
            context.mapped_local(super::super::frame::LocalKind::FloatList, local.0),
        )
    }

    fn lower_function(
        &self,
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> Representability<execution::FloatListFunctionId> {
        context.float_list_function_id(function)
    }
}

impl LowerListItem for module::BoolListItem {
    type Execution = execution::BoolListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::BoolListItem::new(context.bool_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> Representability<execution::BoolExpr> {
        bool_expr(element, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut LoweringContext,
    ) -> Representability<execution::ConstantId<execution::BoolListExpr>> {
        context.bool_list_constant(constant)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::BoolListLocalId {
        execution::BoolListLocalId(
            context.mapped_local(super::super::frame::LocalKind::BoolList, local.0),
        )
    }

    fn lower_function(
        &self,
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> Representability<execution::BoolListFunctionId> {
        context.bool_list_function_id(function)
    }
}

impl LowerListItem for module::NilListItem {
    type Execution = execution::NilListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::NilListItem::new(context.nil_list_type())
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> Representability<execution::NilExpr> {
        super::nil_expr(element, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut LoweringContext,
    ) -> Representability<execution::ConstantId<execution::NilListExpr>> {
        context.nil_list_constant(constant)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::NilListLocalId {
        execution::NilListLocalId(
            context.mapped_local(super::super::frame::LocalKind::NilList, local.0),
        )
    }

    fn lower_function(
        &self,
        function: &Self::Function,
        _item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> Representability<execution::NilListFunctionId> {
        context.nil_list_function_id(function)
    }
}

impl LowerListItem for module::TupleListItem {
    type Execution = execution::TupleListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::TupleListItem::new(context.tuple_list_type(self.item_type()))
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> Representability<execution::TupleExpr> {
        tuple_expr(element, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut LoweringContext,
    ) -> Representability<execution::ConstantId<execution::TupleListExpr>> {
        context.tuple_list_constant(constant)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::TupleListLocalId {
        execution::TupleListLocalId(
            context.mapped_local(super::super::frame::LocalKind::TupleList, local.0),
        )
    }

    fn lower_function(
        &self,
        function: &Self::Function,
        item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> Representability<execution::TupleListFunctionId> {
        context.tuple_list_function_id(function, item.type_id())
    }
}

impl LowerListItem for module::FunctionListItem {
    type Execution = execution::FunctionListItem;

    fn lower_item(&self, context: &mut LoweringContext) -> Self::Execution {
        execution::FunctionListItem::new(context.function_list_type(self.item_type()))
    }

    fn lower_element(
        element: &Self::ElementExpr,
        context: &mut LoweringContext,
    ) -> Representability<execution::FunctionExpr> {
        function_expr(element, context)
    }

    fn lower_constant(
        constant: &Self::Constant,
        context: &mut LoweringContext,
    ) -> Representability<execution::ConstantId<execution::FunctionListExpr>> {
        context.function_list_constant(constant)
    }

    fn lower_local(
        local: &Self::Local,
        context: &mut LoweringContext,
    ) -> execution::FunctionListLocalId {
        execution::FunctionListLocalId(
            context.mapped_local(super::super::frame::LocalKind::FunctionList, local.0),
        )
    }

    fn lower_function(
        &self,
        function: &Self::Function,
        item: &Self::Execution,
        context: &mut LoweringContext,
    ) -> Representability<execution::FunctionListFunctionId> {
        context.function_list_function_id(function, item.type_id())
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::specialization::{
        Representability, RepresentationContext, SpecializationKey, SpecializedTypeSubstitution,
        SpecializedValueShape, StoredValueShape,
    };
    use super::super::super::{FunctionTemplates, LoweringContext};
    use crate::plan::execution::{
        ExecutionPlan, IntListFunctionId, ListFunctionId, ListItem, ListLocalExpr, ReturnBody,
        ReturnBodyKind, RuntimeFunctionId, Step, StepKind, TypedListExpr, TypedListExprKind,
    };
    use crate::plan::module::{GenericListReturn, ParameterListListReturn};
    use std::collections::HashSet;

    fn nested_list_lowering_context(substitution: crate::plan::ValueShape) -> LoweringContext {
        let parameter = crate::plan::TypeParameterId(0);
        let main_id = crate::plan::FunctionTemplateId::new(0);
        let main = crate::plan::FunctionTemplate::new(
            main_id,
            "main".into(),
            Vec::new(),
            Vec::new(),
            crate::plan::ReturnExpr::int(
                crate::plan::IntFunctionId(0),
                crate::plan::IntExpr::value(0.into()),
            ),
        );

        let generic_list_shape = crate::plan::FunctionShape::new(
            Vec::new(),
            crate::plan::ValueShape::List(Box::new(crate::plan::ValueShape::Parameter(parameter))),
        );
        let generic_list_signature = crate::plan::FunctionTemplateSignature::new(
            crate::plan::FunctionTemplateId::new(1),
            crate::plan::TypeScheme::new(1),
            generic_list_shape,
        );
        let generic_list = crate::plan::ListExpr::panic(
            crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
            crate::ValueType::Parameter(parameter),
        )
        .into_generic()
        .expect("a parameter item type should create a generic-list expression");
        let generic_list_template = crate::plan::FunctionTemplate::from_signature(
            generic_list_signature,
            "generic_list".into(),
            Vec::new(),
            Vec::new(),
            crate::plan::ReturnExpr::generic_list_body(
                parameter,
                GenericListReturn::expr(generic_list),
            ),
        );

        let parameter_list_shape = crate::plan::FunctionShape::new(
            Vec::new(),
            crate::plan::ValueShape::List(Box::new(crate::plan::ValueShape::List(Box::new(
                crate::plan::ValueShape::Parameter(parameter),
            )))),
        );
        let parameter_list_signature = crate::plan::FunctionTemplateSignature::new(
            crate::plan::FunctionTemplateId::new(2),
            crate::plan::TypeScheme::new(1),
            parameter_list_shape,
        );
        let parameter_list = crate::plan::ListExpr::panic(
            crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
            crate::ValueType::List(Box::new(crate::ValueType::Parameter(parameter))),
        )
        .into_parameter_list()
        .expect("a parameter-list item type should create a nested-list expression");
        let parameter_list_template = crate::plan::FunctionTemplate::from_signature(
            parameter_list_signature,
            "parameter_list".into(),
            Vec::new(),
            Vec::new(),
            crate::plan::ReturnExpr::parameter_list_list_body(
                parameter,
                ParameterListListReturn::expr(parameter_list),
            ),
        );

        let templates = FunctionTemplates::new(
            main,
            vec![generic_list_template, parameter_list_template],
            Vec::new(),
        );
        let mut context = LoweringContext::new(
            &templates,
            SpecializationKey::monomorphic(main_id),
            RepresentationContext::new(Vec::new()),
            crate::plan::ConstantTemplates::from_entries(Vec::new()),
            HashSet::new(),
        );
        context.substitution = SpecializedTypeSubstitution::instantiate(
            &crate::plan::TypeSubstitution::from_arguments(vec![substitution]),
            &SpecializedTypeSubstitution::empty(),
        );
        context
    }

    fn generic_list_instantiation() -> crate::plan::FunctionInstantiation {
        let parameter = crate::plan::TypeParameterId(0);
        crate::plan::FunctionTemplateSignature::new(
            crate::plan::FunctionTemplateId::new(1),
            crate::plan::TypeScheme::new(1),
            crate::plan::FunctionShape::new(
                Vec::new(),
                crate::plan::ValueShape::List(Box::new(crate::plan::ValueShape::Parameter(
                    parameter,
                ))),
            ),
        )
        .identity_instantiation()
    }

    fn parameter_list_instantiation() -> crate::plan::FunctionInstantiation {
        let parameter = crate::plan::TypeParameterId(0);
        crate::plan::FunctionTemplateSignature::new(
            crate::plan::FunctionTemplateId::new(2),
            crate::plan::TypeScheme::new(1),
            crate::plan::FunctionShape::new(
                Vec::new(),
                crate::plan::ValueShape::List(Box::new(crate::plan::ValueShape::List(Box::new(
                    crate::plan::ValueShape::Parameter(parameter),
                )))),
            ),
        )
        .identity_instantiation()
    }

    #[test]
    fn lowering_specializes_parameter_nested_lists_without_generic_runtime_payloads() {
        let parameter = crate::plan::TypeParameterId(0);
        let mut concrete_context = nested_list_lowering_context(crate::plan::ValueShape::Int);
        let concrete_value = crate::plan::ListExpr::panic(
            crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
            crate::ValueType::List(Box::new(crate::ValueType::Parameter(parameter))),
        )
        .into_parameter_list()
        .expect("a parameter-list item type should create a nested-list expression");
        assert_eq!(
            super::parameter_list_list_expr(&concrete_value, &mut concrete_context).map(|_| ()),
            Representability::Inhabited(()),
        );

        let concrete_call = crate::plan::ListExpr::call(
            parameter_list_instantiation(),
            Vec::new(),
            crate::plan::ValueShape::List(Box::new(crate::plan::ValueShape::Parameter(parameter))),
        )
        .into_parameter_list()
        .expect("a parameter-list item shape should create a nested-list expression");
        assert_eq!(
            super::concrete_parameter_list_list_expr(
                &concrete_call,
                &StoredValueShape::Int,
                &mut concrete_context,
            )
            .map(|_| ()),
            Representability::Inhabited(()),
        );

        let mut symbolic_context = nested_list_lowering_context(crate::plan::ValueShape::List(
            Box::new(crate::plan::ValueShape::Parameter(parameter)),
        ));
        let symbolic_call = crate::plan::ListExpr::call(
            generic_list_instantiation(),
            Vec::new(),
            crate::plan::ValueShape::Parameter(parameter),
        )
        .into_generic()
        .expect("a parameter item shape should create a generic-list expression");
        assert_eq!(
            super::generic_nested_list_expr(
                &symbolic_call,
                &SpecializedValueShape::Parameter(parameter),
                &mut symbolic_context,
            )
            .map(|_| ()),
            Representability::Inhabited(()),
        );

        let symbolic_element = crate::plan::Expr::generic(crate::plan::GenericExpr::panic(
            parameter,
            crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
        ));
        let symbolic_spread = crate::plan::ListExpr::spread(
            vec![symbolic_element.clone(), symbolic_element],
            crate::plan::ListExpr::Generic(symbolic_call.clone()),
            crate::ValueType::Parameter(parameter),
        )
        .into_generic()
        .expect("a generic spread should preserve its parameter item");
        assert_eq!(
            super::generic_nested_list_expr(
                &symbolic_spread,
                &SpecializedValueShape::Parameter(parameter),
                &mut symbolic_context,
            )
            .map(|_| ()),
            Representability::Inhabited(()),
        );

        let symbolic_drop = crate::plan::ListExpr::drop_first(
            crate::plan::ListExpr::Generic(symbolic_call.clone()),
            1,
        )
        .into_generic()
        .expect("dropping a generic list should preserve its parameter item");
        assert_eq!(
            super::generic_nested_list_expr(
                &symbolic_drop,
                &SpecializedValueShape::Parameter(parameter),
                &mut symbolic_context,
            )
            .map(|_| ()),
            Representability::Inhabited(()),
        );

        let mut parameter_context =
            nested_list_lowering_context(crate::plan::ValueShape::Parameter(parameter));
        assert_eq!(
            super::parameter_list_expr(&symbolic_drop, parameter, &mut parameter_context)
                .map(|_| ()),
            Representability::Inhabited(()),
        );

        let mut stored_context = nested_list_lowering_context(crate::plan::ValueShape::Int);
        assert_eq!(
            super::generic_nested_list_expr(
                &symbolic_call,
                &SpecializedValueShape::Int,
                &mut stored_context,
            )
            .map(|_| ()),
            Representability::Inhabited(()),
        );
    }

    #[test]
    fn lowering_derives_nested_list_index_result_from_parent_type() {
        let source = r#"
pub fn main() {
  case [[1]] {
    [first, ..] -> first
    _ -> []
  }
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let main = expect_int_list_main(&plan);
        let (_, return_) = expect_block(plan.int_list_function(main).return_());
        let true_ = expect_bool_case_true(return_);
        let (steps, _) = expect_block(true_);
        let value = expect_int_list_binding(&steps[0]);
        let source = expect_list_index(value);

        assert_eq!(
            source.list().item().type_id().item_type(),
            value.item().type_id().list_type(),
        );
        assert_eq!(source.index(), 0);
    }

    #[test]
    #[should_panic(expected = "expected a List(Int) main function")]
    fn int_list_main_fixture_guard_rejects_int_main() {
        let plan = execution_plan("pub fn main() { 1 }");
        let _ = expect_int_list_main(&plan);
    }

    #[test]
    #[should_panic(expected = "expected a block return body")]
    fn block_return_fixture_guard_rejects_expression_return() {
        let plan = execution_plan("pub fn main() -> List(Int) { [] }");
        let main = expect_int_list_main(&plan);
        let _ = expect_block(plan.int_list_function(main).return_());
    }

    #[test]
    #[should_panic(expected = "expected a Bool case return body")]
    fn bool_case_fixture_guard_rejects_expression_return() {
        let plan = execution_plan("pub fn main() -> List(Int) { [] }");
        let main = expect_int_list_main(&plan);
        let _ = expect_bool_case_true(plan.int_list_function(main).return_());
    }

    #[test]
    #[should_panic(expected = "expected a List(Int) binding step")]
    fn int_list_binding_fixture_guard_rejects_int_binding() {
        let plan = execution_plan("pub fn main() -> List(Int) { let value = 1 [] }");
        let main = expect_int_list_main(&plan);
        let _ = expect_int_list_binding(&plan.int_list_function(main).steps()[0]);
    }

    #[test]
    #[should_panic(expected = "expected a list-index expression")]
    fn list_index_fixture_guard_rejects_list_value() {
        let plan = execution_plan("pub fn main() -> List(Int) { [] }");
        let main = expect_int_list_main(&plan);
        let value = expect_expression(plan.int_list_function(main).return_());
        let _ = expect_list_index(value);
    }

    #[test]
    #[should_panic(expected = "expected an expression return body")]
    fn expression_return_fixture_guard_rejects_block_return() {
        let plan = execution_plan(
            r#"
pub fn main() {
  case [[1]] {
    [first, ..] -> first
    _ -> []
  }
}
"#,
        );
        let main = expect_int_list_main(&plan);
        let _ = expect_expression(plan.int_list_function(main).return_());
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }

    fn expect_int_list_main(plan: &ExecutionPlan) -> IntListFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::List(ListFunctionId::Int(main)) => main,
            _ => panic!("expected a List(Int) main function"),
        }
    }

    fn expect_block<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> (&[Step], &ReturnBody<Expression, Function>) {
        match body.kind() {
            ReturnBodyKind::Block { steps, return_ } => (steps, return_),
            _ => panic!("expected a block return body"),
        }
    }

    fn expect_bool_case_true<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> &ReturnBody<Expression, Function> {
        match body.kind() {
            ReturnBodyKind::BoolCase { true_, .. } => true_,
            _ => panic!("expected a Bool case return body"),
        }
    }

    fn expect_int_list_binding(step: &Step) -> &TypedListExpr<crate::plan::execution::IntListItem> {
        match step.kind() {
            StepKind::LetList {
                value: ListLocalExpr::Int { value, .. },
            } => value,
            _ => panic!("expected a List(Int) binding step"),
        }
    }

    fn expect_list_index<Item: ListItem>(
        expression: &TypedListExpr<Item>,
    ) -> &crate::plan::execution::ListIndexSource<Item> {
        match expression.kind() {
            TypedListExprKind::ListIndex(source) => source,
            _ => panic!("expected a list-index expression"),
        }
    }

    fn expect_expression<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> &Expression {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => expression,
            _ => panic!("expected an expression return body"),
        }
    }
}
