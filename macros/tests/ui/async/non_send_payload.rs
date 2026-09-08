use ecow::EcoString;
use geam_core::provider::{BigInt, ExternalPayload};
use geam_core::{
    AsyncHostComponentProfile, AsyncHostProviderComponent, HostProfile, HostProviderComponent,
};
use geam_core::host::TransferHostProviderComponentRegistration;
use std::rc::Rc;

#[geam_macros::provider(
    package = "non_send_payload",
    modules = [native],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "non_send_payload/native", crate_path = geam_core)]
mod native {
    use super::{BigInt, EcoString, ExternalPayload, Rc};

    #[geam_macros::external(name = "Value", manual)]
    struct Value(Rc<BigInt>);

    impl ExternalPayload for Value {
        fn source_equal(&self, other: &Self) -> bool {
            self.0 == other.0
        }

        fn source_hash(&self) -> u64 {
            0
        }

        fn inspect(&self) -> EcoString {
            "Value".into()
        }
    }

    #[geam_macros::function]
    fn identity(value: &Value) -> BigInt {
        value.0.as_ref().clone()
    }
}

struct Profile;

impl HostProfile for Profile {
    type RunState = ();
    type ExternalStores = <Component as AsyncHostProviderComponent>::AsyncStores;
}

impl AsyncHostComponentProfile<Component> for Profile {
    fn component_async_stores(
        stores: &Self::ExternalStores,
    ) -> &<Component as AsyncHostProviderComponent>::AsyncStores {
        stores
    }

    fn component_state(
        state: &mut Self::RunState,
    ) -> &mut <Component as HostProviderComponent>::RunState {
        state
    }
}

fn main() {
    let _ = <Component as TransferHostProviderComponentRegistration<Profile>>::providers();
}
