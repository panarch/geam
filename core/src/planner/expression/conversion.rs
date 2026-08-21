use crate::plan::{
    BitArrayExpr, BoolExpr, CustomExpr, Expr, ExprKind, ExternalExpr, FloatExpr, FunctionExpr,
    GenericExpr, IntExpr, ListExpr, NilExpr, StringExpr, TupleExpr, UtfCodepointExpr, ValueShape,
    ValueType,
};
use crate::planner::error::{InvalidExpressionType, InvalidTypedAstReason, PlanError};
use gleam_compiler_core::type_::Type;

pub(in crate::planner) fn expect_expression<Family>(expression: Expr) -> Result<Family, PlanError>
where
    Family: PlannedExpressionFamily,
{
    let actual = InvalidExpressionType::from_value_type(expression.value_type());
    Family::from_kind(expression.into_kind()).ok_or(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: Family::TYPE,
            actual,
        },
    })
}

pub(in crate::planner) fn refine_expression_shape(
    expression: Expr,
    shape: ValueShape,
) -> Result<Expr, PlanError> {
    refine_shape(expression, shape)
}

pub(in crate::planner) fn refine_value_shape(
    expression: ValueShape,
    shape: ValueShape,
) -> Result<ValueShape, PlanError> {
    refine_shape(expression, shape)
}

pub(in crate::planner) fn validate_expression_value_type(
    expected: &ValueType,
    actual: &ValueType,
) -> Result<(), PlanError> {
    expect_value_type_result(
        (expected == actual)
            .then_some(())
            .ok_or_else(|| (expected.clone(), actual.clone())),
        |types| types,
    )
}

pub(in crate::planner) fn validate_expression_shape_flow(
    source: &ValueShape,
    target: &ValueShape,
) -> Result<(), PlanError> {
    expect_shape_result(
        target.value_type(),
        source.value_type(),
        source.can_flow_to(target).then_some(()),
    )
}

pub(in crate::planner) fn value_type_from_gleam(
    type_: &Type,
    expected: InvalidExpressionType,
) -> Result<ValueType, PlanError> {
    ValueType::from_gleam(type_).ok_or(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::UnsupportedExpressionType { expected },
    })
}

pub(in crate::planner) fn expect_list_spread(
    elements: Result<crate::plan::ListSpreadElements, crate::plan::ListSpreadConstructionError>,
) -> Result<crate::plan::ListSpreadElements, PlanError> {
    match elements {
        Ok(elements) => Ok(elements),
        Err(crate::plan::ListSpreadConstructionError::ElementTypeMismatch(error)) => {
            expect_value_type_result(Err(error), |error| (error.expected, error.actual))
        }
        Err(crate::plan::ListSpreadConstructionError::EmptyPrefix) => {
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::InvalidExpressionShapeKind::ListSpreadEmptyPrefix,
                },
            })
        }
    }
}

pub(in crate::planner) fn expect_value_type_result<Value, Error>(
    result: Result<Value, Error>,
    types: impl FnOnce(Error) -> (ValueType, ValueType),
) -> Result<Value, PlanError> {
    result.map_err(|error| {
        let (expected, actual) = types(error);
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionValueTypeMismatch { expected, actual },
        }
    })
}

fn refine_shape<Value>(value: Value, shape: ValueShape) -> Result<Value::Output, PlanError>
where
    Value: PlannedShapeRefinement,
{
    let actual = value.value_type();
    let expected = shape.value_type();
    expect_shape_result(expected, actual, value.refine(shape))
}

fn expect_shape_result<Value>(
    expected: ValueType,
    actual: ValueType,
    value: Option<Value>,
) -> Result<Value, PlanError> {
    value.ok_or(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionShapeRefinement { expected, actual },
    })
}

mod sealed {
    pub trait ExpressionFamily {}
    pub trait ShapeRefinement {}
}

pub(in crate::planner) trait PlannedExpressionFamily:
    sealed::ExpressionFamily + Sized
{
    const TYPE: InvalidExpressionType;

    fn from_kind(kind: ExprKind) -> Option<Self>;
}

macro_rules! expression_family {
    ($type_:ty, $family:ident, $kind:ident) => {
        impl sealed::ExpressionFamily for $type_ {}

        impl PlannedExpressionFamily for $type_ {
            const TYPE: InvalidExpressionType = InvalidExpressionType::$family;

            fn from_kind(kind: ExprKind) -> Option<Self> {
                match kind {
                    ExprKind::$kind(expression) => Some(expression),
                    _ => None,
                }
            }
        }
    };
}

expression_family!(GenericExpr, TypeParameter, Generic);
expression_family!(IntExpr, Int, Int);
expression_family!(StringExpr, String, String);
expression_family!(BitArrayExpr, BitArray, BitArray);
expression_family!(UtfCodepointExpr, UtfCodepoint, UtfCodepoint);
expression_family!(CustomExpr, Custom, Custom);
expression_family!(ExternalExpr, External, External);
expression_family!(FloatExpr, Float, Float);
expression_family!(BoolExpr, Bool, Bool);
expression_family!(NilExpr, Nil, Nil);
expression_family!(TupleExpr, Tuple, Tuple);
expression_family!(ListExpr, List, List);
expression_family!(FunctionExpr, Function, Function);

trait PlannedShapeRefinement: sealed::ShapeRefinement + Sized {
    type Output;

    fn value_type(&self) -> ValueType;

    fn refine(self, shape: ValueShape) -> Option<Self::Output>;
}

impl sealed::ShapeRefinement for Expr {}

impl PlannedShapeRefinement for Expr {
    type Output = Self;

    fn value_type(&self) -> ValueType {
        self.value_type()
    }

    fn refine(self, shape: ValueShape) -> Option<Self::Output> {
        self.with_shape(shape)
    }
}

impl sealed::ShapeRefinement for ValueShape {}

impl PlannedShapeRefinement for ValueShape {
    type Output = Self;

    fn value_type(&self) -> ValueType {
        self.value_type()
    }

    fn refine(self, shape: ValueShape) -> Option<Self::Output> {
        ValueShape::refine(&self, &shape)
    }
}

#[cfg(test)]
mod tests {
    use super::{expect_expression, refine_expression_shape};
    use crate::plan::{
        BitArrayExpr, BitArrayLocalId, BoolExpr, CustomExpr, CustomLocal, CustomLocalId,
        CustomType, CustomTypeName, CustomValueShape, Expr, ExternalExpr, ExternalLocal,
        ExternalLocalId, ExternalType, ExternalTypeName, ExternalValueShape, FloatExpr,
        FunctionExpr, GenericExpr, GenericLocal, GenericLocalId, IntExpr, ListExpr, NilExpr,
        StringExpr, TupleExpr, TypeParameterId, UtfCodepointExpr, UtfCodepointLocalId, ValueShape,
        ValueType,
    };
    use crate::planner::error::{InvalidExpressionType, InvalidTypedAstReason, PlanError};

    fn mismatch(expected: InvalidExpressionType, actual: InvalidExpressionType) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType { expected, actual },
        }
    }

    #[test]
    fn converts_every_planned_expression_family() {
        let parameter = TypeParameterId(0);
        let custom = CustomType::new(
            CustomTypeName::new("app".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let custom_shape = CustomValueShape::any(custom);
        let external = ExternalType::new(
            ExternalTypeName::new("app".into(), "main".into(), "Token".into()),
            Vec::new(),
        );
        let external_shape = ExternalValueShape::any(external);
        let function: Expr =
            crate::planner::dsl::int_function_ref(0, Vec::<crate::plan::ParamLocal>::new()).into();

        assert!(
            expect_expression::<GenericExpr>(Expr::generic(GenericExpr::local_get(
                GenericLocal::new(GenericLocalId(0), parameter),
                "value".into(),
            )))
            .is_ok()
        );
        assert!(expect_expression::<IntExpr>(Expr::int(IntExpr::value(1.into()))).is_ok());
        assert!(
            expect_expression::<StringExpr>(Expr::string(StringExpr::value("a".into()))).is_ok()
        );
        assert!(
            expect_expression::<BitArrayExpr>(Expr::bit_array(BitArrayExpr::local_get(
                BitArrayLocalId(0),
                "bits".into(),
            )))
            .is_ok()
        );
        assert!(
            expect_expression::<UtfCodepointExpr>(Expr::utf_codepoint(
                UtfCodepointExpr::local_get(UtfCodepointLocalId(0), "codepoint".into()),
            ))
            .is_ok()
        );
        assert!(
            expect_expression::<CustomExpr>(Expr::custom(CustomExpr::local_get(
                CustomLocal::from_shape(CustomLocalId(0), custom_shape),
                "boxed".into(),
            )))
            .is_ok()
        );
        assert!(
            expect_expression::<ExternalExpr>(Expr::external(ExternalExpr::local_get(
                ExternalLocal::from_shape(ExternalLocalId(0), external_shape),
                "token".into(),
            )))
            .is_ok()
        );
        assert!(expect_expression::<FloatExpr>(Expr::float(FloatExpr::value(1.0))).is_ok());
        assert!(expect_expression::<BoolExpr>(Expr::bool(BoolExpr::value(true))).is_ok());
        assert!(expect_expression::<NilExpr>(Expr::nil(NilExpr::value())).is_ok());
        assert!(
            expect_expression::<TupleExpr>(
                crate::planner::dsl::tuple([crate::planner::dsl::int(1)]).into(),
            )
            .is_ok()
        );
        assert!(
            expect_expression::<ListExpr>(
                crate::planner::dsl::list([crate::planner::dsl::int(1)], ValueType::Int).into(),
            )
            .is_ok()
        );
        assert!(expect_expression::<FunctionExpr>(function).is_ok());
    }

    #[test]
    fn reports_family_and_shape_refinement_failures() {
        assert_eq!(
            expect_expression::<StringExpr>(Expr::int(IntExpr::value(1.into()))),
            Err(mismatch(
                InvalidExpressionType::String,
                InvalidExpressionType::Int
            )),
        );

        let expression: Expr =
            crate::planner::dsl::list([crate::planner::dsl::int(1)], ValueType::Int).into();
        assert_eq!(
            refine_expression_shape(expression, ValueShape::List(Box::new(ValueShape::String)),),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShapeRefinement {
                    expected: ValueType::List(Box::new(ValueType::String)),
                    actual: ValueType::List(Box::new(ValueType::Int)),
                },
            }),
        );
    }
}
