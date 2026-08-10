use super::discovery::{CandidateRejection, RegistryDiscoveryError, protocol};
use semver::Version;
use serde::Deserialize;
use std::collections::BTreeMap;

#[derive(Debug)]
pub(super) struct IndexedVersion {
    pub(super) version: Version,
    pub(super) checksum: Result<[u8; 32], String>,
    pub(super) yanked: bool,
}

pub(super) fn parse(
    crate_name: &str,
    source: &[u8],
) -> Result<(Vec<IndexedVersion>, Vec<CandidateRejection>), RegistryDiscoveryError> {
    let source = std::str::from_utf8(source).map_err(|error| protocol("sparse index", error))?;
    let mut versions = BTreeMap::new();
    let mut rejections = Vec::new();
    for line in source.lines().filter(|line| !line.is_empty()) {
        let record = serde_json::from_str::<IndexRecord>(line)
            .map_err(|error| protocol("sparse index", error))?;
        if record.name != crate_name {
            return Err(RegistryDiscoveryError::Protocol {
                response: "sparse index",
                reason: format!("record names crate {}, expected {crate_name}", record.name),
            });
        }
        let version = match record.version.parse::<Version>() {
            Ok(version) => version,
            Err(error) => {
                rejections.push(CandidateRejection::new(
                    crate_name,
                    Some(record.version),
                    format!("invalid Cargo version: {error}"),
                ));
                continue;
            }
        };
        let checksum = checksum(&record.checksum);
        if versions
            .insert(
                version.clone(),
                IndexedVersion {
                    version,
                    checksum,
                    yanked: record.yanked,
                },
            )
            .is_some()
        {
            return Err(RegistryDiscoveryError::Protocol {
                response: "sparse index",
                reason: "duplicate Cargo version record".to_owned(),
            });
        }
    }
    Ok((versions.into_values().rev().collect(), rejections))
}

fn checksum(checksum: &str) -> Result<[u8; 32], String> {
    let bytes = hex::decode(checksum).map_err(|error| format!("invalid checksum: {error}"))?;
    bytes
        .try_into()
        .map_err(|_| "checksum must contain exactly 32 bytes".to_owned())
}

#[derive(Deserialize)]
struct IndexRecord {
    name: String,
    #[serde(rename = "vers")]
    version: String,
    #[serde(rename = "cksum")]
    checksum: String,
    #[serde(default)]
    yanked: bool,
}

#[cfg(test)]
mod tests {
    use super::{checksum, parse};
    use crate::provider::registry::RegistryDiscoveryError;
    use semver::Version;

    #[test]
    fn parses_sparse_records_in_descending_semver_order() {
        let (versions, rejections) = parse(
            "geam-images",
            &index(&[
                record("geam-images", "1.0.0", &"00".repeat(32), false),
                record("geam-images", "2.0.0", &"11".repeat(32), true),
            ]),
        )
        .expect("sparse index should parse");

        assert!(rejections.is_empty());
        assert_eq!(versions[0].version, Version::new(2, 0, 0));
        assert!(versions[0].yanked);
        assert_eq!(versions[1].version, Version::new(1, 0, 0));
        assert!(!versions[1].yanked);
    }

    #[test]
    fn preserves_invalid_versions_and_checksums_as_candidate_rejections() {
        let (versions, rejections) = parse(
            "geam-images",
            &index(&[
                record("geam-images", "not-semver", &"00".repeat(32), false),
                record("geam-images", "1.0.0", "00", false),
            ]),
        )
        .expect("candidate-local errors should not invalidate the index");

        assert_eq!(rejections.len(), 1);
        assert_eq!(rejections[0].version(), Some("not-semver"));
        assert!(rejections[0].reason().starts_with("invalid Cargo version:"));
        assert_eq!(versions[0].version, Version::new(1, 0, 0));
        assert_eq!(
            versions[0]
                .checksum
                .as_ref()
                .expect_err("short checksum should be retained for candidate rejection"),
            "checksum must contain exactly 32 bytes",
        );
    }

    #[test]
    fn rejects_malformed_sparse_index_protocol_data() {
        let cases = [
            vec![0xff],
            b"{".to_vec(),
            index(&[record("other", "1.0.0", &"00".repeat(32), false)]),
            index(&[
                record("geam-images", "1.0.0", &"00".repeat(32), false),
                record("geam-images", "1.0.0", &"11".repeat(32), false),
            ]),
        ];
        for source in cases {
            assert!(matches!(
                parse("geam-images", &source),
                Err(RegistryDiscoveryError::Protocol {
                    response: "sparse index",
                    ..
                })
            ));
        }
    }

    #[test]
    fn parses_exact_sha256_checksums() {
        assert_eq!(
            checksum(&"ab".repeat(32)).expect("checksum should parse"),
            [0xab; 32],
        );
        assert!(checksum("zz").unwrap_err().starts_with("invalid checksum:"));
        assert_eq!(
            checksum("00").expect_err("short checksum should fail"),
            "checksum must contain exactly 32 bytes",
        );
    }

    fn record(crate_name: &str, version: &str, checksum: &str, yanked: bool) -> String {
        serde_json::json!({
            "name": crate_name,
            "vers": version,
            "cksum": checksum,
            "yanked": yanked,
        })
        .to_string()
    }

    fn index(records: &[String]) -> Vec<u8> {
        let mut source = records.join("\n");
        source.push('\n');
        source.into_bytes()
    }
}
