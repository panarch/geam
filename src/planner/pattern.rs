mod bit_array;
mod runtime;
mod validation;

pub(in crate::planner) use bit_array::plan_bit_array_pattern;
pub(in crate::planner) use runtime::{
    PlannedCustomBinding, pattern_value_type_in_context, plan_custom_subject_pattern,
    plan_runtime_pattern_with_source_shape,
};
pub(in crate::planner) use validation::{
    ValidatedListTail, pattern_kind, pattern_type_mismatch, pattern_value_shape,
    pattern_value_type_from_gleam, resolved_constructor, unexpected_pattern,
    validate_constructor_arity, validate_list_pattern, validate_list_tail, validate_pattern,
    validate_pattern_value_type, validate_tuple_arity, validate_tuple_pattern,
};
