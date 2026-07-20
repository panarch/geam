use super::{ParameterListItem, ParameterListListExpr};
use crate::plan::execution::{
    BoolExpr, ConstantId, CustomFieldAccess, DirectCall, FloatExpr, FunctionCall, IntExpr,
    ListFunctionExpr, NeverExpr, PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) struct ParameterListExpr {
    item: ParameterListItem,
    kind: ParameterListExprKind,
}

pub(crate) struct ParameterListIndexSource {
    list: Box<ParameterListListExpr>,
    index: usize,
}

pub(crate) enum ParameterListExprKind {
    Value,
    Never(NeverExpr),
    Constant(ConstantId<ParameterListExpr>),
    LocalGet {
        local: crate::plan::execution::ParameterListLocalId,
    },
    Call(DirectCall<crate::plan::execution::ParameterListFunctionId>),
    FunctionCall(FunctionCall<ListFunctionExpr>),
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex(ParameterListIndexSource),
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<ParameterListExprKind>,
        false_: Box<ParameterListExprKind>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, ParameterListExprKind)>,
        fallback: Box<ParameterListExprKind>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, ParameterListExprKind)>,
        fallback: Box<ParameterListExprKind>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, ParameterListExprKind)>,
        fallback: Box<ParameterListExprKind>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<ParameterListExprKind>,
    },
}

impl ParameterListExpr {
    pub(in crate::plan::execution) fn from_parts(
        item: ParameterListItem,
        kind: ParameterListExprKind,
    ) -> Self {
        Self { item, kind }
    }

    pub(crate) fn item(&self) -> &ParameterListItem {
        &self.item
    }

    pub(crate) fn kind(&self) -> &ParameterListExprKind {
        &self.kind
    }

    pub(in crate::plan::execution) fn into_kind(self) -> ParameterListExprKind {
        self.kind
    }
}

impl ParameterListIndexSource {
    pub(in crate::plan::execution) fn new(list: ParameterListListExpr, index: usize) -> Self {
        Self {
            list: Box::new(list),
            index,
        }
    }

    pub(crate) fn list(&self) -> &ParameterListListExpr {
        &self.list
    }

    pub(crate) fn index(&self) -> usize {
        self.index
    }
}
