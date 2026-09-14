use super::{ContractError, Registration, Types};
use crate::plan::ValueType as Nominal;
use crate::plan::execution::host::{
    HostedFunctionMetadata, NativeConversion, NativeConversionId, NativeConversionKind,
};
use crate::plan::execution::type_::{CustomTypeId, TypeMetadata, ValueType};
use std::collections::HashSet;

pub(super) fn admit(
    metadata: &HostedFunctionMetadata,
    registration: &Registration,
    arguments: &[Nominal],
    expanded: &HashSet<CustomTypeId>,
    types: &Types<'_>,
) -> Result<(), ContractError> {
    let conversions = &metadata.constructions.natives;
    let Some(rules) = registration.constructions.native_rules() else {
        return if conversions.roots.is_empty() && conversions.nodes.is_empty() {
            Ok(())
        } else {
            Err(ContractError::Native)
        };
    };
    let resolve = |descriptor: &crate::host::HostTypeDescriptor| {
        descriptor.resolve_sealed(&|index| arguments[index].clone())
    };
    let rules = rules.iter().map(resolve).collect::<Vec<_>>();
    let mut rule_types = HashSet::new();
    if rules.iter().any(|rule| !rule_types.insert(rule)) {
        return Err(ContractError::Native);
    }
    if conversions.roots.len() != registration.constructions.types().len() {
        return Err(ContractError::Native);
    }
    let mut node_types = HashSet::new();
    for node in conversions.nodes.iter() {
        types.metadata(&node.type_).map_err(ContractError::Type)?;
        if !node_types.insert(&node.type_) {
            return Err(ContractError::Native);
        }
    }
    for (root, descriptor) in conversions
        .roots
        .iter()
        .zip(registration.constructions.types())
    {
        if !node(&conversions.nodes, *root)?
            .type_
            .compare(&resolve(descriptor))
            .is_eq()
        {
            return Err(ContractError::Native);
        }
    }
    for conversion in conversions.nodes.iter() {
        let rule = rules
            .iter()
            .position(|rule| conversion.type_.compare(rule).is_eq());
        if let Some(rule) = rule {
            if !matches!(conversion.kind, NativeConversionKind::External { rule: stored } if rule == stored)
            {
                return Err(ContractError::Native);
            }
            continue;
        }
        match (&conversion.type_, &conversion.kind) {
            (TypeMetadata::Int, NativeConversionKind::Int)
            | (TypeMetadata::Float, NativeConversionKind::Float)
            | (TypeMetadata::String, NativeConversionKind::String)
            | (TypeMetadata::BitArray, NativeConversionKind::BitArray)
            | (TypeMetadata::UtfCodepoint, NativeConversionKind::UtfCodepoint)
            | (TypeMetadata::Bool, NativeConversionKind::Bool)
            | (TypeMetadata::Nil, NativeConversionKind::Nil)
            | (
                TypeMetadata::Parameter(_) | TypeMetadata::Function(_) | TypeMetadata::External(_),
                NativeConversionKind::Exact,
            ) => {}
            (TypeMetadata::Tuple(items), NativeConversionKind::Tuple(children)) => {
                if children.len() != items.len() {
                    return Err(ContractError::Native);
                }
                for (child, item) in children.iter().zip(items.iter()) {
                    if &node(&conversions.nodes, *child)?.type_ != item {
                        return Err(ContractError::Native);
                    }
                }
            }
            (
                TypeMetadata::List(item),
                NativeConversionKind::List {
                    storage,
                    item: child,
                },
            ) => {
                if !types
                    .matches(&conversion.type_, &ValueType::List(*storage))
                    .map_err(ContractError::Type)?
                    || &node(&conversions.nodes, *child)?.type_ != item.as_ref()
                {
                    return Err(ContractError::Native);
                }
            }
            (TypeMetadata::Custom(nominal), kind) => {
                let construction = metadata.constructions.customs.entries.iter().find(
                    |(type_, _)| matches!(type_, TypeMetadata::Custom(type_) if type_ == nominal),
                );
                let expanded_id = construction
                    .map(|(_, id)| *id)
                    .filter(|id| expanded.contains(id));
                let Some(id) = expanded_id else {
                    if !matches!(kind, NativeConversionKind::Exact) {
                        return Err(ContractError::Native);
                    }
                    continue;
                };
                let NativeConversionKind::Custom(constructors) = kind else {
                    return Err(ContractError::Native);
                };
                let declaration = &types.customs.types[id.index()];
                if constructors.len() != declaration.constructors.len() {
                    return Err(ContractError::Native);
                }
                for (constructor, declaration) in
                    constructors.iter().zip(declaration.constructors.iter())
                {
                    if constructor.constructor != declaration.id
                        || constructor.tag != declaration.native_tag
                        || constructor.fields.len() != declaration.fields.len()
                    {
                        return Err(ContractError::Native);
                    }
                    for (field, declaration) in
                        constructor.fields.iter().zip(declaration.fields.iter())
                    {
                        if !types.metadata_matches_value(
                            &node(&conversions.nodes, *field)?.type_,
                            &declaration.type_,
                        ) {
                            return Err(ContractError::Native);
                        }
                    }
                }
            }
            _ => return Err(ContractError::Native),
        }
    }
    let mut seen = HashSet::new();
    let mut pending = conversions.roots.iter().copied().collect::<Vec<_>>();
    while let Some(id) = pending.pop() {
        if !seen.insert(id.0) {
            continue;
        }
        match &conversions.nodes[id.0].kind {
            NativeConversionKind::Tuple(children) => pending.extend(children.iter().copied()),
            NativeConversionKind::List { item, .. } => pending.push(*item),
            NativeConversionKind::Custom(constructors) => {
                for constructor in constructors.iter() {
                    pending.extend(constructor.fields.iter().copied());
                }
            }
            NativeConversionKind::Exact
            | NativeConversionKind::Int
            | NativeConversionKind::Float
            | NativeConversionKind::String
            | NativeConversionKind::BitArray
            | NativeConversionKind::UtfCodepoint
            | NativeConversionKind::Bool
            | NativeConversionKind::Nil
            | NativeConversionKind::External { .. } => {}
        }
    }
    if seen.len() != conversions.nodes.len() {
        return Err(ContractError::Native);
    }
    Ok(())
}

fn node(
    nodes: &[NativeConversion],
    id: NativeConversionId,
) -> Result<&NativeConversion, ContractError> {
    nodes.get(id.0).ok_or(ContractError::Native)
}

#[cfg(test)]
mod tests {
    use super::{
        ContractError, HashSet, NativeConversion, NativeConversionId, NativeConversionKind,
        Registration, TypeMetadata, Types, admit,
    };
    use crate::host::native::{NativeCall, NativeRules};
    use crate::plan::execution::host::NativeConversions;
    use crate::plan::execution::prepared::admission::hosts::{NativeFunctions, tests::lowered};
    use crate::plan::execution::prepared::admission::type_::TypeError;
    use crate::plan::execution::storage::{Node, Table};
    use crate::{
        HostCallCompletion, HostCallError, HostProvider, HostProviderModule, HostProviderSet,
        HostTypeList, HostTypeListEnd, HostTypeParameter, HostValue, StatelessHostProfile,
    };

    struct Provider;
    impl HostProvider<StatelessHostProfile> for Provider {
        type State = ();
        fn project(state: &mut ()) -> &mut () {
            state
        }
    }

    #[test]
    fn rejects_wrong_roots_edges_types_and_unclaimed_native_nodes_without_execution() {
        type Item = HostTypeParameter<0>;
        type Targets = HostTypeList<Item, HostTypeListEnd>;
        fn check<'call>(
            call: NativeCall<'call, StatelessHostProfile, Provider, bool, Targets>,
            _value: HostValue<'call, Item>,
        ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
            Ok(call.finish(true))
        }
        let hosts = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_native_function::<Provider, (Item,), bool, Targets, _>(
                    "check",
                    NativeRules::default(),
                    check,
                )
                .unwrap()])
            .unwrap()
        };
        let source = r#"
@external(erlang, "native", "check")
fn check(value: a) -> Bool
pub fn main() { check(#(1, "text", [2], True, Nil, 1.25, <<1>>)) }
"#;
        let (program, mut values, nevers) = lowered(source, hosts());
        let registration = NativeFunctions::new(&values, &nevers, hosts())
            .unwrap()
            .registrations
            .pop()
            .unwrap();
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let metadata = &mut values[0];
        let arguments = metadata
            .type_arguments
            .iter()
            .map(|argument| argument.type_.materialize())
            .collect::<Vec<_>>();
        let expanded = HashSet::new();
        assert_eq!(
            admit(metadata, &registration, &arguments, &expanded, &types),
            Ok(())
        );
        assert_eq!(
            metadata
                .constructions
                .natives
                .roots
                .iter()
                .map(|id| id.0)
                .collect::<Vec<_>>(),
            [0]
        );
        assert_eq!(metadata.constructions.natives.nodes.len(), 8);
        assert_eq!(
            metadata.constructions.natives.nodes[1].type_,
            TypeMetadata::Int
        );
        assert_eq!(
            metadata.constructions.natives.nodes[3].type_,
            TypeMetadata::List(Node::Static(&TypeMetadata::Int))
        );
        let original = metadata.constructions.natives.clone();
        let cases: [fn(&mut NativeConversions); 13] = [
            |graph| graph.roots = Table::Static(&[]),
            |graph| graph.roots = vec![NativeConversionId(99)].into(),
            |graph| graph.roots = vec![NativeConversionId(1)].into(),
            |graph| {
                let mut nodes = graph.nodes.clone().into_vec();
                nodes.push(nodes[1].clone());
                graph.nodes = nodes.into();
            },
            |graph| {
                let mut nodes = graph.nodes.clone().into_vec();
                nodes.push(NativeConversion {
                    type_: TypeMetadata::Parameter(crate::plan::TypeParameterId(99)),
                    kind: NativeConversionKind::Exact,
                });
                graph.nodes = nodes.into();
            },
            |graph| {
                let mut nodes = graph.nodes.clone().into_vec();
                nodes[1].kind = NativeConversionKind::Bool;
                graph.nodes = nodes.into();
            },
            |graph| {
                let mut nodes = graph.nodes.clone().into_vec();
                nodes[0].kind = NativeConversionKind::Tuple(Table::Static(&[]));
                graph.nodes = nodes.into();
            },
            |graph| {
                let mut nodes = graph.nodes.clone().into_vec();
                nodes[0].kind = NativeConversionKind::Tuple(
                    [2, 2, 3, 4, 5, 6, 7]
                        .map(NativeConversionId)
                        .to_vec()
                        .into(),
                );
                graph.nodes = nodes.into();
            },
            |graph| {
                let mut nodes = graph.nodes.clone().into_vec();
                nodes[0].kind = NativeConversionKind::Tuple(
                    [99, 2, 3, 4, 5, 6, 7]
                        .map(NativeConversionId)
                        .to_vec()
                        .into(),
                );
                graph.nodes = nodes.into();
            },
            |graph| {
                let mut nodes = graph.nodes.clone().into_vec();
                nodes[3].kind = NativeConversionKind::List {
                    storage: crate::plan::execution::type_::ListTypeId(0),
                    item: NativeConversionId(2),
                };
                graph.nodes = nodes.into();
            },
            |graph| {
                let mut nodes = graph.nodes.clone().into_vec();
                nodes[3].kind = NativeConversionKind::List {
                    storage: crate::plan::execution::type_::ListTypeId(99),
                    item: NativeConversionId(1),
                };
                graph.nodes = nodes.into();
            },
            |graph| {
                let mut nodes = graph.nodes.clone().into_vec();
                static CYCLE: TypeMetadata = TypeMetadata::List(Node::Static(&CYCLE));
                nodes[1].type_ = CYCLE.clone();
                graph.nodes = nodes.into();
            },
            |graph| {
                let mut nodes = graph.nodes.clone().into_vec();
                nodes[3].kind = NativeConversionKind::List {
                    storage: crate::plan::execution::type_::ListTypeId(0),
                    item: NativeConversionId(99),
                };
                graph.nodes = nodes.into();
            },
        ];
        for (index, mutate) in cases.into_iter().enumerate() {
            metadata.constructions.natives = original.clone();
            mutate(&mut metadata.constructions.natives);
            let expected = match index {
                10 => ContractError::Type(TypeError::MissingList { index: 99 }),
                11 => ContractError::Type(TypeError::RecursiveMetadata),
                _ => ContractError::Native,
            };
            assert_eq!(
                admit(metadata, &registration, &arguments, &expanded, &types),
                Err(expected),
                "mutation {index}"
            );
        }
        let registration = Registration {
            schema: registration.schema,
            constructions: crate::host::RegisteredHostConstructions::empty(),
        };
        metadata.constructions.natives = NativeConversions::default();
        assert_eq!(
            admit(metadata, &registration, &arguments, &expanded, &types),
            Ok(())
        );
        metadata.constructions.natives = original;
        assert_eq!(
            admit(metadata, &registration, &arguments, &expanded, &types),
            Err(ContractError::Native)
        );
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
            crate::Value::Bool(true)
        );
        assert_eq!(
            <Provider as HostProvider<StatelessHostProfile>>::project(&mut ()),
            &()
        );
    }

    #[test]
    fn recursive_custom_conversions_preserve_declared_constructors_tags_and_fields() {
        use crate::host::{
            HostCustomConstructorDefinition, HostCustomConstructorList,
            HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList,
            HostCustomFieldListEnd, HostCustomSchema, HostCustomType, HostCustomTypeArgument,
            HostListType, HostTypeIndex0,
        };
        use crate::plan::execution::type_::CustomTypeId;
        struct Tree;
        struct Leaf;
        struct Branch;
        struct Item;
        struct Children;
        impl HostCustomSchema for Tree {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "main";
            const NAME: &'static str = "Tree";
            const PARAMETER_COUNT: usize = 1;
            type Constructors = HostCustomConstructorList<
                Leaf,
                HostCustomConstructorList<Branch, HostCustomConstructorListEnd>,
            >;
        }
        impl HostCustomConstructorDefinition for Leaf {
            const NAME: &'static str = "Leaf";
            type Fields = HostCustomFieldList<Item, HostCustomFieldListEnd>;
        }
        impl HostCustomConstructorDefinition for Branch {
            const NAME: &'static str = "Branch";
            type Fields = HostCustomFieldList<Children, HostCustomFieldListEnd>;
        }
        impl HostCustomField for Item {
            const LABEL: Option<&'static str> = Some("value");
            type Type = HostCustomTypeArgument<HostTypeIndex0>;
        }
        impl HostCustomField for Children {
            const LABEL: Option<&'static str> = None;
            type Type = HostListType<
                HostCustomType<
                    Tree,
                    HostTypeList<HostCustomTypeArgument<HostTypeIndex0>, HostTypeListEnd>,
                >,
            >;
        }
        type Input = HostTypeParameter<0>;
        type Targets = HostTypeList<
            HostCustomType<Tree, HostTypeList<Input, HostTypeListEnd>>,
            HostTypeListEnd,
        >;
        fn ready<'call>(
            call: NativeCall<'call, StatelessHostProfile, Provider, bool, Targets>,
            _value: HostValue<'call, Input>,
        ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
            Ok(call.finish(true))
        }
        let hosts = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_native_function::<Provider, (Input,), bool, Targets, _>(
                    "ready",
                    NativeRules::default(),
                    ready,
                )
                .unwrap()])
            .unwrap()
        };
        let source = r#"
pub type Tree(a) { Leaf(value: a) Branch(List(Tree(a))) }
@external(erlang, "native", "ready")
fn ready(value: a) -> Bool
pub fn main() { ready(42) }
"#;
        let (program, mut values, nevers) = lowered(source, hosts());
        let registration = NativeFunctions::new(&values, &nevers, hosts())
            .unwrap()
            .registrations
            .pop()
            .unwrap();
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let metadata = &mut values[0];
        let arguments = [crate::plan::ValueType::Int];
        let expanded = HashSet::from([CustomTypeId(0)]);
        assert_eq!(
            super::super::admit(metadata, &registration, &arguments, &types),
            Ok(())
        );
        assert_eq!(
            admit(metadata, &registration, &arguments, &expanded, &types),
            Ok(())
        );
        assert_eq!(metadata.constructions.natives.nodes.len(), 3);
        let root = metadata.constructions.natives.roots[0].0;
        let original = metadata.constructions.natives.clone();
        let constructor_tables = original
            .nodes
            .iter()
            .filter_map(|node| match &node.kind {
                NativeConversionKind::Custom(constructors) => Some(constructors.clone()),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(constructor_tables.len(), 1);
        let constructors = &constructor_tables[0];
        assert_eq!(constructors.len(), 2);
        assert_eq!(constructors[0].tag.as_str(), "leaf");
        assert_eq!(constructors[1].tag.as_str(), "branch");
        let integer = constructors[0].fields[0];
        let list = constructors[1].fields[0];
        assert_eq!(original.nodes[integer.0].type_, TypeMetadata::Int);
        assert_eq!(
            original.nodes[list.0].type_,
            TypeMetadata::List(Box::new(original.nodes[root].type_.clone()).into())
        );
        for change in 0..7 {
            let mut changed = constructors.clone().into_vec();
            match change {
                0 => {
                    changed.pop();
                }
                1 => changed[0].constructor.index = 1,
                2 => changed[0].tag = "wrong".into(),
                3 => changed[0].fields = Table::Static(&[]),
                4 => changed[0].fields = vec![list].into(),
                5 => changed[0].fields = vec![NativeConversionId(99)].into(),
                _ => {}
            }
            let mut nodes = original.nodes.clone().into_vec();
            nodes[root].kind = if change == 6 {
                NativeConversionKind::Exact
            } else {
                NativeConversionKind::Custom(changed.into())
            };
            metadata.constructions.natives.nodes = nodes.into();
            assert_eq!(
                admit(metadata, &registration, &arguments, &expanded, &types),
                Err(ContractError::Native),
                "change {change}"
            );
        }
        metadata.constructions.natives = original;
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
            crate::Value::Bool(true)
        );
    }

    #[test]
    fn unknown_customs_and_callable_values_keep_exact_native_identity() {
        type Input = HostTypeParameter<0>;
        type Targets = HostTypeList<Input, HostTypeListEnd>;
        fn ready<'call>(
            call: NativeCall<'call, StatelessHostProfile, Provider, bool, Targets>,
            _value: HostValue<'call, Input>,
        ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
            Ok(call.finish(true))
        }
        let hosts = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_native_function::<Provider, (Input,), bool, Targets, _>(
                    "ready",
                    NativeRules::default(),
                    ready,
                )
                .unwrap()])
            .unwrap()
        };
        let source = r#"
pub type Unknown { Unknown(Int) }
@external(erlang, "native", "ready")
fn ready(value: a) -> Bool
pub fn main() {
  let assert <<point:utf8_codepoint>> = <<65>>
  ready(#(Unknown(42), fn(value: Int) { value + 1 }, point))
}
"#;
        let (program, mut values, nevers) = lowered(source, hosts());
        let registration = NativeFunctions::new(&values, &nevers, hosts())
            .unwrap()
            .registrations
            .pop()
            .unwrap();
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let metadata = &mut values[0];
        let arguments = metadata
            .type_arguments
            .iter()
            .map(|argument| argument.type_.materialize())
            .collect::<Vec<_>>();
        assert_eq!(
            admit(metadata, &registration, &arguments, &HashSet::new(), &types),
            Ok(())
        );
        let original = metadata.constructions.natives.clone();
        let custom = original
            .nodes
            .iter()
            .position(|node| matches!(node.type_, TypeMetadata::Custom(_)))
            .unwrap();
        let mut nodes = original.nodes.clone().into_vec();
        nodes[custom].kind = NativeConversionKind::Custom(Table::Static(&[]));
        metadata.constructions.natives.nodes = nodes.into();
        assert_eq!(
            admit(metadata, &registration, &arguments, &HashSet::new(), &types),
            Err(ContractError::Native)
        );
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
            crate::Value::Bool(true)
        );
    }

    #[test]
    fn external_conversions_keep_the_linked_rule_index_for_each_specialization() {
        use crate::host::{
            HostExternalBinding, HostExternalEquality, HostExternalHashing, HostExternalInspection,
            HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType,
            HostProfile, HostTypeIndex0, HostTypeIndexNext,
        };
        struct Profile;
        struct Native;
        struct Envelope;
        impl HostProfile for Profile {
            type RunState = ();
            type ExternalStores = HostExternalStore<u8>;
            type ExecutionState = ();
        }
        impl HostProvider<Profile> for Native {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        impl HostExternalSchema for Envelope {
            const PACKAGE: &'static str = "app";
            const MODULE: &'static str = "main";
            const NAME: &'static str = "Envelope";
            const PARAMETER_COUNT: usize = 1;
        }
        impl HostExternalBinding<Profile, Envelope> for Native {
            type Storage = Self;
        }
        impl HostExternalStorage<Profile, Envelope> for Native {
            type Payload = u8;
            fn store(stores: &HostExternalStore<u8>) -> &HostExternalStore<u8> {
                stores
            }
            fn source_equal(_: &HostExternalEquality<'_>, left: &u8, right: &u8) -> bool {
                left == right
            }
            fn source_hash(_: &HostExternalHashing<'_>, value: &u8) -> u64 {
                u64::from(*value)
            }
            fn inspect(_: &HostExternalInspection<'_>, value: &u8) -> ecow::EcoString {
                format!("Envelope({value})").into()
            }
        }
        type Input = HostTypeParameter<0>;
        type Other = HostTypeParameter<1>;
        type Arguments = HostTypeList<Input, HostTypeListEnd>;
        type OtherArguments = HostTypeList<Other, HostTypeListEnd>;
        type Value = HostExternalType<Envelope, Arguments>;
        type OtherValue = HostExternalType<Envelope, OtherArguments>;
        type Targets = HostTypeList<Value, HostTypeList<OtherValue, HostTypeListEnd>>;
        fn ready<'call>(
            mut call: NativeCall<'call, Profile, Native, bool, Targets>,
            value: HostValue<'call, Input>,
            second: HostValue<'call, Other>,
        ) -> Result<HostCallCompletion<'call, bool>, HostCallError> {
            let source = call.source::<Input>(value);
            let converted = call.convert::<HostTypeIndex0>(&source).unwrap();
            let other = call.convert::<HostTypeIndex0>(&source).unwrap();
            let second = call.source::<Other>(second);
            let second = call
                .convert::<HostTypeIndexNext<HostTypeIndex0>>(&second)
                .unwrap();
            let host = call.call();
            assert_eq!(host.state(), &());
            assert!(host.equal::<Value>(converted, other));
            assert_eq!(
                host.source_hash::<Value>(converted),
                host.source_hash::<Value>(other)
            );
            assert_eq!(host.inspect::<Value>(converted), "Envelope(42)");
            assert_eq!(host.inspect::<OtherValue>(second), "Envelope(21)");
            Ok(call.finish(true))
        }
        let hosts = || {
            HostProviderSet::from_providers([HostProviderModule::new("app", "main")
                .unwrap()
                .with_external_type::<Native, Envelope>()
                .unwrap()
                .with_native_function::<Native, (Input, Other), bool, Targets, _>(
                    "ready",
                    NativeRules::default()
                        .external::<Envelope, Arguments>(|call, target, _| {
                            Some(call.construct_external(target, 42))
                        })
                        .external::<Envelope, OtherArguments>(|call, target, _| {
                            Some(call.construct_external(target, 21))
                        }),
                    ready,
                )
                .unwrap()])
            .unwrap()
        };
        let source = r#"
pub type Envelope(a)
@external(erlang, "native", "ready")
fn ready(value: a, other: b) -> Bool
pub fn main() { #(ready(42, "text"), ready("text", 42)) }
"#;
        let (program, mut values, nevers) = lowered(source, hosts());
        let registration = NativeFunctions::new(&values, &nevers, hosts())
            .unwrap()
            .registrations
            .pop()
            .unwrap();
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        assert_eq!(values.len(), 2);
        for metadata in &mut values {
            let arguments = metadata
                .type_arguments
                .iter()
                .map(|argument| argument.type_.materialize())
                .collect::<Vec<_>>();
            assert_eq!(
                admit(metadata, &registration, &arguments, &HashSet::new(), &types),
                Ok(())
            );
            assert_eq!(
                admit(
                    metadata,
                    &registration,
                    &[crate::plan::ValueType::Int, crate::plan::ValueType::Int],
                    &HashSet::new(),
                    &types
                ),
                Err(ContractError::Native)
            );
            assert_eq!(metadata.constructions.natives.nodes.len(), 2);
            let original = metadata.constructions.natives.nodes[0].clone();
            for kind in [
                NativeConversionKind::Exact,
                NativeConversionKind::External { rule: 1 },
            ] {
                let mut nodes = metadata.constructions.natives.nodes.to_vec();
                nodes[0] = NativeConversion {
                    type_: original.type_.clone(),
                    kind,
                };
                metadata.constructions.natives.nodes = nodes.into();
                assert_eq!(
                    admit(metadata, &registration, &arguments, &HashSet::new(), &types),
                    Err(ContractError::Native)
                );
            }
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
        assert_eq!(
            crate::execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap(),
            crate::Value::Tuple(vec![crate::Value::Bool(true), crate::Value::Bool(true)])
        );
    }
}
