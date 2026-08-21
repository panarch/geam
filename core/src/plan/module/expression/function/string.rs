use crate::plan::CustomFieldAccess;
use crate::plan::{
    BoolExpr, CaptureArg, ConstantStringFunctionInstantiation, FloatExpr, FunctionFunctionExpr,
    FunctionInstantiation, FunctionListExpr, FunctionType, IntExpr, PanicExpr, Step, StringExpr,
    StringFunctionLocalId, StringFunctionReference, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct StringFunctionExpr {
    type_: FunctionType,
    kind: StringFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum StringFunctionExprKind {
    Constant(ConstantStringFunctionInstantiation),
    Reference(StringFunctionReference),
    Closure {
        function: FunctionInstantiation,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: StringFunctionLocalId,
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
        true_: Box<StringFunctionExpr>,
        false_: Box<StringFunctionExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, StringFunctionExpr)>,
        fallback: Box<StringFunctionExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, StringFunctionExpr)>,
        fallback: Box<StringFunctionExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, StringFunctionExpr)>,
        fallback: Box<StringFunctionExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<StringFunctionExpr>,
    },
}

impl StringFunctionExpr {
    pub(crate) fn constant(
        value: ConstantStringFunctionInstantiation,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: StringFunctionExprKind::Constant(value),
        }
    }

    pub(crate) fn reference(value: StringFunctionReference) -> Self {
        let type_ = value.instantiation().shape().type_();
        Self {
            type_,
            kind: StringFunctionExprKind::Reference(value),
        }
    }

    pub(crate) fn closure(
        function: FunctionInstantiation,
        captures: Vec<CaptureArg>,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: StringFunctionExprKind::Closure { function, captures },
        }
    }

    pub(crate) fn local_get(
        local: StringFunctionLocalId,
        name: EcoString,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_,
            kind: StringFunctionExprKind::LocalGet { local, name },
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
            kind: StringFunctionExprKind::Call {
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
            kind: StringFunctionExprKind::FunctionCall {
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
            kind: StringFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
                type_,
            },
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: StringFunctionExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(
        list: impl Into<FunctionListExpr>,
        index: usize,
        type_: FunctionType,
    ) -> Self {
        Self {
            type_: type_.clone(),
            kind: StringFunctionExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
                type_,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: FunctionType) -> Self {
        Self {
            type_,
            kind: StringFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: StringFunctionExpr,
        false_: StringFunctionExpr,
    ) -> Self {
        Self {
            type_: true_.type_.clone(),
            kind: StringFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: StringFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: StringFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    ) -> Self {
        Self {
            type_: fallback.type_.clone(),
            kind: StringFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: StringFunctionExpr) -> Self {
        Self {
            type_: return_.type_.clone(),
            kind: StringFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> &FunctionType {
        &self.type_
    }

    pub(crate) fn kind(&self) -> &StringFunctionExprKind {
        &self.kind
    }
}

#[cfg(test)]
mod tests {
    use super::{StringFunctionExpr, StringFunctionExprKind};
    use crate::plan::{
        BoolExpr, Expr, FunctionFunctionExpr, FunctionFunctionReference, FunctionInstantiation,
        FunctionShape, FunctionType, IntExpr, Step, StringExpr, StringFunctionLocalId,
        StringFunctionReference, ValueShape, ValueType, monomorphic_function_instantiation,
    };

    #[test]
    fn string_function_expr_kind_accessors() {
        assert_eq!(
            function_value().kind(),
            &StringFunctionExprKind::Reference(StringFunctionReference::new(
                function_instantiation()
            )),
        );
        assert_eq!(
            StringFunctionExpr::closure(function_instantiation(), Vec::new(), function_type(),)
                .kind(),
            &StringFunctionExprKind::Closure {
                function: function_instantiation(),
                captures: Vec::new(),
            },
        );
        assert_eq!(
            StringFunctionExpr::local_get(StringFunctionLocalId(0), "f".into(), function_type())
                .kind(),
            &StringFunctionExprKind::LocalGet {
                local: StringFunctionLocalId(0),
                name: "f".into(),
            },
        );
        assert_eq!(
            StringFunctionExpr::call(
                function_returning_function_instantiation(),
                Vec::new(),
                function_type(),
            )
            .kind(),
            &StringFunctionExprKind::Call {
                function: function_returning_function_instantiation(),
                args: Vec::new(),
                type_: function_type(),
                site: crate::plan::HostCallSite::unknown(),
            },
        );
        assert_eq!(
            StringFunctionExpr::function_call(
                function_function_value(),
                Vec::new(),
                function_type(),
            )
            .kind(),
            &StringFunctionExprKind::FunctionCall {
                function: Box::new(function_function_value()),
                args: Vec::new(),
                type_: function_type(),
                site: crate::plan::HostCallSite::unknown(),
            },
        );
        assert_eq!(
            StringFunctionExpr::tuple_index(tuple_expr(), 0, function_type()).kind(),
            &StringFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
                type_: function_type(),
            },
        );
        assert_eq!(
            StringFunctionExpr::bool_case(
                BoolExpr::value(true),
                function_value(),
                function_value(),
            )
            .kind(),
            &StringFunctionExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(function_value()),
                false_: Box::new(function_value()),
            },
        );
        assert_eq!(
            StringFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                function_value(),
            )
            .kind(),
            &StringFunctionExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            StringFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), function_value())],
                function_value(),
            )
            .kind(),
            &StringFunctionExprKind::StringCase {
                subject: Box::new(StringExpr::value("one".into())),
                clauses: vec![("one".into(), function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            StringFunctionExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, function_value())],
                function_value(),
            )
            .kind(),
            &StringFunctionExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, function_value())],
                fallback: Box::new(function_value()),
            },
        );
        assert_eq!(
            StringFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                function_value(),
            )
            .kind(),
            &StringFunctionExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                return_: Box::new(function_value()),
            },
        );
    }

    #[test]
    fn string_function_expr_type() {
        assert_eq!(function_value().type_(), &function_type());
    }

    fn function_value() -> StringFunctionExpr {
        StringFunctionExpr::reference(StringFunctionReference::new(function_instantiation()))
    }

    fn function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::String], ValueType::String)
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
            vec![Expr::function(crate::plan::FunctionExpr::string(
                function_value(),
            ))],
            vec![ValueType::Function(Box::new(function_type()))],
        )
    }
}
