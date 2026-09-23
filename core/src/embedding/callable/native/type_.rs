use crate::embedding::value::{Arguments, EmbeddingValue};
use crate::embedding::{CallableType, CustomType, ExternalType, List, NamedTypeSchema};
use crate::host::{
    HostAbiTypeSequence, HostCustomSchema, HostCustomType, HostExternalSchema, HostExternalType,
    HostFunctionType, HostListType, HostTupleType, HostType, HostTypeList, HostTypeListEnd,
    HostTypeSequence,
};
use std::marker::PhantomData;

#[allow(private_bounds)]
pub trait NativeType: HostType {
    type Shape: EmbeddingValue;
}

#[allow(private_bounds)]
pub trait NativeArguments: HostTypeSequence {
    type Shape: Arguments;
}

macro_rules! scalar {
    ($($type:ty),+ $(,)?) => { $(impl NativeType for $type { type Shape = Self; })+ };
}
scalar!(
    crate::embedding::BigInt,
    f64,
    crate::StringValue,
    crate::BitArrayValue,
    char,
    bool,
    ()
);

impl<Item: NativeType> NativeType for HostListType<Item> {
    type Shape = List<Item::Shape>;
}
impl<Args: NativeArguments, Return: NativeType> NativeType for HostFunctionType<Args, Return> {
    type Shape = CallableType<Args::Shape, Return::Shape>;
}

pub struct CustomSchema<Schema, Types>(PhantomData<fn() -> (Schema, Types)>);
pub struct ExternalSchema<Schema, Types>(PhantomData<fn() -> (Schema, Types)>);

macro_rules! named {
    ($host:ident, $schema:ident, $trait:ident, $value:ident) => {
        impl<Schema: $trait, Types: HostTypeSequence> NamedTypeSchema for $schema<Schema, Types> {
            const PACKAGE: &'static str = Schema::PACKAGE;
            const MODULE: &'static str = Schema::MODULE;
            const NAME: &'static str = Schema::NAME;
            fn arguments() -> Vec<crate::ValueType> {
                <Types as HostAbiTypeSequence>::descriptors()
                    .iter()
                    .map(|type_| type_.value_shape().value_type())
                    .collect()
            }
        }
        impl<Schema: $trait, Types: HostTypeSequence> NativeType for $host<Schema, Types> {
            type Shape = $value<$schema<Schema, Types>>;
        }
    };
}
named!(HostCustomType, CustomSchema, HostCustomSchema, CustomType);
named!(
    HostExternalType,
    ExternalSchema,
    HostExternalSchema,
    ExternalType
);

macro_rules! sequence {
    () => { HostTypeListEnd };
    ($head:ident $(, $tail:ident)*) => { HostTypeList<$head, sequence!($($tail),*)> };
}
impl NativeArguments for HostTypeListEnd {
    type Shape = ();
}
macro_rules! arguments {
    ($($type:ident),+) => {
        impl<$($type: NativeType),+> NativeArguments for sequence!($($type),+) {
            type Shape = ($($type::Shape,)+);
        }
        impl<$($type: NativeType),+> NativeType for HostTupleType<sequence!($($type),+)> {
            type Shape = ($($type::Shape,)+);
        }
    };
}
arguments!(A);
arguments!(A, B);
arguments!(A, B, C);
arguments!(A, B, C, D);
arguments!(A, B, C, D, E);
arguments!(A, B, C, D, E, F);
arguments!(A, B, C, D, E, F, G);
