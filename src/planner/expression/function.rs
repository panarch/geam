use crate::plan::{Expr, FunctionExpr, FunctionType, ValueType};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidExpressionType, InvalidFunctionShapeReason,
    InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
};
use crate::planner::function::{anonymous_function_plan, plan_anonymous_function_body};
use crate::planner::module::function_params;
use gleam_core::ast::{FunctionLiteralKind, TypedArg, TypedStatement};
use gleam_core::type_::Type;
use std::sync::Arc;
use vec1::Vec1;

pub(super) fn plan_anonymous(
    type_: Arc<Type>,
    kind: FunctionLiteralKind,
    arguments: Vec<TypedArg>,
    body: Vec1<TypedStatement>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match kind {
        FunctionLiteralKind::Anonymous { .. } => {}
        FunctionLiteralKind::Capture { .. } => {
            return Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::FunctionCaptureLiteral,
            });
        }
        FunctionLiteralKind::Use { .. } => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::error::InvalidExpressionShapeKind::Invalid,
                },
            });
        }
    }

    let function_type = anonymous_function_type(type_.as_ref())?;
    let error_name = context.anonymous_function_error_name();
    let params = function_params(error_name.clone(), &arguments)?;
    validate_argument_types(&error_name, &function_type, &params)?;

    let planned = {
        let mut body_context = context.anonymous_function_context();
        plan_anonymous_function_body(&params, body, &mut body_context)
    };

    match planned {
        Ok(planned) => {
            let (name, info) =
                context.allocate_anonymous_function(function_type.return_().clone(), params);
            let value = info.value();
            let function = anonymous_function_plan(info, name, planned)?;
            context.push_anonymous_function(function);
            Ok(Expr::function(FunctionExpr::value(value)))
        }
        Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::UnknownLocal { name },
        }) if context.is_outer_binding_name(&name) => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::CapturingClosure,
        }),
        Err(error) => Err(error),
    }
}

fn anonymous_function_type(type_: &Type) -> Result<FunctionType, PlanError> {
    match ValueType::from_gleam(type_) {
        Some(ValueType::Function(type_)) => Ok(*type_),
        Some(ValueType::Int) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Function,
                actual: InvalidExpressionType::Int,
            },
        }),
        Some(ValueType::String) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Function,
                actual: InvalidExpressionType::String,
            },
        }),
        Some(ValueType::Bool) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Function,
                actual: InvalidExpressionType::Bool,
            },
        }),
        Some(ValueType::Nil) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Function,
                actual: InvalidExpressionType::Nil,
            },
        }),
        None if type_.fn_types().is_some() => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::UnsupportedFunctionLiteralType,
        }),
        None => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        }),
    }
}

fn validate_argument_types(
    name: &ecow::EcoString,
    type_: &FunctionType,
    params: &[crate::planner::context::FunctionParam],
) -> Result<(), PlanError> {
    let actual = params
        .iter()
        .map(|param| param.local.value_type())
        .collect::<Vec<_>>();

    if actual == type_.argument_types() {
        Ok(())
    } else {
        Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: name.clone(),
                reason: InvalidFunctionShapeReason::ArgumentTypeMismatch,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        FunctionFunctionId, FunctionType, IntFunctionFunctionId, IntFunctionId, IntLocalId,
        LocalId, ParamLocal, RuntimeFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        call_int, call_int_function, function, function_function_ref, function_ref, int, int_arg,
        int_function_call_arg, int_function_ref, let_int_function_step, local_int,
        local_int_function, module_with_anonymous,
    };
    use crate::planner::error::{
        InvalidExpressionShapeKind, InvalidExpressionType, InvalidFunctionShapeReason,
        InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use gleam_core::ast::{FunctionLiteralKind, Statement, TypedArg, TypedExpr, TypedModule};

    #[test]
    fn plan_non_capturing_anonymous_function() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let add_one = fn(value) { value + 1 }
  add_one(41)
}
"#,
        ))
        .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function(
                    local_int_function(0, "add_one", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(0, int(41))],
                ),
            )
            .step(let_int_function_step(
                0,
                "add_one",
                int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
            )),
            [],
            [
                function("<anonymous:0>", local_int(0, "value").add_int(int(1)))
                    .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_anonymous_function_referencing_top_level_function() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let wrapped = fn(value) { add_one(value) }
  wrapped(41)
}
"#,
        ))
        .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function(
                    local_int_function(0, "wrapped", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(0, int(41))],
                ),
            )
            .step(let_int_function_step(
                0,
                "wrapped",
                int_function_ref(2, [LocalId::Int(IntLocalId(0))]),
            )),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
            [function(
                "<anonymous:0>",
                call_int(1, [int_arg(0, local_int(0, "value"))]),
            )
            .param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_main_returning_anonymous_function() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  fn(value) { value + 1 }
}
"#,
        ))
        .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                function_ref(
                    RuntimeFunctionId::Int(IntFunctionId(0)),
                    [LocalId::Int(IntLocalId(0))],
                ),
            ),
            [],
            [
                function("<anonymous:0>", local_int(0, "value").add_int(int(1)))
                    .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_nested_anonymous_function_storage_in_postorder() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  fn() { fn(value) { value + 1 } }
}
"#,
        ))
        .expect("source should plan");
        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type.clone(),
                ),
            ),
            [],
            [
                function("<anonymous:0>", local_int(0, "value").add_int(int(1)))
                    .param_int(0, "value"),
                function(
                    "<anonymous:1>",
                    function_ref(
                        RuntimeFunctionId::Int(IntFunctionId(0)),
                        [LocalId::Int(IntLocalId(0))],
                    ),
                ),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_capturing_anonymous_function() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  let value = 1
  fn() { value }
  1
}
"#,
            )),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::CapturingClosure,
            }),
        );
    }

    #[test]
    fn reject_profile_nested_capturing_anonymous_function() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  let value = 1
  fn() { fn() { value } }
  1
}
"#,
            )),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::CapturingClosure,
            }),
        );
    }

    #[test]
    fn reject_profile_function_capture_literal() {
        assert_eq!(
            plan_module(compile(
                r#"
fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  let add_one = add(1, _)
  add_one(41)
}
"#,
            )),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::FunctionCaptureLiteral,
            }),
        );
    }

    #[test]
    fn reject_profile_unsupported_anonymous_function_type() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  fn(value) { [value] }
  1
}
"#,
            )),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::UnsupportedFunctionLiteralType,
            }),
        );
    }

    #[test]
    fn reject_margin_non_function_literal_type() {
        for (type_, actual) in [
            (gleam_core::type_::int(), InvalidExpressionType::Int),
            (gleam_core::type_::string(), InvalidExpressionType::String),
            (gleam_core::type_::bool(), InvalidExpressionType::Bool),
            (gleam_core::type_::nil(), InvalidExpressionType::Nil),
        ] {
            let mut module = anonymous_function_module();
            let (expression_type, _, _) = anonymous_function_expression_mut(&mut module);
            *expression_type = type_;

            assert_eq!(
                plan_module(module),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Function,
                        actual,
                    },
                }),
            );
        }
    }

    #[test]
    fn reject_margin_anonymous_function_argument_type_mismatch() {
        let mut module = anonymous_function_module();
        let (_, arguments, _) = anonymous_function_expression_mut(&mut module);
        arguments[0].type_ = gleam_core::type_::string();

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous:0>".into(),
                    reason: InvalidFunctionShapeReason::ArgumentTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_anonymous_function_return_type_mismatch() {
        let mut module = anonymous_function_module();
        let (type_, _, _) = anonymous_function_expression_mut(&mut module);
        *type_ =
            gleam_core::type_::fn_(vec![gleam_core::type_::int()], gleam_core::type_::string());

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous:0>".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_non_supported_non_function_literal_type() {
        let mut module = anonymous_function_module();
        let (type_, _, _) = anonymous_function_expression_mut(&mut module);
        *type_ = gleam_core::type_::list(gleam_core::type_::int());

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_use_function_literal_expression_kind() {
        let mut module = anonymous_function_module();
        let (_, _, kind) = anonymous_function_expression_mut(&mut module);
        *kind = FunctionLiteralKind::Use {
            location: dummy_span(),
        };

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected anonymous function expression statement")]
    fn anonymous_function_expression_mut_panics_on_non_function_statement() {
        let mut module = compile(r#"pub fn main() { 1 }"#);

        let _ = anonymous_function_expression_mut(&mut module);
    }

    fn anonymous_function_module() -> TypedModule {
        compile(
            r#"
pub fn main() {
  fn(value) { value + 1 }
  1
}
"#,
        )
    }

    fn anonymous_function_expression_mut(
        module: &mut TypedModule,
    ) -> (
        &mut std::sync::Arc<gleam_core::type_::Type>,
        &mut Vec<TypedArg>,
        &mut FunctionLiteralKind,
    ) {
        let Statement::Expression(TypedExpr::Fn {
            type_,
            arguments,
            kind,
            ..
        }) = &mut module.definitions.functions[0].body[0]
        else {
            panic!("expected anonymous function expression statement");
        };

        (type_, arguments, kind)
    }
}
