use super::{
    HostAbiType, HostCustomSchemaId, HostCustomTypeSchema, HostSchemaType, HostTypeDescriptor,
    private,
};
use crate::host::HostScopedValue;
use std::collections::HashSet;
use std::marker::PhantomData;

/// One element followed by the remainder of a recursive host type sequence.
pub struct HostTypeList<Head, Tail>(PhantomData<(Head, Tail)>);

/// The end of a recursive host type sequence.
pub struct HostTypeListEnd;

/// Ordered composition used by generated construction registrations.
#[doc(hidden)]
pub struct HostTypeBranch<Left, Right>(PhantomData<fn() -> (Left, Right)>);

/// The first position in a recursive host type sequence.
pub struct HostTypeIndex0;

/// A position after `Index` in a recursive host type sequence.
pub struct HostTypeIndexNext<Index>(PhantomData<Index>);

/// A sealed recursive sequence of scoped host ABI types.
#[allow(private_bounds)]
pub trait HostTypeSequence: private::Sequence + Send + Sync + 'static {
    type Values<'call>: Clone;

    /// Appends construction registrations without flattening their proof tree.
    #[doc(hidden)]
    type Joined<Tail: HostTypeSequence>: HostTypeSequence;
}

pub(crate) trait HostAbiTypeSequence: HostTypeSequence {
    fn descriptors() -> Vec<HostTypeDescriptor> {
        <Self as private::Sequence>::descriptors()
    }

    fn schema_types() -> Vec<HostSchemaType> {
        <Self as private::Sequence>::schema_types()
    }

    fn callable_constructions() -> Vec<crate::host::RegisteredCallableConstruction> {
        let mut callables = Vec::with_capacity(<Self as private::Sequence>::CALLABLE_COUNT);
        <Self as private::Sequence>::collect_callable_constructions(&mut callables);
        callables
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomSchemaId>,
    ) {
        <Self as private::Sequence>::collect_custom_schemas(output, visited);
    }

    fn into_scoped_values(values: Self::Values<'_>, output: &mut Vec<HostScopedValue>) {
        <Self as private::Sequence>::into_scoped_values(values, output);
    }
}

/// Selects one type from a recursive host type sequence by position.
#[allow(private_bounds)]
pub trait HostTypeAt<Index>: HostTypeSequence + private::ConstructionPosition<Index> {
    type Type: super::HostType;
}

impl<Head, Tail> HostTypeAt<HostTypeIndex0> for HostTypeList<Head, Tail>
where
    Head: HostAbiType,
    Tail: HostTypeSequence,
{
    type Type = Head;
}

impl<Head, Tail, Index> HostTypeAt<HostTypeIndexNext<Index>> for HostTypeList<Head, Tail>
where
    Head: HostAbiType,
    Tail: HostTypeAt<Index>,
{
    type Type = <Tail as HostTypeAt<Index>>::Type;
}

impl<Head: HostAbiType, Tail: HostTypeSequence> private::ConstructionPosition<HostTypeIndex0>
    for HostTypeList<Head, Tail>
{
    const CALLABLE_INDEX: usize = 0;
}

impl<Head: HostAbiType, Tail: HostTypeAt<Index>, Index>
    private::ConstructionPosition<HostTypeIndexNext<Index>> for HostTypeList<Head, Tail>
{
    const CALLABLE_INDEX: usize = <Head as private::Abi>::CALLABLE_CONSTRUCTION
        + <Tail as private::ConstructionPosition<Index>>::CALLABLE_INDEX;
}

impl<Types: HostTypeSequence> HostAbiTypeSequence for Types {}

impl HostTypeSequence for HostTypeListEnd {
    type Values<'call> = ();
    type Joined<Tail: HostTypeSequence> = Tail;
}

impl private::Sequence for HostTypeListEnd {
    const CALLABLE_COUNT: usize = 0;
    fn descriptors() -> Vec<HostTypeDescriptor> {
        Vec::new()
    }

    fn schema_types() -> Vec<HostSchemaType> {
        Vec::new()
    }

    fn collect_callable_constructions(_: &mut Vec<crate::host::RegisteredCallableConstruction>) {}

    fn collect_custom_schemas(
        _output: &mut Vec<HostCustomTypeSchema>,
        _visited: &mut HashSet<HostCustomSchemaId>,
    ) {
    }

    fn into_scoped_values(
        (): <Self as HostTypeSequence>::Values<'_>,
        _output: &mut Vec<HostScopedValue>,
    ) {
    }

    fn from_tokens<'call, Runtime: crate::host::HostTokenRuntime + ?Sized>(
        _runtime: &Runtime,
        _tokens: &[crate::host::HostValueToken],
        _index: &mut usize,
    ) -> <Self as HostTypeSequence>::Values<'call> {
    }
}

impl<Head, Tail> HostTypeSequence for HostTypeList<Head, Tail>
where
    Head: HostAbiType,
    Tail: HostTypeSequence,
{
    type Values<'call> = (Head::Value<'call>, Tail::Values<'call>);
    type Joined<End: HostTypeSequence> = HostTypeBranch<Self, End>;
}

impl<Left: HostTypeSequence, Right: HostTypeSequence> HostTypeSequence
    for HostTypeBranch<Left, Right>
{
    type Values<'call> = (Left::Values<'call>, Right::Values<'call>);
    type Joined<Tail: HostTypeSequence> = HostTypeBranch<Self, Tail>;
}

impl<Left: HostTypeSequence, Right: HostTypeSequence> private::Sequence
    for HostTypeBranch<Left, Right>
{
    const CALLABLE_COUNT: usize = Left::CALLABLE_COUNT + Right::CALLABLE_COUNT;

    fn descriptors() -> Vec<HostTypeDescriptor> {
        let mut types = <Left as private::Sequence>::descriptors();
        types.extend(<Right as private::Sequence>::descriptors());
        types
    }

    fn schema_types() -> Vec<HostSchemaType> {
        let mut types = <Left as private::Sequence>::schema_types();
        types.extend(<Right as private::Sequence>::schema_types());
        types
    }

    fn collect_callable_constructions(
        output: &mut Vec<crate::host::RegisteredCallableConstruction>,
    ) {
        <Left as private::Sequence>::collect_callable_constructions(output);
        <Right as private::Sequence>::collect_callable_constructions(output);
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomSchemaId>,
    ) {
        <Left as private::Sequence>::collect_custom_schemas(output, visited);
        <Right as private::Sequence>::collect_custom_schemas(output, visited);
    }

    fn into_scoped_values(
        (left, right): <Self as HostTypeSequence>::Values<'_>,
        output: &mut Vec<HostScopedValue>,
    ) {
        <Left as private::Sequence>::into_scoped_values(left, output);
        <Right as private::Sequence>::into_scoped_values(right, output);
    }

    fn from_tokens<'call, Runtime: crate::host::HostTokenRuntime + ?Sized>(
        runtime: &Runtime,
        tokens: &[crate::host::HostValueToken],
        index: &mut usize,
    ) -> <Self as HostTypeSequence>::Values<'call> {
        let left = <Left as private::Sequence>::from_tokens(runtime, tokens, index);
        let right = <Right as private::Sequence>::from_tokens(runtime, tokens, index);
        (left, right)
    }
}

impl<Head, Tail> private::Sequence for HostTypeList<Head, Tail>
where
    Head: HostAbiType,
    Tail: HostAbiTypeSequence,
{
    const CALLABLE_COUNT: usize =
        <Head as private::Abi>::CALLABLE_CONSTRUCTION + <Tail as private::Sequence>::CALLABLE_COUNT;

    fn descriptors() -> Vec<HostTypeDescriptor> {
        let mut types = vec![<Head as HostAbiType>::descriptor()];
        types.extend(<Tail as HostAbiTypeSequence>::descriptors());
        types
    }

    fn schema_types() -> Vec<HostSchemaType> {
        let mut types = vec![<Head as HostAbiType>::schema_type()];
        types.extend(<Tail as HostAbiTypeSequence>::schema_types());
        types
    }

    fn collect_callable_constructions(
        output: &mut Vec<crate::host::RegisteredCallableConstruction>,
    ) {
        <Head as private::Abi>::collect_callable_constructions(output);
        <Tail as private::Sequence>::collect_callable_constructions(output);
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomSchemaId>,
    ) {
        <Head as HostAbiType>::collect_custom_schemas(output, visited);
        <Tail as HostAbiTypeSequence>::collect_custom_schemas(output, visited);
    }

    fn into_scoped_values(
        (head, tail): <Self as HostTypeSequence>::Values<'_>,
        output: &mut Vec<HostScopedValue>,
    ) {
        output.push(<Head as HostAbiType>::into_scoped(head));
        <Tail as HostAbiTypeSequence>::into_scoped_values(tail, output);
    }

    fn from_tokens<'call, Runtime: crate::host::HostTokenRuntime + ?Sized>(
        runtime: &Runtime,
        tokens: &[crate::host::HostValueToken],
        index: &mut usize,
    ) -> <Self as HostTypeSequence>::Values<'call> {
        let head = <Head as private::Abi>::from_token(runtime, tokens[*index]);
        *index += 1;
        let tail = <Tail as private::Sequence>::from_tokens(runtime, tokens, index);
        (head, tail)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        HostAbiTypeSequence, HostTypeAt, HostTypeBranch, HostTypeIndex0, HostTypeIndexNext,
        HostTypeList, HostTypeListEnd, HostTypeSequence,
    };
    use crate::host::function::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostSchemaType, HostScopedValue, HostTypeDescriptor, HostValueFamily, HostValueToken,
    };
    use num_bigint::BigInt;

    #[test]
    fn recursive_type_sequence_preserves_descriptor_value_and_token_order() {
        type Types = HostTypeList<BigInt, HostTypeList<bool, HostTypeListEnd>>;

        assert_eq!(
            <Types as HostAbiTypeSequence>::descriptors(),
            [HostTypeDescriptor::Int, HostTypeDescriptor::Bool],
        );
        assert_eq!(
            <Types as HostAbiTypeSequence>::schema_types(),
            [HostSchemaType::Int, HostSchemaType::Bool],
        );

        let mut scoped = Vec::new();
        <Types as HostAbiTypeSequence>::into_scoped_values(
            (BigInt::from(2), (true, ())),
            &mut scoped,
        );
        assert_eq!(
            scoped,
            [
                HostScopedValue::Int(BigInt::from(2)),
                HostScopedValue::Bool(true),
            ],
        );

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let runtime = TestHostCallRuntime::new(&mut state, arguments);
        let tokens = [
            HostValueToken {
                family: HostValueFamily::Int,
                index: 0,
            },
            HostValueToken {
                family: HostValueFamily::Bool,
                index: 0,
            },
        ];
        assert_eq!(
            crate::host::type_::from_tokens::<Types, TestHostProfile>(&runtime, &tokens),
            (BigInt::from(0), (false, ())),
        );
    }

    #[test]
    fn recursive_type_sequence_selects_types_by_stable_position() {
        type Types = HostTypeList<BigInt, HostTypeList<bool, HostTypeListEnd>>;

        let first: <Types as HostTypeAt<HostTypeIndex0>>::Type = BigInt::from(7);
        let second: <Types as HostTypeAt<HostTypeIndexNext<HostTypeIndex0>>>::Type = true;

        assert_eq!(first, BigInt::from(7));
        assert!(second);
    }

    #[test]
    fn joined_type_sequences_preserve_descriptor_value_and_token_order() {
        type Left = HostTypeList<BigInt, HostTypeListEnd>;
        type Right = HostTypeList<bool, HostTypeListEnd>;
        type Types = <Left as HostTypeSequence>::Joined<Right>;
        type Identity = <HostTypeListEnd as HostTypeSequence>::Joined<Right>;
        let identity: <Identity as HostTypeSequence>::Values<'_> = (true, ());
        assert!(identity.0);
        assert_eq!(
            <Types as HostAbiTypeSequence>::descriptors(),
            [HostTypeDescriptor::Int, HostTypeDescriptor::Bool]
        );
        assert_eq!(
            <Types as HostAbiTypeSequence>::schema_types(),
            [HostSchemaType::Int, HostSchemaType::Bool]
        );
        let mut scoped = Vec::new();
        <Types as HostAbiTypeSequence>::into_scoped_values(
            ((2.into(), ()), (true, ())),
            &mut scoped,
        );
        assert_eq!(
            scoped,
            [HostScopedValue::Int(2.into()), HostScopedValue::Bool(true)]
        );
        let mut state = TestRunState::default();
        let runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
        let tokens = [
            HostValueToken {
                family: HostValueFamily::Int,
                index: 0,
            },
            HostValueToken {
                family: HostValueFamily::Bool,
                index: 0,
            },
        ];
        assert_eq!(
            crate::host::type_::from_tokens::<Types, TestHostProfile>(&runtime, &tokens),
            ((BigInt::from(0), ()), (false, ()))
        );
        type Joined = <HostTypeBranch<Left, Right> as HostTypeSequence>::Joined<Left>;
        assert_eq!(
            <Joined as HostAbiTypeSequence>::descriptors(),
            [
                HostTypeDescriptor::Int,
                HostTypeDescriptor::Bool,
                HostTypeDescriptor::Int
            ]
        );
    }

    #[test]
    fn joined_sequences_collect_custom_schemas_and_callable_captures_in_order() {
        use crate::host::{
            HostCallableSchema, HostCreatedFunction, HostCustomConstructorDefinition,
            HostCustomConstructorLeaf, HostCustomFieldListEnd, HostCustomSchema, HostCustomType,
            HostCustomTypeSchema, HostReturns, HostSchemaType,
        };
        use std::collections::HashSet;
        struct Schema;
        struct Constructor;
        impl HostCustomSchema for Schema {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "main";
            const NAME: &'static str = "Item";
            const PARAMETER_COUNT: usize = 0;
            type Constructors = HostCustomConstructorLeaf<Constructor>;
        }
        impl HostCustomConstructorDefinition for Constructor {
            const NAME: &'static str = "Item";
            type Fields = HostCustomFieldListEnd;
        }
        struct Body;
        impl HostCallableSchema for Body {
            const PACKAGE: &'static str = "application";
            const MODULE: &'static str = "main";
            const NAME: &'static str = "capture";
            type Arguments = HostTypeListEnd;
            type Return = BigInt;
            type Captures = HostTypeList<HostCustomType<Schema>, HostTypeListEnd>;
            type Constructions = HostTypeListEnd;
            type Completion = HostReturns;
        }
        type Types = HostTypeBranch<
            HostTypeList<HostCustomType<Schema>, HostTypeListEnd>,
            HostTypeList<HostCreatedFunction<Body>, HostTypeListEnd>,
        >;
        let custom = HostCustomTypeSchema::of::<Schema>();
        assert_eq!(
            <Types as HostAbiTypeSequence>::descriptors(),
            [
                HostTypeDescriptor::Custom {
                    schema: custom.clone(),
                    arguments: Box::new([])
                },
                HostTypeDescriptor::Function {
                    arguments: Box::new([]),
                    return_: Box::new(HostTypeDescriptor::Int)
                },
            ]
        );
        assert_eq!(
            <Types as HostAbiTypeSequence>::schema_types(),
            [
                HostSchemaType::Custom {
                    package: "application".into(),
                    module: "main".into(),
                    name: "Item".into(),
                    arguments: Box::new([])
                },
                HostSchemaType::Function {
                    arguments: Box::new([]),
                    return_: Box::new(HostSchemaType::Int)
                },
            ]
        );
        let mut schemas = Vec::new();
        let mut visited = HashSet::new();
        <Types as HostAbiTypeSequence>::collect_custom_schemas(&mut schemas, &mut visited);
        assert_eq!(schemas, [custom]);
        assert_eq!(visited.len(), 1);
        let callables = <Types as HostAbiTypeSequence>::callable_constructions();
        assert_eq!(callables.len(), 1);
        assert_eq!(callables[0].identity.package, "application");
        assert_eq!(callables[0].identity.module, "main");
        assert_eq!(callables[0].identity.name, "capture");
        assert_eq!(
            callables[0].captures.as_ref(),
            [HostTypeDescriptor::Custom {
                schema: schemas[0].clone(),
                arguments: Box::new([])
            }]
        );
        assert_eq!(callables[0].arguments.as_ref(), []);
        assert_eq!(callables[0].return_, HostTypeDescriptor::Int);
    }
}
