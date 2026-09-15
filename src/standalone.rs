#[path = "standalone/configuration.rs"]
mod configuration;
#[path = "standalone/output.rs"]
mod output;

pub use configuration::{RuntimeInputs, read_provider_configuration};
pub use output::{CliIoSink, SharedOutput};
