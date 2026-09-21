mod callback;
mod construction;
mod shape;

use super::ContractError;
use super::link::Registration;
use crate::host::{HostParameter, HostTypeDescriptor};
use crate::plan::execution::graph::ParamSlot;
use crate::plan::execution::host::{HostCallParameter, HostedFunctionMetadata};
use crate::plan::execution::prepared::admission::catalog::Function;
use crate::plan::execution::prepared::admission::local::Locals;
use crate::plan::execution::prepared::admission::type_::Types;
use crate::plan::execution::type_::TypeMetadata;

pub(super) fn metadata(
    metadata: &HostedFunctionMetadata,
    registration: &Registration,
    types: &Types<'_>,
    returns_value: bool,
) -> Result<(), ContractError> {
    let schema = &registration.schema;
    if metadata.callable_entry.is_some() != schema.is_callable() {
        return Err(ContractError::Callable);
    }
    if metadata.type_arguments.len() != schema.scheme().parameters().len() {
        return Err(ContractError::TypeArguments);
    }
    for argument in metadata.type_arguments.iter() {
        types
            .metadata(&argument.type_)
            .map_err(ContractError::Type)?;
        if !types.metadata_matches_value(
            &argument.type_,
            types
                .shape_type(argument.shape)
                .map_err(ContractError::Type)?,
        ) {
            return Err(ContractError::TypeArguments);
        }
    }
    let arguments = metadata
        .type_arguments
        .iter()
        .map(|argument| argument.type_.materialize())
        .collect::<Vec<_>>();
    let signature = &metadata.signature;
    let type_ = &metadata.type_;
    if signature.arguments.len() != schema.parameters().len()
        || type_.arguments.len() != signature.arguments.len()
    {
        return Err(ContractError::Signature);
    }
    for ((declaration, signature), type_) in schema
        .parameters()
        .iter()
        .zip(signature.arguments.iter())
        .zip(type_.arguments.iter())
    {
        if !matches(declaration, signature, &arguments, types)?
            || !types
                .matches(signature, type_)
                .map_err(ContractError::Type)?
        {
            return Err(ContractError::Signature);
        }
    }
    if !matches(schema.return_type(), &signature.return_, &arguments, types)?
        || !types
            .matches(&signature.return_, &type_.return_)
            .map_err(ContractError::Type)?
    {
        return Err(ContractError::Signature);
    }
    if metadata.parameters.call.len() != schema.layout().len() {
        return Err(ContractError::Parameters);
    }
    for (stored, original) in metadata.parameters.call.iter().zip(schema.layout()) {
        if !parameter_kind(stored, original) {
            return Err(ContractError::Parameters);
        }
    }
    if metadata.parameters.captures.len() != schema.captures().len() {
        return Err(ContractError::Captures);
    }
    for (stored, original) in metadata.parameters.captures.iter().zip(schema.captures()) {
        let stored_type = types
            .shape_type(stored.shape)
            .map_err(ContractError::Type)?;
        let expected = original.resolve_sealed(&|index| arguments[index].clone());
        let expected = TypeMetadata::from_public(&expected);
        if !types.metadata_matches_value(&expected, stored_type) {
            return Err(ContractError::Captures);
        }
    }
    construction::admit(metadata, registration, &arguments, types)?;
    callback::admit(registration, &metadata.type_arguments, types, returns_value)
}

pub(super) fn call(
    metadata: &HostedFunctionMetadata,
    registration: &Registration,
    declaration: &Function<'_>,
    types: &Types<'_>,
) -> Result<(), ContractError> {
    if let Some(entry) = metadata.callable_entry
        && (entry.family != declaration.family || entry.index != declaration.index)
    {
        return Err(ContractError::Callable);
    }
    if declaration.captures != metadata.parameters.captures.as_ref() {
        return Err(ContractError::Captures);
    }
    shape::admit(metadata, registration, declaration, types)?;
    let slots = declaration
        .parameters
        .iter()
        .zip(declaration.parameter_shapes)
        .map(|(local, shape)| ParamSlot {
            local: local.clone(),
            shape: *shape,
        })
        .collect::<Vec<_>>();
    let mut locals = Locals::default();
    for ((stored, slot), expected) in metadata
        .parameters
        .call
        .iter()
        .zip(&slots)
        .zip(metadata.type_.arguments.iter())
    {
        if stored.local() != slot.local {
            return Err(ContractError::Parameters);
        }
        locals
            .define(slot, types)
            .map_err(|_| ContractError::Parameters)?;
        if &types.shape_types()[slot.shape.index()] != expected {
            return Err(ContractError::Signature);
        }
    }
    for slot in declaration.captures {
        locals
            .define(slot, types)
            .map_err(|_| ContractError::Captures)?;
    }
    if declaration.return_type != metadata.type_.return_() {
        return Err(ContractError::Signature);
    }
    Ok(())
}

fn matches(
    descriptor: &HostTypeDescriptor,
    metadata: &TypeMetadata,
    arguments: &[crate::plan::ValueType],
    types: &Types<'_>,
) -> Result<bool, ContractError> {
    types.metadata(metadata).map_err(ContractError::Type)?;
    let expected = descriptor.resolve_sealed(&|index| arguments[index].clone());
    Ok(metadata.compare(&expected).is_eq())
}

fn parameter_kind(stored: &HostCallParameter, original: &HostParameter) -> bool {
    match (stored, original) {
        (HostCallParameter::Int(_), HostParameter::Int(_))
        | (HostCallParameter::Float(_), HostParameter::Float(_))
        | (HostCallParameter::String(_), HostParameter::String(_))
        | (HostCallParameter::BitArray(_), HostParameter::BitArray(_))
        | (HostCallParameter::UtfCodepoint(_), HostParameter::UtfCodepoint(_))
        | (HostCallParameter::Bool(_), HostParameter::Bool(_))
        | (HostCallParameter::Nil(_), HostParameter::Nil(_))
        | (HostCallParameter::Value(_), HostParameter::Value(_))
        | (HostCallParameter::List(_), HostParameter::List(_))
        | (HostCallParameter::Tuple(_), HostParameter::Tuple(_))
        | (HostCallParameter::Custom(_), HostParameter::Custom(_))
        | (HostCallParameter::External(_), HostParameter::External(_)) => true,
        (
            HostCallParameter::Function { arity: left, .. },
            HostParameter::Function { arity: right, .. },
        ) => left == right,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::{ContractError, HostedFunctionMetadata, Types, metadata};
    use crate::host::{
        HostCall, HostCallCompletion, HostCallError, HostProvider, HostProviderModule,
        HostProviderSet, HostTypeParameter, HostValue, StatelessHostProfile,
    };
    use crate::plan::execution::prepared::admission::hosts::{
        link::NativeFunctions, tests::lowered,
    };
    use crate::plan::execution::prepared::admission::type_::TypeError;
    use crate::plan::execution::storage::{Node, Table};
    use crate::plan::execution::type_::{ListTypeId, TypeMetadata, ValueShapeId, ValueType};

    #[test]
    fn native_callable_contracts_validate_body_identity_capture_types_and_unique_slots() {
        use crate::plan::execution::prepared::admission::catalog::{Catalog, Function};
        use crate::{
            HostCallableSchema, HostCaptures, HostConstructions, HostCreatedFunction,
            HostFunctionType, HostReturns, HostTypeIndex0, HostTypeList, HostTypeListEnd,
        };
        type End = HostTypeListEnd;
        type One<T> = HostTypeList<T, End>;
        type Captures = HostTypeList<bool, One<bool>>;
        type Thunk = HostFunctionType<One<num_bigint::BigInt>, num_bigint::BigInt>;
        type Permission = One<HostCreatedFunction<Add>>;
        struct Add;
        impl HostCallableSchema for Add {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "callbacks";
            const NAME: &'static str = "add";
            type Arguments = One<num_bigint::BigInt>;
            type Return = num_bigint::BigInt;
            type Captures = Captures;
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
        fn add<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, num_bigint::BigInt>,
            captures: HostCaptures<'call, Captures>,
            _: HostConstructions<'call, End>,
            value: num_bigint::BigInt,
        ) -> Result<HostCallCompletion<'call, num_bigint::BigInt>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            assert_eq!(call.captures(captures), (true, (false, ())));
            Ok(call.return_value(value + 2))
        }
        fn make<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, Thunk>,
            constructions: HostConstructions<'call, Permission>,
            left: bool,
            right: bool,
        ) -> Result<HostCallCompletion<'call, Thunk>, HostCallError> {
            let callback =
                call.construct_function(constructions.at::<HostTypeIndex0>(), (left, (right, ())));
            Ok(call.return_value(callback))
        }
        let hosts = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main").unwrap()
            .with_scoped_function_and_constructions::<Provider, (bool, bool), Thunk, Permission, _>("make", make).unwrap()]).unwrap()
            .with_callable::<Provider, Add, (num_bigint::BigInt,), _>(add).unwrap()
        };
        let source = r#"
@external(erlang, "native", "make") fn make(left: Bool, right: Bool) -> fn(Int) -> Int
pub fn main() { make(True, False)(40) }
"#;
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new("main", "main.gleam", source)],
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
        let (program, values, nevers) = lowered(source, hosts());
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
        let linked = NativeFunctions::new(&values, &nevers, hosts()).unwrap();
        let index = values
            .iter()
            .position(|value| value.name() == "add")
            .unwrap();
        let original = &values[index];
        let registration = &linked.registrations[linked.values[index].2];
        let entry = original.callable_entry.unwrap();
        assert_eq!(metadata(original, registration, &types, true), Ok(()));
        assert_eq!(
            super::call(
                original,
                registration,
                &catalog.function(entry.family, entry.index).unwrap(),
                &types
            ),
            Ok(())
        );
        let (_, mut altered, _) = lowered(source, hosts());
        let mut changed = altered.remove(index);
        changed.callable_entry = None;
        assert_eq!(
            metadata(&changed, registration, &types, true),
            Err(ContractError::Callable)
        );
        changed.callable_entry = original.callable_entry;
        changed.parameters.captures = original.parameters.captures.clone();
        changed.parameters.captures = Vec::new().into();
        assert_eq!(
            metadata(&changed, registration, &types, true),
            Err(ContractError::Captures)
        );
        changed.callable_entry = original.callable_entry;
        changed.parameters.captures = original.parameters.captures.clone();
        let mut captures = changed.parameters.captures.to_vec();
        captures[0].shape = ValueShapeId(999);
        changed.parameters.captures = captures.into();
        assert_eq!(
            metadata(&changed, registration, &types, true),
            Err(ContractError::Type(TypeError::MissingShape { index: 999 }))
        );
        changed.callable_entry = original.callable_entry;
        changed.parameters.captures = original.parameters.captures.clone();
        let mut captures = changed.parameters.captures.to_vec();
        let declaration = catalog.function(entry.family, entry.index).unwrap();
        captures[0].shape = declaration.parameter_shapes[0];
        changed.parameters.captures = captures.into();
        assert_eq!(
            metadata(&changed, registration, &types, true),
            Err(ContractError::Captures)
        );
        changed.callable_entry = original.callable_entry;
        changed.parameters.captures = original.parameters.captures.clone();
        changed.callable_entry = Some(crate::plan::execution::host::HostCallableEntry {
            index: entry.index + 1,
            ..entry
        });
        assert_eq!(
            super::call(&changed, registration, &declaration, &types),
            Err(ContractError::Callable)
        );
        let mut captures = original.parameters.captures.to_vec();
        captures[1] = captures[0].clone();
        changed.callable_entry = original.callable_entry;
        changed.parameters.captures = original.parameters.captures.clone();
        changed.parameters.captures = captures.clone().into();
        assert_eq!(metadata(&changed, registration, &types, true), Ok(()));
        let duplicated = Function {
            captures: &captures,
            ..declaration
        };
        assert_eq!(
            super::call(&changed, registration, &duplicated, &types),
            Err(ContractError::Captures)
        );
    }

    #[test]
    fn callable_parameter_layouts_require_the_registered_arity() {
        use crate::host::{HostParameter, HostParameterLayout};
        use crate::plan::execution::graph::{IntFunctionLocalId, ParamLocal};
        use crate::plan::execution::host::HostCallParameter;
        use crate::plan::execution::type_::FunctionType;

        let mut layout = HostParameterLayout::default();
        let slot = layout.register_function_parameter_slot(1);
        let original = HostParameter::Function { slot, arity: 1 };
        for arity in [0, 1, 2] {
            let stored = HostCallParameter::Function {
                local: ParamLocal::IntFunction {
                    local: IntFunctionLocalId(0),
                    type_: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                },
                arity,
            };
            assert_eq!(super::parameter_kind(&stored, &original), arity == 1);
        }
        assert!(!super::parameter_kind(
            &HostCallParameter::Int(crate::plan::execution::graph::IntLocalId(0)),
            &original,
        ));
    }

    #[test]
    fn native_call_nominal_types_must_match_even_when_their_layouts_agree() {
        use crate::host::{
            HostCustom, HostCustomConstructorDefinition, HostCustomConstructorList,
            HostCustomConstructorListEnd, HostCustomFieldListEnd, HostCustomSchema, HostCustomType,
        };
        use crate::plan::execution::function::{FunctionCatalog, FunctionTableFamily};
        use crate::plan::execution::graph::{CustomLocal, CustomLocalId, ParamLocal};
        use crate::plan::execution::host::HostCallParameter;
        use crate::plan::execution::prepared::admission::{catalog::Catalog, tests::owned_mut};
        use crate::plan::execution::type_::{
            CustomConstructorRefinement, CustomValueShape, CustomValueShapeId, ValueShapeDescriptor,
        };

        struct First;
        impl HostCustomSchema for First {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "main";
            const NAME: &'static str = "First";
            const PARAMETER_COUNT: usize = 0;
            type Constructors = HostCustomConstructorList<Self, HostCustomConstructorListEnd>;
        }
        impl HostCustomConstructorDefinition for First {
            const NAME: &'static str = "First";
            type Fields = HostCustomFieldListEnd;
        }
        struct Native;
        impl HostProvider<StatelessHostProfile> for Native {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        type Item = HostCustomType<First>;
        fn identity<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Native, Item>,
            value: HostCustom<'call, Item>,
        ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            Ok(call.return_value(value))
        }
        let hosts = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_scoped_function::<Native, (Item,), Item, _>("identity", identity)
                .unwrap()])
            .unwrap()
        };
        let source = r#"
pub type First { First }
pub type Second { Second }
fn second(value: Second) { value }
@external(erlang, "native", "identity")
fn identity(value: First) -> First
pub fn main() { echo second(Second) identity(First) }
"#;
        for change_argument in [false, true] {
            let (program, mut values, nevers) = lowered(source, hosts());
            let common = &program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let (second, _) = common
                .value_shapes
                .custom_shapes
                .iter()
                .enumerate()
                .find(|(_, shape)| {
                    shape.constructor == CustomConstructorRefinement::Any
                        && common.custom_types.types[shape.type_id.index()]
                            .type_
                            .name
                            .as_str()
                            == "Second"
                })
                .unwrap();
            let second = CustomValueShapeId(second);
            let second_shape = ValueShapeId(
                common
                    .value_shapes
                    .shapes
                    .iter()
                    .position(|shape| shape == &ValueShapeDescriptor::Custom(second))
                    .unwrap(),
            );
            let second_local = CustomLocal::new(
                CustomLocalId(0),
                CustomValueShape::new(common.value_shapes.custom_shapes[second.0].type_id, second),
            );
            let family = FunctionTableFamily::Custom;
            let range = common.function_parameters.families[family as usize].clone();
            let index = range
                .clone()
                .find(|index| {
                    let declaration = &common.function_parameters.functions[*index];
                    declaration.parameters.len() == 1
                        && types.shape_type(declaration.return_).unwrap()
                            == values[0].type_.return_()
                })
                .unwrap();
            let mut raw = FunctionCatalog {
                families: common.function_parameters.families.clone(),
                functions: common.function_parameters.functions.to_vec().into(),
                parameters: common.function_parameters.parameters.to_vec().into(),
            };
            if change_argument {
                let start = raw.functions[index].parameters.start;
                owned_mut(&mut raw.parameters)[start] = ParamLocal::Custom(second_local);
                owned_mut(&mut raw.functions)[index].parameter_shapes = vec![second_shape].into();
                values[0].parameters.call =
                    vec![HostCallParameter::Custom(ParamLocal::Custom(second_local))].into();
            } else {
                owned_mut(&mut raw.functions)[index].return_ = second_shape;
            }
            let catalog = Catalog::admit(&raw, &program.functions, &types).unwrap();
            let declaration = catalog.function(family, index - range.start).unwrap();
            let linked = NativeFunctions::new(&values, &nevers, hosts()).unwrap();
            let registration = &linked.registrations[linked.values[0].2];
            assert_eq!(metadata(&values[0], registration, &types, true), Ok(()));
            assert_eq!(
                super::call(&values[0], registration, &declaration, &types),
                Err(ContractError::Signature)
            );
        }
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
        let value =
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap();
        assert_eq!(value.inspect().to_string(), "First");
    }

    #[test]
    fn sealed_native_metadata_checks_generic_arguments_signatures_and_layouts() {
        struct Native;
        impl HostProvider<StatelessHostProfile> for Native {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        type Item = HostTypeParameter<0>;
        fn identity<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Native, Item>,
            value: HostValue<'call, Item>,
        ) -> Result<HostCallCompletion<'call, Item>, HostCallError> {
            assert_eq!(call.state(), &mut ());
            Ok(call.return_value(value))
        }
        let hosts = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_scoped_function::<Native, (Item,), Item, _>("identity", identity)
                .unwrap()])
            .unwrap()
        };
        let source = r#"
@external(erlang, "native", "identity")
fn identity(value: a) -> a
pub fn main() { #(identity(42), True) }
"#;
        static CYCLIC: TypeMetadata = TypeMetadata::List(Node::Static(&CYCLIC));
        type Change = fn(&mut HostedFunctionMetadata);
        let cases: &[(Change, ContractError)] = &[
            (
                |value| {
                    value.type_arguments = vec![crate::plan::execution::host::HostTypeArgument {
                        type_: TypeMetadata::List(Node::Static(&CYCLIC)),
                        shape: value.type_arguments[0].shape,
                    }]
                    .into()
                },
                ContractError::Type(TypeError::RecursiveMetadata),
            ),
            (
                |value| {
                    value.type_arguments = vec![crate::plan::execution::host::HostTypeArgument {
                        type_: TypeMetadata::Bool,
                        shape: value.type_arguments[0].shape,
                    }]
                    .into()
                },
                ContractError::TypeArguments,
            ),
            (
                |value| {
                    value.type_arguments = vec![crate::plan::execution::host::HostTypeArgument {
                        type_: TypeMetadata::Int,
                        shape: ValueShapeId(999),
                    }]
                    .into()
                },
                ContractError::Type(TypeError::MissingShape { index: 999 }),
            ),
            (
                |value| value.signature.arguments = Table::Static(&[]),
                ContractError::Signature,
            ),
            (
                |value| value.type_.arguments = Table::Static(&[]),
                ContractError::Signature,
            ),
            (
                |value| value.signature.arguments = Table::Static(&[TypeMetadata::Bool]),
                ContractError::Signature,
            ),
            (
                |value| value.type_.arguments = Table::Static(&[ValueType::Bool]),
                ContractError::Signature,
            ),
            (
                |value| value.signature.return_ = Node::Static(&TypeMetadata::Bool),
                ContractError::Signature,
            ),
            (
                |value| value.type_.return_ = Node::Static(&ValueType::Bool),
                ContractError::Signature,
            ),
            (
                |value| value.parameters.call = Table::Static(&[]),
                ContractError::Parameters,
            ),
            (
                |value| {
                    value.parameters.call =
                        vec![crate::plan::execution::host::HostCallParameter::Bool(
                            crate::plan::execution::graph::BoolLocalId(0),
                        )]
                        .into()
                },
                ContractError::Parameters,
            ),
            (
                |value| {
                    value.signature.arguments =
                        vec![TypeMetadata::List(Node::Static(&CYCLIC))].into()
                },
                ContractError::Type(TypeError::RecursiveMetadata),
            ),
            (
                |value| value.signature.return_ = Node::Static(&CYCLIC),
                ContractError::Type(TypeError::RecursiveMetadata),
            ),
            (
                |value| value.type_.arguments = vec![ValueType::List(ListTypeId(999))].into(),
                ContractError::Type(TypeError::MissingList { index: 999 }),
            ),
            (
                |value| value.type_.return_ = Box::new(ValueType::List(ListTypeId(999))).into(),
                ContractError::Type(TypeError::MissingList { index: 999 }),
            ),
            (
                |value| {
                    value.constructions.lists.entries = vec![(
                        TypeMetadata::List(Box::new(TypeMetadata::Int).into()),
                        ListTypeId(999),
                    )]
                    .into()
                },
                ContractError::Type(TypeError::MissingList { index: 999 }),
            ),
        ];
        for (change, expected) in cases {
            let (program, mut values, nevers) = lowered(source, hosts());
            let common = &program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            assert_eq!(values.len(), 1);
            change(&mut values[0]);
            let linked = NativeFunctions::new(&values, &nevers, hosts()).unwrap();
            let registration = &linked.registrations[linked.values[0].2];
            assert_eq!(
                metadata(&values[0], registration, &types, true).as_ref(),
                Err(expected)
            );
        }
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
        let host = crate::execution_fixture::TestHost::default();
        assert_eq!(
            host.block_on(execution.run_main(&host, &mut (), &mut Vec::new()))
                .unwrap(),
            crate::Value::Tuple(vec![crate::Value::Int(42.into()), crate::Value::Bool(true)])
        );
    }

    #[test]
    fn native_call_slots_reject_captures_and_disagreement_with_the_parameter_catalog() {
        use super::call;
        use crate::plan::execution::function::{FunctionCatalog, FunctionTableFamily};
        use crate::plan::execution::graph::{IntLocalId, ParamLocal, ParamSlot};
        use crate::plan::execution::host::HostCallParameter;
        use crate::plan::execution::prepared::admission::{catalog::Catalog, tests::owned_mut};

        let hosts = || {
            HostProviderSet::<StatelessHostProfile>::from_providers([HostProviderModule::new(
                "app", "main",
            )
            .unwrap()
            .with_function(
                "add",
                |left: num_bigint::BigInt, right: num_bigint::BigInt| left + right,
            )
            .unwrap()])
            .unwrap()
        };
        let source = r#"
@external(erlang, "native", "add")
fn add(left: Int, right: Int) -> Int
pub fn main() { echo True add(20, 22) }
"#;
        type Change = fn(&mut HostedFunctionMetadata, &mut FunctionCatalog, usize, ValueShapeId);
        let cases: &[(Change, ContractError)] = &[
            (
                |_, raw, index, _| {
                    let removed = raw.functions[index].parameters.end - 1;
                    let mut parameters = raw.parameters.to_vec();
                    parameters.remove(removed);
                    raw.parameters = parameters.into();
                    for declaration in owned_mut(&mut raw.functions) {
                        declaration.parameters.start -=
                            usize::from(declaration.parameters.start > removed);
                        declaration.parameters.end -=
                            usize::from(declaration.parameters.end > removed);
                    }
                    let declaration = &mut owned_mut(&mut raw.functions)[index];
                    declaration.parameter_shapes =
                        declaration.parameter_shapes[..1].to_vec().into();
                },
                ContractError::Parameters,
            ),
            (
                |_, raw, index, boolean| {
                    let start = raw.functions[index].parameters.start;
                    owned_mut(&mut raw.parameters)[start] =
                        ParamLocal::Bool(crate::plan::execution::graph::BoolLocalId(0));
                    owned_mut(&mut owned_mut(&mut raw.functions)[index].parameter_shapes)[0] =
                        boolean;
                },
                ContractError::Signature,
            ),
            (
                |value, _, _, _| {
                    value.parameters.call = vec![
                        HostCallParameter::Int(IntLocalId(1)),
                        HostCallParameter::Int(IntLocalId(0)),
                    ]
                    .into()
                },
                ContractError::Parameters,
            ),
            (
                |value, raw, index, _| {
                    let start = raw.functions[index].parameters.start;
                    owned_mut(&mut raw.parameters)[start] = ParamLocal::Int(IntLocalId(99));
                    value.parameters.call = vec![
                        HostCallParameter::Int(IntLocalId(99)),
                        HostCallParameter::Int(IntLocalId(1)),
                    ]
                    .into();
                },
                ContractError::Parameters,
            ),
            (
                |value, raw, index, _| {
                    let start = raw.functions[index].parameters.start;
                    owned_mut(&mut raw.parameters)[start + 1] = ParamLocal::Int(IntLocalId(0));
                    value.parameters.call = vec![
                        HostCallParameter::Int(IntLocalId(0)),
                        HostCallParameter::Int(IntLocalId(0)),
                    ]
                    .into();
                },
                ContractError::Parameters,
            ),
            (
                |_, raw, index, _| {
                    let shape = raw.functions[index].parameter_shapes[0];
                    owned_mut(&mut raw.functions)[index].captures = vec![ParamSlot {
                        local: ParamLocal::Int(IntLocalId(0)),
                        shape,
                    }]
                    .into();
                },
                ContractError::Captures,
            ),
        ];
        for (change, expected) in cases {
            let (program, mut values, nevers) = lowered(source, hosts());
            let common = &program.common;
            let mut raw = FunctionCatalog {
                families: common.function_parameters.families.clone(),
                functions: common.function_parameters.functions.to_vec().into(),
                parameters: common.function_parameters.parameters.to_vec().into(),
            };
            let index = raw
                .functions
                .iter()
                .position(|value| value.parameters.len() == 2)
                .unwrap();
            let family_index = index - raw.families[FunctionTableFamily::Int as usize].start;
            let boolean = ValueShapeId(
                common
                    .value_shapes
                    .shapes
                    .iter()
                    .position(|shape| {
                        shape == &crate::plan::execution::type_::ValueShapeDescriptor::Bool
                    })
                    .unwrap(),
            );
            change(&mut values[0], &mut raw, index, boolean);
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog = Catalog::admit(&raw, &program.functions, &types).unwrap();
            let declaration = catalog
                .function(FunctionTableFamily::Int, family_index)
                .unwrap();
            let linked = NativeFunctions::new(&values, &nevers, hosts()).unwrap();
            let registration = &linked.registrations[linked.values[0].2];
            assert_eq!(metadata(&values[0], registration, &types, true), Ok(()));
            assert_eq!(
                call(&values[0], registration, &declaration, &types).as_ref(),
                Err(expected)
            );
        }
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
        let host = crate::execution_fixture::TestHost::default();
        assert_eq!(
            host.block_on(execution.run_main(&host, &mut (), &mut Vec::new()))
                .unwrap(),
            crate::Value::Int(42.into())
        );
    }
}
