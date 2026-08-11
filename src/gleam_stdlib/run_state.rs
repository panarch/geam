use ecow::EcoString;
use rand::rngs::{ChaCha12Rng, SysRng};
use rand::{Rng, SeedableRng, TryRng};
use std::fmt::{self, Display, Formatter};

use super::IoOutput;

/// Caller-owned mutable state used by the official Gleam standard library.
pub struct GleamStdlibRunState<Io = Vec<IoOutput>> {
    random: ChaCha12Rng,
    io: Io,
}

/// Failure to initialize standard-library run state from system entropy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GleamStdlibRunStateError {
    reason: EcoString,
}

impl GleamStdlibRunState {
    /// Creates reproducible standard-library state from an explicit seed.
    pub fn from_seed(seed: [u8; 32]) -> Self {
        Self::from_seed_with_io(seed, Vec::new())
    }

    /// Creates standard-library state from the operating system random source.
    pub fn try_from_entropy() -> Result<Self, GleamStdlibRunStateError> {
        Self::try_from_entropy_with_io(Vec::new())
    }

    /// Returns the standard-library IO events collected by this run state.
    pub fn io_outputs(&self) -> &[IoOutput] {
        &self.io
    }

    /// Takes all collected standard-library IO events, leaving the state empty.
    pub fn take_io_outputs(&mut self) -> Vec<IoOutput> {
        std::mem::take(&mut self.io)
    }
}

impl<Io> GleamStdlibRunState<Io> {
    /// Creates reproducible standard-library state with a caller-owned IO sink.
    pub fn from_seed_with_io(seed: [u8; 32], io: Io) -> Self {
        Self {
            random: ChaCha12Rng::from_seed(seed),
            io,
        }
    }

    /// Creates standard-library state from system entropy with a caller-owned IO sink.
    pub fn try_from_entropy_with_io(io: Io) -> Result<Self, GleamStdlibRunStateError> {
        Self::try_from_seed_source(io, |seed| SysRng.try_fill_bytes(seed))
    }

    pub(super) fn random_float(&mut self) -> f64 {
        const SCALE: f64 = 1.0 / ((1u64 << 53) as f64);

        ((self.random.next_u64() >> 11) as f64) * SCALE
    }

    pub(super) fn io_sink(&mut self) -> &mut Io {
        &mut self.io
    }

    fn try_from_seed_source<Error>(
        io: Io,
        fill: impl FnOnce(&mut [u8; 32]) -> Result<(), Error>,
    ) -> Result<Self, GleamStdlibRunStateError>
    where
        Error: Display,
    {
        let mut seed = [0; 32];
        fill(&mut seed)
            .map(|()| Self::from_seed_with_io(seed, io))
            .map_err(|error| GleamStdlibRunStateError {
                reason: error.to_string().into(),
            })
    }
}

impl GleamStdlibRunStateError {
    /// Returns the owned entropy-source failure reason.
    pub fn reason(&self) -> &str {
        &self.reason
    }
}

impl Display for GleamStdlibRunStateError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "could not initialize Gleam standard-library random state: {}",
            self.reason
        )
    }
}

impl std::error::Error for GleamStdlibRunStateError {}

#[cfg(test)]
mod tests {
    use super::{GleamStdlibRunState, GleamStdlibRunStateError};
    use crate::gleam_stdlib::{IoOutput, IoSink, IoStream};
    use std::fmt::{self, Display, Formatter};

    #[derive(Debug)]
    struct RejectedEntropyError;

    impl Display for RejectedEntropyError {
        fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
            formatter.write_str("entropy unavailable")
        }
    }

    impl std::error::Error for RejectedEntropyError {}

    #[test]
    fn explicit_seeds_are_reproducible_and_advance_independently() {
        let mut first = GleamStdlibRunState::from_seed([7; 32]);
        let mut second = GleamStdlibRunState::from_seed([7; 32]);

        let first_value = first.random_float();
        assert_eq!(first_value, second.random_float());
        let first_next = first.random_float();
        assert_ne!(first_next, first_value);
        assert_eq!(first_next, second.random_float());
        assert!((0.0..1.0).contains(&first_value));
        assert!(first.io_outputs().is_empty());
        assert!(second.io_outputs().is_empty());
    }

    #[test]
    fn io_outputs_accumulate_and_can_be_taken() {
        let mut state = GleamStdlibRunState::from_seed([8; 32]);
        state
            .io_sink()
            .emit(IoOutput::new(IoStream::Stdout, "first".into()));
        state
            .io_sink()
            .emit(IoOutput::new(IoStream::Stderr, "second".into()));

        assert_eq!(
            state
                .io_outputs()
                .iter()
                .map(|output| (output.stream(), output.text().as_str()))
                .collect::<Vec<_>>(),
            [(IoStream::Stdout, "first"), (IoStream::Stderr, "second")],
        );
        assert_eq!(state.take_io_outputs().len(), 2);
        assert!(state.io_outputs().is_empty());
    }

    #[test]
    fn entropy_failure_preserves_an_owned_reason() {
        let error = GleamStdlibRunState::try_from_seed_source(Vec::<IoOutput>::new(), |_| {
            Err(RejectedEntropyError)
        })
        .err()
        .expect("rejected entropy should fail");

        assert_eq!(error.reason(), "entropy unavailable");
        assert_eq!(
            error,
            GleamStdlibRunStateError {
                reason: "entropy unavailable".into(),
            },
        );
        assert_eq!(
            error.to_string(),
            "could not initialize Gleam standard-library random state: entropy unavailable",
        );
    }

    #[test]
    fn system_entropy_constructs_an_independent_state() {
        let mut state = GleamStdlibRunState::try_from_entropy()
            .expect("system entropy should be available in the test environment");

        assert!((0.0..1.0).contains(&state.random_float()));
        assert!(state.io_outputs().is_empty());
    }

    #[test]
    fn caller_owned_io_is_preserved_by_seeded_and_entropy_construction() {
        let state = GleamStdlibRunState::from_seed_with_io([9; 32], String::from("sink"));
        assert_eq!(state.io, "sink");

        let state = GleamStdlibRunState::try_from_entropy_with_io(String::from("entropy sink"))
            .expect("system entropy should initialize caller-owned IO state");
        assert_eq!(state.io, "entropy sink");
    }
}
