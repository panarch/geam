use camino::Utf8PathBuf;
use clap::{Args, Parser, Subcommand};

#[derive(Debug, PartialEq, Eq, Parser)]
#[command(name = "geam", version, about = "Run Gleam projects through Geam")]
pub(super) struct Cli {
    #[command(subcommand)]
    pub(super) command: Command,
}

#[derive(Debug, PartialEq, Eq, Subcommand)]
pub(super) enum Command {
    Embedding(Embedding),
    Prepare(EntryCommand),
    Run(RunCommand),
    Provider(Provider),
}

#[derive(Debug, PartialEq, Eq, Args)]
pub(super) struct Embedding {
    #[command(subcommand)]
    pub(super) command: EmbeddingCommand,
}

#[derive(Debug, PartialEq, Eq, Subcommand)]
pub(super) enum EmbeddingCommand {
    /// Initialize a Gleam library and bindings for the current Cargo package.
    Init,
    /// Check that the project's dependencies and generated bindings agree.
    Check,
    /// Prepare dependencies and synchronize Rust bindings with Gleam source.
    Sync,
}

#[derive(Debug, PartialEq, Eq, Args)]
pub(super) struct EntryCommand {
    #[arg(short = 'm', long)]
    pub(super) module: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Args)]
pub(super) struct RunCommand {
    #[arg(short = 'm', long)]
    pub(super) module: Option<String>,

    #[arg(long = "provider-config", value_name = "GLEAM_PACKAGE=PATH")]
    pub(super) provider_configs: Vec<String>,
}

#[derive(Debug, PartialEq, Eq, Args)]
pub(super) struct Provider {
    #[command(subcommand)]
    pub(super) command: ProviderCommand,
}

#[derive(Debug, PartialEq, Eq, Subcommand)]
pub(super) enum ProviderCommand {
    Add(AddProvider),
    List,
    Remove(RemoveProvider),
}

#[derive(Debug, PartialEq, Eq, Args)]
pub(super) struct AddProvider {
    #[arg(
        value_name = "CRATE[@VERSION]",
        required_unless_present_any = ["path", "git"],
        conflicts_with_all = ["path", "git"]
    )]
    pub(super) crate_spec: Option<String>,

    #[arg(long, value_name = "PATH", conflicts_with = "git")]
    pub(super) path: Option<Utf8PathBuf>,

    #[arg(long, value_name = "URL", conflicts_with = "path")]
    pub(super) git: Option<String>,

    #[arg(long, value_name = "COMMIT", requires = "git")]
    pub(super) rev: Option<String>,

    #[arg(long, value_name = "CRATE", conflicts_with = "crate_spec")]
    pub(super) package: Option<String>,
}

#[derive(Debug, PartialEq, Eq, Args)]
pub(super) struct RemoveProvider {
    #[arg(value_name = "GLEAM_PACKAGE")]
    pub(super) gleam_package: String,
}

#[cfg(test)]
mod tests {
    use super::{
        AddProvider, Cli, Command, Embedding, EmbeddingCommand, EntryCommand, Provider,
        ProviderCommand, RemoveProvider, RunCommand,
    };
    use camino::Utf8PathBuf;
    use clap::{CommandFactory, Parser};

    #[test]
    fn exposes_complete_command_help() {
        Cli::command().debug_assert();
    }

    #[test]
    fn parses_entry_and_configuration_options() {
        let prepare = Cli::try_parse_from(["geam", "prepare", "--module", "worker"])
            .expect("prepare command should parse");
        assert_eq!(
            prepare,
            Cli {
                command: Command::Prepare(EntryCommand {
                    module: Some("worker".to_owned()),
                }),
            },
        );

        let run = Cli::try_parse_from([
            "geam",
            "run",
            "-m",
            "worker",
            "--provider-config",
            "images=config.toml",
            "--provider-config",
            "search=search.toml",
        ])
        .expect("run command should parse");
        assert_eq!(
            run,
            Cli {
                command: Command::Run(RunCommand {
                    module: Some("worker".to_owned()),
                    provider_configs: vec![
                        "images=config.toml".to_owned(),
                        "search=search.toml".to_owned(),
                    ],
                }),
            },
        );
    }

    #[test]
    fn parses_parameterless_embedding_commands() {
        assert_eq!(
            Cli::try_parse_from(["geam", "embedding", "init"])
                .expect("embedding init command should parse"),
            Cli {
                command: Command::Embedding(Embedding {
                    command: EmbeddingCommand::Init,
                }),
            },
        );
        let check = Cli::try_parse_from(["geam", "embedding", "check"])
            .expect("embedding check command should parse");
        assert_eq!(
            check,
            Cli {
                command: Command::Embedding(Embedding {
                    command: EmbeddingCommand::Check,
                }),
            },
        );

        let nearest = Cli::try_parse_from(["geam", "embedding", "sync"])
            .expect("embedding sync command should parse");
        assert_eq!(
            nearest,
            Cli {
                command: Command::Embedding(Embedding {
                    command: EmbeddingCommand::Sync,
                }),
            },
        );

        for operation in ["init", "sync", "check"] {
            assert_eq!(
                Cli::try_parse_from([
                    "geam",
                    "embedding",
                    operation,
                    "--manifest-path",
                    "Cargo.toml",
                ])
                .expect_err("embedding uses the current Cargo package")
                .kind(),
                clap::error::ErrorKind::UnknownArgument,
            );
        }
    }

    #[test]
    fn parses_registry_path_git_list_and_remove_provider_commands() {
        let registry = Cli::try_parse_from(["geam", "provider", "add", "geam-images@1.2.3"])
            .expect("registry provider should parse");
        assert_eq!(
            registry,
            Cli {
                command: Command::Provider(Provider {
                    command: ProviderCommand::Add(AddProvider {
                        crate_spec: Some("geam-images@1.2.3".to_owned()),
                        path: None,
                        git: None,
                        rev: None,
                        package: None,
                    }),
                }),
            },
        );

        let path = Cli::try_parse_from([
            "geam",
            "provider",
            "add",
            "--path",
            "../provider",
            "--package",
            "geam-images",
        ])
        .expect("path provider should parse");
        assert_eq!(
            path,
            Cli {
                command: Command::Provider(Provider {
                    command: ProviderCommand::Add(AddProvider {
                        crate_spec: None,
                        path: Some(Utf8PathBuf::from("../provider")),
                        git: None,
                        rev: None,
                        package: Some("geam-images".to_owned()),
                    }),
                }),
            },
        );

        let git = Cli::try_parse_from([
            "geam",
            "provider",
            "add",
            "--git",
            "https://example.com/provider.git",
            "--rev",
            "abc123",
            "--package",
            "geam-images",
        ])
        .expect("Git provider should parse");
        assert_eq!(
            git,
            Cli {
                command: Command::Provider(Provider {
                    command: ProviderCommand::Add(AddProvider {
                        crate_spec: None,
                        path: None,
                        git: Some("https://example.com/provider.git".to_owned()),
                        rev: Some("abc123".to_owned()),
                        package: Some("geam-images".to_owned()),
                    }),
                }),
            },
        );

        let list =
            Cli::try_parse_from(["geam", "provider", "list"]).expect("list command should parse");
        assert_eq!(
            list,
            Cli {
                command: Command::Provider(Provider {
                    command: ProviderCommand::List,
                }),
            },
        );

        let remove = Cli::try_parse_from(["geam", "provider", "remove", "images"])
            .expect("remove command should parse");
        assert_eq!(
            remove,
            Cli {
                command: Command::Provider(Provider {
                    command: ProviderCommand::Remove(RemoveProvider {
                        gleam_package: "images".to_owned(),
                    }),
                }),
            },
        );
    }

    #[test]
    fn rejects_ambiguous_provider_sources_and_options() {
        for (arguments, expected) in [
            (
                vec!["geam", "provider", "add"],
                clap::error::ErrorKind::MissingRequiredArgument,
            ),
            (
                vec![
                    "geam",
                    "provider",
                    "add",
                    "geam-images",
                    "--path",
                    "provider",
                ],
                clap::error::ErrorKind::ArgumentConflict,
            ),
            (
                vec!["geam", "provider", "add", "--rev", "abc"],
                clap::error::ErrorKind::MissingRequiredArgument,
            ),
            (
                vec![
                    "geam",
                    "provider",
                    "add",
                    "geam-images",
                    "--package",
                    "geam-images",
                ],
                clap::error::ErrorKind::ArgumentConflict,
            ),
        ] {
            assert_eq!(
                Cli::try_parse_from(arguments)
                    .expect_err("invalid provider command should be rejected")
                    .kind(),
                expected,
            );
        }
    }
}
