use super::returning_function::FunctionFunctionCallMismatch;
#[cfg(test)]
use crate::plan::ParamLocal;
use crate::plan::{
    BoolExpr, CaptureArg, ConstantCustomFunctionInstantiation, CustomConstructor,
    CustomFieldAccess, CustomFunctionLocal, CustomFunctionReference, CustomFunctionType, FloatExpr,
    FunctionFunctionExpr, FunctionInstantiation, FunctionListExpr, FunctionType, IntExpr,
    PanicExpr, ParamSlot, Step, StringExpr, TupleExpr,
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
    Constant(ConstantCustomFunctionInstantiation),
    Constructor(CustomConstructor),
    Reference(CustomFunctionReference),
    Closure {
        function: FunctionInstantiation,
        params: Vec<ParamSlot>,
        captures: Vec<CaptureArg>,
    },
    LocalGet {
        local: CustomFunctionLocal,
        name: EcoString,
    },
    Call {
        function: FunctionInstantiation,
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
    pub(crate) fn constant(
        value: ConstantCustomFunctionInstantiation,
        type_: CustomFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: CustomFunctionExprKind::Constant(value),
        }
    }

    pub(crate) fn constructor(constructor: CustomConstructor) -> Self {
        let type_ = CustomFunctionType::from_shapes(
            constructor
                .fields()
                .iter()
                .map(|field| crate::plan::ValueShape::from_value_type(field.type_().clone()))
                .collect(),
            super::super::custom::custom_constructor_shape(&constructor),
        );
        Self {
            type_,
            kind: CustomFunctionExprKind::Constructor(constructor),
        }
    }

    pub(crate) fn reference(
        value: CustomFunctionReference,
        return_shape: crate::plan::CustomValueShape,
    ) -> Self {
        let type_ = CustomFunctionType::from_shapes(
            value.instantiation().shape().argument_shapes().to_vec(),
            return_shape,
        );
        Self {
            type_,
            kind: CustomFunctionExprKind::Reference(value),
        }
    }

    pub(crate) fn closure_slots(
        function: FunctionInstantiation,
        params: Vec<ParamSlot>,
        captures: Vec<CaptureArg>,
        type_: CustomFunctionType,
    ) -> Self {
        Self {
            type_,
            kind: CustomFunctionExprKind::Closure {
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
        type_: CustomFunctionType,
    ) -> Self {
        Self::closure_slots(
            function,
            params.into_iter().map(ParamSlot::from_local).collect(),
            captures,
            type_,
        )
    }

    pub(crate) fn local_get(local: CustomFunctionLocal, name: EcoString) -> Self {
        let type_ = local.type_().clone();
        Self {
            type_,
            kind: CustomFunctionExprKind::LocalGet { local, name },
        }
    }

    pub(crate) fn call(
        function: FunctionInstantiation,
        args: Vec<crate::plan::CallArg>,
        type_: CustomFunctionType,
    ) -> Self {
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

        let returned = function.function_function_type().return_shape();
        let crate::plan::ValueShape::Custom(return_) = returned.return_shape() else {
            return Err(FunctionFunctionCallMismatch::ReturnFamily);
        };
        let type_ =
            CustomFunctionType::from_shapes(returned.argument_shapes().to_vec(), return_.clone());

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

    pub(super) fn with_type(mut self, type_: CustomFunctionType) -> Self {
        self.type_ = type_;
        self
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
        BoolExpr, CallArg, CustomConstructorRefinement, CustomFunctionReference,
        CustomFunctionType, CustomLocalId, CustomType, CustomTypeName, CustomValueShape,
        FunctionExpr, FunctionFunctionCallMismatch, FunctionFunctionExpr,
        FunctionFunctionReference, FunctionInstantiation, FunctionShape, FunctionType, IntExpr,
        IntLocalId, ParamLocal, ValueShape, ValueType, monomorphic_function_instantiation,
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

    #[test]
    fn facade_shape_updates_the_custom_callable_owner() {
        let return_shape = CustomValueShape::new(
            custom_type().type_name().clone(),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        let requested_shape = FunctionShape::new(
            vec![ValueShape::Custom(return_shape.clone())],
            ValueShape::Custom(return_shape.clone()),
        );
        let expected_type = CustomFunctionType::from_shapes(
            requested_shape.argument_shapes().to_vec(),
            return_shape.clone(),
        );
        let function = CustomFunctionExpr::reference(
            CustomFunctionReference::new(
                custom_function_instantiation_with_param(),
                vec![ParamLocal::custom(CustomLocalId(0), custom_type())],
            ),
            return_shape.clone(),
        );

        assert_eq!(
            FunctionExpr::custom(function.clone()).with_resolved_shape(requested_shape),
            Some(FunctionExpr::custom(
                function.clone().with_type(expected_type.clone()),
            )),
        );
        assert_eq!(
            FunctionExpr::custom(function_value())
                .with_shape(FunctionShape::new(Vec::new(), ValueShape::Int)),
            None,
        );
        assert_eq!(
            FunctionExpr::custom(function.with_type(expected_type)).with_shape(FunctionShape::new(
                vec![ValueShape::Custom(CustomValueShape::any(custom_type()))],
                ValueShape::Custom(CustomValueShape::new(
                    custom_type().type_name().clone(),
                    Vec::new(),
                    CustomConstructorRefinement::Exact(1),
                )),
            ),),
            None,
        );
    }

    #[test]
    fn resolved_shape_assignment_does_not_repeat_function_variance() {
        let exact = CustomValueShape::new(
            custom_type().type_name().clone(),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        let shape = FunctionShape::new(
            vec![ValueShape::Custom(exact.clone())],
            ValueShape::Custom(exact.clone()),
        );
        let function = FunctionExpr::custom(CustomFunctionExpr::reference(
            CustomFunctionReference::new(
                custom_function_instantiation_with_param(),
                vec![ParamLocal::custom(CustomLocalId(0), custom_type())],
            ),
            exact.clone(),
        ));

        let resolved = function
            .clone()
            .with_resolved_shape(shape.clone())
            .expect("resolved shape has the same nominal function type");

        assert_eq!(resolved.shape(), &shape);
        assert_eq!(
            function.with_resolved_shape(FunctionShape::new(Vec::new(), ValueShape::Int)),
            None,
        );
    }

    #[test]
    fn block_preserves_the_custom_callable_shape() {
        let return_shape = CustomValueShape::new(
            custom_type().type_name().clone(),
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        let shape = FunctionShape::new(
            vec![ValueShape::Custom(CustomValueShape::any(custom_type()))],
            ValueShape::Custom(return_shape.clone()),
        );
        let function = CustomFunctionExpr::reference(
            CustomFunctionReference::new(
                custom_function_instantiation_with_param(),
                vec![ParamLocal::custom(CustomLocalId(0), custom_type())],
            ),
            return_shape.clone(),
        );
        let expression = FunctionExpr::custom(function)
            .with_resolved_shape(shape.clone())
            .expect("custom callable shape should match");

        let actual = FunctionExpr::block(Vec::new(), expression);

        assert_eq!(actual.shape(), &shape);
        assert_eq!(
            actual
                .into_custom()
                .map(|expression| expression.custom_function_type().clone()),
            Some(CustomFunctionType::from_shapes(
                shape.argument_shapes().to_vec(),
                return_shape,
            )),
        );
    }

    fn function_value() -> CustomFunctionExpr {
        CustomFunctionExpr::reference(function_reference(), custom_shape())
    }

    fn function_reference() -> CustomFunctionReference {
        CustomFunctionReference::new(custom_function_instantiation(), Vec::new())
    }

    fn function_call_callee() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(
                function_call_callee_instantiation(),
                vec![ParamLocal::int(IntLocalId(0))],
            ),
            function_type().to_function_type(),
        )
    }

    fn wrong_return_family_callee() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(
                wrong_return_family_instantiation(),
                vec![ParamLocal::int(IntLocalId(0))],
            ),
            FunctionType::new(Vec::new(), ValueType::Int),
        )
    }

    fn function_type() -> CustomFunctionType {
        CustomFunctionType::new(Vec::new(), custom_type())
    }

    fn custom_shape() -> CustomValueShape {
        CustomValueShape::any(custom_type())
    }

    fn custom_function_shape() -> FunctionShape {
        FunctionShape::new(Vec::new(), ValueShape::Custom(custom_shape()))
    }

    fn custom_function_instantiation() -> FunctionInstantiation {
        monomorphic_function_instantiation(0, custom_function_shape())
    }

    fn custom_function_instantiation_with_param() -> FunctionInstantiation {
        monomorphic_function_instantiation(
            0,
            FunctionShape::new(
                vec![ValueShape::Custom(custom_shape())],
                ValueShape::Custom(custom_shape()),
            ),
        )
    }

    fn function_call_callee_instantiation() -> FunctionInstantiation {
        monomorphic_function_instantiation(
            1,
            FunctionShape::new(
                vec![ValueShape::Int],
                ValueShape::Function(Box::new(custom_function_shape())),
            ),
        )
    }

    fn wrong_return_family_instantiation() -> FunctionInstantiation {
        monomorphic_function_instantiation(
            2,
            FunctionShape::new(
                vec![ValueShape::Int],
                ValueShape::Function(Box::new(FunctionShape::new(Vec::new(), ValueShape::Int))),
            ),
        )
    }

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }
}
