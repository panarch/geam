use super::approval::ProviderApproval;
use super::manifest::{ManagedProject, ProviderSelection, ProviderSource};
use super::metadata::ProviderMetadata;
use super::registry::{
    CandidateRejection, ProviderCandidate, ProviderRegistry, RegistryDiscoveryError,
};
use crate::error::CliError;
use crate::project::ResolvedProject;
use camino::Utf8Path;
use hexpm::version::Version as GleamVersion;
use std::collections::{BTreeMap, BTreeSet};

pub(crate) trait ProviderSelectionReconciler {
    fn reconcile(
        &mut self,
        project_root: &Utf8Path,
        project: &ResolvedProject,
        program: &geam::TypedProgram,
        managed: &mut ManagedProject,
    ) -> Result<(), CliError>;
}

pub(super) trait ApprovedProviderResolver {
    fn resolve(
        &self,
        project_root: &Utf8Path,
        selection: &ProviderSelection,
    ) -> Result<ProviderMetadata, CliError>;
}

pub(super) struct SystemApprovedProviderResolver;

impl ApprovedProviderResolver for SystemApprovedProviderResolver {
    fn resolve(
        &self,
        project_root: &Utf8Path,
        selection: &ProviderSelection,
    ) -> Result<ProviderMetadata, CliError> {
        super::resolution::resolve_selection(project_root, selection)
    }
}

pub(super) trait ProviderDiscovery {
    fn discover(
        &self,
        package: &str,
        version: &GleamVersion,
    ) -> Result<Vec<ProviderCandidate>, CliError>;
}

pub(super) struct RegistryProviderDiscovery<'registry> {
    registry: &'registry dyn ProviderRegistry,
}

impl<'registry> RegistryProviderDiscovery<'registry> {
    pub(super) fn new(registry: &'registry dyn ProviderRegistry) -> Self {
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

pub(super) struct ProviderReconciler<'a> {
    resolver: &'a dyn ApprovedProviderResolver,
    discovery: &'a dyn ProviderDiscovery,
    approval: &'a mut dyn ProviderApproval,
}

impl<'a> ProviderReconciler<'a> {
    pub(super) fn new(
        resolver: &'a dyn ApprovedProviderResolver,
        discovery: &'a dyn ProviderDiscovery,
        approval: &'a mut dyn ProviderApproval,
    ) -> Self {
        Self {
            resolver,
            discovery,
            approval,
        }
    }

    fn reconcile_packages(
        &mut self,
        project_root: &Utf8Path,
        project: &ResolvedProject,
        required_packages: BTreeSet<String>,
        managed: &mut ManagedProject,
    ) -> Result<(), CliError> {
        managed.retain_packages(&project.package_names());
        let mut compatible = BTreeSet::new();
        let mut pending = BTreeMap::new();

        for (package, version) in project.packages() {
            let Some(selection) = managed.provider(package).cloned() else {
                continue;
            };
            if super::is_built_in_package(package) {
                return Err(CliError::BuiltInProviderPackage {
                    package: package.to_owned(),
                });
            }
            let metadata = self.resolver.resolve(project_root, &selection)?;
            validate_selected_metadata(&selection, &metadata)?;
            if metadata.supports(version) {
                compatible.insert(package.to_owned());
            } else {
                pending.insert(
                    package.to_owned(),
                    PendingProvider {
                        package: package.to_owned(),
                        version: version.clone(),
                        replacing: Some(selection.crate_name().to_owned()),
                    },
                );
            }
        }

        for (package, version) in project.packages() {
            if !required_packages.contains(package)
                || super::is_built_in_package(package)
                || compatible.contains(package)
                || pending.contains_key(package)
            {
                continue;
            }
            pending.insert(
                package.to_owned(),
                PendingProvider {
                    package: package.to_owned(),
                    version: version.clone(),
                    replacing: None,
                },
            );
        }

        let mut discovered = Vec::new();
        for pending in pending.into_values() {
            let candidates = self
                .discovery
                .discover(&pending.package, &pending.version)?;
            if candidates.is_empty() {
                return Err(CliError::ProviderCandidatesUnavailable {
                    package: pending.package,
                    version: pending.version.to_string(),
                    details: "registry returned no candidates".to_owned(),
                });
            }
            discovered.push((pending, candidates));
        }

        let mut approved = Vec::new();
        for (pending, candidates) in discovered {
            let candidate = self.approval.approve(
                &pending.package,
                &pending.version,
                pending.replacing.as_deref(),
                &candidates,
            )?;
            approved.push(ProviderSelection::new(
                pending.package,
                candidate.crate_name().to_owned(),
                ProviderSource::Registry {
                    version: candidate.version().clone(),
                },
            ));
        }

        for selection in approved {
            managed.replace(selection);
        }
        Ok(())
    }
}

impl ProviderSelectionReconciler for ProviderReconciler<'_> {
    fn reconcile(
        &mut self,
        project_root: &Utf8Path,
        project: &ResolvedProject,
        program: &geam::TypedProgram,
        managed: &mut ManagedProject,
    ) -> Result<(), CliError> {
        let required_packages = geam::required_host_functions(program)
            .into_iter()
            .map(|requirement| requirement.package().to_string())
            .collect();
        self.reconcile_packages(project_root, project, required_packages, managed)
    }
}

struct PendingProvider {
    package: String,
    version: GleamVersion,
    replacing: Option<String>,
}

fn validate_selected_metadata(
    selection: &ProviderSelection,
    metadata: &ProviderMetadata,
) -> Result<(), CliError> {
    if metadata.crate_name() != selection.crate_name() {
        return Err(CliError::InvalidProviderMetadata {
            package: selection.crate_name().to_owned(),
            reason: format!(
                "Cargo resolved package {}, expected {}",
                metadata.crate_name(),
                selection.crate_name(),
            ),
        });
    }
    if metadata.gleam_package() != selection.gleam_package() {
        return Err(CliError::InvalidProviderMetadata {
            package: selection.crate_name().to_owned(),
            reason: format!(
                "provider targets Gleam package {}, expected {}",
                metadata.gleam_package(),
                selection.gleam_package(),
            ),
        });
    }
    Ok(())
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
    use super::{
        ApprovedProviderResolver, ProviderDiscovery, ProviderReconciler, RegistryProviderDiscovery,
        SystemApprovedProviderResolver, discovery_error,
    };
    use crate::error::CliError;
    use crate::project::read_resolved_project;
    use crate::provider::approval::ProviderApproval;
    use crate::provider::manifest::{ManagedProject, ProviderSelection, ProviderSource};
    use crate::provider::metadata::ProviderMetadata;
    use crate::provider::registry::{
        CandidateRejection, ProviderCandidate, ProviderRegistry, RegistryAccessError,
        RegistryDiscoveryError,
    };
    use camino::{Utf8Path, Utf8PathBuf};
    use hexpm::version::Version as GleamVersion;
    use semver::Version;
    use std::cell::RefCell;
    use std::collections::{BTreeMap, BTreeSet, VecDeque};
    use std::fs;
    use tempfile::{TempDir, tempdir};

    #[test]
    fn retains_compatible_approved_providers_and_discovers_only_missing_requirements() {
        let project = resolved_project(&[
            ("fallback", "1.0.0"),
            ("images", "1.5.0"),
            ("search", "2.0.0"),
        ]);
        let root = utf8_path(&project);
        let resolved = read_resolved_project(&root).expect("project should resolve");
        let mut managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        managed.replace(selection("fallback", "geam-fallback", "4.0.0"));
        managed.replace(selection("images", "geam-images", "1.0.0"));
        let resolver = FixedResolver::new([
            metadata("geam-fallback", "fallback", ">= 1.0.0 and < 2.0.0"),
            metadata("geam-images", "images", ">= 1.0.0 and < 2.0.0"),
        ]);
        let discovery = FixedDiscovery::new([(
            "search",
            vec![
                candidate("geam-search", "3.0.0", "search", ">= 2.0.0"),
                candidate("geam-search-alt", "9.0.0", "search", ">= 2.0.0"),
            ],
        )]);
        let mut approval = FixedApproval::new([Decision::Select("geam-search-alt")]);
        let mut reconciler = ProviderReconciler::new(&resolver, &discovery, &mut approval);

        reconciler
            .reconcile_packages(
                &root,
                &resolved,
                BTreeSet::from(["images".to_owned(), "search".to_owned()]),
                &mut managed,
            )
            .expect("missing provider should be approved");

        assert_eq!(
            resolver.calls.borrow().as_slice(),
            ["geam-fallback", "geam-images"],
        );
        assert_eq!(discovery.calls.borrow().as_slice(), ["search@2.0.0"]);
        assert_eq!(approval.calls.borrow().as_slice(), ["search:new"]);
        assert_eq!(
            managed
                .provider("images")
                .expect("compatible selection should remain")
                .source(),
            &ProviderSource::Registry {
                version: "1.0.0".parse().expect("version should parse"),
            },
        );
        let search = managed
            .provider("search")
            .expect("approved candidate should be selected");
        assert_eq!(search.crate_name(), "geam-search-alt");
        assert_eq!(
            search.source(),
            &ProviderSource::Registry {
                version: "9.0.0".parse().expect("version should parse"),
            },
        );
        assert!(managed.has_provider("fallback"));
    }

    #[test]
    fn replaces_incompatible_providers_only_after_every_approval_succeeds() {
        let project = resolved_project(&[("images", "2.0.0"), ("search", "1.0.0")]);
        let root = utf8_path(&project);
        let resolved = read_resolved_project(&root).expect("project should resolve");
        let resolver = FixedResolver::new([metadata(
            "geam-images-old",
            "images",
            ">= 1.0.0 and < 2.0.0",
        )]);
        let discovery = FixedDiscovery::new([
            (
                "images",
                vec![candidate("geam-images", "2.1.0", "images", ">= 2.0.0")],
            ),
            (
                "search",
                vec![candidate("geam-search", "1.1.0", "search", ">= 1.0.0")],
            ),
        ]);
        let mut managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        managed.replace(selection("images", "geam-images-old", "1.0.0"));
        let original = managed
            .provider("images")
            .expect("original selection should exist")
            .clone();
        let mut approval =
            FixedApproval::new([Decision::Select("geam-images"), Decision::Cancel("search")]);
        let mut reconciler = ProviderReconciler::new(&resolver, &discovery, &mut approval);

        assert!(matches!(
            reconciler.reconcile_packages(
                &root,
                &resolved,
                BTreeSet::from(["images".to_owned(), "search".to_owned()]),
                &mut managed,
            ),
            Err(CliError::ProviderApprovalCancelled { ref package }) if package == "search"
        ));
        assert_eq!(managed.provider("images"), Some(&original));
        assert!(!managed.has_provider("search"));
        assert_eq!(
            approval.calls.borrow().as_slice(),
            ["images:replace:geam-images-old", "search:new"],
        );

        let mut approval = FixedApproval::new([
            Decision::Select("geam-images"),
            Decision::Select("geam-search"),
        ]);
        let mut reconciler = ProviderReconciler::new(&resolver, &discovery, &mut approval);
        reconciler
            .reconcile_packages(
                &root,
                &resolved,
                BTreeSet::from(["images".to_owned(), "search".to_owned()]),
                &mut managed,
            )
            .expect("all approved replacements should commit together");
        assert_eq!(
            managed
                .provider("images")
                .expect("replacement should exist")
                .crate_name(),
            "geam-images",
        );
        assert!(managed.has_provider("search"));
    }

    #[test]
    fn prunes_absent_packages_and_rejects_selected_builtins() {
        let project = resolved_project(&[]);
        let root = utf8_path(&project);
        let resolved = read_resolved_project(&root).expect("project should resolve");
        let resolver = FixedResolver::new([]);
        let discovery = FixedDiscovery::new([]);
        let mut approval = FixedApproval::new([]);
        let mut managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        managed.replace(selection("removed", "geam-removed", "1.0.0"));
        let mut reconciler = ProviderReconciler::new(&resolver, &discovery, &mut approval);
        reconciler
            .reconcile_packages(&root, &resolved, BTreeSet::new(), &mut managed)
            .expect("absent package should be pruned");
        assert!(!managed.has_provider("removed"));

        let builtin = resolved_project(&[("gleam_json", "3.1.0")]);
        let builtin_root = utf8_path(&builtin);
        let builtin_resolved =
            read_resolved_project(&builtin_root).expect("built-in project should resolve");
        let mut managed = ManagedProject::load(&builtin_root, "application")
            .expect("managed project should initialize");
        managed.replace(selection("gleam_json", "geam-json-other", "1.0.0"));
        assert!(matches!(
            reconciler.reconcile_packages(
                &builtin_root,
                &builtin_resolved,
                BTreeSet::new(),
                &mut managed,
            ),
            Err(CliError::BuiltInProviderPackage { ref package }) if package == "gleam_json"
        ));
    }

    #[test]
    fn rejects_mismatched_selected_metadata_and_empty_discovery() {
        let project = resolved_project(&[("images", "1.0.0")]);
        let root = utf8_path(&project);
        let resolved = read_resolved_project(&root).expect("project should resolve");
        for metadata in [
            metadata("different-crate", "images", ">= 1.0.0"),
            metadata("geam-images", "search", ">= 1.0.0"),
        ] {
            let resolver = FixedResolver::for_selections([("geam-images", metadata)]);
            let discovery = FixedDiscovery::new([]);
            let mut approval = FixedApproval::new([]);
            let mut managed = ManagedProject::load(&root, "application")
                .expect("managed project should initialize");
            managed.replace(selection("images", "geam-images", "1.0.0"));
            let mut reconciler = ProviderReconciler::new(&resolver, &discovery, &mut approval);
            assert!(matches!(
                reconciler.reconcile_packages(
                    &root,
                    &resolved,
                    BTreeSet::new(),
                    &mut managed,
                ),
                Err(CliError::InvalidProviderMetadata { ref package, .. })
                    if package == "geam-images"
            ));
        }

        let resolver = FixedResolver::new([]);
        let discovery = FixedDiscovery::new([("images", Vec::new())]);
        let mut approval = FixedApproval::new([]);
        let mut managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        let mut reconciler = ProviderReconciler::new(&resolver, &discovery, &mut approval);
        assert!(matches!(
            reconciler.reconcile_packages(
                &root,
                &resolved,
                BTreeSet::from(["images".to_owned()]),
                &mut managed,
            ),
            Err(CliError::ProviderCandidatesUnavailable { ref details, .. })
                if details == "registry returned no candidates"
        ));
    }

    #[test]
    fn preserves_resolution_and_discovery_failures_without_changing_selections() {
        let project = resolved_project(&[("images", "1.0.0")]);
        let root = utf8_path(&project);
        let resolved = read_resolved_project(&root).expect("project should resolve");
        let mut managed =
            ManagedProject::load(&root, "application").expect("managed project should initialize");
        managed.replace(selection("images", "geam-images", "1.0.0"));
        let original = managed
            .provider("images")
            .expect("original selection should exist")
            .clone();
        let discovery = FixedDiscovery::new([]);
        let mut approval = FixedApproval::new([]);
        let mut reconciler = ProviderReconciler::new(&FailingResolver, &discovery, &mut approval);

        assert!(matches!(
            reconciler.reconcile_packages(
                &root,
                &resolved,
                BTreeSet::new(),
                &mut managed,
            ),
            Err(CliError::InvalidProviderMetadata { ref reason, .. })
                if reason == "fixture resolution failure"
        ));
        assert_eq!(managed.provider("images"), Some(&original));

        managed
            .remove("images")
            .expect("fixture selection should be removable");
        let resolver = FixedResolver::new([]);
        let mut reconciler = ProviderReconciler::new(&resolver, &FailingDiscovery, &mut approval);
        assert!(matches!(
            reconciler.reconcile_packages(
                &root,
                &resolved,
                BTreeSet::from(["images".to_owned()]),
                &mut managed,
            ),
            Err(CliError::ProviderRegistryAccess { ref reason, .. })
                if reason == "fixture discovery failure"
        ));
        assert!(!managed.has_provider("images"));
    }

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
        for error in [
            RegistryDiscoveryError::Protocol {
                response: "search",
                reason: "invalid".to_owned(),
            },
            RegistryDiscoveryError::SearchLimit {
                query: "geam-images".to_owned(),
                total: 101,
            },
        ] {
            assert!(matches!(
                discovery_error(package, error),
                CliError::ProviderRegistryProtocol { package, .. } if package == "images"
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
            CliError::ProviderCandidatesUnavailable { ref details, .. }
                if details == "geam-images@1.0.0: bad metadata; geam-images-alt: no usable versions"
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
            CliError::ProviderCandidatesUnavailable { ref details, .. }
                if details == "no matching crates were found"
        ));
    }

    #[test]
    fn connects_registry_and_system_metadata_adapters() {
        let registry = AccessFailureRegistry;
        let discovery = RegistryProviderDiscovery::new(&registry);
        assert!(matches!(
            discovery.discover("images", &GleamVersion::new(1, 0, 0)),
            Err(CliError::ProviderRegistryAccess { ref package, .. }) if package == "images"
        ));
        for error in [
            registry.index("geam-images"),
            registry.configuration(),
            registry.download("https://example.com/archive"),
        ] {
            assert_eq!(
                error
                    .expect_err("fixture registry operation should fail")
                    .to_string(),
                "unused fixture operation failed: offline",
            );
        }

        let provider = provider_package("geam-images", "images", ">= 1.0.0 and < 2.0.0");
        let project = tempdir().expect("temporary project should be created");
        let root = utf8_path(&project);
        let selection = ProviderSelection::new(
            "images".to_owned(),
            "geam-images".to_owned(),
            ProviderSource::Path {
                path: utf8_path(&provider),
            },
        );
        let metadata = SystemApprovedProviderResolver
            .resolve(&root, &selection)
            .expect("path provider should resolve through Cargo metadata");
        assert_eq!(metadata.gleam_package(), "images");
    }

    struct FixedResolver {
        metadata: BTreeMap<String, ProviderMetadata>,
        calls: RefCell<Vec<String>>,
    }

    impl FixedResolver {
        fn new<const N: usize>(metadata: [ProviderMetadata; N]) -> Self {
            Self {
                metadata: metadata
                    .into_iter()
                    .map(|metadata| (metadata.crate_name().to_owned(), metadata))
                    .collect(),
                calls: RefCell::new(Vec::new()),
            }
        }

        fn for_selections<const N: usize>(metadata: [(&str, ProviderMetadata); N]) -> Self {
            Self {
                metadata: metadata
                    .into_iter()
                    .map(|(selection, metadata)| (selection.to_owned(), metadata))
                    .collect(),
                calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl ApprovedProviderResolver for FixedResolver {
        fn resolve(
            &self,
            _project_root: &Utf8Path,
            selection: &ProviderSelection,
        ) -> Result<ProviderMetadata, CliError> {
            self.calls
                .borrow_mut()
                .push(selection.crate_name().to_owned());
            Ok(self
                .metadata
                .get(selection.crate_name())
                .expect("fixture metadata should exist")
                .clone())
        }
    }

    struct FailingResolver;

    impl ApprovedProviderResolver for FailingResolver {
        fn resolve(
            &self,
            _project_root: &Utf8Path,
            selection: &ProviderSelection,
        ) -> Result<ProviderMetadata, CliError> {
            Err(CliError::InvalidProviderMetadata {
                package: selection.crate_name().to_owned(),
                reason: "fixture resolution failure".to_owned(),
            })
        }
    }

    struct FixedDiscovery {
        candidates: BTreeMap<String, Vec<ProviderCandidate>>,
        calls: RefCell<Vec<String>>,
    }

    impl FixedDiscovery {
        fn new<const N: usize>(candidates: [(&str, Vec<ProviderCandidate>); N]) -> Self {
            Self {
                candidates: candidates
                    .into_iter()
                    .map(|(package, candidates)| (package.to_owned(), candidates))
                    .collect(),
                calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl ProviderDiscovery for FixedDiscovery {
        fn discover(
            &self,
            package: &str,
            version: &GleamVersion,
        ) -> Result<Vec<ProviderCandidate>, CliError> {
            self.calls.borrow_mut().push(format!("{package}@{version}"));
            Ok(self
                .candidates
                .get(package)
                .expect("fixture candidates should exist")
                .clone())
        }
    }

    struct FailingDiscovery;

    impl ProviderDiscovery for FailingDiscovery {
        fn discover(
            &self,
            package: &str,
            _version: &GleamVersion,
        ) -> Result<Vec<ProviderCandidate>, CliError> {
            Err(CliError::ProviderRegistryAccess {
                package: package.to_owned(),
                reason: "fixture discovery failure".to_owned(),
            })
        }
    }

    enum Decision {
        Select(&'static str),
        Cancel(&'static str),
    }

    struct FixedApproval {
        decisions: VecDeque<Decision>,
        calls: RefCell<Vec<String>>,
    }

    impl FixedApproval {
        fn new<const N: usize>(decisions: [Decision; N]) -> Self {
            Self {
                decisions: decisions.into(),
                calls: RefCell::new(Vec::new()),
            }
        }
    }

    impl ProviderApproval for FixedApproval {
        fn approve(
            &mut self,
            package: &str,
            _gleam_version: &GleamVersion,
            replacing: Option<&str>,
            candidates: &[ProviderCandidate],
        ) -> Result<ProviderCandidate, CliError> {
            self.calls.borrow_mut().push(match replacing {
                Some(provider) => format!("{package}:replace:{provider}"),
                None => format!("{package}:new"),
            });
            match self
                .decisions
                .pop_front()
                .expect("fixture decision should exist")
            {
                Decision::Select(crate_name) => Ok(candidates
                    .iter()
                    .find(|candidate| candidate.crate_name() == crate_name)
                    .expect("selected fixture candidate should exist")
                    .clone()),
                Decision::Cancel(expected_package) => {
                    assert_eq!(package, expected_package);
                    Err(CliError::ProviderApprovalCancelled {
                        package: package.to_owned(),
                    })
                }
            }
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

    fn selection(package: &str, crate_name: &str, version: &str) -> ProviderSelection {
        ProviderSelection::new(
            package.to_owned(),
            crate_name.to_owned(),
            ProviderSource::Registry {
                version: version.parse().expect("version should parse"),
            },
        )
    }

    fn candidate(crate_name: &str, version: &str, package: &str, range: &str) -> ProviderCandidate {
        ProviderCandidate::new(
            version.parse::<Version>().expect("version should parse"),
            metadata(crate_name, package, range),
        )
    }

    fn metadata(crate_name: &str, package: &str, range: &str) -> ProviderMetadata {
        let source = format!(
            "name = \"{crate_name}\"\nversion = \"1.0.0\"\n\n[metadata.geam.provider]\nschema = 1\ngleam-package = \"{package}\"\ngleam-version = \"{range}\"\n"
        );
        ProviderMetadata::from_manifest(
            crate_name,
            &source
                .parse::<toml::Table>()
                .expect("metadata should parse"),
        )
        .expect("metadata should be valid")
    }

    fn resolved_project(packages: &[(&str, &str)]) -> TempDir {
        let project = tempdir().expect("temporary project should be created");
        fs::write(
            project.path().join("gleam.toml"),
            "name = \"application\"\nversion = \"1.0.0\"\n",
        )
        .expect("Gleam config should be written");
        let packages = packages
            .iter()
            .map(|(name, version)| {
                format!(
                    "  {{ name = \"{name}\", version = \"{version}\", build_tools = [\"gleam\"], requirements = [], source = \"hex\", outer_checksum = \"00\" }},\n"
                )
            })
            .collect::<String>();
        fs::write(
            project.path().join("manifest.toml"),
            format!("packages = [\n{packages}]\n\n[requirements]\n"),
        )
        .expect("Gleam manifest should be written");
        project
    }

    fn provider_package(crate_name: &str, package: &str, range: &str) -> TempDir {
        let provider = tempdir().expect("provider directory should be created");
        fs::create_dir(provider.path().join("src"))
            .expect("provider source directory should be created");
        fs::write(
            provider.path().join("Cargo.toml"),
            format!(
                "[package]\nname = \"{crate_name}\"\nversion = \"1.0.0\"\nedition = \"2024\"\n\n[package.metadata.geam.provider]\nschema = 1\ngleam-package = \"{package}\"\ngleam-version = \"{range}\"\n"
            ),
        )
        .expect("provider manifest should be written");
        fs::write(
            provider.path().join("src/lib.rs"),
            "pub struct Component;\n",
        )
        .expect("provider source should be written");
        provider
    }

    fn utf8_path(directory: &TempDir) -> Utf8PathBuf {
        Utf8PathBuf::from_path_buf(directory.path().to_path_buf())
            .expect("temporary path should be valid UTF-8")
    }
}
