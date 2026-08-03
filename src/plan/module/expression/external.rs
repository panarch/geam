use super::{
    BoolExpr, CallArg, CustomFieldAccess, ExternalFunctionExpr, ExternalListExpr, FloatExpr,
    IntExpr, PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::{ExternalLocal, ExternalValueShape, FunctionInstantiation, Step};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct ExternalExpr {
    shape: ExternalValueShape,
    kind: ExternalExprKind,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ExternalArgumentCountMismatch {
    pub(crate) expected: usize,
    pub(crate) actual: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ExternalFunctionCall {
    function: Box<ExternalFunctionExpr>,
    arguments: Box<[CallArg]>,
    site: crate::plan::HostCallSite,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExternalExprKind {
    LocalGet {
        local: ExternalLocal,
        name: EcoString,
    },
    Call {
        function: FunctionInstantiation,
        args: Vec<CallArg>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall(ExternalFunctionCall),
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<ExternalListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<ExternalExpr>,
        false_: Box<ExternalExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, ExternalExpr)>,
        fallback: Box<ExternalExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, ExternalExpr)>,
        fallback: Box<ExternalExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, ExternalExpr)>,
        fallback: Box<ExternalExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<ExternalExpr>,
    },
}

impl ExternalExpr {
    pub(crate) fn local_get(local: ExternalLocal, name: EcoString) -> Self {
        Self::new(
            local.shape().clone(),
            ExternalExprKind::LocalGet { local, name },
        )
    }

    pub(crate) fn call_at(
        function: FunctionInstantiation,
        args: Vec<CallArg>,
        shape: ExternalValueShape,
        site: crate::plan::HostCallSite,
    ) -> Self {
        Self::new(
            shape,
            ExternalExprKind::Call {
                function,
                args,
                site,
            },
        )
    }

    pub(crate) fn try_function_call_at(
        function: ExternalFunctionExpr,
        args: Vec<CallArg>,
        site: crate::plan::HostCallSite,
    ) -> Result<Self, ExternalArgumentCountMismatch> {
        let expected = function.external_function_type().argument_shapes().len();
        if expected != args.len() {
            return Err(ExternalArgumentCountMismatch {
                expected,
                actual: args.len(),
            });
        }
        let shape = function.external_function_type().return_().clone();
        Ok(Self::new(
            shape,
            ExternalExprKind::FunctionCall(ExternalFunctionCall {
                function: Box::new(function),
                arguments: args.into_boxed_slice(),
                site,
            }),
        ))
    }

    pub(crate) fn tuple_index_shape(
        tuple: TupleExpr,
        index: usize,
        shape: ExternalValueShape,
    ) -> Self {
        Self::new(
            shape,
            ExternalExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        )
    }

    pub(crate) fn custom_field_shape(access: CustomFieldAccess, shape: ExternalValueShape) -> Self {
        Self::new(shape, ExternalExprKind::CustomField(access))
    }

    pub(crate) fn list_index_shape(
        list: ExternalListExpr,
        index: usize,
        shape: ExternalValueShape,
    ) -> Self {
        Self::new(
            shape,
            ExternalExprKind::ListIndex {
                list: Box::new(list),
                index,
            },
        )
    }

    pub(crate) fn panic_shape(panic: PanicExpr, shape: ExternalValueShape) -> Self {
        Self::new(shape, ExternalExprKind::Panic(panic))
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Self {
        Self::new(
            true_.shape.clone(),
            ExternalExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        )
    }

    pub(crate) fn int_case(subject: IntExpr, clauses: Vec<(BigInt, Self)>, fallback: Self) -> Self {
        Self::new(
            fallback.shape.clone(),
            ExternalExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        clauses: Vec<(EcoString, Self)>,
        fallback: Self,
    ) -> Self {
        Self::new(
            fallback.shape.clone(),
            ExternalExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(crate) fn float_case(
        subject: FloatExpr,
        clauses: Vec<(f64, Self)>,
        fallback: Self,
    ) -> Self {
        Self::new(
            fallback.shape.clone(),
            ExternalExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self::new(
            return_.shape.clone(),
            ExternalExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        )
    }

    pub fn type_(&self) -> &crate::plan::ExternalType {
        self.shape.type_()
    }

    pub(crate) fn shape(&self) -> &ExternalValueShape {
        &self.shape
    }

    pub(super) fn with_shape(mut self, shape: ExternalValueShape) -> Self {
        self.shape = shape;
        self
    }

    pub(crate) fn kind(&self) -> &ExternalExprKind {
        &self.kind
    }

    pub(crate) fn into_parts(self) -> (ExternalValueShape, ExternalExprKind) {
        (self.shape, self.kind)
    }

    fn new(shape: ExternalValueShape, kind: ExternalExprKind) -> Self {
        Self { shape, kind }
    }
}

impl ExternalFunctionCall {
    pub(crate) fn function(&self) -> &ExternalFunctionExpr {
        &self.function
    }

    pub(crate) fn arguments(&self) -> &[CallArg] {
        &self.arguments
    }

    pub(crate) fn site(&self) -> &crate::plan::HostCallSite {
        &self.site
    }
}

#[cfg(test)]
mod tests {
    use super::{ExternalArgumentCountMismatch, ExternalExpr};
    use crate::plan::{
        CallArg, Expr, ExternalFunctionExpr, ExternalFunctionReference, ExternalTypeName,
        ExternalValueShape, FunctionShape, IntExpr, ValueShape, monomorphic_function_instantiation,
    };

    #[test]
    fn external_function_call_derives_its_return_shape_and_checks_argument_count() {
        let return_ = ExternalValueShape::new(
            ExternalTypeName::new("geam".into(), "main".into(), "Resource".into()),
            Vec::new(),
        );
        let function = ExternalFunctionExpr::reference(
            ExternalFunctionReference::new(monomorphic_function_instantiation(
                0,
                FunctionShape::new(vec![ValueShape::Int], ValueShape::External(return_.clone())),
            )),
            return_.clone(),
        );
        let argument = CallArg::new(Expr::int(IntExpr::value(1.into())));

        let expression = ExternalExpr::try_function_call_at(
            function.clone(),
            vec![argument],
            crate::plan::HostCallSite::unknown(),
        )
        .expect("one argument should match the external function");

        assert_eq!(expression.shape(), &return_);
        assert_eq!(
            ExternalExpr::try_function_call_at(
                function,
                Vec::new(),
                crate::plan::HostCallSite::unknown(),
            ),
            Err(ExternalArgumentCountMismatch {
                expected: 1,
                actual: 0,
            }),
        );
    }
}
