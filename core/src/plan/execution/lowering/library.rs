use super::function as function_lowering;
use super::specialization::{
    self, SpecializationKey, SpecializedCustomConstructor, SpecializedCustomConstructorField,
    SpecializedCustomValueShape, SpecializedExternalValueShape, SpecializedTypeSubstitution,
    SpecializedValueShape, StoredValueShape,
};
use super::{LoweringContext, SpecializationOutcome};
use crate::plan::execution::function::{
    BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionId, BoolListFunctionId,
    CustomFunctionId, CustomListFunctionId, ExternalFunctionId, ExternalListFunctionId,
    FloatFunctionId, FloatListFunctionId, IntFunctionId, IntListFunctionId, LibraryListFunctionId,
    ListListFunctionId, NilFunctionId, NilListFunctionId, ProfiledCoreRuntimeFunctionId,
    ProfiledRuntimeFunctionId, StringFunctionId, StringListFunctionId, TupleFunctionId,
    TupleListFunctionId, UtfCodepointFunctionId, UtfCodepointListFunctionId,
};
use crate::plan::execution::function::{ExecutionGraphProfile, HostedExecutionGraph};
use crate::plan::execution::{
    LibraryFunctionEntries, LibraryFunctionEntry, LibraryInputConstructions,
    LibraryListConstructions,
};
use crate::plan::{
    CustomValueShape, LibraryEntry, LibraryValueType, LibraryVariant, StandardVariant, ValueShape,
};
use std::convert::Infallible;

#[derive(Clone)]
pub(super) struct Entry<External: crate::plan::LibraryProfile = crate::plan::ExternalType> {
    template: crate::plan::FunctionTemplateId,
    return_: LibraryValueType<External>,
    input_variants: Box<[LibraryVariant]>,
    input_lists: Box<[LibraryValueType]>,
    callables: Box<[crate::plan::LibraryCallableSignature]>,
}

pub(super) struct Reserved<Function> {
    key: SpecializationKey,
    function: specialization::Representability<Function>,
    inputs: LibraryInputConstructions,
    callables: Vec<crate::plan::execution::LibraryCallable>,
}

pub(super) struct Sealed<Function> {
    function: Function,
    inputs: LibraryInputConstructions,
    callables: Vec<crate::plan::execution::LibraryCallable>,
}

pub(super) enum ReservedEntry<Graph: ExecutionGraphProfile = HostedExecutionGraph> {
    Int(Reserved<IntFunctionId>),
    Float(Reserved<FloatFunctionId>),
    String(Reserved<StringFunctionId>),
    BitArray(Reserved<BitArrayFunctionId>),
    UtfCodepoint(Reserved<UtfCodepointFunctionId>),
    Custom(Reserved<CustomFunctionId>),
    External(Reserved<Graph::ExternalFunctionId>),
    Bool(Reserved<BoolFunctionId>),
    Nil(Reserved<NilFunctionId>),
    List(Reserved<LibraryListFunctionId<Graph>>),
    Function(Reserved<crate::plan::execution::LibraryCallableEntry<Graph>>),
    Tuple {
        reserved: Reserved<TupleFunctionId>,
        return_type: Vec<crate::plan::execution::type_::ValueType>,
    },
}

pub(super) enum SealedEntry<Graph: ExecutionGraphProfile = HostedExecutionGraph> {
    Int(Sealed<IntFunctionId>),
    Float(Sealed<FloatFunctionId>),
    String(Sealed<StringFunctionId>),
    BitArray(Sealed<BitArrayFunctionId>),
    UtfCodepoint(Sealed<UtfCodepointFunctionId>),
    Custom(Sealed<CustomFunctionId>),
    External(Sealed<Graph::ExternalFunctionId>),
    Bool(Sealed<BoolFunctionId>),
    Nil(Sealed<NilFunctionId>),
    List(Sealed<LibraryListFunctionId<Graph>>),
    Function(Sealed<crate::plan::execution::LibraryCallableEntry<Graph>>),
    Tuple {
        sealed: Sealed<TupleFunctionId>,
        return_type: Vec<crate::plan::execution::type_::ValueType>,
    },
}

pub(super) struct Entries {
    first: Entry,
    remaining: Vec<Entry>,
}

pub(super) struct ReservedEntries {
    first: ReservedEntry,
    remaining: Vec<ReservedEntry>,
}

pub(super) struct SealedEntries {
    first: SealedEntry,
    remaining: Vec<SealedEntry>,
}

pub(super) struct EntryIds<Graph: ExecutionGraphProfile = HostedExecutionGraph> {
    ints: Vec<LibraryFunctionEntry<IntFunctionId>>,
    floats: Vec<LibraryFunctionEntry<FloatFunctionId>>,
    strings: Vec<LibraryFunctionEntry<StringFunctionId>>,
    bit_arrays: Vec<LibraryFunctionEntry<BitArrayFunctionId>>,
    utf_codepoints: Vec<LibraryFunctionEntry<UtfCodepointFunctionId>>,
    customs: Vec<LibraryFunctionEntry<CustomFunctionId>>,
    externals: Vec<LibraryFunctionEntry<Graph::ExternalFunctionId>>,
    bools: Vec<LibraryFunctionEntry<BoolFunctionId>>,
    nils: Vec<LibraryFunctionEntry<NilFunctionId>>,
    tuples: Vec<LibraryFunctionEntry<TupleFunctionId>>,
    lists: Vec<LibraryFunctionEntry<LibraryListFunctionId<Graph>>>,
    functions: Vec<LibraryFunctionEntry<crate::plan::execution::LibraryCallableEntry<Graph>>>,
}

#[derive(Default)]
struct InputLists {
    ints: Vec<crate::plan::execution::type_::IntListTypeId>,
    floats: Vec<crate::plan::execution::type_::FloatListTypeId>,
    strings: Vec<crate::plan::execution::type_::StringListTypeId>,
    bit_arrays: Vec<crate::plan::execution::type_::BitArrayListTypeId>,
    utf_codepoints: Vec<crate::plan::execution::type_::UtfCodepointListTypeId>,
    customs: Vec<crate::plan::execution::type_::CustomListTypeId>,
    externals: Vec<crate::plan::execution::type_::ExternalListTypeId>,
    bools: Vec<crate::plan::execution::type_::BoolListTypeId>,
    nils: Vec<crate::plan::execution::type_::NilListTypeId>,
    tuples: Vec<crate::plan::execution::type_::TupleListTypeId>,
    lists: Vec<crate::plan::execution::type_::ListListTypeId>,
    functions: Vec<crate::plan::execution::type_::FunctionListTypeId>,
}

impl<Graph: ExecutionGraphProfile> Default for EntryIds<Graph> {
    fn default() -> Self {
        Self {
            ints: Vec::new(),
            floats: Vec::new(),
            strings: Vec::new(),
            bit_arrays: Vec::new(),
            utf_codepoints: Vec::new(),
            customs: Vec::new(),
            externals: Vec::new(),
            bools: Vec::new(),
            nils: Vec::new(),
            tuples: Vec::new(),
            lists: Vec::new(),
            functions: Vec::new(),
        }
    }
}

impl Entries {
    pub(super) fn new(first: LibraryEntry, remaining: Vec<LibraryEntry>) -> Self {
        Self {
            first: first.into(),
            remaining: remaining.into_iter().map(Entry::from).collect(),
        }
    }

    pub(super) fn initial_key(&self) -> SpecializationKey {
        SpecializationKey::monomorphic(self.first.template())
    }

    pub(super) fn invalid_callback(
        &self,
        context: &LoweringContext,
    ) -> Option<(crate::plan::FunctionTemplateId, crate::plan::FunctionType)> {
        for entry in std::iter::once(&self.first).chain(&self.remaining) {
            let key = SpecializationKey::monomorphic(entry.template);
            if let Some(callback) = invalid_callback(&entry.callables, &key, context) {
                return Some((entry.template, callback));
            }
        }
        None
    }

    pub(super) fn reserve(&self, context: &mut LoweringContext) -> ReservedEntries {
        ReservedEntries {
            first: self.first.reserve(context),
            remaining: self
                .remaining
                .iter()
                .map(|entry| entry.reserve(context))
                .collect(),
        }
    }
}

pub(super) fn invalid_callback(
    callables: &[crate::plan::LibraryCallableSignature],
    key: &SpecializationKey,
    context: &LoweringContext,
) -> Option<crate::plan::FunctionType> {
    let mut pending: Vec<_> = callables.iter().collect();
    while let Some(callable) = pending.pop() {
        let shape = callable_shape(&callable.type_, key.substitution());
        if shape
            .arguments()
            .iter()
            .any(|argument| !context.representations.is_inhabited(argument))
        {
            return Some(callable.type_.clone());
        }
        pending.extend(&callable.callables);
    }
    None
}

impl ReservedEntries {
    pub(super) fn seal(self) -> SpecializationOutcome<SealedEntries> {
        let (first_key, first) = self.first.seal();
        self.remaining.into_iter().fold(
            SpecializationOutcome::from_representability(first, first_key).map(|first| {
                SealedEntries {
                    first,
                    remaining: Vec::new(),
                }
            }),
            |entries, entry| {
                let (key, entry) = entry.seal();
                entries.zip_with(
                    SpecializationOutcome::from_representability(entry, key),
                    |mut entries, entry| {
                        entries.remaining.push(entry);
                        entries
                    },
                )
            },
        )
    }
}

impl SealedEntries {
    pub(super) fn finish(
        self,
    ) -> (
        crate::plan::execution::function::RuntimeFunctionId,
        LibraryFunctionEntries,
    ) {
        let main = self.first.runtime_id();
        let mut ids = EntryIds::default();
        ids.push(self.first);
        for entry in self.remaining {
            ids.push(entry);
        }
        (main, ids.finish())
    }
}

impl<External: crate::plan::LibraryProfile> From<LibraryEntry<External>> for Entry<External> {
    fn from(entry: LibraryEntry<External>) -> Self {
        let (template, return_, input_variants, input_lists, callables) = entry.into_parts();
        Self {
            template,
            return_,
            input_variants,
            input_lists,
            callables,
        }
    }
}

pub(super) trait LibraryExternal: crate::plan::LibraryProfile {
    fn reserve_callable(
        value: &Self::Callable,
        key: SpecializationKey,
        context: &mut LoweringContext,
    ) -> specialization::Representability<crate::plan::execution::LibraryCallableEntry<Self::Graph>>;

    type Graph: ExecutionGraphProfile;

    fn specialize(
        &self,
        substitution: &SpecializedTypeSubstitution,
    ) -> SpecializedExternalValueShape;

    fn reserve(
        &self,
        key: SpecializationKey,
        context: &mut LoweringContext,
    ) -> specialization::Representability<<Self::Graph as ExecutionGraphProfile>::ExternalFunctionId>;

    fn list_function(
        &self,
        substitution: &SpecializedTypeSubstitution,
        index: usize,
        types: &mut super::value_type::TypeInterner,
    ) -> <Self::Graph as ExecutionGraphProfile>::ExternalListFunctionId;
}

impl LibraryExternal for Infallible {
    type Graph = Infallible;

    fn reserve_callable(
        value: &Self::Callable,
        _: SpecializationKey,
        _: &mut LoweringContext,
    ) -> specialization::Representability<crate::plan::execution::LibraryCallableEntry<Self::Graph>>
    {
        match *value {}
    }

    fn specialize(&self, _: &SpecializedTypeSubstitution) -> SpecializedExternalValueShape {
        match *self {}
    }

    fn reserve(
        &self,
        _: SpecializationKey,
        _: &mut LoweringContext,
    ) -> specialization::Representability<Infallible> {
        match *self {}
    }

    fn list_function(
        &self,
        _: &SpecializedTypeSubstitution,
        _: usize,
        _: &mut super::value_type::TypeInterner,
    ) -> Infallible {
        match *self {}
    }
}

impl LibraryExternal for crate::plan::ExternalType {
    type Graph = HostedExecutionGraph;

    fn reserve_callable(
        value: &Self::Callable,
        key: SpecializationKey,
        context: &mut LoweringContext,
    ) -> specialization::Representability<crate::plan::execution::LibraryCallableEntry<Self::Graph>>
    {
        let shape = callable_shape(value, key.substitution());
        let family =
            function_lowering::function_function_table_family(&shape, &context.representations);
        context
            .provisional_specialization(key, family)
            .map(
                |specialization| crate::plan::execution::LibraryCallableEntry {
                    function: function_lowering::invocable_function_function_id(
                        &shape,
                        specialization.index,
                        &mut context.types,
                        &context.representations,
                    ),
                    type_: context.lower_concrete_function_type(&shape),
                },
            )
    }

    fn specialize(
        &self,
        substitution: &SpecializedTypeSubstitution,
    ) -> SpecializedExternalValueShape {
        SpecializedExternalValueShape::instantiate(
            &crate::plan::ExternalValueShape::new(
                self.type_name().clone(),
                self.arguments()
                    .iter()
                    .cloned()
                    .map(ValueShape::from_value_type)
                    .collect(),
            ),
            substitution,
        )
    }

    fn reserve(
        &self,
        key: SpecializationKey,
        context: &mut LoweringContext,
    ) -> specialization::Representability<ExternalFunctionId> {
        let return_type =
            context.lower_concrete_external_type(&self.specialize(key.substitution()));
        context
            .provisional_specialization(key, function_lowering::FunctionTableFamily::External)
            .map(|specialization| ExternalFunctionId::new(specialization.index, return_type))
    }

    fn list_function(
        &self,
        substitution: &SpecializedTypeSubstitution,
        index: usize,
        types: &mut super::value_type::TypeInterner,
    ) -> ExternalListFunctionId {
        ExternalListFunctionId::new(
            index,
            types.external_list_type(&self.specialize(substitution)),
        )
    }
}

impl<External: LibraryExternal> Entry<External> {
    pub(super) fn template(&self) -> crate::plan::FunctionTemplateId {
        self.template
    }

    pub(super) fn reserve(&self, context: &mut LoweringContext) -> ReservedEntry<External::Graph> {
        let key = SpecializationKey::monomorphic(self.template());
        let inputs =
            context.library_input_constructions(&key, &self.input_variants, &self.input_lists);
        let callables = context.library_callables(&key, &self.callables);
        match &self.return_ {
            LibraryValueType::Int => ReservedEntry::Int(Reserved {
                function: context.reserve_int_entry(key.clone()),
                key,
                inputs,
                callables,
            }),
            LibraryValueType::Float => ReservedEntry::Float(Reserved {
                function: context.reserve_float_entry(key.clone()),
                key,
                inputs,
                callables,
            }),
            LibraryValueType::String => ReservedEntry::String(Reserved {
                function: context.reserve_string_entry(key.clone()),
                key,
                inputs,
                callables,
            }),
            LibraryValueType::BitArray => ReservedEntry::BitArray(Reserved {
                function: context.reserve_bit_array_entry(key.clone()),
                key,
                inputs,
                callables,
            }),
            LibraryValueType::UtfCodepoint => ReservedEntry::UtfCodepoint(Reserved {
                function: context.reserve_utf_codepoint_entry(key.clone()),
                key,
                inputs,
                callables,
            }),
            LibraryValueType::Custom(type_) => {
                let shape = SpecializedCustomValueShape::instantiate(
                    &CustomValueShape::any(type_.clone()),
                    key.substitution(),
                );
                let return_shape = context.lower_concrete_custom_shape(&shape);
                ReservedEntry::Custom(Reserved {
                    function: context.reserve_custom_entry(key.clone(), return_shape),
                    key,
                    inputs,
                    callables,
                })
            }
            LibraryValueType::External(type_) => ReservedEntry::External(Reserved {
                function: type_.reserve(key.clone(), context),
                key,
                inputs,
                callables,
            }),
            LibraryValueType::Bool => ReservedEntry::Bool(Reserved {
                function: context.reserve_bool_entry(key.clone()),
                key,
                inputs,
                callables,
            }),
            LibraryValueType::Nil => ReservedEntry::Nil(Reserved {
                function: context.reserve_nil_entry(key.clone()),
                key,
                inputs,
                callables,
            }),
            LibraryValueType::List(item) => {
                let specialized_item =
                    library_stored_shape(&**item, key.substitution()).to_specialized();
                let family = function_lowering::list_function_table_family(&specialized_item);
                let function =
                    context
                        .provisional_specialization(key.clone(), family)
                        .map(|specialization| {
                            library_list_function_id(
                                item,
                                key.substitution(),
                                specialization.index,
                                &mut context.types,
                            )
                        });
                ReservedEntry::List(Reserved {
                    key,
                    function,
                    inputs,
                    callables,
                })
            }
            LibraryValueType::Function(type_) => ReservedEntry::Function(Reserved {
                function: External::reserve_callable(type_, key.clone(), context),
                key,
                inputs,
                callables,
            }),
            LibraryValueType::Tuple(elements) => {
                let return_type = elements
                    .iter()
                    .map(|element| {
                        let shape = SpecializedValueShape::instantiate(
                            &ValueShape::from_value_type(element.clone()),
                            key.substitution(),
                        );
                        context.lower_concrete_value_type(&shape)
                    })
                    .collect();
                ReservedEntry::Tuple {
                    reserved: Reserved {
                        function: context.reserve_tuple_entry(key.clone()),
                        key,
                        inputs,
                        callables,
                    },
                    return_type,
                }
            }
        }
    }
}

impl<Graph: ExecutionGraphProfile> ReservedEntry<Graph> {
    pub(super) fn seal(
        self,
    ) -> (
        SpecializationKey,
        specialization::Representability<SealedEntry<Graph>>,
    ) {
        match self {
            Self::Int(reserved) => map_sealed(reserved, SealedEntry::Int),
            Self::Float(reserved) => map_sealed(reserved, SealedEntry::Float),
            Self::String(reserved) => map_sealed(reserved, SealedEntry::String),
            Self::BitArray(reserved) => map_sealed(reserved, SealedEntry::BitArray),
            Self::UtfCodepoint(reserved) => map_sealed(reserved, SealedEntry::UtfCodepoint),
            Self::Custom(reserved) => map_sealed(reserved, SealedEntry::Custom),
            Self::External(reserved) => map_sealed(reserved, SealedEntry::External),
            Self::Bool(reserved) => map_sealed(reserved, SealedEntry::Bool),
            Self::Nil(reserved) => map_sealed(reserved, SealedEntry::Nil),
            Self::List(reserved) => map_sealed(reserved, SealedEntry::List),
            Self::Function(reserved) => map_sealed(reserved, SealedEntry::Function),
            Self::Tuple {
                reserved,
                return_type,
            } => map_sealed(reserved, |sealed| SealedEntry::Tuple {
                sealed,
                return_type,
            }),
        }
    }
}

fn map_sealed<Function, Graph: ExecutionGraphProfile>(
    reserved: Reserved<Function>,
    map: impl FnOnce(Sealed<Function>) -> SealedEntry<Graph>,
) -> (
    SpecializationKey,
    specialization::Representability<SealedEntry<Graph>>,
) {
    let Reserved {
        key,
        function,
        inputs,
        callables,
    } = reserved;
    (
        key,
        function.map(|function| {
            map(Sealed {
                function,
                inputs,
                callables,
            })
        }),
    )
}

impl<Graph: ExecutionGraphProfile> SealedEntry<Graph> {
    pub(super) fn runtime_id(&self) -> ProfiledRuntimeFunctionId<Graph> {
        let core = ProfiledRuntimeFunctionId::Core;
        match self {
            Self::Function(sealed) => core(ProfiledCoreRuntimeFunctionId::Function {
                id: Graph::function_value_target(&sealed.function.function),
                return_type: sealed.function.type_.clone(),
            }),
            Self::Int(sealed) => core(ProfiledCoreRuntimeFunctionId::Int(sealed.function)),
            Self::Float(sealed) => core(ProfiledCoreRuntimeFunctionId::Float(sealed.function)),
            Self::String(sealed) => core(ProfiledCoreRuntimeFunctionId::String(sealed.function)),
            Self::BitArray(sealed) => {
                core(ProfiledCoreRuntimeFunctionId::BitArray(sealed.function))
            }
            Self::UtfCodepoint(sealed) => {
                core(ProfiledCoreRuntimeFunctionId::UtfCodepoint(sealed.function))
            }
            Self::Custom(sealed) => core(ProfiledCoreRuntimeFunctionId::Custom(sealed.function)),
            Self::External(sealed) => ProfiledRuntimeFunctionId::External(sealed.function.clone()),
            Self::Bool(sealed) => core(ProfiledCoreRuntimeFunctionId::Bool(sealed.function)),
            Self::Nil(sealed) => core(ProfiledCoreRuntimeFunctionId::Nil(sealed.function)),
            Self::List(sealed) => core(ProfiledCoreRuntimeFunctionId::List(
                sealed.function.profiled_runtime_id(),
            )),
            Self::Tuple {
                sealed,
                return_type,
            } => core(ProfiledCoreRuntimeFunctionId::Tuple {
                id: sealed.function,
                return_type: return_type.clone().into(),
            }),
        }
    }
}

fn library_list_function_id<External: LibraryExternal>(
    item: &LibraryValueType<External>,
    substitution: &SpecializedTypeSubstitution,
    index: usize,
    types: &mut crate::plan::execution::lowering::value_type::TypeInterner,
) -> LibraryListFunctionId<External::Graph> {
    match item {
        LibraryValueType::Int => {
            LibraryListFunctionId::Int(IntListFunctionId::new(index, types.int_list_type()))
        }
        LibraryValueType::Float => {
            LibraryListFunctionId::Float(FloatListFunctionId::new(index, types.float_list_type()))
        }
        LibraryValueType::String => LibraryListFunctionId::String(StringListFunctionId::new(
            index,
            types.string_list_type(),
        )),
        LibraryValueType::BitArray => LibraryListFunctionId::BitArray(BitArrayListFunctionId::new(
            index,
            types.bit_array_list_type(),
        )),
        LibraryValueType::UtfCodepoint => LibraryListFunctionId::UtfCodepoint(
            UtfCodepointListFunctionId::new(index, types.utf_codepoint_list_type()),
        ),
        LibraryValueType::Custom(item) => {
            let item = SpecializedCustomValueShape::instantiate(
                &CustomValueShape::any(item.clone()),
                substitution,
            );
            LibraryListFunctionId::Custom(CustomListFunctionId::new(
                index,
                types.custom_list_type(&item),
            ))
        }
        LibraryValueType::External(item) => {
            LibraryListFunctionId::External(item.list_function(substitution, index, types))
        }
        LibraryValueType::Bool => {
            LibraryListFunctionId::Bool(BoolListFunctionId::new(index, types.bool_list_type()))
        }
        LibraryValueType::Nil => {
            LibraryListFunctionId::Nil(NilListFunctionId::new(index, types.nil_list_type()))
        }
        LibraryValueType::Tuple(items) => {
            let items = items
                .iter()
                .map(|item| {
                    SpecializedValueShape::instantiate(
                        &ValueShape::from_value_type(item.clone()),
                        substitution,
                    )
                })
                .collect::<Vec<_>>();
            LibraryListFunctionId::Tuple(TupleListFunctionId::new(
                index,
                types.tuple_list_type(&items),
            ))
        }
        LibraryValueType::Function(item) => LibraryListFunctionId::Function(
            crate::plan::execution::function::FunctionListFunctionId::new(
                index,
                types.function_list_type(&callable_shape(
                    External::callable_type(item),
                    substitution,
                )),
            ),
        ),
        LibraryValueType::List(item) => {
            let item = library_stored_shape(&**item, substitution);
            LibraryListFunctionId::List(ListListFunctionId::new(
                index,
                types.stored_list_list_type(&item),
            ))
        }
    }
}

impl<Graph: ExecutionGraphProfile> EntryIds<Graph> {
    pub(super) fn push(&mut self, entry: SealedEntry<Graph>) {
        match entry {
            SealedEntry::Function(sealed) => self.functions.push(sealed.into_entry()),
            SealedEntry::Int(sealed) => self.ints.push(sealed.into_entry()),
            SealedEntry::Float(sealed) => self.floats.push(sealed.into_entry()),
            SealedEntry::String(sealed) => self.strings.push(sealed.into_entry()),
            SealedEntry::BitArray(sealed) => self.bit_arrays.push(sealed.into_entry()),
            SealedEntry::UtfCodepoint(sealed) => self.utf_codepoints.push(sealed.into_entry()),
            SealedEntry::Custom(sealed) => self.customs.push(sealed.into_entry()),
            SealedEntry::External(sealed) => self.externals.push(sealed.into_entry()),
            SealedEntry::Bool(sealed) => self.bools.push(sealed.into_entry()),
            SealedEntry::Nil(sealed) => self.nils.push(sealed.into_entry()),
            SealedEntry::Tuple { sealed, .. } => self.tuples.push(sealed.into_entry()),
            SealedEntry::List(sealed) => self.lists.push(sealed.into_entry()),
        }
    }

    pub(super) fn finish(self) -> LibraryFunctionEntries<Graph> {
        LibraryFunctionEntries {
            ints: self.ints.into(),
            floats: self.floats.into(),
            strings: self.strings.into(),
            bit_arrays: self.bit_arrays.into(),
            utf_codepoints: self.utf_codepoints.into(),
            customs: self.customs.into(),
            externals: self.externals.into(),
            bools: self.bools.into(),
            nils: self.nils.into(),
            tuples: self.tuples.into(),
            lists: self.lists.into(),
            functions: self.functions.into(),
        }
    }
}

impl InputLists {
    fn finish(self) -> LibraryListConstructions {
        LibraryListConstructions {
            ints: self.ints.into(),
            floats: self.floats.into(),
            strings: self.strings.into(),
            bit_arrays: self.bit_arrays.into(),
            utf_codepoints: self.utf_codepoints.into(),
            customs: self.customs.into(),
            externals: self.externals.into(),
            bools: self.bools.into(),
            nils: self.nils.into(),
            tuples: self.tuples.into(),
            lists: self.lists.into(),
            functions: self.functions.into(),
        }
    }
}

impl<Function> Sealed<Function> {
    fn into_entry(self) -> LibraryFunctionEntry<Function> {
        LibraryFunctionEntry::new(self.function, self.inputs, self.callables.into())
    }
}

fn library_stored_shape<External: LibraryExternal>(
    type_: &LibraryValueType<External>,
    substitution: &SpecializedTypeSubstitution,
) -> StoredValueShape {
    match type_ {
        LibraryValueType::Int => StoredValueShape::Int,
        LibraryValueType::Float => StoredValueShape::Float,
        LibraryValueType::String => StoredValueShape::String,
        LibraryValueType::BitArray => StoredValueShape::BitArray,
        LibraryValueType::UtfCodepoint => StoredValueShape::UtfCodepoint,
        LibraryValueType::Custom(type_) => {
            StoredValueShape::Custom(SpecializedCustomValueShape::instantiate(
                &CustomValueShape::any(type_.clone()),
                substitution,
            ))
        }
        LibraryValueType::External(type_) => {
            StoredValueShape::External(type_.specialize(substitution))
        }
        LibraryValueType::Bool => StoredValueShape::Bool,
        LibraryValueType::Nil => StoredValueShape::Nil,
        LibraryValueType::Tuple(elements) => StoredValueShape::Tuple(
            elements
                .iter()
                .map(|element| {
                    SpecializedValueShape::instantiate(
                        &ValueShape::from_value_type(element.clone()),
                        substitution,
                    )
                })
                .collect(),
        ),
        LibraryValueType::Function(type_) => StoredValueShape::Function(Box::new(callable_shape(
            External::callable_type(type_),
            substitution,
        ))),
        LibraryValueType::List(item) => StoredValueShape::List(Box::new(
            library_stored_shape(&**item, substitution).to_specialized(),
        )),
    }
}

fn callable_shape(
    type_: &crate::plan::FunctionType,
    substitution: &SpecializedTypeSubstitution,
) -> super::specialization::SpecializedFunctionShape {
    super::specialization::SpecializedFunctionShape::instantiate(
        &crate::plan::FunctionShape::from_function_type(type_.clone()),
        substitution,
    )
}

impl LoweringContext {
    pub(super) fn library_callables(
        &mut self,
        key: &SpecializationKey,
        callables: &[crate::plan::LibraryCallableSignature],
    ) -> Vec<crate::plan::execution::LibraryCallable> {
        callables
            .iter()
            .map(|callable| self.library_callable(key, callable))
            .collect()
    }

    pub(super) fn library_callable(
        &mut self,
        key: &SpecializationKey,
        callable: &crate::plan::LibraryCallableSignature,
    ) -> crate::plan::execution::LibraryCallable {
        let shape = callable_shape(&callable.type_, key.substitution());
        crate::plan::execution::LibraryCallable {
            type_: self.lower_concrete_function_type(&shape),
            inputs: self.library_input_constructions(
                key,
                &callable.input_variants,
                &callable.input_lists,
            ),
            callables: self.library_callables(key, &callable.callables).into(),
        }
    }

    pub(super) fn library_input_constructions(
        &mut self,
        key: &SpecializationKey,
        variants: &[LibraryVariant],
        input_lists: &[LibraryValueType],
    ) -> LibraryInputConstructions {
        let mut constructions = Vec::with_capacity(variants.len());
        let mut lists = InputLists::default();
        for variant in variants {
            let shape = SpecializedCustomValueShape::instantiate(
                &CustomValueShape::any(variant.kind.custom_type(variant.arguments.clone())),
                key.substitution(),
            );
            constructions.push(self.standard_variant_constructors(&shape, variant.kind));
        }
        for item in input_lists {
            self.collect_library_list_construction(item, key.substitution(), &mut lists);
        }
        LibraryInputConstructions::new(constructions, lists.finish())
    }

    fn collect_library_list_construction(
        &mut self,
        item: &LibraryValueType,
        substitution: &SpecializedTypeSubstitution,
        lists: &mut InputLists,
    ) {
        match item {
            LibraryValueType::Function(type_) => lists.functions.push(
                self.types
                    .function_list_type(&callable_shape(type_, substitution)),
            ),
            LibraryValueType::Int => lists.ints.push(self.types.int_list_type()),
            LibraryValueType::Float => lists.floats.push(self.types.float_list_type()),
            LibraryValueType::String => lists.strings.push(self.types.string_list_type()),
            LibraryValueType::BitArray => lists.bit_arrays.push(self.types.bit_array_list_type()),
            LibraryValueType::UtfCodepoint => lists
                .utf_codepoints
                .push(self.types.utf_codepoint_list_type()),
            LibraryValueType::Custom(item) => {
                let shape = SpecializedCustomValueShape::instantiate(
                    &CustomValueShape::any(item.clone()),
                    substitution,
                );
                lists.customs.push(self.types.custom_list_type(&shape))
            }
            LibraryValueType::External(item) => {
                let shape = SpecializedExternalValueShape::instantiate(
                    &crate::plan::ExternalValueShape::new(
                        item.type_name().clone(),
                        item.arguments()
                            .iter()
                            .cloned()
                            .map(ValueShape::from_value_type)
                            .collect(),
                    ),
                    substitution,
                );
                lists.externals.push(self.types.external_list_type(&shape));
            }
            LibraryValueType::Bool => lists.bools.push(self.types.bool_list_type()),
            LibraryValueType::Nil => lists.nils.push(self.types.nil_list_type()),
            LibraryValueType::Tuple(items) => {
                let items = items
                    .iter()
                    .map(|item| {
                        SpecializedValueShape::instantiate(
                            &ValueShape::from_value_type(item.clone()),
                            substitution,
                        )
                    })
                    .collect::<Vec<_>>();
                lists.tuples.push(self.types.tuple_list_type(&items))
            }
            LibraryValueType::List(item) => {
                lists.lists.push(
                    self.types
                        .stored_list_list_type(&library_stored_shape(&**item, substitution)),
                );
            }
        }
    }

    fn standard_variant_constructors(
        &mut self,
        shape: &SpecializedCustomValueShape,
        variant: StandardVariant,
    ) -> [crate::plan::execution::type_::CustomConstructorId; 2] {
        match variant {
            StandardVariant::Result => [
                self.types
                    .custom_constructor(SpecializedCustomConstructor::new(
                        shape.clone(),
                        "Ok".into(),
                        0,
                        vec![SpecializedCustomConstructorField::new(
                            None,
                            shape.arguments()[0].clone(),
                            crate::plan::execution::type_::custom::FieldRefinement::Argument(0),
                        )]
                        .into_boxed_slice(),
                    )),
                self.types
                    .custom_constructor(SpecializedCustomConstructor::new(
                        shape.clone(),
                        "Error".into(),
                        1,
                        vec![SpecializedCustomConstructorField::new(
                            None,
                            shape.arguments()[1].clone(),
                            crate::plan::execution::type_::custom::FieldRefinement::Argument(1),
                        )]
                        .into_boxed_slice(),
                    )),
            ],
            StandardVariant::Option => [
                self.types
                    .custom_constructor(SpecializedCustomConstructor::new(
                        shape.clone(),
                        "Some".into(),
                        0,
                        vec![SpecializedCustomConstructorField::new(
                            None,
                            shape.arguments()[0].clone(),
                            crate::plan::execution::type_::custom::FieldRefinement::Argument(0),
                        )]
                        .into_boxed_slice(),
                    )),
                self.types
                    .custom_constructor(SpecializedCustomConstructor::new(
                        shape.clone(),
                        "None".into(),
                        1,
                        Vec::new().into_boxed_slice(),
                    )),
            ],
        }
    }

    fn reserve_int_entry(
        &mut self,
        key: SpecializationKey,
    ) -> specialization::Representability<IntFunctionId> {
        self.provisional_specialization(key, function_lowering::FunctionTableFamily::Int)
            .map(|specialization| IntFunctionId(specialization.index))
    }

    fn reserve_float_entry(
        &mut self,
        key: SpecializationKey,
    ) -> specialization::Representability<FloatFunctionId> {
        self.provisional_specialization(key, function_lowering::FunctionTableFamily::Float)
            .map(|specialization| FloatFunctionId(specialization.index))
    }

    fn reserve_string_entry(
        &mut self,
        key: SpecializationKey,
    ) -> specialization::Representability<StringFunctionId> {
        self.provisional_specialization(key, function_lowering::FunctionTableFamily::String)
            .map(|specialization| StringFunctionId(specialization.index))
    }

    fn reserve_bit_array_entry(
        &mut self,
        key: SpecializationKey,
    ) -> specialization::Representability<BitArrayFunctionId> {
        self.provisional_specialization(key, function_lowering::FunctionTableFamily::BitArray)
            .map(|specialization| BitArrayFunctionId(specialization.index))
    }

    fn reserve_utf_codepoint_entry(
        &mut self,
        key: SpecializationKey,
    ) -> specialization::Representability<UtfCodepointFunctionId> {
        self.provisional_specialization(key, function_lowering::FunctionTableFamily::UtfCodepoint)
            .map(|specialization| UtfCodepointFunctionId(specialization.index))
    }

    fn reserve_custom_entry(
        &mut self,
        key: SpecializationKey,
        return_shape: crate::plan::execution::type_::CustomValueShape,
    ) -> specialization::Representability<CustomFunctionId> {
        self.provisional_specialization(key, function_lowering::FunctionTableFamily::Custom)
            .map(|specialization| CustomFunctionId::new(specialization.index, return_shape))
    }

    fn reserve_bool_entry(
        &mut self,
        key: SpecializationKey,
    ) -> specialization::Representability<BoolFunctionId> {
        self.provisional_specialization(key, function_lowering::FunctionTableFamily::Bool)
            .map(|specialization| BoolFunctionId(specialization.index))
    }

    fn reserve_nil_entry(
        &mut self,
        key: SpecializationKey,
    ) -> specialization::Representability<NilFunctionId> {
        self.provisional_specialization(key, function_lowering::FunctionTableFamily::Nil)
            .map(|specialization| NilFunctionId(specialization.index))
    }

    fn reserve_tuple_entry(
        &mut self,
        key: SpecializationKey,
    ) -> specialization::Representability<TupleFunctionId> {
        self.provisional_specialization(key, function_lowering::FunctionTableFamily::Tuple)
            .map(|specialization| TupleFunctionId(specialization.index))
    }
}

#[cfg(test)]
mod tests {
    use super::{
        LibraryValueType, SpecializedCustomValueShape, SpecializedTypeSubstitution,
        SpecializedValueShape, StoredValueShape,
    };
    use crate::plan::{CustomConstructorRefinement, StandardVariant, ValueType};

    #[test]
    fn library_list_items_preserve_each_concrete_storage_family() {
        let result = StandardVariant::Result.custom_type(vec![ValueType::Int, ValueType::String]);
        let result_shape = SpecializedCustomValueShape::new(
            result.type_name().clone(),
            vec![SpecializedValueShape::Int, SpecializedValueShape::String],
            CustomConstructorRefinement::Any,
        );
        let cases = [
            (
                LibraryValueType::<crate::plan::ExternalType>::Int,
                StoredValueShape::Int,
            ),
            (LibraryValueType::Float, StoredValueShape::Float),
            (LibraryValueType::String, StoredValueShape::String),
            (LibraryValueType::BitArray, StoredValueShape::BitArray),
            (
                LibraryValueType::UtfCodepoint,
                StoredValueShape::UtfCodepoint,
            ),
            (
                LibraryValueType::Custom(result),
                StoredValueShape::Custom(result_shape),
            ),
            (LibraryValueType::Bool, StoredValueShape::Bool),
            (LibraryValueType::Nil, StoredValueShape::Nil),
            (
                LibraryValueType::Tuple(vec![
                    ValueType::Int,
                    ValueType::List(Box::new(ValueType::Bool)),
                ]),
                StoredValueShape::Tuple(
                    vec![
                        SpecializedValueShape::Int,
                        SpecializedValueShape::List(Box::new(SpecializedValueShape::Bool)),
                    ]
                    .into_boxed_slice(),
                ),
            ),
            (
                LibraryValueType::List(Box::new(LibraryValueType::List(Box::new(
                    LibraryValueType::String,
                )))),
                StoredValueShape::List(Box::new(SpecializedValueShape::List(Box::new(
                    SpecializedValueShape::String,
                )))),
            ),
        ];
        for (type_, expected) in cases {
            assert_eq!(
                super::library_stored_shape(&type_, &SpecializedTypeSubstitution::empty()),
                expected
            );
        }
    }
}
