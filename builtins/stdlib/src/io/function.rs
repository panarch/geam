use super::{IoOutput, IoSink, IoStream};
use ecow::EcoString;

pub(super) fn print<Io>(io: &mut Io, text: EcoString)
where
    Io: IoSink,
{
    emit(io, IoStream::Stdout, text);
}

pub(super) fn print_error<Io>(io: &mut Io, text: EcoString)
where
    Io: IoSink,
{
    emit(io, IoStream::Stderr, text);
}

pub(super) fn println<Io>(io: &mut Io, mut text: EcoString)
where
    Io: IoSink,
{
    text.push('\n');
    emit(io, IoStream::Stdout, text);
}

pub(super) fn println_error<Io>(io: &mut Io, mut text: EcoString)
where
    Io: IoSink,
{
    text.push('\n');
    emit(io, IoStream::Stderr, text);
}

fn emit<Io>(io: &mut Io, stream: IoStream, text: EcoString)
where
    Io: IoSink,
{
    io.emit(IoOutput::new(stream, text));
}

#[cfg(test)]
mod tests {
    use crate::{GleamStdlibProfile, GleamStdlibRunState, IoStream};
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
