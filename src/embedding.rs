//! Statically typed Rust calls into Gleam, including Geam's explicit Future API.

#[cfg(feature = "geam-builtin")]
pub use geam_builtin::embedding::{Future, FutureType};
pub use geam_core::embedding::{
    BigInt, BindingError, BitArrayValue, CallError, Completed, Custom, CustomType, EcoString,
    ExecutionScope, External, ExternalType, Function, FunctionDeclaration, HostedModule,
    HostedModuleBindings, HostedModuleBuilder, HostedProject, HostedProjectError, InputShape, Iter,
    List, Module, ModuleBindings, ModuleBuilder, NamedTypeSchema, ObservationError, Project,
    ReadValue, SharedExecutionError, SharedList, SourceType,
};
