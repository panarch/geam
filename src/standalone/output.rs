use geam_stdlib::{IoOutput, IoSink, IoStream};
use std::io::{self, Write};
use std::sync::{Arc, OnceLock};

#[derive(Clone)]
pub struct SharedOutput {
    streams: Arc<dyn Streams>,
    failure: Arc<OnceLock<Arc<io::Error>>>,
}

impl SharedOutput {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn io_sink(&self) -> CliIoSink {
        CliIoSink {
            output: self.clone(),
        }
    }

    pub fn echo_sink(&self) -> CliEchoSink {
        CliEchoSink {
            output: self.clone(),
        }
    }

    pub fn finish(&self) -> Result<(), Arc<io::Error>> {
        match self.failure.get() {
            Some(error) => Err(Arc::clone(error)),
            None => Ok(()),
        }
    }

    fn write(&self, stream: OutputStream, text: &[u8]) {
        if self.failure.get().is_some() {
            return;
        }
        if let Err(error) = self.streams.write(stream, text) {
            let _ = self.failure.set(Arc::new(error));
        }
    }
}

impl Default for SharedOutput {
    fn default() -> Self {
        Self {
            streams: Arc::new(SystemStreams),
            failure: Arc::new(OnceLock::new()),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OutputStream {
    Stdout,
    Stderr,
}

trait Streams: Send + Sync {
    fn write(&self, stream: OutputStream, text: &[u8]) -> io::Result<()>;
}

struct SystemStreams;

impl Streams for SystemStreams {
    fn write(&self, stream: OutputStream, text: &[u8]) -> io::Result<()> {
        match stream {
            OutputStream::Stdout => write_flushed(&mut io::stdout().lock(), text),
            OutputStream::Stderr => write_flushed(&mut io::stderr().lock(), text),
        }
    }
}

fn write_flushed(writer: &mut impl Write, text: &[u8]) -> io::Result<()> {
    writer.write_all(text)?;
    writer.flush()
}

pub struct CliIoSink {
    output: SharedOutput,
}

impl IoSink for CliIoSink {
    fn emit(&mut self, output: IoOutput) {
        let stream = match output.stream() {
            IoStream::Stdout => OutputStream::Stdout,
            IoStream::Stderr => OutputStream::Stderr,
        };
        self.output.write(stream, output.text().as_bytes());
    }
}

pub struct CliEchoSink {
    output: SharedOutput,
}

impl geam_core::EchoSink for CliEchoSink {
    fn emit(&mut self, output: geam_core::EchoOutput) {
        let mut text = output.to_string();
        text.push('\n');
        self.output.write(OutputStream::Stderr, text.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::{OutputStream, SharedOutput, Streams, SystemStreams, write_flushed};
    use geam_core::StringValue;
    use geam_core::embedding::{FunctionDeclaration, HostedModuleBuilder};
    use geam_core::{EchoSink, HostProviderSet, ModuleSource, PackageSource};
    use geam_stdlib::{GleamStdlibProfile, GleamStdlibRunState, IoSink};
    use std::io::{self, Write};
    use std::sync::{Arc, Mutex, OnceLock};

    #[derive(Default)]
    struct RecordedStreams(Mutex<Vec<(OutputStream, Vec<u8>)>>);

    impl Streams for RecordedStreams {
        fn write(&self, stream: OutputStream, text: &[u8]) -> io::Result<()> {
            self.0.lock().unwrap().push((stream, text.to_owned()));
            Ok(())
        }
    }

    #[test]
    fn native_io_and_echo_keep_their_exact_text_and_destinations() {
        let source = r#"
import gleam/io
pub fn main() {
  io.print("message")
  io.print_error("problem")
  echo 42
  Nil
}
pub fn raw(value: String) {
  io.println(value)
  io.println_error(value)
  Nil
}
"#
        .trim_start();
        let io_source = r#"
@external(erlang, "fixture", "print")
pub fn print(value: String) -> Nil
@external(erlang, "fixture", "print_error")
pub fn print_error(value: String) -> Nil
@external(erlang, "fixture", "println")
pub fn println(value: String) -> Nil
@external(erlang, "fixture", "println_error")
pub fn println_error(value: String) -> Nil
"#;
        let program = geam_core::compile_typed_host_program(
            "app",
            "main",
            [
                PackageSource::new(
                    "app",
                    ["gleam_stdlib"],
                    [ModuleSource::new("main", "src/main.gleam", source)],
                ),
                PackageSource::new(
                    "gleam_stdlib",
                    Vec::<String>::new(),
                    [ModuleSource::new(
                        "gleam/io",
                        "src/gleam/io.gleam",
                        io_source,
                    )],
                ),
            ],
            HostProviderSet::from_providers(
                geam_stdlib::host_providers::<GleamStdlibProfile>()
                    .unwrap()
                    .into_iter()
                    .filter(|provider| provider.module() == "gleam/io"),
            )
            .unwrap(),
        )
        .unwrap();
        let (mut builder, entry) = HostedModuleBuilder::new(program)
            .unwrap()
            .function(FunctionDeclaration::<(), ()>::new("main"))
            .unwrap();
        let raw = builder
            .function(FunctionDeclaration::<(StringValue,), ()>::new("raw"))
            .unwrap();
        let mut module = builder.seal().unwrap();
        let mut state = GleamStdlibRunState::from_seed([0; 32]);
        let mut echo = Vec::new();
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let host = geam_core::execution::TokioHost::new(runtime.handle().clone());
        runtime
            .block_on(
                module.with_execution(&host, &mut state, &mut echo, async |scope| {
                    scope.call(&entry, ()).await.unwrap();
                    scope
                        .call(&raw, (StringValue::from_bytes(vec![0, 0x80, 0xff, 0xc3]),))
                        .await
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap();
        let streams = Arc::new(RecordedStreams::default());
        let output = SharedOutput {
            streams: streams.clone(),
            failure: Arc::new(OnceLock::new()),
        };
        for value in state.take_io_outputs() {
            output.io_sink().emit(value);
        }
        for value in echo {
            output.echo_sink().emit(value);
        }
        assert_eq!(
            *streams.0.lock().unwrap(),
            [
                (OutputStream::Stdout, b"message".to_vec()),
                (OutputStream::Stderr, b"problem".to_vec()),
                (OutputStream::Stdout, vec![0, 0x80, 0xff, 0xc3, b'\n']),
                (OutputStream::Stderr, vec![0, 0x80, 0xff, 0xc3, b'\n']),
                (OutputStream::Stderr, b"src/main.gleam:5\n42\n".to_vec()),
            ]
        );
        output.finish().unwrap();
    }

    struct ClosedStreams(std::sync::atomic::AtomicUsize);

    impl Streams for ClosedStreams {
        fn write(&self, _stream: OutputStream, _text: &[u8]) -> io::Result<()> {
            self.0.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Err(io::Error::new(io::ErrorKind::BrokenPipe, "closed stream"))
        }
    }

    #[test]
    fn preserves_first_output_failure_and_stops_writing_through_every_alias() {
        let streams = Arc::new(ClosedStreams(0.into()));
        let output = SharedOutput {
            streams: streams.clone(),
            failure: Arc::new(OnceLock::new()),
        };
        output.write(OutputStream::Stdout, b"first");
        let first = output.finish().unwrap_err();
        output.clone().write(OutputStream::Stderr, b"second");
        let second = output.finish().unwrap_err();
        assert!(Arc::ptr_eq(&first, &second));
        assert_eq!(first.kind(), io::ErrorKind::BrokenPipe);
        assert_eq!(first.to_string(), "closed stream");
        assert_eq!(streams.0.load(std::sync::atomic::Ordering::Relaxed), 1);
    }

    struct Writer {
        fail_write: bool,
        fail_flush: bool,
        bytes: Vec<u8>,
        flushes: usize,
    }

    impl Write for Writer {
        fn write(&mut self, bytes: &[u8]) -> io::Result<usize> {
            if self.fail_write {
                return Err(io::Error::new(io::ErrorKind::BrokenPipe, "write failed"));
            }
            self.bytes.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> io::Result<()> {
            self.flushes += 1;
            if self.fail_flush {
                Err(io::Error::new(io::ErrorKind::BrokenPipe, "flush failed"))
            } else {
                Ok(())
            }
        }
    }

    #[test]
    fn write_and_flush_failures_remain_errors_after_an_intentional_exit() {
        use geam_core::execution::{ExecutionOutcome, ExitStatus};
        use geam_core::{
            HostCall, HostCallCompletion, HostCallError, HostProfile, HostProvider,
            HostProviderModule,
        };

        struct Stop;
        impl HostProfile for Stop {
            type RunState = ExitStatus;
            type ExternalStores = ();
            type ExecutionState = ();
        }
        impl HostProvider<Stop> for Stop {
            type State = ExitStatus;
            fn project(state: &mut Self::State) -> &mut Self::State {
                state
            }
        }
        fn exit<'call>(
            mut call: HostCall<'call, Stop, Stop, ()>,
        ) -> Result<HostCallCompletion<'call, ()>, HostCallError> {
            let status = *call.state();
            call.exit(status)
        }
        struct WriterStreams(Mutex<Writer>);
        impl Streams for WriterStreams {
            fn write(&self, _: OutputStream, text: &[u8]) -> io::Result<()> {
                write_flushed(&mut *self.0.lock().unwrap(), text)
            }
        }
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let host = geam_core::execution::TokioHost::new(runtime.handle().clone());
        for fail_flush in [false, true] {
            for status in [None, Some(ExitStatus::new(0)), Some(ExitStatus::new(7))] {
                let source = if status.is_some() {
                    r#"
@external(erlang, "native", "exit")
fn exit() -> Nil
pub fn main() { echo "before" exit() echo "after" Nil }
"#
                } else {
                    "@external(erlang, \"native\", \"exit\")\nfn exit() -> Nil\npub fn main() { echo \"before\" Nil }\n"
                };
                let typed = geam_core::compile_typed_host_program(
                    "app",
                    "main",
                    [PackageSource::new(
                        "app",
                        Vec::<String>::new(),
                        [ModuleSource::new("main", "main.gleam", source)],
                    )],
                    HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                        .unwrap()
                        .with_scoped_function::<Stop, (), (), _>("exit", exit)
                        .unwrap()])
                    .unwrap(),
                )
                .unwrap();
                let (builder, main) = HostedModuleBuilder::new(typed)
                    .unwrap()
                    .function(FunctionDeclaration::<(), ()>::new("main"))
                    .unwrap();
                let mut module = builder.seal().unwrap();
                let writer = Arc::new(WriterStreams(Mutex::new(Writer {
                    fail_write: !fail_flush,
                    fail_flush,
                    bytes: Vec::new(),
                    flushes: 0,
                })));
                let output = SharedOutput {
                    streams: writer.clone(),
                    failure: Arc::new(OnceLock::new()),
                };
                let mut state = status.unwrap_or(ExitStatus::new(0));
                let mut echo = output.echo_sink();
                let outcome = runtime
                    .block_on(
                        module.with_execution(&host, &mut state, &mut echo, async |scope| {
                            scope.call(&main, ()).await
                        }),
                    )
                    .unwrap();
                assert_eq!(
                    outcome,
                    match status {
                        Some(status) => ExecutionOutcome::Exited(status),
                        None => ExecutionOutcome::Returned(Ok(())),
                    }
                );
                let failure = output.finish().unwrap_err();
                assert_eq!(failure.kind(), io::ErrorKind::BrokenPipe);
                assert_eq!(
                    failure.to_string(),
                    if fail_flush {
                        "flush failed"
                    } else {
                        "write failed"
                    }
                );
                let writer = writer.0.lock().unwrap();
                assert_eq!(writer.flushes, usize::from(fail_flush));
                if fail_flush {
                    assert!(
                        std::str::from_utf8(&writer.bytes)
                            .unwrap()
                            .ends_with("\"before\"\n")
                    );
                } else {
                    assert!(writer.bytes.is_empty());
                }
            }
        }
    }

    #[test]
    fn writes_and_flushes_native_streams_without_buffering_or_losing_errors() {
        for (fail_write, fail_flush, expected, bytes, flushes) in [
            (false, false, None, "text", 1),
            (true, false, Some("write failed"), "", 0),
            (false, true, Some("flush failed"), "text", 1),
        ] {
            let mut writer = Writer {
                fail_write,
                fail_flush,
                bytes: Vec::new(),
                flushes: 0,
            };
            assert_eq!(
                write_flushed(&mut writer, b"text")
                    .err()
                    .map(|error| error.to_string())
                    .as_deref(),
                expected
            );
            assert_eq!(writer.bytes, bytes.as_bytes());
            assert_eq!(writer.flushes, flushes);
        }
        SystemStreams.write(OutputStream::Stdout, b"").unwrap();
        SystemStreams.write(OutputStream::Stderr, b"").unwrap();
        let output = SharedOutput::new();
        output.finish().unwrap();
    }
}
