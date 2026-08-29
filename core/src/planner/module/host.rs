mod body;
mod constant;
mod declaration;
mod link;

use crate::frontend::{HostedTypedProgram, HostedTypedProgramModule};
use crate::host::{
    HostProfile, RegisteredHostConstructions, RegisteredHostImplementationId,
    RegisteredHostImplementations, RegisteredHostProviderModule,
};
use crate::plan::{HostImplementationBinding, HostedLibraryModulePlan, HostedModulePlan, ModuleId};
use crate::planner::error::PlanError;

pub fn plan_host_program<Profile: HostProfile>(
    program: HostedTypedProgram<Profile>,
) -> Result<HostedModulePlan<Profile>, PlanError> {
    let (root_index, modules, providers, implementations) = program.into_parts();
    plan_host_program_schema(root_index, modules, providers, super::ModuleRole::Root).map(
        |planned| {
            let implementation_bindings =
                bind_implementations(planned.implementations, &implementations);
            HostedModulePlan::new(
                planned.root,
                crate::plan::FunctionTemplateId::in_module(planned.root, 0),
                planned.modules,
                implementation_bindings,
            )
        },
    )
}

pub(crate) fn plan_host_library_program<Profile: HostProfile>(
    program: HostedTypedProgram<Profile>,
) -> Result<HostedLibraryModulePlan<Profile>, PlanError> {
    let (root_index, modules, providers, implementations) = program.into_parts();
    plan_host_program_schema(root_index, modules, providers, super::ModuleRole::Library).map(
        |planned| {
            let implementation_bindings =
                bind_implementations(planned.implementations, &implementations);
            HostedLibraryModulePlan::new(planned.root, planned.modules, implementation_bindings)
        },
    )
}

fn bind_implementations<Profile: HostProfile>(
    planned: Vec<(
        crate::plan::FunctionTemplateId,
        RegisteredHostConstructions,
        RegisteredHostImplementationId,
    )>,
    implementations: &RegisteredHostImplementations<Profile>,
) -> Vec<HostImplementationBinding<Profile>> {
    planned
        .into_iter()
        .map(|(template, constructions, implementation)| {
            HostImplementationBinding::new(
                template,
                constructions,
                implementations.implementation(implementation),
            )
        })
        .collect()
}

fn plan_host_program_schema(
    root_index: usize,
    modules: Vec<HostedTypedProgramModule>,
    providers: Vec<RegisteredHostProviderModule>,
    root_role: super::ModuleRole,
) -> Result<body::PlannedHostedProgram, PlanError> {
    let root = ModuleId::new(root_index);
    declaration::collect_hosted_module_declarations(modules, providers)
        .and_then(|declarations| link::link_hosted_modules(root, root_role, declarations))
        .and_then(constant::reserve_hosted_constants)
        .and_then(|(registry, modules)| constant::plan_hosted_constant_bodies(registry, modules))
        .and_then(|(registry, modules)| body::plan_hosted_modules(root, &registry, modules))
}

#[cfg(test)]
mod tests {
    use super::plan_host_program;
    use crate::frontend::{ModuleSource, PackageSource, compile_typed_host_program};
    use crate::host::{HostModule, HostParameter, HostProviderSet};
    use crate::plan::{
        FunctionShape, FunctionTemplateId, FunctionType, ModuleId, ValueShape, ValueType,
    };
    use crate::planner::{ExternalTypeProviderLinkReason, PlanError, UnsupportedFunctionReason};
    use ecow::EcoString;
    use num_bigint::BigInt;

    #[test]
    fn plan_host_program_bodyless_templates_with_module_qualified_ids() {
        let choose = |condition: bool, left: BigInt, right: BigInt| {
            if condition { left } else { right }
        };
        assert_eq!(
            choose(false, BigInt::from(10), BigInt::from(20)),
            BigInt::from(20),
        );
        assert_eq!(
            choose(true, BigInt::from(10), BigInt::from(20)),
            BigInt::from(10),
        );
        let all = |a: bool, b: bool, c: bool, d: bool, e: bool, f: bool, g: bool| {
            a && b && c && d && e && f && g
        };
        assert!(all(true, true, true, true, true, true, true));

        let hosts = HostProviderSet::new([HostModule::new("host_support", "host/math")
            .expect("host module should be valid")
            .with_function("add", <BigInt as std::ops::Add>::add)
            .expect("host function should be valid")
            .with_function("subtract", <BigInt as std::ops::Sub>::sub)
            .expect("host function should be valid")
            .with_function("ready", <bool as Default>::default)
            .expect("host function should be valid")
            .with_function("choose", choose)
            .expect("host function should be valid")
            .with_function("all", all)
            .expect("host function should be valid")
            .with_function(
                "consume",
                |_: BigInt,
                 _: f64,
                 _: EcoString,
                 _: crate::BitArrayValue,
                 _: char,
                 _: bool,
                 (): ()| (),
            )
            .expect("host function should be valid")])
        .expect("host modules should be unique");
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                ["host_support"],
                [ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
import host/math.{add}

pub fn main() {
  add(1, 2)
}
"#,
                )],
            )],
            hosts,
        )
        .expect("host program should compile");
        let plan = plan_host_program(typed).expect("host program should plan");

        assert_eq!(plan.root(), ModuleId::new(1));
        assert_eq!(
            plan.entry(),
            FunctionTemplateId::in_module(ModuleId::new(1), 0)
        );
        assert_eq!(
            plan.modules()
                .iter()
                .map(|module| (module.package().as_str(), module.module().as_str()))
                .collect::<Vec<_>>(),
            [("host_support", "host/math"), ("application", "main")],
        );
        assert_eq!(plan.modules()[0].id(), ModuleId::new(0));
        assert_eq!(plan.modules()[1].id(), ModuleId::new(1));
        assert!(plan.modules()[0].source_context().is_none());
        assert!(plan.modules()[1].source_context().is_some());
        let host = &plan.modules()[0];
        assert_eq!(host.id(), ModuleId::new(0));
        assert_eq!(host.functions().len(), 6);
        let functions = host
            .functions()
            .iter()
            .map(|function| {
                function
                    .host_template()
                    .expect("source-less module should retain host templates")
            })
            .collect::<Vec<_>>();
        assert_eq!(functions[0].name(), "add");
        assert_eq!(
            functions[0].id(),
            FunctionTemplateId::in_module(ModuleId::new(0), 0),
        );
        assert_eq!(functions[0].package(), "host_support");
        assert_eq!(functions[0].module(), "host/math");
        assert_eq!(functions[0].scheme().parameters(), &[]);
        assert_eq!(
            functions[0].signature().shape(),
            &FunctionShape::new(vec![ValueShape::Int, ValueShape::Int], ValueShape::Int,),
        );
        assert_eq!(
            functions[0].type_(),
            &FunctionType::new(vec![ValueType::Int, ValueType::Int], ValueType::Int),
        );
        assert!(matches!(
            functions[0].layout(),
            [HostParameter::Int(left), HostParameter::Int(right)]
                if left.index() == 0 && right.index() == 1
        ));
        assert_eq!(functions[1].name(), "subtract");
        assert_eq!(functions[2].name(), "ready");
        assert_eq!(
            functions[2].signature().shape(),
            &FunctionShape::new(Vec::new(), ValueShape::Bool),
        );
        assert_eq!(
            functions[2].type_(),
            &FunctionType::new(Vec::new(), ValueType::Bool),
        );
        assert_eq!(functions[3].name(), "choose");
        assert_eq!(
            functions[3].signature().shape(),
            &FunctionShape::new(
                vec![ValueShape::Bool, ValueShape::Int, ValueShape::Int],
                ValueShape::Int,
            ),
        );
        assert_eq!(
            functions[3].type_(),
            &FunctionType::new(
                vec![ValueType::Bool, ValueType::Int, ValueType::Int],
                ValueType::Int,
            ),
        );
        assert!(matches!(
            functions[3].layout(),
            [
                HostParameter::Bool(condition),
                HostParameter::Int(left),
                HostParameter::Int(right),
            ] if condition.index() == 0 && left.index() == 0 && right.index() == 1
        ));
        assert_eq!(functions[4].name(), "all");
        assert_eq!(
            functions[4].signature().shape(),
            &FunctionShape::new(vec![ValueShape::Bool; 7], ValueShape::Bool),
        );
        assert_eq!(
            functions[4].type_(),
            &FunctionType::new(vec![ValueType::Bool; 7], ValueType::Bool),
        );
        assert!(matches!(
            functions[4].layout(),
            [
                HostParameter::Bool(first),
                HostParameter::Bool(second),
                HostParameter::Bool(third),
                HostParameter::Bool(fourth),
                HostParameter::Bool(fifth),
                HostParameter::Bool(sixth),
                HostParameter::Bool(seventh),
            ] if first.index() == 0
                && second.index() == 1
                && third.index() == 2
                && fourth.index() == 3
                && fifth.index() == 4
                && sixth.index() == 5
                && seventh.index() == 6
        ));
        assert_eq!(functions[5].name(), "consume");
        assert_eq!(
            functions[5].signature().shape(),
            &FunctionShape::new(
                vec![
                    ValueShape::Int,
                    ValueShape::Float,
                    ValueShape::String,
                    ValueShape::BitArray,
                    ValueShape::UtfCodepoint,
                    ValueShape::Bool,
                    ValueShape::Nil,
                ],
                ValueShape::Nil,
            ),
        );
        assert_eq!(
            functions[5].type_(),
            &FunctionType::new(
                vec![
                    ValueType::Int,
                    ValueType::Float,
                    ValueType::String,
                    ValueType::BitArray,
                    ValueType::UtfCodepoint,
                    ValueType::Bool,
                    ValueType::Nil,
                ],
                ValueType::Nil,
            ),
        );
        assert!(matches!(
            functions[5].layout(),
            [
                HostParameter::Int(int),
                HostParameter::Float(float),
                HostParameter::String(string),
                HostParameter::BitArray(bit_array),
                HostParameter::UtfCodepoint(utf_codepoint),
                HostParameter::Bool(bool_),
                HostParameter::Nil(nil),
            ] if int.index() == 0
                && float.index() == 0
                && string.index() == 0
                && bit_array.index() == 0
                && utf_codepoint.index() == 0
                && bool_.index() == 0
                && nil.index() == 0
        ));
        let source = plan.modules()[1].functions()[0]
            .gleam_body()
            .expect("root module should retain its source function");
        assert_eq!(source.name(), "main");
    }

    #[test]
    fn plan_host_program_source_dependencies_as_dependency_modules() {
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
                        "pub fn main() { 1 }",
                    )],
                ),
                PackageSource::new(
                    "library",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new(
                        "support",
                        "support.gleam",
                        "pub fn unused() { 2 }",
                    )],
                ),
            ],
            HostProviderSet::new(Vec::<HostModule>::new())
                .expect("empty host modules should be valid"),
        )
        .expect("hosted source program should compile");
        let plan = plan_host_program(typed).expect("hosted source program should plan");

        assert_eq!(plan.root(), ModuleId::new(1));
        assert_eq!(
            plan.modules()
                .iter()
                .map(|module| (module.package().as_str(), module.module().as_str()))
                .collect::<Vec<_>>(),
            [("library", "support"), ("application", "main")],
        );
        assert_eq!(
            plan.modules()[0].functions()[0]
                .gleam_body()
                .expect("dependency should remain a source function")
                .name(),
            "unused",
        );
    }

    #[test]
    fn reject_profile_host_program_source_owner_boundaries() {
        let cases = [
            (
                "pub fn other() { 1 }",
                PlanError::UnsupportedFunction {
                    name: "main".into(),
                    reason: UnsupportedFunctionReason::MissingMain,
                },
            ),
            (
                r#"
@external(erlang, "external", "thing")
pub type Thing

pub fn main() { 1 }
"#,
                PlanError::ExternalTypeProviderLink {
                    package: "application".into(),
                    module: "main".into(),
                    type_: "Thing".into(),
                    reason: Box::new(ExternalTypeProviderLinkReason::MissingRegistration),
                },
            ),
            (
                r#"
const unsupported = <<1:native>>

pub fn main() { 1 }
"#,
                PlanError::UnsupportedBitArraySegment {
                    reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
                },
            ),
            (
                r#"
fn unsupported() { <<1:native>> }

pub fn main() { 1 }
"#,
                PlanError::UnsupportedBitArraySegment {
                    reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
                },
            ),
        ];

        for (source, expected) in cases {
            let typed = compile_typed_host_program(
                "application",
                "main",
                [PackageSource::new(
                    "application",
                    Vec::<EcoString>::new(),
                    [ModuleSource::new("main", "main.gleam", source)],
                )],
                HostProviderSet::new(Vec::<HostModule>::new())
                    .expect("empty host modules should be valid"),
            )
            .expect("profile-out source should still compile");
            assert_eq!(plan_host_program(typed).err(), Some(expected));
        }
    }
}
