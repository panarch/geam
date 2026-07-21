use super::{
    BoolExpr, CallArg, CustomFieldAccess, CustomFunctionExpr, CustomListExpr, FloatExpr, IntExpr,
    PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::module::constant::MaterializedConstantCustomConstruction;
use crate::plan::{
    ConstantCustomReference, CustomConstructor, CustomConstructorRefinement, CustomLocal,
    CustomLocalId, CustomType, CustomValueShape, FunctionInstantiation, Step, ValueShape,
};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomExpr {
    shape: CustomValueShape,
    kind: CustomExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomBoolCaseBranches {
    shape: CustomValueShape,
    true_: CustomExprKind,
    false_: CustomExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomCaseBranches<Pattern> {
    shape: CustomValueShape,
    clauses: Vec<(Pattern, CustomExprKind)>,
    fallback: CustomExprKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomLocalExpr {
    local: CustomLocal,
    value: CustomExpr,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct CustomArgumentCountMismatch {
    pub(crate) expected: usize,
    pub(crate) actual: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomConstruction {
    constructor: CustomConstructor,
    fields: Box<[super::Expr]>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomCallArguments {
    values: Box<[CallArg]>,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomFunctionCall {
    function: Box<CustomFunctionExpr>,
    arguments: CustomCallArguments,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum CustomExprKind {
    Constructor(CustomConstruction),
    Constant(ConstantCustomReference),
    LocalGet {
        local: CustomLocal,
        name: EcoString,
    },
    Call {
        function: FunctionInstantiation,
        args: Vec<CallArg>,
    },
    FunctionCall(CustomFunctionCall),
    TupleIndex {
        tuple: Box<TupleExpr>,
        index: usize,
    },
    CustomField(CustomFieldAccess),
    ListIndex {
        list: Box<CustomListExpr>,
        index: usize,
    },
    Panic(PanicExpr),
    BoolCase {
        subject: Box<BoolExpr>,
        true_: Box<CustomExprKind>,
        false_: Box<CustomExprKind>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, CustomExprKind)>,
        fallback: Box<CustomExprKind>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, CustomExprKind)>,
        fallback: Box<CustomExprKind>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, CustomExprKind)>,
        fallback: Box<CustomExprKind>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<CustomExprKind>,
    },
}

pub(crate) fn custom_constructor_expr(constructor: CustomConstructor) -> super::Expr {
    if constructor.fields().is_empty() {
        let shape = custom_constructor_shape(&constructor);
        super::Expr::custom(CustomExpr::new(
            shape,
            CustomExprKind::Constructor(CustomConstruction {
                constructor,
                fields: Vec::new().into_boxed_slice(),
            }),
        ))
    } else {
        super::Expr::function(super::FunctionExpr::custom(
            CustomFunctionExpr::constructor(constructor),
        ))
    }
}

impl CustomExpr {
    pub(in crate::plan::module) fn constant(reference: ConstantCustomReference) -> Self {
        let shape = reference.shape().clone();
        Self::new(shape, CustomExprKind::Constant(reference))
    }

    #[cfg(test)]
    pub(crate) fn try_constructor(
        constructor: CustomConstructor,
        fields: Vec<super::Expr>,
    ) -> Result<Self, CustomArgumentCountMismatch> {
        let shape = custom_constructor_shape(&constructor);
        CustomConstruction::try_new(constructor, fields)
            .map(|construction| Self::new(shape, CustomExprKind::Constructor(construction)))
    }

    pub(crate) fn from_construction(
        shape: CustomValueShape,
        construction: CustomConstruction,
    ) -> Self {
        Self::new(shape, CustomExprKind::Constructor(construction))
    }

    pub(crate) fn local_get(local: CustomLocal, name: EcoString) -> Self {
        Self::new(
            local.shape().clone(),
            CustomExprKind::LocalGet { local, name },
        )
    }

    pub(crate) fn call(
        function: FunctionInstantiation,
        args: Vec<CallArg>,
        shape: CustomValueShape,
    ) -> Self {
        Self::new(shape, CustomExprKind::Call { function, args })
    }

    pub(crate) fn try_function_call(
        function: CustomFunctionExpr,
        args: Vec<CallArg>,
    ) -> Result<Self, CustomArgumentCountMismatch> {
        let shape = function.custom_function_type().return_().clone();
        CustomFunctionCall::try_new(function, args)
            .map(|call| Self::new(shape, CustomExprKind::FunctionCall(call)))
    }

    pub(crate) fn tuple_index_shape(
        tuple: TupleExpr,
        index: usize,
        shape: CustomValueShape,
    ) -> Self {
        Self::new(
            shape,
            CustomExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        )
    }

    pub(crate) fn custom_field_shape(access: CustomFieldAccess, shape: CustomValueShape) -> Self {
        Self::new(shape, CustomExprKind::CustomField(access))
    }

    pub(crate) fn list_index_shape(
        list: CustomListExpr,
        index: usize,
        shape: CustomValueShape,
    ) -> Self {
        Self::new(
            shape,
            CustomExprKind::ListIndex {
                list: Box::new(list),
                index,
            },
        )
    }

    pub(crate) fn panic_shape(panic: PanicExpr, shape: CustomValueShape) -> Self {
        Self::new(shape, CustomExprKind::Panic(panic))
    }

    pub(crate) fn bool_case(subject: BoolExpr, branches: CustomBoolCaseBranches) -> Self {
        let (shape, true_, false_) = branches.into_parts();
        Self::new(
            shape,
            CustomExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        )
    }

    pub(crate) fn int_case(subject: IntExpr, branches: CustomCaseBranches<BigInt>) -> Self {
        let (shape, clauses, fallback) = branches.into_parts();
        Self::new(
            shape,
            CustomExprKind::IntCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(crate) fn string_case(
        subject: StringExpr,
        branches: CustomCaseBranches<EcoString>,
    ) -> Self {
        let (shape, clauses, fallback) = branches.into_parts();
        Self::new(
            shape,
            CustomExprKind::StringCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(crate) fn float_case(subject: FloatExpr, branches: CustomCaseBranches<f64>) -> Self {
        let (shape, clauses, fallback) = branches.into_parts();
        Self::new(
            shape,
            CustomExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        let (shape, return_) = return_.into_parts();
        Self::new(
            shape,
            CustomExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        )
    }

    pub fn type_(&self) -> &CustomType {
        self.shape.type_()
    }
    pub(crate) fn shape(&self) -> &CustomValueShape {
        &self.shape
    }

    pub(super) fn with_shape(mut self, shape: CustomValueShape) -> Self {
        self.shape = shape;
        self
    }
    pub(crate) fn kind(&self) -> &CustomExprKind {
        &self.kind
    }
    pub(crate) fn into_parts(self) -> (CustomValueShape, CustomExprKind) {
        (self.shape, self.kind)
    }

    fn new(shape: CustomValueShape, kind: CustomExprKind) -> Self {
        Self { shape, kind }
    }
}

impl CustomConstruction {
    pub(in crate::plan::module) fn from_constant(
        construction: MaterializedConstantCustomConstruction,
    ) -> Self {
        let (constructor, fields) = construction.into_parts();
        Self {
            constructor,
            fields,
        }
    }
}

impl CustomLocalExpr {
    pub(crate) fn from_value(local: CustomLocalId, value: CustomExpr) -> Self {
        let local = CustomLocal::from_shape(local, value.shape().clone());
        Self { local, value }
    }

    pub(crate) fn local(&self) -> &CustomLocal {
        &self.local
    }

    pub(crate) fn value(&self) -> &CustomExpr {
        &self.value
    }
}

impl CustomBoolCaseBranches {
    pub(crate) fn try_new(true_: CustomExpr, false_: CustomExpr) -> Option<Self> {
        let shape = true_.shape.merge(&false_.shape)?;
        Some(Self {
            shape,
            true_: true_.kind,
            false_: false_.kind,
        })
    }

    fn into_parts(self) -> (CustomValueShape, CustomExprKind, CustomExprKind) {
        (self.shape, self.true_, self.false_)
    }
}

impl<Pattern> CustomCaseBranches<Pattern> {
    pub(crate) fn try_new(
        clauses: Vec<(Pattern, CustomExpr)>,
        fallback: CustomExpr,
    ) -> Option<Self> {
        let mut shape = fallback.shape.clone();
        let mut bodies = Vec::with_capacity(clauses.len());
        for (pattern, branch) in clauses {
            shape = shape.merge(&branch.shape)?;
            bodies.push((pattern, branch.kind));
        }
        Some(Self {
            shape,
            clauses: bodies,
            fallback: fallback.kind,
        })
    }

    fn into_parts(
        self,
    ) -> (
        CustomValueShape,
        Vec<(Pattern, CustomExprKind)>,
        CustomExprKind,
    ) {
        (self.shape, self.clauses, self.fallback)
    }
}

pub(super) fn custom_constructor_shape(constructor: &CustomConstructor) -> CustomValueShape {
    CustomValueShape::new(
        constructor.type_().type_name().clone(),
        constructor
            .type_()
            .arguments()
            .iter()
            .cloned()
            .map(ValueShape::from_value_type)
            .collect(),
        CustomConstructorRefinement::Exact(constructor.index()),
    )
}

impl CustomConstruction {
    pub(crate) fn try_new(
        constructor: CustomConstructor,
        fields: Vec<super::Expr>,
    ) -> Result<Self, CustomArgumentCountMismatch> {
        if constructor.fields().len() != fields.len() {
            return Err(CustomArgumentCountMismatch {
                expected: constructor.fields().len(),
                actual: fields.len(),
            });
        }

        Ok(Self {
            constructor,
            fields: fields.into_boxed_slice(),
        })
    }

    pub(crate) fn fields(&self) -> &[super::Expr] {
        &self.fields
    }

    pub(crate) fn constructor(&self) -> &CustomConstructor {
        &self.constructor
    }
}

impl CustomFunctionCall {
    pub(crate) fn try_new(
        function: CustomFunctionExpr,
        arguments: Vec<CallArg>,
    ) -> Result<Self, CustomArgumentCountMismatch> {
        let expected = function.custom_function_type().argument_types().len();
        if expected != arguments.len() {
            return Err(CustomArgumentCountMismatch {
                expected,
                actual: arguments.len(),
            });
        }

        Ok(Self {
            function: Box::new(function),
            arguments: CustomCallArguments {
                values: arguments.into_boxed_slice(),
            },
        })
    }

    pub(crate) fn function(&self) -> &CustomFunctionExpr {
        &self.function
    }

    pub(crate) fn arguments(&self) -> &[CallArg] {
        &self.arguments.values
    }
}

#[cfg(test)]
mod tests {
    use super::{CustomArgumentCountMismatch, CustomExpr, CustomExprKind};
    use crate::plan::{
        CallArg, CustomConstructor, CustomConstructorField, CustomFunctionExpr, CustomType,
        CustomTypeName, Expr, IntExpr, ValueType,
    };

    #[test]
    fn custom_construction_owns_an_exact_field_pack() {
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let constructor = CustomConstructor::new(
            type_.clone(),
            "Boxed".into(),
            0,
            vec![CustomConstructorField::new(None, ValueType::Int)],
        );
        let field = Expr::int(IntExpr::value(1.into()));

        assert_eq!(
            CustomExpr::try_constructor(constructor.clone(), Vec::new()),
            Err(CustomArgumentCountMismatch {
                expected: 1,
                actual: 0,
            }),
        );
        assert_eq!(
            CustomExpr::try_constructor(constructor.clone(), vec![field.clone(), field.clone()]),
            Err(CustomArgumentCountMismatch {
                expected: 1,
                actual: 2,
            }),
        );

        let expression = CustomExpr::try_constructor(constructor.clone(), vec![field.clone()])
            .expect("exact custom construction should be valid");
        assert_eq!(expression.type_(), &type_);
        assert_eq!(
            expression.kind(),
            &CustomExprKind::Constructor(
                super::CustomConstruction::try_new(constructor, vec![field])
                    .expect("exact custom construction should be valid"),
            ),
        );
    }

    #[test]
    fn zero_field_custom_construction_preserves_an_empty_pack() {
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Empty".into()),
            Vec::new(),
        );
        let constructor = CustomConstructor::new(type_.clone(), "Empty".into(), 0, Vec::new());
        let expression = CustomExpr::try_constructor(constructor.clone(), Vec::new())
            .expect("zero-field custom construction should be valid");

        assert_eq!(expression.type_(), &type_);
        assert_eq!(
            expression.kind(),
            &CustomExprKind::Constructor(
                super::CustomConstruction::try_new(constructor, Vec::new())
                    .expect("zero-field custom construction should be valid"),
            ),
        );
    }

    #[test]
    fn custom_function_call_owns_an_exact_argument_pack() {
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let constructor = CustomConstructor::new(
            type_.clone(),
            "Boxed".into(),
            0,
            vec![CustomConstructorField::new(None, ValueType::Int)],
        );
        let function = CustomFunctionExpr::constructor(constructor);
        let argument = CallArg::new(crate::plan::Expr::int(IntExpr::value(1.into())));

        assert_eq!(
            CustomExpr::try_function_call(function.clone(), Vec::new()),
            Err(CustomArgumentCountMismatch {
                expected: 1,
                actual: 0,
            }),
        );
        assert_eq!(
            CustomExpr::try_function_call(
                function.clone(),
                vec![argument.clone(), argument.clone()],
            ),
            Err(CustomArgumentCountMismatch {
                expected: 1,
                actual: 2,
            }),
        );

        let expression = CustomExpr::try_function_call(function.clone(), vec![argument.clone()])
            .expect("exact custom function call should be valid");
        assert_eq!(expression.type_(), &type_);
        assert_eq!(
            expression.kind(),
            &CustomExprKind::FunctionCall(
                super::CustomFunctionCall::try_new(function, vec![argument])
                    .expect("exact custom function call should be valid"),
            ),
        );
    }

    #[test]
    fn custom_case_branch_owners_reject_incompatible_nominal_types() {
        let boxed = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let other = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Other".into()),
            Vec::new(),
        );
        let boxed = CustomExpr::try_constructor(
            CustomConstructor::new(boxed, "Boxed".into(), 0, Vec::new()),
            Vec::new(),
        )
        .expect("zero-field custom construction should be valid");
        let other = CustomExpr::try_constructor(
            CustomConstructor::new(other, "Other".into(), 0, Vec::new()),
            Vec::new(),
        )
        .expect("zero-field custom construction should be valid");

        assert_eq!(
            super::CustomBoolCaseBranches::try_new(boxed.clone(), other.clone()),
            None,
        );
        assert_eq!(
            super::CustomCaseBranches::try_new(vec![(1, boxed.clone()), (2, other)], boxed,),
            None,
        );
    }

    #[test]
    fn same_result_children_store_only_custom_bodies() {
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let constructor = CustomConstructor::new(type_.clone(), "Boxed".into(), 0, Vec::new());
        let branch = CustomExpr::try_constructor(constructor.clone(), Vec::new())
            .expect("zero-field custom construction should be valid");
        let shape = branch.shape().clone();
        let fallback = CustomExpr::try_constructor(constructor.clone(), Vec::new())
            .expect("zero-field custom construction should be valid");

        let expression = CustomExpr::block(
            Vec::new(),
            CustomExpr::bool_case(
                crate::plan::BoolExpr::value(true),
                super::CustomBoolCaseBranches::try_new(branch, fallback)
                    .expect("matching custom branches should be valid"),
            ),
        );

        assert_eq!(
            expression.into_parts(),
            (
                shape,
                CustomExprKind::Block {
                    steps: Vec::new(),
                    return_: Box::new(CustomExprKind::BoolCase {
                        subject: Box::new(crate::plan::BoolExpr::value(true)),
                        true_: Box::new(CustomExprKind::Constructor(
                            super::CustomConstruction::try_new(constructor.clone(), Vec::new())
                                .expect("zero-field custom construction should be valid"),
                        )),
                        false_: Box::new(CustomExprKind::Constructor(
                            super::CustomConstruction::try_new(constructor, Vec::new())
                                .expect("zero-field custom construction should be valid"),
                        )),
                    }),
                },
            ),
        );
    }
}
