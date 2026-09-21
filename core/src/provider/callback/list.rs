use super::{
    ArgumentCodec, Callback, ProviderCallbackCodec, ProviderCallbackContext, ReturnCodec,
    decode_return, encode_arguments,
};
use crate::host::{
    CallableRetention, HostCall, HostProfile, HostProvider, HostType, HostTypeSequence,
};
use crate::provider::{ProviderConstructions, ProviderListItemDecoder, ProviderListItemValue};
use std::marker::PhantomData;

/// Decoder selected by a static callback codec, with normalized stored views.
#[doc(hidden)]
pub type ProviderOwnedCallbackListDecoder<Profile, Provider, Codec> = ProviderCallbackListDecoder<
    Profile,
    <Codec as ProviderCallbackCodec<Profile, Provider, ()>>::Arguments,
    <Codec as ProviderCallbackCodec<Profile, Provider, ()>>::Returned,
    <Codec as ProviderCallbackCodec<Profile, Provider, ()>>::HostArguments,
    <Codec as ProviderCallbackCodec<Profile, Provider, ()>>::HostReturn,
>;

/// Retains only a demanded function item, using the list's original static codec.
#[doc(hidden)]
pub struct ProviderCallbackListDecoder<Profile, Arguments, Returned, HostArguments, HostReturn>
where
    Profile: HostProfile,
    HostArguments: HostTypeSequence,
    HostReturn: HostType,
{
    retention: CallableRetention<Profile, super::CallbackProvider>,
    encode: ArgumentCodec<Profile, Arguments, HostArguments>,
    decode: ReturnCodec<Profile, Returned, HostReturn>,
}

impl<Profile, Arguments, Returned, HostArguments, HostReturn> Clone
    for ProviderCallbackListDecoder<Profile, Arguments, Returned, HostArguments, HostReturn>
where
    Profile: HostProfile,
    HostArguments: HostTypeSequence,
    HostReturn: HostType,
{
    fn clone(&self) -> Self {
        Self {
            retention: self.retention.clone(),
            encode: self.encode,
            decode: self.decode,
        }
    }
}

impl<Profile, Arguments, Returned, HostArguments, HostReturn>
    ProviderCallbackListDecoder<Profile, Arguments, Returned, HostArguments, HostReturn>
where
    Profile: HostProfile,
    HostArguments: HostTypeSequence,
    HostReturn: HostType,
{
    pub fn from_host<'call, Codec, Provider, Return: HostType>(
        call: &HostCall<'call, Profile, Provider, Return>,
        constructions: ProviderConstructions<'call, Codec::Requirements>,
    ) -> Self
    where
        Provider: HostProvider<Profile>,
        Codec: ProviderCallbackCodec<
                Profile,
                Provider,
                (),
                Arguments = Arguments,
                Returned = Returned,
                HostArguments = HostArguments,
                HostReturn = HostReturn,
            >,
    {
        Self::from_host_with::<Codec, Provider, Provider, Return>(call, constructions)
    }

    pub fn from_host_with<'call, Codec, Provider, CallerProvider, Return>(
        call: &HostCall<'call, Profile, CallerProvider, Return>,
        constructions: ProviderConstructions<'call, Codec::Requirements>,
    ) -> Self
    where
        Provider: HostProvider<Profile>,
        CallerProvider: HostProvider<Profile>,
        Return: HostType,
        Codec: ProviderCallbackCodec<
                Profile,
                Provider,
                (),
                Arguments = Arguments,
                Returned = Returned,
                HostArguments = HostArguments,
                HostReturn = HostReturn,
            >,
    {
        Self {
            retention: call
                .callable_retention_with::<super::CallbackProvider, _>(&constructions.host()),
            encode: encode_arguments::<Profile, Provider, Codec>,
            decode: decode_return::<Profile, Provider, Codec>,
        }
    }
}

impl<Signature, Profile, Arguments, Returned, HostArguments, HostReturn>
    ProviderListItemDecoder<Callback<Signature>>
    for ProviderCallbackListDecoder<Profile, Arguments, Returned, HostArguments, HostReturn>
where
    Profile: HostProfile,
    HostArguments: HostTypeSequence,
    HostReturn: HostType,
{
    type View = Callback<
        Signature,
        ProviderCallbackContext<Profile, Arguments, Returned, HostArguments, HostReturn>,
    >;

    fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
        Callback {
            context: ProviderCallbackContext {
                callable: self.retention.clone().bind(value.into_callable()),
                encode: self.encode,
                decode: self.decode,
            },
            signature: PhantomData,
        }
    }
}

impl<Signature, Profile, Arguments, Returned, HostArguments, HostReturn>
    crate::provider::ProviderTypedListItemDecoder<Callback<Signature>>
    for ProviderCallbackListDecoder<Profile, Arguments, Returned, HostArguments, HostReturn>
where
    Profile: HostProfile,
    HostArguments: HostTypeSequence,
    HostReturn: HostType,
{
    type Host = crate::HostFunctionType<HostArguments, HostReturn>;
}

#[cfg(test)]
mod tests {
    use super::ProviderCallbackListDecoder;
    use crate::host::test::{TestHostProfile, TestRunState};
    use crate::provider::{
        Call, Callback, List, ProviderCallbackCodec, ProviderConstructions, ProviderListContext,
        ProviderListItemDecoder, ProviderListItemValue, ProviderNoConstructions,
    };
    use crate::{
        HostCall, HostCallContinuation, HostCallError, HostConstructions, HostFunctionType,
        HostList, HostListType, HostOwnedCompletion, HostProvider, HostProviderModule,
        HostProviderSet, HostTypeList, HostTypeListEnd, HostedExecution, ModuleSource,
        PackageSource,
    };
    use num_bigint::BigInt;

    type End = HostTypeListEnd;
    type Arguments = HostTypeList<BigInt, End>;
    type Function = HostFunctionType<Arguments, BigInt>;
    struct Provider;
    impl HostProvider<TestHostProfile> for Provider {
        type State = TestRunState;
        fn project(state: &mut TestRunState) -> &mut TestRunState {
            state
        }
    }
    struct Codec;
    impl ProviderCallbackCodec<TestHostProfile, Provider, ()> for Codec {
        type HostArguments = Arguments;
        type HostReturn = BigInt;
        type Arguments = (BigInt,);
        type Returned = BigInt;
        type Requirements = ProviderNoConstructions;

        fn into_host_arguments<'call>(
            arguments: Self::Arguments,
            _: &mut HostCall<'call, TestHostProfile, Provider, ()>,
            _: &ProviderConstructions<'call, Self::Requirements>,
        ) -> Result<(BigInt, ()), HostCallError> {
            Ok((arguments.0, ()))
        }

        fn from_host_return<'call>(
            value: BigInt,
            _: &mut HostCall<'call, TestHostProfile, Provider, ()>,
            _: &ProviderConstructions<'call, Self::Requirements>,
        ) -> BigInt {
            value
        }
    }

    type LeafDecoder =
        ProviderCallbackListDecoder<TestHostProfile, (BigInt,), BigInt, Arguments, BigInt>;

    struct NestedDecoder(LeafDecoder);

    impl ProviderListItemDecoder<List<Callback<fn(BigInt) -> BigInt>>> for NestedDecoder {
        type View =
            List<Callback<fn(BigInt) -> BigInt>, ProviderListContext<Function, LeafDecoder>>;

        fn decode(&self, value: ProviderListItemValue<'_>) -> Self::View {
            value.into_typed_list(self.0.clone())
        }
    }

    fn invoke<'call>(
        mut call: HostCall<'call, TestHostProfile, Provider, BigInt>,
        constructions: HostConstructions<'call, End>,
        values: HostList<'call, HostListType<Function>>,
    ) -> Result<HostCallContinuation<'call, BigInt>, HostCallError> {
        assert_eq!(call.state().counter, 0);
        let decoder = ProviderCallbackListDecoder::from_host::<Codec, _, _>(
            &call,
            ProviderConstructions::none(),
        );
        let outer = call.provider_retained_list::<List<Callback<fn(BigInt) -> BigInt>>, _, _>(
            values,
            NestedDecoder(decoder),
        );
        assert_eq!(outer.len(), 2);
        let callbacks = outer.get(1).unwrap();
        let outer = outer.__geam_into_context();
        assert_eq!(outer.retained().item_reads(), 1);
        drop(outer);
        assert_eq!(callbacks.len(), 2);
        let callback = callbacks.get(1).unwrap();
        let original = callbacks.__geam_into_context();
        assert_eq!(original.retained().item_reads(), 1);
        drop(original);
        let alias = callback.clone();
        Ok(call.resume(constructions, move |context| {
            Box::pin(async move {
                let mut call = Call::from_execution_context(context);
                let first = call.invoke(&callback, (BigInt::from(7),)).await.unwrap();
                let second = call.invoke(&alias, (BigInt::from(8),)).await.unwrap();
                Ok(HostOwnedCompletion::new(move |call, _| {
                    Ok(call.return_value(first + second))
                }))
            })
        }))
    }

    #[test]
    fn demanded_callback_survives_nested_lists_and_creating_call_without_reading_siblings() {
        let provider = HostProviderModule::new("application", "main")
            .unwrap()
            .with_resumable_function::<Provider, (HostListType<HostListType<Function>>,), BigInt, End, _>(
                "invoke", invoke,
            )
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
@external(erlang, "native", "invoke") fn invoke(callbacks: List(List(fn(Int) -> Int))) -> Int
pub fn main() {
  let offset = 10
  invoke([[fn(value) { echo "unused outer" value }], [fn(value) { echo "unused inner" value }, fn(value) { echo value value + offset }]])
}
"#,
                )],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let mut execution =
            HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echoes = Vec::new();
        let result = crate::execution_fixture::run(
            &mut execution,
            &mut TestRunState::default(),
            &mut echoes,
        )
        .unwrap();
        assert_eq!(result, crate::Value::Int(35.into()));
        assert_eq!(
            echoes
                .iter()
                .map(|echo| echo.value().inspect().to_string())
                .collect::<Vec<_>>(),
            ["7", "8"]
        );
    }
}
