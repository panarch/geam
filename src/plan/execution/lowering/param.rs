use super::id::{list_function_local_at, list_local_at};
use crate::plan::module;

pub(super) fn param_local(
    local: &module::ParamLocal,
    context: &mut super::LoweringContext,
) -> super::super::ParamLocal {
    let index = context.local_index(super::frame::param_local_key(local));
    param_local_at(index, local, context)
}

fn param_local_at(
    index: usize,
    local: &module::ParamLocal,
    context: &mut super::LoweringContext,
) -> super::super::ParamLocal {
    use super::super as execution;

    match local {
        module::ParamLocal::Generic(local) => {
            let shape = context.concrete_parameter(local.parameter());
            super::frame::value_local_at(&shape, index, context)
        }
        module::ParamLocal::GenericFunction(local) => {
            let shape = context.concrete_function_shape(&local.type_().shape());
            super::frame::function_local_as_param(super::frame::function_local_at(
                &shape, index, context,
            ))
        }
        module::ParamLocal::Int(_local) => execution::ParamLocal::Int(execution::IntLocalId(index)),
        module::ParamLocal::Float(_local) => {
            execution::ParamLocal::Float(execution::FloatLocalId(index))
        }
        module::ParamLocal::String(_local) => {
            execution::ParamLocal::String(execution::StringLocalId(index))
        }
        module::ParamLocal::BitArray(_local) => {
            execution::ParamLocal::BitArray(execution::BitArrayLocalId(index))
        }
        module::ParamLocal::UtfCodepoint(_local) => {
            execution::ParamLocal::UtfCodepoint(execution::UtfCodepointLocalId(index))
        }
        module::ParamLocal::Custom(local) => {
            execution::ParamLocal::Custom(execution::CustomLocal::new(
                execution::CustomLocalId(index),
                context.custom_value_shape(local.shape().clone()),
            ))
        }
        module::ParamLocal::Bool(_local) => {
            execution::ParamLocal::Bool(execution::BoolLocalId(index))
        }
        module::ParamLocal::Nil(_local) => execution::ParamLocal::Nil(execution::NilLocalId(index)),
        module::ParamLocal::Tuple { local: _, type_ } => execution::ParamLocal::Tuple {
            local: execution::TupleLocalId(index),
            type_: type_
                .iter()
                .cloned()
                .map(|type_| context.value_type(type_))
                .collect(),
        },
        module::ParamLocal::List(local) => {
            execution::ParamLocal::List(list_local_at(index, local, context))
        }
        module::ParamLocal::IntFunction { local: _, type_ } => execution::ParamLocal::IntFunction {
            local: execution::IntFunctionLocalId(index),
            type_: context.function_type(type_.clone()),
        },
        module::ParamLocal::FloatFunction { local: _, type_ } => {
            execution::ParamLocal::FloatFunction {
                local: execution::FloatFunctionLocalId(index),
                type_: context.function_type(type_.clone()),
            }
        }
        module::ParamLocal::StringFunction { local: _, type_ } => {
            execution::ParamLocal::StringFunction {
                local: execution::StringFunctionLocalId(index),
                type_: context.function_type(type_.clone()),
            }
        }
        module::ParamLocal::BitArrayFunction { local: _, type_ } => {
            execution::ParamLocal::BitArrayFunction {
                local: execution::BitArrayFunctionLocalId(index),
                type_: context.function_type(type_.clone()),
            }
        }
        module::ParamLocal::UtfCodepointFunction { local: _, type_ } => {
            execution::ParamLocal::UtfCodepointFunction {
                local: execution::UtfCodepointFunctionLocalId(index),
                type_: context.function_type(type_.clone()),
            }
        }
        module::ParamLocal::CustomFunction(local) => {
            execution::ParamLocal::CustomFunction(execution::CustomFunctionLocal::new(
                execution::CustomFunctionLocalId(index),
                context.custom_function_type(local.type_().clone()),
            ))
        }
        module::ParamLocal::BoolFunction { local: _, type_ } => {
            execution::ParamLocal::BoolFunction {
                local: execution::BoolFunctionLocalId(index),
                type_: context.function_type(type_.clone()),
            }
        }
        module::ParamLocal::NilFunction { local: _, type_ } => execution::ParamLocal::NilFunction {
            local: execution::NilFunctionLocalId(index),
            type_: context.function_type(type_.clone()),
        },
        module::ParamLocal::TupleFunction { local: _, type_ } => {
            execution::ParamLocal::TupleFunction {
                local: execution::TupleFunctionLocalId(index),
                type_: context.function_type(type_.clone()),
            }
        }
        module::ParamLocal::ListFunction(local) => {
            execution::ParamLocal::ListFunction(list_function_local_at(index, local, context))
        }
        module::ParamLocal::FunctionFunction(local) => {
            execution::ParamLocal::FunctionFunction(execution::FunctionFunctionLocal::new(
                execution::FunctionFunctionLocalId(index),
                context.function_function_type(local.type_().clone()),
            ))
        }
    }
}

pub(super) fn param_slot(
    slot: &module::ParamSlot,
    context: &mut super::LoweringContext,
) -> super::super::ParamSlot {
    let shape = context.value_shape(slot.shape().clone());
    super::super::ParamSlot::new(param_local(slot.local(), context), shape)
}

pub(super) fn target_param_slot(
    function: &module::FunctionInstantiation,
    slot: &module::ParamSlot,
    context: &mut super::LoweringContext,
) -> super::super::ParamSlot {
    let target = context.target_local(function, super::frame::param_local_key(slot.local()));
    let shape = context.types.value_shape(target.shape());
    let local = super::frame::value_local_at(target.shape(), target.index(), context);
    super::super::ParamSlot::new(local, shape)
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{
        CustomFunctionExprKind, ExecutionPlan, FunctionType, IntListFunctionLocalId,
        ListFunctionExpr, ListFunctionExprKind, ListFunctionId, ListFunctionLocal,
        ListListFunctionId, ListListTypeId, ListLocal, ParamLocal, RuntimeFunctionId, Step,
        StepKind, ValueType,
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
            plan.value_type(&plan.shape_value_type(reference.params()[0].shape())),
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
        let (main_id, _) = expect_custom_main(&plan);
        let main = plan.custom_function(main_id);
        let reference = main
            .steps()
            .iter()
            .find_map(|step| match step.kind() {
                StepKind::LetCustomFunction { value, .. } => match value.expression().kind() {
                    CustomFunctionExprKind::Reference(reference) => Some(reference),
                    _ => None,
                },
                _ => None,
            })
            .expect("main should bind the custom-returning function reference");
        assert_eq!(reference.function(), &plan.custom_function_id(1));
        let target = plan.custom_function(*reference.function());
        assert_eq!(
            reference
                .params()
                .iter()
                .map(crate::plan::execution::ParamSlot::local)
                .collect::<Vec<_>>(),
            vec![
                &ParamLocal::CustomFunction(target.frame_layout().custom_functions()[0].clone()),
                &ParamLocal::Int(crate::plan::execution::IntLocalId(0)),
            ],
        );
    }

    #[test]
    fn generic_list_and_list_function_params_specialize_to_one_frame_type() {
        let plan = execution_plan(
            r#"
fn make() { [1] }

fn apply(values: List(value), make: fn() -> List(value)) {
  let _ = values
  make()
}

pub fn main() {
  apply([1], make)
}
"#,
        );
        let main_id = plan.int_list_function_id(0);
        assert_eq!(
            plan.main_runtime(),
            RuntimeFunctionId::List(ListFunctionId::Int(main_id)),
        );
        let apply_id = plan.int_list_function_id(2);
        let apply = plan.int_list_function(apply_id);

        assert_eq!(apply.frame_layout().int_lists(), &[apply_id.type_id()]);
        assert_eq!(
            apply.frame_layout().list_functions(),
            &[ListFunctionLocal::Int {
                local: IntListFunctionLocalId(0),
                type_: FunctionType::new(
                    Vec::new(),
                    ValueType::List(apply_id.type_id().list_type()),
                ),
                list_type: apply_id.type_id(),
            }],
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

    #[test]
    #[should_panic(expected = "expected one nested-list parameter")]
    fn nested_list_param_fixture_guard_rejects_multiple_params() {
        let plan = execution_plan(
            "fn identity(values: List(List(Int)), other: Int) { values } pub fn main() { let function = identity function([], 1) }",
        );
        let main = plan.list_list_function_id(0);
        let value = expect_list_function_binding(&plan.list_list_function(main).steps()[0]);
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
            RuntimeFunctionId::Custom(id) => (id, id.return_type()),
            _ => panic!("expected a custom-returning main function"),
        }
    }

    fn expect_list_function_binding(step: &Step) -> &ListFunctionExpr {
        match step.kind() {
            StepKind::LetListFunction { value, .. } => value.expression(),
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

    fn expect_single_nested_list_param(
        params: &[crate::plan::execution::ParamSlot],
    ) -> ListListTypeId {
        match params {
            [param] => match param.local() {
                ParamLocal::List(ListLocal::List { type_id, .. }) => *type_id,
                _ => panic!("expected one nested-list parameter"),
            },
            _ => panic!("expected one nested-list parameter"),
        }
    }
}
