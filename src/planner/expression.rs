mod block;
mod call;
mod case;
mod function;
mod operator;
mod pipeline;
mod var;

use crate::plan::{BoolExpr, Expr, IntExpr, StringExpr, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    UnsupportedExpressionKind,
};
use gleam_core::ast::TypedExpr;

pub(super) fn plan_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match expression {
        TypedExpr::Int { int_value, .. } => Ok(Expr::int(IntExpr::value(int_value))),
        TypedExpr::String { value, .. } => Ok(Expr::string(StringExpr::value(value))),
        TypedExpr::Var {
            constructor, name, ..
        } => var::plan_var(name, constructor, context),
        TypedExpr::Call {
            type_,
            fun,
            arguments,
            ..
        } => call::plan_call(type_, *fun, arguments, context),
        TypedExpr::BinOp {
            operator,
            left,
            right,
            ..
        } => operator::plan_bin_op(operator, *left, *right, context),
        TypedExpr::NegateInt { value, .. } => operator::plan_negate_int(*value, context),
        TypedExpr::NegateBool { value, .. } => operator::plan_negate_bool(*value, context),
        TypedExpr::Float { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::Float,
        }),
        TypedExpr::Block { statements, .. } => block::plan(statements, context),
        TypedExpr::Pipeline {
            first_value,
            assignments,
            finally,
            finally_kind,
            ..
        } => pipeline::plan(first_value, assignments, *finally, finally_kind, context),
        TypedExpr::Fn {
            type_,
            kind,
            arguments,
            body,
            ..
        } => function::plan_anonymous(type_, kind, arguments, body, context),
        TypedExpr::List { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::List,
        }),
        TypedExpr::Case {
            type_,
            subjects,
            clauses,
            ..
        } => case::plan_case(type_, subjects, clauses, context),
        TypedExpr::RecordAccess { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::RecordAccess,
            },
        }),
        TypedExpr::PositionalAccess { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::PositionalAccess,
            },
        }),
        TypedExpr::ModuleSelect { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::ModuleSelect,
            },
        }),
        TypedExpr::Tuple { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::Tuple,
        }),
        TypedExpr::TupleIndex { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::TupleIndex,
        }),
        TypedExpr::Todo { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::Todo,
        }),
        TypedExpr::Panic { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::Panic,
        }),
        TypedExpr::Echo { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::Echo,
        }),
        TypedExpr::BitArray { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::BitArray,
        }),
        TypedExpr::RecordUpdate { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::RecordUpdate,
            },
        }),
        TypedExpr::Invalid { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        }),
    }
}

pub(super) fn plan_use_call(
    call: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    call::plan_use_call(call, context)
}

fn plan_int_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<IntExpr, PlanError> {
    let expression = plan_expr(expression, context)?;
    let actual = expression_type(&expression);
    expression
        .into_int()
        .ok_or_else(|| invalid_expression_type(InvalidExpressionType::Int, actual))
}

fn plan_string_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<StringExpr, PlanError> {
    let expression = plan_expr(expression, context)?;
    let actual = expression_type(&expression);
    expression
        .into_string()
        .ok_or_else(|| invalid_expression_type(InvalidExpressionType::String, actual))
}

fn plan_bool_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<BoolExpr, PlanError> {
    let expression = plan_expr(expression, context)?;
    let actual = expression_type(&expression);
    expression
        .into_bool()
        .ok_or_else(|| invalid_expression_type(InvalidExpressionType::Bool, actual))
}

fn invalid_expression_type(
    expected: InvalidExpressionType,
    actual: InvalidExpressionType,
) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType { expected, actual },
    }
}

fn invalid_expression_type_for_value(expected: ValueType, actual: ValueType) -> PlanError {
    invalid_expression_type(
        value_type_expression_type(expected),
        value_type_expression_type(actual),
    )
}

fn expression_type(expression: &Expr) -> InvalidExpressionType {
    match expression.value_type() {
        ValueType::Int => InvalidExpressionType::Int,
        ValueType::String => InvalidExpressionType::String,
        ValueType::Bool => InvalidExpressionType::Bool,
        ValueType::Nil => InvalidExpressionType::Nil,
        ValueType::Function(_) => InvalidExpressionType::Function,
    }
}

fn value_type_expression_type(type_: ValueType) -> InvalidExpressionType {
    match type_ {
        ValueType::Int => InvalidExpressionType::Int,
        ValueType::String => InvalidExpressionType::String,
        ValueType::Bool => InvalidExpressionType::Bool,
        ValueType::Nil => InvalidExpressionType::Nil,
        ValueType::Function(_) => InvalidExpressionType::Function,
    }
}

#[cfg(test)]
pub(in crate::planner::expression) fn module_returning_typed_expr(
    expression: TypedExpr,
) -> gleam_core::ast::TypedModule {
    let mut module = crate::planner::support::compile_minimal_module();
    module.definitions.functions[0].body = vec![gleam_core::ast::Statement::Expression(expression)];
    module
}

#[cfg(test)]
pub(in crate::planner::expression) fn typed_int_expr(value: i64) -> TypedExpr {
    use num_bigint::BigInt;

    TypedExpr::Int {
        location: crate::planner::support::dummy_span(),
        type_: gleam_core::type_::int(),
        value: value.to_string().into(),
        int_value: BigInt::from(value),
    }
}

#[cfg(test)]
pub(in crate::planner::expression) fn typed_string_expr(value: &str) -> TypedExpr {
    TypedExpr::String {
        location: crate::planner::support::dummy_span(),
        type_: gleam_core::type_::string(),
        value: value.into(),
    }
}

#[cfg(test)]
pub(in crate::planner::expression) fn typed_prelude_constructor(
    name: &str,
    type_: std::sync::Arc<gleam_core::type_::Type>,
) -> TypedExpr {
    use gleam_core::ast::Publicity;
    use gleam_core::type_::{
        Deprecation, PRELUDE_MODULE_NAME, ValueConstructor, ValueConstructorVariant,
    };

    TypedExpr::Var {
        location: crate::planner::support::dummy_span(),
        name: name.into(),
        constructor: ValueConstructor {
            publicity: Publicity::Private,
            deprecation: Deprecation::NotDeprecated,
            type_,
            variant: ValueConstructorVariant::Record {
                name: name.into(),
                arity: 0,
                field_map: None,
                location: crate::planner::support::dummy_span(),
                module: PRELUDE_MODULE_NAME.into(),
                variants_count: 1,
                variant_index: 0,
                documentation: None,
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::{
        expression_type, invalid_expression_type, module_returning_typed_expr, typed_int_expr,
    };
    use crate::plan::{Expr, FunctionExpr, FunctionValue, NilFunctionId, RuntimeFunctionId};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionShapeKind, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedExpressionKind,
    };
    use gleam_core::ast::{Constant, TypedExpr};
    use gleam_core::type_::{self, ModuleValueConstructor};
    use num_bigint::BigInt;

    #[test]
    fn reject_profile_expression_variants() {
        let cases = [
            (
                r#"
pub fn main() {
  1.0
  1
}
"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::Float,
                },
            ),
            (
                r#"
pub fn main() {
  [1, 2, 3]
  1
}
"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::List,
                },
            ),
            (
                r#"
pub fn main() {
  #(1, 2)
  1
}
"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::Tuple,
                },
            ),
            (
                r#"pub fn main() { #(1, 2).0 }"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::TupleIndex,
                },
            ),
            (
                r#"
pub fn main() {
  todo
  1
}
"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::Todo,
                },
            ),
            (
                r#"
pub fn main() {
  panic
  1
}
"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::Panic,
                },
            ),
            (
                r#"pub fn main() { echo 1 }"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::Echo,
                },
            ),
            (
                r#"
pub fn main() {
  <<1>>
  1
}
"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::BitArray,
                },
            ),
        ];

        for (src, expected) in cases {
            assert_eq!(expect_plan_error(src), expected);
        }
    }

    #[test]
    fn reject_margin_expression_shapes() {
        let synthetic_cases = [
            (
                module_returning_typed_expr(TypedExpr::PositionalAccess {
                    location: dummy_span(),
                    type_: type_::int(),
                    index: 0,
                    record: Box::new(typed_int_expr(1)),
                }),
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::PositionalAccess,
                    },
                },
            ),
            (
                module_returning_typed_expr(TypedExpr::ModuleSelect {
                    location: dummy_span(),
                    field_start: 0,
                    type_: type_::int(),
                    label: "answer".into(),
                    module_name: "other".into(),
                    module_alias: "other".into(),
                    constructor: ModuleValueConstructor::Constant {
                        literal: Constant::Int {
                            location: dummy_span(),
                            value: "1".into(),
                            int_value: BigInt::from(1),
                        },
                        location: dummy_span(),
                        documentation: None,
                    },
                }),
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::ModuleSelect,
                    },
                },
            ),
        ];

        for (module, expected) in synthetic_cases {
            assert_eq!(plan_module(module), Err(expected));
        }

        let mut record_access = compile(
            r#"
pub type Boxed {
  Boxed(value: Int)
}

pub fn main() {
  Boxed(1).value
}
"#,
        );
        record_access.definitions.custom_types.clear();
        assert_eq!(
            plan_module(record_access),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordAccess,
                },
            }),
        );

        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::RecordUpdate {
                location: dummy_span(),
                spread_start: 0,
                type_: type_::int(),
                updated_record: Box::new(typed_int_expr(1)),
                updated_record_assigned_name: None,
                constructor: Box::new(typed_int_expr(1)),
                arguments: Vec::new(),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordUpdate,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_invalid_expression() {
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::Invalid {
                location: dummy_span(),
                type_: type_::int(),
                extra_information: None,
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_function_expression_type() {
        let expression = Expr::function(FunctionExpr::value(FunctionValue::new(
            RuntimeFunctionId::Nil(NilFunctionId(0)),
            Vec::new(),
        )));

        assert_eq!(
            invalid_expression_type(InvalidExpressionType::Int, expression_type(&expression)),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::Function,
                },
            },
        );
    }
}
