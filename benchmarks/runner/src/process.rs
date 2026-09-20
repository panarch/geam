use crate::{Result, invalid, write_json_new};
use nix::errno::Errno;
use nix::sys::signal::{Signal, killpg};
use nix::unistd::Pid;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io;
use std::os::unix::process::CommandExt;
use std::path::{Path, PathBuf};
use std::process::{Child, Command, ExitStatus, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant, SystemTime, SystemTimeError, UNIX_EPOCH};

/// A child process group owned until it is terminated and reaped.
struct Running<C: ProcessControl = Child> {
    child: C,
    state: GroupState,
}

#[derive(PartialEq)]
enum GroupState {
    Running,
    Terminated,
    Reaped,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Receipt {
    pub(super) program: String,
    pub(super) arguments: Vec<String>,
    pub(super) directory: String,
    pub(super) environment: BTreeMap<String, Option<String>>,
    pub(super) started_unix_ms: u128,
    pub(super) timeout_ms: u128,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Completion {
    pub(super) status: Status,
    pub(super) exit_code: Option<i32>,
    pub(super) elapsed_ms: u128,
}

#[derive(Debug, Deserialize, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(super) enum Status {
    Success,
    Failed,
    TimedOut,
    Cancelled,
    SpawnFailed,
}

pub(super) fn execute(
    command: &mut Command,
    output: &Path,
    timeout: Duration,
    cancelled: &AtomicBool,
) -> Result<Completion> {
    fs::create_dir(output)?;
    let receipt = Receipt::capture(
        command,
        command
            .get_current_dir()
            .map(Path::to_path_buf)
            .map(Ok)
            .unwrap_or_else(std::env::current_dir),
        SystemTime::now().duration_since(UNIX_EPOCH),
        timeout,
    );
    execute_recorded(command, output, timeout, cancelled, receipt)
}

fn execute_recorded(
    command: &mut Command,
    output: &Path,
    timeout: Duration,
    cancelled: &AtomicBool,
    receipt: Result<Receipt>,
) -> Result<Completion> {
    let receipt = receipt?;
    write_json_new(&output.join("start.json"), &receipt)?;
    command
        .stdin(Stdio::null())
        .stdout(File::create_new(output.join("stdout"))?)
        .stderr(File::create_new(output.join("stderr"))?);
    let started = Instant::now();
    let completion = if cancelled.load(Ordering::Relaxed) {
        Ok(Completion {
            status: Status::Cancelled,
            exit_code: None,
            elapsed_ms: started.elapsed().as_millis(),
        })
    } else {
        match Running::spawn(command) {
            Ok(mut running) => running
                .wait(timeout, cancelled)
                .map(|(status, exit)| Completion {
                    status,
                    exit_code: exit.code(),
                    elapsed_ms: started.elapsed().as_millis(),
                }),
            Err(error) => {
                fs::write(output.join("spawn-error.txt"), error.to_string())?;
                Ok(Completion {
                    status: Status::SpawnFailed,
                    exit_code: None,
                    elapsed_ms: started.elapsed().as_millis(),
                })
            }
        }
    };
    Completion::record(completion, output)
}

impl Completion {
    fn record(outcome: Result<Self>, output: &Path) -> Result<Self> {
        let completion = outcome?;
        write_json_new(&output.join("finish.json"), &completion)?;
        Ok(completion)
    }
}

pub(super) fn checked(
    command: &mut Command,
    output: &Path,
    timeout: Duration,
    cancelled: &AtomicBool,
) -> Result<String> {
    let completion = execute(command, output, timeout, cancelled)?;
    if completion.status != Status::Success {
        return Err(invalid(format!(
            "process {:?} ({:?}); receipts: {}",
            completion.status,
            completion.exit_code,
            output.display()
        ))
        .into());
    }
    Ok(fs::read_to_string(output.join("stdout"))?)
}

impl Receipt {
    fn capture(
        command: &Command,
        directory: io::Result<PathBuf>,
        started: std::result::Result<Duration, SystemTimeError>,
        timeout: Duration,
    ) -> Result<Self> {
        Ok(Self {
            program: command.get_program().to_string_lossy().into_owned(),
            arguments: command
                .get_args()
                .map(|arg| arg.to_string_lossy().into_owned())
                .collect(),
            directory: directory?.display().to_string(),
            environment: command
                .get_envs()
                .map(|(key, value)| {
                    (
                        key.to_string_lossy().into_owned(),
                        value.map(|v| v.to_string_lossy().into_owned()),
                    )
                })
                .collect(),
            started_unix_ms: started?.as_millis(),
            timeout_ms: timeout.as_millis(),
        })
    }
}

/// Operations required to terminate and reap an owned process group.
trait ProcessControl {
    fn terminate(&mut self) -> std::result::Result<(), Errno>;
    fn wait(&mut self) -> io::Result<ExitStatus>;
    fn poll(&mut self) -> io::Result<Option<ExitStatus>>;
}

impl ProcessControl for Child {
    fn terminate(&mut self) -> std::result::Result<(), Errno> {
        killpg(Pid::from_raw(self.id() as i32), Signal::SIGKILL)
    }

    fn wait(&mut self) -> io::Result<ExitStatus> {
        Child::wait(self)
    }

    fn poll(&mut self) -> io::Result<Option<ExitStatus>> {
        self.try_wait()
    }
}

impl Running<Child> {
    fn spawn(command: &mut Command) -> std::io::Result<Self> {
        Ok(Self {
            child: command.process_group(0).spawn()?,
            state: GroupState::Running,
        })
    }
}

impl<C: ProcessControl> Running<C> {
    fn wait(&mut self, timeout: Duration, cancelled: &AtomicBool) -> Result<(Status, ExitStatus)> {
        let started = Instant::now();
        loop {
            if cancelled.load(Ordering::Relaxed) {
                let exit = self.close()?;
                return Ok((Status::Cancelled, exit));
            }
            if started.elapsed() >= timeout {
                let exit = self.close()?;
                return Ok((Status::TimedOut, exit));
            }
            if let Some(exit) = self.child.poll()? {
                self.close()?;
                return Ok((
                    if exit.success() {
                        Status::Success
                    } else {
                        Status::Failed
                    },
                    exit,
                ));
            }
            std::thread::sleep(Duration::from_millis(5));
        }
    }

    fn close(&mut self) -> Result<ExitStatus> {
        if self.state == GroupState::Running {
            match self.child.terminate() {
                Err(Errno::ESRCH) => {}
                result => result?,
            }
            self.state = GroupState::Terminated;
        }
        let exit = self.child.wait()?;
        self.state = GroupState::Reaped;
        Ok(exit)
    }
}

impl<C: ProcessControl> Drop for Running<C> {
    fn drop(&mut self) {
        if self.state != GroupState::Reaped {
            // Best-effort cleanup also covers a receipt/write/wait error.
            let _ = self.close();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Completion, Receipt, Running, Status, checked, execute};
    use std::collections::BTreeMap;
    use std::fs;
    use std::process::{Command, Stdio};
    use std::sync::atomic::AtomicBool;
    use std::time::Duration;

    #[test]
    fn receipts_keep_arguments_environment_and_both_streams() {
        let dir = tempfile::tempdir().unwrap();
        let mut command = Command::new("/bin/sh");
        command
            .args([
                "-c",
                "printf '%s' \"$1\"; printf problem >&2",
                "fixture",
                "spaces; $literal",
            ])
            .current_dir(dir.path())
            .env("BENCH_RECEIPT", "value")
            .env_remove("GEAM_CONFIG");
        let output = dir.path().join("success");
        let completion = execute(
            &mut command,
            &output,
            Duration::from_secs(2),
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(completion.status, Status::Success);
        assert_eq!(completion.exit_code, Some(0));
        assert_eq!(
            fs::read(output.join("stdout")).unwrap(),
            b"spaces; $literal"
        );
        assert_eq!(fs::read(output.join("stderr")).unwrap(), b"problem");
        let receipt: Receipt =
            serde_json::from_slice(&fs::read(output.join("start.json")).unwrap()).unwrap();
        assert_eq!(receipt.program, "/bin/sh");
        assert_eq!(
            receipt.arguments,
            [
                "-c",
                "printf '%s' \"$1\"; printf problem >&2",
                "fixture",
                "spaces; $literal"
            ]
        );
        assert_eq!(receipt.directory, dir.path().display().to_string());
        assert_eq!(
            receipt.environment,
            BTreeMap::from([
                ("BENCH_RECEIPT".into(), Some("value".into())),
                ("GEAM_CONFIG".into(), None)
            ])
        );
        assert_eq!(receipt.timeout_ms, 2000);
        assert!(
            execute(
                &mut command,
                &output,
                Duration::from_secs(2),
                &AtomicBool::new(false)
            )
            .is_err()
        );
    }

    #[test]
    fn failures_timeouts_cancellation_and_drop_close_the_owned_group() {
        let dir = tempfile::tempdir().unwrap();
        for (name, arguments, timeout, cancelled, expected) in [
            (
                "failed",
                vec!["-c", "exit 7"],
                Duration::from_secs(2),
                false,
                Status::Failed,
            ),
            (
                "timeout",
                vec!["-c", "sleep 30 & wait"],
                Duration::ZERO,
                false,
                Status::TimedOut,
            ),
            (
                "cancelled",
                vec!["-c", "sleep 30 & wait"],
                Duration::from_secs(2),
                true,
                Status::Cancelled,
            ),
        ] {
            let completion = execute(
                Command::new("/bin/sh").args(arguments),
                &dir.path().join(name),
                timeout,
                &AtomicBool::new(cancelled),
            )
            .unwrap();
            assert_eq!(completion.status, expected);
        }
        let completion = execute(
            &mut Command::new(dir.path().join("absent")),
            &dir.path().join("missing"),
            Duration::from_secs(2),
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(completion.status, Status::SpawnFailed);
        assert_eq!(completion.exit_code, None);
        assert!(
            checked(
                Command::new("/bin/sh").args(["-c", "exit 2"]),
                &dir.path().join("checked"),
                Duration::from_secs(2),
                &AtomicBool::new(false)
            )
            .is_err()
        );
        assert_eq!(
            checked(
                Command::new("/bin/sh").args(["-c", "printf ' ready\\n'"]),
                &dir.path().join("whitespace"),
                Duration::from_secs(2),
                &AtomicBool::new(false)
            )
            .unwrap(),
            " ready\n"
        );
        let running = Running::spawn(
            Command::new("/bin/sleep")
                .arg("30")
                .stdout(Stdio::null())
                .stderr(Stdio::null()),
        )
        .unwrap();
        let pid = running.child.id();
        drop(running);
        assert_eq!(
            nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), None),
            Err(nix::errno::Errno::ESRCH)
        );
    }

    #[test]
    fn checked_processes_preserve_io_and_invalid_unicode_failures() {
        let root = tempfile::tempdir().unwrap();
        let collision = root.path().join("collision");
        fs::create_dir(&collision).unwrap();
        assert!(
            checked(
                &mut Command::new("/usr/bin/true"),
                &collision,
                Duration::from_secs(2),
                &AtomicBool::new(false)
            )
            .is_err()
        );
        let result = checked(
            Command::new("/bin/sh").args(["-c", "printf '\\377'"]),
            &root.path().join("invalid-unicode"),
            Duration::from_secs(2),
            &AtomicBool::new(false),
        );
        assert_eq!(
            result
                .unwrap_err()
                .downcast_ref::<std::io::Error>()
                .unwrap()
                .kind(),
            std::io::ErrorKind::InvalidData
        );
    }

    #[test]
    fn process_evidence_failures_never_publish_a_completion() {
        use super::{Receipt, execute_recorded};
        use std::io;
        use std::time::{SystemTime, UNIX_EPOCH};
        let root = tempfile::tempdir().unwrap();
        assert_eq!(
            Completion::record(Err(io::Error::other("wait failed").into()), root.path())
                .unwrap_err()
                .to_string(),
            "wait failed"
        );
        assert!(!root.path().join("finish.json").exists());
        let command = Command::new("/usr/bin/true");
        let before_epoch = UNIX_EPOCH
            .checked_sub(Duration::from_secs(1))
            .unwrap()
            .duration_since(UNIX_EPOCH);
        assert!(
            Receipt::capture(
                &command,
                Err(io::Error::other("cwd unavailable")),
                Ok(Duration::ZERO),
                Duration::ZERO
            )
            .unwrap_err()
            .to_string()
            .contains("cwd unavailable")
        );
        assert!(
            Receipt::capture(
                &command,
                Ok(root.path().into()),
                before_epoch,
                Duration::ZERO
            )
            .is_err()
        );
        for target in [
            "receipt",
            "start.json",
            "stdout",
            "stderr",
            "spawn-error.txt",
            "finish.json",
        ] {
            let output = root.path().join(target);
            fs::create_dir(&output).unwrap();
            if target != "receipt" {
                fs::create_dir(output.join(target)).unwrap();
            }
            let mut command = Command::new(if target == "spawn-error.txt" {
                "/geam-bench-absent-program"
            } else {
                "/usr/bin/true"
            });
            let receipt = if target == "receipt" {
                Err(io::Error::other("receipt unavailable").into())
            } else {
                Receipt::capture(
                    &command,
                    Ok(root.path().into()),
                    SystemTime::now().duration_since(UNIX_EPOCH),
                    Duration::from_secs(2),
                )
            };
            let error = execute_recorded(
                &mut command,
                &output,
                Duration::from_secs(2),
                &AtomicBool::new(false),
                receipt,
            )
            .unwrap_err();
            assert!(
                error.downcast_ref::<io::Error>().is_some(),
                "{target}: {error}"
            );
            assert!(!output.join("finish.json").is_file(), "{target}");
        }
    }

    #[test]
    fn wait_errors_keep_process_group_cleanup_under_ownership() {
        use super::{GroupState, ProcessControl};
        use nix::errno::Errno;
        use std::cell::RefCell;
        use std::io;
        use std::process::{Child, ExitStatus};
        use std::rc::Rc;

        struct ObservedChild {
            child: Child,
            fail: Option<&'static str>,
            calls: Rc<RefCell<Vec<&'static str>>>,
        }
        impl ProcessControl for ObservedChild {
            fn terminate(&mut self) -> std::result::Result<(), Errno> {
                self.calls.borrow_mut().push("terminate");
                if self.fail == Some("terminate") {
                    self.fail = None;
                    return Err(Errno::EIO);
                }
                self.child.terminate()
            }
            fn wait(&mut self) -> io::Result<ExitStatus> {
                self.calls.borrow_mut().push("wait");
                if self.fail == Some("wait") {
                    self.fail = None;
                    return Err(io::Error::other("wait failed"));
                }
                self.child.wait()
            }
            fn poll(&mut self) -> io::Result<Option<ExitStatus>> {
                if self.fail == Some("poll") {
                    self.fail = None;
                    return Err(io::Error::other("poll failed"));
                }
                self.child.try_wait()
            }
        }
        for (operation, cancelled, timeout, sleeping) in [
            ("terminate", true, Duration::from_secs(2), true),
            ("terminate", false, Duration::ZERO, true),
            ("terminate", false, Duration::from_secs(2), false),
            ("wait", true, Duration::from_secs(2), true),
            ("poll", false, Duration::from_secs(2), true),
            ("none", true, Duration::from_secs(2), true),
            ("none", false, Duration::ZERO, true),
            ("none", false, Duration::from_secs(2), false),
            ("exit", false, Duration::from_secs(2), false),
        ] {
            use std::os::unix::process::CommandExt;
            let mut command = Command::new("/bin/sh");
            command.args([
                "-c",
                if sleeping {
                    "sleep 30 & wait"
                } else if operation == "exit" {
                    "exit 7"
                } else {
                    "exit 0"
                },
            ]);
            let child = command
                .process_group(0)
                .stdout(Stdio::null())
                .stderr(Stdio::null())
                .spawn()
                .unwrap();
            let pid = child.id();
            let calls = Rc::new(RefCell::new(Vec::new()));
            let mut running = Running {
                child: ObservedChild {
                    child,
                    fail: Some(operation),
                    calls: Rc::clone(&calls),
                },
                state: GroupState::Running,
            };
            let result = running.wait(timeout, &AtomicBool::new(cancelled));
            if matches!(operation, "terminate" | "wait" | "poll") {
                let error = result.unwrap_err();
                assert_eq!(
                    error.to_string(),
                    match operation {
                        "terminate" => Errno::EIO.to_string(),
                        "wait" => "wait failed".into(),
                        _ => "poll failed".into(),
                    }
                );
            } else {
                let (status, exit) = result.unwrap();
                assert_eq!(
                    status,
                    if cancelled {
                        Status::Cancelled
                    } else if timeout.is_zero() {
                        Status::TimedOut
                    } else if operation == "exit" {
                        Status::Failed
                    } else {
                        Status::Success
                    }
                );
                if operation == "exit" {
                    assert_eq!(exit.code(), Some(7));
                }
            }
            drop(running);
            assert_eq!(
                *calls.borrow(),
                match operation {
                    "terminate" => vec!["terminate", "terminate", "wait"],
                    "wait" => vec!["terminate", "wait", "wait"],
                    _ => vec!["terminate", "wait"],
                }
            );
            assert_eq!(
                nix::sys::wait::waitpid(
                    nix::unistd::Pid::from_raw(pid as i32),
                    Some(nix::sys::wait::WaitPidFlag::WNOHANG)
                ),
                Err(Errno::ECHILD)
            );
        }
    }

    #[test]
    fn completion_and_start_protocols_are_exact() {
        for (status, expected) in [
            (Status::Success, "success"),
            (Status::Failed, "failed"),
            (Status::TimedOut, "timed_out"),
            (Status::Cancelled, "cancelled"),
            (Status::SpawnFailed, "spawn_failed"),
        ] {
            let completion = Completion {
                status,
                exit_code: None,
                elapsed_ms: 12,
            };
            let json =
                format!("{{\"status\":\"{expected}\",\"exit_code\":null,\"elapsed_ms\":12}}");
            assert_eq!(serde_json::to_string(&completion).unwrap(), json);
            assert_eq!(
                serde_json::from_str::<Completion>(&json).unwrap(),
                completion
            );
        }
        let receipt = Receipt {
            program: "app".into(),
            arguments: vec!["one two".into()],
            directory: "work".into(),
            environment: BTreeMap::from([("GEAM_CONFIG".into(), None)]),
            started_unix_ms: 1,
            timeout_ms: 2000,
        };
        let json = r#"{"program":"app","arguments":["one two"],"directory":"work","environment":{"GEAM_CONFIG":null},"started_unix_ms":1,"timeout_ms":2000}"#;
        assert_eq!(serde_json::to_string(&receipt).unwrap(), json);
        assert_eq!(serde_json::from_str::<Receipt>(json).unwrap(), receipt);
    }

    #[test]
    fn cancellation_of_a_running_process_reaps_its_child() {
        let mut running = Running::spawn(Command::new("/bin/sleep").arg("30")).unwrap();
        let pid = running.child.id();
        let (status, exit) = running
            .wait(Duration::from_secs(10), &AtomicBool::new(true))
            .unwrap();
        assert_eq!(status, Status::Cancelled);
        assert!(!exit.success());
        assert_eq!(
            nix::sys::signal::kill(nix::unistd::Pid::from_raw(pid as i32), None),
            Err(nix::errno::Errno::ESRCH)
        );
    }
}
