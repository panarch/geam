use super::super::boundary::PlainBindings;
use super::super::package::Generation;
use super::super::profile::{HostedBindings, HostedCapabilities};
use super::{
    generics, hosted, plain, profile_type, push_binding, push_binding_result, push_bounds_open,
};
use camino::Utf8Path;

pub(super) fn push_plain_load(output: &mut String, bindings: &PlainBindings) {
    output.push_str("pub fn load() -> Result<(Module, Functions), PreparedError> {\n");
    output.push_str("    let mut bindings = program::PROGRAM.load()?;\n");
    push_loaded_bindings(output, bindings);
    output.push_str("}\n");
}

pub(super) fn push_hosted_load(output: &mut String, bindings: &HostedBindings) {
    let components = &bindings.components;
    let parameters = generics(components);
    output.push_str(&format!(
        "pub fn load{parameters}() -> Result<(HostedModule<{}>, Functions), PreparedError>",
        profile_type(components),
    ));
    push_bounds_open(output, bindings.boundary.geam_alias.as_str(), components);
    let registration = if parameters.is_empty() {
        "host_providers()".to_owned()
    } else {
        format!("host_providers::{parameters}()")
    };
    output.push_str(&format!(
        "    let mut bindings = program::PROGRAM.load({registration}?)?;\n",
    ));
    push_loaded_bindings(output, &bindings.boundary);
    output.push_str("}\n");
}

fn push_loaded_bindings(output: &mut String, bindings: &PlainBindings) {
    for (index, function) in bindings.functions().enumerate() {
        push_binding(
            output,
            &format!("function_{index}"),
            "bindings",
            function,
            "function",
        );
    }
    push_binding_result(output, bindings, "Functions", "bindings.seal()");
}

pub(in crate::embedding) fn plain_helper(bindings: &PlainBindings) -> String {
    let mut output = plain(bindings, Utf8Path::new("gleam"), Generation::Dynamic);
    push_helper_arguments(&mut output);
    output.push_str("    let program = Project::new(root, ROOT_MODULE).compile()?\n");
    push_source_paths(&mut output);
    output.push_str(
        "    let builder = ModuleBuilder::from_program(program)?;\n    let (bindings, _) = bind(builder)?;\n    std::fs::write(destination, bindings.prepare().emit_rust())?;\n    Ok(())\n}\n",
    );
    output
}

pub(in crate::embedding) fn hosted_helper(bindings: &HostedBindings) -> String {
    let mut output = hosted(bindings, Utf8Path::new("gleam"), Generation::Dynamic);
    push_helper_arguments(&mut output);
    let alias = bindings.boundary.geam_alias.as_str();
    let registration = match bindings.components.capabilities() {
        HostedCapabilities::None => "host_providers".to_owned(),
        HostedCapabilities::Io => {
            format!("host_providers::<Vec<{alias}::gleam_stdlib::IoOutput>>")
        }
        HostedCapabilities::IoAndTime => format!(
            "host_providers::<Vec<{alias}::gleam_stdlib::IoOutput>, {alias}::gleam_time::SystemTimeSource>",
        ),
    };
    output.push_str(&format!(
        "    let program = HostedProject::new(root, ROOT_MODULE, {registration}).compile()?\n",
    ));
    push_source_paths(&mut output);
    output.push_str(
        "    let builder = HostedModuleBuilder::new(program)?;\n    let (bindings, _) = bind(builder)?;\n    std::fs::write(destination, bindings.prepare()?.emit_rust())?;\n    Ok(())\n}\n",
    );
    output
}

fn push_helper_arguments(output: &mut String) {
    output.push_str(
        "\nfn main() -> Result<(), Box<dyn std::error::Error>> {\n    let mut arguments = std::env::args().skip(1);\n    let root = arguments.next().ok_or(\"missing Gleam project path\")?;\n    let destination = arguments.next().ok_or(\"missing prepared output path\")?;\n",
    );
}

fn push_source_paths(output: &mut String) {
    output.push_str(
        "        .map_source_paths(|package, module, _| format!(\"{package}/src/{module}.gleam\").into());\n",
    );
}

#[cfg(test)]
mod tests {
    use super::{hosted_helper, plain_helper, push_hosted_load, push_plain_load};
    use crate::builtin::BuiltInProvider;
    use crate::embedding::boundary::{DataType, FunctionBinding, PlainBindings};
    use crate::embedding::identifier::RustIdentifier;
    use crate::embedding::profile::{HostedBindings, HostedComponents};

    fn boundary() -> PlainBindings {
        PlainBindings {
            geam_alias: RustIdentifier::parse("runtime").unwrap(),
            root_module: "application".into(),
            first: FunctionBinding {
                gleam_name: "double".into(),
                rust_name: RustIdentifier::parse("double").unwrap(),
                arguments: vec![DataType::Int],
                return_type: DataType::Int,
            },
            remaining: Vec::new(),
            named_types: Vec::new(),
        }
    }

    #[test]
    fn renders_exact_plain_load_without_source_compilation() {
        let mut source = String::new();
        push_plain_load(&mut source, &boundary());
        assert_eq!(
            source,
            r#"pub fn load() -> Result<(Module, Functions), PreparedError> {
    let mut bindings = program::PROGRAM.load()?;
    let function_0 = bindings.function(FunctionDeclaration::new("double"))?;
    Ok((
        bindings.seal(),
        Functions {
            double: function_0.with_input_shape(),
        },
    ))
}
"#
        );
        let helper = plain_helper(&boundary());
        let (_, main) = helper.split_once("\nfn main()").unwrap();
        assert_eq!(
            main,
            r#" -> Result<(), Box<dyn std::error::Error>> {
    let mut arguments = std::env::args().skip(1);
    let root = arguments.next().ok_or("missing Gleam project path")?;
    let destination = arguments.next().ok_or("missing prepared output path")?;
    let program = Project::new(root, ROOT_MODULE).compile()?
        .map_source_paths(|package, module, _| format!("{package}/src/{module}.gleam").into());
    let builder = ModuleBuilder::from_program(program)?;
    let (bindings, _) = bind(builder)?;
    std::fs::write(destination, bindings.prepare().emit_rust())?;
    Ok(())
}
"#
        );
        assert!(!helper.contains("mod program;"));
    }

    #[test]
    fn keeps_hosted_registration_and_capability_generics_in_each_profile() {
        for (components, generic, bounds, registration, preparation) in [
            (
                HostedComponents::default(),
                "",
                " {\n",
                "host_providers()",
                "host_providers",
            ),
            (
                HostedComponents::from_builtin(BuiltInProvider::Stdlib),
                "<Io>",
                "\nwhere\n    Io: runtime::gleam_stdlib::IoSink + 'static,\n{\n",
                "host_providers::<Io>()",
                "host_providers::<Vec<runtime::gleam_stdlib::IoOutput>>",
            ),
            (
                HostedComponents::from_builtin(BuiltInProvider::Time),
                "<Io, Source>",
                "\nwhere\n    Io: runtime::gleam_stdlib::IoSink + 'static,\n    Source: runtime::gleam_time::TimeSource,\n{\n",
                "host_providers::<Io, Source>()",
                "host_providers::<Vec<runtime::gleam_stdlib::IoOutput>, runtime::gleam_time::SystemTimeSource>",
            ),
        ] {
            let bindings = HostedBindings {
                boundary: boundary(),
                components,
            };
            let mut source = String::new();
            push_hosted_load(&mut source, &bindings);
            assert_eq!(
                source,
                format!(
                    "pub fn load{generic}() -> Result<(HostedModule<Profile{generic}>, Functions), PreparedError>{bounds}    let mut bindings = program::PROGRAM.load({registration}?)?;\n    let function_0 = bindings.function(FunctionDeclaration::new(\"double\"))?;\n    Ok((\n        bindings.seal(),\n        Functions {{\n            double: function_0.with_input_shape(),\n        }},\n    ))\n}}\n"
                )
            );
            let helper = hosted_helper(&bindings);
            let (_, main) = helper.split_once("\nfn main()").unwrap();
            assert_eq!(
                main,
                format!(
                    " -> Result<(), Box<dyn std::error::Error>> {{\n    let mut arguments = std::env::args().skip(1);\n    let root = arguments.next().ok_or(\"missing Gleam project path\")?;\n    let destination = arguments.next().ok_or(\"missing prepared output path\")?;\n    let program = HostedProject::new(root, ROOT_MODULE, {preparation}).compile()?\n        .map_source_paths(|package, module, _| format!(\"{{package}}/src/{{module}}.gleam\").into());\n    let builder = HostedModuleBuilder::new(program)?;\n    let (bindings, _) = bind(builder)?;\n    std::fs::write(destination, bindings.prepare()?.emit_rust())?;\n    Ok(())\n}}\n"
                )
            );
            assert!(!helper.contains("mod program;"));
        }
    }
}
