use crate::provider::{List, ProviderNoConstructions, ProviderValue};
use crate::{HostListType, HostTupleType, HostTypeList, HostTypeListEnd};

// These source markers provide nominal type arguments. They do not implement
// value conversion or grant invocation rights to opaque function shapes.
impl<Item: ProviderValue> ProviderValue for List<Item> {
    type Host = HostListType<Item::Host>;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Item: ProviderValue> ProviderValue for Option<Item> {
    type Host = crate::provider::ProviderOption<Item::Host>;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

impl<Success: ProviderValue, Failure: ProviderValue> ProviderValue for Result<Success, Failure> {
    type Host = crate::provider::ProviderResult<Success::Host, Failure::Host>;
    type OutputRequirements = ProviderNoConstructions;
    type RootRequirements = ProviderNoConstructions;
}

macro_rules! source_sequence {
    () => { HostTypeListEnd };
    ($head:ident $(, $tail:ident)*) => { HostTypeList<$head::Host, source_sequence!($($tail),*)> };
}

macro_rules! function_type {
    ($($argument:ident),*) => {
        impl<Return: ProviderValue, $($argument: ProviderValue,)*> ProviderValue for fn($($argument),*) -> Return {
            type Host = crate::provider_support::HostOpaqueFunctionType<source_sequence!($($argument),*), Return::Host>;
            type OutputRequirements = ProviderNoConstructions;
            type RootRequirements = ProviderNoConstructions;
        }
    };
}
macro_rules! tuple_type {
    ($($element:ident),+) => {
        impl<$($element: ProviderValue),+> ProviderValue for ($($element,)+) {
            type Host = HostTupleType<source_sequence!($($element),+)>;
            type OutputRequirements = ProviderNoConstructions;
            type RootRequirements = ProviderNoConstructions;
        }
        function_type!($($element),+);
    };
}
function_type!();
tuple_type!(A);
tuple_type!(A, B);
tuple_type!(A, B, C);
tuple_type!(A, B, C, D);
tuple_type!(A, B, C, D, E);
tuple_type!(A, B, C, D, E, F);
tuple_type!(A, B, C, D, E, F, G);
