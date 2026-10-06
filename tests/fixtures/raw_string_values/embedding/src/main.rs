mod geam_bindings;

use geam::embedding::{CallError, HostedModuleBuilder, StringValue};
use geam::execution::TokioHost;
use geam::{
    BitArraySegmentPanicReason, ExecutionError, HostProviderConfiguration, PanicDetails, PanicKind,
    PanicSite, SourceSpan,
};
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let executor = tokio::runtime::Builder::new_current_thread().build()?;
    let host = TokioHost::new(executor.handle().clone());
    let mut executions = Vec::new();
    if std::env::args().nth(1).as_deref() != Some("--prepared") {
        let program = geam_bindings::project().compile()?;
        let (bindings, functions) = geam_bindings::bind(HostedModuleBuilder::new(program)?)?;
        executions.push((bindings.seal()?, functions));
    }
    let (bindings, functions) = geam_bindings::load()?;
    executions.push((bindings, functions));
    for (mut module, functions) in executions {
        let mut state = geam_bindings::RunStateInputs {
            stdlib: geam::gleam_stdlib::GleamStdlibRunState::from_seed([0; 32]),
            raw_string_values_fixture: HostProviderConfiguration::empty(),
        }
        .initialize()?;
        let mut echo = Vec::new();
        for _ in 0..2 {
            executor
                .block_on(
                    module.with_execution(&host, &mut state, &mut echo, async |scope| {
                        let raw = StringValue::from_bytes(vec![255, 254]);
                        let restored = scope
                            .call(
                                &functions.strip_prefix,
                                (StringValue::from("λ").concat(&raw),),
                            )
                            .await?;
                        assert_eq!(restored.as_bytes(), [255, 254]);
                        let counted = scope
                            .call(
                                &functions.count_prefix,
                                (StringValue::from("λλ").concat(&raw), 7.into()),
                            )
                            .await?;
                        assert_eq!(counted, 9.into());
                        let valid = StringValue::from_bytes("é🙂".as_bytes().to_vec());
                        assert_eq!(
                            scope
                                .call(&functions.utf16_encode, (valid.clone(),))
                                .await?
                                .bytes(),
                            &[0, 0xe9, 0xd8, 0x3d, 0xde, 0x42],
                        );
                        assert_eq!(
                            scope.call(&functions.utf32_encode, (valid,)).await?.bytes(),
                            &[0, 0, 0, 0xe9, 0, 1, 0xf6, 0x42],
                        );
                        let invalid = StringValue::from_bytes(vec![b'a', 0xff]);
                        let utf8_error = invalid.as_str().unwrap_err();
                        for (error, name, segment) in [
                            (
                                scope
                                    .call(&functions.utf16_encode, (invalid.clone(),))
                                    .await
                                    .unwrap_err(),
                                "utf16_encode",
                                "text:utf16",
                            ),
                            (
                                scope
                                    .call(&functions.utf32_encode, (invalid,))
                                    .await
                                    .unwrap_err(),
                                "utf32_encode",
                                "text:utf32",
                            ),
                        ] {
                            let error = error.into_materialized();
                            let CallError::Execution(ExecutionError::Panic(panic)) = error else {
                                panic!("expected string encoding source panic");
                            };
                            let source =
                                include_str!("../gleam/src/raw_string_values_embedding.gleam");
                            let start = source.find(segment).unwrap();
                            assert_eq!(panic.kind(), PanicKind::BitArraySegment);
                            assert_eq!(
                                panic.site(),
                                &PanicSite::new(
                                    "raw_string_values_embedding".into(),
                                    name.into(),
                                    SourceSpan::new(start, start + segment.len()),
                                )
                            );
                            assert_eq!(
                                panic.details(),
                                Some(&PanicDetails::BitArraySegment {
                                    reason: BitArraySegmentPanicReason::InvalidStringEncoding {
                                        error: utf8_error,
                                    },
                                })
                            );
                        }
                        scope.call(&functions.main, ()).await
                    }),
                )?
                .try_into_value()
                .map_err(|status| format!("unexpected exit status {status}"))?
                .map_err(|error| error.to_string())?;
        }
        assert!(echo.is_empty());
        assert_eq!(state.stdlib().io_outputs().len(), 4);
        for output in state.stdlib_mut().take_io_outputs() {
            io::stdout().lock().write_all(output.text().as_bytes())?;
        }
    }
    Ok(())
}
