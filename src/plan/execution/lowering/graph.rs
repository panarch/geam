mod build;
mod draft;
mod freeze;
mod liveness;

pub(super) use build::{
    bit_array_expr, bit_array_function_expr, bit_array_list_expr, bool_expr, bool_function_expr,
    bool_list_expr, custom_expr, custom_expr_kind, custom_function_expr, custom_function_expr_kind,
    custom_list_expr, custom_never_expr_kind, custom_never_function_expr,
    custom_never_function_expr_kind, float_expr, float_function_expr, float_list_expr,
    function_function_expr, function_function_expr_kind, function_list_expr,
    generic_bit_array_function_expr, generic_bool_function_expr, generic_custom_function_expr,
    generic_expr, generic_float_function_expr, generic_function_expr,
    generic_function_function_expr, generic_int_function_expr, generic_list_expr,
    generic_list_function_expr, generic_never_function_expr, generic_nil_function_expr,
    generic_string_function_expr, generic_tuple_function_expr, generic_utf_codepoint_function_expr,
    int_expr, int_function_expr, int_list_expr, list_function_expr, list_list_expr, never_expr,
    nil_expr, nil_function_expr, nil_list_expr, parameter_list_list_expr, string_expr,
    string_function_expr, string_list_expr, symbolic_bit_array_function_expr,
    symbolic_bool_function_expr, symbolic_custom_function_expr_kind, symbolic_float_function_expr,
    symbolic_function_function_expr_kind, symbolic_generic_function_expr,
    symbolic_int_function_expr, symbolic_list_function_expr, symbolic_nil_function_expr,
    symbolic_string_function_expr, symbolic_tuple_function_expr,
    symbolic_utf_codepoint_function_expr, tuple_expr, tuple_function_expr, tuple_list_expr,
    tuple_never_expr, tuple_never_function_expr, utf_codepoint_expr, utf_codepoint_function_expr,
    utf_codepoint_list_expr,
};
pub(super) use draft::{
    DraftBitArray, DraftBitArrayFunction, DraftBitArrayList, DraftBool, DraftBoolFunction,
    DraftBoolList, DraftCursor, DraftCustom, DraftCustomFunction, DraftCustomList, DraftFloat,
    DraftFloatFunction, DraftFloatList, DraftFlow, DraftFunction, DraftFunctionFunction,
    DraftFunctionList, DraftFunctionValue, DraftGenericFunction, DraftGraph, DraftGraphValue,
    DraftInt, DraftIntFunction, DraftIntList, DraftList, DraftListFunction, DraftListList,
    DraftNeverFunction, DraftNil, DraftNilFunction, DraftNilList, DraftParameterList,
    DraftParameterListList, DraftString, DraftStringFunction, DraftStringList, DraftTuple,
    DraftTupleFunction, DraftTupleList, DraftUtfCodepoint, DraftUtfCodepointFunction,
    DraftUtfCodepointList, DraftValueRef, LoweredFunctionGraph,
};
pub(super) use freeze::FreezeGraphValue;

use super::LoweringContext;
use super::specialization::Representability;
use crate::plan::{execution, module};
use std::convert::Infallible;

pub(super) fn lower_function_graph<ModuleExpression, DraftReturn, FrozenReturn, TailCall>(
    template: &module::FunctionTemplate,
    body: &module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    context: &mut LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        draft::DraftCursor,
        &mut draft::DraftGraph,
        &mut LoweringContext,
    ) -> Representability<draft::DraftFlow<DraftReturn>>,
    lower_function: impl Copy
    + Fn(
        &module::FunctionInstantiation,
        &mut LoweringContext,
    ) -> Representability<TailCall>,
) -> Representability<draft::LoweredFunctionGraph<execution::FunctionBody<FrozenReturn, TailCall>>>
where
    DraftReturn: draft::DraftGraphValue + freeze::FreezeGraphValue<Frozen = FrozenReturn>,
    TailCall: Clone,
{
    build::build_function_graph(template, body, context, lower_expression, lower_function)
        .map(|graph| freeze::freeze(graph, context))
}

pub(super) fn lower_never_function_graph<ModuleExpression>(
    template: &module::FunctionTemplate,
    body: &module::ReturnBody<ModuleExpression, module::FunctionInstantiation>,
    context: &mut LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        draft::DraftCursor,
        &mut draft::DraftGraph,
        &mut LoweringContext,
    ) -> Representability<()>,
) -> Representability<
    draft::LoweredFunctionGraph<execution::FunctionBody<Infallible, execution::NeverFunctionId>>,
> {
    build::build_never_function_graph(template, body, context, lower_expression)
        .map(|graph| freeze::freeze(graph, context))
}

pub(super) fn lower_constant_graph<ModuleExpression, DraftReturn, FrozenReturn>(
    expression: &ModuleExpression,
    context: &mut LoweringContext,
    lower_expression: impl Copy
    + Fn(
        &ModuleExpression,
        draft::DraftCursor,
        &mut draft::DraftGraph,
        &mut LoweringContext,
    ) -> Representability<draft::DraftFlow<DraftReturn>>,
) -> Representability<execution::ConstantProgram<FrozenReturn>>
where
    DraftReturn: draft::DraftGraphValue + freeze::FreezeGraphValue<Frozen = FrozenReturn>,
{
    build::build_constant_graph(expression, context, lower_expression)
        .map(|graph| freeze::freeze_constant(graph, context))
}
