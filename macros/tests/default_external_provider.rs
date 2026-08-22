use ecow::EcoString;
use geam_core::provider::{Configuration, ExternalPayload};
use geam_core::{
    HostComponentProfile, HostModule, HostProfile, HostProviderComponent,
    HostProviderComponentInitialization, HostProviderComponentRegistration, HostProviderSet,
    HostedExecution, ModuleSource, PackageSource, Value, ValueType, compile_typed_host_program,
    plan_host_program,
};

#[geam_macros::provider(
    package = "tags",
    modules = [tags],
    crate_path = geam_core,
)]
pub struct Component;

#[geam_macros::module(path = "tags", crate_path = geam_core)]
mod tags {
    use super::EcoString;

    #[geam_macros::external(name = "Tag")]
    #[derive(Clone, PartialEq, Eq, Hash)]
    pub(super) struct Tag(EcoString);

    #[geam_macros::function]
    pub(super) fn new(value: EcoString) -> Tag {
        Tag(value)
    }

    #[geam_macros::function]
    fn append(tag: &Tag, suffix: EcoString) -> Tag {
        Tag(format!("{}{suffix}", tag.0).into())
    }
}

struct Profile;

#[derive(Default)]
struct ProfileStores {
    component: <Component as HostProviderComponent>::Stores,
}

struct ProfileState {
    component: <Component as HostProviderComponent>::RunState,
}

impl HostProfile for Profile {
    type RunState = ProfileState;
    type ExternalStores = ProfileStores;
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(
        stores: &Self::ExternalStores,
    ) -> &<Component as HostProviderComponent>::Stores {
        &stores.component
    }

    fn component_state(
        state: &mut Self::RunState,
    ) -> &mut <Component as HostProviderComponent>::RunState {
        &mut state.component
    }
}

const TAG_SOURCE: &str = r#"
@external(erlang, "macro_tags", "Tag")
pub type Tag

@external(erlang, "macro_tags", "new")
pub fn new(value: String) -> Tag

@external(erlang, "macro_tags", "append")
pub fn append(tag: Tag, suffix: String) -> Tag

pub fn main() {
  let original = new("alpha")
  let updated = append(original, "-beta")

  assert original == new("alpha")
  assert original != updated
  assert updated == new("alpha-beta")

  updated
}
"#;

fn providers() -> Vec<geam_core::HostProviderModule<Profile>> {
    <Component as HostProviderComponentRegistration<Profile>>::providers()
        .expect("macro-authored default external component should register")
}

fn execution() -> HostedExecution<Profile> {
    let hosts = HostProviderSet::with_providers(Vec::<HostModule<Profile>>::new(), providers())
        .expect("macro-authored module should be unique");
    let typed = compile_typed_host_program(
        "tags",
        "tags",
        [PackageSource::new(
            "tags",
            Vec::<&str>::new(),
            [ModuleSource::new("tags", "src/tags.gleam", TAG_SOURCE)],
        )],
        hosts,
    )
    .expect("complete default external provider source should compile");
    let plan = plan_host_program(typed).expect("matching default external provider should plan");
    HostedExecution::try_from_module_plan(plan)
        .expect("matching default external provider should seal")
}

#[test]
fn generated_default_external_schema_and_semantics_are_exact() {
    assert_eq!(Component::initialize(&Configuration::empty()), Ok(()));
    let providers = providers();
    assert_eq!(providers.len(), 1);
    let provider = &providers[0];
    assert_eq!(provider.package().as_str(), "tags");
    assert_eq!(provider.module().as_str(), "tags");
    let external_types = provider.external_types().collect::<Vec<_>>();
    assert_eq!(external_types.len(), 1);
    assert_eq!(external_types[0].package().as_str(), "tags");
    assert_eq!(external_types[0].module().as_str(), "tags");
    assert_eq!(external_types[0].name().as_str(), "Tag");
    assert_eq!(external_types[0].parameter_count(), 0);

    let functions = provider.functions().collect::<Vec<_>>();
    assert_eq!(
        functions
            .iter()
            .map(|function| function.name().as_str())
            .collect::<Vec<_>>(),
        ["new", "append"],
    );
    let tag = functions[0].type_().return_().clone();
    assert!(matches!(tag, ValueType::External(_)));
    assert_eq!(functions[0].type_().argument_types(), &[ValueType::String]);
    assert_eq!(
        functions[1].type_().argument_types(),
        &[tag.clone(), ValueType::String]
    );
    assert_eq!(functions[1].type_().return_(), &tag);

    let value = tags::new("alpha".into());
    let equal = tags::new("alpha".into());
    let different = tags::new("beta".into());
    assert!(value.source_equal(&equal));
    assert!(!value.source_equal(&different));
    assert_eq!(value.source_hash(), equal.source_hash());
    assert_eq!(value.inspect(), "Tag(<opaque>)");
}

#[test]
fn default_external_updates_and_escaped_values_remain_persistent() {
    let (first, second) = {
        let execution = execution();
        let mut first_state = ProfileState { component: () };
        let mut second_state = ProfileState { component: () };
        let first = execution
            .run_main(&mut first_state, &mut Vec::new())
            .expect("default external provider should execute");
        let second = execution
            .run_main(&mut second_state, &mut Vec::new())
            .expect("default external provider should repeat independently");
        (first, second)
    };

    let Value::External(first) = first else {
        panic!("main should return the updated default external value");
    };
    let Value::External(second) = second else {
        panic!("repeated main should return the updated default external value");
    };
    assert_eq!(first.inspection(), "Tag(<opaque>)");
    assert_eq!(second.inspection(), "Tag(<opaque>)");
    assert_ne!(first.identity(), second.identity());
}
