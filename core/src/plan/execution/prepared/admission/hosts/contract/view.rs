use super::{ContractError, HostedFunctionMetadata, Types};
use crate::plan::execution::host::{HostCallParameter, HostFunctionCompletion, HostNativeView};
use crate::plan::execution::type_::TypeMetadata;

pub(super) fn metadata(
    metadata: &HostedFunctionMetadata,
    view: &HostNativeView,
    types: &Types<'_>,
    returns_value: bool,
) -> Result<(), ContractError> {
    if metadata.completion
        != if returns_value {
            HostFunctionCompletion::Value
        } else {
            HostFunctionCompletion::Uninhabited
        }
    {
        return Err(ContractError::Native);
    }
    if metadata.callable_entry.is_none() {
        return Err(ContractError::Callable);
    }
    let signature = &metadata.signature;
    if signature.arguments.len() != metadata.type_.arguments.len()
        || signature.arguments.len() != metadata.parameters.call.len()
        || signature.arguments.len() != view.arguments.len()
        || signature.arguments.len() != view.source.arguments.len()
    {
        return Err(ContractError::Signature);
    }
    for (nominal, actual) in signature
        .arguments
        .iter()
        .zip(metadata.type_.arguments.iter())
        .chain([(&*signature.return_, &*metadata.type_.return_)])
    {
        types.metadata(nominal).map_err(ContractError::Type)?;
        if !types
            .matches(nominal, actual)
            .map_err(ContractError::Type)?
        {
            return Err(ContractError::Signature);
        }
    }
    if !returns_value && types.matched_metadata_inhabited(&signature.return_) {
        return Err(ContractError::Native);
    }
    if metadata
        .parameters
        .call
        .iter()
        .any(|parameter| !matches!(parameter, HostCallParameter::Value(_)))
    {
        return Err(ContractError::Parameters);
    }
    let [capture] = &*metadata.parameters.captures else {
        return Err(ContractError::Captures);
    };
    let source = TypeMetadata::Function(view.source.clone());
    types.metadata(&source).map_err(ContractError::Type)?;
    if !types.metadata_matches_value(
        &source,
        types
            .shape_type(capture.shape)
            .map_err(ContractError::Type)?,
    ) {
        return Err(ContractError::Captures);
    }
    let constructions = &metadata.constructions;
    if !constructions.lists.entries.is_empty()
        || !constructions.customs.entries.is_empty()
        || !constructions.externals.entries.is_empty()
        || !constructions.callables.is_empty()
        || !constructions.natives.roots.is_empty()
        || !constructions.natives.nodes.is_empty()
    {
        return Err(ContractError::Construction);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{ContractError, HostCallParameter, HostFunctionCompletion, TypeMetadata, Types};
    use crate::host::native::{NativeCall, NativeRules};
    use crate::plan::execution::graph::{IntLocalId, ParamSlot};
    use crate::plan::execution::host::{HostedFunctionMetadata, NativeConversionId};
    use crate::plan::execution::prepared::admission::hosts::tests::lowered;
    use crate::plan::execution::prepared::admission::type_::TypeError;
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::{ListTypeId, ValueShapeId, ValueType};
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
    fn view_metadata_requires_checked_signatures_one_source_capture_and_no_constructions() {
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
                        let converted = call.convert::<HostTypeIndex0>(&source).unwrap();
                        Ok::<HostCallCompletion<'_, Output>, HostCallError>(call.finish(converted))
                    },
                )
                .unwrap()])
            .unwrap()
        };
        let source = r#"
@external(erlang, "gleam@function", "identity") fn coerce(value: a) -> b
pub fn main() {
  let calculate: fn(String) -> BitArray = coerce(fn(_value: BitArray) { "converted" })
  calculate("input") == <<"converted":utf8>>
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
        let original = values
            .iter()
            .find(|metadata| metadata.native_view.is_some())
            .unwrap();
        let admit = |metadata: &HostedFunctionMetadata, returns_value| {
            super::metadata(
                metadata,
                metadata.native_view.as_ref().unwrap(),
                &types,
                returns_value,
            )
        };
        let linked = crate::plan::execution::prepared::admission::hosts::NativeFunctions::new(
            &values,
            &nevers,
            providers(),
        )
        .unwrap();
        let registration = &linked.registrations[linked
            .values
            .iter()
            .find(|(metadata, _, _)| metadata.native_view.is_some())
            .unwrap()
            .2];
        let catalog = crate::plan::execution::prepared::admission::catalog::Catalog::admit(
            &common.function_parameters,
            &program.functions,
            &types,
        )
        .unwrap();
        let entry = original.callable_entry.unwrap();
        let declaration = catalog.function(entry.family, entry.index).unwrap();
        assert_eq!(
            super::super::call(original, registration, &declaration, &types),
            Ok(())
        );
        let changed = crate::plan::execution::prepared::admission::catalog::Function {
            parameters: &[],
            parameter_shapes: &[],
            ..declaration
        };
        assert_eq!(
            super::super::call(original, registration, &changed, &types),
            Err(ContractError::Parameters)
        );
        assert_eq!(admit(original, true), Ok(()));
        assert_eq!(admit(original, false), Err(ContractError::Native));

        type Mutation = fn(&mut HostedFunctionMetadata);
        let cases: [(Mutation, ContractError); 11] = [
            (
                |metadata| metadata.completion = HostFunctionCompletion::Uninhabited,
                ContractError::Native,
            ),
            (
                |metadata| metadata.callable_entry = None,
                ContractError::Callable,
            ),
            (
                |metadata| metadata.signature.arguments = Vec::new().into(),
                ContractError::Signature,
            ),
            (
                |metadata| metadata.type_.arguments = Vec::new().into(),
                ContractError::Signature,
            ),
            (
                |metadata| metadata.parameters.call = Vec::new().into(),
                ContractError::Signature,
            ),
            (
                |metadata| metadata.native_view.as_mut().unwrap().arguments = Vec::new().into(),
                ContractError::Signature,
            ),
            (
                |metadata| {
                    metadata.native_view.as_mut().unwrap().source.arguments = Vec::new().into()
                },
                ContractError::Signature,
            ),
            (
                |metadata| metadata.type_.arguments = vec![ValueType::Bool].into(),
                ContractError::Signature,
            ),
            (
                |metadata| {
                    metadata.parameters.call = vec![HostCallParameter::Int(IntLocalId(0))].into()
                },
                ContractError::Parameters,
            ),
            (
                |metadata| metadata.parameters.captures = Vec::new().into(),
                ContractError::Captures,
            ),
            (
                |metadata| {
                    metadata.constructions.natives.roots = vec![NativeConversionId(0)].into()
                },
                ContractError::Construction,
            ),
        ];
        for (change, expected) in cases {
            let mut metadata = original.clone();
            change(&mut metadata);
            assert_eq!(admit(&metadata, true), Err(expected));
        }

        let mut metadata = original.clone();
        metadata.type_.arguments = vec![ValueType::List(ListTypeId(999))].into();
        assert_eq!(
            admit(&metadata, true),
            Err(ContractError::Type(TypeError::MissingList { index: 999 }))
        );

        let mut metadata = original.clone();
        let capture = &metadata.parameters.captures[0];
        metadata.parameters.captures = vec![ParamSlot {
            local: capture.local.clone(),
            shape: ValueShapeId(999),
        }]
        .into();
        assert_eq!(
            admit(&metadata, true),
            Err(ContractError::Type(TypeError::MissingShape { index: 999 }))
        );

        let mut metadata = original.clone();
        let capture = &metadata.parameters.captures[0];
        let string_shape = types
            .shape_types()
            .iter()
            .position(|type_| *type_ == ValueType::String)
            .unwrap();
        metadata.parameters.captures = vec![ParamSlot {
            local: capture.local.clone(),
            shape: ValueShapeId(string_shape),
        }]
        .into();
        assert_eq!(admit(&metadata, true), Err(ContractError::Captures));

        static RECURSIVE: TypeMetadata = TypeMetadata::List(Node::Static(&RECURSIVE));
        let mut metadata = original.clone();
        metadata.signature.arguments = vec![RECURSIVE.clone()].into();
        assert_eq!(
            admit(&metadata, true),
            Err(ContractError::Type(TypeError::RecursiveMetadata))
        );
        let mut metadata = original.clone();
        metadata.native_view.as_mut().unwrap().source.arguments = vec![RECURSIVE.clone()].into();
        assert_eq!(
            admit(&metadata, true),
            Err(ContractError::Type(TypeError::RecursiveMetadata))
        );

        let mut metadata = original.clone();
        metadata.completion = HostFunctionCompletion::Uninhabited;
        assert_eq!(admit(&metadata, false), Err(ContractError::Native));

        // Never completion may inspect inhabitation only after the supplied
        // signature has passed type validation and matched an admitted type.
        static RECURSIVE_TUPLE: [TypeMetadata; 1] =
            [TypeMetadata::Tuple(Table::Static(&RECURSIVE_TUPLE))];
        let mut metadata = original.clone();
        metadata.completion = HostFunctionCompletion::Uninhabited;
        metadata.signature.return_ = Node::Static(&RECURSIVE_TUPLE[0]);
        assert_eq!(
            admit(&metadata, false),
            Err(ContractError::Type(TypeError::RecursiveMetadata))
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
