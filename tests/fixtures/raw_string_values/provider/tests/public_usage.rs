use geam::embedding::{BigInt, FunctionDeclaration, HostedModuleBuilder, StringValue};
use geam::execution::TokioHost;
use geam::gleam_stdlib::{
    Component as StdlibComponent, GleamStdlibHostProfile, GleamStdlibRunState, GleamStdlibStores,
    IoOutput,
};
use geam::{HostComponentProfile, HostProfile, HostProviderComponentRegistration, HostProviderSet};
use geam_raw_string_values_fixture::{Component, Stores as ProviderStores};
use std::{path::Path, process::Command};

struct Profile;

struct State {
    stdlib: GleamStdlibRunState,
    provider: (),
}

#[derive(Default)]
struct Stores {
    provider: ProviderStores,
    stdlib: GleamStdlibStores,
}

impl HostProfile for Profile {
    type RunState = State;
    type ExternalStores = Stores;
    type ExecutionState = ();
}

impl GleamStdlibHostProfile for Profile {
    type Io = Vec<IoOutput>;
}

impl HostComponentProfile<StdlibComponent> for Profile {
    fn component_state(state: &mut State) -> &mut GleamStdlibRunState {
        &mut state.stdlib
    }
    fn component_stores(stores: &Stores) -> &GleamStdlibStores {
        &stores.stdlib
    }
}

impl HostComponentProfile<Component> for Profile {
    fn component_state(state: &mut State) -> &mut () {
        &mut state.provider
    }
    fn component_stores(stores: &Stores) -> &ProviderStores {
        &stores.provider
    }
}

#[test]
fn separate_crypto_calls_and_public_embedding_preserve_exact_string_bytes() {
    let project = Path::new(env!("CARGO_MANIFEST_DIR")).join("../project");
    let acquisition = Command::new("gleam")
        .args(["deps", "download"])
        .current_dir(&project)
        .output()
        .unwrap();
    assert!(
        acquisition.status.success(),
        "{}",
        String::from_utf8_lossy(&acquisition.stderr)
    );
    let mut providers = geam::gleam_stdlib::host_providers::<Profile>().unwrap();
    providers
        .extend(<Component as HostProviderComponentRegistration<Profile>>::providers().unwrap());
    let typed = geam::compile_typed_host_project(
        project.to_str().unwrap(),
        "raw_string_values_fixture",
        HostProviderSet::from_providers(providers).unwrap(),
    )
    .unwrap();
    let (mut bindings, handshake) = HostedModuleBuilder::new(typed)
        .unwrap()
        .function(FunctionDeclaration::<
            (StringValue,),
            (StringValue, StringValue),
        >::new("handshake"))
        .unwrap();
    let sample = bindings
        .function(FunctionDeclaration::<(BigInt,), Result<StringValue, ()>>::new("sample"))
        .unwrap();
    let slice = bindings
        .function(FunctionDeclaration::<
            (StringValue, BigInt, BigInt),
            Result<StringValue, ()>,
        >::new("byte_slice"))
        .unwrap();
    let strip = bindings
        .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
            "strip_prefix",
        ))
        .unwrap();
    let captured = bindings
        .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
            "captured_round_trip",
        ))
        .unwrap();
    let main = bindings
        .function(FunctionDeclaration::<(), ()>::new("main"))
        .unwrap();
    let mut module = bindings.seal().unwrap();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut state = State {
        stdlib: GleamStdlibRunState::from_seed([0; 32]),
        provider: (),
    };
    let mut echo = Vec::new();
    for _ in 0..2 {
        runtime
            .block_on(
                module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    let (digest, accept) = scope
                        .call(&handshake, ("dGhlIHNhbXBsZSBub25jZQ==".into(),))
                        .await
                        .unwrap();
                    assert_eq!(
                        digest.as_bytes(),
                        &[
                            0xb3, 0x7a, 0x4f, 0x2c, 0xc0, 0x62, 0x4f, 0x16, 0x90, 0xf6, 0x46, 0x06,
                            0xcf, 0x38, 0x59, 0x45, 0xb2, 0xbe, 0xc4, 0xea
                        ]
                    );
                    assert!(digest.as_str().is_err());
                    assert_eq!(accept.as_str(), Ok("s3pPLMBiTxaQ9kYGzzhZRbK+xOo="));
                    for (index, expected) in [
                        b"".as_slice(),
                        b"\0",
                        b"\x80",
                        b"\xff\xfe",
                        b"\xc3",
                        "é🙂".as_bytes(),
                    ]
                    .into_iter()
                    .enumerate()
                    {
                        let value = scope.call(&sample, (index.into(),)).await.unwrap().unwrap();
                        assert_eq!(value.as_bytes(), expected);
                        assert_eq!(
                            scope
                                .call(&captured, (value.clone(),))
                                .await
                                .unwrap()
                                .as_bytes(),
                            expected
                        );
                        let original = value.clone();
                        let prefixed = StringValue::from("λ").concat(&value);
                        drop(value);
                        let restored = scope.call(&strip, (prefixed,)).await.unwrap();
                        assert_eq!(restored.as_bytes(), expected);
                        assert_eq!(
                            scope.call(&strip, (original,)).await.unwrap().as_bytes(),
                            expected
                        );
                    }
                    for index in [-1, 6] {
                        assert_eq!(scope.call(&sample, (index.into(),)).await.unwrap(), Err(()));
                    }
                    let unicode = StringValue::from("é🙂");
                    let first = scope
                        .call(&slice, (unicode.clone(), 0.into(), 1.into()))
                        .await
                        .unwrap()
                        .unwrap();
                    let second = scope
                        .call(&slice, (unicode, 1.into(), 1.into()))
                        .await
                        .unwrap()
                        .unwrap();
                    assert_eq!(first.as_bytes(), &[0xc3]);
                    assert!(first.as_str().is_err());
                    assert_eq!(first.concat(&second).as_str(), Ok("é"));
                    for (start, length) in [
                        (BigInt::from(-1), BigInt::from(1)),
                        (0.into(), (-1).into()),
                        (BigInt::from(usize::MAX), 1.into()),
                        (10.into(), 1.into()),
                    ] {
                        assert_eq!(
                            scope
                                .call(&slice, ("abc".into(), start, length))
                                .await
                                .unwrap(),
                            Err(())
                        );
                    }
                    scope.call(&main, ()).await.unwrap();
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap();
    }
    assert!(echo.is_empty());
    let outputs = state.stdlib.take_io_outputs();
    assert_eq!(
        outputs
            .iter()
            .map(|output| output.text().as_bytes())
            .collect::<Vec<_>>(),
        [
            b"s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\n".as_slice(),
            b"\xff\xfe\n",
            b"s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\n",
            b"\xff\xfe\n"
        ]
    );
}
