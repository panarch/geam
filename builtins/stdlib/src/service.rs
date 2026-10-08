//! Producer-owned standard-library value construction and consumption.
//!
//! Construction uses the calling function's explicit permissions. Consumption
//! uses retained typed inputs and explicit owned reads. The caller composes the
//! standard-library component; it does not implement its storage or binding.

pub use crate::bytes_tree::{BytesTreeInput, BytesTreeOutput};
pub use crate::dict::dict_from_entries;
pub use crate::dynamic::with_native_dynamic;

mod decoder;
pub use decoder::Decoder;
pub(crate) use decoder::DecoderSchema;
