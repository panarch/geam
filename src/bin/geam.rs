#[path = "geam/command.rs"]
mod command;
#[path = "geam/error.rs"]
mod error;
#[path = "geam/process.rs"]
mod process;
#[path = "geam/project.rs"]
mod project;
#[path = "geam/provider.rs"]
mod provider;
#[path = "geam/runner.rs"]
mod runner;
#[path = "geam/standalone.rs"]
mod standalone;

use clap::Parser;
use command::{Cli, Command, ProviderCommand};
use error::CliError;
use std::env;
use std::process::ExitCode;

fn main() -> ExitCode {
    let cli = Cli::parse();
    let result = env::current_dir()
        .map_err(CliError::CurrentDirectory)
        .and_then(project::into_utf8_path)
        .and_then(|current_directory| run(cli, current_directory));
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("geam: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run(cli: Cli, current_directory: camino::Utf8PathBuf) -> Result<(), CliError> {
    let project_root = project::find_project_root(&current_directory)?;
    match cli.command {
        Command::Prepare(command) => {
            let module = project::entry_module(&project_root, command.module)?;
            standalone::prepare(&project_root, module)
        }
        Command::Run(command) => {
            let module = project::entry_module(&project_root, command.module)?;
            standalone::run(
                &project_root,
                &current_directory,
                module,
                command.provider_configs,
            )
        }
        Command::Provider(command) => match command.command {
            ProviderCommand::Add(command) => {
                provider::add(&project_root, current_directory.as_std_path(), command)
            }
            ProviderCommand::Remove(command) => provider::remove(&project_root, command),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{Cli, run};
    use crate::error::CliError;
    use camino::Utf8PathBuf;
    use clap::Parser;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn preserves_entry_resolution_failures_for_prepare_and_run() {
        let project = tempdir().expect("temporary project should be created");
        fs::write(project.path().join("gleam.toml"), "invalid")
            .expect("invalid config should be written");

        for arguments in [vec!["geam", "prepare"], vec!["geam", "run"]] {
            let error = run(
                Cli::try_parse_from(arguments).expect("command should parse"),
                Utf8PathBuf::from_path_buf(project.path().to_path_buf())
                    .expect("temporary path should be valid UTF-8"),
            )
            .expect_err("entry resolution should fail");
            assert_eq!(
                std::mem::discriminant(&error),
                std::mem::discriminant(&CliError::InvalidToml {
                    kind: "Gleam package config",
                    path: Utf8PathBuf::new(),
                    reason: String::new(),
                }),
            );
        }
    }

    #[test]
    fn preserves_project_compilation_failures_for_prepare_and_run() {
        let project = tempdir().expect("temporary project should be created");
        fs::create_dir(project.path().join("src")).expect("source directory should be created");
        fs::write(
            project.path().join("gleam.toml"),
            "name = \"application\"\nversion = \"1.0.0\"\n",
        )
        .expect("package config should be written");
        fs::write(
            project.path().join("manifest.toml"),
            "packages = []\n[requirements]\n",
        )
        .expect("manifest should be written");
        fs::write(
            project.path().join("src/application.gleam"),
            "pub fn main() { 1 }\n",
        )
        .expect("source should be written");

        for arguments in [
            vec!["geam", "prepare", "--module", "missing"],
            vec!["geam", "run", "--module", "missing"],
        ] {
            let error = run(
                Cli::try_parse_from(arguments).expect("command should parse"),
                Utf8PathBuf::from_path_buf(project.path().to_path_buf())
                    .expect("temporary path should be valid UTF-8"),
            )
            .expect_err("missing entry module should fail");
            assert_eq!(
                std::mem::discriminant(&error),
                std::mem::discriminant(&CliError::Project(geam::ProjectError::InvalidManifest {
                    path: Utf8PathBuf::new(),
                    reason: String::new(),
                },)),
            );
        }
    }
}
