use crate::plan::module;

pub(super) fn list_local(local: module::ListLocal) -> super::super::ListLocal {
    use super::super as execution;

    match local {
        module::ListLocal::Int(local) => {
            execution::ListLocal::Int(execution::IntListLocalId(local.0))
        }
        module::ListLocal::String(local) => {
            execution::ListLocal::String(execution::StringListLocalId(local.0))
        }
        module::ListLocal::Float(local) => {
            execution::ListLocal::Float(execution::FloatListLocalId(local.0))
        }
        module::ListLocal::Bool(local) => {
            execution::ListLocal::Bool(execution::BoolListLocalId(local.0))
        }
        module::ListLocal::Nil(local) => {
            execution::ListLocal::Nil(execution::NilListLocalId(local.0))
        }
        module::ListLocal::Tuple { local, item_type } => execution::ListLocal::Tuple {
            local: execution::TupleListLocalId(local.0),
            item_type,
        },
        module::ListLocal::List { local, item_type } => execution::ListLocal::List {
            local: execution::ListListLocalId(local.0),
            item_type,
        },
        module::ListLocal::Function { local, item_type } => execution::ListLocal::Function {
            local: execution::FunctionListLocalId(local.0),
            item_type,
        },
    }
}

pub(super) fn list_function_local(
    local: module::ListFunctionLocal,
) -> super::super::ListFunctionLocal {
    use super::super as execution;

    match local {
        module::ListFunctionLocal::Int { local, type_ } => execution::ListFunctionLocal::Int {
            local: execution::IntListFunctionLocalId(local.0),
            type_,
        },
        module::ListFunctionLocal::String { local, type_ } => {
            execution::ListFunctionLocal::String {
                local: execution::StringListFunctionLocalId(local.0),
                type_,
            }
        }
        module::ListFunctionLocal::Float { local, type_ } => execution::ListFunctionLocal::Float {
            local: execution::FloatListFunctionLocalId(local.0),
            type_,
        },
        module::ListFunctionLocal::Bool { local, type_ } => execution::ListFunctionLocal::Bool {
            local: execution::BoolListFunctionLocalId(local.0),
            type_,
        },
        module::ListFunctionLocal::Nil { local, type_ } => execution::ListFunctionLocal::Nil {
            local: execution::NilListFunctionLocalId(local.0),
            type_,
        },
        module::ListFunctionLocal::Tuple {
            local,
            type_,
            item_type,
        } => execution::ListFunctionLocal::Tuple {
            local: execution::TupleListFunctionLocalId(local.0),
            type_,
            item_type,
        },
        module::ListFunctionLocal::List {
            local,
            type_,
            item_type,
        } => execution::ListFunctionLocal::List {
            local: execution::ListListFunctionLocalId(local.0),
            type_,
            item_type,
        },
        module::ListFunctionLocal::Function {
            local,
            type_,
            item_type,
        } => execution::ListFunctionLocal::Function {
            local: execution::FunctionListFunctionLocalId(local.0),
            type_,
            item_type,
        },
    }
}

pub(super) fn list_function_id(id: module::ListFunctionId) -> super::super::ListFunctionId {
    use super::super as execution;

    match id {
        module::ListFunctionId::Int(id) => {
            execution::ListFunctionId::Int(execution::IntListFunctionId(id.0))
        }
        module::ListFunctionId::String(id) => {
            execution::ListFunctionId::String(execution::StringListFunctionId(id.0))
        }
        module::ListFunctionId::Float(id) => {
            execution::ListFunctionId::Float(execution::FloatListFunctionId(id.0))
        }
        module::ListFunctionId::Bool(id) => {
            execution::ListFunctionId::Bool(execution::BoolListFunctionId(id.0))
        }
        module::ListFunctionId::Nil(id) => {
            execution::ListFunctionId::Nil(execution::NilListFunctionId(id.0))
        }
        module::ListFunctionId::Tuple { id, item_type } => execution::ListFunctionId::Tuple {
            id: execution::TupleListFunctionId(id.0),
            item_type,
        },
        module::ListFunctionId::List { id, item_type } => execution::ListFunctionId::List {
            id: execution::ListListFunctionId(id.0),
            item_type,
        },
        module::ListFunctionId::Function { id, item_type } => execution::ListFunctionId::Function {
            id: execution::FunctionListFunctionId(id.0),
            item_type,
        },
    }
}

pub(super) fn function_function_id(
    id: module::FunctionFunctionId,
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
            execution::FunctionFunctionId::List(list_function_function_id(id))
        }
        module::FunctionFunctionId::Function(id) => {
            execution::FunctionFunctionId::Function(execution::FunctionFunctionFunctionId(id.0))
        }
    }
}

pub(super) fn list_function_function_id(
    id: module::ListFunctionFunctionId,
) -> super::super::ListFunctionFunctionId {
    use super::super as execution;

    match id {
        module::ListFunctionFunctionId::Int { id, type_ } => {
            execution::ListFunctionFunctionId::Int {
                id: execution::IntListFunctionFunctionId(id.0),
                type_,
            }
        }
        module::ListFunctionFunctionId::String { id, type_ } => {
            execution::ListFunctionFunctionId::String {
                id: execution::StringListFunctionFunctionId(id.0),
                type_,
            }
        }
        module::ListFunctionFunctionId::Float { id, type_ } => {
            execution::ListFunctionFunctionId::Float {
                id: execution::FloatListFunctionFunctionId(id.0),
                type_,
            }
        }
        module::ListFunctionFunctionId::Bool { id, type_ } => {
            execution::ListFunctionFunctionId::Bool {
                id: execution::BoolListFunctionFunctionId(id.0),
                type_,
            }
        }
        module::ListFunctionFunctionId::Nil { id, type_ } => {
            execution::ListFunctionFunctionId::Nil {
                id: execution::NilListFunctionFunctionId(id.0),
                type_,
            }
        }
        module::ListFunctionFunctionId::Tuple {
            id,
            type_,
            item_type,
        } => execution::ListFunctionFunctionId::Tuple {
            id: execution::TupleListFunctionFunctionId(id.0),
            type_,
            item_type,
        },
        module::ListFunctionFunctionId::List {
            id,
            type_,
            item_type,
        } => execution::ListFunctionFunctionId::List {
            id: execution::ListListFunctionFunctionId(id.0),
            type_,
            item_type,
        },
        module::ListFunctionFunctionId::Function {
            id,
            type_,
            item_type,
        } => execution::ListFunctionFunctionId::Function {
            id: execution::FunctionListFunctionFunctionId(id.0),
            type_,
            item_type,
        },
    }
}
