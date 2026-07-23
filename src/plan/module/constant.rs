mod function;
mod list;

use super::{
    BitArrayBitsSize, BitArrayExpr, BitArrayFunctionExpr, BitArrayListExpr, BitArrayListItem,
    BitArraySegment, BoolExpr, BoolFunctionExpr, BoolListExpr, BoolListItem, CustomConstruction,
    CustomConstructor, CustomExpr, CustomFunctionExpr, CustomListExpr, CustomListItem, Expr,
    FloatExpr, FloatFunctionExpr, FloatListExpr, FloatListItem, FunctionExpr, FunctionFunctionExpr,
    FunctionListExpr, FunctionListItem, FunctionReference, GenericFunctionExpr, GenericListExpr,
    GenericListItem, IntExpr, IntFunctionExpr, IntListExpr, IntListItem, ListExpr,
    ListFunctionExpr, ListListExpr, ListListItem, NilExpr, NilFunctionExpr, NilListExpr,
    NilListItem, ParameterListListExpr, ParameterListListItem, StoredListExpr, StringExpr,
    StringFunctionExpr, StringListExpr, StringListItem, TupleExpr, TupleFunctionExpr,
    TupleListExpr, TupleListItem, TypeScheme, TypeSubstitution, TypedFunctionReference,
    UtfCodepointFunctionExpr, UtfCodepointListExpr, UtfCodepointListItem,
};
use crate::plan::{
    CustomValueShape, Endianness, FloatBitSize, FunctionShape, PanicSite, StringEncoding,
    ValueShape, ValueType,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) use function::{
    ConstantBitArrayFunctionInstantiation, ConstantBoolFunctionInstantiation,
    ConstantCustomFunctionInstantiation, ConstantFloatFunctionInstantiation,
    ConstantFunctionFunctionInstantiation, ConstantFunctionInstantiation,
    ConstantFunctionTemplateSource, ConstantGenericFunctionInstantiation,
    ConstantIntFunctionInstantiation, ConstantListFunctionInstantiation,
    ConstantNilFunctionInstantiation, ConstantStringFunctionInstantiation,
    ConstantTupleFunctionInstantiation, ConstantUtfCodepointFunctionInstantiation,
};
use function::{
    ConstantBitArrayFunctionTemplateId, ConstantBitArrayFunctionValue,
    ConstantBoolFunctionTemplateId, ConstantBoolFunctionValue, ConstantCustomFunctionTarget,
    ConstantCustomFunctionTemplateId, ConstantCustomFunctionValue, ConstantFloatFunctionTemplateId,
    ConstantFloatFunctionValue, ConstantFunctionFunctionTemplateId, ConstantFunctionFunctionValue,
    ConstantFunctionTemplate, ConstantFunctionValue, ConstantGenericFunctionTemplateId,
    ConstantGenericFunctionValue, ConstantIntFunctionTemplateId, ConstantIntFunctionValue,
    ConstantListFunctionTemplateId, ConstantListFunctionValue, ConstantNilFunctionTemplateId,
    ConstantNilFunctionValue, ConstantStringFunctionTemplateId, ConstantStringFunctionValue,
    ConstantTupleFunctionTemplateId, ConstantTupleFunctionValue,
    ConstantUtfCodepointFunctionTemplateId, ConstantUtfCodepointFunctionValue,
    TypedConstantFunctionValueKind,
};

pub(crate) use list::{
    ConstantBitArrayListInstantiation, ConstantBitArrayListTemplateId,
    ConstantBoolListInstantiation, ConstantBoolListTemplateId, ConstantCustomListInstantiation,
    ConstantCustomListTemplateId, ConstantFloatListInstantiation, ConstantFloatListTemplateId,
    ConstantFunctionListInstantiation, ConstantFunctionListTemplateId,
    ConstantGenericListInstantiation, ConstantGenericListTemplateId, ConstantIntListInstantiation,
    ConstantIntListTemplateId, ConstantListInstantiation, ConstantListListInstantiation,
    ConstantListListTemplateId, ConstantListTemplateSource, ConstantNestedListTemplateSource,
    ConstantNilListInstantiation, ConstantNilListTemplateId,
    ConstantParameterListListInstantiation, ConstantParameterListListTemplateId,
    ConstantStringListInstantiation, ConstantStringListTemplateId, ConstantTupleListInstantiation,
    ConstantTupleListTemplateId, ConstantUtfCodepointListInstantiation,
    ConstantUtfCodepointListTemplateId,
};
use list::{
    ConstantBitArrayListValue, ConstantBoolListValue, ConstantCustomListValue,
    ConstantFloatListValue, ConstantFunctionListValue, ConstantGenericListValue,
    ConstantGenericListValueKind, ConstantIntListValue, ConstantListListValue, ConstantListParts,
    ConstantListTemplate, ConstantListValue, ConstantNilListValue, ConstantParameterListListValue,
    ConstantStoredListValue, ConstantStringListValue, ConstantTupleListValue,
    ConstantUtfCodepointListValue, ConstantUtfCodepointListValueKind, TypedConstantListValueKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ConstantTemplateId {
    module: crate::plan::ModuleId,
    index: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ConstantTemplate {
    signature: ConstantTemplateSignature,
    name: EcoString,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConstantTemplates {
    module: crate::plan::ModuleId,
    headers: Vec<ConstantTemplate>,
    generic_lists: Vec<ConstantGenericListValue>,
    ints: Vec<ConstantIntValue>,
    strings: Vec<ConstantStringValue>,
    bit_arrays: Vec<ConstantBitArrayValue>,
    custom_values: Vec<ConstantCustomValue>,
    floats: Vec<ConstantFloatValue>,
    bools: Vec<ConstantBoolValue>,
    nils: Vec<ConstantNilValue>,
    tuples: Vec<ConstantTupleValue>,
    int_lists: Vec<ConstantIntListValue>,
    string_lists: Vec<ConstantStringListValue>,
    bit_array_lists: Vec<ConstantBitArrayListValue>,
    utf_codepoint_lists: Vec<ConstantUtfCodepointListValue>,
    custom_lists: Vec<ConstantCustomListValue>,
    float_lists: Vec<ConstantFloatListValue>,
    bool_lists: Vec<ConstantBoolListValue>,
    nil_lists: Vec<ConstantNilListValue>,
    tuple_lists: Vec<ConstantTupleListValue>,
    parameter_list_lists: Vec<ConstantParameterListListValue>,
    list_lists: Vec<ConstantListListValue>,
    function_lists: Vec<ConstantFunctionListValue>,
    generic_functions: Vec<ConstantGenericFunctionValue>,
    int_functions: Vec<ConstantIntFunctionValue>,
    string_functions: Vec<ConstantStringFunctionValue>,
    bit_array_functions: Vec<ConstantBitArrayFunctionValue>,
    utf_codepoint_functions: Vec<ConstantUtfCodepointFunctionValue>,
    custom_functions: Vec<ConstantCustomFunctionValue>,
    float_functions: Vec<ConstantFloatFunctionValue>,
    bool_functions: Vec<ConstantBoolFunctionValue>,
    nil_functions: Vec<ConstantNilFunctionValue>,
    tuple_functions: Vec<ConstantTupleFunctionValue>,
    list_functions: Vec<ConstantListFunctionValue>,
    function_functions: Vec<ConstantFunctionFunctionValue>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ConstantTemplateSignature {
    id: ConstantTemplateId,
    scheme: TypeScheme,
    shape: ValueShape,
    kind: ConstantTemplateSignatureKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ConstantTemplateSignatureKind {
    Int(ConstantIntTemplateId),
    String(ConstantStringTemplateId),
    BitArray(ConstantBitArrayTemplateId),
    Custom {
        template: ConstantCustomTemplateId,
        shape: CustomValueShape,
    },
    Float(ConstantFloatTemplateId),
    Bool(ConstantBoolTemplateId),
    Nil(ConstantNilTemplateId),
    Tuple {
        template: ConstantTupleTemplateId,
        shape: Box<[ValueShape]>,
    },
    List(ConstantListTemplate),
    Function {
        template: ConstantFunctionTemplate,
        shape: FunctionShape,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantIntTemplateId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantStringTemplateId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantBitArrayTemplateId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantCustomTemplateId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantFloatTemplateId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantBoolTemplateId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantNilTemplateId(pub(crate) usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) struct ConstantTupleTemplateId(pub(crate) usize);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct TypedConstantInstantiation<Id, Shape> {
    module: crate::plan::ModuleId,
    template: Id,
    substitution: TypeSubstitution,
    shape: Shape,
}

pub(crate) type ConstantIntInstantiation = TypedConstantInstantiation<ConstantIntTemplateId, ()>;
pub(crate) type ConstantStringInstantiation =
    TypedConstantInstantiation<ConstantStringTemplateId, ()>;
pub(crate) type ConstantBitArrayInstantiation =
    TypedConstantInstantiation<ConstantBitArrayTemplateId, ()>;
pub(crate) type ConstantCustomInstantiation =
    TypedConstantInstantiation<ConstantCustomTemplateId, CustomValueShape>;
pub(crate) type ConstantFloatInstantiation =
    TypedConstantInstantiation<ConstantFloatTemplateId, ()>;
pub(crate) type ConstantBoolInstantiation = TypedConstantInstantiation<ConstantBoolTemplateId, ()>;
pub(crate) type ConstantNilInstantiation = TypedConstantInstantiation<ConstantNilTemplateId, ()>;
pub(crate) type ConstantTupleInstantiation =
    TypedConstantInstantiation<ConstantTupleTemplateId, Box<[ValueShape]>>;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ConstantInstantiation {
    kind: ConstantInstantiationKind,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ConstantInstantiationKind {
    Int(ConstantIntInstantiation),
    String(ConstantStringInstantiation),
    BitArray(ConstantBitArrayInstantiation),
    Custom(ConstantCustomInstantiation),
    Float(ConstantFloatInstantiation),
    Bool(ConstantBoolInstantiation),
    Nil(ConstantNilInstantiation),
    Tuple(ConstantTupleInstantiation),
    List(ConstantListInstantiation),
    Function(ConstantFunctionInstantiation),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ConstantIntReference(ConstantIntInstantiation);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ConstantStringReference(ConstantStringInstantiation);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ConstantBitArrayReference(ConstantBitArrayInstantiation);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ConstantCustomReference(ConstantCustomInstantiation);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ConstantFloatReference(ConstantFloatInstantiation);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ConstantBoolReference(ConstantBoolInstantiation);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ConstantNilReference(ConstantNilInstantiation);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct ConstantTupleReference(ConstantTupleInstantiation);

impl ConstantCustomReference {
    pub(crate) fn instantiation(&self) -> &ConstantCustomInstantiation {
        &self.0
    }

    pub(crate) fn shape(&self) -> &CustomValueShape {
        self.0.shape()
    }
}

impl ConstantTupleReference {
    pub(crate) fn instantiation(&self) -> &ConstantTupleInstantiation {
        &self.0
    }

    pub(crate) fn shape(&self) -> &[ValueShape] {
        self.0.shape()
    }
}

impl ConstantIntReference {
    pub(crate) fn instantiation(&self) -> &ConstantIntInstantiation {
        &self.0
    }
}

impl ConstantStringReference {
    pub(crate) fn instantiation(&self) -> &ConstantStringInstantiation {
        &self.0
    }
}

impl ConstantBitArrayReference {
    pub(crate) fn instantiation(&self) -> &ConstantBitArrayInstantiation {
        &self.0
    }
}

impl ConstantFloatReference {
    pub(crate) fn instantiation(&self) -> &ConstantFloatInstantiation {
        &self.0
    }
}

impl ConstantBoolReference {
    pub(crate) fn instantiation(&self) -> &ConstantBoolInstantiation {
        &self.0
    }
}

impl ConstantNilReference {
    pub(crate) fn instantiation(&self) -> &ConstantNilInstantiation {
        &self.0
    }
}

impl ConstantInstantiation {
    pub(crate) fn module(&self) -> crate::plan::ModuleId {
        match &self.kind {
            ConstantInstantiationKind::Int(value) => value.module(),
            ConstantInstantiationKind::String(value) => value.module(),
            ConstantInstantiationKind::BitArray(value) => value.module(),
            ConstantInstantiationKind::Custom(value) => value.module(),
            ConstantInstantiationKind::Float(value) => value.module(),
            ConstantInstantiationKind::Bool(value) => value.module(),
            ConstantInstantiationKind::Nil(value) => value.module(),
            ConstantInstantiationKind::Tuple(value) => value.module(),
            ConstantInstantiationKind::List(value) => value.module(),
            ConstantInstantiationKind::Function(value) => value.module(),
        }
    }

    pub(crate) fn from_int(value: ConstantIntInstantiation) -> Self {
        Self {
            kind: ConstantInstantiationKind::Int(value),
        }
    }

    pub(crate) fn from_string(value: ConstantStringInstantiation) -> Self {
        Self {
            kind: ConstantInstantiationKind::String(value),
        }
    }

    pub(crate) fn from_bit_array(value: ConstantBitArrayInstantiation) -> Self {
        Self {
            kind: ConstantInstantiationKind::BitArray(value),
        }
    }

    pub(crate) fn from_custom(value: ConstantCustomInstantiation) -> Self {
        Self {
            kind: ConstantInstantiationKind::Custom(value),
        }
    }

    pub(crate) fn from_float(value: ConstantFloatInstantiation) -> Self {
        Self {
            kind: ConstantInstantiationKind::Float(value),
        }
    }

    pub(crate) fn from_bool(value: ConstantBoolInstantiation) -> Self {
        Self {
            kind: ConstantInstantiationKind::Bool(value),
        }
    }

    pub(crate) fn from_nil(value: ConstantNilInstantiation) -> Self {
        Self {
            kind: ConstantInstantiationKind::Nil(value),
        }
    }

    pub(crate) fn from_tuple(value: ConstantTupleInstantiation) -> Self {
        Self {
            kind: ConstantInstantiationKind::Tuple(value),
        }
    }

    pub(crate) fn from_list(value: ConstantListInstantiation) -> Self {
        Self {
            kind: ConstantInstantiationKind::List(value),
        }
    }

    pub(crate) fn from_function(value: ConstantFunctionInstantiation) -> Self {
        Self {
            kind: ConstantInstantiationKind::Function(value),
        }
    }

    pub(crate) fn substitute(&self, outer: &TypeSubstitution) -> Self {
        let kind = match &self.kind {
            ConstantInstantiationKind::Int(value) => {
                ConstantInstantiationKind::Int(value.substitute_leaf(outer))
            }
            ConstantInstantiationKind::String(value) => {
                ConstantInstantiationKind::String(value.substitute_leaf(outer))
            }
            ConstantInstantiationKind::BitArray(value) => {
                ConstantInstantiationKind::BitArray(value.substitute_leaf(outer))
            }
            ConstantInstantiationKind::Custom(value) => {
                ConstantInstantiationKind::Custom(value.substitute_custom(outer))
            }
            ConstantInstantiationKind::Float(value) => {
                ConstantInstantiationKind::Float(value.substitute_leaf(outer))
            }
            ConstantInstantiationKind::Bool(value) => {
                ConstantInstantiationKind::Bool(value.substitute_leaf(outer))
            }
            ConstantInstantiationKind::Nil(value) => {
                ConstantInstantiationKind::Nil(value.substitute_leaf(outer))
            }
            ConstantInstantiationKind::Tuple(value) => {
                ConstantInstantiationKind::Tuple(value.substitute_tuple(outer))
            }
            ConstantInstantiationKind::List(value) => {
                ConstantInstantiationKind::List(value.substitute(outer))
            }
            ConstantInstantiationKind::Function(value) => {
                ConstantInstantiationKind::Function(value.substitute(outer))
            }
        };
        Self { kind }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ConstantIntValue {
    Value(BigInt),
    Reference(ConstantIntInstantiation),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ConstantFloatValue {
    Value(f64),
    Reference(ConstantFloatInstantiation),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ConstantStringValue {
    Value(EcoString),
    Concatenation {
        left: Box<ConstantStringValue>,
        right: Box<ConstantStringValue>,
    },
    Reference(ConstantStringInstantiation),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ConstantBoolValue {
    Value(bool),
    Reference(ConstantBoolInstantiation),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ConstantNilValue {
    Value,
    Reference(ConstantNilInstantiation),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConstantTupleValue {
    shape: Box<[ValueShape]>,
    kind: ConstantTupleValueKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ConstantTupleValueKind {
    Value(Box<[ConstantValue]>),
    Reference(ConstantTupleInstantiation),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConstantBitArrayValue {
    kind: ConstantBitArrayValueKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ConstantBitArrayValueKind {
    Value(Box<[ConstantBitArraySegment]>),
    Reference(ConstantBitArrayInstantiation),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConstantCustomValue {
    shape: CustomValueShape,
    kind: ConstantCustomValueKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ConstantCustomValueKind {
    Construction(ConstantCustomConstruction),
    Reference(ConstantCustomInstantiation),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConstantCustomConstruction {
    constructor: CustomConstructor,
    fields: Box<[ConstantValue]>,
}

#[derive(Debug)]
pub(crate) struct MaterializedConstantCustomConstruction {
    constructor: CustomConstructor,
    fields: Box<[super::Expr]>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ConstantValue {
    kind: ConstantValueKind,
}

#[derive(Debug, Clone, PartialEq)]
enum ConstantValueKind {
    Int(ConstantIntValue),
    Float(ConstantFloatValue),
    String(ConstantStringValue),
    Bool(ConstantBoolValue),
    Nil(ConstantNilValue),
    Tuple(ConstantTupleValue),
    List(ConstantListValue),
    BitArray(ConstantBitArrayValue),
    Custom(ConstantCustomValue),
    Function(ConstantFunctionValue),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ConstantListConstructionError {
    TypeMismatch {
        expected: ValueType,
        actual: ValueType,
    },
    SpreadWithoutElements,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ConstantBitArraySegment {
    Int {
        value: ConstantIntValue,
        bit_size: usize,
        endianness: Endianness,
    },
    Float {
        value: ConstantFloatValue,
        bit_size: FloatBitSize,
        endianness: Endianness,
    },
    String {
        value: ConstantStringValue,
        encoding: StringEncoding,
    },
    Bits(ConstantBitArrayValue),
    SizedBits {
        value: ConstantBitArrayValue,
        bit_size: usize,
        site: PanicSite,
    },
}

impl ConstantTemplateId {
    #[cfg(test)]
    pub(crate) fn new(index: usize) -> Self {
        Self::in_module(crate::plan::ModuleId::root(), index)
    }

    pub(crate) fn in_module(module: crate::plan::ModuleId, index: usize) -> Self {
        Self { module, index }
    }

    pub fn module(self) -> crate::plan::ModuleId {
        self.module
    }

    pub(crate) fn index(self) -> usize {
        self.index
    }
}

impl ConstantTemplate {
    pub(crate) fn new(signature: ConstantTemplateSignature, name: EcoString) -> Self {
        Self { signature, name }
    }

    pub fn id(&self) -> ConstantTemplateId {
        self.signature.id()
    }

    pub fn name(&self) -> &EcoString {
        &self.name
    }

    pub fn scheme(&self) -> &TypeScheme {
        self.signature.scheme()
    }

    #[cfg(test)]
    pub(crate) fn signature(&self) -> &ConstantTemplateSignature {
        &self.signature
    }
}

impl ConstantTemplates {
    pub(crate) fn from_module_entries(
        module: crate::plan::ModuleId,
        entries: Vec<(ConstantTemplate, ConstantValue)>,
    ) -> Self {
        let mut templates = Self {
            module,
            headers: Vec::with_capacity(entries.len()),
            generic_lists: Vec::new(),
            ints: Vec::new(),
            strings: Vec::new(),
            bit_arrays: Vec::new(),
            custom_values: Vec::new(),
            floats: Vec::new(),
            bools: Vec::new(),
            nils: Vec::new(),
            tuples: Vec::new(),
            int_lists: Vec::new(),
            string_lists: Vec::new(),
            bit_array_lists: Vec::new(),
            utf_codepoint_lists: Vec::new(),
            custom_lists: Vec::new(),
            float_lists: Vec::new(),
            bool_lists: Vec::new(),
            nil_lists: Vec::new(),
            tuple_lists: Vec::new(),
            parameter_list_lists: Vec::new(),
            list_lists: Vec::new(),
            function_lists: Vec::new(),
            generic_functions: Vec::new(),
            int_functions: Vec::new(),
            string_functions: Vec::new(),
            bit_array_functions: Vec::new(),
            utf_codepoint_functions: Vec::new(),
            custom_functions: Vec::new(),
            float_functions: Vec::new(),
            bool_functions: Vec::new(),
            nil_functions: Vec::new(),
            tuple_functions: Vec::new(),
            list_functions: Vec::new(),
            function_functions: Vec::new(),
        };
        for (template, value) in entries {
            templates.headers.push(template);
            templates.push_value(value);
        }
        templates
    }

    #[cfg(test)]
    pub(crate) fn from_entries(entries: Vec<(ConstantTemplate, ConstantValue)>) -> Self {
        Self::from_module_entries(crate::plan::ModuleId::root(), entries)
    }

    #[cfg(test)]
    pub(crate) fn empty() -> Self {
        Self::from_entries(Vec::new())
    }

    fn owns(&self, module: crate::plan::ModuleId) -> bool {
        self.module == module
    }

    pub(crate) fn headers(&self) -> &[ConstantTemplate] {
        &self.headers
    }

    #[cfg(test)]
    pub(crate) fn header(&self, id: ConstantTemplateId) -> &ConstantTemplate {
        &self.headers[id.index()]
    }

    pub(crate) fn int(&self, id: ConstantIntTemplateId) -> &ConstantIntValue {
        &self.ints[id.0]
    }

    pub(crate) fn string(&self, id: ConstantStringTemplateId) -> &ConstantStringValue {
        &self.strings[id.0]
    }

    pub(crate) fn bit_array(&self, id: ConstantBitArrayTemplateId) -> &ConstantBitArrayValue {
        &self.bit_arrays[id.0]
    }

    pub(crate) fn custom(&self, id: ConstantCustomTemplateId) -> &ConstantCustomValue {
        &self.custom_values[id.0]
    }

    pub(crate) fn float(&self, id: ConstantFloatTemplateId) -> &ConstantFloatValue {
        &self.floats[id.0]
    }

    pub(crate) fn bool(&self, id: ConstantBoolTemplateId) -> &ConstantBoolValue {
        &self.bools[id.0]
    }

    pub(crate) fn nil(&self, id: ConstantNilTemplateId) -> &ConstantNilValue {
        &self.nils[id.0]
    }

    pub(crate) fn tuple(&self, id: ConstantTupleTemplateId) -> &ConstantTupleValue {
        &self.tuples[id.0]
    }

    fn generic_list(&self, id: ConstantGenericListTemplateId) -> &ConstantGenericListValue {
        &self.generic_lists[id.0]
    }

    fn int_list(&self, id: ConstantIntListTemplateId) -> &ConstantIntListValue {
        &self.int_lists[id.0]
    }

    fn string_list(&self, id: ConstantStringListTemplateId) -> &ConstantStringListValue {
        &self.string_lists[id.0]
    }

    fn bit_array_list(&self, id: ConstantBitArrayListTemplateId) -> &ConstantBitArrayListValue {
        &self.bit_array_lists[id.0]
    }

    fn utf_codepoint_list(
        &self,
        id: ConstantUtfCodepointListTemplateId,
    ) -> &ConstantUtfCodepointListValue {
        &self.utf_codepoint_lists[id.0]
    }

    fn custom_list(&self, id: ConstantCustomListTemplateId) -> &ConstantCustomListValue {
        &self.custom_lists[id.0]
    }

    fn float_list(&self, id: ConstantFloatListTemplateId) -> &ConstantFloatListValue {
        &self.float_lists[id.0]
    }

    fn bool_list(&self, id: ConstantBoolListTemplateId) -> &ConstantBoolListValue {
        &self.bool_lists[id.0]
    }

    fn nil_list(&self, id: ConstantNilListTemplateId) -> &ConstantNilListValue {
        &self.nil_lists[id.0]
    }

    fn tuple_list(&self, id: ConstantTupleListTemplateId) -> &ConstantTupleListValue {
        &self.tuple_lists[id.0]
    }

    fn list_list(&self, id: ConstantListListTemplateId) -> &ConstantListListValue {
        &self.list_lists[id.0]
    }

    fn parameter_list_list(
        &self,
        id: ConstantParameterListListTemplateId,
    ) -> &ConstantParameterListListValue {
        &self.parameter_list_lists[id.0]
    }

    fn function_list(&self, id: ConstantFunctionListTemplateId) -> &ConstantFunctionListValue {
        &self.function_lists[id.0]
    }

    fn generic_function(
        &self,
        id: ConstantGenericFunctionTemplateId,
    ) -> &ConstantGenericFunctionValue {
        &self.generic_functions[id.0]
    }

    fn int_function(&self, id: ConstantIntFunctionTemplateId) -> &ConstantIntFunctionValue {
        &self.int_functions[id.0]
    }

    fn string_function(
        &self,
        id: ConstantStringFunctionTemplateId,
    ) -> &ConstantStringFunctionValue {
        &self.string_functions[id.0]
    }

    fn bit_array_function(
        &self,
        id: ConstantBitArrayFunctionTemplateId,
    ) -> &ConstantBitArrayFunctionValue {
        &self.bit_array_functions[id.0]
    }

    fn utf_codepoint_function(
        &self,
        id: ConstantUtfCodepointFunctionTemplateId,
    ) -> &ConstantUtfCodepointFunctionValue {
        &self.utf_codepoint_functions[id.0]
    }

    fn custom_function(
        &self,
        id: ConstantCustomFunctionTemplateId,
    ) -> &ConstantCustomFunctionValue {
        &self.custom_functions[id.0]
    }

    fn float_function(&self, id: ConstantFloatFunctionTemplateId) -> &ConstantFloatFunctionValue {
        &self.float_functions[id.0]
    }

    fn bool_function(&self, id: ConstantBoolFunctionTemplateId) -> &ConstantBoolFunctionValue {
        &self.bool_functions[id.0]
    }

    fn nil_function(&self, id: ConstantNilFunctionTemplateId) -> &ConstantNilFunctionValue {
        &self.nil_functions[id.0]
    }

    fn tuple_function(&self, id: ConstantTupleFunctionTemplateId) -> &ConstantTupleFunctionValue {
        &self.tuple_functions[id.0]
    }

    fn list_function(&self, id: ConstantListFunctionTemplateId) -> &ConstantListFunctionValue {
        &self.list_functions[id.0]
    }

    fn function_function(
        &self,
        id: ConstantFunctionFunctionTemplateId,
    ) -> &ConstantFunctionFunctionValue {
        &self.function_functions[id.0]
    }

    fn push_value(&mut self, value: ConstantValue) {
        match value.kind {
            ConstantValueKind::Int(value) => self.ints.push(value),
            ConstantValueKind::Float(value) => self.floats.push(value),
            ConstantValueKind::String(value) => self.strings.push(value),
            ConstantValueKind::Bool(value) => self.bools.push(value),
            ConstantValueKind::Nil(value) => self.nils.push(value),
            ConstantValueKind::Tuple(value) => self.tuples.push(value),
            ConstantValueKind::List(value) => match value {
                ConstantListValue::Generic(value) => self.generic_lists.push(value),
                ConstantListValue::ParameterList(value) => self.parameter_list_lists.push(value),
                ConstantListValue::Int(value) => self.int_lists.push(value),
                ConstantListValue::String(value) => self.string_lists.push(value),
                ConstantListValue::BitArray(value) => self.bit_array_lists.push(value),
                ConstantListValue::UtfCodepoint(value) => self.utf_codepoint_lists.push(value),
                ConstantListValue::Custom(value) => self.custom_lists.push(value),
                ConstantListValue::Float(value) => self.float_lists.push(value),
                ConstantListValue::Bool(value) => self.bool_lists.push(value),
                ConstantListValue::Nil(value) => self.nil_lists.push(value),
                ConstantListValue::Tuple(value) => self.tuple_lists.push(value),
                ConstantListValue::List(value) => self.list_lists.push(value),
                ConstantListValue::Function(value) => self.function_lists.push(value),
            },
            ConstantValueKind::BitArray(value) => self.bit_arrays.push(value),
            ConstantValueKind::Custom(value) => self.custom_values.push(value),
            ConstantValueKind::Function(value) => match value {
                ConstantFunctionValue::Generic(value) => self.generic_functions.push(value),
                ConstantFunctionValue::Int(value) => self.int_functions.push(value),
                ConstantFunctionValue::String(value) => self.string_functions.push(value),
                ConstantFunctionValue::BitArray(value) => self.bit_array_functions.push(value),
                ConstantFunctionValue::UtfCodepoint(value) => {
                    self.utf_codepoint_functions.push(value)
                }
                ConstantFunctionValue::Custom(value) => self.custom_functions.push(value),
                ConstantFunctionValue::Float(value) => self.float_functions.push(value),
                ConstantFunctionValue::Bool(value) => self.bool_functions.push(value),
                ConstantFunctionValue::Nil(value) => self.nil_functions.push(value),
                ConstantFunctionValue::Tuple(value) => self.tuple_functions.push(value),
                ConstantFunctionValue::List(value) => self.list_functions.push(value),
                ConstantFunctionValue::Function(value) => self.function_functions.push(value),
            },
        }
    }

    pub(crate) fn reference(instantiation: ConstantInstantiation) -> Expr {
        match instantiation.kind {
            ConstantInstantiationKind::Int(value) => {
                Expr::int(IntExpr::constant(ConstantIntReference(value)))
            }
            ConstantInstantiationKind::String(value) => {
                Expr::string(StringExpr::constant(ConstantStringReference(value)))
            }
            ConstantInstantiationKind::BitArray(value) => {
                Expr::bit_array(BitArrayExpr::constant(ConstantBitArrayReference(value)))
            }
            ConstantInstantiationKind::Custom(value) => {
                Expr::custom(CustomExpr::constant(ConstantCustomReference(value)))
            }
            ConstantInstantiationKind::Float(value) => {
                Expr::float(FloatExpr::constant(ConstantFloatReference(value)))
            }
            ConstantInstantiationKind::Bool(value) => {
                Expr::bool(BoolExpr::constant(ConstantBoolReference(value)))
            }
            ConstantInstantiationKind::Nil(value) => {
                Expr::nil(NilExpr::constant(ConstantNilReference(value)))
            }
            ConstantInstantiationKind::Tuple(value) => {
                Expr::tuple(TupleExpr::constant(ConstantTupleReference(value)))
            }
            ConstantInstantiationKind::List(value) => Expr::list(ListExpr::constant(value)),
            ConstantInstantiationKind::Function(value) => {
                Expr::function(FunctionExpr::constant(value.clone()))
            }
        }
    }

    fn materialize_value(&self, value: &ConstantValue, substitution: &TypeSubstitution) -> Expr {
        match &value.kind {
            ConstantValueKind::Int(value) => {
                Expr::int(self.materialize_int_value(value, substitution))
            }
            ConstantValueKind::Float(value) => {
                Expr::float(self.materialize_float_value(value, substitution))
            }
            ConstantValueKind::String(value) => {
                Expr::string(self.materialize_string_value(value, substitution))
            }
            ConstantValueKind::Bool(value) => {
                Expr::bool(self.materialize_bool_value(value, substitution))
            }
            ConstantValueKind::Nil(value) => {
                Expr::nil(self.materialize_nil_value(value, substitution))
            }
            ConstantValueKind::Tuple(value) => {
                Expr::tuple(self.materialize_tuple_value(value, substitution))
            }
            ConstantValueKind::List(value) => {
                Expr::list(self.materialize_list_value(value, substitution))
            }
            ConstantValueKind::BitArray(value) => {
                Expr::bit_array(self.materialize_bit_array_value(value, substitution))
            }
            ConstantValueKind::Custom(value) => {
                Expr::custom(self.materialize_custom_value(value, substitution))
            }
            ConstantValueKind::Function(value) => {
                Expr::function(self.materialize_function_value(value, substitution))
            }
        }
    }

    pub(crate) fn materialize_int(&self, value: &ConstantIntInstantiation) -> IntExpr {
        if !self.owns(value.module()) {
            return IntExpr::constant(ConstantIntReference(value.clone()));
        }
        self.materialize_int_value(self.int(value.template()), value.substitution())
    }

    fn materialize_int_value(
        &self,
        value: &ConstantIntValue,
        substitution: &TypeSubstitution,
    ) -> IntExpr {
        match value {
            ConstantIntValue::Value(value) => IntExpr::value(value.clone()),
            ConstantIntValue::Reference(value) => {
                self.materialize_int(&value.substitute_leaf(substitution))
            }
        }
    }

    pub(crate) fn materialize_float(&self, value: &ConstantFloatInstantiation) -> FloatExpr {
        if !self.owns(value.module()) {
            return FloatExpr::constant(ConstantFloatReference(value.clone()));
        }
        self.materialize_float_value(self.float(value.template()), value.substitution())
    }

    fn materialize_float_value(
        &self,
        value: &ConstantFloatValue,
        substitution: &TypeSubstitution,
    ) -> FloatExpr {
        match value {
            ConstantFloatValue::Value(value) => FloatExpr::value(*value),
            ConstantFloatValue::Reference(value) => {
                self.materialize_float(&value.substitute_leaf(substitution))
            }
        }
    }

    pub(crate) fn materialize_string(&self, value: &ConstantStringInstantiation) -> StringExpr {
        if !self.owns(value.module()) {
            return StringExpr::constant(ConstantStringReference(value.clone()));
        }
        self.materialize_string_value(self.string(value.template()), value.substitution())
    }

    fn materialize_string_value(
        &self,
        value: &ConstantStringValue,
        substitution: &TypeSubstitution,
    ) -> StringExpr {
        match value {
            ConstantStringValue::Value(value) => StringExpr::value(value.clone()),
            ConstantStringValue::Concatenation { left, right } => StringExpr::concatenate(
                self.materialize_string_value(left, substitution),
                self.materialize_string_value(right, substitution),
            ),
            ConstantStringValue::Reference(value) => {
                self.materialize_string(&value.substitute_leaf(substitution))
            }
        }
    }

    pub(crate) fn materialize_bool(&self, value: &ConstantBoolInstantiation) -> BoolExpr {
        if !self.owns(value.module()) {
            return BoolExpr::constant(ConstantBoolReference(value.clone()));
        }
        self.materialize_bool_value(self.bool(value.template()), value.substitution())
    }

    fn materialize_bool_value(
        &self,
        value: &ConstantBoolValue,
        substitution: &TypeSubstitution,
    ) -> BoolExpr {
        match value {
            ConstantBoolValue::Value(value) => BoolExpr::value(*value),
            ConstantBoolValue::Reference(value) => {
                self.materialize_bool(&value.substitute_leaf(substitution))
            }
        }
    }

    pub(crate) fn materialize_nil(&self, value: &ConstantNilInstantiation) -> NilExpr {
        if !self.owns(value.module()) {
            return NilExpr::constant(ConstantNilReference(value.clone()));
        }
        self.materialize_nil_value(self.nil(value.template()), value.substitution())
    }

    fn materialize_nil_value(
        &self,
        value: &ConstantNilValue,
        substitution: &TypeSubstitution,
    ) -> NilExpr {
        match value {
            ConstantNilValue::Value => NilExpr::value(),
            ConstantNilValue::Reference(value) => {
                self.materialize_nil(&value.substitute_leaf(substitution))
            }
        }
    }

    pub(crate) fn materialize_tuple(&self, value: &ConstantTupleInstantiation) -> TupleExpr {
        if !self.owns(value.module()) {
            return TupleExpr::constant(ConstantTupleReference(value.clone()));
        }
        self.materialize_tuple_value(self.tuple(value.template()), value.substitution())
    }

    fn materialize_tuple_value(
        &self,
        value: &ConstantTupleValue,
        substitution: &TypeSubstitution,
    ) -> TupleExpr {
        match value.kind() {
            ConstantTupleValueKind::Value(elements) => {
                let shape = value
                    .shape()
                    .iter()
                    .map(|shape| shape.substitute(substitution))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                TupleExpr::value(
                    elements
                        .iter()
                        .map(|element| self.materialize_value(element, substitution))
                        .collect(),
                    shape.iter().map(ValueShape::value_type).collect(),
                )
                .with_shape(shape)
            }
            ConstantTupleValueKind::Reference(value) => {
                self.materialize_tuple(&value.substitute_tuple(substitution))
            }
        }
    }

    pub(crate) fn materialize_bit_array(
        &self,
        value: &ConstantBitArrayInstantiation,
    ) -> BitArrayExpr {
        if !self.owns(value.module()) {
            return BitArrayExpr::constant(ConstantBitArrayReference(value.clone()));
        }
        self.materialize_bit_array_value(self.bit_array(value.template()), value.substitution())
    }

    fn materialize_bit_array_value(
        &self,
        value: &ConstantBitArrayValue,
        substitution: &TypeSubstitution,
    ) -> BitArrayExpr {
        match value.kind() {
            ConstantBitArrayValueKind::Value(segments) => BitArrayExpr::value(
                segments
                    .iter()
                    .map(|segment| self.materialize_bit_array_segment(segment, substitution))
                    .collect(),
            ),
            ConstantBitArrayValueKind::Reference(value) => {
                self.materialize_bit_array(&value.substitute_leaf(substitution))
            }
        }
    }

    fn materialize_bit_array_segment(
        &self,
        segment: &ConstantBitArraySegment,
        substitution: &TypeSubstitution,
    ) -> BitArraySegment {
        match segment {
            ConstantBitArraySegment::Int {
                value,
                bit_size,
                endianness,
            } => BitArraySegment::Int {
                value: self.materialize_int_value(value, substitution),
                bit_size: *bit_size,
                endianness: *endianness,
            },
            ConstantBitArraySegment::Float {
                value,
                bit_size,
                endianness,
            } => BitArraySegment::Float {
                value: self.materialize_float_value(value, substitution),
                bit_size: *bit_size,
                endianness: *endianness,
            },
            ConstantBitArraySegment::String { value, encoding } => BitArraySegment::String {
                value: self.materialize_string_value(value, substitution),
                encoding: *encoding,
            },
            ConstantBitArraySegment::Bits(value) => {
                BitArraySegment::Bits(self.materialize_bit_array_value(value, substitution))
            }
            ConstantBitArraySegment::SizedBits {
                value,
                bit_size,
                site,
            } => BitArraySegment::SizedBits {
                value: self.materialize_bit_array_value(value, substitution),
                size: BitArrayBitsSize::Fixed(*bit_size),
                site: site.clone(),
            },
        }
    }

    pub(crate) fn materialize_custom(&self, value: &ConstantCustomInstantiation) -> CustomExpr {
        if !self.owns(value.module()) {
            return CustomExpr::constant(ConstantCustomReference(value.clone()));
        }
        self.materialize_custom_value(self.custom(value.template()), value.substitution())
    }

    fn materialize_custom_value(
        &self,
        value: &ConstantCustomValue,
        substitution: &TypeSubstitution,
    ) -> CustomExpr {
        match value.kind() {
            ConstantCustomValueKind::Construction(construction) => {
                let construction = MaterializedConstantCustomConstruction::new(
                    construction.constructor().substitute(substitution),
                    construction
                        .fields()
                        .iter()
                        .map(|field| self.materialize_value(field, substitution))
                        .collect(),
                );
                CustomExpr::from_construction(
                    value.shape().substitute(substitution),
                    CustomConstruction::from_constant(construction),
                )
            }
            ConstantCustomValueKind::Reference(value) => {
                self.materialize_custom(&value.substitute_custom(substitution))
            }
        }
    }

    fn materialize_function_value(
        &self,
        value: &ConstantFunctionValue,
        substitution: &TypeSubstitution,
    ) -> FunctionExpr {
        match value {
            ConstantFunctionValue::Generic(value) => match value.kind() {
                TypedConstantFunctionValueKind::Target(reference) => {
                    FunctionExpr::reference(reference.substitute(substitution))
                }
                TypedConstantFunctionValueKind::Reference(reference) => FunctionExpr::constant(
                    ConstantFunctionInstantiation::Generic(reference.clone())
                        .substitute(substitution),
                ),
            },
            ConstantFunctionValue::Int(value) => match value.kind() {
                TypedConstantFunctionValueKind::Target(reference) => {
                    FunctionExpr::reference(reference.substitute(substitution))
                }
                TypedConstantFunctionValueKind::Reference(reference) => FunctionExpr::constant(
                    ConstantFunctionInstantiation::Int(reference.substitute(substitution)),
                ),
            },
            ConstantFunctionValue::String(value) => match value.kind() {
                TypedConstantFunctionValueKind::Target(reference) => {
                    FunctionExpr::reference(reference.substitute(substitution))
                }
                TypedConstantFunctionValueKind::Reference(reference) => FunctionExpr::constant(
                    ConstantFunctionInstantiation::String(reference.substitute(substitution)),
                ),
            },
            ConstantFunctionValue::BitArray(value) => match value.kind() {
                TypedConstantFunctionValueKind::Target(reference) => {
                    FunctionExpr::reference(reference.substitute(substitution))
                }
                TypedConstantFunctionValueKind::Reference(reference) => FunctionExpr::constant(
                    ConstantFunctionInstantiation::BitArray(reference.substitute(substitution)),
                ),
            },
            ConstantFunctionValue::UtfCodepoint(value) => match value.kind() {
                TypedConstantFunctionValueKind::Target(reference) => {
                    FunctionExpr::reference(reference.substitute(substitution))
                }
                TypedConstantFunctionValueKind::Reference(reference) => FunctionExpr::constant(
                    ConstantFunctionInstantiation::UtfCodepoint(reference.substitute(substitution)),
                ),
            },
            ConstantFunctionValue::Custom(value) => match value.kind() {
                TypedConstantFunctionValueKind::Target(
                    ConstantCustomFunctionTarget::Reference(reference),
                ) => FunctionExpr::reference(reference.substitute(substitution)),
                TypedConstantFunctionValueKind::Target(
                    ConstantCustomFunctionTarget::Constructor(constructor),
                ) => FunctionExpr::custom(CustomFunctionExpr::constructor(
                    constructor.substitute(substitution),
                )),
                TypedConstantFunctionValueKind::Reference(reference) => FunctionExpr::constant(
                    ConstantFunctionInstantiation::Custom(reference.substitute(substitution)),
                ),
            },
            ConstantFunctionValue::Float(value) => match value.kind() {
                TypedConstantFunctionValueKind::Target(reference) => {
                    FunctionExpr::reference(reference.substitute(substitution))
                }
                TypedConstantFunctionValueKind::Reference(reference) => FunctionExpr::constant(
                    ConstantFunctionInstantiation::Float(reference.substitute(substitution)),
                ),
            },
            ConstantFunctionValue::Bool(value) => match value.kind() {
                TypedConstantFunctionValueKind::Target(reference) => {
                    FunctionExpr::reference(reference.substitute(substitution))
                }
                TypedConstantFunctionValueKind::Reference(reference) => FunctionExpr::constant(
                    ConstantFunctionInstantiation::Bool(reference.substitute(substitution)),
                ),
            },
            ConstantFunctionValue::Nil(value) => match value.kind() {
                TypedConstantFunctionValueKind::Target(reference) => {
                    FunctionExpr::reference(reference.substitute(substitution))
                }
                TypedConstantFunctionValueKind::Reference(reference) => FunctionExpr::constant(
                    ConstantFunctionInstantiation::Nil(reference.substitute(substitution)),
                ),
            },
            ConstantFunctionValue::Tuple(value) => match value.kind() {
                TypedConstantFunctionValueKind::Target(reference) => {
                    FunctionExpr::reference(reference.substitute(substitution))
                }
                TypedConstantFunctionValueKind::Reference(reference) => FunctionExpr::constant(
                    ConstantFunctionInstantiation::Tuple(reference.substitute(substitution)),
                ),
            },
            ConstantFunctionValue::List(value) => match value.kind() {
                TypedConstantFunctionValueKind::Target(reference) => {
                    FunctionExpr::reference(reference.substitute(substitution))
                }
                TypedConstantFunctionValueKind::Reference(reference) => FunctionExpr::constant(
                    ConstantFunctionInstantiation::List(reference.substitute(substitution)),
                ),
            },
            ConstantFunctionValue::Function(value) => match value.kind() {
                TypedConstantFunctionValueKind::Target(reference) => {
                    FunctionExpr::reference(reference.substitute(substitution))
                }
                TypedConstantFunctionValueKind::Reference(reference) => FunctionExpr::constant(
                    ConstantFunctionInstantiation::Function(reference.substitute(substitution)),
                ),
            },
        }
    }

    fn typed_function_reference<Function>(
        reference: &FunctionReference,
        substitution: &TypeSubstitution,
    ) -> TypedFunctionReference<Function> {
        TypedFunctionReference::new(reference.substitute(substitution).into_instantiation())
    }

    pub(crate) fn materialize_generic_function(
        &self,
        value: &ConstantGenericFunctionInstantiation,
    ) -> GenericFunctionExpr {
        let type_ = crate::plan::GenericFunctionType::new(
            value.shape().argument_shapes().to_vec(),
            *value.return_(),
        );
        match self.generic_function(*value.source()).kind() {
            TypedConstantFunctionValueKind::Target(reference) => GenericFunctionExpr::reference(
                Self::typed_function_reference(reference, value.substitution()),
                type_,
            ),
            TypedConstantFunctionValueKind::Reference(reference) => GenericFunctionExpr::constant(
                reference.substitute_generic(value.substitution(), *value.return_()),
                type_,
            ),
        }
    }

    pub(crate) fn materialize_int_function(
        &self,
        value: &ConstantIntFunctionInstantiation,
    ) -> IntFunctionExpr {
        let type_ = value.shape().type_();
        match value.source() {
            ConstantFunctionTemplateSource::Generic(source) => {
                match self.generic_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        IntFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        IntFunctionExpr::constant(
                            reference.specialize(value.substitution(), ()),
                            type_,
                        )
                    }
                }
            }
            ConstantFunctionTemplateSource::Exact(source) => {
                match self.int_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        IntFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        let reference = reference.substitute(value.substitution());
                        let type_ = reference.shape().type_();
                        IntFunctionExpr::constant(reference, type_)
                    }
                }
            }
        }
    }

    pub(crate) fn materialize_string_function(
        &self,
        value: &ConstantStringFunctionInstantiation,
    ) -> StringFunctionExpr {
        let type_ = value.shape().type_();
        match value.source() {
            ConstantFunctionTemplateSource::Generic(source) => {
                match self.generic_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        StringFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        StringFunctionExpr::constant(
                            reference.specialize(value.substitution(), ()),
                            type_,
                        )
                    }
                }
            }
            ConstantFunctionTemplateSource::Exact(source) => {
                match self.string_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        StringFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        let reference = reference.substitute(value.substitution());
                        let type_ = reference.shape().type_();
                        StringFunctionExpr::constant(reference, type_)
                    }
                }
            }
        }
    }

    pub(crate) fn materialize_bit_array_function(
        &self,
        value: &ConstantBitArrayFunctionInstantiation,
    ) -> BitArrayFunctionExpr {
        let type_ = value.shape().type_();
        match value.source() {
            ConstantFunctionTemplateSource::Generic(source) => {
                match self.generic_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        BitArrayFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        BitArrayFunctionExpr::constant(
                            reference.specialize(value.substitution(), ()),
                            type_,
                        )
                    }
                }
            }
            ConstantFunctionTemplateSource::Exact(source) => {
                match self.bit_array_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        BitArrayFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        let reference = reference.substitute(value.substitution());
                        let type_ = reference.shape().type_();
                        BitArrayFunctionExpr::constant(reference, type_)
                    }
                }
            }
        }
    }

    pub(crate) fn materialize_utf_codepoint_function(
        &self,
        value: &ConstantUtfCodepointFunctionInstantiation,
    ) -> UtfCodepointFunctionExpr {
        let type_ = value.shape().type_();
        match value.source() {
            ConstantFunctionTemplateSource::Generic(source) => {
                match self.generic_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        UtfCodepointFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        UtfCodepointFunctionExpr::constant(
                            reference.specialize(value.substitution(), ()),
                            type_,
                        )
                    }
                }
            }
            ConstantFunctionTemplateSource::Exact(source) => {
                match self.utf_codepoint_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        UtfCodepointFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        let reference = reference.substitute(value.substitution());
                        let type_ = reference.shape().type_();
                        UtfCodepointFunctionExpr::constant(reference, type_)
                    }
                }
            }
        }
    }

    pub(crate) fn materialize_float_function(
        &self,
        value: &ConstantFloatFunctionInstantiation,
    ) -> FloatFunctionExpr {
        let type_ = value.shape().type_();
        match value.source() {
            ConstantFunctionTemplateSource::Generic(source) => {
                match self.generic_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        FloatFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        FloatFunctionExpr::constant(
                            reference.specialize(value.substitution(), ()),
                            type_,
                        )
                    }
                }
            }
            ConstantFunctionTemplateSource::Exact(source) => {
                match self.float_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        FloatFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        let reference = reference.substitute(value.substitution());
                        let type_ = reference.shape().type_();
                        FloatFunctionExpr::constant(reference, type_)
                    }
                }
            }
        }
    }

    pub(crate) fn materialize_bool_function(
        &self,
        value: &ConstantBoolFunctionInstantiation,
    ) -> BoolFunctionExpr {
        let type_ = value.shape().type_();
        match value.source() {
            ConstantFunctionTemplateSource::Generic(source) => {
                match self.generic_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        BoolFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        BoolFunctionExpr::constant(
                            reference.specialize(value.substitution(), ()),
                            type_,
                        )
                    }
                }
            }
            ConstantFunctionTemplateSource::Exact(source) => {
                match self.bool_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        BoolFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        let reference = reference.substitute(value.substitution());
                        let type_ = reference.shape().type_();
                        BoolFunctionExpr::constant(reference, type_)
                    }
                }
            }
        }
    }

    pub(crate) fn materialize_nil_function(
        &self,
        value: &ConstantNilFunctionInstantiation,
    ) -> NilFunctionExpr {
        let type_ = value.shape().type_();
        match value.source() {
            ConstantFunctionTemplateSource::Generic(source) => {
                match self.generic_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        NilFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        NilFunctionExpr::constant(
                            reference.specialize(value.substitution(), ()),
                            type_,
                        )
                    }
                }
            }
            ConstantFunctionTemplateSource::Exact(source) => {
                match self.nil_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        NilFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        let reference = reference.substitute(value.substitution());
                        let type_ = reference.shape().type_();
                        NilFunctionExpr::constant(reference, type_)
                    }
                }
            }
        }
    }

    pub(crate) fn materialize_tuple_function(
        &self,
        value: &ConstantTupleFunctionInstantiation,
    ) -> TupleFunctionExpr {
        let type_ = value.shape().type_();
        match value.source() {
            ConstantFunctionTemplateSource::Generic(source) => {
                match self.generic_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        TupleFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        TupleFunctionExpr::constant(
                            reference.specialize(value.substitution(), value.return_().clone()),
                            type_,
                        )
                    }
                }
            }
            ConstantFunctionTemplateSource::Exact(source) => {
                match self.tuple_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        TupleFunctionExpr::reference(Self::typed_function_reference(
                            reference,
                            value.substitution(),
                        ))
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        let reference = reference.substitute(value.substitution());
                        let type_ = reference.shape().type_();
                        TupleFunctionExpr::constant(reference, type_)
                    }
                }
            }
        }
    }

    pub(crate) fn materialize_list_function(
        &self,
        value: &ConstantListFunctionInstantiation,
    ) -> ListFunctionExpr {
        let type_ = value.shape().type_();
        let item_type = value.return_().value_type();
        match value.source() {
            ConstantFunctionTemplateSource::Generic(source) => {
                match self.generic_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        ListFunctionExpr::reference(
                            Self::typed_function_reference(reference, value.substitution()),
                            item_type,
                        )
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        ListFunctionExpr::constant(
                            reference.specialize(value.substitution(), value.return_().clone()),
                            type_,
                            item_type,
                        )
                    }
                }
            }
            ConstantFunctionTemplateSource::Exact(source) => {
                match self.list_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        ListFunctionExpr::reference(
                            Self::typed_function_reference(reference, value.substitution()),
                            item_type,
                        )
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        let reference = reference.substitute(value.substitution());
                        let type_ = reference.shape().type_();
                        let item_type = reference.return_().value_type();
                        ListFunctionExpr::constant(reference, type_, item_type)
                    }
                }
            }
        }
    }

    pub(crate) fn materialize_custom_function(
        &self,
        value: &ConstantCustomFunctionInstantiation,
    ) -> CustomFunctionExpr {
        let type_ = crate::plan::CustomFunctionType::from_shapes(
            value.shape().argument_shapes().to_vec(),
            value.return_().clone(),
        );
        match value.source() {
            ConstantFunctionTemplateSource::Generic(source) => {
                match self.generic_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        CustomFunctionExpr::reference(
                            Self::typed_function_reference(reference, value.substitution()),
                            value.return_().clone(),
                        )
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        CustomFunctionExpr::constant(
                            reference.specialize(value.substitution(), value.return_().clone()),
                            type_,
                        )
                    }
                }
            }
            ConstantFunctionTemplateSource::Exact(source) => {
                match self.custom_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(
                        ConstantCustomFunctionTarget::Reference(reference),
                    ) => CustomFunctionExpr::reference(
                        Self::typed_function_reference(reference, value.substitution()),
                        value.return_().clone(),
                    ),
                    TypedConstantFunctionValueKind::Target(
                        ConstantCustomFunctionTarget::Constructor(constructor),
                    ) => CustomFunctionExpr::constructor(
                        constructor.substitute(value.substitution()),
                    ),
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        let reference = reference.substitute(value.substitution());
                        let type_ = crate::plan::CustomFunctionType::from_shapes(
                            reference.shape().argument_shapes().to_vec(),
                            reference.return_().clone(),
                        );
                        CustomFunctionExpr::constant(reference, type_)
                    }
                }
            }
        }
    }

    pub(crate) fn materialize_function_function(
        &self,
        value: &ConstantFunctionFunctionInstantiation,
    ) -> FunctionFunctionExpr {
        let type_ = crate::plan::FunctionFunctionType::from_shapes(
            value.shape().argument_shapes().to_vec(),
            value.return_().as_ref().clone(),
        );
        match value.source() {
            ConstantFunctionTemplateSource::Generic(source) => {
                match self.generic_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        FunctionFunctionExpr::reference(
                            Self::typed_function_reference(reference, value.substitution()),
                            value.return_().type_(),
                        )
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        FunctionFunctionExpr::constant(
                            reference.specialize(value.substitution(), value.return_().clone()),
                            type_,
                        )
                    }
                }
            }
            ConstantFunctionTemplateSource::Exact(source) => {
                match self.function_function(*source).kind() {
                    TypedConstantFunctionValueKind::Target(reference) => {
                        FunctionFunctionExpr::reference(
                            Self::typed_function_reference(reference, value.substitution()),
                            value.return_().type_(),
                        )
                    }
                    TypedConstantFunctionValueKind::Reference(reference) => {
                        let reference = reference.substitute(value.substitution());
                        let type_ = crate::plan::FunctionFunctionType::from_shapes(
                            reference.shape().argument_shapes().to_vec(),
                            reference.return_().as_ref().clone(),
                        );
                        FunctionFunctionExpr::constant(reference, type_)
                    }
                }
            }
        }
    }

    pub(crate) fn materialize_generic_list(
        &self,
        value: &ConstantGenericListInstantiation,
    ) -> GenericListExpr {
        if !self.owns(value.module()) {
            let parameter = *value.item_shape();
            return GenericListExpr::constant(
                ValueShape::Parameter(parameter),
                GenericListItem::new(parameter),
                value.clone(),
            );
        }
        self.materialize_generic_parameter_list(
            self.generic_list(*value.source()),
            value.substitution(),
            *value.item_shape(),
        )
    }

    fn materialize_list_value(
        &self,
        value: &ConstantListValue,
        substitution: &TypeSubstitution,
    ) -> ListExpr {
        match value {
            ConstantListValue::Generic(value) => {
                let item_shape = ValueShape::Parameter(value.parameter()).substitute(substitution);
                self.materialize_generic_list_value(value, substitution, &item_shape)
            }
            ConstantListValue::ParameterList(value) => {
                match ValueShape::Parameter(*value.item_shape())
                    .substitute(substitution)
                    .representation()
                {
                    crate::plan::ValueRepresentation::Uninhabited(parameter) => {
                        ListExpr::ParameterList(self.materialize_parameter_list_list_value(
                            value,
                            substitution,
                            parameter,
                        ))
                    }
                    crate::plan::ValueRepresentation::Stored(shape) => {
                        ListExpr::List(self.materialize_parameter_list_list_value_as_stored(
                            value,
                            substitution,
                            &shape,
                        ))
                    }
                }
            }
            ConstantListValue::Int(value) => {
                ListExpr::Int(self.materialize_int_list_value(value, substitution))
            }
            ConstantListValue::String(value) => {
                ListExpr::String(self.materialize_string_list_value(value, substitution))
            }
            ConstantListValue::BitArray(value) => {
                ListExpr::BitArray(self.materialize_bit_array_list_value(value, substitution))
            }
            ConstantListValue::UtfCodepoint(value) => ListExpr::UtfCodepoint(
                self.materialize_utf_codepoint_list_value(value, substitution),
            ),
            ConstantListValue::Custom(value) => {
                ListExpr::Custom(self.materialize_custom_list_value(value, substitution))
            }
            ConstantListValue::Float(value) => {
                ListExpr::Float(self.materialize_float_list_value(value, substitution))
            }
            ConstantListValue::Bool(value) => {
                ListExpr::Bool(self.materialize_bool_list_value(value, substitution))
            }
            ConstantListValue::Nil(value) => {
                ListExpr::Nil(self.materialize_nil_list_value(value, substitution))
            }
            ConstantListValue::Tuple(value) => {
                ListExpr::Tuple(self.materialize_tuple_list_value(value, substitution))
            }
            ConstantListValue::List(value) => {
                ListExpr::List(self.materialize_list_list_value(value, substitution))
            }
            ConstantListValue::Function(value) => {
                ListExpr::Function(self.materialize_function_list_value(value, substitution))
            }
        }
    }

    fn materialize_generic_list_value(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
        item_shape: &ValueShape,
    ) -> ListExpr {
        match item_shape {
            ValueShape::Parameter(parameter) => ListExpr::Generic(
                self.materialize_generic_parameter_list(value, substitution, *parameter),
            ),
            ValueShape::Int => {
                ListExpr::Int(self.materialize_generic_int_list(value, substitution))
            }
            ValueShape::String => {
                ListExpr::String(self.materialize_generic_string_list(value, substitution))
            }
            ValueShape::BitArray => {
                ListExpr::BitArray(self.materialize_generic_bit_array_list(value, substitution))
            }
            ValueShape::UtfCodepoint => ListExpr::UtfCodepoint(
                self.materialize_generic_utf_codepoint_list(value, substitution),
            ),
            ValueShape::Custom(shape) => {
                ListExpr::Custom(self.materialize_generic_custom_list(value, substitution, shape))
            }
            ValueShape::Float => {
                ListExpr::Float(self.materialize_generic_float_list(value, substitution))
            }
            ValueShape::Bool => {
                ListExpr::Bool(self.materialize_generic_bool_list(value, substitution))
            }
            ValueShape::Nil => {
                ListExpr::Nil(self.materialize_generic_nil_list(value, substitution))
            }
            ValueShape::Tuple(shape) => {
                ListExpr::Tuple(self.materialize_generic_tuple_list(value, substitution, shape))
            }
            ValueShape::List(shape) => match shape.representation() {
                crate::plan::ValueRepresentation::Uninhabited(parameter) => {
                    ListExpr::ParameterList(self.materialize_generic_parameter_list_list(
                        value,
                        substitution,
                        parameter,
                    ))
                }
                crate::plan::ValueRepresentation::Stored(item_shape) => ListExpr::List(
                    self.materialize_generic_list_list(value, substitution, &item_shape),
                ),
            },
            ValueShape::Function(shape) => ListExpr::Function(
                self.materialize_generic_function_list(value, substitution, shape),
            ),
        }
    }

    fn materialize_generic_stored_list_value(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
        shape: &crate::plan::ValueStorageShape,
    ) -> StoredListExpr {
        match shape {
            crate::plan::ValueStorageShape::Int => {
                StoredListExpr::Int(self.materialize_generic_int_list(value, substitution))
            }
            crate::plan::ValueStorageShape::Float => {
                StoredListExpr::Float(self.materialize_generic_float_list(value, substitution))
            }
            crate::plan::ValueStorageShape::String => {
                StoredListExpr::String(self.materialize_generic_string_list(value, substitution))
            }
            crate::plan::ValueStorageShape::BitArray => StoredListExpr::BitArray(
                self.materialize_generic_bit_array_list(value, substitution),
            ),
            crate::plan::ValueStorageShape::UtfCodepoint => StoredListExpr::UtfCodepoint(
                self.materialize_generic_utf_codepoint_list(value, substitution),
            ),
            crate::plan::ValueStorageShape::Custom(shape) => StoredListExpr::Custom(
                self.materialize_generic_custom_list(value, substitution, shape),
            ),
            crate::plan::ValueStorageShape::Bool => {
                StoredListExpr::Bool(self.materialize_generic_bool_list(value, substitution))
            }
            crate::plan::ValueStorageShape::Nil => {
                StoredListExpr::Nil(self.materialize_generic_nil_list(value, substitution))
            }
            crate::plan::ValueStorageShape::Tuple(shape) => StoredListExpr::Tuple(
                self.materialize_generic_tuple_list(value, substitution, shape),
            ),
            crate::plan::ValueStorageShape::List(shape) => match shape.representation() {
                crate::plan::ValueRepresentation::Uninhabited(parameter) => {
                    StoredListExpr::ParameterList(self.materialize_generic_parameter_list_list(
                        value,
                        substitution,
                        parameter,
                    ))
                }
                crate::plan::ValueRepresentation::Stored(shape) => StoredListExpr::List(
                    self.materialize_generic_list_list(value, substitution, &shape),
                ),
            },
            crate::plan::ValueStorageShape::Function(shape) => StoredListExpr::Function(
                self.materialize_generic_function_list(value, substitution, shape),
            ),
        }
    }

    fn materialize_generic_parameter_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
        parameter: crate::plan::TypeParameterId,
    ) -> GenericListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => {
                GenericListExpr::value(GenericListItem::new(parameter), Vec::new())
                    .with_item_shape(ValueShape::Parameter(parameter))
            }
            ConstantGenericListValueKind::Reference(value) => {
                self.materialize_generic_list(&value.retarget_generic(substitution, parameter))
            }
        }
    }

    fn materialize_generic_int_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
    ) -> IntListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => {
                IntListExpr::value(IntListItem, Vec::new()).with_item_shape(ValueShape::Int)
            }
            ConstantGenericListValueKind::Reference(value) => {
                self.materialize_int_list(&value.retarget(substitution, ()))
            }
        }
    }

    fn materialize_generic_string_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
    ) -> StringListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => {
                StringListExpr::value(StringListItem, Vec::new())
                    .with_item_shape(ValueShape::String)
            }
            ConstantGenericListValueKind::Reference(value) => {
                self.materialize_string_list(&value.retarget(substitution, ()))
            }
        }
    }

    fn materialize_generic_bit_array_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
    ) -> BitArrayListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => {
                BitArrayListExpr::value(BitArrayListItem, Vec::new())
                    .with_item_shape(ValueShape::BitArray)
            }
            ConstantGenericListValueKind::Reference(value) => {
                self.materialize_bit_array_list(&value.retarget(substitution, ()))
            }
        }
    }

    fn materialize_generic_utf_codepoint_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
    ) -> UtfCodepointListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => {
                UtfCodepointListExpr::value(UtfCodepointListItem, Vec::new())
                    .with_item_shape(ValueShape::UtfCodepoint)
            }
            ConstantGenericListValueKind::Reference(value) => {
                self.materialize_utf_codepoint_list(&value.retarget(substitution, ()))
            }
        }
    }

    fn materialize_generic_custom_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
        shape: &CustomValueShape,
    ) -> CustomListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => {
                CustomListExpr::value(CustomListItem::new(shape.type_().clone()), Vec::new())
                    .with_item_shape(ValueShape::Custom(shape.clone()))
            }
            ConstantGenericListValueKind::Reference(value) => {
                self.materialize_custom_list(&value.retarget(substitution, shape.clone()))
            }
        }
    }

    fn materialize_generic_float_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
    ) -> FloatListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => {
                FloatListExpr::value(FloatListItem, Vec::new()).with_item_shape(ValueShape::Float)
            }
            ConstantGenericListValueKind::Reference(value) => {
                self.materialize_float_list(&value.retarget(substitution, ()))
            }
        }
    }

    fn materialize_generic_bool_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
    ) -> BoolListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => {
                BoolListExpr::value(BoolListItem, Vec::new()).with_item_shape(ValueShape::Bool)
            }
            ConstantGenericListValueKind::Reference(value) => {
                self.materialize_bool_list(&value.retarget(substitution, ()))
            }
        }
    }

    fn materialize_generic_nil_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
    ) -> NilListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => {
                NilListExpr::value(NilListItem, Vec::new()).with_item_shape(ValueShape::Nil)
            }
            ConstantGenericListValueKind::Reference(value) => {
                self.materialize_nil_list(&value.retarget(substitution, ()))
            }
        }
    }

    fn materialize_generic_tuple_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
        shape: &[ValueShape],
    ) -> TupleListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => TupleListExpr::value(
                TupleListItem::new(shape.iter().map(ValueShape::value_type).collect()),
                Vec::new(),
            )
            .with_item_shape(ValueShape::Tuple(shape.to_vec().into_boxed_slice())),
            ConstantGenericListValueKind::Reference(value) => self.materialize_tuple_list(
                &value.retarget(substitution, shape.to_vec().into_boxed_slice()),
            ),
        }
    }

    fn materialize_generic_list_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
        shape: &crate::plan::ValueStorageShape,
    ) -> ListListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => {
                ListListExpr::value(ListListItem::new(shape.clone()), Vec::new())
                    .with_item_shape(ValueShape::List(Box::new(shape.to_value_shape())))
            }
            ConstantGenericListValueKind::Reference(value) => {
                self.materialize_list_list(&value.retarget_nested(substitution, shape.clone()))
            }
        }
    }

    fn materialize_generic_parameter_list_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
        parameter: crate::plan::TypeParameterId,
    ) -> crate::plan::ParameterListListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => crate::plan::ParameterListListExpr::value(
                crate::plan::ParameterListListItem::new(parameter),
                Vec::new(),
            )
            .with_item_shape(ValueShape::List(Box::new(ValueShape::Parameter(parameter)))),
            ConstantGenericListValueKind::Reference(value) => {
                self.materialize_parameter_list_list(&value.retarget(substitution, parameter))
            }
        }
    }

    fn materialize_generic_function_list(
        &self,
        value: &ConstantGenericListValue,
        substitution: &TypeSubstitution,
        shape: &FunctionShape,
    ) -> FunctionListExpr {
        match value.kind() {
            ConstantGenericListValueKind::Empty => {
                FunctionListExpr::value(FunctionListItem::new(shape.type_()), Vec::new())
                    .with_item_shape(ValueShape::Function(Box::new(shape.clone())))
            }
            ConstantGenericListValueKind::Reference(value) => {
                self.materialize_function_list(&value.retarget(substitution, shape.clone()))
            }
        }
    }

    pub(crate) fn materialize_int_list(&self, value: &ConstantIntListInstantiation) -> IntListExpr {
        if !self.owns(value.module()) {
            return IntListExpr::constant(ValueShape::Int, IntListItem, value.clone());
        }
        match value.source() {
            ConstantListTemplateSource::Generic(source) => {
                self.materialize_generic_int_list(self.generic_list(*source), value.substitution())
            }
            ConstantListTemplateSource::Exact(source) => {
                self.materialize_int_list_value(self.int_list(*source), value.substitution())
            }
        }
    }

    fn materialize_int_list_value(
        &self,
        value: &ConstantIntListValue,
        substitution: &TypeSubstitution,
    ) -> IntListExpr {
        match value.kind() {
            TypedConstantListValueKind::Value(elements) => IntListExpr::value(
                IntListItem,
                elements
                    .iter()
                    .map(|value| self.materialize_int_value(value, substitution))
                    .collect(),
            ),
            TypedConstantListValueKind::Spread { elements, tail } => IntListExpr::spread(
                elements.mapped_ref(|value| self.materialize_int_value(value, substitution)),
                self.materialize_int_list_value(tail, substitution),
            ),
            TypedConstantListValueKind::Reference(value) => {
                self.materialize_int_list(&value.substitute_leaf(substitution))
            }
        }
    }

    pub(crate) fn materialize_string_list(
        &self,
        value: &ConstantStringListInstantiation,
    ) -> StringListExpr {
        if !self.owns(value.module()) {
            return StringListExpr::constant(ValueShape::String, StringListItem, value.clone());
        }
        match value.source() {
            ConstantListTemplateSource::Generic(source) => self
                .materialize_generic_string_list(self.generic_list(*source), value.substitution()),
            ConstantListTemplateSource::Exact(source) => {
                self.materialize_string_list_value(self.string_list(*source), value.substitution())
            }
        }
    }

    fn materialize_string_list_value(
        &self,
        value: &ConstantStringListValue,
        substitution: &TypeSubstitution,
    ) -> StringListExpr {
        match value.kind() {
            TypedConstantListValueKind::Value(elements) => StringListExpr::value(
                StringListItem,
                elements
                    .iter()
                    .map(|value| self.materialize_string_value(value, substitution))
                    .collect(),
            ),
            TypedConstantListValueKind::Spread { elements, tail } => StringListExpr::spread(
                elements.mapped_ref(|value| self.materialize_string_value(value, substitution)),
                self.materialize_string_list_value(tail, substitution),
            ),
            TypedConstantListValueKind::Reference(value) => {
                self.materialize_string_list(&value.substitute_leaf(substitution))
            }
        }
    }

    pub(crate) fn materialize_bit_array_list(
        &self,
        value: &ConstantBitArrayListInstantiation,
    ) -> BitArrayListExpr {
        if !self.owns(value.module()) {
            return BitArrayListExpr::constant(
                ValueShape::BitArray,
                BitArrayListItem,
                value.clone(),
            );
        }
        match value.source() {
            ConstantListTemplateSource::Generic(source) => self.materialize_generic_bit_array_list(
                self.generic_list(*source),
                value.substitution(),
            ),
            ConstantListTemplateSource::Exact(source) => self.materialize_bit_array_list_value(
                self.bit_array_list(*source),
                value.substitution(),
            ),
        }
    }

    fn materialize_bit_array_list_value(
        &self,
        value: &ConstantBitArrayListValue,
        substitution: &TypeSubstitution,
    ) -> BitArrayListExpr {
        match value.kind() {
            TypedConstantListValueKind::Value(elements) => BitArrayListExpr::value(
                BitArrayListItem,
                elements
                    .iter()
                    .map(|value| self.materialize_bit_array_value(value, substitution))
                    .collect(),
            ),
            TypedConstantListValueKind::Spread { elements, tail } => BitArrayListExpr::spread(
                elements.mapped_ref(|value| self.materialize_bit_array_value(value, substitution)),
                self.materialize_bit_array_list_value(tail, substitution),
            ),
            TypedConstantListValueKind::Reference(value) => {
                self.materialize_bit_array_list(&value.substitute_leaf(substitution))
            }
        }
    }

    pub(crate) fn materialize_utf_codepoint_list(
        &self,
        value: &ConstantUtfCodepointListInstantiation,
    ) -> UtfCodepointListExpr {
        if !self.owns(value.module()) {
            return UtfCodepointListExpr::constant(
                ValueShape::UtfCodepoint,
                UtfCodepointListItem,
                value.clone(),
            );
        }
        match value.source() {
            ConstantListTemplateSource::Generic(source) => self
                .materialize_generic_utf_codepoint_list(
                    self.generic_list(*source),
                    value.substitution(),
                ),
            ConstantListTemplateSource::Exact(source) => self.materialize_utf_codepoint_list_value(
                self.utf_codepoint_list(*source),
                value.substitution(),
            ),
        }
    }

    fn materialize_utf_codepoint_list_value(
        &self,
        value: &ConstantUtfCodepointListValue,
        substitution: &TypeSubstitution,
    ) -> UtfCodepointListExpr {
        match value.kind() {
            ConstantUtfCodepointListValueKind::Empty => {
                UtfCodepointListExpr::value(UtfCodepointListItem, Vec::new())
            }
            ConstantUtfCodepointListValueKind::Reference(value) => {
                self.materialize_utf_codepoint_list(&value.substitute_leaf(substitution))
            }
        }
    }

    pub(crate) fn materialize_custom_list(
        &self,
        value: &ConstantCustomListInstantiation,
    ) -> CustomListExpr {
        if !self.owns(value.module()) {
            let shape = value.item_shape().clone();
            return CustomListExpr::constant(
                ValueShape::Custom(shape.clone()),
                CustomListItem::new(shape.type_().clone()),
                value.clone(),
            );
        }
        match value.source() {
            ConstantListTemplateSource::Generic(source) => self.materialize_generic_custom_list(
                self.generic_list(*source),
                value.substitution(),
                value.item_shape(),
            ),
            ConstantListTemplateSource::Exact(source) => {
                self.materialize_custom_list_value(self.custom_list(*source), value.substitution())
            }
        }
    }

    fn materialize_custom_list_value(
        &self,
        value: &ConstantCustomListValue,
        substitution: &TypeSubstitution,
    ) -> CustomListExpr {
        let shape = value.item_shape().substitute(substitution);
        match value.kind() {
            TypedConstantListValueKind::Value(elements) => CustomListExpr::value(
                CustomListItem::new(shape.type_().clone()),
                elements
                    .iter()
                    .map(|value| self.materialize_custom_value(value, substitution))
                    .collect(),
            )
            .with_item_shape(ValueShape::Custom(shape)),
            TypedConstantListValueKind::Spread { elements, tail } => CustomListExpr::spread(
                elements.mapped_ref(|value| self.materialize_custom_value(value, substitution)),
                self.materialize_custom_list_value(tail, substitution),
            )
            .with_item_shape(ValueShape::Custom(shape)),
            TypedConstantListValueKind::Reference(value) => {
                self.materialize_custom_list(&value.substitute_custom(substitution))
            }
        }
    }

    pub(crate) fn materialize_float_list(
        &self,
        value: &ConstantFloatListInstantiation,
    ) -> FloatListExpr {
        if !self.owns(value.module()) {
            return FloatListExpr::constant(ValueShape::Float, FloatListItem, value.clone());
        }
        match value.source() {
            ConstantListTemplateSource::Generic(source) => self
                .materialize_generic_float_list(self.generic_list(*source), value.substitution()),
            ConstantListTemplateSource::Exact(source) => {
                self.materialize_float_list_value(self.float_list(*source), value.substitution())
            }
        }
    }

    fn materialize_float_list_value(
        &self,
        value: &ConstantFloatListValue,
        substitution: &TypeSubstitution,
    ) -> FloatListExpr {
        match value.kind() {
            TypedConstantListValueKind::Value(elements) => FloatListExpr::value(
                FloatListItem,
                elements
                    .iter()
                    .map(|value| self.materialize_float_value(value, substitution))
                    .collect(),
            ),
            TypedConstantListValueKind::Spread { elements, tail } => FloatListExpr::spread(
                elements.mapped_ref(|value| self.materialize_float_value(value, substitution)),
                self.materialize_float_list_value(tail, substitution),
            ),
            TypedConstantListValueKind::Reference(value) => {
                self.materialize_float_list(&value.substitute_leaf(substitution))
            }
        }
    }

    pub(crate) fn materialize_bool_list(
        &self,
        value: &ConstantBoolListInstantiation,
    ) -> BoolListExpr {
        if !self.owns(value.module()) {
            return BoolListExpr::constant(ValueShape::Bool, BoolListItem, value.clone());
        }
        match value.source() {
            ConstantListTemplateSource::Generic(source) => {
                self.materialize_generic_bool_list(self.generic_list(*source), value.substitution())
            }
            ConstantListTemplateSource::Exact(source) => {
                self.materialize_bool_list_value(self.bool_list(*source), value.substitution())
            }
        }
    }

    fn materialize_bool_list_value(
        &self,
        value: &ConstantBoolListValue,
        substitution: &TypeSubstitution,
    ) -> BoolListExpr {
        match value.kind() {
            TypedConstantListValueKind::Value(elements) => BoolListExpr::value(
                BoolListItem,
                elements
                    .iter()
                    .map(|value| self.materialize_bool_value(value, substitution))
                    .collect(),
            ),
            TypedConstantListValueKind::Spread { elements, tail } => BoolListExpr::spread(
                elements.mapped_ref(|value| self.materialize_bool_value(value, substitution)),
                self.materialize_bool_list_value(tail, substitution),
            ),
            TypedConstantListValueKind::Reference(value) => {
                self.materialize_bool_list(&value.substitute_leaf(substitution))
            }
        }
    }

    pub(crate) fn materialize_nil_list(&self, value: &ConstantNilListInstantiation) -> NilListExpr {
        if !self.owns(value.module()) {
            return NilListExpr::constant(ValueShape::Nil, NilListItem, value.clone());
        }
        match value.source() {
            ConstantListTemplateSource::Generic(source) => {
                self.materialize_generic_nil_list(self.generic_list(*source), value.substitution())
            }
            ConstantListTemplateSource::Exact(source) => {
                self.materialize_nil_list_value(self.nil_list(*source), value.substitution())
            }
        }
    }

    fn materialize_nil_list_value(
        &self,
        value: &ConstantNilListValue,
        substitution: &TypeSubstitution,
    ) -> NilListExpr {
        match value.kind() {
            TypedConstantListValueKind::Value(elements) => NilListExpr::value(
                NilListItem,
                elements
                    .iter()
                    .map(|value| self.materialize_nil_value(value, substitution))
                    .collect(),
            ),
            TypedConstantListValueKind::Spread { elements, tail } => NilListExpr::spread(
                elements.mapped_ref(|value| self.materialize_nil_value(value, substitution)),
                self.materialize_nil_list_value(tail, substitution),
            ),
            TypedConstantListValueKind::Reference(value) => {
                self.materialize_nil_list(&value.substitute_leaf(substitution))
            }
        }
    }

    pub(crate) fn materialize_tuple_list(
        &self,
        value: &ConstantTupleListInstantiation,
    ) -> TupleListExpr {
        if !self.owns(value.module()) {
            let shape = value.item_shape().clone();
            return TupleListExpr::constant(
                ValueShape::Tuple(shape.clone()),
                TupleListItem::new(shape.iter().map(ValueShape::value_type).collect()),
                value.clone(),
            );
        }
        match value.source() {
            ConstantListTemplateSource::Generic(source) => self.materialize_generic_tuple_list(
                self.generic_list(*source),
                value.substitution(),
                value.item_shape(),
            ),
            ConstantListTemplateSource::Exact(source) => {
                self.materialize_tuple_list_value(self.tuple_list(*source), value.substitution())
            }
        }
    }

    fn materialize_tuple_list_value(
        &self,
        value: &ConstantTupleListValue,
        substitution: &TypeSubstitution,
    ) -> TupleListExpr {
        let shape = value
            .item_shape()
            .iter()
            .map(|shape| shape.substitute(substitution))
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let item = TupleListItem::new(shape.iter().map(ValueShape::value_type).collect());
        match value.kind() {
            TypedConstantListValueKind::Value(elements) => TupleListExpr::value(
                item,
                elements
                    .iter()
                    .map(|value| self.materialize_tuple_value(value, substitution))
                    .collect(),
            )
            .with_item_shape(ValueShape::Tuple(shape)),
            TypedConstantListValueKind::Spread { elements, tail } => TupleListExpr::spread(
                elements.mapped_ref(|value| self.materialize_tuple_value(value, substitution)),
                self.materialize_tuple_list_value(tail, substitution),
            )
            .with_item_shape(ValueShape::Tuple(shape)),
            TypedConstantListValueKind::Reference(value) => {
                self.materialize_tuple_list(&value.substitute_tuple(substitution))
            }
        }
    }

    pub(crate) fn materialize_parameter_list_list(
        &self,
        value: &ConstantParameterListListInstantiation,
    ) -> ParameterListListExpr {
        if !self.owns(value.module()) {
            let parameter = *value.item_shape();
            return ParameterListListExpr::constant(
                ValueShape::List(Box::new(ValueShape::Parameter(parameter))),
                ParameterListListItem::new(parameter),
                value.clone(),
            );
        }
        match value.source() {
            ConstantListTemplateSource::Generic(source) => self
                .materialize_generic_parameter_list_list(
                    self.generic_list(*source),
                    value.substitution(),
                    *value.item_shape(),
                ),
            ConstantListTemplateSource::Exact(source) => self
                .materialize_parameter_list_list_value(
                    self.parameter_list_list(*source),
                    value.substitution(),
                    *value.item_shape(),
                ),
        }
    }

    fn materialize_parameter_list_list_value(
        &self,
        value: &ConstantParameterListListValue,
        substitution: &TypeSubstitution,
        parameter: crate::plan::TypeParameterId,
    ) -> ParameterListListExpr {
        match value.kind() {
            TypedConstantListValueKind::Value(elements) => ParameterListListExpr::value(
                ParameterListListItem::new(parameter),
                elements
                    .iter()
                    .map(|value| {
                        self.materialize_generic_parameter_list(value, substitution, parameter)
                    })
                    .collect(),
            )
            .with_item_shape(ValueShape::List(Box::new(ValueShape::Parameter(parameter)))),
            TypedConstantListValueKind::Spread { elements, tail } => ParameterListListExpr::spread(
                elements.mapped_ref(|value| {
                    self.materialize_generic_parameter_list(value, substitution, parameter)
                }),
                self.materialize_parameter_list_list_value(tail, substitution, parameter),
            )
            .with_item_shape(ValueShape::List(Box::new(ValueShape::Parameter(parameter)))),
            TypedConstantListValueKind::Reference(value) => self.materialize_parameter_list_list(
                &value.retarget_parameter(substitution, parameter),
            ),
        }
    }

    fn materialize_parameter_list_list_value_as_stored(
        &self,
        value: &ConstantParameterListListValue,
        substitution: &TypeSubstitution,
        shape: &crate::plan::ValueStorageShape,
    ) -> ListListExpr {
        match value.kind() {
            TypedConstantListValueKind::Value(elements) => ListListExpr::value(
                ListListItem::new(shape.clone()),
                elements
                    .iter()
                    .map(|value| {
                        self.materialize_generic_stored_list_value(value, substitution, shape)
                    })
                    .collect(),
            )
            .with_item_shape(ValueShape::List(Box::new(shape.to_value_shape()))),
            TypedConstantListValueKind::Spread { elements, tail } => ListListExpr::spread(
                elements.mapped_ref(|value| {
                    self.materialize_generic_stored_list_value(value, substitution, shape)
                }),
                self.materialize_parameter_list_list_value_as_stored(tail, substitution, shape),
            )
            .with_item_shape(ValueShape::List(Box::new(shape.to_value_shape()))),
            TypedConstantListValueKind::Reference(value) => {
                self.materialize_list_list(&value.retarget_stored(substitution, shape.clone()))
            }
        }
    }

    fn materialize_stored_list_value(
        &self,
        value: &ConstantStoredListValue,
        substitution: &TypeSubstitution,
    ) -> StoredListExpr {
        match value {
            ConstantStoredListValue::ParameterList(value) => {
                match ValueShape::Parameter(*value.item_shape())
                    .substitute(substitution)
                    .representation()
                {
                    crate::plan::ValueRepresentation::Uninhabited(parameter) => {
                        StoredListExpr::ParameterList(self.materialize_parameter_list_list_value(
                            value,
                            substitution,
                            parameter,
                        ))
                    }
                    crate::plan::ValueRepresentation::Stored(shape) => {
                        StoredListExpr::List(self.materialize_parameter_list_list_value_as_stored(
                            value,
                            substitution,
                            &shape,
                        ))
                    }
                }
            }
            ConstantStoredListValue::Int(value) => {
                StoredListExpr::Int(self.materialize_int_list_value(value, substitution))
            }
            ConstantStoredListValue::String(value) => {
                StoredListExpr::String(self.materialize_string_list_value(value, substitution))
            }
            ConstantStoredListValue::BitArray(value) => {
                StoredListExpr::BitArray(self.materialize_bit_array_list_value(value, substitution))
            }
            ConstantStoredListValue::UtfCodepoint(value) => StoredListExpr::UtfCodepoint(
                self.materialize_utf_codepoint_list_value(value, substitution),
            ),
            ConstantStoredListValue::Custom(value) => {
                StoredListExpr::Custom(self.materialize_custom_list_value(value, substitution))
            }
            ConstantStoredListValue::Float(value) => {
                StoredListExpr::Float(self.materialize_float_list_value(value, substitution))
            }
            ConstantStoredListValue::Bool(value) => {
                StoredListExpr::Bool(self.materialize_bool_list_value(value, substitution))
            }
            ConstantStoredListValue::Nil(value) => {
                StoredListExpr::Nil(self.materialize_nil_list_value(value, substitution))
            }
            ConstantStoredListValue::Tuple(value) => {
                StoredListExpr::Tuple(self.materialize_tuple_list_value(value, substitution))
            }
            ConstantStoredListValue::List(value) => {
                StoredListExpr::List(self.materialize_list_list_value(value, substitution))
            }
            ConstantStoredListValue::Function(value) => {
                StoredListExpr::Function(self.materialize_function_list_value(value, substitution))
            }
        }
    }

    pub(crate) fn materialize_list_list(
        &self,
        value: &ConstantListListInstantiation,
    ) -> ListListExpr {
        if !self.owns(value.module()) {
            let shape = value.item_shape().clone();
            return ListListExpr::constant(
                ValueShape::List(Box::new(shape.to_value_shape())),
                ListListItem::new(shape),
                value.clone(),
            );
        }
        match value.source() {
            ConstantNestedListTemplateSource::Generic(source) => self
                .materialize_generic_list_list(
                    self.generic_list(*source),
                    value.substitution(),
                    value.item_shape(),
                ),
            ConstantNestedListTemplateSource::ParameterList(source) => self
                .materialize_parameter_list_list_value_as_stored(
                    self.parameter_list_list(*source),
                    value.substitution(),
                    value.item_shape(),
                ),
            ConstantNestedListTemplateSource::Exact(source) => {
                self.materialize_list_list_value(self.list_list(*source), value.substitution())
            }
        }
    }

    fn materialize_list_list_value(
        &self,
        value: &ConstantListListValue,
        substitution: &TypeSubstitution,
    ) -> ListListExpr {
        let shape = value.item_shape().substitute(substitution);
        match value.kind() {
            TypedConstantListValueKind::Value(elements) => ListListExpr::value(
                ListListItem::new(shape.clone()),
                elements
                    .iter()
                    .map(|value| self.materialize_stored_list_value(value, substitution))
                    .collect(),
            )
            .with_item_shape(ValueShape::List(Box::new(shape.to_value_shape()))),
            TypedConstantListValueKind::Spread { elements, tail } => ListListExpr::spread(
                elements
                    .mapped_ref(|value| self.materialize_stored_list_value(value, substitution)),
                self.materialize_list_list_value(tail, substitution),
            )
            .with_item_shape(ValueShape::List(Box::new(shape.to_value_shape()))),
            TypedConstantListValueKind::Reference(value) => {
                self.materialize_list_list(&value.substitute_list(substitution))
            }
        }
    }

    pub(crate) fn materialize_function_list(
        &self,
        value: &ConstantFunctionListInstantiation,
    ) -> FunctionListExpr {
        if !self.owns(value.module()) {
            let shape = value.item_shape().clone();
            return FunctionListExpr::constant(
                ValueShape::Function(Box::new(shape.clone())),
                FunctionListItem::new(shape.type_()),
                value.clone(),
            );
        }
        match value.source() {
            ConstantListTemplateSource::Generic(source) => self.materialize_generic_function_list(
                self.generic_list(*source),
                value.substitution(),
                value.item_shape(),
            ),
            ConstantListTemplateSource::Exact(source) => self
                .materialize_function_list_value(self.function_list(*source), value.substitution()),
        }
    }

    fn materialize_function_list_value(
        &self,
        value: &ConstantFunctionListValue,
        substitution: &TypeSubstitution,
    ) -> FunctionListExpr {
        let shape = value.item_shape().substitute(substitution);
        match value.kind() {
            TypedConstantListValueKind::Value(elements) => FunctionListExpr::value(
                FunctionListItem::new(shape.type_()),
                elements
                    .iter()
                    .map(|value| self.materialize_function_value(value, substitution))
                    .collect(),
            )
            .with_item_shape(ValueShape::Function(Box::new(shape))),
            TypedConstantListValueKind::Spread { elements, tail } => FunctionListExpr::spread(
                elements.mapped_ref(|value| self.materialize_function_value(value, substitution)),
                self.materialize_function_list_value(tail, substitution),
            )
            .with_item_shape(ValueShape::Function(Box::new(shape))),
            TypedConstantListValueKind::Reference(value) => {
                self.materialize_function_list(&value.substitute_function(substitution))
            }
        }
    }
}

impl ConstantTemplateSignature {
    pub(crate) fn int(id: ConstantTemplateId, storage_index: usize, scheme: TypeScheme) -> Self {
        Self::new(
            id,
            scheme,
            ValueShape::Int,
            ConstantTemplateSignatureKind::Int(ConstantIntTemplateId(storage_index)),
        )
    }

    pub(crate) fn string(id: ConstantTemplateId, storage_index: usize, scheme: TypeScheme) -> Self {
        Self::new(
            id,
            scheme,
            ValueShape::String,
            ConstantTemplateSignatureKind::String(ConstantStringTemplateId(storage_index)),
        )
    }

    pub(crate) fn bit_array(
        id: ConstantTemplateId,
        storage_index: usize,
        scheme: TypeScheme,
    ) -> Self {
        Self::new(
            id,
            scheme,
            ValueShape::BitArray,
            ConstantTemplateSignatureKind::BitArray(ConstantBitArrayTemplateId(storage_index)),
        )
    }

    pub(crate) fn custom(
        id: ConstantTemplateId,
        storage_index: usize,
        scheme: TypeScheme,
        shape: CustomValueShape,
    ) -> Self {
        let kind_shape = shape.clone();
        Self::new(
            id,
            scheme,
            ValueShape::Custom(shape),
            ConstantTemplateSignatureKind::Custom {
                template: ConstantCustomTemplateId(storage_index),
                shape: kind_shape,
            },
        )
    }

    pub(crate) fn float(id: ConstantTemplateId, storage_index: usize, scheme: TypeScheme) -> Self {
        Self::new(
            id,
            scheme,
            ValueShape::Float,
            ConstantTemplateSignatureKind::Float(ConstantFloatTemplateId(storage_index)),
        )
    }

    pub(crate) fn bool(id: ConstantTemplateId, storage_index: usize, scheme: TypeScheme) -> Self {
        Self::new(
            id,
            scheme,
            ValueShape::Bool,
            ConstantTemplateSignatureKind::Bool(ConstantBoolTemplateId(storage_index)),
        )
    }

    pub(crate) fn nil(id: ConstantTemplateId, storage_index: usize, scheme: TypeScheme) -> Self {
        Self::new(
            id,
            scheme,
            ValueShape::Nil,
            ConstantTemplateSignatureKind::Nil(ConstantNilTemplateId(storage_index)),
        )
    }

    pub(crate) fn tuple(
        id: ConstantTemplateId,
        storage_index: usize,
        scheme: TypeScheme,
        shape: Box<[ValueShape]>,
    ) -> Self {
        let kind_shape = shape.clone();
        Self::new(
            id,
            scheme,
            ValueShape::Tuple(shape),
            ConstantTemplateSignatureKind::Tuple {
                template: ConstantTupleTemplateId(storage_index),
                shape: kind_shape,
            },
        )
    }

    pub(crate) fn list(
        id: ConstantTemplateId,
        storage_index: usize,
        scheme: TypeScheme,
        item_shape: ValueShape,
    ) -> Self {
        let template = ConstantListTemplate::from_item_shape(item_shape, storage_index);
        let item_shape = template.item_shape();
        Self::new(
            id,
            scheme,
            ValueShape::List(Box::new(item_shape)),
            ConstantTemplateSignatureKind::List(template),
        )
    }

    pub(crate) fn function(
        id: ConstantTemplateId,
        storage_index: usize,
        scheme: TypeScheme,
        shape: FunctionShape,
    ) -> Self {
        let template = ConstantFunctionTemplate::from_shape(&shape, storage_index);
        let kind_shape = shape.clone();
        Self::new(
            id,
            scheme,
            ValueShape::Function(Box::new(shape)),
            ConstantTemplateSignatureKind::Function {
                template,
                shape: kind_shape,
            },
        )
    }

    fn new(
        id: ConstantTemplateId,
        scheme: TypeScheme,
        shape: ValueShape,
        kind: ConstantTemplateSignatureKind,
    ) -> Self {
        Self {
            id,
            scheme,
            shape,
            kind,
        }
    }

    pub(crate) fn id(&self) -> ConstantTemplateId {
        self.id
    }

    pub(crate) fn scheme(&self) -> &TypeScheme {
        &self.scheme
    }

    pub(crate) fn shape(&self) -> &ValueShape {
        &self.shape
    }

    pub(crate) fn try_instantiate(
        &self,
        arguments: Vec<ValueShape>,
    ) -> Option<ConstantInstantiation> {
        let substitution = self.scheme.try_substitution(arguments)?;
        let kind = match &self.kind {
            ConstantTemplateSignatureKind::Int(template) => {
                ConstantInstantiationKind::Int(TypedConstantInstantiation::in_module(
                    self.id.module(),
                    *template,
                    substitution,
                    (),
                ))
            }
            ConstantTemplateSignatureKind::String(template) => {
                ConstantInstantiationKind::String(TypedConstantInstantiation::in_module(
                    self.id.module(),
                    *template,
                    substitution,
                    (),
                ))
            }
            ConstantTemplateSignatureKind::BitArray(template) => {
                ConstantInstantiationKind::BitArray(TypedConstantInstantiation::in_module(
                    self.id.module(),
                    *template,
                    substitution,
                    (),
                ))
            }
            ConstantTemplateSignatureKind::Custom { template, shape } => {
                let shape = shape.substitute(&substitution);
                ConstantInstantiationKind::Custom(TypedConstantInstantiation::in_module(
                    self.id.module(),
                    *template,
                    substitution,
                    shape,
                ))
            }
            ConstantTemplateSignatureKind::Float(template) => {
                ConstantInstantiationKind::Float(TypedConstantInstantiation::in_module(
                    self.id.module(),
                    *template,
                    substitution,
                    (),
                ))
            }
            ConstantTemplateSignatureKind::Bool(template) => {
                ConstantInstantiationKind::Bool(TypedConstantInstantiation::in_module(
                    self.id.module(),
                    *template,
                    substitution,
                    (),
                ))
            }
            ConstantTemplateSignatureKind::Nil(template) => {
                ConstantInstantiationKind::Nil(TypedConstantInstantiation::in_module(
                    self.id.module(),
                    *template,
                    substitution,
                    (),
                ))
            }
            ConstantTemplateSignatureKind::Tuple { template, shape } => {
                let shape = shape
                    .iter()
                    .map(|shape| shape.substitute(&substitution))
                    .collect::<Vec<_>>()
                    .into_boxed_slice();
                ConstantInstantiationKind::Tuple(TypedConstantInstantiation::in_module(
                    self.id.module(),
                    *template,
                    substitution,
                    shape,
                ))
            }
            ConstantTemplateSignatureKind::List(template) => ConstantInstantiationKind::List(
                template.instantiate(self.id.module(), substitution),
            ),
            ConstantTemplateSignatureKind::Function { template, shape } => {
                let shape = shape.substitute(&substitution);
                ConstantInstantiationKind::Function(template.instantiate(
                    self.id.module(),
                    substitution,
                    shape,
                ))
            }
        };
        Some(ConstantInstantiation { kind })
    }
}

impl<Id: Copy, Shape> TypedConstantInstantiation<Id, Shape> {
    #[cfg(test)]
    fn new(template: Id, substitution: TypeSubstitution, shape: Shape) -> Self {
        Self::in_module(crate::plan::ModuleId::root(), template, substitution, shape)
    }

    fn in_module(
        module: crate::plan::ModuleId,
        template: Id,
        substitution: TypeSubstitution,
        shape: Shape,
    ) -> Self {
        Self {
            module,
            template,
            substitution,
            shape,
        }
    }

    pub(crate) fn template(&self) -> Id {
        self.template
    }

    pub(crate) fn module(&self) -> crate::plan::ModuleId {
        self.module
    }

    pub(crate) fn substitution(&self) -> &TypeSubstitution {
        &self.substitution
    }

    pub(crate) fn shape(&self) -> &Shape {
        &self.shape
    }
}

impl<Id: Copy> TypedConstantInstantiation<Id, ()> {
    fn substitute_leaf(&self, outer: &TypeSubstitution) -> Self {
        Self::in_module(
            self.module,
            self.template,
            self.substitution.substitute(outer),
            (),
        )
    }
}

impl TypedConstantInstantiation<ConstantCustomTemplateId, CustomValueShape> {
    fn substitute_custom(&self, outer: &TypeSubstitution) -> Self {
        Self::in_module(
            self.module,
            self.template,
            self.substitution.substitute(outer),
            self.shape.substitute(outer),
        )
    }
}

impl TypedConstantInstantiation<ConstantTupleTemplateId, Box<[ValueShape]>> {
    fn substitute_tuple(&self, outer: &TypeSubstitution) -> Self {
        Self::in_module(
            self.module,
            self.template,
            self.substitution.substitute(outer),
            self.shape
                .iter()
                .map(|shape| shape.substitute(outer))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        )
    }
}

impl ConstantValue {
    pub(crate) fn int(value: BigInt) -> Self {
        Self {
            kind: ConstantValueKind::Int(ConstantIntValue::Value(value)),
        }
    }

    pub(crate) fn float(value: f64) -> Self {
        Self {
            kind: ConstantValueKind::Float(ConstantFloatValue::Value(value)),
        }
    }

    pub(crate) fn string(value: EcoString) -> Self {
        Self {
            kind: ConstantValueKind::String(ConstantStringValue::Value(value)),
        }
    }

    pub(crate) fn string_concatenation(
        left: ConstantStringValue,
        right: ConstantStringValue,
    ) -> Self {
        Self {
            kind: ConstantValueKind::String(ConstantStringValue::Concatenation {
                left: Box::new(left),
                right: Box::new(right),
            }),
        }
    }

    pub(crate) fn bool(value: bool) -> Self {
        Self {
            kind: ConstantValueKind::Bool(ConstantBoolValue::Value(value)),
        }
    }

    pub(crate) fn nil() -> Self {
        Self {
            kind: ConstantValueKind::Nil(ConstantNilValue::Value),
        }
    }

    pub(crate) fn tuple(element_shapes: Box<[ValueShape]>, elements: Box<[Self]>) -> Self {
        Self {
            kind: ConstantValueKind::Tuple(ConstantTupleValue {
                shape: element_shapes,
                kind: ConstantTupleValueKind::Value(elements),
            }),
        }
    }

    pub(crate) fn try_list(
        item_shape: ValueShape,
        elements: Vec<Self>,
        tail: Option<Self>,
    ) -> Result<Self, ConstantListConstructionError> {
        let tail = tail
            .map(|tail| {
                let actual = tail.shape();
                match tail.into_list() {
                    Some(tail) => Ok(tail),
                    None => Err(constant_list_tail_mismatch(&item_shape, actual)),
                }
            })
            .transpose()?;
        let parts = ConstantListParts::try_from_parts(elements, tail)?;

        let value = match &item_shape {
            ValueShape::Parameter(parameter) => {
                match parts {
                    ConstantListParts::Value(elements) if elements.is_empty() => {}
                    ConstantListParts::Value(elements) => {
                        return Err(constant_list_element_mismatch(
                            &item_shape,
                            elements[0].shape(),
                        ));
                    }
                    ConstantListParts::Spread { elements, .. } => {
                        return Err(constant_list_element_mismatch(
                            &item_shape,
                            elements.first().shape(),
                        ));
                    }
                }
                ConstantListValue::generic(*parameter)
            }
            ValueShape::Int => ConstantListValue::int(parts.try_map(
                |value| {
                    let actual = value.shape();
                    match value.into_int() {
                        Some(value) => Ok(value),
                        None => Err(constant_list_element_mismatch(&item_shape, actual)),
                    }
                },
                |tail| {
                    let actual = tail.shape();
                    match tail.into_int() {
                        Some(tail) => Ok(tail),
                        None => Err(constant_list_tail_mismatch(&item_shape, actual)),
                    }
                },
            )?),
            ValueShape::String => ConstantListValue::string(parts.try_map(
                |value| {
                    let actual = value.shape();
                    match value.into_string() {
                        Some(value) => Ok(value),
                        None => Err(constant_list_element_mismatch(&item_shape, actual)),
                    }
                },
                |tail| {
                    let actual = tail.shape();
                    match tail.into_string() {
                        Some(tail) => Ok(tail),
                        None => Err(constant_list_tail_mismatch(&item_shape, actual)),
                    }
                },
            )?),
            ValueShape::BitArray => ConstantListValue::bit_array(parts.try_map(
                |value| {
                    let actual = value.shape();
                    match value.into_bit_array() {
                        Some(value) => Ok(value),
                        None => Err(constant_list_element_mismatch(&item_shape, actual)),
                    }
                },
                |tail| {
                    let actual = tail.shape();
                    match tail.into_bit_array() {
                        Some(tail) => Ok(tail),
                        None => Err(constant_list_tail_mismatch(&item_shape, actual)),
                    }
                },
            )?),
            ValueShape::UtfCodepoint => {
                match parts {
                    ConstantListParts::Value(elements) if elements.is_empty() => {}
                    ConstantListParts::Value(elements) => {
                        return Err(constant_list_element_mismatch(
                            &item_shape,
                            elements[0].shape(),
                        ));
                    }
                    ConstantListParts::Spread { elements, .. } => {
                        return Err(constant_list_element_mismatch(
                            &item_shape,
                            elements.first().shape(),
                        ));
                    }
                }
                ConstantListValue::utf_codepoint()
            }
            ValueShape::Custom(shape) => ConstantListValue::custom(
                shape.clone(),
                parts.try_map(
                    |value| {
                        let actual = value.shape();
                        match value.kind {
                            ConstantValueKind::Custom(value) if actual.can_flow_to(&item_shape) => {
                                Ok(value)
                            }
                            _ => Err(constant_list_element_mismatch(&item_shape, actual)),
                        }
                    },
                    |tail| {
                        let actual = tail.shape();
                        match tail {
                            ConstantListValue::Custom(value)
                                if actual.can_flow_to(&ValueShape::List(Box::new(
                                    item_shape.clone(),
                                ))) =>
                            {
                                Ok(value)
                            }
                            _ => Err(constant_list_tail_mismatch(&item_shape, actual)),
                        }
                    },
                )?,
            ),
            ValueShape::Float => ConstantListValue::float(parts.try_map(
                |value| {
                    let actual = value.shape();
                    match value.into_float() {
                        Some(value) => Ok(value),
                        None => Err(constant_list_element_mismatch(&item_shape, actual)),
                    }
                },
                |tail| {
                    let actual = tail.shape();
                    match tail.into_float() {
                        Some(tail) => Ok(tail),
                        None => Err(constant_list_tail_mismatch(&item_shape, actual)),
                    }
                },
            )?),
            ValueShape::Bool => ConstantListValue::bool(parts.try_map(
                |value| {
                    let actual = value.shape();
                    match value.into_bool() {
                        Some(value) => Ok(value),
                        None => Err(constant_list_element_mismatch(&item_shape, actual)),
                    }
                },
                |tail| {
                    let actual = tail.shape();
                    match tail.into_bool() {
                        Some(tail) => Ok(tail),
                        None => Err(constant_list_tail_mismatch(&item_shape, actual)),
                    }
                },
            )?),
            ValueShape::Nil => ConstantListValue::nil(parts.try_map(
                |value| {
                    let actual = value.shape();
                    match value.into_nil() {
                        Some(value) => Ok(value),
                        None => Err(constant_list_element_mismatch(&item_shape, actual)),
                    }
                },
                |tail| {
                    let actual = tail.shape();
                    match tail.into_nil() {
                        Some(tail) => Ok(tail),
                        None => Err(constant_list_tail_mismatch(&item_shape, actual)),
                    }
                },
            )?),
            ValueShape::Tuple(shape) => ConstantListValue::tuple(
                shape.clone(),
                parts.try_map(
                    |value| {
                        let actual = value.shape();
                        match value.kind {
                            ConstantValueKind::Tuple(value) if actual.can_flow_to(&item_shape) => {
                                Ok(value)
                            }
                            _ => Err(constant_list_element_mismatch(&item_shape, actual)),
                        }
                    },
                    |tail| {
                        let actual = tail.shape();
                        match tail {
                            ConstantListValue::Tuple(value)
                                if actual.can_flow_to(&ValueShape::List(Box::new(
                                    item_shape.clone(),
                                ))) =>
                            {
                                Ok(value)
                            }
                            _ => Err(constant_list_tail_mismatch(&item_shape, actual)),
                        }
                    },
                )?,
            ),
            ValueShape::List(shape) => match shape.representation() {
                crate::plan::ValueRepresentation::Uninhabited(parameter) => {
                    ConstantListValue::parameter_list(
                        parameter,
                        parts.try_map(
                            |value| {
                                let actual = value.shape();
                                match value.kind {
                                    ConstantValueKind::List(ConstantListValue::Generic(value))
                                        if actual.can_flow_to(&item_shape) =>
                                    {
                                        Ok(value)
                                    }
                                    _ => Err(constant_list_element_mismatch(&item_shape, actual)),
                                }
                            },
                            |tail| {
                                let actual = tail.shape();
                                match tail {
                                    ConstantListValue::ParameterList(value)
                                        if actual.can_flow_to(&ValueShape::List(Box::new(
                                            item_shape.clone(),
                                        ))) =>
                                    {
                                        Ok(value)
                                    }
                                    _ => Err(constant_list_tail_mismatch(&item_shape, actual)),
                                }
                            },
                        )?,
                    )
                }
                crate::plan::ValueRepresentation::Stored(shape) => ConstantListValue::list(
                    shape,
                    parts.try_map(
                        |value| {
                            let actual = value.shape();
                            let value = match value.kind {
                                ConstantValueKind::List(ConstantListValue::ParameterList(
                                    value,
                                )) => ConstantStoredListValue::ParameterList(value),
                                ConstantValueKind::List(ConstantListValue::Int(value)) => {
                                    ConstantStoredListValue::Int(value)
                                }
                                ConstantValueKind::List(ConstantListValue::String(value)) => {
                                    ConstantStoredListValue::String(value)
                                }
                                ConstantValueKind::List(ConstantListValue::BitArray(value)) => {
                                    ConstantStoredListValue::BitArray(value)
                                }
                                ConstantValueKind::List(ConstantListValue::UtfCodepoint(value)) => {
                                    ConstantStoredListValue::UtfCodepoint(value)
                                }
                                ConstantValueKind::List(ConstantListValue::Custom(value)) => {
                                    ConstantStoredListValue::Custom(value)
                                }
                                ConstantValueKind::List(ConstantListValue::Float(value)) => {
                                    ConstantStoredListValue::Float(value)
                                }
                                ConstantValueKind::List(ConstantListValue::Bool(value)) => {
                                    ConstantStoredListValue::Bool(value)
                                }
                                ConstantValueKind::List(ConstantListValue::Nil(value)) => {
                                    ConstantStoredListValue::Nil(value)
                                }
                                ConstantValueKind::List(ConstantListValue::Tuple(value)) => {
                                    ConstantStoredListValue::Tuple(value)
                                }
                                ConstantValueKind::List(ConstantListValue::List(value)) => {
                                    ConstantStoredListValue::List(value)
                                }
                                ConstantValueKind::List(ConstantListValue::Function(value)) => {
                                    ConstantStoredListValue::Function(value)
                                }
                                _ => {
                                    return Err(constant_list_element_mismatch(
                                        &item_shape,
                                        actual,
                                    ));
                                }
                            };
                            if actual.can_flow_to(&item_shape) {
                                Ok(value)
                            } else {
                                Err(constant_list_element_mismatch(&item_shape, actual))
                            }
                        },
                        |tail| {
                            let actual = tail.shape();
                            match tail {
                                ConstantListValue::List(value)
                                    if actual.can_flow_to(&ValueShape::List(Box::new(
                                        item_shape.clone(),
                                    ))) =>
                                {
                                    Ok(value)
                                }
                                _ => Err(constant_list_tail_mismatch(&item_shape, actual)),
                            }
                        },
                    )?,
                ),
            },
            ValueShape::Function(shape) => ConstantListValue::function(
                (**shape).clone(),
                parts.try_map(
                    |value| {
                        let actual = value.shape();
                        match value.kind {
                            ConstantValueKind::Function(value)
                                if actual.can_flow_to(&item_shape) =>
                            {
                                Ok(value)
                            }
                            _ => Err(constant_list_element_mismatch(&item_shape, actual)),
                        }
                    },
                    |tail| {
                        let actual = tail.shape();
                        match tail {
                            ConstantListValue::Function(value)
                                if actual.can_flow_to(&ValueShape::List(Box::new(
                                    item_shape.clone(),
                                ))) =>
                            {
                                Ok(value)
                            }
                            _ => Err(constant_list_tail_mismatch(&item_shape, actual)),
                        }
                    },
                )?,
            ),
        };

        Ok(Self {
            kind: ConstantValueKind::List(value),
        })
    }

    pub(crate) fn bit_array(segments: Box<[ConstantBitArraySegment]>) -> Self {
        Self {
            kind: ConstantValueKind::BitArray(ConstantBitArrayValue {
                kind: ConstantBitArrayValueKind::Value(segments),
            }),
        }
    }

    pub(crate) fn custom(
        shape: CustomValueShape,
        constructor: CustomConstructor,
        fields: Box<[Self]>,
    ) -> Self {
        Self {
            kind: ConstantValueKind::Custom(ConstantCustomValue {
                shape,
                kind: ConstantCustomValueKind::Construction(ConstantCustomConstruction {
                    constructor,
                    fields,
                }),
            }),
        }
    }

    pub(crate) fn function(shape: FunctionShape, reference: FunctionReference) -> Self {
        let value = ConstantFunctionValue::function_reference(shape, reference);
        Self {
            kind: ConstantValueKind::Function(value),
        }
    }

    pub(crate) fn constructor_function(
        shape: FunctionShape,
        return_: CustomValueShape,
        constructor: CustomConstructor,
    ) -> Self {
        let value = ConstantFunctionValue::constructor(shape, return_, constructor);
        Self {
            kind: ConstantValueKind::Function(value),
        }
    }

    pub(crate) fn reference(instantiation: ConstantInstantiation) -> Self {
        let kind = match instantiation.kind {
            ConstantInstantiationKind::Int(value) => {
                ConstantValueKind::Int(ConstantIntValue::Reference(value))
            }
            ConstantInstantiationKind::String(value) => {
                ConstantValueKind::String(ConstantStringValue::Reference(value))
            }
            ConstantInstantiationKind::BitArray(value) => {
                ConstantValueKind::BitArray(ConstantBitArrayValue {
                    kind: ConstantBitArrayValueKind::Reference(value),
                })
            }
            ConstantInstantiationKind::Custom(value) => {
                ConstantValueKind::Custom(ConstantCustomValue {
                    shape: value.shape().clone(),
                    kind: ConstantCustomValueKind::Reference(value),
                })
            }
            ConstantInstantiationKind::Float(value) => {
                ConstantValueKind::Float(ConstantFloatValue::Reference(value))
            }
            ConstantInstantiationKind::Bool(value) => {
                ConstantValueKind::Bool(ConstantBoolValue::Reference(value))
            }
            ConstantInstantiationKind::Nil(value) => {
                ConstantValueKind::Nil(ConstantNilValue::Reference(value))
            }
            ConstantInstantiationKind::Tuple(value) => {
                ConstantValueKind::Tuple(ConstantTupleValue {
                    shape: value.shape().clone(),
                    kind: ConstantTupleValueKind::Reference(value),
                })
            }
            ConstantInstantiationKind::List(value) => {
                ConstantValueKind::List(ConstantListValue::reference(value))
            }
            ConstantInstantiationKind::Function(value) => {
                ConstantValueKind::Function(ConstantFunctionValue::reference(value))
            }
        };
        Self { kind }
    }

    pub(crate) fn shape(&self) -> ValueShape {
        match &self.kind {
            ConstantValueKind::Int(_) => ValueShape::Int,
            ConstantValueKind::Float(_) => ValueShape::Float,
            ConstantValueKind::String(_) => ValueShape::String,
            ConstantValueKind::Bool(_) => ValueShape::Bool,
            ConstantValueKind::Nil(_) => ValueShape::Nil,
            ConstantValueKind::Tuple(value) => ValueShape::Tuple(value.shape.clone()),
            ConstantValueKind::List(value) => value.shape(),
            ConstantValueKind::BitArray(_) => ValueShape::BitArray,
            ConstantValueKind::Custom(value) => ValueShape::Custom(value.shape.clone()),
            ConstantValueKind::Function(value) => {
                ValueShape::Function(Box::new(value.shape().clone()))
            }
        }
    }

    pub(crate) fn into_int(self) -> Option<ConstantIntValue> {
        match self.kind {
            ConstantValueKind::Int(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn into_float(self) -> Option<ConstantFloatValue> {
        match self.kind {
            ConstantValueKind::Float(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn into_string(self) -> Option<ConstantStringValue> {
        match self.kind {
            ConstantValueKind::String(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn into_bit_array(self) -> Option<ConstantBitArrayValue> {
        match self.kind {
            ConstantValueKind::BitArray(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn into_bool(self) -> Option<ConstantBoolValue> {
        match self.kind {
            ConstantValueKind::Bool(value) => Some(value),
            _ => None,
        }
    }

    pub(crate) fn into_nil(self) -> Option<ConstantNilValue> {
        match self.kind {
            ConstantValueKind::Nil(value) => Some(value),
            _ => None,
        }
    }

    fn into_list(self) -> Option<ConstantListValue> {
        match self.kind {
            ConstantValueKind::List(value) => Some(value),
            _ => None,
        }
    }
}

impl ConstantTupleValue {
    pub(crate) fn shape(&self) -> &[ValueShape] {
        &self.shape
    }

    pub(crate) fn kind(&self) -> &ConstantTupleValueKind {
        &self.kind
    }
}

impl ConstantBitArrayValue {
    pub(crate) fn kind(&self) -> &ConstantBitArrayValueKind {
        &self.kind
    }
}

impl ConstantCustomValue {
    pub(crate) fn shape(&self) -> &CustomValueShape {
        &self.shape
    }

    pub(crate) fn kind(&self) -> &ConstantCustomValueKind {
        &self.kind
    }
}

impl ConstantCustomConstruction {
    pub(crate) fn constructor(&self) -> &CustomConstructor {
        &self.constructor
    }

    pub(crate) fn fields(&self) -> &[ConstantValue] {
        &self.fields
    }
}

impl MaterializedConstantCustomConstruction {
    pub(crate) fn new(constructor: CustomConstructor, fields: Vec<super::Expr>) -> Self {
        Self {
            constructor,
            fields: fields.into_boxed_slice(),
        }
    }

    pub(crate) fn into_parts(self) -> (CustomConstructor, Box<[super::Expr]>) {
        (self.constructor, self.fields)
    }
}

fn constant_list_element_mismatch(
    expected: &ValueShape,
    actual: ValueShape,
) -> ConstantListConstructionError {
    ConstantListConstructionError::TypeMismatch {
        expected: expected.value_type(),
        actual: actual.value_type(),
    }
}

fn constant_list_tail_mismatch(
    expected: &ValueShape,
    actual: ValueShape,
) -> ConstantListConstructionError {
    ConstantListConstructionError::TypeMismatch {
        expected: ValueType::List(Box::new(expected.value_type())),
        actual: actual.value_type(),
    }
}

#[cfg(test)]
mod tests {
    use super::function::ConstantFunctionTemplate;
    use super::list::{ConstantListTemplate, ConstantParameterListListTemplateId};
    use super::{
        ConstantBitArraySegment, ConstantBitArrayTemplateId, ConstantBoolTemplateId,
        ConstantCustomTemplateId, ConstantFloatTemplateId, ConstantInstantiation,
        ConstantInstantiationKind, ConstantIntTemplateId, ConstantListConstructionError,
        ConstantListInstantiation, ConstantListValue, ConstantNilTemplateId,
        ConstantStringTemplateId, ConstantTemplate, ConstantTemplateId, ConstantTemplateSignature,
        ConstantTemplates, ConstantTupleTemplateId, ConstantValue,
        MaterializedConstantCustomConstruction, TypedConstantInstantiation,
    };
    use crate::plan::module::{CustomListExpr, CustomListItem, GenericListExpr, GenericListItem};
    use crate::plan::{
        BitArrayExpr, BitArrayListExpr, BitArrayListItem, BitArraySegment, BoolExpr, BoolListExpr,
        BoolListItem, CustomConstruction, CustomConstructor, CustomConstructorField,
        CustomConstructorRefinement, CustomExpr, CustomTypeName, CustomValueShape, Endianness,
        Expr, FloatBitSize, FloatExpr, FloatListExpr, FloatListItem, FunctionExpr,
        FunctionListExpr, FunctionListItem, FunctionReference, FunctionShape, IntExpr, ListExpr,
        ListListExpr, ListListItem, ModuleId, NilExpr, NilListExpr, NilListItem, PanicSite,
        ParameterListListExpr, ParameterListListItem, StoredListExpr, StringEncoding, StringExpr,
        StringListExpr, StringListItem, TupleExpr, TupleListExpr, TupleListItem, TypeParameterId,
        TypeScheme, TypeSubstitution, UtfCodepointListExpr, UtfCodepointListItem, ValueShape,
        ValueStorageShape, ValueType, monomorphic_function_instantiation,
    };

    #[test]
    fn local_constant_alias_materialization_preserves_foreign_owner() {
        let scheme = TypeScheme::new(0);
        let root_signature = ConstantTemplateSignature::int(
            ConstantTemplateId::in_module(ModuleId::root(), 0),
            0,
            scheme,
        );
        let foreign = TypedConstantInstantiation::in_module(
            ModuleId::new(1),
            ConstantIntTemplateId(0),
            TypeSubstitution::from_arguments(Vec::new()),
            (),
        );
        let local = TypedConstantInstantiation::in_module(
            ModuleId::root(),
            ConstantIntTemplateId(0),
            TypeSubstitution::from_arguments(Vec::new()),
            (),
        );
        let templates = ConstantTemplates::from_module_entries(
            ModuleId::root(),
            vec![(
                ConstantTemplate::new(root_signature, "alias".into()),
                ConstantValue::reference(ConstantInstantiation::from_int(foreign.clone())),
            )],
        );

        assert_eq!(
            templates.materialize_int(&local),
            IntExpr::constant(super::ConstantIntReference(foreign)),
        );
    }

    #[test]
    fn constant_references_materialize_every_top_level_family() {
        let monomorphic = TypeScheme::new(0);
        let int_base =
            ConstantTemplateSignature::int(ConstantTemplateId::new(0), 0, monomorphic.clone());
        let int_alias =
            ConstantTemplateSignature::int(ConstantTemplateId::new(1), 1, monomorphic.clone());
        let int_reference = int_base
            .try_instantiate(Vec::new())
            .expect("a monomorphic Int constant should instantiate without arguments");

        let string_base =
            ConstantTemplateSignature::string(ConstantTemplateId::new(2), 0, monomorphic.clone());
        let string_alias =
            ConstantTemplateSignature::string(ConstantTemplateId::new(3), 1, monomorphic.clone());
        let string_reference = string_base
            .try_instantiate(Vec::new())
            .expect("a monomorphic String constant should instantiate without arguments");

        let float_base =
            ConstantTemplateSignature::float(ConstantTemplateId::new(4), 0, monomorphic.clone());
        let float_alias =
            ConstantTemplateSignature::float(ConstantTemplateId::new(5), 1, monomorphic.clone());
        let float_reference = float_base
            .try_instantiate(Vec::new())
            .expect("a monomorphic Float constant should instantiate without arguments");

        let bool_base =
            ConstantTemplateSignature::bool(ConstantTemplateId::new(6), 0, monomorphic.clone());
        let bool_alias =
            ConstantTemplateSignature::bool(ConstantTemplateId::new(7), 1, monomorphic.clone());
        let bool_reference = bool_base
            .try_instantiate(Vec::new())
            .expect("a monomorphic Bool constant should instantiate without arguments");

        let nil_base =
            ConstantTemplateSignature::nil(ConstantTemplateId::new(8), 0, monomorphic.clone());
        let nil_alias =
            ConstantTemplateSignature::nil(ConstantTemplateId::new(9), 1, monomorphic.clone());
        let nil_reference = nil_base
            .try_instantiate(Vec::new())
            .expect("a monomorphic Nil constant should instantiate without arguments");

        let bit_array_base = ConstantTemplateSignature::bit_array(
            ConstantTemplateId::new(10),
            0,
            monomorphic.clone(),
        );
        let bit_array_alias = ConstantTemplateSignature::bit_array(
            ConstantTemplateId::new(11),
            1,
            monomorphic.clone(),
        );
        let bit_array_reference = bit_array_base
            .try_instantiate(Vec::new())
            .expect("a monomorphic BitArray constant should instantiate without arguments");

        let custom_shape = CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Token".into()),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        let constructor = CustomConstructor::new(
            custom_shape.type_().clone(),
            "Token".into(),
            0,
            vec![CustomConstructorField::new(None, ValueType::Int)],
        );
        let custom_base = ConstantTemplateSignature::custom(
            ConstantTemplateId::new(12),
            0,
            monomorphic.clone(),
            custom_shape.clone(),
        );
        let custom_alias = ConstantTemplateSignature::custom(
            ConstantTemplateId::new(13),
            1,
            monomorphic.clone(),
            custom_shape.clone(),
        );
        let custom_reference = custom_base
            .try_instantiate(Vec::new())
            .expect("a monomorphic custom constant should instantiate without arguments");

        let tuple_shape = vec![ValueShape::Int, ValueShape::String].into_boxed_slice();
        let tuple_base = ConstantTemplateSignature::tuple(
            ConstantTemplateId::new(14),
            0,
            monomorphic.clone(),
            tuple_shape.clone(),
        );
        let tuple_alias = ConstantTemplateSignature::tuple(
            ConstantTemplateId::new(15),
            1,
            monomorphic.clone(),
            tuple_shape.clone(),
        );
        let tuple_reference = tuple_base
            .try_instantiate(Vec::new())
            .expect("a monomorphic tuple constant should instantiate without arguments");

        let function_shape = FunctionShape::new(vec![ValueShape::Int], ValueShape::String);
        let function_reference = FunctionReference::new(monomorphic_function_instantiation(
            3,
            function_shape.clone(),
        ));
        let function_base = ConstantTemplateSignature::function(
            ConstantTemplateId::new(16),
            0,
            monomorphic.clone(),
            function_shape.clone(),
        );
        let function_alias = ConstantTemplateSignature::function(
            ConstantTemplateId::new(17),
            1,
            monomorphic.clone(),
            function_shape.clone(),
        );
        let function_reference_constant = function_base
            .try_instantiate(Vec::new())
            .expect("a monomorphic function constant should instantiate without arguments");

        let constructor_function_shape = FunctionShape::new(
            vec![ValueShape::Int],
            ValueShape::Custom(custom_shape.clone()),
        );
        let constructor_function = ConstantTemplateSignature::function(
            ConstantTemplateId::new(18),
            0,
            monomorphic,
            constructor_function_shape.clone(),
        );

        let empty_bits = ConstantValue::bit_array(Box::new([]));
        let empty_bits_value = empty_bits
            .clone()
            .into_bit_array()
            .expect("a BitArray constant should retain its family");
        let bit_array = ConstantValue::bit_array(
            vec![
                ConstantBitArraySegment::Int {
                    value: ConstantValue::reference(int_reference.clone())
                        .into_int()
                        .expect("an Int reference should retain its family"),
                    bit_size: 12,
                    endianness: Endianness::Little,
                },
                ConstantBitArraySegment::Float {
                    value: ConstantValue::reference(float_reference.clone())
                        .into_float()
                        .expect("a Float reference should retain its family"),
                    bit_size: FloatBitSize::Sixteen,
                    endianness: Endianness::Big,
                },
                ConstantBitArraySegment::String {
                    value: ConstantValue::reference(string_reference.clone())
                        .into_string()
                        .expect("a String reference should retain its family"),
                    encoding: StringEncoding::Utf16(Endianness::Little),
                },
                ConstantBitArraySegment::Bits(empty_bits_value.clone()),
                ConstantBitArraySegment::SizedBits {
                    value: empty_bits_value,
                    bit_size: 0,
                    site: PanicSite::unknown(),
                },
            ]
            .into_boxed_slice(),
        );

        let entries = vec![
            (
                ConstantTemplate::new(int_base.clone(), "int".into()),
                ConstantValue::int(7.into()),
            ),
            (
                ConstantTemplate::new(int_alias.clone(), "int_alias".into()),
                ConstantValue::reference(int_reference.clone()),
            ),
            (
                ConstantTemplate::new(string_base.clone(), "string".into()),
                ConstantValue::string_concatenation(
                    ConstantValue::string("ge".into())
                        .into_string()
                        .expect("a String constant should retain its family"),
                    ConstantValue::string("am".into())
                        .into_string()
                        .expect("a String constant should retain its family"),
                ),
            ),
            (
                ConstantTemplate::new(string_alias.clone(), "string_alias".into()),
                ConstantValue::reference(string_reference.clone()),
            ),
            (
                ConstantTemplate::new(float_base.clone(), "float".into()),
                ConstantValue::float(1.5),
            ),
            (
                ConstantTemplate::new(float_alias.clone(), "float_alias".into()),
                ConstantValue::reference(float_reference.clone()),
            ),
            (
                ConstantTemplate::new(bool_base.clone(), "bool".into()),
                ConstantValue::bool(true),
            ),
            (
                ConstantTemplate::new(bool_alias.clone(), "bool_alias".into()),
                ConstantValue::reference(bool_reference),
            ),
            (
                ConstantTemplate::new(nil_base.clone(), "nil".into()),
                ConstantValue::nil(),
            ),
            (
                ConstantTemplate::new(nil_alias.clone(), "nil_alias".into()),
                ConstantValue::reference(nil_reference),
            ),
            (
                ConstantTemplate::new(bit_array_base.clone(), "bits".into()),
                bit_array,
            ),
            (
                ConstantTemplate::new(bit_array_alias.clone(), "bits_alias".into()),
                ConstantValue::reference(bit_array_reference),
            ),
            (
                ConstantTemplate::new(custom_base.clone(), "token".into()),
                ConstantValue::custom(
                    custom_shape.clone(),
                    constructor.clone(),
                    vec![ConstantValue::reference(int_reference.clone())].into_boxed_slice(),
                ),
            ),
            (
                ConstantTemplate::new(custom_alias.clone(), "token_alias".into()),
                ConstantValue::reference(custom_reference),
            ),
            (
                ConstantTemplate::new(tuple_base.clone(), "pair".into()),
                ConstantValue::tuple(
                    tuple_shape,
                    vec![
                        ConstantValue::reference(int_reference),
                        ConstantValue::reference(string_reference),
                    ]
                    .into_boxed_slice(),
                ),
            ),
            (
                ConstantTemplate::new(tuple_alias.clone(), "pair_alias".into()),
                ConstantValue::reference(tuple_reference),
            ),
            (
                ConstantTemplate::new(function_base.clone(), "function".into()),
                ConstantValue::function(function_shape.clone(), function_reference.clone()),
            ),
            (
                ConstantTemplate::new(function_alias.clone(), "function_alias".into()),
                ConstantValue::reference(function_reference_constant.clone()),
            ),
            (
                ConstantTemplate::new(constructor_function.clone(), "constructor".into()),
                ConstantValue::constructor_function(
                    constructor_function_shape,
                    custom_shape.clone(),
                    constructor.clone(),
                ),
            ),
        ];
        let templates = ConstantTemplates::from_entries(entries);
        let empty = TypeSubstitution::from_arguments(Vec::new());

        assert_eq!(
            templates.materialize_value(&ConstantValue::float(2.5), &empty),
            Expr::float(FloatExpr::value(2.5)),
        );
        assert_eq!(
            templates.materialize_value(&ConstantValue::bool(false), &empty),
            Expr::bool(BoolExpr::value(false)),
        );
        assert_eq!(
            templates.materialize_value(&ConstantValue::nil(), &empty),
            Expr::nil(NilExpr::value()),
        );
        assert_eq!(
            templates.materialize_value(
                &ConstantValue::tuple(
                    vec![ValueShape::Int].into_boxed_slice(),
                    vec![ConstantValue::int(9.into())].into_boxed_slice(),
                ),
                &empty,
            ),
            Expr::tuple(TupleExpr::value(
                vec![Expr::int(IntExpr::value(9.into()))],
                vec![ValueType::Int],
            )),
        );
        assert_eq!(
            templates.materialize_value(
                &ConstantValue::custom(
                    custom_shape.clone(),
                    constructor.clone(),
                    vec![ConstantValue::int(9.into())].into_boxed_slice(),
                ),
                &empty,
            ),
            Expr::custom(CustomExpr::from_construction(
                custom_shape.clone(),
                CustomConstruction::from_constant(MaterializedConstantCustomConstruction::new(
                    constructor.clone(),
                    vec![Expr::int(IntExpr::value(9.into()))],
                )),
            )),
        );
        assert_eq!(
            templates.materialize_value(
                &ConstantValue::function(function_shape.clone(), function_reference.clone()),
                &empty,
            ),
            Expr::function(FunctionExpr::reference(function_reference.clone())),
        );

        assert_eq!(templates.headers().len(), 19);
        assert_eq!(
            templates.header(ConstantTemplateId::new(18)).name(),
            "constructor"
        );
        let int_alias_instantiation =
            TypedConstantInstantiation::new(ConstantIntTemplateId(1), empty.clone(), ());
        assert_eq!(
            int_alias.try_instantiate(Vec::new()),
            Some(ConstantInstantiation {
                kind: ConstantInstantiationKind::Int(int_alias_instantiation.clone()),
            }),
        );
        assert_eq!(
            templates.materialize_int(&int_alias_instantiation),
            IntExpr::value(7.into()),
        );
        let string_alias_instantiation =
            TypedConstantInstantiation::new(ConstantStringTemplateId(1), empty.clone(), ());
        assert_eq!(
            string_alias.try_instantiate(Vec::new()),
            Some(ConstantInstantiation {
                kind: ConstantInstantiationKind::String(string_alias_instantiation.clone()),
            }),
        );
        assert_eq!(
            templates.materialize_string(&string_alias_instantiation),
            StringExpr::concatenate(
                StringExpr::value("ge".into()),
                StringExpr::value("am".into()),
            ),
        );
        let float_alias_instantiation =
            TypedConstantInstantiation::new(ConstantFloatTemplateId(1), empty.clone(), ());
        assert_eq!(
            float_alias.try_instantiate(Vec::new()),
            Some(ConstantInstantiation {
                kind: ConstantInstantiationKind::Float(float_alias_instantiation.clone()),
            }),
        );
        assert_eq!(
            templates.materialize_float(&float_alias_instantiation),
            FloatExpr::value(1.5),
        );
        let bool_alias_instantiation =
            TypedConstantInstantiation::new(ConstantBoolTemplateId(1), empty.clone(), ());
        assert_eq!(
            bool_alias.try_instantiate(Vec::new()),
            Some(ConstantInstantiation {
                kind: ConstantInstantiationKind::Bool(bool_alias_instantiation.clone()),
            }),
        );
        assert_eq!(
            templates.materialize_bool(&bool_alias_instantiation),
            BoolExpr::value(true),
        );
        let nil_alias_instantiation =
            TypedConstantInstantiation::new(ConstantNilTemplateId(1), empty.clone(), ());
        assert_eq!(
            nil_alias.try_instantiate(Vec::new()),
            Some(ConstantInstantiation {
                kind: ConstantInstantiationKind::Nil(nil_alias_instantiation.clone()),
            }),
        );
        assert_eq!(
            templates.materialize_nil(&nil_alias_instantiation),
            NilExpr::value(),
        );
        let bit_array_alias_instantiation =
            TypedConstantInstantiation::new(ConstantBitArrayTemplateId(1), empty.clone(), ());
        assert_eq!(
            bit_array_alias.try_instantiate(Vec::new()),
            Some(ConstantInstantiation {
                kind: ConstantInstantiationKind::BitArray(bit_array_alias_instantiation.clone()),
            }),
        );
        assert_eq!(
            templates.materialize_bit_array(&bit_array_alias_instantiation),
            BitArrayExpr::value(vec![
                BitArraySegment::Int {
                    value: IntExpr::value(7.into()),
                    bit_size: 12,
                    endianness: Endianness::Little,
                },
                BitArraySegment::Float {
                    value: FloatExpr::value(1.5),
                    bit_size: FloatBitSize::Sixteen,
                    endianness: Endianness::Big,
                },
                BitArraySegment::String {
                    value: StringExpr::concatenate(
                        StringExpr::value("ge".into()),
                        StringExpr::value("am".into()),
                    ),
                    encoding: StringEncoding::Utf16(Endianness::Little),
                },
                BitArraySegment::Bits(BitArrayExpr::value(Vec::new())),
                BitArraySegment::SizedBits {
                    value: BitArrayExpr::value(Vec::new()),
                    size: crate::plan::BitArrayBitsSize::Fixed(0),
                    site: PanicSite::unknown(),
                },
            ]),
        );
        let custom_alias_instantiation = TypedConstantInstantiation::new(
            ConstantCustomTemplateId(1),
            empty.clone(),
            custom_shape.clone(),
        );
        assert_eq!(
            custom_alias.try_instantiate(Vec::new()),
            Some(ConstantInstantiation {
                kind: ConstantInstantiationKind::Custom(custom_alias_instantiation.clone()),
            }),
        );
        assert_eq!(
            templates.materialize_custom(&custom_alias_instantiation),
            CustomExpr::from_construction(
                custom_shape.clone(),
                CustomConstruction::from_constant(MaterializedConstantCustomConstruction::new(
                    constructor.clone(),
                    vec![Expr::int(IntExpr::value(7.into()))],
                )),
            ),
        );
        let tuple_alias_instantiation = TypedConstantInstantiation::new(
            ConstantTupleTemplateId(1),
            empty.clone(),
            vec![ValueShape::Int, ValueShape::String].into_boxed_slice(),
        );
        assert_eq!(
            tuple_alias.try_instantiate(Vec::new()),
            Some(ConstantInstantiation {
                kind: ConstantInstantiationKind::Tuple(tuple_alias_instantiation.clone()),
            }),
        );
        assert_eq!(
            templates.materialize_tuple(&tuple_alias_instantiation),
            TupleExpr::value(
                vec![
                    Expr::int(IntExpr::value(7.into())),
                    Expr::string(StringExpr::concatenate(
                        StringExpr::value("ge".into()),
                        StringExpr::value("am".into()),
                    )),
                ],
                vec![ValueType::Int, ValueType::String],
            ),
        );
        let function_alias_instantiation = ConstantFunctionTemplate::from_shape(&function_shape, 1)
            .instantiate(
                crate::plan::ModuleId::root(),
                empty.clone(),
                function_shape.clone(),
            );
        assert_eq!(
            function_alias.try_instantiate(Vec::new()),
            Some(ConstantInstantiation {
                kind: ConstantInstantiationKind::Function(function_alias_instantiation),
            }),
        );
        let constructor_function_shape =
            FunctionShape::new(vec![ValueShape::Int], ValueShape::Custom(custom_shape));
        let constructor_function_instantiation =
            ConstantFunctionTemplate::from_shape(&constructor_function_shape, 0).instantiate(
                crate::plan::ModuleId::root(),
                empty,
                constructor_function_shape,
            );
        assert_eq!(
            constructor_function.try_instantiate(Vec::new()),
            Some(ConstantInstantiation {
                kind: ConstantInstantiationKind::Function(constructor_function_instantiation),
            }),
        );
    }

    #[test]
    fn function_constant_targets_and_references_materialize_every_return_family() {
        let source = r#"
pub type Token { Token(Int) }

fn identity(value: value) { value }
fn int_value() { 1 }
fn float_value() { 1.5 }
fn string_value() { "one" }
fn bit_array_value() { <<1>> }
fn codepoint_value() {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}
fn custom_value() { Token(1) }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(1) }
fn list_value() { [1] }
fn function_value() { int_value }

const generic_function = identity
const generic_alias = generic_function
const int_function = int_value
const int_alias = int_function
const float_function = float_value
const float_alias = float_function
const string_function = string_value
const string_alias = string_function
const bit_array_function = bit_array_value
const bit_array_alias = bit_array_function
const codepoint_function = codepoint_value
const codepoint_alias = codepoint_function
const custom_function = custom_value
const custom_alias = custom_function
const bool_function = bool_value
const bool_alias = bool_function
const nil_function = nil_value
const nil_alias = nil_function
const tuple_function = tuple_value
const tuple_alias = tuple_function
const list_function = list_value
const list_alias = list_function
const function_function = function_value
const function_alias = function_function
const constructor_function = Token
const constructor_alias = constructor_function

pub fn main() {
  #(
    generic_alias == generic_alias,
    int_alias == int_alias,
    float_alias == float_alias,
    string_alias == string_alias,
    bit_array_alias == bit_array_alias,
    codepoint_alias == codepoint_alias,
    custom_alias == custom_alias,
    bool_alias == bool_alias,
    nil_alias == nil_alias,
    tuple_alias == tuple_alias,
    list_alias == list_alias,
    function_alias == function_alias,
    constructor_alias == constructor_alias,
  )
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("function constant fixture should compile");
        let module = crate::plan_module(typed).expect("function constant fixture should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module);
        let value = crate::run_main(&plan).expect("function constant fixture should execute");

        assert_eq!(
            value,
            crate::Value::Tuple(vec![
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(true),
                crate::Value::Bool(false),
            ]),
        );
    }

    #[test]
    fn generic_empty_list_references_materialize_every_item_shape() {
        let parameter = TypeParameterId(0);
        let base_signature = ConstantTemplateSignature::list(
            ConstantTemplateId::new(0),
            0,
            TypeScheme::new(1),
            ValueShape::Parameter(parameter),
        );
        let alias_signature = ConstantTemplateSignature::list(
            ConstantTemplateId::new(1),
            1,
            TypeScheme::new(1),
            ValueShape::Parameter(parameter),
        );
        let identity = base_signature
            .try_instantiate(vec![ValueShape::Parameter(parameter)])
            .expect("one parameter should match the generic constant scheme");
        let templates = ConstantTemplates::from_entries(vec![
            (
                ConstantTemplate::new(base_signature, "empty".into()),
                ConstantValue::try_list(ValueShape::Parameter(parameter), Vec::new(), None)
                    .expect("an uninhabited list constant may be empty"),
            ),
            (
                ConstantTemplate::new(alias_signature.clone(), "alias".into()),
                ConstantValue::reference(identity),
            ),
        ]);

        let custom_shape = CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Token".into()),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        let function_shape = FunctionShape::new(vec![ValueShape::Int], ValueShape::String);
        let item_shapes = vec![
            ValueShape::Parameter(parameter),
            ValueShape::Int,
            ValueShape::String,
            ValueShape::BitArray,
            ValueShape::UtfCodepoint,
            ValueShape::Custom(custom_shape),
            ValueShape::Float,
            ValueShape::Bool,
            ValueShape::Nil,
            ValueShape::Tuple(vec![ValueShape::Int, ValueShape::String].into_boxed_slice()),
            ValueShape::List(Box::new(ValueShape::Parameter(TypeParameterId(1)))),
            ValueShape::List(Box::new(ValueShape::Bool)),
            ValueShape::Function(Box::new(function_shape)),
        ];

        for item_shape in item_shapes {
            let arguments = vec![item_shape.clone()];
            let substitution = TypeSubstitution::from_arguments(arguments.clone());
            let instantiation =
                ConstantListTemplate::from_item_shape(ValueShape::Parameter(parameter), 1)
                    .instantiate(crate::plan::ModuleId::root(), substitution.clone());
            assert_eq!(
                alias_signature.try_instantiate(arguments),
                Some(ConstantInstantiation {
                    kind: ConstantInstantiationKind::List(instantiation.clone()),
                }),
            );
            let actual = match instantiation {
                ConstantListInstantiation::Generic(value) => {
                    ListExpr::Generic(templates.materialize_generic_list(&value))
                }
                ConstantListInstantiation::ParameterList(value) => {
                    ListExpr::ParameterList(templates.materialize_parameter_list_list(&value))
                }
                ConstantListInstantiation::Int(value) => {
                    ListExpr::Int(templates.materialize_int_list(&value))
                }
                ConstantListInstantiation::String(value) => {
                    ListExpr::String(templates.materialize_string_list(&value))
                }
                ConstantListInstantiation::BitArray(value) => {
                    ListExpr::BitArray(templates.materialize_bit_array_list(&value))
                }
                ConstantListInstantiation::UtfCodepoint(value) => {
                    ListExpr::UtfCodepoint(templates.materialize_utf_codepoint_list(&value))
                }
                ConstantListInstantiation::Custom(value) => {
                    ListExpr::Custom(templates.materialize_custom_list(&value))
                }
                ConstantListInstantiation::Float(value) => {
                    ListExpr::Float(templates.materialize_float_list(&value))
                }
                ConstantListInstantiation::Bool(value) => {
                    ListExpr::Bool(templates.materialize_bool_list(&value))
                }
                ConstantListInstantiation::Nil(value) => {
                    ListExpr::Nil(templates.materialize_nil_list(&value))
                }
                ConstantListInstantiation::Tuple(value) => {
                    ListExpr::Tuple(templates.materialize_tuple_list(&value))
                }
                ConstantListInstantiation::List(value) => {
                    ListExpr::List(templates.materialize_list_list(&value))
                }
                ConstantListInstantiation::Function(value) => {
                    ListExpr::Function(templates.materialize_function_list(&value))
                }
            };
            assert_eq!(
                templates.materialize_list_value(
                    &ConstantListValue::Generic(
                        templates
                            .generic_list(super::ConstantGenericListTemplateId(0))
                            .clone(),
                    ),
                    &substitution,
                ),
                actual.clone(),
            );
            assert_eq!(
                actual,
                ListExpr::value(Vec::new(), item_shape.value_type()).with_item_shape(item_shape),
            );
        }
    }

    #[test]
    fn nested_generic_list_values_materialize_parameter_and_stored_items() {
        let parameter = TypeParameterId(0);
        let nested_parameter = TypeParameterId(1);
        let templates = ConstantTemplates::from_entries(Vec::new());
        let generic = ConstantValue::try_list(ValueShape::Parameter(parameter), Vec::new(), None)
            .expect("a bare-parameter list constant may be empty")
            .into_list()
            .and_then(ConstantListValue::into_generic)
            .expect("the empty bare-parameter list should retain its generic storage");

        let unresolved_nested_shape =
            ValueShape::List(Box::new(ValueShape::Parameter(nested_parameter)));
        assert_eq!(
            templates.materialize_list_value(
                &ConstantListValue::Generic(generic.clone()),
                &TypeSubstitution::from_arguments(vec![unresolved_nested_shape.clone()]),
            ),
            ListExpr::value(Vec::new(), unresolved_nested_shape.value_type())
                .with_item_shape(unresolved_nested_shape),
        );
        assert_eq!(
            templates.materialize_generic_stored_list_value(
                &generic,
                &TypeSubstitution::from_arguments(Vec::new()),
                &ValueStorageShape::List(Box::new(ValueShape::Parameter(nested_parameter))),
            ),
            StoredListExpr::ParameterList(ParameterListListExpr::value(
                ParameterListListItem::new(nested_parameter),
                Vec::new(),
            )),
        );

        let nested = ConstantValue::try_list(
            ValueShape::List(Box::new(ValueShape::Parameter(parameter))),
            vec![
                ConstantValue::try_list(ValueShape::Parameter(parameter), Vec::new(), None)
                    .expect("a nested bare-parameter list element may be empty"),
            ],
            None,
        )
        .expect("a list may contain an empty bare-parameter list")
        .into_list()
        .expect("the nested constant should retain its list family");
        let nested_stored = nested
            .clone()
            .into_stored()
            .expect("the nested list should have stored outer-list representation");

        assert_eq!(
            templates
                .materialize_list_value(&nested, &TypeSubstitution::from_arguments(Vec::new()),),
            ListExpr::ParameterList(ParameterListListExpr::value(
                ParameterListListItem::new(parameter),
                vec![GenericListExpr::value(
                    GenericListItem::new(parameter),
                    Vec::new(),
                )],
            )),
        );
        assert_eq!(
            templates.materialize_stored_list_value(
                &nested_stored,
                &TypeSubstitution::from_arguments(Vec::new()),
            ),
            StoredListExpr::ParameterList(ParameterListListExpr::value(
                ParameterListListItem::new(parameter),
                vec![GenericListExpr::value(
                    GenericListItem::new(parameter),
                    Vec::new(),
                )],
            )),
        );
        assert_eq!(
            templates.materialize_list_value(
                &nested,
                &TypeSubstitution::from_arguments(vec![ValueShape::Int]),
            ),
            ListExpr::List(ListListExpr::value(
                ListListItem::new(ValueStorageShape::Int),
                vec![StoredListExpr::Int(crate::plan::IntListExpr::value(
                    crate::plan::IntListItem,
                    Vec::new(),
                ))],
            )),
        );
        assert_eq!(
            templates.materialize_stored_list_value(
                &nested_stored,
                &TypeSubstitution::from_arguments(vec![ValueShape::Int]),
            ),
            StoredListExpr::List(ListListExpr::value(
                ListListItem::new(ValueStorageShape::Int),
                vec![StoredListExpr::Int(crate::plan::IntListExpr::value(
                    crate::plan::IntListItem,
                    Vec::new(),
                ))],
            )),
        );

        let custom_shape = CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Token".into()),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        let tuple_shape = vec![ValueShape::Int, ValueShape::String].into_boxed_slice();
        let function_shape = FunctionShape::new(vec![ValueShape::Int], ValueShape::String);
        let stored = vec![
            (
                ValueStorageShape::Int,
                StoredListExpr::Int(crate::plan::IntListExpr::value(
                    crate::plan::IntListItem,
                    Vec::new(),
                )),
            ),
            (
                ValueStorageShape::Float,
                StoredListExpr::Float(FloatListExpr::value(FloatListItem, Vec::new())),
            ),
            (
                ValueStorageShape::String,
                StoredListExpr::String(StringListExpr::value(StringListItem, Vec::new())),
            ),
            (
                ValueStorageShape::BitArray,
                StoredListExpr::BitArray(BitArrayListExpr::value(BitArrayListItem, Vec::new())),
            ),
            (
                ValueStorageShape::UtfCodepoint,
                StoredListExpr::UtfCodepoint(UtfCodepointListExpr::value(
                    UtfCodepointListItem,
                    Vec::new(),
                )),
            ),
            (
                ValueStorageShape::Custom(custom_shape.clone()),
                StoredListExpr::Custom(
                    CustomListExpr::value(
                        CustomListItem::new(custom_shape.type_().clone()),
                        Vec::new(),
                    )
                    .with_item_shape(ValueShape::Custom(custom_shape.clone())),
                ),
            ),
            (
                ValueStorageShape::Bool,
                StoredListExpr::Bool(BoolListExpr::value(BoolListItem, Vec::new())),
            ),
            (
                ValueStorageShape::Nil,
                StoredListExpr::Nil(NilListExpr::value(NilListItem, Vec::new())),
            ),
            (
                ValueStorageShape::Tuple(tuple_shape.clone()),
                StoredListExpr::Tuple(TupleListExpr::value(
                    TupleListItem::new(tuple_shape.iter().map(ValueShape::value_type).collect()),
                    Vec::new(),
                )),
            ),
            (
                ValueStorageShape::List(Box::new(ValueShape::Bool)),
                StoredListExpr::List(ListListExpr::value(
                    ListListItem::new(ValueStorageShape::Bool),
                    Vec::new(),
                )),
            ),
            (
                ValueStorageShape::Function(Box::new(function_shape.clone())),
                StoredListExpr::Function(FunctionListExpr::value(
                    FunctionListItem::new(function_shape.type_()),
                    Vec::new(),
                )),
            ),
        ];
        for (shape, expected) in stored {
            let stored_value = ConstantValue::try_list(shape.to_value_shape(), Vec::new(), None)
                .expect("an empty list should accept every stored item family")
                .into_list()
                .expect("the empty constant should retain its list family")
                .into_stored()
                .expect("a stored item family should use stored list representation");
            assert_eq!(
                templates.materialize_generic_stored_list_value(
                    &generic,
                    &TypeSubstitution::from_arguments(Vec::new()),
                    &shape,
                ),
                expected.clone(),
            );
            assert_eq!(
                templates.materialize_stored_list_value(
                    &stored_value,
                    &TypeSubstitution::from_arguments(Vec::new()),
                ),
                expected,
            );
        }

        let deeply_nested = ConstantValue::try_list(
            ValueShape::List(Box::new(ValueShape::List(Box::new(ValueShape::Parameter(
                parameter,
            ))))),
            vec![
                ConstantValue::try_list(
                    ValueShape::List(Box::new(ValueShape::Parameter(parameter))),
                    vec![
                        ConstantValue::try_list(ValueShape::Parameter(parameter), Vec::new(), None)
                            .expect("a nested bare-parameter list element may be empty"),
                    ],
                    None,
                )
                .expect("an outer list may store a parameter-list payload"),
            ],
            None,
        )
        .expect("a deeply nested list may store an empty bare-parameter leaf");
        assert_eq!(
            deeply_nested.shape(),
            ValueShape::List(Box::new(ValueShape::List(Box::new(ValueShape::List(
                Box::new(ValueShape::Parameter(parameter)),
            ))))),
        );
        let deeply_nested = deeply_nested
            .into_list()
            .expect("the deeply nested constant should retain its list family");
        assert_eq!(
            templates.materialize_list_value(
                &deeply_nested,
                &TypeSubstitution::from_arguments(Vec::new()),
            ),
            ListExpr::List(ListListExpr::value(
                ListListItem::new(ValueStorageShape::List(Box::new(ValueShape::Parameter(
                    parameter,
                )))),
                vec![StoredListExpr::ParameterList(ParameterListListExpr::value(
                    ParameterListListItem::new(parameter),
                    vec![GenericListExpr::value(
                        GenericListItem::new(parameter),
                        Vec::new(),
                    )],
                ))],
            )),
        );
        assert_eq!(
            templates.materialize_list_value(
                &deeply_nested,
                &TypeSubstitution::from_arguments(vec![ValueShape::Int]),
            ),
            ListExpr::List(ListListExpr::value(
                ListListItem::new(ValueStorageShape::List(Box::new(ValueShape::Int))),
                vec![StoredListExpr::List(ListListExpr::value(
                    ListListItem::new(ValueStorageShape::Int),
                    vec![StoredListExpr::Int(crate::plan::IntListExpr::value(
                        crate::plan::IntListItem,
                        Vec::new(),
                    ))],
                ))],
            )),
        );
    }

    #[test]
    fn parameter_list_list_spread_reference_materializes_uninhabited_and_stored_items() {
        let parameter = TypeParameterId(0);
        let item_shape = ValueShape::List(Box::new(ValueShape::Parameter(parameter)));
        let generic_element = || {
            ConstantValue::try_list(ValueShape::Parameter(parameter), Vec::new(), None)
                .expect("a bare-parameter list element may be empty")
        };
        let tail = ConstantValue::try_list(item_shape.clone(), vec![generic_element()], None)
            .expect("a parameter-list tail should accept an empty generic list");
        let base = ConstantValue::try_list(item_shape.clone(), vec![generic_element()], Some(tail))
            .expect("a parameter-list spread should accept a matching tail");
        let base_signature = ConstantTemplateSignature::list(
            ConstantTemplateId::new(0),
            0,
            TypeScheme::new(1),
            item_shape.clone(),
        );
        let alias_signature = ConstantTemplateSignature::list(
            ConstantTemplateId::new(1),
            1,
            TypeScheme::new(1),
            item_shape,
        );
        let reference = base_signature
            .try_instantiate(vec![ValueShape::Parameter(parameter)])
            .expect("the parameter-list constant should accept its identity substitution");
        let templates = ConstantTemplates::from_entries(vec![
            (ConstantTemplate::new(base_signature, "base".into()), base),
            (
                ConstantTemplate::new(alias_signature, "alias".into()),
                ConstantValue::reference(reference),
            ),
        ]);
        let alias = ConstantListValue::ParameterList(
            templates
                .parameter_list_list(ConstantParameterListListTemplateId(1))
                .clone(),
        );
        let generic = GenericListExpr::value(GenericListItem::new(parameter), Vec::new());

        assert_eq!(
            templates
                .materialize_list_value(&alias, &TypeSubstitution::from_arguments(Vec::new()),),
            ListExpr::ParameterList(ParameterListListExpr::spread(
                vec1::vec1![generic.clone()],
                ParameterListListExpr::value(ParameterListListItem::new(parameter), vec![generic],),
            )),
        );

        let int_list = StoredListExpr::Int(crate::plan::IntListExpr::value(
            crate::plan::IntListItem,
            Vec::new(),
        ));
        assert_eq!(
            templates.materialize_list_value(
                &alias,
                &TypeSubstitution::from_arguments(vec![ValueShape::Int]),
            ),
            ListExpr::List(ListListExpr::spread(
                vec1::vec1![int_list.clone()],
                ListListExpr::value(ListListItem::new(ValueStorageShape::Int), vec![int_list],),
            )),
        );
    }

    #[test]
    fn exact_list_references_materialize_every_item_family() {
        let monomorphic = TypeScheme::new(0);
        let custom_shape = CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Token".into()),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        let constructor = CustomConstructor::new(
            custom_shape.type_().clone(),
            "Token".into(),
            0,
            vec![CustomConstructorField::new(None, ValueType::Int)],
        );
        let tuple_shape = vec![ValueShape::Int, ValueShape::String].into_boxed_slice();
        let function_shape = FunctionShape::new(vec![ValueShape::Int], ValueShape::String);
        let function_reference = FunctionReference::new(monomorphic_function_instantiation(
            5,
            function_shape.clone(),
        ));

        let item_shapes = vec![
            ValueShape::Int,
            ValueShape::String,
            ValueShape::BitArray,
            ValueShape::UtfCodepoint,
            ValueShape::Custom(custom_shape.clone()),
            ValueShape::Float,
            ValueShape::Bool,
            ValueShape::Nil,
            ValueShape::Tuple(tuple_shape.clone()),
            ValueShape::List(Box::new(ValueShape::Int)),
            ValueShape::Function(Box::new(function_shape.clone())),
        ];
        let signatures = item_shapes
            .iter()
            .enumerate()
            .flat_map(|(index, shape)| {
                [
                    ConstantTemplateSignature::list(
                        ConstantTemplateId::new(index * 2),
                        0,
                        monomorphic.clone(),
                        shape.clone(),
                    ),
                    ConstantTemplateSignature::list(
                        ConstantTemplateId::new(index * 2 + 1),
                        1,
                        monomorphic.clone(),
                        shape.clone(),
                    ),
                ]
            })
            .collect::<Vec<_>>();

        let int_tail =
            ConstantValue::try_list(ValueShape::Int, vec![ConstantValue::int(2.into())], None)
                .expect("an Int list tail should accept Int elements");
        let int_list = ConstantValue::try_list(
            ValueShape::Int,
            vec![ConstantValue::int(1.into())],
            Some(int_tail),
        )
        .expect("an Int list spread should accept an Int list tail");

        let string_tail = ConstantValue::try_list(
            ValueShape::String,
            vec![ConstantValue::string("two".into())],
            None,
        )
        .expect("a String list tail should accept String elements");
        let string_list = ConstantValue::try_list(
            ValueShape::String,
            vec![ConstantValue::string("one".into())],
            Some(string_tail),
        )
        .expect("a String list spread should accept a String list tail");

        let bit_array_tail = ConstantValue::try_list(
            ValueShape::BitArray,
            vec![ConstantValue::bit_array(Box::new([]))],
            None,
        )
        .expect("a BitArray list tail should accept BitArray elements");
        let bit_array_list = ConstantValue::try_list(
            ValueShape::BitArray,
            vec![ConstantValue::bit_array(Box::new([]))],
            Some(bit_array_tail),
        )
        .expect("a BitArray list spread should accept a BitArray list tail");

        let utf_codepoint_list =
            ConstantValue::try_list(ValueShape::UtfCodepoint, Vec::new(), None)
                .expect("a UtfCodepoint constant list may be empty");

        let token = |value: i64| {
            ConstantValue::custom(
                custom_shape.clone(),
                constructor.clone(),
                vec![ConstantValue::int(value.into())].into_boxed_slice(),
            )
        };
        let custom_tail = ConstantValue::try_list(
            ValueShape::Custom(custom_shape.clone()),
            vec![token(4)],
            None,
        )
        .expect("a custom list tail should accept matching custom elements");
        let custom_list = ConstantValue::try_list(
            ValueShape::Custom(custom_shape.clone()),
            vec![token(3)],
            Some(custom_tail),
        )
        .expect("a custom list spread should accept a matching custom list tail");

        let float_tail =
            ConstantValue::try_list(ValueShape::Float, vec![ConstantValue::float(2.5)], None)
                .expect("a Float list tail should accept Float elements");
        let float_list = ConstantValue::try_list(
            ValueShape::Float,
            vec![ConstantValue::float(1.5)],
            Some(float_tail),
        )
        .expect("a Float list spread should accept a Float list tail");

        let bool_tail =
            ConstantValue::try_list(ValueShape::Bool, vec![ConstantValue::bool(false)], None)
                .expect("a Bool list tail should accept Bool elements");
        let bool_list = ConstantValue::try_list(
            ValueShape::Bool,
            vec![ConstantValue::bool(true)],
            Some(bool_tail),
        )
        .expect("a Bool list spread should accept a Bool list tail");

        let nil_tail = ConstantValue::try_list(ValueShape::Nil, vec![ConstantValue::nil()], None)
            .expect("a Nil list tail should accept Nil elements");
        let nil_list =
            ConstantValue::try_list(ValueShape::Nil, vec![ConstantValue::nil()], Some(nil_tail))
                .expect("a Nil list spread should accept a Nil list tail");

        let pair = |number: i64, string: &str| {
            ConstantValue::tuple(
                tuple_shape.clone(),
                vec![
                    ConstantValue::int(number.into()),
                    ConstantValue::string(string.into()),
                ]
                .into_boxed_slice(),
            )
        };
        let tuple_tail = ConstantValue::try_list(
            ValueShape::Tuple(tuple_shape.clone()),
            vec![pair(2, "two")],
            None,
        )
        .expect("a tuple list tail should accept matching tuple elements");
        let tuple_list = ConstantValue::try_list(
            ValueShape::Tuple(tuple_shape.clone()),
            vec![pair(1, "one")],
            Some(tuple_tail),
        )
        .expect("a tuple list spread should accept a matching tuple list tail");

        let int_element = |value: i64| {
            ConstantValue::try_list(
                ValueShape::Int,
                vec![ConstantValue::int(value.into())],
                None,
            )
            .expect("a nested Int list should accept Int elements")
        };
        let list_tail = ConstantValue::try_list(
            ValueShape::List(Box::new(ValueShape::Int)),
            vec![int_element(2)],
            None,
        )
        .expect("a nested list tail should accept matching lists");
        let list_list = ConstantValue::try_list(
            ValueShape::List(Box::new(ValueShape::Int)),
            vec![int_element(1)],
            Some(list_tail),
        )
        .expect("a nested list spread should accept a matching nested list tail");

        let function_value =
            || ConstantValue::function(function_shape.clone(), function_reference.clone());
        let function_tail = ConstantValue::try_list(
            ValueShape::Function(Box::new(function_shape.clone())),
            vec![function_value()],
            None,
        )
        .expect("a function list tail should accept matching functions");
        let function_list = ConstantValue::try_list(
            ValueShape::Function(Box::new(function_shape.clone())),
            vec![function_value()],
            Some(function_tail),
        )
        .expect("a function list spread should accept a matching function list tail");

        let base_values = vec![
            int_list,
            string_list,
            bit_array_list,
            utf_codepoint_list,
            custom_list,
            float_list,
            bool_list,
            nil_list,
            tuple_list,
            list_list,
            function_list,
        ];
        let mut entries = Vec::with_capacity(base_values.len() * 2);
        let mut aliases = Vec::with_capacity(base_values.len());
        for (index, value) in base_values.into_iter().enumerate() {
            let base_signature = signatures[index * 2].clone();
            let alias_signature = signatures[index * 2 + 1].clone();
            let reference = base_signature
                .try_instantiate(Vec::new())
                .expect("a monomorphic list constant should instantiate without arguments");
            let alias = ConstantValue::reference(reference);
            entries.push((
                ConstantTemplate::new(base_signature, format!("base_{index}").into()),
                value,
            ));
            entries.push((
                ConstantTemplate::new(alias_signature, format!("alias_{index}").into()),
                alias.clone(),
            ));
            aliases.push(alias);
        }
        let templates = ConstantTemplates::from_entries(entries);

        let expected = vec![
            ListExpr::spread(
                vec![Expr::int(IntExpr::value(1.into()))],
                ListExpr::value(vec![Expr::int(IntExpr::value(2.into()))], ValueType::Int),
                ValueType::Int,
            ),
            ListExpr::spread(
                vec![Expr::string(StringExpr::value("one".into()))],
                ListExpr::value(
                    vec![Expr::string(StringExpr::value("two".into()))],
                    ValueType::String,
                ),
                ValueType::String,
            ),
            ListExpr::spread(
                vec![Expr::bit_array(BitArrayExpr::value(Vec::new()))],
                ListExpr::value(
                    vec![Expr::bit_array(BitArrayExpr::value(Vec::new()))],
                    ValueType::BitArray,
                ),
                ValueType::BitArray,
            ),
            ListExpr::value(Vec::new(), ValueType::UtfCodepoint),
            ListExpr::spread(
                vec![Expr::custom(
                    CustomExpr::try_constructor(
                        constructor.clone(),
                        vec![Expr::int(IntExpr::value(3.into()))],
                    )
                    .expect("the custom element should match its constructor"),
                )],
                ListExpr::value(
                    vec![Expr::custom(
                        CustomExpr::try_constructor(
                            constructor.clone(),
                            vec![Expr::int(IntExpr::value(4.into()))],
                        )
                        .expect("the custom tail element should match its constructor"),
                    )],
                    ValueType::Custom(custom_shape.type_().clone()),
                )
                .with_item_shape(ValueShape::Custom(custom_shape.clone())),
                ValueType::Custom(custom_shape.type_().clone()),
            )
            .with_item_shape(ValueShape::Custom(custom_shape)),
            ListExpr::spread(
                vec![Expr::float(FloatExpr::value(1.5))],
                ListExpr::value(vec![Expr::float(FloatExpr::value(2.5))], ValueType::Float),
                ValueType::Float,
            ),
            ListExpr::spread(
                vec![Expr::bool(BoolExpr::value(true))],
                ListExpr::value(vec![Expr::bool(BoolExpr::value(false))], ValueType::Bool),
                ValueType::Bool,
            ),
            ListExpr::spread(
                vec![Expr::nil(NilExpr::value())],
                ListExpr::value(vec![Expr::nil(NilExpr::value())], ValueType::Nil),
                ValueType::Nil,
            ),
            ListExpr::spread(
                vec![Expr::tuple(TupleExpr::value(
                    vec![
                        Expr::int(IntExpr::value(1.into())),
                        Expr::string(StringExpr::value("one".into())),
                    ],
                    vec![ValueType::Int, ValueType::String],
                ))],
                ListExpr::value(
                    vec![Expr::tuple(TupleExpr::value(
                        vec![
                            Expr::int(IntExpr::value(2.into())),
                            Expr::string(StringExpr::value("two".into())),
                        ],
                        vec![ValueType::Int, ValueType::String],
                    ))],
                    ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
                ),
                ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
            ),
            ListExpr::spread(
                vec![Expr::list(ListExpr::value(
                    vec![Expr::int(IntExpr::value(1.into()))],
                    ValueType::Int,
                ))],
                ListExpr::value(
                    vec![Expr::list(ListExpr::value(
                        vec![Expr::int(IntExpr::value(2.into()))],
                        ValueType::Int,
                    ))],
                    ValueType::List(Box::new(ValueType::Int)),
                ),
                ValueType::List(Box::new(ValueType::Int)),
            ),
            ListExpr::spread(
                vec![Expr::function(FunctionExpr::reference(
                    function_reference.clone(),
                ))],
                ListExpr::value(
                    vec![Expr::function(FunctionExpr::reference(
                        function_reference.clone(),
                    ))],
                    ValueType::Function(Box::new(function_shape.type_())),
                ),
                ValueType::Function(Box::new(function_shape.type_())),
            ),
        ];

        for (index, expected) in expected.into_iter().enumerate() {
            let instantiation =
                ConstantListTemplate::from_item_shape(item_shapes[index].clone(), 1).instantiate(
                    crate::plan::ModuleId::root(),
                    TypeSubstitution::from_arguments(Vec::new()),
                );
            assert_eq!(
                signatures[index * 2 + 1].try_instantiate(Vec::new()),
                Some(ConstantInstantiation {
                    kind: ConstantInstantiationKind::List(instantiation.clone()),
                }),
            );
            assert_eq!(
                templates.materialize_value(
                    &aliases[index],
                    &TypeSubstitution::from_arguments(Vec::new()),
                ),
                Expr::list(expected),
            );
        }
    }

    #[test]
    fn constant_template_instantiation_rejects_wrong_argument_count() {
        let signature = ConstantTemplateSignature::list(
            ConstantTemplateId::new(3),
            0,
            TypeScheme::new(1),
            ValueShape::Parameter(TypeParameterId(0)),
        );

        assert_eq!(signature.try_instantiate(Vec::new()), None);
        let instantiation = signature
            .try_instantiate(vec![ValueShape::String])
            .expect("one argument should match the constant scheme");
        assert_eq!(
            ConstantValue::reference(instantiation).shape(),
            ValueShape::List(Box::new(ValueShape::String)),
        );
    }

    #[test]
    fn constant_instantiations_preserve_every_family_through_substitution() {
        let parameter = TypeParameterId(0);
        let replacement = ValueShape::String;
        let substitution = TypeSubstitution::from_arguments(vec![replacement.clone()]);
        let custom_shape = CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            vec![ValueShape::Parameter(parameter)],
            CustomConstructorRefinement::Exact(0),
        );
        let tuple_shape = vec![ValueShape::Parameter(parameter)].into_boxed_slice();

        let int = TypedConstantInstantiation::new(
            ConstantIntTemplateId(0),
            TypeSubstitution::from_arguments(vec![ValueShape::Parameter(parameter)]),
            (),
        );
        let string = TypedConstantInstantiation::new(
            ConstantStringTemplateId(0),
            TypeSubstitution::from_arguments(vec![ValueShape::Parameter(parameter)]),
            (),
        );
        let float = TypedConstantInstantiation::new(
            ConstantFloatTemplateId(0),
            TypeSubstitution::from_arguments(vec![ValueShape::Parameter(parameter)]),
            (),
        );
        let bool_ = TypedConstantInstantiation::new(
            super::ConstantBoolTemplateId(0),
            TypeSubstitution::from_arguments(vec![ValueShape::Parameter(parameter)]),
            (),
        );
        let nil = TypedConstantInstantiation::new(
            ConstantNilTemplateId(0),
            TypeSubstitution::from_arguments(vec![ValueShape::Parameter(parameter)]),
            (),
        );
        let custom = TypedConstantInstantiation::new(
            ConstantCustomTemplateId(0),
            TypeSubstitution::from_arguments(vec![ValueShape::Parameter(parameter)]),
            custom_shape.clone(),
        );
        let tuple = TypedConstantInstantiation::new(
            ConstantTupleTemplateId(0),
            TypeSubstitution::from_arguments(vec![ValueShape::Parameter(parameter)]),
            tuple_shape,
        );

        let cases = [
            ConstantInstantiation::from_int(int.clone()),
            ConstantInstantiation::from_string(string.clone()),
            ConstantInstantiation::from_custom(custom.clone()),
            ConstantInstantiation::from_float(float.clone()),
            ConstantInstantiation::from_bool(bool_.clone()),
            ConstantInstantiation::from_nil(nil.clone()),
        ];
        let expected = [
            ConstantInstantiation::from_int(int.substitute_leaf(&substitution)),
            ConstantInstantiation::from_string(string.substitute_leaf(&substitution)),
            ConstantInstantiation::from_custom(custom.substitute_custom(&substitution)),
            ConstantInstantiation::from_float(float.substitute_leaf(&substitution)),
            ConstantInstantiation::from_bool(bool_.substitute_leaf(&substitution)),
            ConstantInstantiation::from_nil(nil.substitute_leaf(&substitution)),
        ];

        for (value, expected) in cases.iter().zip(expected) {
            assert_eq!(value.substitute(&substitution), expected);
        }
        assert_eq!(
            ConstantInstantiation::from_tuple(tuple.clone()).substitute(&substitution),
            ConstantInstantiation::from_tuple(tuple.substitute_tuple(&substitution)),
        );

        assert_eq!(
            super::ConstantIntReference(int.clone()).instantiation(),
            &int
        );
        assert_eq!(
            super::ConstantStringReference(string.clone()).instantiation(),
            &string,
        );
        assert_eq!(
            super::ConstantCustomReference(custom.clone()).instantiation(),
            &custom,
        );
        assert_eq!(
            super::ConstantFloatReference(float.clone()).instantiation(),
            &float,
        );
        assert_eq!(
            super::ConstantBoolReference(bool_.clone()).instantiation(),
            &bool_,
        );
        assert_eq!(
            super::ConstantNilReference(nil.clone()).instantiation(),
            &nil
        );

        assert_eq!(ConstantValue::string("not int".into()).into_int(), None);
        assert_eq!(ConstantValue::int(1.into()).into_float(), None);
        assert_eq!(ConstantValue::int(1.into()).into_string(), None);
        assert_eq!(ConstantValue::int(1.into()).into_bit_array(), None);
        assert_eq!(ConstantValue::int(1.into()).into_bool(), None);
        assert_eq!(ConstantValue::int(1.into()).into_nil(), None);
        assert_eq!(ConstantValue::int(1.into()).into_list(), None);
    }

    #[test]
    fn list_construction_rejects_uninhabited_and_mismatched_payloads() {
        let int_list =
            ConstantValue::try_list(ValueShape::Int, vec![ConstantValue::int(1.into())], None)
                .expect("an Int list should accept Int elements");
        let string_list = ConstantValue::try_list(
            ValueShape::String,
            vec![ConstantValue::string("one".into())],
            None,
        )
        .expect("a String list should accept String elements");
        let generic_list =
            ConstantValue::try_list(ValueShape::Parameter(TypeParameterId(0)), Vec::new(), None)
                .expect("a bare-parameter list may be empty");
        let utf_codepoint_list =
            ConstantValue::try_list(ValueShape::UtfCodepoint, Vec::new(), None)
                .expect("a UtfCodepoint list may be empty");

        assert_eq!(
            ConstantValue::try_list(ValueShape::Int, Vec::new(), Some(int_list.clone())),
            Err(ConstantListConstructionError::SpreadWithoutElements),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Parameter(TypeParameterId(0)),
                Vec::new(),
                Some(generic_list.clone()),
            ),
            Err(ConstantListConstructionError::SpreadWithoutElements),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::UtfCodepoint,
                Vec::new(),
                Some(utf_codepoint_list.clone()),
            ),
            Err(ConstantListConstructionError::SpreadWithoutElements),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Parameter(TypeParameterId(0)),
                vec![ConstantValue::int(1.into())],
                None,
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::Parameter(TypeParameterId(0)),
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::UtfCodepoint,
                vec![ConstantValue::int(1.into())],
                None,
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::UtfCodepoint,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Parameter(TypeParameterId(0)),
                vec![ConstantValue::int(1.into())],
                Some(generic_list.clone()),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::Parameter(TypeParameterId(0)),
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::UtfCodepoint,
                vec![ConstantValue::int(1.into())],
                Some(utf_codepoint_list),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::UtfCodepoint,
                actual: ValueType::Int,
            }),
        );

        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Int,
                vec![ConstantValue::string("one".into())],
                None,
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::Int,
                actual: ValueType::String,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(ValueShape::Float, vec![ConstantValue::int(1.into())], None,),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::Float,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(ValueShape::String, vec![ConstantValue::int(1.into())], None,),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::String,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::BitArray,
                vec![ConstantValue::int(1.into())],
                None,
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::BitArray,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(ValueShape::Bool, vec![ConstantValue::int(1.into())], None,),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::Bool,
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(ValueShape::Nil, vec![ConstantValue::int(1.into())], None,),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::Nil,
                actual: ValueType::Int,
            }),
        );

        let custom_shape = CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Token".into()),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        let constructor =
            CustomConstructor::new(custom_shape.type_().clone(), "Token".into(), 0, Vec::new());
        let custom = ConstantValue::custom(custom_shape.clone(), constructor.clone(), Box::new([]));
        let tuple = ConstantValue::tuple(
            vec![ValueShape::Int].into_boxed_slice(),
            vec![ConstantValue::int(1.into())].into_boxed_slice(),
        );
        let function_shape =
            FunctionShape::new(Vec::new(), ValueShape::Custom(custom_shape.clone()));
        let function = ConstantValue::constructor_function(
            function_shape.clone(),
            custom_shape.clone(),
            constructor,
        );

        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Custom(custom_shape.clone()),
                vec![ConstantValue::int(1.into())],
                None,
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::Custom(custom_shape.type_().clone()),
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
                vec![ConstantValue::int(1.into())],
                None,
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::Tuple(vec![ValueType::Int]),
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::List(Box::new(ValueShape::Int)),
                vec![ConstantValue::int(1.into())],
                None,
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::List(Box::new(ValueShape::Parameter(TypeParameterId(0)))),
                vec![int_list.clone()],
                None,
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0)))),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::List(Box::new(ValueShape::Parameter(TypeParameterId(0)))),
                vec![ConstantValue::int(1.into())],
                None,
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0)))),
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::List(Box::new(ValueShape::List(Box::new(ValueShape::Parameter(
                    TypeParameterId(0)
                ),)))),
                vec![int_list.clone()],
                Some(int_list.clone()),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::List(Box::new(
                    ValueType::Parameter(TypeParameterId(0)),
                )))),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::List(Box::new(ValueShape::Parameter(TypeParameterId(0)))),
                vec![
                    ConstantValue::try_list(
                        ValueShape::Parameter(TypeParameterId(0)),
                        Vec::new(),
                        None,
                    )
                    .expect("a bare-parameter list may be empty")
                ],
                Some(int_list.clone()),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::List(Box::new(
                    ValueType::Parameter(TypeParameterId(0)),
                )))),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::List(Box::new(ValueShape::Int)),
                vec![generic_list],
                None,
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0)))),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Function(Box::new(function_shape.clone())),
                vec![ConstantValue::int(1.into())],
                None,
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::Function(Box::new(function_shape.type_())),
                actual: ValueType::Int,
            }),
        );

        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Int,
                vec![ConstantValue::int(1.into())],
                Some(ConstantValue::int(2.into())),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::Int,
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Int,
                vec![ConstantValue::int(1.into())],
                Some(string_list.clone()),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Int)),
                actual: ValueType::List(Box::new(ValueType::String)),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::String,
                vec![ConstantValue::string("one".into())],
                Some(int_list.clone()),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::String)),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::BitArray,
                vec![ConstantValue::bit_array(Box::new([]))],
                Some(int_list.clone()),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::BitArray)),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Float,
                vec![ConstantValue::float(1.5)],
                Some(int_list.clone()),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Float)),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Bool,
                vec![ConstantValue::bool(true)],
                Some(int_list.clone()),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Bool)),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Nil,
                vec![ConstantValue::nil()],
                Some(int_list.clone()),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Nil)),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Custom(custom_shape.clone()),
                vec![custom],
                Some(int_list.clone()),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Custom(
                    custom_shape.type_().clone(),
                ))),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
                vec![tuple],
                Some(int_list.clone()),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::List(Box::new(ValueShape::Int)),
                vec![int_list.clone()],
                Some(int_list.clone()),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
        assert_eq!(
            ConstantValue::try_list(
                ValueShape::Function(Box::new(function_shape.clone())),
                vec![function],
                Some(int_list),
            ),
            Err(ConstantListConstructionError::TypeMismatch {
                expected: ValueType::List(Box::new(ValueType::Function(Box::new(
                    function_shape.type_(),
                )))),
                actual: ValueType::List(Box::new(ValueType::Int)),
            }),
        );
    }
}
