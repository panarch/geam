use super::{ContractError, NativeError, NativeFunctions};
use crate::host::HostProfile;
use crate::plan::execution::function::HostedExecutionGraph;
use crate::plan::execution::host::{HostCallableEntry, NativeConversionKind};
use crate::plan::execution::prepared::admission::call::Target;
use crate::plan::execution::prepared::admission::instruction::Instructions;
use crate::plan::execution::type_::TypeMetadata;
use std::collections::HashSet;

pub(super) fn admit<Profile: HostProfile>(
    hosts: &NativeFunctions<'_, Profile>,
    context: &Instructions<'_, '_, HostedExecutionGraph>,
) -> Result<(), NativeError> {
    // Host table and function admission already checked conversion kinds and
    // adapter ABIs. Bind each candidate to that checked entry and parent codec.
    let mut claimed = HashSet::new();
    for (parent_value, parent_index, parent, registration) in hosts
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
        )
    {
        let fail = || NativeError::Contract {
            value: parent_value,
            index: parent_index,
            reason: ContractError::Native,
        };
        for node in parent.constructions.natives.nodes.iter() {
            let (NativeConversionKind::Function(views), TypeMetadata::Function(target_type)) =
                (&node.kind, &node.type_)
            else {
                continue;
            };
            for candidate in views.iter() {
                let (metadata, target_registration) = if candidate.host_value {
                    hosts
                        .values
                        .get(candidate.host)
                        .map(|(metadata, _, registration)| (*metadata, *registration))
                } else {
                    hosts
                        .nevers
                        .get(candidate.host)
                        .map(|(metadata, _, registration)| (*metadata, *registration))
                }
                .ok_or_else(fail)?;
                let operation = metadata.native_view.as_ref().ok_or_else(fail)?;
                if !claimed.insert((candidate.host_value, candidate.host))
                    || parent.native_view.is_some()
                    || target_registration != registration
                    || operation.parent_value != parent_value
                    || operation.parent != parent_index
                    || operation.source != candidate.source
                    || &metadata.signature != target_type
                    || metadata.package != parent.package
                    || metadata.site != parent.site
                    || metadata.type_ != candidate.type_
                    || metadata.parameters.captures != candidate.captures
                    || metadata.type_arguments.len() != parent.type_arguments.len()
                    || metadata
                        .type_arguments
                        .iter()
                        .zip(parent.type_arguments.iter())
                        .any(|(a, b)| a.type_ != b.type_ || a.shape != b.shape)
                {
                    return Err(fail());
                }
                let target = candidate
                    .target
                    .resolve(context.catalog, context.types)
                    .map_err(|_| fail())?;
                let entry = HostCallableEntry {
                    family: target.family,
                    index: target.index,
                };
                if metadata.callable_entry != Some(entry)
                    || hosts.callable_bindings.borrow().get(&entry).copied()
                        != Some((candidate.host_value, candidate.host))
                {
                    return Err(fail());
                }
                for (id, expected) in operation
                    .arguments
                    .iter()
                    .zip(candidate.source.arguments.iter())
                    .chain([(&operation.return_, target_type.return_.as_ref())])
                {
                    let node = parent
                        .constructions
                        .natives
                        .nodes
                        .get(id.0)
                        .ok_or_else(fail)?;
                    if &node.type_ != expected {
                        return Err(fail());
                    }
                }
            }
        }
    }
    if hosts
        .values
        .iter()
        .enumerate()
        .any(|(index, (metadata, _, _))| {
            metadata.native_view.is_some() != claimed.contains(&(true, index))
        })
        || hosts
            .nevers
            .iter()
            .enumerate()
            .any(|(index, (metadata, _, _))| {
                metadata.native_view.is_some() != claimed.contains(&(false, index))
            })
    {
        return Err(NativeError::Call(ContractError::Native));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ContractError, NativeError, NativeFunctions, admit};
    use crate::host::native::{NativeCall, NativeRules};
    use crate::plan::execution::function::{
        BitArrayFunctionId, CoreRuntimeFunctionId, RuntimeFunctionId,
    };
    use crate::plan::execution::host::{
        HostNativeView, NativeConversionId, NativeConversionKind, NativeFunctionView,
    };
    use crate::plan::execution::prepared::admission::tests::owned_mut;
    use crate::plan::execution::prepared::admission::{
        catalog::Catalog,
        functions::{self, Hosts},
        hosts::tests::lowered,
        instruction::Instructions,
        source::Sources,
        type_::Types,
    };
    use crate::{
        HostCallCompletion, HostCallError, HostProvider, HostProviderModule, HostProviderSet,
        HostTypeIndex0, HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue,
        StatelessHostProfile,
    };

    struct Provider;
    impl HostProvider<StatelessHostProfile> for Provider {
        type State = ();
        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    #[test]
    fn candidates_require_unique_checked_entries_parent_edges_and_complete_claims() {
        type Input = HostTypeParameter<1>;
        type Output = HostTypeParameter<0>;
        type One<Type> = HostTypeList<Type, HostTypeListEnd>;
        let providers = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_native_function::<Provider, (Input,), Output, One<Output>, _>(
                    "coerce",
                    NativeRules::default().retained_views::<One<Input>>(),
                    |mut call: NativeCall<
                        '_,
                        StatelessHostProfile,
                        Provider,
                        Output,
                        One<Output>,
                    >,
                     value: HostValue<'_, Input>| {
                        assert_eq!(call.call().state(), &mut ());
                        let source = call.source::<Input>(value);
                        let value = call.convert::<HostTypeIndex0>(&source).unwrap();
                        Ok::<HostCallCompletion<'_, Output>, HostCallError>(call.finish(value))
                    },
                )
                .unwrap()])
            .unwrap()
        };
        let source = r#"
@external(erlang, "gleam@function", "identity") fn coerce(value: a) -> b
pub fn main() {
  let #(first, _second): #(fn(String) -> BitArray, fn(String) -> BitArray) =
    coerce(#(fn(_input: BitArray) { "converted" }, fn(input: Bool) { input }))
  first("input") == <<"converted":utf8>>
}
"#;
        let (program, values, nevers) = lowered(source, providers());
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
        let parent = values
            .iter()
            .position(|metadata| metadata.native_view.is_none())
            .unwrap();
        let node = values[parent]
            .constructions
            .natives
            .nodes
            .iter()
            .position(|node| matches!(node.kind, NativeConversionKind::Function(_)))
            .unwrap();
        let candidates = values[parent]
            .constructions
            .natives
            .nodes
            .iter()
            .find_map(|node| match &node.kind {
                NativeConversionKind::Function(candidates) => Some(candidates.to_vec()),
                _ => None,
            })
            .unwrap();
        let candidate = candidates[0].host;
        type Mutation = fn(&mut [NativeFunctionView], &mut HostNativeView, usize);
        let cases: [Mutation; 9] = [
            |candidates, _, _| candidates[0].host = 999,
            |candidates, _, _| candidates[0].host_value = false,
            |candidates, _, _| {
                candidates[0].target = RuntimeFunctionId::Core(CoreRuntimeFunctionId::BitArray(
                    BitArrayFunctionId(999),
                ))
            },
            |candidates, _, _| candidates[1].host = candidates[0].host,
            |_, operation, _| operation.arguments = vec![NativeConversionId(999)].into(),
            |_, operation, _| operation.arguments = vec![NativeConversionId(0)].into(),
            |_, operation, _| operation.return_ = NativeConversionId(0),
            |candidates, _, parent| candidates[0].host = parent,
            |candidates, _, _| candidates[0].target = candidates[1].target.clone(),
        ];
        let linked = NativeFunctions::new(&values, &nevers, providers()).unwrap();
        assert_eq!(linked.tables(&context), Ok(()));
        assert_eq!(
            functions::all(&program.functions, &context, &linked),
            Ok(())
        );
        assert_eq!(admit(&linked, &context), Ok(()));
        assert_eq!(super::super::callable::admit(&linked, &context), Ok(()));
        for mutate in cases {
            let mut changed = values.clone();
            let mut changed_candidates = candidates.clone();
            mutate(
                &mut changed_candidates,
                changed[candidate].native_view.as_mut().unwrap(),
                parent,
            );
            owned_mut(&mut changed[parent].constructions.natives.nodes)[node].kind =
                NativeConversionKind::Function(changed_candidates.into());
            let linked = NativeFunctions::new(&changed, &nevers, providers()).unwrap();
            assert_eq!(linked.tables(&context), Ok(()));
            assert_eq!(
                functions::all(&program.functions, &context, &linked),
                Ok(())
            );
            assert_eq!(
                admit(&linked, &context),
                Err(NativeError::Contract {
                    value: true,
                    index: parent,
                    reason: ContractError::Native
                })
            );
        }
        let mut changed = values.clone();
        owned_mut(&mut changed[parent].constructions.natives.nodes)[node].kind =
            NativeConversionKind::Exact;
        // The checked view bodies remain, but their parent no longer claims them.
        let linked = NativeFunctions::new(&changed, &nevers, providers()).unwrap();
        assert_eq!(
            admit(&linked, &context),
            Err(NativeError::Call(ContractError::Native))
        );
        assert_eq!(
            super::super::callable::admit(&linked, &context),
            Err(NativeError::Call(ContractError::Native))
        );

        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new("main", "src/main.gleam", source)],
            )],
            providers(),
        )
        .unwrap();
        let mut execution =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()),
            Ok(crate::Value::Bool(true))
        );
    }
}
