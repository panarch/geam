mod geam_bindings;

use geam::HostProviderConfiguration;
use geam::embedding::{BigInt, EcoString, HostedModuleBuilder};
use geam::gleam_stdlib::{GleamStdlibRunState, IoStream};
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let program = geam_bindings::project()?.compile()?;
    let builder = HostedModuleBuilder::new(program)?;
    let (bindings, functions) = geam_bindings::bind(builder)?;
    let module = bindings.seal()?;

    let mut state = geam_bindings::RunStateInputs {
        stdlib: GleamStdlibRunState::from_seed([7; 32]),
        example_text_pattern: HostProviderConfiguration::empty(),
    }
    .initialize()?;
    let mut echo = Vec::new();

    let first = module.call(
        &functions.normalize,
        (" ab-12 ".into(),),
        &mut state,
        &mut echo,
    )?;
    let second = module.call(
        &functions.normalize,
        (" c-7 ".into(),),
        &mut state,
        &mut echo,
    )?;
    let valid = module.call(
        &functions.validate,
        (" ab-12 ".into(), 3.into()),
        &mut state,
        &mut echo,
    )?;
    let invalid = module.call(
        &functions.validate,
        ("invalid".into(), 2.into()),
        &mut state,
        &mut echo,
    )?;
    assert_eq!(first, "AB-12");
    assert_eq!(second, "C-7");
    assert_eq!(valid, Ok(("AB-12".into(), 3.into())));
    assert_eq!(invalid, Err("invalid code".into()));

    let rows: Vec<(EcoString, BigInt)> = vec![
        (first, 3.into()),
        ("invalid".into(), 2.into()),
        (second, 4.into()),
        ("D-1".into(), (-1).into()),
    ];
    let checked = module.call(&functions.validate_batch, (rows,), &mut state, &mut echo)?;
    assert_eq!(checked.len(), 4);
    assert_eq!(checked.get(0), Some(Ok(("AB-12".into(), 3.into()))));
    assert_eq!(checked.get(1), Some(Err("invalid code".into())));
    assert_eq!(
        checked.get(3),
        Some(Err("quantity must not be negative".into()))
    );

    let total = module.call(
        &functions.total_quantity,
        (&checked,),
        &mut state,
        &mut echo,
    )?;
    let first_valid = module.call(&functions.first_valid, (&checked,), &mut state, &mut echo)?;
    assert_eq!(total, BigInt::from(7));
    assert_eq!(first_valid, Some(("AB-12".into(), 3.into())));
    assert!(echo.is_empty());

    assert_eq!(state.stdlib().io_outputs().len(), 2);
    let outputs = state.stdlib_mut().take_io_outputs();
    assert_eq!(outputs.len(), 2);
    for output in outputs {
        assert_eq!(output.stream(), IoStream::Stdout);
        assert_eq!(output.text(), "normalizing code\n");
    }

    println!("total quantity: {total}");
    Ok(())
}
