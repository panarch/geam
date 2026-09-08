//! Statically typed Rust calls into Gleam, including Geam's explicit Future API.

pub use geam_core::embedding::{
    AsyncCallError, BigInt, BindingError, BitArrayValue, CallError, Completed, EcoString,
    ExecutionGuard, ExecutionScope, Function, FunctionDeclaration, HostedModule,
    HostedModuleBindings, HostedModuleBuilder, HostedProject, HostedProjectError, InputShape, Iter,
    List, Module, ModuleBindings, ModuleBuilder, ObservationError, Project, ReadValue,
    SharedExecutionError, SharedList, SourceType, TransferHostedProject, WorkModule,
    WorkModuleBindings, WorkModuleBuilder, with_execution_scope,
};
#[cfg(feature = "geam-runtime-api")]
pub use geam_runtime_api::embedding::{Future, FutureType};
