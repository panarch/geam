use camino::Utf8PathBuf;
use ecow::EcoString;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ModuleSource {
    module: EcoString,
    path: Utf8PathBuf,
    source: String,
}

impl ModuleSource {
    pub fn new(
        module: impl Into<EcoString>,
        path: impl Into<Utf8PathBuf>,
        source: impl Into<String>,
    ) -> Self {
        Self {
            module: module.into(),
            path: path.into(),
            source: source.into(),
        }
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn path(&self) -> &Utf8PathBuf {
        &self.path
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub(super) fn into_parts(self) -> (EcoString, Utf8PathBuf, String) {
        (self.module, self.path, self.source)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PackageSource {
    package: EcoString,
    direct_dependencies: Box<[EcoString]>,
    modules: Box<[ModuleSource]>,
}

impl PackageSource {
    pub fn new<Dependency>(
        package: impl Into<EcoString>,
        direct_dependencies: impl IntoIterator<Item = Dependency>,
        modules: impl IntoIterator<Item = ModuleSource>,
    ) -> Self
    where
        Dependency: Into<EcoString>,
    {
        Self {
            package: package.into(),
            direct_dependencies: direct_dependencies.into_iter().map(Into::into).collect(),
            modules: modules.into_iter().collect(),
        }
    }

    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn direct_dependencies(&self) -> &[EcoString] {
        &self.direct_dependencies
    }

    pub fn modules(&self) -> &[ModuleSource] {
        &self.modules
    }

    pub(super) fn into_parts(self) -> (EcoString, Box<[EcoString]>, Box<[ModuleSource]>) {
        (self.package, self.direct_dependencies, self.modules)
    }
}

#[cfg(test)]
mod tests {
    use super::{ModuleSource, PackageSource};

    #[test]
    fn module_source_exposes_owned_source_parts() {
        let source = ModuleSource::new("main", "src/main.gleam", "pub fn main() { 1 }");

        assert_eq!(source.module(), "main");
        assert_eq!(source.path().as_str(), "src/main.gleam");
        assert_eq!(source.source(), "pub fn main() { 1 }");
    }

    #[test]
    fn package_source_exposes_package_dependencies_and_modules() {
        let source = PackageSource::new(
            "application",
            ["library"],
            [ModuleSource::new(
                "main",
                "src/main.gleam",
                "pub fn main() { 1 }",
            )],
        );

        assert_eq!(source.package(), "application");
        assert_eq!(source.direct_dependencies(), ["library"]);
        assert_eq!(source.modules().len(), 1);
        assert_eq!(source.modules()[0].module(), "main");
    }
}
