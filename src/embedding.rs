//! Statically typed Rust calls into Gleam, including Geam's explicit Future API.
//!
//! Generated bindings can load current source through `project()` and `bind()`,
//! or load a prepared program through `load()`. Select the generated interfaces
//! with `generate = "dynamic"`, `"prepared"`, or `"both"` in
//! `[package.metadata.geam.embedding]`; the default is dynamic.
//!
//! Run `geam embedding sync` after changing inputs. Prepared generation includes
//! compiler-visible execution data alongside the typed bindings, so Cargo builds
//! and the resulting executable do not need the original Gleam project. Each
//! load creates a fresh module and its owner-bound handles. Capabilities, mutable
//! state, Echo and the execution host remain caller-owned.

#[cfg(feature = "geam-builtin")]
pub use geam_builtin::embedding::{Future, FutureType};
pub use geam_core::embedding::{
    BigInt, BindingError, BitArrayValue, CallError, Completed, Custom, CustomType, EcoString,
    ExecutionScope, External, ExternalType, Function, FunctionDeclaration, HostedModule,
    HostedModuleBindings, HostedModuleBuilder, HostedProject, HostedProjectError, InputShape, Iter,
    List, Module, ModuleBindings, ModuleBuilder, NamedTypeSchema, ObservationError, PreparedError,
    PreparedHostedModule, PreparedHostedModuleBindings, PreparedModule, PreparedModuleBindings,
    Project, ReadValue, SharedExecutionError, SharedList, SourceType, StringValue,
};
