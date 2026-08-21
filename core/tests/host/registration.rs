use geam_core::{HostModule, HostProviderSet, HostRegistrationError};
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
