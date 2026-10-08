use geam_core::{
    HostModule, HostProviderSet, HostRegistrationError, HostedExecution, ModuleSource,
    PackageSource, Value, compile_typed_host_program, plan_host_program,
};
use num_bigint::BigInt;

#[test]
fn exposes_structured_host_registration_errors_to_external_callers() {
    assert_eq!(
        HostModule::new("host_support", "").err(),
        Some(HostRegistrationError::InvalidModuleName { module: "".into() }),
    );
    assert_eq!(
        HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("Add", <BigInt as std::ops::Add>::add)
            .err(),
        Some(HostRegistrationError::InvalidFunctionName {
            module: "host/math".into(),
            function: "Add".into(),
        }),
    );
    assert_eq!(
        HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .err(),
        Some(HostRegistrationError::DuplicateFunction {
            module: "host/math".into(),
            function: "add".into(),
        }),
    );
    assert_eq!(
        HostProviderSet::new([
            HostModule::new("first", "host/math").expect("host module should be valid"),
            HostModule::new("second", "host/math").expect("host module should be valid"),
        ])
        .err(),
        Some(HostRegistrationError::DuplicateModule {
            module: "host/math".into(),
            first_package: "first".into(),
            second_package: "second".into(),
        }),
    );
}

#[test]
fn resumable_provider_registration_preserves_invalid_and_duplicate_name_errors() {
    use geam_core::{
        HostCall, HostCallContinuation, HostCallError, HostConstructions, HostOwnedCompletion,
        HostProvider, HostProviderModule, HostTypeListEnd, StatelessHostProfile,
    };

    struct Provider;
    impl HostProvider<StatelessHostProfile> for Provider {
        type State = ();
        fn project(state: &mut ()) -> &mut () {
            state
        }
    }
    fn resume<'call>(
        mut call: HostCall<'call, StatelessHostProfile, Provider, ()>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
    ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
        assert_eq!(call.state(), &mut ());
        Ok(call.resume(constructions, |_| {
            Box::pin(async {
                Ok(HostOwnedCompletion::new(
                    |call, _| Ok(call.return_value(())),
                ))
            })
        }))
    }

    for name in ["Bad", "resume"] {
        let error = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .unwrap()
            .with_resumable_function::<Provider, (), (), HostTypeListEnd, _>("resume", resume)
            .unwrap()
            .with_resumable_function::<Provider, (), (), HostTypeListEnd, _>(name, resume)
            .err();
        let expected = if name == "Bad" {
            HostRegistrationError::InvalidFunctionName {
                module: "main".into(),
                function: name.into(),
            }
        } else {
            HostRegistrationError::DuplicateFunction {
                module: "main".into(),
                function: name.into(),
            }
        };
        assert_eq!(error, Some(expected));
    }
    let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
        .unwrap()
        .with_resumable_function::<Provider, (), (), HostTypeListEnd, _>("resume", resume)
        .unwrap();
    let typed = compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<String>::new(),
            [ModuleSource::new(
                "main",
                "main.gleam",
                "@external(erlang, \"host\", \"resume\") fn resume() -> Nil\npub fn main() { resume() }",
            )],
        )],
        HostProviderSet::from_providers([provider]).unwrap(),
    )
    .unwrap();
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    assert_eq!(
        crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()),
        Ok(Value::Nil),
    );
}

#[test]
fn resumable_provider_registration_rejects_a_gap_in_source_type_parameters() {
    use geam_core::{
        HostCall, HostCallContinuation, HostCallError, HostConstructions, HostOwnedCompletion,
        HostProfile, HostProvider, HostProviderModule, HostTypeListEnd, HostTypeParameter,
        HostValue,
    };

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = ();
        type ExecutionState = ();
    }
    struct Provider;
    impl HostProvider<Profile> for Provider {
        type State = ();
        fn project(state: &mut ()) -> &mut () {
            state
        }
    }
    type Parameter = HostTypeParameter<1>;
    fn resume<'call>(
        call: HostCall<'call, Profile, Provider, ()>,
        constructions: HostConstructions<'call, HostTypeListEnd>,
        _value: HostValue<'call, Parameter>,
    ) -> Result<HostCallContinuation<'call, ()>, HostCallError> {
        Ok(call.resume(constructions, |_| {
            Box::pin(async {
                Ok(HostOwnedCompletion::new(
                    |call, _| Ok(call.return_value(())),
                ))
            })
        }))
    }
    assert_eq!(
        HostModule::<Profile>::new_for_profile("application", "main")
            .unwrap()
            .with_resumable_function::<Provider, (Parameter,), (), HostTypeListEnd, _>(
                "resume", resume
            )
            .err(),
        Some(HostRegistrationError::NonContiguousTypeParameters {
            function: "resume".into(),
            parameters: Box::new([1]),
        }),
    );
    assert_eq!(
        HostProviderModule::<Profile>::new("application", "main")
            .unwrap()
            .with_resumable_function::<Provider, (Parameter,), (), HostTypeListEnd, _>(
                "resume", resume
            )
            .err(),
        Some(HostRegistrationError::NonContiguousTypeParameters {
            function: "resume".into(),
            parameters: Box::new([1]),
        }),
    );
}
