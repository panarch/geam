use crate::builtin::BuiltInProvider;

pub(super) fn render_source(provider_aliases: &[String]) -> String {
    render_host(provider_aliases) + RUNNER_TEMPLATE
}

pub(super) fn render_application(provider_aliases: &[String]) -> String {
    let packages = runner_components(provider_aliases)
        .into_iter()
        .filter_map(|component| match component.initialization {
            ComponentInitialization::Configured { package } => Some(package),
            _ => None,
        })
        .collect::<Vec<_>>();
    render_host(provider_aliases)
        + &APPLICATION_TEMPLATE.replace("__PROVIDER_PACKAGES__", &format!("{packages:?}"))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct RunnerComponent {
    field: String,
    type_path: String,
    initialization: ComponentInitialization,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ComponentInitialization {
    Stdlib,
    Unit,
    SystemTime,
    PackageResources,
    Configured { package: String },
}

impl RunnerComponent {
    fn built_in(provider: BuiltInProvider) -> Self {
        match provider {
            BuiltInProvider::Stdlib => Self {
                field: "stdlib".to_owned(),
                type_path: "geam::gleam_stdlib::Component<CliIoSink>".to_owned(),
                initialization: ComponentInitialization::Stdlib,
            },
            BuiltInProvider::Json => Self {
                field: "json".to_owned(),
                type_path: "geam::gleam_json::Component".to_owned(),
                initialization: ComponentInitialization::Unit,
            },
            BuiltInProvider::Time => Self {
                field: "time".to_owned(),
                type_path: "geam::gleam_time::Component".to_owned(),
                initialization: ComponentInitialization::SystemTime,
            },
            BuiltInProvider::Erlang => Self {
                field: "erlang".to_owned(),
                type_path: "geam::gleam_erlang::Component<Profile>".to_owned(),
                initialization: ComponentInitialization::PackageResources,
            },
            BuiltInProvider::Geam => Self {
                field: "future".to_owned(),
                type_path: "geam::FutureComponent".to_owned(),
                initialization: ComponentInitialization::Unit,
            },
        }
    }

    fn external(alias: String) -> Self {
        Self {
            field: alias.clone(),
            type_path: format!("{alias}::Component"),
            initialization: ComponentInitialization::Configured {
                package: provider_package(&alias).to_owned(),
            },
        }
    }

    fn store_field(&self) -> String {
        format!(
            "    {}: <{} as geam::HostProviderComponent>::Stores,\n",
            self.field, self.type_path,
        )
    }

    fn state_field(&self) -> String {
        format!(
            "    {}: <{} as geam::HostProviderComponent>::RunState,\n",
            self.field, self.type_path,
        )
    }

    fn profile(&self) -> String {
        format!(
            "\nimpl geam::HostComponentProfile<{type_path}> for Profile {{\n    fn component_stores(stores: &Self::ExternalStores) -> &<{type_path} as geam::HostProviderComponent>::Stores {{\n        &stores.{field}\n    }}\n\n    fn component_state(state: &mut Self::RunState) -> &mut <{type_path} as geam::HostProviderComponent>::RunState {{\n        &mut state.{field}\n    }}\n}}\n",
            type_path = self.type_path,
            field = self.field,
        )
    }

    fn registration(&self) -> String {
        format!(
            "    providers.extend(<{} as geam::HostProviderComponentRegistration<Profile>>::providers()?);\n",
            self.type_path,
        )
    }

    fn configuration_selection(&self) -> String {
        match &self.initialization {
            ComponentInitialization::Configured { package } => format!(
                "    let configuration_{field} = configurations.remove(\"{package}\").unwrap_or_else(geam::HostProviderConfiguration::empty);\n",
                field = self.field,
            ),
            ComponentInitialization::Stdlib
            | ComponentInitialization::PackageResources
            | ComponentInitialization::Unit
            | ComponentInitialization::SystemTime => String::new(),
        }
    }

    fn configured_initialization(&self) -> String {
        match &self.initialization {
            ComponentInitialization::Configured { .. } => format!(
                "    let state_{field} = <{type_path} as geam::HostProviderComponentInitialization>::initialize(&configuration_{field})?;\n",
                field = self.field,
                type_path = self.type_path,
            ),
            ComponentInitialization::Stdlib
            | ComponentInitialization::PackageResources
            | ComponentInitialization::Unit
            | ComponentInitialization::SystemTime => String::new(),
        }
    }

    fn capability_initialization(&self) -> String {
        let value = match &self.initialization {
            ComponentInitialization::Stdlib => {
                "geam::gleam_stdlib::GleamStdlibRunState::try_from_entropy_with_io(output.io_sink())?".to_owned()
            }
            ComponentInitialization::Unit => "()".to_owned(),
            ComponentInitialization::SystemTime => {
                "geam::gleam_time::SystemTimeSource".to_owned()
            }
            ComponentInitialization::PackageResources => {
                "resources".to_owned()
            }
            ComponentInitialization::Configured { .. } => return String::new(),
        };
        format!("    let state_{} = {value};\n", self.field)
    }

    fn state_initializer(&self) -> String {
        format!("        {}: state_{},\n", self.field, self.field)
    }
}

fn runner_components(provider_aliases: &[String]) -> Vec<RunnerComponent> {
    let mut provider_aliases = provider_aliases.to_vec();
    provider_aliases.sort();
    provider_aliases.dedup();

    BuiltInProvider::ALL
        .into_iter()
        .map(RunnerComponent::built_in)
        .chain(provider_aliases.into_iter().map(RunnerComponent::external))
        .collect()
}

fn render_host(provider_aliases: &[String]) -> String {
    let components = runner_components(provider_aliases);
    let store_fields = components
        .iter()
        .map(RunnerComponent::store_field)
        .collect::<String>();
    let state_fields = components
        .iter()
        .map(RunnerComponent::state_field)
        .collect::<String>();
    let component_profiles = components
        .iter()
        .map(RunnerComponent::profile)
        .collect::<String>();
    let component_registrations = components
        .iter()
        .map(RunnerComponent::registration)
        .collect::<String>();
    let configuration_selections = components
        .iter()
        .map(RunnerComponent::configuration_selection)
        .collect::<String>();
    let configured_initializations = components
        .iter()
        .map(RunnerComponent::configured_initialization)
        .collect::<String>();
    let capability_initializations = components
        .iter()
        .map(RunnerComponent::capability_initialization)
        .collect::<String>();
    let state_initializers = components
        .iter()
        .map(RunnerComponent::state_initializer)
        .collect::<String>();
    let configuration_mutability = if components.iter().any(|component| {
        matches!(
            component.initialization,
            ComponentInitialization::Configured { .. }
        )
    }) {
        "mut "
    } else {
        ""
    };

    HOST_TEMPLATE
        .replace("__STORE_FIELDS__", &store_fields)
        .replace("__STATE_FIELDS__", &state_fields)
        .replace("__COMPONENT_PROFILES__", &component_profiles)
        .replace("__COMPONENT_REGISTRATIONS__", &component_registrations)
        .replace("__CONFIGURATION_SELECTIONS__", &configuration_selections)
        .replace(
            "__CONFIGURED_INITIALIZATIONS__",
            &configured_initializations,
        )
        .replace(
            "__CAPABILITY_INITIALIZATIONS__",
            &capability_initializations,
        )
        .replace("__STATE_INITIALIZERS__", &state_initializers)
        .replace("__CONFIGURATION_MUTABILITY__", configuration_mutability)
}

fn provider_package(alias: &str) -> &str {
    alias.strip_prefix("geam_provider_").unwrap_or(alias)
}

const HOST_TEMPLATE: &str = r#"// Generated by Geam. Do not edit.

use geam::__standalone_support::{CliIoSink, SharedOutput, run_driver};

#[derive(Default)]
struct Stores {
__STORE_FIELDS__}

struct RunState {
__STATE_FIELDS__}

struct Profile;

impl geam::HostProfile for Profile {
    type RunState = RunState;
    type ExternalStores = Stores;
    type ExecutionState = geam::gleam_erlang::ErlangExecution;
}

impl geam::HostWorkProfile for Profile {
    type Work = geam::FutureComponent;
}

__COMPONENT_PROFILES__

impl geam::gleam_stdlib::GleamStdlibHostProfile for Profile {
    type Io = CliIoSink;
}

impl geam::gleam_time::GleamTimeHostProfile for Profile {
    type Source = geam::gleam_time::SystemTimeSource;
}

impl geam::gleam_erlang::GleamErlangHostProfile for Profile {
    fn erlang_execution(state: &mut Self::ExecutionState) -> &mut geam::gleam_erlang::ErlangExecution {
        state
    }
}

fn host_providers() -> Result<geam::HostProviderSet<Profile>, geam::HostRegistrationError> {
    let mut providers = Vec::new();
__COMPONENT_REGISTRATIONS__    geam::HostProviderSet::from_providers(providers)
}

fn run(
    mut execution: geam::HostedEntry<Profile>,
    __CONFIGURATION_MUTABILITY__configurations: std::collections::BTreeMap<String, geam::HostProviderConfiguration>,
    resources: geam::gleam_erlang::Configuration,
) -> Result<(), Box<dyn std::error::Error>> {
__CONFIGURATION_SELECTIONS__    if let Some(package) = configurations.keys().next() {
        return Err(invalid_data(format!("no selected provider accepts configuration for Gleam package {package}")).into());
    }
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .enable_io()
        .enable_time()
        .build()?;
    let _runtime_context = runtime.enter();
__CONFIGURED_INITIALIZATIONS__    let output = SharedOutput::new();
__CAPABILITY_INITIALIZATIONS__
    let mut state = RunState {
__STATE_INITIALIZERS__    };
    let mut echo = output.echo_sink();
    let host = geam::execution::TokioHost::new(runtime.handle().clone());
    let (execution, state, echo, execution_result) = run_driver(&runtime, async move {
        let result = execution.run(&host, &mut state, &mut echo).await;
        (execution, state, echo, result)
    })?;
    let result = output.finish().map_err(Box::<dyn std::error::Error>::from)
        .and_then(|()| execution_result.map_err(Into::into));
    // State must outlive output handling and release blocking helpers before runtime shutdown.
    drop(echo);
    drop(state);
    drop(output);
    drop(_runtime_context);
    drop(runtime);
    drop(execution);
    result
}

fn invalid_data(reason: impl Into<String>) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, reason.into())
}
"#;

const RUNNER_TEMPLATE: &str = r#"
fn check(project_root: String, module: String) -> Result<(), Box<dyn std::error::Error>> {
    let typed = geam::compile_typed_host_project(project_root, module, host_providers()?)?;
    let plan = geam::plan_host_program(typed)?;
    let _execution = geam::HostedEntry::try_from_module_plan(plan)?;
    Ok(())
}

fn run_project(
    project_root: String,
    module: String,
    configuration_arguments: impl Iterator<Item = std::ffi::OsString>,
) -> Result<(), Box<dyn std::error::Error>> {
    let typed = geam::compile_typed_host_project(project_root, module, host_providers()?)?;
    let configurations = load_configurations(configuration_arguments)?;
    let resources = geam::gleam_erlang::Configuration { resources: typed.package_resources().clone() };
    let execution = geam::HostedEntry::try_from_module_plan(geam::plan_host_program(typed)?)?;
    run(execution, configurations, resources)
}

fn prepare_project(
    project_root: String,
    module: String,
    destination: std::path::PathBuf,
) -> Result<(), Box<dyn std::error::Error>> {
    let typed = geam::compile_typed_host_project(project_root, module, host_providers()?)?
        .map_source_paths(|package, module, _| format!("{package}/src/{module}.gleam").into());
    let packages = typed.package_resources().keys().map(|name| name.to_string()).collect::<Vec<_>>();
    let prepared = geam::PreparedHostedEntry::try_from_module_plan(geam::plan_host_program(typed)?)?;
    let expression = prepared.emit_rust();
    std::fs::write(destination, format!(
        "// Generated by Geam. Do not edit.\n\nuse geam::__prepared_support as data;\n\npub(super) const PACKAGES: &[&str] = &{packages:?};\n\n#[rustfmt::skip]\npub(super) static PROGRAM: data::HostedEntryArtifact = {expression};\n"
    ))?;
    Ok(())
}

fn load_configurations(
    arguments: impl Iterator<Item = std::ffi::OsString>,
) -> Result<std::collections::BTreeMap<String, geam::HostProviderConfiguration>, Box<dyn std::error::Error>> {
    let mut configurations = std::collections::BTreeMap::new();
    for argument in arguments {
        let argument = argument.into_string().map_err(|_| invalid_arguments())?;
        let Some((package, path)) = argument.split_once('=') else {
            return Err(invalid_data("expected provider configuration argument PACKAGE=PATH").into());
        };
        let configuration = geam::__standalone_support::read_provider_configuration(std::path::Path::new(path))?;
        if configurations.insert(package.to_owned(), configuration).is_some() {
            return Err(invalid_data(format!("provider configuration for {package} was supplied more than once")).into());
        }
    }
    Ok(configurations)
}

fn entry() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args_os().skip(1);
    let mode = arguments.next().ok_or_else(invalid_arguments)?;
    let project_root = arguments.next().ok_or_else(invalid_arguments)?
        .into_string().map_err(|_| invalid_arguments())?;
    let module = arguments.next().ok_or_else(invalid_arguments)?
        .into_string().map_err(|_| invalid_arguments())?;
    if mode == "check" && arguments.next().is_none() {
        check(project_root, module)
    } else if mode == "run" {
        run_project(project_root, module, arguments)
    } else if mode == "prepare" {
        let destination = arguments.next().ok_or_else(invalid_arguments)?.into();
        if arguments.next().is_some() {
            return Err(invalid_arguments().into());
        }
        prepare_project(project_root, module, destination)
    } else {
        Err(invalid_arguments().into())
    }
}

fn main() -> std::process::ExitCode {
    match entry() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("geam runner: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}

fn invalid_arguments() -> std::io::Error {
    invalid_data("expected internal runner arguments: check|run PROJECT_ROOT MODULE [PACKAGE=PATH ...], or prepare PROJECT_ROOT MODULE OUTPUT")
}
"#;

const APPLICATION_TEMPLATE: &str = r#"
mod program;

fn entry() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = geam::__standalone_support::RuntimeInputs::read(
        &std::env::current_exe()?,
        &std::env::current_dir()?,
        std::env::var_os("GEAM_CONFIG").as_deref(),
        program::PACKAGES,
        &__PROVIDER_PACKAGES__,
    )?;
    let execution = program::PROGRAM.load(host_providers()?)?;
    run(execution, inputs.configurations, inputs.resources)
}

fn main() -> std::process::ExitCode {
    match entry() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("geam application: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}
"#;

#[cfg(test)]
mod tests {
    use super::{
        ComponentInitialization, RunnerComponent, render_application, render_host, render_source,
        runner_components,
    };

    #[test]
    fn deployed_entry_uses_prepared_data_and_runtime_inputs_without_parsing_arguments() {
        let aliases = [
            "geam_provider_zeta".into(),
            "geam_provider_alpha".into(),
            "geam_provider_zeta".into(),
        ];
        let application = render_application(&aliases);
        let host = render_host(&aliases);
        assert_eq!(
            application.strip_prefix(&host).unwrap(),
            r#"
mod program;

fn entry() -> Result<(), Box<dyn std::error::Error>> {
    let inputs = geam::__standalone_support::RuntimeInputs::read(
        &std::env::current_exe()?,
        &std::env::current_dir()?,
        std::env::var_os("GEAM_CONFIG").as_deref(),
        program::PACKAGES,
        &["alpha", "zeta"],
    )?;
    let execution = program::PROGRAM.load(host_providers()?)?;
    run(execution, inputs.configurations, inputs.resources)
}

fn main() -> std::process::ExitCode {
    match entry() {
        Ok(()) => std::process::ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("geam application: {error}");
            std::process::ExitCode::FAILURE
        }
    }
}
"#
        );
        assert!(!application.contains("compile_typed"));
        assert!(!application.contains("std::env::args"));
        assert_eq!(render_application(&[]).matches("&[],").count(), 1);
    }

    #[test]
    fn renders_static_profiles_and_initialization_in_sorted_component_order() {
        let aliases = [
            "geam_provider_zeta".to_owned(),
            "geam_provider_alpha".to_owned(),
            "geam_provider_alpha".to_owned(),
        ];
        assert_eq!(
            runner_components(&aliases),
            [
                RunnerComponent {
                    field: "stdlib".to_owned(),
                    type_path: "geam::gleam_stdlib::Component<CliIoSink>".to_owned(),
                    initialization: ComponentInitialization::Stdlib,
                },
                RunnerComponent {
                    field: "json".to_owned(),
                    type_path: "geam::gleam_json::Component".to_owned(),
                    initialization: ComponentInitialization::Unit,
                },
                RunnerComponent {
                    field: "time".to_owned(),
                    type_path: "geam::gleam_time::Component".to_owned(),
                    initialization: ComponentInitialization::SystemTime,
                },
                RunnerComponent {
                    field: "erlang".to_owned(),
                    type_path: "geam::gleam_erlang::Component<Profile>".to_owned(),
                    initialization: ComponentInitialization::PackageResources,
                },
                RunnerComponent {
                    field: "future".to_owned(),
                    type_path: "geam::FutureComponent".to_owned(),
                    initialization: ComponentInitialization::Unit,
                },
                RunnerComponent {
                    field: "geam_provider_alpha".to_owned(),
                    type_path: "geam_provider_alpha::Component".to_owned(),
                    initialization: ComponentInitialization::Configured {
                        package: "alpha".to_owned(),
                    },
                },
                RunnerComponent {
                    field: "geam_provider_zeta".to_owned(),
                    type_path: "geam_provider_zeta::Component".to_owned(),
                    initialization: ComponentInitialization::Configured {
                        package: "zeta".to_owned(),
                    },
                },
            ],
        );

        let source = render_source(&aliases);

        assert!(source.starts_with("// Generated by Geam. Do not edit.\n"));
        for field in [
            "stdlib",
            "json",
            "time",
            "erlang",
            "future",
            "geam_provider_alpha",
            "geam_provider_zeta",
        ] {
            assert_eq!(source.matches(&format!("    {field}: <")).count(), 2);
        }
        assert!(source.contains("impl geam::gleam_stdlib::GleamStdlibHostProfile for Profile"));
        assert!(source.contains("impl geam::gleam_time::GleamTimeHostProfile for Profile"));
        assert!(source.contains("impl geam::gleam_erlang::GleamErlangHostProfile for Profile"));
        assert!(source.contains("impl geam::HostWorkProfile for Profile"));

        let type_paths = [
            "geam::gleam_stdlib::Component<CliIoSink>",
            "geam::gleam_json::Component",
            "geam::gleam_time::Component",
            "geam::gleam_erlang::Component<Profile>",
            "geam::FutureComponent",
            "geam_provider_alpha::Component",
            "geam_provider_zeta::Component",
        ];
        let mut previous_profile = 0;
        let mut previous_registration = 0;
        for type_path in type_paths {
            let profile = source
                .find(&format!(
                    "impl geam::HostComponentProfile<{type_path}> for Profile"
                ))
                .expect("component profile should render");
            let registration = source
                .find(&format!(
                    "<{type_path} as geam::HostProviderComponentRegistration<Profile>>::providers()?"
                ))
                .expect("component registration should render");
            assert!(profile > previous_profile);
            assert!(registration > previous_registration);
            previous_profile = profile;
            previous_registration = registration;
        }
        assert!(!source.contains("geam::gleam_stdlib::host_providers::<Profile>()"));
        assert!(!source.contains("geam::gleam_json::host_providers::<Profile>()"));
        assert!(!source.contains("geam::gleam_time::host_providers::<Profile>()"));

        let alpha_initialization = source
            .find("let state_geam_provider_alpha")
            .expect("alpha state should initialize");
        let zeta_initialization = source
            .find("let state_geam_provider_zeta")
            .expect("zeta state should initialize");
        let output_initialization = source
            .find("let output = SharedOutput::new();")
            .expect("shared output should initialize");
        assert!(alpha_initialization < zeta_initialization);
        assert!(zeta_initialization < output_initialization);
        let runtime_context = source
            .find("let _runtime_context = runtime.enter();")
            .expect("entered runtime");
        assert!(runtime_context < alpha_initialization);

        let mut previous_initialization = output_initialization;
        for field in ["stdlib", "json", "time", "erlang", "future"] {
            let initialization = source
                .find(&format!("let state_{field}"))
                .expect("runner capability should initialize");
            assert!(initialization > previous_initialization);
            previous_initialization = initialization;
        }
        assert!(source.contains(
            "let state_stdlib = geam::gleam_stdlib::GleamStdlibRunState::try_from_entropy_with_io(output.io_sink())?;"
        ));
        assert!(source.contains("let state_json = ();"));
        assert!(source.contains("let state_time = geam::gleam_time::SystemTimeSource;"));
        assert!(source.contains("let state_erlang = resources;"));
        assert!(source.contains("let result = execution.run(&host, &mut state, &mut echo).await;"));
        let mut previous = previous_initialization;
        for step in [
            "run_driver(&runtime, async move",
            "(execution, state, echo, result)",
            "output.finish()",
            "execution_result.map_err",
            "drop(echo)",
            "drop(state)",
            "drop(output)",
            "drop(_runtime_context)",
            "drop(runtime)",
            "drop(execution)",
        ] {
            let position = source.find(step).expect("driver lifecycle step");
            assert!(position > previous, "{step}");
            previous = position;
        }
        assert_eq!(
            source,
            render_source(&[
                "geam_provider_alpha".to_owned(),
                "geam_provider_zeta".to_owned(),
            ])
        );
    }
}
