mod scoped;

use self::scoped::ScopedValues;
use crate::host::{
    HostCallArguments, HostCallRuntime, HostCustomArgumentSlot, HostCustomToken,
    HostFunctionArgumentSlot, HostFunctionToken, HostListArgumentSlot, HostListToken, HostProfile,
    HostScopedValue, HostTupleArgumentSlot, HostTupleToken, HostValueArgumentSlot, HostValueToken,
};
use crate::plan::execution::host::{HostCallParameter, HostedFunction};
use crate::plan::execution::type_::{ListTypeId, ValueType};
use crate::runtime::ExecutableRuntimePlan;
use crate::runtime::evaluated::EvaluatedCustomValue;
use crate::runtime::graph::{BlockEnvironment, RetainedValues};
use crate::runtime::state::RuntimeStateFor;
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) struct RuntimeHostCall<'call, 'run, Plan, Profile>
where
    Plan: ExecutableRuntimePlan<RunState = Profile::RunState> + 'run,
    Profile: HostProfile,
{
    plan: &'call Plan,
    state: &'call mut RuntimeStateFor<'run, Plan>,
    arguments: RetainedValues,
    value_arguments: Vec<HostValueToken>,
    list_arguments: Vec<HostListToken>,
    tuple_arguments: Vec<HostTupleToken>,
    custom_arguments: Vec<HostCustomToken>,
    function_arguments: Vec<HostFunctionToken>,
    scoped: ScopedValues,
    return_lists: Box<[ListTypeId]>,
    return_customs: Box<[crate::plan::execution::type_::CustomTypeId]>,
    origin: crate::runtime::error::HostCallOrigin,
    profile: std::marker::PhantomData<Profile>,
}

impl<'call, 'run, Plan, Profile> RuntimeHostCall<'call, 'run, Plan, Profile>
where
    Plan: ExecutableRuntimePlan<RunState = Profile::RunState> + 'run,
    Profile: HostProfile,
{
    pub(super) fn new(
        plan: &'call Plan,
        state: &'call mut RuntimeStateFor<'run, Plan>,
        function: &HostedFunction<impl Sized>,
        inputs: RetainedValues,
    ) -> Self {
        let environment = BlockEnvironment::from_retained(inputs);
        let mut arguments = RetainedValues::empty();
        let mut scoped = ScopedValues::default();
        let mut value_arguments = Vec::new();
        let mut list_arguments = Vec::new();
        let mut tuple_arguments = Vec::new();
        let mut custom_arguments = Vec::new();
        let mut function_arguments = Vec::new();

        for parameter in function.call_parameters() {
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
                HostCallParameter::Function(_) => {
                    let token = scoped.push(environment.value(&parameter.local()));
                    function_arguments.push(scoped.function_token(token));
                }
            }
        }

        drop(environment);
        state.values_mut().drain_releases();

        let mut return_lists = Vec::new();
        let mut return_customs = Vec::new();
        match function.type_().return_() {
            ValueType::List(type_id) => return_lists.push(type_id.to_owned()),
            ValueType::Custom(type_id) => return_customs.push(type_id.to_owned()),
            ValueType::Parameter(_)
            | ValueType::Int
            | ValueType::Float
            | ValueType::String
            | ValueType::BitArray
            | ValueType::UtfCodepoint
            | ValueType::Bool
            | ValueType::Nil
            | ValueType::Tuple(_)
            | ValueType::Function(_) => {}
        }

        Self {
            plan,
            state,
            arguments,
            value_arguments,
            list_arguments,
            tuple_arguments,
            custom_arguments,
            function_arguments,
            scoped,
            return_lists: return_lists.into_boxed_slice(),
            return_customs: return_customs.into_boxed_slice(),
            origin: crate::runtime::error::HostCallOrigin::host(function.metadata()),
            profile: std::marker::PhantomData,
        }
    }

    pub(super) fn finish<Value: crate::runtime::graph::GraphValue>(
        &self,
        returned: HostValueToken,
        local: &Value,
    ) -> Value::Evaluated {
        let mut retained = RetainedValues::empty();
        self.scoped.retain(returned, &mut retained);
        local.read(&BlockEnvironment::from_retained(retained))
    }
}

impl<'run, Plan, Profile> HostCallRuntime<Profile> for RuntimeHostCall<'_, 'run, Plan, Profile>
where
    Plan: ExecutableRuntimePlan<RunState = Profile::RunState> + 'run,
    Profile: HostProfile,
{
    fn state(&mut self) -> &mut Profile::RunState {
        self.state.host_state()
    }

    fn arguments(&self) -> &dyn HostCallArguments {
        &self.arguments
    }

    fn scalar_context(&mut self) -> (&mut Profile::RunState, &dyn HostCallArguments) {
        (self.state.host_state(), &self.arguments)
    }

    fn value(&self, slot: HostValueArgumentSlot) -> HostValueToken {
        self.value_arguments[slot.index()]
    }

    fn list(&self, slot: HostListArgumentSlot) -> HostListToken {
        self.list_arguments[slot.index()]
    }

    fn tuple(&self, slot: HostTupleArgumentSlot) -> HostTupleToken {
        self.tuple_arguments[slot.index()]
    }

    fn custom(&self, slot: HostCustomArgumentSlot) -> HostCustomToken {
        self.custom_arguments[slot.index()]
    }

    fn function(&self, slot: HostFunctionArgumentSlot) -> HostFunctionToken {
        self.function_arguments[slot.index()]
    }

    fn int(&self, value: HostValueToken) -> BigInt {
        self.scoped.int(value)
    }

    fn float(&self, value: HostValueToken) -> f64 {
        self.scoped.float(value)
    }

    fn string(&self, value: HostValueToken) -> EcoString {
        self.scoped.string(value)
    }

    fn bit_array(&self, value: HostValueToken) -> crate::BitArrayValue {
        self.scoped.bit_array(value)
    }

    fn utf_codepoint(&self, value: HostValueToken) -> char {
        self.scoped.utf_codepoint(value)
    }

    fn bool(&self, value: HostValueToken) -> bool {
        self.scoped.bool(value)
    }

    fn nil(&self, _value: HostValueToken) {}

    fn list_token(&self, value: HostValueToken) -> HostListToken {
        self.scoped.list_token(value)
    }

    fn tuple_token(&self, value: HostValueToken) -> HostTupleToken {
        self.scoped.tuple_token(value)
    }

    fn custom_token(&self, value: HostValueToken) -> HostCustomToken {
        self.scoped.custom_token(value)
    }

    fn function_token(&self, value: HostValueToken) -> HostFunctionToken {
        self.scoped.function_token(value)
    }

    fn list_len(&self, value: HostListToken) -> usize {
        self.state.values().list_len(&self.scoped.list_value(value))
    }

    fn list_item(&mut self, value: HostListToken, index: usize) -> Option<HostValueToken> {
        let value = self.scoped.list_value(value);
        self.state
            .values()
            .evaluated_value_at(&value, index)
            .map(|value| self.scoped.push(value))
    }

    fn tuple_len(&self, value: HostTupleToken) -> usize {
        self.scoped.tuple_len(value)
    }

    fn tuple_values(&mut self, value: HostTupleToken) -> Box<[HostValueToken]> {
        self.scoped
            .tuple_values(value)
            .into_iter()
            .map(|value| self.scoped.push(value))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn custom_constructor(&self, value: HostCustomToken) -> usize {
        self.scoped.custom_constructor(value)
    }

    fn custom_fields(&mut self, value: HostCustomToken) -> Box<[HostValueToken]> {
        self.scoped
            .custom_fields(value)
            .into_iter()
            .map(|value| self.scoped.push(value))
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    fn invoke(
        &mut self,
        function: HostFunctionToken,
        arguments: Box<[HostScopedValue]>,
    ) -> Result<HostValueToken, crate::HostCallError> {
        let function = self.scoped.function(function);
        let arguments = arguments
            .into_vec()
            .into_iter()
            .map(|value| self.scoped.value_from_scoped(value))
            .collect::<Vec<_>>();
        let mut inputs = RetainedValues::empty();
        for value in &arguments {
            inputs.push_evaluated(value.clone());
        }
        crate::runtime::function::invoke_callable(
            self.plan,
            self.state,
            function,
            self.origin.clone(),
            inputs,
            arguments.into_boxed_slice(),
        )
        .map(|value| self.scoped.push(value))
        .map_err(crate::HostCallError::nested)
    }

    fn equal(&self, left: HostScopedValue, right: HostScopedValue) -> bool {
        crate::runtime::evaluated::values_equal(
            self.plan,
            self.state,
            &self.scoped.value_from_scoped(left),
            &self.scoped.value_from_scoped(right),
        )
    }

    fn complete(&mut self, value: HostScopedValue) -> HostValueToken {
        self.scoped.push_scoped(value)
    }

    fn build_list(&mut self, values: Box<[HostScopedValue]>) -> HostValueToken {
        let values = values
            .into_vec()
            .into_iter()
            .map(|value| self.scoped.push_scoped(value))
            .collect::<Vec<_>>();
        let list = self
            .scoped
            .allocate_list(self.plan, self.state, self.return_lists[0], &values);
        self.scoped.push_list(list)
    }

    fn build_tuple(&mut self, values: Box<[HostScopedValue]>) -> HostValueToken {
        let values = values
            .into_vec()
            .into_iter()
            .map(|value| self.scoped.value_from_scoped(value))
            .collect();
        self.scoped.push_tuple(values)
    }

    fn build_custom(
        &mut self,
        constructor: usize,
        fields: Box<[HostScopedValue]>,
    ) -> HostValueToken {
        let fields = fields
            .into_vec()
            .into_iter()
            .map(|value| self.scoped.value_from_scoped(value))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let constructor = self
            .plan
            .custom_constructor_id(self.return_customs[0], constructor);
        self.scoped
            .push_custom(EvaluatedCustomValue::from_fields(constructor, fields))
    }
}
