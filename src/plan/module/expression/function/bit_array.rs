use crate::plan::CustomFieldAccess;
#[cfg(test)]
use crate::plan::ParamLocal;
use crate::plan::{
    BitArrayFunctionLocalId, BitArrayFunctionReference, BoolExpr, CaptureArg,
    ConstantBitArrayFunctionInstantiation, FloatExpr, FunctionFunctionExpr, FunctionInstantiation,
    FunctionListExpr, FunctionType, IntExpr, PanicExpr, ParamSlot, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct BitArrayFunctionExpr {
    type_: FunctionType,
    kind: BitArrayFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum BitArrayFunctionExprKind {
    Constant(ConstantBitArrayFunctionInstantiation),
    Reference(BitArrayFunctionReference),
    Closure {
        function: FunctionInstantiation,
        params: Vec<ParamSlot>,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: BitArrayFunctionLocalId,
        name: EcoString,
    },
    Call {
        function: FunctionInstantiation,
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
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<BitArrayFunctionExpr>,
        false_: Box<BitArrayFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, BitArrayFunctionExpr)>,
        fallback: Box<BitArrayFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, BitArrayFunctionExpr)>,
        fallback: Box<BitArrayFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, BitArrayFunctionExpr)>,
        fallback: Box<BitArrayFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<BitArrayFunctionExpr>,
    },
}

impl BitArrayFunctionExpr {
    pub(crate) fn constant(
        value: ConstantBitArrayFunctionInstantiation,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: BitArrayFunctionExprKind::Constant(value),
        }
    }

    pub(crate) fn reference(value: BitArrayFunctionReference) -> Self {
        let type_ = value.instantiation().shape().type_();
        Self {
            type_,
            kind: BitArrayFunctionExprKind::Reference(value),
        }
    }

    pub(crate) fn closure_slots(
        function: FunctionInstantiation,
        params: Vec<ParamSlot>,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: BitArrayFunctionExprKind::Closure {
                function,
                params,
                captures,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn closure(
        function: FunctionInstantiation,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
    ) -> Self {
        Self::closure_slots(
            function,
            params.into_iter().map(ParamSlot::from_local).collect(),
            captures,
            type_,
        )
    }

    pub(crate) fn local_get(
        local: BitArrayFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: BitArrayFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: FunctionInstantiation,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: BitArrayFunctionExprKind::Call {
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
            kind: BitArrayFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: FunctionType) -> Self {
        Self {
            type_: type_.clone(),
            kind: BitArrayFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
                type_,
            },
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: BitArrayFunctionExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(
        list: impl Into<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: BitArrayFunctionExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
                type_,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: BitArrayFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: BitArrayFunctionExpr,
        false_: BitArrayFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: BitArrayFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, BitArrayFunctionExpr)>,
        fallback: BitArrayFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: BitArrayFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, BitArrayFunctionExpr)>,
        fallback: BitArrayFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: BitArrayFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, BitArrayFunctionExpr)>,
        fallback: BitArrayFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: BitArrayFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: BitArrayFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: BitArrayFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &BitArrayFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{BitArrayFunctionExpr, BitArrayFunctionExprKind};
    use crate::plan::{
        BitArrayExpr, BitArrayFunctionLocalId, BitArrayFunctionReference, BitArrayLocalId,
        BoolExpr, Expr, FunctionFunctionExpr, FunctionFunctionReference, FunctionInstantiation,
        FunctionShape, FunctionType, IntExpr, ParamLocal, Step, StringExpr, ValueShape, ValueType,
        monomorphic_function_instantiation,
    };

    #[test]
    fn bit_array_function_expr_kind_accessors() {
        assert_eq!(
            function_value().kind(),
            &BitArrayFunctionExprKind::Reference(BitArrayFunctionReference::new(
                function_instantiation(),
                vec![ParamLocal::bit_array(BitArrayLocalId(0))],
            )),
        );
        assert_eq!(
            BitArrayFunctionExpr::closure(
                function_instantiation(),
                vec![ParamLocal::bit_array(BitArrayLocalId(0))],
                Vec::new(),
                function_type(),
            )
            .kind(),
            &BitArrayFunctionExprKind::Closure {
                function: function_instantiation(),
                params: vec![crate::plan::ParamSlot::from_local(ParamLocal::bit_array(
                    BitArrayLocalId(0)
                ))],
                captures: Vec::new(),
            },
        );
        assert_eq!(
            BitArrayFunctionExpr::local_get(
                BitArrayFunctionLocalId(0),
                "f".into(),
                function_type(),
            )
            .kind(),
            &BitArrayFunctionExprKind::LocalGet {
                local: BitArrayFunctionLocalId(0),
                name: "f".into(),
            },
        );
        assert_eq!(
            BitArrayFunctionExpr::call(
                function_returning_function_instantiation(),
                Vec::new(),
                function_type(),
            )
            .kind(),
            &BitArrayFunctionExprKind::Call {
                function: function_returning_function_instantiation(),
                args: Vec::new(),
                type_: function_type(),
            },
        );
        assert_eq!(
            BitArrayFunctionExpr::function_call(
                function_function_value(),
                Vec::new(),
                function_type(),
            )
            .kind(),
            &BitArrayFunctionExprKind::FunctionCall {
                function: Box::new(function_function_value()),
                args: Vec::new(),
                type_: function_type(),
            },
        );
        assert_eq!(
            BitArrayFunctionExpr::tuple_index(tuple_expr(), 0, function_type()).kind(),
            &BitArrayFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
                type_: function_type(),
            },
        );
        assert_eq!(
            BitArrayFunctionExpr::bool_case(
                BoolExpr::value(true),
                function_value(),
                function_value(),
            )
            .kind(),
            &BitArrayFunctionExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(function_value()),
                false_: Box::new(function_value()),
            },
        );
        assert_eq!(
            BitArrayFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                function_value(),
            )
            .kind(),
            &BitArrayFunctionExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            BitArrayFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), function_value())],
                function_value(),
            )
            .kind(),
            &BitArrayFunctionExprKind::StringCase {
                subject: Box::new(StringExpr::value("one".into())),
                clauses: vec![("one".into(), function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            BitArrayFunctionExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, function_value())],
                function_value(),
            )
            .kind(),
            &BitArrayFunctionExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            BitArrayFunctionExpr::block(
                vec![Step::evaluate(Expr::bit_array(BitArrayExpr::value(
                    Vec::new(),
                )))],
                function_value(),
            )
            .kind(),
            &BitArrayFunctionExprKind::Block {
                steps: vec![Step::evaluate(Expr::bit_array(BitArrayExpr::value(
                    Vec::new(),
                )))],
                return_: Box::new(function_value()),
            },
        );
    }

    #[test]
    fn bit_array_function_expr_type() {
        assert_eq!(function_value().type_(), &function_type());
    }

    fn function_value() -> BitArrayFunctionExpr {
        BitArrayFunctionExpr::reference(BitArrayFunctionReference::new(
            function_instantiation(),
            vec![ParamLocal::bit_array(BitArrayLocalId(0))],
        ))
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray)
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(function_returning_function_instantiation(), Vec::new()),
            function_type(),
        )
    }

    fn function_instantiation() -> FunctionInstantiation {
        monomorphic_function_instantiation(0, FunctionShape::from_function_type(function_type()))
    }

    fn function_returning_function_instantiation() -> FunctionInstantiation {
        monomorphic_function_instantiation(
            1,
            FunctionShape::new(
                Vec::new(),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(function_type()))),
            ),
        )
    }

    fn tuple_expr() -> crate::plan::TupleExpr {
        crate::plan::TupleExpr::value(
            vec![Expr::function(crate::plan::FunctionExpr::bit_array(
                function_value(),
            ))],
            vec![ValueType::Function(Box::new(function_type()))],
        )
    }
}
