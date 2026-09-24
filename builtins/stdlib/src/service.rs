//! Producer-owned standard-library value construction for other providers.
//!
//! Services use the calling function's explicit construction permissions. The
//! caller composes the standard-library component; it does not implement that
//! component's external storage or binding.

pub use crate::dict::dict_from_entries;
