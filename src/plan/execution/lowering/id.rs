use crate::plan::module;

use super::LoweringContext;

pub(super) fn list_local(
    local: module::ListLocal,
    context: &mut LoweringContext,
) -> super::super::ListLocal {
    use super::super as execution;

    match local {
        module::ListLocal::Int(local) => execution::ListLocal::Int {
            local: execution::IntListLocalId(local.0),
            type_id: context.int_list_type(),
        },
        module::ListLocal::String(local) => execution::ListLocal::String {
            local: execution::StringListLocalId(local.0),
            type_id: context.string_list_type(),
        },
        module::ListLocal::BitArray(local) => execution::ListLocal::BitArray {
            local: execution::BitArrayListLocalId(local.0),
            type_id: context.bit_array_list_type(),
        },
        module::ListLocal::UtfCodepoint(local) => execution::ListLocal::UtfCodepoint {
            local: execution::UtfCodepointListLocalId(local.0),
            type_id: context.utf_codepoint_list_type(),
        },
        module::ListLocal::Custom { local, item_type } => execution::ListLocal::Custom {
            local: execution::CustomListLocalId(local.0),
            type_id: context.custom_list_type(item_type),
        },
        module::ListLocal::Float(local) => execution::ListLocal::Float {
            local: execution::FloatListLocalId(local.0),
            type_id: context.float_list_type(),
        },
        module::ListLocal::Bool(local) => execution::ListLocal::Bool {
            local: execution::BoolListLocalId(local.0),
            type_id: context.bool_list_type(),
        },
        module::ListLocal::Nil(local) => execution::ListLocal::Nil {
            local: execution::NilListLocalId(local.0),
            type_id: context.nil_list_type(),
        },
        module::ListLocal::Tuple { local, item_type } => execution::ListLocal::Tuple {
            local: execution::TupleListLocalId(local.0),
            type_id: context.tuple_list_type(item_type),
        },
        module::ListLocal::List { local, item_type } => execution::ListLocal::List {
            local: execution::ListListLocalId(local.0),
            type_id: context.list_list_type(*item_type),
        },
        module::ListLocal::Function { local, item_type } => execution::ListLocal::Function {
            local: execution::FunctionListLocalId(local.0),
            type_id: context.function_list_type(item_type),
        },
    }
}

pub(super) fn list_function_local(
    local: module::ListFunctionLocal,
    context: &mut LoweringContext,
) -> super::super::ListFunctionLocal {
    use super::super as execution;

    match local {
        module::ListFunctionLocal::Int { local, type_ } => execution::ListFunctionLocal::Int {
            local: execution::IntListFunctionLocalId(local.0),
            type_: context.function_type(type_),
            list_type: context.int_list_type(),
        },
        module::ListFunctionLocal::String { local, type_ } => {
            execution::ListFunctionLocal::String {
                local: execution::StringListFunctionLocalId(local.0),
                type_: context.function_type(type_),
                list_type: context.string_list_type(),
            }
        }
        module::ListFunctionLocal::BitArray { local, type_ } => {
            execution::ListFunctionLocal::BitArray {
                local: execution::BitArrayListFunctionLocalId(local.0),
                type_: context.function_type(type_),
                list_type: context.bit_array_list_type(),
            }
        }
        module::ListFunctionLocal::UtfCodepoint { local, type_ } => {
            execution::ListFunctionLocal::UtfCodepoint {
                local: execution::UtfCodepointListFunctionLocalId(local.0),
                type_: context.function_type(type_),
                list_type: context.utf_codepoint_list_type(),
            }
        }
        module::ListFunctionLocal::Custom {
            local,
            type_,
            item_type,
        } => execution::ListFunctionLocal::Custom {
            local: execution::CustomListFunctionLocalId(local.0),
            type_: context.function_type(type_),
            list_type: context.custom_list_type(item_type),
        },
        module::ListFunctionLocal::Float { local, type_ } => execution::ListFunctionLocal::Float {
            local: execution::FloatListFunctionLocalId(local.0),
            type_: context.function_type(type_),
            list_type: context.float_list_type(),
        },
        module::ListFunctionLocal::Bool { local, type_ } => execution::ListFunctionLocal::Bool {
            local: execution::BoolListFunctionLocalId(local.0),
            type_: context.function_type(type_),
            list_type: context.bool_list_type(),
        },
        module::ListFunctionLocal::Nil { local, type_ } => execution::ListFunctionLocal::Nil {
            local: execution::NilListFunctionLocalId(local.0),
            type_: context.function_type(type_),
            list_type: context.nil_list_type(),
        },
        module::ListFunctionLocal::Tuple {
            local,
            type_,
            item_type,
        } => execution::ListFunctionLocal::Tuple {
            local: execution::TupleListFunctionLocalId(local.0),
            type_: context.function_type(type_),
            list_type: context.tuple_list_type(item_type),
        },
        module::ListFunctionLocal::List {
            local,
            type_,
            item_type,
        } => execution::ListFunctionLocal::List {
            local: execution::ListListFunctionLocalId(local.0),
            type_: context.function_type(type_),
            list_type: context.list_list_type(*item_type),
        },
        module::ListFunctionLocal::Function {
            local,
            type_,
            item_type,
        } => execution::ListFunctionLocal::Function {
            local: execution::FunctionListFunctionLocalId(local.0),
            type_: context.function_type(type_),
            list_type: context.function_list_type(*item_type),
        },
    }
}

pub(super) fn list_function_id(
    id: module::ListFunctionId,
    context: &mut LoweringContext,
) -> super::super::ListFunctionId {
    use super::super as execution;

    match id {
        module::ListFunctionId::Int(id) => execution::ListFunctionId::Int(
            execution::IntListFunctionId::new(id.0, context.int_list_type()),
        ),
        module::ListFunctionId::String(id) => execution::ListFunctionId::String(
            execution::StringListFunctionId::new(id.0, context.string_list_type()),
        ),
        module::ListFunctionId::BitArray(id) => execution::ListFunctionId::BitArray(
            execution::BitArrayListFunctionId::new(id.0, context.bit_array_list_type()),
        ),
        module::ListFunctionId::UtfCodepoint(id) => execution::ListFunctionId::UtfCodepoint(
            execution::UtfCodepointListFunctionId::new(id.0, context.utf_codepoint_list_type()),
        ),
        module::ListFunctionId::Custom { id, item_type } => execution::ListFunctionId::Custom(
            execution::CustomListFunctionId::new(id.0, context.custom_list_type(item_type)),
        ),
        module::ListFunctionId::Float(id) => execution::ListFunctionId::Float(
            execution::FloatListFunctionId::new(id.0, context.float_list_type()),
        ),
        module::ListFunctionId::Bool(id) => execution::ListFunctionId::Bool(
            execution::BoolListFunctionId::new(id.0, context.bool_list_type()),
        ),
        module::ListFunctionId::Nil(id) => execution::ListFunctionId::Nil(
            execution::NilListFunctionId::new(id.0, context.nil_list_type()),
        ),
        module::ListFunctionId::Tuple { id, item_type } => execution::ListFunctionId::Tuple(
            execution::TupleListFunctionId::new(id.0, context.tuple_list_type(item_type)),
        ),
        module::ListFunctionId::List { id, item_type } => execution::ListFunctionId::List(
            execution::ListListFunctionId::new(id.0, context.list_list_type(*item_type)),
        ),
        module::ListFunctionId::Function { id, item_type } => execution::ListFunctionId::Function(
            execution::FunctionListFunctionId::new(id.0, context.function_list_type(item_type)),
        ),
    }
}

pub(super) fn function_function_id(
    id: module::FunctionFunctionId,
    context: &mut LoweringContext,
) -> super::super::FunctionFunctionId {
    use super::super as execution;

    match id {
        module::FunctionFunctionId::Int(id) => {
            execution::FunctionFunctionId::Int(execution::IntFunctionFunctionId(id.0))
        }
        module::FunctionFunctionId::Float(id) => {
            execution::FunctionFunctionId::Float(execution::FloatFunctionFunctionId(id.0))
        }
        module::FunctionFunctionId::String(id) => {
            execution::FunctionFunctionId::String(execution::StringFunctionFunctionId(id.0))
        }
        module::FunctionFunctionId::BitArray(id) => {
            execution::FunctionFunctionId::BitArray(execution::BitArrayFunctionFunctionId(id.0))
        }
        module::FunctionFunctionId::UtfCodepoint(id) => {
            execution::FunctionFunctionId::UtfCodepoint(execution::UtfCodepointFunctionFunctionId(
                id.0,
            ))
        }
        module::FunctionFunctionId::Custom(id) => {
            execution::FunctionFunctionId::Custom(execution::CustomFunctionFunctionId(id.0))
        }
        module::FunctionFunctionId::Bool(id) => {
            execution::FunctionFunctionId::Bool(execution::BoolFunctionFunctionId(id.0))
        }
        module::FunctionFunctionId::Nil(id) => {
            execution::FunctionFunctionId::Nil(execution::NilFunctionFunctionId(id.0))
        }
        module::FunctionFunctionId::Tuple(id) => {
            execution::FunctionFunctionId::Tuple(execution::TupleFunctionFunctionId(id.0))
        }
        module::FunctionFunctionId::List(id) => {
            execution::FunctionFunctionId::List(list_function_function_id(id, context))
        }
        module::FunctionFunctionId::Function(id) => {
            execution::FunctionFunctionId::Function(execution::FunctionFunctionFunctionId(id.0))
        }
    }
}

pub(super) fn list_function_function_id(
    id: module::ListFunctionFunctionId,
    context: &mut LoweringContext,
) -> super::super::ListFunctionFunctionId {
    use super::super as execution;

    match id {
        module::ListFunctionFunctionId::Int { id, type_ } => {
            execution::ListFunctionFunctionId::Int {
                id: execution::IntListFunctionFunctionId(id.0),
                type_: context.function_type(type_),
                list_type: context.int_list_type(),
            }
        }
        module::ListFunctionFunctionId::String { id, type_ } => {
            execution::ListFunctionFunctionId::String {
                id: execution::StringListFunctionFunctionId(id.0),
                type_: context.function_type(type_),
                list_type: context.string_list_type(),
            }
        }
        module::ListFunctionFunctionId::BitArray { id, type_ } => {
            execution::ListFunctionFunctionId::BitArray {
                id: execution::BitArrayListFunctionFunctionId(id.0),
                type_: context.function_type(type_),
                list_type: context.bit_array_list_type(),
            }
        }
        module::ListFunctionFunctionId::UtfCodepoint { id, type_ } => {
            execution::ListFunctionFunctionId::UtfCodepoint {
                id: execution::UtfCodepointListFunctionFunctionId(id.0),
                type_: context.function_type(type_),
                list_type: context.utf_codepoint_list_type(),
            }
        }
        module::ListFunctionFunctionId::Custom {
            id,
            type_,
            item_type,
        } => execution::ListFunctionFunctionId::Custom {
            id: execution::CustomListFunctionFunctionId(id.0),
            type_: context.function_type(type_),
            list_type: context.custom_list_type(item_type),
        },
        module::ListFunctionFunctionId::Float { id, type_ } => {
            execution::ListFunctionFunctionId::Float {
                id: execution::FloatListFunctionFunctionId(id.0),
                type_: context.function_type(type_),
                list_type: context.float_list_type(),
            }
        }
        module::ListFunctionFunctionId::Bool { id, type_ } => {
            execution::ListFunctionFunctionId::Bool {
                id: execution::BoolListFunctionFunctionId(id.0),
                type_: context.function_type(type_),
                list_type: context.bool_list_type(),
            }
        }
        module::ListFunctionFunctionId::Nil { id, type_ } => {
            execution::ListFunctionFunctionId::Nil {
                id: execution::NilListFunctionFunctionId(id.0),
                type_: context.function_type(type_),
                list_type: context.nil_list_type(),
            }
        }
        module::ListFunctionFunctionId::Tuple {
            id,
            type_,
            item_type,
        } => execution::ListFunctionFunctionId::Tuple {
            id: execution::TupleListFunctionFunctionId(id.0),
            type_: context.function_type(type_),
            list_type: context.tuple_list_type(item_type),
        },
        module::ListFunctionFunctionId::List {
            id,
            type_,
            item_type,
        } => execution::ListFunctionFunctionId::List {
            id: execution::ListListFunctionFunctionId(id.0),
            type_: context.function_type(type_),
            list_type: context.list_list_type(*item_type),
        },
        module::ListFunctionFunctionId::Function {
            id,
            type_,
            item_type,
        } => execution::ListFunctionFunctionId::Function {
            id: execution::FunctionListFunctionFunctionId(id.0),
            type_: context.function_type(type_),
            list_type: context.function_list_type(*item_type),
        },
    }
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
            StepKind::LetListFunction { local, value } => (local, value),
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
