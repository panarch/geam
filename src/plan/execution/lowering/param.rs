use super::id::{custom_function_local, function_function_local, list_function_local, list_local};
use crate::plan::module;

pub(super) fn param_local(
    local: module::ParamLocal,
    context: &mut super::LoweringContext,
) -> super::super::ParamLocal {
    use super::super as execution;

    match local {
        module::ParamLocal::Int(local) => {
            execution::ParamLocal::Int(execution::IntLocalId(local.0))
        }
        module::ParamLocal::Float(local) => {
            execution::ParamLocal::Float(execution::FloatLocalId(local.0))
        }
        module::ParamLocal::String(local) => {
            execution::ParamLocal::String(execution::StringLocalId(local.0))
        }
        module::ParamLocal::BitArray(local) => {
            execution::ParamLocal::BitArray(execution::BitArrayLocalId(local.0))
        }
        module::ParamLocal::UtfCodepoint(local) => {
            execution::ParamLocal::UtfCodepoint(execution::UtfCodepointLocalId(local.0))
        }
        module::ParamLocal::Custom { local, type_ } => execution::ParamLocal::Custom {
            local: execution::CustomLocalId(local.0),
            type_id: context.custom_type(type_),
        },
        module::ParamLocal::Bool(local) => {
            execution::ParamLocal::Bool(execution::BoolLocalId(local.0))
        }
        module::ParamLocal::Nil(local) => {
            execution::ParamLocal::Nil(execution::NilLocalId(local.0))
        }
        module::ParamLocal::Tuple { local, type_ } => execution::ParamLocal::Tuple {
            local: execution::TupleLocalId(local.0),
            type_: type_
                .into_iter()
                .map(|type_| context.value_type(type_))
                .collect(),
        },
        module::ParamLocal::List(local) => execution::ParamLocal::List(list_local(local, context)),
        module::ParamLocal::IntFunction { local, type_ } => execution::ParamLocal::IntFunction {
            local: execution::IntFunctionLocalId(local.0),
            type_: context.function_type(type_),
        },
        module::ParamLocal::FloatFunction { local, type_ } => {
            execution::ParamLocal::FloatFunction {
                local: execution::FloatFunctionLocalId(local.0),
                type_: context.function_type(type_),
            }
        }
        module::ParamLocal::StringFunction { local, type_ } => {
            execution::ParamLocal::StringFunction {
                local: execution::StringFunctionLocalId(local.0),
                type_: context.function_type(type_),
            }
        }
        module::ParamLocal::BitArrayFunction { local, type_ } => {
            execution::ParamLocal::BitArrayFunction {
                local: execution::BitArrayFunctionLocalId(local.0),
                type_: context.function_type(type_),
            }
        }
        module::ParamLocal::UtfCodepointFunction { local, type_ } => {
            execution::ParamLocal::UtfCodepointFunction {
                local: execution::UtfCodepointFunctionLocalId(local.0),
                type_: context.function_type(type_),
            }
        }
        module::ParamLocal::CustomFunction(local) => {
            execution::ParamLocal::CustomFunction(custom_function_local(local, context))
        }
        module::ParamLocal::BoolFunction { local, type_ } => execution::ParamLocal::BoolFunction {
            local: execution::BoolFunctionLocalId(local.0),
            type_: context.function_type(type_),
        },
        module::ParamLocal::NilFunction { local, type_ } => execution::ParamLocal::NilFunction {
            local: execution::NilFunctionLocalId(local.0),
            type_: context.function_type(type_),
        },
        module::ParamLocal::TupleFunction { local, type_ } => {
            execution::ParamLocal::TupleFunction {
                local: execution::TupleFunctionLocalId(local.0),
                type_: context.function_type(type_),
            }
        }
        module::ParamLocal::ListFunction(local) => {
            execution::ParamLocal::ListFunction(list_function_local(local, context))
        }
        module::ParamLocal::FunctionFunction(local) => {
            execution::ParamLocal::FunctionFunction(function_function_local(local, context))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{
        CustomFunctionExprKind, CustomFunctionId, ExecutionPlan, ListFunctionExpr,
        ListFunctionExprKind, ListFunctionId, ListListFunctionId, ListListTypeId, ListLocal,
        ParamLocal, RuntimeFunctionId, Step, StepKind, ValueType,
    };

    #[test]
    fn lowering_preserves_nested_list_parameter_identity_in_function_references() {
        let source = r#"
fn identity(values: List(List(Int))) { values }

pub fn main() {
  let function = identity
  function([])
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let main = expect_list_list_main(&plan);
        let value = expect_list_function_binding(&plan.list_list_function(main).steps()[0]);
        let reference = expect_list_function_reference(value);
        let target = expect_nested_list_function_id(reference.function());
        let type_id = expect_single_nested_list_param(reference.params());

        assert_eq!(type_id, target.type_id());
        assert_eq!(
            plan.value_type(&reference.params()[0].value_type()),
            crate::plan::ValueType::List(Box::new(crate::plan::ValueType::List(Box::new(
                crate::plan::ValueType::Int,
            )))),
        );
    }

    #[test]
    fn lowering_preserves_custom_function_parameter_identity_in_function_references() {
        let plan = execution_plan(
            r#"
pub type Boxed { Boxed(Int) }

fn apply(make: fn(Int) -> Boxed, value: Int) { make(value) }

pub fn main() {
  let captured = 1
  let closure = fn(make: fn(Int) -> Boxed, value: Int) {
    let _ = captured
    make(value)
  }
  closure(Boxed, 0)
  let function = apply
  function(Boxed, 1)
}
"#,
        );
        let (main_id, return_type) = expect_custom_main(&plan);
        let main = plan.custom_function(main_id);
        let reference = main
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::LetCustomFunction { value, .. } => match value.kind() {
                    CustomFunctionExprKind::Reference(reference) => Some(reference),
                    _ => None,
                },
                _ => None,
            })
            .expect("main should bind the custom-returning function reference");

        assert_eq!(reference.function(), &CustomFunctionId(1));
        assert_eq!(
            reference.params(),
            &[
                ParamLocal::CustomFunction(crate::plan::execution::CustomFunctionLocal::new(
                    crate::plan::execution::CustomFunctionLocalId(0),
                    crate::plan::execution::CustomFunctionType::new(
                        vec![ValueType::Int],
                        return_type,
                    ),
                )),
                ParamLocal::Int(crate::plan::execution::IntLocalId(0)),
            ],
        );
    }

    #[test]
    #[should_panic(expected = "expected a List(List) main function")]
    fn nested_list_main_fixture_guard_rejects_int_main() {
        let plan = execution_plan("pub fn main() { 1 }");
        let _ = expect_list_list_main(&plan);
    }

    #[test]
    #[should_panic(expected = "expected a custom-returning main function")]
    fn custom_main_fixture_guard_rejects_int_main() {
        let plan = execution_plan("pub fn main() { 1 }");
        let _ = expect_custom_main(&plan);
    }

    #[test]
    #[should_panic(expected = "expected a list-function binding step")]
    fn list_function_binding_fixture_guard_rejects_int_binding() {
        let plan = execution_plan("pub fn main() -> List(List(Int)) { let value = 1 [] }");
        let main = expect_list_list_main(&plan);
        let _ = expect_list_function_binding(&plan.list_list_function(main).steps()[0]);
    }

    #[test]
    #[should_panic(expected = "expected a list-function reference")]
    fn list_function_reference_fixture_guard_rejects_closure() {
        let plan = execution_plan(
            "pub fn main() { let captured = 1 let function = fn(values: List(List(Int))) { let _ = captured values } function([]) }",
        );
        let main = expect_list_list_main(&plan);
        let value = expect_list_function_binding(&plan.list_list_function(main).steps()[1]);
        let _ = expect_list_function_reference(value);
    }

    #[test]
    #[should_panic(expected = "expected a nested-list function id")]
    fn nested_list_function_id_fixture_guard_rejects_int_list_id() {
        let plan = execution_plan(
            "fn identity(values: List(Int)) { values } pub fn main() { let function = identity function([]) }",
        );
        let main = plan.int_list_function_id(0);
        let value = expect_list_function_binding(&plan.int_list_function(main).steps()[0]);
        let reference = expect_list_function_reference(value);
        let _ = expect_nested_list_function_id(reference.function());
    }

    #[test]
    #[should_panic(expected = "expected one nested-list parameter")]
    fn nested_list_param_fixture_guard_rejects_int_list_param() {
        let plan = execution_plan(
            "fn identity(values: List(Int)) { values } pub fn main() { let function = identity function([]) }",
        );
        let main = plan.int_list_function_id(0);
        let value = expect_list_function_binding(&plan.int_list_function(main).steps()[0]);
        let reference = expect_list_function_reference(value);
        let _ = expect_single_nested_list_param(reference.params());
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

    fn expect_custom_main(
        plan: &ExecutionPlan,
    ) -> (
        crate::plan::execution::CustomFunctionId,
        crate::plan::execution::CustomTypeId,
    ) {
        match plan.main_runtime() {
            RuntimeFunctionId::Custom { id, return_type } => (id, return_type),
            _ => panic!("expected a custom-returning main function"),
        }
    }

    fn expect_list_function_binding(step: &Step) -> &ListFunctionExpr {
        match step.kind() {
            StepKind::LetListFunction { value, .. } => value,
            _ => panic!("expected a list-function binding step"),
        }
    }

    fn expect_list_function_reference(
        value: &ListFunctionExpr,
    ) -> &crate::plan::execution::FunctionReference<ListFunctionId> {
        match value.kind() {
            ListFunctionExprKind::Reference(reference) => reference,
            _ => panic!("expected a list-function reference"),
        }
    }

    fn expect_nested_list_function_id(function: &ListFunctionId) -> &ListListFunctionId {
        match function {
            ListFunctionId::List(function) => function,
            _ => panic!("expected a nested-list function id"),
        }
    }

    fn expect_single_nested_list_param(params: &[ParamLocal]) -> ListListTypeId {
        match params {
            [ParamLocal::List(ListLocal::List { type_id, .. })] => *type_id,
            _ => panic!("expected one nested-list parameter"),
        }
    }
}
