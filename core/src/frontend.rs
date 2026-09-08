mod error;
mod module;
mod program;
mod project;

pub use error::FrontendError;
pub use module::{ModuleSource, PackageSource};
pub(crate) use program::HostedTypedProgramModule;
pub use program::{
    HostedTypedProgram, TransferHostedTypedProgram, TypedProgram, compile_typed_host_program,
    compile_typed_module, compile_typed_package_program, compile_typed_program,
    compile_typed_transfer_host_program,
};
pub use project::{
    ProjectError, compile_typed_host_project, compile_typed_project,
    compile_typed_transfer_host_project,
};
