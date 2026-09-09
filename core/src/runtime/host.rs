mod call;
mod invoke;
mod scoped;

pub(in crate::runtime) use self::call::RuntimeHostCall;
pub(super) use self::invoke::{invoke_never, invoke_value};

use self::scoped::ScopedValues;
pub(crate) use self::scoped::{
    StoredRuntimeList, StoredRuntimeListCustomFields, StoredRuntimeListItem,
    StoredRuntimeListTupleItems, StoredRuntimeValue,
};
use crate::host::{
    HostCustomToken, HostExternalToken, HostFunctionToken, HostListToken, HostTupleToken,
    HostValueToken,
};
use crate::plan::execution::host::HostCallParameter;
use crate::runtime::graph::{BlockEnvironment, RetainedValues};

struct PreparedHostCall {
    arguments: RetainedValues,
    value_arguments: Vec<HostValueToken>,
    list_arguments: Vec<HostListToken>,
    tuple_arguments: Vec<HostTupleToken>,
    custom_arguments: Vec<HostCustomToken>,
    external_arguments: Vec<HostExternalToken>,
    function_arguments: Vec<HostFunctionToken>,
    scoped: ScopedValues,
}

impl PreparedHostCall {
    fn new(parameters: &[HostCallParameter], inputs: RetainedValues) -> Self {
        let environment = BlockEnvironment::from_retained(inputs);
        let mut arguments = RetainedValues::empty();
        let mut scoped = ScopedValues::default();
        let mut value_arguments = Vec::new();
        let mut list_arguments = Vec::new();
        let mut tuple_arguments = Vec::new();
        let mut custom_arguments = Vec::new();
        let mut external_arguments = Vec::new();
        let mut function_arguments = Vec::new();

        for parameter in parameters {
            match parameter {
                HostCallParameter::Int(_)
                | HostCallParameter::Float(_)
                | HostCallParameter::String(_)
                | HostCallParameter::BitArray(_)
                | HostCallParameter::UtfCodepoint(_)
                | HostCallParameter::Bool(_)
                | HostCallParameter::Nil(_) => {
                    arguments.push_evaluated(environment.value(&parameter.local()));
                }
                HostCallParameter::Value(_) => {
                    value_arguments.push(scoped.push(environment.value(&parameter.local())));
                }
                HostCallParameter::List(_) => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    list_arguments.push(scoped.list_token(token));
                }
                HostCallParameter::Tuple(_) => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    tuple_arguments.push(scoped.tuple_token(token));
                }
                HostCallParameter::Custom(_) => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    custom_arguments.push(scoped.custom_token(token));
                }
                HostCallParameter::External(_) => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    external_arguments.push(scoped.external_token(token));
                }
                HostCallParameter::Function { .. } => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    function_arguments.push(scoped.function_token(token));
                }
            }
        }

        Self {
            arguments,
            value_arguments,
            list_arguments,
            tuple_arguments,
            custom_arguments,
            external_arguments,
            function_arguments,
            scoped,
        }
    }
}

#[cfg(test)]
pub(crate) mod call_fixture;
