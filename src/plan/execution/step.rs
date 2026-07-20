use super::expression::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CustomExpr, CustomFunctionExpr,
    CustomLocalExpr, Expr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr, GenericFunctionExpr,
    IntExpr, IntFunctionExpr, ListFunctionExpr, ListLocalExpr, NeverFunctionExpr, NilExpr,
    NilFunctionExpr, StringExpr, StringFunctionExpr, TupleExpr, TupleFunctionExpr,
    TypedFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
};
use super::id::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId,
    CustomFunctionLocal, CustomLocal, FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocal,
    GenericFunctionLocal, IntFunctionLocalId, IntLocalId, ListFunctionLocal, ListLocal,
    NeverFunctionLocal, NilFunctionLocalId, NilLocalId, StringFunctionLocalId, StringLocalId,
    TupleFunctionLocalId, TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointLocalId,
};
use super::{ParamLocal, ParamSlot};
use crate::plan::execution::{BitArrayPattern, CustomBindingPattern};
use crate::plan::{PanicSite, SourceSpan};

pub struct Step {
    kind: StepKind,
}

pub(crate) struct AssertBinding {
    slot: ParamSlot,
}

pub(crate) struct StringAssertBinding {
    local: StringLocalId,
}

pub(crate) enum AssertPattern {
    Bind(AssertBinding),
    Discard,
    Int(num_bigint::BigInt),
    Float(f64),
    String(ecow::EcoString),
    Bool(bool),
    Nil,
    Tuple(Vec<AssertPattern>),
    List(ListAssertPattern),
    BitArray(BitArrayPattern),
    Custom(crate::plan::execution::CustomPattern),
    StringPrefix {
        prefix: ecow::EcoString,
        left: Option<StringAssertBinding>,
        right: Option<StringAssertBinding>,
    },
    Alias {
        pattern: Box<AssertPattern>,
        binding: AssertBinding,
    },
}

pub(crate) enum AssertSubject {
    Int(IntLocalId),
    Float(FloatLocalId),
    String(StringLocalId),
    BitArray(BitArrayLocalId),
    Custom(CustomLocal),
    Bool(BoolLocalId),
    Nil(NilLocalId),
    Tuple(TupleLocalId),
    List(ListLocal),
}

pub(crate) struct ListAssertPattern {
    elements: Vec<AssertPattern>,
    tail: Option<ListAssertTail>,
}

pub(crate) struct ListAssertTailBinding {
    local: ListLocal,
}

pub(crate) enum ListAssertTail {
    Ignore,
    Bind(ListAssertTailBinding),
}

pub(crate) enum StepKind {
    LetInt {
        local: IntLocalId,
        value: IntExpr,
    },
    LetFloat {
        local: FloatLocalId,
        value: FloatExpr,
    },
    LetString {
        local: StringLocalId,
        value: StringExpr,
    },
    LetBitArray {
        local: BitArrayLocalId,
        value: BitArrayExpr,
    },
    LetUtfCodepoint {
        local: UtfCodepointLocalId,
        value: UtfCodepointExpr,
    },
    LetCustom(CustomLocalExpr),
    LetBool {
        local: BoolLocalId,
        value: BoolExpr,
    },
    LetNil {
        local: NilLocalId,
        value: NilExpr,
    },
    LetTuple {
        local: TupleLocalId,
        value: TupleExpr,
    },
    LetList {
        value: ListLocalExpr,
    },
    LetIntFunction {
        local: IntFunctionLocalId,
        value: TypedFunctionExpr<IntFunctionExpr>,
    },
    LetFloatFunction {
        local: FloatFunctionLocalId,
        value: TypedFunctionExpr<FloatFunctionExpr>,
    },
    LetStringFunction {
        local: StringFunctionLocalId,
        value: TypedFunctionExpr<StringFunctionExpr>,
    },
    LetBitArrayFunction {
        local: BitArrayFunctionLocalId,
        value: TypedFunctionExpr<BitArrayFunctionExpr>,
    },
    LetUtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        value: TypedFunctionExpr<UtfCodepointFunctionExpr>,
    },
    LetGenericFunction {
        local: GenericFunctionLocal,
        value: TypedFunctionExpr<GenericFunctionExpr>,
    },
    LetNeverFunction {
        local: NeverFunctionLocal,
        value: TypedFunctionExpr<NeverFunctionExpr>,
    },
    LetCustomFunction {
        local: CustomFunctionLocal,
        value: TypedFunctionExpr<CustomFunctionExpr>,
    },
    LetBoolFunction {
        local: BoolFunctionLocalId,
        value: TypedFunctionExpr<BoolFunctionExpr>,
    },
    LetNilFunction {
        local: NilFunctionLocalId,
        value: TypedFunctionExpr<NilFunctionExpr>,
    },
    LetTupleFunction {
        local: TupleFunctionLocalId,
        value: TypedFunctionExpr<TupleFunctionExpr>,
    },
    LetListFunction {
        local: ListFunctionLocal,
        value: TypedFunctionExpr<ListFunctionExpr>,
    },
    LetFunctionFunction {
        local: FunctionFunctionLocal,
        value: TypedFunctionExpr<FunctionFunctionExpr>,
    },
    AssertPattern {
        subject: AssertSubject,
        pattern: AssertPattern,
        message: Option<StringExpr>,
        site: PanicSite,
        pattern_span: SourceSpan,
    },
    BindCustomFields {
        local: CustomLocal,
        pattern: CustomBindingPattern,
    },
    BindCustomValueFields {
        value: CustomExpr,
        pattern: CustomBindingPattern,
    },
    AssertBool {
        condition: BoolExpr,
        message: Option<StringExpr>,
        site: PanicSite,
    },
    Evaluate(Expr),
}

impl AssertBinding {
    pub(crate) fn new(slot: ParamSlot) -> Self {
        Self { slot }
    }

    pub(crate) fn local(&self) -> &ParamLocal {
        self.slot.local()
    }

    pub(crate) fn shape(&self) -> super::ValueShapeId {
        self.slot.shape()
    }
}

impl StringAssertBinding {
    pub(super) fn new(local: StringLocalId) -> Self {
        Self { local }
    }

    pub(crate) fn local(&self) -> StringLocalId {
        self.local
    }
}

impl ListAssertPattern {
    pub(crate) fn new(elements: Vec<AssertPattern>, tail: Option<ListAssertTail>) -> Self {
        Self { elements, tail }
    }

    pub(crate) fn elements(&self) -> &[AssertPattern] {
        &self.elements
    }

    pub(crate) fn tail(&self) -> Option<&ListAssertTail> {
        self.tail.as_ref()
    }
}

impl ListAssertTail {
    pub(crate) fn bind(local: ListLocal) -> Self {
        Self::Bind(ListAssertTailBinding { local })
    }
}

impl ListAssertTailBinding {
    pub(crate) fn local(&self) -> &ListLocal {
        &self.local
    }
}

impl Step {
    pub(in crate::plan::execution) fn from_kind(kind: StepKind) -> Self {
        Self { kind }
    }

    pub(crate) fn kind(&self) -> &StepKind {
        &self.kind
    }
}
