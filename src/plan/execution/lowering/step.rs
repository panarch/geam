use super::expression::{
    bool_expr, expr, float_expr, function_function_expr, int_expr, int_function_expr,
    list_function_expr, nil_expr, nil_function_expr, string_expr, string_function_expr, tuple_expr,
    tuple_function_expr,
};
use super::id::{list_function_local, list_local};
use super::param::param_local;
use crate::plan::{execution, module};

pub(super) fn steps(steps: Vec<module::Step>) -> Vec<execution::Step> {
    steps.into_iter().map(step).collect()
}

fn step(step: module::Step) -> execution::Step {
    use execution::StepKind as E;
    use module::StepKind as M;

    execution::Step::from_kind(match step.into_kind() {
        M::LetInt {
            local,
            name: _,
            value,
        } => E::LetInt {
            local: execution::IntLocalId(local.0),
            value: int_expr(value),
        },
        M::LetFloat {
            local,
            name: _,
            value,
        } => E::LetFloat {
            local: execution::FloatLocalId(local.0),
            value: float_expr(value),
        },
        M::LetString {
            local,
            name: _,
            value,
        } => E::LetString {
            local: execution::StringLocalId(local.0),
            value: string_expr(value),
        },
        M::LetBool {
            local,
            name: _,
            value,
        } => E::LetBool {
            local: execution::BoolLocalId(local.0),
            value: bool_expr(value),
        },
        M::LetNil {
            local,
            name: _,
            value,
        } => E::LetNil {
            local: execution::NilLocalId(local.0),
            value: nil_expr(value),
        },
        M::LetTuple {
            local,
            name: _,
            value,
        } => E::LetTuple {
            local: execution::TupleLocalId(local.0),
            value: tuple_expr(value),
        },
        M::LetList { name: _, value } => E::LetList {
            value: super::expression::list_local_expr(value),
        },
        M::LetIntFunction {
            local,
            name: _,
            value,
        } => E::LetIntFunction {
            local: execution::IntFunctionLocalId(local.0),
            value: int_function_expr(value),
        },
        M::LetFloatFunction {
            local,
            name: _,
            value,
        } => E::LetFloatFunction {
            local: execution::FloatFunctionLocalId(local.0),
            value: super::expression::float_function_expr(value),
        },
        M::LetStringFunction {
            local,
            name: _,
            value,
        } => E::LetStringFunction {
            local: execution::StringFunctionLocalId(local.0),
            value: string_function_expr(value),
        },
        M::LetBoolFunction {
            local,
            name: _,
            value,
        } => E::LetBoolFunction {
            local: execution::BoolFunctionLocalId(local.0),
            value: super::expression::bool_function_expr(value),
        },
        M::LetNilFunction {
            local,
            name: _,
            value,
        } => E::LetNilFunction {
            local: execution::NilFunctionLocalId(local.0),
            value: nil_function_expr(value),
        },
        M::LetTupleFunction {
            local,
            name: _,
            value,
        } => E::LetTupleFunction {
            local: execution::TupleFunctionLocalId(local.0),
            value: tuple_function_expr(value),
        },
        M::LetListFunction {
            local,
            name: _,
            value,
        } => E::LetListFunction {
            local: list_function_local(local),
            value: list_function_expr(value),
        },
        M::LetFunctionFunction {
            local,
            name: _,
            value,
        } => E::LetFunctionFunction {
            local: execution::FunctionFunctionLocalId(local.0),
            value: function_function_expr(value),
        },
        M::AssertList {
            local,
            pattern,
            message,
            site,
            pattern_span,
        } => E::AssertList {
            local: list_local(local),
            pattern: assert_pattern(pattern),
            message: message.map(string_expr),
            site,
            pattern_span,
        },
        M::AssertBool {
            condition,
            message,
            site,
        } => E::AssertBool {
            condition: bool_expr(condition),
            message: message.map(string_expr),
            site,
        },
        M::Evaluate(value) => E::Evaluate(expr(value)),
    })
}

fn assert_pattern(pattern: module::AssertPattern) -> execution::AssertPattern {
    match pattern {
        module::AssertPattern::Bind(binding) => {
            execution::AssertPattern::Bind(assert_binding(binding))
        }
        module::AssertPattern::Discard => execution::AssertPattern::Discard,
        module::AssertPattern::Tuple(elements) => {
            execution::AssertPattern::Tuple(elements.into_iter().map(assert_pattern).collect())
        }
        module::AssertPattern::List(pattern) => {
            let (_element_type, elements, tail) = pattern.into_parts();
            execution::AssertPattern::List(execution::ListAssertPattern::new(
                elements.into_iter().map(assert_pattern).collect(),
                tail.map(assert_tail),
            ))
        }
        module::AssertPattern::Alias { pattern, binding } => execution::AssertPattern::Alias {
            pattern: Box::new(assert_pattern(*pattern)),
            binding: assert_binding(binding),
        },
    }
}

fn assert_binding(binding: module::AssertBinding) -> execution::AssertBinding {
    let (local, _) = binding.into_parts();
    execution::AssertBinding::new(param_local(local))
}

fn assert_tail(tail: module::ListAssertTail) -> execution::ListAssertTail {
    match tail {
        module::ListAssertTail::Ignore => execution::ListAssertTail::Ignore,
        module::ListAssertTail::Bind(binding) => {
            let (local, _) = binding.into_parts();
            execution::ListAssertTail::bind(list_local(local))
        }
    }
}
