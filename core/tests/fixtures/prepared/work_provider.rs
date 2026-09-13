use super::work_fixture::{WorkComponent, WorkType};
use geam_core::embedding::{
    BigInt, FunctionDeclaration, HostedModuleBuilder, List, PreparedHostedModule,
};
use geam_core::host::{
    HostComponentProfile, HostFutureStore, HostProfile, HostProviderSet, HostWorkProfile,
};
use geam_core::{ModuleSource, PackageSource};

pub struct Profile;

impl HostProfile for Profile {
    type RunState = ();
    type ExternalStores = HostFutureStore;
    type ExecutionState = ();
}

impl HostWorkProfile for Profile {
    type Work = WorkComponent;
}

impl HostComponentProfile<WorkComponent> for Profile {
    fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
        stores
    }
    fn component_state(state: &mut ()) -> &mut () {
        state
    }
}

pub fn hosts() -> HostProviderSet<Profile> {
    HostProviderSet::from_providers(WorkComponent::providers().unwrap()).unwrap()
}

pub fn prepare() -> PreparedHostedModule {
    let program = geam_core::compile_typed_host_program(
        "app",
        "app",
        [
            PackageSource::new(
                "work_fixture",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "fixture/work",
                    "src/fixture/work.gleam",
                    WorkComponent::SOURCE,
                )],
            ),
            PackageSource::new(
                "app",
                ["work_fixture"],
                [ModuleSource::new(
                    "app",
                    "src/app.gleam",
                    include_str!("work.gleam"),
                )],
            ),
        ],
        hosts(),
    )
    .unwrap();
    let (mut bindings, _) = HostedModuleBuilder::new(program)
        .unwrap()
        .function(FunctionDeclaration::<(BigInt,), WorkType<BigInt>>::new(
            "make",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BigInt,), WorkType<List<BigInt>>>::new("collect"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(WorkType<BigInt>,), WorkType<BigInt>>::new("keep"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(), WorkType<BigInt>>::new("failure"))
        .unwrap();
    bindings.prepare().unwrap()
}
