use crate::host::{HostExternalEquality, HostExternalHashing, HostExternalInspection};
use crate::plan::execution::runtime::{OwnedRuntimeValueMetadata, RuntimeValueMetadata};
use crate::runtime::evaluated::EvaluatedValue;
use crate::runtime::retained_list::RetainedList;
use crate::runtime::state::list::ListValueId;
use crate::runtime::{RetainedValueRef, StoredRuntimeValue};
use bitvec::order::Msb0;
use bitvec::slice::BitSlice;
use bitvec::view::BitView;
use ecow::EcoString;
use num_bigint::BigInt;
use std::borrow::Cow;
use std::hash::{Hash, Hasher};
use std::sync::Arc;

mod context;
mod map;

pub use context::NativeValues;
pub use map::{NativeMap, NativeMapEntry};

/// An immutable native representation over retained source values.
///
/// Viewing a value does not change its source type or grant permission to
/// restore it as another type. Tuple composition retains existing views.
pub struct NativeValue(Representation);

enum Representation {
    Stored(StoredRuntimeValue),
    Symbol(EcoString),
    Tuple(Arc<[NativeValue]>),
    ListTuple {
        list: ListValueId,
        metadata: OwnedRuntimeValueMetadata,
    },
    Map(NativeMap),
    Closure(Arc<NativeClosure>),
}

struct NativeClosure {
    definition: &'static str,
    captures: Box<[NativeValue]>,
}

/// A native structural kind, before source-specific decoding.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NativeKind {
    Int,
    Float,
    Binary,
    Symbol,
    Tuple,
    List,
    Map,
    External,
    Function,
}

enum Node<'value> {
    Int(Cow<'value, BigInt>),
    Float(f64),
    Binary(&'value BitSlice<u8, Msb0>),
    Symbol(&'value str),
    Tuple(Sequence<'value>),
    List(Sequence<'value>),
    Map(&'value NativeMap),
    External(RetainedValueRef<'value>),
    Function(RetainedValueRef<'value>, RuntimeValueMetadata<'value>),
    Closure(&'value NativeClosure),
}

enum Sequence<'value> {
    Declared(&'value [NativeValue]),
    Tuple(&'value [EvaluatedValue], RuntimeValueMetadata<'value>),
    Custom {
        tag: &'value str,
        fields: &'value [EvaluatedValue],
        metadata: RuntimeValueMetadata<'value>,
    },
    List(RetainedList<ListValueId>, RuntimeValueMetadata<'value>),
}

#[expect(
    clippy::large_enum_variant,
    reason = "requested list items stay on the stack instead of allocating a box for each traversal step"
)]
enum Item<'value> {
    Declared(&'value NativeValue),
    Symbol(&'value str),
    Source(Cow<'value, EvaluatedValue>, RuntimeValueMetadata<'value>),
}

enum Items<'value> {
    Declared(std::slice::Iter<'value, NativeValue>),
    Tuple(
        std::slice::Iter<'value, EvaluatedValue>,
        RuntimeValueMetadata<'value>,
    ),
    Custom {
        tag: Option<&'value str>,
        fields: std::slice::Iter<'value, EvaluatedValue>,
        metadata: RuntimeValueMetadata<'value>,
    },
    List {
        list: RetainedList<ListValueId>,
        next: usize,
        metadata: RuntimeValueMetadata<'value>,
    },
}

impl Clone for NativeValue {
    fn clone(&self) -> Self {
        Self(match &self.0 {
            Representation::Stored(value) => Representation::Stored(value.clone_retained()),
            Representation::Symbol(value) => Representation::Symbol(value.clone()),
            Representation::Tuple(values) => Representation::Tuple(Arc::clone(values)),
            Representation::ListTuple { list, metadata } => Representation::ListTuple {
                list: list.clone(),
                metadata: metadata.clone(),
            },
            Representation::Map(value) => Representation::Map(value.clone()),
            Representation::Closure(value) => Representation::Closure(Arc::clone(value)),
        })
    }
}

impl NativeValue {
    /// Declares a symbol, distinct from a string or binary.
    pub fn symbol(value: impl Into<EcoString>) -> Self {
        Self(Representation::Symbol(value.into()))
    }

    /// Declares a tuple without changing its fields' retained source types.
    pub fn tuple(values: impl IntoIterator<Item = Self>) -> Self {
        Self(Representation::Tuple(values.into_iter().collect()))
    }

    /// Declares a map view without traversing or changing its retained entries.
    pub fn map(value: NativeMap) -> Self {
        Self(Representation::Map(value))
    }

    /// Declares an immutable one-argument native closure's symbolic representation.
    ///
    /// `definition` identifies one provider-owned closure body and must be
    /// globally qualified. Equal definitions and captures describe equal
    /// closures. This view does not grant typed Gleam invocation permission;
    /// invocation remains with the provider's sealed callable implementation.
    pub fn unary_closure(
        definition: &'static str,
        captures: impl IntoIterator<Item = Self>,
    ) -> Self {
        Self(Representation::Closure(Arc::new(NativeClosure {
            definition,
            captures: captures.into_iter().collect(),
        })))
    }

    pub(in crate::runtime) fn tuple_from_list(
        list: ListValueId,
        metadata: RuntimeValueMetadata<'_>,
    ) -> Self {
        Self(Representation::ListTuple {
            list,
            metadata: metadata.to_owned(),
        })
    }

    pub(crate) fn from_stored(value: StoredRuntimeValue) -> Self {
        Self(Representation::Stored(value))
    }

    pub(crate) fn find_source<Output>(
        &self,
        read: impl Fn(&StoredRuntimeValue) -> Option<Output>,
    ) -> Option<Output> {
        let Representation::Stored(value) = &self.0 else {
            return None;
        };
        if let Some(result) = read(value) {
            return Some(result);
        }
        let EvaluatedValue::External(external) = value.value() else {
            return None;
        };
        external.lease().native_view()?.find_source(read)
    }

    /// Returns the structural kind after following declared external views.
    pub fn kind(&self) -> NativeKind {
        self.with_node(|node| node.kind())
    }

    /// Retains a tuple or list element without visiting preceding elements.
    pub fn index(&self, index: usize) -> Option<Self> {
        self.with_node(|node| match node {
            Node::Tuple(values) | Node::List(values) => values.item(index).map(Item::retain),
            _ => None,
        })
    }

    /// Returns a tuple or list length without decoding its elements.
    pub fn len(&self) -> Option<usize> {
        self.with_node(|node| match node {
            Node::Tuple(values) | Node::List(values) => Some(values.len()),
            _ => None,
        })
    }

    /// Returns whether a tuple or list is empty.
    pub fn is_empty(&self) -> Option<bool> {
        self.len().map(|len| len == 0)
    }

    /// Reads a symbol, including the representations of Bool and Nil.
    pub fn as_symbol(&self) -> Option<EcoString> {
        self.with_node(|node| match node {
            Node::Symbol(value) => Some(value.into()),
            _ => None,
        })
    }

    /// Reads an integer or a Unicode codepoint's integer value.
    pub fn as_int(&self) -> Option<BigInt> {
        self.with_node(|node| match node {
            Node::Int(value) => Some(value.into_owned()),
            _ => None,
        })
    }

    /// Reads a float without converting an integer.
    pub fn as_float(&self) -> Option<f64> {
        self.with_node(|node| match node {
            Node::Float(value) => Some(value),
            _ => None,
        })
    }

    /// Retains a native map view without decoding its keys or values.
    pub fn as_map(&self) -> Option<NativeMap> {
        self.with_node(|node| match node {
            Node::Map(value) => Some(value.clone()),
            _ => None,
        })
    }

    /// Reads a byte-aligned UTF-8 binary as a string.
    pub fn as_string(&self) -> Option<EcoString> {
        self.with_node(|node| match node {
            Node::Binary(bits) => binary_string(bits),
            _ => None,
        })
    }

    /// Reads the exact bits of a string or bit array.
    pub fn as_bit_array(&self) -> Option<crate::BitArrayValue> {
        self.with_node(|node| match node {
            Node::Binary(bits) => Some(crate::BitArrayValue::from_evaluated(bits.to_bitvec())),
            _ => None,
        })
    }

    /// Returns a native binary's bit length without copying its contents.
    pub fn bit_len(&self) -> Option<usize> {
        self.with_node(|node| match node {
            Node::Binary(bits) => Some(bits.len()),
            _ => None,
        })
    }

    /// Compares native representations without changing exact source identity.
    pub fn source_equal(&self, context: &HostExternalEquality<'_>, other: &Self) -> bool {
        self.with_node(|left| other.with_node(|right| nodes_equal(left, right, context)))
    }

    /// Hashes consistently with native equality.
    pub fn source_hash(&self, context: &HostExternalHashing<'_>) -> u64 {
        self.with_node(|node| hash_node(node, context))
    }

    /// Inspects the native representation using Gleam value formatting.
    pub fn inspect(&self, context: &HostExternalInspection<'_>) -> EcoString {
        self.with_node(|node| inspect_node(node, context))
    }

    fn with_node<Output>(&self, read: impl FnOnce(Node<'_>) -> Output) -> Output {
        match &self.0 {
            Representation::Stored(value) => {
                with_source_node(value.value(), value.metadata(), read)
            }
            Representation::Symbol(value) => read(Node::Symbol(value)),
            Representation::Closure(value) => read(Node::Closure(value)),
            Representation::Tuple(values) => read(Node::Tuple(Sequence::Declared(values))),
            Representation::ListTuple { list, metadata } => read(Node::Tuple(Sequence::List(
                RetainedList::new(list.clone()),
                metadata.as_borrowed(),
            ))),
            Representation::Map(value) => read(Node::Map(value)),
        }
    }

    pub(crate) fn convert_tuple<Item>(
        &self,
        arity: usize,
        mut convert: impl FnMut(usize, Self) -> Option<Item>,
    ) -> Option<Box<[Item]>> {
        self.with_node(|node| match node {
            Node::Tuple(values) if values.len() == arity => values
                .into_items()
                .enumerate()
                .map(|(index, value)| convert(index, value.retain()))
                .collect(),
            _ => None,
        })
    }

    pub(crate) fn convert_list<Item>(
        &self,
        mut convert: impl FnMut(Self) -> Option<Item>,
    ) -> Option<Box<[Item]>> {
        self.with_node(|node| match node {
            Node::List(values) => values
                .into_items()
                .map(|value| convert(value.retain()))
                .collect(),
            _ => None,
        })
    }

    pub(crate) fn convert_custom<Field>(
        &self,
        constructors: &[crate::plan::execution::host::NativeConstructor],
        mut convert: impl FnMut(crate::plan::execution::host::NativeConversionId, Self) -> Option<Field>,
    ) -> Option<(
        crate::plan::execution::type_::CustomConstructorId,
        Box<[Field]>,
    )> {
        self.with_node(|node| match node {
            Node::Symbol(tag) => constructors
                .iter()
                .find(|constructor| constructor.tag() == tag && constructor.fields().is_empty())
                .map(|constructor| (constructor.constructor(), Box::new([]) as Box<[_]>)),
            Node::Tuple(values) => {
                let length = values.len();
                let mut items = values.into_items();
                let first = items.next()?;
                first.with_node(|node| {
                    let Node::Symbol(tag) = node else {
                        return None;
                    };
                    let constructor = constructors.iter().find(|constructor| {
                        !constructor.fields().is_empty()
                            && constructor.tag() == tag
                            && constructor.fields().len() + 1 == length
                    })?;
                    let fields = constructor
                        .fields()
                        .iter()
                        .zip(items)
                        .map(|(id, value)| convert(*id, value.retain()))
                        .collect::<Option<Box<[_]>>>()?;
                    Some((constructor.constructor(), fields))
                })
            }
            _ => None,
        })
    }
}

impl<'value> Iterator for Items<'value> {
    type Item = Item<'value>;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Declared(values) => values.next().map(Item::Declared),
            Self::Tuple(values, metadata) => values
                .next()
                .map(|value| Item::Source(Cow::Borrowed(value), *metadata)),
            Self::Custom {
                tag,
                fields,
                metadata,
            } => match tag.take() {
                Some(tag) => Some(Item::Symbol(tag)),
                None => fields
                    .next()
                    .map(|value| Item::Source(Cow::Borrowed(value), *metadata)),
            },
            Self::List {
                list,
                next,
                metadata,
            } => {
                let value = list.item(*next)?;
                *next += 1;
                Some(Item::Source(Cow::Owned(value), *metadata))
            }
        }
    }
}

impl<'value> Sequence<'value> {
    fn len(&self) -> usize {
        match self {
            Self::Declared(values) => values.len(),
            Self::Tuple(values, _) => values.len(),
            Self::Custom { fields, .. } => fields.len() + 1,
            Self::List(values, _) => values.len(),
        }
    }

    fn item(&self, index: usize) -> Option<Item<'_>> {
        match self {
            Self::Declared(values) => values.get(index).map(Item::Declared),
            Self::Tuple(values, metadata) => values
                .get(index)
                .map(|value| Item::Source(Cow::Borrowed(value), *metadata)),
            Self::Custom {
                tag,
                fields,
                metadata,
            } => match index.checked_sub(1) {
                None => Some(Item::Symbol(tag)),
                Some(index) => fields
                    .get(index)
                    .map(|value| Item::Source(Cow::Borrowed(value), *metadata)),
            },
            Self::List(values, metadata) => values
                .item(index)
                .map(|value| Item::Source(Cow::Owned(value), *metadata)),
        }
    }

    fn into_items(self) -> Items<'value> {
        match self {
            Self::Declared(values) => Items::Declared(values.iter()),
            Self::Tuple(values, metadata) => Items::Tuple(values.iter(), metadata),
            Self::Custom {
                tag,
                fields,
                metadata,
            } => Items::Custom {
                tag: Some(tag),
                fields: fields.iter(),
                metadata,
            },
            Self::List(list, metadata) => Items::List {
                list,
                next: 0,
                metadata,
            },
        }
    }
}

impl Item<'_> {
    fn with_node<Output>(&self, read: impl FnOnce(Node<'_>) -> Output) -> Output {
        match self {
            Self::Declared(value) => value.with_node(read),
            Self::Symbol(value) => read(Node::Symbol(value)),
            Self::Source(value, metadata) => with_source_node(value, *metadata, read),
        }
    }

    fn retain(self) -> NativeValue {
        match self {
            Self::Declared(value) => value.clone(),
            Self::Symbol(value) => NativeValue::symbol(value),
            Self::Source(value, metadata) => {
                NativeValue::from_stored(StoredRuntimeValue::new(value.into_owned(), metadata))
            }
        }
    }
}

pub(in crate::runtime) fn values_equal(
    storage: &crate::runtime::RuntimeListStorage,
    left: &NativeValue,
    right: &NativeValue,
) -> bool {
    let equal = |left: &RetainedValueRef, right: &RetainedValueRef| {
        crate::runtime::evaluated::values_equal(storage, left.value(), right.value())
    };
    let context = crate::host::RetainedValueEquality::new(&equal);
    left.source_equal(&HostExternalEquality(&context), right)
}

pub(in crate::runtime) fn value_hash(
    storage: &crate::runtime::RuntimeListStorage,
    value: &NativeValue,
) -> u64 {
    let hash = |value: &RetainedValueRef| {
        crate::runtime::evaluated::value_source_hash(storage, value.value())
    };
    let context = crate::host::RetainedValueHashing::new(&hash);
    value.source_hash(&HostExternalHashing(&context))
}

fn with_source_node<Output>(
    value: &EvaluatedValue,
    metadata: RuntimeValueMetadata<'_>,
    read: impl FnOnce(Node<'_>) -> Output,
) -> Output {
    let node = match value {
        EvaluatedValue::Int(value) => Node::Int(Cow::Borrowed(value)),
        EvaluatedValue::UtfCodepoint(value) => Node::Int(Cow::Owned(u32::from(*value).into())),
        EvaluatedValue::Float(value) => Node::Float(*value),
        EvaluatedValue::String(value) => Node::Binary(value.as_bytes().view_bits::<Msb0>()),
        EvaluatedValue::BitArray(value) => Node::Binary(value.bits()),
        EvaluatedValue::Bool(true) => Node::Symbol("true"),
        EvaluatedValue::Bool(false) => Node::Symbol("false"),
        EvaluatedValue::Nil => Node::Symbol("nil"),
        EvaluatedValue::Tuple(values) => Node::Tuple(Sequence::Tuple(values, metadata)),
        EvaluatedValue::List(list) => Node::List(Sequence::List(
            RetainedList::new(list.clone().into()),
            metadata,
        )),
        EvaluatedValue::ParameterList(list) => {
            Node::List(Sequence::List(RetainedList::new((*list).into()), metadata))
        }
        EvaluatedValue::Custom(value) => {
            let constructor = metadata.custom_constructor(value.constructor());
            if value.fields().is_empty() {
                Node::Symbol(constructor.native_tag())
            } else {
                Node::Tuple(Sequence::Custom {
                    tag: constructor.native_tag(),
                    fields: value.fields(),
                    metadata,
                })
            }
        }
        EvaluatedValue::Function(_) => Node::Function(RetainedValueRef::new(value), metadata),
        EvaluatedValue::External(external) => {
            // Projection ends payload access before traversal or semantic callbacks.
            if let Some(view) = external.lease().native_view() {
                return view.with_node(read);
            }
            Node::External(RetainedValueRef::new(value))
        }
    };
    read(node)
}

impl Node<'_> {
    fn kind(&self) -> NativeKind {
        match self {
            Self::Int(_) => NativeKind::Int,
            Self::Float(_) => NativeKind::Float,
            Self::Binary(_) => NativeKind::Binary,
            Self::Symbol(_) => NativeKind::Symbol,
            Self::Tuple(_) => NativeKind::Tuple,
            Self::List(_) => NativeKind::List,
            Self::Map(_) => NativeKind::Map,
            Self::External(_) => NativeKind::External,
            Self::Function(..) | Self::Closure(_) => NativeKind::Function,
        }
    }
}

fn nodes_equal(left: Node<'_>, right: Node<'_>, context: &HostExternalEquality<'_>) -> bool {
    match (left, right) {
        (Node::Int(left), Node::Int(right)) => left == right,
        (Node::Float(left), Node::Float(right)) => left == right,
        (Node::Binary(left), Node::Binary(right)) => left == right,
        (Node::Symbol(left), Node::Symbol(right)) => left == right,
        (Node::Tuple(left), Node::Tuple(right)) | (Node::List(left), Node::List(right)) => {
            left.len() == right.len()
                && left
                    .into_items()
                    .zip(right.into_items())
                    .all(|(left, right)| {
                        left.with_node(|left| {
                            right.with_node(|right| nodes_equal(left, right, context))
                        })
                    })
        }
        (Node::External(left), Node::External(right)) => {
            context.0.stored_values_equal(&left, &right)
        }
        (Node::Map(left), Node::Map(right)) => {
            left.len() == right.len()
                && left.entries().all(|entry| {
                    right
                        .get(entry.key_hash, &entry.key, |left, right| {
                            left.source_equal(context, right)
                        })
                        .is_some_and(|value| entry.value.source_equal(context, &value))
                })
        }
        (Node::Function(left, left_owner), Node::Function(right, right_owner)) => {
            left_owner.shares_owner(right_owner) && context.0.stored_values_equal(&left, &right)
        }
        (Node::Closure(left), Node::Closure(right)) => {
            left.definition == right.definition
                && left.captures.len() == right.captures.len()
                && left
                    .captures
                    .iter()
                    .zip(right.captures.iter())
                    .all(|(left, right)| left.source_equal(context, right))
        }
        _ => false,
    }
}

fn hash_node(node: Node<'_>, context: &HostExternalHashing<'_>) -> u64 {
    let mut hash = std::collections::hash_map::DefaultHasher::new();
    node.kind().hash(&mut hash);
    match node {
        Node::Int(value) => value.hash(&mut hash),
        Node::Float(value) => (if value == 0.0 { 0 } else { value.to_bits() }).hash(&mut hash),
        Node::Binary(value) => value.hash(&mut hash),
        Node::Symbol(value) => value.hash(&mut hash),
        Node::Tuple(values) | Node::List(values) => {
            values.len().hash(&mut hash);
            for value in values.into_items() {
                value
                    .with_node(|node| hash_node(node, context))
                    .hash(&mut hash);
            }
        }
        Node::External(value) | Node::Function(value, _) => {
            context.0.stored_value_hash(&value).hash(&mut hash)
        }
        Node::Closure(value) => {
            value.definition.hash(&mut hash);
            value.captures.len().hash(&mut hash);
            for value in &value.captures {
                value.source_hash(context).hash(&mut hash);
            }
        }
        Node::Map(value) => {
            let mut sum = 0u64;
            let mut xor = 0u64;
            for entry in value.entries() {
                let mut entry_hash = std::collections::hash_map::DefaultHasher::new();
                entry.key_hash.hash(&mut entry_hash);
                entry.value.source_hash(context).hash(&mut entry_hash);
                let entry_hash = entry_hash.finish();
                sum = sum.wrapping_add(entry_hash);
                xor ^= entry_hash.rotate_left(29);
            }
            value.len().hash(&mut hash);
            sum.hash(&mut hash);
            xor.hash(&mut hash);
        }
    }
    hash.finish()
}

fn binary_string(bits: &BitSlice<u8, Msb0>) -> Option<EcoString> {
    if !bits.len().is_multiple_of(8) {
        return None;
    }
    let bytes = bits
        .chunks_exact(8)
        .map(|byte| {
            byte.iter()
                .by_vals()
                .fold(0u8, |value, bit| (value << 1) | u8::from(bit))
        })
        .collect::<Vec<_>>();
    std::str::from_utf8(&bytes).ok().map(Into::into)
}

fn inspect_node(node: Node<'_>, context: &HostExternalInspection<'_>) -> EcoString {
    match node {
        Node::Int(value) => value.to_string().into(),
        Node::Float(value) => crate::Value::Float(value).inspect().to_string().into(),
        Node::Binary(bits) => match binary_string(bits) {
            Some(value) => crate::Value::String(value).inspect().to_string().into(),
            None => crate::Value::BitArray(crate::BitArrayValue::from_evaluated(bits.to_bitvec()))
                .inspect()
                .to_string()
                .into(),
        },
        Node::Symbol(value) => match value {
            "true" => "True".into(),
            "false" => "False".into(),
            "nil" => "Nil".into(),
            value => symbol_constructor(value)
                .unwrap_or_else(|| format!("atom.create(\"{value}\")").into()),
        },
        Node::Tuple(values) => {
            let mut items = values.into_items();
            let first = items.next();
            let constructor = first.as_ref().and_then(|first| {
                first.with_node(|node| match node {
                    Node::Symbol(value) if !matches!(value, "true" | "false" | "nil") => {
                        symbol_constructor(value)
                    }
                    _ => None,
                })
            });
            match constructor {
                Some(constructor) => inspect_items(items, &format!("{constructor}("), ')', context),
                None => inspect_items(first.into_iter().chain(items), "#(", ')', context),
            }
        }
        Node::List(values) => inspect_list(values, context),
        Node::Map(value) => {
            let mut entries = value
                .entries()
                .map(|entry| {
                    format!(
                        "#({}, {})",
                        entry.key.inspect(context),
                        entry.value.inspect(context)
                    )
                })
                .collect::<Vec<_>>();
            entries.sort_unstable();
            format!("dict.from_list([{}])", entries.join(", ")).into()
        }
        Node::External(value) | Node::Function(value, _) => context.0.inspect_stored_value(&value),
        Node::Closure(_) => "//fn(a) { ... }".into(),
    }
}

fn symbol_constructor(value: &str) -> Option<EcoString> {
    let mut name = EcoString::new();
    for (index, part) in value.split('_').enumerate() {
        let first = part.bytes().next()?;
        if (index == 0 && !first.is_ascii_lowercase())
            || !part
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit())
        {
            return None;
        }
        name.push(char::from(first.to_ascii_uppercase()));
        name.push_str(&part[1..]);
    }
    Some(name)
}

fn inspect_list(values: Sequence<'_>, context: &HostExternalInspection<'_>) -> EcoString {
    let mut text = String::from("[");
    let mut chars = (values.len() != 0).then(String::new);
    for (index, item) in values.into_items().enumerate() {
        if index != 0 {
            text.push_str(", ");
        }
        item.with_node(|node| {
            if let Some(characters) = chars.as_mut() {
                let byte = match &node {
                    Node::Int(value) => u8::try_from(value.as_ref()).ok(),
                    _ => None,
                };
                match byte {
                    Some(byte @ 32..=126) => characters.push(char::from(byte)),
                    _ => chars = None,
                }
            }
            text.push_str(&inspect_node(node, context));
        });
    }
    text.push(']');
    match chars {
        Some(chars) => format!("charlist.from_string(\"{chars}\")").into(),
        None => text.into(),
    }
}

fn inspect_items<'value>(
    values: impl Iterator<Item = Item<'value>>,
    start: &str,
    end: char,
    context: &HostExternalInspection<'_>,
) -> EcoString {
    let mut text = String::from(start);
    for (index, value) in values.enumerate() {
        if index != 0 {
            text.push_str(", ");
        }
        text.push_str(&value.with_node(|node| inspect_node(node, context)));
    }
    text.push(end);
    text.into()
}

#[cfg(test)]
fn native_source(source: &str) -> NativeValue {
    use crate::plan::execution::function::ProfiledRuntimeFunctionId;
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    let plan = crate::runtime::plan_src(source);
    let mut echo = Vec::new();
    let mut state = crate::runtime::state::RuntimeState::new(&mut echo);
    let entry = match RuntimeExecutionPlan::main_runtime(&plan) {
        ProfiledRuntimeFunctionId::Core(entry) => entry,
        ProfiledRuntimeFunctionId::External(never) => match never {},
    };
    let value = crate::runtime::run_core_program(
        &plan,
        &mut state,
        entry,
        crate::runtime::RetainedValues::empty(),
    )
    .expect("source should execute");
    NativeValue::from_stored(StoredRuntimeValue::new(value, plan.value_metadata()))
}

#[cfg(test)]
mod tests {
    use super::{NativeKind, NativeValue, native_source as source};
    use crate::host::{
        HostExternalEquality, HostExternalHashing, HostExternalInspection, RetainedValueEquality,
        RetainedValueHashing, RetainedValueInspection,
    };
    use crate::runtime::RetainedValueRef;
    use ecow::EcoString;

    fn opaque_equal(left: &RetainedValueRef, right: &RetainedValueRef) -> bool {
        crate::runtime::evaluated::values_equal(
            &crate::runtime::RuntimeListStorage::default(),
            left.value(),
            right.value(),
        )
    }

    fn opaque_hash(value: &RetainedValueRef) -> u64 {
        crate::runtime::evaluated::value_source_hash(
            &crate::runtime::RuntimeListStorage::default(),
            value.value(),
        )
    }

    fn opaque_inspection(_: &RetainedValueRef) -> EcoString {
        "opaque".into()
    }

    #[test]
    fn declared_tuples_and_symbols_match_source_values_without_relaxing_exact_types() {
        let source = source("pub fn main() { #(True, False, Nil, 42, 2.5) }");
        let declared = NativeValue::tuple([
            NativeValue::symbol("true"),
            NativeValue::symbol("false"),
            NativeValue::symbol("nil"),
            source.index(3).expect("integer"),
            source.index(4).expect("float"),
        ]);
        let equality = RetainedValueEquality::new(&opaque_equal);
        let hashing = RetainedValueHashing::new(&opaque_hash);
        let inspection = RetainedValueInspection::new(&opaque_inspection);
        let equality = HostExternalEquality(&equality);
        let hashing = HostExternalHashing(&hashing);
        let inspection = HostExternalInspection(&inspection);
        assert!(source.source_equal(&equality, &declared));
        assert!(declared.source_equal(&equality, &source));
        assert_eq!(source.source_hash(&hashing), declared.source_hash(&hashing));
        assert_eq!(source.inspect(&inspection), "#(True, False, Nil, 42, 2.5)");
        assert_eq!(source.inspect(&inspection), declared.inspect(&inspection));
        assert_eq!(source.kind(), NativeKind::Tuple);
        assert_eq!(source.len(), Some(5));
        assert_eq!(source.is_empty(), Some(false));
        assert!(source.index(5).is_none());
        assert_eq!(source.index(3).unwrap().as_int(), Some(42.into()));
        assert_eq!(source.index(4).unwrap().as_float(), Some(2.5));
        assert_eq!(
            declared.index(0).unwrap().as_symbol().as_deref(),
            Some("true")
        );
        assert_eq!(declared.index(3).unwrap().as_int(), Some(42.into()));
        assert!(declared.index(5).is_none());
        let boolean = source.index(0).unwrap();
        assert_eq!(boolean.as_symbol().as_deref(), Some("true"));
        assert!(boolean.as_int().is_none());
        assert!(boolean.as_float().is_none());
        assert!(boolean.as_string().is_none());
        assert!(boolean.as_bit_array().is_none());
        assert!(boolean.index(0).is_none());
        assert!(boolean.len().is_none());
        assert_eq!(
            boolean.find_source(|value| Some((value.type_().clone(), value.value().clone()))),
            Some((
                crate::plan::ValueType::Bool,
                crate::runtime::EvaluatedValue::Bool(true)
            ))
        );
        assert_eq!(
            declared.clone().inspect(&inspection),
            declared.inspect(&inspection)
        );
    }

    #[test]
    fn native_binary_preserves_bits_and_checks_utf8_without_changing_source_storage() {
        let values =
            source(r#"pub fn main() { #("hello", <<"hello":utf8>>, <<255>>, <<1:size(1)>>) }"#);
        let string = values.index(0).unwrap();
        let binary = values.index(1).unwrap();
        let invalid = values.index(2).unwrap();
        let partial = values.index(3).unwrap();
        let equality = RetainedValueEquality::new(&opaque_equal);
        let hashing = RetainedValueHashing::new(&opaque_hash);
        let inspection = RetainedValueInspection::new(&opaque_inspection);
        let equality = HostExternalEquality(&equality);
        let hashing = HostExternalHashing(&hashing);
        let inspection = HostExternalInspection(&inspection);
        assert!(string.source_equal(&equality, &binary));
        assert!(binary.source_equal(&equality, &string));
        assert_eq!(string.source_hash(&hashing), binary.source_hash(&hashing));
        assert_eq!(string.inspect(&inspection), binary.inspect(&inspection));
        assert_eq!(string.inspect(&inspection), "\"hello\"");
        assert_eq!(string.kind(), NativeKind::Binary);
        assert_eq!(binary.as_string().as_deref(), Some("hello"));
        assert_eq!(binary.as_bit_array().unwrap().bytes(), b"hello");
        assert!(invalid.as_string().is_none());
        assert!(partial.as_string().is_none());
        assert_eq!(partial.as_bit_array().unwrap().bit_len(), 1);
        assert!(!invalid.source_equal(&equality, &partial));
        assert_eq!(invalid.inspect(&inspection), "<<255>>");
        assert!(binary.as_symbol().is_none());
        assert_eq!(binary.bit_len(), Some(40));
        assert_eq!(partial.bit_len(), Some(1));
        assert_eq!(NativeValue::symbol("binary").bit_len(), None);
    }

    #[test]
    fn native_scalars_compare_across_source_owners_without_coercing_distinct_kinds() {
        let float = source("pub fn main() { 2.5 }");
        let same_float = source("pub fn main() { 2.5 }");
        let integer = source("pub fn main() { 2 }");
        let string = source("pub fn main() { \"hello\" }");
        let same_string = source("pub fn main() { \"hello\" }");
        let different_string = source("pub fn main() { \"world\" }");
        let true_value = source("pub fn main() { True }");
        let false_value = source("pub fn main() { False }");
        let context = RetainedValueEquality::new(&opaque_equal);
        let context = HostExternalEquality(&context);
        assert!(float.source_equal(&context, &same_float));
        assert!(!float.source_equal(&context, &integer));
        assert!(string.source_equal(&context, &same_string));
        assert!(!string.source_equal(&context, &different_string));
        assert!(!string.source_equal(&context, &NativeValue::symbol("hello")));
        assert!(true_value.source_equal(&context, &NativeValue::symbol("true")));
        assert!(false_value.source_equal(&context, &NativeValue::symbol("false")));
        assert!(!true_value.source_equal(&context, &false_value));
    }

    #[test]
    fn native_lists_remain_lists_and_keep_nested_values_after_the_plan_is_dropped() {
        let values = source("pub fn main() { #([[1, 2], [3]], [1, 2], #(1, 2), []) }");
        let list = values.index(1).unwrap();
        let tuple = values.index(2).unwrap();
        let nested = values.index(0).unwrap();
        let empty = values.index(3).unwrap();
        drop(values);
        let equality = RetainedValueEquality::new(&opaque_equal);
        let inspection = RetainedValueInspection::new(&opaque_inspection);
        let equality = HostExternalEquality(&equality);
        let inspection = HostExternalInspection(&inspection);
        assert_eq!(list.kind(), NativeKind::List);
        assert_eq!(empty.is_empty(), Some(true));
        assert!(empty.index(0).is_none());
        assert!(!list.source_equal(&equality, &tuple));
        assert!(!empty.source_equal(&equality, &list));
        assert!(list.source_equal(&equality, &list));
        assert_eq!(nested.inspect(&inspection), "[[1, 2], [3]]");
        assert_eq!(
            nested.index(1).unwrap().index(0).unwrap().as_int(),
            Some(3.into())
        );
        let copied = list.clone();
        assert_eq!(
            list.find_source(|left| copied
                .find_source(|right| { Some(std::ptr::eq(left.value(), right.value())) })),
            Some(true)
        );
        assert!(list.index(2).is_none());
    }

    #[test]
    fn custom_views_use_frozen_tags_and_specialized_fields_without_erasing_exact_identity() {
        let value = source(
            r#"
pub type Packet(a) { Empty Packet(value: a, rest: List(a)) }
pub type Other { PacketAlias(value: Int) }
pub fn main() {
  let empty: Packet(Int) = Empty
  #(Packet(42, [1, 2]), Packet("text", ["more"]), empty, PacketAlias(42))
}
"#,
        );
        let integer = value.index(0).unwrap();
        let string = value.index(1).unwrap();
        let empty = value.index(2).unwrap();
        let other = value.index(3).unwrap();
        drop(value);
        let equal = RetainedValueEquality::new(&opaque_equal);
        let hash = RetainedValueHashing::new(&opaque_hash);
        let inspect = RetainedValueInspection::new(&opaque_inspection);
        let equal = HostExternalEquality(&equal);
        let hash = HostExternalHashing(&hash);
        let inspect = HostExternalInspection(&inspect);
        let tuple = NativeValue::tuple([
            NativeValue::symbol("packet"),
            integer.index(1).unwrap(),
            integer.index(2).unwrap(),
        ]);
        assert_eq!(integer.kind(), NativeKind::Tuple);
        assert_eq!(integer.len(), Some(3));
        assert_eq!(
            integer.index(0).unwrap().as_symbol().as_deref(),
            Some("packet")
        );
        assert_eq!(integer.index(1).unwrap().as_int(), Some(42.into()));
        assert_eq!(
            string.index(1).unwrap().as_string().as_deref(),
            Some("text")
        );
        assert_eq!(empty.as_symbol().as_deref(), Some("empty"));
        assert!(empty.index(0).is_none());
        assert!(integer.index(3).is_none());
        assert!(integer.source_equal(&equal, &tuple));
        assert!(tuple.source_equal(&equal, &integer));
        assert!(!integer.source_equal(&equal, &string));
        assert!(!integer.source_equal(&equal, &other));
        assert_eq!(integer.source_hash(&hash), tuple.source_hash(&hash));
        assert_eq!(integer.inspect(&inspect), "Packet(42, [1, 2])");
        assert_eq!(other.inspect(&inspect), "PacketAlias(42)");
        for (value, argument) in [
            (integer, crate::plan::ValueType::Int),
            (string, crate::plan::ValueType::String),
        ] {
            assert_eq!(
                value.find_source(|value| Some(value.type_().clone())),
                Some(crate::plan::ValueType::Custom(
                    crate::plan::CustomType::new(
                        crate::plan::CustomTypeName::new(
                            "geam".into(),
                            "main".into(),
                            "Packet".into()
                        ),
                        vec![argument],
                    )
                ))
            );
        }
    }

    #[test]
    fn native_function_identity_keeps_its_owner_and_delegates_opaque_operations() {
        let values = source(
            "fn inc(value) { value + 1 } pub fn main() { #(inc, inc, fn(value) { value + 2 }) }",
        );
        let first = values.index(0).unwrap();
        let alias = values.index(1).unwrap();
        let other = values.index(2).unwrap();
        let foreign = source("fn inc(value) { value + 1 } pub fn main() { inc }");
        let equality = RetainedValueEquality::new(&opaque_equal);
        let hashing = RetainedValueHashing::new(&opaque_hash);
        let inspection = RetainedValueInspection::new(&opaque_inspection);
        let equality = HostExternalEquality(&equality);
        let hashing = HostExternalHashing(&hashing);
        let inspection = HostExternalInspection(&inspection);
        assert_eq!(first.kind(), NativeKind::Function);
        assert!(first.source_equal(&equality, &alias));
        assert!(!first.source_equal(&equality, &other));
        assert!(!first.source_equal(&equality, &foreign));
        assert_eq!(first.source_hash(&hashing), alias.source_hash(&hashing));
        assert_eq!(first.inspect(&inspection), "opaque");
        let storage = crate::runtime::RuntimeListStorage::default();
        assert!(super::values_equal(&storage, &first, &alias));
        assert_eq!(
            super::value_hash(&storage, &first),
            super::value_hash(&storage, &alias)
        );
    }

    #[test]
    fn declared_native_closures_keep_definition_and_capture_semantics_without_invocation() {
        let equality = RetainedValueEquality::new(&opaque_equal);
        let hashing = RetainedValueHashing::new(&opaque_hash);
        let inspection = RetainedValueInspection::new(&opaque_inspection);
        let equality = HostExternalEquality(&equality);
        let hashing = HostExternalHashing(&hashing);
        let inspection = HostExternalInspection(&inspection);
        let value = NativeValue::unary_closure("provider/module:map", [NativeValue::symbol("one")]);
        let alias = value.clone();
        let same = NativeValue::unary_closure("provider/module:map", [NativeValue::symbol("one")]);
        let different_capture =
            NativeValue::unary_closure("provider/module:map", [NativeValue::symbol("two")]);
        let different_body =
            NativeValue::unary_closure("provider/module:other", [NativeValue::symbol("one")]);
        let no_capture = NativeValue::unary_closure("provider/module:map", []);
        let function = source("pub fn main() { fn(value: Int) { value } }");
        assert_eq!(value.kind(), NativeKind::Function);
        assert_eq!(value.inspect(&inspection), "//fn(a) { ... }");
        for same in [&alias, &same] {
            assert!(value.source_equal(&equality, same));
            assert!(same.source_equal(&equality, &value));
            assert_eq!(value.source_hash(&hashing), same.source_hash(&hashing));
        }
        for different in [&different_capture, &different_body, &no_capture, &function] {
            assert!(!value.source_equal(&equality, different));
            assert!(!different.source_equal(&equality, &value));
        }
        let source_type = |value: &crate::runtime::StoredRuntimeValue| Some(value.type_().clone());
        assert_eq!(value.find_source(source_type), None);
        assert!(function.find_source(source_type).is_some());
        assert!(value.index(0).is_none());
        assert_eq!(value.len(), None);
    }

    #[test]
    fn native_inspection_matches_symbol_charlist_and_scalar_grammar() {
        let inspection = RetainedValueInspection::new(&opaque_inspection);
        let inspection = HostExternalInspection(&inspection);
        for (symbol, expected) in [
            ("", "atom.create(\"\")"),
            ("trailing_", "atom.create(\"trailing_\")"),
            ("double__part", "atom.create(\"double__part\")"),
            ("Upper", "atom.create(\"Upper\")"),
            ("some_2d_value", "Some2dValue"),
            ("a\"b\\c\n", "atom.create(\"a\"b\\c\n\")"),
        ] {
            assert_eq!(NativeValue::symbol(symbol).inspect(&inspection), expected);
        }
        assert_eq!(
            source("pub fn main() { [65, 90, 32, 126] }").inspect(&inspection),
            "charlist.from_string(\"AZ ~\")"
        );
        assert_eq!(
            source("pub fn main() { [65, 256] }").inspect(&inspection),
            "[65, 256]"
        );
        let values = source(
            "pub fn main() { let assert <<cp:utf8_codepoint>> = <<65>> #(cp, 65, 0.0, -0.0) }",
        );
        let codepoint = values.index(0).unwrap();
        let integer = values.index(1).unwrap();
        let storage = crate::runtime::RuntimeListStorage::default();
        assert_eq!(codepoint.kind(), NativeKind::Int);
        assert_eq!(codepoint.as_int(), Some(65.into()));
        assert!(super::values_equal(&storage, &codepoint, &integer));
        assert_eq!(
            super::value_hash(&storage, &codepoint),
            super::value_hash(&storage, &integer)
        );
        let zero = values.index(2).unwrap();
        let negative = values.index(3).unwrap();
        assert!(super::values_equal(&storage, &zero, &negative));
        assert_eq!(
            super::value_hash(&storage, &zero),
            super::value_hash(&storage, &negative)
        );
    }
}
