use super::LoweringContext;
use super::expression::{
    bit_array_expr, bit_array_function_expr, bit_array_list_expr, bool_expr, bool_function_expr,
    bool_list_expr, concrete_parameter_list_list_expr, custom_expr_kind, custom_function_expr_kind,
    custom_list_expr, direct_call, float_expr, float_function_expr, float_list_expr,
    function_function_expr_kind, function_list_expr, generic_bit_array_expr,
    generic_bit_array_function_expr, generic_bit_array_list_expr, generic_bool_expr,
    generic_bool_function_expr, generic_bool_list_expr, generic_custom_expr_kind,
    generic_custom_function_expr_kind, generic_custom_list_expr, generic_float_expr,
    generic_float_function_expr, generic_float_list_expr, generic_function_function_expr_kind,
    generic_function_list_expr, generic_int_expr, generic_int_function_expr, generic_int_list_expr,
    generic_list_function_expr, generic_never_function_expr, generic_nil_expr,
    generic_nil_function_expr, generic_nil_list_expr, generic_parameter_list_list_expr,
    generic_stored_nested_list_expr, generic_string_expr, generic_string_function_expr,
    generic_string_list_expr, generic_symbolic_function_expr, generic_tuple_expr,
    generic_tuple_function_expr, generic_tuple_list_expr, generic_utf_codepoint_expr,
    generic_utf_codepoint_function_expr, generic_utf_codepoint_list_expr,
    generic_value_bit_array_function_expr, generic_value_bit_array_list_expr,
    generic_value_bool_function_expr, generic_value_bool_list_expr,
    generic_value_custom_function_expr_kind, generic_value_custom_list_expr,
    generic_value_float_function_expr, generic_value_float_list_expr,
    generic_value_function_function_expr_kind, generic_value_function_list_expr,
    generic_value_generic_function_expr, generic_value_int_function_expr,
    generic_value_int_list_expr, generic_value_list_function_expr,
    generic_value_never_function_expr, generic_value_nil_function_expr,
    generic_value_nil_list_expr, generic_value_parameter_list_list_expr,
    generic_value_stored_nested_list_expr, generic_value_string_function_expr,
    generic_value_string_list_expr, generic_value_tuple_function_expr,
    generic_value_tuple_list_expr, generic_value_utf_codepoint_function_expr,
    generic_value_utf_codepoint_list_expr, int_expr, int_function_expr, int_list_expr,
    list_function_expr, list_list_expr, lower_bool_subject, nil_expr, nil_function_expr,
    nil_list_expr, parameter_list_expr, parameter_list_value_expr, string_expr,
    string_function_expr, string_list_expr, tuple_expr, tuple_function_expr, tuple_list_expr,
    unresolved_parameter_list_list_expr, utf_codepoint_expr, utf_codepoint_function_expr,
    utf_codepoint_list_expr,
};
use super::specialization::{Representability, SpecializedValueShape};
use crate::plan::{execution, module};

pub(super) fn never_return(
    body: &module::GenericReturn,
    context: &mut LoweringContext,
) -> Representability<execution::NeverReturn> {
    never_return_graph(body, context, |expression, context| {
        super::expression::never_expr(expression, context)
    })
}

pub(super) fn tuple_never_return(
    body: &module::TupleReturn,
    proof: &super::specialization::UninhabitedTupleValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::NeverReturn> {
    never_return_graph(body, context, |expression, context| {
        super::expression::tuple_never_expr(expression, proof, context)
    })
}

pub(super) fn custom_never_return(
    body: &module::CustomReturn,
    proof: &super::specialization::UninhabitedCustomValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::NeverReturn> {
    never_return_graph(body.body(), context, |kind, context| {
        super::expression::custom_never_expr_kind(kind, proof, context)
    })
}

fn never_return_graph<ModuleExpression>(
    body: &module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    context: &mut LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        &mut LoweringContext,
    ) -> Representability<execution::NeverExpr>,
) -> Representability<execution::NeverReturn> {
    return_graph(body, context, lower_expression, |function, context| {
        context.never_function_id(function)
    })
}

macro_rules! generic_primitive_return {
    ($lower:ident, $return:ty, $expression:ident, $function:ident) => {
        pub(super) fn $lower(
            body: &module::GenericReturn,
            context: &mut LoweringContext,
        ) -> Representability<$return> {
            return_graph(body, context, $expression, |function, context| {
                context.$function(function)
            })
        }
    };
}

generic_primitive_return!(
    generic_int_return,
    execution::IntReturn,
    generic_int_expr,
    int_function_id
);
generic_primitive_return!(
    generic_float_return,
    execution::FloatReturn,
    generic_float_expr,
    float_function_id
);
generic_primitive_return!(
    generic_string_return,
    execution::StringReturn,
    generic_string_expr,
    string_function_id
);
generic_primitive_return!(
    generic_bit_array_return,
    execution::BitArrayReturn,
    generic_bit_array_expr,
    bit_array_function_id
);
generic_primitive_return!(
    generic_utf_codepoint_return,
    execution::UtfCodepointReturn,
    generic_utf_codepoint_expr,
    utf_codepoint_function_id
);
generic_primitive_return!(
    generic_bool_return,
    execution::BoolReturn,
    generic_bool_expr,
    bool_function_id
);
generic_primitive_return!(
    generic_nil_return,
    execution::NilReturn,
    generic_nil_expr,
    nil_function_id
);

pub(super) fn generic_custom_return(
    body: &module::GenericReturn,
    shape: &super::specialization::SpecializedCustomValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::CustomReturn> {
    let lowered_shape = context.lower_concrete_custom_shape(shape);
    let body = return_graph(
        body,
        context,
        |expression, context| generic_custom_expr_kind(expression, shape, context),
        |function, context| {
            context
                .custom_function_id(function, shape)
                .map(|function| function.index())
        },
    );
    body.map(|body| execution::CustomReturn::from_parts(lowered_shape, lowered_shape, body))
}

pub(super) fn generic_tuple_return(
    body: &module::GenericReturn,
    elements: &[super::specialization::SpecializedValueShape],
    context: &mut LoweringContext,
) -> Representability<execution::TupleReturn> {
    return_graph(
        body,
        context,
        |expression, context| generic_tuple_expr(expression, elements, context),
        |function, context| context.tuple_function_id(function),
    )
}

macro_rules! generic_value_primitive_list_return {
    ($lower:ident, $return:ty, $expression:ident, $function:ident) => {
        pub(super) fn $lower(
            body: &module::GenericReturn,
            context: &mut LoweringContext,
        ) -> Representability<$return> {
            return_graph(body, context, $expression, |function, context| {
                context.$function(function)
            })
        }
    };
}

generic_value_primitive_list_return!(
    generic_value_int_list_return,
    execution::IntListReturn,
    generic_value_int_list_expr,
    int_list_function_id
);
generic_value_primitive_list_return!(
    generic_value_string_list_return,
    execution::StringListReturn,
    generic_value_string_list_expr,
    string_list_function_id
);
generic_value_primitive_list_return!(
    generic_value_bit_array_list_return,
    execution::BitArrayListReturn,
    generic_value_bit_array_list_expr,
    bit_array_list_function_id
);
generic_value_primitive_list_return!(
    generic_value_utf_codepoint_list_return,
    execution::UtfCodepointListReturn,
    generic_value_utf_codepoint_list_expr,
    utf_codepoint_list_function_id
);
generic_value_primitive_list_return!(
    generic_value_float_list_return,
    execution::FloatListReturn,
    generic_value_float_list_expr,
    float_list_function_id
);
generic_value_primitive_list_return!(
    generic_value_bool_list_return,
    execution::BoolListReturn,
    generic_value_bool_list_expr,
    bool_list_function_id
);
generic_value_primitive_list_return!(
    generic_value_nil_list_return,
    execution::NilListReturn,
    generic_value_nil_list_expr,
    nil_list_function_id
);

pub(super) fn generic_value_custom_list_return(
    body: &module::GenericReturn,
    shape: &super::specialization::SpecializedCustomValueShape,
    type_id: execution::CustomListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::CustomListReturn> {
    return_graph(
        body,
        context,
        |expression, context| generic_value_custom_list_expr(expression, shape, context),
        move |function, context| context.custom_list_function_id(function, type_id),
    )
}

pub(super) fn generic_value_tuple_list_return(
    body: &module::GenericReturn,
    elements: &[super::specialization::SpecializedValueShape],
    type_id: execution::TupleListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::TupleListReturn> {
    return_graph(
        body,
        context,
        |expression, context| generic_value_tuple_list_expr(expression, elements, context),
        move |function, context| context.tuple_list_function_id(function, type_id),
    )
}

pub(super) fn generic_value_nested_list_return(
    body: &module::GenericReturn,
    item: &super::specialization::StoredValueShape,
    type_id: execution::ListListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::ListListReturn> {
    return_graph(
        body,
        context,
        |expression, context| generic_value_stored_nested_list_expr(expression, item, context),
        move |function, context| context.list_list_function_id(function, type_id),
    )
}

pub(super) fn generic_value_parameter_list_return(
    body: &module::GenericReturn,
    parameter: crate::plan::TypeParameterId,
    context: &mut LoweringContext,
) -> Representability<execution::ParameterListReturn> {
    return_graph(
        body,
        context,
        |expression, context| parameter_list_value_expr(expression, parameter, context),
        move |function, context| context.parameter_list_function_id(function, parameter),
    )
}

pub(super) fn generic_value_parameter_list_list_return(
    body: &module::GenericReturn,
    parameter: crate::plan::TypeParameterId,
    type_id: execution::ParameterListListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::ParameterListListReturn> {
    return_graph(
        body,
        context,
        |expression, context| {
            generic_value_parameter_list_list_expr(expression, parameter, context)
        },
        move |function, context| context.parameter_list_list_function_id(function, type_id),
    )
}

pub(super) fn generic_value_function_list_return(
    body: &module::GenericReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    type_id: execution::FunctionListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::FunctionListReturn> {
    return_graph(
        body,
        context,
        |expression, context| generic_value_function_list_expr(expression, function_shape, context),
        move |function, context| context.function_list_function_id(function, type_id),
    )
}

macro_rules! generic_item_primitive_list_return {
    ($lower:ident, $return:ty, $expression:ident, $function:ident) => {
        pub(super) fn $lower(
            body: &module::GenericListReturn,
            context: &mut LoweringContext,
        ) -> Representability<$return> {
            return_graph(body, context, $expression, |function, context| {
                context.$function(function)
            })
        }
    };
}

generic_item_primitive_list_return!(
    generic_item_int_list_return,
    execution::IntListReturn,
    generic_int_list_expr,
    int_list_function_id
);
generic_item_primitive_list_return!(
    generic_item_string_list_return,
    execution::StringListReturn,
    generic_string_list_expr,
    string_list_function_id
);
generic_item_primitive_list_return!(
    generic_item_bit_array_list_return,
    execution::BitArrayListReturn,
    generic_bit_array_list_expr,
    bit_array_list_function_id
);
generic_item_primitive_list_return!(
    generic_item_utf_codepoint_list_return,
    execution::UtfCodepointListReturn,
    generic_utf_codepoint_list_expr,
    utf_codepoint_list_function_id
);
generic_item_primitive_list_return!(
    generic_item_float_list_return,
    execution::FloatListReturn,
    generic_float_list_expr,
    float_list_function_id
);
generic_item_primitive_list_return!(
    generic_item_bool_list_return,
    execution::BoolListReturn,
    generic_bool_list_expr,
    bool_list_function_id
);
generic_item_primitive_list_return!(
    generic_item_nil_list_return,
    execution::NilListReturn,
    generic_nil_list_expr,
    nil_list_function_id
);

pub(super) fn generic_item_custom_list_return(
    body: &module::GenericListReturn,
    shape: &super::specialization::SpecializedCustomValueShape,
    type_id: execution::CustomListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::CustomListReturn> {
    return_graph(
        body,
        context,
        |expression, context| generic_custom_list_expr(expression, shape, context),
        move |function, context| context.custom_list_function_id(function, type_id),
    )
}

pub(super) fn generic_item_tuple_list_return(
    body: &module::GenericListReturn,
    elements: &[super::specialization::SpecializedValueShape],
    type_id: execution::TupleListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::TupleListReturn> {
    return_graph(
        body,
        context,
        |expression, context| generic_tuple_list_expr(expression, elements, context),
        move |function, context| context.tuple_list_function_id(function, type_id),
    )
}

pub(super) fn generic_item_nested_list_return(
    body: &module::GenericListReturn,
    item: &super::specialization::StoredValueShape,
    type_id: execution::ListListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::ListListReturn> {
    return_graph(
        body,
        context,
        |expression, context| generic_stored_nested_list_expr(expression, item, context),
        move |function, context| context.list_list_function_id(function, type_id),
    )
}

pub(super) fn generic_item_parameter_list_return(
    body: &module::GenericListReturn,
    parameter: crate::plan::TypeParameterId,
    context: &mut LoweringContext,
) -> Representability<execution::ParameterListReturn> {
    return_graph(
        body,
        context,
        |expression, context| parameter_list_expr(expression, parameter, context),
        move |function, context| context.parameter_list_function_id(function, parameter),
    )
}

pub(super) fn generic_item_parameter_list_list_return(
    body: &module::GenericListReturn,
    parameter: crate::plan::TypeParameterId,
    type_id: execution::ParameterListListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::ParameterListListReturn> {
    return_graph(
        body,
        context,
        |expression, context| generic_parameter_list_list_expr(expression, parameter, context),
        move |function, context| context.parameter_list_list_function_id(function, type_id),
    )
}

pub(super) fn generic_item_function_list_return(
    body: &module::GenericListReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    type_id: execution::FunctionListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::FunctionListReturn> {
    return_graph(
        body,
        context,
        |expression, context| generic_function_list_expr(expression, function_shape, context),
        move |function, context| context.function_list_function_id(function, type_id),
    )
}

macro_rules! generic_value_primitive_function_return {
    ($lower:ident, $return:ty, $expression:ident, $function:ident) => {
        pub(super) fn $lower(
            body: &module::GenericReturn,
            function_shape: &super::specialization::SpecializedFunctionShape,
            context: &mut LoweringContext,
        ) -> Representability<$return> {
            let lowered = return_graph(
                body,
                context,
                |expression, context| $expression(expression, function_shape, context),
                |function, context| context.$function(function),
            );
            lowered.map(|lowered| {
                execution::TypedFunctionReturn::new(
                    context.lower_concrete_function_shape(function_shape),
                    lowered,
                )
            })
        }
    };
}

generic_value_primitive_function_return!(
    generic_value_int_function_return,
    execution::IntFunctionReturn,
    generic_value_int_function_expr,
    int_function_function_id
);
generic_value_primitive_function_return!(
    generic_value_float_function_return,
    execution::FloatFunctionReturn,
    generic_value_float_function_expr,
    float_function_function_id
);
generic_value_primitive_function_return!(
    generic_value_string_function_return,
    execution::StringFunctionReturn,
    generic_value_string_function_expr,
    string_function_function_id
);
generic_value_primitive_function_return!(
    generic_value_bit_array_function_return,
    execution::BitArrayFunctionReturn,
    generic_value_bit_array_function_expr,
    bit_array_function_function_id
);
generic_value_primitive_function_return!(
    generic_value_utf_codepoint_function_return,
    execution::UtfCodepointFunctionReturn,
    generic_value_utf_codepoint_function_expr,
    utf_codepoint_function_function_id
);
generic_value_primitive_function_return!(
    generic_value_bool_function_return,
    execution::BoolFunctionReturn,
    generic_value_bool_function_expr,
    bool_function_function_id
);
generic_value_primitive_function_return!(
    generic_value_nil_function_return,
    execution::NilFunctionReturn,
    generic_value_nil_function_expr,
    nil_function_function_id
);

pub(super) fn generic_value_tuple_function_return(
    body: &module::GenericReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::TupleFunctionReturn> {
    let lowered = return_graph(
        body,
        context,
        |expression, context| {
            generic_value_tuple_function_expr(expression, function_shape, context)
        },
        |function, context| context.tuple_function_function_id(function),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

pub(super) fn generic_value_custom_function_return(
    body: &module::GenericReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    return_shape: &super::specialization::SpecializedCustomValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::CustomFunctionReturn> {
    let type_ = context.specialized_custom_function_type(function_shape.arguments(), return_shape);
    let lowered = return_graph(
        body,
        context,
        |expression, context| {
            generic_value_custom_function_expr_kind(expression, function_shape, &type_, context)
        },
        |function, context| {
            context
                .custom_function_function_id(function, type_.clone())
                .map(|function| function.index())
        },
    );
    let shape = context.lower_concrete_function_shape(function_shape);
    lowered.map(|lowered| execution::CustomFunctionReturn::from_parts(shape, type_, lowered))
}

pub(super) fn generic_value_list_function_return(
    body: &module::GenericReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    item: &super::specialization::SpecializedValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::ListFunctionReturn> {
    let lowered = return_graph(
        body,
        context,
        |expression, context| {
            generic_value_list_function_expr(expression, function_shape, item, context)
        },
        |function, context| context.list_function_function_id(function, function_shape, item),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

pub(super) fn generic_value_function_function_return(
    body: &module::GenericReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    return_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::FunctionFunctionReturn> {
    let type_ =
        context.specialized_function_function_type(function_shape.arguments(), return_shape);
    let lowered = return_graph(
        body,
        context,
        |expression, context| {
            generic_value_function_function_expr_kind(expression, function_shape, &type_, context)
        },
        |function, context| {
            context
                .function_function_function_id(function, type_.clone())
                .map(|function| function.index())
        },
    );
    let shape = context.lower_concrete_function_shape(function_shape);
    lowered.map(|lowered| execution::FunctionFunctionReturn::from_parts(shape, type_, lowered))
}

pub(super) fn generic_value_generic_function_return(
    body: &module::GenericReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::GenericFunctionReturn> {
    let type_ = context.generic_function_type(function_shape);
    let lowered = return_graph(
        body,
        context,
        |expression, context| {
            generic_value_generic_function_expr(expression, function_shape, context)
        },
        |function, context| context.generic_function_function_id(function, type_.clone()),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

pub(super) fn generic_value_never_function_return(
    body: &module::GenericReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::NeverFunctionReturn> {
    let type_ = context.generic_function_type(function_shape);
    let lowered = return_graph(
        body,
        context,
        |expression, context| {
            generic_value_never_function_expr(expression, function_shape, context)
        },
        |function, context| context.never_function_function_id(function, type_.clone()),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

macro_rules! generic_result_primitive_function_return {
    ($lower:ident, $return:ty, $expression:ident, $function:ident) => {
        pub(super) fn $lower(
            body: &module::GenericFunctionReturn,
            function_shape: &super::specialization::SpecializedFunctionShape,
            context: &mut LoweringContext,
        ) -> Representability<$return> {
            let lowered = return_graph(body, context, $expression, |function, context| {
                context.$function(function)
            });
            lowered.map(|lowered| {
                execution::TypedFunctionReturn::new(
                    context.lower_concrete_function_shape(function_shape),
                    lowered,
                )
            })
        }
    };
}

generic_result_primitive_function_return!(
    generic_result_int_function_return,
    execution::IntFunctionReturn,
    generic_int_function_expr,
    int_function_function_id
);
generic_result_primitive_function_return!(
    generic_result_float_function_return,
    execution::FloatFunctionReturn,
    generic_float_function_expr,
    float_function_function_id
);
generic_result_primitive_function_return!(
    generic_result_string_function_return,
    execution::StringFunctionReturn,
    generic_string_function_expr,
    string_function_function_id
);
generic_result_primitive_function_return!(
    generic_result_bit_array_function_return,
    execution::BitArrayFunctionReturn,
    generic_bit_array_function_expr,
    bit_array_function_function_id
);
generic_result_primitive_function_return!(
    generic_result_utf_codepoint_function_return,
    execution::UtfCodepointFunctionReturn,
    generic_utf_codepoint_function_expr,
    utf_codepoint_function_function_id
);
generic_result_primitive_function_return!(
    generic_result_bool_function_return,
    execution::BoolFunctionReturn,
    generic_bool_function_expr,
    bool_function_function_id
);
generic_result_primitive_function_return!(
    generic_result_nil_function_return,
    execution::NilFunctionReturn,
    generic_nil_function_expr,
    nil_function_function_id
);

pub(super) fn generic_result_tuple_function_return(
    body: &module::GenericFunctionReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::TupleFunctionReturn> {
    let lowered = return_graph(
        body,
        context,
        generic_tuple_function_expr,
        |function, context| context.tuple_function_function_id(function),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

pub(super) fn generic_result_custom_function_return(
    body: &module::GenericFunctionReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    return_shape: &super::specialization::SpecializedCustomValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::CustomFunctionReturn> {
    let type_ = context.specialized_custom_function_type(function_shape.arguments(), return_shape);
    let lowered = return_graph(
        body,
        context,
        |expression, context| {
            generic_custom_function_expr_kind(expression, return_shape, &type_, context)
        },
        |function, context| {
            context
                .custom_function_function_id(function, type_.clone())
                .map(|function| function.index())
        },
    );
    let shape = context.lower_concrete_function_shape(function_shape);
    lowered.map(|lowered| execution::CustomFunctionReturn::from_parts(shape, type_, lowered))
}

pub(super) fn generic_result_list_function_return(
    body: &module::GenericFunctionReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    item: &super::specialization::SpecializedValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::ListFunctionReturn> {
    let lowered = return_graph(
        body,
        context,
        |expression, context| generic_list_function_expr(expression, item, context),
        |function, context| context.list_function_function_id(function, function_shape, item),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

pub(super) fn generic_result_function_function_return(
    body: &module::GenericFunctionReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    return_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::FunctionFunctionReturn> {
    let type_ =
        context.specialized_function_function_type(function_shape.arguments(), return_shape);
    let lowered = return_graph(
        body,
        context,
        |expression, context| {
            generic_function_function_expr_kind(expression, return_shape, &type_, context)
        },
        |function, context| {
            context
                .function_function_function_id(function, type_.clone())
                .map(|function| function.index())
        },
    );
    let shape = context.lower_concrete_function_shape(function_shape);
    lowered.map(|lowered| execution::FunctionFunctionReturn::from_parts(shape, type_, lowered))
}

pub(super) fn generic_result_generic_function_return(
    body: &module::GenericFunctionReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::GenericFunctionReturn> {
    let type_ = context.generic_function_type(function_shape);
    let lowered = return_graph(
        body,
        context,
        generic_symbolic_function_expr,
        |function, context| context.generic_function_function_id(function, type_.clone()),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

pub(super) fn generic_result_never_function_return(
    body: &module::GenericFunctionReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::NeverFunctionReturn> {
    let type_ = context.generic_function_type(function_shape);
    let lowered = return_graph(
        body,
        context,
        generic_never_function_expr,
        |function, context| context.never_function_function_id(function, type_.clone()),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

pub(super) fn int_return(
    body: &module::IntReturn,
    context: &mut LoweringContext,
) -> Representability<execution::IntReturn> {
    return_graph(body, context, int_expr, |function, context| {
        context.int_function_id(function)
    })
}

pub(super) fn float_return(
    body: &module::FloatReturn,
    context: &mut LoweringContext,
) -> Representability<execution::FloatReturn> {
    return_graph(body, context, float_expr, |function, context| {
        context.float_function_id(function)
    })
}

pub(super) fn string_return(
    body: &module::StringReturn,
    context: &mut LoweringContext,
) -> Representability<execution::StringReturn> {
    return_graph(body, context, string_expr, |function, context| {
        context.string_function_id(function)
    })
}

pub(super) fn bit_array_return(
    body: &module::BitArrayReturn,
    context: &mut LoweringContext,
) -> Representability<execution::BitArrayReturn> {
    return_graph(body, context, bit_array_expr, |function, context| {
        context.bit_array_function_id(function)
    })
}

pub(super) fn utf_codepoint_return(
    body: &module::UtfCodepointReturn,
    context: &mut LoweringContext,
) -> Representability<execution::UtfCodepointReturn> {
    return_graph(body, context, utf_codepoint_expr, |function, context| {
        context.utf_codepoint_function_id(function)
    })
}

pub(super) fn custom_return(
    body: &module::CustomReturn,
    context: &mut LoweringContext,
) -> Representability<execution::CustomReturn> {
    let signature_shape = context.concrete_custom_value_shape(body.signature_shape());
    let body_shape = context.concrete_custom_value_shape(body.shape());
    let lowered_signature_shape = context.lower_concrete_custom_shape(&signature_shape);
    let lowered_body_shape = context.lower_concrete_custom_shape(&body_shape);
    let body = match context.representations.custom_inhabitation(&body_shape) {
        super::specialization::CompoundInhabitation::Inhabited => return_graph(
            body.body(),
            context,
            |kind, context| custom_expr_kind(kind, &body_shape, context),
            |function, context| {
                context
                    .custom_function_id(function, &signature_shape)
                    .map(|function| function.index())
            },
        ),
        super::specialization::CompoundInhabitation::Uninhabited(proof) => return_graph(
            body.body(),
            context,
            |kind, context| {
                super::expression::custom_never_expr_kind(kind, &proof, context)
                    .map(execution::CustomExprKind::Never)
            },
            |function, context| {
                context
                    .custom_function_id(function, &signature_shape)
                    .map(|function| function.index())
            },
        ),
    };
    body.map(|body| {
        execution::CustomReturn::from_parts(lowered_signature_shape, lowered_body_shape, body)
    })
}

pub(super) fn bool_return(
    body: &module::BoolReturn,
    context: &mut LoweringContext,
) -> Representability<execution::BoolReturn> {
    return_graph(body, context, bool_expr, |function, context| {
        context.bool_function_id(function)
    })
}

pub(super) fn nil_return(
    body: &module::NilReturn,
    context: &mut LoweringContext,
) -> Representability<execution::NilReturn> {
    return_graph(body, context, nil_expr, |function, context| {
        context.nil_function_id(function)
    })
}

pub(super) fn tuple_return(
    body: &module::TupleReturn,
    context: &mut LoweringContext,
) -> Representability<execution::TupleReturn> {
    return_graph(body, context, tuple_expr, |function, context| {
        context.tuple_function_id(function)
    })
}
pub(super) fn int_list_return(
    body: &module::IntListReturn,
    context: &mut LoweringContext,
) -> Representability<execution::IntListReturn> {
    return_graph(body, context, int_list_expr, |function, context| {
        context.int_list_function_id(function)
    })
}

pub(super) fn string_list_return(
    body: &module::StringListReturn,
    context: &mut LoweringContext,
) -> Representability<execution::StringListReturn> {
    return_graph(body, context, string_list_expr, |function, context| {
        context.string_list_function_id(function)
    })
}

pub(super) fn bit_array_list_return(
    body: &module::BitArrayListReturn,
    context: &mut LoweringContext,
) -> Representability<execution::BitArrayListReturn> {
    return_graph(body, context, bit_array_list_expr, |function, context| {
        context.bit_array_list_function_id(function)
    })
}

pub(super) fn utf_codepoint_list_return(
    body: &module::UtfCodepointListReturn,
    context: &mut LoweringContext,
) -> Representability<execution::UtfCodepointListReturn> {
    return_graph(
        body,
        context,
        utf_codepoint_list_expr,
        |function, context| context.utf_codepoint_list_function_id(function),
    )
}

pub(super) fn custom_list_return(
    body: &module::CustomListReturn,
    type_id: execution::CustomListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::CustomListReturn> {
    return_graph(body, context, custom_list_expr, move |function, context| {
        context.custom_list_function_id(function, type_id)
    })
}

pub(super) fn float_list_return(
    body: &module::FloatListReturn,
    context: &mut LoweringContext,
) -> Representability<execution::FloatListReturn> {
    return_graph(body, context, float_list_expr, |function, context| {
        context.float_list_function_id(function)
    })
}

pub(super) fn bool_list_return(
    body: &module::BoolListReturn,
    context: &mut LoweringContext,
) -> Representability<execution::BoolListReturn> {
    return_graph(body, context, bool_list_expr, |function, context| {
        context.bool_list_function_id(function)
    })
}

pub(super) fn nil_list_return(
    body: &module::NilListReturn,
    context: &mut LoweringContext,
) -> Representability<execution::NilListReturn> {
    return_graph(body, context, nil_list_expr, |function, context| {
        context.nil_list_function_id(function)
    })
}

pub(super) fn tuple_list_return(
    body: &module::TupleListReturn,
    type_id: execution::TupleListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::TupleListReturn> {
    return_graph(body, context, tuple_list_expr, move |function, context| {
        context.tuple_list_function_id(function, type_id)
    })
}

pub(super) fn list_list_return(
    body: &module::ListListReturn,
    type_id: execution::ListListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::ListListReturn> {
    return_graph(body, context, list_list_expr, move |function, context| {
        context.list_list_function_id(function, type_id)
    })
}

pub(super) fn parameter_list_list_return(
    body: &module::ParameterListListReturn,
    parameter: crate::plan::TypeParameterId,
    type_id: execution::ParameterListListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::ParameterListListReturn> {
    return_graph(
        body,
        context,
        |expression, context| unresolved_parameter_list_list_expr(expression, parameter, context),
        move |function, context| context.parameter_list_list_function_id(function, type_id),
    )
}

pub(super) fn stored_parameter_list_list_return(
    body: &module::ParameterListListReturn,
    item: &super::specialization::StoredValueShape,
    type_id: execution::ListListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::ListListReturn> {
    return_graph(
        body,
        context,
        |expression, context| concrete_parameter_list_list_expr(expression, item, context),
        move |function, context| context.list_list_function_id(function, type_id),
    )
}

pub(super) fn function_list_return(
    body: &module::FunctionListReturn,
    type_id: execution::FunctionListTypeId,
    context: &mut LoweringContext,
) -> Representability<execution::FunctionListReturn> {
    return_graph(
        body,
        context,
        function_list_expr,
        move |function, context| context.function_list_function_id(function, type_id),
    )
}
pub(super) fn int_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::IntFunctionReturn,
    context: &mut LoweringContext,
) -> Representability<execution::IntFunctionReturn> {
    let body = return_graph(body, context, int_function_expr, |function, context| {
        context.int_function_function_id(function)
    });
    typed_function_return(shape.clone(), body, context)
}

pub(super) fn float_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::FloatFunctionReturn,
    context: &mut LoweringContext,
) -> Representability<execution::FloatFunctionReturn> {
    let body = return_graph(body, context, float_function_expr, |function, context| {
        context.float_function_function_id(function)
    });
    typed_function_return(shape.clone(), body, context)
}

pub(super) fn string_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::StringFunctionReturn,
    context: &mut LoweringContext,
) -> Representability<execution::StringFunctionReturn> {
    let body = return_graph(body, context, string_function_expr, |function, context| {
        context.string_function_function_id(function)
    });
    typed_function_return(shape.clone(), body, context)
}

pub(super) fn bit_array_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::BitArrayFunctionReturn,
    context: &mut LoweringContext,
) -> Representability<execution::BitArrayFunctionReturn> {
    let body = return_graph(
        body,
        context,
        bit_array_function_expr,
        |function, context| context.bit_array_function_function_id(function),
    );
    typed_function_return(shape.clone(), body, context)
}

pub(super) fn utf_codepoint_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::UtfCodepointFunctionReturn,
    context: &mut LoweringContext,
) -> Representability<execution::UtfCodepointFunctionReturn> {
    let body = return_graph(
        body,
        context,
        utf_codepoint_function_expr,
        |function, context| context.utf_codepoint_function_function_id(function),
    );
    typed_function_return(shape.clone(), body, context)
}

pub(super) fn custom_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::CustomFunctionReturn,
    context: &mut LoweringContext,
) -> Representability<execution::CustomFunctionReturn> {
    let return_shape = context.concrete_custom_value_shape(body.type_().return_());
    let type_ = context.custom_function_type(body.type_().clone());
    let lowered = return_graph(
        body.body(),
        context,
        |kind, context| custom_function_expr_kind(kind, &return_shape, &type_, context),
        |function, context| {
            context
                .custom_function_function_id(function, type_.clone())
                .map(|function| function.index())
        },
    );
    let shape = context.function_shape(shape.clone());
    lowered.map(|lowered| execution::CustomFunctionReturn::from_parts(shape, type_, lowered))
}

pub(super) fn symbolic_custom_function_return(
    body: &module::CustomFunctionReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::GenericFunctionReturn> {
    let type_ = context.generic_function_type(function_shape);
    let lowered = return_graph(
        body.body(),
        context,
        |kind, context| {
            super::expression::symbolic_custom_function_expr_kind(kind, function_shape, context)
                .map(|kind| execution::GenericFunctionExpr::from_parts(type_.clone(), kind))
        },
        |function, context| context.generic_function_function_id(function, type_.clone()),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

pub(super) fn custom_never_function_return(
    body: &module::CustomFunctionReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::NeverFunctionReturn> {
    let type_ = context.generic_function_type(function_shape);
    let lowered = return_graph(
        body.body(),
        context,
        |kind, context| {
            super::expression::custom_never_function_expr_kind(kind, &type_, context)
                .map(|kind| execution::NeverFunctionExpr::from_parts(type_.clone(), kind))
        },
        |function, context| context.never_function_function_id(function, type_.clone()),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

pub(super) fn bool_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::BoolFunctionReturn,
    context: &mut LoweringContext,
) -> Representability<execution::BoolFunctionReturn> {
    let body = return_graph(body, context, bool_function_expr, |function, context| {
        context.bool_function_function_id(function)
    });
    typed_function_return(shape.clone(), body, context)
}

pub(super) fn nil_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::NilFunctionReturn,
    context: &mut LoweringContext,
) -> Representability<execution::NilFunctionReturn> {
    let body = return_graph(body, context, nil_function_expr, |function, context| {
        context.nil_function_function_id(function)
    });
    typed_function_return(shape.clone(), body, context)
}

pub(super) fn tuple_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::TupleFunctionReturn,
    context: &mut LoweringContext,
) -> Representability<execution::TupleFunctionReturn> {
    let body = return_graph(body, context, tuple_function_expr, |function, context| {
        context.tuple_function_function_id(function)
    });
    typed_function_return(shape.clone(), body, context)
}

pub(super) fn tuple_never_function_return(
    body: &module::TupleFunctionReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::NeverFunctionReturn> {
    let type_ = context.generic_function_type(function_shape);
    let lowered = return_graph(
        body,
        context,
        super::expression::tuple_never_function_expr,
        |function, context| context.never_function_function_id(function, type_.clone()),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

pub(super) fn list_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::ListFunctionReturn,
    item: &SpecializedValueShape,
    context: &mut LoweringContext,
) -> Representability<execution::ListFunctionReturn> {
    let concrete = context.concrete_function_shape(shape);
    let body = return_graph(body, context, list_function_expr, |function, context| {
        context.list_function_function_id(function, &concrete, item)
    });
    typed_function_return(shape.clone(), body, context)
}

pub(super) fn symbolic_list_function_return(
    body: &module::ListFunctionReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::GenericFunctionReturn> {
    let type_ = context.generic_function_type(function_shape);
    let lowered = return_graph(
        body,
        context,
        |expression, context| {
            super::expression::symbolic_list_function_expr(expression, function_shape, context)
        },
        |function, context| context.generic_function_function_id(function, type_.clone()),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

pub(super) fn function_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::FunctionFunctionReturn,
    context: &mut LoweringContext,
) -> Representability<execution::FunctionFunctionReturn> {
    let return_shape = context.concrete_function_shape(body.type_().return_shape());
    let type_ = context.function_function_type(body.type_().clone());
    let lowered = return_graph(
        body.body(),
        context,
        |kind, context| function_function_expr_kind(kind, &return_shape, &type_, context),
        |function, context| {
            context
                .function_function_function_id(function, type_.clone())
                .map(|function| function.index())
        },
    );
    let shape = context.function_shape(shape.clone());
    lowered.map(|lowered| execution::FunctionFunctionReturn::from_parts(shape, type_, lowered))
}

pub(super) fn symbolic_function_function_return(
    body: &module::FunctionFunctionReturn,
    function_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
) -> Representability<execution::GenericFunctionReturn> {
    let type_ = context.generic_function_type(function_shape);
    let lowered = return_graph(
        body.body(),
        context,
        |kind, context| {
            super::expression::symbolic_function_function_expr_kind(kind, function_shape, context)
                .map(|kind| execution::GenericFunctionExpr::from_parts(type_.clone(), kind))
        },
        |function, context| context.generic_function_function_id(function, type_.clone()),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

fn typed_function_return<Body>(
    shape: crate::plan::FunctionShape,
    body: Representability<Body>,
    context: &mut LoweringContext,
) -> Representability<execution::TypedFunctionReturn<Body>> {
    let shape = context.function_shape(shape);
    body.map(|body| execution::TypedFunctionReturn::new(shape, body))
}

fn specialized_typed_function_return<Body>(
    shape: &super::specialization::SpecializedFunctionShape,
    body: Representability<Body>,
    context: &mut LoweringContext,
) -> Representability<execution::TypedFunctionReturn<Body>> {
    let shape = context.lower_concrete_function_shape(shape);
    body.map(|body| execution::TypedFunctionReturn::new(shape, body))
}

pub(super) fn symbolic_function_return<ModuleExpression>(
    body: &module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    function_shape: &super::specialization::SpecializedFunctionShape,
    context: &mut LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        &super::specialization::SpecializedFunctionShape,
        &mut LoweringContext,
    ) -> Representability<execution::GenericFunctionExpr>,
) -> Representability<execution::GenericFunctionReturn> {
    let type_ = context.generic_function_type(function_shape);
    let lowered = return_graph(
        body,
        context,
        |expression, context| lower_expression(expression, function_shape, context),
        |function, context| context.generic_function_function_id(function, type_.clone()),
    );
    specialized_typed_function_return(function_shape, lowered, context)
}

fn return_graph<ModuleExpression, ExecutionExpression, ExecutionFunction>(
    body: &module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    context: &mut LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        &mut LoweringContext,
    ) -> Representability<ExecutionExpression>,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &mut LoweringContext,
    ) -> Representability<ExecutionFunction>,
) -> Representability<execution::ReturnGraph<ExecutionExpression, ExecutionFunction>> {
    let mut builder = ReturnGraphBuilder::new();

    let entry = if let Some(expression) = context.take_return_divergence() {
        Representability::Inhabited(builder.push(execution::ReturnBlock::Never(expression)))
    } else {
        lower_return_target(
            body,
            context,
            lower_expression,
            lower_function,
            &mut builder,
        )
    };

    entry.map(|entry| builder.freeze(entry))
}

struct ReturnGraphBuilder<Expression, Function> {
    blocks: Vec<execution::ReturnBlock>,
    expressions: Vec<Expression>,
    tail_calls: Vec<execution::ReturnTailCall<Function>>,
}

impl<Expression, Function> ReturnGraphBuilder<Expression, Function> {
    fn new() -> Self {
        Self {
            blocks: Vec::new(),
            expressions: Vec::new(),
            tail_calls: Vec::new(),
        }
    }

    fn push(&mut self, block: execution::ReturnBlock) -> execution::ReturnTarget {
        let target = execution::ReturnTarget::from_block_index(self.blocks.len());
        self.blocks.push(block);
        target
    }

    fn push_return(&mut self, expression: Expression) -> execution::ReturnTarget {
        let expression_id =
            execution::ReturnExpressionId::from_expression_index(self.expressions.len());
        self.expressions.push(expression);
        self.push(execution::ReturnBlock::Return {
            expression: expression_id,
        })
    }

    fn push_tail_call(
        &mut self,
        function: Function,
        args: Vec<execution::CallArg>,
    ) -> execution::ReturnTarget {
        let call = execution::ReturnTailCallId::from_call_index(self.tail_calls.len());
        self.tail_calls
            .push(execution::ReturnTailCall::new(function, args));
        self.push(execution::ReturnBlock::TailCall { call })
    }

    fn freeze(
        self,
        entry: execution::ReturnTarget,
    ) -> execution::ReturnGraph<Expression, Function> {
        execution::ReturnGraph::from_parts(entry, self.blocks, self.expressions, self.tail_calls)
    }
}

fn lower_return_target<ModuleExpression, ExecutionExpression, ExecutionFunction>(
    body: &module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    context: &mut LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        &mut LoweringContext,
    ) -> Representability<ExecutionExpression>,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &mut LoweringContext,
    ) -> Representability<ExecutionFunction>,
    builder: &mut ReturnGraphBuilder<ExecutionExpression, ExecutionFunction>,
) -> Representability<execution::ReturnTarget> {
    use module::ReturnBodyKind as M;

    match body.kind() {
        M::Expr(expression) => {
            lower_expression(expression, context).map(|expression| builder.push_return(expression))
        }
        M::TailCall { function, args } => {
            direct_call(function, args, context, lower_function).map(|call| match call {
                execution::DirectCall::Executable { function, args } => {
                    builder.push_tail_call(function, args)
                }
                execution::DirectCall::Diverging(expression) => {
                    builder.push(execution::ReturnBlock::Never(expression))
                }
            })
        }
        M::BoolCase {
            subject,
            true_,
            false_,
        } => bool_expr(subject, context).and_then(|subject| match lower_bool_subject(subject) {
            super::expression::LoweredBoolSubject::True(steps) => {
                lower_return_target(true_, context, lower_expression, lower_function, builder)
                    .map(|target| prepend_steps(builder, steps, target))
            }
            super::expression::LoweredBoolSubject::False(steps) => {
                lower_return_target(false_, context, lower_expression, lower_function, builder)
                    .map(|target| prepend_steps(builder, steps, target))
            }
            super::expression::LoweredBoolSubject::Dynamic(subject) => {
                lower_return_target(true_, context, lower_expression, lower_function, builder)
                    .and_then(|true_| {
                        lower_return_target(
                            false_,
                            context,
                            lower_expression,
                            lower_function,
                            builder,
                        )
                        .map(|false_| {
                            builder.push(execution::ReturnBlock::BoolBranch {
                                subject,
                                true_,
                                false_,
                            })
                        })
                    })
            }
        }),
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => int_expr(subject, context).and_then(|subject| {
            let clauses = lower_clauses(
                clauses,
                context,
                lower_expression,
                lower_function,
                builder,
                |pattern| pattern.clone(),
            );
            clauses.and_then(|clauses| {
                lower_return_target(fallback, context, lower_expression, lower_function, builder)
                    .map(|fallback| {
                        builder.push(execution::ReturnBlock::IntSwitch {
                            subject,
                            clauses: clauses.into_boxed_slice(),
                            fallback,
                        })
                    })
            })
        }),
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => float_expr(subject, context).and_then(|subject| {
            let clauses = lower_clauses(
                clauses,
                context,
                lower_expression,
                lower_function,
                builder,
                |pattern| *pattern,
            );
            clauses.and_then(|clauses| {
                lower_return_target(fallback, context, lower_expression, lower_function, builder)
                    .map(|fallback| {
                        builder.push(execution::ReturnBlock::FloatSwitch {
                            subject,
                            clauses: clauses.into_boxed_slice(),
                            fallback,
                        })
                    })
            })
        }),
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => string_expr(subject, context).and_then(|subject| {
            let clauses = lower_clauses(
                clauses,
                context,
                lower_expression,
                lower_function,
                builder,
                |pattern| pattern.clone(),
            );
            clauses.and_then(|clauses| {
                lower_return_target(fallback, context, lower_expression, lower_function, builder)
                    .map(|fallback| {
                        builder.push(execution::ReturnBlock::StringSwitch {
                            subject,
                            clauses: clauses.into_boxed_slice(),
                            fallback,
                        })
                    })
            })
        }),
        M::Block { steps, return_ } => {
            super::step::steps_until_never(steps, context).and_then(|steps| match steps {
                super::step::StepsUntilNever::Complete(steps) => {
                    lower_return_target(return_, context, lower_expression, lower_function, builder)
                        .map(|next| {
                            builder.push(execution::ReturnBlock::Steps {
                                steps: steps.into_boxed_slice(),
                                next,
                            })
                        })
                }
                super::step::StepsUntilNever::Diverging { prefix, expression } => {
                    Representability::Inhabited(builder.push(execution::ReturnBlock::Never(
                        execution::NeverExpr::from_kind(execution::NeverExprKind::Block {
                            steps: prefix,
                            return_: Box::new(expression),
                        }),
                    )))
                }
            })
        }
    }
}

fn prepend_steps<Expression, Function>(
    builder: &mut ReturnGraphBuilder<Expression, Function>,
    steps: Vec<execution::Step>,
    next: execution::ReturnTarget,
) -> execution::ReturnTarget {
    if steps.is_empty() {
        next
    } else {
        builder.push(execution::ReturnBlock::Steps {
            steps: steps.into_boxed_slice(),
            next,
        })
    }
}

fn lower_clauses<
    ModuleExpression,
    ExecutionExpression,
    ExecutionFunction,
    Pattern,
    LoweredPattern,
>(
    clauses: &[(
        Pattern,
        module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    )],
    context: &mut LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        &mut LoweringContext,
    ) -> Representability<ExecutionExpression>,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &mut LoweringContext,
    ) -> Representability<ExecutionFunction>,
    builder: &mut ReturnGraphBuilder<ExecutionExpression, ExecutionFunction>,
    lower_pattern: impl Copy + Fn(&Pattern) -> LoweredPattern,
) -> Representability<Vec<(LoweredPattern, execution::ReturnTarget)>> {
    Representability::collect(clauses.iter().map(|(pattern, branch)| {
        lower_return_target(branch, context, lower_expression, lower_function, builder)
            .map(|target| (lower_pattern(pattern), target))
    }))
}

#[cfg(test)]
mod tests {
    use super::super::specialization::{
        Representability, RepresentationContext, SpecializationKey,
    };
    use super::super::{FunctionTemplates, LoweringContext};
    use crate::plan::execution::{
        ExecutionPlan, FunctionFunctionId, IntFunctionId, ListFunctionId, ListListFunctionId,
        ReturnBlock, ReturnGraph, RuntimeFunctionId,
    };
    use std::collections::HashSet;

    #[test]
    fn lowering_seals_custom_callable_return_type_around_tail_indices() {
        let plan = execution_plan(
            r#"
pub type Boxed { Boxed(Int) }

fn build(value: Int) -> Boxed { Boxed(value) }

fn factory() -> fn(Int) -> Boxed { factory() }

pub fn main() -> fn(Int) -> Boxed { factory() }
"#,
        );
        let main = plan.custom_function_function_id(0);
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Custom(main.clone()),
                return_type: main.type_().to_function_type(),
            },
        );
        let return_ = plan.custom_function_function(&main).return_();

        assert_eq!(return_.type_(), main.type_());
        assert_eq!(return_.function_id(1).type_(), main.type_());
    }

    #[test]
    fn lowering_seals_nested_callable_return_type_around_tail_indices() {
        let plan = execution_plan(
            r#"
fn factory() -> fn() -> fn(Int) -> Int { factory() }

pub fn main() -> fn() -> fn(Int) -> Int { factory() }
"#,
        );
        let main = plan.function_function_function_id(0);
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Function(main.clone()),
                return_type: main.type_().to_function_type(),
            },
        );
        let return_ = plan.function_function_function(&main).return_();

        assert_eq!(return_.type_(), main.type_());
        assert_eq!(return_.function_id(1).type_(), main.type_());
    }

    #[test]
    fn lowering_carries_exact_nested_list_type_through_tail_calls() {
        let source = r#"
fn repeat(values: List(List(Int))) -> List(List(Int)) {
  repeat(values)
}

pub fn main() -> List(List(Int)) {
  let _ = repeat
  []
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let function = plan.list_list_function_id(1);
        let (next, argument_count) = expect_tail_call(plan.list_list_function(function).return_());
        let main = expect_list_list_main(&plan);

        assert_eq!(*next, function);
        assert_eq!(argument_count, 1);
        assert_eq!(next.type_id(), function.type_id());
        assert_eq!(main.type_id(), function.type_id());
    }

    #[test]
    fn lowering_preserves_tail_calls_in_exact_uninhabited_custom_body_refinements() {
        let parameter = crate::plan::TypeParameterId(0);
        let type_name =
            crate::plan::CustomTypeName::new("geam".into(), "main".into(), "Maybe".into());
        let definition = crate::plan::CustomTypeDefinition::new(
            type_name.clone(),
            crate::plan::CustomTypePublicity::Private,
            false,
            vec![crate::plan::CustomTypeParameterId(0)],
            vec![
                crate::plan::CustomConstructorDefinition::new("None".into(), 0, Vec::new()),
                crate::plan::CustomConstructorDefinition::new(
                    "Some".into(),
                    1,
                    vec![crate::plan::CustomFieldDefinition::new(
                        None,
                        crate::plan::CustomTypeTemplate::Parameter(
                            crate::plan::CustomTypeParameterId(0),
                        ),
                    )],
                ),
            ],
        );
        let type_ = crate::plan::CustomType::new(
            type_name.clone(),
            vec![crate::plan::ValueType::Parameter(parameter)],
        );
        let signature_shape = crate::plan::CustomValueShape::any(type_);
        let body_shape = crate::plan::CustomValueShape::new(
            type_name,
            vec![crate::plan::ValueShape::Parameter(parameter)],
            crate::plan::CustomConstructorRefinement::Exact(1),
        );
        let target_id = crate::plan::FunctionTemplateId::new(1);
        let target_signature = crate::plan::FunctionTemplateSignature::new(
            target_id,
            crate::plan::TypeScheme::new(0),
            crate::plan::FunctionShape::new(
                Vec::new(),
                crate::plan::ValueShape::Custom(signature_shape.clone()),
            ),
        );
        let target = crate::plan::FunctionTemplate::from_signature(
            target_signature,
            "always_some".into(),
            Vec::new(),
            Vec::new(),
            crate::plan::ReturnExpr::custom_body(crate::plan::CustomReturn::with_signature_shape(
                signature_shape.clone(),
                crate::plan::CustomExpr::panic_shape(
                    crate::plan::PanicExpr::panic_at(None, crate::plan::PanicSite::unknown()),
                    body_shape.clone(),
                ),
            )),
        );
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
        let templates = FunctionTemplates::new(main, vec![target], Vec::new());
        let mut context = LoweringContext::new(
            &templates,
            SpecializationKey::monomorphic(main_id),
            RepresentationContext::new(vec![definition]),
            crate::plan::ConstantTemplates::from_entries(Vec::new()),
            HashSet::new(),
        );
        let function = crate::plan::monomorphic_function_instantiation(
            1,
            crate::plan::FunctionShape::new(
                Vec::new(),
                crate::plan::ValueShape::Custom(signature_shape.clone()),
            ),
        );
        let body = crate::plan::CustomReturn::with_signature_shape(
            signature_shape,
            crate::plan::CustomExpr::call(function, Vec::new(), body_shape),
        );

        assert_eq!(
            super::custom_return(&body, &mut context).map(|body| {
                let (function, argument_count) = expect_tail_call(body.body());
                (*function, argument_count)
            }),
            Representability::Inhabited((0, 0)),
        );
    }

    #[test]
    fn lowering_freezes_return_blocks_in_child_first_postorder() {
        let plan = execution_plan(
            r#"
fn choose(flag: Bool, number: Int, decimal: Float, text: String) -> Int {
  case flag {
    True -> case number {
      1 -> choose(flag, number, decimal, text)
      _ -> {
        let next = number
        next
      }
    }
    False -> case decimal {
      1.5 -> 20
      _ -> case text {
        "x" -> 30
        _ -> 40
      }
    }
  }
}

pub fn main() { choose(False, 0, 0.0, "") }
"#,
        );
        let graph = plan.int_function(IntFunctionId(1)).return_();

        assert_eq!(graph.entry().index(), 9);
        assert_eq!(
            summarize_graph(graph),
            vec![
                ReturnBlockSummary::TailCall { argument_count: 4 },
                ReturnBlockSummary::Return,
                ReturnBlockSummary::Steps {
                    step_count: 1,
                    next: 1,
                },
                ReturnBlockSummary::IntSwitch {
                    clauses: vec![(1.into(), 0)],
                    fallback: 2,
                },
                ReturnBlockSummary::Return,
                ReturnBlockSummary::Return,
                ReturnBlockSummary::Return,
                ReturnBlockSummary::StringSwitch {
                    clauses: vec![("x".into(), 5)],
                    fallback: 6,
                },
                ReturnBlockSummary::FloatSwitch {
                    clauses: vec![(1.5, 4)],
                    fallback: 7,
                },
                ReturnBlockSummary::BoolBranch {
                    true_: 3,
                    false_: 8,
                },
            ],
        );
    }

    #[test]
    fn lowering_static_bool_keeps_only_the_selected_graph_branch_and_prefix_steps() {
        let plan = execution_plan(
            r#"
fn selected() -> Int {
  case {
    let prefix = 1
    True
  } {
    True -> 10
    False -> panic as "unselected"
  }
}

pub fn main() { selected() }
"#,
        );
        let graph = plan.int_function(IntFunctionId(1)).return_();

        assert_eq!(graph.entry().index(), 1);
        assert_eq!(
            summarize_graph(graph),
            vec![
                ReturnBlockSummary::Return,
                ReturnBlockSummary::Steps {
                    step_count: 1,
                    next: 0,
                },
            ],
        );
    }

    #[test]
    fn lowering_source_stop_return_freezes_one_never_block() {
        let plan = execution_plan(
            r#"
fn stop() -> value { panic as "stop" }
fn consume(value: value) -> Int { consume(value) }

pub fn main() { consume(stop()) }
"#,
        );
        let main = IntFunctionId(0);
        assert_eq!(plan.main_runtime(), RuntimeFunctionId::Int(main));
        let graph = plan.int_function(main).return_();

        assert_eq!(graph.entry().index(), 0);
        assert_eq!(summarize_graph(graph), vec![ReturnBlockSummary::Never]);
    }

    #[test]
    #[should_panic(expected = "expected a tail-call return body")]
    fn tail_call_fixture_guard_rejects_expression_return() {
        let plan = execution_plan("pub fn main() -> List(List(Int)) { [] }");
        let main = expect_list_list_main(&plan);
        let _ = expect_tail_call(plan.list_list_function(main).return_());
    }

    #[test]
    #[should_panic(expected = "expected a List(List) main function")]
    fn nested_list_main_fixture_guard_rejects_int_main() {
        let plan = execution_plan("pub fn main() { 1 }");
        let _ = expect_list_list_main(&plan);
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }

    fn expect_tail_call<Expression, Function>(
        graph: &ReturnGraph<Expression, Function>,
    ) -> (&Function, usize) {
        match graph.block(graph.entry()) {
            ReturnBlock::TailCall { call } => {
                let call = graph.tail_call(*call);
                (call.function(), call.args().len())
            }
            _ => panic!("expected a tail-call return body"),
        }
    }

    fn expect_list_list_main(plan: &ExecutionPlan) -> ListListFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::List(ListFunctionId::List(main)) => main,
            _ => panic!("expected a List(List) main function"),
        }
    }

    #[derive(Debug, PartialEq)]
    enum ReturnBlockSummary {
        Return,
        Never,
        TailCall {
            argument_count: usize,
        },
        BoolBranch {
            true_: usize,
            false_: usize,
        },
        IntSwitch {
            clauses: Vec<(num_bigint::BigInt, usize)>,
            fallback: usize,
        },
        FloatSwitch {
            clauses: Vec<(f64, usize)>,
            fallback: usize,
        },
        StringSwitch {
            clauses: Vec<(String, usize)>,
            fallback: usize,
        },
        Steps {
            step_count: usize,
            next: usize,
        },
    }

    fn summarize_graph(
        graph: &ReturnGraph<crate::plan::execution::IntExpr, IntFunctionId>,
    ) -> Vec<ReturnBlockSummary> {
        graph
            .blocks()
            .iter()
            .map(|block| match block {
                ReturnBlock::Return { .. } => ReturnBlockSummary::Return,
                ReturnBlock::Never(_) => ReturnBlockSummary::Never,
                ReturnBlock::TailCall { call } => ReturnBlockSummary::TailCall {
                    argument_count: graph.tail_call(*call).args().len(),
                },
                ReturnBlock::BoolBranch { true_, false_, .. } => ReturnBlockSummary::BoolBranch {
                    true_: true_.index(),
                    false_: false_.index(),
                },
                ReturnBlock::IntSwitch {
                    clauses, fallback, ..
                } => ReturnBlockSummary::IntSwitch {
                    clauses: clauses
                        .iter()
                        .map(|(pattern, target)| (pattern.clone(), target.index()))
                        .collect(),
                    fallback: fallback.index(),
                },
                ReturnBlock::FloatSwitch {
                    clauses, fallback, ..
                } => ReturnBlockSummary::FloatSwitch {
                    clauses: clauses
                        .iter()
                        .map(|(pattern, target)| (*pattern, target.index()))
                        .collect(),
                    fallback: fallback.index(),
                },
                ReturnBlock::StringSwitch {
                    clauses, fallback, ..
                } => ReturnBlockSummary::StringSwitch {
                    clauses: clauses
                        .iter()
                        .map(|(pattern, target)| (pattern.to_string(), target.index()))
                        .collect(),
                    fallback: fallback.index(),
                },
                ReturnBlock::Steps { steps, next } => ReturnBlockSummary::Steps {
                    step_count: steps.len(),
                    next: next.index(),
                },
            })
            .collect()
    }
}
