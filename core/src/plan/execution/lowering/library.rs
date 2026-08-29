use super::function as function_lowering;
use super::specialization::{self, SpecializationKey};
use super::{LoweringContext, SpecializationOutcome};
use crate::plan::LibraryEntry;
use crate::plan::execution::LibraryFunctionEntries;
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, ExecutionGraphProfile, FloatFunctionId, IntFunctionId,
    NilFunctionId, ProfiledCoreRuntimeFunctionId, ProfiledRuntimeFunctionId, StringFunctionId,
    UtfCodepointFunctionId,
};

#[derive(Clone, Copy)]
pub(super) enum Entry {
    Int(crate::plan::FunctionTemplateId),
    Float(crate::plan::FunctionTemplateId),
    String(crate::plan::FunctionTemplateId),
    BitArray(crate::plan::FunctionTemplateId),
    UtfCodepoint(crate::plan::FunctionTemplateId),
    Bool(crate::plan::FunctionTemplateId),
    Nil(crate::plan::FunctionTemplateId),
}

pub(super) enum ReservedEntry {
    Int {
        key: SpecializationKey,
        id: specialization::Representability<IntFunctionId>,
    },
    Float {
        key: SpecializationKey,
        id: specialization::Representability<FloatFunctionId>,
    },
    String {
        key: SpecializationKey,
        id: specialization::Representability<StringFunctionId>,
    },
    BitArray {
        key: SpecializationKey,
        id: specialization::Representability<BitArrayFunctionId>,
    },
    UtfCodepoint {
        key: SpecializationKey,
        id: specialization::Representability<UtfCodepointFunctionId>,
    },
    Bool {
        key: SpecializationKey,
        id: specialization::Representability<BoolFunctionId>,
    },
    Nil {
        key: SpecializationKey,
        id: specialization::Representability<NilFunctionId>,
    },
}

pub(super) enum SealedEntry {
    Int(IntFunctionId),
    Float(FloatFunctionId),
    String(StringFunctionId),
    BitArray(BitArrayFunctionId),
    UtfCodepoint(UtfCodepointFunctionId),
    Bool(BoolFunctionId),
    Nil(NilFunctionId),
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
    ints: Vec<IntFunctionId>,
    floats: Vec<FloatFunctionId>,
    strings: Vec<StringFunctionId>,
    bit_arrays: Vec<BitArrayFunctionId>,
    utf_codepoints: Vec<UtfCodepointFunctionId>,
    bools: Vec<BoolFunctionId>,
    nils: Vec<NilFunctionId>,
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
                .copied()
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
        match entry {
            LibraryEntry::Int(template) => Self::Int(template),
            LibraryEntry::Float(template) => Self::Float(template),
            LibraryEntry::String(template) => Self::String(template),
            LibraryEntry::BitArray(template) => Self::BitArray(template),
            LibraryEntry::UtfCodepoint(template) => Self::UtfCodepoint(template),
            LibraryEntry::Bool(template) => Self::Bool(template),
            LibraryEntry::Nil(template) => Self::Nil(template),
        }
    }
}

impl Entry {
    pub(super) fn template(self) -> crate::plan::FunctionTemplateId {
        match self {
            Self::Int(template)
            | Self::Float(template)
            | Self::String(template)
            | Self::BitArray(template)
            | Self::UtfCodepoint(template)
            | Self::Bool(template)
            | Self::Nil(template) => template,
        }
    }

    pub(super) fn reserve(self, context: &mut LoweringContext) -> ReservedEntry {
        let key = SpecializationKey::monomorphic(self.template());
        match self {
            Self::Int(_) => ReservedEntry::Int {
                id: context.reserve_int_entry(key.clone()),
                key,
            },
            Self::Float(_) => ReservedEntry::Float {
                id: context.reserve_float_entry(key.clone()),
                key,
            },
            Self::String(_) => ReservedEntry::String {
                id: context.reserve_string_entry(key.clone()),
                key,
            },
            Self::BitArray(_) => ReservedEntry::BitArray {
                id: context.reserve_bit_array_entry(key.clone()),
                key,
            },
            Self::UtfCodepoint(_) => ReservedEntry::UtfCodepoint {
                id: context.reserve_utf_codepoint_entry(key.clone()),
                key,
            },
            Self::Bool(_) => ReservedEntry::Bool {
                id: context.reserve_bool_entry(key.clone()),
                key,
            },
            Self::Nil(_) => ReservedEntry::Nil {
                id: context.reserve_nil_entry(key.clone()),
                key,
            },
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
            Self::Int { key, id } => (key, id.map(SealedEntry::Int)),
            Self::Float { key, id } => (key, id.map(SealedEntry::Float)),
            Self::String { key, id } => (key, id.map(SealedEntry::String)),
            Self::BitArray { key, id } => (key, id.map(SealedEntry::BitArray)),
            Self::UtfCodepoint { key, id } => (key, id.map(SealedEntry::UtfCodepoint)),
            Self::Bool { key, id } => (key, id.map(SealedEntry::Bool)),
            Self::Nil { key, id } => (key, id.map(SealedEntry::Nil)),
        }
    }
}

impl SealedEntry {
    pub(super) fn runtime_id<Graph: ExecutionGraphProfile>(
        &self,
    ) -> ProfiledRuntimeFunctionId<Graph> {
        ProfiledRuntimeFunctionId::Core(match self {
            Self::Int(function) => ProfiledCoreRuntimeFunctionId::Int(*function),
            Self::Float(function) => ProfiledCoreRuntimeFunctionId::Float(*function),
            Self::String(function) => ProfiledCoreRuntimeFunctionId::String(*function),
            Self::BitArray(function) => ProfiledCoreRuntimeFunctionId::BitArray(*function),
            Self::UtfCodepoint(function) => ProfiledCoreRuntimeFunctionId::UtfCodepoint(*function),
            Self::Bool(function) => ProfiledCoreRuntimeFunctionId::Bool(*function),
            Self::Nil(function) => ProfiledCoreRuntimeFunctionId::Nil(*function),
        })
    }
}

impl EntryIds {
    pub(super) fn push(&mut self, entry: SealedEntry) {
        match entry {
            SealedEntry::Int(function) => self.ints.push(function),
            SealedEntry::Float(function) => self.floats.push(function),
            SealedEntry::String(function) => self.strings.push(function),
            SealedEntry::BitArray(function) => self.bit_arrays.push(function),
            SealedEntry::UtfCodepoint(function) => self.utf_codepoints.push(function),
            SealedEntry::Bool(function) => self.bools.push(function),
            SealedEntry::Nil(function) => self.nils.push(function),
        }
    }

    pub(super) fn finish(self) -> LibraryFunctionEntries {
        LibraryFunctionEntries {
            ints: self.ints.into_boxed_slice(),
            floats: self.floats.into_boxed_slice(),
            strings: self.strings.into_boxed_slice(),
            bit_arrays: self.bit_arrays.into_boxed_slice(),
            utf_codepoints: self.utf_codepoints.into_boxed_slice(),
            bools: self.bools.into_boxed_slice(),
            nils: self.nils.into_boxed_slice(),
        }
    }
}

impl LoweringContext {
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
}
