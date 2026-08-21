use super::{HostSchemaType, HostType, HostTypeDescriptor, private};
use crate::BitArrayValue;
use crate::host::HostScopedValue;
use ecow::EcoString;
use num_bigint::BigInt;

impl private::Sealed for BigInt {}

impl HostType for BigInt {
    type Value<'call> = BigInt;
}

impl private::Abi for BigInt {
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::Int
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::Int
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::Int(value)
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        runtime.int(token)
    }
}

impl private::Sealed for f64 {}

impl HostType for f64 {
    type Value<'call> = f64;
}

impl private::Abi for f64 {
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::Float
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::Float
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::Float(value)
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        runtime.float(token)
    }
}

impl private::Sealed for EcoString {}

impl HostType for EcoString {
    type Value<'call> = EcoString;
}

impl private::Abi for EcoString {
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::String
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::String
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::String(value)
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        runtime.string(token)
    }
}

impl private::Sealed for BitArrayValue {}

impl HostType for BitArrayValue {
    type Value<'call> = BitArrayValue;
}

impl private::Abi for BitArrayValue {
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::BitArray
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::BitArray
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::BitArray(value)
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        runtime.bit_array(token)
    }
}

impl private::Sealed for char {}

impl HostType for char {
    type Value<'call> = char;
}

impl private::Abi for char {
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::UtfCodepoint
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::UtfCodepoint
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::UtfCodepoint(value)
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        runtime.utf_codepoint(token)
    }
}

impl private::Sealed for bool {}

impl HostType for bool {
    type Value<'call> = bool;
}

impl private::Abi for bool {
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::Bool
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::Bool
    }

    fn into_scoped(value: <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::Bool(value)
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        runtime.bool(token)
    }
}

impl private::Sealed for () {}

impl HostType for () {
    type Value<'call> = ();
}

impl private::Abi for () {
    fn descriptor() -> HostTypeDescriptor {
        HostTypeDescriptor::Nil
    }

    fn schema_type() -> HostSchemaType {
        HostSchemaType::Nil
    }

    fn into_scoped((): <Self as HostType>::Value<'_>) -> HostScopedValue {
        HostScopedValue::Nil
    }

    fn from_token<'call, Profile: crate::host::HostProfile>(
        runtime: &dyn crate::host::HostCallRuntime<Profile>,
        token: crate::host::HostValueToken,
    ) -> <Self as HostType>::Value<'call> {
        runtime.nil(token);
    }
}

#[cfg(test)]
mod tests {
    use super::{HostSchemaType, HostTypeDescriptor};
    use crate::BitArrayValue;
    use crate::host::function::CallArguments;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{HostAbiType, HostScopedValue, HostValueFamily, HostValueToken};
    use ecow::EcoString;
    use num_bigint::BigInt;

    #[test]
    fn scalar_abi_types_preserve_every_descriptor_value_and_token() {
        assert_eq!(
            <BigInt as HostAbiType>::descriptor(),
            HostTypeDescriptor::Int
        );
        assert_eq!(
            <f64 as HostAbiType>::descriptor(),
            HostTypeDescriptor::Float
        );
        assert_eq!(
            <EcoString as HostAbiType>::descriptor(),
            HostTypeDescriptor::String,
        );
        assert_eq!(
            <BitArrayValue as HostAbiType>::descriptor(),
            HostTypeDescriptor::BitArray,
        );
        assert_eq!(
            <char as HostAbiType>::descriptor(),
            HostTypeDescriptor::UtfCodepoint,
        );
        assert_eq!(
            <bool as HostAbiType>::descriptor(),
            HostTypeDescriptor::Bool
        );
        assert_eq!(<() as HostAbiType>::descriptor(), HostTypeDescriptor::Nil);

        assert_eq!(<BigInt as HostAbiType>::schema_type(), HostSchemaType::Int);
        assert_eq!(<f64 as HostAbiType>::schema_type(), HostSchemaType::Float);
        assert_eq!(
            <EcoString as HostAbiType>::schema_type(),
            HostSchemaType::String,
        );
        assert_eq!(
            <BitArrayValue as HostAbiType>::schema_type(),
            HostSchemaType::BitArray,
        );
        assert_eq!(
            <char as HostAbiType>::schema_type(),
            HostSchemaType::UtfCodepoint,
        );
        assert_eq!(<bool as HostAbiType>::schema_type(), HostSchemaType::Bool);
        assert_eq!(<() as HostAbiType>::schema_type(), HostSchemaType::Nil);

        assert_eq!(
            <BigInt as HostAbiType>::into_scoped(1.into()),
            HostScopedValue::Int(1.into()),
        );
        assert_eq!(
            <f64 as HostAbiType>::into_scoped(1.5),
            HostScopedValue::Float(1.5),
        );
        assert_eq!(
            <EcoString as HostAbiType>::into_scoped("one".into()),
            HostScopedValue::String("one".into()),
        );
        assert_eq!(
            <BitArrayValue as HostAbiType>::into_scoped(BitArrayValue::from_bytes(vec![1])),
            HostScopedValue::BitArray(BitArrayValue::from_bytes(vec![1])),
        );
        assert_eq!(
            <char as HostAbiType>::into_scoped('A'),
            HostScopedValue::UtfCodepoint('A'),
        );
        assert_eq!(
            <bool as HostAbiType>::into_scoped(true),
            HostScopedValue::Bool(true),
        );
        assert_eq!(<() as HostAbiType>::into_scoped(()), HostScopedValue::Nil);

        let mut state = TestRunState::default();
        let arguments = CallArguments::new(Vec::new(), Vec::new());
        let runtime = TestHostCallRuntime::new(&mut state, arguments);
        let token = HostValueToken {
            family: HostValueFamily::Int,
            index: 0,
        };

        assert_eq!(
            crate::host::type_::from_token::<BigInt, TestHostProfile>(&runtime, token),
            BigInt::from(0),
        );
        assert_eq!(
            crate::host::type_::from_token::<f64, TestHostProfile>(&runtime, token),
            0.0,
        );
        assert_eq!(
            crate::host::type_::from_token::<EcoString, TestHostProfile>(&runtime, token),
            "",
        );
        assert_eq!(
            crate::host::type_::from_token::<BitArrayValue, TestHostProfile>(&runtime, token),
            BitArrayValue::from_bytes(Vec::new()),
        );
        assert_eq!(
            crate::host::type_::from_token::<char, TestHostProfile>(&runtime, token),
            '\0',
        );
        assert!(!crate::host::type_::from_token::<bool, TestHostProfile>(
            &runtime, token
        ));
        crate::host::type_::from_token::<(), TestHostProfile>(&runtime, token);
    }
}
