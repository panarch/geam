use super::function::{ExternalFunctionId, ProfiledRuntimeFunctionId};
use super::{HostSpecializationError, HostedExecution};
use crate::execution::{ExecutionHost, RunError};
use crate::host::{HostExternalSchema, HostWorkProfile, HostWorkSchema};
use crate::plan::HostedModulePlan;
use crate::runtime::run_hosted_entry;
use crate::{EchoSink, ExternalTypeName};

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
        Ok(Self::from_execution(execution))
    }

    /// Executes main once, then completes only its returned outer work.
    pub async fn run(
        &mut self,
        host: &dyn ExecutionHost,
        state: &mut Profile::RunState,
        echo: &mut (dyn EchoSink + Send),
    ) -> Result<(), RunError> {
        run_hosted_entry(self, host, state, echo).await
    }

    pub(super) fn from_execution(execution: HostedExecution<Profile>) -> Self {
        let common = &execution.execution.program.common;
        let completion = match common.main {
            ProfiledRuntimeFunctionId::External(function) => {
                let type_ = common.external_types.value_type(function.return_type());
                let work = ExternalTypeName::new(
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
        Self {
            execution,
            completion,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{EntryCompletion, HostSpecializationError, HostedEntry};
    use crate::execution::RunError;
    use crate::execution_fixture::TestHost;
    use crate::host::{
        HostCall, HostCallCompletion, HostCallContinuation, HostCallError, HostCallable,
        HostComponentProfile, HostConstructions, HostExecutionError, HostExternalStore,
        HostFailure, HostFunctionType, HostFutureStore, HostOwnedCompletion, HostProfile,
        HostProviderModule, HostProviderSet, HostType, HostTypeListEnd, HostTypeParameter,
        HostWorkProfile,
    };
    use crate::provider::{self, ProviderValueContext};
    use crate::work_fixture::{WorkComponent, WorkHostType};
    use crate::{
        EchoOutput, EchoSink, ExecutionError, ModuleSource, PackageSource,
        compile_typed_host_program, plan_host_program,
    };
    use num_bigint::BigInt;
    use std::future::ready;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = (HostFutureStore, HostExternalStore<u64>);
        type ExecutionState = ();
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
        additional: Vec<HostProviderModule<Profile>>,
    ) -> Result<HostedEntry<Profile>, HostSpecializationError> {
        let mut providers = WorkComponent::providers::<Profile>().expect("work registration");
        providers.extend(additional);
        let typed = compile_typed_host_program(
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
        let plan = plan_host_program(typed).expect("planned source");
        HostedEntry::try_from_module_plan(plan)
    }

    #[test]
    fn completes_the_outer_nominal_work_for_every_return_family() {
        let execution_host = TestHost::default();

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
            execution_host
                .block_on(entry.run(&execution_host, &mut (), &mut echo))
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
        let execution_host = TestHost::default();

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
            execution_host
                .block_on(entry.run(&execution_host, &mut (), &mut echo))
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
        let execution_host = TestHost::default();

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
        execution_host
            .block_on(entry.run(&execution_host, &mut (), &mut echo))
            .expect("result");
        assert!(echo.0.is_empty());
    }

    #[test]
    fn a_foreign_nominal_work_return_remains_ordinary_data() {
        let execution_host = TestHost::default();

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
        execution_host
            .block_on(entry.run(&execution_host, &mut (), &mut echo))
            .expect("foreign Work is not driven");
        assert_eq!(echo.0, ["src/main.gleam:4\nWork(41)"]);
    }

    #[test]
    fn native_cancellation_is_an_entry_lifecycle_outcome() {
        let execution_host = TestHost::default();

        fn cancel<'call>(
            mut call: HostCall<'call, Profile, WorkComponent, WorkHostType<BigInt>>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, HostCallError> {
            assert_eq!(call.state(), &());
            Ok(call.return_future(constructions, |_| {
                Box::pin(async { Err(HostExecutionError::Cancelled) })
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
        let error = execution_host
            .block_on(entry.run(&execution_host, &mut (), &mut Vec::new()))
            .expect_err("native cancellation");
        assert_eq!(format!("{error:?}"), "Cancelled");
        assert!(execution_failure(&error).is_none());
    }

    #[test]
    fn sealing_preserves_uninhabited_callback_argument_errors() {
        use crate::plan::TypeParameterId;
        use crate::{FunctionType, HostSpecializationErrorReason, ValueType};
        let error = entry(
            "import fixture/work\npub fn main() { let _ = work.map 42 }",
            Vec::new(),
        )
        .err()
        .unwrap();
        assert_eq!(error.package(), "work_fixture");
        assert_eq!(error.module(), "fixture/work");
        assert_eq!(error.function(), "map");
        assert_eq!(
            error.reason(),
            &HostSpecializationErrorReason::UninhabitedCallbackArguments {
                callback: FunctionType::new(
                    vec![ValueType::Parameter(TypeParameterId(0))],
                    ValueType::Parameter(TypeParameterId(1)),
                ),
            }
        );
    }

    #[test]
    fn generic_producers_only_fail_when_main_calls_them() {
        let execution_host = TestHost::default();

        fn produce<'call>(
            _call: HostCall<'call, Profile, WorkComponent, HostTypeParameter<0>>,
        ) -> Result<HostCallCompletion<'call, HostTypeParameter<0>>, HostCallError> {
            Err(HostFailure::new("native producer failed").into())
        }
        for (body, invoked) in [
            ("let _ = produce 42", false),
            ("let _ = produce() 42", true),
            ("produce() + 1", true),
        ] {
            let native = HostProviderModule::new("application", "main")
                .expect("native module")
                .with_scoped_function::<WorkComponent, (), HostTypeParameter<0>, _>(
                    "produce", produce,
                )
                .expect("generic producer");
            let source = format!(
                "@external(erlang, \"native\", \"produce\")\nfn produce() -> value\npub fn main() {{ {body} }}"
            );
            let mut entry = entry(&source, vec![native]).expect("generic producer should seal");
            let returned =
                execution_host.block_on(entry.run(&execution_host, &mut (), &mut Vec::new()));
            if invoked {
                assert_eq!(
                    returned.unwrap_err().to_string(),
                    "host function application::main.produce failed: native producer failed"
                );
            } else {
                returned.expect("function references do not call the native body");
            }
        }
    }

    #[test]
    fn a_resumable_native_call_can_cancel_before_main_produces_its_result() {
        fn cancel<'call, Return: HostType>(
            call: HostCall<'call, Profile, WorkComponent, Return>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
        ) -> Result<HostCallContinuation<'call, Return>, HostCallError> {
            Ok(call.resume(constructions, |_| {
                Box::pin(ready(Err(HostExecutionError::Cancelled)))
            }))
        }
        for (return_, provider) in [
            ("Int", HostProviderModule::new("application", "main").unwrap()
                .with_resumable_function::<WorkComponent, (), BigInt, HostTypeListEnd, _>("cancel", cancel::<BigInt>).unwrap()),
            ("work.Work(Int)", HostProviderModule::new("application", "main").unwrap()
                .with_resumable_function::<WorkComponent, (), WorkHostType<BigInt>, HostTypeListEnd, _>("cancel", cancel::<WorkHostType<BigInt>>).unwrap()),
        ] {
            let source = format!(r#"import fixture/work
@external(erlang, "native", "cancel")
fn cancel() -> {return_}
pub fn main() {{ echo "before" cancel() }}
"#);
            let mut entry = entry(&source, vec![provider]).unwrap();
            let host = TestHost::default();
            let mut echo = Echo::default();
            let error = host.block_on(entry.run(&host, &mut (), &mut echo)).unwrap_err();
            assert_eq!(error.to_string(), "the Gleam entry was cancelled");
            assert_eq!(echo.0, ["src/main.gleam:4\n\"before\""]);
        }
    }

    #[cfg(feature = "tokio")]
    #[test]
    fn a_shutdown_executor_rejects_entries_without_running_source_effects() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let host = crate::execution::TokioHost::new(runtime.handle().clone());
        drop(runtime);
        let driver = TestHost::default();
        for source in [
            "pub fn main() { echo \"entered\" 42 }",
            "import fixture/work pub fn main() { echo \"entered\" work.ready(42) }",
        ] {
            let mut entry = entry(source, Vec::new()).unwrap();
            let mut echo = Echo::default();
            let error = driver
                .block_on(entry.run(&host, &mut (), &mut echo))
                .unwrap_err();
            assert_eq!(
                error.to_string(),
                "the host executor cancelled an active worker"
            );
            let error = driver
                .block_on(entry.execution.run_main(&host, &mut (), &mut echo))
                .unwrap_err();
            assert_eq!(
                error.to_string(),
                "the host executor cancelled an active worker"
            );
            assert!(echo.0.is_empty());
        }
    }

    #[test]
    fn native_work_failure_keeps_the_host_callback_caller() {
        let execution_host = TestHost::default();

        fn fail<'call>(
            call: HostCall<'call, Profile, WorkComponent, WorkHostType<BigInt>>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
        ) -> Result<HostCallCompletion<'call, WorkHostType<BigInt>>, HostCallError> {
            Ok(call.return_future(constructions, |_| {
                Box::pin(async {
                    Ok(HostOwnedCompletion::new(|_, _| {
                        Err(HostFailure::new("native failed").into())
                    }))
                })
            }))
        }
        fn bridge<'call>(
            call: HostCall<'call, Profile, WorkComponent, WorkHostType<BigInt>>,
            constructions: HostConstructions<'call, HostTypeListEnd>,
            callback: HostCallable<'call, HostTypeListEnd, WorkHostType<BigInt>>,
        ) -> Result<HostCallContinuation<'call, WorkHostType<BigInt>>, HostCallError> {
            type Owned =
                provider::Value<WorkHostType<BigInt>, ProviderValueContext<WorkHostType<BigInt>>>;
            let callback = call.owned_callable(callback, &constructions);
            Ok(call.resume(constructions, move |context| {
                Box::pin(async move {
                    let work = callback
                        .invoke(
                            &context,
                            |_, _| (),
                            |call, _, value| Ok(Owned::from_host(&call, value)),
                        )
                        .await?;
                    Ok(HostOwnedCompletion::new(move |mut call, _| {
                        let value = work.into_host(&mut call);
                        Ok(call.return_value(value))
                    }))
                })
            }))
        }
        for (body, native_failure) in [
            ("bridge(fail)", true),
            ("bridge(fn() { panic as \"construction\" })", false),
        ] {
            let native = HostProviderModule::new("application", "main").expect("native module")
            .with_scoped_function_and_constructions::<WorkComponent, (), WorkHostType<BigInt>, HostTypeListEnd, _>("fail", fail)
            .expect("failing work")
            .with_resumable_function::<WorkComponent, (HostFunctionType<HostTypeListEnd, WorkHostType<BigInt>>,), WorkHostType<BigInt>, HostTypeListEnd, _>("bridge", bridge)
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
            let error = execution_host
                .block_on(entry.run(&execution_host, &mut (), &mut Vec::new()))
                .expect_err("native work failure");
            let error = execution_failure(&error).expect("provider execution failure");
            if !native_failure {
                assert_eq!(error.to_string(), "panic: construction");
                continue;
            }
            assert_eq!(
                error.to_string(),
                "host function application::main.fail failed: native failed"
            );
            assert!(matches!(error, ExecutionError::Host(host)
                if host.location().caller().map(|caller| (caller.package().as_str(), caller.module().as_str(), caller.function().as_str()))
                    == Some(("application", "main", "bridge"))));
        }
    }

    #[test]
    fn construction_and_completion_keep_source_failure_origins() {
        let execution_host = TestHost::default();

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
            let error = execution_host
                .block_on(entry.run(&execution_host, &mut (), &mut echo))
                .expect_err("source panic");
            let execution = execution_failure(&error).expect("execution failure");
            {
                let error = execution;
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
            }
            assert!(echo.0.is_empty());
        }
    }

    fn returns_work(entry: &HostedEntry<Profile>) -> bool {
        matches!(entry.completion, EntryCompletion::Work(_))
    }

    fn execution_failure(error: &RunError) -> Option<&ExecutionError> {
        match error {
            RunError::Execution(error) => Some(error),
            RunError::Cancelled | RunError::Driver(_) => None,
        }
    }
}
