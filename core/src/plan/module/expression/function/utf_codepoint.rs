use crate::plan::CustomFieldAccess;
use crate::plan::{
    BoolExpr, CaptureArg, ConstantUtfCodepointFunctionInstantiation, FloatExpr,
    FunctionFunctionExpr, FunctionInstantiation, FunctionListExpr, FunctionType, IntExpr,
    PanicExpr, Step, StringExpr, TupleExpr, UtfCodepointFunctionLocalId,
    UtfCodepointFunctionReference,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct UtfCodepointFunctionExpr {
    type_: FunctionType,
    kind: UtfCodepointFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum UtfCodepointFunctionExprKind {
    Constant(ConstantUtfCodepointFunctionInstantiation),
    Reference(UtfCodepointFunctionReference),
    Closure {
        function: FunctionInstantiation,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: UtfCodepointFunctionLocalId,
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
        true_: Box<UtfCodepointFunctionExpr>,
        false_: Box<UtfCodepointFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, UtfCodepointFunctionExpr)>,
        fallback: Box<UtfCodepointFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, UtfCodepointFunctionExpr)>,
        fallback: Box<UtfCodepointFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, UtfCodepointFunctionExpr)>,
        fallback: Box<UtfCodepointFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<UtfCodepointFunctionExpr>,
    },
}

impl UtfCodepointFunctionExpr {
    pub(crate) fn constant(
        value: ConstantUtfCodepointFunctionInstantiation,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: UtfCodepointFunctionExprKind::Constant(value),
        }
    }

    pub(crate) fn reference(value: UtfCodepointFunctionReference) -> Self {
        let type_ = value.instantiation().shape().type_();
        Self {
            type_,
            kind: UtfCodepointFunctionExprKind::Reference(value),
        }
    }

    pub(crate) fn closure(
        function: FunctionInstantiation,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: UtfCodepointFunctionExprKind::Closure { function, captures },
        }
    }

    pub(crate) fn local_get(
        local: UtfCodepointFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: UtfCodepointFunctionExprKind::LocalGet { local, name },
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
            kind: UtfCodepointFunctionExprKind::Call {
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
            kind: UtfCodepointFunctionExprKind::FunctionCall {
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
            kind: UtfCodepointFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
                type_,
            },
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: UtfCodepointFunctionExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(
        list: impl Into<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: UtfCodepointFunctionExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
                type_,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: UtfCodepointFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: UtfCodepointFunctionExpr,
        false_: UtfCodepointFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: UtfCodepointFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, UtfCodepointFunctionExpr)>,
        fallback: UtfCodepointFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: UtfCodepointFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, UtfCodepointFunctionExpr)>,
        fallback: UtfCodepointFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: UtfCodepointFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, UtfCodepointFunctionExpr)>,
        fallback: UtfCodepointFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: UtfCodepointFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: UtfCodepointFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: UtfCodepointFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &UtfCodepointFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{UtfCodepointFunctionExpr, UtfCodepointFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionFunctionExpr, FunctionFunctionReference, FunctionInstantiation,
        FunctionShape, FunctionType, IntExpr, Step, StringExpr, UtfCodepointFunctionLocalId,
        UtfCodepointFunctionReference, ValueShape, ValueType, monomorphic_function_instantiation,
    };

    #[test]
    fn utf_codepoint_function_expr_kind_accessors() {
        assert_eq!(
            function_value().kind(),
            &UtfCodepointFunctionExprKind::Reference(UtfCodepointFunctionReference::new(
                function_instantiation()
            )),
        );
        assert_eq!(
            UtfCodepointFunctionExpr::closure(
                function_instantiation(),
                Vec::new(),
                function_type(),
            )
            .kind(),
            &UtfCodepointFunctionExprKind::Closure {
                function: function_instantiation(),
                captures: Vec::new(),
            },
        );
        assert_eq!(
            UtfCodepointFunctionExpr::local_get(
                UtfCodepointFunctionLocalId(0),
                "f".into(),
                function_type(),
            )
            .kind(),
            &UtfCodepointFunctionExprKind::LocalGet {
                local: UtfCodepointFunctionLocalId(0),
                name: "f".into(),
            },
        );
        assert_eq!(
            UtfCodepointFunctionExpr::call(
                function_returning_function_instantiation(),
                Vec::new(),
                function_type(),
            )
            .kind(),
            &UtfCodepointFunctionExprKind::Call {
                function: function_returning_function_instantiation(),
                args: Vec::new(),
                type_: function_type(),
                site: crate::plan::HostCallSite::unknown(),
            },
        );
        assert_eq!(
            UtfCodepointFunctionExpr::function_call(
                function_function_value(),
                Vec::new(),
                function_type(),
            )
            .kind(),
            &UtfCodepointFunctionExprKind::FunctionCall {
                function: Box::new(function_function_value()),
                args: Vec::new(),
                type_: function_type(),
                site: crate::plan::HostCallSite::unknown(),
            },
        );
        assert_eq!(
            UtfCodepointFunctionExpr::tuple_index(tuple_expr(), 0, function_type()).kind(),
            &UtfCodepointFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
                type_: function_type(),
            },
        );
        assert_eq!(
            UtfCodepointFunctionExpr::bool_case(
                BoolExpr::value(true),
                function_value(),
                function_value(),
            )
            .kind(),
            &UtfCodepointFunctionExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(function_value()),
                false_: Box::new(function_value()),
            },
        );
        assert_eq!(
            UtfCodepointFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                function_value(),
            )
            .kind(),
            &UtfCodepointFunctionExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            UtfCodepointFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), function_value())],
                function_value(),
            )
            .kind(),
            &UtfCodepointFunctionExprKind::StringCase {
                subject: Box::new(StringExpr::value("one".into())),
                clauses: vec![("one".into(), function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            UtfCodepointFunctionExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, function_value())],
                function_value(),
            )
            .kind(),
            &UtfCodepointFunctionExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            UtfCodepointFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(0.into())))],
                function_value(),
            )
            .kind(),
            &UtfCodepointFunctionExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(0.into())))],
                return_: Box::new(function_value()),
            },
        );
    }

    #[test]
    fn utf_codepoint_function_expr_type() {
        assert_eq!(function_value().type_(), &function_type());
    }

    fn function_value() -> UtfCodepointFunctionExpr {
        UtfCodepointFunctionExpr::reference(UtfCodepointFunctionReference::new(
            function_instantiation(),
        ))
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint)
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(function_returning_function_instantiation()),
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
            vec![Expr::function(crate::plan::FunctionExpr::utf_codepoint(
                function_value(),
            ))],
            vec![ValueType::Function(Box::new(function_type()))],
        )
    }
}
