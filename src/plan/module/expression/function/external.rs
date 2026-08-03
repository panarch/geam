use super::returning_function::FunctionFunctionCallMismatch;
use crate::plan::{
    BoolExpr, CaptureArg, ConstantExternalFunctionInstantiation, CustomFieldAccess,
    ExternalFunctionLocal, ExternalFunctionReference, ExternalFunctionType, ExternalValueShape,
    FloatExpr, FunctionFunctionExpr, FunctionInstantiation, FunctionListExpr, FunctionType,
    IntExpr, PanicExpr, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct ExternalFunctionExpr {
    type_: ExternalFunctionType,
    kind: ExternalFunctionExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum ExternalFunctionExprKind {
    Constant(ConstantExternalFunctionInstantiation),
    Reference(ExternalFunctionReference),
    Closure {
        function: FunctionInstantiation,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: ExternalFunctionLocal,
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
        true_: Box<ExternalFunctionExprKind>,
        false_: Box<ExternalFunctionExprKind>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, ExternalFunctionExprKind)>,
        fallback: Box<ExternalFunctionExprKind>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, ExternalFunctionExprKind)>,
        fallback: Box<ExternalFunctionExprKind>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, ExternalFunctionExprKind)>,
        fallback: Box<ExternalFunctionExprKind>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<ExternalFunctionExprKind>,
    },
}

impl ExternalFunctionExpr {
    pub(crate) fn constant(
        value: ConstantExternalFunctionInstantiation,
        type_: ExternalFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: ExternalFunctionExprKind::Constant(value),
        }
    }

    pub(crate) fn reference(
        value: ExternalFunctionReference,
        return_shape: ExternalValueShape,
    ) -> Self {
        let type_ = ExternalFunctionType::from_shapes(
            value.instantiation().shape().argument_shapes().to_vec(),
            return_shape,
        );
        Self {
            type_,
            kind: ExternalFunctionExprKind::Reference(value),
        }
    }

    pub(crate) fn closure(
        function: FunctionInstantiation,
        captures: Vec<CaptureArg>,
        type_: ExternalFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: ExternalFunctionExprKind::Closure { function, captures },
        }
    }

    pub(crate) fn local_get(local: ExternalFunctionLocal, name: EcoString) -> Self {
        let type_ = local.type_().clone();
        Self {
            type_,
            kind: ExternalFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call_at(
        function: FunctionInstantiation,
        args: Vec<crate::plan::CallArg>,
        type_: ExternalFunctionType,
        site: crate::plan::HostCallSite,
    ) -> Self {
        Self {
            type_,
            kind: ExternalFunctionExprKind::Call {
                function,
                args,
                site,
            },
        }
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
        let crate::plan::ValueShape::External(return_) = returned.return_shape() else {
            return Err(FunctionFunctionCallMismatch::ReturnFamily);
        };
        let type_ =
            ExternalFunctionType::from_shapes(returned.argument_shapes().to_vec(), return_.clone());
        Ok(Self {
            type_,
            kind: ExternalFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
                site,
            },
        })
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: ExternalFunctionType) -> Self {
        Self {
            type_,
            kind: ExternalFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess, type_: ExternalFunctionType) -> Self {
        Self {
            type_,
            kind: ExternalFunctionExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(
        list: FunctionListExpr,
        index: usize,
        type_: ExternalFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: ExternalFunctionExprKind::ListIndex {
                list: Box::new(list),
                index,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: ExternalFunctionType) -> Self {
        Self {
            type_,
            kind: ExternalFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Self {
        let (type_, true_) = true_.into_parts();
        let (_, false_) = false_.into_parts();
        Self {
            type_,
            kind: ExternalFunctionExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        }
    }

    pub(crate) fn int_case(subject: IntExpr, clauses: Vec<(BigInt, Self)>, fallback: Self) -> Self {
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| (pattern, branch.into_parts().1))
            .collect();
        let (type_, fallback) = fallback.into_parts();
        Self {
            type_,
            kind: ExternalFunctionExprKind::IntCase {
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
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| (pattern, branch.into_parts().1))
            .collect();
        let (type_, fallback) = fallback.into_parts();
        Self {
            type_,
            kind: ExternalFunctionExprKind::StringCase {
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
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| (pattern, branch.into_parts().1))
            .collect();
        let (type_, fallback) = fallback.into_parts();
        Self {
            type_,
            kind: ExternalFunctionExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        }
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        let (type_, return_) = return_.into_parts();
        Self {
            type_,
            kind: ExternalFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub fn type_(&self) -> FunctionType {
        self.type_.to_function_type()
    }

    pub(crate) fn external_function_type(&self) -> &ExternalFunctionType {
        &self.type_
    }

    pub(super) fn with_type(mut self, type_: ExternalFunctionType) -> Self {
        self.type_ = type_;
        self
    }

    pub(crate) fn kind(&self) -> &ExternalFunctionExprKind {
        &self.kind
    }

    pub(crate) fn into_parts(self) -> (ExternalFunctionType, ExternalFunctionExprKind) {
        (self.type_, self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::ExternalFunctionExpr;
    use crate::plan::{
        CallArg, Expr, ExternalTypeName, ExternalValueShape, FunctionFunctionCallMismatch,
        FunctionFunctionExpr, FunctionFunctionReference, FunctionShape, FunctionType, IntExpr,
        ValueShape, ValueType, monomorphic_function_instantiation,
    };

    #[test]
    fn function_call_derives_external_callable_type_and_checks_its_boundaries() {
        let external = ExternalValueShape::new(
            ExternalTypeName::new("geam".into(), "main".into(), "Resource".into()),
            Vec::new(),
        );
        let returned = FunctionShape::new(Vec::new(), ValueShape::External(external.clone()));
        let returned_type =
            FunctionType::new(Vec::new(), ValueType::External(external.type_().clone()));
        let callee = FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(monomorphic_function_instantiation(
                0,
                FunctionShape::new(
                    vec![ValueShape::Int],
                    ValueShape::Function(Box::new(returned.clone())),
                ),
            )),
            returned_type.clone(),
        );
        let argument = CallArg::new(Expr::int(IntExpr::value(1.into())));

        let expression = ExternalFunctionExpr::try_function_call_at(
            callee.clone(),
            vec![argument.clone()],
            crate::plan::HostCallSite::unknown(),
        )
        .expect("one argument and an external-returning function should match");

        assert_eq!(expression.type_(), returned_type,);
        assert_eq!(
            ExternalFunctionExpr::try_function_call_at(
                callee,
                Vec::new(),
                crate::plan::HostCallSite::unknown(),
            ),
            Err(FunctionFunctionCallMismatch::ArgumentCount {
                expected: 1,
                actual: 0,
            }),
        );

        let wrong_return = FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(monomorphic_function_instantiation(
                1,
                FunctionShape::new(
                    vec![ValueShape::Int],
                    ValueShape::Function(Box::new(FunctionShape::new(Vec::new(), ValueShape::Int))),
                ),
            )),
            FunctionType::new(Vec::new(), ValueType::Int),
        );
        assert_eq!(
            ExternalFunctionExpr::try_function_call_at(
                wrong_return,
                vec![argument],
                crate::plan::HostCallSite::unknown(),
            ),
            Err(FunctionFunctionCallMismatch::ReturnFamily),
        );
    }
}
