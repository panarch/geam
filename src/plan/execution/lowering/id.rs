use crate::plan::module;

use super::LoweringContext;

pub(super) fn custom_local(
    local: &module::CustomLocal,
    context: &mut LoweringContext,
) -> super::super::CustomLocal {
    super::super::CustomLocal::new(
        super::super::CustomLocalId(
            context.mapped_local(super::frame::LocalKind::Custom, local.id().0),
        ),
        context.custom_value_shape(local.shape().clone()),
    )
}

pub(super) fn custom_function_local(
    local: &module::CustomFunctionLocal,
    context: &mut LoweringContext,
) -> super::super::CustomFunctionLocal {
    super::super::CustomFunctionLocal::new(
        super::super::CustomFunctionLocalId(
            context.mapped_local(super::frame::LocalKind::CustomFunction, local.id().0),
        ),
        context.custom_function_type(local.type_().clone()),
    )
}

pub(super) fn function_function_local(
    local: &module::FunctionFunctionLocal,
    context: &mut LoweringContext,
) -> super::super::FunctionFunctionLocal {
    super::super::FunctionFunctionLocal::new(
        super::super::FunctionFunctionLocalId(
            context.mapped_local(super::frame::LocalKind::FunctionFunction, local.id().0),
        ),
        context.function_function_type(local.type_().clone()),
    )
}

pub(super) fn list_local(
    local: &module::ListLocal,
    context: &mut LoweringContext,
) -> super::super::ListLocal {
    let index = context.local_index(super::frame::list_local_key(local));
    list_local_at(index, local, context)
}

pub(super) fn list_local_at(
    index: usize,
    local: &module::ListLocal,
    context: &mut LoweringContext,
) -> super::super::ListLocal {
    use super::super as execution;

    match local {
        module::ListLocal::Generic {
            local: _,
            parameter,
        } => {
            let item = context.concrete_parameter(*parameter);
            super::frame::list_local_at(&item, index, context)
        }
        module::ListLocal::Int(_local) => execution::ListLocal::Int {
            local: execution::IntListLocalId(index),
            type_id: context.int_list_type(),
        },
        module::ListLocal::String(_local) => execution::ListLocal::String {
            local: execution::StringListLocalId(index),
            type_id: context.string_list_type(),
        },
        module::ListLocal::BitArray(_local) => execution::ListLocal::BitArray {
            local: execution::BitArrayListLocalId(index),
            type_id: context.bit_array_list_type(),
        },
        module::ListLocal::UtfCodepoint(_local) => execution::ListLocal::UtfCodepoint {
            local: execution::UtfCodepointListLocalId(index),
            type_id: context.utf_codepoint_list_type(),
        },
        module::ListLocal::Custom {
            local: _,
            item_type,
        } => execution::ListLocal::Custom {
            local: execution::CustomListLocalId(index),
            type_id: context.custom_list_type(item_type.clone()),
        },
        module::ListLocal::Float(_local) => execution::ListLocal::Float {
            local: execution::FloatListLocalId(index),
            type_id: context.float_list_type(),
        },
        module::ListLocal::Bool(_local) => execution::ListLocal::Bool {
            local: execution::BoolListLocalId(index),
            type_id: context.bool_list_type(),
        },
        module::ListLocal::Nil(_local) => execution::ListLocal::Nil {
            local: execution::NilListLocalId(index),
            type_id: context.nil_list_type(),
        },
        module::ListLocal::Tuple {
            local: _,
            item_type,
        } => execution::ListLocal::Tuple {
            local: execution::TupleListLocalId(index),
            type_id: context.tuple_list_type(item_type.clone()),
        },
        module::ListLocal::List {
            local: _,
            item_type,
        } => match context.list_list_type(item_type.as_ref().clone()) {
            super::value_type::NestedListTypeId::Parameter(type_id) => {
                execution::ListLocal::ParameterList {
                    local: execution::ParameterListListLocalId(index),
                    type_id,
                }
            }
            super::value_type::NestedListTypeId::Stored(type_id) => execution::ListLocal::List {
                local: execution::ListListLocalId(index),
                type_id,
            },
        },
        module::ListLocal::Function {
            local: _,
            item_type,
        } => execution::ListLocal::Function {
            local: execution::FunctionListLocalId(index),
            type_id: context.function_list_type(item_type.clone()),
        },
    }
}

pub(super) fn list_function_local(
    local: &module::ListFunctionLocal,
    context: &mut LoweringContext,
) -> super::super::ListFunctionLocal {
    let index = context.local_index(super::frame::list_function_local_key(local));
    list_function_local_at(index, local, context)
}

pub(super) fn list_function_local_at(
    index: usize,
    local: &module::ListFunctionLocal,
    context: &mut LoweringContext,
) -> super::super::ListFunctionLocal {
    use super::super as execution;

    match local {
        module::ListFunctionLocal::Generic {
            local: _,
            type_,
            parameter,
        } => {
            let item = context.concrete_parameter(*parameter);
            let type_ = context.function_type(type_.clone());
            super::frame::list_function_local_at(&item, type_, index, context)
        }
        module::ListFunctionLocal::Int { local: _, type_ } => execution::ListFunctionLocal::Int {
            local: execution::IntListFunctionLocalId(index),
            type_: context.function_type(type_.clone()),
            list_type: context.int_list_type(),
        },
        module::ListFunctionLocal::String { local: _, type_ } => {
            execution::ListFunctionLocal::String {
                local: execution::StringListFunctionLocalId(index),
                type_: context.function_type(type_.clone()),
                list_type: context.string_list_type(),
            }
        }
        module::ListFunctionLocal::BitArray { local: _, type_ } => {
            execution::ListFunctionLocal::BitArray {
                local: execution::BitArrayListFunctionLocalId(index),
                type_: context.function_type(type_.clone()),
                list_type: context.bit_array_list_type(),
            }
        }
        module::ListFunctionLocal::UtfCodepoint { local: _, type_ } => {
            execution::ListFunctionLocal::UtfCodepoint {
                local: execution::UtfCodepointListFunctionLocalId(index),
                type_: context.function_type(type_.clone()),
                list_type: context.utf_codepoint_list_type(),
            }
        }
        module::ListFunctionLocal::Custom {
            local: _,
            type_,
            item_type,
        } => execution::ListFunctionLocal::Custom {
            local: execution::CustomListFunctionLocalId(index),
            type_: context.function_type(type_.clone()),
            list_type: context.custom_list_type(item_type.clone()),
        },
        module::ListFunctionLocal::Float { local: _, type_ } => {
            execution::ListFunctionLocal::Float {
                local: execution::FloatListFunctionLocalId(index),
                type_: context.function_type(type_.clone()),
                list_type: context.float_list_type(),
            }
        }
        module::ListFunctionLocal::Bool { local: _, type_ } => execution::ListFunctionLocal::Bool {
            local: execution::BoolListFunctionLocalId(index),
            type_: context.function_type(type_.clone()),
            list_type: context.bool_list_type(),
        },
        module::ListFunctionLocal::Nil { local: _, type_ } => execution::ListFunctionLocal::Nil {
            local: execution::NilListFunctionLocalId(index),
            type_: context.function_type(type_.clone()),
            list_type: context.nil_list_type(),
        },
        module::ListFunctionLocal::Tuple {
            local: _,
            type_,
            item_type,
        } => execution::ListFunctionLocal::Tuple {
            local: execution::TupleListFunctionLocalId(index),
            type_: context.function_type(type_.clone()),
            list_type: context.tuple_list_type(item_type.clone()),
        },
        module::ListFunctionLocal::List {
            local: _,
            type_,
            item_type,
        } => {
            let type_ = context.function_type(type_.clone());
            match context.list_list_type(item_type.as_ref().clone()) {
                super::value_type::NestedListTypeId::Parameter(list_type) => {
                    execution::ListFunctionLocal::ParameterList {
                        local: execution::ParameterListListFunctionLocalId(index),
                        type_,
                        list_type,
                    }
                }
                super::value_type::NestedListTypeId::Stored(list_type) => {
                    execution::ListFunctionLocal::List {
                        local: execution::ListListFunctionLocalId(index),
                        type_,
                        list_type,
                    }
                }
            }
        }
        module::ListFunctionLocal::Function {
            local: _,
            type_,
            item_type,
        } => execution::ListFunctionLocal::Function {
            local: execution::FunctionListFunctionLocalId(index),
            type_: context.function_type(type_.clone()),
            list_type: context.function_list_type(item_type.as_ref().clone()),
        },
    }
}

pub(super) fn list_function_local_at_target(
    index: usize,
    local: &module::ListFunctionLocal,
    target: &super::StoredTargetLocal,
    context: &mut LoweringContext,
) -> super::super::ListFunctionLocal {
    let function = target.function_shape(&crate::plan::FunctionShape::from_function_type(
        local.type_().clone(),
    ));
    let item = super::specialization::SpecializedValueShape::instantiate(
        &crate::plan::ValueShape::from_value_type(local.item_type()),
        target.substitution(),
    );
    let type_ = context.lower_concrete_function_type(&function);
    super::frame::list_function_local_at(&item, type_, index, context)
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{
        ExecutionPlan, ListFunctionExpr, ListFunctionExprKind, ListFunctionId, ListFunctionLocal,
        ListListFunctionId, ListListTypeId, RuntimeFunctionId, Step, StepKind,
    };

    #[test]
    fn lowering_shares_nested_list_type_between_reference_local_and_runtime_id() {
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
        let (local, value) =
            expect_list_function_binding(&plan.list_list_function(main).steps()[0]);
        let list_type = expect_nested_list_function_local(local);
        let reference = expect_list_function_reference(value);
        let target = expect_nested_list_function_id(reference.function());

        assert_eq!(list_type, target.type_id());
        assert_eq!(main.type_id(), target.type_id());
        assert_eq!(target.index(), 1);
    }

    #[test]
    #[should_panic(expected = "expected a List(List) main function")]
    fn nested_list_main_fixture_guard_rejects_int_main() {
        let plan = execution_plan("pub fn main() { 1 }");
        let _ = expect_list_list_main(&plan);
    }

    #[test]
    #[should_panic(expected = "expected a list-function binding step")]
    fn list_function_binding_fixture_guard_rejects_int_binding() {
        let plan = execution_plan("pub fn main() -> List(List(Int)) { let value = 1 [] }");
        let main = expect_list_list_main(&plan);
        let _ = expect_list_function_binding(&plan.list_list_function(main).steps()[0]);
    }

    #[test]
    #[should_panic(expected = "expected a nested-list function local")]
    fn nested_list_function_local_fixture_guard_rejects_int_list_local() {
        let plan = execution_plan(
            "fn identity(values: List(Int)) { values } pub fn main() { let function = identity function([]) }",
        );
        let main = plan.int_list_function_id(0);
        let (local, _) = expect_list_function_binding(&plan.int_list_function(main).steps()[0]);
        let _ = expect_nested_list_function_local(local);
    }

    #[test]
    #[should_panic(expected = "expected a list-function reference")]
    fn list_function_reference_fixture_guard_rejects_closure() {
        let plan = execution_plan(
            "pub fn main() { let captured = 1 let function = fn(values: List(List(Int))) { let _ = captured values } function([]) }",
        );
        let main = expect_list_list_main(&plan);
        let (_, value) = expect_list_function_binding(&plan.list_list_function(main).steps()[1]);
        let _ = expect_list_function_reference(value);
    }

    #[test]
    #[should_panic(expected = "expected a nested-list function id")]
    fn nested_list_function_id_fixture_guard_rejects_int_list_id() {
        let plan = execution_plan(
            "fn identity(values: List(Int)) { values } pub fn main() { let function = identity function([]) }",
        );
        let main = plan.int_list_function_id(0);
        let (_, value) = expect_list_function_binding(&plan.int_list_function(main).steps()[0]);
        let reference = expect_list_function_reference(value);
        let _ = expect_nested_list_function_id(reference.function());
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

    fn expect_list_function_binding(step: &Step) -> (&ListFunctionLocal, &ListFunctionExpr) {
        match step.kind() {
            StepKind::LetListFunction { local, value } => (local, value.expression()),
            _ => panic!("expected a list-function binding step"),
        }
    }

    fn expect_nested_list_function_local(local: &ListFunctionLocal) -> ListListTypeId {
        match local {
            ListFunctionLocal::List { list_type, .. } => *list_type,
            _ => panic!("expected a nested-list function local"),
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
}
