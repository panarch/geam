use super::function as function_lowering;
use super::graph;
use super::specialization::{
    self, SpecializationKey, SpecializedCustomConstructor, SpecializedCustomConstructorField,
    SpecializedCustomValueShape, SpecializedTypeSubstitution, SpecializedValueShape,
    StoredValueShape,
};
use super::{LoweringContext, SpecializationOutcome};
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExecutionGraphProfile, FloatFunctionId,
    IntFunctionId, NilFunctionId, ProfiledCoreRuntimeFunctionId, ProfiledListFunctionId,
    ProfiledRuntimeFunctionId, RuntimeListFunctionId, StringFunctionId, TupleFunctionId,
    UtfCodepointFunctionId,
};
use crate::plan::execution::{
    LibraryFunctionEntries, LibraryFunctionEntry, LibraryInputConstructions,
    LibraryListConstructions,
};
use crate::plan::{CustomValueShape, LibraryEntry, LibraryValueType, StandardVariant, ValueShape};
use std::convert::Infallible;

#[derive(Clone)]
pub(super) struct Entry {
    template: crate::plan::FunctionTemplateId,
    return_: LibraryValueType,
    input_variants: Box<[StandardVariant]>,
    input_lists: Box<[LibraryValueType]>,
}

pub(super) struct Reserved<Function> {
    key: SpecializationKey,
    function: specialization::Representability<Function>,
    inputs: LibraryInputConstructions,
}

pub(super) struct Sealed<Function> {
    function: Function,
    inputs: LibraryInputConstructions,
}

pub(super) enum ReservedEntry {
    Int(Reserved<IntFunctionId>),
    Float(Reserved<FloatFunctionId>),
    String(Reserved<StringFunctionId>),
    BitArray(Reserved<BitArrayFunctionId>),
    UtfCodepoint(Reserved<UtfCodepointFunctionId>),
    Custom(Reserved<CustomFunctionId>),
    Bool(Reserved<BoolFunctionId>),
    Nil(Reserved<NilFunctionId>),
    List(Reserved<RuntimeListFunctionId>),
    Tuple {
        reserved: Reserved<TupleFunctionId>,
        return_type: Vec<crate::plan::execution::type_::ValueType>,
    },
}

pub(super) enum SealedEntry {
    Int(Sealed<IntFunctionId>),
    Float(Sealed<FloatFunctionId>),
    String(Sealed<StringFunctionId>),
    BitArray(Sealed<BitArrayFunctionId>),
    UtfCodepoint(Sealed<UtfCodepointFunctionId>),
    Custom(Sealed<CustomFunctionId>),
    Bool(Sealed<BoolFunctionId>),
    Nil(Sealed<NilFunctionId>),
    List(Sealed<ProfiledListFunctionId<Infallible>>),
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

#[derive(Default)]
pub(super) struct EntryIds {
    ints: Vec<LibraryFunctionEntry<IntFunctionId>>,
    floats: Vec<LibraryFunctionEntry<FloatFunctionId>>,
    strings: Vec<LibraryFunctionEntry<StringFunctionId>>,
    bit_arrays: Vec<LibraryFunctionEntry<BitArrayFunctionId>>,
    utf_codepoints: Vec<LibraryFunctionEntry<UtfCodepointFunctionId>>,
    customs: Vec<LibraryFunctionEntry<CustomFunctionId>>,
    bools: Vec<LibraryFunctionEntry<BoolFunctionId>>,
    nils: Vec<LibraryFunctionEntry<NilFunctionId>>,
    tuples: Vec<LibraryFunctionEntry<TupleFunctionId>>,
    lists: Vec<LibraryFunctionEntry<ProfiledListFunctionId<Infallible>>>,
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
    pub(super) fn finish<Graph: ExecutionGraphProfile>(
        self,
    ) -> (ProfiledRuntimeFunctionId<Graph>, LibraryFunctionEntries) {
        let main = self.first.runtime_id();
        let mut ids = EntryIds::default();
        ids.push(self.first);
        for entry in self.remaining {
            ids.push(entry);
        }
        (main, ids.finish())
    }
}

impl From<LibraryEntry> for Entry {
    fn from(entry: LibraryEntry) -> Self {
        let (template, return_, input_variants, input_lists) = entry.into_parts();
        Self {
            template,
            return_,
            input_variants,
            input_lists,
        }
    }
}

impl Entry {
    pub(super) fn template(&self) -> crate::plan::FunctionTemplateId {
        self.template
    }

    pub(super) fn reserve(&self, context: &mut LoweringContext) -> ReservedEntry {
        let key = SpecializationKey::monomorphic(self.template());
        let inputs =
            context.library_input_constructions(&key, &self.input_variants, &self.input_lists);
        match &self.return_ {
            LibraryValueType::Int => ReservedEntry::Int(Reserved {
                function: context.reserve_int_entry(key.clone()),
                key,
                inputs,
            }),
            LibraryValueType::Float => ReservedEntry::Float(Reserved {
                function: context.reserve_float_entry(key.clone()),
                key,
                inputs,
            }),
            LibraryValueType::String => ReservedEntry::String(Reserved {
                function: context.reserve_string_entry(key.clone()),
                key,
                inputs,
            }),
            LibraryValueType::BitArray => ReservedEntry::BitArray(Reserved {
                function: context.reserve_bit_array_entry(key.clone()),
                key,
                inputs,
            }),
            LibraryValueType::UtfCodepoint => ReservedEntry::UtfCodepoint(Reserved {
                function: context.reserve_utf_codepoint_entry(key.clone()),
                key,
                inputs,
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
                })
            }
            LibraryValueType::Bool => ReservedEntry::Bool(Reserved {
                function: context.reserve_bool_entry(key.clone()),
                key,
                inputs,
            }),
            LibraryValueType::Nil => ReservedEntry::Nil(Reserved {
                function: context.reserve_nil_entry(key.clone()),
                key,
                inputs,
            }),
            LibraryValueType::List(item) => {
                let item = SpecializedValueShape::instantiate(
                    &ValueShape::from_value_type(item.value_type()),
                    key.substitution(),
                );
                let family = function_lowering::list_function_table_family(&item);
                let function =
                    context
                        .provisional_specialization(key.clone(), family)
                        .map(|specialization| {
                            function_lowering::list_function_id(
                                &item,
                                specialization.index,
                                &mut context.types,
                            )
                        });
                ReservedEntry::List(Reserved {
                    key,
                    function,
                    inputs,
                })
            }
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
                    },
                    return_type,
                }
            }
        }
    }
}

impl ReservedEntry {
    pub(super) fn seal(
        self,
    ) -> (
        SpecializationKey,
        specialization::Representability<SealedEntry>,
    ) {
        match self {
            Self::Int(reserved) => map_sealed(reserved, SealedEntry::Int),
            Self::Float(reserved) => map_sealed(reserved, SealedEntry::Float),
            Self::String(reserved) => map_sealed(reserved, SealedEntry::String),
            Self::BitArray(reserved) => map_sealed(reserved, SealedEntry::BitArray),
            Self::UtfCodepoint(reserved) => map_sealed(reserved, SealedEntry::UtfCodepoint),
            Self::Custom(reserved) => map_sealed(reserved, SealedEntry::Custom),
            Self::Bool(reserved) => map_sealed(reserved, SealedEntry::Bool),
            Self::Nil(reserved) => map_sealed(reserved, SealedEntry::Nil),
            Self::List(reserved) => map_sealed(
                Reserved {
                    key: reserved.key,
                    function: reserved
                        .function
                        .and_then(graph::seal_plain_list_function_id),
                    inputs: reserved.inputs,
                },
                SealedEntry::List,
            ),
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

fn map_sealed<Function>(
    reserved: Reserved<Function>,
    map: impl FnOnce(Sealed<Function>) -> SealedEntry,
) -> (
    SpecializationKey,
    specialization::Representability<SealedEntry>,
) {
    let Reserved {
        key,
        function,
        inputs,
    } = reserved;
    (
        key,
        function.map(|function| map(Sealed { function, inputs })),
    )
}

impl SealedEntry {
    pub(super) fn runtime_id<Graph: ExecutionGraphProfile>(
        &self,
    ) -> ProfiledRuntimeFunctionId<Graph> {
        ProfiledRuntimeFunctionId::Core(match self {
            Self::Int(sealed) => ProfiledCoreRuntimeFunctionId::Int(sealed.function),
            Self::Float(sealed) => ProfiledCoreRuntimeFunctionId::Float(sealed.function),
            Self::String(sealed) => ProfiledCoreRuntimeFunctionId::String(sealed.function),
            Self::BitArray(sealed) => ProfiledCoreRuntimeFunctionId::BitArray(sealed.function),
            Self::UtfCodepoint(sealed) => {
                ProfiledCoreRuntimeFunctionId::UtfCodepoint(sealed.function)
            }
            Self::Custom(sealed) => ProfiledCoreRuntimeFunctionId::Custom(sealed.function),
            Self::Bool(sealed) => ProfiledCoreRuntimeFunctionId::Bool(sealed.function),
            Self::Nil(sealed) => ProfiledCoreRuntimeFunctionId::Nil(sealed.function),
            Self::List(sealed) => ProfiledCoreRuntimeFunctionId::List(match &sealed.function {
                ProfiledListFunctionId::Core(function) => {
                    ProfiledListFunctionId::Core(function.clone())
                }
                ProfiledListFunctionId::External(never) => match *never {},
            }),
            Self::Tuple {
                sealed,
                return_type,
            } => ProfiledCoreRuntimeFunctionId::Tuple {
                id: sealed.function,
                return_type: return_type.clone(),
            },
        })
    }
}

impl EntryIds {
    pub(super) fn push(&mut self, entry: SealedEntry) {
        match entry {
            SealedEntry::Int(sealed) => self.ints.push(sealed.into_entry()),
            SealedEntry::Float(sealed) => self.floats.push(sealed.into_entry()),
            SealedEntry::String(sealed) => self.strings.push(sealed.into_entry()),
            SealedEntry::BitArray(sealed) => self.bit_arrays.push(sealed.into_entry()),
            SealedEntry::UtfCodepoint(sealed) => self.utf_codepoints.push(sealed.into_entry()),
            SealedEntry::Custom(sealed) => self.customs.push(sealed.into_entry()),
            SealedEntry::Bool(sealed) => self.bools.push(sealed.into_entry()),
            SealedEntry::Nil(sealed) => self.nils.push(sealed.into_entry()),
            SealedEntry::Tuple { sealed, .. } => self.tuples.push(sealed.into_entry()),
            SealedEntry::List(sealed) => self.lists.push(sealed.into_entry()),
        }
    }

    pub(super) fn finish(self) -> LibraryFunctionEntries {
        LibraryFunctionEntries {
            ints: self.ints.into_boxed_slice(),
            floats: self.floats.into_boxed_slice(),
            strings: self.strings.into_boxed_slice(),
            bit_arrays: self.bit_arrays.into_boxed_slice(),
            utf_codepoints: self.utf_codepoints.into_boxed_slice(),
            customs: self.customs.into_boxed_slice(),
            bools: self.bools.into_boxed_slice(),
            nils: self.nils.into_boxed_slice(),
            tuples: self.tuples.into_boxed_slice(),
            lists: self.lists.into_boxed_slice(),
        }
    }
}

impl<Function> Sealed<Function> {
    fn into_entry(self) -> LibraryFunctionEntry<Function> {
        LibraryFunctionEntry::new(self.function, self.inputs)
    }
}

impl LibraryValueType {
    fn stored_shape(&self, substitution: &SpecializedTypeSubstitution) -> StoredValueShape {
        match self {
            Self::Int => StoredValueShape::Int,
            Self::Float => StoredValueShape::Float,
            Self::String => StoredValueShape::String,
            Self::BitArray => StoredValueShape::BitArray,
            Self::UtfCodepoint => StoredValueShape::UtfCodepoint,
            Self::Custom(type_) => {
                StoredValueShape::Custom(SpecializedCustomValueShape::instantiate(
                    &CustomValueShape::any(type_.clone()),
                    substitution,
                ))
            }
            Self::Bool => StoredValueShape::Bool,
            Self::Nil => StoredValueShape::Nil,
            Self::Tuple(elements) => StoredValueShape::Tuple(
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
            Self::List(item) => {
                StoredValueShape::List(Box::new(item.stored_shape(substitution).to_specialized()))
            }
        }
    }
}

impl LoweringContext {
    fn library_input_constructions(
        &mut self,
        key: &SpecializationKey,
        variants: &[StandardVariant],
        input_lists: &[LibraryValueType],
    ) -> LibraryInputConstructions {
        let parameter_shapes = self.entry_templates[&key.template()]
            .parameter_shapes()
            .to_vec();
        let mut next_variant = 0;
        let mut constructions = Vec::with_capacity(variants.len());
        let mut lists = LibraryListConstructions::default();
        for shape in parameter_shapes {
            let shape = SpecializedValueShape::instantiate(&shape, key.substitution());
            self.collect_library_input_constructions(
                &shape,
                variants,
                &mut next_variant,
                &mut constructions,
            );
        }
        for item in input_lists {
            self.collect_library_list_construction(item, key.substitution(), &mut lists);
        }
        LibraryInputConstructions::new(constructions, lists)
    }

    fn collect_library_input_constructions(
        &mut self,
        shape: &SpecializedValueShape,
        variants: &[StandardVariant],
        next_variant: &mut usize,
        constructions: &mut Vec<[crate::plan::execution::type_::CustomConstructorId; 2]>,
    ) {
        match shape {
            SpecializedValueShape::Tuple(elements) => {
                for element in elements {
                    self.collect_library_input_constructions(
                        element,
                        variants,
                        next_variant,
                        constructions,
                    );
                }
            }
            SpecializedValueShape::Custom(custom) => {
                let variant = variants[*next_variant];
                *next_variant += 1;
                constructions.push(self.standard_variant_constructors(custom, variant));
                for argument in custom.arguments() {
                    self.collect_library_input_constructions(
                        argument,
                        variants,
                        next_variant,
                        constructions,
                    );
                }
            }
            SpecializedValueShape::List(item) => {
                self.collect_library_input_constructions(
                    item,
                    variants,
                    next_variant,
                    constructions,
                );
            }
            SpecializedValueShape::Parameter(_)
            | SpecializedValueShape::Int
            | SpecializedValueShape::Float
            | SpecializedValueShape::String
            | SpecializedValueShape::BitArray
            | SpecializedValueShape::UtfCodepoint
            | SpecializedValueShape::Bool
            | SpecializedValueShape::Nil
            | SpecializedValueShape::Function(_)
            | SpecializedValueShape::External(_) => {}
        }
    }

    fn collect_library_list_construction(
        &mut self,
        item: &LibraryValueType,
        substitution: &SpecializedTypeSubstitution,
        lists: &mut LibraryListConstructions,
    ) {
        match item {
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
                        .stored_list_list_type(&item.stored_shape(substitution)),
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
            (LibraryValueType::Int, StoredValueShape::Int),
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
                type_.stored_shape(&SpecializedTypeSubstitution::empty()),
                expected
            );
        }
    }
}
