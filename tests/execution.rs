use geam::{
    ExecutionError, FunctionType, Panic, SourceContext, Value, ValueType, compile_typed_module,
    plan_module, plan_module_with_source, run_main,
};
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

mod values {
    execution_cases!("values";
        integer_return,
        float_value,
        tuple_value,
        list_value,
        list_spread,
        bool_value,
        nil_value,
    );
}

mod module_items {
    execution_cases!("module_items";
        constant,
        constant_value_families,
        constant_function_value,
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
        let_assert_list_destructuring,
        let_assert_fixed_list,
        let_assert_empty_list,
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
            list_case_return_families,
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
            main_returning_string_function,
            main_returning_bool_function,
            main_returning_nil_function,
            function_value_local,
            function_value_block_callee,
            function_value_case_callee,
            float_function_value_shapes,
            float_function_value_expressions,
            tuple_function_value_projection,
            list_function_value,
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
            block_case_tail_call,
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
            panic_float,
            panic_bool,
            panic_tuple,
            panic_list,
            panic_int_function,
            panic_string_function,
            panic_float_function,
            panic_bool_function,
            panic_nil_function,
            panic_tuple_function,
            panic_list_function,
            panic_function_function,
            todo,
            todo_message,
            todo_assignment,
            empty_function,
            empty_block,
        );
    }

    mod functions {
        mod use_syntax {
            execution_error_cases!("functions/use";
                incomplete_use,
            );
        }
    }

    mod patterns {
        execution_error_cases!("patterns";
            let_assert_empty_head,
            let_assert_nested_prefix,
            let_assert_compound_failed_value,
            let_assert_fixed_length,
            let_assert_empty_list,
            let_assert_message,
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
            constant_bit_array,
            constant_list_bit_array,
            constant_result_constructor,
            custom_type,
            external_function,
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
            generic_function,
            unsupported_argument_type,
            unsupported_function_argument_type,
            unsupported_return_type,
            unsupported_tuple_return_type,
            unsupported_list_return_type,
            unsupported_body_before_main,
            unsupported_body_after_main,
            anonymous_unsupported_argument_type,
            anonymous_unsupported_return_type,
        );
    }

    mod expressions {
        rejection_cases!("expressions";
            echo,
            bit_array,
            result_constructor,
            unsupported_list_element_type,
        );
    }

    mod operators {
        rejection_cases!("operators";
            function_equality,
            tuple_function_equality,
            list_function_equality,
        );
    }

    mod patterns {
        rejection_cases!("patterns";
            list_assignment,
            use_list_assignment,
        );
    }

    mod case_patterns {
        rejection_cases!("case_patterns";
            guard,
            alternative_patterns,
            multiple_subjects,
            variable_pattern,
            tuple_pattern,
            tuple_subject,
            string_prefix_pattern,
            list_pattern,
            pattern_alias,
            unsupported_subject_type,
        );
    }

    mod pipeline {
        rejection_cases!("pipeline";
            echo,
        );
    }
}

fn run_fixture(file_name: &str) {
    let path = format!("tests/fixtures/execution/{file_name}");
    let src = std::fs::read_to_string(&path).expect("fixture should be readable");
    let expected = expected_text_with_prefix(&src, "// geam:expect ");
    let module = compile_typed_module("main", path, &src).expect("fixture should compile");
    let plan = plan_module(module).expect("fixture should plan");
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
    let plan = plan_module_with_source(module, source_context).expect("fixture should plan");
    let error = run_main(&plan).expect_err("fixture should fail during execution");
    let ExecutionError::Panic(panic) = error else {
        panic!("execution-error fixture should fail with source panic");
    };

    assert_eq!(render_panic(&panic), expected);
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

fn render_panic(panic: &Panic) -> String {
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
        .render_report(&mut rendered, panic)
        .expect("diagnostic should render");

    rendered.trim_end().to_string()
}

fn render_value(value: &Value) -> String {
    match value {
        Value::Int(value) => format!("Int({value})"),
        Value::Float(value) => format!("Float({value:?})"),
        Value::String(value) => format!("String({value:?})"),
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
            render_value_type(value.element_type()),
            value
                .values()
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
        ValueType::Int => "Int".into(),
        ValueType::Float => "Float".into(),
        ValueType::String => "String".into(),
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
