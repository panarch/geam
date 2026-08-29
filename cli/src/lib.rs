mod builtin;
mod cargo;
mod command;
mod embedding;
mod error;
mod process;
mod project;
mod provider;
mod runner;
mod standalone;

use clap::Parser;
use command::{
    Cli, Command, EmbeddingCommand, EntryCommand, Provider, ProviderCommand, RunCommand,
};
use error::CliError;
use std::env;
use std::process::ExitCode;

pub fn run() -> ExitCode {
    let cli = Cli::parse();
    let result = env::current_dir()
        .map_err(CliError::CurrentDirectory)
        .and_then(project::into_utf8_path)
        .and_then(|current_directory| run_command(cli, current_directory));
    match result {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("geam: {error}");
            ExitCode::FAILURE
        }
    }
}

fn run_command(cli: Cli, current_directory: camino::Utf8PathBuf) -> Result<(), CliError> {
    let command = match cli.command {
        Command::Embedding(command) => match command.command {
            EmbeddingCommand::Check(target) => {
                return embedding::check(&current_directory, target);
            }
            EmbeddingCommand::Sync(target) => return embedding::sync(&current_directory, target),
        },
        Command::Prepare(command) => ProjectCommand::Prepare(command),
        Command::Run(command) => ProjectCommand::Run(command),
        Command::Provider(command) => ProjectCommand::Provider(command),
    };
    run_project_command(command, current_directory)
}

enum ProjectCommand {
    Prepare(EntryCommand),
    Run(RunCommand),
    Provider(Provider),
}

fn run_project_command(
    command: ProjectCommand,
    current_directory: camino::Utf8PathBuf,
) -> Result<(), CliError> {
    let project_root = project::find_project_root(&current_directory)?;
    match command {
        ProjectCommand::Prepare(command) => project::entry_module(&project_root, command.module)
            .and_then(|module| standalone::prepare(&project_root, module)),
        ProjectCommand::Run(command) => project::entry_module(&project_root, command.module)
            .and_then(|module| {
                standalone::run(
                    &project_root,
                    &current_directory,
                    module,
                    command.provider_configs,
                )
            }),
        ProjectCommand::Provider(command) => match command.command {
            ProviderCommand::Add(command) => {
                provider::add(&project_root, current_directory.as_std_path(), command)
            }
            ProviderCommand::List => provider::list(&project_root),
            ProviderCommand::Remove(command) => provider::remove(&project_root, command),
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{Cli, run_command};
    use crate::error::CliError;
    use camino::Utf8PathBuf;
    use clap::Parser;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn routes_embedding_before_gleam_project_discovery() {
        let directory = tempdir().expect("temporary directory should be created");
        let root = Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");
        for operation in ["check", "sync"] {
            let error = run_command(
                Cli::try_parse_from([
                    "geam",
                    "embedding",
                    operation,
                    "--manifest-path",
                    "missing/Cargo.toml",
                ])
                .expect("embedding command should parse"),
                root.clone(),
            )
            .expect_err("missing explicit Cargo manifest should fail");

            assert!(matches!(
                error,
                CliError::FileRead { path, error }
                    if path == root.join("missing/Cargo.toml")
                        && error.kind() == std::io::ErrorKind::NotFound
            ));
        }
    }

    #[test]
    fn preserves_entry_resolution_failures_for_prepare_and_run() {
        let project = tempdir().expect("temporary project should be created");
        fs::write(project.path().join("gleam.toml"), "invalid")
            .expect("invalid config should be written");
        let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");

        for arguments in [vec!["geam", "prepare"], vec!["geam", "run"]] {
            let error = run_command(
                Cli::try_parse_from(arguments).expect("command should parse"),
                root.clone(),
            )
            .expect_err("entry resolution should fail");
            assert!(matches!(
                error,
                CliError::InvalidToml { kind, path, reason }
                    if kind == "Gleam package config"
                        && path == root.join("gleam.toml")
                        && reason.contains("expected")
            ));
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
        let root = Utf8PathBuf::from_path_buf(project.path().to_path_buf())
            .expect("temporary path should be valid UTF-8");

        for arguments in [
            vec!["geam", "prepare", "--module", "missing"],
            vec!["geam", "run", "--module", "missing"],
        ] {
            let error = run_command(
                Cli::try_parse_from(arguments).expect("command should parse"),
                root.clone(),
            )
            .expect_err("missing entry module should fail");
            assert!(matches!(
                error,
                CliError::Project(geam_core::ProjectError::Frontend(
                    geam_core::FrontendError::MissingRootModule { package, module }
                )) if package == "application" && module == "missing"
            ));
        }
    }
}
