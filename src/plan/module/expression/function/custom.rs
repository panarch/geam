use crate::plan::{
    BoolExpr, CaptureArg, CustomConstructor, CustomFunctionFunctionId, CustomFunctionId,
    CustomFunctionLocalId, CustomFunctionReference, CustomType, FloatExpr, FunctionFunctionExpr,
    FunctionListExpr, FunctionType, IntExpr, PanicExpr, ParamLocal, Step, StringExpr, TupleExpr,
    ValueType,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomFunctionExpr {
    type_: FunctionType,
    kind: CustomFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CustomFunctionExprKind {
    Constructor(CustomConstructor),
    Reference(CustomFunctionReference),
    Closure {
        runtime_id: CustomFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: CustomFunctionLocalId,
        name: EcoString,
    },
    Call {
        function: CustomFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    },
    FunctionCall {
        function: Box<FunctionFunctionExpr>,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
        type_: FunctionType,
    },
    ListIndex {
        list: Box<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<CustomFunctionExpr>,
        false_: Box<CustomFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, CustomFunctionExpr)>,
        fallback: Box<CustomFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, CustomFunctionExpr)>,
        fallback: Box<CustomFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, CustomFunctionExpr)>,
        fallback: Box<CustomFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<CustomFunctionExpr>,
    },
}

impl CustomFunctionExpr {
    pub(crate) fn constructor(constructor: CustomConstructor) -> Self {
        let type_ = FunctionType::new(
            constructor
                .fields()
                .iter()
                .map(|field| field.type_().clone())
                .collect(),
            ValueType::Custom(constructor.type_().clone()),
        );
        Self {
            type_,
            kind: CustomFunctionExprKind::Constructor(constructor),
        }
    }

    pub(crate) fn reference(value: CustomFunctionReference, return_type: CustomType) -> Self {
        let type_ = FunctionType::new(
            value.params().iter().map(ParamLocal::value_type).collect(),
            ValueType::Custom(return_type),
        );
        Self {
            type_,
            kind: CustomFunctionExprKind::Reference(value),
        }
    }

    pub(crate) fn closure(
        runtime_id: CustomFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: CustomFunctionExprKind::Closure {
                runtime_id,
                params,
                captures,
            },
        }
    }

    pub(crate) fn local_get(
        local: CustomFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: CustomFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: CustomFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: CustomFunctionExprKind::Call {
                function,
                args,
                type_,
            },
        }
    }

    pub(crate) fn function_call(
        function: FunctionFunctionExpr,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: CustomFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: FunctionType) -> Self {
        Self {
            type_: type_.clone(),
            kind: CustomFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
                type_,
            },
        }
    }

    pub(crate) fn list_index(list: FunctionListExpr, index: usize, type_: FunctionType) -> Self {
        Self {
            type_: type_.clone(),
            kind: CustomFunctionExprKind::ListIndex {
                list: Box::new(list),
                index,
                type_,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: CustomFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: CustomFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(subject: IntExpr, clauses: Vec<(BigInt, Self)>, fallback: Self) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: CustomFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, Self)>,
        fallback: Self,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: CustomFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, Self)>,
        fallback: Self,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: CustomFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: CustomFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }
    pub(crate) fn kind(&self) -> &CustomFunctionExprKind {
        &self.kind
    }
    pub(crate) fn into_parts(self) -> (FunctionType, CustomFunctionExprKind) {
        (self.type_, self.kind)
    }
}
