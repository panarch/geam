//! A consumer of the stdlib-owned Decoder and Erlang-owned Subject SDKs.

use geam::__macro_support::{
    ProviderConstructions, ProviderContextualValueForms, ProviderInputValue,
    ProviderRootOutputValue,
};
use geam::gleam_stdlib::service::Decoder;
use geam::host::{
    HostCall, HostCallCompletion, HostCallError, HostComponentProfile, HostCustom, HostProfile,
    HostProviderModule, HostRegistrationError,
};

#[geam::provider(package = "decoder_service_fixture", state = usize, modules = [native, manual])]
pub struct Component;

#[geam::module(path = "decoder_service_fixture/native", crate_path = geam)]
mod native {
    use geam::gleam_erlang::service as erlang_service;
    use geam::gleam_stdlib::{Dynamic, service as stdlib_service};
    use geam::provider::{Call, Restore, Value};

    #[geam::custom(input = MessageInput)]
    pub enum Message<Item> {
        Configure(stdlib_service::Decoder<Item>),
        Dispatch(Value<Item>),
    }

    #[geam::function]
    fn keep<Item>(decoder: stdlib_service::Decoder<Item>) -> stdlib_service::Decoder<Item> {
        decoder
    }

    #[geam::function]
    fn restored<Item>(
        #[geam::call] call: &mut Call<usize>,
        #[geam::restore] restore: Restore<stdlib_service::Decoder<Item>>,
        decoder: stdlib_service::Decoder<Item>,
    ) -> Option<stdlib_service::Decoder<Item>> {
        let retained = call.store_dynamic::<_, Dynamic>(decoder);
        call.restore_dynamic(&restore, &retained)
    }

    #[geam::function]
    fn last_or<Item>(
        decoders: List<stdlib_service::Decoder<Item>>,
        default: stdlib_service::Decoder<Item>,
    ) -> stdlib_service::Decoder<Item> {
        decoders
            .len()
            .checked_sub(1)
            .and_then(|index| decoders.get(index))
            .unwrap_or(default)
    }

    #[geam::function]
    fn pair<Item>(
        value: (stdlib_service::Decoder<Item>, stdlib_service::Decoder<Item>),
    ) -> (stdlib_service::Decoder<Item>, stdlib_service::Decoder<Item>) {
        value
    }

    #[geam::function]
    fn message<Item>(value: MessageInput<Item>) -> Message<Item> {
        match value {
            MessageInput::Configure(decoder) => Message::Configure(decoder.clone()),
            MessageInput::Dispatch(value) => Message::Dispatch(value),
        }
    }

    #[geam::function]
    fn subject<Item>(
        value: erlang_service::Subject<Message<Item>>,
    ) -> erlang_service::Subject<Message<Item>> {
        value
    }
}

type HostDecoder<Profile> =
    <Decoder<geam::provider::BigInt> as ProviderContextualValueForms<Profile>>::Host;
struct Manual;
impl<Profile: HostComponentProfile<Component>> geam::HostProvider<Profile> for Manual {
    type State = usize;
    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        Profile::component_state(state)
    }
}

mod manual {
    use super::{
        Component, HostComponentProfile, HostDecoder, HostProfile, HostProviderModule,
        HostRegistrationError, Manual, keep,
    };
    pub type __GeamStores = ();
    pub struct __GeamModule;
    impl<Profile: HostProfile + HostComponentProfile<Component>>
        geam::__macro_support::ProviderModuleRegistration<Profile> for __GeamModule
    {
        fn module() -> Result<HostProviderModule<Profile>, HostRegistrationError> {
            __geam_module::<Profile>()
        }
    }

    pub fn __geam_module<Profile: HostProfile + HostComponentProfile<Component>>()
    -> Result<HostProviderModule<Profile>, HostRegistrationError> {
        HostProviderModule::new("decoder_service_fixture", "decoder_service_fixture/manual")
            .expect("static provider module identity")
            .with_scoped_function::<Manual, (HostDecoder<Profile>,), HostDecoder<Profile>, _>(
                "keep",
                keep::<Profile>,
            )
    }
}

fn keep<'call, Profile: HostProfile + HostComponentProfile<Component>>(
    mut call: HostCall<'call, Profile, Manual, HostDecoder<Profile>>,
    value: HostCustom<'call, HostDecoder<Profile>>,
) -> Result<HostCallCompletion<'call, HostDecoder<Profile>>, HostCallError> {
    *call.state() += 1;
    let decoder = <Decoder<geam::provider::BigInt> as ProviderContextualValueForms<Profile>>::OwnedInput::from_host(&mut call, value);
    decoder.complete(call, &ProviderConstructions::none())
}

pub use native::Message;
