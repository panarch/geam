//! Built-in Rust implementations for the `geam` Gleam package.
//!
//! The `geam` Gleam package supplies ordinary source declarations. This crate
//! supplies their native implementations; generic execution remains in geam-core.

pub mod embedding;
pub mod future;

pub use future::{FutureComponent, HostFutureSchema, HostFutureStorage, HostFutureType};
