use crate::host::{
    HostCallArguments, HostCustomArgumentSlot, HostCustomToken, HostFunctionArgumentSlot,
    HostFunctionToken, HostListArgumentSlot, HostListToken, HostProfile, HostScopedValue,
    HostTupleArgumentSlot, HostTupleToken, HostValueArgumentSlot, HostValueToken,
};

pub(crate) trait HostCallRuntime<Profile: HostProfile> {
    fn state(&mut self) -> &mut Profile::RunState;
    fn arguments(&self) -> &dyn HostCallArguments;
    fn scalar_context(&mut self) -> (&mut Profile::RunState, &dyn HostCallArguments);
    fn value(&self, slot: HostValueArgumentSlot) -> HostValueToken;
    fn list(&self, slot: HostListArgumentSlot) -> HostListToken;
    fn tuple(&self, slot: HostTupleArgumentSlot) -> HostTupleToken;
    fn custom(&self, slot: HostCustomArgumentSlot) -> HostCustomToken;
    fn function(&self, slot: HostFunctionArgumentSlot) -> HostFunctionToken;
    fn int(&self, value: HostValueToken) -> num_bigint::BigInt;
    fn float(&self, value: HostValueToken) -> f64;
    fn string(&self, value: HostValueToken) -> ecow::EcoString;
    fn bit_array(&self, value: HostValueToken) -> crate::BitArrayValue;
    fn utf_codepoint(&self, value: HostValueToken) -> char;
    fn bool(&self, value: HostValueToken) -> bool;
    fn nil(&self, value: HostValueToken);
    fn list_token(&self, value: HostValueToken) -> HostListToken;
    fn tuple_token(&self, value: HostValueToken) -> HostTupleToken;
    fn custom_token(&self, value: HostValueToken) -> HostCustomToken;
    fn function_token(&self, value: HostValueToken) -> HostFunctionToken;
    fn list_len(&self, value: HostListToken) -> usize;
    fn list_item(&mut self, value: HostListToken, index: usize) -> Option<HostValueToken>;
    fn tuple_len(&self, value: HostTupleToken) -> usize;
    fn tuple_values(&mut self, value: HostTupleToken) -> Box<[HostValueToken]>;
    fn custom_constructor(&self, value: HostCustomToken) -> usize;
    fn custom_fields(&mut self, value: HostCustomToken) -> Box<[HostValueToken]>;
    fn invoke(
        &mut self,
        function: HostFunctionToken,
        arguments: Box<[HostScopedValue]>,
    ) -> Result<HostValueToken, crate::HostCallError>;
    fn equal(&self, left: HostScopedValue, right: HostScopedValue) -> bool;
    fn complete(&mut self, value: HostScopedValue) -> HostValueToken;
    fn build_list(&mut self, values: Box<[HostScopedValue]>) -> HostValueToken;
    fn build_tuple(&mut self, values: Box<[HostScopedValue]>) -> HostValueToken;
    fn build_custom(
        &mut self,
        constructor: usize,
        fields: Box<[HostScopedValue]>,
    ) -> HostValueToken;
}

#[cfg(test)]
pub(crate) mod test {
    use super::HostCallRuntime;
    use crate::host::{
        HostCall, HostCallArguments, HostCallCompletion, HostCallError, HostCustomArgumentSlot,
        HostCustomToken, HostFunctionArgumentSlot, HostFunctionToken, HostListArgumentSlot,
        HostListToken, HostProfile, HostProvider, HostScopedValue, HostTupleArgumentSlot,
        HostTupleToken, HostTypeParameter, HostValue, HostValueArgumentSlot, HostValueFamily,
        HostValueToken, StatelessHostProfile,
    };

    pub(crate) struct TestHostProfile;
    pub(crate) struct StatelessTestProvider;
    pub(crate) type TestTypeParameter = HostTypeParameter<0>;

    #[derive(Default)]
    pub(crate) struct TestRunState {
        pub(crate) counter: usize,
        pub(crate) unrelated: bool,
    }

    pub(crate) struct TestHostCallRuntime<'state> {
        state: &'state mut TestRunState,
        arguments: Box<dyn HostCallArguments>,
        completed: Option<HostScopedValue>,
    }

    impl HostProfile for TestHostProfile {
        type RunState = TestRunState;
    }

    impl HostProvider<StatelessHostProfile> for StatelessTestProvider {
        type State = ();

        fn project(state: &mut ()) -> &mut Self::State {
            state
        }
    }

    pub(crate) fn stateless_identity<'call>(
        call: HostCall<'call, StatelessHostProfile, StatelessTestProvider, TestTypeParameter>,
        value: HostValue<'call, TestTypeParameter>,
    ) -> Result<HostCallCompletion<'call, TestTypeParameter>, HostCallError> {
        Ok(call.return_value(value))
    }

    impl<'state> TestHostCallRuntime<'state> {
        pub(crate) fn new(
            state: &'state mut TestRunState,
            arguments: impl HostCallArguments + 'static,
        ) -> Self {
            Self {
                state,
                arguments: Box::new(arguments),
                completed: None,
            }
        }

        pub(crate) fn completed(&self) -> Option<&HostScopedValue> {
            self.completed.as_ref()
        }
    }

    impl HostCallRuntime<TestHostProfile> for TestHostCallRuntime<'_> {
        fn state(&mut self) -> &mut TestRunState {
            self.state
        }

        fn arguments(&self) -> &dyn HostCallArguments {
            self.arguments.as_ref()
        }

        fn scalar_context(&mut self) -> (&mut TestRunState, &dyn HostCallArguments) {
            (self.state, self.arguments.as_ref())
        }

        fn value(&self, _slot: HostValueArgumentSlot) -> HostValueToken {
            HostValueToken {
                family: HostValueFamily::Bool,
                index: 0,
            }
        }

        fn list(&self, _slot: HostListArgumentSlot) -> HostListToken {
            HostListToken::Stored(0)
        }

        fn tuple(&self, _slot: HostTupleArgumentSlot) -> HostTupleToken {
            HostTupleToken(0)
        }

        fn custom(&self, _slot: HostCustomArgumentSlot) -> HostCustomToken {
            HostCustomToken(0)
        }

        fn function(&self, _slot: HostFunctionArgumentSlot) -> HostFunctionToken {
            HostFunctionToken(0)
        }

        fn int(&self, _value: HostValueToken) -> num_bigint::BigInt {
            0.into()
        }

        fn float(&self, _value: HostValueToken) -> f64 {
            0.0
        }

        fn string(&self, _value: HostValueToken) -> ecow::EcoString {
            "".into()
        }

        fn bit_array(&self, _value: HostValueToken) -> crate::BitArrayValue {
            crate::BitArrayValue::from_bytes(Vec::new())
        }

        fn utf_codepoint(&self, _value: HostValueToken) -> char {
            '\0'
        }

        fn bool(&self, _value: HostValueToken) -> bool {
            false
        }

        fn nil(&self, _value: HostValueToken) {}

        fn list_token(&self, _value: HostValueToken) -> HostListToken {
            HostListToken::Stored(0)
        }

        fn tuple_token(&self, _value: HostValueToken) -> HostTupleToken {
            HostTupleToken(0)
        }

        fn custom_token(&self, _value: HostValueToken) -> HostCustomToken {
            HostCustomToken(0)
        }

        fn function_token(&self, _value: HostValueToken) -> HostFunctionToken {
            HostFunctionToken(0)
        }

        fn list_len(&self, _value: HostListToken) -> usize {
            0
        }

        fn list_item(&mut self, _value: HostListToken, _index: usize) -> Option<HostValueToken> {
            None
        }

        fn tuple_len(&self, _value: HostTupleToken) -> usize {
            0
        }

        fn tuple_values(&mut self, _value: HostTupleToken) -> Box<[HostValueToken]> {
            Box::new([])
        }

        fn custom_constructor(&self, _value: HostCustomToken) -> usize {
            0
        }

        fn custom_fields(&mut self, _value: HostCustomToken) -> Box<[HostValueToken]> {
            Box::new([])
        }

        fn invoke(
            &mut self,
            _function: HostFunctionToken,
            arguments: Box<[HostScopedValue]>,
        ) -> Result<HostValueToken, HostCallError> {
            match arguments.into_vec().into_iter().next() {
                Some(value) => Ok(self.complete(value)),
                None => Ok(token(HostValueFamily::Nil)),
            }
        }

        fn equal(&self, _left: HostScopedValue, _right: HostScopedValue) -> bool {
            false
        }

        fn complete(&mut self, value: HostScopedValue) -> HostValueToken {
            let token = match &value {
                HostScopedValue::Value(token) => *token,
                HostScopedValue::Int(_) => token(HostValueFamily::Int),
                HostScopedValue::Float(_) => token(HostValueFamily::Float),
                HostScopedValue::String(_) => token(HostValueFamily::String),
                HostScopedValue::BitArray(_) => token(HostValueFamily::BitArray),
                HostScopedValue::UtfCodepoint(_) => token(HostValueFamily::UtfCodepoint),
                HostScopedValue::Bool(_) => token(HostValueFamily::Bool),
                HostScopedValue::Nil => token(HostValueFamily::Nil),
                HostScopedValue::List(_) => token(HostValueFamily::List),
                HostScopedValue::Tuple(_) => token(HostValueFamily::Tuple),
                HostScopedValue::Custom(_) => token(HostValueFamily::Custom),
                HostScopedValue::Function(_) => token(HostValueFamily::Function),
            };
            self.completed = Some(value);
            token
        }

        fn build_list(&mut self, _values: Box<[HostScopedValue]>) -> HostValueToken {
            token(HostValueFamily::List)
        }

        fn build_tuple(&mut self, _values: Box<[HostScopedValue]>) -> HostValueToken {
            token(HostValueFamily::Tuple)
        }

        fn build_custom(
            &mut self,
            _constructor: usize,
            _fields: Box<[HostScopedValue]>,
        ) -> HostValueToken {
            token(HostValueFamily::Custom)
        }
    }

    fn token(family: HostValueFamily) -> HostValueToken {
        HostValueToken { family, index: 0 }
    }
}
