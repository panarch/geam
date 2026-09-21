mod flow;

use super::body::PlannedHostedProgram;
use crate::host::RegisteredHostCallable;
use crate::plan::instantiate_native_callable as instantiate;
use crate::plan::{ConstantTemplates, HostedPlannedModule, HostedPlannedModuleParts, ModuleId};
use crate::planner::error::{HostProviderLinkReason, PlanError};
use crate::planner::module::registry::ProgramRegistry;
use std::collections::HashMap;

pub(super) fn attach(
    mut planned: PlannedHostedProgram,
    callables: Vec<RegisteredHostCallable>,
    registry: &ProgramRegistry,
) -> Result<PlannedHostedProgram, PlanError> {
    let mut definitions = HashMap::new();
    for callable in callables {
        let (schema, constructions, implementation) = callable.function.into_parts();
        let index = match planned.modules.iter().position(|module| {
            module.package() == &callable.identity.package
                && module.module() == &callable.identity.module
        }) {
            Some(index) => index,
            None => {
                let index = planned.modules.len();
                let id = ModuleId::new(index);
                planned
                    .modules
                    .push(HostedPlannedModule::new(HostedPlannedModuleParts {
                        id,
                        package: callable.identity.package.clone(),
                        module: callable.identity.module.clone(),
                        source_context: None,
                        custom_types: Vec::new(),
                        external_types: Vec::new(),
                        constants: ConstantTemplates::from_module_entries(id, Vec::new()),
                        functions: Vec::new(),
                        anonymous_functions: Vec::new(),
                        native_callables: Vec::new(),
                    }));
                index
            }
        };
        let module = &mut planned.modules[index];
        let template = module.push_native_callable(schema).clone();
        super::link::validate_host_schemas(
            registry,
            module.source_context(),
            &template,
            &constructions,
        )?;
        planned
            .implementations
            .push((template.id(), constructions, implementation));
        definitions.insert(callable.identity, (template, callable.completion));
    }
    let constructions = planned
        .implementations
        .iter()
        .map(|(id, constructions, _)| (*id, constructions))
        .collect::<HashMap<_, _>>();
    for template in planned
        .modules
        .iter_mut()
        .flat_map(HostedPlannedModule::host_templates_mut)
    {
        let mut targets = Vec::new();
        for construction in constructions[&template.id()].callables() {
            let failure = |reason| PlanError::HostProviderLink {
                package: template.package().clone(),
                module: template.module().into(),
                function: template.name().into(),
                reason: Box::new(reason),
            };
            let Some((definition, completion)) = definitions.get(&construction.identity) else {
                return Err(failure(HostProviderLinkReason::MissingCallable {
                    package: construction.identity.package.clone(),
                    module: construction.identity.module.clone(),
                    function: construction.identity.name.clone(),
                }));
            };
            let target = (completion == &construction.completion)
                .then(|| instantiate(definition, construction))
                .flatten()
                .ok_or_else(|| {
                    failure(HostProviderLinkReason::CallableContractMismatch {
                        package: construction.identity.package.clone(),
                        module: construction.identity.module.clone(),
                        function: construction.identity.name.clone(),
                    })
                })?;
            targets.push(target);
        }
        template.bind_callable_constructions(targets.into_boxed_slice());
    }
    let templates = planned
        .modules
        .iter_mut()
        .flat_map(HostedPlannedModule::host_templates_mut)
        .map(|template| &*template)
        .collect::<Vec<_>>();
    flow::validate(&templates)?;
    Ok(planned)
}

#[cfg(test)]
mod tests {
    use crate::embedding::HostPreparation;
    use crate::planner::{HostProviderLinkReason, PlanError};
    use crate::{
        HostCallableSchema, HostCreatedFunction, HostDeclarations, HostFunctionDeclaration,
        HostFunctionType, HostListType, HostProviderModuleDeclaration, HostReturns, HostType,
        HostTypeList, HostTypeListEnd, HostTypeParameter, ModuleSource, PackageSource,
    };
    use std::marker::PhantomData;

    type End = HostTypeListEnd;
    type One<T> = HostTypeList<T, End>;
    type T = HostTypeParameter<0>;
    struct Growing<Type>(PhantomData<Type>);
    struct Stable<Type>(PhantomData<Type>);
    struct WrongCaptures<Type>(PhantomData<Type>);

    macro_rules! definition {
        ($schema:ident, $captures:ty, $constructions:ty) => {
            impl<Type: HostType> HostCallableSchema for $schema<Type> {
                const PACKAGE: &'static str = "application";
                const MODULE: &'static str = "library";
                const NAME: &'static str = "constant";
                type Arguments = End;
                type Return = Type;
                type Captures = $captures;
                type Constructions = $constructions;
                type Completion = HostReturns;
            }
        };
    }
    definition!(
        Growing,
        One<Type>,
        One<HostCreatedFunction<Growing<HostListType<Type>>>>
    );
    definition!(Stable, One<Type>, One<HostCreatedFunction<Stable<Type>>>);
    definition!(WrongCaptures, One<HostListType<Type>>, End);

    struct GrowingTuple<Type>(PhantomData<Type>);
    struct GrowingFunction<Type>(PhantomData<Type>);
    struct GrowingCustom<Type>(PhantomData<Type>);
    struct GrowingExternal<Type>(PhantomData<Type>);
    struct Parcel;
    impl crate::HostExternalSchema for Parcel {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "Parcel";
        const PARAMETER_COUNT: usize = 1;
    }
    definition!(
        GrowingTuple,
        One<Type>,
        One<HostCreatedFunction<GrowingTuple<crate::HostTupleType<One<Type>>>>>
    );
    definition!(
        GrowingFunction,
        One<Type>,
        One<HostCreatedFunction<GrowingFunction<HostFunctionType<One<Type>, Type>>>>
    );
    definition!(
        GrowingCustom,
        One<Type>,
        One<HostCreatedFunction<GrowingCustom<crate::provider::ProviderResult<Type, bool>>>>
    );
    definition!(
        GrowingExternal,
        One<Type>,
        One<HostCreatedFunction<GrowingExternal<crate::HostExternalType<Parcel, One<Type>>>>>
    );

    #[test]
    fn reject_profile_expanding_callable_cycles_through_each_compound_type() {
        let declarations = || {
            HostDeclarations::from_providers([HostProviderModuleDeclaration::new(
                "application",
                "library",
            )
            .unwrap()
            .with_external_type::<Parcel>()
            .unwrap()])
            .unwrap()
        };
        for declarations in [
            declarations().with_callable::<GrowingTuple<T>>().unwrap(),
            declarations()
                .with_callable::<GrowingFunction<T>>()
                .unwrap(),
            declarations().with_callable::<GrowingCustom<T>>().unwrap(),
            declarations()
                .with_callable::<GrowingExternal<T>>()
                .unwrap(),
        ] {
            let program = crate::compile_declared_host_program(
                "application",
                "library",
                [PackageSource::new(
                    "application",
                    Vec::<&str>::new(),
                    [ModuleSource::new(
                        "library",
                        "library.gleam",
                        "pub type Parcel(a)\npub fn run() { 42 }",
                    )],
                )],
                declarations,
            )
            .unwrap();
            assert_eq!(
                HostPreparation::new(program).err(),
                Some(PlanError::HostProviderLink {
                    package: "application".into(),
                    module: "library".into(),
                    function: "constant".into(),
                    reason: Box::new(HostProviderLinkReason::ExpandingCallableCycle {
                        package: "application".into(),
                        module: "library".into(),
                        function: "constant".into(),
                    }),
                })
            );
        }
    }

    fn declarations() -> HostDeclarations {
        type Factory = HostFunctionDeclaration<
            (T,),
            HostFunctionType<End, T>,
            One<HostCreatedFunction<Stable<T>>>,
        >;
        const FACTORY: Factory = HostFunctionDeclaration::new("make");
        HostDeclarations::from_providers([HostProviderModuleDeclaration::new(
            "application",
            "library",
        )
        .unwrap()
        .with_function(FACTORY)
        .unwrap()])
        .unwrap()
    }

    #[test]
    fn private_callable_schemas_are_linked_even_without_a_public_factory() {
        struct Missing;
        impl crate::HostCustomSchema for Missing {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "library";
            const NAME: &'static str = "Missing";
            const PARAMETER_COUNT: usize = 0;
            type Constructors = crate::HostCustomConstructorListEnd;
        }
        let typed = crate::compile_declared_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    "pub fn run() { 42 }",
                )],
            )],
            HostDeclarations::new([])
                .unwrap()
                .with_callable::<Stable<crate::HostCustomType<Missing>>>()
                .unwrap(),
        )
        .unwrap();
        assert_eq!(
            HostPreparation::new(typed).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "library".into(),
                function: "constant".into(),
                reason: Box::new(HostProviderLinkReason::MissingCustomType {
                    custom_type: crate::plan::CustomTypeName::new(
                        "application".into(),
                        "library".into(),
                        "Missing".into()
                    ),
                }),
            })
        );
    }

    fn plan(declarations: HostDeclarations) -> Result<HostPreparation, PlanError> {
        let typed = crate::compile_declared_host_program(
            "application",
            "library",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new(
                    "library",
                    "library.gleam",
                    r#"
    @external(erlang, "ffi", "make")
    fn make(value: a) -> fn() -> a
    pub fn run() { make(42)() }
    "#,
                )],
            )],
            declarations,
        )
        .unwrap();
        HostPreparation::new(typed)
    }

    #[test]
    fn missing_native_body_and_incompatible_captures_fail_during_planning() {
        assert_eq!(
            plan(declarations()).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "library".into(),
                function: "make".into(),
                reason: Box::new(HostProviderLinkReason::MissingCallable {
                    package: "application".into(),
                    module: "library".into(),
                    function: "constant".into(),
                }),
            })
        );
        assert_eq!(
            plan(declarations().with_callable::<WrongCaptures<T>>().unwrap()).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "library".into(),
                function: "make".into(),
                reason: Box::new(HostProviderLinkReason::CallableContractMismatch {
                    package: "application".into(),
                    module: "library".into(),
                    function: "constant".into(),
                }),
            })
        );
    }

    #[test]
    fn generic_construction_cycles_may_reuse_types_but_cannot_expand_them_forever() {
        let stable = plan(declarations().with_callable::<Stable<T>>().unwrap()).unwrap();
        stable
            .function(crate::embedding::FunctionDeclaration::<
                (),
                num_bigint::BigInt,
            >::new("run"))
            .unwrap()
            .prepare()
            .unwrap();
        assert_eq!(
            plan(declarations().with_callable::<Growing<T>>().unwrap()).err(),
            Some(PlanError::HostProviderLink {
                package: "application".into(),
                module: "library".into(),
                function: "constant".into(),
                reason: Box::new(HostProviderLinkReason::ExpandingCallableCycle {
                    package: "application".into(),
                    module: "library".into(),
                    function: "constant".into(),
                }),
            })
        );
    }

    struct Swap<First, Second>(PhantomData<(First, Second)>);
    impl<First: HostType, Second: HostType> HostCallableSchema for Swap<First, Second> {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "swap";
        type Arguments = End;
        type Return = First;
        type Captures = HostTypeList<First, One<Second>>;
        type Constructions = One<HostCreatedFunction<Swap<Second, First>>>;
        type Completion = HostReturns;
    }

    #[test]
    fn plan_callable_capture_only_parameters_permute_without_expanding_specializations() {
        use crate::plan::{TypeParameterId, ValueShape};
        type U = HostTypeParameter<1>;
        let declared = || {
            crate::compile_declared_host_program(
                "application",
                "library",
                [PackageSource::new(
                    "application",
                    Vec::<&str>::new(),
                    [ModuleSource::new(
                        "library",
                        "library.gleam",
                        "pub fn run() { 42 }",
                    )],
                )],
                HostDeclarations::from_providers([])
                    .unwrap()
                    .with_callable::<Swap<T, U>>()
                    .unwrap(),
            )
            .unwrap()
        };
        let plan = crate::planner::plan_declared_library_program(declared())
            .unwrap()
            .into_parts();
        assert_eq!(plan.modules.len(), 1);
        let native = plan.modules[0].native_callables();
        assert_eq!(native.len(), 1);
        assert_eq!(native[0].name(), "swap");
        let constructions = native[0].callable_constructions();
        assert_eq!(constructions.len(), 1);
        assert_eq!(constructions[0].template(), native[0].id());
        assert_eq!(
            constructions[0].substitution().arguments(),
            &[
                ValueShape::Parameter(TypeParameterId(1)),
                ValueShape::Parameter(TypeParameterId(0)),
            ]
        );

        let mut preparation = HostPreparation::new(declared())
            .unwrap()
            .function(crate::embedding::FunctionDeclaration::<
                (),
                num_bigint::BigInt,
            >::new("run"))
            .unwrap();
        preparation
            .callable::<Swap<num_bigint::BigInt, bool>>()
            .unwrap();
        // The reachable Int/Bool and Bool/Int instances form a finite closed set.
        preparation.prepare().unwrap();
    }

    struct Expand<Type>(PhantomData<Type>);
    struct Reset<Type>(PhantomData<Type>);
    impl<Type: HostType> HostCallableSchema for Expand<Type> {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "expand";
        type Arguments = End;
        type Return = Type;
        type Captures = One<Type>;
        type Constructions =
            HostTypeList<HostListType<Type>, One<HostCreatedFunction<Reset<HostListType<Type>>>>>;
        type Completion = HostReturns;
    }
    impl<Type: HostType> HostCallableSchema for Reset<Type> {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "library";
        const NAME: &'static str = "reset";
        type Arguments = End;
        type Return = Type;
        type Captures = One<Type>;
        type Constructions = One<HostCreatedFunction<Expand<num_bigint::BigInt>>>;
        type Completion = HostReturns;
    }

    #[test]
    fn plan_callable_construction_cycles_can_reset_a_previously_expanded_type() {
        use crate::plan::{TypeParameterId, ValueShape};
        let declared = || {
            crate::compile_declared_host_program(
                "application",
                "library",
                [PackageSource::new(
                    "application",
                    Vec::<&str>::new(),
                    [ModuleSource::new(
                        "library",
                        "library.gleam",
                        "pub fn run() { 42 }",
                    )],
                )],
                HostDeclarations::from_providers([])
                    .unwrap()
                    .with_callable::<Expand<T>>()
                    .unwrap()
                    .with_callable::<Reset<T>>()
                    .unwrap(),
            )
            .unwrap()
        };
        let plan = crate::planner::plan_declared_library_program(declared())
            .unwrap()
            .into_parts();
        let native = plan.modules[0].native_callables();
        assert_eq!(
            native
                .iter()
                .map(|template| template.name())
                .collect::<Vec<_>>(),
            ["expand", "reset"]
        );
        let expand = &native[0].callable_constructions()[0];
        let reset = &native[1].callable_constructions()[0];
        assert_eq!(expand.template(), native[1].id());
        assert_eq!(
            expand.substitution().arguments(),
            &[ValueShape::List(Box::new(ValueShape::Parameter(
                TypeParameterId(0)
            ))),]
        );
        assert_eq!(reset.template(), native[0].id());
        assert_eq!(reset.substitution().arguments(), &[ValueShape::Int]);

        let mut preparation = HostPreparation::new(declared())
            .unwrap()
            .function(crate::embedding::FunctionDeclaration::<
                (),
                num_bigint::BigInt,
            >::new("run"))
            .unwrap();
        preparation.callable::<Expand<bool>>().unwrap();
        // Bool -> List(Bool) -> Int -> List(Int) -> Int terminates specialization.
        preparation.prepare().unwrap();
    }
}
