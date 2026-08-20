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
    Configured { package: String },
}

impl RunnerComponent {
    fn stdlib() -> Self {
        Self {
            field: "stdlib".to_owned(),
            type_path: "geam::gleam_stdlib::Component<CliIoSink>".to_owned(),
            initialization: ComponentInitialization::Stdlib,
        }
    }

    fn json() -> Self {
        Self {
            field: "json".to_owned(),
            type_path: "geam::gleam_json::Component".to_owned(),
            initialization: ComponentInitialization::Unit,
        }
    }

    fn time() -> Self {
        Self {
            field: "time".to_owned(),
            type_path: "geam::gleam_time::Component".to_owned(),
            initialization: ComponentInitialization::SystemTime,
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

    [
        RunnerComponent::stdlib(),
        RunnerComponent::json(),
        RunnerComponent::time(),
    ]
    .into_iter()
    .chain(provider_aliases.into_iter().map(RunnerComponent::external))
    .collect()
}

pub(super) fn render_source(provider_aliases: &[String]) -> String {
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

    RUNNER_TEMPLATE
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

const RUNNER_TEMPLATE: &str = r#"// Generated by Geam. Do not edit.

#[derive(Default)]
struct Stores {
__STORE_FIELDS__}

struct RunState {
__STATE_FIELDS__}

struct Profile;

impl geam::HostProfile for Profile {
    type RunState = RunState;
    type ExternalStores = Stores;
}

__COMPONENT_PROFILES__

impl geam::gleam_stdlib::GleamStdlibHostProfile for Profile {
    type Io = CliIoSink;
}

impl geam::gleam_time::GleamTimeHostProfile for Profile {
    type Source = geam::gleam_time::SystemTimeSource;
}

fn host_providers() -> Result<geam::HostProviderSet<Profile>, geam::HostRegistrationError> {
    let mut providers = Vec::new();
__COMPONENT_REGISTRATIONS__    geam::HostProviderSet::with_providers(Vec::<geam::HostModule<Profile>>::new(), providers)
}

fn check(project_root: String, module: String) -> Result<(), Box<dyn std::error::Error>> {
    let typed = geam::compile_typed_host_project(project_root, module, host_providers()?)?;
    let plan = geam::plan_host_program(typed)?;
    let _execution = geam::HostedExecution::try_from_module_plan(plan)?;
    Ok(())
}

fn run_project(
    project_root: String,
    module: String,
    configuration_arguments: impl Iterator<Item = String>,
) -> Result<(), Box<dyn std::error::Error>> {
    let typed = geam::compile_typed_host_project(project_root, module, host_providers()?)?;
    let __CONFIGURATION_MUTABILITY__configurations = load_configurations(configuration_arguments)?;
__CONFIGURATION_SELECTIONS__    if let Some(package) = configurations.keys().next() {
        return Err(invalid_data(format!("no selected provider accepts configuration for Gleam package {package}")).into());
    }
__CONFIGURED_INITIALIZATIONS__    let output = SharedOutput::new();
__CAPABILITY_INITIALIZATIONS__
    let mut state = RunState {
__STATE_INITIALIZERS__    };
    let plan = geam::plan_host_program(typed)?;
    let execution = geam::HostedExecution::try_from_module_plan(plan)?;
    let mut echo = output.echo_sink();
    let execution_result = execution.run_main(&mut state, &mut echo);
    output.finish()?;
    execution_result?;
    Ok(())
}

fn load_configurations(
    arguments: impl Iterator<Item = String>,
) -> Result<std::collections::BTreeMap<String, geam::HostProviderConfiguration>, Box<dyn std::error::Error>> {
    let mut configurations = std::collections::BTreeMap::new();
    for argument in arguments {
        let Some((package, path)) = argument.split_once('=') else {
            return Err(invalid_data("expected provider configuration argument PACKAGE=PATH").into());
        };
        let configuration = read_configuration(path)?;
        if configurations.insert(package.to_owned(), configuration).is_some() {
            return Err(invalid_data(format!("provider configuration for {package} was supplied more than once")).into());
        }
    }
    Ok(configurations)
}

fn read_configuration(path: &str) -> Result<geam::HostProviderConfiguration, Box<dyn std::error::Error>> {
    let source = std::fs::read_to_string(path).map_err(|error| {
        std::io::Error::new(error.kind(), format!("failed to read provider configuration {path}: {error}"))
    })?;
    let table = toml::from_str::<toml::Table>(&source).map_err(|error| {
        invalid_data(format!("invalid provider configuration {path}: {error}"))
    })?;
    configuration_from_table(table)
}

fn configuration_from_table(
    table: toml::Table,
) -> Result<geam::HostProviderConfiguration, Box<dyn std::error::Error>> {
    let values = table
        .into_iter()
        .map(|(key, value)| configuration_value(value).map(|value| (key.into(), value)))
        .collect::<Result<_, _>>()?;
    Ok(geam::HostProviderConfiguration::new(values))
}

fn configuration_value(
    value: toml::Value,
) -> Result<geam::HostProviderConfigurationValue, Box<dyn std::error::Error>> {
    Ok(match value {
        toml::Value::String(value) => geam::HostProviderConfigurationValue::String(value.into()),
        toml::Value::Integer(value) => geam::HostProviderConfigurationValue::Integer(value),
        toml::Value::Float(value) => geam::HostProviderConfigurationValue::Float(value),
        toml::Value::Boolean(value) => geam::HostProviderConfigurationValue::Bool(value),
        toml::Value::Array(values) => geam::HostProviderConfigurationValue::Array(
            values
                .into_iter()
                .map(configuration_value)
                .collect::<Result<_, _>>()?,
        ),
        toml::Value::Table(value) => {
            geam::HostProviderConfigurationValue::Table(configuration_from_table(value)?)
        }
        toml::Value::Datetime(value) => {
            return Err(invalid_data(format!("TOML datetime configuration values are unsupported: {value}")).into());
        }
    })
}

#[derive(Clone)]
struct SharedOutput {
    failure: std::rc::Rc<std::cell::RefCell<Option<std::io::Error>>>,
}

impl SharedOutput {
    fn new() -> Self {
        Self {
            failure: std::rc::Rc::new(std::cell::RefCell::new(None)),
        }
    }

    fn io_sink(&self) -> CliIoSink {
        CliIoSink {
            output: self.clone(),
        }
    }

    fn echo_sink(&self) -> CliEchoSink {
        CliEchoSink {
            output: self.clone(),
        }
    }

    fn write(&self, stream: OutputStream, text: &str) {
        if self.failure.borrow().is_some() {
            return;
        }
        let result = match stream {
            OutputStream::Stdout => write_stdout(text),
            OutputStream::Stderr => write_stderr(text),
        };
        if let Err(error) = result {
            *self.failure.borrow_mut() = Some(error);
        }
    }

    fn finish(&self) -> Result<(), std::io::Error> {
        match self.failure.borrow_mut().take() {
            Some(error) => Err(error),
            None => Ok(()),
        }
    }
}

enum OutputStream {
    Stdout,
    Stderr,
}

struct CliIoSink {
    output: SharedOutput,
}

impl geam::gleam_stdlib::IoSink for CliIoSink {
    fn emit(&mut self, output: geam::gleam_stdlib::IoOutput) {
        let stream = match output.stream() {
            geam::gleam_stdlib::IoStream::Stdout => OutputStream::Stdout,
            geam::gleam_stdlib::IoStream::Stderr => OutputStream::Stderr,
        };
        self.output.write(stream, output.text().as_str());
    }
}

struct CliEchoSink {
    output: SharedOutput,
}

impl geam::EchoSink for CliEchoSink {
    fn emit(&mut self, output: geam::EchoOutput) {
        let mut text = output.to_string();
        text.push('\n');
        self.output.write(OutputStream::Stderr, &text);
    }
}

fn write_stdout(text: &str) -> Result<(), std::io::Error> {
    let stdout = std::io::stdout();
    let mut stdout = stdout.lock();
    std::io::Write::write_all(&mut stdout, text.as_bytes())?;
    std::io::Write::flush(&mut stdout)
}

fn write_stderr(text: &str) -> Result<(), std::io::Error> {
    let stderr = std::io::stderr();
    let mut stderr = stderr.lock();
    std::io::Write::write_all(&mut stderr, text.as_bytes())?;
    std::io::Write::flush(&mut stderr)
}

fn entry() -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args().skip(1);
    let mode = arguments.next().ok_or_else(invalid_arguments)?;
    let project_root = arguments.next().ok_or_else(invalid_arguments)?;
    let module = arguments.next().ok_or_else(invalid_arguments)?;
    match mode.as_str() {
        "check" if arguments.next().is_none() => check(project_root, module),
        "run" => run_project(project_root, module, arguments),
        _ => Err(invalid_arguments().into()),
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
    invalid_data("expected internal runner arguments: check|run PROJECT_ROOT MODULE [PACKAGE=PATH ...]")
}

fn invalid_data(reason: impl Into<String>) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidInput, reason.into())
}
"#;

#[cfg(test)]
mod tests {
    use super::{ComponentInitialization, RunnerComponent, render_source, runner_components};

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
            "geam_provider_alpha",
            "geam_provider_zeta",
        ] {
            assert_eq!(source.matches(&format!("    {field}: <")).count(), 2);
        }
        assert!(source.contains("impl geam::gleam_stdlib::GleamStdlibHostProfile for Profile"));
        assert!(source.contains("impl geam::gleam_time::GleamTimeHostProfile for Profile"));

        let type_paths = [
            "geam::gleam_stdlib::Component<CliIoSink>",
            "geam::gleam_json::Component",
            "geam::gleam_time::Component",
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

        let mut previous_initialization = output_initialization;
        for field in ["stdlib", "json", "time"] {
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
        assert!(
            source.contains("let execution_result = execution.run_main(&mut state, &mut echo);")
        );
        assert_eq!(
            source,
            render_source(&[
                "geam_provider_alpha".to_owned(),
                "geam_provider_zeta".to_owned(),
            ])
        );
    }
}
