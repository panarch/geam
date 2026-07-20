use super::{
    BitArrayExpr, BitArrayFunctionExpr, BitArrayFunctionFunctionId, BitArrayFunctionId,
    BitArrayListExpr, BitArrayListFunctionId, BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId,
    BoolFunctionId, BoolListExpr, BoolListFunctionId, CallArg, CustomFunctionExprKind,
    CustomFunctionFunctionId, CustomFunctionId, CustomFunctionType, CustomListExpr,
    CustomListFunctionId, FloatExpr, FloatFunctionExpr, FloatFunctionFunctionId, FloatFunctionId,
    FloatListExpr, FloatListFunctionId, FunctionFunctionExprKind, FunctionFunctionFunctionId,
    FunctionFunctionType, FunctionListExpr, FunctionListFunctionId, GenericFunctionExpr,
    GenericFunctionFunctionId, IntExpr, IntFunctionExpr, IntFunctionFunctionId, IntFunctionId,
    IntListExpr, IntListFunctionId, ListFunctionExpr, ListFunctionFunctionId, ListListExpr,
    ListListFunctionId, NeverExpr, NeverFunctionId, NilExpr, NilFunctionExpr,
    NilFunctionFunctionId, NilFunctionId, NilListExpr, NilListFunctionId, ParameterListExpr,
    ParameterListFunctionId, ParameterListListExpr, ParameterListListFunctionId, Step, StringExpr,
    StringFunctionExpr, StringFunctionFunctionId, StringFunctionId, StringListExpr,
    StringListFunctionId, TupleExpr, TupleFunctionExpr, TupleFunctionFunctionId, TupleFunctionId,
    TupleListExpr, TupleListFunctionId, UtfCodepointExpr, UtfCodepointFunctionExpr,
    UtfCodepointFunctionFunctionId, UtfCodepointFunctionId, UtfCodepointListExpr,
    UtfCodepointListFunctionId,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) type IntReturn = ReturnGraph<IntExpr, IntFunctionId>;
pub(crate) type NeverReturn = ReturnGraph<NeverExpr, NeverFunctionId>;
pub(crate) type FloatReturn = ReturnGraph<FloatExpr, FloatFunctionId>;
pub(crate) type StringReturn = ReturnGraph<StringExpr, StringFunctionId>;
pub(crate) type BitArrayReturn = ReturnGraph<BitArrayExpr, BitArrayFunctionId>;
pub(crate) type UtfCodepointReturn = ReturnGraph<UtfCodepointExpr, UtfCodepointFunctionId>;
pub(crate) struct CustomReturn {
    signature_shape: super::CustomValueShape,
    body_shape: super::CustomValueShape,
    body: ReturnGraph<super::CustomExprKind, usize>,
}
pub(crate) type BoolReturn = ReturnGraph<BoolExpr, BoolFunctionId>;
pub(crate) type NilReturn = ReturnGraph<NilExpr, NilFunctionId>;
pub(crate) type TupleReturn = ReturnGraph<TupleExpr, TupleFunctionId>;
pub(crate) type ParameterListReturn = ReturnGraph<ParameterListExpr, ParameterListFunctionId>;
pub(crate) type IntListReturn = ReturnGraph<IntListExpr, IntListFunctionId>;
pub(crate) type FloatListReturn = ReturnGraph<FloatListExpr, FloatListFunctionId>;
pub(crate) type StringListReturn = ReturnGraph<StringListExpr, StringListFunctionId>;
pub(crate) type BitArrayListReturn = ReturnGraph<BitArrayListExpr, BitArrayListFunctionId>;
pub(crate) type UtfCodepointListReturn =
    ReturnGraph<UtfCodepointListExpr, UtfCodepointListFunctionId>;
pub(crate) type CustomListReturn = ReturnGraph<CustomListExpr, CustomListFunctionId>;
pub(crate) type BoolListReturn = ReturnGraph<BoolListExpr, BoolListFunctionId>;
pub(crate) type NilListReturn = ReturnGraph<NilListExpr, NilListFunctionId>;
pub(crate) type TupleListReturn = ReturnGraph<TupleListExpr, TupleListFunctionId>;
pub(crate) type ParameterListListReturn =
    ReturnGraph<ParameterListListExpr, ParameterListListFunctionId>;
pub(crate) type ListListReturn = ReturnGraph<ListListExpr, ListListFunctionId>;
pub(crate) type FunctionListReturn = ReturnGraph<FunctionListExpr, FunctionListFunctionId>;
pub(crate) type IntFunctionReturn =
    TypedFunctionReturn<ReturnGraph<IntFunctionExpr, IntFunctionFunctionId>>;
pub(crate) type FloatFunctionReturn =
    TypedFunctionReturn<ReturnGraph<FloatFunctionExpr, FloatFunctionFunctionId>>;
pub(crate) type StringFunctionReturn =
    TypedFunctionReturn<ReturnGraph<StringFunctionExpr, StringFunctionFunctionId>>;
pub(crate) type BitArrayFunctionReturn =
    TypedFunctionReturn<ReturnGraph<BitArrayFunctionExpr, BitArrayFunctionFunctionId>>;
pub(crate) type UtfCodepointFunctionReturn =
    TypedFunctionReturn<ReturnGraph<UtfCodepointFunctionExpr, UtfCodepointFunctionFunctionId>>;
pub(crate) type GenericFunctionReturn =
    TypedFunctionReturn<ReturnGraph<GenericFunctionExpr, GenericFunctionFunctionId>>;
pub(crate) type NeverFunctionReturn =
    TypedFunctionReturn<ReturnGraph<super::NeverFunctionExpr, super::NeverFunctionFunctionId>>;
pub(crate) struct CustomFunctionReturn {
    shape: super::FunctionShape,
    type_: CustomFunctionType,
    body: ReturnGraph<CustomFunctionExprKind, usize>,
}
pub(crate) type BoolFunctionReturn =
    TypedFunctionReturn<ReturnGraph<BoolFunctionExpr, BoolFunctionFunctionId>>;
pub(crate) type NilFunctionReturn =
    TypedFunctionReturn<ReturnGraph<NilFunctionExpr, NilFunctionFunctionId>>;
pub(crate) type TupleFunctionReturn =
    TypedFunctionReturn<ReturnGraph<TupleFunctionExpr, TupleFunctionFunctionId>>;
pub(crate) type ListFunctionReturn =
    TypedFunctionReturn<ReturnGraph<ListFunctionExpr, ListFunctionFunctionId>>;
pub(crate) struct FunctionFunctionReturn {
    shape: super::FunctionShape,
    type_: FunctionFunctionType,
    body: ReturnGraph<FunctionFunctionExprKind, usize>,
}

pub(crate) struct TypedFunctionReturn<Body> {
    shape: super::FunctionShape,
    body: Body,
}

pub(crate) struct ReturnGraph<Expression, Function> {
    entry: ReturnTarget,
    blocks: Box<[ReturnBlock]>,
    expressions: Box<[Expression]>,
    tail_calls: Box<[ReturnTailCall<Function>]>,
}

pub(crate) enum ReturnBlock {
    Return {
        expression: ReturnExpressionId,
    },
    Never(super::NeverExpr),
    TailCall {
        call: ReturnTailCallId,
    },
    BoolBranch {
        subject: BoolExpr,
        true_: ReturnTarget,
        false_: ReturnTarget,
    },
    IntSwitch {
        subject: IntExpr,
        clauses: Box<[(BigInt, ReturnTarget)]>,
        fallback: ReturnTarget,
    },
    FloatSwitch {
        subject: FloatExpr,
        clauses: Box<[(f64, ReturnTarget)]>,
        fallback: ReturnTarget,
    },
    StringSwitch {
        subject: StringExpr,
        clauses: Box<[(EcoString, ReturnTarget)]>,
        fallback: ReturnTarget,
    },
    Steps {
        steps: Box<[Step]>,
        next: ReturnTarget,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct ReturnTarget(usize);

#[derive(Clone, Copy)]
pub(crate) struct ReturnExpressionId(usize);

#[derive(Clone, Copy)]
pub(crate) struct ReturnTailCallId(usize);

pub(crate) struct ReturnTailCall<Function> {
    function: Function,
    args: Box<[CallArg]>,
}

impl<Expression, Function> ReturnGraph<Expression, Function> {
    pub(super) fn from_parts(
        entry: ReturnTarget,
        blocks: Vec<ReturnBlock>,
        expressions: Vec<Expression>,
        tail_calls: Vec<ReturnTailCall<Function>>,
    ) -> Self {
        Self {
            entry,
            blocks: blocks.into_boxed_slice(),
            expressions: expressions.into_boxed_slice(),
            tail_calls: tail_calls.into_boxed_slice(),
        }
    }

    pub(crate) fn entry(&self) -> ReturnTarget {
        self.entry
    }

    pub(super) fn blocks(&self) -> &[ReturnBlock] {
        &self.blocks
    }

    pub(crate) fn block(&self, target: ReturnTarget) -> &ReturnBlock {
        &self.blocks[target.0]
    }

    pub(crate) fn expression(&self, id: ReturnExpressionId) -> &Expression {
        &self.expressions[id.0]
    }

    pub(crate) fn tail_call(&self, id: ReturnTailCallId) -> &ReturnTailCall<Function> {
        &self.tail_calls[id.0]
    }
}

impl ReturnTarget {
    pub(super) fn from_block_index(index: usize) -> Self {
        Self(index)
    }

    pub(super) fn index(self) -> usize {
        self.0
    }
}

impl ReturnExpressionId {
    pub(super) fn from_expression_index(index: usize) -> Self {
        Self(index)
    }
}

impl ReturnTailCallId {
    pub(super) fn from_call_index(index: usize) -> Self {
        Self(index)
    }
}

impl<Function> ReturnTailCall<Function> {
    pub(super) fn new(function: Function, args: Vec<CallArg>) -> Self {
        Self {
            function,
            args: args.into_boxed_slice(),
        }
    }

    pub(crate) fn function(&self) -> &Function {
        &self.function
    }

    pub(crate) fn args(&self) -> &[CallArg] {
        &self.args
    }
}

impl CustomReturn {
    pub(in crate::plan::execution) fn from_parts(
        signature_shape: super::CustomValueShape,
        body_shape: super::CustomValueShape,
        body: ReturnGraph<super::CustomExprKind, usize>,
    ) -> Self {
        Self {
            signature_shape,
            body_shape,
            body,
        }
    }

    pub(crate) fn type_id(&self) -> super::CustomTypeId {
        self.body_shape.type_id()
    }

    #[cfg(test)]
    pub(crate) fn body_shape(&self) -> &super::CustomValueShape {
        &self.body_shape
    }

    #[cfg(test)]
    pub(crate) fn signature_shape(&self) -> &super::CustomValueShape {
        &self.signature_shape
    }

    pub(crate) fn body(&self) -> &ReturnGraph<super::CustomExprKind, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> CustomFunctionId {
        CustomFunctionId::new(index, self.signature_shape)
    }
}

impl CustomFunctionReturn {
    pub(in crate::plan::execution) fn from_parts(
        shape: super::FunctionShape,
        type_: CustomFunctionType,
        body: ReturnGraph<CustomFunctionExprKind, usize>,
    ) -> Self {
        Self { shape, type_, body }
    }

    pub(crate) fn shape(&self) -> &super::FunctionShape {
        &self.shape
    }

    pub(crate) fn type_(&self) -> &CustomFunctionType {
        &self.type_
    }

    pub(crate) fn body(&self) -> &ReturnGraph<CustomFunctionExprKind, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> CustomFunctionFunctionId {
        CustomFunctionFunctionId::new(index, self.type_.clone())
    }
}

impl FunctionFunctionReturn {
    pub(in crate::plan::execution) fn from_parts(
        shape: super::FunctionShape,
        type_: FunctionFunctionType,
        body: ReturnGraph<FunctionFunctionExprKind, usize>,
    ) -> Self {
        Self { shape, type_, body }
    }

    pub(crate) fn shape(&self) -> &super::FunctionShape {
        &self.shape
    }

    pub(crate) fn type_(&self) -> &FunctionFunctionType {
        &self.type_
    }

    pub(crate) fn body(&self) -> &ReturnGraph<FunctionFunctionExprKind, usize> {
        &self.body
    }

    pub(crate) fn function_id(&self, index: usize) -> FunctionFunctionFunctionId {
        FunctionFunctionFunctionId::new(index, self.type_.clone())
    }
}

impl<Body> TypedFunctionReturn<Body> {
    pub(in crate::plan::execution) fn new(shape: super::FunctionShape, body: Body) -> Self {
        Self { shape, body }
    }

    pub(crate) fn shape(&self) -> &super::FunctionShape {
        &self.shape
    }

    pub(crate) fn body(&self) -> &Body {
        &self.body
    }
}
