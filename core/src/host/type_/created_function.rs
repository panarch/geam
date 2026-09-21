use super::{HostAbiType, HostFunctionType, HostType, private};
use crate::host::{HostCallableSchema, HostScopedValue};
use std::marker::PhantomData;

/// Permission to create instances of one statically registered native body.
///
/// Register this type in a function's construction sequence. Its ordinary Gleam
/// value is a function with the schema's argument and result types; construction
/// additionally requires the exact body identity and typed captures.
pub struct HostCreatedFunction<Schema>(PhantomData<Schema>);

impl<Schema: HostCallableSchema> private::Sealed for HostCreatedFunction<Schema> {}

impl<Schema: HostCallableSchema> HostType for HostCreatedFunction<Schema> {
    type Value<'call> = crate::HostCallable<'call, Schema::Arguments, Schema::Return>;
}

impl<Schema: HostCallableSchema> private::Abi for HostCreatedFunction<Schema> {
    const CALLABLE_CONSTRUCTION: usize = 1;
    fn descriptor() -> super::HostTypeDescriptor {
        <HostFunctionType<Schema::Arguments, Schema::Return> as HostAbiType>::descriptor()
    }

    fn schema_type() -> super::HostSchemaType {
        <HostFunctionType<Schema::Arguments, Schema::Return> as HostAbiType>::schema_type()
    }

    fn collect_custom_schemas(
        output: &mut Vec<super::HostCustomTypeSchema>,
        visited: &mut std::collections::HashSet<super::HostCustomIdentity>,
    ) {
        <HostFunctionType<Schema::Arguments, Schema::Return> as HostAbiType>::collect_custom_schemas(
            output, visited,
        );
        <Schema::Captures as super::HostAbiTypeSequence>::collect_custom_schemas(output, visited);
    }

    fn collect_callable_constructions(
        output: &mut Vec<crate::host::RegisteredCallableConstruction>,
    ) {
        output.push(crate::host::RegisteredCallableConstruction::of::<Schema>());
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::Function(value.token)
    }

    fn from_token<'call, Runtime: crate::host::HostTokenRuntime + ?Sized>(
        runtime: &Runtime,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        crate::HostCallable::new(runtime.function_token(token))
    }
}

#[cfg(test)]
mod tests {
    use super::HostCreatedFunction;
    use crate::host::function::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostAbiType, HostCallable, HostCallableSchema, HostFunctionToken, HostReturns,
        HostSchemaType, HostScopedValue, HostTypeDescriptor, HostTypeIndex0, HostTypeIndexNext,
        HostTypeList, HostTypeListEnd, HostValueFamily, HostValueToken,
    };
    use num_bigint::BigInt;

    struct Add;
    impl HostCallableSchema for Add {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "callbacks";
        const NAME: &'static str = "add";
        type Arguments = HostTypeList<BigInt, HostTypeListEnd>;
        type Return = BigInt;
        type Captures = HostTypeList<BigInt, HostTypeListEnd>;
        type Constructions = HostTypeListEnd;
        type Completion = HostReturns;
    }

    #[test]
    fn created_function_abi_preserves_its_function_schema_token_and_permission_positions() {
        type Created = HostCreatedFunction<Add>;
        assert_eq!(
            <Created as HostAbiType>::descriptor(),
            HostTypeDescriptor::Function {
                arguments: Box::new([HostTypeDescriptor::Int]),
                return_: Box::new(HostTypeDescriptor::Int),
            }
        );
        assert_eq!(
            <Created as HostAbiType>::schema_type(),
            HostSchemaType::function([HostSchemaType::Int], HostSchemaType::Int)
        );
        assert_eq!(
            <Created as HostAbiType>::into_scoped(HostCallable::new(HostFunctionToken(3))),
            HostScopedValue::Function(HostFunctionToken(3))
        );
        let mut state = TestRunState::default();
        let runtime =
            TestHostCallRuntime::new(&mut state, CallArguments::new(Vec::new(), Vec::new()));
        let token = HostValueToken {
            family: HostValueFamily::Function,
            index: 2,
        };
        assert_eq!(
            crate::host::type_::from_token::<Created, TestHostProfile>(&runtime, token).token,
            HostFunctionToken(0)
        );
        type Permissions = HostTypeList<
            BigInt,
            HostTypeList<Created, HostTypeList<bool, HostTypeList<Created, HostTypeListEnd>>>,
        >;
        assert_eq!(
            crate::host::construction_callable_count::<HostTypeListEnd>(),
            0
        );
        assert_eq!(
            crate::host::construction_callable_count::<HostTypeList<BigInt, HostTypeListEnd>>(),
            0
        );
        assert_eq!(crate::host::construction_callable_count::<Permissions>(), 2);
        assert_eq!(
            crate::host::type_::construction_callable_index::<
                Permissions,
                HostTypeIndexNext<HostTypeIndex0>,
            >(),
            0
        );
        assert_eq!(
            crate::host::type_::construction_callable_index::<
                Permissions,
                HostTypeIndexNext<HostTypeIndexNext<HostTypeIndexNext<HostTypeIndex0>>>,
            >(),
            1
        );
    }
}
