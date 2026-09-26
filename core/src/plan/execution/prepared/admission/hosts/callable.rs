use super::{ContractError, NativeError, NativeFunctions};
use crate::host::{HostFunctionBinding, HostProfile};
use crate::plan::execution::function::HostedExecutionGraph;
use crate::plan::execution::host::{HostCallableEntry, HostedFunctionMetadata};
use crate::plan::execution::prepared::admission::call::Target;
use crate::plan::execution::prepared::admission::instruction::Instructions;
use crate::plan::execution::type_::TypeMetadata;
use std::collections::HashMap;

pub(super) fn library<Profile: HostProfile>(
    hosts: &NativeFunctions<'_, Profile>,
    entries: &[crate::plan::execution::LibraryNativeConstruction],
    catalog: &super::super::catalog::Catalog<'_>,
    types: &super::super::type_::Types<'_>,
) -> Result<(), NativeError> {
    for entry in entries {
        let fail = || NativeError::Call(ContractError::Callable);
        let construction = &entry.construction;
        let target = construction
            .target
            .resolve(catalog, types)
            .map_err(|_| fail())?;
        let id = HostCallableEntry {
            family: target.family,
            index: target.index,
        };
        let (returns_value, index) = hosts
            .callable_bindings
            .borrow()
            .get(&id)
            .copied()
            .ok_or_else(fail)?;
        let metadata = if returns_value {
            hosts.values[index].0
        } else {
            hosts.nevers[index].0
        };
        let declaration = &entry.declaration;
        if declaration.package.as_ref() != metadata.package()
            || declaration.module.as_ref() != metadata.module()
            || declaration.name.as_ref() != metadata.name()
            || declaration.returns_value != metadata.completion.declared_value()
            || metadata.type_ != construction.type_
            || entry.invocation.type_ != construction.type_
            || construction.parameters.len() != target.parameters.len()
            || construction.captures.as_ref() != target.captures
        {
            return Err(fail());
        }
        for (slot, (local, shape)) in construction
            .parameters
            .iter()
            .zip(target.parameters.iter().zip(target.parameter_shapes))
        {
            types.slot(slot).map_err(|_| fail())?;
            if slot.local() != local || slot.shape() != *shape {
                return Err(fail());
            }
        }
        super::super::callables::admit(std::slice::from_ref(&entry.invocation), types)
            .map_err(|_| fail())?;
        super::super::input::admit(&entry.captures, types).map_err(|_| fail())?;
    }
    Ok(())
}

pub(super) fn admit<Profile: HostProfile>(
    hosts: &NativeFunctions<'_, Profile>,
    context: &Instructions<'_, '_, HostedExecutionGraph>,
) -> Result<(), NativeError> {
    let metadata = hosts
        .values
        .iter()
        .enumerate()
        .map(|(index, (metadata, _, registration))| (true, index, *metadata, *registration))
        .chain(
            hosts
                .nevers
                .iter()
                .enumerate()
                .map(|(index, (metadata, _, registration))| {
                    (false, index, *metadata, *registration)
                }),
        );
    let mut definitions = HashMap::new();
    for (value, index, metadata, _) in metadata.clone() {
        if let Some(entry) = metadata.callable_entry
            && (hosts.callable_bindings.borrow().get(&entry).copied() != Some((value, index))
                || definitions.insert(entry, metadata).is_some())
        {
            return Err(failure(value, index));
        }
    }
    for (value, index, metadata, registration) in metadata {
        let registered = &hosts.registrations[registration].constructions;
        if metadata.constructions.callables.len() != registered.callables().len() {
            return Err(failure(value, index));
        }
        for (construction, expected) in metadata
            .constructions
            .callables
            .iter()
            .zip(registered.callables())
        {
            for type_ in construction
                .type_
                .arguments
                .iter()
                .chain([construction.type_.return_.as_ref()])
            {
                context
                    .types
                    .value(type_)
                    .map_err(|reason| NativeError::Contract {
                        value,
                        index,
                        reason: ContractError::Type(reason),
                    })?;
            }
            for slot in construction
                .parameters
                .iter()
                .chain(construction.captures.iter())
            {
                context
                    .types
                    .slot(slot)
                    .map_err(|reason| NativeError::Contract {
                        value,
                        index,
                        reason: ContractError::Type(reason),
                    })?;
            }
            let target = construction
                .target
                .resolve(context.catalog, context.types)
                .map_err(|_| failure(value, index))?;
            let entry = HostCallableEntry {
                family: target.family,
                index: target.index,
            };
            let Some(body) = definitions.get(&entry) else {
                return Err(failure(value, index));
            };
            if body.package() != expected.identity.package.as_str()
                || body.module() != expected.identity.module.as_str()
                || body.name() != expected.identity.name.as_str()
                || body.completion.declared_value()
                    != matches!(expected.completion, HostFunctionBinding::Value(()))
                || construction.type_ != body.type_
                || construction.parameters.len() != target.parameters.len()
                || construction.captures.as_ref() != target.captures
                || construction.captures.len() != expected.captures.len()
                || construction.type_.arguments.len() != expected.arguments.len()
            {
                return Err(failure(value, index));
            }
            for (slot, (local, shape)) in construction
                .parameters
                .iter()
                .zip(target.parameters.iter().zip(target.parameter_shapes))
            {
                if slot.local() != local || slot.shape() != *shape {
                    return Err(failure(value, index));
                }
            }
            for (expected, actual) in expected
                .arguments
                .iter()
                .zip(construction.type_.arguments.iter())
                .chain([(&expected.return_, construction.type_.return_.as_ref())])
            {
                let expected = resolve(metadata, expected);
                if !context.types.metadata_matches_value(&expected, actual) {
                    return Err(failure(value, index));
                }
            }
            for (expected, actual) in expected.captures.iter().zip(construction.captures.iter()) {
                let expected = resolve(metadata, expected);
                let actual = &context.types.shape_types()[actual.shape().index()];
                if !context.types.metadata_matches_value(&expected, actual) {
                    return Err(failure(value, index));
                }
            }
        }
    }
    Ok(())
}

fn resolve(
    metadata: &HostedFunctionMetadata,
    descriptor: &crate::host::HostTypeDescriptor,
) -> TypeMetadata {
    TypeMetadata::from_public(
        &descriptor.resolve_sealed(&|index| metadata.type_arguments[index].type_.materialize()),
    )
}

fn failure(value: bool, index: usize) -> NativeError {
    NativeError::Contract {
        value,
        index,
        reason: ContractError::Callable,
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::type_::TypeError;
    use super::NativeFunctions;
    use crate::plan::execution::function::{IntFunctionId, ValueFunctionEntry};
    use crate::plan::execution::graph::{IntLocalId, ParamLocal};
    use crate::plan::execution::prepared::admission::tests::{owned, owned_mut};
    use crate::plan::execution::prepared::admission::{
        catalog::Catalog, functions, functions::Hosts, instruction::Instructions, source::Sources,
        type_::Types,
    };
    use crate::plan::execution::type_::{CustomTypeId, TypeMetadata, ValueShapeId, ValueType};
    use crate::{
        HostCall, HostCallCompletion, HostCallError, HostCallableSchema, HostCaptures,
        HostConstructions, HostCreatedFunction, HostFunctionDeclaration, HostFunctionType,
        HostProvider, HostProviderModule, HostProviderSet, HostReturns, HostTypeIndex0,
        HostTypeList, HostTypeListEnd, StatelessHostProfile,
    };
    use crate::{HostType, HostTypeParameter, HostValue};
    use num_bigint::BigInt;
    use std::marker::PhantomData;

    #[test]
    fn library_factories_admit_value_and_diverging_bodies_with_exact_invocation_and_capture_layouts()
     {
        use crate::plan::execution::function::{CoreRuntimeFunctionId, RuntimeFunctionId};
        use crate::plan::execution::type_::{IntListTypeId, ListTypeId};
        use crate::plan::{
            FunctionType, LibraryCallableSignature, LibraryEntry, LibraryNativeSignature,
            LibraryValueType, ValueType as Public,
        };
        type End = HostTypeListEnd;
        type One<T> = HostTypeList<T, End>;
        struct Add;
        impl HostCallableSchema for Add {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "private/bodies";
            const NAME: &'static str = "add";
            type Arguments = One<BigInt>;
            type Return = BigInt;
            type Captures = One<bool>;
            type Constructions = End;
            type Completion = HostReturns;
        }
        struct Stop<Return>(PhantomData<Return>);
        impl<Return: HostType> HostCallableSchema for Stop<Return> {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "private/bodies";
            const NAME: &'static str = "stop";
            type Arguments = End;
            type Return = Return;
            type Captures = End;
            type Constructions = End;
            type Completion = crate::HostDiverges;
        }
        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        fn add<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
            captures: HostCaptures<'call, One<bool>>,
            _: HostConstructions<'call, End>,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            assert_eq!(call.captures(captures), (true, ()));
            Ok(call.return_value(value + 2))
        }
        fn stop<'call>(
            _: HostCall<'call, StatelessHostProfile, Provider, HostTypeParameter<0>>,
            _: HostCaptures<'call, End>,
            _: HostConstructions<'call, End>,
        ) -> Result<std::convert::Infallible, HostCallError> {
            Err(crate::HostFailure::new("library stop").into())
        }
        type StopFunction = HostFunctionType<End, BigInt>;
        type StopPermission = One<HostCreatedFunction<Stop<BigInt>>>;
        fn make_stop<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, StopFunction>,
            constructions: HostConstructions<'call, StopPermission>,
        ) -> Result<HostCallCompletion<'call, StopFunction>, HostCallError> {
            let callback = call.construct_function(constructions.at::<HostTypeIndex0>(), ());
            Ok(call.return_value(callback))
        }
        let hosts = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_declared_function::<Provider, (), StopFunction, StopPermission, _>(
                    HostFunctionDeclaration::new("make_stop"),
                    make_stop,
                )
                .unwrap()])
            .unwrap()
            .with_callable::<Provider, Add, (BigInt,), _>(add)
            .unwrap()
            .with_callable::<Provider, Stop<HostTypeParameter<0>>, (), _>(stop)
            .unwrap()
        };
        let packages = || {
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new(
                    "main",
                    "main.gleam",
                    "@external(erlang, \"native\", \"make_stop\") fn make_stop() -> fn() -> Int\npub fn main() { let _stop = make_stop() 42 }",
                )],
            )]
        };
        let compile =
            || crate::compile_typed_host_program("app", "main", packages(), hosts()).unwrap();
        let (mut bindings, main) = crate::embedding::HostedModuleBuilder::new(compile())
            .unwrap()
            .function(crate::embedding::FunctionDeclaration::<(), BigInt>::new(
                "main",
            ))
            .unwrap();
        let addition = bindings.callable::<Add>().unwrap();
        let failure = bindings.callable::<Stop<BigInt>>().unwrap();
        let mut module = bindings.seal().unwrap();
        let host = crate::execution_fixture::TestHost::default();
        let mut echo = Vec::new();
        host.block_on(
            module.with_execution(&host, &mut (), &mut echo, async |scope| {
                assert_eq!(scope.call(&main, ()).await.unwrap(), BigInt::from(42));
                let callback = scope.construct(&addition, (true, ())).unwrap();
                assert_eq!(
                    scope.invoke(&callback, (BigInt::from(40),)).await.unwrap(),
                    BigInt::from(42)
                );
                let callback = scope.construct(&failure, ()).unwrap();
                assert_eq!(
                    scope.invoke(&callback, ()).await.unwrap_err().to_string(),
                    "host function app::private/bodies.stop failed: library stop"
                );
            }),
        )
        .unwrap();
        assert!(echo.is_empty());
        let (_, _, declarations, _) = hosts().into_declarations().into_registered();
        assert_eq!(declarations.len(), 2);
        assert_eq!(declarations[0].identity.name, "add");
        assert_eq!(declarations[1].identity.name, "stop");
        assert_eq!(
            declarations[1].completion,
            crate::host::HostFunctionBinding::Never(())
        );

        let mut plan = crate::planner::plan_host_library_program(compile()).unwrap();
        for (declaration, arguments, captures) in [
            (
                crate::host::RegisteredCallableConstruction::of::<Add>(),
                vec![Public::Int],
                vec![Public::Bool],
            ),
            (
                crate::host::RegisteredCallableConstruction::of::<Stop<BigInt>>(),
                vec![],
                vec![],
            ),
        ] {
            plan.callable(
                declaration,
                LibraryNativeSignature {
                    invocation: LibraryCallableSignature {
                        type_: FunctionType::new(arguments, Public::Int),
                        input_variants: vec![],
                        input_lists: vec![],
                        callables: vec![],
                    },
                    captures,
                    capture_variants: vec![],
                    capture_lists: vec![],
                },
            )
            .unwrap();
        }
        let main = plan
            .functions()
            .iter()
            .find(|function| function.name() == "main")
            .unwrap()
            .signature()
            .id();
        let (program, functions, _, entries) =
            crate::plan::execution::lowering::lower_hosted_library(
                plan,
                LibraryEntry::new(main, LibraryValueType::Int, vec![], vec![]),
                vec![],
            )
            .unwrap();
        let (values, nevers) = functions.into_metadata();
        let values = values
            .into_vec()
            .into_iter()
            .map(|value| std::sync::Arc::try_unwrap(value).ok().unwrap())
            .collect::<Vec<_>>();
        let nevers = nevers
            .into_vec()
            .into_iter()
            .map(|value| std::sync::Arc::try_unwrap(value).ok().unwrap())
            .collect::<Vec<_>>();
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let linked = NativeFunctions::new(&values, &nevers, hosts()).unwrap();
        linked.tables(&context).unwrap();
        functions::all(&program.functions, &context, &linked).unwrap();
        assert_eq!(linked.callables(&context), Ok(()));
        assert_eq!(super::library(&linked, &entries, &catalog, &types), Ok(()));
        let bindings = linked.callable_bindings.replace(Default::default());
        assert_eq!(
            super::library(&linked, &entries, &catalog, &types),
            Err(super::NativeError::Call(super::ContractError::Callable))
        );
        linked.callable_bindings.replace(bindings);
        enum Change {
            Target,
            Name,
            ParameterShape,
            ParameterLocal,
            InvocationInputs,
            CaptureInputs,
        }
        for change in [
            Change::Target,
            Change::Name,
            Change::ParameterShape,
            Change::ParameterLocal,
            Change::InvocationInputs,
            Change::CaptureInputs,
        ] {
            let mut changed = entries[0].clone();
            match change {
                Change::Target => {
                    changed.construction.target =
                        RuntimeFunctionId::Core(CoreRuntimeFunctionId::Int(IntFunctionId(999)))
                }
                Change::Name => changed.declaration.name = "other".into(),
                Change::ParameterShape => {
                    let mut parameters = changed.construction.parameters.to_vec();
                    parameters[0].shape = ValueShapeId(999);
                    changed.construction.parameters = parameters.into();
                }
                Change::ParameterLocal => {
                    let mut parameters = changed.construction.parameters.to_vec();
                    parameters[0].local = ParamLocal::Int(IntLocalId(99));
                    changed.construction.parameters = parameters.into();
                }
                Change::InvocationInputs => {
                    changed.invocation.inputs.lists.ints =
                        vec![IntListTypeId::new(ListTypeId(999))].into()
                }
                Change::CaptureInputs => {
                    changed.captures.lists.ints = vec![IntListTypeId::new(ListTypeId(999))].into()
                }
            }
            assert_eq!(
                super::library(&linked, &[changed], &catalog, &types),
                Err(super::NativeError::Call(super::ContractError::Callable))
            );
        }
    }

    #[test]
    fn native_constructions_match_observed_bodies_and_concrete_argument_and_capture_types() {
        type End = HostTypeListEnd;
        type T = HostTypeParameter<0>;
        type U = HostTypeParameter<1>;
        type One<T> = HostTypeList<T, End>;
        type Thunk = HostFunctionType<One<T>, T>;
        type Permission = One<HostCreatedFunction<Constant<T, U>>>;
        struct Constant<Input, Capture>(PhantomData<(Input, Capture)>);
        impl<Input: HostType, Capture: HostType> HostCallableSchema for Constant<Input, Capture> {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "callbacks";
            const NAME: &'static str = "constant";
            type Arguments = One<Input>;
            type Return = Input;
            type Captures = One<Capture>;
            type Constructions = End;
            type Completion = HostReturns;
        }
        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        fn constant<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, T>,
            captures: HostCaptures<'call, One<U>>,
            _: HostConstructions<'call, End>,
            argument: HostValue<'call, T>,
        ) -> Result<HostCallCompletion<'call, T>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            let (_captured, ()) = call.captures(captures);
            Ok(call.return_value(argument))
        }
        fn make<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, Thunk>,
            constructions: HostConstructions<'call, Permission>,
            _argument_type: HostValue<'call, T>,
            captured: HostValue<'call, U>,
        ) -> Result<HostCallCompletion<'call, Thunk>, HostCallError> {
            let value =
                call.construct_function(constructions.at::<HostTypeIndex0>(), (captured, ()));
            Ok(call.return_value(value))
        }
        let hosts = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_declared_function::<Provider, (T, U), Thunk, Permission, _>(
                    HostFunctionDeclaration::new("make"),
                    make,
                )
                .unwrap()])
            .unwrap()
            .with_callable::<Provider, Constant<T, U>, (T,), _>(constant)
            .unwrap()
        };
        let source = r#"
@external(erlang, "native", "make") fn make(argument: a, captured: b) -> fn(a) -> a
pub fn main() {
  let first = make(0, True)(42)
  let second = make(False, 7)(True)
  let third = make(0, 7)(42)
  case first == 42 && second && third == 42 {
    True -> 42
    False -> panic as "specialization"
  }
}
"#;
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            hosts(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap(),
            crate::Value::Int(42.into())
        );

        #[derive(Clone, Copy)]
        enum ConstructionChange {
            MissingTarget,
            SourceTarget,
            InvalidArgumentType,
            InvalidReturnType,
            InvalidCaptureShape,
            ParameterLocal,
            ParameterCount,
            CaptureCount,
            ArgumentSpecialization,
            CaptureSpecialization,
        }
        #[derive(Clone, Copy)]
        enum Change {
            None,
            Construction(ConstructionChange),
        }
        for change in [
            Change::None,
            Change::Construction(ConstructionChange::MissingTarget),
            Change::Construction(ConstructionChange::SourceTarget),
            Change::Construction(ConstructionChange::InvalidArgumentType),
            Change::Construction(ConstructionChange::InvalidReturnType),
            Change::Construction(ConstructionChange::InvalidCaptureShape),
            Change::Construction(ConstructionChange::ParameterLocal),
            Change::Construction(ConstructionChange::ParameterCount),
            Change::Construction(ConstructionChange::CaptureCount),
            Change::Construction(ConstructionChange::ArgumentSpecialization),
            Change::Construction(ConstructionChange::CaptureSpecialization),
        ] {
            let (program, mut values, nevers) = super::super::tests::lowered(source, hosts());
            let factory = values
                .iter()
                .position(|metadata| {
                    metadata.name() == "make"
                        && metadata.signature.arguments.as_ref()
                            == [TypeMetadata::Int, TypeMetadata::Bool]
                })
                .unwrap();
            let expected = match change {
                Change::None => Ok(()),
                Change::Construction(change) => {
                    let mut constructions =
                        values[factory].constructions.callables.clone().into_vec();
                    let construction = &mut constructions[0];
                    let reason = match change {
                        ConstructionChange::MissingTarget => {
                            construction.target =
                                crate::plan::execution::function::RuntimeFunctionId::Core(
                                    crate::plan::execution::function::CoreRuntimeFunctionId::Int(
                                        IntFunctionId(999),
                                    ),
                                );
                            None
                        }
                        ConstructionChange::SourceTarget => {
                            construction.target = program.common.main.clone();
                            None
                        }
                        ConstructionChange::InvalidArgumentType => {
                            construction.type_.arguments =
                                vec![ValueType::Custom(CustomTypeId(999))].into();
                            Some(TypeError::MissingCustom { index: 999 })
                        }
                        ConstructionChange::InvalidReturnType => {
                            construction.type_.return_ =
                                Box::new(ValueType::Custom(CustomTypeId(999))).into();
                            Some(TypeError::MissingCustom { index: 999 })
                        }
                        ConstructionChange::InvalidCaptureShape => {
                            let mut captures = construction.captures.clone().into_vec();
                            captures[0].shape = ValueShapeId(999);
                            construction.captures = captures.into();
                            Some(TypeError::MissingShape { index: 999 })
                        }
                        ConstructionChange::ParameterLocal => {
                            let mut parameters = construction.parameters.clone().into_vec();
                            parameters[0].local = ParamLocal::Int(IntLocalId(99));
                            construction.parameters = parameters.into();
                            None
                        }
                        ConstructionChange::ParameterCount => {
                            construction.parameters = Vec::new().into();
                            None
                        }
                        ConstructionChange::CaptureCount => {
                            construction.captures = Vec::new().into();
                            None
                        }
                        ConstructionChange::ArgumentSpecialization
                        | ConstructionChange::CaptureSpecialization => {
                            let arguments =
                                if matches!(change, ConstructionChange::ArgumentSpecialization) {
                                    [TypeMetadata::Bool, TypeMetadata::Int]
                                } else {
                                    [TypeMetadata::Int, TypeMetadata::Int]
                                };
                            let replacement = values
                                .iter()
                                .find(|metadata| {
                                    metadata.name() == "make"
                                        && metadata.signature.arguments.as_ref() == arguments
                                })
                                .unwrap();
                            *construction = replacement.constructions.callables[0].clone();
                            None
                        }
                    };
                    values[factory].constructions.callables = constructions.into();
                    Err(super::NativeError::Contract {
                        value: true,
                        index: factory,
                        reason: reason
                            .map_or(super::ContractError::Callable, super::ContractError::Type),
                    })
                }
            };
            let common = &program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
            let sources = Sources::admit(common.root, &common.modules).unwrap();
            let context = Instructions {
                types: &types,
                catalog: &catalog,
                sources: &sources,
                constants: &common.constants,
            };
            let linked = NativeFunctions::new(&values, &nevers, hosts()).unwrap();
            assert!(linked.callable_bindings.borrow().is_empty());
            functions::all(&program.functions, &context, &linked).unwrap();
            assert_eq!(linked.callables(&context), expected);
            let common = std::sync::Arc::try_unwrap(program.common).ok().unwrap();
            let artifact = crate::plan::execution::prepared::ProgramTables {
                root: common.root,
                modules: common.modules,
                main: common.main,
                functions: *super::super::super::tests::owned(program.functions),
                constants: *super::super::super::tests::owned(common.constants),
                function_parameters: std::sync::Arc::try_unwrap(common.function_parameters)
                    .ok()
                    .unwrap(),
                list_types: std::sync::Arc::try_unwrap(common.list_types).ok().unwrap(),
                custom_types: std::sync::Arc::try_unwrap(common.custom_types)
                    .ok()
                    .unwrap(),
                external_types: std::sync::Arc::try_unwrap(common.external_types)
                    .ok()
                    .unwrap(),
                value_shapes: *super::super::super::tests::owned(common.value_shapes),
            };
            assert_eq!(
                super::super::super::program(&artifact, &linked).map(|_| ()),
                expected.map_err(super::super::super::Error::Hosts)
            );
        }
    }
    #[test]
    fn private_bodies_must_be_observed_by_the_function_walk_before_constructions_are_admitted() {
        type End = HostTypeListEnd;
        type Thunk = HostFunctionType<End, BigInt>;
        type Permission = HostTypeList<HostCreatedFunction<Constant>, End>;
        struct Constant;
        impl HostCallableSchema for Constant {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "callbacks";
            const NAME: &'static str = "constant";
            type Arguments = End;
            type Return = BigInt;
            type Captures = End;
            type Constructions = End;
            type Completion = HostReturns;
        }
        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        fn constant<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, BigInt>,
            _: HostCaptures<'call, End>,
            _: HostConstructions<'call, End>,
        ) -> Result<HostCallCompletion<'call, BigInt>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            Ok(call.return_value(42.into()))
        }
        fn make<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, Thunk>,
            constructions: HostConstructions<'call, Permission>,
        ) -> Result<HostCallCompletion<'call, Thunk>, HostCallError> {
            let value = call.construct_function(constructions.at::<HostTypeIndex0>(), ());
            Ok(call.return_value(value))
        }
        let hosts = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_declared_function::<Provider, (), Thunk, Permission, _>(
                    HostFunctionDeclaration::new("make"),
                    make,
                )
                .unwrap()])
            .unwrap()
            .with_callable::<Provider, Constant, (), _>(constant)
            .unwrap()
        };
        let source = r#"
@external(erlang, "native", "make") fn make() -> fn() -> Int
pub fn main() { make()() }
"#;
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            hosts(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap(),
            crate::Value::Int(42.into())
        );

        #[derive(Clone, Copy)]
        enum Change {
            None,
            GraphBody,
            DuplicateBody,
            MissingPermission,
            SourceTarget,
        }
        for change in [
            Change::None,
            Change::GraphBody,
            Change::DuplicateBody,
            Change::MissingPermission,
            Change::SourceTarget,
        ] {
            let (mut program, mut values, nevers) = super::super::tests::lowered(source, hosts());
            let body = values
                .iter()
                .position(|metadata| metadata.name() == "constant")
                .unwrap();
            let factory = values
                .iter()
                .position(|metadata| metadata.name() == "make")
                .unwrap();
            let entry = values[body].callable_entry.unwrap();
            let source_entry = program
                .functions
                .value_returns
                .int_functions
                .iter()
                .position(|entry| matches!(entry, ValueFunctionEntry::Graph(_)))
                .unwrap();
            let expected = match change {
                Change::None => Ok(()),
                Change::GraphBody => {
                    let (replacement, _, _) = super::super::tests::lowered(source, hosts());
                    let graph = owned(owned(replacement.functions).value_returns.int_functions)
                        .into_vec()
                        .into_iter()
                        .nth(source_entry)
                        .unwrap();
                    owned_mut(
                        &mut owned_mut(&mut program.functions)
                            .value_returns
                            .int_functions,
                    )[entry.index] = graph;
                    Err(super::failure(true, body))
                }
                Change::DuplicateBody => {
                    let duplicate = values.len();
                    let (_, mut repeated, _) = super::super::tests::lowered(source, hosts());
                    values.push(repeated.swap_remove(body));
                    Err(super::failure(true, duplicate))
                }
                Change::MissingPermission => {
                    values[factory].constructions.callables = Vec::new().into();
                    Err(super::failure(true, factory))
                }
                Change::SourceTarget => {
                    let mut constructions =
                        values[factory].constructions.callables.clone().into_vec();
                    constructions[0].target =
                        crate::plan::execution::function::RuntimeFunctionId::Core(
                            crate::plan::execution::function::CoreRuntimeFunctionId::Int(
                                IntFunctionId(source_entry),
                            ),
                        );
                    values[factory].constructions.callables = constructions.into();
                    Err(super::failure(true, factory))
                }
            };
            let common = &program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
            let sources = Sources::admit(common.root, &common.modules).unwrap();
            let context = Instructions {
                types: &types,
                catalog: &catalog,
                sources: &sources,
                constants: &common.constants,
            };
            let linked = NativeFunctions::new(&values, &nevers, hosts()).unwrap();
            assert!(linked.callable_bindings.borrow().is_empty());
            functions::all(&program.functions, &context, &linked).unwrap();
            assert_eq!(linked.callables(&context), expected);
        }
    }
}
