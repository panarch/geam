use crate::plan::CustomFieldAccess;
use crate::plan::{
    BoolExpr, CaptureArg, ConstantIntFunctionInstantiation, FloatExpr, FunctionFunctionExpr,
    FunctionInstantiation, FunctionListExpr, FunctionType, IntExpr, IntFunctionLocalId,
    IntFunctionReference, PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct IntFunctionExpr {
    type_: FunctionType,
    kind: IntFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum IntFunctionExprKind {
    Constant(ConstantIntFunctionInstantiation),
    Reference(IntFunctionReference),
    Closure {
        function: FunctionInstantiation,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: IntFunctionLocalId,
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
        true_: Box<IntFunctionExpr>,
        false_: Box<IntFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, IntFunctionExpr)>,
        fallback: Box<IntFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, IntFunctionExpr)>,
        fallback: Box<IntFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, IntFunctionExpr)>,
        fallback: Box<IntFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<IntFunctionExpr>,
    },
}

impl IntFunctionExpr {
    pub(crate) fn constant(value: ConstantIntFunctionInstantiation, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: IntFunctionExprKind::Constant(value),
        }
    }

    pub(crate) fn reference(value: IntFunctionReference) -> Self {
        let type_ = value.instantiation().shape().type_();
        Self {
            type_,
            kind: IntFunctionExprKind::Reference(value),
        }
    }

    pub(crate) fn closure(
        function: FunctionInstantiation,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: IntFunctionExprKind::Closure { function, captures },
        }
    }

    pub(crate) fn local_get(
        local: IntFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: IntFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: FunctionInstantiation,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: IntFunctionExprKind::Call {
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
            kind: IntFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                type_,
            },
        }
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: FunctionType) -> Self {
        Self {
            type_: type_.clone(),
            kind: IntFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
                type_,
            },
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: IntFunctionExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(
        list: impl Into<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: IntFunctionExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
                type_,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: IntFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: IntFunctionExpr,
        false_: IntFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: IntFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: IntFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: IntFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: IntFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: IntFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: IntFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &IntFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{IntFunctionExpr, IntFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionFunctionExpr, FunctionFunctionReference, FunctionInstantiation,
        FunctionShape, FunctionType, IntExpr, IntFunctionLocalId, IntFunctionReference, Step,
        StringExpr, ValueShape, ValueType, monomorphic_function_instantiation,
    };

    #[test]
    fn int_function_expr_kind_accessors() {
        assert_eq!(
            int_function_type(),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        assert_eq!(
            int_function_value().kind(),
            &IntFunctionExprKind::Reference(IntFunctionReference::new(function_instantiation())),
        );
        assert_eq!(
            IntFunctionExpr::closure(function_instantiation(), Vec::new(), int_function_type(),)
                .kind(),
            &IntFunctionExprKind::Closure {
                function: function_instantiation(),
                captures: Vec::new(),
            },
        );
        assert_eq!(
            IntFunctionExpr::local_get(IntFunctionLocalId(0), "f".into(), int_function_type(),)
                .kind(),
            &IntFunctionExprKind::LocalGet {
                local: IntFunctionLocalId(0),
                name: "f".into(),
            },
        );
        assert_eq!(
            IntFunctionExpr::call(
                function_returning_function_instantiation(),
                Vec::new(),
                int_function_type(),
            )
            .kind(),
            &IntFunctionExprKind::Call {
                function: function_returning_function_instantiation(),
                args: Vec::new(),
                type_: int_function_type(),
            },
        );
        assert_eq!(
            IntFunctionExpr::function_call(
                function_function_value(),
                Vec::new(),
                int_function_type(),
            )
            .kind(),
            &IntFunctionExprKind::FunctionCall {
                function: Box::new(function_function_value()),
                args: Vec::new(),
                type_: int_function_type(),
            },
        );
        assert_eq!(
            IntFunctionExpr::tuple_index(tuple_expr(), 0, int_function_type()).kind(),
            &IntFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
                type_: int_function_type(),
            },
        );
        assert_eq!(
            IntFunctionExpr::bool_case(
                BoolExpr::value(true),
                int_function_value(),
                int_function_value(),
            )
            .kind(),
            &IntFunctionExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(int_function_value()),
                false_: Box::new(int_function_value()),
            },
        );
        assert_eq!(
            IntFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), int_function_value())],
                int_function_value(),
            )
            .kind(),
            &IntFunctionExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), int_function_value())],
                fallback: Box::new(int_function_value()),
            },
        );
        assert_eq!(
            IntFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), int_function_value())],
                int_function_value(),
            )
            .kind(),
            &IntFunctionExprKind::StringCase {
                subject: Box::new(StringExpr::value("one".into())),
                clauses: vec![("one".into(), int_function_value())],
                fallback: Box::new(int_function_value()),
            },
        );
        assert_eq!(
            IntFunctionExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, int_function_value())],
                int_function_value(),
            )
            .kind(),
            &IntFunctionExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, int_function_value())],
                fallback: Box::new(int_function_value()),
            },
        );
        assert_eq!(
            IntFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                int_function_value(),
            )
            .kind(),
            &IntFunctionExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                return_: Box::new(int_function_value()),
            },
        );
    }

    #[test]
    fn int_function_expr_type() {
        assert_eq!(int_function_value().type_(), &int_function_type());
        assert_eq!(
            IntFunctionExpr::bool_case(
                BoolExpr::value(true),
                int_function_value(),
                int_function_value(),
            )
            .type_(),
            &int_function_type(),
        );
        assert_eq!(
            IntFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), int_function_value())],
                int_function_value(),
            )
            .type_(),
            &int_function_type(),
        );
        assert_eq!(
            IntFunctionExpr::block(Vec::new(), int_function_value()).type_(),
            &int_function_type(),
        );
    }

    fn int_function_value() -> IntFunctionExpr {
        IntFunctionExpr::reference(IntFunctionReference::new(function_instantiation()))
    }

    fn int_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(function_returning_function_instantiation()),
            int_function_type(),
        )
    }

    fn function_instantiation() -> FunctionInstantiation {
        monomorphic_function_instantiation(
            0,
            FunctionShape::from_function_type(int_function_type()),
        )
    }

    fn function_returning_function_instantiation() -> FunctionInstantiation {
        monomorphic_function_instantiation(
            1,
            FunctionShape::new(
                Vec::new(),
                ValueShape::Function(Box::new(FunctionShape::from_function_type(
                    int_function_type(),
                ))),
            ),
        )
    }

    fn tuple_expr() -> crate::plan::TupleExpr {
        crate::plan::TupleExpr::value(
            vec![Expr::function(crate::plan::FunctionExpr::int(
                int_function_value(),
            ))],
            vec![ValueType::Function(Box::new(int_function_type()))],
        )
    }
}
