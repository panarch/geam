mod bundle;
mod cases;
mod environment;
mod evidence;
mod prepare;
mod process;
mod results;
mod run;

use bundle::Target;
use clap::{Args, Parser, Subcommand};
use evidence::write_json_new;
use run::Mode;
use std::error::Error;
use std::io;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

#[derive(Debug, Parser)]
#[command(
    name = "geam-bench",
    version,
    about = "Prepare, run and analyze shared Gleam workloads"
)]
struct Arguments {
    #[command(subcommand)]
    command: Operation,
}

#[derive(Debug, Subcommand)]
enum Operation {
    /// Build a relocatable execution bundle in a new directory.
    Prepare(Preparation),
    /// Execute selected targets from a prepared bundle, without build tools.
    Run {
        #[arg(long)]
        bundle: PathBuf,
        #[arg(
            long,
            value_enum,
            value_delimiter = ',',
            default_value = "geam,erlang,javascript"
        )]
        targets: Vec<Target>,
        #[command(flatten)]
        execution: Execution,
    },
    /// Run matched Geam bundles in four balanced paired rounds.
    Compare {
        #[arg(long)]
        baseline: PathBuf,
        #[arg(long)]
        candidate: PathBuf,
        #[command(flatten)]
        execution: Execution,
    },
    /// Revalidate a completed run and reconstruct reports without any runtime.
    Analyze {
        #[arg(long)]
        input: PathBuf,
        #[arg(long)]
        output: PathBuf,
    },
    /// List the maintained workload/size identities and independent checksums.
    Cases,
}

#[derive(Debug, Args)]
struct Preparation {
    /// Reuse an exclusively locked build workspace owned by this runner.
    #[arg(long)]
    build_directory: Option<PathBuf>,

    #[arg(long)]
    checkout: PathBuf,
    #[arg(long, default_value = "benchmarks")]
    suite: PathBuf,
    /// Checkout CLI used for Geam preparation (required when Geam is selected).
    #[arg(long)]
    geam: Option<PathBuf>,
    #[arg(long)]
    output: PathBuf,
    #[arg(
        long,
        value_enum,
        value_delimiter = ',',
        default_value = "geam,erlang,javascript"
    )]
    targets: Vec<Target>,
}

#[derive(Debug, Args)]
struct Execution {
    #[arg(long)]
    output: PathBuf,
    #[arg(long, value_enum, default_value = "smoke")]
    mode: Mode,
    /// Human-readable measurement-host description; required for baseline runs.
    #[arg(long, required_if_eq("mode", "baseline"))]
    machine: Option<String>,
    /// Exact workload/size identity; repeat or separate with commas. Default: all.
    #[arg(long = "case", value_delimiter = ',')]
    cases: Vec<String>,
    #[arg(long, default_value_t=240, value_parser=clap::value_parser!(u64).range(1..))]
    timeout_seconds: u64,
}

fn main() -> Result<()> {
    let arguments = Arguments::parse();
    let cancelled = Arc::new(AtomicBool::new(false));
    dispatch(
        arguments,
        &cancelled,
        [signal_hook::consts::SIGINT, signal_hook::consts::SIGTERM]
            .into_iter()
            .map(|signal| signal_hook::flag::register(signal, Arc::clone(&cancelled))),
    )
}

fn dispatch(
    arguments: Arguments,
    cancelled: &AtomicBool,
    signals: impl IntoIterator<Item = io::Result<signal_hook::SigId>>,
) -> Result<()> {
    for registration in signals {
        registration?;
    }
    match arguments.command {
        Operation::Prepare(arguments) => prepare::execute(arguments, cancelled),
        Operation::Run {
            bundle,
            targets,
            execution,
        } => run::execute(&bundle, None, targets, execution, cancelled),
        Operation::Compare {
            baseline,
            candidate,
            execution,
        } => run::execute(
            &baseline,
            Some(&candidate),
            vec![Target::Geam],
            execution,
            cancelled,
        ),
        Operation::Analyze { input, output } => run::analyze(&input, &output),
        Operation::Cases => {
            for case in cases::suite(None) {
                println!(
                    "{}/{}\t{}",
                    case.workload.name(),
                    case.size,
                    case.expected_checksum()
                );
            }
            Ok(())
        }
    }
}

fn new_directory(path: &Path) -> Result<PathBuf> {
    if let Some(parent) = path.parent().filter(|path| !path.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    std::fs::create_dir(path)?;
    path.canonicalize().map_err(Into::into)
}

fn invalid(message: impl Into<String>) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.into())
}

#[cfg(test)]
mod tests {
    use super::Arguments;
    use clap::Parser;

    #[test]
    fn command_contract_requires_inputs_and_a_baseline_host_description() {
        for args in [
            vec!["geam-bench"],
            vec![
                "geam-bench",
                "run",
                "--bundle",
                "one",
                "--output",
                "out",
                "--mode",
                "baseline",
            ],
            vec![
                "geam-bench",
                "run",
                "--bundle",
                "one",
                "--output",
                "out",
                "--timeout-seconds",
                "0",
            ],
            vec!["geam-bench", "prepare", "--output", "out"],
            vec![
                "geam-bench",
                "compare",
                "--baseline",
                "one",
                "--output",
                "out",
            ],
        ] {
            assert!(Arguments::try_parse_from(args).is_err());
        }
        for args in [
            vec!["geam-bench", "run", "--bundle", "one", "--output", "out"],
            vec![
                "geam-bench",
                "run",
                "--bundle",
                "one",
                "--output",
                "out",
                "--mode",
                "baseline",
                "--machine",
                "dedicated host",
            ],
            vec![
                "geam-bench",
                "prepare",
                "--checkout",
                ".",
                "--output",
                "out",
                "--geam",
                "geam",
            ],
            vec![
                "geam-bench",
                "compare",
                "--baseline",
                "one",
                "--candidate",
                "two",
                "--output",
                "out",
            ],
            vec!["geam-bench", "analyze", "--input", "one", "--output", "out"],
            vec!["geam-bench", "cases"],
        ] {
            assert!(Arguments::try_parse_from(args).is_ok());
        }
    }

    #[test]
    fn signal_registration_must_succeed_before_any_operation() {
        use std::sync::{Arc, atomic::AtomicBool};
        let cancelled = Arc::new(AtomicBool::new(false));
        let registration =
            signal_hook::flag::register(signal_hook::consts::SIGUSR1, Arc::clone(&cancelled))
                .unwrap();
        for (args, success) in [
            (vec!["geam-bench", "cases"], true),
            (
                vec![
                    "geam-bench",
                    "prepare",
                    "--checkout",
                    "/geam-bench-missing",
                    "--output",
                    "/geam-bench-missing",
                ],
                false,
            ),
            (
                vec![
                    "geam-bench",
                    "run",
                    "--bundle",
                    "/geam-bench-missing",
                    "--output",
                    "/geam-bench-missing",
                ],
                false,
            ),
            (
                vec![
                    "geam-bench",
                    "compare",
                    "--baseline",
                    "/geam-bench-missing",
                    "--candidate",
                    "/geam-bench-missing",
                    "--output",
                    "/geam-bench-missing",
                ],
                false,
            ),
            (
                vec![
                    "geam-bench",
                    "analyze",
                    "--input",
                    "/geam-bench-missing",
                    "--output",
                    "/geam-bench-missing",
                ],
                false,
            ),
        ] {
            let result = super::dispatch(
                Arguments::try_parse_from(args.clone()).unwrap(),
                &cancelled,
                [Ok(registration)],
            );
            assert_eq!(result.is_ok(), success, "{args:?}");
            let result = super::dispatch(
                Arguments::try_parse_from(args).unwrap(),
                &cancelled,
                [Err(std::io::Error::other("registration failed"))],
            );
            assert_eq!(result.unwrap_err().to_string(), "registration failed");
        }
        assert!(signal_hook::low_level::unregister(registration));
    }
    #[test]
    fn fresh_directories_accept_bare_and_nested_paths_and_reject_collisions() {
        let directory = tempfile::Builder::new()
            .prefix("geam-bench-test-")
            .tempdir_in(".")
            .unwrap();
        let name = directory.path().file_name().unwrap().to_owned();
        std::fs::remove_dir(directory.path()).unwrap();
        let created = super::new_directory(std::path::Path::new(&name)).unwrap();
        assert_eq!(created, directory.path().canonicalize().unwrap());
        assert!(super::new_directory(std::path::Path::new(&name)).is_err());
        let nested = created.join("nested/output");
        assert_eq!(super::new_directory(&nested).unwrap(), nested);
        let blocked = created.join("blocked");
        std::fs::write(&blocked, b"file").unwrap();
        assert!(super::new_directory(&blocked.join("child")).is_err());
    }
}
