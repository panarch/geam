use crate::plan::CustomFieldAccess;
use crate::plan::{
    BoolExpr, CaptureArg, ConstantFunctionFunctionInstantiation, FloatExpr, FunctionFunctionLocal,
    FunctionFunctionReference, FunctionFunctionType, FunctionInstantiation, FunctionListExpr,
    FunctionType, IntExpr, PanicExpr, Step, StringExpr, TupleExpr, ValueShape,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionFunctionExpr {
    type_: FunctionFunctionType,
    kind: FunctionFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FunctionFunctionCallMismatch {
    ArgumentCount { expected: usize, actual: usize },
    ReturnFamily,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionFunctionExprKind {
    Constant(ConstantFunctionFunctionInstantiation),
    Reference(FunctionFunctionReference),
    Closure {
        function: FunctionInstantiation,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: FunctionFunctionLocal,
        name: EcoString,
    },
    Call {
        function: FunctionInstantiation,
        args: Vec<crate::plan::CallArg>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: Box<FunctionFunctionExpr>,
        args: Vec<crate::plan::CallArg>,
        site: crate::plan::HostCallSite,
    },
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<FunctionListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<FunctionFunctionExprKind>,
        false_: Box<FunctionFunctionExprKind>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, FunctionFunctionExprKind)>,
        fallback: Box<FunctionFunctionExprKind>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, FunctionFunctionExprKind)>,
        fallback: Box<FunctionFunctionExprKind>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, FunctionFunctionExprKind)>,
        fallback: Box<FunctionFunctionExprKind>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<FunctionFunctionExprKind>,
    },
}

impl FunctionFunctionExpr {
    pub(crate) fn constant(
        value: ConstantFunctionFunctionInstantiation,
        type_: FunctionFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: FunctionFunctionExprKind::Constant(value),
        }
    }

    pub(crate) fn reference(value: FunctionFunctionReference, return_type: FunctionType) -> Self {
        let type_ = FunctionFunctionType::new(
            value
                .instantiation()
                .shape()
                .argument_shapes()
                .iter()
                .map(crate::plan::ValueShape::value_type)
                .collect(),
            return_type,
        );
        Self {
            type_,
            kind: FunctionFunctionExprKind::Reference(value),
        }
    }

    pub(crate) fn closure(
        function: FunctionInstantiation,
        captures: Vec<CaptureArg>,
        type_: FunctionFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: FunctionFunctionExprKind::Closure { function, captures },
        }
    }

    pub(crate) fn local_get(local: FunctionFunctionLocal, name: EcoString) -> Self {
        let type_ = local.type_().clone();
        Self {
            type_,
            kind: FunctionFunctionExprKind::LocalGet { local, name },
        }
    }

    #[cfg(test)]
    pub(crate) fn call(
        function: FunctionInstantiation,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionFunctionType,
    ) -> Self {
        Self::call_at(function, args, type_, crate::plan::HostCallSite::unknown())
    }

    pub(crate) fn call_at(
        function: FunctionInstantiation,
        args: Vec<crate::plan::CallArg>,
        type_: FunctionFunctionType,
        site: crate::plan::HostCallSite,
    ) -> Self {
        Self {
            type_,
            kind: FunctionFunctionExprKind::Call {
                function,
                args,
                site,
            },
        }
    }

    #[cfg(test)]
    pub(crate) fn try_function_call(
        function: FunctionFunctionExpr,
        args: Vec<crate::plan::CallArg>,
    ) -> Result<Self, FunctionFunctionCallMismatch> {
        Self::try_function_call_at(function, args, crate::plan::HostCallSite::unknown())
    }

    pub(crate) fn try_function_call_at(
        function: FunctionFunctionExpr,
        args: Vec<crate::plan::CallArg>,
        site: crate::plan::HostCallSite,
    ) -> Result<Self, FunctionFunctionCallMismatch> {
        let expected = function.function_function_type().argument_types().len();
        if expected != args.len() {
            return Err(FunctionFunctionCallMismatch::ArgumentCount {
                expected,
                actual: args.len(),
            });
        }

        let returned = function.function_function_type().return_shape();
        let ValueShape::Function(return_) = returned.return_shape() else {
            return Err(FunctionFunctionCallMismatch::ReturnFamily);
        };
        let type_ = FunctionFunctionType::from_shapes(
            returned.argument_shapes().to_vec(),
            return_.as_ref().clone(),
        );

        Ok(Self {
            type_,
            kind: FunctionFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                site,
            },
        })
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: FunctionFunctionType) -> Self {
        Self {
            type_,
            kind: FunctionFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess, type_: FunctionFunctionType) -> Self {
        Self {
            type_,
            kind: FunctionFunctionExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(
        list: impl Into<FunctionListExpr>,
        index: usize,
        type_: FunctionFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: FunctionFunctionExprKind::ListIndex {
                list: Box::new(list.into()),
                index,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: FunctionFunctionType) -> Self {
        Self {
            type_,
            kind: FunctionFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(
        subject: BoolExpr,
        true_: FunctionFunctionExpr,
        false_: FunctionFunctionExpr,
    ) -> Self {
        let (type_, true_) = true_.into_parts();
        let (_, false_) = false_.into_parts();
        Self {
            type_,
            kind: FunctionFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(
        subject: IntExpr,
        clauses: Vec<(BigInt, FunctionFunctionExpr)>,
        fallback: FunctionFunctionExpr,
    ) -> Self {
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| (pattern, branch.into_parts().1))
            .collect();
        let (type_, fallback) = fallback.into_parts();
        Self {
            type_,
            kind: FunctionFunctionExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, FunctionFunctionExpr)>,
        fallback: FunctionFunctionExpr,
    ) -> Self {
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| (pattern, branch.into_parts().1))
            .collect();
        let (type_, fallback) = fallback.into_parts();
        Self {
            type_,
            kind: FunctionFunctionExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, FunctionFunctionExpr)>,
        fallback: FunctionFunctionExpr,
    ) -> Self {
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| (pattern, branch.into_parts().1))
            .collect();
        let (type_, fallback) = fallback.into_parts();
        Self {
            type_,
            kind: FunctionFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: FunctionFunctionExpr) -> Self {
        let (type_, return_) = return_.into_parts();
        Self {
            type_,
            kind: FunctionFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> FunctionType {
        self.type_.to_function_type()
    }
    pub(crate) fn function_function_type(&self) -> &FunctionFunctionType {
        &self.type_
    }

    pub(crate) fn with_type(mut self, type_: FunctionFunctionType) -> Self {
        self.type_ = type_;
        self
    }

    pub(crate) fn kind(&self) -> &FunctionFunctionExprKind {
        &self.kind
    }

    pub(crate) fn into_parts(self) -> (FunctionFunctionType, FunctionFunctionExprKind) {
        (self.type_, self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionFunctionCallMismatch, FunctionFunctionExpr, FunctionFunctionExprKind};
    use crate::plan::{
        BoolExpr, CallArg, Expr, FunctionFunctionLocal, FunctionFunctionLocalId,
        FunctionFunctionReference, FunctionFunctionType, FunctionInstantiation, FunctionShape,
        FunctionType, IntExpr, Step, StringExpr, ValueShape, ValueType,
        monomorphic_function_instantiation,
    };

    #[test]
    fn function_function_expr_kind_accessors() {
        assert_eq!(
            function_value().kind(),
            &FunctionFunctionExprKind::Reference(FunctionFunctionReference::new(
                function_instantiation()
            )),
        );
        assert_eq!(
            FunctionFunctionExpr::closure(
                function_instantiation(),
                Vec::new(),
                function_function_type(),
            )
            .kind(),
            &FunctionFunctionExprKind::Closure {
                function: function_instantiation(),
                captures: Vec::new(),
            },
        );
        let local =
            FunctionFunctionLocal::new(FunctionFunctionLocalId(0), function_function_type());
        assert_eq!(
            FunctionFunctionExpr::local_get(local.clone(), "f".into()).kind(),
            &FunctionFunctionExprKind::LocalGet {
                local,
                name: "f".into(),
            },
        );
        let function = function_returning_function_instantiation();
        assert_eq!(
            FunctionFunctionExpr::call(function.clone(), Vec::new(), function_function_type(),)
                .kind(),
            &FunctionFunctionExprKind::Call {
                function,
                args: Vec::new(),
                site: crate::plan::HostCallSite::unknown(),
            },
        );
        assert_eq!(
            FunctionFunctionExpr::tuple_index(tuple_expr(), 0, function_function_type()).kind(),
            &FunctionFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple_expr()),
                index: 0,
            },
        );
        assert_eq!(
            FunctionFunctionExpr::bool_case(
                BoolExpr::value(true),
                function_value(),
                function_value(),
            )
            .kind(),
            &FunctionFunctionExprKind::BoolCase {
                subject: Box::new(BoolExpr::value(true)),
                true_: Box::new(function_value_kind()),
                false_: Box::new(function_value_kind()),
            },
        );
        assert_eq!(
            FunctionFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), function_value())],
                function_value(),
            )
            .kind(),
            &FunctionFunctionExprKind::IntCase {
                subject: Box::new(IntExpr::value(1.into())),
                clauses: vec![(1.into(), function_value_kind())],
                fallback: Box::new(function_value_kind()),
            },
        );
        assert_eq!(
            FunctionFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), function_value())],
                function_value(),
            )
            .kind(),
            &FunctionFunctionExprKind::StringCase {
                subject: Box::new(StringExpr::value("one".into())),
                clauses: vec![("one".into(), function_value_kind())],
                fallback: Box::new(function_value_kind()),
            },
        );
        assert_eq!(
            FunctionFunctionExpr::float_case(
                crate::plan::FloatExpr::value(1.0),
                vec![(1.0, function_value())],
                function_value(),
            )
            .kind(),
            &FunctionFunctionExprKind::FloatCase {
                subject: Box::new(crate::plan::FloatExpr::value(1.0)),
                clauses: vec![(1.0, function_value_kind())],
                fallback: Box::new(function_value_kind()),
            },
        );
        assert_eq!(
            FunctionFunctionExpr::block(
                vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                function_value(),
            )
            .kind(),
            &FunctionFunctionExprKind::Block {
                steps: vec![Step::evaluate(Expr::int(IntExpr::value(1.into())))],
                return_: Box::new(function_value_kind()),
            },
        );
    }

    #[test]
    fn function_function_expr_type() {
        assert_eq!(function_value().type_(), function_type());
        assert_eq!(
            FunctionFunctionExpr::bool_case(
                BoolExpr::value(true),
                function_value(),
                function_value(),
            )
            .type_(),
            function_type(),
        );
    }

    #[test]
    fn function_call_derives_return_type_and_checks_argument_count() {
        let function = function_call_callee();
        let argument = CallArg::new(crate::plan::Expr::int(IntExpr::value(1.into())));
        let expression =
            FunctionFunctionExpr::try_function_call(function.clone(), vec![argument.clone()])
                .expect("exact function call");

        assert_eq!(
            expression.into_parts(),
            (
                function_function_type(),
                FunctionFunctionExprKind::FunctionCall {
                    function: Box::new(function.clone()),
                    args: vec![argument.clone()],
                    site: crate::plan::HostCallSite::unknown(),
                },
            ),
        );
        assert_eq!(
            FunctionFunctionExpr::try_function_call(function, Vec::new()),
            Err(FunctionFunctionCallMismatch::ArgumentCount {
                expected: 1,
                actual: 0,
            }),
        );
        assert_eq!(
            FunctionFunctionExpr::try_function_call(function_value(), vec![argument]),
            Err(FunctionFunctionCallMismatch::ReturnFamily),
        );
    }

    fn function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(function_instantiation()),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        )
    }

    fn function_value_kind() -> FunctionFunctionExprKind {
        function_value().into_parts().1
    }

    fn function_call_callee() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(function_call_callee_instantiation()),
            function_type(),
        )
    }

    fn function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Int,
            ))),
        )
    }

    fn returned_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn function_function_type() -> FunctionFunctionType {
        FunctionFunctionType::new(vec![ValueType::Int], returned_function_type())
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

    fn function_call_callee_instantiation() -> FunctionInstantiation {
        monomorphic_function_instantiation(
            2,
            FunctionShape::new(
                vec![ValueShape::Int],
                ValueShape::Function(Box::new(FunctionShape::from_function_type(function_type()))),
            ),
        )
    }

    fn tuple_expr() -> crate::plan::TupleExpr {
        crate::plan::TupleExpr::value(
            vec![Expr::function(crate::plan::FunctionExpr::function(
                function_value(),
            ))],
            vec![ValueType::Function(Box::new(function_type()))],
        )
    }
}
