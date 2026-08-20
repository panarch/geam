use crate::provider::metadata::ProviderMetadata;
use flate2::read::GzDecoder;
use semver::Version;
use sha2::{Digest, Sha256};
use std::io::Read;

pub(super) const MAX_COMPRESSED_SIZE: usize = 20 * 1024 * 1024;
const MAX_DECLARED_SIZE: u64 = 64 * 1024 * 1024;
const MAX_MANIFEST_SIZE: u64 = 1024 * 1024;
const TAR_BLOCK_SIZE: u64 = 512;

pub(super) fn verify(
    crate_name: &str,
    version: &Version,
    expected_checksum: &[u8; 32],
    bytes: &[u8],
) -> Result<ProviderMetadata, String> {
    if bytes.len() > MAX_COMPRESSED_SIZE {
        return Err(format!(
            "compressed archive exceeds the {} byte limit",
            MAX_COMPRESSED_SIZE
        ));
    }
    let actual_checksum = Sha256::digest(bytes);
    if actual_checksum[..] != expected_checksum[..] {
        return Err(format!(
            "archive checksum mismatch: expected {}, found {}",
            hex::encode(expected_checksum),
            hex::encode(actual_checksum),
        ));
    }

    let expected_manifest = format!("{crate_name}-{version}/Cargo.toml");
    let mut decoder = GzDecoder::new(bytes);
    let reader: &mut dyn Read = &mut decoder;
    let mut archive = tar::Archive::new(reader);
    let manifest = read_manifest(&mut archive, &expected_manifest)?;

    let source = std::str::from_utf8(&manifest)
        .map_err(|error| format!("packaged manifest is not valid UTF-8: {error}"))?;
    let document = source
        .parse::<toml::Table>()
        .map_err(|error| format!("packaged manifest is not valid TOML: {error}"))?;
    let package = document
        .get("package")
        .and_then(toml::Value::as_table)
        .ok_or_else(|| "packaged manifest is missing [package]".to_owned())?;
    let packaged_name = package
        .get("name")
        .and_then(toml::Value::as_str)
        .ok_or_else(|| "packaged manifest package.name must be a string".to_owned())?;
    if packaged_name != crate_name {
        return Err(format!(
            "packaged manifest names crate {packaged_name}, expected {crate_name}"
        ));
    }
    let packaged_version = package
        .get("version")
        .and_then(toml::Value::as_str)
        .ok_or_else(|| "packaged manifest package.version must be a string".to_owned())?
        .parse::<Version>()
        .map_err(|error| format!("packaged manifest has an invalid package.version: {error}"))?;
    if &packaged_version != version {
        return Err(format!(
            "packaged manifest version is {packaged_version}, expected {version}"
        ));
    }
    ProviderMetadata::from_manifest(crate_name, package)
        .map_err(|reason| format!("invalid provider metadata: {reason}"))
}

fn read_manifest(
    archive: &mut tar::Archive<&mut dyn Read>,
    expected_manifest: &str,
) -> Result<Vec<u8>, String> {
    let entries = archive
        .entries()
        .map_err(|error| format!("invalid gzip or tar archive: {error}"))?;
    let mut manifest = None;
    for entry in entries {
        let mut entry = entry.map_err(|error| format!("invalid tar entry: {error}"))?;
        let size = entry.size();
        let declared_end = declared_entry_end(entry.raw_file_position(), size);
        if !matches!(declared_end, Some(end) if end <= MAX_DECLARED_SIZE) {
            return Err(format!(
                "declared archive contents exceed the {} byte limit",
                MAX_DECLARED_SIZE
            ));
        }
        if entry.path_bytes().as_ref() != expected_manifest.as_bytes() {
            continue;
        }
        if manifest.is_some() {
            return Err(format!("archive contains duplicate {expected_manifest}"));
        }
        if size > MAX_MANIFEST_SIZE {
            return Err(format!(
                "packaged manifest exceeds the {} byte limit",
                MAX_MANIFEST_SIZE
            ));
        }
        let mut source = Vec::with_capacity(size as usize);
        entry
            .read_to_end(&mut source)
            .map_err(|error| format!("could not read packaged manifest: {error}"))?;
        manifest = Some(source);
    }

    manifest.ok_or_else(|| format!("archive is missing {expected_manifest}"))
}

fn declared_entry_end(file_position: u64, size: u64) -> Option<u64> {
    let padded_size = size.checked_add(TAR_BLOCK_SIZE - 1)? / TAR_BLOCK_SIZE * TAR_BLOCK_SIZE;
    file_position.checked_add(padded_size)
}

#[cfg(test)]
mod tests {
    use super::{
        MAX_COMPRESSED_SIZE, MAX_DECLARED_SIZE, MAX_MANIFEST_SIZE, declared_entry_end,
        read_manifest, verify,
    };
    use flate2::{Compression, write::GzEncoder};
    use semver::Version;
    use sha2::{Digest, Sha256};
    use std::io::{self, Cursor, Read, Write};

    #[test]
    fn verifies_exact_packaged_manifest_and_provider_metadata() {
        let bytes = archive(&[(
            "geam-images-1.2.3/Cargo.toml",
            valid_manifest("geam-images", "1.2.3").as_bytes(),
        )]);

        let metadata = verify(
            "geam-images",
            &Version::new(1, 2, 3),
            &checksum(&bytes),
            &bytes,
        )
        .expect("exact packaged provider should verify");

        assert_eq!(metadata.crate_name(), "geam-images");
        assert_eq!(metadata.gleam_package(), "images");
        assert_eq!(metadata.gleam_range().as_str(), ">= 1.0.0 and < 2.0.0",);
    }

    #[test]
    fn verifies_checksum_before_reading_archive_contents() {
        let oversized = vec![0; MAX_COMPRESSED_SIZE + 1];
        assert_eq!(
            verify("geam-images", &Version::new(1, 0, 0), &[0; 32], &oversized,)
                .expect_err("oversized archive should be rejected first"),
            format!(
                "compressed archive exceeds the {} byte limit",
                MAX_COMPRESSED_SIZE
            ),
        );

        let malformed = b"not a gzip archive";
        assert_eq!(
            verify("geam-images", &Version::new(1, 0, 0), &[0; 32], malformed,)
                .expect_err("checksum mismatch should precede archive parsing"),
            format!(
                "archive checksum mismatch: expected {}, found {}",
                "00".repeat(32),
                hex::encode(Sha256::digest(malformed)),
            ),
        );

        let error = verify(
            "geam-images",
            &Version::new(1, 0, 0),
            &checksum(malformed),
            malformed,
        )
        .expect_err("checksum-valid malformed gzip should be parsed and rejected");
        assert!(error.starts_with("invalid tar entry: invalid gzip header"));
    }

    #[test]
    fn bounds_declared_archive_and_manifest_sizes_without_extracting() {
        let declared = header_only("geam-images-1.0.0/large.bin", MAX_DECLARED_SIZE + 1);
        assert_eq!(
            verify(
                "geam-images",
                &Version::new(1, 0, 0),
                &checksum(&declared),
                &declared,
            )
            .expect_err("oversized declared contents should be rejected"),
            format!(
                "declared archive contents exceed the {} byte limit",
                MAX_DECLARED_SIZE
            ),
        );

        let manifest = header_only("geam-images-1.0.0/Cargo.toml", MAX_MANIFEST_SIZE + 1);
        assert_eq!(
            verify(
                "geam-images",
                &Version::new(1, 0, 0),
                &checksum(&manifest),
                &manifest,
            )
            .expect_err("oversized manifest should be rejected"),
            format!(
                "packaged manifest exceeds the {} byte limit",
                MAX_MANIFEST_SIZE
            ),
        );
    }

    #[test]
    fn counts_tar_headers_and_payload_padding_in_declared_size() {
        assert_eq!(declared_entry_end(512, 0), Some(512));
        assert_eq!(declared_entry_end(512, 1), Some(1024));
        assert_eq!(declared_entry_end(512, 512), Some(1024));
        assert_eq!(declared_entry_end(0, u64::MAX), None);
        assert_eq!(declared_entry_end(u64::MAX, 1), None);
    }

    #[test]
    fn requires_one_manifest_at_the_exact_package_path() {
        let missing = archive(&[("geam-images-1.0.0/README.md", b"read me")]);
        assert_eq!(
            verify(
                "geam-images",
                &Version::new(1, 0, 0),
                &checksum(&missing),
                &missing,
            )
            .expect_err("archive without exact manifest should fail"),
            "archive is missing geam-images-1.0.0/Cargo.toml",
        );

        let source = valid_manifest("geam-images", "1.0.0");
        let duplicate = archive(&[
            ("geam-images-1.0.0/Cargo.toml", source.as_bytes()),
            ("geam-images-1.0.0/Cargo.toml", source.as_bytes()),
        ]);
        assert_eq!(
            verify(
                "geam-images",
                &Version::new(1, 0, 0),
                &checksum(&duplicate),
                &duplicate,
            )
            .expect_err("duplicate manifest should fail"),
            "archive contains duplicate geam-images-1.0.0/Cargo.toml",
        );
    }

    #[test]
    fn rejects_malformed_tar_entries_and_manifest_contents() {
        let malformed_tar = gzip(&[1; 512]);
        assert!(
            verify(
                "geam-images",
                &Version::new(1, 0, 0),
                &checksum(&malformed_tar),
                &malformed_tar,
            )
            .expect_err("malformed tar should fail")
            .starts_with("invalid tar entry:"),
        );

        let cases: &[(&[u8], &str)] = &[
            (&[0xff], "packaged manifest is not valid UTF-8:"),
            (b"[package", "packaged manifest is not valid TOML:"),
            (b"[workspace]", "packaged manifest is missing [package]"),
            (
                b"[package]\nname = 1\nversion = \"1.0.0\"",
                "packaged manifest package.name must be a string",
            ),
            (
                b"[package]\nname = \"other\"\nversion = \"1.0.0\"",
                "packaged manifest names crate other, expected geam-images",
            ),
            (
                b"[package]\nname = \"geam-images\"\nversion = 1",
                "packaged manifest package.version must be a string",
            ),
            (
                b"[package]\nname = \"geam-images\"\nversion = \"bad\"",
                "packaged manifest has an invalid package.version:",
            ),
            (
                b"[package]\nname = \"geam-images\"\nversion = \"2.0.0\"",
                "packaged manifest version is 2.0.0, expected 1.0.0",
            ),
            (
                b"[package]\nname = \"geam-images\"\nversion = \"1.0.0\"",
                "invalid provider metadata: missing [package.metadata.geam.provider] table",
            ),
        ];
        for (source, expected) in cases {
            let bytes = archive(&[("geam-images-1.0.0/Cargo.toml", source)]);
            let error = verify(
                "geam-images",
                &Version::new(1, 0, 0),
                &checksum(&bytes),
                &bytes,
            )
            .expect_err("malformed packaged manifest should fail");
            assert!(
                error.starts_with(expected),
                "expected {expected:?}, found {error:?}",
            );
        }
    }

    #[test]
    fn maps_archive_enumeration_and_manifest_read_failures() {
        let raw = tar_contents(&[("entry", b"contents")]);
        let mut cursor = Cursor::new(raw);
        let reader: &mut dyn Read = &mut cursor;
        let mut archive = tar::Archive::new(reader);
        {
            let mut entries = archive.entries().expect("fixture archive should enumerate");
            let entry = entries
                .next()
                .expect("fixture entry should exist")
                .expect("fixture entry should parse");
            drop(entry);
        }
        assert!(
            read_manifest(&mut archive, "unused")
                .expect_err("advanced archive should reject a second enumeration")
                .starts_with("invalid gzip or tar archive: cannot call entries"),
        );

        let expected = "geam-images-1.0.0/Cargo.toml";
        let mut header = tar::Header::new_gnu();
        header.set_path(expected).expect("fixture path should fit");
        header.set_mode(0o644);
        header.set_size(1);
        header.set_cksum();
        let mut failing_reader = HeaderThenError {
            header: Cursor::new(header.as_bytes().to_vec()),
        };
        let reader: &mut dyn Read = &mut failing_reader;
        let mut archive = tar::Archive::new(reader);
        assert_eq!(
            read_manifest(&mut archive, expected)
                .expect_err("manifest stream failure should be preserved"),
            "could not read packaged manifest: fixture read failure",
        );
    }

    fn valid_manifest(crate_name: &str, version: &str) -> String {
        format!(
            r#"[package]
name = "{crate_name}"
version = "{version}"

[package.metadata.geam.provider]
schema = 1
gleam-package = "images"
gleam-version = ">= 1.0.0 and < 2.0.0"
"#,
        )
    }

    fn archive(entries: &[(&str, &[u8])]) -> Vec<u8> {
        gzip(&tar_contents(entries))
    }

    fn tar_contents(entries: &[(&str, &[u8])]) -> Vec<u8> {
        let mut archive = tar::Builder::new(Vec::new());
        for (path, contents) in entries {
            let mut header = tar::Header::new_gnu();
            header.set_mode(0o644);
            header.set_size(contents.len() as u64);
            header.set_cksum();
            archive
                .append_data(&mut header, path, *contents)
                .expect("fixture entry should enter archive");
        }
        archive
            .into_inner()
            .expect("fixture tar should finish writing")
    }

    fn header_only(path: &str, size: u64) -> Vec<u8> {
        let mut header = tar::Header::new_gnu();
        header.set_path(path).expect("fixture path should fit");
        header.set_mode(0o644);
        header.set_size(size);
        header.set_cksum();
        gzip(header.as_bytes())
    }

    fn gzip(contents: &[u8]) -> Vec<u8> {
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(contents)
            .expect("fixture gzip should accept contents");
        encoder
            .finish()
            .expect("fixture gzip should finish writing")
    }

    fn checksum(bytes: &[u8]) -> [u8; 32] {
        Sha256::digest(bytes).into()
    }

    struct HeaderThenError {
        header: Cursor<Vec<u8>>,
    }

    impl Read for HeaderThenError {
        fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
            if self.header.position() < self.header.get_ref().len() as u64 {
                self.header.read(buffer)
            } else {
                Err(io::Error::other("fixture read failure"))
            }
        }
    }
}
