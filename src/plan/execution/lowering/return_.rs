use super::LoweringContext;
use super::expression::{
    bit_array_expr, bit_array_function_expr, bit_array_list_expr, bool_expr, bool_function_expr,
    bool_list_expr, custom_expr_kind, custom_function_expr_kind, custom_list_expr,
    direct_call_args, float_expr, float_function_expr, float_list_expr,
    function_function_expr_kind, function_list_expr, generic_bit_array_expr,
    generic_bit_array_function_expr, generic_bit_array_list_expr, generic_bool_expr,
    generic_bool_function_expr, generic_bool_list_expr, generic_custom_expr_kind,
    generic_custom_function_expr_kind, generic_custom_list_expr, generic_float_expr,
    generic_float_function_expr, generic_float_list_expr, generic_function_function_expr_kind,
    generic_function_list_expr, generic_int_expr, generic_int_function_expr, generic_int_list_expr,
    generic_list_function_expr, generic_nested_list_expr, generic_nil_expr,
    generic_nil_function_expr, generic_nil_list_expr, generic_string_expr,
    generic_string_function_expr, generic_string_list_expr, generic_tuple_expr,
    generic_tuple_function_expr, generic_tuple_list_expr, generic_utf_codepoint_expr,
    generic_utf_codepoint_function_expr, generic_utf_codepoint_list_expr,
    generic_value_bit_array_function_expr, generic_value_bit_array_list_expr,
    generic_value_bool_function_expr, generic_value_bool_list_expr,
    generic_value_custom_function_expr_kind, generic_value_custom_list_expr,
    generic_value_float_function_expr, generic_value_float_list_expr,
    generic_value_function_function_expr_kind, generic_value_function_list_expr,
    generic_value_int_function_expr, generic_value_int_list_expr, generic_value_list_function_expr,
    generic_value_nested_list_expr, generic_value_nil_function_expr, generic_value_nil_list_expr,
    generic_value_string_function_expr, generic_value_string_list_expr,
    generic_value_tuple_function_expr, generic_value_tuple_list_expr,
    generic_value_utf_codepoint_function_expr, generic_value_utf_codepoint_list_expr, int_expr,
    int_function_expr, int_list_expr, list_function_expr, list_list_expr, nil_expr,
    nil_function_expr, nil_list_expr, string_expr, string_function_expr, string_list_expr,
    tuple_expr, tuple_function_expr, tuple_list_expr, utf_codepoint_expr,
    utf_codepoint_function_expr, utf_codepoint_list_expr,
};
use crate::plan::{execution, module};

macro_rules! generic_primitive_return {
    ($lower:ident, $return:ty, $expression:ident, $function:ident) => {
        pub(super) fn $lower(
            body: &module::GenericReturn,
            context: &mut LoweringContext,
        ) -> $return {
            return_body(body, context, $expression, |function, context| {
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
    shape: &super::specialization::ConcreteCustomValueShape,
    context: &mut LoweringContext,
) -> execution::CustomReturn {
    let lowered_shape = context.lower_concrete_custom_shape(shape);
    let body = return_body(
        body,
        context,
        |expression, context| generic_custom_expr_kind(expression, shape, context),
        |function, context| context.custom_function_id(function, shape).index(),
    );
    execution::CustomReturn::from_parts(lowered_shape, body)
}

pub(super) fn generic_tuple_return(
    body: &module::GenericReturn,
    elements: &[super::specialization::ConcreteValueShape],
    context: &mut LoweringContext,
) -> execution::TupleReturn {
    return_body(
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
        ) -> $return {
            return_body(body, context, $expression, |function, context| {
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
    shape: &super::specialization::ConcreteCustomValueShape,
    type_id: execution::CustomListTypeId,
    context: &mut LoweringContext,
) -> execution::CustomListReturn {
    return_body(
        body,
        context,
        |expression, context| generic_value_custom_list_expr(expression, shape, context),
        move |function, context| context.custom_list_function_id(function, type_id),
    )
}

pub(super) fn generic_value_tuple_list_return(
    body: &module::GenericReturn,
    elements: &[super::specialization::ConcreteValueShape],
    type_id: execution::TupleListTypeId,
    context: &mut LoweringContext,
) -> execution::TupleListReturn {
    return_body(
        body,
        context,
        |expression, context| generic_value_tuple_list_expr(expression, elements, context),
        move |function, context| context.tuple_list_function_id(function, type_id),
    )
}

pub(super) fn generic_value_nested_list_return(
    body: &module::GenericReturn,
    item: &super::specialization::ConcreteValueShape,
    type_id: execution::ListListTypeId,
    context: &mut LoweringContext,
) -> execution::ListListReturn {
    return_body(
        body,
        context,
        |expression, context| generic_value_nested_list_expr(expression, item, context),
        move |function, context| context.list_list_function_id(function, type_id),
    )
}

pub(super) fn generic_value_function_list_return(
    body: &module::GenericReturn,
    function_shape: &super::specialization::ConcreteFunctionShape,
    type_id: execution::FunctionListTypeId,
    context: &mut LoweringContext,
) -> execution::FunctionListReturn {
    return_body(
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
        ) -> $return {
            return_body(body, context, $expression, |function, context| {
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
    shape: &super::specialization::ConcreteCustomValueShape,
    type_id: execution::CustomListTypeId,
    context: &mut LoweringContext,
) -> execution::CustomListReturn {
    return_body(
        body,
        context,
        |expression, context| generic_custom_list_expr(expression, shape, context),
        move |function, context| context.custom_list_function_id(function, type_id),
    )
}

pub(super) fn generic_item_tuple_list_return(
    body: &module::GenericListReturn,
    elements: &[super::specialization::ConcreteValueShape],
    type_id: execution::TupleListTypeId,
    context: &mut LoweringContext,
) -> execution::TupleListReturn {
    return_body(
        body,
        context,
        |expression, context| generic_tuple_list_expr(expression, elements, context),
        move |function, context| context.tuple_list_function_id(function, type_id),
    )
}

pub(super) fn generic_item_nested_list_return(
    body: &module::GenericListReturn,
    item: &super::specialization::ConcreteValueShape,
    type_id: execution::ListListTypeId,
    context: &mut LoweringContext,
) -> execution::ListListReturn {
    return_body(
        body,
        context,
        |expression, context| generic_nested_list_expr(expression, item, context),
        move |function, context| context.list_list_function_id(function, type_id),
    )
}

pub(super) fn generic_item_function_list_return(
    body: &module::GenericListReturn,
    function_shape: &super::specialization::ConcreteFunctionShape,
    type_id: execution::FunctionListTypeId,
    context: &mut LoweringContext,
) -> execution::FunctionListReturn {
    return_body(
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
            function_shape: &super::specialization::ConcreteFunctionShape,
            context: &mut LoweringContext,
        ) -> $return {
            let lowered = return_body(
                body,
                context,
                |expression, context| $expression(expression, function_shape, context),
                |function, context| context.$function(function),
            );
            execution::TypedFunctionReturn::new(
                context.function_shape(function_shape.to_module_shape()),
                lowered,
            )
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
    function_shape: &super::specialization::ConcreteFunctionShape,
    context: &mut LoweringContext,
) -> execution::TupleFunctionReturn {
    let lowered = return_body(
        body,
        context,
        |expression, context| {
            generic_value_tuple_function_expr(expression, function_shape, context)
        },
        |function, context| context.tuple_function_function_id(function),
    );
    execution::TypedFunctionReturn::new(
        context.function_shape(function_shape.to_module_shape()),
        lowered,
    )
}

pub(super) fn generic_value_custom_function_return(
    body: &module::GenericReturn,
    function_shape: &super::specialization::ConcreteFunctionShape,
    return_shape: &super::specialization::ConcreteCustomValueShape,
    context: &mut LoweringContext,
) -> execution::CustomFunctionReturn {
    let type_ = context.custom_function_type(crate::plan::CustomFunctionType::from_shapes(
        function_shape
            .arguments()
            .iter()
            .map(super::specialization::ConcreteValueShape::to_module_shape)
            .collect(),
        return_shape.to_module_shape(),
    ));
    let lowered = return_body(
        body,
        context,
        |expression, context| {
            generic_value_custom_function_expr_kind(expression, function_shape, &type_, context)
        },
        |function, context| {
            context
                .custom_function_function_id(function, type_.clone())
                .index()
        },
    );
    execution::CustomFunctionReturn::from_parts(
        context.function_shape(function_shape.to_module_shape()),
        type_,
        lowered,
    )
}

pub(super) fn generic_value_list_function_return(
    body: &module::GenericReturn,
    function_shape: &super::specialization::ConcreteFunctionShape,
    item: &super::specialization::ConcreteValueShape,
    context: &mut LoweringContext,
) -> execution::ListFunctionReturn {
    let lowered = return_body(
        body,
        context,
        |expression, context| {
            generic_value_list_function_expr(expression, function_shape, item, context)
        },
        |function, context| context.list_function_function_id(function, function_shape, item),
    );
    execution::TypedFunctionReturn::new(
        context.function_shape(function_shape.to_module_shape()),
        lowered,
    )
}

pub(super) fn generic_value_function_function_return(
    body: &module::GenericReturn,
    function_shape: &super::specialization::ConcreteFunctionShape,
    return_shape: &super::specialization::ConcreteFunctionShape,
    context: &mut LoweringContext,
) -> execution::FunctionFunctionReturn {
    let type_ = context.function_function_type(crate::plan::FunctionFunctionType::from_shapes(
        function_shape
            .arguments()
            .iter()
            .map(super::specialization::ConcreteValueShape::to_module_shape)
            .collect(),
        return_shape.to_module_shape(),
    ));
    let lowered = return_body(
        body,
        context,
        |expression, context| {
            generic_value_function_function_expr_kind(expression, function_shape, &type_, context)
        },
        |function, context| {
            context
                .function_function_function_id(function, type_.clone())
                .index()
        },
    );
    execution::FunctionFunctionReturn::from_parts(
        context.function_shape(function_shape.to_module_shape()),
        type_,
        lowered,
    )
}

macro_rules! generic_result_primitive_function_return {
    ($lower:ident, $return:ty, $expression:ident, $function:ident) => {
        pub(super) fn $lower(
            body: &module::GenericFunctionReturn,
            function_shape: &super::specialization::ConcreteFunctionShape,
            context: &mut LoweringContext,
        ) -> $return {
            let lowered = return_body(body, context, $expression, |function, context| {
                context.$function(function)
            });
            execution::TypedFunctionReturn::new(
                context.function_shape(function_shape.to_module_shape()),
                lowered,
            )
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
    function_shape: &super::specialization::ConcreteFunctionShape,
    context: &mut LoweringContext,
) -> execution::TupleFunctionReturn {
    let lowered = return_body(
        body,
        context,
        generic_tuple_function_expr,
        |function, context| context.tuple_function_function_id(function),
    );
    execution::TypedFunctionReturn::new(
        context.function_shape(function_shape.to_module_shape()),
        lowered,
    )
}

pub(super) fn generic_result_custom_function_return(
    body: &module::GenericFunctionReturn,
    function_shape: &super::specialization::ConcreteFunctionShape,
    return_shape: &super::specialization::ConcreteCustomValueShape,
    context: &mut LoweringContext,
) -> execution::CustomFunctionReturn {
    let type_ = context.custom_function_type(crate::plan::CustomFunctionType::from_shapes(
        function_shape
            .arguments()
            .iter()
            .map(super::specialization::ConcreteValueShape::to_module_shape)
            .collect(),
        return_shape.to_module_shape(),
    ));
    let lowered = return_body(
        body,
        context,
        |expression, context| {
            generic_custom_function_expr_kind(expression, return_shape, &type_, context)
        },
        |function, context| {
            context
                .custom_function_function_id(function, type_.clone())
                .index()
        },
    );
    execution::CustomFunctionReturn::from_parts(
        context.function_shape(function_shape.to_module_shape()),
        type_,
        lowered,
    )
}

pub(super) fn generic_result_list_function_return(
    body: &module::GenericFunctionReturn,
    function_shape: &super::specialization::ConcreteFunctionShape,
    item: &super::specialization::ConcreteValueShape,
    context: &mut LoweringContext,
) -> execution::ListFunctionReturn {
    let lowered = return_body(
        body,
        context,
        |expression, context| generic_list_function_expr(expression, item, context),
        |function, context| context.list_function_function_id(function, function_shape, item),
    );
    execution::TypedFunctionReturn::new(
        context.function_shape(function_shape.to_module_shape()),
        lowered,
    )
}

pub(super) fn generic_result_function_function_return(
    body: &module::GenericFunctionReturn,
    function_shape: &super::specialization::ConcreteFunctionShape,
    return_shape: &super::specialization::ConcreteFunctionShape,
    context: &mut LoweringContext,
) -> execution::FunctionFunctionReturn {
    let type_ = context.function_function_type(crate::plan::FunctionFunctionType::from_shapes(
        function_shape
            .arguments()
            .iter()
            .map(super::specialization::ConcreteValueShape::to_module_shape)
            .collect(),
        return_shape.to_module_shape(),
    ));
    let lowered = return_body(
        body,
        context,
        |expression, context| {
            generic_function_function_expr_kind(expression, return_shape, &type_, context)
        },
        |function, context| {
            context
                .function_function_function_id(function, type_.clone())
                .index()
        },
    );
    execution::FunctionFunctionReturn::from_parts(
        context.function_shape(function_shape.to_module_shape()),
        type_,
        lowered,
    )
}

pub(super) fn int_return(
    body: &module::IntReturn,
    context: &mut LoweringContext,
) -> execution::IntReturn {
    return_body(body, context, int_expr, |function, context| {
        context.int_function_id(function)
    })
}

pub(super) fn float_return(
    body: &module::FloatReturn,
    context: &mut LoweringContext,
) -> execution::FloatReturn {
    return_body(body, context, float_expr, |function, context| {
        context.float_function_id(function)
    })
}

pub(super) fn string_return(
    body: &module::StringReturn,
    context: &mut LoweringContext,
) -> execution::StringReturn {
    return_body(body, context, string_expr, |function, context| {
        context.string_function_id(function)
    })
}

pub(super) fn bit_array_return(
    body: &module::BitArrayReturn,
    context: &mut LoweringContext,
) -> execution::BitArrayReturn {
    return_body(body, context, bit_array_expr, |function, context| {
        context.bit_array_function_id(function)
    })
}

pub(super) fn utf_codepoint_return(
    body: &module::UtfCodepointReturn,
    context: &mut LoweringContext,
) -> execution::UtfCodepointReturn {
    return_body(body, context, utf_codepoint_expr, |function, context| {
        context.utf_codepoint_function_id(function)
    })
}

pub(super) fn custom_return(
    body: &module::CustomReturn,
    context: &mut LoweringContext,
) -> execution::CustomReturn {
    let shape = context.concrete_custom_value_shape(body.shape());
    let lowered_shape = context.lower_concrete_custom_shape(&shape);
    let body = return_body(
        body.body(),
        context,
        |kind, context| custom_expr_kind(kind, &shape, context),
        |function, context| context.custom_function_id(function, &shape).index(),
    );
    execution::CustomReturn::from_parts(lowered_shape, body)
}

pub(super) fn bool_return(
    body: &module::BoolReturn,
    context: &mut LoweringContext,
) -> execution::BoolReturn {
    return_body(body, context, bool_expr, |function, context| {
        context.bool_function_id(function)
    })
}

pub(super) fn nil_return(
    body: &module::NilReturn,
    context: &mut LoweringContext,
) -> execution::NilReturn {
    return_body(body, context, nil_expr, |function, context| {
        context.nil_function_id(function)
    })
}

pub(super) fn tuple_return(
    body: &module::TupleReturn,
    context: &mut LoweringContext,
) -> execution::TupleReturn {
    return_body(body, context, tuple_expr, |function, context| {
        context.tuple_function_id(function)
    })
}
pub(super) fn int_list_return(
    body: &module::IntListReturn,
    context: &mut LoweringContext,
) -> execution::IntListReturn {
    return_body(body, context, int_list_expr, |function, context| {
        context.int_list_function_id(function)
    })
}

pub(super) fn string_list_return(
    body: &module::StringListReturn,
    context: &mut LoweringContext,
) -> execution::StringListReturn {
    return_body(body, context, string_list_expr, |function, context| {
        context.string_list_function_id(function)
    })
}

pub(super) fn bit_array_list_return(
    body: &module::BitArrayListReturn,
    context: &mut LoweringContext,
) -> execution::BitArrayListReturn {
    return_body(body, context, bit_array_list_expr, |function, context| {
        context.bit_array_list_function_id(function)
    })
}

pub(super) fn utf_codepoint_list_return(
    body: &module::UtfCodepointListReturn,
    context: &mut LoweringContext,
) -> execution::UtfCodepointListReturn {
    return_body(
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
) -> execution::CustomListReturn {
    return_body(body, context, custom_list_expr, move |function, context| {
        context.custom_list_function_id(function, type_id)
    })
}

pub(super) fn float_list_return(
    body: &module::FloatListReturn,
    context: &mut LoweringContext,
) -> execution::FloatListReturn {
    return_body(body, context, float_list_expr, |function, context| {
        context.float_list_function_id(function)
    })
}

pub(super) fn bool_list_return(
    body: &module::BoolListReturn,
    context: &mut LoweringContext,
) -> execution::BoolListReturn {
    return_body(body, context, bool_list_expr, |function, context| {
        context.bool_list_function_id(function)
    })
}

pub(super) fn nil_list_return(
    body: &module::NilListReturn,
    context: &mut LoweringContext,
) -> execution::NilListReturn {
    return_body(body, context, nil_list_expr, |function, context| {
        context.nil_list_function_id(function)
    })
}

pub(super) fn tuple_list_return(
    body: &module::TupleListReturn,
    type_id: execution::TupleListTypeId,
    context: &mut LoweringContext,
) -> execution::TupleListReturn {
    return_body(body, context, tuple_list_expr, move |function, context| {
        context.tuple_list_function_id(function, type_id)
    })
}

pub(super) fn list_list_return(
    body: &module::ListListReturn,
    type_id: execution::ListListTypeId,
    context: &mut LoweringContext,
) -> execution::ListListReturn {
    return_body(body, context, list_list_expr, move |function, context| {
        context.list_list_function_id(function, type_id)
    })
}

pub(super) fn function_list_return(
    body: &module::FunctionListReturn,
    type_id: execution::FunctionListTypeId,
    context: &mut LoweringContext,
) -> execution::FunctionListReturn {
    return_body(
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
) -> execution::IntFunctionReturn {
    let body = return_body(body, context, int_function_expr, |function, context| {
        context.int_function_function_id(function)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape.clone()), body)
}

pub(super) fn float_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::FloatFunctionReturn,
    context: &mut LoweringContext,
) -> execution::FloatFunctionReturn {
    let body = return_body(body, context, float_function_expr, |function, context| {
        context.float_function_function_id(function)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape.clone()), body)
}

pub(super) fn string_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::StringFunctionReturn,
    context: &mut LoweringContext,
) -> execution::StringFunctionReturn {
    let body = return_body(body, context, string_function_expr, |function, context| {
        context.string_function_function_id(function)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape.clone()), body)
}

pub(super) fn bit_array_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::BitArrayFunctionReturn,
    context: &mut LoweringContext,
) -> execution::BitArrayFunctionReturn {
    let body = return_body(
        body,
        context,
        bit_array_function_expr,
        |function, context| context.bit_array_function_function_id(function),
    );
    execution::TypedFunctionReturn::new(context.function_shape(shape.clone()), body)
}

pub(super) fn utf_codepoint_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::UtfCodepointFunctionReturn,
    context: &mut LoweringContext,
) -> execution::UtfCodepointFunctionReturn {
    let body = return_body(
        body,
        context,
        utf_codepoint_function_expr,
        |function, context| context.utf_codepoint_function_function_id(function),
    );
    execution::TypedFunctionReturn::new(context.function_shape(shape.clone()), body)
}

pub(super) fn custom_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::CustomFunctionReturn,
    context: &mut LoweringContext,
) -> execution::CustomFunctionReturn {
    let return_shape = context.concrete_custom_value_shape(body.type_().return_());
    let type_ = context.custom_function_type(body.type_().clone());
    let lowered = return_body(
        body.body(),
        context,
        |kind, context| custom_function_expr_kind(kind, &return_shape, &type_, context),
        |function, context| {
            context
                .custom_function_function_id(function, type_.clone())
                .index()
        },
    );
    execution::CustomFunctionReturn::from_parts(
        context.function_shape(shape.clone()),
        type_,
        lowered,
    )
}

pub(super) fn bool_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::BoolFunctionReturn,
    context: &mut LoweringContext,
) -> execution::BoolFunctionReturn {
    let body = return_body(body, context, bool_function_expr, |function, context| {
        context.bool_function_function_id(function)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape.clone()), body)
}

pub(super) fn nil_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::NilFunctionReturn,
    context: &mut LoweringContext,
) -> execution::NilFunctionReturn {
    let body = return_body(body, context, nil_function_expr, |function, context| {
        context.nil_function_function_id(function)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape.clone()), body)
}

pub(super) fn tuple_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::TupleFunctionReturn,
    context: &mut LoweringContext,
) -> execution::TupleFunctionReturn {
    let body = return_body(body, context, tuple_function_expr, |function, context| {
        context.tuple_function_function_id(function)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape.clone()), body)
}

pub(super) fn list_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::ListFunctionReturn,
    item: &crate::plan::execution::lowering::specialization::ConcreteValueShape,
    context: &mut LoweringContext,
) -> execution::ListFunctionReturn {
    let concrete = context.concrete_function_shape(shape);
    let body = return_body(body, context, list_function_expr, |function, context| {
        context.list_function_function_id(function, &concrete, item)
    });
    execution::TypedFunctionReturn::new(context.function_shape(shape.clone()), body)
}

pub(super) fn function_function_return(
    shape: &crate::plan::FunctionShape,
    body: &module::FunctionFunctionReturn,
    context: &mut LoweringContext,
) -> execution::FunctionFunctionReturn {
    let return_shape = context.concrete_function_shape(body.type_().return_shape());
    let type_ = context.function_function_type(body.type_().clone());
    let lowered = return_body(
        body.body(),
        context,
        |kind, context| function_function_expr_kind(kind, &return_shape, &type_, context),
        |function, context| {
            context
                .function_function_function_id(function, type_.clone())
                .index()
        },
    );
    execution::FunctionFunctionReturn::from_parts(
        context.function_shape(shape.clone()),
        type_,
        lowered,
    )
}

fn return_body<ModuleExpression, ExecutionExpression, ExecutionFunction>(
    body: &module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    context: &mut LoweringContext,
    lower_expression: impl Copy + Fn(&ModuleExpression, &mut LoweringContext) -> ExecutionExpression,
    lower_function: impl Copy
    + Fn(&module::FunctionInstantiation, &mut LoweringContext) -> ExecutionFunction,
) -> execution::ReturnBody<ExecutionExpression, ExecutionFunction> {
    use execution::ReturnBodyKind as E;
    use module::ReturnBodyKind as M;

    let kind = match body.kind() {
        M::Expr(expression) => E::Expr(lower_expression(expression, context)),
        M::TailCall { function, args } => E::TailCall {
            function: lower_function(function, context),
            args: direct_call_args(function, args, context),
        },
        M::BoolCase {
            subject,
            true_,
            false_,
        } => E::BoolCase {
            subject: bool_expr(subject, context),
            true_: Box::new(return_body(
                true_,
                context,
                lower_expression,
                lower_function,
            )),
            false_: Box::new(return_body(
                false_,
                context,
                lower_expression,
                lower_function,
            )),
        },
        M::IntCase {
            subject,
            clauses,
            fallback,
        } => E::IntCase {
            subject: int_expr(subject, context),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        return_body(branch, context, lower_expression, lower_function),
                    )
                })
                .collect(),
            fallback: Box::new(return_body(
                fallback,
                context,
                lower_expression,
                lower_function,
            )),
        },
        M::FloatCase {
            subject,
            clauses,
            fallback,
        } => E::FloatCase {
            subject: float_expr(subject, context),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        *pattern,
                        return_body(branch, context, lower_expression, lower_function),
                    )
                })
                .collect(),
            fallback: Box::new(return_body(
                fallback,
                context,
                lower_expression,
                lower_function,
            )),
        },
        M::StringCase {
            subject,
            clauses,
            fallback,
        } => E::StringCase {
            subject: string_expr(subject, context),
            clauses: clauses
                .iter()
                .map(|(pattern, branch)| {
                    (
                        pattern.clone(),
                        return_body(branch, context, lower_expression, lower_function),
                    )
                })
                .collect(),
            fallback: Box::new(return_body(
                fallback,
                context,
                lower_expression,
                lower_function,
            )),
        },
        M::Block { steps, return_ } => E::Block {
            steps: super::step::steps(steps, context),
            return_: Box::new(return_body(
                return_,
                context,
                lower_expression,
                lower_function,
            )),
        },
    };

    execution::ReturnBody::from_kind(kind)
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{
        ExecutionPlan, FunctionFunctionId, ListFunctionId, ListListFunctionId, ReturnBody,
        ReturnBodyKind, RuntimeFunctionId,
    };

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

pub fn main() -> List(List(Int)) { [] }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let function = plan.list_list_function_id(1);
        let next = expect_tail_call(plan.list_list_function(function).return_());
        let main = expect_list_list_main(&plan);

        assert_eq!(*next, function);
        assert_eq!(next.type_id(), function.type_id());
        assert_eq!(main.type_id(), function.type_id());
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

    fn expect_tail_call<Expression>(
        body: &ReturnBody<Expression, ListListFunctionId>,
    ) -> &ListListFunctionId {
        match body.kind() {
            ReturnBodyKind::TailCall { function, .. } => function,
            _ => panic!("expected a tail-call return body"),
        }
    }

    fn expect_list_list_main(plan: &ExecutionPlan) -> ListListFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::List(ListFunctionId::List(main)) => main,
            _ => panic!("expected a List(List) main function"),
        }
    }
}
