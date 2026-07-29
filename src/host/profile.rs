use crate::host::{
    HostCallArguments, HostCallCompletion, HostCustom, HostCustomArgumentSlot,
    HostCustomConstructor, HostCustomToken, HostCustomType, HostFunctionArgumentSlot,
    HostFunctionToken, HostList, HostListArgumentSlot, HostListToken, HostListType,
    HostScopedValue, HostTuple, HostTupleArgumentSlot, HostTupleToken, HostTupleType, HostType,
    HostTypeSequence, HostValue, HostValueArgumentSlot, HostValueToken,
};
use std::marker::PhantomData;

pub trait HostProfile: Send + Sync + 'static {
    type RunState;
}

pub trait HostProvider<Profile: HostProfile>: Send + Sync + 'static {
    type State;

    fn project(state: &mut Profile::RunState) -> &mut Self::State;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct StatelessHostProfile;

pub struct HostCall<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    runtime: &'call mut dyn HostCallRuntime<Profile>,
    marker: PhantomData<(Provider, Return)>,
}

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

impl HostProfile for StatelessHostProfile {
    type RunState = ();
}

impl<'call, Profile, Provider, Return> HostCall<'call, Profile, Provider, Return>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
{
    pub(crate) fn new(runtime: &'call mut dyn HostCallRuntime<Profile>) -> Self {
        Self {
            runtime,
            marker: PhantomData,
        }
    }

    pub fn state(&mut self) -> &mut Provider::State {
        Provider::project(self.runtime.state())
    }

    pub fn return_value(self, value: Return::Value<'call>) -> HostCallCompletion<'call, Return> {
        HostCallCompletion::new(
            self.runtime
                .complete(crate::host::type_::into_scoped::<Return>(value)),
        )
    }

    pub fn equal<Type: HostType>(
        &self,
        left: Type::Value<'call>,
        right: Type::Value<'call>,
    ) -> bool {
        self.runtime.equal(
            crate::host::type_::into_scoped::<Type>(left),
            crate::host::type_::into_scoped::<Type>(right),
        )
    }

    pub(crate) fn arguments(&self) -> &dyn HostCallArguments {
        self.runtime.arguments()
    }

    pub(crate) fn value<Type>(&self, slot: HostValueArgumentSlot) -> HostValue<'call, Type> {
        HostValue::new(self.runtime.value(slot))
    }

    pub(crate) fn list<Item>(&self, slot: HostListArgumentSlot) -> HostList<'call, Item> {
        HostList::new(self.runtime.list(slot))
    }

    pub(crate) fn tuple<Elements>(
        &self,
        slot: HostTupleArgumentSlot,
    ) -> HostTuple<'call, Elements> {
        HostTuple::new(self.runtime.tuple(slot))
    }

    pub(crate) fn custom<Custom>(&self, slot: HostCustomArgumentSlot) -> HostCustom<'call, Custom> {
        HostCustom::new(self.runtime.custom(slot))
    }

    pub(crate) fn function<Arguments, FunctionReturn>(
        &self,
        slot: HostFunctionArgumentSlot,
    ) -> crate::host::HostCallable<'call, Arguments, FunctionReturn> {
        crate::host::HostCallable::new(self.runtime.function(slot))
    }

    pub fn list_len<Item>(&self, value: HostList<'call, Item>) -> usize {
        self.runtime.list_len(value.token)
    }

    pub fn list_item<Item: HostType>(
        &mut self,
        value: HostList<'call, Item>,
        index: usize,
    ) -> Option<Item::Value<'call>> {
        self.runtime
            .list_item(value.token, index)
            .map(|token| crate::host::type_::from_token::<Item, Profile>(self.runtime, token))
    }

    pub fn tuple_len<Elements>(&self, value: HostTuple<'call, Elements>) -> usize {
        self.runtime.tuple_len(value.token)
    }

    pub fn tuple_values<Elements: HostTypeSequence>(
        &mut self,
        value: HostTuple<'call, Elements>,
    ) -> Elements::Values<'call> {
        let values = self.runtime.tuple_values(value.token);
        crate::host::type_::from_tokens::<Elements, Profile>(self.runtime, &values)
    }

    pub fn custom_constructor<Custom>(&self, value: HostCustom<'call, Custom>) -> usize {
        self.runtime.custom_constructor(value.token)
    }

    pub fn custom_fields<Constructor>(
        &mut self,
        value: HostCustom<'call, Constructor::Custom>,
    ) -> Option<<Constructor::Fields as HostTypeSequence>::Values<'call>>
    where
        Constructor: HostCustomConstructor,
    {
        if self.runtime.custom_constructor(value.token)
            != crate::host::type_::custom_constructor_index::<Constructor>()
        {
            return None;
        }
        let fields = self.runtime.custom_fields(value.token);
        Some(crate::host::type_::from_tokens::<
            Constructor::Fields,
            Profile,
        >(self.runtime, &fields))
    }

    /// Invokes a Gleam callable while this host call owns the active runtime.
    ///
    /// A provider-state borrow must end before re-entry.
    ///
    /// ```compile_fail
    /// use geam::{
    ///     HostCall, HostCallCompletion, HostCallError, HostCallable, HostProfile, HostProvider,
    ///     HostTypeList, HostTypeListEnd,
    /// };
    /// use num_bigint::BigInt;
    ///
    /// struct Profile;
    /// struct Provider;
    ///
    /// impl HostProfile for Profile {
    ///     type RunState = usize;
    /// }
    ///
    /// impl HostProvider<Profile> for Provider {
    ///     type State = usize;
    ///
    ///     fn project(state: &mut usize) -> &mut Self::State {
    ///         state
    ///     }
    /// }
    ///
    /// type Arguments = HostTypeList<BigInt, HostTypeListEnd>;
    ///
    /// fn reenter_with_live_state<'call>(
    ///     mut call: HostCall<'call, Profile, Provider, BigInt>,
    ///     callable: HostCallable<'call, Arguments, BigInt>,
    /// ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
    ///     let state = call.state();
    ///     let returned = call.invoke(callable, (BigInt::from(1), ()))?;
    ///     *state += 1;
    ///     Ok(call.return_value(returned))
    /// }
    /// ```
    pub fn invoke<Arguments, FunctionReturn>(
        &mut self,
        function: crate::host::HostCallable<'call, Arguments, FunctionReturn>,
        arguments: Arguments::Values<'call>,
    ) -> Result<FunctionReturn::Value<'call>, crate::HostCallError>
    where
        Arguments: HostTypeSequence,
        FunctionReturn: HostType,
    {
        let mut values = Vec::new();
        crate::host::type_::into_scoped_values::<Arguments>(arguments, &mut values);
        let returned = self
            .runtime
            .invoke(function.token, values.into_boxed_slice())?;
        Ok(crate::host::type_::from_token::<FunctionReturn, Profile>(
            self.runtime,
            returned,
        ))
    }
}

impl<'call, Profile, Provider, Item> HostCall<'call, Profile, Provider, HostListType<Item>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Item: HostType,
{
    pub fn return_list(
        self,
        values: impl IntoIterator<Item = Item::Value<'call>>,
    ) -> HostCallCompletion<'call, HostListType<Item>> {
        let values = values
            .into_iter()
            .map(crate::host::type_::into_scoped::<Item>)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        HostCallCompletion::new(self.runtime.build_list(values))
    }
}

impl<'call, Profile, Provider, Elements> HostCall<'call, Profile, Provider, HostTupleType<Elements>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Elements: HostTypeSequence,
{
    pub fn return_tuple(
        self,
        values: <Elements as crate::host::HostTypeSequence>::Values<'call>,
    ) -> HostCallCompletion<'call, HostTupleType<Elements>> {
        let mut output = Vec::new();
        crate::host::type_::into_scoped_values::<Elements>(values, &mut output);
        HostCallCompletion::new(self.runtime.build_tuple(output.into_boxed_slice()))
    }
}

impl<'call, Profile, Provider, Schema, Arguments>
    HostCall<'call, Profile, Provider, HostCustomType<Schema, Arguments>>
where
    Profile: HostProfile,
    Provider: HostProvider<Profile>,
    Schema: crate::host::HostCustomSchema,
    Arguments: HostTypeSequence,
{
    pub fn return_custom<Constructor>(
        self,
        fields: <Constructor::Fields as crate::host::HostTypeSequence>::Values<'call>,
    ) -> HostCallCompletion<'call, HostCustomType<Schema, Arguments>>
    where
        Constructor: HostCustomConstructor<Custom = HostCustomType<Schema, Arguments>>,
        Constructor::Fields: HostTypeSequence,
    {
        let mut output = Vec::new();
        crate::host::type_::into_scoped_values::<Constructor::Fields>(fields, &mut output);
        HostCallCompletion::new(self.runtime.build_custom(
            crate::host::type_::custom_constructor_index::<Constructor>(),
            output.into_boxed_slice(),
        ))
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::{HostCall, HostCallRuntime, HostProfile, HostProvider, StatelessHostProfile};
    use crate::host::{
        HostCallArguments, HostCallCompletion, HostCallError, HostCustomArgumentSlot,
        HostCustomToken, HostFunctionArgumentSlot, HostFunctionToken, HostListArgumentSlot,
        HostListToken, HostScopedValue, HostTupleArgumentSlot, HostTupleToken, HostTypeParameter,
        HostValue, HostValueArgumentSlot, HostValueFamily, HostValueToken,
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

#[cfg(test)]
mod tests {
    use super::{HostCall, HostProvider};
    use crate::BitArrayValue;
    use crate::host::function::CallArguments;
    use crate::host::test::{
        StatelessTestProvider, TestHostCallRuntime, TestHostProfile, TestRunState,
    };
    use crate::host::{
        HostCallable, HostCustom, HostCustomConstructorAt, HostCustomConstructorDefinition,
        HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomConstructorSchema,
        HostCustomFieldListEnd, HostCustomFieldSchema, HostCustomIndex0, HostCustomIndexNext,
        HostCustomSchema, HostCustomToken, HostCustomType, HostCustomTypeSchema, HostFunctionToken,
        HostFunctionType, HostList, HostListToken, HostListType, HostScopedValue, HostTuple,
        HostTupleToken, HostTupleType, HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue,
        HostValueFamily, HostValueToken, StatelessHostProfile,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    struct Counter;

    impl HostProvider<TestHostProfile> for Counter {
        type State = usize;

        fn project(state: &mut TestRunState) -> &mut Self::State {
            &mut state.counter
        }
    }

    struct MarkerSchema;

    struct MarkerConstructor;

    impl HostCustomConstructorDefinition for MarkerConstructor {
        const NAME: &'static str = "Marker";

        type Fields = HostCustomFieldListEnd;
    }

    struct OtherConstructor;

    impl HostCustomConstructorDefinition for OtherConstructor {
        const NAME: &'static str = "Other";

        type Fields = HostCustomFieldListEnd;
    }

    impl HostCustomSchema for MarkerSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/marker";
        const NAME: &'static str = "Marker";
        const PARAMETER_COUNT: usize = 0;

        type Constructors = HostCustomConstructorList<
            MarkerConstructor,
            HostCustomConstructorList<OtherConstructor, HostCustomConstructorListEnd>,
        >;
    }

    type MarkerType = HostCustomType<MarkerSchema>;
    type Marker = HostCustomConstructorAt<MarkerType, HostCustomIndex0, MarkerConstructor>;
    type Other = HostCustomConstructorAt<
        MarkerType,
        HostCustomIndexNext<HostCustomIndex0>,
        OtherConstructor,
    >;

    #[test]
    fn host_call_exposes_only_the_selected_provider_state() {
        let mut state = TestRunState {
            counter: 1,
            unrelated: true,
        };
        let arguments = crate::host::function::CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        *HostCall::<TestHostProfile, Counter, bool>::new(&mut runtime).state() += 1;

        assert_eq!(state.counter, 2);
        assert!(state.unrelated);
    }

    #[test]
    fn stateless_provider_projects_the_complete_run_state() {
        let mut state = ();

        let projected =
            <StatelessTestProvider as HostProvider<StatelessHostProfile>>::project(&mut state);

        assert_eq!(*projected, ());
    }

    #[test]
    fn host_call_reads_and_compares_call_scoped_values() {
        type EmptyTuple = HostTupleType<HostTypeListEnd>;

        assert_eq!(
            HostCustomTypeSchema::of::<MarkerSchema>(),
            HostCustomTypeSchema::new(
                "domain",
                "domain/marker",
                "Marker",
                0,
                [
                    HostCustomConstructorSchema::new("Marker", Vec::<HostCustomFieldSchema>::new(),),
                    HostCustomConstructorSchema::new("Other", Vec::<HostCustomFieldSchema>::new(),),
                ],
            ),
        );
        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        let mut call = HostCall::<TestHostProfile, Counter, bool>::new(&mut runtime);
        let list = HostList::<BigInt>::new(HostListToken::Stored(0));
        let tuple = HostTuple::<HostTypeListEnd>::new(HostTupleToken(0));
        let custom = HostCustom::<MarkerType>::new(HostCustomToken(0));

        assert_eq!(call.list_len(list), 0);
        assert_eq!(call.list_item(list, 0), None);
        assert_eq!(call.tuple_len(tuple), 0);
        assert_eq!(call.tuple_values::<HostTypeListEnd>(tuple), ());
        assert_eq!(call.custom_constructor(custom), 0);
        assert_eq!(call.custom_fields::<Marker>(custom), Some(()),);
        assert_eq!(call.custom_fields::<Other>(custom), None,);
        assert!(!call.equal::<BigInt>(1.into(), 1.into()));
        assert!(!call.equal::<HostListType<BigInt>>(list, list));
        assert!(!call.equal::<EmptyTuple>(tuple, tuple));
        assert!(!call.equal::<MarkerType>(custom, custom));
    }

    #[test]
    fn host_call_completes_every_scalar_and_scoped_handle_family() {
        type Parameter = HostTypeParameter<0>;
        type List = HostListType<BigInt>;
        type Tuple = HostTupleType<HostTypeListEnd>;

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        let parameter = HostValue::<Parameter>::new(HostValueToken {
            family: HostValueFamily::Bool,
            index: 4,
        });
        let list = HostList::<BigInt>::new(HostListToken::Stored(0));
        let tuple = HostTuple::<HostTypeListEnd>::new(HostTupleToken(0));
        let custom = HostCustom::<MarkerType>::new(HostCustomToken(0));

        let tokens = [
            HostCall::<TestHostProfile, Counter, BigInt>::new(&mut runtime)
                .return_value(1.into())
                .token,
            HostCall::<TestHostProfile, Counter, f64>::new(&mut runtime)
                .return_value(1.5)
                .token,
            HostCall::<TestHostProfile, Counter, EcoString>::new(&mut runtime)
                .return_value("text".into())
                .token,
            HostCall::<TestHostProfile, Counter, BitArrayValue>::new(&mut runtime)
                .return_value(BitArrayValue::from_bytes(vec![1]))
                .token,
            HostCall::<TestHostProfile, Counter, char>::new(&mut runtime)
                .return_value('A')
                .token,
            HostCall::<TestHostProfile, Counter, bool>::new(&mut runtime)
                .return_value(true)
                .token,
            HostCall::<TestHostProfile, Counter, ()>::new(&mut runtime)
                .return_value(())
                .token,
            HostCall::<TestHostProfile, Counter, Parameter>::new(&mut runtime)
                .return_value(parameter)
                .token,
            HostCall::<TestHostProfile, Counter, List>::new(&mut runtime)
                .return_value(list)
                .token,
            HostCall::<TestHostProfile, Counter, Tuple>::new(&mut runtime)
                .return_value(tuple)
                .token,
            HostCall::<TestHostProfile, Counter, MarkerType>::new(&mut runtime)
                .return_value(custom)
                .token,
        ];

        assert_eq!(
            tokens.map(|token| token.family),
            [
                HostValueFamily::Int,
                HostValueFamily::Float,
                HostValueFamily::String,
                HostValueFamily::BitArray,
                HostValueFamily::UtfCodepoint,
                HostValueFamily::Bool,
                HostValueFamily::Nil,
                HostValueFamily::Bool,
                HostValueFamily::List,
                HostValueFamily::Tuple,
                HostValueFamily::Custom,
            ],
        );
        assert_eq!(tokens[7].index, 4);
    }

    #[test]
    fn host_call_builds_typed_compound_returns() {
        type List = HostListType<BigInt>;
        type Tuple = HostTupleType<HostTypeListEnd>;

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);

        let list = HostCall::<TestHostProfile, Counter, List>::new(&mut runtime)
            .return_list([BigInt::from(1), BigInt::from(2)])
            .token;
        let tuple = HostCall::<TestHostProfile, Counter, Tuple>::new(&mut runtime)
            .return_tuple(())
            .token;
        let custom = HostCall::<TestHostProfile, Counter, MarkerType>::new(&mut runtime)
            .return_custom::<Marker>(())
            .token;

        assert_eq!(list.family, HostValueFamily::List);
        assert_eq!(tuple.family, HostValueFamily::Tuple);
        assert_eq!(custom.family, HostValueFamily::Custom);
    }

    #[test]
    fn host_call_invokes_and_completes_typed_function_handles() {
        type Arguments = HostTypeList<BigInt, HostTypeListEnd>;
        type Function = HostFunctionType<Arguments, BigInt>;

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        let callable = HostCallable::<Arguments, BigInt>::new(HostFunctionToken(0));
        let returned = HostCall::<TestHostProfile, Counter, BigInt>::new(&mut runtime)
            .invoke(callable, (BigInt::from(7), ()))
            .expect("test runtime should return the first callback argument");

        assert_eq!(returned, BigInt::from(0));
        assert_eq!(
            runtime.completed(),
            Some(&HostScopedValue::Int(BigInt::from(7))),
        );

        let completion = HostCall::<TestHostProfile, Counter, Function>::new(&mut runtime)
            .return_value(callable)
            .token;
        assert_eq!(completion.family, HostValueFamily::Function);
        assert_eq!(
            runtime.completed(),
            Some(&HostScopedValue::Function(HostFunctionToken(0))),
        );

        let empty = HostCallable::<HostTypeListEnd, ()>::new(HostFunctionToken(1));
        HostCall::<TestHostProfile, Counter, ()>::new(&mut runtime)
            .invoke(empty, ())
            .expect("zero-argument test callback should return Nil");
    }
}
