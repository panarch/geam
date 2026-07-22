use geam::planner::{InvalidExpressionType, InvalidTypedAstReason};
use geam::{
    ExecutionError, FunctionType, ListValue, PlanError, SourceContext, Value, ValueType,
    compile_typed_module, plan_module, plan_module_with_source, run_main,
};
use gleam_core::ast::Constant;
use miette::{GraphicalReportHandler, GraphicalTheme};

macro_rules! fixture_cases {
    ($runner:path, $dir:literal; $($name:ident),+ $(,)?) => {
        $(
            #[test]
            fn $name() {
                $runner(concat!($dir, "/", stringify!($name), ".gleam"));
            }
        )+
    };
}

macro_rules! execution_cases {
    ($dir:literal; $($name:ident),+ $(,)?) => {
        fixture_cases!(crate::run_fixture, $dir; $($name),+);
    };
}

macro_rules! rejection_cases {
    ($dir:literal; $($name:ident),+ $(,)?) => {
        fixture_cases!(crate::reject_fixture, $dir; $($name),+);
    };
}

macro_rules! execution_error_cases {
    ($dir:literal; $($name:ident),+ $(,)?) => {
        fixture_cases!(crate::run_error_fixture, $dir; $($name),+);
    };
}

macro_rules! explain_cases {
    ($($name:ident),+ $(,)?) => {
        fixture_cases!(crate::run_explain_fixture, "explain"; $($name),+);
    };
}

mod explain {
    explain_cases!(
        control_terminators,
        closure_captures,
        constant_programs,
        constant_utf_codepoint_list,
        function_instructions,
        generic_instructions,
        list_instructions,
        pattern_details,
        return_topology,
        source_stops,
        value_instructions,
        value_return_tables,
        list_return_tables,
        function_return_tables,
        list_returning_function_tables,
        return_table_group_order,
    );
}

mod values {
    execution_cases!("values";
        integer_return,
        float_value,
        bit_array_value,
        bit_array_segments,
        bit_array_composition,
        bit_array_expression_paths,
        bit_array_function_value_paths,
        bit_array_list_case_result,
        bit_array_list_function_paths,
        bit_array_returned_closure_capture,
        utf_codepoint_binding_paths,
        utf_codepoint_composition,
        utf_codepoint_expression_paths,
        utf_codepoint_function_value_paths,
        utf_codepoint_list_function_paths,
        utf_codepoint_returned_closure_capture,
        utf_codepoint_segments,
        custom_type_core,
        custom_type_composition,
        custom_type_equality_families,
        custom_runtime_handoff,
        result_value,
        tuple_value,
        tuple_expression_shapes,
        list_value,
        list_spread,
        list_expression_item_families,
        list_tuple_return,
        list_nested_return,
        list_function_return,
        primitive_case_branch_matrix,
        bool_value,
        nil_value,
        nil_expression_shapes,
    );
}

mod expressions {
    execution_cases!("expressions";
        bit_array,
        bit_array_float_16,
        bit_array_dynamic_size,
        bit_array_sized_bits,
        list_bit_array_element,
        list_utf_codepoint_element,
        record_access,
        record_access_combinations,
        record_access_families,
        record_update,
        record_update_combinations,
        record_update_families,
        result_constructor,
    );
}

mod module_items {
    execution_cases!("module_items";
        constant,
        constant_bit_array,
        constant_bit_array_segments,
        constant_list_bit_array,
        constant_value_families,
        constant_function_value,
        constant_custom_constructor,
        constant_result_constructor,
        constant_list_families,
        generic_constant_list_families,
        generic_nested_list_main,
        generic_constant_specialization,
        generic_function_constant_families,
        generic_function_constant_specialization_paths,
        generic_list_constant_specialization_paths,
        generic_symbolic_function_families,
        generic_custom_function_constant,
        generic_list_function_constant,
        generic_function_function_constant,
        custom_type,
        record_type,
        type_alias,
        type_alias_function_signature,
        type_alias_compound_signature,
        type_alias_generic_signature,
    );
}

mod bindings {
    execution_cases!("bindings";
        let_binding,
        let_discard_binding,
        float_let_binding,
        tuple_let_binding,
        list_let_binding,
        string_let_binding,
        bool_let_binding,
        nil_let_binding,
        pattern_alias_assignment,
        variable_alias_assignment,
        discard_alias_assignment,
        tuple_destructuring,
        tuple_destructuring_discard,
        nested_tuple_destructuring,
        nested_pattern_alias_assignment,
        custom_field_families,
        custom_total_binding,
        list_tail_assignment,
        let_assert_bit_array_list,
        let_assert_bit_array_pattern,
        let_assert_bit_array_patterns,
        custom_let_assert,
        bit_array_total_pattern,
        let_assert_list_destructuring,
        let_assert_fixed_list,
        let_assert_empty_list,
        let_assert_literal_pattern,
        let_assert_constructor_pattern,
        let_assert_string_prefix_pattern,
        let_assert_nested_list_pattern,
        let_assert_pattern_families,
        let_assert_discard_alias,
        final_let_assert_list,
        expression_steps,
    );
}

mod statements {
    execution_cases!("statements";
        final_assignment,
        final_discard_assignment,
        final_tuple_destructuring,
        final_pattern_alias_assignment,
        assert_statement,
        final_assert,
    );
}

mod operators {
    execution_cases!("operators";
        integer_arithmetic,
        integer_comparison,
        integer_division,
        float_arithmetic,
        float_comparison,
        float_division,
        tuple_equality,
        list_equality,
        function_equality,
        function_inequality,
        function_identity,
        function_identity_families,
        tuple_function_equality,
        list_function_equality,
        custom_function_equality,
        nested_generic_custom_function_equality,
        recursive_custom_function_equality,
        result_function_equality,
        string_concatenation,
        bool_operators,
        short_circuit_block_scope,
    );
}

mod control_flow {
    execution_cases!("control_flow";
        block_expression,
    );

    mod case {
        execution_cases!("control_flow/case";
            block_case_return_families,
            bool_case,
            bool_case_fallback,
            case_block_scope,
            float_case,
            float_case_function_values,
            float_case_return_families,
            int_case,
            string_literal_case,
            string_case_ordering,
            string_case_return_families,
            string_case_function_values,
            string_case_function_returns,
            tuple_projection_case,
            tuple_case_return_families,
            tuple_pattern,
            tuple_pattern_alias,
            tuple_pattern_nested,
            tuple_pattern_literals,
            tuple_pattern_list_and_function_values,
            tuple_pattern_guard,
            tuple_pattern_fallthrough,
            custom_pattern,
            custom_pattern_combinations,
            result_subject,
            tuple_pattern_closure_capture,
            list_case_return_families,
            tuple_subject,
            list_subject,
            nil_subject,
            function_subject,
            tuple_subject_guard,
            list_subject_closure_capture,
            list_pattern,
            list_pattern_fixed_length,
            list_pattern_empty,
            list_pattern_fallthrough,
            list_pattern_total_rest,
            list_pattern_nested,
            list_pattern_alias,
            list_pattern_alternative_guard,
            list_pattern_closure_capture,
            list_pattern_element_families,
            list_pattern_string_prefix_element,
            tuple_inner_list_pattern,
            guard,
            guard_fallthrough,
            guard_operator_surface,
            guard_subject_families,
            guard_closure_capture,
            literal_guard_ordering,
            variable_pattern,
            variable_pattern_ordering,
            variable_pattern_families,
            variable_pattern_closure_capture,
            variable_pattern_scope,
            pattern_alias,
            pattern_alias_literal_and_discard,
            pattern_alias_families,
            pattern_alias_guard,
            pattern_alias_scope,
            pattern_alias_closure_capture,
            string_prefix_pattern,
            string_prefix_whole_alias,
            string_prefix_fallthrough,
            string_prefix_left_alias,
            string_prefix_discard_suffix,
            string_prefix_guard,
            string_prefix_unicode,
            string_prefix_empty,
            tuple_inner_string_prefix_pattern,
            string_prefix_closure_capture,
            alternative_patterns,
            alternative_guard_fallthrough,
            alternative_tuple_binding_positions,
            alternative_subject_families,
            alternative_closure_capture,
            tuple_alternative_patterns,
            multiple_subjects,
            multiple_subject_variables,
            multiple_subject_alternative_guard,
            multiple_subject_mixed_patterns,
            multiple_subject_list_pattern,
            multiple_subject_closure_capture,
            bit_array_subject,
            multiple_subject_bit_array_subject,
            bit_array_pattern,
            bit_array_pattern_options,
            bit_array_pattern_integers,
            bit_array_pattern_bits,
            bit_array_pattern_dynamic_bytes,
            bit_array_pattern_floats,
            bit_array_pattern_strings,
            bit_array_pattern_composition,
            bit_array_pattern_utf_codepoint,
            tuple_inner_bit_array_pattern,
            list_inner_bit_array_pattern,
            list_inner_bit_array_pattern_fallthrough,
        );
    }
}

mod pipeline {
    execution_cases!("pipeline";
        pipeline,
        float_pipeline,
        tuple_pipeline,
        list_pipeline,
        labelled_argument,
        labelled_hole_argument,
        function_value_call,
        anonymous_function_call,
        function_value_hole_call,
        function_returning_function_call,
    );
}

mod functions {
    execution_cases!("functions";
        bit_array_argument,
        function_bit_array_argument,
        bit_array_return,
        tuple_bit_array_return,
        list_bit_array_return,
        anonymous_bit_array_argument,
        anonymous_bit_array_return,
        utf_codepoint_argument,
        utf_codepoint_return,
        result_argument,
        result_return,
        custom_function_paths,
        generic_family_specialization,
        generic_function,
        generic_function_value_family_specialization,
        generic_function_return_cases,
        generic_custom_specialization,
        generic_container_specialization,
        generic_specialization,
        generic_bool_case_families,
        generic_expression_false_cases,
        function_expression_false_cases,
        generic_never_function_handoffs,
        generic_never_function_list_identity,
        generic_never_function_runtime_handoffs,
        generic_recursive_never_function_handoffs,
        generic_recursive_never_value_handoffs,
        generic_never_function_materialization,
        generic_symbolic_handoffs,
        generic_symbolic_constructor_payload,
        generic_symbolic_values,
        generic_assert_bindings,
        generic_parameter_list_call_main,
        generic_parameter_list_empty_case,
        generic_parameter_list_function_call_main,
        generic_parameter_list_list_call_main,
        generic_parameter_list_list_constant_main,
        generic_parameter_list_list_function_call_main,
        generic_parameter_list_list_tail_binding_main,
        generic_parameter_list_list_tail_call_main,
        generic_parameter_list_tail_call_main,
        generic_parameter_list_total_tail_main,
        generic_materialized_capture_families,
        generic_returned_closure_captures,
        generic_symbolic_tail_returns,
        generic_uninhabited_custom_case,
        generic_uninhabited_parameter_function_value,
        generic_uninhabited_custom_function_list_handoff,
        generic_unused_uninhabited_function,
        generic_unresolved_list_handoffs,
        generic_nested_list_handoffs,
        generic_runtime_list_storage_paths,
        generic_custom_list_main,
        generic_function_main,
        generic_nested_result_inhabitation,
        generic_certain_custom_case_families,
        generic_uninhabited_result_functions,
        generic_phantom_main,
        unresolved_generic_main,
    );

    mod basic {
        execution_cases!("functions/basic";
            local_function_call,
            string_function_call,
            float_function_call,
            tuple_function_call,
            list_function_call,
            bool_function_call,
            nil_function_call,
            function_after_main,
            panic_body_before_main_not_called,
            panic_body_after_main_not_called,
        );
    }

    mod value {
        execution_cases!("functions/value";
            main_returning_int_function,
            main_returning_float_function,
            main_returning_string_function,
            main_returning_tuple_function,
            main_returning_bool_function,
            main_returning_nil_function,
            function_value_local,
            function_value_block_callee,
            function_value_case_callee,
            float_function_value_shapes,
            float_function_value_expressions,
            tuple_function_value_projection,
            tuple_function_value_shapes,
            tuple_function_value_expressions,
            list_function_value,
            list_function_value_shapes,
            list_function_value_expressions,
            function_value_block_list_spread,
            function_value_expression_steps,
            function_value_shadowing,
        );
    }

    mod argument {
        execution_cases!("functions/argument";
            discard_argument,
            discard_mixed_arguments,
            labelled_argument,
            labelled_argument_call,
            labelled_discard_argument,
            function_value_argument_callback,
            function_value_argument_discard,
            function_value_argument_float,
            list_function_argument,
            list_function_value_argument,
            list_boundary_item_families,
            function_value_argument_higher_order_alias,
            function_value_argument_higher_order_return_shapes,
            function_value_argument_input_shapes,
            function_value_argument_local_value,
            function_value_argument_multi_arity,
            function_value_argument_return_shapes,
        );
    }

    mod returning {
        execution_cases!("functions/returning";
            main_returning_function_returning_function,
            function_returning_function_argument,
            function_returning_function_deep,
            function_returning_function_direct_shapes,
            function_returning_float_function,
            function_returning_list_function,
            function_returning_function_recursive,
        );
    }

    mod tail_call {
        execution_cases!("functions/tail_call";
            tail_recursion_int,
            mutual_tail_recursion_bool,
            string_nil_tail_recursion,
            float_tail_recursion,
            tuple_tail_recursion,
            list_tail_recursion,
            list_tail_recursion_replaces_allocations,
            list_tail_recursion_item_families,
            block_case_tail_call,
            bit_array_return_families,
            function_returning_tail_call,
            function_returning_tail_call_families,
        );
    }

    mod anonymous {
        execution_cases!("functions/anonymous";
            anonymous_discard_argument,
            anonymous_function_local_call,
            anonymous_function_immediate_call,
            anonymous_function_argument,
            anonymous_function_return_shapes,
            anonymous_function_returning_function,
            anonymous_float_function,
            anonymous_list_function,
            anonymous_assert_statement,
            anonymous_function_main_returning_function,
            anonymous_todo_body_not_called,
            function_capture_literal,
            function_capture_labelled_argument,
            function_capture_literal_first_argument,
            function_capture_literal_closure,
            function_capture_literal_local_function_value,
            capturing_closure_local_call,
            capturing_closure_float,
            capturing_closure_list,
            capturing_closure_block_scope,
            capturing_closure_nested,
            capturing_closure_shadowing,
            capturing_closure_value_families,
            capturing_closure_tuple,
            capturing_closure_return_shapes,
            capturing_closure_list_function,
            capturing_closure_nested_list,
        );
    }

    mod use_syntax {
        execution_cases!("functions/use";
            use_no_assignment,
            use_value,
            use_labelled_argument,
            use_discard_assignment,
            use_multiple_assignments,
            use_nested,
            use_capture,
            use_float_value,
            use_tuple_value,
            use_list_value,
            use_tuple_assignment,
            use_list_tail_assignment,
            use_bit_array_total_pattern,
            use_pattern_alias_assignment,
            use_nested_tuple_alias_assignment,
            use_block_scope,
            use_function_value_provider,
            use_inside_anonymous_function,
        );
    }
}

mod execution_errors {
    mod expressions {
        execution_error_cases!("expressions";
            panic,
            panic_message,
            panic_assignment,
            panic_int,
            panic_string,
            panic_bit_array,
            panic_utf_codepoint,
            panic_float,
            panic_bool,
            panic_tuple,
            panic_list,
            panic_list_string,
            panic_list_bit_array,
            panic_list_utf_codepoint,
            panic_list_float,
            panic_list_bool,
            panic_list_nil,
            panic_list_tuple,
            panic_list_nested,
            panic_function_list,
            panic_int_function,
            panic_string_function,
            panic_bit_array_function,
            panic_utf_codepoint_function,
            panic_float_function,
            panic_bool_function,
            panic_nil_function,
            panic_tuple_function,
            panic_list_function,
            panic_function_function,
            panic_generic_function,
            panic_generic_list,
            panic_generic_list_element,
            panic_generic_list_spread,
            panic_nested_generic_list_element,
            panic_nested_generic_list_item,
            constant_bit_array_segment_failure,
            panic_generic_local_return,
            panic_generic_tuple_local_return,
            panic_generic_custom_field,
            panic_generic_tuple_projection,
            panic_generic_custom_projection,
            todo,
            todo_utf_codepoint,
            todo_list_utf_codepoint,
            todo_utf_codepoint_function,
            todo_message,
            todo_assignment,
            empty_function,
            empty_block,
            bit_array_invalid_float_size,
            bit_array_fixed_bits_insufficient,
            bit_array_dynamic_bits_insufficient,
            bit_array_size_out_of_range,
            bit_array_float_size_out_of_range,
            bit_array_bits_size_out_of_range,
            bit_array_size_expression_panic,
            bit_array_value_before_size_panic,
            bit_array_dynamic_int_value_panic,
            bit_array_dynamic_int_size_panic,
            bit_array_dynamic_float_value_panic,
            bit_array_dynamic_float_size_panic,
            bit_array_static_int_value_panic,
            bit_array_static_float_value_panic,
            bit_array_static_string_value_panic,
            bit_array_static_utf_codepoint_value_panic,
            bit_array_static_bits_value_panic,
        );
    }

    mod functions {
        execution_error_cases!("functions";
            generic_unresolved_argument,
            custom_function_binding_failure,
            generic_unresolved_discarded_value,
            generic_unresolved_direct_argument,
            generic_unresolved_equality_operand,
            generic_unresolved_false_case_operand,
            generic_unresolved_tuple_equality_operand,
            generic_unresolved_never_function_prefix,
            generic_diverging_function_call_family_lowering,
            generic_symbolic_function_call_family_lowering,
            generic_concrete_function_specialization_divergence,
            generic_uninhabited_function_call_handoffs,
            generic_uninhabited_custom_specialization,
            generic_symbolic_function_binding_failure,
            generic_function_binding_projection_failure,
            generic_symbolic_function_argument_failure,
            generic_unresolved_generic_function_argument,
            generic_unresolved_function_argument,
            generic_unresolved_int_function_argument,
            generic_unresolved_float_function_argument,
            generic_unresolved_string_function_argument,
            generic_unresolved_bit_array_function_argument,
            generic_unresolved_utf_codepoint_function_argument,
            generic_unresolved_custom_function_argument,
            generic_unresolved_bool_function_argument,
            generic_unresolved_nil_function_argument,
            generic_unresolved_tuple_function_argument,
            generic_unresolved_list_function_argument,
            generic_unresolved_function_function_argument,
            generic_typed_diverging_function_arguments,
            generic_typed_diverging_returned_function_arguments,
            generic_never_direct_argument_failure,
            generic_never_function_argument_failure,
            generic_never_function_binding_failure,
            generic_never_function_binding_projection_failure,
            generic_never_function_list_element_failure,
            generic_never_function_value_argument_failure,
            generic_never_returned_function_argument_failure,
            generic_never_returned_function_value_argument_failure,
            generic_never_recursion,
            generic_never_todo,
            generic_never_function_call,
            generic_never_function_callee_failure,
            generic_never_closure_call,
            generic_never_function_panic,
            generic_symbolic_function_panic,
            generic_recursive_never_block_handoffs,
            generic_never_bool_case,
            generic_never_int_case,
            generic_never_string_case,
            generic_never_float_case,
            generic_never_block,
            generic_never_let_assert,
            generic_never_list_case,
            generic_never_custom_case,
            generic_certain_custom_never_case,
            generic_never_return_cases,
            return_bool_case_subject,
            return_int_case_subject,
            return_float_case_subject,
            return_string_case_subject,
            return_block_step,
            utf_codepoint_return_tuple_subject,
            utf_codepoint_return_bool_case_subject,
            utf_codepoint_return_int_case_subject,
            utf_codepoint_return_string_case_subject,
            utf_codepoint_return_float_case_subject,
            utf_codepoint_return_block_step,
            utf_codepoint_list_call_argument,
            utf_codepoint_let_step,
        );

        mod use_syntax {
            execution_error_cases!("functions/use";
                incomplete_use,
            );
        }
    }

    mod patterns {
        execution_error_cases!("patterns";
            let_assert_empty_head,
            let_assert_uninhabited_list_item,
            let_assert_uninhabited_list_alias,
            let_assert_uninhabited_list_whole_alias,
            let_assert_uninhabited_list_tail,
            let_assert_uninhabited_custom_alias,
            let_assert_nested_prefix,
            let_assert_bound_tail_prefix,
            let_assert_compound_failed_value,
            let_assert_fixed_length,
            let_assert_empty_list,
            let_assert_message,
            let_assert_custom_pattern,
            let_assert_bit_array_pattern,
            let_assert_bit_array_utf_codepoint_default_message,
            let_assert_bit_array_utf_codepoint_message_error,
            let_assert_literal_pattern,
            let_assert_bool_pattern,
            let_assert_string_prefix_pattern,
            let_assert_nested_compound_pattern,
            let_assert_message_failure,
        );
    }

    mod statements {
        execution_error_cases!("statements";
            assert_statement,
            assert_message,
            assert_message_before_condition,
            assert_condition_error_after_message,
        );
    }
}

mod rejection {
    mod module_items {
        rejection_cases!("module_items";
            import,
            external_function,
            external_custom_type,
        );
    }

    mod entrypoint {
        rejection_cases!("entrypoint";
            missing_main,
            main_with_arguments,
        );
    }

    mod functions {
        rejection_cases!("functions";
            unsupported_body_before_main,
            unsupported_body_after_main,
        );
    }

    mod expressions {
        rejection_cases!("expressions";
            echo,
            bit_array_native_endian,
        );
    }

    mod case_patterns {
        rejection_cases!("case_patterns";
            bit_array_pattern_native_endian,
        );
    }

    mod pipeline {
        rejection_cases!("pipeline";
            echo,
        );
    }
}

#[test]
fn public_empty_parameter_list_preserves_inspected_module_metadata() {
    let typed = compile_typed_module(
        "main",
        "main.gleam",
        "fn identity(value: value) { value } pub fn main() { identity(1) }",
    )
    .expect("source should compile");
    let plan = plan_module(typed).expect("source should plan");
    let parameter = plan.functions()[0].scheme().parameters()[0];
    let list = ListValue::empty(ValueType::Parameter(parameter));

    assert_eq!(list.item_type(), ValueType::Parameter(parameter));
    assert_eq!(list.len(), 0);
    assert!(list.is_empty());
    assert_eq!(list.to_values(), Vec::<Value>::new());
}

#[test]
fn public_generic_constant_preserves_inspected_module_metadata() {
    let typed = compile_typed_module(
        "main",
        "main.gleam",
        "const empty = [] pub fn main() { empty }",
    )
    .expect("source should compile");
    let plan = plan_module(typed).expect("source should plan");

    assert_eq!(plan.constants().len(), 1);
    assert_eq!(plan.constants()[0].name(), "empty");
    assert_eq!(plan.constants()[0].scheme().parameters().len(), 1);
}

#[test]
fn generic_constant_rejects_inhabited_typed_ast_payloads() {
    let value_module = compile_typed_module(
        "main",
        "value.gleam",
        "const populated = [1] pub fn main() { populated }",
    )
    .expect("inhabited list source should compile");
    let value = value_module.definitions.constants[0].value.clone();
    let mut generic_value_module = compile_typed_module(
        "main",
        "generic_value.gleam",
        "const empty = [] pub fn main() { empty }",
    )
    .expect("generic empty list source should compile");
    generic_value_module.definitions.constants[0].value = value;
    let generic_value_type = generic_value_module.definitions.constants[0].type_.clone();
    let Constant::List { type_, .. } = generic_value_module.definitions.constants[0].value.as_mut()
    else {
        panic!("compiled inhabited value should remain a list constant");
    };
    *type_ = generic_value_type;

    let spread_module = compile_typed_module(
        "main",
        "spread.gleam",
        "const populated = [1, ..[2]] pub fn main() { populated }",
    )
    .expect("inhabited list spread source should compile");
    let spread = spread_module
        .definitions
        .constants
        .into_iter()
        .find(|constant| constant.name == "populated")
        .expect("compiled spread constant should be present")
        .value;
    let mut generic_spread_module = compile_typed_module(
        "main",
        "generic_spread.gleam",
        "const empty = [] pub fn main() { empty }",
    )
    .expect("generic empty list source should compile");
    generic_spread_module.definitions.constants[0].value = spread;
    let generic_spread_type = generic_spread_module.definitions.constants[0].type_.clone();
    let Constant::List { type_, .. } = generic_spread_module.definitions.constants[0]
        .value
        .as_mut()
    else {
        panic!("compiled inhabited spread should remain a list constant");
    };
    *type_ = generic_spread_type;

    let expected = Err(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::TypeParameter,
            actual: InvalidExpressionType::Int,
        },
    });
    assert_eq!(plan_module(generic_value_module), expected);
    assert_eq!(plan_module(generic_spread_module), expected);
}

#[test]
fn constant_list_rejects_mismatched_typed_ast_tail() {
    let string_module = compile_typed_module(
        "main",
        "strings.gleam",
        "const strings = [\"two\"] pub fn main() { strings }",
    )
    .expect("String list source should compile");
    let string_tail = string_module.definitions.constants[0].value.clone();
    let mut int_module = compile_typed_module(
        "main",
        "ints.gleam",
        "const ints = [1, ..[2]] pub fn main() { ints }",
    )
    .expect("Int list spread source should compile");
    let Constant::List {
        tail: Some(tail), ..
    } = int_module.definitions.constants[0].value.as_mut()
    else {
        panic!("compiled Int spread should retain its tail");
    };
    *tail = string_tail;

    assert_eq!(
        plan_module(int_module),
        Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::List,
                actual: InvalidExpressionType::List,
            },
        }),
    );
}

#[test]
fn constant_nested_list_rejects_mismatched_typed_ast_elements() {
    let scalar_module = compile_typed_module(
        "main",
        "scalar.gleam",
        "const scalar = 1 pub fn main() { scalar }",
    )
    .expect("Int constant source should compile");
    let scalar = scalar_module.definitions.constants[0].value.clone();
    let string_list_module = compile_typed_module(
        "main",
        "strings.gleam",
        "const strings = [\"one\"] pub fn main() { strings }",
    )
    .expect("String list constant source should compile");
    let string_list = string_list_module.definitions.constants[0].value.clone();

    let mut scalar_element_module = compile_typed_module(
        "main",
        "scalar_element.gleam",
        "const nested = [[1]] pub fn main() { nested }",
    )
    .expect("nested Int list source should compile");
    let Constant::List { elements, .. } = scalar_element_module.definitions.constants[0]
        .value
        .as_mut()
    else {
        panic!("compiled nested value should remain a list constant");
    };
    elements[0] = *scalar;

    let mut list_element_module = compile_typed_module(
        "main",
        "list_element.gleam",
        "const nested = [[1]] pub fn main() { nested }",
    )
    .expect("nested Int list source should compile");
    let Constant::List { elements, .. } =
        list_element_module.definitions.constants[0].value.as_mut()
    else {
        panic!("compiled nested value should remain a list constant");
    };
    elements[0] = *string_list;

    assert_eq!(
        plan_module(scalar_element_module),
        Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::List,
                actual: InvalidExpressionType::Int,
            },
        }),
    );
    assert_eq!(
        plan_module(list_element_module),
        Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::List,
                actual: InvalidExpressionType::List,
            },
        }),
    );
}

fn expected_explanation(source: &str) -> String {
    let (_, comments) = source
        .split_once("\n// geam:explain\n")
        .expect("explain fixture should contain an expected output block");
    let mut expected = String::new();

    for line in comments.lines() {
        let comment = line
            .strip_prefix("//")
            .expect("expected output lines should be comments");
        expected.push_str(comment.strip_prefix(' ').unwrap_or(comment));
        expected.push('\n');
    }

    expected
}

fn run_explain_fixture(file_name: &str) {
    let path = format!("tests/fixtures/{file_name}");
    let source = std::fs::read_to_string(&path).expect("fixture should be readable");
    let expected = expected_explanation(&source);
    let module = compile_typed_module("main", path, &source).expect("fixture should compile");
    let module_plan = plan_module(module).expect("fixture should plan");
    let plan = geam::ExecutionPlan::from_module_plan(module_plan);

    assert_eq!(plan.explain().to_string(), expected);
}

fn run_fixture(file_name: &str) {
    let path = format!("tests/fixtures/execution/{file_name}");
    let src = std::fs::read_to_string(&path).expect("fixture should be readable");
    let expected = expected_text_with_prefix(&src, "// geam:expect ");
    let module = compile_typed_module("main", path, &src).expect("fixture should compile");
    let module_plan = plan_module(module).expect("fixture should plan");
    let plan = geam::ExecutionPlan::from_module_plan(module_plan);
    let actual = run_main(&plan).expect("fixture should run");

    assert_eq!(render_value(&actual), expected);
}

fn run_error_fixture(file_name: &str) {
    let path = format!("tests/fixtures/execution_errors/{file_name}");
    let src = std::fs::read_to_string(&path).expect("fixture should be readable");
    let expected = expected_error_text(&src);
    let module = compile_typed_module("main", path, &src).expect("fixture should compile");
    let source_context = SourceContext::new(
        format!("tests/fixtures/execution_errors/{file_name}"),
        src.clone(),
    );
    let module_plan = plan_module_with_source(module, source_context).expect("fixture should plan");
    let plan = geam::ExecutionPlan::from_module_plan(module_plan);
    let error = run_main(&plan).expect_err("fixture should fail during execution");
    assert!(
        matches!(error, ExecutionError::Panic(_)),
        "execution-error fixture should fail with source panic"
    );

    assert_eq!(render_execution_error(&error), expected);
}

fn reject_fixture(file_name: &str) {
    let path = format!("tests/fixtures/rejection/{file_name}");
    let src = std::fs::read_to_string(&path).expect("fixture should be readable");
    let module = compile_typed_module("main", path, &src).expect("fixture should compile");

    assert!(plan_module(module).is_err());
}

fn expected_text_with_prefix<'a>(src: &'a str, prefix: &str) -> &'a str {
    let line = src
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .expect("fixture should not be empty")
        .trim();
    let Some(value) = line.strip_prefix(prefix) else {
        panic!("last non-empty fixture line must start with `{prefix}`");
    };

    value
}

fn expected_error_text(src: &str) -> String {
    let mut lines = src.lines();
    for line in lines.by_ref() {
        let line = line.trim();
        if line == "// geam:expect-error" {
            let mut expected = lines
                .map(|line| {
                    let line = line.trim_start();
                    if line == "//" {
                        String::new()
                    } else if let Some(line) = line.strip_prefix("// ") {
                        line.to_string()
                    } else {
                        panic!(
                            "error fixture expected block lines must be comments after `// geam:expect-error`"
                        );
                    }
                })
                .collect::<Vec<_>>();

            while expected.last().is_some_and(String::is_empty) {
                expected.pop();
            }
            assert!(
                !expected.is_empty(),
                "error fixture expected block must not be empty"
            );
            return expected.join("\n");
        }
    }

    panic!("fixture should include `// geam:expect-error`");
}

fn render_execution_error(error: &ExecutionError) -> String {
    let handler = GraphicalReportHandler::new_themed(GraphicalTheme::none())
        .with_links(false)
        .with_urls(false)
        .without_cause_chain()
        .without_syntax_highlighting()
        .with_context_lines(1)
        .with_width(120)
        .with_wrap_lines(false)
        .with_break_words(false);
    let mut rendered = String::new();
    handler
        .render_report(&mut rendered, error)
        .expect("diagnostic should render");

    rendered.trim_end().to_string()
}

fn render_value(value: &Value) -> String {
    match value {
        Value::Int(value) => format!("Int({value})"),
        Value::Float(value) => format!("Float({value:?})"),
        Value::String(value) => format!("String({value:?})"),
        Value::BitArray(value) => format!(
            "BitArray(bytes={:?}, bit_len={})",
            value.bytes(),
            value.bit_len(),
        ),
        Value::UtfCodepoint(value) => format!("UtfCodepoint({value:?})"),
        Value::Custom(value) => format!(
            "Custom(type={}, constructor={}#{}, fields=[{}])",
            render_custom_type(value.type_()),
            value.constructor_name(),
            value.constructor_index(),
            value
                .fields()
                .iter()
                .map(|field| match field.label() {
                    Some(label) => format!("{label}: {}", render_value(field.value())),
                    None => render_value(field.value()),
                })
                .collect::<Vec<_>>()
                .join(", "),
        ),
        Value::Bool(value) => format!("Bool({value})"),
        Value::Nil => "Nil".into(),
        Value::Tuple(values) => format!(
            "Tuple([{}])",
            values
                .iter()
                .map(render_value)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Value::List(value) => format!(
            "List({})([{}])",
            render_value_type(&value.item_type()),
            value
                .to_values()
                .iter()
                .map(render_value)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        Value::Function(function) => {
            let type_ = function.type_();
            format!("Function({})", render_function_type(&type_))
        }
    }
}

fn render_function_type(type_: &FunctionType) -> String {
    let arguments = type_
        .argument_types()
        .iter()
        .map(render_value_type)
        .collect::<Vec<_>>()
        .join(", ");

    format!("fn({arguments}) -> {}", render_value_type(type_.return_()))
}

fn render_value_type(type_: &ValueType) -> String {
    match type_ {
        ValueType::Parameter(parameter) => format!("Parameter({})", parameter.index()),
        ValueType::Int => "Int".into(),
        ValueType::Float => "Float".into(),
        ValueType::String => "String".into(),
        ValueType::BitArray => "BitArray".into(),
        ValueType::UtfCodepoint => "UtfCodepoint".into(),
        ValueType::Custom(type_) => render_custom_type(type_),
        ValueType::Bool => "Bool".into(),
        ValueType::Nil => "Nil".into(),
        ValueType::Tuple(elements) => format!(
            "#({})",
            elements
                .iter()
                .map(render_value_type)
                .collect::<Vec<_>>()
                .join(", ")
        ),
        ValueType::List(element) => format!("List({})", render_value_type(element)),
        ValueType::Function(type_) => render_function_type(type_),
    }
}

fn render_custom_type(type_: &geam::CustomType) -> String {
    let name = type_.type_name();
    let arguments = type_
        .arguments()
        .iter()
        .map(render_value_type)
        .collect::<Vec<_>>();
    let identity = format!("{}/{}/{}", name.package(), name.module(), name.name());
    if arguments.is_empty() {
        identity
    } else {
        format!("{identity}({})", arguments.join(", "))
    }
}
