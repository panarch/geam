use camino::Utf8PathBuf;
use ecow::EcoString;
use gleam_compiler_core::parse::error::ParseError;
use gleam_compiler_core::type_::Error as TypeError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum FrontendError {
    #[error("failed to parse Gleam module {path}")]
    Parse {
        path: Utf8PathBuf,
        error: Box<ParseError>,
    },

    #[error("failed to analyse Gleam module")]
    Analyse { errors: Vec<TypeError> },

    #[error("package {package} was supplied more than once")]
    DuplicatePackage { package: EcoString },

    #[error(
        "module {module} was supplied by both package {first_package} and package {second_package}"
    )]
    DuplicateModule {
        module: EcoString,
        first_package: EcoString,
        first_path: Utf8PathBuf,
        second_package: EcoString,
        second_path: Utf8PathBuf,
    },

    #[error(
        "module {module} was supplied as source by package {source_package} and as a host module by package {host_package}"
    )]
    SourceHostModuleCollision {
        module: EcoString,
        source_package: EcoString,
        source_path: Utf8PathBuf,
        host_package: EcoString,
    },

    #[error("root package {package} was not supplied")]
    MissingRootPackage { package: EcoString },

    #[error("root module {module} was not supplied by package {package}")]
    MissingRootModule {
        package: EcoString,
        module: EcoString,
    },

    #[error("module import cycle: {modules:?}")]
    ImportCycle { modules: Vec<EcoString> },
}
