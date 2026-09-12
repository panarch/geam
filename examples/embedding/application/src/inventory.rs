use crate::geam_bindings::{Functions, Profile};
use geam::embedding::{BigInt, CallError, EcoString, ExecutionScope, SharedList};
use geam::gleam_stdlib::IoSink;
use std::io::{self, Write};

pub(super) struct Review {
    rows: SharedList<Result<(EcoString, BigInt), EcoString>>,
    total_quantity: BigInt,
    first_valid: Option<(EcoString, BigInt)>,
}

pub(super) async fn review<Io: IoSink + 'static>(
    scope: &ExecutionScope<'_, '_, Profile<Io>>,
    functions: &Functions,
    rows: Vec<(EcoString, BigInt)>,
) -> Result<Review, CallError> {
    let checked = scope.call(&functions.validate_batch, (rows,)).await?;

    // Borrow the same retained List for both calls, without rebuilding its rows.
    let total_quantity = scope.call(&functions.total_quantity, (&checked,)).await?;
    let first_valid = scope.call(&functions.first_valid, (&checked,)).await?;

    Ok(Review {
        rows: checked,
        total_quantity,
        first_valid,
    })
}

impl Review {
    pub(super) fn write_report(&self, output: &mut impl Write) -> io::Result<()> {
        writeln!(output, "Inventory validation:")?;
        for index in 0..self.rows.len() {
            self.rows
                .read_item(index, |row| match row {
                    Ok((code, quantity)) => writeln!(output, "  {code}: {quantity}"),
                    Err(message) => writeln!(output, "  Row {} rejected: {message}", index + 1),
                })
                .expect("index below retained row count")?;
        }
        writeln!(output, "Total quantity: {}", self.total_quantity)?;
        match &self.first_valid {
            Some((code, quantity)) => {
                writeln!(output, "First valid item: {code} ({quantity})")
            }
            None => writeln!(output, "First valid item: none"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::review;
    use crate::geam_bindings;
    use geam::HostProviderConfiguration;
    use geam::embedding::{BigInt, HostedModuleBuilder};
    use geam::gleam_stdlib::{GleamStdlibRunState, IoStream};
    use std::io::ErrorKind;

    #[test]
    fn reviews_receipts_with_reusable_bindings_and_caller_owned_outputs() {
        let executor = tokio::runtime::Builder::new_current_thread()
            .build()
            .expect("executor");
        let host = geam::execution::TokioHost::new(executor.handle().clone());
        let program = geam_bindings::project()
            .compile()
            .expect("example Gleam project");
        let builder = HostedModuleBuilder::new(program).expect("example library");
        let (bindings, functions) = geam_bindings::bind(builder).expect("generated bindings");
        let mut module = bindings.seal().expect("example execution");
        let mut state = geam_bindings::RunStateInputs {
            stdlib: GleamStdlibRunState::from_seed([7; 32]),
            example_text_pattern: HostProviderConfiguration::empty(),
        }
        .initialize()
        .expect("example state");
        let mut echo = Vec::new();

        executor
            .block_on(
                module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    let mixed = review(
                        &scope,
                        &functions,
                        vec![
                            (" ab-12 ".into(), 3.into()),
                            ("invalid".into(), 2.into()),
                            (" c-7 ".into(), 4.into()),
                            ("D-1".into(), (-1).into()),
                        ],
                    )
                    .await
                    .expect("mixed receipt review");
                    assert_eq!(mixed.rows.len(), 4);
                    let row = |value: Result<
                        (&geam::embedding::EcoString, &BigInt),
                        &geam::embedding::EcoString,
                    >| {
                        value
                            .map(|(code, quantity)| (code.clone(), quantity.clone()))
                            .map_err(Clone::clone)
                    };
                    assert_eq!(
                        mixed.rows.read_item(0, row),
                        Some(Ok(("AB-12".into(), 3.into())))
                    );
                    assert_eq!(
                        mixed.rows.read_item(1, row),
                        Some(Err("invalid code".into()))
                    );
                    assert_eq!(
                        mixed.rows.read_item(2, row),
                        Some(Ok(("C-7".into(), 4.into())))
                    );
                    assert_eq!(
                        mixed.rows.read_item(3, row),
                        Some(Err("quantity must not be negative".into()))
                    );
                    assert_eq!(mixed.total_quantity, BigInt::from(7));
                    assert_eq!(mixed.first_valid, Some(("AB-12".into(), 3.into())));
                    let mut output = Vec::new();
                    mixed.write_report(&mut output).expect("mixed report");
                    assert_eq!(
                        output,
                        br"Inventory validation:
  AB-12: 3
  Row 2 rejected: invalid code
  C-7: 4
  Row 4 rejected: quantity must not be negative
Total quantity: 7
First valid item: AB-12 (3)
"
                    );
                    let mut full_output: &mut [u8] = &mut [];
                    assert_eq!(
                        mixed
                            .write_report(&mut full_output)
                            .expect_err("full output buffer")
                            .kind(),
                        ErrorKind::WriteZero
                    );

                    let rejected = review(
                        &scope,
                        &functions,
                        vec![("invalid".into(), 2.into()), ("D-1".into(), (-1).into())],
                    )
                    .await
                    .expect("rejected receipt review");
                    assert_eq!(rejected.rows.len(), 2);
                    assert_eq!(
                        rejected.rows.read_item(0, row),
                        Some(Err("invalid code".into()))
                    );
                    assert_eq!(
                        rejected.rows.read_item(1, row),
                        Some(Err("quantity must not be negative".into()))
                    );
                    assert_eq!(rejected.total_quantity, BigInt::from(0));
                    assert_eq!(rejected.first_valid, None);
                    let mut output = Vec::new();
                    rejected.write_report(&mut output).expect("rejected report");
                    assert_eq!(
                        output,
                        br"Inventory validation:
  Row 1 rejected: invalid code
  Row 2 rejected: quantity must not be negative
Total quantity: 0
First valid item: none
"
                    );

                    let empty = review(&scope, &functions, Vec::new())
                        .await
                        .expect("empty receipt review");
                    assert!(empty.rows.is_empty());
                    assert_eq!(empty.total_quantity, BigInt::from(0));
                    assert_eq!(empty.first_valid, None);
                    let mut output = Vec::new();
                    empty.write_report(&mut output).expect("empty report");
                    assert_eq!(
                        output,
                        br"Inventory validation:
Total quantity: 0
First valid item: none
"
                    );
                }),
            )
            .expect("controlled execution");
        assert!(echo.is_empty());
        assert_eq!(state.stdlib().io_outputs().len(), 3);
        let outputs = state.stdlib_mut().take_io_outputs();
        assert_eq!(
            outputs
                .iter()
                .map(|output| (output.stream(), output.text().as_str()))
                .collect::<Vec<_>>(),
            [(IoStream::Stdout, "validating inventory\n"); 3]
        );
        assert!(state.stdlib().io_outputs().is_empty());
    }
}
