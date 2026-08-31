use super::registry::{
    CandidateRejection, ProviderCandidate, ProviderRegistry, RegistryDiscoveryError,
};
use crate::error::CliError;
use hexpm::version::Version as GleamVersion;

pub(crate) trait ProviderDiscovery {
    fn discover(
        &self,
        package: &str,
        version: &GleamVersion,
    ) -> Result<Vec<ProviderCandidate>, CliError>;
}

pub(crate) struct RegistryProviderDiscovery<'registry> {
    registry: &'registry dyn ProviderRegistry,
}

impl<'registry> RegistryProviderDiscovery<'registry> {
    pub(crate) fn new(registry: &'registry dyn ProviderRegistry) -> Self {
        Self { registry }
    }
}

impl ProviderDiscovery for RegistryProviderDiscovery<'_> {
    fn discover(
        &self,
        package: &str,
        version: &GleamVersion,
    ) -> Result<Vec<ProviderCandidate>, CliError> {
        super::registry::discover(self.registry, package, version)
            .map_err(|error| discovery_error(package, error))
    }
}

fn discovery_error(package: &str, error: RegistryDiscoveryError) -> CliError {
    match error {
        RegistryDiscoveryError::Access(error) => CliError::ProviderRegistryAccess {
            package: package.to_owned(),
            reason: error.to_string(),
        },
        error @ (RegistryDiscoveryError::Protocol { .. }
        | RegistryDiscoveryError::SearchLimit { .. }) => CliError::ProviderRegistryProtocol {
            package: package.to_owned(),
            reason: error.to_string(),
        },
        RegistryDiscoveryError::NoCandidates {
            package,
            version,
            rejections,
        } => CliError::ProviderCandidatesUnavailable {
            package,
            version: version.to_string(),
            details: rejection_details(&rejections),
        },
    }
}

fn rejection_details(rejections: &[CandidateRejection]) -> String {
    if rejections.is_empty() {
        return "no matching crates were found".to_owned();
    }
    rejections
        .iter()
        .map(|rejection| {
            let version = rejection
                .version()
                .map(|version| format!("@{version}"))
                .unwrap_or_default();
            format!(
                "{}{version}: {}",
                rejection.crate_name(),
                rejection.reason(),
            )
        })
        .collect::<Vec<_>>()
        .join("; ")
}

#[cfg(test)]
mod tests {
    use super::{ProviderDiscovery, RegistryProviderDiscovery, discovery_error};
    use crate::error::CliError;
    use crate::provider::registry::{
        CandidateRejection, ProviderRegistry, RegistryAccessError, RegistryDiscoveryError,
    };
    use hexpm::version::Version as GleamVersion;

    #[test]
    fn maps_each_registry_failure_without_losing_rejection_context() {
        let package = "images";
        let access = discovery_error(
            package,
            RegistryDiscoveryError::Access(RegistryAccessError::new("search", "offline")),
        );
        assert!(matches!(
            access,
            CliError::ProviderRegistryAccess { ref package, ref reason }
                if package == "images" && reason == "search failed: offline"
        ));
        for (error, expected_reason) in [
            (
                RegistryDiscoveryError::Protocol {
                    response: "search",
                    reason: "invalid".to_owned(),
                },
                "provider registry search response is invalid: invalid",
            ),
            (
                RegistryDiscoveryError::SearchLimit {
                    query: "geam-images".to_owned(),
                    total: 101,
                },
                "provider search for geam-images returned 101 results; use an explicit provider selection",
            ),
        ] {
            assert!(matches!(
                discovery_error(package, error),
                CliError::ProviderRegistryProtocol { package, reason }
                    if package == "images" && reason == expected_reason
            ));
        }
        let unavailable = discovery_error(
            package,
            RegistryDiscoveryError::NoCandidates {
                package: package.to_owned(),
                version: GleamVersion::new(1, 0, 0),
                rejections: vec![
                    CandidateRejection::new(
                        "geam-images",
                        Some("1.0.0".to_owned()),
                        "bad metadata",
                    ),
                    CandidateRejection::new("geam-images-alt", None, "no usable versions"),
                ],
            },
        );
        assert!(matches!(
            unavailable,
            CliError::ProviderCandidatesUnavailable {
                package,
                version,
                details,
            } if package == "images"
                && version == "1.0.0"
                && details
                    == "geam-images@1.0.0: bad metadata; geam-images-alt: no usable versions"
        ));
        let absent = discovery_error(
            package,
            RegistryDiscoveryError::NoCandidates {
                package: package.to_owned(),
                version: GleamVersion::new(1, 0, 0),
                rejections: Vec::new(),
            },
        );
        assert!(matches!(
            absent,
            CliError::ProviderCandidatesUnavailable {
                package,
                version,
                details,
            } if package == "images"
                && version == "1.0.0"
                && details == "no matching crates were found"
        ));
    }

    #[test]
    fn connects_the_registry_protocol_without_losing_access_failures() {
        let registry = AccessFailureRegistry;
        let discovery = RegistryProviderDiscovery::new(&registry);
        assert!(matches!(
            discovery.discover("images", &GleamVersion::new(1, 0, 0)),
            Err(CliError::ProviderRegistryAccess { package, reason })
                if package == "images" && reason == "search fixture failed: offline"
        ));
        for error in [
            registry.index("geam-images"),
            registry.configuration(),
            registry.download("https://example.com/archive"),
        ] {
            assert_eq!(
                error.expect_err("fixture registry operation should fail"),
                RegistryAccessError::new("unused fixture operation", "offline"),
            );
        }
    }

    struct AccessFailureRegistry;

    impl ProviderRegistry for AccessFailureRegistry {
        fn search(&self, _query: &str) -> Result<Vec<u8>, RegistryAccessError> {
            Err(RegistryAccessError::new("search fixture", "offline"))
        }

        fn index(&self, _crate_name: &str) -> Result<Vec<u8>, RegistryAccessError> {
            Err(RegistryAccessError::new(
                "unused fixture operation",
                "offline",
            ))
        }

        fn configuration(&self) -> Result<Vec<u8>, RegistryAccessError> {
            Err(RegistryAccessError::new(
                "unused fixture operation",
                "offline",
            ))
        }

        fn download(&self, _url: &str) -> Result<Vec<u8>, RegistryAccessError> {
            Err(RegistryAccessError::new(
                "unused fixture operation",
                "offline",
            ))
        }
    }
}
