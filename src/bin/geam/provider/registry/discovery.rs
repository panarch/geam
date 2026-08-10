use super::configuration;
use super::index;
use super::search;
use super::{ProviderRegistry, RegistryAccessError, archive};
use crate::provider::metadata::ProviderMetadata;
use hexpm::version::Version as GleamVersion;
use semver::Version;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::provider) struct ProviderCandidate {
    version: Version,
    metadata: ProviderMetadata,
}

impl ProviderCandidate {
    pub(in crate::provider) fn crate_name(&self) -> &str {
        self.metadata.crate_name()
    }

    pub(in crate::provider) fn version(&self) -> &Version {
        &self.version
    }

    pub(in crate::provider) fn gleam_range(&self) -> &hexpm::version::Range {
        self.metadata.gleam_range()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::provider) struct CandidateRejection {
    crate_name: String,
    version: Option<String>,
    reason: String,
}

impl CandidateRejection {
    pub(in crate::provider) fn new(
        crate_name: impl Into<String>,
        version: Option<String>,
        reason: impl Into<String>,
    ) -> Self {
        Self {
            crate_name: crate_name.into(),
            version,
            reason: reason.into(),
        }
    }

    pub(in crate::provider) fn crate_name(&self) -> &str {
        &self.crate_name
    }

    pub(in crate::provider) fn version(&self) -> Option<&str> {
        self.version.as_deref()
    }

    pub(in crate::provider) fn reason(&self) -> &str {
        &self.reason
    }
}

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub(in crate::provider) enum RegistryDiscoveryError {
    #[error(transparent)]
    Access(#[from] RegistryAccessError),

    #[error("provider registry {response} response is invalid: {reason}")]
    Protocol {
        response: &'static str,
        reason: String,
    },

    #[error(
        "provider search for {query} returned {total} results; use an explicit provider selection"
    )]
    SearchLimit { query: String, total: usize },

    #[error("no metadata-verified provider candidate was found for {package} {version}")]
    NoCandidates {
        package: String,
        version: GleamVersion,
        rejections: Vec<CandidateRejection>,
    },
}

impl RegistryDiscoveryError {
    pub(in crate::provider) fn rejections(&self) -> &[CandidateRejection] {
        match self {
            Self::NoCandidates { rejections, .. } => rejections,
            _ => &[],
        }
    }
}

pub(in crate::provider) fn discover(
    registry: &dyn ProviderRegistry,
    gleam_package: &str,
    gleam_version: &GleamVersion,
) -> Result<Vec<ProviderCandidate>, RegistryDiscoveryError> {
    let canonical = format!("geam-{gleam_package}");
    let crate_names = search::crate_names(&registry.search(&canonical)?, &canonical)?;
    if crate_names.is_empty() {
        return Err(no_candidates(gleam_package, gleam_version, Vec::new()));
    }
    let download = configuration::parse(&registry.configuration()?)?;
    let mut candidates = Vec::new();
    let mut rejections = Vec::new();
    for crate_name in crate_names {
        let (versions, mut index_rejections) =
            index::parse(&crate_name, &registry.index(&crate_name)?)?;
        let mut rejected = !index_rejections.is_empty();
        rejections.append(&mut index_rejections);
        let mut found = false;
        for version in versions.into_iter().filter(|version| !version.yanked) {
            let checksum = match version.checksum {
                Ok(checksum) => checksum,
                Err(reason) => {
                    rejections.push(rejection(&crate_name, Some(&version.version), reason));
                    rejected = true;
                    continue;
                }
            };
            let url = download.url(&crate_name, &version.version, &checksum)?;
            let bytes = registry.download(&url)?;
            let metadata = match archive::verify(&crate_name, &version.version, &checksum, &bytes) {
                Ok(metadata) => metadata,
                Err(reason) => {
                    rejections.push(rejection(&crate_name, Some(&version.version), reason));
                    rejected = true;
                    continue;
                }
            };
            if metadata.gleam_package() != gleam_package {
                rejections.push(rejection(
                    &crate_name,
                    Some(&version.version),
                    format!(
                        "provider metadata targets Gleam package {}, expected {gleam_package}",
                        metadata.gleam_package()
                    ),
                ));
                rejected = true;
                continue;
            }
            if !metadata.supports(gleam_version) {
                rejections.push(rejection(
                    &crate_name,
                    Some(&version.version),
                    format!(
                        "Gleam {gleam_version} is outside provider range {}",
                        metadata.gleam_range()
                    ),
                ));
                rejected = true;
                continue;
            }
            candidates.push(ProviderCandidate {
                version: version.version,
                metadata,
            });
            found = true;
            break;
        }
        if !found && !rejected {
            rejections.push(CandidateRejection::new(
                crate_name,
                None,
                "sparse index contains no usable non-yanked versions",
            ));
        }
    }
    if candidates.is_empty() {
        return Err(no_candidates(gleam_package, gleam_version, rejections));
    }
    candidates.sort_by(|left, right| {
        let left_canonical = left.crate_name() != canonical;
        let right_canonical = right.crate_name() != canonical;
        left_canonical
            .cmp(&right_canonical)
            .then_with(|| left.crate_name().cmp(right.crate_name()))
    });
    Ok(candidates)
}

pub(super) fn protocol(
    response: &'static str,
    error: impl std::fmt::Display,
) -> RegistryDiscoveryError {
    RegistryDiscoveryError::Protocol {
        response,
        reason: error.to_string(),
    }
}

fn rejection(crate_name: &str, version: Option<&Version>, reason: String) -> CandidateRejection {
    CandidateRejection::new(crate_name, version.map(ToString::to_string), reason)
}

fn no_candidates(
    package: &str,
    version: &GleamVersion,
    rejections: Vec<CandidateRejection>,
) -> RegistryDiscoveryError {
    RegistryDiscoveryError::NoCandidates {
        package: package.to_owned(),
        version: version.clone(),
        rejections,
    }
}

#[cfg(test)]
mod tests {
    use crate::provider::registry::{
        CandidateRejection, ProviderCandidate, ProviderRegistry, RegistryAccessError,
        RegistryDiscoveryError, discover,
    };
    use flate2::{Compression, write::GzEncoder};
    use hexpm::version::Version as GleamVersion;
    use semver::Version;
    use sha2::{Digest, Sha256};
    use std::{cell::RefCell, collections::BTreeMap};

    #[test]
    fn discovers_highest_metadata_valid_version_for_each_exact_candidate() {
        let exact_old = provider_archive("geam-images", "1.2.0", "images", ">= 1.0.0 and < 2.0.0");
        let exact_incompatible =
            provider_archive("geam-images", "1.5.0", "images", ">= 2.0.0 and < 3.0.0");
        let malformed = ProviderArchive {
            bytes: b"not a gzip archive".to_vec(),
            checksum: hex::encode(Sha256::digest(b"not a gzip archive")),
        };
        let alternative =
            provider_archive("geam-images-alt", "9.0.0", "images", ">= 1.0.0 and < 2.0.0");
        let second_alternative =
            provider_archive("geam-images-zed", "4.0.0", "images", ">= 1.0.0 and < 2.0.0");
        let mut registry = FakeRegistry::new(
            search_response(
                5,
                &[
                    "geam-images-alt",
                    "geam-images-zed",
                    "geam-images",
                    "geam-images-alt",
                    "other",
                ],
            ),
            marker_configuration(),
        );
        registry.add_index(
            "geam-images",
            index(&[
                record("geam-images", "1.2.0", &exact_old.checksum, false),
                record("geam-images", "1.5.0", &exact_incompatible.checksum, false),
                record("geam-images", "1.6.0", &malformed.checksum, false),
                record("geam-images", "2.0.0", &exact_old.checksum, true),
            ]),
        );
        registry.add_index(
            "geam-images-alt",
            index(&[record(
                "geam-images-alt",
                "9.0.0",
                &alternative.checksum,
                false,
            )]),
        );
        registry.add_index(
            "geam-images-zed",
            index(&[record(
                "geam-images-zed",
                "4.0.0",
                &second_alternative.checksum,
                false,
            )]),
        );
        let malformed_url = marker_download_url("geam-images", "1.6.0", &malformed.checksum);
        let incompatible_url =
            marker_download_url("geam-images", "1.5.0", &exact_incompatible.checksum);
        let exact_url = marker_download_url("geam-images", "1.2.0", &exact_old.checksum);
        let alternative_url =
            marker_download_url("geam-images-alt", "9.0.0", &alternative.checksum);
        let second_alternative_url =
            marker_download_url("geam-images-zed", "4.0.0", &second_alternative.checksum);
        for (url, archive) in [
            (malformed_url.clone(), malformed),
            (incompatible_url.clone(), exact_incompatible),
            (exact_url.clone(), exact_old),
            (alternative_url.clone(), alternative),
            (second_alternative_url.clone(), second_alternative),
        ] {
            registry.add_download(url, archive.bytes);
        }

        let candidates: Vec<ProviderCandidate> =
            discover(&registry, "images", &GleamVersion::new(1, 3, 0))
                .expect("metadata-valid candidates should be discovered");

        assert_eq!(candidates.len(), 3);
        assert_eq!(candidates[0].crate_name(), "geam-images");
        assert_eq!(candidates[0].version(), &Version::new(1, 2, 0));
        assert_eq!(candidates[0].gleam_range().as_str(), ">= 1.0.0 and < 2.0.0",);
        assert_eq!(candidates[1].crate_name(), "geam-images-alt");
        assert_eq!(candidates[2].crate_name(), "geam-images-zed");
        assert_eq!(
            registry.calls(),
            [
                "search:geam-images".to_owned(),
                "configuration".to_owned(),
                "index:geam-images".to_owned(),
                format!("download:{malformed_url}"),
                format!("download:{incompatible_url}"),
                format!("download:{exact_url}"),
                "index:geam-images-alt".to_owned(),
                format!("download:{alternative_url}"),
                "index:geam-images-zed".to_owned(),
                format!("download:{second_alternative_url}"),
            ],
        );
    }

    #[test]
    fn preserves_candidate_rejections_when_no_provider_is_compatible() {
        let wrong_package =
            provider_archive("geam-images", "2.0.0", "other", ">= 1.0.0 and < 3.0.0");
        let incompatible =
            provider_archive("geam-images-alt", "3.0.0", "images", ">= 3.0.0 and < 4.0.0");
        let mut registry = FakeRegistry::new(
            search_response(2, &["geam-images", "geam-images-alt"]),
            legacy_configuration(),
        );
        for (crate_name, version, archive) in [
            ("geam-images", "2.0.0", wrong_package),
            ("geam-images-alt", "3.0.0", incompatible),
        ] {
            registry.add_index(
                crate_name,
                index(&[record(crate_name, version, &archive.checksum, false)]),
            );
            registry.add_download(legacy_download_url(crate_name, version), archive.bytes);
        }

        let error = discover(&registry, "images", &GleamVersion::new(1, 4, 0))
            .expect_err("incompatible providers should be rejected");

        assert!(matches!(
            error,
            RegistryDiscoveryError::NoCandidates {
                ref package,
                ref version,
                ..
            } if package == "images" && version == &GleamVersion::new(1, 4, 0)
        ));
        assert_eq!(error.rejections().len(), 2);
        assert_eq!(error.rejections()[0].crate_name(), "geam-images");
        assert_eq!(error.rejections()[0].version(), Some("2.0.0"));
        assert_eq!(
            error.rejections()[0].reason(),
            "provider metadata targets Gleam package other, expected images",
        );
        assert_eq!(
            error.rejections()[1].reason(),
            "Gleam 1.4.0 is outside provider range >= 3.0.0 and < 4.0.0",
        );
    }

    #[test]
    fn reports_invalid_unusable_and_absent_candidates() {
        let mut registry =
            FakeRegistry::new(search_response(1, &["geam-images"]), legacy_configuration());
        registry.add_index(
            "geam-images",
            index(&[
                record("geam-images", "not-semver", &"00".repeat(32), false),
                record("geam-images", "2.0.0", &"00".repeat(32), true),
                record("geam-images", "1.0.0", "00", false),
            ]),
        );
        let error = discover(&registry, "images", &GleamVersion::new(1, 0, 0))
            .expect_err("invalid sparse versions should not yield a candidate");
        assert_eq!(error.rejections().len(), 2);
        assert!(
            error.rejections()[0]
                .reason()
                .starts_with("invalid Cargo version:")
        );
        assert_eq!(
            error.rejections()[1].reason(),
            "checksum must contain exactly 32 bytes",
        );

        for index_source in [
            Vec::new(),
            index(&[record("geam-images", "1.0.0", &"00".repeat(32), true)]),
        ] {
            let mut registry =
                FakeRegistry::new(search_response(1, &["geam-images"]), legacy_configuration());
            registry.add_index("geam-images", index_source);
            assert_eq!(
                discover(&registry, "images", &GleamVersion::new(1, 0, 0))
                    .expect_err("index without usable version should fail")
                    .rejections(),
                [CandidateRejection::new(
                    "geam-images",
                    None,
                    "sparse index contains no usable non-yanked versions",
                )],
            );
        }

        let registry = FakeRegistry::new(
            search_response(2, &["other", "prefix-geam-images"]),
            Vec::new(),
        );
        let error = discover(&registry, "images", &GleamVersion::new(1, 0, 0))
            .expect_err("unrelated search results should not be candidates");
        assert_eq!(
            error,
            RegistryDiscoveryError::NoCandidates {
                package: "images".to_owned(),
                version: GleamVersion::new(1, 0, 0),
                rejections: Vec::new(),
            },
        );
        assert!(error.rejections().is_empty());
        assert_eq!(registry.calls(), ["search:geam-images"]);
    }

    #[test]
    fn propagates_search_configuration_and_index_protocol_failures() {
        let search_error = super::super::search::crate_names(b"{", "geam-images")
            .expect_err("malformed search fixture should fail");
        let registry = FakeRegistry::new(b"{".to_vec(), legacy_configuration());
        assert_eq!(
            discover(&registry, "images", &GleamVersion::new(1, 0, 0))
                .expect_err("search protocol failure should propagate"),
            search_error,
        );

        let configuration_error = super::super::configuration::parse(b"{")
            .expect_err("malformed configuration fixture should fail");
        let registry = FakeRegistry::new(search_response(1, &["geam-images"]), b"{".to_vec());
        assert_eq!(
            discover(&registry, "images", &GleamVersion::new(1, 0, 0))
                .expect_err("configuration protocol failure should propagate"),
            configuration_error,
        );

        let index_error = super::super::index::parse("geam-images", b"{")
            .expect_err("malformed index fixture should fail");
        let mut registry =
            FakeRegistry::new(search_response(1, &["geam-images"]), legacy_configuration());
        registry.add_index("geam-images", b"{".to_vec());
        assert_eq!(
            discover(&registry, "images", &GleamVersion::new(1, 0, 0))
                .expect_err("index protocol failure should propagate"),
            index_error,
        );
    }

    #[test]
    fn distinguishes_registry_access_failures_at_each_operation() {
        for failed_operation in [
            FailedOperation::Search,
            FailedOperation::Configuration,
            FailedOperation::Index,
            FailedOperation::Download,
        ] {
            let archive =
                provider_archive("geam-images", "1.0.0", "images", ">= 1.0.0 and < 2.0.0");
            let mut registry =
                FakeRegistry::new(search_response(1, &["geam-images"]), legacy_configuration());
            registry.add_index(
                "geam-images",
                index(&[record("geam-images", "1.0.0", &archive.checksum, false)]),
            );
            registry.add_download(legacy_download_url("geam-images", "1.0.0"), archive.bytes);
            registry.fail(failed_operation);

            let error = discover(&registry, "images", &GleamVersion::new(1, 0, 0))
                .expect_err("registry access failure should be preserved");
            assert_eq!(
                error,
                RegistryDiscoveryError::Access(RegistryAccessError::new(
                    failed_operation.name(),
                    "fixture failure",
                )),
            );
            assert!(error.rejections().is_empty());
            assert_eq!(
                error.to_string(),
                format!("{} failed: fixture failure", failed_operation.name())
            );
        }
    }

    #[test]
    fn propagates_download_configuration_markers_from_discovery() {
        let mut registry = FakeRegistry::new(
            search_response(1, &["geam-images"]),
            br#"{"dl":"https://downloads.example/{crate}/{unknown}"}"#.to_vec(),
        );
        registry.add_index(
            "geam-images",
            index(&[record("geam-images", "1.0.0", &"ab".repeat(32), false)]),
        );

        assert!(matches!(
            discover(&registry, "images", &GleamVersion::new(1, 0, 0)),
            Err(RegistryDiscoveryError::Protocol {
                response: "configuration",
                ref reason,
            }) if reason.contains("unsupported marker")
        ));
        assert_eq!(
            registry.calls(),
            ["search:geam-images", "configuration", "index:geam-images"],
        );
    }

    struct FakeRegistry {
        search: Result<Vec<u8>, RegistryAccessError>,
        indexes: BTreeMap<String, Result<Vec<u8>, RegistryAccessError>>,
        configuration: Result<Vec<u8>, RegistryAccessError>,
        downloads: BTreeMap<String, Result<Vec<u8>, RegistryAccessError>>,
        calls: RefCell<Vec<String>>,
    }

    impl FakeRegistry {
        fn new(search: Vec<u8>, configuration: Vec<u8>) -> Self {
            Self {
                search: Ok(search),
                indexes: BTreeMap::new(),
                configuration: Ok(configuration),
                downloads: BTreeMap::new(),
                calls: RefCell::new(Vec::new()),
            }
        }

        fn add_index(&mut self, crate_name: &str, source: Vec<u8>) {
            self.indexes.insert(crate_name.to_owned(), Ok(source));
        }

        fn add_download(&mut self, url: String, bytes: Vec<u8>) {
            self.downloads.insert(url, Ok(bytes));
        }

        fn fail(&mut self, operation: FailedOperation) {
            let failure = Err(RegistryAccessError::new(
                operation.name(),
                "fixture failure",
            ));
            match operation {
                FailedOperation::Search => self.search = failure,
                FailedOperation::Configuration => self.configuration = failure,
                FailedOperation::Index => {
                    self.indexes.insert("geam-images".to_owned(), failure);
                }
                FailedOperation::Download => {
                    self.downloads
                        .insert(legacy_download_url("geam-images", "1.0.0"), failure);
                }
            }
        }

        fn calls(&self) -> Vec<String> {
            self.calls.borrow().clone()
        }
    }

    #[derive(Clone, Copy)]
    enum FailedOperation {
        Search,
        Configuration,
        Index,
        Download,
    }

    impl FailedOperation {
        fn name(self) -> &'static str {
            match self {
                Self::Search => "search",
                Self::Configuration => "configuration",
                Self::Index => "index",
                Self::Download => "download",
            }
        }
    }

    impl ProviderRegistry for FakeRegistry {
        fn search(&self, query: &str) -> Result<Vec<u8>, RegistryAccessError> {
            self.calls.borrow_mut().push(format!("search:{query}"));
            self.search.clone()
        }

        fn index(&self, crate_name: &str) -> Result<Vec<u8>, RegistryAccessError> {
            self.calls.borrow_mut().push(format!("index:{crate_name}"));
            self.indexes[crate_name].clone()
        }

        fn configuration(&self) -> Result<Vec<u8>, RegistryAccessError> {
            self.calls.borrow_mut().push("configuration".to_owned());
            self.configuration.clone()
        }

        fn download(&self, url: &str) -> Result<Vec<u8>, RegistryAccessError> {
            self.calls.borrow_mut().push(format!("download:{url}"));
            self.downloads[url].clone()
        }
    }

    struct ProviderArchive {
        bytes: Vec<u8>,
        checksum: String,
    }

    fn provider_archive(
        crate_name: &str,
        version: &str,
        gleam_package: &str,
        gleam_range: &str,
    ) -> ProviderArchive {
        let manifest = format!(
            r#"[package]
name = "{crate_name}"
version = "{version}"
edition = "2024"

[package.metadata.geam.provider]
schema = 1
gleam-package = "{gleam_package}"
gleam-version = "{gleam_range}"
"#,
        );
        let encoder = GzEncoder::new(Vec::new(), Compression::default());
        let mut archive = tar::Builder::new(encoder);
        let mut header = tar::Header::new_gnu();
        header.set_mode(0o644);
        header.set_size(manifest.len() as u64);
        header.set_cksum();
        archive
            .append_data(
                &mut header,
                format!("{crate_name}-{version}/Cargo.toml"),
                manifest.as_bytes(),
            )
            .expect("fixture manifest should enter archive");
        let encoder = archive
            .into_inner()
            .expect("fixture tar should finish writing");
        let bytes = encoder
            .finish()
            .expect("fixture gzip should finish writing");
        let checksum = hex::encode(Sha256::digest(&bytes));
        ProviderArchive { bytes, checksum }
    }

    fn search_response(total: usize, crate_names: &[&str]) -> Vec<u8> {
        serde_json::to_vec(&serde_json::json!({
            "crates": crate_names
                .iter()
                .map(|crate_name| serde_json::json!({ "id": crate_name }))
                .collect::<Vec<_>>(),
            "meta": { "total": total },
        }))
        .expect("search fixture should serialize")
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

    fn legacy_configuration() -> Vec<u8> {
        br#"{"dl":"https://downloads.example"}"#.to_vec()
    }

    fn marker_configuration() -> Vec<u8> {
        br#"{"dl":"https://downloads.example/{crate}/{version}/{prefix}/{lowerprefix}/{sha256-checksum}"}"#.to_vec()
    }

    fn legacy_download_url(crate_name: &str, version: &str) -> String {
        format!("https://downloads.example/{crate_name}/{version}/download")
    }

    fn marker_download_url(crate_name: &str, version: &str, checksum: &str) -> String {
        let index_path = super::super::crates_io::sparse_index_path(crate_name);
        let prefix = index_path.rsplit_once('/').map_or("", |(prefix, _)| prefix);
        format!(
            "https://downloads.example/{crate_name}/{version}/{prefix}/{}/{}",
            prefix.to_ascii_lowercase(),
            checksum,
        )
    }
}
