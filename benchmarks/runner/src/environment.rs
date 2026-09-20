use crate::process;
use crate::{Result, invalid};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::BTreeMap;
use std::fs;
use std::io;
use std::num::NonZeroUsize;
use std::path::{Component, Path, PathBuf};
use std::process::Command;
use std::sync::atomic::AtomicBool;
use std::time::Duration;

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Host {
    pub(super) os: String,
    pub(super) architecture: String,
    pub(super) system: String,
    pub(super) logical_cpus: usize,
    pub(super) machine: String,
    pub(super) overrides: BTreeMap<String, Option<String>>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Source {
    pub(super) commit: String,
    pub(super) changes: String,
    pub(super) excluded_outputs: Vec<String>,
    pub(super) files: BTreeMap<String, String>,
}

pub(super) fn host(machine: String, logs: &Path, cancelled: &AtomicBool) -> Result<Host> {
    Host::capture(
        machine,
        probe(
            Command::new("/usr/bin/uname").arg("-a"),
            &logs.join("host-system"),
            cancelled,
        ),
        std::thread::available_parallelism(),
    )
}

impl Host {
    fn capture(
        machine: String,
        system: Result<String>,
        logical_cpus: io::Result<NonZeroUsize>,
    ) -> Result<Self> {
        Ok(Self {
            os: std::env::consts::OS.into(),
            architecture: std::env::consts::ARCH.into(),
            system: system?,
            logical_cpus: logical_cpus?.get(),
            machine,
            overrides: overrides(),
        })
    }
}

pub(super) fn controller(path: io::Result<PathBuf>) -> Result<Vec<u8>> {
    Ok(fs::read(path?)?)
}

pub(super) fn overrides() -> BTreeMap<String, Option<String>> {
    [
        "NODE_OPTIONS",
        "ERL_FLAGS",
        "ERL_AFLAGS",
        "ERL_ZFLAGS",
        "ERL_LIBS",
        "LANG",
        "LC_ALL",
        "RUSTFLAGS",
        "CARGO_ENCODED_RUSTFLAGS",
        "RUSTC",
        "RUSTC_WRAPPER",
        "RUSTC_WORKSPACE_WRAPPER",
        "CARGO_BUILD_TARGET",
        "CARGO_PROFILE_RELEASE_LTO",
        "CARGO_PROFILE_RELEASE_OPT_LEVEL",
        "CARGO_PROFILE_RELEASE_CODEGEN_UNITS",
        "CARGO_PROFILE_RELEASE_DEBUG",
        "MACOSX_DEPLOYMENT_TARGET",
        "SDKROOT",
        "LD_LIBRARY_PATH",
        "LD_PRELOAD",
        "DYLD_LIBRARY_PATH",
        "DYLD_INSERT_LIBRARIES",
    ]
    .into_iter()
    .map(|name| {
        (
            name.into(),
            std::env::var_os(name).map(|v| v.to_string_lossy().into_owned()),
        )
    })
    .collect()
}

pub(super) fn source(
    root: &Path,
    outputs: &[&Path],
    logs: &Path,
    cancelled: &AtomicBool,
) -> Result<Source> {
    let root = root.canonicalize()?;
    let excluded_outputs = output_paths(&root, outputs.iter().map(|output| output.canonicalize()))?;
    let mut pathspecs = vec![".".to_owned()];
    pathspecs.extend(
        excluded_outputs
            .iter()
            .map(|path| format!(":(top,literal,exclude){path}")),
    );
    let commit = probe(
        Command::new("git")
            .current_dir(&root)
            .args(["rev-parse", "HEAD"]),
        &logs.join("source-commit"),
        cancelled,
    )?;
    let changes = process::checked(
        Command::new("git")
            .current_dir(&root)
            .args(["status", "--porcelain", "--untracked-files=all", "--"])
            .args(&pathspecs),
        &logs.join("source-status"),
        Duration::from_secs(30),
        cancelled,
    )?;
    let paths = process::checked(
        Command::new("git")
            .current_dir(&root)
            .args([
                "ls-files",
                "-z",
                "--cached",
                "--others",
                "--exclude-standard",
                "--",
            ])
            .args(&pathspecs),
        &logs.join("source-files"),
        Duration::from_secs(30),
        cancelled,
    )?;
    // Git's NUL-separated filenames must not be whitespace-trimmed.
    let mut files = BTreeMap::new();
    for name in paths.split('\0').filter(|name| !name.is_empty()) {
        let path = root.join(name);
        // Git may retain a deleted path in its index. The explicit marker makes
        // deletions distinguishable without pretending the old blob was built.
        let digest = match fs::read(&path) {
            Ok(bytes) => hash(&bytes),
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => "deleted".into(),
            Err(error) => return Err(error.into()),
        };
        files.insert(name.into(), digest);
    }
    Ok(Source {
        commit,
        changes,
        excluded_outputs,
        files,
    })
}

fn output_paths(
    root: &Path,
    outputs: impl IntoIterator<Item = io::Result<PathBuf>>,
) -> Result<Vec<String>> {
    let mut excluded_outputs = Vec::new();
    for output in outputs {
        if let Ok(relative) = output?.strip_prefix(root) {
            excluded_outputs.push(
                relative
                    .to_str()
                    .ok_or_else(|| invalid("non-UTF-8 output path"))?
                    .to_owned(),
            );
        }
    }
    excluded_outputs.sort();
    excluded_outputs.dedup();
    Ok(excluded_outputs)
}

pub(super) fn probe(command: &mut Command, logs: &Path, cancelled: &AtomicBool) -> Result<String> {
    Ok(
        process::checked(command, logs, Duration::from_secs(30), cancelled)?
            .trim()
            .to_owned(),
    )
}

pub(super) fn hash(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}

pub(super) fn files(directory: &Path) -> Result<BTreeMap<String, String>> {
    let mut hashes = BTreeMap::new();
    visit(Path::new(""), &mut hashes, fs::read_dir(directory))?;
    Ok(hashes)
}

fn visit(
    relative: &Path,
    hashes: &mut BTreeMap<String, String>,
    entries: io::Result<impl IntoIterator<Item = io::Result<fs::DirEntry>>>,
) -> Result<()> {
    for entry in entries? {
        let entry = entry?;
        visit_entry(
            &entry.path(),
            &relative.join(entry.file_name()),
            entry.file_type(),
            hashes,
        )?;
    }
    Ok(())
}

fn visit_entry(
    path: &Path,
    name: &Path,
    kind: io::Result<fs::FileType>,
    hashes: &mut BTreeMap<String, String>,
) -> Result<()> {
    let kind = kind?;
    if kind.is_dir() {
        visit(name, hashes, fs::read_dir(path))?;
    } else if kind.is_file() {
        hashes.insert(
            name.to_str()
                .ok_or_else(|| invalid("non-UTF-8 payload path"))?
                .into(),
            hash(&fs::read(path)?),
        );
    } else {
        return Err(invalid(format!(
            "expected an ordinary file or directory: {}",
            path.display()
        ))
        .into());
    }
    Ok(())
}

pub(super) fn contained(root: &Path, name: &str) -> Result<PathBuf> {
    let path = Path::new(name);
    if name.is_empty()
        || path
            .components()
            .any(|part| !matches!(part, Component::Normal(_)))
        || name.split('/').any(|part| part.is_empty() || part == ".")
    {
        return Err(invalid(format!(
            "payload path is not a normalized relative path: {name}"
        ))
        .into());
    }
    let root = root.canonicalize()?;
    let resolved = root.join(path).canonicalize()?;
    if !resolved.starts_with(&root) {
        return Err(invalid(format!("payload path escapes its bundle: {name}")).into());
    }
    Ok(resolved)
}

pub(super) fn copy_files(
    source: &Path,
    destination: &Path,
    names: impl IntoIterator<Item = String>,
) -> Result<()> {
    for name in names {
        let input = contained(source, &name)?;
        let output = destination.join(name);
        // A validated nonempty relative filename always has a parent here.
        fs::create_dir_all(output.with_file_name(""))?;
        fs::copy(input, output)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{contained, copy_files, files, hash};
    use std::collections::BTreeMap;
    use std::fs;
    use std::os::unix::fs::symlink;

    #[test]
    fn inventories_and_copies_report_real_filesystem_failures() {
        use std::os::unix::fs::PermissionsExt;
        let root = tempfile::tempdir().unwrap();
        let source = root.path().join("source");
        let output = root.path().join("output");
        fs::create_dir_all(source.join("nested")).unwrap();
        fs::write(source.join("nested/file"), b"contents").unwrap();
        assert!(files(&root.path().join("missing")).is_err());
        assert!(contained(&root.path().join("missing"), "file").is_err());
        assert!(contained(&source, "missing").is_err());
        assert!(copy_files(&source, &output, ["missing".into()]).is_err());
        fs::write(&output, b"parent is a file").unwrap();
        assert!(copy_files(&source, &output, ["nested/file".into()]).is_err());
        fs::remove_file(&output).unwrap();
        fs::create_dir_all(output.join("nested/file")).unwrap();
        assert!(copy_files(&source, &output, ["nested/file".into()]).is_err());

        let path = source.join("nested/file");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o000)).unwrap();
        let result = files(&source);
        fs::set_permissions(&path, fs::Permissions::from_mode(0o600)).unwrap();
        assert_eq!(
            result
                .unwrap_err()
                .downcast_ref::<std::io::Error>()
                .unwrap()
                .kind(),
            std::io::ErrorKind::PermissionDenied
        );
        let path = source.join("nested");
        fs::set_permissions(&path, fs::Permissions::from_mode(0o000)).unwrap();
        let result = files(&source);
        fs::set_permissions(&path, fs::Permissions::from_mode(0o700)).unwrap();
        assert_eq!(
            result
                .unwrap_err()
                .downcast_ref::<std::io::Error>()
                .unwrap()
                .kind(),
            std::io::ErrorKind::PermissionDenied
        );
    }

    #[test]
    fn host_metadata_is_exact_and_a_failed_probe_is_not_accepted() {
        use super::{Host, host, probe};
        use std::process::Command;
        use std::sync::atomic::AtomicBool;
        let expected = r#"{"os":"linux","architecture":"aarch64","system":"kernel identity","logical_cpus":4,"machine":"dedicated host","overrides":{"LANG":"C","NODE_OPTIONS":null}}"#;
        let host_value: Host = serde_json::from_str(expected).unwrap();
        assert_eq!(host_value.logical_cpus, 4);
        assert_eq!(serde_json::to_string(&host_value).unwrap(), expected);
        let root = tempfile::tempdir().unwrap();
        let error = host("host".into(), root.path(), &AtomicBool::new(true)).unwrap_err();
        assert!(
            error
                .to_string()
                .starts_with("process Cancelled (None); receipts:")
        );
        assert!(host("host".into(), root.path(), &AtomicBool::new(false)).is_err());
        let successful_logs = root.path().join("successful-host");
        fs::create_dir(&successful_logs).unwrap();
        let actual = host(
            "dedicated host".into(),
            &successful_logs,
            &AtomicBool::new(false),
        )
        .unwrap();
        assert_eq!(actual.os, std::env::consts::OS);
        assert_eq!(actual.architecture, std::env::consts::ARCH);
        assert_eq!(actual.machine, "dedicated host");
        assert_eq!(
            actual.logical_cpus,
            std::thread::available_parallelism().unwrap().get()
        );
        assert_eq!(
            actual.system,
            fs::read_to_string(successful_logs.join("host-system/stdout"))
                .unwrap()
                .trim()
        );
        assert_eq!(
            probe(
                Command::new("/bin/sh").args(["-c", "printf ' identity\\n'"]),
                &root.path().join("probe"),
                &AtomicBool::new(false)
            )
            .unwrap(),
            "identity"
        );
    }

    #[test]
    fn inventories_and_host_capture_preserve_acquisition_errors() {
        use super::{Host, controller, output_paths, visit, visit_entry};
        use std::io;
        use std::os::unix::ffi::OsStringExt;
        use std::path::PathBuf;
        let root = tempfile::tempdir().unwrap();
        let path = root.path().join("file");
        fs::write(&path, b"bytes").unwrap();
        let mut hashes = BTreeMap::new();
        let missing = || io::Error::other("acquisition failed");
        let entries: Vec<io::Result<fs::DirEntry>> = vec![Err(missing())];
        assert_eq!(
            visit(std::path::Path::new(""), &mut hashes, Ok(entries))
                .unwrap_err()
                .to_string(),
            "acquisition failed"
        );
        let absent: io::Result<Vec<io::Result<fs::DirEntry>>> = Err(missing());
        assert!(visit(std::path::Path::new(""), &mut hashes, absent).is_err());
        let entries = fs::read_dir(root.path()).unwrap().collect::<Vec<_>>();
        visit(std::path::Path::new(""), &mut hashes, Ok(entries)).unwrap();
        assert_eq!(hashes, BTreeMap::from([("file".into(), hash(b"bytes"))]));
        let entries = fs::read_dir(root.path()).unwrap().collect::<Vec<_>>();
        fs::remove_file(&path).unwrap();
        assert!(visit(std::path::Path::new(""), &mut hashes, Ok(entries)).is_err());
        fs::write(&path, b"bytes").unwrap();
        let invalid_name = PathBuf::from(std::ffi::OsString::from_vec(vec![255]));
        assert_eq!(
            visit_entry(
                &path,
                &invalid_name,
                fs::metadata(&path).map(|m| m.file_type()),
                &mut hashes
            )
            .unwrap_err()
            .to_string(),
            "non-UTF-8 payload path"
        );
        assert_eq!(
            visit_entry(
                &path,
                std::path::Path::new("file"),
                Err(missing()),
                &mut hashes
            )
            .unwrap_err()
            .to_string(),
            "acquisition failed"
        );
        assert_eq!(
            output_paths(root.path(), vec![Ok(root.path().join(&invalid_name))])
                .unwrap_err()
                .to_string(),
            "non-UTF-8 output path"
        );
        assert!(output_paths(root.path(), vec![Err(missing())]).is_err());
        assert_eq!(
            output_paths(
                root.path(),
                vec![
                    Ok(root.path().join("out")),
                    Ok(root.path().join("out")),
                    Ok(root.path().parent().unwrap().into())
                ]
            )
            .unwrap(),
            ["out"]
        );
        assert_eq!(
            Host::capture("host".into(), Ok("system".into()), Err(missing()))
                .unwrap_err()
                .to_string(),
            "acquisition failed"
        );
        assert_eq!(
            controller(Err(missing())).unwrap_err().to_string(),
            "acquisition failed"
        );
        assert!(controller(Ok(root.path().join("missing"))).is_err());
        assert_eq!(controller(Ok(path)).unwrap(), b"bytes");
    }

    #[test]
    fn inventory_and_copy_keep_exact_bytes_and_contained_names() {
        assert_eq!(
            hash(b"abc"),
            "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
        );
        let dir = tempfile::tempdir().unwrap();
        let input = dir.path().join("input");
        let output = dir.path().join("output");
        fs::create_dir_all(input.join("nested")).unwrap();
        fs::write(input.join("nested/file"), b"abc").unwrap();
        assert_eq!(
            files(&input).unwrap(),
            BTreeMap::from([("nested/file".into(), hash(b"abc"))])
        );
        copy_files(&input, &output, ["nested/file".into()]).unwrap();
        assert_eq!(files(&input).unwrap(), files(&output).unwrap());
        for name in [
            "",
            "../file",
            "/absolute",
            "./nested/file",
            "nested//file",
            "nested/./file",
            "nested/file/",
        ] {
            assert!(contained(&input, name).is_err(), "{name}");
        }
        symlink(&output, input.join("escape")).unwrap();
        assert!(contained(&input, "escape/nested/file").is_err());
        assert!(files(&input).is_err());
    }

    #[test]
    fn source_inventory_preserves_whitespace_deletions_and_dirty_bytes() {
        use super::{Source, source};
        use std::process::Command;
        use std::sync::atomic::AtomicBool;
        let root = tempfile::tempdir().unwrap();
        let checkout = root.path().join("checkout");
        fs::create_dir(&checkout).unwrap();
        for arguments in [
            vec!["init", "--quiet"],
            vec!["config", "user.name", "Benchmark test"],
            vec!["config", "user.email", "benchmark@example.invalid"],
        ] {
            assert!(
                Command::new("git")
                    .current_dir(&checkout)
                    .args(arguments)
                    .status()
                    .unwrap()
                    .success()
            );
        }
        fs::write(checkout.join(" keep whitespace "), b"original").unwrap();
        fs::write(checkout.join("deleted"), b"old").unwrap();
        assert!(
            Command::new("git")
                .current_dir(&checkout)
                .args(["add", "."])
                .status()
                .unwrap()
                .success()
        );
        assert!(
            Command::new("git")
                .current_dir(&checkout)
                .args(["commit", "--quiet", "-m", "fixture"])
                .status()
                .unwrap()
                .success()
        );
        fs::write(checkout.join(" keep whitespace "), b"changed").unwrap();
        fs::remove_file(checkout.join("deleted")).unwrap();
        let generated = checkout.join("generated output");
        fs::create_dir(&generated).unwrap();
        fs::write(generated.join("log"), b"changes while building").unwrap();
        let logs = root.path().join("logs");
        fs::create_dir(&logs).unwrap();
        assert!(
            source(
                &root.path().join("missing"),
                &[],
                &logs,
                &AtomicBool::new(false)
            )
            .is_err()
        );
        assert!(
            source(
                &checkout,
                &[&root.path().join("missing")],
                &logs,
                &AtomicBool::new(false)
            )
            .is_err()
        );
        for blocked in ["source-commit", "source-status", "source-files"] {
            let collision = root.path().join(blocked);
            fs::create_dir_all(collision.join(blocked)).unwrap();
            assert!(source(&checkout, &[], &collision, &AtomicBool::new(false)).is_err());
        }
        let actual = source(&checkout, &[&generated], &logs, &AtomicBool::new(false)).unwrap();
        assert_eq!(
            actual.files,
            BTreeMap::from([
                (" keep whitespace ".into(), hash(b"changed")),
                ("deleted".into(), "deleted".into())
            ])
        );
        assert_eq!(actual.commit.len(), 40);
        assert_eq!(actual.excluded_outputs, ["generated output"]);
        assert!(!actual.changes.contains("generated"));
        assert!(actual.changes.starts_with(" M "));
        fs::write(generated.join("log"), b"new build output").unwrap();
        let second = root.path().join("again");
        fs::create_dir(&second).unwrap();
        assert_eq!(
            source(
                &checkout,
                &[&generated, root.path()],
                &second,
                &AtomicBool::new(false)
            )
            .unwrap(),
            actual
        );
        assert!(actual.changes.contains("deleted"));
        let json = r#"{"commit":"revision","changes":" M file","excluded_outputs":[],"files":{"file":"digest"}}"#;
        let parsed: Source = serde_json::from_str(json).unwrap();
        assert_eq!(serde_json::to_string(&parsed).unwrap(), json);
        fs::create_dir(checkout.join("deleted")).unwrap();
        let logs = root.path().join("invalid");
        fs::create_dir(&logs).unwrap();
        assert!(source(&checkout, &[&generated], &logs, &AtomicBool::new(false)).is_err());
    }

    // APFS rejects these filenames at creation. Linux can admit their bytes,
    // so its mandatory owner suite reaches this filesystem boundary.
    #[cfg(target_os = "linux")]
    #[test]
    fn inventory_rejects_non_unicode_paths() {
        use std::os::unix::ffi::OsStringExt;
        let root = tempfile::tempdir().unwrap();
        fs::write(
            root.path().join(std::ffi::OsString::from_vec(vec![255])),
            b"bytes",
        )
        .unwrap();
        assert_eq!(
            files(root.path()).unwrap_err().to_string(),
            "non-UTF-8 payload path"
        );
    }
}
