use crate::plan::{BoolExpr, Expr, FunctionId, IntExpr, LocalId, NilExpr, StringExpr, ValueType};
use crate::planner::context::{FunctionInfo, PlanContext};
use crate::planner::error::{
    InvalidCallShapeReason, InvalidExpressionShapeKind, InvalidExpressionType,
    InvalidTypedAstReason, PlanError, UnsupportedBinOpKind, UnsupportedCallReason,
    UnsupportedExpressionKind,
};
use ecow::EcoString;
use gleam_core::ast::{BinOp as GleamBinOp, TypedExpr};
use gleam_core::type_::{PRELUDE_MODULE_NAME, Type, ValueConstructor, ValueConstructorVariant};
use std::sync::Arc;

pub(super) fn plan_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match expression {
        TypedExpr::Int { int_value, .. } => Ok(Expr::Int(IntExpr::Value(int_value))),
        TypedExpr::String { value, .. } => Ok(Expr::String(StringExpr::Value(value))),
        TypedExpr::Var {
            constructor, name, ..
        } => plan_var(name, constructor, context),
        TypedExpr::Call {
            type_,
            fun,
            arguments,
            ..
        } => plan_call(type_, *fun, arguments, context),
        TypedExpr::BinOp {
            operator,
            left,
            right,
            ..
        } => plan_bin_op(operator, *left, *right, context),
        TypedExpr::NegateInt { value, .. } => Ok(Expr::Int(IntExpr::Negate(Box::new(
            plan_int_expr(*value, context)?,
        )))),
        TypedExpr::NegateBool { value, .. } => Ok(Expr::Bool(BoolExpr::Not(Box::new(
            plan_bool_expr(*value, context)?,
        )))),
        TypedExpr::Float { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::Float,
        }),
        TypedExpr::Block { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::Block,
        }),
        TypedExpr::Pipeline { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::Pipeline,
        }),
        TypedExpr::Fn { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::AnonymousFunction,
        }),
        TypedExpr::List { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::List,
        }),
        TypedExpr::Case { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::Case,
        }),
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

fn plan_var(
    name: EcoString,
    constructor: ValueConstructor,
    context: &PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match constructor.variant {
        ValueConstructorVariant::LocalVariable { .. } => {
            let local = context
                .lookup_local(&name)
                .ok_or_else(|| PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::UnknownLocal { name: name.clone() },
                })?;
            Ok(local_get(local, name))
        }
        ValueConstructorVariant::Record {
            name,
            module,
            arity,
            ..
        } if arity == 0 && module == PRELUDE_MODULE_NAME => match name.as_str() {
            "True" => Ok(Expr::Bool(BoolExpr::Value(true))),
            "False" => Ok(Expr::Bool(BoolExpr::Value(false))),
            "Nil" => Ok(Expr::Nil(NilExpr::Value)),
            _ => Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::PreludeConstructor,
                },
            }),
        },
        ValueConstructorVariant::ModuleFn { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::FunctionReference,
        }),
        ValueConstructorVariant::ModuleConstant { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::ModuleConstant,
            },
        }),
        ValueConstructorVariant::Record { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::RecordConstructor,
            },
        }),
    }
}

fn plan_call(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<gleam_core::ast::CallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    if arguments.iter().any(|argument| argument.label.is_some()) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LabelledArguments,
            },
        });
    }

    if arguments.iter().any(|argument| argument.implicit.is_some()) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::ImplicitArguments,
            },
        });
    }

    let function = plan_function_ref(fun, context)?;
    if function.arity != arguments.len() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LocalFunctionCallArityMismatch,
            },
        });
    }
    let args = arguments
        .into_iter()
        .map(|argument| plan_expr(argument.value, context))
        .collect::<Result<Vec<_>, _>>()?;

    call_expr(type_.as_ref(), function.id, args)
}

fn plan_function_ref(
    expression: TypedExpr,
    context: &PlanContext<'_>,
) -> Result<FunctionInfo, PlanError> {
    let TypedExpr::Var { constructor, .. } = expression else {
        return Err(PlanError::UnsupportedCall {
            reason: UnsupportedCallReason::NonDirectLocalFunction,
        });
    };

    match constructor.variant {
        ValueConstructorVariant::ModuleFn {
            module,
            name,
            external_erlang,
            external_javascript,
            ..
        } if module == *context.module_name
            && external_erlang.is_none()
            && external_javascript.is_none() =>
        {
            context
                .lookup_function(&name)
                .ok_or(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::MissingCurrentModuleFunction,
                    },
                })
        }
        ValueConstructorVariant::ModuleFn { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::NonCurrentModuleFunction,
            },
        }),
        ValueConstructorVariant::LocalVariable { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LocalFunctionValue,
            },
        }),
        ValueConstructorVariant::ModuleConstant { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::ModuleConstant,
            },
        }),
        ValueConstructorVariant::Record { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::RecordConstructor,
            },
        }),
    }
}

fn plan_bin_op(
    operator: GleamBinOp,
    left: TypedExpr,
    right: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match operator {
        GleamBinOp::AddInt => Ok(Expr::Int(IntExpr::Add {
            left: Box::new(plan_int_expr(left, context)?),
            right: Box::new(plan_int_expr(right, context)?),
        })),
        GleamBinOp::SubInt => Ok(Expr::Int(IntExpr::Sub {
            left: Box::new(plan_int_expr(left, context)?),
            right: Box::new(plan_int_expr(right, context)?),
        })),
        GleamBinOp::MultInt => Ok(Expr::Int(IntExpr::Mult {
            left: Box::new(plan_int_expr(left, context)?),
            right: Box::new(plan_int_expr(right, context)?),
        })),
        GleamBinOp::DivInt => Ok(Expr::Int(IntExpr::Div {
            left: Box::new(plan_int_expr(left, context)?),
            right: Box::new(plan_int_expr(right, context)?),
        })),
        GleamBinOp::RemainderInt => Ok(Expr::Int(IntExpr::Remainder {
            left: Box::new(plan_int_expr(left, context)?),
            right: Box::new(plan_int_expr(right, context)?),
        })),
        GleamBinOp::LtInt => Ok(Expr::Bool(BoolExpr::LtInt {
            left: Box::new(plan_int_expr(left, context)?),
            right: Box::new(plan_int_expr(right, context)?),
        })),
        GleamBinOp::LtEqInt => Ok(Expr::Bool(BoolExpr::LtEqInt {
            left: Box::new(plan_int_expr(left, context)?),
            right: Box::new(plan_int_expr(right, context)?),
        })),
        GleamBinOp::GtInt => Ok(Expr::Bool(BoolExpr::GtInt {
            left: Box::new(plan_int_expr(left, context)?),
            right: Box::new(plan_int_expr(right, context)?),
        })),
        GleamBinOp::GtEqInt => Ok(Expr::Bool(BoolExpr::GtEqInt {
            left: Box::new(plan_int_expr(left, context)?),
            right: Box::new(plan_int_expr(right, context)?),
        })),
        GleamBinOp::Eq => Ok(Expr::Bool(BoolExpr::Equal {
            left: Box::new(plan_expr(left, context)?),
            right: Box::new(plan_expr(right, context)?),
        })),
        GleamBinOp::NotEq => Ok(Expr::Bool(BoolExpr::NotEqual {
            left: Box::new(plan_expr(left, context)?),
            right: Box::new(plan_expr(right, context)?),
        })),
        GleamBinOp::Concatenate => Ok(Expr::String(StringExpr::Concatenate {
            left: Box::new(plan_string_expr(left, context)?),
            right: Box::new(plan_string_expr(right, context)?),
        })),
        GleamBinOp::And => Err(PlanError::UnsupportedBinOp {
            operator: UnsupportedBinOpKind::And,
        }),
        GleamBinOp::Or => Err(PlanError::UnsupportedBinOp {
            operator: UnsupportedBinOpKind::Or,
        }),
        GleamBinOp::LtFloat => Err(PlanError::UnsupportedBinOp {
            operator: UnsupportedBinOpKind::LtFloat,
        }),
        GleamBinOp::LtEqFloat => Err(PlanError::UnsupportedBinOp {
            operator: UnsupportedBinOpKind::LtEqFloat,
        }),
        GleamBinOp::GtEqFloat => Err(PlanError::UnsupportedBinOp {
            operator: UnsupportedBinOpKind::GtEqFloat,
        }),
        GleamBinOp::GtFloat => Err(PlanError::UnsupportedBinOp {
            operator: UnsupportedBinOpKind::GtFloat,
        }),
        GleamBinOp::AddFloat => Err(PlanError::UnsupportedBinOp {
            operator: UnsupportedBinOpKind::AddFloat,
        }),
        GleamBinOp::SubFloat => Err(PlanError::UnsupportedBinOp {
            operator: UnsupportedBinOpKind::SubFloat,
        }),
        GleamBinOp::MultFloat => Err(PlanError::UnsupportedBinOp {
            operator: UnsupportedBinOpKind::MultFloat,
        }),
        GleamBinOp::DivFloat => Err(PlanError::UnsupportedBinOp {
            operator: UnsupportedBinOpKind::DivFloat,
        }),
    }
}

fn local_get(local: LocalId, name: EcoString) -> Expr {
    match local {
        LocalId::Int(local) => Expr::Int(IntExpr::LocalGet { local, name }),
        LocalId::String(local) => Expr::String(StringExpr::LocalGet { local, name }),
        LocalId::Bool(local) => Expr::Bool(BoolExpr::LocalGet { local, name }),
        LocalId::Nil(local) => Expr::Nil(NilExpr::LocalGet { local, name }),
    }
}

fn call_expr(type_: &Type, function: FunctionId, args: Vec<Expr>) -> Result<Expr, PlanError> {
    match ValueType::from_gleam(type_) {
        Some(ValueType::Int) => Ok(Expr::Int(IntExpr::Call { function, args })),
        Some(ValueType::String) => Ok(Expr::String(StringExpr::Call { function, args })),
        Some(ValueType::Bool) => Ok(Expr::Bool(BoolExpr::Call { function, args })),
        Some(ValueType::Nil) => Ok(Expr::Nil(NilExpr::Call { function, args })),
        None => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::LocalFunctionCallUnsupportedReturnType,
            },
        }),
    }
}

fn plan_int_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<IntExpr, PlanError> {
    match plan_expr(expression, context)? {
        Expr::Int(expression) => Ok(expression),
        other => Err(invalid_expression_type(InvalidExpressionType::Int, &other)),
    }
}

fn plan_string_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<StringExpr, PlanError> {
    match plan_expr(expression, context)? {
        Expr::String(expression) => Ok(expression),
        other => Err(invalid_expression_type(
            InvalidExpressionType::String,
            &other,
        )),
    }
}

fn plan_bool_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<BoolExpr, PlanError> {
    match plan_expr(expression, context)? {
        Expr::Bool(expression) => Ok(expression),
        other => Err(invalid_expression_type(InvalidExpressionType::Bool, &other)),
    }
}

fn invalid_expression_type(expected: InvalidExpressionType, actual: &Expr) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected,
            actual: expression_type(actual),
        },
    }
}

fn expression_type(expression: &Expr) -> InvalidExpressionType {
    match expression {
        Expr::Int(_) => InvalidExpressionType::Int,
        Expr::String(_) => InvalidExpressionType::String,
        Expr::Bool(_) => InvalidExpressionType::Bool,
        Expr::Nil(_) => InvalidExpressionType::Nil,
    }
}

#[cfg(test)]
mod tests {
    use crate::planner::dsl::{bool_, call, function, int, local, module, nil, string};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidCallShapeReason, InvalidExpressionShapeKind, InvalidExpressionType,
        InvalidTypedAstReason, PlanError, UnsupportedBinOpKind, UnsupportedCallReason,
        UnsupportedExpressionKind,
    };
    use gleam_core::ast::Publicity;
    use gleam_core::ast::{
        CallArg, Constant, ImplicitCallArgOrigin, Statement, TypedExpr, TypedModule, TypedStatement,
    };
    use gleam_core::type_::error::VariableOrigin;
    use gleam_core::type_::{
        self, Deprecation, ModuleValueConstructor, PRELUDE_MODULE_NAME, ValueConstructor,
        ValueConstructorVariant,
    };
    use num_bigint::BigInt;

    #[test]
    fn plan_string_concatenation() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  "hello, " <> "geam"
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main")
            .function(function("main").return_(string("hello, ").concatenate(string("geam"))))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_integer_comparisons() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1 < 2
}

pub fn lte() {
  1 <= 2
}

pub fn gt() {
  2 > 1
}

pub fn gte() {
  2 >= 1
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main")
            .function(function("main").return_(int(1).lt_int(int(2))))
            .function(function("lte").return_(int(1).lte_int(int(2))))
            .function(function("gt").return_(int(2).gt_int(int(1))))
            .function(function("gte").return_(int(2).gte_int(int(1))))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_integer_division_and_remainder() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  11 / 3
}

pub fn remainder() {
  11 % 3
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main")
            .function(function("main").return_(int(11).div_int(int(3))))
            .function(function("remainder").return_(int(11).remainder_int(int(3))))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_negation_expressions() {
        let actual = plan_module(compile(
            r#"
pub fn negate(value: Int) {
  -value
}

pub fn invert(value: Bool) {
  !value
}

pub fn main() {
  negate(1)
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main")
            .function(
                function("negate")
                    .param("value")
                    .return_(local("value").negate_int()),
            )
            .function(
                function("invert")
                    .param_bool("value")
                    .return_(local("value").negate_bool()),
            )
            .function(function("main").return_(call("negate", [int(1)])))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bool_and_nil_constructors() {
        let actual = plan_module(compile(
            r#"
pub fn truth() {
  True
}

pub fn falsehood() {
  False
}

pub fn main() {
  Nil
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main")
            .function(function("truth").return_(bool_(true)))
            .function(function("falsehood").return_(bool_(false)))
            .function(function("main").return_(nil()))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_pipeline_expression() {
        assert_eq!(
            expect_plan_error(
                r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  1 |> identity
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Pipeline,
            },
        );
    }

    #[test]
    fn reject_profile_list_expression() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  [1, 2, 3]
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::List,
            },
        );
    }

    #[test]
    fn reject_profile_expression_variants() {
        let cases = [
            (
                r#"pub fn main() { 1.0 }"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::Float,
                },
            ),
            (
                r#"pub fn main() { { 1 } }"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::Block,
                },
            ),
            (
                r#"pub fn main() { fn(x) { x } }"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::AnonymousFunction,
                },
            ),
            (
                r#"pub fn main() { fn(x) { x }(1) }"#,
                PlanError::UnsupportedCall {
                    reason: UnsupportedCallReason::NonDirectLocalFunction,
                },
            ),
            (
                r#"pub fn main() { case 1 { 1 -> 2 _ -> 3 } }"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::Case,
                },
            ),
            (
                r#"pub fn main() { #(1, 2) }"#,
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
                r#"pub fn main() { todo }"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::Todo,
                },
            ),
            (
                r#"pub fn main() { panic }"#,
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
                r#"pub fn main() { <<1>> }"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::BitArray,
                },
            ),
            (
                r#"fn identity(value: Int) { value } pub fn main() { identity }"#,
                PlanError::UnsupportedExpression {
                    kind: UnsupportedExpressionKind::FunctionReference,
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
    fn reject_margin_expression_type_mismatch() {
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::BinOp {
                location: dummy_span(),
                type_: type_::int(),
                operator: gleam_core::ast::BinOp::AddInt,
                operator_start: 0,
                left: Box::new(typed_string_expr("bad")),
                right: Box::new(typed_int_expr(1)),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::String,
                },
            }),
        );
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::BinOp {
                location: dummy_span(),
                type_: type_::string(),
                operator: gleam_core::ast::BinOp::Concatenate,
                operator_start: 0,
                left: Box::new(typed_int_expr(1)),
                right: Box::new(typed_string_expr("bad")),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::NegateBool {
                location: dummy_span(),
                value: Box::new(typed_int_expr(1)),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Bool,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::BinOp {
                location: dummy_span(),
                type_: type_::int(),
                operator: gleam_core::ast::BinOp::AddInt,
                operator_start: 0,
                left: Box::new(typed_prelude_constructor("True", type_::bool())),
                right: Box::new(typed_int_expr(1)),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::Bool,
                },
            }),
        );
        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::BinOp {
                location: dummy_span(),
                type_: type_::int(),
                operator: gleam_core::ast::BinOp::AddInt,
                operator_start: 0,
                left: Box::new(typed_prelude_constructor("Nil", type_::nil())),
                right: Box::new(typed_int_expr(1)),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::Nil,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_value_constructor_variants() {
        let mut unbound_local = compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        );
        let variable = unbound_local.definitions.functions[0].body.remove(1);
        unbound_local.definitions.functions[0].body = vec![variable];
        assert_eq!(
            plan_module(unbound_local),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal { name: "x".into() },
            }),
        );

        let mut module_constant = compile(
            r#"
const answer = 1

pub fn main() {
  answer
}
"#,
        );
        module_constant.definitions.constants.clear();
        assert_eq!(
            plan_module(module_constant),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::ModuleConstant,
                },
            }),
        );

        let mut record_constructor = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed
}
"#,
        );
        record_constructor.definitions.custom_types.clear();
        assert_eq!(
            plan_module(record_constructor),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );

        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::Var {
                location: dummy_span(),
                name: "Other".into(),
                constructor: ValueConstructor {
                    publicity: Publicity::Private,
                    deprecation: Deprecation::NotDeprecated,
                    type_: type_::bool(),
                    variant: ValueConstructorVariant::Record {
                        name: "Other".into(),
                        arity: 0,
                        field_map: None,
                        location: dummy_span(),
                        module: PRELUDE_MODULE_NAME.into(),
                        variants_count: 1,
                        variant_index: 0,
                        documentation: None,
                    },
                },
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::PreludeConstructor,
                },
            }),
        );

        let module_constant_call = compile(
            r#"
const answer = 1

pub fn main() {
  answer
}
"#,
        );
        reject_margin_module_constant_call(module_constant_call);
    }

    #[test]
    fn reject_profile_local_function_value_call() {
        assert_eq!(
            expect_plan_error(
                r#"
fn apply(callback: fn(Int) -> Int) {
  callback(1)
}

pub fn main() {
  1
}
"#,
            ),
            PlanError::UnsupportedArgument {
                function: "apply".into(),
                reason: crate::planner::UnsupportedArgumentReason::UnsupportedType,
            },
        );
    }

    fn reject_margin_module_constant_call(mut module_constant_call: TypedModule) {
        module_constant_call.definitions.constants.clear();
        let statement = module_constant_call.definitions.functions[0].body.remove(0);
        let Statement::Expression(module_constant) = statement else {
            panic!("expected expression statement");
        };
        module_constant_call.definitions.functions[0].body =
            vec![Statement::Expression(TypedExpr::Call {
                location: dummy_span(),
                type_: type_::int(),
                fun: Box::new(module_constant),
                arguments: Vec::new(),
                open_parenthesis: Some(0),
            })];
        assert_eq!(
            plan_module(module_constant_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::ModuleConstant,
                },
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected expression statement")]
    fn reject_margin_module_constant_call_panics_on_assignment_statement() {
        reject_margin_module_constant_call(compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        ));
    }

    #[test]
    fn reject_margin_call_shapes() {
        let mut labelled_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut labelled_call.definitions.functions[1].body[0]);
        arguments[0].label = Some("value".into());
        assert_eq!(
            plan_module(labelled_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LabelledArguments,
                },
            }),
        );

        let mut implicit_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut implicit_call.definitions.functions[1].body[0]);
        arguments[0].implicit = Some(ImplicitCallArgOrigin::Pipe);
        assert_eq!(
            plan_module(implicit_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::ImplicitArguments,
                },
            }),
        );

        let mut arity_mismatch_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, _, arguments) =
            expect_call_statement_mut(&mut arity_mismatch_call.definitions.functions[1].body[0]);
        arguments.clear();
        assert_eq!(
            plan_module(arity_mismatch_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionCallArityMismatch,
                },
            }),
        );

        let mut unsupported_return_type_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (type_, _, _) = expect_call_statement_mut(
            &mut unsupported_return_type_call.definitions.functions[1].body[0],
        );
        *type_ = type_::tuple(vec![type_::int()]);
        assert_eq!(
            plan_module(unsupported_return_type_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionCallUnsupportedReturnType,
                },
            }),
        );

        let mut local_function_value_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, fun, _) = expect_call_statement_mut(
            &mut local_function_value_call.definitions.functions[1].body[0],
        );
        let constructor = expect_var_constructor_mut(fun);
        constructor.variant = ValueConstructorVariant::LocalVariable {
            location: dummy_span(),
            origin: VariableOrigin::generated(),
        };
        assert_eq!(
            plan_module(local_function_value_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LocalFunctionValue,
                },
            }),
        );

        let mut missing_current_module_fn = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        missing_current_module_fn.definitions.functions.remove(0);
        assert_eq!(
            plan_module(missing_current_module_fn),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::MissingCurrentModuleFunction,
                },
            }),
        );

        let non_local_module_fn = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        reject_margin_non_local_module_fn_call(non_local_module_fn);

        let mut record_constructor_call = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed(1)
}
"#,
        );
        record_constructor_call.definitions.custom_types.clear();
        assert_eq!(
            plan_module(record_constructor_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::RecordConstructor,
                },
            }),
        );
    }

    fn reject_margin_non_local_module_fn_call(mut non_local_module_fn: TypedModule) {
        let function = non_local_module_fn
            .definitions
            .functions
            .last_mut()
            .expect("expected test module to have a function");
        let (_, fun, _) = expect_call_statement_mut(&mut function.body[0]);
        let constructor = expect_var_constructor_mut(fun);
        let ValueConstructorVariant::ModuleFn { module, .. } = &mut constructor.variant else {
            panic!("expected module function constructor");
        };
        *module = "other".into();
        assert_eq!(
            plan_module(non_local_module_fn),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::NonCurrentModuleFunction,
                },
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected module function constructor")]
    fn reject_margin_non_local_module_fn_call_panics_on_record_constructor() {
        let record_constructor_call = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed(1)
}
"#,
        );
        reject_margin_non_local_module_fn_call(record_constructor_call);
    }

    #[test]
    fn reject_profile_binary_operators() {
        let cases = [
            (
                r#"pub fn main() { True && False }"#,
                UnsupportedBinOpKind::And,
            ),
            (
                r#"pub fn main() { True || False }"#,
                UnsupportedBinOpKind::Or,
            ),
            (
                r#"pub fn main() { 1.0 <. 2.0 }"#,
                UnsupportedBinOpKind::LtFloat,
            ),
            (
                r#"pub fn main() { 1.0 <=. 2.0 }"#,
                UnsupportedBinOpKind::LtEqFloat,
            ),
            (
                r#"pub fn main() { 1.0 >=. 2.0 }"#,
                UnsupportedBinOpKind::GtEqFloat,
            ),
            (
                r#"pub fn main() { 1.0 >. 2.0 }"#,
                UnsupportedBinOpKind::GtFloat,
            ),
            (
                r#"pub fn main() { 1.0 +. 2.0 }"#,
                UnsupportedBinOpKind::AddFloat,
            ),
            (
                r#"pub fn main() { 1.0 -. 2.0 }"#,
                UnsupportedBinOpKind::SubFloat,
            ),
            (
                r#"pub fn main() { 1.0 *. 2.0 }"#,
                UnsupportedBinOpKind::MultFloat,
            ),
            (
                r#"pub fn main() { 1.0 /. 2.0 }"#,
                UnsupportedBinOpKind::DivFloat,
            ),
        ];

        for (src, expected) in cases {
            assert_eq!(
                expect_plan_error(src),
                PlanError::UnsupportedBinOp { operator: expected },
            );
        }
    }

    fn module_returning_typed_expr(expression: TypedExpr) -> TypedModule {
        let mut module = compile_minimal_module();
        module.definitions.functions[0].body = vec![Statement::Expression(expression)];
        module
    }

    fn expect_call_statement_mut(
        statement: &mut TypedStatement,
    ) -> (
        &mut std::sync::Arc<type_::Type>,
        &mut TypedExpr,
        &mut Vec<CallArg<TypedExpr>>,
    ) {
        let Statement::Expression(TypedExpr::Call {
            type_,
            fun,
            arguments,
            ..
        }) = statement
        else {
            panic!("expected call expression statement");
        };
        (type_, fun.as_mut(), arguments)
    }

    #[test]
    #[should_panic(expected = "expected call expression statement")]
    fn expect_call_statement_mut_panics_on_expression() {
        let mut module = compile_minimal_module();

        expect_call_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    fn expect_var_constructor_mut(expression: &mut TypedExpr) -> &mut ValueConstructor {
        let TypedExpr::Var { constructor, .. } = expression else {
            panic!("expected variable expression");
        };
        constructor
    }

    #[test]
    #[should_panic(expected = "expected variable expression")]
    fn expect_var_constructor_mut_panics_on_int() {
        let mut expression = typed_int_expr(1);

        expect_var_constructor_mut(&mut expression);
    }

    fn typed_int_expr(value: i64) -> TypedExpr {
        TypedExpr::Int {
            location: dummy_span(),
            type_: type_::int(),
            value: value.to_string().into(),
            int_value: BigInt::from(value),
        }
    }

    fn typed_string_expr(value: &str) -> TypedExpr {
        TypedExpr::String {
            location: dummy_span(),
            type_: type_::string(),
            value: value.into(),
        }
    }

    fn typed_prelude_constructor(name: &str, type_: std::sync::Arc<type_::Type>) -> TypedExpr {
        TypedExpr::Var {
            location: dummy_span(),
            name: name.into(),
            constructor: ValueConstructor {
                publicity: Publicity::Private,
                deprecation: Deprecation::NotDeprecated,
                type_,
                variant: ValueConstructorVariant::Record {
                    name: name.into(),
                    arity: 0,
                    field_map: None,
                    location: dummy_span(),
                    module: PRELUDE_MODULE_NAME.into(),
                    variants_count: 1,
                    variant_index: 0,
                    documentation: None,
                },
            },
        }
    }
}
