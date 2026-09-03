use crate::geam_bindings::{Functions, Profile, RunState};
use geam::EchoOutput;
use geam::embedding::{BigInt, CallError, EcoString, HostedModule, List};
use geam::gleam_stdlib::IoSink;
use std::io::{self, Write};

pub(super) struct Review {
    rows: List<Result<(EcoString, BigInt), EcoString>>,
    total_quantity: BigInt,
    first_valid: Option<(EcoString, BigInt)>,
}

pub(super) fn review<Io: IoSink + 'static>(
    module: &HostedModule<Profile<Io>>,
    functions: &Functions,
    rows: Vec<(EcoString, BigInt)>,
    state: &mut RunState<Io>,
    echo: &mut Vec<EchoOutput>,
) -> Result<Review, CallError> {
    let checked = module.call(&functions.validate_batch, (rows,), state, echo)?;

    // Borrow the same retained List for both calls, without rebuilding its rows.
    let total_quantity = module.call(&functions.total_quantity, (&checked,), state, echo)?;
    let first_valid = module.call(&functions.first_valid, (&checked,), state, echo)?;

    Ok(Review {
        rows: checked,
        total_quantity,
        first_valid,
    })
}

impl Review {
    pub(super) fn write_report(&self, output: &mut impl Write) -> io::Result<()> {
        writeln!(output, "Inventory validation:")?;
        for (index, row) in self.rows.iter().enumerate() {
            match row {
                Ok((code, quantity)) => writeln!(output, "  {code}: {quantity}")?,
                Err(message) => writeln!(output, "  Row {} rejected: {message}", index + 1)?,
            }
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
        let program = geam_bindings::project()
            .compile()
            .expect("example Gleam project");
        let builder = HostedModuleBuilder::new(program).expect("example library");
        let (bindings, functions) = geam_bindings::bind(builder).expect("generated bindings");
        let module = bindings.seal().expect("example execution");
        let mut state = geam_bindings::RunStateInputs {
            stdlib: GleamStdlibRunState::from_seed([7; 32]),
            example_text_pattern: HostProviderConfiguration::empty(),
        }
        .initialize()
        .expect("example state");
        let mut echo = Vec::new();

        let mixed = review(
            &module,
            &functions,
            vec![
                (" ab-12 ".into(), 3.into()),
                ("invalid".into(), 2.into()),
                (" c-7 ".into(), 4.into()),
                ("D-1".into(), (-1).into()),
            ],
            &mut state,
            &mut echo,
        )
        .expect("mixed receipt review");
        assert_eq!(mixed.rows.len(), 4);
        assert_eq!(mixed.rows.get(0), Some(Ok(("AB-12".into(), 3.into()))));
        assert_eq!(mixed.rows.get(1), Some(Err("invalid code".into())));
        assert_eq!(mixed.rows.get(2), Some(Ok(("C-7".into(), 4.into()))));
        assert_eq!(
            mixed.rows.get(3),
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
            &module,
            &functions,
            vec![("invalid".into(), 2.into()), ("D-1".into(), (-1).into())],
            &mut state,
            &mut echo,
        )
        .expect("rejected receipt review");
        assert_eq!(rejected.rows.len(), 2);
        assert_eq!(rejected.rows.get(0), Some(Err("invalid code".into())));
        assert_eq!(
            rejected.rows.get(1),
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

        let empty = review(&module, &functions, Vec::new(), &mut state, &mut echo)
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
