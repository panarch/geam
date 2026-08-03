use super::super::super::{AnonymousFunctions, FunctionToPlan, discarded_function_params};
use super::{LinkedFunction, LinkedModule};
use crate::host::{
    HostFunctionConstructions, HostFunctionSchema, RegisteredHostFunction,
    RegisteredHostImplementationId,
};
use crate::plan::{FunctionTemplateId, HostFunctionTemplate, ModuleId};
use crate::planner::context::FunctionInfo;
use crate::planner::error::{HostProviderLinkReason, PlanError};
use crate::planner::type_parameter::TypeParameterScope;
use ecow::EcoString;
use gleam_core::ast::TypedFunction;
use std::collections::{BTreeMap, HashMap, HashSet};

pub(super) fn select_erlang_hosted_functions(
    functions: Vec<TypedFunction>,
    providers: &[RegisteredHostFunction],
) -> Vec<TypedFunction> {
    let provided = providers
        .iter()
        .map(|provider| provider.schema().name())
        .collect::<HashSet<_>>();
    functions
        .into_iter()
        .filter(|function| {
            function.implementations.can_run_on_erlang
                || function
                    .name
                    .as_ref()
                    .is_none_or(|(_, name)| provided.contains(name))
        })
        .collect()
}

pub(super) fn link_source_functions(
    package: EcoString,
    module: EcoString,
    functions: Vec<FunctionToPlan>,
    providers: Vec<RegisteredHostFunction>,
) -> Result<(Vec<LinkedFunction>, HashSet<EcoString>), PlanError> {
    let providers = providers
        .into_iter()
        .map(|definition| (definition.schema().name().clone(), definition))
        .collect::<BTreeMap<_, _>>();
    functions
        .into_iter()
        .try_fold(
            (Vec::new(), providers, HashSet::new()),
            |(mut linked, mut providers, mut executable_externals), function| {
                let name = function.name;
                let external = function.function.external_erlang.is_some()
                    || function.function.external_javascript.is_some();
                if let Some(provider) = providers.remove(&name) {
                    if !external {
                        return Err(PlanError::HostProviderLink {
                            package: package.clone(),
                            module: module.clone(),
                            function: name,
                            reason: Box::new(HostProviderLinkReason::NonExternalFunction),
                        });
                    }
                    executable_externals.insert(name);
                    bind_source_host_function(
                        package.clone(),
                        module.clone(),
                        provider,
                        &function.info,
                    )
                    .map(|(template, constructions, implementation)| {
                        linked.push(LinkedFunction::Host {
                            template,
                            constructions,
                            implementation,
                        });
                        (linked, providers, executable_externals)
                    })
                } else if external {
                    if function.function.body.is_empty() {
                        return Err(PlanError::MissingHostProvider {
                            package: package.clone(),
                            module: module.clone(),
                            function: name,
                        });
                    }
                    executable_externals.insert(name.clone());
                    linked.push(LinkedFunction::ExternalFallback {
                        name,
                        info: function.info,
                        function: function.function,
                    });
                    Ok((linked, providers, executable_externals))
                } else {
                    linked.push(LinkedFunction::Gleam {
                        info: function.info,
                        function: function.function,
                    });
                    Ok((linked, providers, executable_externals))
                }
            },
        )
        .and_then(|(linked, providers, executable_externals)| {
            if let Some((function, _)) = providers.into_iter().next() {
                Err(PlanError::HostProviderLink {
                    package,
                    module,
                    function,
                    reason: Box::new(HostProviderLinkReason::MissingDeclaration),
                })
            } else {
                Ok((linked, executable_externals))
            }
        })
}

pub(super) fn link_source_less_module(
    id: ModuleId,
    package: EcoString,
    module_name: EcoString,
    functions: Vec<RegisteredHostFunction>,
) -> LinkedModule {
    let function_count = functions.len();
    let mut functions_by_name = HashMap::with_capacity(function_count);
    let mut linked_functions = Vec::with_capacity(function_count);
    for (function_index, definition) in functions.into_iter().enumerate() {
        let function_id = FunctionTemplateId::in_module(id, function_index);
        let (template, constructions, implementation) = bind_source_less_host_function(
            function_id,
            package.clone(),
            module_name.clone(),
            definition,
        );
        let info = host_function_info(&template);
        functions_by_name.insert(template.name().clone(), info);
        linked_functions.push(LinkedFunction::Host {
            template,
            constructions,
            implementation,
        });
    }
    LinkedModule {
        id,
        package,
        module_name,
        source_context: None,
        custom_types: Vec::new(),
        external_types: Vec::new(),
        functions_by_name,
        functions: linked_functions,
        executable_externals: HashSet::new(),
        constants: Vec::new(),
        anonymous_functions: AnonymousFunctions::in_module(id, function_count),
    }
}

fn bind_source_host_function(
    package: EcoString,
    module: EcoString,
    definition: RegisteredHostFunction,
    source: &FunctionInfo,
) -> Result<
    (
        HostFunctionTemplate,
        HostFunctionConstructions,
        RegisteredHostImplementationId,
    ),
    PlanError,
> {
    let (schema, constructions, implementation) = definition.into_parts();
    let registered_shape = host_function_shape(&schema);
    if source.signature.scheme() != schema.scheme() || source.signature.shape() != &registered_shape
    {
        return Err(PlanError::HostProviderLink {
            package,
            module,
            function: schema.name().clone(),
            reason: Box::new(HostProviderLinkReason::SchemeMismatch {
                expected_scheme: source.signature.scheme().clone(),
                expected_type: source.signature.shape().type_(),
                actual_scheme: schema.scheme().clone(),
                actual_type: registered_shape.type_(),
            }),
        });
    }
    let site =
        crate::plan::HostCallSite::new(module, schema.name().clone(), source.definition_span);
    let template =
        HostFunctionTemplate::from_schema(source.signature.clone(), package, site, schema);
    Ok((template, constructions, implementation))
}

fn bind_source_less_host_function(
    id: FunctionTemplateId,
    package: EcoString,
    module: EcoString,
    definition: RegisteredHostFunction,
) -> (
    HostFunctionTemplate,
    HostFunctionConstructions,
    RegisteredHostImplementationId,
) {
    let (schema, constructions, implementation) = definition.into_parts();
    let registered_shape = host_function_shape(&schema);
    let signature =
        crate::plan::FunctionTemplateSignature::new(id, schema.scheme().clone(), registered_shape);
    let site = crate::plan::HostCallSite::new(
        module,
        schema.name().clone(),
        crate::plan::SourceSpan::new(0, 0),
    );
    let template = HostFunctionTemplate::from_schema(signature, package, site, schema);
    (template, constructions, implementation)
}

fn host_function_shape(schema: &HostFunctionSchema) -> crate::plan::FunctionShape {
    crate::plan::FunctionShape::new(
        schema
            .parameters()
            .iter()
            .map(crate::host::HostTypeDescriptor::value_shape)
            .collect(),
        schema.return_type().value_shape(),
    )
}

fn host_function_info(template: &HostFunctionTemplate) -> FunctionInfo {
    let shape = template.signature().shape();
    FunctionInfo {
        signature: template.signature().clone(),
        type_parameters: TypeParameterScope::default(),
        return_shape: shape.return_shape().clone(),
        params: discarded_function_params(shape.argument_shapes()),
        definition_span: template.site().span(),
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::plan_host_program;
    use crate::frontend::{ModuleSource, PackageSource, compile_typed_host_program};
    use crate::host::{HostModule, HostProviderModule, HostProviderSet, StatelessHostProfile};
    use crate::plan::{FunctionType, ValueType};
    use crate::planner::{HostProviderLinkReason, PlanError};
    use ecow::EcoString;
    use num_bigint::BigInt;

    #[test]
    fn omits_unprovided_functions_unavailable_on_the_selected_target() {
        let typed = compile_typed_host_program(
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
import support

pub fn main() {
  support.available()
}
"#,
                    )],
                ),
                PackageSource::new(
                    "library",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "support",
                        "support.gleam",
                        r#"
@external(javascript, "./support.mjs", "javascript_only")
fn javascript_only() -> Int

pub fn available() {
  1
}
"#,
                    )],
                ),
            ],
            HostProviderSet::new(Vec::<HostModule>::new())
                .expect("empty host modules should be valid"),
        )
        .expect("target-unavailable private dependency function should compile");

        let plan = plan_host_program(typed).expect("selected target functions should plan");

        assert_eq!(
            plan.modules()[0]
                .functions()
                .iter()
                .map(|function| {
                    function
                        .gleam_body()
                        .expect("selected source function should have a body")
                        .name()
                        .as_str()
                })
                .collect::<Vec<_>>(),
            ["available"],
        );
        assert_eq!(plan.modules()[1].functions().len(), 1);
        assert_eq!(
            plan.modules()[1].functions()[0]
                .gleam_body()
                .expect("root main should have a body")
                .name(),
            "main",
        );
    }

    #[test]
    fn retains_target_unavailable_external_when_a_provider_is_registered() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_function("javascript_only", BigInt::default)
            .expect("provider function should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
@external(javascript, "./support.mjs", "javascript_only")
fn javascript_only() -> Int

pub fn main() {
  1
}
"#,
                )],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("target-unavailable private function should compile");

        let plan = plan_host_program(typed).expect("explicit provider should link");

        assert_eq!(plan.modules()[0].functions().len(), 2);
        assert_eq!(
            plan.modules()[0].functions()[1]
                .host_template()
                .expect("registered target-unavailable external should remain a host template")
                .name(),
            "javascript_only",
        );
    }

    #[test]
    fn source_provider_and_gleam_fallback_keep_distinct_body_owners() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_function("provided", std::convert::identity::<BigInt>)
            .expect("provider function should be valid");
        let source = r#"
@external(erlang, "host", "provided")
fn provided(value: Int) -> Int

@external(erlang, "host", "fallback")
fn fallback(value: Int) -> Int {
  value + 1
}

pub fn main() {
  #(provided(1), fallback(2))
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("source should compile");
        let plan = plan_host_program(typed).expect("provider and fallback should plan");
        let functions = plan.modules()[0].functions();

        assert_eq!(
            functions
                .iter()
                .map(|function| {
                    (
                        function
                            .gleam_body()
                            .map(|function| function.name().as_str()),
                        function
                            .host_template()
                            .map(|function| function.name().as_str()),
                    )
                })
                .collect::<Vec<_>>(),
            [
                (Some("main"), None),
                (None, Some("provided")),
                (Some("fallback"), None),
            ],
        );
    }

    #[test]
    fn missing_provider_precedes_source_body_planning() {
        let typed = compile_typed_host_program(
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
const unsupported = <<1:native>>

pub fn main() { 1 }
"#,
                    )],
                ),
                PackageSource::new(
                    "library",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "support",
                        "support.gleam",
                        r#"
@external(erlang, "support", "native")
pub fn native() -> Int
"#,
                    )],
                ),
            ],
            HostProviderSet::new(Vec::<HostModule>::new())
                .expect("empty host modules should be valid"),
        )
        .expect("profile-out source should still compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::MissingHostProvider {
                package: "library".into(),
                module: "support".into(),
                function: "native".into(),
            }),
        );
    }

    #[test]
    fn provider_function_must_link_to_a_source_declaration() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_function("native", BigInt::default)
            .expect("provider function should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { 1 }",
                )],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("host program should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "main".into(),
                function: "native".into(),
                reason: Box::new(HostProviderLinkReason::MissingDeclaration),
            }),
        );
    }

    #[test]
    fn missing_provider_declaration_reports_the_lexically_first_function() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_function("zeta", BigInt::default)
            .expect("provider function should be valid")
            .with_function("alpha", BigInt::default)
            .expect("provider function should be valid");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    "pub fn main() { 1 }",
                )],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("host program should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "main".into(),
                function: "alpha".into(),
                reason: Box::new(HostProviderLinkReason::MissingDeclaration),
            }),
        );
    }

    #[test]
    fn provider_function_cannot_override_an_ordinary_gleam_body() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_function("native", BigInt::default)
            .expect("provider function should be valid");
        let source = r#"
fn native() {
  1
}

pub fn main() {
  native()
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("host program should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "main".into(),
                function: "native".into(),
                reason: Box::new(HostProviderLinkReason::NonExternalFunction),
            }),
        );
    }

    #[test]
    fn provider_function_scheme_must_exactly_match_the_external_declaration() {
        let provider = HostProviderModule::<StatelessHostProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_function("native", |value: BigInt| value)
            .expect("provider function should be valid");
        let source = r#"
@external(erlang, "host", "native")
fn native(value: Bool) -> Bool

pub fn main() {
  native(True)
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<EcoString>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::with_providers(Vec::<HostModule>::new(), [provider])
                .expect("provider module should be unique"),
        )
        .expect("host program should compile");

        assert_eq!(
            plan_host_program(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "main".into(),
                function: "native".into(),
                reason: Box::new(HostProviderLinkReason::SchemeMismatch {
                    expected_scheme: crate::plan::TypeScheme::new(0),
                    expected_type: FunctionType::new(vec![ValueType::Bool], ValueType::Bool,),
                    actual_scheme: crate::plan::TypeScheme::new(0),
                    actual_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                }),
            }),
        );
    }
}
