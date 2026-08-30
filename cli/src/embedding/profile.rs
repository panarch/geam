use super::boundary::PlainBindings;
use super::identifier::RustIdentifier;
use super::package::{DirectDependency, EmbeddingPackage};
use crate::builtin::BuiltInProvider;
use crate::error::CliError;
use crate::project::ResolvedProject;
use crate::provider::ProviderMetadata;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug)]
pub(super) struct HostedBindings {
    pub(super) boundary: PlainBindings,
    pub(super) components: HostedComponents,
}

#[derive(Debug)]
pub(super) struct HostedComponents {
    first: ComponentBinding,
    remaining: Vec<ComponentBinding>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum HostedCapabilities {
    None,
    Io,
    IoAndTime,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum ComponentBinding {
    Stdlib,
    Json,
    Time,
    External(ExternalComponent),
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(super) struct ExternalComponent {
    pub(super) package: String,
    pub(super) configuration_field: RustIdentifier,
    pub(super) state_field: RustIdentifier,
    pub(super) crate_alias: RustIdentifier,
}

impl HostedBindings {
    pub(super) fn resolve(
        package: &EmbeddingPackage,
        boundary: PlainBindings,
        first_required_package: &str,
        remaining_required_packages: &BTreeSet<String>,
        resolved_project: &ResolvedProject,
    ) -> Result<Self, CliError> {
        let providers = ProviderCandidates::load(package)?;
        let mut components = components_for_package(
            package,
            &providers,
            first_required_package,
            resolved_project,
        )?;
        for required_package in remaining_required_packages {
            let additional =
                components_for_package(package, &providers, required_package, resolved_project)?;
            components.extend(additional);
        }
        components.require_geam_features(package)?;
        Ok(Self {
            boundary,
            components,
        })
    }
}

impl HostedComponents {
    pub(super) fn from_builtin(provider: BuiltInProvider) -> Self {
        let closure = provider.component_closure();
        let mut components = Self::new(ComponentBinding::from(closure.first()));
        for component in closure.remaining() {
            components.insert(ComponentBinding::from(component));
        }
        components
    }

    pub(super) fn from_external(component: ExternalComponent) -> Self {
        Self::new(ComponentBinding::External(component))
    }

    fn new(first: ComponentBinding) -> Self {
        Self {
            first,
            remaining: Vec::new(),
        }
    }

    fn insert(&mut self, component: ComponentBinding) {
        if component == self.first || self.remaining.contains(&component) {
            return;
        }
        if component < self.first {
            let previous = std::mem::replace(&mut self.first, component);
            self.remaining.insert(0, previous);
            return;
        }
        let index = self
            .remaining
            .partition_point(|current| current < &component);
        self.remaining.insert(index, component);
    }

    pub(super) fn extend(&mut self, components: Self) {
        self.insert(components.first);
        for component in components.remaining {
            self.insert(component);
        }
    }

    pub(super) fn iter(&self) -> impl Iterator<Item = &ComponentBinding> {
        std::iter::once(&self.first).chain(self.remaining.iter())
    }

    pub(super) fn first(&self) -> &ComponentBinding {
        &self.first
    }

    pub(super) fn remaining(&self) -> impl Iterator<Item = &ComponentBinding> {
        self.remaining.iter()
    }

    pub(super) fn has_multiple(&self) -> bool {
        !self.remaining.is_empty()
    }

    pub(super) fn capabilities(&self) -> HostedCapabilities {
        if self.has_time() {
            HostedCapabilities::IoAndTime
        } else if self.has_stdlib() {
            HostedCapabilities::Io
        } else {
            HostedCapabilities::None
        }
    }

    pub(super) fn has_stdlib(&self) -> bool {
        self.iter()
            .any(|component| component == &ComponentBinding::Stdlib)
    }

    pub(super) fn has_time(&self) -> bool {
        self.iter()
            .any(|component| component == &ComponentBinding::Time)
    }

    pub(super) fn has_external(&self) -> bool {
        self.iter()
            .any(|component| matches!(component, ComponentBinding::External(_)))
    }

    fn require_geam_features(&self, package: &EmbeddingPackage) -> Result<(), CliError> {
        for component in self.iter() {
            let Some(provider) = component.built_in() else {
                continue;
            };
            package.require_geam_feature(
                provider.geam_feature(),
                &format!(
                    "because the selected source closure requires Gleam package `{}`",
                    provider.package(),
                ),
            )?;
        }
        Ok(())
    }
}

fn components_for_package(
    package: &EmbeddingPackage,
    providers: &ProviderCandidates,
    required_package: &str,
    resolved_project: &ResolvedProject,
) -> Result<HostedComponents, CliError> {
    if let Some(built_in) = BuiltInProvider::from_package(required_package) {
        return Ok(HostedComponents::from_builtin(built_in));
    }

    providers
        .select(package, required_package, resolved_project)
        .map(HostedComponents::from_external)
}

impl From<BuiltInProvider> for ComponentBinding {
    fn from(provider: BuiltInProvider) -> Self {
        match provider {
            BuiltInProvider::Stdlib => Self::Stdlib,
            BuiltInProvider::Json => Self::Json,
            BuiltInProvider::Time => Self::Time,
        }
    }
}

impl ComponentBinding {
    fn built_in(&self) -> Option<BuiltInProvider> {
        match self {
            Self::Stdlib => Some(BuiltInProvider::Stdlib),
            Self::Json => Some(BuiltInProvider::Json),
            Self::Time => Some(BuiltInProvider::Time),
            Self::External(_) => None,
        }
    }
}

struct ProviderCandidates<'metadata> {
    by_gleam_package: BTreeMap<String, Vec<ProviderCandidate<'metadata>>>,
}

struct ProviderCandidate<'metadata> {
    dependency: &'metadata DirectDependency,
    metadata: ProviderMetadata,
}

impl<'metadata> ProviderCandidates<'metadata> {
    fn load(package: &'metadata EmbeddingPackage) -> Result<Self, CliError> {
        let mut by_gleam_package = BTreeMap::<String, Vec<ProviderCandidate<'_>>>::new();
        for dependency in package.direct_dependencies() {
            let metadata =
                ProviderMetadata::from_optional_package(&dependency.package).map_err(|reason| {
                    provider_error(
                        package,
                        dependency.package.name.as_str(),
                        format!(
                            "provider metadata in {} is invalid: {reason}",
                            dependency.package.manifest_path
                        ),
                    )
                })?;
            let Some(metadata) = metadata else {
                continue;
            };
            by_gleam_package
                .entry(metadata.gleam_package().to_owned())
                .or_default()
                .push(ProviderCandidate {
                    dependency,
                    metadata,
                });
        }
        Ok(Self { by_gleam_package })
    }

    fn select(
        &self,
        package: &EmbeddingPackage,
        required_package: &str,
        resolved_project: &ResolvedProject,
    ) -> Result<ExternalComponent, CliError> {
        let candidates = self
            .by_gleam_package
            .get(required_package)
            .map(Vec::as_slice)
            .unwrap_or_default();
        let candidate = match candidates {
            [candidate] => candidate,
            [] => {
                return Err(provider_error(
                    package,
                    required_package,
                    "no enabled direct provider dependency targets the required Gleam package",
                ));
            }
            candidates => {
                return Err(provider_error(
                    package,
                    required_package,
                    format!(
                        "multiple enabled direct providers target the package through aliases: {}",
                        candidates
                            .iter()
                            .map(|candidate| candidate.dependency.alias.as_str())
                            .collect::<Vec<_>>()
                            .join(", ")
                    ),
                ));
            }
        };
        let version = resolved_project
            .package_version(required_package)
            .ok_or_else(|| {
                provider_error(
                    package,
                    required_package,
                    "the resolved Gleam project does not contain the required package",
                )
            })?;
        if !candidate.metadata.supports(version) {
            return Err(provider_error(
                package,
                required_package,
                format!(
                    "provider crate {} does not support resolved Gleam version {} (declared range {})",
                    candidate.metadata.crate_name(),
                    version,
                    candidate.metadata.gleam_range(),
                ),
            ));
        }
        verify_provider_geam_identity(package, candidate)?;

        let configuration_field = RustIdentifier::from_compiled_package(required_package);
        let state_field = configuration_field.with_prefix("provider_");
        let alias = candidate.dependency.alias.as_str();
        let crate_alias = RustIdentifier::crate_alias(alias).map_err(|reason| {
            provider_error(
                package,
                required_package,
                format!("Cargo alias `{alias}` is unusable in generated Rust: {reason}"),
            )
        })?;
        Ok(ExternalComponent {
            package: required_package.to_owned(),
            configuration_field,
            state_field,
            crate_alias,
        })
    }
}

fn verify_provider_geam_identity(
    package: &EmbeddingPackage,
    candidate: &ProviderCandidate<'_>,
) -> Result<(), CliError> {
    let provider_geam = match candidate.dependency.geam_dependencies.as_slice() {
        [dependency] => &dependency.package_id,
        [] => {
            return Err(provider_error(
                package,
                candidate.metadata.gleam_package(),
                format!(
                    "provider crate {} has no enabled direct normal dependency on package `geam`",
                    candidate.metadata.crate_name(),
                ),
            ));
        }
        dependencies => {
            return Err(provider_error(
                package,
                candidate.metadata.gleam_package(),
                format!(
                    "provider crate {} resolves multiple Geam dependencies through aliases: {}",
                    candidate.metadata.crate_name(),
                    dependencies
                        .iter()
                        .map(|dependency| dependency.alias.as_str())
                        .collect::<Vec<_>>()
                        .join(", "),
                ),
            ));
        }
    };
    if provider_geam != package.geam_package_id() {
        return Err(provider_error(
            package,
            candidate.metadata.gleam_package(),
            format!(
                "provider crate {} resolves Geam as {}, but the embedding package resolves {}",
                candidate.metadata.crate_name(),
                provider_geam,
                package.geam_package_id(),
            ),
        ));
    }
    Ok(())
}

fn provider_error(
    package: &EmbeddingPackage,
    affected_package: impl Into<String>,
    reason: impl Into<String>,
) -> CliError {
    CliError::InvalidEmbeddingProvider {
        package: affected_package.into(),
        manifest: package.manifest().to_path_buf(),
        reason: reason.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        ComponentBinding, ExternalComponent, HostedBindings, HostedCapabilities, HostedComponents,
    };
    use crate::builtin::BuiltInProvider;
    use crate::embedding::boundary::{FunctionBinding, PlainBindings, Scalar};
    use crate::embedding::identifier::RustIdentifier;
    use crate::embedding::package::EmbeddingPackage;
    use crate::error::CliError;
    use crate::project::read_existing_resolved_project;
    use camino::{Utf8Path, Utf8PathBuf};
    use std::collections::BTreeSet;
    use std::fs;
    use std::process::{Command, Output};
    use tempfile::{TempDir, tempdir};

    #[test]
    fn expands_only_the_required_built_in_component_closure() {
        let stdlib = HostedComponents::from_builtin(BuiltInProvider::Stdlib);
        assert_eq!(
            stdlib.iter().collect::<Vec<_>>(),
            [&ComponentBinding::Stdlib]
        );
        assert_eq!(stdlib.capabilities(), HostedCapabilities::Io);

        let json = HostedComponents::from_builtin(BuiltInProvider::Json);
        assert_eq!(
            json.iter().collect::<Vec<_>>(),
            [&ComponentBinding::Stdlib, &ComponentBinding::Json],
        );
        assert_eq!(json.capabilities(), HostedCapabilities::Io);

        let time = HostedComponents::from_builtin(BuiltInProvider::Time);
        assert_eq!(
            time.iter().collect::<Vec<_>>(),
            [&ComponentBinding::Stdlib, &ComponentBinding::Time],
        );
        assert_eq!(time.capabilities(), HostedCapabilities::IoAndTime);
    }

    #[test]
    fn orders_and_deduplicates_mixed_components_without_weakening_dependencies() {
        let mut components = HostedComponents::from_external(ExternalComponent {
            package: "example_text_pattern".to_owned(),
            configuration_field: identifier("example_text_pattern"),
            state_field: identifier("provider_example_text_pattern"),
            crate_alias: identifier("patterns"),
        });
        components.extend(HostedComponents::from_builtin(BuiltInProvider::Time));
        components.extend(HostedComponents::from_builtin(BuiltInProvider::Stdlib));

        assert_eq!(
            components.iter().collect::<Vec<_>>(),
            [
                &ComponentBinding::Stdlib,
                &ComponentBinding::Time,
                &ComponentBinding::External(ExternalComponent {
                    package: "example_text_pattern".to_owned(),
                    configuration_field: identifier("example_text_pattern"),
                    state_field: identifier("provider_example_text_pattern"),
                    crate_alias: identifier("patterns"),
                }),
            ],
        );
        assert_eq!(components.capabilities(), HostedCapabilities::IoAndTime);
    }

    #[test]
    fn selects_one_compatible_direct_provider_by_actual_alias_and_ignores_unused_providers() {
        let fixture = ProviderGraphFixture::new(
            vec![
                ProviderSpec::valid("patterns", "pattern-provider", "images", ">= 1.0.0"),
                ProviderSpec::valid(
                    "unused_provider",
                    "unused-provider",
                    "unused_package",
                    ">= 1.0.0",
                ),
            ],
            Some("1.2.0"),
        );

        let hosted = fixture
            .resolve("images")
            .expect("one compatible direct provider should resolve");
        assert_eq!(
            hosted.components.iter().collect::<Vec<_>>(),
            [&ComponentBinding::External(ExternalComponent {
                package: "images".to_owned(),
                configuration_field: identifier("images"),
                state_field: identifier("provider_images"),
                crate_alias: identifier("patterns"),
            })],
        );

        let error = fixture
            .resolve_with_remaining("images", &BTreeSet::from(["missing_package".to_owned()]))
            .expect_err("an unresolved additional package should fail");
        assert!(matches!(
            error,
            CliError::InvalidEmbeddingProvider { package, reason, .. }
                if package == "missing_package"
                    && reason.contains("no enabled direct provider dependency")
        ));
    }

    #[test]
    fn rejects_invalid_external_provider_graphs_before_rendering() {
        let cases = [
            (
                ProviderGraphFixture::new(vec![], Some("1.2.0")),
                "no enabled direct provider dependency",
            ),
            (
                ProviderGraphFixture::new(
                    vec![
                        ProviderSpec::valid("patterns", "pattern-provider", "images", ">= 1.0.0")
                            .transitive(),
                    ],
                    Some("1.2.0"),
                ),
                "no enabled direct provider dependency",
            ),
            (
                ProviderGraphFixture::new(
                    vec![
                        ProviderSpec::valid(
                            "first_patterns",
                            "first-pattern-provider",
                            "images",
                            ">= 1.0.0",
                        ),
                        ProviderSpec::valid(
                            "second_patterns",
                            "second-pattern-provider",
                            "images",
                            ">= 1.0.0",
                        ),
                    ],
                    Some("1.2.0"),
                ),
                "multiple enabled direct providers",
            ),
            (
                ProviderGraphFixture::new(
                    vec![ProviderSpec::valid(
                        "patterns",
                        "pattern-provider",
                        "images",
                        "< 1.0.0",
                    )],
                    Some("1.2.0"),
                ),
                "does not support resolved Gleam version 1.2.0",
            ),
            (
                ProviderGraphFixture::new(
                    vec![
                        ProviderSpec::valid("patterns", "pattern-provider", "images", ">= 1.0.0")
                            .without_geam(),
                    ],
                    Some("1.2.0"),
                ),
                "has no enabled direct normal dependency on package `geam`",
            ),
            (
                ProviderGraphFixture::new(
                    vec![
                        ProviderSpec::valid("patterns", "pattern-provider", "images", ">= 1.0.0")
                            .with_multiple_geam(),
                    ],
                    Some("1.2.0"),
                ),
                "resolves multiple Geam dependencies",
            ),
            (
                ProviderGraphFixture::new(
                    vec![
                        ProviderSpec::valid("patterns", "pattern-provider", "images", ">= 1.0.0")
                            .with_split_geam(),
                    ],
                    Some("1.2.0"),
                ),
                "but the embedding package resolves",
            ),
            (
                ProviderGraphFixture::new(
                    vec![ProviderSpec::malformed(
                        "patterns",
                        "pattern-provider",
                        "images",
                    )],
                    Some("1.2.0"),
                ),
                "provider metadata",
            ),
            (
                ProviderGraphFixture::new(
                    vec![ProviderSpec::valid(
                        "self",
                        "pattern-provider",
                        "images",
                        ">= 1.0.0",
                    )],
                    Some("1.2.0"),
                ),
                "Cargo alias `self` is unusable",
            ),
            (
                ProviderGraphFixture::new(
                    vec![ProviderSpec::valid(
                        "patterns",
                        "pattern-provider",
                        "images",
                        ">= 1.0.0",
                    )],
                    None,
                ),
                "resolved Gleam project does not contain",
            ),
        ];

        for (fixture, expected) in cases {
            let error = fixture
                .resolve("images")
                .expect_err("invalid provider graph should fail");
            assert!(
                matches!(
                    &error,
                    CliError::InvalidEmbeddingProvider { manifest, reason, .. }
                        if manifest == &fixture.application.join("Cargo.toml")
                            && reason.contains(expected)
                ),
                "expected {expected}: {error}",
            );
        }
    }

    fn identifier(value: &str) -> RustIdentifier {
        RustIdentifier::parse(value).expect("fixture identifier should be valid")
    }

    #[derive(Clone, Copy)]
    enum ProviderGeam {
        Application,
        Missing,
        Split,
        Multiple,
    }

    struct ProviderSpec<'fixture> {
        alias: &'fixture str,
        crate_name: &'fixture str,
        gleam_package: &'fixture str,
        gleam_range: &'fixture str,
        direct: bool,
        schema: i64,
        geam: ProviderGeam,
    }

    impl<'fixture> ProviderSpec<'fixture> {
        fn valid(
            alias: &'fixture str,
            crate_name: &'fixture str,
            gleam_package: &'fixture str,
            gleam_range: &'fixture str,
        ) -> Self {
            Self {
                alias,
                crate_name,
                gleam_package,
                gleam_range,
                direct: true,
                schema: 1,
                geam: ProviderGeam::Application,
            }
        }

        fn malformed(
            alias: &'fixture str,
            crate_name: &'fixture str,
            gleam_package: &'fixture str,
        ) -> Self {
            Self {
                schema: 2,
                ..Self::valid(alias, crate_name, gleam_package, ">= 1.0.0")
            }
        }

        fn transitive(mut self) -> Self {
            self.direct = false;
            self
        }

        fn without_geam(mut self) -> Self {
            self.geam = ProviderGeam::Missing;
            self
        }

        fn with_split_geam(mut self) -> Self {
            self.geam = ProviderGeam::Split;
            self
        }

        fn with_multiple_geam(mut self) -> Self {
            self.geam = ProviderGeam::Multiple;
            self
        }
    }

    struct ProviderGraphFixture {
        _directory: TempDir,
        application: Utf8PathBuf,
    }

    impl ProviderGraphFixture {
        fn new<'fixture>(
            providers: Vec<ProviderSpec<'fixture>>,
            gleam_version: Option<&str>,
        ) -> Self {
            let directory = tempdir().expect("temporary directory should be created");
            let root = Utf8PathBuf::from_path_buf(
                fs::canonicalize(directory.path()).expect("temporary path should canonicalize"),
            )
            .expect("temporary path should be valid UTF-8");
            let repository = Utf8PathBuf::from(env!("CARGO_MANIFEST_DIR"))
                .parent()
                .map(Utf8Path::to_path_buf)
                .expect("CLI package should be inside the repository");
            let application = root.join("application");
            fs::create_dir_all(application.join("src"))
                .expect("application source directory should be created");
            fs::create_dir_all(application.join("gleam"))
                .expect("Gleam project directory should be created");
            fs::write(application.join("src/lib.rs"), "")
                .expect("application source should be written");

            let mut dependencies = vec![format!(
                "runtime = {{ package = \"geam\", path = {repository:?}, default-features = false, features = [\"embedding\"] }}"
            )];
            let split_geam = root.join("split-geam");
            for provider in providers {
                let provider_root = root.join(provider.crate_name);
                fs::create_dir_all(provider_root.join("src"))
                    .expect("provider source directory should be created");
                fs::write(provider_root.join("src/lib.rs"), "")
                    .expect("provider source should be written");
                let geam_dependency = match provider.geam {
                    ProviderGeam::Application => {
                        format!(
                            "\n[dependencies]\ngeam = {{ path = {repository:?}, default-features = false, features = [\"provider\"] }}\n"
                        )
                    }
                    ProviderGeam::Missing => String::new(),
                    ProviderGeam::Split => {
                        write_split_geam(&split_geam);
                        format!(
                            "\n[dependencies]\ngeam = {{ path = {split_geam:?}, default-features = false, features = [\"provider\"] }}\n"
                        )
                    }
                    ProviderGeam::Multiple => {
                        write_split_geam(&split_geam);
                        format!(
                            "\n[dependencies]\ngeam = {{ path = {repository:?}, default-features = false, features = [\"provider\"] }}\nsplit_geam = {{ package = \"geam\", path = {split_geam:?}, default-features = false, features = [\"provider\"] }}\n"
                        )
                    }
                };
                fs::write(
                    provider_root.join("Cargo.toml"),
                    format!(
                        r#"[package]
name = {:?}
version = "1.0.0"
edition = "2024"

[package.metadata.geam.provider]
schema = {}
gleam-package = {:?}
gleam-version = {:?}
{geam_dependency}
[workspace]
resolver = "3"
"#,
                        provider.crate_name,
                        provider.schema,
                        provider.gleam_package,
                        provider.gleam_range,
                    ),
                )
                .expect("provider manifest should be written");

                if provider.direct {
                    dependencies.push(format!(
                        "{} = {{ package = {:?}, path = {provider_root:?} }}",
                        provider.alias, provider.crate_name,
                    ));
                } else {
                    let wrapper_name = format!("{}-wrapper", provider.crate_name);
                    let wrapper = root.join(&wrapper_name);
                    fs::create_dir_all(wrapper.join("src"))
                        .expect("wrapper source directory should be created");
                    fs::write(wrapper.join("src/lib.rs"), "")
                        .expect("wrapper source should be written");
                    fs::write(
                        wrapper.join("Cargo.toml"),
                        format!(
                            r#"[package]
name = {wrapper_name:?}
version = "1.0.0"
edition = "2024"

[dependencies]
provider = {{ package = {:?}, path = {provider_root:?} }}

[workspace]
resolver = "3"
"#,
                            provider.crate_name,
                        ),
                    )
                    .expect("wrapper manifest should be written");
                    dependencies.push(format!(
                        "{}_wrapper = {{ package = {wrapper_name:?}, path = {wrapper:?} }}",
                        provider.alias,
                    ));
                }
            }
            fs::write(
                application.join("Cargo.toml"),
                format!(
                    r#"[package]
name = "embedding-application"
version = "1.0.0"
edition = "2024"

[dependencies]
{}

[package.metadata.geam.embedding]
project = "gleam"
module = "boundary"

[workspace]
resolver = "3"
"#,
                    dependencies.join("\n"),
                ),
            )
            .expect("application manifest should be written");
            fs::write(
                application.join("gleam/gleam.toml"),
                "name = \"embedding_application\"\nversion = \"1.0.0\"\n",
            )
            .expect("Gleam config should be written");
            let manifest = gleam_version.map_or_else(
                || "packages = []\n\n[requirements]\n".to_owned(),
                |version| {
                    format!(
                        "packages = [{{ name = \"images\", version = {version:?}, build_tools = [], requirements = [], source = \"local\", path = \"images\" }}]\n\n[requirements]\nimages = {{ path = \"images\" }}\n"
                    )
                },
            );
            fs::write(application.join("gleam/manifest.toml"), manifest)
                .expect("Gleam manifest should be written");

            assert_success(
                Command::new("cargo")
                    .arg("generate-lockfile")
                    .arg("--manifest-path")
                    .arg(application.join("Cargo.toml"))
                    .arg("--offline")
                    .current_dir(&application),
                "provider graph lockfile generation",
            );
            Self {
                _directory: directory,
                application,
            }
        }

        fn resolve(&self, required_package: &str) -> Result<HostedBindings, CliError> {
            self.resolve_with_remaining(required_package, &BTreeSet::new())
        }

        fn resolve_with_remaining(
            &self,
            required_package: &str,
            remaining_packages: &BTreeSet<String>,
        ) -> Result<HostedBindings, CliError> {
            let package = EmbeddingPackage::load(&self.application, None)
                .expect("fixture embedding package should load");
            let project = read_existing_resolved_project(package.project_root())
                .expect("fixture Gleam project should be resolved");
            HostedBindings::resolve(
                &package,
                PlainBindings {
                    geam_alias: package.geam_alias().clone(),
                    root_module: "boundary".to_owned(),
                    first: FunctionBinding {
                        gleam_name: "ready".to_owned(),
                        rust_name: identifier("ready"),
                        arguments: Vec::new(),
                        return_type: Scalar::Bool,
                    },
                    remaining: Vec::new(),
                },
                required_package,
                remaining_packages,
                &project,
            )
        }
    }

    fn write_split_geam(root: &Utf8Path) {
        fs::create_dir_all(root.join("src"))
            .expect("split Geam source directory should be created");
        fs::write(root.join("src/lib.rs"), "").expect("split Geam source should be written");
        fs::write(
            root.join("Cargo.toml"),
            r#"[package]
name = "geam"
version = "9.9.9"
edition = "2024"

[features]
provider = []

[workspace]
resolver = "3"
"#,
        )
        .expect("split Geam manifest should be written");
    }

    fn assert_success(command: &mut Command, operation: &str) {
        let Output {
            status,
            stdout,
            stderr,
        } = command.output().expect("fixture command should start");
        let stdout = String::from_utf8_lossy(&stdout);
        let stderr = String::from_utf8_lossy(&stderr);
        assert!(
            status.success(),
            "{operation} failed\nstdout:\n{stdout}\nstderr:\n{stderr}",
        );
    }
}
