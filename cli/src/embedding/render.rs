use super::boundary::{DataType, FunctionBinding, PlainBindings};
use super::profile::{ComponentBinding, HostedBindings, HostedCapabilities, HostedComponents};
use camino::Utf8Path;
use std::collections::BTreeSet;

mod named;
mod value;
use value::{push_function_field, push_input_shapes};

pub(super) fn plain(bindings: &PlainBindings, project_path: &Utf8Path) -> String {
    let mut output = format!("{}\n", super::GENERATED_HEADER);
    let alias = bindings.geam_alias.as_str();
    let mut imports = BTreeSet::from([
        "BindingError",
        "Function",
        "FunctionDeclaration",
        "InputShape",
        "ModuleBindings",
        "ModuleBuilder",
        "Project",
    ]);
    for function in bindings.functions() {
        for type_ in function
            .arguments
            .iter()
            .chain(std::iter::once(&function.return_type))
        {
            type_.collect_imports(&mut imports);
        }
    }
    for import in imports {
        output.push_str(&format!("use {alias}::embedding::{import};\n"));
    }
    output.push_str(&format!(
        "\npub const ROOT_MODULE: &str = {:?};\n\n",
        bindings.root_module
    ));
    push_plain_project(&mut output, project_path);
    output.push_str("#[allow(dead_code, clippy::type_complexity)]\npub struct Functions {\n");
    for (index, function) in bindings.functions().enumerate() {
        push_function_field(&mut output, index, function);
    }
    output.push_str("}\n\n");
    push_input_shapes(&mut output, bindings);
    output.push_str(
        "#[allow(dead_code)]\npub fn bind(builder: ModuleBuilder) -> Result<(ModuleBindings, Functions), BindingError> {\n",
    );
    push_bind_body(&mut output, bindings, "Functions", "function");
    output.push_str("}\n");
    output
}

pub(super) fn hosted(bindings: &HostedBindings, project_path: &Utf8Path) -> String {
    let mut output = format!("{}\n", super::GENERATED_HEADER);
    let boundary = &bindings.boundary;
    let alias = boundary.geam_alias.as_str();
    let components = &bindings.components;
    let mut host_imports =
        BTreeSet::from(["HostProfile", "HostProviderSet", "HostRegistrationError"]);
    if !components.is_empty() {
        host_imports.extend([
            "HostComponentProfile",
            "HostProviderComponent",
            "HostProviderComponentRegistration",
        ]);
    }
    if components.has_work() {
        host_imports.insert("HostWorkProfile");
    }
    if components.has_external() {
        host_imports.extend([
            "HostProviderComponentInitialization",
            "HostProviderConfiguration",
            "HostProviderInitializationError",
        ]);
    }
    for import in host_imports {
        let import = if import == "HostProviderComponentRegistration" {
            "HostProviderComponentRegistration as ComponentRegistration"
        } else {
            import
        };
        output.push_str(&format!("use {alias}::{import};\n"));
    }
    output.push('\n');
    let mut embedding_imports = BTreeSet::from([
        "BindingError",
        "Function",
        "FunctionDeclaration",
        "HostedModuleBindings",
        "HostedModuleBuilder",
        "HostedProject",
        "InputShape",
    ]);
    for function in boundary.functions() {
        for type_ in function
            .arguments
            .iter()
            .chain(std::iter::once(&function.return_type))
        {
            type_.collect_imports(&mut embedding_imports);
        }
    }
    for import in embedding_imports {
        output.push_str(&format!("use {alias}::embedding::{import};\n"));
    }
    if components.has_stdlib() || components.has_time() {
        output.push_str("use std::marker::PhantomData;\n");
    }

    output.push_str(&format!(
        "\npub const ROOT_MODULE: &str = {:?};\n\n",
        boundary.root_module
    ));
    named::push_types(&mut output, alias, &boundary.named_types);
    push_profile_declaration(&mut output, components);
    push_provider_set_alias(&mut output, components);
    push_stores(&mut output, alias, components);
    push_run_state_inputs(&mut output, alias, components);
    push_run_state(&mut output, alias, components);
    push_host_profile(&mut output, alias, components);
    for component in components.iter() {
        push_component_profile(&mut output, alias, components, component);
    }
    if components.has_stdlib() {
        push_stdlib_profile(&mut output, alias, components);
    }
    if components.has_time() {
        push_time_profile(&mut output, alias, components);
    }
    push_host_providers(&mut output, alias, components);
    push_hosted_project(&mut output, alias, components, project_path);

    output.push_str("#[allow(clippy::type_complexity)]\npub struct Functions {\n");
    for (index, function) in boundary.functions().enumerate() {
        push_function_field(&mut output, index, function);
    }
    output.push_str("}\n\n");
    push_input_shapes(&mut output, boundary);
    push_hosted_bind(&mut output, alias, components, boundary);
    output
}

fn push_plain_project(output: &mut String, project_path: &Utf8Path) {
    let root = project_root(project_path);
    let construction = format!("    Project::new({root}, ROOT_MODULE)\n");
    output.push_str("pub fn project() -> Project {\n");
    if construction.trim_end().len() <= 100 {
        output.push_str(&construction);
    } else {
        output.push_str("    Project::new(\n");
        push_project_root_argument(output, project_path, "        ");
        output.push_str("        ROOT_MODULE,\n    )\n");
    }
    output.push_str("}\n\n");
}

fn push_hosted_project(
    output: &mut String,
    alias: &str,
    components: &HostedComponents,
    project_path: &Utf8Path,
) {
    let profile = profile_type(components);
    output.push_str(&format!(
        "pub fn project{}() -> {}<{profile}>",
        generics(components),
        "HostedProject",
    ));
    push_bounds_open(output, alias, components);
    output.push_str(&format!("    {}::new(\n", "HostedProject"));
    push_project_root_argument(output, project_path, "        ");
    output.push_str("        ROOT_MODULE,\n");
    let registration = match generics(components) {
        "" => "host_providers".to_owned(),
        generics => format!("host_providers::{generics}"),
    };
    output.push_str(&format!("        {registration},\n    )\n}}\n\n"));
}

fn project_root(project_path: &Utf8Path) -> String {
    let suffix = format!("/{project_path}");
    format!("concat!(env!(\"CARGO_MANIFEST_DIR\"), {suffix:?})")
}

fn push_project_root_argument(output: &mut String, project_path: &Utf8Path, indent: &str) {
    let root = project_root(project_path);
    if indent.len() + root.len() < 100 {
        output.push_str(&format!("{indent}{root},\n"));
        return;
    }
    let suffix = format!("/{project_path}");
    output.push_str(&format!(
        "{indent}concat!(\n{indent}    env!(\"CARGO_MANIFEST_DIR\"),\n{indent}    {suffix:?}\n{indent}),\n"
    ));
}

fn push_provider_set_alias(output: &mut String, components: &HostedComponents) {
    output.push_str(&format!(
        "pub type ProviderSet{} = {}<{}>;\n\n",
        generics(components),
        "HostProviderSet",
        profile_type(components),
    ));
}

fn push_profile_declaration(output: &mut String, components: &HostedComponents) {
    match components.capabilities() {
        HostedCapabilities::None => output.push_str("pub struct Profile;\n\n"),
        HostedCapabilities::Io => {
            output.push_str("pub struct Profile<Io>(PhantomData<fn() -> Io>);\n\n")
        }
        HostedCapabilities::IoAndTime => output
            .push_str("pub struct Profile<Io, Source>(PhantomData<fn() -> (Io, Source)>);\n\n"),
    }
}

fn push_stores(output: &mut String, alias: &str, components: &HostedComponents) {
    if components.is_empty() {
        output.push_str("#[derive(Default)]\npub struct Stores {}\n\n");
        return;
    }
    let derives_default = components.capabilities() == HostedCapabilities::None;
    if derives_default {
        output.push_str("#[derive(Default)]\n");
    }
    output.push_str(&format!("pub struct Stores{}", generics(components)));
    push_bounds_open(output, alias, components);
    for component in components.iter() {
        push_component_field(output, alias, component, "HostProviderComponent", "Stores");
    }
    output.push_str("}\n\n");

    if derives_default {
        return;
    }
    output.push_str(&format!(
        "impl{} Default for Stores{}",
        generics(components),
        generics(components),
    ));
    push_bounds_open(output, alias, components);
    output.push_str("    fn default() -> Self {\n        Self {\n");
    for component in components.iter() {
        let value = if component == &ComponentBinding::Time {
            "()"
        } else {
            "Default::default()"
        };
        output.push_str(&format!(
            "            {}: {value},\n",
            component_field(component),
        ));
    }
    output.push_str("        }\n    }\n}\n\n");
}

fn push_run_state_inputs(output: &mut String, alias: &str, components: &HostedComponents) {
    if components.is_empty() {
        output.push_str("pub struct RunStateInputs {}\n\nimpl RunStateInputs {\n    pub fn initialize(self) -> RunState {\n        RunState {}\n    }\n}\n\n");
        return;
    }
    if components.only_work() {
        output.push_str(
            "pub struct RunStateInputs {}\n\nimpl RunStateInputs {\n    pub fn initialize(self) -> RunState {\n        RunState { future: () }\n    }\n}\n\n",
        );
        return;
    }
    output.push_str(&format!(
        "pub struct RunStateInputs{}",
        generics(components),
    ));
    push_bounds_open(output, alias, components);
    for component in components.iter() {
        match component {
            ComponentBinding::Future => {}
            ComponentBinding::Stdlib => output.push_str(&format!(
                "    pub stdlib: {alias}::gleam_stdlib::GleamStdlibRunState<Io>,\n"
            )),
            ComponentBinding::Json => {}
            ComponentBinding::Time => output.push_str("    pub time: Source,\n"),
            ComponentBinding::External(component) => output.push_str(&format!(
                "    pub {}: HostProviderConfiguration,\n",
                component.input_field.as_str(),
            )),
        }
    }
    output.push_str("}\n\n");

    output.push_str(&format!(
        "impl{} RunStateInputs{}",
        generics(components),
        generics(components),
    ));
    push_bounds_open(output, alias, components);
    output.push_str("    pub fn initialize(self)");
    let return_type = format!("RunState{}", generics(components));
    if components.has_external() {
        output.push_str(&format!(
            " -> Result<{return_type}, HostProviderInitializationError> {{\n        Ok(RunState {{\n"
        ));
    } else {
        output.push_str(&format!(" -> {return_type} {{\n        RunState {{\n"));
    }
    for component in components.iter() {
        match component {
            ComponentBinding::Future => output.push_str("            future: (),\n"),
            ComponentBinding::Stdlib => output.push_str("            stdlib: self.stdlib,\n"),
            ComponentBinding::Json => output.push_str("            json: (),\n"),
            ComponentBinding::Time => output.push_str("            time: self.time,\n"),
            ComponentBinding::External(component) => {
                push_external_initialization(output, alias, component)
            }
        }
    }
    if components.has_external() {
        output.push_str("        })\n    }\n");
    } else {
        output.push_str("        }\n    }\n");
    }
    output.push_str("}\n\n");
}

fn push_run_state(output: &mut String, alias: &str, components: &HostedComponents) {
    if components.is_empty() {
        output.push_str("pub struct RunState {}\n\n");
        return;
    }
    output.push_str(&format!("pub struct RunState{}", generics(components)));
    push_bounds_open(output, alias, components);
    for component in components.iter() {
        push_component_field(
            output,
            alias,
            component,
            "HostProviderComponent",
            "RunState",
        );
    }
    output.push_str("}\n\n");

    if components.has_stdlib() {
        output.push_str(&format!(
            "impl{} RunState{}",
            generics(components),
            generics(components),
        ));
        push_bounds_open(output, alias, components);
        output.push_str(&format!(
            "    pub fn stdlib(&self) -> &{alias}::gleam_stdlib::GleamStdlibRunState<Io> {{\n        &self.stdlib\n    }}\n\n    pub fn stdlib_mut(&mut self) -> &mut {alias}::gleam_stdlib::GleamStdlibRunState<Io> {{\n        &mut self.stdlib\n    }}\n"
        ));
        output.push_str("}\n\n");
    }
}

fn push_host_profile(output: &mut String, alias: &str, components: &HostedComponents) {
    let profile = profile_type(components);
    output.push_str(&format!(
        "impl{} HostProfile for {profile}",
        generics(components),
    ));
    push_bounds_open(output, alias, components);
    output.push_str(&format!(
        "    type ExternalStores = Stores{};\n    type RunState = RunState{};\n}}\n\n",
        generics(components),
        generics(components),
    ));
    if !components.has_work() {
        return;
    }
    output.push_str(&format!(
        "impl{} HostWorkProfile for {profile}",
        generics(components)
    ));
    push_bounds_open(output, alias, components);
    output.push_str(&format!(
        "    type Work = {alias}::FutureComponent;\n}}\n\n"
    ));
}

fn push_component_profile(
    output: &mut String,
    alias: &str,
    components: &HostedComponents,
    component: &ComponentBinding,
) {
    let component_type = component_type(alias, component);
    let field = component_field(component);
    let implementation = format!(
        "impl{} {}<{component_type}> for {}",
        generics(components),
        "HostComponentProfile",
        profile_type(components),
    );
    let needs_brace = components.capabilities() == HostedCapabilities::None;
    let inline = implementation.len() + if needs_brace { 2 } else { 0 } <= 100;
    if inline {
        output.push_str(&implementation);
    } else if format!(
        "impl{} HostComponentProfile<{component_type}>",
        generics(components),
    )
    .len()
        <= 100
    {
        output.push_str(&format!(
            "impl{} {}<{component_type}>\n    for {}",
            generics(components),
            "HostComponentProfile",
            profile_type(components),
        ));
    } else {
        output.push_str(&format!(
            "impl{}\n    HostComponentProfile<\n        {component_type},\n    > for {}",
            generics(components),
            profile_type(components),
        ));
    }
    if !inline && needs_brace {
        output.push_str("\n{\n");
    } else {
        push_bounds_open(output, alias, components);
    }
    output.push_str(&format!(
        "    fn {}(\n        stores: &Self::ExternalStores,\n",
        "component_stores"
    ));
    push_method_open(
        output,
        &format!(
            "    ) -> &<{component_type} as {}>::{}",
            "HostProviderComponent", "Stores"
        ),
    );
    output.push_str(&format!("        &stores.{field}\n    }}\n\n"));
    output.push_str("    fn component_state(\n        state: &mut Self::RunState,\n");
    push_method_open(
        output,
        &format!("    ) -> &mut <{component_type} as HostProviderComponent>::RunState"),
    );
    output.push_str(&format!("        &mut state.{field}\n    }}\n}}\n\n"));
}

fn push_method_open(output: &mut String, signature: &str) {
    output.push_str(signature);
    // Rustfmt reserves a method indent when budgeting a multiline return type.
    if 4 + signature.len() + 2 <= 100 {
        output.push_str(" {\n");
    } else {
        output.push_str("\n    {\n");
    }
}

fn push_stdlib_profile(output: &mut String, alias: &str, components: &HostedComponents) {
    output.push_str(&format!(
        "impl{} {alias}::gleam_stdlib::GleamStdlibHostProfile for {}",
        generics(components),
        profile_type(components),
    ));
    push_bounds_open(output, alias, components);
    output.push_str("    type Io = Io;\n}\n\n");
}

fn push_time_profile(output: &mut String, alias: &str, components: &HostedComponents) {
    output.push_str(&format!(
        "impl{} {alias}::gleam_time::GleamTimeHostProfile for {}",
        generics(components),
        profile_type(components),
    ));
    push_bounds_open(output, alias, components);
    output.push_str("    type Source = Source;\n}\n\n");
}

fn push_host_providers(output: &mut String, alias: &str, components: &HostedComponents) {
    let profile = profile_type(components);
    output.push_str(&format!(
        "pub fn host_providers{}() -> Result<ProviderSet{}, HostRegistrationError>",
        generics(components),
        generics(components),
    ));
    push_bounds_open(output, alias, components);
    let registered = components
        .iter()
        .filter(|component| component != &&ComponentBinding::Future || components.future_source)
        .collect::<Vec<_>>();
    if registered.is_empty() {
        output.push_str("    HostProviderSet::from_providers([])\n}\n\n");
        return;
    }
    for (index, component) in registered.iter().enumerate() {
        let component = component_type(alias, component);
        let declaration = if index == 0 {
            if registered.len() == 1 {
                "let providers"
            } else {
                "let mut providers"
            }
        } else {
            "let additional_providers"
        };
        let registration =
            format!("<{component} as ComponentRegistration<{profile}>>::providers()?;");
        let statement = format!("    {declaration} = {registration}");
        if statement.len() <= 100 {
            output.push_str(&format!("{statement}\n"));
        } else if 8 + registration.len() <= 100 {
            output.push_str(&format!("    {declaration} =\n        {registration}\n"));
        } else if format!("    {declaration} = <{component} as ComponentRegistration<").len() < 100
        {
            output.push_str(&format!(
                "    {declaration} = <{component} as ComponentRegistration<\n        {profile},\n    >>::providers()?;\n"
            ));
        } else if 8 + registration.len() - 3 <= 100 {
            output.push_str(&format!(
                "    {declaration} =\n        <{component} as ComponentRegistration<{profile}>>::providers(\n        )?;\n"
            ));
        } else {
            output.push_str(&format!(
                "    {declaration} =\n        <{component} as ComponentRegistration<\n            {profile},\n        >>::providers()?;\n",
            ));
        }
        if index != 0 {
            output.push_str("    providers.extend(additional_providers);\n");
        }
    }
    output.push_str("    HostProviderSet::from_providers(providers)\n}\n\n");
}

fn push_hosted_bind(
    output: &mut String,
    alias: &str,
    components: &HostedComponents,
    boundary: &PlainBindings,
) {
    let profile = profile_type(components);
    output.push_str(&format!(
        "pub fn bind{}(\n    builder: {}<{profile}>,\n) -> Result<({}<{profile}>, Functions), BindingError>",
        generics(components),
        "HostedModuleBuilder",
        "HostedModuleBindings",
    ));
    push_bounds_open(output, alias, components);
    push_bind_body(output, boundary, "Functions", "function");
    output.push_str("}\n");
}

fn push_bind_body(output: &mut String, bindings: &PlainBindings, functions: &str, method: &str) {
    let mutability = if bindings.remaining.is_empty() {
        ""
    } else {
        "mut "
    };
    push_binding(
        output,
        &format!("({mutability}bindings, function_0)"),
        "builder",
        &bindings.first,
        method,
    );
    for (index, function) in bindings.remaining.iter().enumerate() {
        push_binding(
            output,
            &format!("function_{}", index + 1),
            "bindings",
            function,
            method,
        );
    }
    push_binding_result(output, bindings, functions);
}

fn push_binding_result(output: &mut String, bindings: &PlainBindings, functions: &str) {
    output.push_str(&format!(
        "    Ok((\n        bindings,\n        {functions} {{\n"
    ));
    for (index, function) in bindings.functions().enumerate() {
        let name = function.rust_name.as_str();
        let value = format!("function_{index}.with_input_shape()");
        if 12 + name.len() + 2 + value.len() < 100 {
            output.push_str(&format!("            {name}: {value},\n"));
        } else {
            output.push_str(&format!("            {name}:\n                {value},\n"));
        }
    }
    output.push_str("        },\n    ))\n");
}

fn push_component_field(
    output: &mut String,
    alias: &str,
    component: &ComponentBinding,
    component_trait: &str,
    associated_type: &str,
) {
    let prefix = format!("    {}:", component_field(component));
    let type_path = format!(
        "<{} as {component_trait}>::{associated_type},",
        component_type(alias, component),
    );
    if prefix.len() + 1 + type_path.len() <= 100 {
        output.push_str(&format!("{prefix} {type_path}\n"));
    } else {
        output.push_str(&format!("{prefix}\n        {type_path}\n"));
    }
}

fn push_external_initialization(
    output: &mut String,
    alias: &str,
    component: &super::profile::ExternalComponent,
) {
    let field = component.state_field.as_str();
    let type_path = component_type(alias, &ComponentBinding::External(component.clone()));
    let initialization =
        format!("<{type_path} as HostProviderComponentInitialization>::initialize(");
    let statement = format!(
        "            {field}: {initialization}&self.{input})?,\n",
        input = component.input_field.as_str(),
    );
    if statement.trim_end().len() <= 100 {
        output.push_str(&statement);
        return;
    }
    let opening = format!("            {field}: {initialization}\n");
    if opening.trim_end().len() <= 100 {
        output.push_str(&format!(
            "{opening}                &self.{input},\n            )?,\n",
            input = component.input_field.as_str(),
        ));
    } else {
        output.push_str(&format!(
            "            {field}:\n                {initialization}\n                    &self.{input},\n                )?,\n",
            input = component.input_field.as_str(),
        ));
    }
}

fn component_field(component: &ComponentBinding) -> &str {
    match component {
        ComponentBinding::Future => "future",
        ComponentBinding::Stdlib => "stdlib",
        ComponentBinding::Json => "json",
        ComponentBinding::Time => "time",
        ComponentBinding::External(component) => component.state_field.as_str(),
    }
}

fn component_type(alias: &str, component: &ComponentBinding) -> String {
    match component {
        ComponentBinding::Future => format!("{alias}::FutureComponent"),
        ComponentBinding::Stdlib => format!("{alias}::gleam_stdlib::Component<Io>"),
        ComponentBinding::Json => format!("{alias}::gleam_json::Component"),
        ComponentBinding::Time => format!("{alias}::gleam_time::Component<Source>"),
        ComponentBinding::External(component) => {
            format!("{}::Component", component.crate_alias.as_str())
        }
    }
}

fn generics(components: &HostedComponents) -> &'static str {
    match components.capabilities() {
        HostedCapabilities::None => "",
        HostedCapabilities::Io => "<Io>",
        HostedCapabilities::IoAndTime => "<Io, Source>",
    }
}

fn profile_type(components: &HostedComponents) -> String {
    format!("Profile{}", generics(components))
}

fn push_bounds(output: &mut String, alias: &str, components: &HostedComponents) {
    output.push_str("where\n");
    output.push_str(&format!(
        "    Io: {alias}::gleam_stdlib::IoSink + 'static,\n"
    ));
    if components.has_time() {
        output.push_str(&format!("    Source: {alias}::gleam_time::TimeSource,\n"));
    }
}

fn push_bounds_open(output: &mut String, alias: &str, components: &HostedComponents) {
    if components.capabilities() == HostedCapabilities::None {
        output.push_str(" {\n");
    } else {
        output.push('\n');
        push_bounds(output, alias, components);
        output.push_str("{\n");
    }
}

fn push_binding(
    output: &mut String,
    pattern: &str,
    owner: &str,
    function: &FunctionBinding,
    method: &str,
) {
    let statement = format!(
        "    let {pattern} = {owner}.{method}(FunctionDeclaration::new({:?}))?;\n",
        function.gleam_name
    );
    // Rustfmt wraps this fallible call once the statement exceeds 98 columns.
    if statement.trim_end().len() <= 98 {
        output.push_str(&statement);
    } else {
        output.push_str(&format!(
            "    let {pattern} =\n        {owner}.{method}(FunctionDeclaration::new({:?}))?;\n",
            function.gleam_name
        ));
    }
}

impl DataType {
    fn collect_imports(&self, imports: &mut BTreeSet<&'static str>) {
        match self {
            Self::Int => {
                imports.insert("BigInt");
            }
            Self::String => {
                imports.insert("EcoString");
            }
            Self::BitArray => {
                imports.insert("BitArrayValue");
            }
            Self::List(item) => {
                imports.insert("List");
                item.collect_imports(imports);
            }
            Self::Option(item) => item.collect_imports(imports),
            Self::Future(item) => {
                imports.insert("FutureType");
                item.collect_imports(imports);
            }
            Self::Named(_) => {}
            Self::Tuple(elements) => {
                for element in elements {
                    element.collect_imports(imports);
                }
            }
            Self::Result(ok, error) => {
                ok.collect_imports(imports);
                error.collect_imports(imports);
            }
            Self::Float | Self::UtfCodepoint | Self::Bool | Self::Nil => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{hosted, plain, push_binding};
    use crate::builtin::BuiltInProvider;
    use crate::embedding::boundary::{DataType, FunctionBinding, PlainBindings};
    use crate::embedding::identifier::RustIdentifier;
    use crate::embedding::profile::{ExternalComponent, HostedBindings, HostedComponents};
    use camino::Utf8Path;
    use std::fs;

    #[test]
    fn reserves_layout_space_for_fallible_binding_statements() {
        let mut source = "fn bind() {\n".to_owned();
        for (pattern, owner, name) in [
            ("(mut bindings, function_0)", "builder", "validate_item"),
            ("(mut bindings, function_0)", "builder", "validate_batch"),
            ("(bindings, function_0)", "builder", "validate_products"),
            ("(bindings, function_0)", "builder", "validate_inventory"),
            ("function_1", "bindings", "normalize_inventory_codes_v1"),
            ("function_1", "bindings", "normalize_inventory_codes_v12"),
        ] {
            push_binding(
                &mut source,
                pattern,
                owner,
                &FunctionBinding {
                    gleam_name: name.to_owned(),
                    rust_name: identifier(name),
                    arguments: Vec::new(),
                    return_type: DataType::Nil,
                },
                "function",
            );
        }
        source.push_str("}\n");
        assert_eq!(
            source,
            r#"fn bind() {
    let (mut bindings, function_0) = builder.function(FunctionDeclaration::new("validate_item"))?;
    let (mut bindings, function_0) =
        builder.function(FunctionDeclaration::new("validate_batch"))?;
    let (bindings, function_0) = builder.function(FunctionDeclaration::new("validate_products"))?;
    let (bindings, function_0) =
        builder.function(FunctionDeclaration::new("validate_inventory"))?;
    let function_1 = bindings.function(FunctionDeclaration::new("normalize_inventory_codes_v1"))?;
    let function_1 =
        bindings.function(FunctionDeclaration::new("normalize_inventory_codes_v12"))?;
}
"#
        );
        assert_rustfmt_stable("fallible binding widths", &source);
    }

    #[test]
    fn renders_deterministic_plain_bindings_for_all_scalar_paths() {
        let bindings = PlainBindings {
            named_types: Vec::new(),
            geam_alias: RustIdentifier::parse("runtime")
                .expect("fixture crate alias should be valid"),
            root_module: "inventory_rules".to_owned(),
            first: FunctionBinding {
                gleam_name: "async".to_owned(),
                rust_name: RustIdentifier::parse("async")
                    .expect("keyword should use a raw identifier"),
                arguments: Vec::new(),
                return_type: DataType::Nil,
            },
            remaining: vec![FunctionBinding {
                gleam_name: "all_values".to_owned(),
                rust_name: RustIdentifier::parse("all_values")
                    .expect("fixture function should be valid"),
                arguments: vec![
                    DataType::Int,
                    DataType::Float,
                    DataType::String,
                    DataType::BitArray,
                    DataType::UtfCodepoint,
                    DataType::Bool,
                    DataType::Nil,
                ],
                return_type: DataType::String,
            }],
        };

        let source = plain(&bindings, Utf8Path::new("gleam"));
        assert_eq!(
            source,
            r#"// Generated by `geam embedding sync`. Do not edit.

use runtime::embedding::BigInt;
use runtime::embedding::BindingError;
use runtime::embedding::BitArrayValue;
use runtime::embedding::EcoString;
use runtime::embedding::Function;
use runtime::embedding::FunctionDeclaration;
use runtime::embedding::InputShape;
use runtime::embedding::ModuleBindings;
use runtime::embedding::ModuleBuilder;
use runtime::embedding::Project;

pub const ROOT_MODULE: &str = "inventory_rules";

pub fn project() -> Project {
    Project::new(concat!(env!("CARGO_MANIFEST_DIR"), "/gleam"), ROOT_MODULE)
}

#[allow(dead_code, clippy::type_complexity)]
pub struct Functions {
    pub r#async: Function<(), (), Function0Input>,
    pub all_values: Function<
        (BigInt, f64, EcoString, BitArrayValue, char, bool, ()),
        EcoString,
        Function1Input,
    >,
}

pub struct Function0Input;

impl InputShape<()> for Function0Input {}

pub struct Function1Input;

impl InputShape<(BigInt, f64, EcoString, BitArrayValue, char, bool, ())> for Function1Input {}

#[allow(dead_code)]
pub fn bind(builder: ModuleBuilder) -> Result<(ModuleBindings, Functions), BindingError> {
    let (mut bindings, function_0) = builder.function(FunctionDeclaration::new("async"))?;
    let function_1 = bindings.function(FunctionDeclaration::new("all_values"))?;
    Ok((
        bindings,
        Functions {
            r#async: function_0.with_input_shape(),
            all_values: function_1.with_input_shape(),
        },
    ))
}
"#,
        );
        assert_eq!(plain(&bindings, Utf8Path::new("gleam")), source);
        assert_rustfmt_stable("all scalar types", &source);
    }

    #[test]
    fn collects_recursive_imports_once() {
        let type_ = DataType::Future(Box::new(DataType::List(Box::new(DataType::List(
            Box::new(DataType::Result(
                Box::new(DataType::Tuple(vec![DataType::String, DataType::Int])),
                Box::new(DataType::Option(Box::new(DataType::BitArray))),
            )),
        )))));
        let mut imports = std::collections::BTreeSet::new();
        type_.collect_imports(&mut imports);
        type_.collect_imports(&mut imports);
        assert_eq!(
            imports.into_iter().collect::<Vec<_>>(),
            ["BigInt", "BitArrayValue", "EcoString", "FutureType", "List"]
        );
    }

    #[test]
    fn renders_one_host_contract_with_only_the_required_source_registration() {
        for source_required in [false, true] {
            let components = if source_required {
                HostedComponents::from_builtin(BuiltInProvider::Geam)
            } else {
                external_components()
            };
            let runtime = hosted_source(components);
            assert!(runtime.contains("pub fn project() -> HostedProject<Profile>"));
            assert!(runtime.contains("HostedModuleBuilder<Profile>"));
            assert!(runtime.contains("HostedModuleBindings<Profile>"));
            assert!(!runtime.contains("AsyncFunctions"));
            assert!(!runtime.contains("bind_async"));
            assert_eq!(
                runtime.contains("FutureComponent as ComponentRegistration"),
                source_required
            );
            assert!(runtime.contains("HostWorkProfile"));
            assert_rustfmt_stable("host runtime", &runtime);

            for builtin in [
                BuiltInProvider::Stdlib,
                BuiltInProvider::Json,
                BuiltInProvider::Time,
            ] {
                let mut components = HostedComponents::from_builtin(builtin);
                if source_required {
                    components.extend(HostedComponents::from_builtin(BuiltInProvider::Geam));
                }
                components.extend(external_components());
                let mixed = hosted_source(components);
                assert!(mixed.contains("HostComponentProfile"));
                assert!(mixed.contains("HostProviderComponent"));
                assert!(mixed.contains("Io: runtime::gleam_stdlib::IoSink + 'static"));
                assert_rustfmt_stable("built-in and external", &mixed);
            }
        }
        let mut long = HostedComponents::from_external(ExternalComponent {
            package: "a".to_owned(),
            input_field: identifier("a"),
            state_field: identifier("provider_a"),
            crate_alias: identifier("provider_with_a_deliberately_long_cargo_alias_for_calls"),
        });
        long.extend(external_components());
        let long = hosted_source(long);
        assert!(long.contains(
            "impl HostComponentProfile<provider_with_a_deliberately_long_cargo_alias_for_calls::Component>\n    for Profile\n{\n"
        ));
        assert_rustfmt_stable("long external component", &long);

        let very_long = HostedComponents::from_external(ExternalComponent {
            package: "native".to_owned(),
            input_field: identifier("native"),
            state_field: identifier("provider_native"),
            crate_alias: identifier(
                "provider_with_a_deliberately_long_cargo_alias_that_requires_wrapping",
            ),
        });
        let runtime = hosted_source(very_long);
        assert!(runtime.contains(
            "impl\n    HostComponentProfile<\n        provider_with_a_deliberately_long_cargo_alias_that_requires_wrapping::Component,\n    > for Profile\n{\n"
        ));
        assert_rustfmt_stable("wrapped component type", &runtime);

        let mut wrapped_call = external_components();
        wrapped_call.extend(HostedComponents::from_external(ExternalComponent {
            package: "long_name".to_owned(),
            input_field: identifier("long_name"),
            state_field: identifier("provider_long_name"),
            crate_alias: identifier("provider_with_a_long_public_name"),
        }));
        let wrapped_call = hosted_source(wrapped_call);
        assert!(wrapped_call.contains(
            "let additional_providers =\n        <provider_with_a_long_public_name::Component as ComponentRegistration<Profile>>::providers(\n        )?;"
        ));
        assert_rustfmt_stable("transfer wrapped registration call", &wrapped_call);
    }

    #[test]
    fn keeps_long_representable_identifiers_rustfmt_stable() {
        let name = "function_with_a_deliberately_long_but_representable_name_that_remains_part_of_the_public_boundary";
        let bindings = PlainBindings {
            named_types: Vec::new(),
            geam_alias: RustIdentifier::parse("runtime")
                .expect("fixture crate alias should be valid"),
            root_module: "boundary".to_owned(),
            first: FunctionBinding {
                gleam_name: name.to_owned(),
                rust_name: RustIdentifier::parse(name)
                    .expect("long identifier should remain representable"),
                arguments: vec![
                    DataType::Int,
                    DataType::Float,
                    DataType::String,
                    DataType::BitArray,
                    DataType::UtfCodepoint,
                    DataType::Bool,
                    DataType::Nil,
                ],
                return_type: DataType::String,
            },
            remaining: Vec::new(),
        };
        let source = plain(
            &bindings,
            Utf8Path::new(
                "../nested/gleam project with a deliberately long and \"quoted\" directory name for generated Rust loading",
            ),
        );
        assert!(source.contains(r#""/../nested/gleam project with a deliberately long and \"quoted\" directory name for generated Rust loading""#));
        assert_rustfmt_stable("long plain", &source);
    }

    #[test]
    fn renders_exact_host_capability_and_component_closures() {
        let stdlib = hosted_source(HostedComponents::from_builtin(BuiltInProvider::Stdlib));
        assert!(stdlib.contains("pub struct Profile<Io>"));
        assert!(stdlib.contains("pub fn project<Io>() -> HostedProject<Profile<Io>>"));
        assert!(stdlib.contains("host_providers::<Io>"));
        assert!(stdlib.contains("pub struct RunStateInputs<Io>"));
        assert!(stdlib.contains("pub stdlib: runtime::gleam_stdlib::GleamStdlibRunState<Io>"));
        assert!(stdlib.contains("pub fn initialize(self) -> RunState<Io>"));
        assert!(
            stdlib
                .contains("RunState {\n            future: (),\n            stdlib: self.stdlib,")
        );
        assert!(!stdlib.contains("ProviderConfigurations"));
        assert!(stdlib.contains("gleam_stdlib::Component<Io>"));
        assert!(!stdlib.contains("gleam_json::Component"));
        assert!(!stdlib.contains("gleam_time::Component"));
        assert!(!stdlib.contains("HostProviderComponentInitialization"));

        let json = hosted_source(HostedComponents::from_builtin(BuiltInProvider::Json));
        assert!(json.contains("pub struct Profile<Io>"));
        assert!(json.contains("gleam_stdlib::Component<Io>"));
        assert!(json.contains("gleam_json::Component"));
        assert!(!json.contains("    pub json:"));
        assert!(!json.contains("gleam_time::Component"));

        let time = hosted_source(HostedComponents::from_builtin(BuiltInProvider::Time));
        assert!(time.contains("pub struct Profile<Io, Source>"));
        assert!(
            time.contains(
                "pub type ProviderSet<Io, Source> = HostProviderSet<Profile<Io, Source>>;"
            )
        );
        assert!(time.contains("gleam_stdlib::Component<Io>"));
        assert!(time.contains("gleam_time::Component<Source>"));
        assert!(time.contains("    pub time: Source,"));
        assert!(time.contains("Source: runtime::gleam_time::TimeSource"));
        assert!(!time.contains("gleam_json::Component"));

        let mut json_and_time = HostedComponents::from_builtin(BuiltInProvider::Json);
        json_and_time.extend(HostedComponents::from_builtin(BuiltInProvider::Time));
        let json_and_time = hosted_source(json_and_time);
        assert!(json_and_time.contains("gleam_json::Component"));
        assert!(json_and_time.contains("gleam_time::Component<Source>"));

        for (label, source) in [
            ("stdlib", stdlib),
            ("json", json),
            ("time", time),
            ("json and time", json_and_time),
        ] {
            assert_rustfmt_stable(label, &source);
        }
    }

    #[test]
    fn renders_external_alias_configuration_and_mixed_state_without_accessors() {
        let external = external_components();
        let external_only = hosted_source(external_components());
        assert!(external_only.contains("pub struct Profile;"));
        assert!(external_only.contains("pub fn project() -> HostedProject<Profile>"));
        assert!(external_only.contains("        host_providers,"));
        assert!(external_only.contains("pub struct RunStateInputs"));
        assert!(external_only.contains("pub example_text_pattern: HostProviderConfiguration"));
        assert!(external_only.contains(
            "pub fn initialize(self) -> Result<RunState, HostProviderInitializationError>"
        ));
        assert!(external_only.contains("&self.example_text_pattern"));
        assert!(!external_only.contains("ProviderConfigurations"));
        assert!(external_only.contains("patterns::Component"));
        assert!(external_only.contains("HostProviderInitializationError"));
        assert!(external_only.contains("#[derive(Default)]\npub struct Stores"));
        assert!(!external_only.contains("impl Default for Stores"));
        assert!(!external_only.contains("pub fn stdlib("));
        assert!(!external_only.contains("gleam_stdlib::Component"));

        let mut two_external = external_components();
        two_external.extend(HostedComponents::from_external(ExternalComponent {
            package: "other_provider".to_owned(),
            input_field: identifier("other_provider"),
            state_field: identifier("provider_other_provider"),
            crate_alias: identifier("other"),
        }));
        let two_external = hosted_source(two_external);
        assert!(two_external.contains("pub other_provider: HostProviderConfiguration"));
        assert!(two_external.contains("other::Component"));
        let pattern_input = two_external
            .find("pub example_text_pattern:")
            .expect("text pattern input should be generated");
        let other_input = two_external
            .find("pub other_provider:")
            .expect("other provider input should be generated");
        assert!(pattern_input < other_input);

        let mut two_short = HostedComponents::from_external(ExternalComponent {
            package: "a".to_owned(),
            input_field: identifier("a"),
            state_field: identifier("provider_a"),
            crate_alias: identifier("p"),
        });
        two_short.extend(HostedComponents::from_external(ExternalComponent {
            package: "bb".to_owned(),
            input_field: identifier("bb"),
            state_field: identifier("provider_bb"),
            crate_alias: identifier("q"),
        }));
        let two_short = hosted_source(two_short);
        assert!(two_short.contains(
            "            provider_bb: <q::Component as HostProviderComponentInitialization>::initialize(\n                &self.bb,\n            )?,"
        ));

        let long_external = hosted_source(HostedComponents::from_external(ExternalComponent {
            package: "package_with_a_deliberately_long_name".to_owned(),
            input_field: identifier("package_with_a_deliberately_long_name"),
            state_field: identifier("provider_package_with_a_deliberately_long_name"),
            crate_alias: identifier("provider_with_a_deliberately_long_cargo_alias"),
        }));

        let reserved_external = hosted_source(HostedComponents::from_external(ExternalComponent {
            package: "stdlib".to_owned(),
            input_field: identifier("stdlib"),
            state_field: identifier("provider_stdlib"),
            crate_alias: identifier("reserved"),
        }));
        assert!(reserved_external.contains("pub provider_stdlib: HostProviderConfiguration"));
        assert!(!reserved_external.contains("pub stdlib: HostProviderConfiguration"));

        let mut escaped_collisions = HostedComponents::from_external(ExternalComponent {
            package: "crate".to_owned(),
            input_field: identifier("_crate"),
            state_field: identifier("provider__crate"),
            crate_alias: identifier("escaped"),
        });
        escaped_collisions.extend(HostedComponents::from_external(ExternalComponent {
            package: "_crate".to_owned(),
            input_field: identifier("_crate"),
            state_field: identifier("provider__crate"),
            crate_alias: identifier("natural"),
        }));
        let escaped_collisions = hosted_source(escaped_collisions);
        assert!(escaped_collisions.contains("pub _crate: HostProviderConfiguration"));
        assert!(escaped_collisions.contains("pub provider__crate: HostProviderConfiguration"));
        assert!(escaped_collisions.contains("provider__crate: <natural::Component"));
        assert!(escaped_collisions.contains("provider_provider__crate: <escaped::Component"));

        let mut mixed = HostedComponents::from_builtin(BuiltInProvider::Time);
        mixed.extend(external);
        mixed.extend(HostedComponents::from_external(ExternalComponent {
            package: "other_provider".to_owned(),
            input_field: identifier("other_provider"),
            state_field: identifier("provider_other_provider"),
            crate_alias: identifier("other"),
        }));
        let mixed = hosted_source(mixed);
        assert!(mixed.contains("pub struct Profile<Io, Source>"));
        assert!(
            mixed.contains(
                "pub type ProviderSet<Io, Source> = HostProviderSet<Profile<Io, Source>>;"
            )
        );
        assert!(mixed.contains("gleam_stdlib::Component<Io>"));
        assert!(mixed.contains("gleam_time::Component<Source>"));
        assert!(mixed.contains("patterns::Component"));
        assert!(mixed.contains("other::Component"));
        assert!(mixed.contains("pub stdlib: runtime::gleam_stdlib::GleamStdlibRunState<Io>"));
        assert!(mixed.contains("pub time: Source"));
        assert!(mixed.contains("pub example_text_pattern: HostProviderConfiguration"));
        assert!(mixed.contains("pub other_provider: HostProviderConfiguration"));
        assert!(mixed.contains("pub fn stdlib("));
        assert!(!mixed.contains("pub fn provider_example_text_pattern"));

        assert_rustfmt_stable("external", &external_only);
        assert_rustfmt_stable("two external", &two_external);
        assert_rustfmt_stable("two short external", &two_short);
        assert_rustfmt_stable("long external", &long_external);
        assert_rustfmt_stable("reserved external", &reserved_external);
        assert_rustfmt_stable("escaped collisions", &escaped_collisions);
        assert_rustfmt_stable("mixed", &mixed);
    }

    fn hosted_source(components: HostedComponents) -> String {
        hosted(
            &HostedBindings {
                boundary: PlainBindings {
                    named_types: Vec::new(),
                    geam_alias: identifier("runtime"),
                    root_module: "inventory_rules".to_owned(),
                    first: FunctionBinding {
                        gleam_name: "normalize".to_owned(),
                        rust_name: identifier("normalize"),
                        arguments: vec![DataType::String],
                        return_type: DataType::String,
                    },
                    remaining: vec![FunctionBinding {
                        gleam_name: "ready".to_owned(),
                        rust_name: identifier("ready"),
                        arguments: Vec::new(),
                        return_type: DataType::Bool,
                    }],
                },
                components,
            },
            Utf8Path::new("gleam"),
        )
    }

    fn external_components() -> HostedComponents {
        HostedComponents::from_external(ExternalComponent {
            package: "example_text_pattern".to_owned(),
            input_field: identifier("example_text_pattern"),
            state_field: identifier("provider_example_text_pattern"),
            crate_alias: identifier("patterns"),
        })
    }

    fn identifier(value: &str) -> RustIdentifier {
        RustIdentifier::parse(value).expect("fixture identifier should be valid")
    }

    #[test]
    fn named_generic_metadata_preserves_nested_shapes_and_formatting() {
        use super::named::push_types;
        use crate::embedding::boundary::{ClosedType, NamedKind, NamedType};
        let nested = NamedType {
            package: "some_package".to_owned(),
            module: "nested/module".to_owned(),
            name: "Resource".to_owned(),
            kind: NamedKind::External,
            arguments: vec![ClosedType::String],
        };
        let types = [
            NamedType {
                arguments: vec![],
                ..nested.clone()
            },
            NamedType {
                name: "Session".to_owned(),
                kind: NamedKind::Custom,
                arguments: vec![
                    ClosedType::Int,
                    ClosedType::Float,
                    ClosedType::String,
                    ClosedType::BitArray,
                    ClosedType::UtfCodepoint,
                    ClosedType::Bool,
                    ClosedType::Nil,
                    ClosedType::List(Box::new(ClosedType::Tuple(vec![
                        ClosedType::Function(vec![], Box::new(ClosedType::Int)),
                        ClosedType::Named(nested.clone()),
                    ]))),
                    ClosedType::Named(NamedType {
                        kind: NamedKind::Custom,
                        ..nested
                    }),
                ],
                package: "application".to_owned(),
                module: "library".to_owned(),
            },
        ];
        for alias in ["geam", "runtime_dependency_with_a_long_name"] {
            let mut output = String::new();
            push_types(&mut output, alias, &types);
            output.push_str("fn main() {}\n");
            assert!(output.contains("ValueType::Function(Box::new("));
            assert!(output.contains("ValueType::External("));
            assert!(output.contains("ValueType::Custom("));
            assert_rustfmt_stable("nested opaque metadata", &output);
        }
    }

    fn assert_rustfmt_stable(label: &str, source: &str) {
        let directory = tempfile::tempdir().expect("temporary directory should be created");
        let path = directory.path().join("geam_bindings.rs");
        fs::write(&path, source).expect("generated source should be written");
        for style_edition in ["2015", "2024"] {
            let output = std::process::Command::new("rustfmt")
                .args([
                    "--edition",
                    "2024",
                    "--style-edition",
                    style_edition,
                    "--check",
                ])
                .arg(&path)
                .output()
                .expect("rustfmt should start");
            let stdout = String::from_utf8_lossy(&output.stdout);
            let stderr = String::from_utf8_lossy(&output.stderr);
            assert!(
                output.status.success(),
                "{label} generated source should already be formatted under style edition {style_edition}:\nstdout:\n{stdout}\nstderr:\n{stderr}\nsource:\n{source}",
            );
        }
    }
}
