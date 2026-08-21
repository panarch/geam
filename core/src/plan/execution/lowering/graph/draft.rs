pub(in crate::plan::execution::lowering::graph) mod instruction;
pub(in crate::plan::execution::lowering::graph) mod pattern;
use instruction::DraftInstructionKind;
use pattern::DraftMatchPattern;

use super::super::specialization::{
    SpecializedCustomValueShape, SpecializedExternalValueShape, SpecializedFunctionShape,
    SpecializedValueShape, StoredValueShape,
};
use crate::plan::execution;
use std::collections::HashMap;
use std::marker::PhantomData;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub(in crate::plan::execution::lowering) struct DraftValueKey(
    pub(in crate::plan::execution::lowering::graph) usize,
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub(in crate::plan::execution::lowering) struct DraftBlockId(
    pub(in crate::plan::execution::lowering::graph) usize,
);

pub(in crate::plan::execution::lowering) enum DraftIntFamily {}
pub(in crate::plan::execution::lowering) enum DraftFloatFamily {}
pub(in crate::plan::execution::lowering) enum DraftStringFamily {}
pub(in crate::plan::execution::lowering) enum DraftBitArrayFamily {}
pub(in crate::plan::execution::lowering) enum DraftUtfCodepointFamily {}
pub(in crate::plan::execution::lowering) enum DraftCustomFamily {}
pub(in crate::plan::execution::lowering) enum DraftExternalFamily {}
pub(in crate::plan::execution::lowering) enum DraftBoolFamily {}
pub(in crate::plan::execution::lowering) enum DraftNilFamily {}
pub(in crate::plan::execution::lowering) enum DraftTupleFamily {}
pub(in crate::plan::execution::lowering) enum DraftListFamily {}
pub(in crate::plan::execution::lowering) enum DraftFunctionFamily {}

pub(in crate::plan::execution::lowering) enum ParameterListFamily {}
pub(in crate::plan::execution::lowering) enum ParameterListListFamily {}
pub(in crate::plan::execution::lowering) enum IntListFamily {}
pub(in crate::plan::execution::lowering) enum StringListFamily {}
pub(in crate::plan::execution::lowering) enum BitArrayListFamily {}
pub(in crate::plan::execution::lowering) enum UtfCodepointListFamily {}
pub(in crate::plan::execution::lowering) enum CustomListFamily {}
pub(in crate::plan::execution::lowering) enum ExternalListFamily {}
pub(in crate::plan::execution::lowering) enum FloatListFamily {}
pub(in crate::plan::execution::lowering) enum BoolListFamily {}
pub(in crate::plan::execution::lowering) enum NilListFamily {}
pub(in crate::plan::execution::lowering) enum TupleListFamily {}
pub(in crate::plan::execution::lowering) enum ListListFamily {}
pub(in crate::plan::execution::lowering) enum FunctionListFamily {}

pub(in crate::plan::execution::lowering) enum GenericFunctionFamily {}
pub(in crate::plan::execution::lowering) enum NeverFunctionFamily {}
pub(in crate::plan::execution::lowering) enum IntFunctionFamily {}
pub(in crate::plan::execution::lowering) enum FloatFunctionFamily {}
pub(in crate::plan::execution::lowering) enum StringFunctionFamily {}
pub(in crate::plan::execution::lowering) enum BitArrayFunctionFamily {}
pub(in crate::plan::execution::lowering) enum UtfCodepointFunctionFamily {}
pub(in crate::plan::execution::lowering) enum CustomFunctionFamily {}
pub(in crate::plan::execution::lowering) enum ExternalFunctionFamily {}
pub(in crate::plan::execution::lowering) enum BoolFunctionFamily {}
pub(in crate::plan::execution::lowering) enum NilFunctionFamily {}
pub(in crate::plan::execution::lowering) enum TupleFunctionFamily {}
pub(in crate::plan::execution::lowering) enum ListFunctionFamily {}
pub(in crate::plan::execution::lowering) enum FunctionFunctionFamily {}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(in crate::plan::execution::lowering) struct DraftValue<Family> {
    pub(in crate::plan::execution::lowering::graph) key: DraftValueKey,
    shape: StoredValueShape,
    family: PhantomData<fn() -> Family>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(in crate::plan::execution::lowering) struct DraftValueRef {
    pub(in crate::plan::execution::lowering::graph) key: DraftValueKey,
    shape: StoredValueShape,
}

pub(in crate::plan::execution::lowering) type DraftInt = DraftValue<DraftIntFamily>;
pub(in crate::plan::execution::lowering) type DraftFloat = DraftValue<DraftFloatFamily>;
pub(in crate::plan::execution::lowering) type DraftString = DraftValue<DraftStringFamily>;
pub(in crate::plan::execution::lowering) type DraftBitArray = DraftValue<DraftBitArrayFamily>;
pub(in crate::plan::execution::lowering) type DraftUtfCodepoint =
    DraftValue<DraftUtfCodepointFamily>;
pub(in crate::plan::execution::lowering) type DraftCustom = DraftValue<DraftCustomFamily>;
pub(in crate::plan::execution::lowering) type DraftExternal = DraftValue<DraftExternalFamily>;
pub(in crate::plan::execution::lowering) type DraftBool = DraftValue<DraftBoolFamily>;
pub(in crate::plan::execution::lowering) type DraftNil = DraftValue<DraftNilFamily>;
pub(in crate::plan::execution::lowering) type DraftTuple = DraftValue<DraftTupleFamily>;
pub(in crate::plan::execution::lowering) type DraftList = DraftValue<DraftListFamily>;
pub(in crate::plan::execution::lowering) type DraftFunction = DraftValue<DraftFunctionFamily>;

pub(in crate::plan::execution::lowering) enum DraftStoredList {
    ParameterList(DraftList),
    Int(DraftList),
    String(DraftList),
    BitArray(DraftList),
    UtfCodepoint(DraftList),
    Custom(DraftList),
    External(DraftList),
    Float(DraftList),
    Bool(DraftList),
    Nil(DraftList),
    Tuple(DraftList),
    List(DraftList),
    Function(DraftList),
}

impl DraftStoredList {
    pub(in crate::plan::execution::lowering) fn into_list(self) -> DraftList {
        match self {
            Self::ParameterList(value)
            | Self::Int(value)
            | Self::String(value)
            | Self::BitArray(value)
            | Self::UtfCodepoint(value)
            | Self::Custom(value)
            | Self::External(value)
            | Self::Float(value)
            | Self::Bool(value)
            | Self::Nil(value)
            | Self::Tuple(value)
            | Self::List(value)
            | Self::Function(value) => value,
        }
    }
}

pub(in crate::plan::execution::lowering) struct DraftTypedList<Family> {
    value: DraftList,
    family: PhantomData<fn() -> Family>,
}

pub(in crate::plan::execution::lowering) struct DraftTypedFunction<Family> {
    value: DraftFunction,
    family: PhantomData<fn() -> Family>,
}

pub(in crate::plan::execution::lowering) type DraftParameterList =
    DraftTypedList<ParameterListFamily>;
pub(in crate::plan::execution::lowering) type DraftParameterListList =
    DraftTypedList<ParameterListListFamily>;
pub(in crate::plan::execution::lowering) type DraftIntList = DraftTypedList<IntListFamily>;
pub(in crate::plan::execution::lowering) type DraftStringList = DraftTypedList<StringListFamily>;
pub(in crate::plan::execution::lowering) type DraftBitArrayList =
    DraftTypedList<BitArrayListFamily>;
pub(in crate::plan::execution::lowering) type DraftUtfCodepointList =
    DraftTypedList<UtfCodepointListFamily>;
pub(in crate::plan::execution::lowering) type DraftCustomList = DraftTypedList<CustomListFamily>;
pub(in crate::plan::execution::lowering) type DraftExternalList =
    DraftTypedList<ExternalListFamily>;
pub(in crate::plan::execution::lowering) type DraftFloatList = DraftTypedList<FloatListFamily>;
pub(in crate::plan::execution::lowering) type DraftBoolList = DraftTypedList<BoolListFamily>;
pub(in crate::plan::execution::lowering) type DraftNilList = DraftTypedList<NilListFamily>;
pub(in crate::plan::execution::lowering) type DraftTupleList = DraftTypedList<TupleListFamily>;
pub(in crate::plan::execution::lowering) type DraftListList = DraftTypedList<ListListFamily>;
pub(in crate::plan::execution::lowering) type DraftFunctionList =
    DraftTypedList<FunctionListFamily>;

pub(in crate::plan::execution::lowering) type DraftGenericFunction =
    DraftTypedFunction<GenericFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftNeverFunction =
    DraftTypedFunction<NeverFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftIntFunction =
    DraftTypedFunction<IntFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftFloatFunction =
    DraftTypedFunction<FloatFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftStringFunction =
    DraftTypedFunction<StringFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftBitArrayFunction =
    DraftTypedFunction<BitArrayFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftUtfCodepointFunction =
    DraftTypedFunction<UtfCodepointFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftCustomFunction =
    DraftTypedFunction<CustomFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftExternalFunction =
    DraftTypedFunction<ExternalFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftBoolFunction =
    DraftTypedFunction<BoolFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftNilFunction =
    DraftTypedFunction<NilFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftTupleFunction =
    DraftTypedFunction<TupleFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftListFunction =
    DraftTypedFunction<ListFunctionFamily>;
pub(in crate::plan::execution::lowering) type DraftFunctionFunction =
    DraftTypedFunction<FunctionFunctionFamily>;

pub(in crate::plan::execution::lowering) enum DraftNeverReturn {}

pub(in crate::plan::execution::lowering) trait DraftGraphValue {
    fn erase(&self) -> DraftValueRef;
}

pub(in crate::plan::execution::lowering) struct DraftCursor {
    id: DraftBlockId,
    explicit_params: Vec<DraftValueRef>,
    instructions: Vec<DraftInstruction>,
    scope: DraftScope,
}

#[derive(Clone, Default)]
pub(in crate::plan::execution::lowering) struct DraftScope {
    locals: HashMap<super::super::local::LocalKey, DraftValueRef>,
}

pub(in crate::plan::execution::lowering) enum DraftInstruction {
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
    External {
        output: DraftExternal,
        kind: instruction::DraftExternalInstruction,
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

pub(in crate::plan::execution::lowering) struct DraftBlock {
    pub(in crate::plan::execution::lowering) explicit_params: Vec<DraftValueRef>,
    pub(in crate::plan::execution::lowering) instructions: Vec<DraftInstruction>,
    pub(in crate::plan::execution::lowering) terminator: DraftTerminator,
}

pub(in crate::plan::execution::lowering) enum DraftTerminator {
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
    Echo {
        subject: DraftValueRef,
        message: Option<DraftString>,
        site: crate::plan::EchoSite,
        next: DraftEdge,
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
        site: crate::plan::HostCallSite,
    },
}

pub(in crate::plan::execution::lowering) enum DraftNeverCallTarget {
    Direct(execution::function::NeverFunctionId),
    Value(DraftFunction),
}

pub(in crate::plan::execution::lowering) struct DraftEdge {
    pub(in crate::plan::execution::lowering) target: DraftBlockId,
    pub(in crate::plan::execution::lowering) explicit_args: Vec<DraftValueRef>,
}

pub(in crate::plan::execution::lowering) struct DraftMatchEdge {
    pub(in crate::plan::execution::lowering) target: DraftBlockId,
    pub(in crate::plan::execution::lowering) explicit_args: Vec<DraftMatchEdgeArgument>,
}

pub(in crate::plan::execution::lowering) enum DraftMatchEdgeArgument {
    Binding(usize),
}

pub(in crate::plan::execution::lowering) enum DraftFlow<T> {
    Value { cursor: DraftCursor, value: T },
    Diverged,
}

pub(in crate::plan::execution::lowering) struct DraftGraphBuilder<Return, TailCall> {
    pub(in crate::plan::execution::lowering) graph: DraftGraph,
    pub(in crate::plan::execution::lowering) returns: Vec<Return>,
    pub(in crate::plan::execution::lowering) tail_calls: Vec<TailCall>,
}

pub(in crate::plan::execution::lowering) struct DraftGraph {
    pub(in crate::plan::execution::lowering) entry: DraftBlockId,
    pub(in crate::plan::execution::lowering) parameter_count: usize,
    pub(in crate::plan::execution::lowering) blocks: HashMap<DraftBlockId, DraftBlock>,
    pub(in crate::plan::execution::lowering::graph) next_value: usize,
    pub(in crate::plan::execution::lowering::graph) next_block: usize,
}

pub(in crate::plan::execution::lowering) struct LoweredFunctionGraph<Body> {
    pub(in crate::plan::execution::lowering) parameter_count: usize,
    pub(in crate::plan::execution::lowering) body: Body,
}

impl<Body> LoweredFunctionGraph<Body> {
    pub(in crate::plan::execution::lowering) fn map<Mapped>(
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
    pub(in crate::plan::execution::lowering) fn shape(&self) -> &StoredValueShape {
        &self.shape
    }

    pub(in crate::plan::execution::lowering) fn erase(&self) -> DraftValueRef {
        DraftValueRef {
            key: self.key,
            shape: self.shape.clone(),
        }
    }

    pub(in crate::plan::execution::lowering::graph) fn from_ref(value: &DraftValueRef) -> Self {
        Self {
            key: value.key,
            shape: value.shape.clone(),
            family: PhantomData,
        }
    }

    pub(in crate::plan::execution::lowering) fn from_owned(value: DraftValueRef) -> Self {
        Self::from_ref(&value)
    }
}

impl<Family> DraftTypedList<Family> {
    pub(in crate::plan::execution::lowering) fn new(value: DraftList) -> Self {
        Self {
            value,
            family: PhantomData,
        }
    }

    pub(in crate::plan::execution::lowering) fn value(&self) -> &DraftList {
        &self.value
    }

    pub(in crate::plan::execution::lowering) fn from_ref(value: &DraftValueRef) -> Self {
        Self::new(DraftList::from_ref(value))
    }
}

impl<Family> DraftTypedFunction<Family> {
    pub(in crate::plan::execution::lowering) fn new(value: DraftFunction) -> Self {
        Self {
            value,
            family: PhantomData,
        }
    }

    pub(in crate::plan::execution::lowering) fn value(&self) -> &DraftFunction {
        &self.value
    }

    pub(in crate::plan::execution::lowering) fn from_ref(value: &DraftValueRef) -> Self {
        Self::new(DraftFunction::from_ref(value))
    }

    fn into_value(self) -> DraftFunction {
        self.value
    }
}

pub(in crate::plan::execution::lowering) trait DraftFunctionValue {
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
            | Self::External(value)
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
    pub(in crate::plan::execution::lowering) fn shape(&self) -> &StoredValueShape {
        &self.shape
    }
}

impl DraftScope {
    pub(in crate::plan::execution::lowering) fn insert(
        &mut self,
        key: super::super::local::LocalKey,
        value: DraftValueRef,
    ) {
        self.locals.insert(key, value);
    }

    pub(in crate::plan::execution::lowering) fn get(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftValueRef {
        self.locals[&key].clone()
    }

    pub(in crate::plan::execution::lowering) fn int(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftInt {
        DraftInt::from_ref(&self.locals[&key])
    }

    pub(in crate::plan::execution::lowering) fn float(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftFloat {
        DraftFloat::from_ref(&self.locals[&key])
    }

    pub(in crate::plan::execution::lowering) fn string(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftString {
        DraftString::from_ref(&self.locals[&key])
    }

    pub(in crate::plan::execution::lowering) fn bit_array(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftBitArray {
        DraftBitArray::from_ref(&self.locals[&key])
    }

    pub(in crate::plan::execution::lowering) fn utf_codepoint(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftUtfCodepoint {
        DraftUtfCodepoint::from_ref(&self.locals[&key])
    }

    pub(in crate::plan::execution::lowering) fn custom(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftCustom {
        DraftCustom::from_ref(&self.locals[&key])
    }

    pub(in crate::plan::execution::lowering) fn external(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftExternal {
        DraftExternal::from_ref(&self.locals[&key])
    }

    pub(in crate::plan::execution::lowering) fn bool(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftBool {
        DraftBool::from_ref(&self.locals[&key])
    }

    pub(in crate::plan::execution::lowering) fn nil(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftNil {
        DraftNil::from_ref(&self.locals[&key])
    }

    pub(in crate::plan::execution::lowering) fn tuple(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftTuple {
        DraftTuple::from_ref(&self.locals[&key])
    }

    pub(in crate::plan::execution::lowering) fn list(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftList {
        DraftList::from_ref(&self.locals[&key])
    }

    pub(in crate::plan::execution::lowering) fn function(
        &self,
        key: super::super::local::LocalKey,
    ) -> DraftFunction {
        DraftFunction::from_ref(&self.locals[&key])
    }
}

impl DraftCursor {
    pub(in crate::plan::execution::lowering) fn scope(&self) -> &DraftScope {
        &self.scope
    }

    pub(in crate::plan::execution::lowering) fn scope_mut(&mut self) -> &mut DraftScope {
        &mut self.scope
    }

    pub(in crate::plan::execution::lowering) fn id(&self) -> DraftBlockId {
        self.id
    }
}

impl<Return, TailCall> DraftGraphBuilder<Return, TailCall> {
    pub(in crate::plan::execution::lowering) fn new(
        params: Vec<(super::super::local::LocalKey, StoredValueShape)>,
        captures: Vec<(super::super::local::LocalKey, StoredValueShape)>,
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

    pub(in crate::plan::execution::lowering) fn finish_return(
        &mut self,
        cursor: DraftCursor,
        value: Return,
    ) where
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

    pub(in crate::plan::execution::lowering) fn finish_tail_call(
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

impl<Return, TailCall> DraftGraphBuilder<Return, TailCall> {
    pub(in crate::plan::execution::lowering) fn graph(&self) -> &DraftGraph {
        &self.graph
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

impl DraftGraph {
    fn new(
        params: Vec<(super::super::local::LocalKey, StoredValueShape)>,
        captures: Vec<(super::super::local::LocalKey, StoredValueShape)>,
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

    pub(in crate::plan::execution::lowering::graph) fn value_ref(
        &mut self,
        shape: StoredValueShape,
    ) -> DraftValueRef {
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

    pub(in crate::plan::execution::lowering) fn int_instruction(
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

    pub(in crate::plan::execution::lowering) fn float_instruction(
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

    pub(in crate::plan::execution::lowering) fn string_instruction(
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

    pub(in crate::plan::execution::lowering) fn bit_array_instruction(
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

    pub(in crate::plan::execution::lowering) fn utf_codepoint_instruction(
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

    pub(in crate::plan::execution::lowering) fn custom_instruction(
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

    pub(in crate::plan::execution::lowering) fn external_instruction(
        &mut self,
        cursor: &mut DraftCursor,
        shape: SpecializedExternalValueShape,
        kind: instruction::DraftExternalInstruction,
    ) -> DraftExternal {
        self.instruction(
            cursor,
            StoredValueShape::External(shape),
            DraftInstructionKind::External(kind),
        )
    }

    pub(in crate::plan::execution::lowering) fn bool_instruction(
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

    pub(in crate::plan::execution::lowering) fn nil_instruction(
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

    pub(in crate::plan::execution::lowering) fn tuple_instruction(
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

    pub(in crate::plan::execution::lowering) fn list_instruction(
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

    pub(in crate::plan::execution::lowering) fn function_instruction(
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

    pub(in crate::plan::execution::lowering) fn block(
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

    pub(in crate::plan::execution::lowering) fn empty_block(
        &mut self,
        scope: DraftScope,
    ) -> DraftCursor {
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

    pub(in crate::plan::execution::lowering) fn finish_jump(
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

    pub(in crate::plan::execution::lowering) fn finish_bool_branch(
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

    pub(in crate::plan::execution::lowering) fn finish_int_switch(
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

    pub(in crate::plan::execution::lowering) fn finish_float_switch(
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

    pub(in crate::plan::execution::lowering) fn finish_string_switch(
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

    pub(in crate::plan::execution::lowering) fn finish_match(
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

    pub(in crate::plan::execution::lowering) fn finish_let_assert_panic(
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

    pub(in crate::plan::execution::lowering) fn finish_echo(
        &mut self,
        cursor: DraftCursor,
        subject: DraftValueRef,
        message: Option<DraftString>,
        site: crate::plan::EchoSite,
        next: DraftBlockId,
    ) {
        self.finish(
            cursor,
            DraftTerminator::Echo {
                subject,
                message,
                site,
                next: DraftEdge {
                    target: next,
                    explicit_args: Vec::new(),
                },
            },
        );
    }

    pub(in crate::plan::execution::lowering) fn finish_source_stop(
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

    pub(in crate::plan::execution::lowering) fn finish_never_call(
        &mut self,
        cursor: DraftCursor,
        function: execution::function::NeverFunctionId,
        args: Vec<DraftValueRef>,
        site: crate::plan::HostCallSite,
    ) {
        self.finish(
            cursor,
            DraftTerminator::NeverCall {
                function: DraftNeverCallTarget::Direct(function),
                args,
                site,
            },
        );
    }

    pub(in crate::plan::execution::lowering) fn finish_never_function_call(
        &mut self,
        cursor: DraftCursor,
        function: DraftFunction,
        args: Vec<DraftValueRef>,
        site: crate::plan::HostCallSite,
    ) {
        self.finish(
            cursor,
            DraftTerminator::NeverCall {
                function: DraftNeverCallTarget::Value(function),
                args,
                site,
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
            DraftInstructionKind::External(kind) => Self::External {
                output: DraftExternal::from_ref(&output),
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

    pub(in crate::plan::execution::lowering::graph) fn output(&self) -> DraftValueRef {
        match self {
            Self::Int { output, .. } => output.erase(),
            Self::Float { output, .. } => output.erase(),
            Self::String { output, .. } => output.erase(),
            Self::BitArray { output, .. } => output.erase(),
            Self::UtfCodepoint { output, .. } => output.erase(),
            Self::Custom { output, .. } => output.erase(),
            Self::External { output, .. } => output.erase(),
            Self::Bool { output, .. } => output.erase(),
            Self::Nil { output, .. } => output.erase(),
            Self::Tuple { output, .. } => output.erase(),
            Self::List { output, .. } => output.erase(),
            Self::Function { output, .. } => output.erase(),
        }
    }

    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        match self {
            Self::Int { kind, .. } => kind.uses(values),
            Self::Float { kind, .. } => kind.uses(values),
            Self::String { kind, .. } => kind.uses(values),
            Self::BitArray { kind, .. } => kind.uses(values),
            Self::UtfCodepoint { kind, .. } => kind.uses(values),
            Self::Custom { kind, .. } => kind.uses(values),
            Self::External { kind, .. } => kind.uses(values),
            Self::Bool { kind, .. } => kind.uses(values),
            Self::Nil { kind, .. } => kind.uses(values),
            Self::Tuple { kind, .. } => kind.uses(values),
            Self::List { kind, .. } => kind.uses(values),
            Self::Function { kind, .. } => kind.uses(values),
        }
    }
}

impl DraftTerminator {
    pub(in crate::plan::execution::lowering::graph) fn successors(&self) -> Vec<DraftBlockId> {
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
            Self::Echo { next, .. } => vec![next.target],
            Self::Return { .. }
            | Self::TailCall { .. }
            | Self::SourceStop { .. }
            | Self::LetAssertPanic { .. }
            | Self::NeverCall { .. } => Vec::new(),
        }
    }

    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
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
            Self::Echo {
                subject,
                message,
                next,
                ..
            } => {
                values.push(subject.clone());
                if let Some(message) = message {
                    values.push(message.erase());
                }
                next.uses(values);
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
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        values: &mut Vec<DraftValueRef>,
    ) {
        values.extend(self.explicit_args.iter().cloned());
    }
}

impl DraftMatchEdge {
    pub(in crate::plan::execution::lowering::graph) fn uses(
        &self,
        _values: &mut Vec<DraftValueRef>,
    ) {
    }
}

impl<T> DraftFlow<T> {
    pub(in crate::plan::execution::lowering) fn value(cursor: DraftCursor, value: T) -> Self {
        Self::Value { cursor, value }
    }

    pub(in crate::plan::execution::lowering) fn map<U>(
        self,
        map: impl FnOnce(T) -> U,
    ) -> DraftFlow<U> {
        match self {
            Self::Value { cursor, value } => DraftFlow::Value {
                cursor,
                value: map(value),
            },
            Self::Diverged => DraftFlow::Diverged,
        }
    }

    pub(in crate::plan::execution::lowering) fn map_cursor<U>(
        self,
        map: impl FnOnce(&mut DraftCursor, T) -> U,
    ) -> DraftFlow<U> {
        match self {
            Self::Value { mut cursor, value } => {
                let value = map(&mut cursor, value);
                DraftFlow::Value { cursor, value }
            }
            Self::Diverged => DraftFlow::Diverged,
        }
    }

    pub(in crate::plan::execution::lowering) fn and_then<U>(
        self,
        next: impl FnOnce(
            DraftCursor,
            T,
        ) -> super::super::specialization::Representability<DraftFlow<U>>,
    ) -> super::super::specialization::Representability<DraftFlow<U>> {
        match self {
            Self::Value { cursor, value } => next(cursor, value),
            Self::Diverged => {
                super::super::specialization::Representability::Inhabited(DraftFlow::Diverged)
            }
        }
    }

    pub(in crate::plan::execution::lowering) fn fold<U>(
        self,
        diverged: U,
        value: impl FnOnce(DraftCursor, T) -> U,
    ) -> U {
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
            DraftStoredList::External(list.clone()),
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
