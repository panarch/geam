use geam_core::provider::advanced::{Equality, Hashing, Inspection, RetainedExternalPayload};
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponentRegistration,
    HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
    compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[geam_macros::provider(package = "local_retained", modules = [native], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "local_retained", crate_path = geam_core)]
mod native {
    use super::{BigInt, Equality, Hashing, Inspection, RetainedExternalPayload};
    use std::rc::Rc;

    pub struct Payload(Rc<BigInt>);

    impl RetainedExternalPayload for Payload {
        fn source_equal(&self, _: &Equality<'_>, other: &Self) -> bool {
            self.0 == other.0
        }
        fn source_hash(&self, _: &Hashing<'_>) -> u64 {
            geam_core::__macro_support::external_payload_hash(&self.0)
        }
        fn inspect(&self, _: &Inspection<'_>) -> ecow::EcoString {
            self.0.to_string().into()
        }
    }

    #[geam_macros::external(
        name = "Box", parameters = [Item], input = BoxInput, payload = Payload, manual,
    )]
    struct BoxValue<Item>;

    #[geam_macros::function]
    fn new(value: BigInt) -> BoxValue<BigInt> {
        BoxValue::from_payload(Payload(Rc::new(value)))
    }

    #[geam_macros::function]
    fn read(value: BoxInput<BigInt>) -> BigInt {
        (*value.payload().0).clone()
    }
}

struct Profile;
impl HostProfile for Profile {
    type RunState = ();
    type ExternalStores = Stores;
}
impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Stores) -> &Stores {
        stores
    }
    fn component_state(state: &mut ()) -> &mut () {
        state
    }
}

#[test]
fn a_local_manual_payload_does_not_acquire_transfer_or_send_requirements() {
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("local-only provider registration");
    let program = compile_typed_host_program(
        "local_retained",
        "local_retained",
        [PackageSource::new(
            "local_retained",
            Vec::<String>::new(),
            [ModuleSource::new(
                "local_retained",
                "src/local_retained.gleam",
                r#"
pub type Box(item)
@external(erlang, "native", "new")
fn new(value: Int) -> Box(Int)
@external(erlang, "native", "read")
fn read(value: Box(Int)) -> Int
pub fn main() { read(new(42)) }
"#,
            )],
        )],
        HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers)
            .expect("static local composition"),
    )
    .expect("source should compile");
    let execution =
        HostedExecution::try_from_module_plan(plan_host_program(program).expect("plan"))
            .expect("seal");
    assert_eq!(
        execution.run_main(&mut (), &mut Vec::new()),
        Ok(Value::Int(42.into()))
    );
}
