use crate::plan::{
    BitArrayExpr, BitArrayFunctionExpr, BoolCaseBranches, BoolExpr, BoolFunctionExpr,
    BoolListCaseBranches, CustomBoolCaseBranches, CustomCaseBranches, CustomExpr,
    CustomFunctionExpr, Expr, ExprKind, ExternalExpr, ExternalFunctionExpr, FloatCaseBranches,
    FloatExpr, FloatFunctionExpr, FunctionExpr, FunctionExprKind, FunctionFunctionExpr,
    GenericExpr, GenericFunctionExpr, IntCaseBranches, IntExpr, IntFunctionExpr, ListCaseBranches,
    ListExpr, ListFunctionExpr, NilExpr, NilFunctionExpr, StringCaseBranches, StringExpr,
    StringFunctionExpr, TupleExpr, TupleFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
    ValueShape, ValueType,
};
use crate::planner::error::{InvalidCaseShapeReason, InvalidTypedAstReason, PlanError};
use ecow::EcoString;
use num_bigint::BigInt;

pub(super) fn validate_branch_type(expected: &ValueShape, branch: &Expr) -> Result<(), PlanError> {
    let expected = expected.value_type();
    let actual = branch.value_type();
    if expected == actual {
        return Ok(());
    }

    Err(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CaseShape {
            reason: InvalidCaseShapeReason::BranchAnnotatedTypeMismatch { expected, actual },
        },
    })
}

pub(super) fn bool_case_expr(
    subject: BoolExpr,
    true_: Expr,
    false_: Expr,
) -> Result<Expr, PlanError> {
    let true_shape = true_.value_shape().clone();
    let false_shape = false_.value_shape().clone();
    let expected = false_shape.value_type();
    let actual = true_shape.value_type();

    let expression = match false_.into_kind() {
        ExprKind::Generic(false_) => {
            let true_ = convert_branch::<GenericExpr>(true_, &expected)?;
            GenericExpr::bool_case(subject, true_, false_).map(Expr::generic)
        }
        ExprKind::Int(false_) => {
            let true_ = convert_branch::<IntExpr>(true_, &expected)?;
            Some(Expr::bool_case(
                subject,
                BoolCaseBranches::Int { true_, false_ },
            ))
        }
        ExprKind::String(false_) => {
            let true_ = convert_branch::<StringExpr>(true_, &expected)?;
            Some(Expr::bool_case(
                subject,
                BoolCaseBranches::String { true_, false_ },
            ))
        }
        ExprKind::BitArray(false_) => {
            let true_ = convert_branch::<BitArrayExpr>(true_, &expected)?;
            Some(Expr::bool_case(
                subject,
                BoolCaseBranches::BitArray { true_, false_ },
            ))
        }
        ExprKind::UtfCodepoint(false_) => {
            let true_ = convert_branch::<UtfCodepointExpr>(true_, &expected)?;
            Some(Expr::bool_case(
                subject,
                BoolCaseBranches::UtfCodepoint { true_, false_ },
            ))
        }
        ExprKind::Custom(false_) => {
            let true_ = convert_branch::<CustomExpr>(true_, &expected)?;
            let shape = branch_shape(
                true_.shape().merge(false_.shape()),
                expected.clone(),
                actual.clone(),
            )?;
            return Ok(Expr::bool_case(
                subject,
                BoolCaseBranches::Custom(CustomBoolCaseBranches::from_resolved_shape(
                    shape, true_, false_,
                )),
            ));
        }
        ExprKind::External(false_) => {
            let true_ = convert_branch::<ExternalExpr>(true_, &expected)?;
            let shape = branch_shape(
                true_.shape().merge(false_.shape()),
                expected.clone(),
                actual.clone(),
            )?;
            let expression = Expr::bool_case(subject, BoolCaseBranches::External { true_, false_ });
            let expression_type = expression.value_type();
            let shape = ValueShape::External(shape);
            let shape_type = shape.value_type();
            return family_assembly(
                expression.with_resolved_shape(shape),
                shape_type,
                expression_type,
            );
        }
        ExprKind::Float(false_) => {
            let true_ = convert_branch::<FloatExpr>(true_, &expected)?;
            Some(Expr::bool_case(
                subject,
                BoolCaseBranches::Float { true_, false_ },
            ))
        }
        ExprKind::Bool(false_) => {
            let true_ = convert_branch::<BoolExpr>(true_, &expected)?;
            Some(Expr::bool_case(
                subject,
                BoolCaseBranches::Bool { true_, false_ },
            ))
        }
        ExprKind::Nil(false_) => {
            let true_ = convert_branch::<NilExpr>(true_, &expected)?;
            Some(Expr::bool_case(
                subject,
                BoolCaseBranches::Nil { true_, false_ },
            ))
        }
        ExprKind::Tuple(false_) => {
            let true_ = convert_branch::<TupleExpr>(true_, &expected)?;
            Some(Expr::bool_case(
                subject,
                BoolCaseBranches::Tuple { true_, false_ },
            ))
        }
        ExprKind::List(false_) => {
            let true_ = convert_branch::<ListExpr>(true_, &expected)?;
            let branches = family_assembly(
                bool_list_case_branches(true_, false_),
                expected.clone(),
                actual.clone(),
            )?;
            Some(Expr::bool_case(subject, BoolCaseBranches::List(branches)))
        }
        ExprKind::Function(false_) => {
            bool_function_case_expr(subject, true_, false_, &expected, &actual)?
        }
    };

    let expression = family_assembly(expression, expected, actual)?;
    let shape = result_shape(std::slice::from_ref(&true_shape), &false_shape)?;
    let expression_type = expression.value_type();
    let shape_type = shape.value_type();
    family_assembly(
        expression.with_resolved_shape(shape),
        shape_type,
        expression_type,
    )
}

pub(super) fn int_case_expr(
    subject: IntExpr,
    clauses: Vec<(BigInt, Expr)>,
    fallback: Expr,
) -> Result<Expr, PlanError> {
    keyed_case_expr::<IntCaseAssembly>(subject, clauses, fallback)
}

pub(super) fn string_case_expr(
    subject: StringExpr,
    clauses: Vec<(EcoString, Expr)>,
    fallback: Expr,
) -> Result<Expr, PlanError> {
    keyed_case_expr::<StringCaseAssembly>(subject, clauses, fallback)
}

pub(super) fn float_case_expr(
    subject: FloatExpr,
    clauses: Vec<(f64, Expr)>,
    fallback: Expr,
) -> Result<Expr, PlanError> {
    keyed_case_expr::<FloatCaseAssembly>(subject, clauses, fallback)
}

fn keyed_case_expr<Assembly>(
    subject: Assembly::Subject,
    clauses: Vec<(Assembly::Key, Expr)>,
    fallback: Expr,
) -> Result<Expr, PlanError>
where
    Assembly: KeyedCaseAssembly,
{
    let clause_shapes = clauses
        .iter()
        .map(|(_, branch)| branch.value_shape().clone())
        .collect::<Vec<_>>();
    let fallback_shape = fallback.value_shape().clone();
    let expected = fallback_shape.value_type();
    let actual = clause_shapes
        .first()
        .map(ValueShape::value_type)
        .unwrap_or_else(|| expected.clone());
    let branches = keyed_case_branches(clauses, fallback, &expected)?;
    let expression = Assembly::assemble(subject, branches);
    let expression = family_assembly(expression, expected, actual)?;
    let shape = result_shape(&clause_shapes, &fallback_shape)?;
    let expression_type = expression.value_type();
    let shape_type = shape.value_type();
    family_assembly(
        expression.with_resolved_shape(shape),
        shape_type,
        expression_type,
    )
}

fn result_shape(branches: &[ValueShape], fallback: &ValueShape) -> Result<ValueShape, PlanError> {
    let mut shape = fallback.clone();
    for branch in branches {
        let expected = shape.value_type();
        let actual = branch.value_type();
        shape = branch_shape(branch.merge(&shape), expected, actual)?;
    }
    Ok(shape)
}

fn branch_shape<Value>(
    value: Option<Value>,
    expected: ValueType,
    actual: ValueType,
) -> Result<Value, PlanError> {
    value.ok_or(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::CaseShape {
            reason: InvalidCaseShapeReason::BranchShapeIncompatibility { expected, actual },
        },
    })
}

fn convert_branch<Family>(expression: Expr, expected: &ValueType) -> Result<Family, PlanError>
where
    Family: CaseResultFamily,
{
    let actual = expression.value_type();
    family_assembly(
        Family::from_expression(expression),
        expected.clone(),
        actual,
    )
}

fn convert_clauses<Key, Family>(
    clauses: Vec<(Key, Expr)>,
    expected: &ValueType,
) -> Result<Vec<(Key, Family)>, PlanError>
where
    Family: CaseResultFamily,
{
    clauses
        .into_iter()
        .map(|(key, expression)| convert_branch(expression, expected).map(|branch| (key, branch)))
        .collect()
}

fn family_assembly<Value>(
    value: Option<Value>,
    expected: ValueType,
    actual: ValueType,
) -> Result<Value, PlanError> {
    expect_family_assembly(value.ok_or((expected, actual)), |types| types)
}

fn expect_family_assembly<Value, Error>(
    value: Result<Value, Error>,
    types: impl FnOnce(Error) -> (ValueType, ValueType),
) -> Result<Value, PlanError> {
    value.map_err(|error| {
        let (expected, actual) = types(error);
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CaseShape {
                reason: InvalidCaseShapeReason::BranchFamilyAssemblyMismatch { expected, actual },
            },
        }
    })
}

mod sealed {
    pub trait CaseResultFamily {}
}

trait CaseResultFamily: sealed::CaseResultFamily + Sized {
    fn from_expression(expression: Expr) -> Option<Self>;
}

macro_rules! expression_result_family {
    ($type_:ty, $kind:ident) => {
        impl sealed::CaseResultFamily for $type_ {}

        impl CaseResultFamily for $type_ {
            fn from_expression(expression: Expr) -> Option<Self> {
                match expression.into_kind() {
                    ExprKind::$kind(expression) => Some(expression),
                    _ => None,
                }
            }
        }
    };
}

macro_rules! function_result_family {
    ($type_:ty, $kind:ident) => {
        impl sealed::CaseResultFamily for $type_ {}

        impl CaseResultFamily for $type_ {
            fn from_expression(expression: Expr) -> Option<Self> {
                let ExprKind::Function(expression) = expression.into_kind() else {
                    return None;
                };
                match expression.into_kind() {
                    FunctionExprKind::$kind(expression) => Some(expression),
                    _ => None,
                }
            }
        }
    };
}

expression_result_family!(GenericExpr, Generic);
expression_result_family!(IntExpr, Int);
expression_result_family!(StringExpr, String);
expression_result_family!(BitArrayExpr, BitArray);
expression_result_family!(UtfCodepointExpr, UtfCodepoint);
expression_result_family!(CustomExpr, Custom);
expression_result_family!(ExternalExpr, External);
expression_result_family!(FloatExpr, Float);
expression_result_family!(BoolExpr, Bool);
expression_result_family!(NilExpr, Nil);
expression_result_family!(TupleExpr, Tuple);
expression_result_family!(ListExpr, List);

function_result_family!(GenericFunctionExpr, Generic);
function_result_family!(IntFunctionExpr, Int);
function_result_family!(StringFunctionExpr, String);
function_result_family!(BitArrayFunctionExpr, BitArray);
function_result_family!(UtfCodepointFunctionExpr, UtfCodepoint);
function_result_family!(CustomFunctionExpr, Custom);
function_result_family!(ExternalFunctionExpr, External);
function_result_family!(FloatFunctionExpr, Float);
function_result_family!(BoolFunctionExpr, Bool);
function_result_family!(NilFunctionExpr, Nil);
function_result_family!(TupleFunctionExpr, Tuple);
function_result_family!(ListFunctionExpr, List);
function_result_family!(FunctionFunctionExpr, Function);

enum KeyedCaseBranches<Key> {
    Generic {
        clauses: Vec<(Key, GenericExpr)>,
        fallback: GenericExpr,
    },
    Int {
        clauses: Vec<(Key, IntExpr)>,
        fallback: IntExpr,
    },
    String {
        clauses: Vec<(Key, StringExpr)>,
        fallback: StringExpr,
    },
    BitArray {
        clauses: Vec<(Key, BitArrayExpr)>,
        fallback: BitArrayExpr,
    },
    UtfCodepoint {
        clauses: Vec<(Key, UtfCodepointExpr)>,
        fallback: UtfCodepointExpr,
    },
    Custom(CustomCaseBranches<Key>),
    External {
        clauses: Vec<(Key, ExternalExpr)>,
        fallback: ExternalExpr,
    },
    Float {
        clauses: Vec<(Key, FloatExpr)>,
        fallback: FloatExpr,
    },
    Bool {
        clauses: Vec<(Key, BoolExpr)>,
        fallback: BoolExpr,
    },
    Nil {
        clauses: Vec<(Key, NilExpr)>,
        fallback: NilExpr,
    },
    Tuple {
        clauses: Vec<(Key, TupleExpr)>,
        fallback: TupleExpr,
    },
    List(ListCaseBranches<Key>),
    GenericFunction {
        clauses: Vec<(Key, GenericFunctionExpr)>,
        fallback: GenericFunctionExpr,
    },
    IntFunction {
        clauses: Vec<(Key, IntFunctionExpr)>,
        fallback: IntFunctionExpr,
    },
    StringFunction {
        clauses: Vec<(Key, StringFunctionExpr)>,
        fallback: StringFunctionExpr,
    },
    BitArrayFunction {
        clauses: Vec<(Key, BitArrayFunctionExpr)>,
        fallback: BitArrayFunctionExpr,
    },
    UtfCodepointFunction {
        clauses: Vec<(Key, UtfCodepointFunctionExpr)>,
        fallback: UtfCodepointFunctionExpr,
    },
    CustomFunction {
        clauses: Vec<(Key, CustomFunctionExpr)>,
        fallback: CustomFunctionExpr,
    },
    ExternalFunction {
        clauses: Vec<(Key, ExternalFunctionExpr)>,
        fallback: ExternalFunctionExpr,
    },
    FloatFunction {
        clauses: Vec<(Key, FloatFunctionExpr)>,
        fallback: FloatFunctionExpr,
    },
    BoolFunction {
        clauses: Vec<(Key, BoolFunctionExpr)>,
        fallback: BoolFunctionExpr,
    },
    NilFunction {
        clauses: Vec<(Key, NilFunctionExpr)>,
        fallback: NilFunctionExpr,
    },
    TupleFunction {
        clauses: Vec<(Key, TupleFunctionExpr)>,
        fallback: TupleFunctionExpr,
    },
    ListFunction {
        clauses: Vec<(Key, ListFunctionExpr)>,
        fallback: ListFunctionExpr,
    },
    FunctionFunction {
        clauses: Vec<(Key, FunctionFunctionExpr)>,
        fallback: FunctionFunctionExpr,
    },
}

fn keyed_case_branches<Key>(
    clauses: Vec<(Key, Expr)>,
    fallback: Expr,
    expected: &ValueType,
) -> Result<KeyedCaseBranches<Key>, PlanError> {
    Ok(match fallback.into_kind() {
        ExprKind::Generic(fallback) => KeyedCaseBranches::Generic {
            clauses: convert_clauses(clauses, expected)?,
            fallback,
        },
        ExprKind::Int(fallback) => KeyedCaseBranches::Int {
            clauses: convert_clauses(clauses, expected)?,
            fallback,
        },
        ExprKind::String(fallback) => KeyedCaseBranches::String {
            clauses: convert_clauses(clauses, expected)?,
            fallback,
        },
        ExprKind::BitArray(fallback) => KeyedCaseBranches::BitArray {
            clauses: convert_clauses(clauses, expected)?,
            fallback,
        },
        ExprKind::UtfCodepoint(fallback) => KeyedCaseBranches::UtfCodepoint {
            clauses: convert_clauses(clauses, expected)?,
            fallback,
        },
        ExprKind::Custom(fallback) => {
            let clauses = convert_clauses(clauses, expected)?;
            let branches = custom_case_branches(clauses, fallback)?;
            KeyedCaseBranches::Custom(branches)
        }
        ExprKind::External(fallback) => KeyedCaseBranches::External {
            clauses: convert_clauses(clauses, expected)?,
            fallback,
        },
        ExprKind::Float(fallback) => KeyedCaseBranches::Float {
            clauses: convert_clauses(clauses, expected)?,
            fallback,
        },
        ExprKind::Bool(fallback) => KeyedCaseBranches::Bool {
            clauses: convert_clauses(clauses, expected)?,
            fallback,
        },
        ExprKind::Nil(fallback) => KeyedCaseBranches::Nil {
            clauses: convert_clauses(clauses, expected)?,
            fallback,
        },
        ExprKind::Tuple(fallback) => KeyedCaseBranches::Tuple {
            clauses: convert_clauses(clauses, expected)?,
            fallback,
        },
        ExprKind::List(fallback) => {
            let clauses = convert_clauses(clauses, expected)?;
            let branches =
                expect_family_assembly(ListCaseBranches::from_exprs(clauses, fallback), |error| {
                    (
                        ValueType::List(Box::new(error.expected)),
                        ValueType::List(Box::new(error.actual)),
                    )
                })?;
            KeyedCaseBranches::List(branches)
        }
        ExprKind::Function(fallback) => match fallback.into_kind() {
            FunctionExprKind::Generic(fallback) => KeyedCaseBranches::GenericFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
            FunctionExprKind::Int(fallback) => KeyedCaseBranches::IntFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
            FunctionExprKind::String(fallback) => KeyedCaseBranches::StringFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
            FunctionExprKind::BitArray(fallback) => KeyedCaseBranches::BitArrayFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
            FunctionExprKind::UtfCodepoint(fallback) => KeyedCaseBranches::UtfCodepointFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
            FunctionExprKind::Custom(fallback) => KeyedCaseBranches::CustomFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
            FunctionExprKind::External(fallback) => KeyedCaseBranches::ExternalFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
            FunctionExprKind::Float(fallback) => KeyedCaseBranches::FloatFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
            FunctionExprKind::Bool(fallback) => KeyedCaseBranches::BoolFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
            FunctionExprKind::Nil(fallback) => KeyedCaseBranches::NilFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
            FunctionExprKind::Tuple(fallback) => KeyedCaseBranches::TupleFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
            FunctionExprKind::List(fallback) => KeyedCaseBranches::ListFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
            FunctionExprKind::Function(fallback) => KeyedCaseBranches::FunctionFunction {
                clauses: convert_clauses(clauses, expected)?,
                fallback,
            },
        },
    })
}

fn custom_case_branches<Key>(
    clauses: Vec<(Key, CustomExpr)>,
    fallback: CustomExpr,
) -> Result<CustomCaseBranches<Key>, PlanError> {
    let mut shape = fallback.shape().clone();
    for (_, branch) in &clauses {
        let expected = ValueType::Custom(shape.type_().clone());
        let actual = ValueType::Custom(branch.type_().clone());
        shape = branch_shape(branch.shape().merge(&shape), expected, actual)?;
    }
    Ok(CustomCaseBranches::from_resolved_shape(
        shape, clauses, fallback,
    ))
}

trait KeyedCaseAssembly {
    type Key;
    type Subject;

    fn assemble(subject: Self::Subject, branches: KeyedCaseBranches<Self::Key>) -> Option<Expr>;
}

struct IntCaseAssembly;
struct StringCaseAssembly;
struct FloatCaseAssembly;

macro_rules! keyed_case_assembly {
    ($assembly:ty, $key:ty, $subject:ty, $branches:ident, $case:ident) => {
        impl KeyedCaseAssembly for $assembly {
            type Key = $key;
            type Subject = $subject;

            fn assemble(
                subject: Self::Subject,
                branches: KeyedCaseBranches<Self::Key>,
            ) -> Option<Expr> {
                match branches {
                    KeyedCaseBranches::Generic { clauses, fallback } => {
                        GenericExpr::$case(subject, clauses, fallback).map(Expr::generic)
                    }
                    KeyedCaseBranches::Int { clauses, fallback } => {
                        Some(Expr::$case(subject, $branches::Int { clauses, fallback }))
                    }
                    KeyedCaseBranches::String { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::String { clauses, fallback },
                    )),
                    KeyedCaseBranches::BitArray { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::BitArray { clauses, fallback },
                    )),
                    KeyedCaseBranches::UtfCodepoint { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::UtfCodepoint { clauses, fallback },
                    )),
                    KeyedCaseBranches::Custom(branches) => {
                        Some(Expr::$case(subject, $branches::Custom(branches)))
                    }
                    KeyedCaseBranches::External { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::External { clauses, fallback },
                    )),
                    KeyedCaseBranches::Float { clauses, fallback } => {
                        Some(Expr::$case(subject, $branches::Float { clauses, fallback }))
                    }
                    KeyedCaseBranches::Bool { clauses, fallback } => {
                        Some(Expr::$case(subject, $branches::Bool { clauses, fallback }))
                    }
                    KeyedCaseBranches::Nil { clauses, fallback } => {
                        Some(Expr::$case(subject, $branches::Nil { clauses, fallback }))
                    }
                    KeyedCaseBranches::Tuple { clauses, fallback } => {
                        Some(Expr::$case(subject, $branches::Tuple { clauses, fallback }))
                    }
                    KeyedCaseBranches::List(branches) => {
                        Some(Expr::$case(subject, $branches::List(branches)))
                    }
                    KeyedCaseBranches::GenericFunction { clauses, fallback } => {
                        GenericFunctionExpr::$case(subject, clauses, fallback)
                            .map(|expression| Expr::function(FunctionExpr::generic(expression)))
                    }
                    KeyedCaseBranches::IntFunction { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::IntFunction { clauses, fallback },
                    )),
                    KeyedCaseBranches::StringFunction { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::StringFunction { clauses, fallback },
                    )),
                    KeyedCaseBranches::BitArrayFunction { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::BitArrayFunction { clauses, fallback },
                    )),
                    KeyedCaseBranches::UtfCodepointFunction { clauses, fallback } => {
                        Some(Expr::$case(
                            subject,
                            $branches::UtfCodepointFunction { clauses, fallback },
                        ))
                    }
                    KeyedCaseBranches::CustomFunction { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::CustomFunction { clauses, fallback },
                    )),
                    KeyedCaseBranches::ExternalFunction { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::ExternalFunction { clauses, fallback },
                    )),
                    KeyedCaseBranches::FloatFunction { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::FloatFunction { clauses, fallback },
                    )),
                    KeyedCaseBranches::BoolFunction { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::BoolFunction { clauses, fallback },
                    )),
                    KeyedCaseBranches::NilFunction { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::NilFunction { clauses, fallback },
                    )),
                    KeyedCaseBranches::TupleFunction { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::TupleFunction { clauses, fallback },
                    )),
                    KeyedCaseBranches::ListFunction { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::ListFunction { clauses, fallback },
                    )),
                    KeyedCaseBranches::FunctionFunction { clauses, fallback } => Some(Expr::$case(
                        subject,
                        $branches::FunctionFunction { clauses, fallback },
                    )),
                }
            }
        }
    };
}

keyed_case_assembly!(IntCaseAssembly, BigInt, IntExpr, IntCaseBranches, int_case);
keyed_case_assembly!(
    StringCaseAssembly,
    EcoString,
    StringExpr,
    StringCaseBranches,
    string_case
);
keyed_case_assembly!(
    FloatCaseAssembly,
    f64,
    FloatExpr,
    FloatCaseBranches,
    float_case
);

fn bool_list_case_branches(true_: ListExpr, false_: ListExpr) -> Option<BoolListCaseBranches> {
    match (true_, false_) {
        (ListExpr::Generic(true_), ListExpr::Generic(false_)) if true_.item() == false_.item() => {
            Some(BoolListCaseBranches::Generic { true_, false_ })
        }
        (ListExpr::ParameterList(true_), ListExpr::ParameterList(false_))
            if true_.item() == false_.item() =>
        {
            Some(BoolListCaseBranches::ParameterList { true_, false_ })
        }
        (ListExpr::Int(true_), ListExpr::Int(false_)) => {
            Some(BoolListCaseBranches::Int { true_, false_ })
        }
        (ListExpr::String(true_), ListExpr::String(false_)) => {
            Some(BoolListCaseBranches::String { true_, false_ })
        }
        (ListExpr::BitArray(true_), ListExpr::BitArray(false_)) => {
            Some(BoolListCaseBranches::BitArray { true_, false_ })
        }
        (ListExpr::UtfCodepoint(true_), ListExpr::UtfCodepoint(false_)) => {
            Some(BoolListCaseBranches::UtfCodepoint { true_, false_ })
        }
        (ListExpr::Custom(true_), ListExpr::Custom(false_)) if true_.item() == false_.item() => {
            Some(BoolListCaseBranches::Custom { true_, false_ })
        }
        (ListExpr::External(true_), ListExpr::External(false_))
            if true_.item() == false_.item() =>
        {
            Some(BoolListCaseBranches::External { true_, false_ })
        }
        (ListExpr::Float(true_), ListExpr::Float(false_)) => {
            Some(BoolListCaseBranches::Float { true_, false_ })
        }
        (ListExpr::Bool(true_), ListExpr::Bool(false_)) => {
            Some(BoolListCaseBranches::Bool { true_, false_ })
        }
        (ListExpr::Nil(true_), ListExpr::Nil(false_)) => {
            Some(BoolListCaseBranches::Nil { true_, false_ })
        }
        (ListExpr::Tuple(true_), ListExpr::Tuple(false_)) if true_.item() == false_.item() => {
            Some(BoolListCaseBranches::Tuple { true_, false_ })
        }
        (ListExpr::List(true_), ListExpr::List(false_)) if true_.item() == false_.item() => {
            Some(BoolListCaseBranches::List { true_, false_ })
        }
        (ListExpr::Function(true_), ListExpr::Function(false_))
            if true_.item() == false_.item() =>
        {
            Some(BoolListCaseBranches::Function { true_, false_ })
        }
        _ => None,
    }
}

fn bool_function_case_expr(
    subject: BoolExpr,
    true_: Expr,
    false_: FunctionExpr,
    expected: &ValueType,
    actual: &ValueType,
) -> Result<Option<Expr>, PlanError> {
    let branches = match false_.into_kind() {
        FunctionExprKind::Generic(false_) => {
            let true_ = convert_branch::<GenericFunctionExpr>(true_, expected)?;
            return Ok(GenericFunctionExpr::bool_case(subject, true_, false_)
                .map(|expression| Expr::function(FunctionExpr::generic(expression))));
        }
        FunctionExprKind::Int(false_) => BoolCaseBranches::IntFunction {
            true_: convert_branch(true_, expected)?,
            false_,
        },
        FunctionExprKind::String(false_) => BoolCaseBranches::StringFunction {
            true_: convert_branch(true_, expected)?,
            false_,
        },
        FunctionExprKind::BitArray(false_) => BoolCaseBranches::BitArrayFunction {
            true_: convert_branch(true_, expected)?,
            false_,
        },
        FunctionExprKind::UtfCodepoint(false_) => BoolCaseBranches::UtfCodepointFunction {
            true_: convert_branch(true_, expected)?,
            false_,
        },
        FunctionExprKind::Custom(false_) => {
            let true_: CustomFunctionExpr = convert_branch(true_, expected)?;
            let branches = (true_.type_() == false_.type_())
                .then_some(BoolCaseBranches::CustomFunction { true_, false_ });
            family_assembly(branches, expected.clone(), actual.clone())?
        }
        FunctionExprKind::External(false_) => {
            let true_: ExternalFunctionExpr = convert_branch(true_, expected)?;
            let branches = (true_.type_() == false_.type_())
                .then_some(BoolCaseBranches::ExternalFunction { true_, false_ });
            family_assembly(branches, expected.clone(), actual.clone())?
        }
        FunctionExprKind::Float(false_) => BoolCaseBranches::FloatFunction {
            true_: convert_branch(true_, expected)?,
            false_,
        },
        FunctionExprKind::Bool(false_) => BoolCaseBranches::BoolFunction {
            true_: convert_branch(true_, expected)?,
            false_,
        },
        FunctionExprKind::Nil(false_) => BoolCaseBranches::NilFunction {
            true_: convert_branch(true_, expected)?,
            false_,
        },
        FunctionExprKind::Tuple(false_) => BoolCaseBranches::TupleFunction {
            true_: convert_branch(true_, expected)?,
            false_,
        },
        FunctionExprKind::List(false_) => BoolCaseBranches::ListFunction {
            true_: convert_branch(true_, expected)?,
            false_,
        },
        FunctionExprKind::Function(false_) => BoolCaseBranches::FunctionFunction {
            true_: convert_branch(true_, expected)?,
            false_,
        },
    };

    Ok(Some(Expr::bool_case(subject, branches)))
}

#[cfg(test)]
mod tests {
    use super::{
        bool_case_expr, float_case_expr, int_case_expr, string_case_expr, validate_branch_type,
    };
    use crate::plan::{
        BitArrayExpr, BitArrayFunctionExpr, BitArrayFunctionLocalId, BitArrayLocalId,
        BoolCaseBranches, BoolExpr, BoolFunctionExpr, BoolFunctionLocalId,
        CustomConstructorRefinement, CustomFunctionExpr, CustomFunctionLocal,
        CustomFunctionLocalId, CustomFunctionType, CustomType, CustomTypeName, CustomValueShape,
        Expr, ExternalExpr, ExternalFunctionExpr, ExternalFunctionLocal, ExternalFunctionLocalId,
        ExternalFunctionType, ExternalLocal, ExternalLocalId, ExternalType, ExternalTypeName,
        ExternalValueShape, FloatFunctionExpr, FloatFunctionLocalId, FunctionExpr,
        FunctionFunctionExpr, FunctionFunctionLocal, FunctionFunctionLocalId, FunctionFunctionType,
        FunctionShape, FunctionType, GenericExpr, GenericFunctionExpr, GenericFunctionLocal,
        GenericFunctionLocalId, GenericFunctionType, GenericLocal, GenericLocalId, IntFunctionExpr,
        IntFunctionId, IntFunctionLocalId, IntLocalId, ListFunctionExpr, ListFunctionLocal,
        LocalId, NilExpr, NilFunctionExpr, NilFunctionLocalId, RuntimeFunctionId,
        StringFunctionExpr, StringFunctionLocalId, TupleFunctionExpr, TupleFunctionLocalId,
        TypeParameterId, UtfCodepointExpr, UtfCodepointFunctionExpr, UtfCodepointFunctionLocalId,
        UtfCodepointLocalId, ValueShape, ValueType,
    };
    use crate::planner::dsl::{float, function_ref, int, list, string};
    use crate::planner::{InvalidCaseShapeReason, InvalidTypedAstReason, PlanError};
    use num_bigint::BigInt;

    #[test]
    fn reject_branch_annotation_mismatch_at_result_entry() {
        assert_eq!(
            validate_branch_type(&ValueShape::String, &Expr::from(int(1))),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchAnnotatedTypeMismatch {
                        expected: ValueType::String,
                        actual: ValueType::Int,
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_incompatible_branch_shapes_before_family_assembly() {
        let first_shape = refined_custom_shape(0);
        let second_shape = refined_custom_shape(1);
        let type_ = first_shape.type_().clone();
        let first = custom_local_with_shape(first_shape, 0);
        let second = custom_local_with_shape(second_shape, 1);
        assert_eq!(
            bool_case_expr(BoolExpr::value(true), first, second),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchShapeIncompatibility {
                        expected: ValueType::Custom(type_.clone()),
                        actual: ValueType::Custom(type_),
                    },
                },
            }),
        );

        assert_eq!(
            float_case_expr(
                float(1.0).into(),
                vec![(1.0, Expr::from(list([string("wrong")], ValueType::String)))],
                Expr::from(list([int(0)], ValueType::Int)),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchFamilyAssemblyMismatch {
                        expected: ValueType::List(Box::new(ValueType::Int)),
                        actual: ValueType::List(Box::new(ValueType::String)),
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_family_assembly_after_shapes_merge() {
        let malformed = custom_type("Malformed");
        let malformed_function = |id| {
            let function = int_function(id)
                .into_function()
                .expect("test expression is function-valued")
                .into_int()
                .expect("test expression returns Int");
            Expr::function(FunctionExpr::int_with_shape(
                function,
                FunctionShape::new(
                    vec![ValueShape::Int],
                    ValueShape::Custom(CustomValueShape::any(malformed.clone())),
                ),
            ))
        };
        let function_type = ValueType::Function(Box::new(FunctionType::new(
            vec![ValueType::Int],
            ValueType::Custom(malformed.clone()),
        )));

        assert_eq!(
            bool_case_expr(
                BoolExpr::value(true),
                malformed_function(0),
                malformed_function(1),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchFamilyAssemblyMismatch {
                        expected: function_type,
                        actual: ValueType::Function(Box::new(FunctionType::new(
                            vec![ValueType::Int],
                            ValueType::Int,
                        ))),
                    },
                },
            }),
        );

        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let function_value_type = ValueType::Function(Box::new(function_type.clone()));
        assert_eq!(
            bool_case_expr(BoolExpr::value(true), int(1).into(), int_function(1)),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchFamilyAssemblyMismatch {
                        expected: function_value_type.clone(),
                        actual: ValueType::Int,
                    },
                },
            }),
        );

        let malformed_string_function = Expr::function(FunctionExpr::string_with_shape(
            StringFunctionExpr::local_get(
                StringFunctionLocalId(0),
                "malformed".into(),
                FunctionType::new(vec![ValueType::Int], ValueType::String),
            ),
            FunctionShape::new(vec![ValueShape::Int], ValueShape::Int),
        ));
        assert_eq!(
            bool_case_expr(
                BoolExpr::value(true),
                malformed_string_function,
                int_function(1),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchFamilyAssemblyMismatch {
                        expected: function_value_type.clone(),
                        actual: function_value_type,
                    },
                },
            }),
        );
    }

    #[test]
    fn bool_case_preserves_external_nominal_shape() {
        let shape = external_shape();
        let true_ = ExternalExpr::local_get(
            ExternalLocal::from_shape(ExternalLocalId(0), shape.clone()),
            "true".into(),
        );
        let false_ = ExternalExpr::local_get(
            ExternalLocal::from_shape(ExternalLocalId(1), shape.clone()),
            "false".into(),
        );

        assert_eq!(
            bool_case_expr(
                BoolExpr::value(true),
                Expr::external(true_.clone()),
                Expr::external(false_.clone()),
            ),
            Ok(Expr::bool_case(
                BoolExpr::value(true),
                BoolCaseBranches::External {
                    true_: true_.clone(),
                    false_: false_.clone(),
                },
            )),
        );

        let item_type = ValueType::External(shape.type_().clone());
        let true_list = Expr::from(list([Expr::external(true_)], item_type.clone()));
        let false_list = Expr::from(list([Expr::external(false_)], item_type));
        assert!(
            bool_case_expr(BoolExpr::value(true), true_list, false_list).is_ok(),
            "matching external list items form a Bool case",
        );

        let first_shape = refined_external_shape(0);
        let second_shape = refined_external_shape(1);
        let type_ = first_shape.type_().clone();
        let first = ExternalExpr::local_get(
            ExternalLocal::from_shape(ExternalLocalId(2), first_shape),
            "first".into(),
        );
        let second = ExternalExpr::local_get(
            ExternalLocal::from_shape(ExternalLocalId(3), second_shape),
            "second".into(),
        );
        assert_eq!(
            bool_case_expr(
                BoolExpr::value(true),
                Expr::external(first),
                Expr::external(second),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchShapeIncompatibility {
                        expected: ValueType::External(type_.clone()),
                        actual: ValueType::External(type_),
                    },
                },
            }),
        );
    }

    #[test]
    fn bool_case_rejects_incompatible_list_result_families() {
        let branch = Expr::from(list([string("wrong")], ValueType::String));
        let fallback = Expr::from(list([int(0)], ValueType::Int));
        let error = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CaseShape {
                reason: InvalidCaseShapeReason::BranchFamilyAssemblyMismatch {
                    expected: ValueType::List(Box::new(ValueType::Int)),
                    actual: ValueType::List(Box::new(ValueType::String)),
                },
            },
        };

        assert_eq!(
            bool_case_expr(BoolExpr::value(true), branch.clone(), fallback.clone()),
            Err(error.clone()),
        );
        assert_eq!(
            int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), branch.clone())],
                fallback.clone(),
            ),
            Err(error.clone()),
        );
        assert_eq!(
            string_case_expr(
                string("one").into(),
                vec![("one".into(), branch.clone())],
                fallback.clone(),
            ),
            Err(error.clone()),
        );
        assert_eq!(
            float_case_expr(float(1.0).into(), vec![(1.0, branch)], fallback,),
            Err(error),
        );
    }

    #[test]
    fn rejects_generic_and_refined_shape_assembly_failures() {
        let generic_error = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CaseShape {
                reason: InvalidCaseShapeReason::BranchFamilyAssemblyMismatch {
                    expected: ValueType::Parameter(TypeParameterId(1)),
                    actual: ValueType::Parameter(TypeParameterId(0)),
                },
            },
        };
        let generic_branch = Expr::generic(generic(0, 0));
        let generic_fallback = Expr::generic(generic(1, 1));

        assert_eq!(
            bool_case_expr(
                BoolExpr::value(true),
                generic_branch.clone(),
                generic_fallback.clone(),
            ),
            Err(generic_error.clone()),
        );
        assert_eq!(
            int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), generic_branch.clone())],
                generic_fallback.clone(),
            ),
            Err(generic_error.clone()),
        );
        assert_eq!(
            string_case_expr(
                string("one").into(),
                vec![("one".into(), generic_branch.clone())],
                generic_fallback.clone(),
            ),
            Err(generic_error.clone()),
        );
        assert_eq!(
            float_case_expr(
                float(1.0).into(),
                vec![(1.0, generic_branch)],
                generic_fallback,
            ),
            Err(generic_error),
        );

        let first_shape = refined_custom_shape(0);
        let second_shape = refined_custom_shape(1);
        let type_ = first_shape.type_().clone();
        let shape_error = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CaseShape {
                reason: InvalidCaseShapeReason::BranchShapeIncompatibility {
                    expected: ValueType::Tuple(vec![ValueType::Custom(type_.clone())]),
                    actual: ValueType::Tuple(vec![ValueType::Custom(type_)]),
                },
            },
        };
        let branch: Expr =
            crate::planner::dsl::tuple([custom_local_with_shape(first_shape, 0)]).into();
        let fallback: Expr =
            crate::planner::dsl::tuple([custom_local_with_shape(second_shape, 1)]).into();

        assert_eq!(
            bool_case_expr(BoolExpr::value(true), branch.clone(), fallback.clone()),
            Err(shape_error.clone()),
        );
        assert_eq!(
            int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), branch.clone())],
                fallback.clone(),
            ),
            Err(shape_error.clone()),
        );
        assert_eq!(
            string_case_expr(
                string("one").into(),
                vec![("one".into(), branch.clone())],
                fallback.clone(),
            ),
            Err(shape_error.clone()),
        );
        assert_eq!(
            float_case_expr(float(1.0).into(), vec![(1.0, branch)], fallback),
            Err(shape_error),
        );
    }

    #[test]
    fn rejects_keyed_custom_shape_and_bool_function_nominal_mismatches() {
        let first_shape = refined_custom_shape(0);
        let second_shape = refined_custom_shape(1);
        let type_ = first_shape.type_().clone();
        let branch = custom_local_with_shape(first_shape, 0);
        let fallback = custom_local_with_shape(second_shape, 1);
        let custom_shape_error = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CaseShape {
                reason: InvalidCaseShapeReason::BranchShapeIncompatibility {
                    expected: ValueType::Custom(type_.clone()),
                    actual: ValueType::Custom(type_),
                },
            },
        };

        assert_eq!(
            int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), branch.clone())],
                fallback.clone(),
            ),
            Err(custom_shape_error.clone()),
        );
        assert_eq!(
            string_case_expr(
                string("one").into(),
                vec![("one".into(), branch.clone())],
                fallback.clone(),
            ),
            Err(custom_shape_error.clone()),
        );
        assert_eq!(
            float_case_expr(float(1.0).into(), vec![(1.0, branch)], fallback),
            Err(custom_shape_error),
        );

        let first_custom = custom_type("First");
        let second_custom = custom_type("Second");
        let custom_branch = Expr::function(FunctionExpr::custom(CustomFunctionExpr::local_get(
            CustomFunctionLocal::new(
                CustomFunctionLocalId(0),
                CustomFunctionType::new(Vec::new(), first_custom),
            ),
            "first".into(),
        )));
        let custom_fallback = Expr::function(FunctionExpr::custom(CustomFunctionExpr::local_get(
            CustomFunctionLocal::new(
                CustomFunctionLocalId(1),
                CustomFunctionType::new(Vec::new(), second_custom),
            ),
            "second".into(),
        )));
        assert_nominal_function_mismatch(custom_branch, custom_fallback);

        let external_type = |name: &str| {
            ExternalType::new(
                ExternalTypeName::new("geam".into(), "main".into(), name.into()),
                Vec::new(),
            )
        };
        let external_branch =
            Expr::function(FunctionExpr::external(ExternalFunctionExpr::local_get(
                ExternalFunctionLocal::new(
                    ExternalFunctionLocalId(0),
                    ExternalFunctionType::from_shapes(
                        Vec::new(),
                        ExternalValueShape::any(external_type("First")),
                    ),
                ),
                "first".into(),
            )));
        let external_fallback =
            Expr::function(FunctionExpr::external(ExternalFunctionExpr::local_get(
                ExternalFunctionLocal::new(
                    ExternalFunctionLocalId(1),
                    ExternalFunctionType::from_shapes(
                        Vec::new(),
                        ExternalValueShape::any(external_type("Second")),
                    ),
                ),
                "second".into(),
            )));
        assert_nominal_function_mismatch(external_branch, external_fallback);
    }

    #[test]
    fn assembles_every_result_family_for_bool_and_keyed_subjects() {
        let branches = case_result_families(0);
        let fallbacks = case_result_families(1);

        for (branch, fallback) in branches.into_iter().zip(fallbacks) {
            let expected = fallback.value_type();

            let bool_result =
                bool_case_expr(BoolExpr::value(true), branch.clone(), fallback.clone())
                    .expect("matching result families form a Bool case");
            assert_eq!(bool_result.value_type(), expected);

            let int_result = int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), branch.clone())],
                fallback.clone(),
            )
            .expect("matching result families form an Int case");
            assert_eq!(int_result.value_type(), expected);

            let string_result = string_case_expr(
                string("one").into(),
                vec![("one".into(), branch.clone())],
                fallback.clone(),
            )
            .expect("matching result families form a String case");
            assert_eq!(string_result.value_type(), expected);

            let float_result = float_case_expr(float(1.0).into(), vec![(1.0, branch)], fallback)
                .expect("matching result families form a Float case");
            assert_eq!(float_result.value_type(), expected);
        }
    }

    #[test]
    fn rejects_every_result_family_mismatch_at_the_common_adapter() {
        let fallbacks = case_result_families(1);

        for (index, fallback) in fallbacks.iter().cloned().enumerate() {
            let branch: Expr = if index == 1 {
                string("wrong").into()
            } else {
                int(0).into()
            };
            let expected = fallback.value_type();
            let actual = branch.value_type();
            let error = PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchFamilyAssemblyMismatch {
                        expected,
                        actual,
                    },
                },
            };

            assert_eq!(
                bool_case_expr(BoolExpr::value(true), branch.clone(), fallback.clone(),),
                Err(error.clone()),
            );
            assert_eq!(
                int_case_expr(
                    int(1).into(),
                    vec![(BigInt::from(1), branch.clone())],
                    fallback.clone(),
                ),
                Err(error.clone()),
            );
            assert_eq!(
                string_case_expr(
                    string("one").into(),
                    vec![("one".into(), branch.clone())],
                    fallback.clone(),
                ),
                Err(error.clone()),
            );
            assert_eq!(
                float_case_expr(float(1.0).into(), vec![(1.0, branch)], fallback,),
                Err(error),
            );
        }

        let function_families = &fallbacks[12..];
        for (index, fallback) in function_families.iter().cloned().enumerate() {
            let branch = function_families[(index + 1) % function_families.len()].clone();
            let expected = fallback.value_type();
            let actual = branch.value_type();

            assert_eq!(
                bool_case_expr(BoolExpr::value(true), branch, fallback),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CaseShape {
                        reason: InvalidCaseShapeReason::BranchFamilyAssemblyMismatch {
                            expected,
                            actual,
                        },
                    },
                }),
            );
        }
    }

    #[test]
    fn keyed_cases_preserve_function_result_families() {
        let int_branch = int_function(0)
            .into_function()
            .expect("test expression is function-valued")
            .into_int()
            .expect("test expression returns Int");
        let int_fallback = int_function(1)
            .into_function()
            .expect("test expression is function-valued")
            .into_int()
            .expect("test expression returns Int");

        assert_eq!(
            int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), int_function(0))],
                int_function(1),
            ),
            Ok(Expr::function(FunctionExpr::int(
                IntFunctionExpr::int_case(
                    int(1).into(),
                    vec![(BigInt::from(1), int_branch.clone())],
                    int_fallback.clone(),
                )
            ))),
        );
        assert_eq!(
            string_case_expr(
                string("one").into(),
                vec![("one".into(), int_function(0))],
                int_function(1),
            ),
            Ok(Expr::function(FunctionExpr::int(
                IntFunctionExpr::string_case(
                    string("one").into(),
                    vec![("one".into(), int_branch.clone())],
                    int_fallback.clone(),
                ),
            ))),
        );
        assert_eq!(
            float_case_expr(
                float(1.0).into(),
                vec![(1.0, int_function(0))],
                int_function(1),
            ),
            Ok(Expr::function(FunctionExpr::int(
                IntFunctionExpr::float_case(
                    float(1.0).into(),
                    vec![(1.0, int_branch)],
                    int_fallback,
                )
            ))),
        );
    }

    #[test]
    fn keyed_cases_preserve_generic_result_shapes() {
        let branch = generic(0, 0);
        let fallback = generic(0, 1);
        assert_eq!(
            int_case_expr(
                int(1).into(),
                vec![(BigInt::from(1), Expr::generic(branch.clone()))],
                Expr::generic(fallback.clone()),
            ),
            Ok(Expr::generic(
                GenericExpr::int_case(
                    int(1).into(),
                    vec![(BigInt::from(1), branch.clone())],
                    fallback.clone(),
                )
                .expect("matching generic branches form an Int case"),
            )),
        );
        assert_eq!(
            string_case_expr(
                string("one").into(),
                vec![("one".into(), Expr::generic(branch.clone()))],
                Expr::generic(fallback.clone()),
            ),
            Ok(Expr::generic(
                GenericExpr::string_case(
                    string("one").into(),
                    vec![("one".into(), branch.clone())],
                    fallback.clone(),
                )
                .expect("matching generic branches form a String case"),
            )),
        );
        assert_eq!(
            float_case_expr(
                float(1.0).into(),
                vec![(1.0, Expr::generic(branch.clone()))],
                Expr::generic(fallback.clone()),
            ),
            Ok(Expr::generic(
                GenericExpr::float_case(float(1.0).into(), vec![(1.0, branch)], fallback,)
                    .expect("matching generic branches form a Float case"),
            )),
        );
    }

    #[test]
    fn keyed_cases_preserve_generic_function_result_shapes() {
        let branch = generic_function(0, 0);
        let fallback = generic_function(0, 1);
        assert_eq!(
            int_case_expr(
                int(1).into(),
                vec![(
                    BigInt::from(1),
                    Expr::function(FunctionExpr::generic(branch.clone())),
                )],
                Expr::function(FunctionExpr::generic(fallback.clone())),
            ),
            Ok(Expr::function(FunctionExpr::generic(
                GenericFunctionExpr::int_case(
                    int(1).into(),
                    vec![(BigInt::from(1), branch.clone())],
                    fallback.clone(),
                )
                .expect("matching generic functions form an Int case"),
            ))),
        );
        assert_eq!(
            string_case_expr(
                string("one").into(),
                vec![(
                    "one".into(),
                    Expr::function(FunctionExpr::generic(branch.clone())),
                )],
                Expr::function(FunctionExpr::generic(fallback.clone())),
            ),
            Ok(Expr::function(FunctionExpr::generic(
                GenericFunctionExpr::string_case(
                    string("one").into(),
                    vec![("one".into(), branch.clone())],
                    fallback.clone(),
                )
                .expect("matching generic functions form a String case"),
            ))),
        );
        assert_eq!(
            float_case_expr(
                float(1.0).into(),
                vec![(1.0, Expr::function(FunctionExpr::generic(branch.clone())))],
                Expr::function(FunctionExpr::generic(fallback.clone())),
            ),
            Ok(Expr::function(FunctionExpr::generic(
                GenericFunctionExpr::float_case(float(1.0).into(), vec![(1.0, branch)], fallback,)
                    .expect("matching generic functions form a Float case"),
            ))),
        );
    }

    fn custom_type(name: &str) -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), name.into()),
            Vec::new(),
        )
    }

    fn refined_custom_shape(constructor: usize) -> CustomValueShape {
        let member = CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Member".into()),
            Vec::new(),
            CustomConstructorRefinement::Exact(constructor),
        );
        CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Container".into()),
            vec![ValueShape::Function(Box::new(FunctionShape::new(
                vec![ValueShape::Custom(member)],
                ValueShape::Int,
            )))],
            CustomConstructorRefinement::Any,
        )
    }

    fn custom_local_with_shape(shape: CustomValueShape, local: usize) -> Expr {
        Expr::custom(crate::plan::CustomExpr::local_get(
            crate::plan::CustomLocal::from_shape(crate::plan::CustomLocalId(local), shape),
            "container".into(),
        ))
    }

    fn external_shape() -> ExternalValueShape {
        ExternalValueShape::new(
            ExternalTypeName::new(
                "dependency".into(),
                "dependency/token".into(),
                "Token".into(),
            ),
            Vec::new(),
        )
    }

    fn refined_external_shape(constructor: usize) -> ExternalValueShape {
        let member = CustomValueShape::new(
            CustomTypeName::new("geam".into(), "main".into(), "Member".into()),
            Vec::new(),
            CustomConstructorRefinement::Exact(constructor),
        );
        ExternalValueShape::new(
            ExternalTypeName::new(
                "dependency".into(),
                "dependency/token".into(),
                "Token".into(),
            ),
            vec![ValueShape::Function(Box::new(FunctionShape::new(
                vec![ValueShape::Custom(member)],
                ValueShape::Int,
            )))],
        )
    }

    fn int_function(id: usize) -> Expr {
        function_ref(
            RuntimeFunctionId::Int(IntFunctionId(id)),
            [LocalId::Int(IntLocalId(0))],
        )
        .into()
    }

    fn generic(parameter: usize, local: usize) -> GenericExpr {
        GenericExpr::local_get(
            GenericLocal::new(GenericLocalId(local), TypeParameterId(parameter)),
            "generic".into(),
        )
    }

    fn generic_function(parameter: usize, local: usize) -> GenericFunctionExpr {
        GenericFunctionExpr::local_get(
            GenericFunctionLocal::new(
                GenericFunctionLocalId(local),
                GenericFunctionType::new(vec![ValueShape::Int], TypeParameterId(parameter)),
            ),
            "generic_function".into(),
        )
    }

    fn case_result_families(local: usize) -> Vec<Expr> {
        let parameter = TypeParameterId(0);
        let custom = custom_type("Result");
        let external_shape = external_shape();
        let scalar_function = |return_: ValueType| FunctionType::new(Vec::new(), return_);
        let int_function_type = scalar_function(ValueType::Int);
        let string_function_type = scalar_function(ValueType::String);
        let bit_array_function_type = scalar_function(ValueType::BitArray);
        let utf_codepoint_function_type = scalar_function(ValueType::UtfCodepoint);
        let float_function_type = scalar_function(ValueType::Float);
        let bool_function_type = scalar_function(ValueType::Bool);
        let nil_function_type = scalar_function(ValueType::Nil);
        let tuple_function_type = scalar_function(ValueType::Tuple(vec![ValueType::Int]));
        let list_function_type = scalar_function(ValueType::List(Box::new(ValueType::Int)));
        let returned_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function_function_type =
            FunctionFunctionType::new(Vec::new(), returned_function_type.clone());

        vec![
            Expr::generic(generic(0, local)),
            int(local as i64).into(),
            string(format!("value-{local}")).into(),
            Expr::bit_array(BitArrayExpr::local_get(
                BitArrayLocalId(local),
                "bits".into(),
            )),
            Expr::utf_codepoint(UtfCodepointExpr::local_get(
                UtfCodepointLocalId(local),
                "codepoint".into(),
            )),
            Expr::custom(crate::plan::CustomExpr::local_get(
                crate::plan::CustomLocal::from_shape(
                    crate::plan::CustomLocalId(local),
                    CustomValueShape::any(custom.clone()),
                ),
                "custom".into(),
            )),
            Expr::external(ExternalExpr::local_get(
                ExternalLocal::from_shape(ExternalLocalId(local), external_shape.clone()),
                "external".into(),
            )),
            float(local as f64).into(),
            Expr::bool(BoolExpr::value(local.is_multiple_of(2))),
            Expr::nil(NilExpr::value()),
            crate::planner::dsl::tuple([int(local as i64)]).into(),
            list([int(local as i64)], ValueType::Int).into(),
            Expr::function(FunctionExpr::generic(GenericFunctionExpr::local_get(
                GenericFunctionLocal::new(
                    GenericFunctionLocalId(local),
                    GenericFunctionType::new(Vec::new(), parameter),
                ),
                "generic_function".into(),
            ))),
            Expr::function(FunctionExpr::int(IntFunctionExpr::local_get(
                IntFunctionLocalId(local),
                "int_function".into(),
                int_function_type,
            ))),
            Expr::function(FunctionExpr::string(StringFunctionExpr::local_get(
                StringFunctionLocalId(local),
                "string_function".into(),
                string_function_type,
            ))),
            Expr::function(FunctionExpr::bit_array(BitArrayFunctionExpr::local_get(
                BitArrayFunctionLocalId(local),
                "bit_array_function".into(),
                bit_array_function_type,
            ))),
            Expr::function(FunctionExpr::utf_codepoint(
                UtfCodepointFunctionExpr::local_get(
                    UtfCodepointFunctionLocalId(local),
                    "utf_codepoint_function".into(),
                    utf_codepoint_function_type,
                ),
            )),
            Expr::function(FunctionExpr::custom(CustomFunctionExpr::local_get(
                CustomFunctionLocal::new(
                    CustomFunctionLocalId(local),
                    CustomFunctionType::new(Vec::new(), custom),
                ),
                "custom_function".into(),
            ))),
            Expr::function(FunctionExpr::external(ExternalFunctionExpr::local_get(
                ExternalFunctionLocal::new(
                    ExternalFunctionLocalId(local),
                    ExternalFunctionType::from_shapes(Vec::new(), external_shape),
                ),
                "external_function".into(),
            ))),
            Expr::function(FunctionExpr::float(FloatFunctionExpr::local_get(
                FloatFunctionLocalId(local),
                "float_function".into(),
                float_function_type,
            ))),
            Expr::function(FunctionExpr::bool(BoolFunctionExpr::local_get(
                BoolFunctionLocalId(local),
                "bool_function".into(),
                bool_function_type,
            ))),
            Expr::function(FunctionExpr::nil(NilFunctionExpr::local_get(
                NilFunctionLocalId(local),
                "nil_function".into(),
                nil_function_type,
            ))),
            Expr::function(FunctionExpr::tuple(TupleFunctionExpr::local_get(
                TupleFunctionLocalId(local),
                "tuple_function".into(),
                tuple_function_type,
            ))),
            Expr::function(FunctionExpr::list(ListFunctionExpr::local_get(
                ListFunctionLocal::from_item_type(local, list_function_type, ValueType::Int),
                "list_function".into(),
            ))),
            Expr::function(FunctionExpr::function(FunctionFunctionExpr::local_get(
                FunctionFunctionLocal::new(FunctionFunctionLocalId(local), function_function_type),
                "function_function".into(),
            ))),
        ]
    }

    fn assert_nominal_function_mismatch(branch: Expr, fallback: Expr) {
        let expected = fallback.value_type();
        let actual = branch.value_type();
        assert_eq!(
            bool_case_expr(BoolExpr::value(true), branch, fallback),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchFamilyAssemblyMismatch {
                        expected,
                        actual,
                    },
                },
            }),
        );
    }
}
