use super::{
    HostAbiType, HostCustomIdentity, HostCustomTypeSchema, HostSchemaType, HostTypeDescriptor,
    private,
};
use crate::host::HostScopedValue;
use std::collections::HashSet;
use std::marker::PhantomData;

/// One element followed by the remainder of a recursive host type sequence.
pub struct HostTypeList<Head, Tail>(PhantomData<(Head, Tail)>);

/// The end of a recursive host type sequence.
pub struct HostTypeListEnd;

/// A sealed recursive sequence of scoped host ABI types.
#[allow(private_bounds)]
pub trait HostTypeSequence: private::Sequence + Send + Sync + 'static {
    type Values<'call>: Clone;
}

pub(crate) trait HostAbiTypeSequence: HostTypeSequence {
    fn descriptors() -> Vec<HostTypeDescriptor> {
        <Self as private::Sequence>::descriptors()
    }

    fn schema_types() -> Vec<HostSchemaType> {
        <Self as private::Sequence>::schema_types()
    }

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
    ) {
        <Self as private::Sequence>::collect_custom_schemas(output, visited);
    }

    fn into_scoped_values(values: Self::Values<'_>, output: &mut Vec<HostScopedValue>) {
        <Self as private::Sequence>::into_scoped_values(values, output);
    }
}

impl<Types: HostTypeSequence> HostAbiTypeSequence for Types {}

impl HostTypeSequence for HostTypeListEnd {
    type Values<'call> = ();
}

impl private::Sequence for HostTypeListEnd {
    fn descriptors() -> Vec<HostTypeDescriptor> {
        Vec::new()
    }

    fn schema_types() -> Vec<HostSchemaType> {
        Vec::new()
    }

    fn collect_custom_schemas(
        _output: &mut Vec<HostCustomTypeSchema>,
        _visited: &mut HashSet<HostCustomIdentity>,
    ) {
    }

    fn into_scoped_values(
        (): <Self as HostTypeSequence>::Values<'_>,
        _output: &mut Vec<HostScopedValue>,
    ) {
    }

    fn from_tokens<'call, Profile: crate::host::HostProfile>(
        _runtime: &dyn crate::host::HostCallRuntime<Profile>,
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
}

impl<Head, Tail> private::Sequence for HostTypeList<Head, Tail>
where
    Head: HostAbiType,
    Tail: HostAbiTypeSequence,
{
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

    fn collect_custom_schemas(
        output: &mut Vec<HostCustomTypeSchema>,
        visited: &mut HashSet<HostCustomIdentity>,
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

    fn from_tokens<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
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
    use super::{HostAbiTypeSequence, HostTypeList, HostTypeListEnd};
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
}
