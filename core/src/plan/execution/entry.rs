use super::function::{ExternalFunctionId, ProfiledRuntimeFunctionId};
use super::{HostSpecializationError, HostedExecution};
use crate::host::{HostExternalSchema, HostWorkProfile, HostWorkSchema};
use crate::plan::HostedModulePlan;

/// A sealed application entry that completes an outer work result, if present.
///
/// The Rust host supplies the executor, state, and output capabilities. Ordinary
/// results are discarded without searching their contents for work.
pub struct HostedEntry<Profile: HostWorkProfile> {
    pub(crate) execution: HostedExecution<Profile>,
    pub(crate) completion: EntryCompletion,
}

#[derive(Clone, Copy)]
pub(crate) enum EntryCompletion {
    Immediate,
    Work(ExternalFunctionId),
}

impl<Profile: HostWorkProfile> HostedEntry<Profile> {
    /// Seals the source entry and selects its completion contract from its type.
    pub fn try_from_module_plan(
        plan: HostedModulePlan<Profile>,
    ) -> Result<Self, HostSpecializationError> {
        let execution = HostedExecution::try_from_module_plan(plan)?;
        let common = &execution.execution.program.common;
        let completion = match common.main {
            ProfiledRuntimeFunctionId::External(function) => {
                let type_ = common.external_types.value_type(function.return_type());
                let work = crate::ExternalTypeName::new(
                    HostWorkSchema::<Profile>::PACKAGE.into(),
                    HostWorkSchema::<Profile>::MODULE.into(),
                    HostWorkSchema::<Profile>::NAME.into(),
                );
                if type_.type_name() == &work {
                    EntryCompletion::Work(function)
                } else {
                    EntryCompletion::Immediate
                }
            }
            ProfiledRuntimeFunctionId::Core(_) => EntryCompletion::Immediate,
        };
        Ok(Self {
            execution,
            completion,
        })
    }

    /// Executes main once, then completes only its returned outer work.
    pub async fn run(
        &mut self,
        state: &mut Profile::RunState,
        echo: &mut dyn crate::EchoSink,
    ) -> Result<(), crate::runtime::ObservationError> {
        crate::runtime::run_hosted_entry(self, state, echo).await
    }
}

#[cfg(test)]
mod tests {
    use super::{EntryCompletion, HostedEntry};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostCallable, HostComponentProfile,
        HostConstructions, HostFunctionType, HostFutureError, HostFutureStore, HostProfile,
        HostProviderModule, HostProviderSet, HostTypeListEnd, HostTypeParameter, HostWorkProfile,
    };
    use crate::work_fixture::{WorkComponent, WorkHostType};
    use crate::{EchoOutput, EchoSink, ModuleSource, PackageSource};
    use futures_util::FutureExt;
    use num_bigint::BigInt;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = (HostFutureStore, crate::HostExternalStore<u64>);
    }
    impl HostWorkProfile for Profile {
        type Work = WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &Self::ExternalStores) -> &HostFutureStore {
            &stores.0
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }

    #[derive(Default)]
    struct Echo(Vec<String>);
    impl EchoSink for Echo {
        fn emit(&mut self, value: EchoOutput) {
            self.0.push(value.to_string());
        }
    }

    fn entry(
        source: &str,
        additional: Vec<crate::HostProviderModule<Profile>>,
    ) -> Result<HostedEntry<Profile>, super::HostSpecializationError> {
        let mut providers = WorkComponent::providers::<Profile>().expect("work registration");
        providers.extend(additional);
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["work_fixture"],
                    [ModuleSource::new("main", "src/main.gleam", source)],
                ),
                PackageSource::new(
                    "work_fixture",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "fixture/work",
                        "src/fixture/work.gleam",
                        r#"
pub type Work(value)
@external(erlang, "fixture", "ready")
pub fn ready(value: value) -> Work(value)
@external(erlang, "fixture", "map")
pub fn map(value: Work(a), callback: fn(a) -> b) -> Work(b)
@external(erlang, "fixture", "flatten")
pub fn flatten(value: Work(Work(a))) -> Work(a)
@external(erlang, "fixture", "all")
pub fn all(values: List(Work(a))) -> Work(List(a))
"#,
                    )],
                ),
            ],
            HostProviderSet::from_providers(providers).expect("provider set"),
        )
        .expect("typed source");
        let plan = crate::plan_host_program(typed).expect("planned source");
        HostedEntry::try_from_module_plan(plan)
    }

    #[test]
    fn completes_the_outer_nominal_work_for_every_return_family() {
        for value in [
            "42",
            "1.5",
            "\"text\"",
            "True",
            "Nil",
            "<<1, 2>>",
            "#(1, \"two\")",
            "[1, 2]",
            "Some(42)",
            "Ok(42)",
            "Error(\"data\")",
            "fn(x: Int) { x + 1 }",
            "work.ready(42)",
        ] {
            let source = format!(
                r#"import fixture/work
pub type Option(a) {{ Some(a) None }}
pub fn main() {{
  echo "main"
  work.map(work.ready({value}), fn(value) {{
    echo "completed"
    value
  }})
}}
"#
            );
            let mut entry = entry(&source, Vec::new()).expect("sealed entry");
            assert!(returns_work(&entry));
            let mut echo = Echo::default();
            entry
                .run(&mut (), &mut echo)
                .now_or_never()
                .expect("ready composition")
                .expect("completed entry");
            assert_eq!(
                echo.0,
                [
                    "src/main.gleam:4\n\"main\"",
                    "src/main.gleam:6\n\"completed\""
                ]
            );
        }
    }

    #[test]
    fn ordinary_containers_and_returned_inner_work_are_not_driven() {
        for (outer, work) in [
            ("pending", true),
            ("[pending]", false),
            ("#(pending, 1)", false),
            ("Some(pending)", false),
            ("Ok(pending)", false),
            ("work.ready(pending)", true),
        ] {
            let source = format!(
                r#"import fixture/work
pub type Option(a) {{ Some(a) None }}
pub fn main() {{
  echo "main"
  let pending = work.map(work.ready(42), fn(value) {{
    echo "inner"
    value
  }})
  {outer}
}}
"#
            );
            let mut entry = entry(&source, Vec::new()).expect("sealed entry");
            assert_eq!(returns_work(&entry), work);
            let mut echo = Echo::default();
            entry
                .run(&mut (), &mut echo)
                .now_or_never()
                .expect("ready entry")
                .expect("entry result");
            let expected = if outer == "pending" {
                vec!["src/main.gleam:4\n\"main\"", "src/main.gleam:6\n\"inner\""]
            } else {
                vec!["src/main.gleam:4\n\"main\""]
            };
            assert_eq!(echo.0, expected);
        }
    }

    #[test]
    fn resolved_aliases_use_the_registered_nominal_schema() {
        let mut entry = entry(
            r#"import fixture/work
pub type Response = work.Work(Int)
pub fn main() -> Response { work.ready(42) }
"#,
            Vec::new(),
        )
        .expect("sealed alias");
        assert!(returns_work(&entry));
        let mut echo = Echo::default();
        entry
            .run(&mut (), &mut echo)
            .now_or_never()
            .expect("ready alias")
            .expect("result");
        assert!(echo.0.is_empty());
    }

    #[test]
    fn a_foreign_nominal_work_return_remains_ordinary_data() {
        use crate::host::{
            HostExternalBinding, HostExternalEquality, HostExternalHashing, HostExternalInspection,
            HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType,
            HostTypeList,
        };
        struct ForeignWork;
        impl HostExternalSchema for ForeignWork {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "main";
            const NAME: &'static str = "Work";
            const PARAMETER_COUNT: usize = 1;
        }
        impl HostExternalBinding<Profile, ForeignWork> for WorkComponent {
            type Storage = ForeignWork;
        }
        impl HostExternalStorage<Profile, ForeignWork> for ForeignWork {
            type Payload = u64;
            fn store(stores: &<Profile as HostProfile>::ExternalStores) -> &HostExternalStore<u64> {
                &stores.1
            }
            fn source_equal(_: &HostExternalEquality<'_>, left: &u64, right: &u64) -> bool {
                left == right
            }
            fn source_hash(_: &HostExternalHashing<'_>, value: &u64) -> u64 {
                *value
            }
            fn inspect(_: &HostExternalInspection<'_>, value: &u64) -> ecow::EcoString {
                format!("Work({value})").into()
            }
        }
        type Foreign = HostExternalType<ForeignWork, HostTypeList<BigInt, HostTypeListEnd>>;
        fn make<'call>(
            mut call: HostCall<'call, Profile, WorkComponent, Foreign>,
        ) -> Result<HostCallCompletion<'call, Foreign>, HostCallError> {
            let value = call.create_external_with_binding::<WorkComponent>(41);
            let equal = call.create_external_with_binding::<WorkComponent>(41);
            assert!(call.equal::<Foreign>(value, equal));
            assert_eq!(
                call.source_hash::<Foreign>(value),
                call.source_hash::<Foreign>(equal)
            );
            Ok(call.return_value(value))
        }
        let native = HostProviderModule::new("application", "main")
            .expect("foreign module")
            .with_external_type::<WorkComponent, ForeignWork>()
            .expect("foreign Work")
            .with_scoped_function::<WorkComponent, (), Foreign, _>("make", make)
            .expect("foreign value");
        let mut entry = entry(
            "pub type Work(a)\n@external(erlang, \"native\", \"make\")\nfn make() -> Work(Int)\npub fn main() { echo make() }",
            vec![native],
        )
        .expect("sealed ordinary entry");
        assert!(!returns_work(&entry));
        let mut echo = Echo::default();
        entry
            .run(&mut (), &mut echo)
            .now_or_never()
            .expect("ordinary entry completes")
            .expect("foreign Work is not driven");
        assert_eq!(echo.0, ["src/main.gleam:4\nWork(41)"]);
    }

    #[test]
    fn native_cancellation_is_an_entry_lifecycle_outcome() {
        fn cancel<'call>(
            mut call: HostCall<'call, Profile, WorkComponent, WorkHostType<BigInt>>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, HostCallError> {
            assert_eq!(call.state(), &());
            Ok(call.return_future(constructions, |_| {
                Box::pin(async { Err(HostFutureError::Cancelled) })
            }))
        }
        let native = HostProviderModule::new("application", "main")
            .expect("native module")
            .with_scoped_function_and_constructions::<WorkComponent, (), WorkHostType<BigInt>, HostTypeListEnd, _>(
                "cancel", cancel,
            ).expect("cancellable work");
        let mut entry = entry(
            "import fixture/work\n@external(erlang, \"native\", \"cancel\")\nfn cancel() -> work.Work(Int)\npub fn main() { cancel() }",
            vec![native],
        ).expect("sealed work entry");
        let error = entry
            .run(&mut (), &mut Vec::new())
            .now_or_never()
            .expect("cancelled operation resolves")
            .expect_err("native cancellation");
        assert_eq!(format!("{error:?}"), "Cancelled");
        assert!(execution_failure(&error).is_none());
    }

    #[test]
    fn sealing_rejects_an_unrepresentable_native_return_before_running_main() {
        fn produce<'call>(
            _call: HostCall<'call, Profile, WorkComponent, HostTypeParameter<0>>,
        ) -> Result<HostCallCompletion<'call, HostTypeParameter<0>>, HostCallError> {
            Err(crate::HostFailure::new("native producer failed").into())
        }
        for (body, sealed) in [("let _ = produce 42", false), ("produce() + 1", true)] {
            let native = HostProviderModule::new("application", "main")
                .expect("native module")
                .with_scoped_function::<WorkComponent, (), HostTypeParameter<0>, _>(
                    "produce", produce,
                )
                .expect("generic producer");
            let source = format!(
                "@external(erlang, \"native\", \"produce\")\nfn produce() -> value\npub fn main() {{ {body} }}"
            );
            match entry(&source, vec![native]) {
                Ok(mut entry) => {
                    assert!(sealed);
                    let error = entry
                        .run(&mut (), &mut Vec::new())
                        .now_or_never()
                        .expect("immediate native failure")
                        .expect_err("producer failure");
                    assert_eq!(
                        error.to_string(),
                        "host function application::main.produce failed: native producer failed"
                    );
                }
                Err(error) => {
                    assert!(!sealed);
                    assert_eq!(error.function(), "produce");
                    assert_eq!(
                        error.reason(),
                        &crate::HostSpecializationErrorReason::UndeterminedReturnStorage
                    );
                }
            }
        }
    }

    #[test]
    fn native_work_failure_keeps_the_host_callback_caller() {
        fn fail<'call>(
            call: HostCall<'call, Profile, WorkComponent, WorkHostType<BigInt>>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, HostCallError> {
            Ok(call.return_future(constructions, |_| {
                Box::pin(async {
                    Ok(crate::host::HostFutureCompletion::new(|_, _| {
                        Err(crate::HostFailure::new("native failed").into())
                    }))
                })
            }))
        }
        fn bridge<'call>(
            mut call: HostCall<'call, Profile, WorkComponent, WorkHostType<BigInt>>,
            callback: HostCallable<'call, HostTypeListEnd, WorkHostType<BigInt>>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, HostCallError> {
            let work = call.invoke(callback, ())?;
            Ok(call.return_value(work))
        }
        for (body, native_failure) in [
            ("bridge(fail)", true),
            ("bridge(fn() { panic as \"construction\" })", false),
        ] {
            let native = HostProviderModule::new("application", "main").expect("native module")
            .with_scoped_function_and_constructions::<WorkComponent, (), WorkHostType<BigInt>, HostTypeListEnd, _>("fail", fail)
            .expect("failing work")
            .with_scoped_function::<WorkComponent, (HostFunctionType<HostTypeListEnd, WorkHostType<BigInt>>,), WorkHostType<BigInt>, _>("bridge", bridge)
            .expect("native callback");
            let source = format!(
                r#"import fixture/work
@external(erlang, "native", "fail")
fn fail() -> work.Work(Int)
@external(erlang, "native", "bridge")
fn bridge(callback: fn() -> work.Work(Int)) -> work.Work(Int)
pub fn main() {{ {body} }}
"#
            );
            let mut entry = entry(&source, vec![native]).expect("sealed native callback");
            let error = entry
                .run(&mut (), &mut Vec::new())
                .now_or_never()
                .expect("native work completes")
                .expect_err("native work failure");
            execution_failure(&error).expect("provider execution failure").read(|error| {
            if !native_failure {
                assert_eq!(error.to_string(), "panic: construction");
                return;
            }
            assert_eq!(error.to_string(), "host function application::main.fail failed: native failed");
            assert!(matches!(error, crate::ExecutionError::Host(host)
                if host.location().caller().map(|caller| (caller.package().as_str(), caller.module().as_str(), caller.function().as_str()))
                    == Some(("application", "main", "bridge"))));
        });
        }
    }

    #[test]
    fn construction_and_completion_keep_source_failure_origins() {
        for (body, label) in [
            ("panic as \"original\"", "panic in main.main"),
            (
                "work.map(work.ready(42), fn(_) { panic as \"original\" })",
                "panic in main.<anonymous:0>",
            ),
        ] {
            let source =
                format!("import fixture/work\npub fn main() -> work.Work(Int) {{\n  {body}\n}}\n");
            let mut entry = entry(&source, Vec::new()).expect("sealed entry");
            let mut echo = Echo::default();
            let error = entry
                .run(&mut (), &mut echo)
                .now_or_never()
                .expect("source failure completes")
                .expect_err("source panic");
            let execution = execution_failure(&error).expect("execution failure");
            execution.read(|error| {
                use miette::Diagnostic;
                assert_eq!(error.to_string(), "panic: original");
                assert_eq!(error.code().expect("panic code").to_string(), "geam::panic");
                let labels = error.labels().expect("source location").collect::<Vec<_>>();
                assert_eq!(labels.len(), 1);
                assert_eq!(
                    labels[0].offset(),
                    source.find("panic").expect("source expression")
                );
                assert_eq!(labels[0].len(), "panic as \"original\"".len());
                assert_eq!(labels[0].label(), Some(label));
            });
            assert!(echo.0.is_empty());
        }
    }

    fn returns_work(entry: &HostedEntry<Profile>) -> bool {
        matches!(entry.completion, EntryCompletion::Work(_))
    }

    fn execution_failure(
        error: &crate::runtime::ObservationError,
    ) -> Option<&crate::runtime::SharedExecutionError> {
        match error {
            crate::runtime::ObservationError::Execution(error) => Some(error),
            crate::runtime::ObservationError::Cancelled => None,
        }
    }
}
