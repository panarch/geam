//! Statically typed Rust calls into Gleam, including Geam's explicit Future API.

#[cfg(feature = "geam-builtin")]
pub use geam_builtin::embedding::{Future, FutureType};
pub use geam_core::embedding::{
    BigInt, BindingError, BitArrayValue, CallError, Completed, EcoString, ExecutionGuard,
    ExecutionScope, Function, FunctionDeclaration, HostedModule, HostedModuleBindings,
    HostedModuleBuilder, HostedProject, HostedProjectError, InputShape, Iter, List, Module,
    ModuleBindings, ModuleBuilder, ObservationError, Project, ReadValue, SharedExecutionError,
    SharedList, SourceType, with_execution_scope,
};
