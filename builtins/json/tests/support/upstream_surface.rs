use geam_core::TypedProgram;
use gleam_core::type_::printer::Printer;

pub(crate) struct ExpectedSurface {
    pub(crate) values: &'static [&'static str],
    pub(crate) types: &'static [(&'static str, usize)],
    pub(crate) type_aliases: &'static [&'static str],
    pub(crate) constructors: &'static [(&'static str, &'static str, usize)],
    pub(crate) functions: &'static str,
}

pub(crate) fn assert_module_surface(
    program: &TypedProgram,
    package: &str,
    module_name: &str,
    expected: &ExpectedSurface,
) {
    let module = program
        .modules()
        .find(|module| module.type_info.package == package && module.name == module_name)
        .expect("upstream dependency module should be loaded");

    let mut values = module
        .type_info
        .values
        .iter()
        .filter(|(_, value)| value.publicity.is_public())
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    values.sort_unstable();
    assert_eq!(values.as_slice(), expected.values);

    let mut types = module
        .type_info
        .types
        .iter()
        .filter(|(_, type_)| type_.publicity.is_public())
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    types.sort_unstable();
    let mut expected_types = expected
        .types
        .iter()
        .map(|(name, _)| *name)
        .chain(expected.type_aliases.iter().copied())
        .collect::<Vec<_>>();
    expected_types.sort_unstable();
    assert_eq!(types, expected_types);

    let mut type_aliases = module
        .type_info
        .types
        .iter()
        .filter(|(name, type_)| {
            type_.publicity.is_public() && module.type_info.type_aliases.contains_key(*name)
        })
        .map(|(name, _)| name.as_str())
        .collect::<Vec<_>>();
    type_aliases.sort_unstable();
    assert_eq!(type_aliases.as_slice(), expected.type_aliases);

    let mut custom_types = module
        .definitions
        .custom_types
        .iter()
        .filter(|type_| type_.publicity.is_public())
        .map(|type_| (type_.name.as_str(), type_.parameters.len()))
        .collect::<Vec<_>>();
    custom_types.sort_unstable();
    assert_eq!(custom_types.as_slice(), expected.types);

    let mut constructors = module
        .definitions
        .custom_types
        .iter()
        .filter(|type_| type_.publicity.is_public())
        .flat_map(|type_| {
            type_
                .constructors
                .iter()
                .filter(|constructor| values.contains(&constructor.name.as_str()))
                .map(|constructor| {
                    (
                        type_.name.as_str(),
                        constructor.name.as_str(),
                        constructor.arguments.len(),
                    )
                })
        })
        .collect::<Vec<_>>();
    constructors.sort_unstable();
    assert_eq!(constructors.as_slice(), expected.constructors);

    let mut functions = module
        .definitions
        .functions
        .iter()
        .filter(|function| function.publicity.is_public())
        .map(|function| {
            let (_, name) = function
                .name
                .as_ref()
                .expect("public module function should have a name");
            let mut printer = Printer::new_without_type_variables(&module.names);
            let mut signature = String::from(name.as_str());
            signature.push_str(": fn(");

            for (index, argument) in function.arguments.iter().enumerate() {
                if index > 0 {
                    signature.push_str(", ");
                }
                if let Some(label) = argument.names.get_label() {
                    signature.push_str(label);
                    signature.push_str(": ");
                }
                signature.push_str(&printer.print_type(&argument.type_));
            }

            signature.push_str(") -> ");
            signature.push_str(&printer.print_type(&function.return_type));
            signature
        })
        .collect::<Vec<_>>();
    functions.sort_unstable();
    assert_eq!(functions.join("\n"), expected.functions.trim());
}
