use super::expression::{
    bit_array_expr, bit_array_function_expr, bool_expr, expr, float_expr, function_function_expr,
    int_expr, int_function_expr, list_function_expr, nil_expr, nil_function_expr, string_expr,
    string_function_expr, tuple_expr, tuple_function_expr,
};
use super::id::{list_function_local, list_local};
use super::param::param_local;
use crate::plan::{execution, module};

pub(super) fn steps(
    steps: Vec<module::Step>,
    context: &mut super::LoweringContext,
) -> Vec<execution::Step> {
    steps
        .into_iter()
        .map(|step| lower_step(step, context))
        .collect()
}

fn lower_step(step: module::Step, context: &mut super::LoweringContext) -> execution::Step {
    use execution::StepKind as E;
    use module::StepKind as M;

    execution::Step::from_kind(match step.into_kind() {
        M::LetInt {
            local,
            name: _,
            value,
        } => E::LetInt {
            local: execution::IntLocalId(local.0),
            value: int_expr(value, context),
        },
        M::LetFloat {
            local,
            name: _,
            value,
        } => E::LetFloat {
            local: execution::FloatLocalId(local.0),
            value: float_expr(value, context),
        },
        M::LetString {
            local,
            name: _,
            value,
        } => E::LetString {
            local: execution::StringLocalId(local.0),
            value: string_expr(value, context),
        },
        M::LetBitArray {
            local,
            name: _,
            value,
        } => E::LetBitArray {
            local: execution::BitArrayLocalId(local.0),
            value: bit_array_expr(value, context),
        },
        M::LetBool {
            local,
            name: _,
            value,
        } => E::LetBool {
            local: execution::BoolLocalId(local.0),
            value: bool_expr(value, context),
        },
        M::LetNil {
            local,
            name: _,
            value,
        } => E::LetNil {
            local: execution::NilLocalId(local.0),
            value: nil_expr(value, context),
        },
        M::LetTuple {
            local,
            name: _,
            value,
        } => E::LetTuple {
            local: execution::TupleLocalId(local.0),
            value: tuple_expr(value, context),
        },
        M::LetList { name: _, value } => E::LetList {
            value: super::expression::list_local_expr(value, context),
        },
        M::LetIntFunction {
            local,
            name: _,
            value,
        } => E::LetIntFunction {
            local: execution::IntFunctionLocalId(local.0),
            value: int_function_expr(value, context),
        },
        M::LetFloatFunction {
            local,
            name: _,
            value,
        } => E::LetFloatFunction {
            local: execution::FloatFunctionLocalId(local.0),
            value: super::expression::float_function_expr(value, context),
        },
        M::LetStringFunction {
            local,
            name: _,
            value,
        } => E::LetStringFunction {
            local: execution::StringFunctionLocalId(local.0),
            value: string_function_expr(value, context),
        },
        M::LetBitArrayFunction {
            local,
            name: _,
            value,
        } => E::LetBitArrayFunction {
            local: execution::BitArrayFunctionLocalId(local.0),
            value: bit_array_function_expr(value, context),
        },
        M::LetBoolFunction {
            local,
            name: _,
            value,
        } => E::LetBoolFunction {
            local: execution::BoolFunctionLocalId(local.0),
            value: super::expression::bool_function_expr(value, context),
        },
        M::LetNilFunction {
            local,
            name: _,
            value,
        } => E::LetNilFunction {
            local: execution::NilFunctionLocalId(local.0),
            value: nil_function_expr(value, context),
        },
        M::LetTupleFunction {
            local,
            name: _,
            value,
        } => E::LetTupleFunction {
            local: execution::TupleFunctionLocalId(local.0),
            value: tuple_function_expr(value, context),
        },
        M::LetListFunction {
            local,
            name: _,
            value,
        } => E::LetListFunction {
            local: list_function_local(local, context),
            value: list_function_expr(value, context),
        },
        M::LetFunctionFunction {
            local,
            name: _,
            value,
        } => E::LetFunctionFunction {
            local: execution::FunctionFunctionLocalId(local.0),
            value: function_function_expr(value, context),
        },
        M::AssertList {
            local,
            pattern,
            message,
            site,
            pattern_span,
        } => E::AssertList {
            local: list_local(local, context),
            pattern: assert_pattern(pattern, context),
            message: message.map(|message| string_expr(message, context)),
            site,
            pattern_span,
        },
        M::AssertBool {
            condition,
            message,
            site,
        } => E::AssertBool {
            condition: bool_expr(condition, context),
            message: message.map(|message| string_expr(message, context)),
            site,
        },
        M::Evaluate(value) => E::Evaluate(expr(value, context)),
    })
}

fn assert_pattern(
    pattern: module::AssertPattern,
    context: &mut super::LoweringContext,
) -> execution::AssertPattern {
    match pattern {
        module::AssertPattern::Bind(binding) => {
            execution::AssertPattern::Bind(assert_binding(binding, context))
        }
        module::AssertPattern::Discard => execution::AssertPattern::Discard,
        module::AssertPattern::Tuple(elements) => execution::AssertPattern::Tuple(
            elements
                .into_iter()
                .map(|element| assert_pattern(element, context))
                .collect(),
        ),
        module::AssertPattern::List(pattern) => {
            let (_element_type, elements, tail) = pattern.into_parts();
            execution::AssertPattern::List(execution::ListAssertPattern::new(
                elements
                    .into_iter()
                    .map(|element| assert_pattern(element, context))
                    .collect(),
                tail.map(|tail| assert_tail(tail, context)),
            ))
        }
        module::AssertPattern::Alias { pattern, binding } => execution::AssertPattern::Alias {
            pattern: Box::new(assert_pattern(*pattern, context)),
            binding: assert_binding(binding, context),
        },
    }
}

fn assert_binding(
    binding: module::AssertBinding,
    context: &mut super::LoweringContext,
) -> execution::AssertBinding {
    let (local, _) = binding.into_parts();
    execution::AssertBinding::new(param_local(local, context))
}

fn assert_tail(
    tail: module::ListAssertTail,
    context: &mut super::LoweringContext,
) -> execution::ListAssertTail {
    match tail {
        module::ListAssertTail::Ignore => execution::ListAssertTail::Ignore,
        module::ListAssertTail::Bind(binding) => {
            let (local, _) = binding.into_parts();
            execution::ListAssertTail::bind(list_local(local, context))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{
        AssertBinding, AssertPattern, ExecutionPlan, IntListLocalId, ListAssertPattern,
        ListAssertTail, ListFunctionId, ListListFunctionId, ListListTypeId, ListLocal,
        ListLocalExpr, ParamLocal, RuntimeFunctionId, Step, StepKind,
    };

    #[test]
    fn lowering_preserves_parent_and_child_list_types_through_assert_bindings() {
        let source = r#"
pub fn main() {
  let values: List(List(Int)) = []
  let assert [first, ..rest] = values
  rest
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let main = expect_list_list_main(&plan);
        let function = plan.list_list_function(main);
        let value = expect_nested_list_binding(&function.steps()[0]);
        let parent_type = value.item().type_id();
        let assert_value = expect_nested_list_binding(&function.steps()[1]);
        let (local, pattern) = expect_list_assert(&function.steps()[2]);
        let subject_type = expect_nested_list_local(local);
        let pattern = expect_list_pattern(pattern);
        let first = expect_single_binding(pattern.elements());
        let first_type = expect_int_list_binding(first);
        let rest = expect_tail_binding(pattern.tail());
        let rest_type = expect_nested_list_local(rest);

        assert_eq!(subject_type, parent_type);
        assert_eq!(rest_type, parent_type);
        assert_eq!(assert_value.item().type_id(), parent_type);
        assert_eq!(parent_type.item_type(), first_type.list_type());
    }

    #[test]
    #[should_panic(expected = "expected a List(List) main function")]
    fn nested_list_main_fixture_guard_rejects_int_main() {
        let plan = execution_plan("pub fn main() { 1 }");
        let _ = expect_list_list_main(&plan);
    }

    #[test]
    #[should_panic(expected = "expected a nested-list binding step")]
    fn nested_list_binding_fixture_guard_rejects_int_binding() {
        let plan = execution_plan("pub fn main() -> List(Int) { let value = 1 [] }");
        let main = plan.int_list_function_id(0);
        let _ = expect_nested_list_binding(&plan.int_list_function(main).steps()[0]);
    }

    #[test]
    #[should_panic(expected = "expected a list-assert step")]
    fn list_assert_fixture_guard_rejects_binding() {
        let plan = assert_execution_plan();
        let main = expect_list_list_main(&plan);
        let _ = expect_list_assert(&plan.list_list_function(main).steps()[0]);
    }

    #[test]
    #[should_panic(expected = "expected a nested-list local")]
    fn nested_list_local_fixture_guard_rejects_int_list_local() {
        let plan = assert_execution_plan();
        let main = expect_list_list_main(&plan);
        let (_, pattern) = expect_list_assert(&plan.list_list_function(main).steps()[2]);
        let pattern = expect_list_pattern(pattern);
        let first = expect_single_binding(pattern.elements());
        let first_type = expect_int_list_binding(first);
        let local = ListLocal::Int {
            local: IntListLocalId(0),
            type_id: first_type,
        };
        let _ = expect_nested_list_local(&local);
    }

    #[test]
    #[should_panic(expected = "expected a list assert pattern")]
    fn list_pattern_fixture_guard_rejects_binding_pattern() {
        let plan = assert_execution_plan();
        let main = expect_list_list_main(&plan);
        let (_, pattern) = expect_list_assert(&plan.list_list_function(main).steps()[2]);
        let pattern = expect_list_pattern(pattern);
        let _ = expect_list_pattern(&pattern.elements()[0]);
    }

    #[test]
    #[should_panic(expected = "expected one assert binding")]
    fn single_binding_fixture_guard_rejects_empty_elements() {
        let _ = expect_single_binding(&[]);
    }

    #[test]
    #[should_panic(expected = "expected a List(Int) binding")]
    fn int_list_binding_fixture_guard_rejects_nested_list_binding() {
        let plan = assert_execution_plan();
        let main = expect_list_list_main(&plan);
        let (_, pattern) = expect_list_assert(&plan.list_list_function(main).steps()[2]);
        let pattern = expect_list_pattern(pattern);
        let rest = expect_tail_binding(pattern.tail());
        let binding = AssertBinding::new(ParamLocal::List(rest.clone()));
        let _ = expect_int_list_binding(&binding);
    }

    #[test]
    #[should_panic(expected = "expected a bound assert tail")]
    fn tail_binding_fixture_guard_rejects_missing_tail() {
        let _ = expect_tail_binding(None);
    }

    fn assert_execution_plan() -> ExecutionPlan {
        execution_plan(
            r#"
pub fn main() {
  let values: List(List(Int)) = []
  let assert [first, ..rest] = values
  rest
}
"#,
        )
    }

    fn execution_plan(source: &str) -> ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }

    fn expect_list_list_main(plan: &ExecutionPlan) -> ListListFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::List(ListFunctionId::List(main)) => main,
            _ => panic!("expected a List(List) main function"),
        }
    }

    fn expect_nested_list_binding(step: &Step) -> &crate::plan::execution::ListListExpr {
        match step.kind() {
            StepKind::LetList {
                value: ListLocalExpr::List { value, .. },
            } => value,
            _ => panic!("expected a nested-list binding step"),
        }
    }

    fn expect_list_assert(step: &Step) -> (&ListLocal, &AssertPattern) {
        match step.kind() {
            StepKind::AssertList { local, pattern, .. } => (local, pattern),
            _ => panic!("expected a list-assert step"),
        }
    }

    fn expect_nested_list_local(local: &ListLocal) -> ListListTypeId {
        match local {
            ListLocal::List { type_id, .. } => *type_id,
            _ => panic!("expected a nested-list local"),
        }
    }

    fn expect_list_pattern(pattern: &AssertPattern) -> &ListAssertPattern {
        match pattern {
            AssertPattern::List(pattern) => pattern,
            _ => panic!("expected a list assert pattern"),
        }
    }

    fn expect_single_binding(elements: &[AssertPattern]) -> &AssertBinding {
        match elements {
            [AssertPattern::Bind(binding)] => binding,
            _ => panic!("expected one assert binding"),
        }
    }

    fn expect_int_list_binding(binding: &AssertBinding) -> crate::plan::execution::IntListTypeId {
        match binding.local() {
            ParamLocal::List(ListLocal::Int { type_id, .. }) => *type_id,
            _ => panic!("expected a List(Int) binding"),
        }
    }

    fn expect_tail_binding(tail: Option<&ListAssertTail>) -> &ListLocal {
        match tail {
            Some(ListAssertTail::Bind(binding)) => binding.local(),
            _ => panic!("expected a bound assert tail"),
        }
    }
}
