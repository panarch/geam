use super::ParamLocal;
use super::expression::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CustomExpr, CustomFunctionExpr,
    Expr, FloatExpr, FloatFunctionExpr, FunctionFunctionExpr, IntExpr, IntFunctionExpr,
    ListFunctionExpr, ListLocalExpr, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr,
    TupleExpr, TupleFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
};
use super::id::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId,
    CustomFunctionLocalId, CustomLocalId, FloatFunctionLocalId, FloatLocalId,
    FunctionFunctionLocalId, IntFunctionLocalId, IntLocalId, ListFunctionLocal, ListLocal,
    NilFunctionLocalId, NilLocalId, StringFunctionLocalId, StringLocalId, TupleFunctionLocalId,
    TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointLocalId,
};
use crate::plan::execution::{BitArrayPattern, CustomBindingPattern};
use crate::plan::{PanicSite, SourceSpan};

pub struct Step {
    kind: StepKind,
}

pub(crate) struct AssertBinding {
    local: ParamLocal,
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
        left: Option<AssertBinding>,
        right: Option<AssertBinding>,
    },
    Alias {
        pattern: Box<AssertPattern>,
        binding: AssertBinding,
    },
}

pub(crate) enum BitArrayAssertPattern {
    Pattern(BitArrayPattern),
    Alias {
        pattern: Box<BitArrayAssertPattern>,
        local: BitArrayLocalId,
    },
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
    LetCustom {
        local: CustomLocalId,
        value: CustomExpr,
    },
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
        value: IntFunctionExpr,
    },
    LetFloatFunction {
        local: FloatFunctionLocalId,
        value: FloatFunctionExpr,
    },
    LetStringFunction {
        local: StringFunctionLocalId,
        value: StringFunctionExpr,
    },
    LetBitArrayFunction {
        local: BitArrayFunctionLocalId,
        value: BitArrayFunctionExpr,
    },
    LetUtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        value: UtfCodepointFunctionExpr,
    },
    LetCustomFunction {
        local: CustomFunctionLocalId,
        value: CustomFunctionExpr,
    },
    LetBoolFunction {
        local: BoolFunctionLocalId,
        value: BoolFunctionExpr,
    },
    LetNilFunction {
        local: NilFunctionLocalId,
        value: NilFunctionExpr,
    },
    LetTupleFunction {
        local: TupleFunctionLocalId,
        value: TupleFunctionExpr,
    },
    LetListFunction {
        local: ListFunctionLocal,
        value: ListFunctionExpr,
    },
    LetFunctionFunction {
        local: FunctionFunctionLocalId,
        value: FunctionFunctionExpr,
    },
    AssertList {
        local: ListLocal,
        pattern: AssertPattern,
        message: Option<StringExpr>,
        site: PanicSite,
        pattern_span: SourceSpan,
    },
    AssertBitArray {
        local: BitArrayLocalId,
        pattern: BitArrayAssertPattern,
        message: Option<StringExpr>,
        site: PanicSite,
        pattern_span: SourceSpan,
    },
    AssertCustom {
        local: CustomLocalId,
        pattern: AssertPattern,
        message: Option<StringExpr>,
        site: PanicSite,
        pattern_span: SourceSpan,
    },
    BindCustomFields {
        local: CustomLocalId,
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
    pub(crate) fn new(local: ParamLocal) -> Self {
        Self { local }
    }

    pub(crate) fn local(&self) -> &ParamLocal {
        &self.local
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
