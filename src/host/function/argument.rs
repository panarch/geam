mod bit_array;
mod bool;
mod float;
mod int;
mod nil;
mod string;
mod utf_codepoint;

use crate::BitArrayValue;
use crate::host::{
    HostAbiType, HostAbiTypeSequence, HostCall, HostCustomSchema, HostCustomType,
    HostExternalSchema, HostExternalType, HostFunctionType, HostListType, HostProfile,
    HostProvider, HostTupleType, HostTypeParameter,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) use bit_array::HostBitArrayArgumentSlot;
pub(crate) use bool::HostBoolArgumentSlot;
pub(crate) use float::HostFloatArgumentSlot;
pub(crate) use int::HostIntArgumentSlot;
pub(crate) use nil::HostNilArgumentSlot;
pub(crate) use string::HostStringArgumentSlot;
pub(crate) use utf_codepoint::HostUtfCodepointArgumentSlot;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum HostParameter {
    Int(HostIntArgumentSlot),
    Float(HostFloatArgumentSlot),
    String(HostStringArgumentSlot),
    BitArray(HostBitArrayArgumentSlot),
    UtfCodepoint(HostUtfCodepointArgumentSlot),
    Bool(HostBoolArgumentSlot),
    Nil(HostNilArgumentSlot),
    Value(HostValueArgumentSlot),
    List(HostListArgumentSlot),
    Tuple(HostTupleArgumentSlot),
    Custom(HostCustomArgumentSlot),
    External(HostExternalArgumentSlot),
    Function(HostFunctionArgumentSlot),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostValueArgumentSlot(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostListArgumentSlot(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostTupleArgumentSlot(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostCustomArgumentSlot(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostExternalArgumentSlot(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostFunctionArgumentSlot(usize);

#[derive(Default)]
pub(super) struct HostParameterLayout {
    parameters: Vec<HostParameter>,
    next_int: usize,
    next_float: usize,
    next_string: usize,
    next_bit_array: usize,
    next_utf_codepoint: usize,
    next_bool: usize,
    next_nil: usize,
    next_value: usize,
    next_list: usize,
    next_tuple: usize,
    next_custom: usize,
    next_external: usize,
    next_function: usize,
}

pub(crate) trait HostCallArguments {
    fn int(&self, slot: HostIntArgumentSlot) -> BigInt;
    fn float(&self, slot: HostFloatArgumentSlot) -> f64;
    fn string(&self, slot: HostStringArgumentSlot) -> EcoString;
    fn bit_array(&self, slot: HostBitArrayArgumentSlot) -> BitArrayValue;
    fn utf_codepoint(&self, slot: HostUtfCodepointArgumentSlot) -> char;
    fn bool(&self, slot: HostBoolArgumentSlot) -> bool;
    fn nil(&self, slot: HostNilArgumentSlot);
}

pub(super) trait HostArgument: super::super::HostAbiType + Sized {
    type Slot: Copy + Send + Sync + 'static;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot;
    fn read(arguments: &dyn HostCallArguments, slot: Self::Slot) -> Self;
}

pub(super) trait HostScopedArgument: HostAbiType {
    type Slot: Copy + Send + Sync + 'static;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot;

    fn read<'call, Profile, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType;
}

impl HostValueArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostListArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostTupleArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostCustomArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostExternalArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostFunctionArgumentSlot {
    pub(crate) fn index(self) -> usize {
        self.0
    }
}

impl HostParameterLayout {
    pub(super) fn register<Argument: HostArgument>(&mut self) -> Argument::Slot {
        Argument::register(self)
    }

    pub(super) fn finish(self) -> Box<[HostParameter]> {
        self.parameters.into_boxed_slice()
    }
}

impl<T> HostScopedArgument for T
where
    T: HostArgument,
    for<'call> T: HostAbiType<Value<'call> = T>,
{
    type Slot = T::Slot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        <T as HostArgument>::register(layout)
    }

    fn read<'call, Profile, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        <T as HostArgument>::read(call.arguments(), slot)
    }
}

impl<const INDEX: usize> HostScopedArgument for HostTypeParameter<INDEX> {
    type Slot = HostValueArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostValueArgumentSlot(layout.next_value);
        layout.next_value += 1;
        layout.parameters.push(HostParameter::Value(slot));
        slot
    }

    fn read<'call, Profile, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        call.value(slot)
    }
}

impl<Item: HostAbiType> HostScopedArgument for HostListType<Item> {
    type Slot = HostListArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostListArgumentSlot(layout.next_list);
        layout.next_list += 1;
        layout.parameters.push(HostParameter::List(slot));
        slot
    }

    fn read<'call, Profile, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        call.list(slot)
    }
}

impl<Elements: HostAbiTypeSequence> HostScopedArgument for HostTupleType<Elements> {
    type Slot = HostTupleArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostTupleArgumentSlot(layout.next_tuple);
        layout.next_tuple += 1;
        layout.parameters.push(HostParameter::Tuple(slot));
        slot
    }

    fn read<'call, Profile, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        call.tuple(slot)
    }
}

impl<Schema, Arguments> HostScopedArgument for HostCustomType<Schema, Arguments>
where
    Schema: HostCustomSchema,
    Arguments: HostAbiTypeSequence,
{
    type Slot = HostCustomArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostCustomArgumentSlot(layout.next_custom);
        layout.next_custom += 1;
        layout.parameters.push(HostParameter::Custom(slot));
        slot
    }

    fn read<'call, Profile, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        call.custom(slot)
    }
}

impl<Schema, Arguments> HostScopedArgument for HostExternalType<Schema, Arguments>
where
    Schema: HostExternalSchema,
    Arguments: HostAbiTypeSequence,
{
    type Slot = HostExternalArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostExternalArgumentSlot(layout.next_external);
        layout.next_external += 1;
        layout.parameters.push(HostParameter::External(slot));
        slot
    }

    fn read<'call, Profile, Provider, Return>(
        call: &HostCall<'call, Profile, Provider, Return>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        Return: HostAbiType,
    {
        call.external(slot)
    }
}

impl<Arguments, Return> HostScopedArgument for HostFunctionType<Arguments, Return>
where
    Arguments: HostAbiTypeSequence,
    Return: HostAbiType,
{
    type Slot = HostFunctionArgumentSlot;

    fn register(layout: &mut HostParameterLayout) -> Self::Slot {
        let slot = HostFunctionArgumentSlot(layout.next_function);
        layout.next_function += 1;
        layout.parameters.push(HostParameter::Function(slot));
        slot
    }

    fn read<'call, Profile, Provider, CallReturn>(
        call: &HostCall<'call, Profile, Provider, CallReturn>,
        slot: Self::Slot,
    ) -> Self::Value<'call>
    where
        Profile: HostProfile,
        Provider: HostProvider<Profile>,
        CallReturn: HostAbiType,
    {
        call.function(slot)
    }
}

#[cfg(test)]
pub(in crate::host) struct CallArguments {
    ints: Vec<BigInt>,
    floats: Vec<f64>,
    strings: Vec<EcoString>,
    bit_arrays: Vec<BitArrayValue>,
    utf_codepoints: Vec<char>,
    bools: Vec<bool>,
    nils: Vec<()>,
}

#[cfg(test)]
impl CallArguments {
    pub(in crate::host) fn new(ints: Vec<BigInt>, bools: Vec<bool>) -> Self {
        Self {
            ints,
            floats: Vec::new(),
            strings: Vec::new(),
            bit_arrays: Vec::new(),
            utf_codepoints: Vec::new(),
            bools,
            nils: Vec::new(),
        }
    }

    pub(in crate::host) fn with_scalar_values(
        mut self,
        floats: Vec<f64>,
        strings: Vec<EcoString>,
        bit_arrays: Vec<BitArrayValue>,
        utf_codepoints: Vec<char>,
        nils: usize,
    ) -> Self {
        self.floats = floats;
        self.strings = strings;
        self.bit_arrays = bit_arrays;
        self.utf_codepoints = utf_codepoints;
        self.nils = vec![(); nils];
        self
    }
}

#[cfg(test)]
impl HostCallArguments for CallArguments {
    fn int(&self, slot: HostIntArgumentSlot) -> BigInt {
        self.ints[slot.index()].clone()
    }

    fn float(&self, slot: HostFloatArgumentSlot) -> f64 {
        self.floats[slot.index()]
    }

    fn string(&self, slot: HostStringArgumentSlot) -> EcoString {
        self.strings[slot.index()].clone()
    }

    fn bit_array(&self, slot: HostBitArrayArgumentSlot) -> BitArrayValue {
        self.bit_arrays[slot.index()].clone()
    }

    fn utf_codepoint(&self, slot: HostUtfCodepointArgumentSlot) -> char {
        self.utf_codepoints[slot.index()]
    }

    fn bool(&self, slot: HostBoolArgumentSlot) -> bool {
        self.bools[slot.index()]
    }

    fn nil(&self, slot: HostNilArgumentSlot) {
        self.nils[slot.index()]
    }
}

#[cfg(test)]
mod tests {
    use super::{HostParameter, HostParameterLayout, HostScopedArgument};
    use crate::BitArrayValue;
    use crate::host::test::{TestHostCallRuntime, TestHostProfile, TestRunState};
    use crate::host::{
        HostCall, HostCustomConstructorDefinition, HostCustomConstructorList,
        HostCustomConstructorListEnd, HostCustomFieldListEnd, HostCustomSchema, HostCustomType,
        HostCustomTypeSchema, HostExternalSchema, HostExternalToken, HostExternalType,
        HostFunctionToken, HostFunctionType, HostListToken, HostListType, HostProvider,
        HostTupleToken, HostTupleType, HostTypeList, HostTypeListEnd, HostTypeParameter,
        HostValueFamily, HostValueToken,
    };
    use ecow::EcoString;
    use num_bigint::BigInt;

    struct Provider;

    impl HostProvider<TestHostProfile> for Provider {
        type State = usize;

        fn project(state: &mut TestRunState) -> &mut Self::State {
            &mut state.counter
        }
    }

    struct MarkerSchema;

    struct MarkerConstructor;

    struct ResourceSchema;

    impl HostExternalSchema for ResourceSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/resource";
        const NAME: &'static str = "Resource";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostCustomConstructorDefinition for MarkerConstructor {
        const NAME: &'static str = "Marker";

        type Fields = HostCustomFieldListEnd;
    }

    impl HostCustomSchema for MarkerSchema {
        const PACKAGE: &'static str = "domain";
        const MODULE: &'static str = "domain/marker";
        const NAME: &'static str = "Marker";
        const PARAMETER_COUNT: usize = 0;

        type Constructors =
            HostCustomConstructorList<MarkerConstructor, HostCustomConstructorListEnd>;
    }

    #[test]
    fn allocates_every_family_local_slot_in_source_order() {
        let mut layout = HostParameterLayout::default();
        let first_int = layout.register::<BigInt>();
        let first_bool = layout.register::<bool>();
        let first_float = layout.register::<f64>();
        let first_string = layout.register::<EcoString>();
        let first_bit_array = layout.register::<BitArrayValue>();
        let first_utf_codepoint = layout.register::<char>();
        let first_nil = layout.register::<()>();
        let second_int = layout.register::<BigInt>();
        let second_bool = layout.register::<bool>();
        let second_float = layout.register::<f64>();
        let second_string = layout.register::<EcoString>();
        let second_bit_array = layout.register::<BitArrayValue>();
        let second_utf_codepoint = layout.register::<char>();
        let second_nil = layout.register::<()>();

        assert_eq!(first_int.index(), 0);
        assert_eq!(first_bool.index(), 0);
        assert_eq!(first_float.index(), 0);
        assert_eq!(first_string.index(), 0);
        assert_eq!(first_bit_array.index(), 0);
        assert_eq!(first_utf_codepoint.index(), 0);
        assert_eq!(first_nil.index(), 0);
        assert_eq!(second_int.index(), 1);
        assert_eq!(second_bool.index(), 1);
        assert_eq!(second_float.index(), 1);
        assert_eq!(second_string.index(), 1);
        assert_eq!(second_bit_array.index(), 1);
        assert_eq!(second_utf_codepoint.index(), 1);
        assert_eq!(second_nil.index(), 1);
        assert_eq!(
            layout.finish().as_ref(),
            [
                HostParameter::Int(first_int),
                HostParameter::Bool(first_bool),
                HostParameter::Float(first_float),
                HostParameter::String(first_string),
                HostParameter::BitArray(first_bit_array),
                HostParameter::UtfCodepoint(first_utf_codepoint),
                HostParameter::Nil(first_nil),
                HostParameter::Int(second_int),
                HostParameter::Bool(second_bool),
                HostParameter::Float(second_float),
                HostParameter::String(second_string),
                HostParameter::BitArray(second_bit_array),
                HostParameter::UtfCodepoint(second_utf_codepoint),
                HostParameter::Nil(second_nil),
            ],
        );
    }

    #[test]
    fn reads_each_call_scoped_family_from_its_typed_slot() {
        type Parameter = HostTypeParameter<0>;
        type List = HostListType<BigInt>;
        type Tuple = HostTupleType<HostTypeList<BigInt, HostTypeListEnd>>;
        type Custom = HostCustomType<MarkerSchema>;
        type External = HostExternalType<ResourceSchema>;
        type Function = HostFunctionType<HostTypeList<BigInt, HostTypeListEnd>, bool>;

        let mut layout = HostParameterLayout::default();
        let int_slot = <BigInt as HostScopedArgument>::register(&mut layout);
        let float_slot = <f64 as HostScopedArgument>::register(&mut layout);
        let string_slot = <EcoString as HostScopedArgument>::register(&mut layout);
        let bit_array_slot = <BitArrayValue as HostScopedArgument>::register(&mut layout);
        let utf_codepoint_slot = <char as HostScopedArgument>::register(&mut layout);
        let bool_slot = <bool as HostScopedArgument>::register(&mut layout);
        let nil_slot = <() as HostScopedArgument>::register(&mut layout);
        let value_slot = <Parameter as HostScopedArgument>::register(&mut layout);
        let list_slot = <List as HostScopedArgument>::register(&mut layout);
        let tuple_slot = <Tuple as HostScopedArgument>::register(&mut layout);
        let custom_slot = <Custom as HostScopedArgument>::register(&mut layout);
        let external_slot = <External as HostScopedArgument>::register(&mut layout);
        let function_slot = <Function as HostScopedArgument>::register(&mut layout);
        assert_eq!(
            layout.finish().as_ref(),
            [
                HostParameter::Int(int_slot),
                HostParameter::Float(float_slot),
                HostParameter::String(string_slot),
                HostParameter::BitArray(bit_array_slot),
                HostParameter::UtfCodepoint(utf_codepoint_slot),
                HostParameter::Bool(bool_slot),
                HostParameter::Nil(nil_slot),
                HostParameter::Value(value_slot),
                HostParameter::List(list_slot),
                HostParameter::Tuple(tuple_slot),
                HostParameter::Custom(custom_slot),
                HostParameter::External(external_slot),
                HostParameter::Function(function_slot),
            ],
        );

        assert_eq!(HostCustomTypeSchema::of::<MarkerSchema>().name(), "Marker");

        let mut state = TestRunState::default();
        let arguments = super::CallArguments::new(vec![BigInt::from(42)], vec![true])
            .with_scalar_values(
                vec![1.5],
                vec!["one".into()],
                vec![BitArrayValue::from_bytes(vec![0xa5])],
                vec!['A'],
                1,
            );
        let mut runtime = TestHostCallRuntime::new(&mut state, arguments);
        {
            let mut call = HostCall::<TestHostProfile, Provider, bool>::new(&mut runtime);
            *call.state() += 1;
            let int = <BigInt as HostScopedArgument>::read(&call, int_slot);
            let float = <f64 as HostScopedArgument>::read(&call, float_slot);
            let string = <EcoString as HostScopedArgument>::read(&call, string_slot);
            let bit_array = <BitArrayValue as HostScopedArgument>::read(&call, bit_array_slot);
            let utf_codepoint = <char as HostScopedArgument>::read(&call, utf_codepoint_slot);
            let bool_ = <bool as HostScopedArgument>::read(&call, bool_slot);
            <() as HostScopedArgument>::read(&call, nil_slot);
            let value = <Parameter as HostScopedArgument>::read(&call, value_slot);
            let list = <List as HostScopedArgument>::read(&call, list_slot);
            let tuple = <Tuple as HostScopedArgument>::read(&call, tuple_slot);
            let custom = <Custom as HostScopedArgument>::read(&call, custom_slot);
            let external = <External as HostScopedArgument>::read(&call, external_slot);
            let function = <Function as HostScopedArgument>::read(&call, function_slot);

            assert_eq!(int, BigInt::from(42));
            assert_eq!(float, 1.5);
            assert_eq!(string, "one");
            assert_eq!(bit_array, BitArrayValue::from_bytes(vec![0xa5]));
            assert_eq!(utf_codepoint, 'A');
            assert!(bool_);
            assert_eq!(
                value.token,
                HostValueToken {
                    family: HostValueFamily::Bool,
                    index: 0,
                },
            );
            assert_eq!(list.token, HostListToken::Stored(0));
            assert_eq!(tuple.token, HostTupleToken(0));
            assert_eq!(custom.token.0, 0);
            assert_eq!(external.token, HostExternalToken(0));
            assert_eq!(function.token, HostFunctionToken(0));
        }
        assert_eq!(state.counter, 1);
    }
}
