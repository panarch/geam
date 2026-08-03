mod function;

use self::function::{IoProvider, print, print_error, println, println_error};
use crate::gleam_stdlib::GleamStdlibHostProfile;
use crate::{HostProviderModule, HostRegistrationError};
use ecow::EcoString;

/// A caller-owned destination for official Gleam standard-library IO events.
pub trait IoSink {
    /// Receives one owned standard-library IO event.
    fn emit(&mut self, output: IoOutput);
}

/// One owned standard-library IO event.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IoOutput {
    stream: IoStream,
    text: EcoString,
}

/// The standard stream selected by a Gleam IO operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IoStream {
    /// Standard output.
    Stdout,
    /// Standard error.
    Stderr,
}

impl IoOutput {
    pub(super) fn new(stream: IoStream, text: EcoString) -> Self {
        Self { stream, text }
    }

    /// Returns the selected standard stream.
    pub fn stream(&self) -> IoStream {
        self.stream
    }

    /// Returns the exact text emitted by the Gleam IO operation.
    pub fn text(&self) -> &EcoString {
        &self.text
    }
}

impl IoSink for Vec<IoOutput> {
    fn emit(&mut self, output: IoOutput) {
        self.push(output);
    }
}

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    HostProviderModule::new("gleam_stdlib", "gleam/io")
        .and_then(|provider| {
            provider.with_scoped_function::<IoProvider<Profile>, (EcoString,), (), _>(
                "print",
                print::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<IoProvider<Profile>, (EcoString,), (), _>(
                "print_error",
                print_error::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<IoProvider<Profile>, (EcoString,), (), _>(
                "println",
                println::<Profile>,
            )
        })
        .and_then(|provider| {
            provider.with_scoped_function::<IoProvider<Profile>, (EcoString,), (), _>(
                "println_error",
                println_error::<Profile>,
            )
        })
}

#[cfg(test)]
mod tests {
    use super::{IoOutput, IoSink, IoStream, host_provider};
    use crate::gleam_stdlib::GleamStdlibProfile;

    #[test]
    fn output_preserves_owned_stream_and_text() {
        let output = IoOutput::new(IoStream::Stderr, "message".into());

        assert_eq!(output.stream(), IoStream::Stderr);
        assert_eq!(output.text(), "message");
        assert_eq!(output.clone(), output);
    }

    #[test]
    fn vector_sink_collects_outputs_in_order() {
        let mut outputs = Vec::new();
        outputs.emit(IoOutput::new(IoStream::Stdout, "first".into()));
        outputs.emit(IoOutput::new(IoStream::Stderr, "second".into()));

        assert_eq!(
            outputs
                .iter()
                .map(|output| (output.stream(), output.text().as_str()))
                .collect::<Vec<_>>(),
            [(IoStream::Stdout, "first"), (IoStream::Stderr, "second")],
        );
    }

    #[test]
    fn registers_the_exact_official_io_provider_inventory() {
        let provider =
            host_provider::<GleamStdlibProfile>().expect("official IO provider should register");
        let functions = provider.functions().collect::<Vec<_>>();

        assert_eq!(provider.package(), "gleam_stdlib");
        assert_eq!(provider.module(), "gleam/io");
        assert_eq!(
            functions
                .iter()
                .map(|function| function.name().as_str())
                .collect::<Vec<_>>(),
            ["print", "print_error", "println", "println_error"],
        );
        for function in functions {
            assert!(function.scheme().is_monomorphic());
            assert_eq!(
                function.type_().argument_types(),
                [crate::ValueType::String],
            );
            assert_eq!(function.type_().return_(), &crate::ValueType::Nil);
        }
    }
}
