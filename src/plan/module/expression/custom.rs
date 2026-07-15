use super::{
    BoolExpr, CallArg, CustomFieldAccess, CustomFunctionExpr, CustomListExpr, FloatExpr, IntExpr,
    PanicExpr, StringExpr, TupleExpr,
};
use crate::plan::{CustomConstructor, CustomFunctionId, CustomLocalId, CustomType, Step};
use ecow::EcoString;
use num_bigint::BigInt;

#[derive(Debug, Clone, PartialEq)]
pub struct CustomExpr {
    type_: CustomType,
    kind: CustomExprKind,
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
    LocalGet {
        local: CustomLocalId,
        name: EcoString,
    },
    Call {
        function: CustomFunctionId,
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
        true_: Box<CustomExpr>,
        false_: Box<CustomExpr>,
    },
    IntCase {
        subject: Box<IntExpr>,
        clauses: Vec<(BigInt, CustomExpr)>,
        fallback: Box<CustomExpr>,
    },
    StringCase {
        subject: Box<StringExpr>,
        clauses: Vec<(EcoString, CustomExpr)>,
        fallback: Box<CustomExpr>,
    },
    FloatCase {
        subject: Box<FloatExpr>,
        clauses: Vec<(f64, CustomExpr)>,
        fallback: Box<CustomExpr>,
    },
    Block {
        steps: Vec<Step>,
        return_: Box<CustomExpr>,
    },
}

pub(crate) fn custom_constructor_expr(constructor: CustomConstructor) -> super::Expr {
    if constructor.fields().is_empty() {
        let type_ = constructor.type_().clone();
        super::Expr::custom(CustomExpr::new(
            type_,
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
    pub(crate) fn try_constructor(
        constructor: CustomConstructor,
        fields: Vec<super::Expr>,
    ) -> Result<Self, CustomArgumentCountMismatch> {
        let type_ = constructor.type_().clone();
        CustomConstruction::try_new(constructor, fields)
            .map(|construction| Self::new(type_, CustomExprKind::Constructor(construction)))
    }

    pub(crate) fn local_get(local: CustomLocalId, name: EcoString, type_: CustomType) -> Self {
        Self::new(type_, CustomExprKind::LocalGet { local, name })
    }

    pub(crate) fn call(function: CustomFunctionId, args: Vec<CallArg>, type_: CustomType) -> Self {
        Self::new(type_, CustomExprKind::Call { function, args })
    }

    pub(crate) fn try_function_call(
        function: CustomFunctionExpr,
        args: Vec<CallArg>,
    ) -> Result<Self, CustomArgumentCountMismatch> {
        let type_ = function.custom_function_type().return_().clone();
        CustomFunctionCall::try_new(function, args)
            .map(|call| Self::new(type_, CustomExprKind::FunctionCall(call)))
    }

    pub(crate) fn tuple_index(tuple: TupleExpr, index: usize, type_: CustomType) -> Self {
        Self::new(
            type_,
            CustomExprKind::TupleIndex {
                tuple: Box::new(tuple),
                index,
            },
        )
    }

    pub(crate) fn custom_field(access: CustomFieldAccess, type_: CustomType) -> Self {
        Self::new(type_, CustomExprKind::CustomField(access))
    }

    pub(crate) fn list_index(list: CustomListExpr, index: usize, type_: CustomType) -> Self {
        Self::new(
            type_,
            CustomExprKind::ListIndex {
                list: Box::new(list),
                index,
            },
        )
    }

    pub(crate) fn panic(panic: PanicExpr, type_: CustomType) -> Self {
        Self::new(type_, CustomExprKind::Panic(panic))
    }

    pub(crate) fn bool_case(subject: BoolExpr, true_: Self, false_: Self) -> Self {
        Self::new(
            true_.type_.clone(),
            CustomExprKind::BoolCase {
                subject: Box::new(subject),
                true_: Box::new(true_),
                false_: Box::new(false_),
            },
        )
    }

    pub(crate) fn int_case(subject: IntExpr, clauses: Vec<(BigInt, Self)>, fallback: Self) -> Self {
        Self::new(
            fallback.type_.clone(),
            CustomExprKind::IntCase {
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
            fallback.type_.clone(),
            CustomExprKind::StringCase {
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
            fallback.type_.clone(),
            CustomExprKind::FloatCase {
                subject: Box::new(subject),
                clauses,
                fallback: Box::new(fallback),
            },
        )
    }

    pub(crate) fn block(steps: Vec<Step>, return_: Self) -> Self {
        Self::new(
            return_.type_.clone(),
            CustomExprKind::Block {
                steps,
                return_: Box::new(return_),
            },
        )
    }

    pub fn type_(&self) -> &CustomType {
        &self.type_
    }
    pub(crate) fn kind(&self) -> &CustomExprKind {
        &self.kind
    }
    pub(crate) fn into_parts(self) -> (CustomType, CustomExprKind) {
        (self.type_, self.kind)
    }

    fn new(type_: CustomType, kind: CustomExprKind) -> Self {
        Self { type_, kind }
    }
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

    pub(crate) fn into_parts(self) -> (CustomConstructor, Box<[super::Expr]>) {
        (self.constructor, self.fields)
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

    pub(crate) fn into_parts(self) -> (CustomFunctionExpr, Box<[CallArg]>) {
        (*self.function, self.arguments.values)
    }
}

#[cfg(test)]
mod tests {
    use super::{CustomArgumentCountMismatch, CustomExpr, CustomExprKind};
    use crate::plan::{
        CallArg, CustomConstructor, CustomConstructorField, CustomFunctionExpr, CustomType,
        CustomTypeName, Expr, IntExpr, IntLocalId, ValueType,
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
        let argument = CallArg::int(IntLocalId(0), IntExpr::value(1.into()));

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
}
