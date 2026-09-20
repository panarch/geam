#[path = "../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam_core::{
    HostComponentProfile, HostProfile, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderSet, HostRegistrationError,
};
use std::marker::PhantomData;

struct ComponentProfile<Component>(PhantomData<Component>);

impl<Component: HostProviderComponent> HostProfile for ComponentProfile<Component> {
    type RunState = Component::RunState;
    type ExternalStores = Component::Stores;
    type ExecutionState = ();
}

impl<Component: HostProviderComponent> HostComponentProfile<Component>
    for ComponentProfile<Component>
{
    fn component_stores(stores: &Component::Stores) -> &Component::Stores {
        stores
    }

    fn component_state(state: &mut Component::RunState) -> &mut Component::RunState {
        state
    }
}

mod raw_names {
    #[geam_macros::provider(package = "names", modules = [native], crate_path = geam_core)]
    pub struct Component;

    #[geam_macros::module(path = "names/native", crate_path = geam_core)]
    mod native {
        #[geam_macros::function]
        fn r#match(value: bool) -> bool {
            !value
        }

        #[geam_macros::function(await)]
        async fn r#move(value: bool) -> bool {
            !value
        }
    }
}

mod empty_module {
    #[geam_macros::provider(package = "names", modules = [native], crate_path = geam_core)]
    pub struct Component;

    #[geam_macros::module(path = "", crate_path = geam_core)]
    mod native {}
}

mod malformed_module {
    #[geam_macros::provider(package = "names", modules = [native], crate_path = geam_core)]
    pub struct Component;

    #[geam_macros::module(path = "names//native", crate_path = geam_core)]
    mod native {}
}

mod invalid_function {
    #[geam_macros::provider(package = "names", modules = [native], crate_path = geam_core)]
    pub struct Component;

    #[geam_macros::module(path = "names/native", crate_path = geam_core)]
    mod native {
        #[geam_macros::function]
        fn todo() -> bool {
            true
        }
    }
}

mod first_error {
    #[geam_macros::provider(package = "names", modules = [first, second], crate_path = geam_core)]
    pub struct Component;

    #[geam_macros::module(path = "names//first", crate_path = geam_core)]
    mod first {}

    #[geam_macros::module(path = "names//second", crate_path = geam_core)]
    mod second {}
}

mod later_error {
    #[geam_macros::provider(package = "names", modules = [first, second], crate_path = geam_core)]
    pub struct Component;

    #[geam_macros::module(path = "names/first", crate_path = geam_core)]
    mod first {}

    #[geam_macros::module(path = "names//second", crate_path = geam_core)]
    mod second {}
}

mod duplicate_modules {
    #[geam_macros::provider(package = "names", modules = [first, second], crate_path = geam_core)]
    pub struct Component;

    #[geam_macros::module(path = "names/shared", crate_path = geam_core)]
    mod first {}

    #[geam_macros::module(path = "names/shared", crate_path = geam_core)]
    mod second {}
}

#[test]
fn raw_rust_function_identifiers_register_and_link_by_their_gleam_names() {
    use geam_core::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use geam_core::{ModuleSource, PackageSource, compile_typed_host_program};
    use raw_names::Component;

    let providers =
        <Component as HostProviderComponentRegistration<ComponentProfile<Component>>>::providers()
            .unwrap();
    assert_eq!(
        providers[0]
            .functions()
            .map(|function| function.name().as_str())
            .collect::<Vec<_>>(),
        ["match", "move"]
    );
    let source = r#"
@external(erlang, "names", "match")
pub fn match(value: Bool) -> Bool
@external(erlang, "names", "move")
pub fn move(value: Bool) -> Bool
pub fn main() { #(match(True), move(False)) }
"#;
    let typed = compile_typed_host_program(
        "names",
        "names/native",
        [PackageSource::new(
            "names",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "names/native",
                "src/names/native.gleam",
                source,
            )],
        )],
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let (builder, main) = HostedModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<(), (bool, bool)>::new("main"))
        .unwrap();
    let mut module = builder.seal().unwrap();
    let host = execution_fixture::TestHost::default();
    let result = host
        .block_on(
            module.with_execution(&host, &mut (), &mut Vec::new(), async |scope| {
                scope.call(&main, ()).await
            }),
        )
        .unwrap()
        .unwrap();
    assert_eq!(result, (false, true));
}

#[test]
fn generated_registration_preserves_core_name_errors_and_declaration_order() {
    assert_eq!(
        <empty_module::Component as HostProviderComponentRegistration<
            ComponentProfile<empty_module::Component>,
        >>::providers()
        .err(),
        Some(HostRegistrationError::InvalidModuleName { module: "".into() })
    );
    assert_eq!(
        <malformed_module::Component as HostProviderComponentRegistration<
            ComponentProfile<malformed_module::Component>,
        >>::providers()
        .err(),
        Some(HostRegistrationError::InvalidModuleName {
            module: "names//native".into()
        })
    );
    assert_eq!(
        <invalid_function::Component as HostProviderComponentRegistration<
            ComponentProfile<invalid_function::Component>,
        >>::providers()
        .err(),
        Some(HostRegistrationError::InvalidFunctionName {
            module: "names/native".into(),
            function: "todo".into(),
        })
    );
    assert_eq!(
        <first_error::Component as HostProviderComponentRegistration<
            ComponentProfile<first_error::Component>,
        >>::providers()
        .err(),
        Some(HostRegistrationError::InvalidModuleName {
            module: "names//first".into()
        })
    );
    assert_eq!(
        <later_error::Component as HostProviderComponentRegistration<
            ComponentProfile<later_error::Component>,
        >>::providers()
        .err(),
        Some(HostRegistrationError::InvalidModuleName {
            module: "names//second".into()
        })
    );
}

#[test]
fn duplicate_module_identity_is_rejected_at_provider_set_composition() {
    use duplicate_modules::Component;
    let providers =
        <Component as HostProviderComponentRegistration<ComponentProfile<Component>>>::providers()
            .unwrap();
    assert_eq!(providers.len(), 2);
    assert_eq!(
        HostProviderSet::from_providers(providers).err(),
        Some(HostRegistrationError::DuplicateModule {
            module: "names/shared".into(),
            first_package: "names".into(),
            second_package: "names".into(),
        })
    );
}
