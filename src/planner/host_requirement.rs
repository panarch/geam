use crate::frontend::TypedProgram;
use ecow::EcoString;
use gleam_core::ast::TypedFunction;
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RequiredHostFunction {
    package: EcoString,
    module: EcoString,
    function: EcoString,
}

impl RequiredHostFunction {
    pub fn package(&self) -> &EcoString {
        &self.package
    }

    pub fn module(&self) -> &EcoString {
        &self.module
    }

    pub fn function(&self) -> &EcoString {
        &self.function
    }
}

pub fn required_host_functions(program: &TypedProgram) -> Vec<RequiredHostFunction> {
    program
        .modules()
        .flat_map(|module| {
            module
                .definitions
                .functions
                .iter()
                .filter(|function| requires_erlang_host_provider(function))
                .filter_map(|function| {
                    function
                        .name
                        .as_ref()
                        .map(|(_, name)| RequiredHostFunction {
                            package: module.type_info.package.clone(),
                            module: module.name.clone(),
                            function: name.clone(),
                        })
                })
        })
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect()
}

pub(crate) fn requires_erlang_host_provider(function: &TypedFunction) -> bool {
    function.external_erlang.is_some() && function.body.is_empty()
}

#[cfg(test)]
mod tests {
    use super::{RequiredHostFunction, required_host_functions};
    use crate::frontend::{
        ModuleSource, PackageSource, compile_typed_package_program, compile_typed_project,
    };
    use camino::Utf8PathBuf;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn inventories_only_bodyless_erlang_externals_in_deterministic_order() {
        let program = compile_typed_package_program(
            "application",
            "main",
            [
                PackageSource::new(
                    "application",
                    ["library"],
                    [ModuleSource::new(
                        "main",
                        "main.gleam",
                        r#"
@external(erlang, "native", "root_required")
fn root_required() -> Int

pub fn main() {
  1
}
"#,
                    )],
                ),
                PackageSource::new(
                    "library",
                    Vec::<String>::new(),
                    [
                        ModuleSource::new(
                            "zeta",
                            "zeta.gleam",
                            r#"
@external(erlang, "native", "private_required")
fn private_required() -> Int

@external(erlang, "native", "fallback")
pub fn fallback() -> Int {
  1
}

@external(javascript, "./ffi.mjs", "javascript_only")
fn javascript_only() -> Int

pub type OrdinaryConstructorless
"#,
                        ),
                        ModuleSource::new(
                            "alpha",
                            "alpha.gleam",
                            r#"
@external(erlang, "native", "dependency_required")
pub fn dependency_required() -> Int
"#,
                        ),
                    ],
                ),
            ],
        )
        .expect("host requirements should compile independently of providers");

        assert_eq!(
            required_host_functions(&program),
            [
                RequiredHostFunction {
                    package: "application".into(),
                    module: "main".into(),
                    function: "root_required".into(),
                },
                RequiredHostFunction {
                    package: "library".into(),
                    module: "alpha".into(),
                    function: "dependency_required".into(),
                },
                RequiredHostFunction {
                    package: "library".into(),
                    module: "zeta".into(),
                    function: "private_required".into(),
                },
            ],
        );
    }

    #[test]
    fn exposes_owned_requirement_identity() {
        let requirement = RequiredHostFunction {
            package: "package".into(),
            module: "module".into(),
            function: "function".into(),
        };

        assert_eq!(requirement.package(), "package");
        assert_eq!(requirement.module(), "module");
        assert_eq!(requirement.function(), "function");
    }

    #[test]
    fn follows_the_resolved_project_source_closure() {
        let project = tempdir().expect("temporary project should be created");
        let root = project.path();
        fs::create_dir(root.join("src")).expect("source directory should be created");
        fs::write(
            root.join("gleam.toml"),
            "name = \"application\"\nversion = \"1.0.0\"\n",
        )
        .expect("project config should be written");
        fs::write(
            root.join("manifest.toml"),
            "packages = []\n\n[requirements]\n",
        )
        .expect("project manifest should be written");
        fs::write(
            root.join("src/main.gleam"),
            "import used\npub fn main() { 1 }",
        )
        .expect("root module should be written");
        fs::write(
            root.join("src/used.gleam"),
            r#"
@external(erlang, "native", "used")
pub fn used() -> Int
"#,
        )
        .expect("selected module should be written");
        fs::write(
            root.join("src/unused.gleam"),
            r#"
@external(erlang, "native", "unused")
pub fn unused() -> Int
"#,
        )
        .expect("unselected module should be written");

        let root = Utf8PathBuf::from_path_buf(root.to_path_buf())
            .expect("temporary path should be valid UTF-8");
        let program = compile_typed_project(root, "main")
            .expect("resolved project source closure should compile");

        assert_eq!(
            program
                .modules()
                .map(|module| module.name.as_str())
                .collect::<Vec<_>>(),
            ["used", "main"],
        );
        assert_eq!(
            required_host_functions(&program),
            [RequiredHostFunction {
                package: "application".into(),
                module: "used".into(),
                function: "used".into(),
            }],
        );
    }
}
