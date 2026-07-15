use super::returning_function::FunctionFunctionCallMismatch;
use crate::plan::{
    BoolExpr, CaptureArg, CustomConstructor, CustomFieldAccess, CustomFunctionFunctionId,
    CustomFunctionId, CustomFunctionLocal, CustomFunctionReference, CustomFunctionType, CustomType,
    FloatExpr, FunctionFunctionExpr, FunctionListExpr, FunctionType, IntExpr, PanicExpr,
    ParamLocal, Step, StringExpr, TupleExpr,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomFunctionExpr {
    type_: CustomFunctionType,
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
        local: CustomFunctionLocal,
        name: EcoString,
    },
    Call {
        function: CustomFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
    },
    FunctionCall {
        function: Box<FunctionFunctionExpr>,
        args: Vec<crate::plan::CallArg>,
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
        true_: Box<CustomFunctionExprKind>,
        false_: Box<CustomFunctionExprKind>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, CustomFunctionExprKind)>,
        fallback: Box<CustomFunctionExprKind>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, CustomFunctionExprKind)>,
        fallback: Box<CustomFunctionExprKind>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, CustomFunctionExprKind)>,
        fallback: Box<CustomFunctionExprKind>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<CustomFunctionExprKind>,
    },
}

impl CustomFunctionExpr {
    pub(crate) fn constructor(constructor: CustomConstructor) -> Self {
        let type_ = CustomFunctionType::new(
            constructor
                .fields()
                .iter()
                .map(|field| field.type_().clone())
                .collect(),
            constructor.type_().clone(),
        );
        Self {
            type_,
            kind: CustomFunctionExprKind::Constructor(constructor),
        }
    }

    pub(crate) fn reference(value: CustomFunctionReference, return_type: CustomType) -> Self {
        let type_ = CustomFunctionType::new(
            value.params().iter().map(ParamLocal::value_type).collect(),
            return_type,
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
        type_: CustomFunctionType,
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

    pub(crate) fn local_get(local: CustomFunctionLocal, name: EcoString) -> Self {
        let type_ = local.type_().clone();
        Self {
            type_,
            kind: CustomFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: CustomFunctionFunctionId,
        args: Vec<crate::plan::CallArg>,
    ) -> Self {
        let type_ = function.type_().clone();
        Self {
            type_,
            kind: CustomFunctionExprKind::Call { function, args },
        }
    }

    pub(crate) fn try_function_call(
        function: FunctionFunctionExpr,
        args: Vec<crate::plan::CallArg>,
    ) -> Result<Self, FunctionFunctionCallMismatch> {
        let expected = function.function_function_type().argument_types().len();
        if expected != args.len() {
            return Err(FunctionFunctionCallMismatch::ArgumentCount {
                expected,
                actual: args.len(),
            });
        }

        let returned = function.function_function_type().return_();
        let crate::plan::ValueType::Custom(return_) = returned.return_() else {
            return Err(FunctionFunctionCallMismatch::ReturnFamily);
        };
        let type_ = CustomFunctionType::new(returned.argument_types().to_vec(), return_.clone());

        Ok(Self {
            type_,
            kind: CustomFunctionExprKind::FunctionCall {
                function: Box::new(function),
                args,
            },
        })
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: CustomFunctionType) -> Self {
        Self {
            type_,
            kind: CustomFunctionExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        }
    }

    pub(crate) fn custom_field(access: CustomFieldAccess, type_: CustomFunctionType) -> Self {
        Self {
            type_,
            kind: CustomFunctionExprKind::CustomField(access),
        }
    }

    pub(crate) fn list_index(
        list: FunctionListExpr,
        index: usize,
        type_: CustomFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: CustomFunctionExprKind::ListIndex {
                list: Box::new(list),
                index,
            },
        }
    }

    pub(crate) fn panic(panic: PanicExpr, type_: CustomFunctionType) -> Self {
        Self {
            type_,
            kind: CustomFunctionExprKind::Panic(panic),
        }
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Self {
        let (type_, true_) = true_.into_parts();
        let (_, false_) = false_.into_parts();
        Self {
            type_,
            kind: CustomFunctionExprKind::BoolCase {
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
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| (pattern, branch.into_parts().1))
            .collect();
        let (type_, fallback) = fallback.into_parts();
        Self {
            type_,
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
        let clauses = clauses
            .into_iter()
            .map(|(pattern, branch)| (pattern, branch.into_parts().1))
            .collect();
        let (type_, fallback) = fallback.into_parts();
        Self {
            type_,
            kind: CustomFunctionExprKind::FloatCase {
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
            kind: CustomFunctionExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        }
    }

    pub(crate) fn custom_function_type(&self) -> &CustomFunctionType {
        &self.type_
    }
    pub fn type_(&self) -> FunctionType {
        self.type_.to_function_type()
    }
    pub(crate) fn kind(&self) -> &CustomFunctionExprKind {
        &self.kind
    }
    pub(crate) fn into_parts(self) -> (CustomFunctionType, CustomFunctionExprKind) {
        (self.type_, self.kind)
    }
}

#[cfg(test)]
mod tests {
    use super::{CustomFunctionExpr, CustomFunctionExprKind};
    use crate::plan::{
        BoolExpr, CallArg, CustomFunctionFunctionId, CustomFunctionId, CustomFunctionReference,
        CustomFunctionType, CustomType, CustomTypeName, FunctionFunctionCallMismatch,
        FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionReference, FunctionType, IntExpr,
        IntFunctionFunctionId, IntLocalId, ParamLocal, ValueType,
    };

    #[test]
    fn same_result_children_store_only_callable_bodies() {
        let expression = CustomFunctionExpr::block(
            Vec::new(),
            CustomFunctionExpr::bool_case(
                BoolExpr::value(true),
                function_value(),
                function_value(),
            ),
        );

        assert_eq!(
            expression.into_parts(),
            (
                function_type(),
                CustomFunctionExprKind::Block {
                    steps: Vec::new(),
                    return_: Box::new(CustomFunctionExprKind::BoolCase {
                        subject: Box::new(BoolExpr::value(true)),
                        true_: Box::new(CustomFunctionExprKind::Reference(function_reference())),
                        false_: Box::new(CustomFunctionExprKind::Reference(function_reference())),
                    }),
                },
            ),
        );
    }

    #[test]
    fn function_call_derives_custom_type_and_checks_argument_count() {
        let function = function_call_callee();
        let argument = CallArg::int(IntLocalId(0), IntExpr::value(1.into()));
        let expression =
            CustomFunctionExpr::try_function_call(function.clone(), vec![argument.clone()])
                .expect("exact custom function call");

        assert_eq!(
            expression.into_parts(),
            (
                function_type(),
                CustomFunctionExprKind::FunctionCall {
                    function: Box::new(function.clone()),
                    args: vec![argument.clone()],
                },
            ),
        );
        assert_eq!(
            CustomFunctionExpr::try_function_call(function, Vec::new()),
            Err(FunctionFunctionCallMismatch::ArgumentCount {
                expected: 1,
                actual: 0,
            }),
        );
        assert_eq!(
            CustomFunctionExpr::try_function_call(wrong_return_family_callee(), vec![argument]),
            Err(FunctionFunctionCallMismatch::ReturnFamily),
        );
    }

    fn function_value() -> CustomFunctionExpr {
        CustomFunctionExpr::reference(function_reference(), custom_type())
    }

    fn function_reference() -> CustomFunctionReference {
        CustomFunctionReference::new(CustomFunctionId(0), Vec::new())
    }

    fn function_call_callee() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(
                FunctionFunctionId::Custom(CustomFunctionFunctionId::new(0, function_type())),
                vec![ParamLocal::int(IntLocalId(0))],
            ),
            function_type().to_function_type(),
        )
    }

    fn wrong_return_family_callee() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                vec![ParamLocal::int(IntLocalId(0))],
            ),
            FunctionType::new(Vec::new(), ValueType::Int),
        )
    }

    fn function_type() -> CustomFunctionType {
        CustomFunctionType::new(Vec::new(), custom_type())
    }

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }
}
