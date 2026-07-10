use crate::plan::{
    BoolExpr, CaptureArg, FloatExpr, FunctionFunctionExpr, FunctionListExpr, FunctionType, IntExpr,
    NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId, NilFunctionReference, PanicExpr,
    ParamLocal, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct NilFunctionExpr {
    type_: FunctionType,
    kind: NilFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum NilFunctionExprKind {
    Reference(NilFunctionReference),
    Closure {
        runtime_id: NilFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: NilFunctionLocalId,
        name: EcoString,
    },
    Call {
        function: NilFunctionFunctionId,
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
        true_: Box<NilFunctionExpr>,
        false_: Box<NilFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, NilFunctionExpr)>,
        fallback: Box<NilFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, NilFunctionExpr)>,
        fallback: Box<NilFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, NilFunctionExpr)>,
        fallback: Box<NilFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<NilFunctionExpr>,
    },
}

impl NilFunctionExpr {
    pub(crate) fn reference(value: NilFunctionReference) -> Self {
        let type_ = FunctionType::new(
            value.params().iter().map(ParamLocal::value_type).collect(),
            crate::plan::ValueType::Nil,
        );
        Self {
            type_,
            kind: NilFunctionExprKind::Reference(value),
        }
    }

    pub(crate) fn closure(
        runtime_id: NilFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: NilFunctionExprKind::Closure {
                runtime_id,
                params,
                captures,
            },
        }
    }

    pub(crate) fn local_get(
        local: NilFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: NilFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: NilFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: NilFunctionExprKind::Call {
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
            kind: NilFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: FunctionType) -> Self {
        Self {
            type_: type_.clone(),
            kind: NilFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
                type_,
            },
        }
    }

    pub(crate) fn list_index(
        list: impl Into<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: NilFunctionExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
                type_,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: NilFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: NilFunctionExpr,
        false_: NilFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: NilFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, NilFunctionExpr)>,
        fallback: NilFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: NilFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, NilFunctionExpr)>,
        fallback: NilFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: NilFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, NilFunctionExpr)>,
        fallback: NilFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: NilFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: NilFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: NilFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &NilFunctionExprKind {
        &self.kind
    }

    pub(crate) fn into_parts(self) -> (FunctionType, NilFunctionExprKind) {
        (self.type_, self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::{NilFunctionExpr, NilFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionReference,
        FunctionType, IntExpr, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId,
        NilFunctionReference, NilLocalId, ParamLocal, Step, StringExpr, ValueType,
    };

    #[test]
    fn nil_function_expr_kind_accessors() {
        assert_eq!(
            function_value().kind(),
            &NilFunctionExprKind::Reference(NilFunctionReference::new(
                NilFunctionId(0),
                vec![ParamLocal::nil(NilLocalId(0))],
            )),
        );
        assert_eq!(
            NilFunctionExpr::closure(
                NilFunctionId(0),
                vec![ParamLocal::nil(NilLocalId(0))],
                Vec::new(),
                function_type(),
            )
            .kind(),
            &NilFunctionExprKind::Closure {
                runtime_id: NilFunctionId(0),
                params: vec![ParamLocal::nil(NilLocalId(0))],
                captures: Vec::new(),
            },
        );
        assert_eq!(
            NilFunctionExpr::local_get(NilFunctionLocalId(0), "f".into(), function_type()).kind(),
            &NilFunctionExprKind::LocalGet {
                local: NilFunctionLocalId(0),
                name: "f".into(),
            },
        );
        assert_eq!(
            NilFunctionExpr::call(NilFunctionFunctionId(0), Vec::new(), function_type()).kind(),
            &NilFunctionExprKind::Call {
                function: NilFunctionFunctionId(0),
                args: Vec::new(),
                type_: function_type(),
            },
        );
        assert_eq!(
            NilFunctionExpr::function_call(function_function_value(), Vec::new(), function_type())
                .kind(),
            &NilFunctionExprKind::FunctionCall {
                function: Box::new(function_function_value()),
                args: Vec::new(),
                type_: function_type(),
            },
        );
        assert_eq!(
            NilFunctionExpr::tuple_index(tuple_expr(), 0, function_type()).kind(),
            &NilFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
                type_: function_type(),
            },
        );
        assert_eq!(
            NilFunctionExpr::bool_case(BoolExpr::value(true), function_value(), function_value(),)
                .kind(),
            &NilFunctionExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(function_value()),
                false_: Box::new(function_value()),
            },
        );
        assert_eq!(
            NilFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                function_value(),
            )
            .kind(),
            &NilFunctionExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            NilFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), function_value())],
                function_value(),
            )
            .kind(),
            &NilFunctionExprKind::StringCase {
                subject: Box::new(StringExpr::value("one".into())),
                clauses: vec![("one".into(), function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            NilFunctionExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, function_value())],
                function_value(),
            )
            .kind(),
            &NilFunctionExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            NilFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                function_value(),
            )
            .kind(),
            &NilFunctionExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                return_: Box::new(function_value()),
            },
        );
    }

    #[test]
    fn nil_function_expr_type() {
        assert_eq!(function_value().type_(), &function_type());
    }

    fn function_value() -> NilFunctionExpr {
        NilFunctionExpr::reference(NilFunctionReference::new(
            NilFunctionId(0),
            vec![ParamLocal::nil(NilLocalId(0))],
        ))
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Nil], ValueType::Nil)
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(
                FunctionFunctionId::Nil(NilFunctionFunctionId(0)),
                Vec::new(),
            ),
            function_type(),
        )
    }

    fn tuple_expr() -> crate::plan::TupleExpr {
        crate::plan::TupleExpr::value(
            vec![Expr::function(crate::plan::FunctionExpr::nil(
                function_value(),
            ))],
            vec![ValueType::Function(Box::new(function_type()))],
        )
    }
}
