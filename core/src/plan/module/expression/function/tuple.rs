use crate::plan::CustomFieldAccess;
use crate::plan::{
    BoolExpr, CaptureArg, ConstantTupleFunctionInstantiation, FloatExpr, FunctionFunctionExpr,
    FunctionInstantiation, FunctionListExpr, FunctionType, IntExpr, PanicExpr, Step, StringExpr,
    TupleExpr, TupleFunctionLocalId, TupleFunctionReference, ValueType,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct TupleFunctionExpr {
    type_: FunctionType,
    kind: TupleFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum TupleFunctionExprKind {
    Constant(ConstantTupleFunctionInstantiation),
    Reference(TupleFunctionReference),
    Closure {
        function: FunctionInstantiation,
        captures: Vec<CaptureArg>,
        return_type: Vec<ValueType>,
    },
    LocalGet {
        local: TupleFunctionLocalId,
        name: EcoString,
    },
    Call {
        function: FunctionInstantiation,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: Box<FunctionFunctionExpr>,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
        site: crate::plan::HostCallSite,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
        type_: FunctionType,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<TupleFunctionExpr>,
        false_: Box<TupleFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, TupleFunctionExpr)>,
        fallback: Box<TupleFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, TupleFunctionExpr)>,
        fallback: Box<TupleFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, TupleFunctionExpr)>,
        fallback: Box<TupleFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<TupleFunctionExpr>,
    },
}

impl TupleFunctionExpr {
    pub(crate) fn constant(value: ConstantTupleFunctionInstantiation, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: TupleFunctionExprKind::Constant(value),
        }
    }

    pub(crate) fn reference(value: TupleFunctionReference) -> Self {
        let type_ = value.instantiation().shape().type_();
        Self {
            type_,
            kind: TupleFunctionExprKind::Reference(value),
        }
    }

    pub(crate) fn closure(
        function: FunctionInstantiation,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
        return_type: Vec<ValueType>,
    ) -> Self {
        Self {
            type_,
            kind: TupleFunctionExprKind::Closure {
                function,
                captures,
                return_type,
            },
        }
    }

    pub(crate) fn local_get(
        local: TupleFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: TupleFunctionExprKind::LocalGet { local, name },
        }
    }

    #[cfg(test)]
    pub(crate) fn call(
        function: FunctionInstantiation,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self::call_at(function, args, type_, crate::plan::HostCallSite::unknown())
    }

    pub(crate) fn call_at(
        function: FunctionInstantiation,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
        site: crate::plan::HostCallSite,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: TupleFunctionExprKind::Call {
                function,
                args,
                type_,
                site,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn function_call(
        function: FunctionFunctionExpr,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self::function_call_at(function, args, type_, crate::plan::HostCallSite::unknown())
    }

    pub(crate) fn function_call_at(
        function: FunctionFunctionExpr,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
        site: crate::plan::HostCallSite,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: TupleFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
                site,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: FunctionType) -> Self {
        Self {
            type_: type_.clone(),
            kind: TupleFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
                type_,
            },
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: TupleFunctionExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(
        list: impl Into<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: TupleFunctionExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
                type_,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: TupleFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: TupleFunctionExpr,
        false_: TupleFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: TupleFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, TupleFunctionExpr)>,
        fallback: TupleFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: TupleFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, TupleFunctionExpr)>,
        fallback: TupleFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: TupleFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, TupleFunctionExpr)>,
        fallback: TupleFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: TupleFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: TupleFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: TupleFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &TupleFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{TupleFunctionExpr, TupleFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionFunctionExpr, FunctionFunctionReference, FunctionInstantiation,
        FunctionShape, FunctionType, IntExpr, Step, TupleExpr, TupleFunctionLocalId,
        TupleFunctionReference, ValueShape, ValueType, monomorphic_function_instantiation,
    };

    #[test]
    fn tuple_function_expr_kind_accessors() {
        assert_eq!(
            tuple_function_value().kind(),
            &TupleFunctionExprKind::Reference(
                TupleFunctionReference::new(function_instantiation())
            ),
        );
        assert_eq!(
            TupleFunctionExpr::closure(
                function_instantiation(),
                Vec::new(),
                tuple_function_type(),
                tuple_type(),
            )
            .kind(),
            &TupleFunctionExprKind::Closure {
                function: function_instantiation(),
                captures: Vec::new(),
                return_type: tuple_type(),
            },
        );
        assert_eq!(
            TupleFunctionExpr::local_get(
                TupleFunctionLocalId(0),
                "f".into(),
                tuple_function_type(),
            )
            .kind(),
            &TupleFunctionExprKind::LocalGet {
                local: TupleFunctionLocalId(0),
                name: "f".into(),
            },
        );
        assert_eq!(
            TupleFunctionExpr::call(
                function_returning_function_instantiation(),
                Vec::new(),
                tuple_function_type(),
            )
            .kind(),
            &TupleFunctionExprKind::Call {
                function: function_returning_function_instantiation(),
                args: Vec::new(),
                type_: tuple_function_type(),
                site: crate::plan::HostCallSite::unknown(),
            },
        );
        assert_eq!(
            TupleFunctionExpr::function_call(
                function_function_value(),
                Vec::new(),
                tuple_function_type(),
            )
            .kind(),
            &TupleFunctionExprKind::FunctionCall {
                function: Box::new(function_function_value()),
                args: Vec::new(),
                type_: tuple_function_type(),
                site: crate::plan::HostCallSite::unknown(),
            },
        );
        assert_eq!(
            TupleFunctionExpr::tuple_index(tuple_expr(), 0, tuple_function_type()).kind(),
            &TupleFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
                type_: tuple_function_type(),
            },
        );
        assert_eq!(
            TupleFunctionExpr::bool_case(
                BoolExpr::value(true),
                tuple_function_value(),
                tuple_function_value(),
            )
            .kind(),
            &TupleFunctionExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(tuple_function_value()),
                false_: Box::new(tuple_function_value()),
            },
        );
        assert_eq!(
            TupleFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), tuple_function_value())],
                tuple_function_value(),
            )
            .kind(),
            &TupleFunctionExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), tuple_function_value())],
                fallback: Box::new(tuple_function_value()),
            },
        );
        assert_eq!(
            TupleFunctionExpr::string_case(
                crate::plan::StringExpr::value("one".into()),
                vec![("one".into(), tuple_function_value())],
                tuple_function_value(),
            )
            .kind(),
            &TupleFunctionExprKind::StringCase {
                subject: Box::new(crate::plan::StringExpr::value("one".into())),
                clauses: vec![("one".into(), tuple_function_value())],
                fallback: Box::new(tuple_function_value()),
            },
        );
        assert_eq!(
            TupleFunctionExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, tuple_function_value())],
                tuple_function_value(),
            )
            .kind(),
            &TupleFunctionExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, tuple_function_value())],
                fallback: Box::new(tuple_function_value()),
            },
        );
        assert_eq!(
            TupleFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                tuple_function_value(),
            )
            .kind(),
            &TupleFunctionExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                return_: Box::new(tuple_function_value()),
            },
        );
    }

    #[test]
    fn tuple_function_expr_type() {
        assert_eq!(tuple_function_value().type_(), &tuple_function_type());
        assert_eq!(
            TupleFunctionExpr::closure(
                function_instantiation(),
                Vec::new(),
                tuple_function_type(),
                tuple_type(),
            )
            .type_(),
            &tuple_function_type(),
        );
        assert_eq!(
            TupleFunctionExpr::bool_case(
                BoolExpr::value(true),
                tuple_function_value(),
                tuple_function_value(),
            )
            .type_(),
            &tuple_function_type(),
        );
        assert_eq!(
            TupleFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), tuple_function_value())],
                tuple_function_value(),
            )
            .type_(),
            &tuple_function_type(),
        );
        assert_eq!(
            TupleFunctionExpr::block(Vec::new(), tuple_function_value()).type_(),
            &tuple_function_type(),
        );
    }

    fn tuple_function_value() -> TupleFunctionExpr {
        TupleFunctionExpr::reference(TupleFunctionReference::new(function_instantiation()))
    }

    fn tuple_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Tuple(tuple_type()))
    }

    fn tuple_type() -> Vec<ValueType> {
        vec![ValueType::Int]
    }

    fn tuple_expr() -> TupleExpr {
        TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Function(Box::new(tuple_function_type()))],
        )
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(function_returning_function_instantiation()),
            tuple_function_type(),
        )
    }

    fn function_instantiation() -> FunctionInstantiation {
        monomorphic_function_instantiation(
            0,
            FunctionShape::from_function_type(tuple_function_type()),
        )
    }

    fn function_returning_function_instantiation() -> FunctionInstantiation {
        monomorphic_function_instantiation(
            1,
            FunctionShape::new(
                Vec::new(),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    tuple_function_type(),
                ))),
            ),
        )
    }
}
