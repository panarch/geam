use super::id::{list_function_local, list_local};
use crate::plan::module;

pub(super) fn param_local(local: module::ParamLocal) -> super::super::ParamLocal {
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
        module::ParamLocal::Bool(local) => {
            execution::ParamLocal::Bool(execution::BoolLocalId(local.0))
        }
        module::ParamLocal::Nil(local) => {
            execution::ParamLocal::Nil(execution::NilLocalId(local.0))
        }
        module::ParamLocal::Tuple { local, type_ } => execution::ParamLocal::Tuple {
            local: execution::TupleLocalId(local.0),
            type_,
        },
        module::ParamLocal::List(local) => execution::ParamLocal::List(list_local(local)),
        module::ParamLocal::IntFunction { local, type_ } => execution::ParamLocal::IntFunction {
            local: execution::IntFunctionLocalId(local.0),
            type_,
        },
        module::ParamLocal::FloatFunction { local, type_ } => {
            execution::ParamLocal::FloatFunction {
                local: execution::FloatFunctionLocalId(local.0),
                type_,
            }
        }
        module::ParamLocal::StringFunction { local, type_ } => {
            execution::ParamLocal::StringFunction {
                local: execution::StringFunctionLocalId(local.0),
                type_,
            }
        }
        module::ParamLocal::BoolFunction { local, type_ } => execution::ParamLocal::BoolFunction {
            local: execution::BoolFunctionLocalId(local.0),
            type_,
        },
        module::ParamLocal::NilFunction { local, type_ } => execution::ParamLocal::NilFunction {
            local: execution::NilFunctionLocalId(local.0),
            type_,
        },
        module::ParamLocal::TupleFunction { local, type_ } => {
            execution::ParamLocal::TupleFunction {
                local: execution::TupleFunctionLocalId(local.0),
                type_,
            }
        }
        module::ParamLocal::ListFunction(local) => {
            execution::ParamLocal::ListFunction(list_function_local(local))
        }
        module::ParamLocal::FunctionFunction { local, type_ } => {
            execution::ParamLocal::FunctionFunction {
                local: execution::FunctionFunctionLocalId(local.0),
                type_,
            }
        }
    }
}
