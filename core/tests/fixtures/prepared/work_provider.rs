use super::work_fixture::{WorkComponent, WorkType};
use geam_core::embedding::{
    BigInt, CustomType, FunctionDeclaration, HostedModuleBuilder, List, NamedTypeSchema,
    PreparedHostedModule,
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
    let (mut bindings, _) = HostedModuleBuilder::new(program())
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
    bindings
        .function(FunctionDeclaration::<(BigInt,), Captured>::new("capture"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(Captured, BigInt), Captured>::new(
            "extend",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(Captured,), (bool, bool)>::new(
            "identities",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(Captured,), WorkType<BigInt>>::new(
            "invoke",
        ))
        .unwrap();
    bindings.prepare().unwrap()
}

pub struct CapturedSchema;

impl NamedTypeSchema for CapturedSchema {
    const PACKAGE: &'static str = "app";
    const MODULE: &'static str = "app";
    const NAME: &'static str = "Captured";
}

pub type Captured = CustomType<CapturedSchema>;

pub fn program() -> geam_core::HostedTypedProgram<Profile> {
    geam_core::compile_typed_host_program(
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
    .unwrap()
}

pub fn prepare_entry(module: &str) -> geam_core::PreparedHostedEntry {
    let program = geam_core::compile_typed_host_program(
        "app",
        module,
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
                [
                    ModuleSource::new("entry", "src/entry.gleam", include_str!("entry.gleam")),
                    ModuleSource::new(
                        "entry_work",
                        "src/entry_work.gleam",
                        include_str!("entry_work.gleam"),
                    ),
                    ModuleSource::new(
                        "entry_failure",
                        "src/entry_failure.gleam",
                        include_str!("entry_failure.gleam"),
                    ),
                ],
            ),
        ],
        hosts(),
    )
    .unwrap();
    geam_core::PreparedHostedEntry::try_from_module_plan(
        geam_core::plan_host_program(program).unwrap(),
    )
    .unwrap()
}
