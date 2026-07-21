mod expression;
mod freeze;
mod instruction;
mod liveness;
mod pattern;
mod return_;
mod step;

pub(super) use expression::function::{
    bit_array_function_expr, bool_function_expr, custom_function_expr, custom_function_expr_kind,
    custom_never_function_expr, custom_never_function_expr_kind, float_function_expr,
    function_function_expr, function_function_expr_kind, generic_bit_array_function_expr,
    generic_bool_function_expr, generic_custom_function_expr, generic_float_function_expr,
    generic_function_expr, generic_function_function_expr, generic_int_function_expr,
    generic_list_function_expr, generic_never_function_expr, generic_nil_function_expr,
    generic_string_function_expr, generic_tuple_function_expr, generic_utf_codepoint_function_expr,
    int_function_expr, list_function_expr, nil_function_expr, string_function_expr,
    symbolic_bit_array_function_expr, symbolic_bool_function_expr,
    symbolic_custom_function_expr_kind, symbolic_float_function_expr,
    symbolic_function_function_expr_kind, symbolic_generic_function_expr,
    symbolic_int_function_expr, symbolic_list_function_expr, symbolic_nil_function_expr,
    symbolic_string_function_expr, symbolic_tuple_function_expr,
    symbolic_utf_codepoint_function_expr, tuple_function_expr, tuple_never_function_expr,
    utf_codepoint_function_expr,
};
pub(super) use expression::{
    bit_array_expr, bit_array_list_expr, bool_expr, bool_list_expr, custom_expr, custom_expr_kind,
    custom_list_expr, custom_never_expr_kind, float_expr, float_list_expr, function_list_expr,
    generic_expr, generic_list_expr, int_expr, int_list_expr, list_list_expr, never_expr, nil_expr,
    nil_list_expr, parameter_list_list_expr, string_expr, string_list_expr, tuple_expr,
    tuple_list_expr, tuple_never_expr, utf_codepoint_expr, utf_codepoint_list_expr,
};
pub(super) use freeze::FreezeGraphValue;
pub(super) use return_::{lower_constant_graph, lower_function_graph, lower_never_function_graph};

pub(super) use instruction::{
    DraftBitArrayBitsSize, DraftBitArrayEvaluatedSize, DraftBitArraySegment, DraftInstructionKind,
};
pub(super) use pattern::DraftMatchPattern;

use super::specialization::{
    SpecializedCustomValueShape, SpecializedFunctionShape, SpecializedValueShape, StoredValueShape,
};
use crate::plan::execution;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(super) struct DraftValueKey(usize);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct DraftBlockId(usize);

pub(super) enum DraftIntFamily {}
pub(super) enum DraftFloatFamily {}
pub(super) enum DraftStringFamily {}
pub(super) enum DraftBitArrayFamily {}
pub(super) enum DraftUtfCodepointFamily {}
pub(super) enum DraftCustomFamily {}
pub(super) enum DraftBoolFamily {}
pub(super) enum DraftNilFamily {}
pub(super) enum DraftTupleFamily {}
pub(super) enum DraftListFamily {}
pub(super) enum DraftFunctionFamily {}

pub(super) enum ParameterListFamily {}
pub(super) enum ParameterListListFamily {}
pub(super) enum IntListFamily {}
pub(super) enum StringListFamily {}
pub(super) enum BitArrayListFamily {}
pub(super) enum UtfCodepointListFamily {}
pub(super) enum CustomListFamily {}
pub(super) enum FloatListFamily {}
pub(super) enum BoolListFamily {}
pub(super) enum NilListFamily {}
pub(super) enum TupleListFamily {}
pub(super) enum ListListFamily {}
pub(super) enum FunctionListFamily {}

pub(super) enum GenericFunctionFamily {}
pub(super) enum NeverFunctionFamily {}
pub(super) enum IntFunctionFamily {}
pub(super) enum FloatFunctionFamily {}
pub(super) enum StringFunctionFamily {}
pub(super) enum BitArrayFunctionFamily {}
pub(super) enum UtfCodepointFunctionFamily {}
pub(super) enum CustomFunctionFamily {}
pub(super) enum BoolFunctionFamily {}
pub(super) enum NilFunctionFamily {}
pub(super) enum TupleFunctionFamily {}
pub(super) enum ListFunctionFamily {}
pub(super) enum FunctionFunctionFamily {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(super) struct DraftValue<Family> {
    key: DraftValueKey,
    shape: StoredValueShape,
    family: PhantomData<fn() -> Family>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct DraftValueRef {
    key: DraftValueKey,
    shape: StoredValueShape,
}

pub(super) type DraftInt = DraftValue<DraftIntFamily>;
pub(super) type DraftFloat = DraftValue<DraftFloatFamily>;
pub(super) type DraftString = DraftValue<DraftStringFamily>;
pub(super) type DraftBitArray = DraftValue<DraftBitArrayFamily>;
pub(super) type DraftUtfCodepoint = DraftValue<DraftUtfCodepointFamily>;
pub(super) type DraftCustom = DraftValue<DraftCustomFamily>;
pub(super) type DraftBool = DraftValue<DraftBoolFamily>;
pub(super) type DraftNil = DraftValue<DraftNilFamily>;
pub(super) type DraftTuple = DraftValue<DraftTupleFamily>;
pub(super) type DraftList = DraftValue<DraftListFamily>;
pub(super) type DraftFunction = DraftValue<DraftFunctionFamily>;

pub(super) enum DraftStoredList {
    ParameterList(DraftList),
    Int(DraftList),
    String(DraftList),
    BitArray(DraftList),
    UtfCodepoint(DraftList),
    Custom(DraftList),
    Float(DraftList),
    Bool(DraftList),
    Nil(DraftList),
    Tuple(DraftList),
    List(DraftList),
    Function(DraftList),
}

impl DraftStoredList {
    pub(super) fn into_list(self) -> DraftList {
        match self {
            Self::ParameterList(value)
            | Self::Int(value)
            | Self::String(value)
            | Self::BitArray(value)
            | Self::UtfCodepoint(value)
            | Self::Custom(value)
            | Self::Float(value)
            | Self::Bool(value)
            | Self::Nil(value)
            | Self::Tuple(value)
            | Self::List(value)
            | Self::Function(value) => value,
        }
    }
}

pub(super) struct DraftTypedList<Family> {
    value: DraftList,
    family: PhantomData<fn() -> Family>,
}

pub(super) struct DraftTypedFunction<Family> {
    value: DraftFunction,
    family: PhantomData<fn() -> Family>,
}

pub(super) type DraftParameterList = DraftTypedList<ParameterListFamily>;
pub(super) type DraftParameterListList = DraftTypedList<ParameterListListFamily>;
pub(super) type DraftIntList = DraftTypedList<IntListFamily>;
pub(super) type DraftStringList = DraftTypedList<StringListFamily>;
pub(super) type DraftBitArrayList = DraftTypedList<BitArrayListFamily>;
pub(super) type DraftUtfCodepointList = DraftTypedList<UtfCodepointListFamily>;
pub(super) type DraftCustomList = DraftTypedList<CustomListFamily>;
pub(super) type DraftFloatList = DraftTypedList<FloatListFamily>;
pub(super) type DraftBoolList = DraftTypedList<BoolListFamily>;
pub(super) type DraftNilList = DraftTypedList<NilListFamily>;
pub(super) type DraftTupleList = DraftTypedList<TupleListFamily>;
pub(super) type DraftListList = DraftTypedList<ListListFamily>;
pub(super) type DraftFunctionList = DraftTypedList<FunctionListFamily>;

pub(super) type DraftGenericFunction = DraftTypedFunction<GenericFunctionFamily>;
pub(super) type DraftNeverFunction = DraftTypedFunction<NeverFunctionFamily>;
pub(super) type DraftIntFunction = DraftTypedFunction<IntFunctionFamily>;
pub(super) type DraftFloatFunction = DraftTypedFunction<FloatFunctionFamily>;
pub(super) type DraftStringFunction = DraftTypedFunction<StringFunctionFamily>;
pub(super) type DraftBitArrayFunction = DraftTypedFunction<BitArrayFunctionFamily>;
pub(super) type DraftUtfCodepointFunction = DraftTypedFunction<UtfCodepointFunctionFamily>;
pub(super) type DraftCustomFunction = DraftTypedFunction<CustomFunctionFamily>;
pub(super) type DraftBoolFunction = DraftTypedFunction<BoolFunctionFamily>;
pub(super) type DraftNilFunction = DraftTypedFunction<NilFunctionFamily>;
pub(super) type DraftTupleFunction = DraftTypedFunction<TupleFunctionFamily>;
pub(super) type DraftListFunction = DraftTypedFunction<ListFunctionFamily>;
pub(super) type DraftFunctionFunction = DraftTypedFunction<FunctionFunctionFamily>;

pub(super) enum DraftNeverReturn {}

pub(super) trait DraftGraphValue {
    fn erase(&self) -> DraftValueRef;
}

pub(super) struct DraftCursor {
    id: DraftBlockId,
    explicit_params: Vec<DraftValueRef>,
    instructions: Vec<DraftInstruction>,
    scope: DraftScope,
}

#[derive(Clone, Default)]
pub(super) struct DraftScope {
    locals: HashMap<super::local::LocalKey, DraftValueRef>,
}

enum DraftInstruction {
    Int {
        output: DraftInt,
        kind: instruction::DraftIntInstruction,
    },
    Float {
        output: DraftFloat,
        kind: instruction::DraftFloatInstruction,
    },
    String {
        output: DraftString,
        kind: instruction::DraftStringInstruction,
    },
    BitArray {
        output: DraftBitArray,
        kind: instruction::DraftBitArrayInstruction,
    },
    UtfCodepoint {
        output: DraftUtfCodepoint,
        kind: instruction::DraftUtfCodepointInstruction,
    },
    Custom {
        output: DraftCustom,
        kind: instruction::DraftCustomInstruction,
    },
    Bool {
        output: DraftBool,
        kind: instruction::DraftBoolInstruction,
    },
    Nil {
        output: DraftNil,
        kind: instruction::DraftNilInstruction,
    },
    Tuple {
        output: DraftTuple,
        kind: instruction::DraftTupleInstruction,
    },
    List {
        output: DraftList,
        kind: instruction::DraftListInstruction,
    },
    Function {
        output: DraftFunction,
        shape: SpecializedFunctionShape,
        kind: instruction::DraftFunctionInstruction,
    },
}

struct DraftBlock {
    explicit_params: Vec<DraftValueRef>,
    instructions: Vec<DraftInstruction>,
    terminator: DraftTerminator,
}

enum DraftTerminator {
    Jump(DraftEdge),
    BoolBranch {
        subject: DraftBool,
        true_: DraftEdge,
        false_: DraftEdge,
    },
    IntSwitch {
        subject: DraftInt,
        clauses: Vec<(num_bigint::BigInt, DraftEdge)>,
        fallback: DraftEdge,
    },
    FloatSwitch {
        subject: DraftFloat,
        clauses: Vec<(f64, DraftEdge)>,
        fallback: DraftEdge,
    },
    StringSwitch {
        subject: DraftString,
        clauses: Vec<(ecow::EcoString, DraftEdge)>,
        fallback: DraftEdge,
    },
    Match {
        subject: DraftValueRef,
        pattern: DraftMatchPattern,
        success: DraftMatchEdge,
        failure: DraftEdge,
    },
    Return {
        value: DraftValueRef,
        index: usize,
    },
    TailCall {
        function: usize,
        args: Vec<DraftValueRef>,
    },
    SourceStop {
        kind: execution::graph::SourceStopKind,
        message: Option<DraftString>,
        site: crate::plan::PanicSite,
    },
    LetAssertPanic {
        subject: DraftValueRef,
        message: Option<DraftString>,
        site: crate::plan::PanicSite,
        pattern_span: crate::plan::SourceSpan,
    },
    NeverCall {
        function: DraftNeverCallTarget,
        args: Vec<DraftValueRef>,
    },
}

enum DraftNeverCallTarget {
    Direct(execution::NeverFunctionId),
    Value(DraftFunction),
}

struct DraftEdge {
    target: DraftBlockId,
    explicit_args: Vec<DraftValueRef>,
}

struct DraftMatchEdge {
    target: DraftBlockId,
    explicit_args: Vec<DraftMatchEdgeArgument>,
}

enum DraftMatchEdgeArgument {
    Binding(usize),
}

pub(super) enum DraftFlow<T> {
    Value { cursor: DraftCursor, value: T },
    Diverged,
}

pub(super) struct DraftGraphBuilder<Return, TailCall> {
    graph: DraftGraph,
    returns: Vec<Return>,
    tail_calls: Vec<TailCall>,
}

pub(super) struct DraftGraph {
    entry: DraftBlockId,
    parameter_count: usize,
    blocks: HashMap<DraftBlockId, DraftBlock>,
    next_value: usize,
    next_block: usize,
}

pub(super) struct LoweredFunctionGraph<Body> {
    pub(super) parameter_count: usize,
    pub(super) body: Body,
}

impl<Body> LoweredFunctionGraph<Body> {
    pub(super) fn map<Mapped>(
        self,
        map: impl FnOnce(Body) -> Mapped,
    ) -> LoweredFunctionGraph<Mapped> {
        LoweredFunctionGraph {
            parameter_count: self.parameter_count,
            body: map(self.body),
        }
    }
}

impl<Family> Clone for DraftValue<Family> {
    fn clone(&self) -> Self {
        Self {
            key: self.key,
            shape: self.shape.clone(),
            family: PhantomData,
        }
    }
}

impl<Family> Clone for DraftTypedList<Family> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<Family> Clone for DraftTypedFunction<Family> {
    fn clone(&self) -> Self {
        Self::new(self.value.clone())
    }
}

impl<Family> DraftValue<Family> {
    pub(super) fn shape(&self) -> &StoredValueShape {
        &self.shape
    }

    pub(super) fn erase(&self) -> DraftValueRef {
        DraftValueRef {
            key: self.key,
            shape: self.shape.clone(),
        }
    }

    fn from_ref(value: &DraftValueRef) -> Self {
        Self {
            key: value.key,
            shape: value.shape.clone(),
            family: PhantomData,
        }
    }

    pub(super) fn from_owned(value: DraftValueRef) -> Self {
        Self::from_ref(&value)
    }
}

impl<Family> DraftTypedList<Family> {
    pub(super) fn new(value: DraftList) -> Self {
        Self {
            value,
            family: PhantomData,
        }
    }

    pub(super) fn value(&self) -> &DraftList {
        &self.value
    }

    pub(super) fn from_ref(value: &DraftValueRef) -> Self {
        Self::new(DraftList::from_ref(value))
    }
}

impl<Family> DraftTypedFunction<Family> {
    pub(super) fn new(value: DraftFunction) -> Self {
        Self {
            value,
            family: PhantomData,
        }
    }

    pub(super) fn value(&self) -> &DraftFunction {
        &self.value
    }

    pub(super) fn from_ref(value: &DraftValueRef) -> Self {
        Self::new(DraftFunction::from_ref(value))
    }

    fn into_value(self) -> DraftFunction {
        self.value
    }
}

pub(super) trait DraftFunctionValue {
    fn into_function(self) -> DraftFunction;
}

impl DraftFunctionValue for DraftFunction {
    fn into_function(self) -> DraftFunction {
        self
    }
}

impl<Family> DraftFunctionValue for DraftTypedFunction<Family> {
    fn into_function(self) -> DraftFunction {
        self.into_value()
    }
}

impl<Family> DraftGraphValue for DraftValue<Family> {
    fn erase(&self) -> DraftValueRef {
        self.erase()
    }
}

impl<Family> DraftGraphValue for DraftTypedList<Family> {
    fn erase(&self) -> DraftValueRef {
        self.value.erase()
    }
}

impl<Family> DraftGraphValue for DraftTypedFunction<Family> {
    fn erase(&self) -> DraftValueRef {
        self.value.erase()
    }
}

impl DraftGraphValue for DraftNeverReturn {
    fn erase(&self) -> DraftValueRef {
        match *self {}
    }
}

impl DraftGraphValue for DraftValueRef {
    fn erase(&self) -> DraftValueRef {
        self.clone()
    }
}

impl DraftGraphValue for DraftStoredList {
    fn erase(&self) -> DraftValueRef {
        match self {
            Self::ParameterList(value)
            | Self::Int(value)
            | Self::String(value)
            | Self::BitArray(value)
            | Self::UtfCodepoint(value)
            | Self::Custom(value)
            | Self::Float(value)
            | Self::Bool(value)
            | Self::Nil(value)
            | Self::Tuple(value)
            | Self::List(value)
            | Self::Function(value) => value.erase(),
        }
    }
}

impl DraftValueRef {
    pub(super) fn shape(&self) -> &StoredValueShape {
        &self.shape
    }
}

impl DraftScope {
    pub(super) fn insert(&mut self, key: super::local::LocalKey, value: DraftValueRef) {
        self.locals.insert(key, value);
    }

    pub(super) fn get(&self, key: super::local::LocalKey) -> DraftValueRef {
        self.locals[&key].clone()
    }

    pub(super) fn int(&self, key: super::local::LocalKey) -> DraftInt {
        DraftInt::from_ref(&self.locals[&key])
    }

    pub(super) fn float(&self, key: super::local::LocalKey) -> DraftFloat {
        DraftFloat::from_ref(&self.locals[&key])
    }

    pub(super) fn string(&self, key: super::local::LocalKey) -> DraftString {
        DraftString::from_ref(&self.locals[&key])
    }

    pub(super) fn bit_array(&self, key: super::local::LocalKey) -> DraftBitArray {
        DraftBitArray::from_ref(&self.locals[&key])
    }

    pub(super) fn utf_codepoint(&self, key: super::local::LocalKey) -> DraftUtfCodepoint {
        DraftUtfCodepoint::from_ref(&self.locals[&key])
    }

    pub(super) fn custom(&self, key: super::local::LocalKey) -> DraftCustom {
        DraftCustom::from_ref(&self.locals[&key])
    }

    pub(super) fn bool(&self, key: super::local::LocalKey) -> DraftBool {
        DraftBool::from_ref(&self.locals[&key])
    }

    pub(super) fn nil(&self, key: super::local::LocalKey) -> DraftNil {
        DraftNil::from_ref(&self.locals[&key])
    }

    pub(super) fn tuple(&self, key: super::local::LocalKey) -> DraftTuple {
        DraftTuple::from_ref(&self.locals[&key])
    }

    pub(super) fn list(&self, key: super::local::LocalKey) -> DraftList {
        DraftList::from_ref(&self.locals[&key])
    }

    pub(super) fn function(&self, key: super::local::LocalKey) -> DraftFunction {
        DraftFunction::from_ref(&self.locals[&key])
    }
}

impl DraftCursor {
    pub(super) fn scope(&self) -> &DraftScope {
        &self.scope
    }

    pub(super) fn scope_mut(&mut self) -> &mut DraftScope {
        &mut self.scope
    }

    pub(super) fn id(&self) -> DraftBlockId {
        self.id
    }
}

impl<Return, TailCall> DraftGraphBuilder<Return, TailCall> {
    pub(super) fn new(
        params: Vec<(super::local::LocalKey, StoredValueShape)>,
        captures: Vec<(super::local::LocalKey, StoredValueShape)>,
    ) -> (Self, DraftCursor) {
        let (graph, cursor) = DraftGraph::new(params, captures);
        (
            Self {
                graph,
                returns: Vec::new(),
                tail_calls: Vec::new(),
            },
            cursor,
        )
    }

    pub(super) fn finish_return(&mut self, cursor: DraftCursor, value: Return)
    where
        Return: DraftGraphValue,
    {
        let index = self.returns.len();
        let erased = value.erase();
        self.returns.push(value);
        self.graph.finish(
            cursor,
            DraftTerminator::Return {
                value: erased,
                index,
            },
        );
    }

    pub(super) fn finish_tail_call(
        &mut self,
        cursor: DraftCursor,
        function: TailCall,
        args: Vec<DraftValueRef>,
    ) {
        let index = self.tail_calls.len();
        self.tail_calls.push(function);
        self.graph.finish(
            cursor,
            DraftTerminator::TailCall {
                function: index,
                args,
            },
        );
    }
}

impl<Return, TailCall> std::ops::Deref for DraftGraphBuilder<Return, TailCall> {
    type Target = DraftGraph;

    fn deref(&self) -> &Self::Target {
        &self.graph
    }
}

impl<Return, TailCall> std::ops::DerefMut for DraftGraphBuilder<Return, TailCall> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.graph
    }
}

impl<Return, TailCall> DraftGraphBuilder<Return, TailCall> {
    pub(super) fn graph(&self) -> &DraftGraph {
        &self.graph
    }
}

impl DraftGraph {
    fn new(
        params: Vec<(super::local::LocalKey, StoredValueShape)>,
        captures: Vec<(super::local::LocalKey, StoredValueShape)>,
    ) -> (Self, DraftCursor) {
        let parameter_count = params.len();
        let mut builder = Self {
            entry: DraftBlockId(0),
            parameter_count,
            blocks: HashMap::new(),
            next_value: 0,
            next_block: 1,
        };
        let mut scope = DraftScope::default();
        let mut explicit_params = Vec::with_capacity(params.len() + captures.len());
        for (key, shape) in params.into_iter().chain(captures) {
            let value = builder.value_ref(shape);
            scope.insert(key, value.clone());
            explicit_params.push(value);
        }
        let cursor = DraftCursor {
            id: builder.entry,
            explicit_params,
            instructions: Vec::new(),
            scope,
        };
        (builder, cursor)
    }

    fn value_ref(&mut self, shape: StoredValueShape) -> DraftValueRef {
        let value = DraftValueRef {
            key: DraftValueKey(self.next_value),
            shape,
        };
        self.next_value += 1;
        value
    }

    fn instruction<Family>(
        &mut self,
        cursor: &mut DraftCursor,
        shape: StoredValueShape,
        kind: DraftInstructionKind,
    ) -> DraftValue<Family> {
        let output = self.value_ref(shape);
        cursor
            .instructions
            .push(DraftInstruction::from_kind(output.clone(), kind));
        DraftValue::from_ref(&output)
    }

    pub(super) fn int_instruction(
        &mut self,
        cursor: &mut DraftCursor,
        kind: instruction::DraftIntInstruction,
    ) -> DraftInt {
        self.instruction(
            cursor,
            StoredValueShape::Int,
            DraftInstructionKind::Int(kind),
        )
    }

    pub(super) fn float_instruction(
        &mut self,
        cursor: &mut DraftCursor,
        kind: instruction::DraftFloatInstruction,
    ) -> DraftFloat {
        self.instruction(
            cursor,
            StoredValueShape::Float,
            DraftInstructionKind::Float(kind),
        )
    }

    pub(super) fn string_instruction(
        &mut self,
        cursor: &mut DraftCursor,
        kind: instruction::DraftStringInstruction,
    ) -> DraftString {
        self.instruction(
            cursor,
            StoredValueShape::String,
            DraftInstructionKind::String(kind),
        )
    }

    pub(super) fn bit_array_instruction(
        &mut self,
        cursor: &mut DraftCursor,
        kind: instruction::DraftBitArrayInstruction,
    ) -> DraftBitArray {
        self.instruction(
            cursor,
            StoredValueShape::BitArray,
            DraftInstructionKind::BitArray(kind),
        )
    }

    pub(super) fn utf_codepoint_instruction(
        &mut self,
        cursor: &mut DraftCursor,
        kind: instruction::DraftUtfCodepointInstruction,
    ) -> DraftUtfCodepoint {
        self.instruction(
            cursor,
            StoredValueShape::UtfCodepoint,
            DraftInstructionKind::UtfCodepoint(kind),
        )
    }

    pub(super) fn custom_instruction(
        &mut self,
        cursor: &mut DraftCursor,
        shape: SpecializedCustomValueShape,
        kind: instruction::DraftCustomInstruction,
    ) -> DraftCustom {
        self.instruction(
            cursor,
            StoredValueShape::Custom(shape),
            DraftInstructionKind::Custom(kind),
        )
    }

    pub(super) fn bool_instruction(
        &mut self,
        cursor: &mut DraftCursor,
        kind: instruction::DraftBoolInstruction,
    ) -> DraftBool {
        self.instruction(
            cursor,
            StoredValueShape::Bool,
            DraftInstructionKind::Bool(kind),
        )
    }

    pub(super) fn nil_instruction(
        &mut self,
        cursor: &mut DraftCursor,
        kind: instruction::DraftNilInstruction,
    ) -> DraftNil {
        self.instruction(
            cursor,
            StoredValueShape::Nil,
            DraftInstructionKind::Nil(kind),
        )
    }

    pub(super) fn tuple_instruction(
        &mut self,
        cursor: &mut DraftCursor,
        elements: Box<[SpecializedValueShape]>,
        kind: instruction::DraftTupleInstruction,
    ) -> DraftTuple {
        self.instruction(
            cursor,
            StoredValueShape::Tuple(elements),
            DraftInstructionKind::Tuple(kind),
        )
    }

    pub(super) fn list_instruction(
        &mut self,
        cursor: &mut DraftCursor,
        item: SpecializedValueShape,
        kind: instruction::DraftListInstruction,
    ) -> DraftList {
        self.instruction(
            cursor,
            StoredValueShape::List(Box::new(item)),
            DraftInstructionKind::List(kind),
        )
    }

    pub(super) fn function_instruction(
        &mut self,
        cursor: &mut DraftCursor,
        shape: SpecializedFunctionShape,
        kind: instruction::DraftFunctionInstruction,
    ) -> DraftFunction {
        self.instruction(
            cursor,
            StoredValueShape::Function(Box::new(shape.clone())),
            DraftInstructionKind::Function { shape, kind },
        )
    }

    pub(super) fn block(
        &mut self,
        scope: DraftScope,
        explicit_params: Vec<DraftValueRef>,
    ) -> DraftCursor {
        let id = DraftBlockId(self.next_block);
        self.next_block += 1;
        DraftCursor {
            id,
            explicit_params,
            instructions: Vec::new(),
            scope,
        }
    }

    pub(super) fn empty_block(&mut self, scope: DraftScope) -> DraftCursor {
        self.block(scope, Vec::new())
    }

    fn finish(&mut self, cursor: DraftCursor, terminator: DraftTerminator) {
        self.blocks.insert(
            cursor.id,
            DraftBlock {
                explicit_params: cursor.explicit_params,
                instructions: cursor.instructions,
                terminator,
            },
        );
    }

    pub(super) fn finish_jump(
        &mut self,
        cursor: DraftCursor,
        target: DraftBlockId,
        args: Vec<DraftValueRef>,
    ) {
        self.finish(
            cursor,
            DraftTerminator::Jump(DraftEdge {
                target,
                explicit_args: args,
            }),
        );
    }

    pub(super) fn finish_bool_branch(
        &mut self,
        cursor: DraftCursor,
        subject: DraftBool,
        true_: DraftBlockId,
        false_: DraftBlockId,
    ) {
        self.finish(
            cursor,
            DraftTerminator::BoolBranch {
                subject,
                true_: DraftEdge {
                    target: true_,
                    explicit_args: Vec::new(),
                },
                false_: DraftEdge {
                    target: false_,
                    explicit_args: Vec::new(),
                },
            },
        );
    }

    pub(super) fn finish_int_switch(
        &mut self,
        cursor: DraftCursor,
        subject: DraftInt,
        clauses: Vec<(num_bigint::BigInt, DraftBlockId)>,
        fallback: DraftBlockId,
    ) {
        self.finish(
            cursor,
            DraftTerminator::IntSwitch {
                subject,
                clauses: clauses
                    .into_iter()
                    .map(|(pattern, target)| {
                        (
                            pattern,
                            DraftEdge {
                                target,
                                explicit_args: Vec::new(),
                            },
                        )
                    })
                    .collect(),
                fallback: DraftEdge {
                    target: fallback,
                    explicit_args: Vec::new(),
                },
            },
        );
    }

    pub(super) fn finish_float_switch(
        &mut self,
        cursor: DraftCursor,
        subject: DraftFloat,
        clauses: Vec<(f64, DraftBlockId)>,
        fallback: DraftBlockId,
    ) {
        self.finish(
            cursor,
            DraftTerminator::FloatSwitch {
                subject,
                clauses: clauses
                    .into_iter()
                    .map(|(pattern, target)| {
                        (
                            pattern,
                            DraftEdge {
                                target,
                                explicit_args: Vec::new(),
                            },
                        )
                    })
                    .collect(),
                fallback: DraftEdge {
                    target: fallback,
                    explicit_args: Vec::new(),
                },
            },
        );
    }

    pub(super) fn finish_string_switch(
        &mut self,
        cursor: DraftCursor,
        subject: DraftString,
        clauses: Vec<(ecow::EcoString, DraftBlockId)>,
        fallback: DraftBlockId,
    ) {
        self.finish(
            cursor,
            DraftTerminator::StringSwitch {
                subject,
                clauses: clauses
                    .into_iter()
                    .map(|(pattern, target)| {
                        (
                            pattern,
                            DraftEdge {
                                target,
                                explicit_args: Vec::new(),
                            },
                        )
                    })
                    .collect(),
                fallback: DraftEdge {
                    target: fallback,
                    explicit_args: Vec::new(),
                },
            },
        );
    }

    pub(super) fn finish_match(
        &mut self,
        cursor: DraftCursor,
        subject: DraftValueRef,
        pattern: DraftMatchPattern,
        success: DraftBlockId,
        binding_count: usize,
        failure: DraftBlockId,
    ) {
        self.finish(
            cursor,
            DraftTerminator::Match {
                subject,
                pattern,
                success: DraftMatchEdge {
                    target: success,
                    explicit_args: (0..binding_count)
                        .map(DraftMatchEdgeArgument::Binding)
                        .collect(),
                },
                failure: DraftEdge {
                    target: failure,
                    explicit_args: Vec::new(),
                },
            },
        );
    }

    pub(super) fn finish_let_assert_panic(
        &mut self,
        cursor: DraftCursor,
        subject: DraftValueRef,
        message: Option<DraftString>,
        site: crate::plan::PanicSite,
        pattern_span: crate::plan::SourceSpan,
    ) {
        self.finish(
            cursor,
            DraftTerminator::LetAssertPanic {
                subject,
                message,
                site,
                pattern_span,
            },
        );
    }

    pub(super) fn finish_source_stop(
        &mut self,
        cursor: DraftCursor,
        kind: execution::graph::SourceStopKind,
        message: Option<DraftString>,
        site: crate::plan::PanicSite,
    ) {
        self.finish(
            cursor,
            DraftTerminator::SourceStop {
                kind,
                message,
                site,
            },
        );
    }

    pub(super) fn finish_never_call(
        &mut self,
        cursor: DraftCursor,
        function: execution::NeverFunctionId,
        args: Vec<DraftValueRef>,
    ) {
        self.finish(
            cursor,
            DraftTerminator::NeverCall {
                function: DraftNeverCallTarget::Direct(function),
                args,
            },
        );
    }

    pub(super) fn finish_never_function_call(
        &mut self,
        cursor: DraftCursor,
        function: DraftFunction,
        args: Vec<DraftValueRef>,
    ) {
        self.finish(
            cursor,
            DraftTerminator::NeverCall {
                function: DraftNeverCallTarget::Value(function),
                args,
            },
        );
    }
}

impl DraftInstruction {
    fn from_kind(output: DraftValueRef, kind: DraftInstructionKind) -> Self {
        match kind {
            DraftInstructionKind::Int(kind) => Self::Int {
                output: DraftInt::from_ref(&output),
                kind,
            },
            DraftInstructionKind::Float(kind) => Self::Float {
                output: DraftFloat::from_ref(&output),
                kind,
            },
            DraftInstructionKind::String(kind) => Self::String {
                output: DraftString::from_ref(&output),
                kind,
            },
            DraftInstructionKind::BitArray(kind) => Self::BitArray {
                output: DraftBitArray::from_ref(&output),
                kind,
            },
            DraftInstructionKind::UtfCodepoint(kind) => Self::UtfCodepoint {
                output: DraftUtfCodepoint::from_ref(&output),
                kind,
            },
            DraftInstructionKind::Custom(kind) => Self::Custom {
                output: DraftCustom::from_ref(&output),
                kind,
            },
            DraftInstructionKind::Bool(kind) => Self::Bool {
                output: DraftBool::from_ref(&output),
                kind,
            },
            DraftInstructionKind::Nil(kind) => Self::Nil {
                output: DraftNil::from_ref(&output),
                kind,
            },
            DraftInstructionKind::Tuple(kind) => Self::Tuple {
                output: DraftTuple::from_ref(&output),
                kind,
            },
            DraftInstructionKind::List(kind) => Self::List {
                output: DraftList::from_ref(&output),
                kind,
            },
            DraftInstructionKind::Function { shape, kind } => Self::Function {
                output: DraftFunction::from_ref(&output),
                shape,
                kind,
            },
        }
    }

    fn output(&self) -> DraftValueRef {
        match self {
            Self::Int { output, .. } => output.erase(),
            Self::Float { output, .. } => output.erase(),
            Self::String { output, .. } => output.erase(),
            Self::BitArray { output, .. } => output.erase(),
            Self::UtfCodepoint { output, .. } => output.erase(),
            Self::Custom { output, .. } => output.erase(),
            Self::Bool { output, .. } => output.erase(),
            Self::Nil { output, .. } => output.erase(),
            Self::Tuple { output, .. } => output.erase(),
            Self::List { output, .. } => output.erase(),
            Self::Function { output, .. } => output.erase(),
        }
    }

    fn uses(&self, values: &mut Vec<DraftValueRef>) {
        match self {
            Self::Int { kind, .. } => kind.uses(values),
            Self::Float { kind, .. } => kind.uses(values),
            Self::String { kind, .. } => kind.uses(values),
            Self::BitArray { kind, .. } => kind.uses(values),
            Self::UtfCodepoint { kind, .. } => kind.uses(values),
            Self::Custom { kind, .. } => kind.uses(values),
            Self::Bool { kind, .. } => kind.uses(values),
            Self::Nil { kind, .. } => kind.uses(values),
            Self::Tuple { kind, .. } => kind.uses(values),
            Self::List { kind, .. } => kind.uses(values),
            Self::Function { kind, .. } => kind.uses(values),
        }
    }
}

impl DraftTerminator {
    fn successors(&self) -> Vec<DraftBlockId> {
        match self {
            Self::Jump(edge) => vec![edge.target],
            Self::BoolBranch { true_, false_, .. } => vec![true_.target, false_.target],
            Self::IntSwitch {
                clauses, fallback, ..
            } => clauses
                .iter()
                .map(|(_, edge)| edge.target)
                .chain(std::iter::once(fallback.target))
                .collect(),
            Self::FloatSwitch {
                clauses, fallback, ..
            } => clauses
                .iter()
                .map(|(_, edge)| edge.target)
                .chain(std::iter::once(fallback.target))
                .collect(),
            Self::StringSwitch {
                clauses, fallback, ..
            } => clauses
                .iter()
                .map(|(_, edge)| edge.target)
                .chain(std::iter::once(fallback.target))
                .collect(),
            Self::Match {
                success, failure, ..
            } => vec![success.target, failure.target],
            Self::Return { .. }
            | Self::TailCall { .. }
            | Self::SourceStop { .. }
            | Self::LetAssertPanic { .. }
            | Self::NeverCall { .. } => Vec::new(),
        }
    }

    fn uses(&self, values: &mut Vec<DraftValueRef>) {
        match self {
            Self::Jump(edge) => edge.uses(values),
            Self::BoolBranch {
                subject,
                true_,
                false_,
            } => {
                values.push(subject.erase());
                true_.uses(values);
                false_.uses(values);
            }
            Self::IntSwitch {
                subject,
                clauses,
                fallback,
            } => {
                values.push(subject.erase());
                for (_, edge) in clauses {
                    edge.uses(values);
                }
                fallback.uses(values);
            }
            Self::FloatSwitch {
                subject,
                clauses,
                fallback,
            } => {
                values.push(subject.erase());
                for (_, edge) in clauses {
                    edge.uses(values);
                }
                fallback.uses(values);
            }
            Self::StringSwitch {
                subject,
                clauses,
                fallback,
            } => {
                values.push(subject.erase());
                for (_, edge) in clauses {
                    edge.uses(values);
                }
                fallback.uses(values);
            }
            Self::Match {
                subject,
                pattern,
                success,
                failure,
            } => {
                values.push(subject.clone());
                pattern.uses(values);
                success.uses(values);
                failure.uses(values);
            }
            Self::Return { value, index: _ } => values.push(value.clone()),
            Self::TailCall { args, .. } | Self::NeverCall { args, .. } => {
                values.extend(args.iter().cloned());
                if let Self::NeverCall {
                    function: DraftNeverCallTarget::Value(function),
                    ..
                } = self
                {
                    values.push(function.erase());
                }
            }
            Self::SourceStop { message, .. } => {
                if let Some(message) = message {
                    values.push(message.erase());
                }
            }
            Self::LetAssertPanic {
                subject, message, ..
            } => {
                values.push(subject.clone());
                if let Some(message) = message {
                    values.push(message.erase());
                }
            }
        }
    }
}

impl DraftEdge {
    fn uses(&self, values: &mut Vec<DraftValueRef>) {
        values.extend(self.explicit_args.iter().cloned());
    }
}

impl DraftMatchEdge {
    fn uses(&self, _values: &mut Vec<DraftValueRef>) {}
}

impl<T> DraftFlow<T> {
    pub(super) fn value(cursor: DraftCursor, value: T) -> Self {
        Self::Value { cursor, value }
    }

    pub(super) fn map<U>(self, map: impl FnOnce(T) -> U) -> DraftFlow<U> {
        match self {
            Self::Value { cursor, value } => DraftFlow::Value {
                cursor,
                value: map(value),
            },
            Self::Diverged => DraftFlow::Diverged,
        }
    }

    pub(super) fn map_cursor<U>(self, map: impl FnOnce(&mut DraftCursor, T) -> U) -> DraftFlow<U> {
        match self {
            Self::Value { mut cursor, value } => {
                let value = map(&mut cursor, value);
                DraftFlow::Value { cursor, value }
            }
            Self::Diverged => DraftFlow::Diverged,
        }
    }

    pub(super) fn and_then<U>(
        self,
        next: impl FnOnce(DraftCursor, T) -> super::specialization::Representability<DraftFlow<U>>,
    ) -> super::specialization::Representability<DraftFlow<U>> {
        match self {
            Self::Value { cursor, value } => next(cursor, value),
            Self::Diverged => {
                super::specialization::Representability::Inhabited(DraftFlow::Diverged)
            }
        }
    }

    pub(super) fn fold<U>(self, diverged: U, value: impl FnOnce(DraftCursor, T) -> U) -> U {
        match self {
            Self::Value {
                cursor,
                value: inner,
            } => value(cursor, inner),
            Self::Diverged => diverged,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        DraftFlow, DraftFunction, DraftGraphBuilder, DraftIntFunction, DraftIntList, DraftList,
        DraftStoredList, DraftValueKey, DraftValueRef,
    };
    use crate::plan::execution::lowering::specialization::{
        SpecializedValueShape, StoredValueShape,
    };
    use crate::plan::{FunctionShape, ValueShape};
    use std::marker::PhantomData;

    #[test]
    fn draft_flow_cursor_mapping_updates_values_and_preserves_divergence() {
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let mapped = DraftFlow::value(cursor, 1).map_cursor(|cursor, value| {
            *cursor = graph.empty_block(Default::default());
            increment_mapped_value(cursor, value)
        });

        assert_eq!(
            flow_value(mapped).map(|(cursor, value)| (cursor.id(), value)),
            Some((super::DraftBlockId(1), 2)),
        );
        assert!(
            flow_value(DraftFlow::<i32>::Diverged.map_cursor(increment_mapped_value)).is_none()
        );
    }

    #[test]
    fn draft_flow_continuation_preserves_divergence_and_representability() {
        use crate::plan::execution::lowering::specialization::Representability;

        let (_, cursor) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let continued = DraftFlow::value(cursor, 2).and_then(increment_flow);
        assert_eq!(flow_outcome(continued), FlowOutcome::Value(3));

        let (_, cursor) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let uninhabited = DraftFlow::value(cursor, 2)
            .and_then(|_, _| Representability::<DraftFlow<i32>>::Uninhabited);
        assert_eq!(flow_outcome(uninhabited), FlowOutcome::Uninhabited);

        let diverged = DraftFlow::<i32>::Diverged.and_then(increment_flow);
        assert_eq!(flow_outcome(diverged), FlowOutcome::Diverged);
    }

    #[test]
    fn draft_flow_fold_interprets_values_and_divergence_explicitly() {
        let (_, cursor) = DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());

        assert_eq!(
            DraftFlow::value(cursor, 4).fold(FlowOutcome::Diverged, increment_outcome),
            FlowOutcome::Value(5),
        );
        assert_eq!(
            DraftFlow::<i32>::Diverged.fold(FlowOutcome::Diverged, increment_outcome),
            FlowOutcome::Diverged,
        );
    }

    fn increment_flow(
        cursor: super::DraftCursor,
        value: i32,
    ) -> crate::plan::execution::lowering::specialization::Representability<DraftFlow<i32>> {
        crate::plan::execution::lowering::specialization::Representability::Inhabited(
            DraftFlow::value(cursor, value + 1),
        )
    }

    fn increment_outcome(_: super::DraftCursor, value: i32) -> FlowOutcome<i32> {
        FlowOutcome::Value(value + 1)
    }

    fn increment_mapped_value(_: &mut super::DraftCursor, value: i32) -> i32 {
        value + 1
    }

    fn flow_value<T>(flow: DraftFlow<T>) -> Option<(super::DraftCursor, T)> {
        match flow {
            DraftFlow::Value { cursor, value } => Some((cursor, value)),
            DraftFlow::Diverged => None,
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    enum FlowOutcome<T> {
        Uninhabited,
        Diverged,
        Value(T),
    }

    fn flow_outcome<T>(
        flow: crate::plan::execution::lowering::specialization::Representability<DraftFlow<T>>,
    ) -> FlowOutcome<T> {
        match flow {
            crate::plan::execution::lowering::specialization::Representability::Inhabited(
                DraftFlow::Value { value, .. },
            ) => FlowOutcome::Value(value),
            crate::plan::execution::lowering::specialization::Representability::Inhabited(
                DraftFlow::Diverged,
            ) => FlowOutcome::Diverged,
            crate::plan::execution::lowering::specialization::Representability::Uninhabited => {
                FlowOutcome::Uninhabited
            }
        }
    }

    #[test]
    fn stored_list_facades_preserve_one_draft_value() {
        let list = DraftList {
            key: DraftValueKey(3),
            shape: StoredValueShape::List(Box::new(SpecializedValueShape::Int)),
            family: PhantomData,
        };
        let expected = list.erase();

        for stored in [
            DraftStoredList::ParameterList(list.clone()),
            DraftStoredList::Int(list.clone()),
            DraftStoredList::String(list.clone()),
            DraftStoredList::BitArray(list.clone()),
            DraftStoredList::UtfCodepoint(list.clone()),
            DraftStoredList::Custom(list.clone()),
            DraftStoredList::Float(list.clone()),
            DraftStoredList::Bool(list.clone()),
            DraftStoredList::Nil(list.clone()),
            DraftStoredList::Tuple(list.clone()),
            DraftStoredList::List(list.clone()),
            DraftStoredList::Function(list),
        ] {
            assert_eq!(stored.into_list().erase(), expected);
        }
    }

    #[test]
    fn typed_list_and_function_clones_preserve_draft_identity() {
        let list = DraftIntList::new(DraftList {
            key: DraftValueKey(4),
            shape: StoredValueShape::List(Box::new(SpecializedValueShape::Int)),
            family: PhantomData,
        });
        assert_eq!(list.clone().value().erase(), list.value().erase());

        let context = crate::plan::execution::lowering::test_support::lowering_context(Vec::new());
        let function_shape =
            context.concrete_function_shape(&FunctionShape::new(Vec::new(), ValueShape::Int));
        let function = DraftIntFunction::new(DraftFunction {
            key: DraftValueKey(5),
            shape: StoredValueShape::Function(Box::new(function_shape)),
            family: PhantomData,
        });
        assert_eq!(function.clone().value().erase(), function.value().erase());
    }
}
