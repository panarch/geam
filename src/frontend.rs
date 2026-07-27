mod error;
mod module;
mod program;
mod project;

pub use error::FrontendError;
pub use module::{ModuleSource, PackageSource};
pub use program::{
    TypedProgram, compile_typed_module, compile_typed_package_program, compile_typed_program,
};
pub use project::{ProjectError, compile_typed_project};
