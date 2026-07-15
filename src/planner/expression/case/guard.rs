use crate::plan::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CustomExpr, CustomFunctionExpr,
    Expr, FloatExpr, FloatFunctionExpr, FunctionExpr, FunctionFunctionExpr, IntExpr,
    IntFunctionExpr, ListExpr, ListFunctionExpr, LocalId, NilExpr, NilFunctionExpr, StringExpr,
    StringFunctionExpr, TupleExpr, TupleFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
    ValueType,
};
use crate::planner::context::{FunctionLocalBinding, PlanContext};
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    UnsupportedBinOpKind,
};
use ecow::EcoString;
use gleam_core::ast::{BinOp, ClauseGuard};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan_bool(
    guard: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<BoolExpr, PlanError> {
    let expression = plan_expr(guard, context)?;
    let actual = expression.value_type();
    expression
        .into_bool()
        .ok_or_else(|| invalid_expression_type_for_value(ValueType::Bool, actual))
}

fn plan_expr(
    guard: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match guard {
        ClauseGuard::Constant(constant) => super::super::constant::plan(constant, context),
        ClauseGuard::Block { value, .. } => plan_expr(*value, context),
        ClauseGuard::Var { name, .. } => plan_local(name, context),
        ClauseGuard::TupleIndex {
            tuple,
            index,
            type_,
            ..
        } => plan_tuple_index(*tuple, index, type_, context),
        ClauseGuard::Not { expression, .. } => {
            Ok(Expr::bool(BoolExpr::not(plan_bool(*expression, context)?)))
        }
        ClauseGuard::BinaryOperator {
            operator,
            left,
            right,
            ..
        } => plan_binary_operator(operator, *left, *right, context),
        ClauseGuard::ModuleSelect {
            module_name,
            literal,
            ..
        } if module_name == *context.module_name => super::super::constant::plan(literal, context),
        ClauseGuard::ModuleSelect { .. } => {
            invalid_expression_shape(InvalidExpressionShapeKind::ModuleSelect)
        }
        ClauseGuard::FieldAccess {
            index: Some(index),
            label,
            type_,
            container,
            ..
        } => {
            let container_type = container.type_();
            let container = plan_expr(*container, context)?;
            super::super::record_access::plan_from_expr(
                type_,
                Some(label),
                index,
                container_type,
                container,
                context,
            )
        }
        ClauseGuard::FieldAccess { index: None, .. } => {
            invalid_expression_shape(InvalidExpressionShapeKind::RecordAccess)
        }
        ClauseGuard::Invalid { .. } => {
            invalid_expression_shape(InvalidExpressionShapeKind::Invalid)
        }
    }
}

fn plan_binary_operator(
    operator: BinOp,
    left: ClauseGuard<Arc<Type>>,
    right: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match operator {
        BinOp::And => bool_binary_operator(left, right, context, BoolExpr::and),
        BinOp::Or => bool_binary_operator(left, right, context, BoolExpr::or),
        BinOp::Eq => equality(
            left,
            right,
            context,
            UnsupportedBinOpKind::EqFunction,
            false,
        ),
        BinOp::NotEq => equality(
            left,
            right,
            context,
            UnsupportedBinOpKind::NotEqFunction,
            true,
        ),
        BinOp::GtInt => int_comparison_operator(left, right, context, BoolExpr::gt_int),
        BinOp::GtEqInt => int_comparison_operator(left, right, context, BoolExpr::gte_int),
        BinOp::LtInt => int_comparison_operator(left, right, context, BoolExpr::lt_int),
        BinOp::LtEqInt => int_comparison_operator(left, right, context, BoolExpr::lte_int),
        BinOp::GtFloat => float_comparison_operator(left, right, context, BoolExpr::gt_float),
        BinOp::GtEqFloat => float_comparison_operator(left, right, context, BoolExpr::gte_float),
        BinOp::LtFloat => float_comparison_operator(left, right, context, BoolExpr::lt_float),
        BinOp::LtEqFloat => float_comparison_operator(left, right, context, BoolExpr::lte_float),
        BinOp::AddInt => int_binary_operator(left, right, context, IntExpr::add),
        BinOp::SubInt => int_binary_operator(left, right, context, IntExpr::sub),
        BinOp::MultInt => int_binary_operator(left, right, context, IntExpr::mult),
        BinOp::DivInt => int_binary_operator(left, right, context, IntExpr::div),
        BinOp::RemainderInt => int_binary_operator(left, right, context, IntExpr::remainder),
        BinOp::AddFloat => float_binary_operator(left, right, context, FloatExpr::add),
        BinOp::SubFloat => float_binary_operator(left, right, context, FloatExpr::sub),
        BinOp::MultFloat => float_binary_operator(left, right, context, FloatExpr::mult),
        BinOp::DivFloat => float_binary_operator(left, right, context, FloatExpr::div),
        BinOp::Concatenate => string_binary_operator(left, right, context, StringExpr::concatenate),
    }
}

fn bool_binary_operator(
    left: ClauseGuard<Arc<Type>>,
    right: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
    operator: fn(BoolExpr, BoolExpr) -> BoolExpr,
) -> Result<Expr, PlanError> {
    let left = plan_bool(left, context)?;
    let right = plan_bool(right, context)?;
    Ok(Expr::bool(operator(left, right)))
}

fn int_comparison_operator(
    left: ClauseGuard<Arc<Type>>,
    right: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
    operator: fn(IntExpr, IntExpr) -> BoolExpr,
) -> Result<Expr, PlanError> {
    let left = plan_int(left, context)?;
    let right = plan_int(right, context)?;
    Ok(Expr::bool(operator(left, right)))
}

fn float_comparison_operator(
    left: ClauseGuard<Arc<Type>>,
    right: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
    operator: fn(FloatExpr, FloatExpr) -> BoolExpr,
) -> Result<Expr, PlanError> {
    let left = plan_float(left, context)?;
    let right = plan_float(right, context)?;
    Ok(Expr::bool(operator(left, right)))
}

fn int_binary_operator(
    left: ClauseGuard<Arc<Type>>,
    right: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
    operator: fn(IntExpr, IntExpr) -> IntExpr,
) -> Result<Expr, PlanError> {
    let left = plan_int(left, context)?;
    let right = plan_int(right, context)?;
    Ok(Expr::int(operator(left, right)))
}

fn float_binary_operator(
    left: ClauseGuard<Arc<Type>>,
    right: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
    operator: fn(FloatExpr, FloatExpr) -> FloatExpr,
) -> Result<Expr, PlanError> {
    let left = plan_float(left, context)?;
    let right = plan_float(right, context)?;
    Ok(Expr::float(operator(left, right)))
}

fn string_binary_operator(
    left: ClauseGuard<Arc<Type>>,
    right: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
    operator: fn(StringExpr, StringExpr) -> StringExpr,
) -> Result<Expr, PlanError> {
    let left = plan_string(left, context)?;
    let right = plan_string(right, context)?;
    Ok(Expr::string(operator(left, right)))
}

fn equality(
    left: ClauseGuard<Arc<Type>>,
    right: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
    operator: UnsupportedBinOpKind,
    negated: bool,
) -> Result<Expr, PlanError> {
    let left = plan_expr(left, context)?;
    let right = plan_expr(right, context)?;
    if context.contains_function_value(&left.value_type())?
        || context.contains_function_value(&right.value_type())?
    {
        return Err(PlanError::UnsupportedBinOp { operator });
    }

    let expression = if negated {
        BoolExpr::not_equal(left, right)
    } else {
        BoolExpr::equal(left, right)
    };
    Ok(Expr::bool(expression))
}

fn plan_int(
    guard: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<IntExpr, PlanError> {
    let expression = plan_expr(guard, context)?;
    let actual = expression.value_type();
    expression
        .into_int()
        .ok_or_else(|| invalid_expression_type_for_value(ValueType::Int, actual))
}

fn plan_float(
    guard: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<FloatExpr, PlanError> {
    let expression = plan_expr(guard, context)?;
    let actual = expression.value_type();
    expression
        .into_float()
        .ok_or_else(|| invalid_expression_type_for_value(ValueType::Float, actual))
}

fn plan_string(
    guard: ClauseGuard<Arc<Type>>,
    context: &mut PlanContext<'_>,
) -> Result<StringExpr, PlanError> {
    let expression = plan_expr(guard, context)?;
    let actual = expression.value_type();
    expression
        .into_string()
        .ok_or_else(|| invalid_expression_type_for_value(ValueType::String, actual))
}

fn plan_tuple_index(
    tuple: ClauseGuard<Arc<Type>>,
    index: u64,
    type_: Arc<Type>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    #[cfg(target_pointer_width = "64")]
    let index = index as usize;
    #[cfg(not(target_pointer_width = "64"))]
    let index = usize::try_from(index).map_err(|_| PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::Tuple,
            actual: InvalidExpressionType::Tuple,
        },
    })?;
    let tuple = plan_expr(tuple, context)?;
    let actual = tuple.value_type();
    let Some(tuple) = tuple.into_tuple() else {
        return Err(invalid_expression_type_for_value(
            ValueType::Tuple(Vec::new()),
            actual,
        ));
    };
    let expected =
        ValueType::from_gleam(type_.as_ref()).ok_or_else(|| PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Unsupported,
                actual: InvalidExpressionType::Tuple,
            },
        })?;
    let actual = tuple.type_().get(index).cloned().ok_or_else(|| {
        invalid_expression_type_for_value(expected.clone(), ValueType::Tuple(Vec::new()))
    })?;
    if actual != expected {
        return Err(invalid_expression_type_for_value(expected.clone(), actual));
    }

    Ok(super::super::tuple_index_expr(tuple, index, expected))
}

fn plan_local(name: EcoString, context: &PlanContext<'_>) -> Result<Expr, PlanError> {
    if let Some((local, type_)) = context.lookup_local(&name) {
        return local_get(local, name, type_);
    }
    if let Some((local, type_)) = context.lookup_custom_local(&name) {
        return Ok(Expr::custom(CustomExpr::local_get(local, name, type_)));
    }
    if let Some((local, type_)) = context.lookup_tuple_local(&name) {
        return Ok(Expr::tuple(TupleExpr::local_get(local, name, type_)));
    }
    if let Some(local) = context.lookup_list_local(&name) {
        return Ok(Expr::list(ListExpr::local_get(local, name)));
    }
    if let Some(binding) = context.lookup_function_local(&name) {
        return Ok(function_local_get(binding, name));
    }

    Err(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::UnknownLocal { name },
    })
}

fn local_get(local: LocalId, name: EcoString, type_: ValueType) -> Result<Expr, PlanError> {
    match (local, type_) {
        (LocalId::Int(local), ValueType::Int) => Ok(Expr::int(IntExpr::local_get(local, name))),
        (LocalId::Float(local), ValueType::Float) => {
            Ok(Expr::float(FloatExpr::local_get(local, name)))
        }
        (LocalId::String(local), ValueType::String) => {
            Ok(Expr::string(StringExpr::local_get(local, name)))
        }
        (LocalId::BitArray(local), ValueType::BitArray) => {
            Ok(Expr::bit_array(BitArrayExpr::local_get(local, name)))
        }
        (LocalId::UtfCodepoint(local), ValueType::UtfCodepoint) => Ok(Expr::utf_codepoint(
            UtfCodepointExpr::local_get(local, name),
        )),
        (LocalId::Bool(local), ValueType::Bool) => Ok(Expr::bool(BoolExpr::local_get(local, name))),
        (LocalId::Nil(local), ValueType::Nil) => Ok(Expr::nil(NilExpr::local_get(local, name))),
        _ => invalid_expression_shape(InvalidExpressionShapeKind::Invalid),
    }
}

fn function_local_get(binding: FunctionLocalBinding, name: EcoString) -> Expr {
    match binding {
        FunctionLocalBinding::Int { local, type_ } => Expr::function(FunctionExpr::int(
            IntFunctionExpr::local_get(local, name, type_),
        )),
        FunctionLocalBinding::String { local, type_ } => Expr::function(FunctionExpr::string(
            StringFunctionExpr::local_get(local, name, type_),
        )),
        FunctionLocalBinding::BitArray { local, type_ } => Expr::function(FunctionExpr::bit_array(
            BitArrayFunctionExpr::local_get(local, name, type_),
        )),
        FunctionLocalBinding::UtfCodepoint { local, type_ } => Expr::function(
            FunctionExpr::utf_codepoint(UtfCodepointFunctionExpr::local_get(local, name, type_)),
        ),
        FunctionLocalBinding::Custom(local) => Expr::function(FunctionExpr::custom(
            CustomFunctionExpr::local_get(local, name),
        )),
        FunctionLocalBinding::Float { local, type_ } => Expr::function(FunctionExpr::float(
            FloatFunctionExpr::local_get(local, name, type_),
        )),
        FunctionLocalBinding::Bool { local, type_ } => Expr::function(FunctionExpr::bool(
            BoolFunctionExpr::local_get(local, name, type_),
        )),
        FunctionLocalBinding::Nil { local, type_ } => Expr::function(FunctionExpr::nil(
            NilFunctionExpr::local_get(local, name, type_),
        )),
        FunctionLocalBinding::Tuple { local, type_ } => Expr::function(FunctionExpr::tuple(
            TupleFunctionExpr::local_get(local, name, type_),
        )),
        FunctionLocalBinding::List(local) => {
            Expr::function(FunctionExpr::list(ListFunctionExpr::local_get(local, name)))
        }
        FunctionLocalBinding::Function(local) => Expr::function(FunctionExpr::function(
            FunctionFunctionExpr::local_get(local, name),
        )),
    }
}

fn invalid_expression_type_for_value(expected: ValueType, actual: ValueType) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: invalid_expression_type(expected),
            actual: invalid_expression_type(actual),
        },
    }
}

fn invalid_expression_type(type_: ValueType) -> InvalidExpressionType {
    match type_ {
        ValueType::Int => InvalidExpressionType::Int,
        ValueType::Float => InvalidExpressionType::Float,
        ValueType::String => InvalidExpressionType::String,
        ValueType::BitArray => InvalidExpressionType::BitArray,
        ValueType::UtfCodepoint => InvalidExpressionType::UtfCodepoint,
        ValueType::Custom(_) => InvalidExpressionType::Custom,
        ValueType::Bool => InvalidExpressionType::Bool,
        ValueType::Nil => InvalidExpressionType::Nil,
        ValueType::Tuple(_) => InvalidExpressionType::Tuple,
        ValueType::List(_) => InvalidExpressionType::List,
        ValueType::Function(_) => InvalidExpressionType::Function,
    }
}

fn invalid_expression_shape(kind: InvalidExpressionShapeKind) -> Result<Expr, PlanError> {
    Err(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionShape { kind },
    })
}

#[cfg(test)]
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use super::{function_local_get, invalid_expression_type, plan_expr};
    use crate::plan::{
        BitArrayExpr, BitArrayFunctionExpr, BitArrayFunctionLocalId, BitArrayLocalId, BoolExpr,
        BoolFunctionExpr, BoolFunctionLocalId, BoolLocalId, CustomFunctionExpr,
        CustomFunctionLocal, CustomFunctionLocalId, CustomFunctionType, CustomType, CustomTypeName,
        Expr, FloatExpr, FloatFunctionExpr, FloatFunctionLocalId, FunctionExpr,
        FunctionFunctionExpr, FunctionFunctionLocal, FunctionFunctionLocalId, FunctionFunctionType,
        FunctionType, IntExpr, IntFunctionExpr, IntFunctionLocalId, IntLocalId, ListExpr,
        ListFunctionExpr, ListLocal, LocalId, NilExpr, NilFunctionExpr, NilFunctionLocalId,
        NilLocalId, StringExpr, StringFunctionExpr, StringFunctionLocalId, StringListLocalId,
        TupleExpr, TupleFunctionExpr, TupleFunctionLocalId, TupleLocalId, UtfCodepointExpr,
        UtfCodepointFunctionExpr, UtfCodepointFunctionLocalId, UtfCodepointLocalId, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionLocalBinding, PlanContext};
    use crate::planner::support::dummy_span;
    use crate::planner::{
        InvalidExpressionShapeKind, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedBinOpKind,
    };
    use ecow::EcoString;
    use gleam_core::ast::{BinOp, ClauseGuard, Constant, Publicity};
    use gleam_core::parse::LiteralFloatValue;
    use gleam_core::type_::{self, Type, error::VariableOrigin};
    use num_bigint::BigInt;
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn plan_expr_handles_non_operator_guard_shapes() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        context.define_bool_local("flag".into());
        context.define_tuple_local("pair".into(), vec![ValueType::Int, ValueType::String]);

        assert_eq!(
            plan_expr(int_constant(1), &mut context),
            Ok(Expr::int(IntExpr::value(1.into()))),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::Block {
                    location: dummy_span(),
                    value: Box::new(int_constant(2)),
                },
                &mut context,
            ),
            Ok(Expr::int(IntExpr::value(2.into()))),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::Not {
                    location: dummy_span(),
                    expression: Box::new(var("flag", type_::bool())),
                },
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::not(BoolExpr::local_get(
                crate::plan::BoolLocalId(0),
                "flag".into(),
            )))),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::TupleIndex {
                    location: dummy_span(),
                    index: 1,
                    type_: type_::string(),
                    tuple: Box::new(var(
                        "pair",
                        type_::tuple(vec![type_::int(), type_::string()]),
                    )),
                },
                &mut context,
            ),
            Ok(Expr::string(StringExpr::tuple_index(
                TupleExpr::local_get(
                    TupleLocalId(0),
                    "pair".into(),
                    vec![ValueType::Int, ValueType::String],
                ),
                1,
            ))),
        );
        assert_eq!(
            plan_expr(module_select("main", int_constant_literal(3)), &mut context),
            Ok(Expr::int(IntExpr::value(3.into()))),
        );
    }

    #[test]
    fn plan_expr_rejects_invalid_guard_shapes() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        context.define_tuple_local("pair".into(), vec![ValueType::String]);

        assert_eq!(
            plan_expr(
                module_select("other", int_constant_literal(1)),
                &mut context
            ),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::ModuleSelect
            )),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::TupleIndex {
                    location: dummy_span(),
                    index: 0,
                    type_: type_::generic_var(0),
                    tuple: Box::new(var("pair", type_::tuple(vec![type_::string()]))),
                },
                &mut context,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Unsupported,
                InvalidExpressionType::Tuple,
            )),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::FieldAccess {
                    label_location: dummy_span(),
                    index: Some(0),
                    label: "field".into(),
                    type_: type_::int(),
                    container: Box::new(int_constant(1)),
                },
                &mut context,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Custom,
                InvalidExpressionType::Int,
            )),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::FieldAccess {
                    label_location: dummy_span(),
                    index: Some(0),
                    label: "field".into(),
                    type_: type_::int(),
                    container: Box::new(ClauseGuard::Invalid {
                        location: dummy_span(),
                        type_: type_::int(),
                    }),
                },
                &mut context,
            ),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::Invalid,
            )),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::FieldAccess {
                    label_location: dummy_span(),
                    index: None,
                    label: "field".into(),
                    type_: type_::int(),
                    container: Box::new(int_constant(1)),
                },
                &mut context,
            ),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::RecordAccess,
            )),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::Invalid {
                    location: dummy_span(),
                    type_: type_::int(),
                },
                &mut context,
            ),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::Invalid
            )),
        );
    }

    #[test]
    fn binary_operators_preserve_success_shapes() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        context.define_bool_local("flag".into());

        let flag = BoolExpr::local_get(BoolLocalId(0), "flag".into());
        let int_one = IntExpr::value(1.into());
        let int_two = IntExpr::value(2.into());
        let float_one = FloatExpr::value(1.0);
        let float_two = FloatExpr::value(1.0);
        let string_left = StringExpr::value("ge".into());
        let string_right = StringExpr::value("am".into());

        assert_eq!(
            plan_expr(
                binary(
                    BinOp::And,
                    var("flag", type_::bool()),
                    var("flag", type_::bool())
                ),
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::and(flag.clone(), flag.clone()))),
        );
        assert_eq!(
            plan_expr(
                binary(
                    BinOp::Or,
                    var("flag", type_::bool()),
                    var("flag", type_::bool())
                ),
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::or(flag.clone(), flag.clone()))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::Eq, int_constant(1), int_constant(2)),
                &mut context
            ),
            Ok(Expr::bool(BoolExpr::equal(
                Expr::int(int_one.clone()),
                Expr::int(int_two.clone()),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::NotEq, int_constant(1), int_constant(2)),
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::not_equal(
                Expr::int(int_one.clone()),
                Expr::int(int_two.clone()),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::GtInt, int_constant(1), int_constant(2)),
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::gt_int(
                int_one.clone(),
                int_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::GtEqInt, int_constant(1), int_constant(2)),
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::gte_int(
                int_one.clone(),
                int_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::LtInt, int_constant(1), int_constant(2)),
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::lt_int(
                int_one.clone(),
                int_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::LtEqInt, int_constant(1), int_constant(2)),
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::lte_int(
                int_one.clone(),
                int_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::GtFloat, float_constant(), float_constant()),
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::gt_float(
                float_one.clone(),
                float_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::GtEqFloat, float_constant(), float_constant()),
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::gte_float(
                float_one.clone(),
                float_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::LtFloat, float_constant(), float_constant()),
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::lt_float(
                float_one.clone(),
                float_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::LtEqFloat, float_constant(), float_constant()),
                &mut context,
            ),
            Ok(Expr::bool(BoolExpr::lte_float(
                float_one.clone(),
                float_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::AddInt, int_constant(1), int_constant(2)),
                &mut context,
            ),
            Ok(Expr::int(IntExpr::add(int_one.clone(), int_two.clone()))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::SubInt, int_constant(1), int_constant(2)),
                &mut context,
            ),
            Ok(Expr::int(IntExpr::sub(int_one.clone(), int_two.clone()))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::MultInt, int_constant(1), int_constant(2)),
                &mut context,
            ),
            Ok(Expr::int(IntExpr::mult(int_one.clone(), int_two.clone()))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::DivInt, int_constant(1), int_constant(2)),
                &mut context,
            ),
            Ok(Expr::int(IntExpr::div(int_one.clone(), int_two.clone()))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::RemainderInt, int_constant(1), int_constant(2)),
                &mut context,
            ),
            Ok(Expr::int(IntExpr::remainder(
                int_one.clone(),
                int_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::AddFloat, float_constant(), float_constant()),
                &mut context,
            ),
            Ok(Expr::float(FloatExpr::add(
                float_one.clone(),
                float_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::SubFloat, float_constant(), float_constant()),
                &mut context,
            ),
            Ok(Expr::float(FloatExpr::sub(
                float_one.clone(),
                float_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::MultFloat, float_constant(), float_constant()),
                &mut context,
            ),
            Ok(Expr::float(FloatExpr::mult(
                float_one.clone(),
                float_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(BinOp::DivFloat, float_constant(), float_constant()),
                &mut context,
            ),
            Ok(Expr::float(FloatExpr::div(
                float_one.clone(),
                float_two.clone(),
            ))),
        );
        assert_eq!(
            plan_expr(
                binary(
                    BinOp::Concatenate,
                    string_constant("ge"),
                    string_constant("am")
                ),
                &mut context,
            ),
            Ok(Expr::string(StringExpr::concatenate(
                string_left,
                string_right,
            ))),
        );
    }

    #[test]
    fn not_guard_rejects_non_bool_expression() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            plan_expr(
                ClauseGuard::Not {
                    location: dummy_span(),
                    expression: Box::new(int_constant(1)),
                },
                &mut context,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Bool,
                InvalidExpressionType::Int,
            )),
        );
    }

    #[test]
    fn binary_operator_helpers_reject_operand_failures_exactly() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        context.define_bool_local("flag".into());

        assert_eq!(
            super::bool_binary_operator(
                int_constant(1),
                var("flag", type_::bool()),
                &mut context,
                BoolExpr::and,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Bool,
                InvalidExpressionType::Int,
            )),
        );
        assert_eq!(
            super::bool_binary_operator(
                var("flag", type_::bool()),
                int_constant(1),
                &mut context,
                BoolExpr::and,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Bool,
                InvalidExpressionType::Int,
            )),
        );
        assert_eq!(
            super::int_comparison_operator(
                string_constant("left"),
                int_constant(1),
                &mut context,
                BoolExpr::gt_int,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Int,
                InvalidExpressionType::String,
            )),
        );
        assert_eq!(
            super::int_comparison_operator(
                int_constant(1),
                string_constant("right"),
                &mut context,
                BoolExpr::gt_int,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Int,
                InvalidExpressionType::String,
            )),
        );
        assert_eq!(
            super::float_comparison_operator(
                int_constant(1),
                float_constant(),
                &mut context,
                BoolExpr::gt_float,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Float,
                InvalidExpressionType::Int,
            )),
        );
        assert_eq!(
            super::float_comparison_operator(
                float_constant(),
                int_constant(1),
                &mut context,
                BoolExpr::gt_float,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Float,
                InvalidExpressionType::Int,
            )),
        );
        assert_eq!(
            super::int_binary_operator(
                ClauseGuard::Invalid {
                    location: dummy_span(),
                    type_: type_::int(),
                },
                int_constant(1),
                &mut context,
                IntExpr::add,
            ),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::Invalid
            )),
        );
        assert_eq!(
            super::int_binary_operator(
                int_constant(1),
                ClauseGuard::Invalid {
                    location: dummy_span(),
                    type_: type_::int(),
                },
                &mut context,
                IntExpr::add,
            ),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::Invalid
            )),
        );
        assert_eq!(
            super::float_binary_operator(
                ClauseGuard::Invalid {
                    location: dummy_span(),
                    type_: type_::float(),
                },
                float_constant(),
                &mut context,
                FloatExpr::add,
            ),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::Invalid
            )),
        );
        assert_eq!(
            super::float_binary_operator(
                float_constant(),
                ClauseGuard::Invalid {
                    location: dummy_span(),
                    type_: type_::float(),
                },
                &mut context,
                FloatExpr::add,
            ),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::Invalid
            )),
        );
        assert_eq!(
            super::string_binary_operator(
                int_constant(1),
                string_constant("right"),
                &mut context,
                StringExpr::concatenate,
            ),
            Err(expression_type_error(
                InvalidExpressionType::String,
                InvalidExpressionType::Int,
            )),
        );
        assert_eq!(
            super::string_binary_operator(
                ClauseGuard::Invalid {
                    location: dummy_span(),
                    type_: type_::string(),
                },
                string_constant("right"),
                &mut context,
                StringExpr::concatenate,
            ),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::Invalid
            )),
        );
        assert_eq!(
            super::string_binary_operator(
                string_constant("left"),
                int_constant(1),
                &mut context,
                StringExpr::concatenate,
            ),
            Err(expression_type_error(
                InvalidExpressionType::String,
                InvalidExpressionType::Int,
            )),
        );
    }

    #[test]
    fn plan_tuple_index_rejects_invalid_typed_ast_shapes() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        context.define_int_local("number".into());
        context.define_tuple_local("pair".into(), vec![ValueType::String]);

        assert_eq!(
            plan_expr(
                ClauseGuard::TupleIndex {
                    location: dummy_span(),
                    index: 0,
                    type_: type_::int(),
                    tuple: Box::new(var("number", type_::int())),
                },
                &mut context,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Tuple,
                InvalidExpressionType::Int,
            )),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::TupleIndex {
                    location: dummy_span(),
                    index: 0,
                    type_: type_::result(type_::int(), type_::nil()),
                    tuple: Box::new(var("pair", type_::tuple(vec![type_::string()]))),
                },
                &mut context,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Custom,
                InvalidExpressionType::String,
            )),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::TupleIndex {
                    location: dummy_span(),
                    index: 1,
                    type_: type_::int(),
                    tuple: Box::new(var("pair", type_::tuple(vec![type_::string()]))),
                },
                &mut context,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Int,
                InvalidExpressionType::Tuple,
            )),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::TupleIndex {
                    location: dummy_span(),
                    index: 0,
                    type_: type_::int(),
                    tuple: Box::new(var("pair", type_::tuple(vec![type_::string()]))),
                },
                &mut context,
            ),
            Err(expression_type_error(
                InvalidExpressionType::Int,
                InvalidExpressionType::String,
            )),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::TupleIndex {
                    location: dummy_span(),
                    index: 0,
                    type_: type_::int(),
                    tuple: Box::new(ClauseGuard::Invalid {
                        location: dummy_span(),
                        type_: type_::tuple(vec![type_::int()]),
                    }),
                },
                &mut context,
            ),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::Invalid
            )),
        );
    }

    #[test]
    fn plan_local_handles_non_primitive_value_families() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        context.define_bit_array_local("bits".into());
        context.define_utf_codepoint_local("codepoint".into());
        context.define_nil_local("none".into());
        context.define_tuple_local("pair".into(), vec![ValueType::Int]);
        context.define_list_local("values".into(), ValueType::String);
        context.define_int_function_local(
            "callback".into(),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );

        assert_eq!(
            super::plan_local("bits".into(), &context),
            Ok(Expr::bit_array(BitArrayExpr::local_get(
                BitArrayLocalId(0),
                "bits".into(),
            ))),
        );
        assert_eq!(
            super::plan_local("codepoint".into(), &context),
            Ok(Expr::utf_codepoint(UtfCodepointExpr::local_get(
                UtfCodepointLocalId(0),
                "codepoint".into(),
            ))),
        );
        assert_eq!(
            super::plan_local("none".into(), &context),
            Ok(Expr::nil(NilExpr::local_get(NilLocalId(0), "none".into()))),
        );
        assert_eq!(
            super::plan_local("pair".into(), &context),
            Ok(Expr::tuple(TupleExpr::local_get(
                TupleLocalId(0),
                "pair".into(),
                vec![ValueType::Int],
            ))),
        );
        assert_eq!(
            super::plan_local("values".into(), &context),
            Ok(Expr::list(ListExpr::local_get(
                ListLocal::string(StringListLocalId(0)),
                "values".into(),
            ))),
        );
        assert_eq!(
            super::plan_local("callback".into(), &context),
            Ok(Expr::function(FunctionExpr::int(
                IntFunctionExpr::local_get(
                    IntFunctionLocalId(0),
                    "callback".into(),
                    FunctionType::new(vec![ValueType::Int], ValueType::Int),
                ),
            ))),
        );
        assert_eq!(
            super::plan_local("missing".into(), &context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal {
                    name: "missing".into(),
                },
            }),
        );
        assert_eq!(
            super::local_get(LocalId::Int(IntLocalId(0)), "bad".into(), ValueType::String),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::Invalid
            )),
        );
    }

    #[test]
    fn function_local_get_handles_all_return_families() {
        let unary_int = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let unary_string = FunctionType::new(vec![ValueType::String], ValueType::String);
        let unary_bit_array = FunctionType::new(vec![ValueType::BitArray], ValueType::BitArray);
        let unary_utf_codepoint =
            FunctionType::new(vec![ValueType::UtfCodepoint], ValueType::UtfCodepoint);
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let unary_custom =
            CustomFunctionType::new(vec![ValueType::Custom(custom_type.clone())], custom_type);
        let unary_float = FunctionType::new(vec![ValueType::Float], ValueType::Float);
        let unary_bool = FunctionType::new(vec![ValueType::Bool], ValueType::Bool);
        let unary_nil = FunctionType::new(vec![ValueType::Nil], ValueType::Nil);
        let tuple_type = FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int]));
        let list_type = FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int)));
        let function_type =
            FunctionFunctionType::new(Vec::new(), FunctionType::new(Vec::new(), ValueType::Int));

        assert_eq!(
            function_local_get(
                FunctionLocalBinding::Int {
                    local: IntFunctionLocalId(0),
                    type_: unary_int.clone(),
                },
                "f".into(),
            ),
            Expr::function(FunctionExpr::int(IntFunctionExpr::local_get(
                IntFunctionLocalId(0),
                "f".into(),
                unary_int,
            ))),
        );
        assert_eq!(
            function_local_get(
                FunctionLocalBinding::String {
                    local: StringFunctionLocalId(0),
                    type_: unary_string.clone(),
                },
                "f".into(),
            ),
            Expr::function(FunctionExpr::string(StringFunctionExpr::local_get(
                StringFunctionLocalId(0),
                "f".into(),
                unary_string,
            ))),
        );
        assert_eq!(
            function_local_get(
                FunctionLocalBinding::BitArray {
                    local: BitArrayFunctionLocalId(0),
                    type_: unary_bit_array.clone(),
                },
                "f".into(),
            ),
            Expr::function(FunctionExpr::bit_array(BitArrayFunctionExpr::local_get(
                BitArrayFunctionLocalId(0),
                "f".into(),
                unary_bit_array,
            ))),
        );
        assert_eq!(
            function_local_get(
                FunctionLocalBinding::UtfCodepoint {
                    local: UtfCodepointFunctionLocalId(0),
                    type_: unary_utf_codepoint.clone(),
                },
                "f".into(),
            ),
            Expr::function(FunctionExpr::utf_codepoint(
                UtfCodepointFunctionExpr::local_get(
                    UtfCodepointFunctionLocalId(0),
                    "f".into(),
                    unary_utf_codepoint,
                ),
            )),
        );
        assert_eq!(
            function_local_get(
                FunctionLocalBinding::Custom(CustomFunctionLocal::new(
                    CustomFunctionLocalId(0),
                    unary_custom.clone(),
                )),
                "f".into(),
            ),
            Expr::function(FunctionExpr::custom(CustomFunctionExpr::local_get(
                CustomFunctionLocal::new(CustomFunctionLocalId(0), unary_custom),
                "f".into(),
            ))),
        );
        assert_eq!(
            function_local_get(
                FunctionLocalBinding::Float {
                    local: FloatFunctionLocalId(0),
                    type_: unary_float.clone(),
                },
                "f".into(),
            ),
            Expr::function(FunctionExpr::float(FloatFunctionExpr::local_get(
                FloatFunctionLocalId(0),
                "f".into(),
                unary_float,
            ))),
        );
        assert_eq!(
            function_local_get(
                FunctionLocalBinding::Bool {
                    local: BoolFunctionLocalId(0),
                    type_: unary_bool.clone(),
                },
                "f".into(),
            ),
            Expr::function(FunctionExpr::bool(BoolFunctionExpr::local_get(
                BoolFunctionLocalId(0),
                "f".into(),
                unary_bool,
            ))),
        );
        assert_eq!(
            function_local_get(
                FunctionLocalBinding::Nil {
                    local: NilFunctionLocalId(0),
                    type_: unary_nil.clone(),
                },
                "f".into(),
            ),
            Expr::function(FunctionExpr::nil(NilFunctionExpr::local_get(
                NilFunctionLocalId(0),
                "f".into(),
                unary_nil,
            ))),
        );
        assert_eq!(
            function_local_get(
                FunctionLocalBinding::Tuple {
                    local: TupleFunctionLocalId(0),
                    type_: tuple_type.clone(),
                },
                "f".into(),
            ),
            Expr::function(FunctionExpr::tuple(TupleFunctionExpr::local_get(
                TupleFunctionLocalId(0),
                "f".into(),
                tuple_type,
            ))),
        );
        assert_eq!(
            function_local_get(
                FunctionLocalBinding::List(crate::plan::ListFunctionLocal::from_item_type(
                    0,
                    list_type,
                    ValueType::Int,
                )),
                "f".into(),
            ),
            Expr::function(FunctionExpr::list(ListFunctionExpr::local_get(
                crate::plan::ListFunctionLocal::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                    ),
                    crate::plan::ValueType::Int,
                ),
                "f".into()
            ))),
        );
        assert_eq!(
            function_local_get(
                FunctionLocalBinding::Function(FunctionFunctionLocal::new(
                    FunctionFunctionLocalId(0),
                    function_type.clone(),
                )),
                "f".into(),
            ),
            Expr::function(FunctionExpr::function(FunctionFunctionExpr::local_get(
                FunctionFunctionLocal::new(FunctionFunctionLocalId(0), function_type),
                "f".into(),
            ))),
        );
    }

    #[test]
    fn guard_helper_type_classification_is_exact() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            context.contains_function_value(&ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Int
            ),))),
            Ok(true),
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Tuple(vec![
                ValueType::Int,
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ])),
            Ok(true)
        );
        assert_eq!(
            context.contains_function_value(&ValueType::List(Box::new(ValueType::Function(
                Box::new(FunctionType::new(Vec::new(), ValueType::Int))
            ),))),
            Ok(true)
        );
        assert_eq!(
            context.contains_function_value(&ValueType::Tuple(vec![
                ValueType::Int,
                ValueType::Float,
                ValueType::String,
                ValueType::BitArray,
                ValueType::UtfCodepoint,
                ValueType::Bool,
                ValueType::Nil,
                ValueType::List(Box::new(ValueType::Int)),
            ])),
            Ok(false)
        );

        assert_eq!(
            [
                ValueType::Int,
                ValueType::Float,
                ValueType::String,
                ValueType::BitArray,
                ValueType::UtfCodepoint,
                ValueType::Bool,
                ValueType::Nil,
                ValueType::Tuple(Vec::new()),
                ValueType::List(Box::new(ValueType::Int)),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ]
            .map(invalid_expression_type),
            [
                InvalidExpressionType::Int,
                InvalidExpressionType::Float,
                InvalidExpressionType::String,
                InvalidExpressionType::BitArray,
                InvalidExpressionType::UtfCodepoint,
                InvalidExpressionType::Bool,
                InvalidExpressionType::Nil,
                InvalidExpressionType::Tuple,
                InvalidExpressionType::List,
                InvalidExpressionType::Function,
            ],
        );
    }

    #[test]
    fn guard_equality_preserves_custom_type_definition_errors_from_either_operand() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let custom_name = CustomTypeName::new("geam".into(), module.clone(), "Missing".into());
        context.define_custom_local("missing".into(), CustomType::new(custom_name, Vec::new()));
        let gleam_type = Arc::new(Type::Named {
            publicity: Publicity::Private,
            name: "Missing".into(),
            module: module.clone(),
            package: "geam".into(),
            arguments: Vec::new(),
            inferred_variant: None,
        });
        let expected = Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                name: "Missing".into(),
                reason: crate::planner::InvalidCustomTypeReason::UnknownDefinition,
            },
        });

        assert_eq!(
            super::equality(
                var("missing", gleam_type.clone()),
                int_constant(1),
                &mut context,
                UnsupportedBinOpKind::EqFunction,
                false,
            ),
            expected.clone(),
        );
        assert_eq!(
            super::equality(
                int_constant(1),
                var("missing", gleam_type),
                &mut context,
                UnsupportedBinOpKind::EqFunction,
                false,
            ),
            expected,
        );
    }

    #[test]
    fn equality_rejects_function_value_guard() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        context.define_int_function_local(
            "callback".into(),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        let function_type = type_::fn_(vec![type_::int()], type_::int());

        assert_eq!(
            super::plan_bool(
                ClauseGuard::BinaryOperator {
                    location: dummy_span(),
                    operator: gleam_core::ast::BinOp::Eq,
                    operator_start: 0,
                    left: Box::new(var("callback", function_type.clone())),
                    right: Box::new(var("callback", function_type)),
                },
                &mut context,
            ),
            Err(PlanError::UnsupportedBinOp {
                operator: UnsupportedBinOpKind::EqFunction,
            }),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::BinaryOperator {
                    location: dummy_span(),
                    operator: BinOp::NotEq,
                    operator_start: 0,
                    left: Box::new(int_constant(1)),
                    right: Box::new(var(
                        "callback",
                        type_::fn_(vec![type_::int()], type_::int())
                    )),
                },
                &mut context,
            ),
            Err(PlanError::UnsupportedBinOp {
                operator: UnsupportedBinOpKind::NotEqFunction,
            }),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::BinaryOperator {
                    location: dummy_span(),
                    operator: BinOp::Eq,
                    operator_start: 0,
                    left: Box::new(ClauseGuard::Invalid {
                        location: dummy_span(),
                        type_: type_::int(),
                    }),
                    right: Box::new(int_constant(1)),
                },
                &mut context,
            ),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::Invalid
            )),
        );
        assert_eq!(
            plan_expr(
                ClauseGuard::BinaryOperator {
                    location: dummy_span(),
                    operator: BinOp::Eq,
                    operator_start: 0,
                    left: Box::new(int_constant(1)),
                    right: Box::new(ClauseGuard::Invalid {
                        location: dummy_span(),
                        type_: type_::int(),
                    }),
                },
                &mut context,
            ),
            Err(invalid_expression_shape(
                InvalidExpressionShapeKind::Invalid
            )),
        );
    }

    fn int_constant(value: i64) -> ClauseGuard<Arc<Type>> {
        ClauseGuard::Constant(Constant::Int {
            location: dummy_span(),
            value: value.to_string().into(),
            int_value: value.into(),
        })
    }

    fn int_constant_literal(value: i64) -> Constant<Arc<Type>> {
        Constant::Int {
            location: dummy_span(),
            value: value.to_string().into(),
            int_value: BigInt::from(value),
        }
    }

    fn float_constant() -> ClauseGuard<Arc<Type>> {
        ClauseGuard::Constant(Constant::Float {
            location: dummy_span(),
            value: "1.0".into(),
            float_value: LiteralFloatValue::ONE,
        })
    }

    fn string_constant(value: impl Into<EcoString>) -> ClauseGuard<Arc<Type>> {
        ClauseGuard::Constant(Constant::String {
            location: dummy_span(),
            value: value.into(),
        })
    }

    fn module_select(
        module_name: impl Into<EcoString>,
        literal: Constant<Arc<Type>>,
    ) -> ClauseGuard<Arc<Type>> {
        let module_name = module_name.into();
        ClauseGuard::ModuleSelect {
            location: dummy_span(),
            field_start: 0,
            definition_location: dummy_span(),
            type_: type_::int(),
            label: "answer".into(),
            module_alias: module_name.clone(),
            module_name,
            literal,
        }
    }

    fn binary(
        operator: BinOp,
        left: ClauseGuard<Arc<Type>>,
        right: ClauseGuard<Arc<Type>>,
    ) -> ClauseGuard<Arc<Type>> {
        ClauseGuard::BinaryOperator {
            location: dummy_span(),
            operator,
            operator_start: 0,
            left: Box::new(left),
            right: Box::new(right),
        }
    }

    fn var(name: impl Into<EcoString>, type_: Arc<Type>) -> ClauseGuard<Arc<Type>> {
        ClauseGuard::Var {
            location: dummy_span(),
            type_,
            name: name.into(),
            definition_location: dummy_span(),
            origin: VariableOrigin::generated(),
        }
    }

    fn invalid_expression_shape(kind: InvalidExpressionShapeKind) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape { kind },
        }
    }

    fn expression_type_error(
        expected: InvalidExpressionType,
        actual: InvalidExpressionType,
    ) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType { expected, actual },
        }
    }
}
