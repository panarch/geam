use geam::{FunctionType, Value, ValueType, compile_typed_module, plan_module, run_main};

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

mod values {
    execution_cases!("values";
        integer_return,
        bool_value,
        nil_value,
    );
}

mod bindings {
    execution_cases!("bindings";
        let_binding,
        string_let_binding,
        bool_let_binding,
        nil_let_binding,
        expression_steps,
    );
}

mod operators {
    execution_cases!("operators";
        integer_arithmetic,
        integer_comparison,
        integer_division,
        string_concatenation,
        bool_operators,
        short_circuit_block_scope,
    );
}

mod control_flow {
    execution_cases!("control_flow";
        block_expression,
        case_block_scope,
        block_case_return_families,
        bool_case,
        bool_case_fallback,
        int_case,
    );
}

mod pipeline {
    execution_cases!("pipeline";
        pipeline,
    );
}

mod functions {
    mod basic {
        execution_cases!("functions/basic";
            local_function_call,
            string_function_call,
            bool_function_call,
            nil_function_call,
            function_after_main,
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
            function_value_shadowing,
        );
    }

    mod argument {
        execution_cases!("functions/argument";
            function_value_argument_callback,
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
            function_returning_function_recursive,
        );
    }

    mod tail_call {
        execution_cases!("functions/tail_call";
            tail_recursion_int,
            mutual_tail_recursion_bool,
            string_nil_tail_recursion,
            block_case_tail_call,
            function_returning_tail_call,
            function_returning_tail_call_families,
        );
    }

    mod anonymous {
        execution_cases!("functions/anonymous";
            anonymous_function_local_call,
            anonymous_function_immediate_call,
            anonymous_function_argument,
            anonymous_function_return_shapes,
            anonymous_function_returning_function,
            anonymous_function_main_returning_function,
            capturing_closure_local_call,
            capturing_closure_block_scope,
            capturing_closure_nested,
            capturing_closure_shadowing,
            capturing_closure_value_families,
            capturing_closure_return_shapes,
        );
    }

    mod use_syntax {
        execution_cases!("functions/use";
            use_no_assignment,
            use_value,
            use_multiple_assignments,
            use_nested,
            use_capture,
            use_block_scope,
            use_function_value_provider,
            use_inside_anonymous_function,
        );
    }
}

mod rejection {
    mod top_level {
        rejection_cases!("top_level";
            top_level_import,
            top_level_constant,
            top_level_custom_type,
            top_level_type_alias,
        );
    }

    mod main {
        rejection_cases!("main";
            missing_main,
            main_with_arguments,
            main_unsupported_body,
        );
    }

    mod function {
        rejection_cases!("function";
            external_function,
            function_unsupported_return_type,
            function_before_main_unsupported_body,
            function_after_main_unsupported_body,
        );
    }

    mod argument {
        rejection_cases!("argument";
            argument_discard,
            argument_labelled,
            argument_unsupported_type,
        );
    }

    mod anonymous {
        rejection_cases!("anonymous";
            anonymous_discard_argument,
            anonymous_assert_statement,
            anonymous_unsupported_body,
            anonymous_unsupported_return_type,
            function_capture_literal,
        );
    }

    mod use_syntax {
        rejection_cases!("use";
            use_discard_assignment,
        );
    }
}

fn run_fixture(file_name: &str) {
    let path = format!("tests/fixtures/execution/{file_name}");
    let src = std::fs::read_to_string(&path).expect("fixture should be readable");
    let expected = expected_text(&src);
    let module = compile_typed_module("main", path, &src).expect("fixture should compile");
    let plan = plan_module(module).expect("fixture should plan");
    let actual = run_main(&plan).expect("fixture should run");

    assert_eq!(render_value(&actual), expected);
}

fn reject_fixture(file_name: &str) {
    let path = format!("tests/fixtures/rejection/{file_name}");
    let src = std::fs::read_to_string(&path).expect("fixture should be readable");
    let module = compile_typed_module("main", path, &src).expect("fixture should compile");

    assert!(plan_module(module).is_err());
}

fn expected_text(src: &str) -> &str {
    let line = src
        .lines()
        .rev()
        .find(|line| !line.trim().is_empty())
        .expect("fixture should not be empty")
        .trim();
    let Some(value) = line.strip_prefix("// geam:expect ") else {
        panic!("last non-empty fixture line must start with `// geam:expect `");
    };

    value
}

fn render_value(value: &Value) -> String {
    match value {
        Value::Int(value) => format!("Int({value})"),
        Value::String(value) => format!("String({value:?})"),
        Value::Bool(value) => format!("Bool({value})"),
        Value::Nil => "Nil".into(),
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
        ValueType::String => "String".into(),
        ValueType::Bool => "Bool".into(),
        ValueType::Nil => "Nil".into(),
        ValueType::Function(type_) => render_function_type(type_),
    }
}
