use super::{IoOutput, IoSink, IoStream};
use crate::gleam_stdlib::GleamStdlibHostProfile;
use crate::{HostCall, HostCallCompletion, HostCallError, HostProvider};
use ecow::EcoString;
use std::marker::PhantomData;

pub(super) struct IoProvider<Profile>(PhantomData<Profile>);

impl<Profile> HostProvider<Profile> for IoProvider<Profile>
where
    Profile: GleamStdlibHostProfile,
{
    type State = Profile::Io;

    fn project(state: &mut Profile::RunState) -> &mut Self::State {
        Profile::gleam_stdlib_io(state)
    }
}

pub(super) fn print<'call, Profile>(
    call: HostCall<'call, Profile, IoProvider<Profile>, ()>,
    text: EcoString,
) -> Result<HostCallCompletion<'call, ()>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    emit(call, IoStream::Stdout, text)
}

pub(super) fn print_error<'call, Profile>(
    call: HostCall<'call, Profile, IoProvider<Profile>, ()>,
    text: EcoString,
) -> Result<HostCallCompletion<'call, ()>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    emit(call, IoStream::Stderr, text)
}

pub(super) fn println<'call, Profile>(
    call: HostCall<'call, Profile, IoProvider<Profile>, ()>,
    mut text: EcoString,
) -> Result<HostCallCompletion<'call, ()>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    text.push('\n');
    emit(call, IoStream::Stdout, text)
}

pub(super) fn println_error<'call, Profile>(
    call: HostCall<'call, Profile, IoProvider<Profile>, ()>,
    mut text: EcoString,
) -> Result<HostCallCompletion<'call, ()>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    text.push('\n');
    emit(call, IoStream::Stderr, text)
}

fn emit<'call, Profile>(
    mut call: HostCall<'call, Profile, IoProvider<Profile>, ()>,
    stream: IoStream,
    text: EcoString,
) -> Result<HostCallCompletion<'call, ()>, HostCallError>
where
    Profile: GleamStdlibHostProfile,
{
    call.state().emit(IoOutput::new(stream, text));
    Ok(call.return_value(()))
}

#[cfg(test)]
mod tests {
    use crate::gleam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, IoStream};
    use crate::{
        HostModule, HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
        compile_typed_host_program, plan_host_program,
    };
    use ecow::EcoString;

    const IO_SOURCE: &str = r#"
@external(erlang, "gleam_stdlib", "print")
pub fn print(string: String) -> Nil

@external(erlang, "gleam_stdlib", "print_error")
pub fn print_error(string: String) -> Nil

@external(erlang, "gleam_stdlib", "println")
pub fn println(string: String) -> Nil

@external(erlang, "gleam_stdlib", "println_error")
pub fn println_error(string: String) -> Nil

pub fn main() {
  print("first")
  print_error("second")
  println("third")
  println_error("fourth")
}
"#;

    #[test]
    fn executes_all_io_callbacks_through_the_typed_provider() {
        let provider = super::super::host_provider::<GleamStdlibProfile>()
            .expect("synthetic IO provider should register");
        let typed = compile_typed_host_program(
            "gleam_stdlib",
            "gleam/io",
            [PackageSource::new(
                "gleam_stdlib",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "gleam/io",
                    "src/gleam/io.gleam",
                    IO_SOURCE,
                )],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<GleamStdlibProfile>>::new(),
                [provider],
            )
            .expect("synthetic IO provider module should be unique"),
        )
        .expect("synthetic IO source should compile");
        let plan = plan_host_program(typed).expect("synthetic IO source should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("synthetic IO source should seal");
        let mut state = GleamStdlibRunState::from_seed([2; 32]);

        assert_eq!(
            execution
                .run_main(&mut state, &mut Vec::new())
                .expect("synthetic IO source should run"),
            Value::Nil,
        );
        assert_eq!(
            state
                .io_outputs()
                .iter()
                .map(|output| (output.stream(), output.text().as_str()))
                .collect::<Vec<_>>(),
            [
                (IoStream::Stdout, "first"),
                (IoStream::Stderr, "second"),
                (IoStream::Stdout, "third\n"),
                (IoStream::Stderr, "fourth\n"),
            ],
        );
    }
}
