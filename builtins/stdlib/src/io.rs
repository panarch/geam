mod function;

use crate::{Component, GleamStdlibHostProfile, GleamStdlibRunState};
use crate::{HostProviderModule, HostRegistrationError};
use ecow::EcoString;
use geam_core::provider::Call;

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

#[geam_macros::module(
    path = "gleam/io",
    crate_path = geam_core,
    profile = crate::GleamStdlibHostProfile,
    component = crate::Component<Profile::Io>,
)]
mod provider {
    use super::{Call, EcoString, GleamStdlibRunState, function};

    #[geam_macros::function(profile = Profile)]
    fn print(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        text: EcoString,
    ) -> () {
        function::print(call.state_mut().io_sink(), text)
    }

    #[geam_macros::function(profile = Profile)]
    fn print_error(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        text: EcoString,
    ) -> () {
        function::print_error(call.state_mut().io_sink(), text)
    }

    #[geam_macros::function(profile = Profile)]
    fn println(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        text: EcoString,
    ) -> () {
        function::println(call.state_mut().io_sink(), text)
    }

    #[geam_macros::function(profile = Profile)]
    fn println_error(
        #[geam_macros::call] call: &mut Call<GleamStdlibRunState<Profile::Io>>,
        text: EcoString,
    ) -> () {
        function::println_error(call.state_mut().io_sink(), text)
    }
}

pub(super) fn host_provider<Profile>() -> Result<HostProviderModule<Profile>, HostRegistrationError>
where
    Profile: GleamStdlibHostProfile,
{
    provider::__geam_module::<Profile>()
}

#[cfg(test)]
mod tests {
    use super::{IoOutput, IoSink, IoStream, host_provider};
    use crate::GleamStdlibProfile;

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
            assert!(function.scheme().parameters().is_empty());
            assert_eq!(
                function.type_().argument_types(),
                [crate::ValueType::String],
            );
            assert_eq!(function.type_().return_(), &crate::ValueType::Nil);
        }
    }
}
