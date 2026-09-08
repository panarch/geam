//! Typed Rust views of `geam/future.Future`.

/// The source declaration `geam/future.Future(value)`.
pub type FutureType<Value> = geam_core::embedding::FutureType<Value, crate::HostFutureSchema>;

/// One shared operation belonging to its original attached execution.
pub type Future<'scope, Value> =
    geam_core::embedding::Future<'scope, Value, crate::HostFutureSchema>;
