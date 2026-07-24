use ecow::EcoString;
use gleam_core::ast::TypedExpr;

use super::{plan_expr, plan_string_expr};
use crate::plan::{
    BitArrayExpr, BoolExpr, CustomExpr, CustomFunctionExpr, CustomLocal, CustomLocalExpr,
    EchoSubject, Expr, ExprKind, FloatExpr, FunctionExpr, FunctionFunctionExpr, GenericExpr,
    GenericFunctionExpr, IntExpr, IntFunctionExpr, ListExpr, ListFunctionExpr, NilExpr, Step,
    StringExpr, StringFunctionExpr, TupleExpr, TupleFunctionExpr, TypedFunctionExprKind,
    UtfCodepointExpr, UtfCodepointFunctionExpr,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError};

pub(super) fn plan(
    location: gleam_core::ast::SrcSpan,
    expression: Option<TypedExpr>,
    message: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let Some(expression) = expression else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        });
    };
    let value = plan_expr(expression, context)?;

    plan_value(location, value, message, context)
}

pub(super) fn plan_value(
    location: gleam_core::ast::SrcSpan,
    value: Expr,
    message: Option<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let message = message
        .map(|message| plan_string_expr(message, context))
        .transpose()?;
    let site = context.echo_site(location);
    let (subject, result) = store_subject(value, context);

    Ok(Expr::block(
        vec![Step::echo(subject, message, site)],
        result,
    ))
}

fn store_subject(value: Expr, context: &mut PlanContext<'_>) -> (EchoSubject, Expr) {
    let name = EcoString::from("<echo>");
    match value.into_kind() {
        ExprKind::Generic(value) => {
            let local = context.define_internal_generic_local(value.parameter());
            (
                EchoSubject::Generic { local, value },
                Expr::generic(GenericExpr::local_get(local, name)),
            )
        }
        ExprKind::Int(value) => {
            let local = context.define_internal_int_local();
            (
                EchoSubject::Int { local, value },
                Expr::int(IntExpr::local_get(local, name)),
            )
        }
        ExprKind::Float(value) => {
            let local = context.define_internal_float_local();
            (
                EchoSubject::Float { local, value },
                Expr::float(FloatExpr::local_get(local, name)),
            )
        }
        ExprKind::String(value) => {
            let local = context.define_internal_string_local();
            (
                EchoSubject::String { local, value },
                Expr::string(StringExpr::local_get(local, name)),
            )
        }
        ExprKind::BitArray(value) => {
            let local = context.define_internal_bit_array_local();
            (
                EchoSubject::BitArray { local, value },
                Expr::bit_array(BitArrayExpr::local_get(local, name)),
            )
        }
        ExprKind::UtfCodepoint(value) => {
            let local = context.define_internal_utf_codepoint_local();
            (
                EchoSubject::UtfCodepoint { local, value },
                Expr::utf_codepoint(UtfCodepointExpr::local_get(local, name)),
            )
        }
        ExprKind::Custom(value) => {
            let shape = value.shape().clone();
            let local = context.define_internal_custom_local();
            let typed_local = CustomLocal::from_shape(local, shape);
            (
                EchoSubject::Custom(CustomLocalExpr::from_value(local, value)),
                Expr::custom(CustomExpr::local_get(typed_local, name)),
            )
        }
        ExprKind::Bool(value) => {
            let local = context.define_internal_bool_local();
            (
                EchoSubject::Bool { local, value },
                Expr::bool(BoolExpr::local_get(local, name)),
            )
        }
        ExprKind::Nil(value) => {
            let local = context.define_internal_nil_local();
            (
                EchoSubject::Nil { local, value },
                Expr::nil(NilExpr::local_get(local, name)),
            )
        }
        ExprKind::Tuple(value) => {
            let shape = value.shape().to_vec().into_boxed_slice();
            let type_ = value.type_().to_vec();
            let local = context.define_internal_tuple_local();
            (
                EchoSubject::Tuple { local, value },
                Expr::tuple(TupleExpr::local_get(local, name, type_).with_shape(shape)),
            )
        }
        ExprKind::List(value) => {
            let item_shape = value.item_shape().clone();
            let (local, value) = context.define_internal_list_value(value);
            (
                EchoSubject::List(value),
                Expr::list(ListExpr::local_get(local, name).with_item_shape(item_shape)),
            )
        }
        ExprKind::Function(value) => {
            let (subject, result) = match value.into_typed_kind() {
                TypedFunctionExprKind::Generic(value) => {
                    let shape = value.shape().clone();
                    let local = context
                        .define_internal_generic_function_local(value.expression().type_().clone());
                    (
                        EchoSubject::GenericFunction {
                            local: local.clone(),
                            value,
                        },
                        FunctionExpr::generic_with_shape(
                            GenericFunctionExpr::local_get(local, name),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::Int(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_internal_int_function_local();
                    (
                        EchoSubject::IntFunction { local, value },
                        FunctionExpr::int_with_shape(
                            IntFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::Float(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_internal_float_function_local();
                    (
                        EchoSubject::FloatFunction { local, value },
                        FunctionExpr::float_with_shape(
                            crate::plan::FloatFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::String(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_internal_string_function_local();
                    (
                        EchoSubject::StringFunction { local, value },
                        FunctionExpr::string_with_shape(
                            StringFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::BitArray(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_internal_bit_array_function_local();
                    (
                        EchoSubject::BitArrayFunction { local, value },
                        FunctionExpr::bit_array_with_shape(
                            crate::plan::BitArrayFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::UtfCodepoint(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_internal_utf_codepoint_function_local();
                    (
                        EchoSubject::UtfCodepointFunction { local, value },
                        FunctionExpr::utf_codepoint_with_shape(
                            UtfCodepointFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::Custom(value) => {
                    let local = context.define_internal_custom_function_local(
                        value.expression().custom_function_type().clone(),
                    );
                    (
                        EchoSubject::CustomFunction {
                            local: local.clone(),
                            value,
                        },
                        FunctionExpr::custom(CustomFunctionExpr::local_get(local, name)),
                    )
                }
                TypedFunctionExprKind::Bool(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_internal_bool_function_local();
                    (
                        EchoSubject::BoolFunction { local, value },
                        FunctionExpr::bool_with_shape(
                            crate::plan::BoolFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::Nil(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_internal_nil_function_local();
                    (
                        EchoSubject::NilFunction { local, value },
                        FunctionExpr::nil_with_shape(
                            crate::plan::NilFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::Tuple(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_internal_tuple_function_local();
                    (
                        EchoSubject::TupleFunction { local, value },
                        FunctionExpr::tuple_with_shape(
                            TupleFunctionExpr::local_get(local, name, type_),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::List(value) => {
                    let shape = value.shape().clone();
                    let type_ = shape.type_();
                    let local = context.define_internal_list_function_local(
                        type_,
                        value.expression().return_item_type(),
                    );
                    (
                        EchoSubject::ListFunction {
                            local: local.clone(),
                            value,
                        },
                        FunctionExpr::list_with_shape(
                            ListFunctionExpr::local_get(local, name),
                            shape,
                        ),
                    )
                }
                TypedFunctionExprKind::Function(value) => {
                    let shape = value.shape().clone();
                    let local = context.define_internal_function_function_local(
                        value.expression().function_function_type().clone(),
                    );
                    (
                        EchoSubject::FunctionFunction {
                            local: local.clone(),
                            value,
                        },
                        FunctionExpr::function_with_shape(
                            FunctionFunctionExpr::local_get(local, name),
                            shape,
                        ),
                    )
                }
            };
            (subject, Expr::function(result))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::plan;
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError,
        UnsupportedBitArraySegmentReason,
    };
    use gleam_core::ast::TypedExpr;
    use gleam_core::type_;
    use std::collections::HashMap;

    #[test]
    fn rejects_missing_subject_expression() {
        let module = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            plan(dummy_span(), None, None, &mut context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    fn plans_subject_before_message() {
        let module = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);

        assert_eq!(
            plan(
                dummy_span(),
                Some(TypedExpr::Invalid {
                    location: dummy_span(),
                    type_: type_::int(),
                    extra_information: None,
                }),
                Some(TypedExpr::Invalid {
                    location: dummy_span(),
                    type_: type_::string(),
                    extra_information: None,
                }),
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    fn preserves_message_planning_errors() {
        let source = r#"
pub fn main() {
  echo 1 as {
    <<1:native>>
    "selected"
  }
}
"#;

        assert_eq!(
            expect_plan_error(source),
            PlanError::UnsupportedBitArraySegment {
                reason: UnsupportedBitArraySegmentReason::NativeEndianness,
            },
        );
    }
}
