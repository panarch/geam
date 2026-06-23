use crate::plan::{Expr, FunctionPlan, FunctionValue, LocalId, Param, ValueType};
use crate::planner::context::{FunctionParam, PlanContext};
use crate::planner::error::{
    InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedArgumentReason,
};
use crate::planner::statement::plan_non_empty_steps_and_return;
use ecow::EcoString;
use gleam_core::ast::{ArgNames, FunctionLiteralKind, TypedArg, TypedStatement};
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
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: anonymous_name(),
                    reason: InvalidFunctionShapeReason::InvalidLiteralKind,
                },
            });
        }
        FunctionLiteralKind::Use { .. } => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: anonymous_name(),
                    reason: InvalidFunctionShapeReason::InvalidLiteralKind,
                },
            });
        }
    }

    let params = anonymous_params(arguments)?;
    let Some(ValueType::Function(type_)) = ValueType::from_gleam(type_.as_ref()) else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: crate::planner::InvalidExpressionShapeKind::Invalid,
            },
        });
    };
    let type_ = *type_;
    if params.len() != type_.arguments().len() {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: "<anonymous>".into(),
                reason: InvalidFunctionShapeReason::ArityMismatch,
            },
        });
    }

    let return_type = type_.return_().clone();
    let runtime_id = context.push_anonymous_function(return_type, |id, _, context| {
        context.with_anonymous_function_scope(|context| {
            let plan_params = params
                .iter()
                .map(|param| {
                    context.define_existing_local(
                        param.name.clone(),
                        param.local,
                        param.type_.clone(),
                    );
                    Param::new(param.local, param.name.clone())
                })
                .collect();
            let planned = plan_non_empty_steps_and_return(body, context)?;

            Ok(FunctionPlan::new(
                id,
                "<anonymous>".into(),
                plan_params,
                planned.steps,
                planned.return_,
            ))
        })
    })?;
    let params = params.iter().map(|param| param.local).collect::<Vec<_>>();
    let value = FunctionValue::new(type_, runtime_id, params);

    Ok(Expr::function(crate::plan::FunctionExpr::value(value)))
}

fn anonymous_params(arguments: Vec<TypedArg>) -> Result<Vec<FunctionParam>, PlanError> {
    let mut next_int = 0;
    let mut next_string = 0;
    let mut next_bool = 0;
    let mut next_nil = 0;
    let mut next_function = 0;

    arguments
        .into_iter()
        .map(|argument| {
            let Some(name) = argument.names.get_variable_name().cloned() else {
                return Err(PlanError::UnsupportedArgument {
                    function: anonymous_name(),
                    reason: UnsupportedArgumentReason::Discard,
                });
            };
            if !matches!(argument.names, ArgNames::Named { .. }) {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::FunctionShape {
                        name: anonymous_name(),
                        reason: InvalidFunctionShapeReason::InvalidArgumentNames,
                    },
                });
            }

            let Some(type_) = ValueType::from_gleam(&argument.type_) else {
                return Err(PlanError::UnsupportedArgument {
                    function: anonymous_name(),
                    reason: UnsupportedArgumentReason::UnsupportedType,
                });
            };
            let local = match &type_ {
                ValueType::Int => {
                    let local = LocalId::Int(crate::plan::IntLocalId(next_int));
                    next_int += 1;
                    local
                }
                ValueType::String => {
                    let local = LocalId::String(crate::plan::StringLocalId(next_string));
                    next_string += 1;
                    local
                }
                ValueType::Bool => {
                    let local = LocalId::Bool(crate::plan::BoolLocalId(next_bool));
                    next_bool += 1;
                    local
                }
                ValueType::Nil => {
                    let local = LocalId::Nil(crate::plan::NilLocalId(next_nil));
                    next_nil += 1;
                    local
                }
                ValueType::Function(_) => {
                    let local = LocalId::Function(crate::plan::FunctionLocalId(next_function));
                    next_function += 1;
                    local
                }
            };

            Ok(FunctionParam { local, name, type_ })
        })
        .collect()
}

fn anonymous_name() -> EcoString {
    "<anonymous>".into()
}

#[cfg(test)]
mod tests {
    use super::super::typed_int_expr;
    use crate::planner::error::{
        InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError, UnsupportedArgumentReason,
        UnsupportedExpressionKind,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use gleam_core::ast::{
        ArgNames, FunctionLiteralKind, Statement, TypedArg, TypedExpr, TypedModule,
    };
    use gleam_core::type_::{self, Type};
    use std::sync::Arc;

    #[test]
    fn plan_anonymous_function_argument_types() {
        assert!(
            plan_module(compile(
                r#"
fn apply_int(function: fn(Int) -> Int) {
  function(1)
}

fn apply_string(function: fn(String) -> String) {
  function("geam")
}

fn apply_bool(function: fn(Bool) -> Bool) {
  function(True)
}

fn apply_nil(function: fn(Nil) -> Nil) {
  function(Nil)
}

fn apply_function(function: fn(fn(Int) -> Int) -> fn(Int) -> Int) {
  function(fn(value) { value })
}

pub fn main() {
  apply_int(fn(value) { value })
  apply_string(fn(value) { value })
  apply_bool(fn(value) { value })
  apply_nil(fn(value) { value })
  let returned = apply_function(fn(function) { function })
  returned(1)
}
"#,
            ))
            .is_ok()
        );
    }

    #[test]
    fn reject_profile_capturing_anonymous_function() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  let value = 1
  let function = fn() { value }
  function()
}
"#,
            )),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::CapturingFunction,
            }),
        );
    }

    #[test]
    fn reject_profile_function_returning_capturing_anonymous_function() {
        assert_eq!(
            plan_module(compile(
                r#"
fn make_adder(value: Int) {
  fn(other) { value + other }
}

pub fn main() {
  let add_one = make_adder(1)
  add_one(2)
}
"#,
            )),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::CapturingFunction,
            }),
        );
    }

    #[test]
    fn reject_margin_use_function_literal() {
        let mut module = compile(r#"pub fn main() { fn(value) { value }(1) }"#);
        let (_, kind, _) = expect_anonymous_function_mut(&mut module);
        *kind = FunctionLiteralKind::Use {
            location: dummy_span(),
        };

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous>".into(),
                    reason: InvalidFunctionShapeReason::InvalidLiteralKind,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_capture_function_literal() {
        let mut module = compile(r#"pub fn main() { fn(value) { value }(1) }"#);
        let (_, kind, _) = expect_anonymous_function_mut(&mut module);
        *kind = FunctionLiteralKind::Capture { hole: dummy_span() };

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous>".into(),
                    reason: InvalidFunctionShapeReason::InvalidLiteralKind,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_invalid_anonymous_function_type() {
        let mut module = compile(r#"pub fn main() { fn(value) { value }(1) }"#);
        let (type_, _, _) = expect_anonymous_function_mut(&mut module);
        *type_ = type_::int();

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_anonymous_function_arity_mismatch() {
        let mut module = compile(r#"pub fn main() { fn(value) { value }(1) }"#);
        let (_, _, arguments) = expect_anonymous_function_mut(&mut module);
        arguments.push(TypedArg {
            names: ArgNames::Named {
                name: "extra".into(),
                location: dummy_span(),
            },
            annotation: None,
            type_: type_::int(),
            location: dummy_span(),
        });

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous>".into(),
                    reason: InvalidFunctionShapeReason::ArityMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_profile_anonymous_function_discard_argument() {
        assert_eq!(
            plan_module(compile(r#"pub fn main() { fn(_) { 1 }(1) }"#)),
            Err(PlanError::UnsupportedArgument {
                function: "<anonymous>".into(),
                reason: UnsupportedArgumentReason::Discard,
            }),
        );
    }

    #[test]
    fn reject_margin_anonymous_function_labelled_argument() {
        let mut module = compile(r#"pub fn main() { fn(value) { value }(1) }"#);
        let (_, _, arguments) = expect_anonymous_function_mut(&mut module);
        arguments[0].names = ArgNames::NamedLabelled {
            label: "label".into(),
            label_location: dummy_span(),
            name: "value".into(),
            name_location: dummy_span(),
        };

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous>".into(),
                    reason: InvalidFunctionShapeReason::InvalidArgumentNames,
                },
            }),
        );
    }

    #[test]
    fn reject_profile_anonymous_function_unsupported_argument_type() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  fn(value: List(Int)) { 1 }([1])
}
"#,
            )),
            Err(PlanError::UnsupportedArgument {
                function: "<anonymous>".into(),
                reason: UnsupportedArgumentReason::UnsupportedType,
            }),
        );
    }

    fn expect_anonymous_function_mut(
        module: &mut TypedModule,
    ) -> (&mut Arc<Type>, &mut FunctionLiteralKind, &mut Vec<TypedArg>) {
        let fun = expect_anonymous_call_fun_mut(module);
        let TypedExpr::Fn {
            type_,
            kind,
            arguments,
            ..
        } = fun
        else {
            panic!("expected function literal");
        };
        (type_, kind, arguments)
    }

    fn expect_anonymous_call_fun_mut(module: &mut TypedModule) -> &mut TypedExpr {
        let Statement::Expression(TypedExpr::Call { fun, .. }) =
            &mut module.definitions.functions[0].body[0]
        else {
            panic!("expected anonymous function call");
        };
        fun.as_mut()
    }

    #[test]
    #[should_panic(expected = "expected anonymous function call")]
    fn expect_anonymous_call_fun_mut_panics_on_expression() {
        let mut module = compile(r#"pub fn main() { 1 }"#);

        expect_anonymous_call_fun_mut(&mut module);
    }

    #[test]
    #[should_panic(expected = "expected function literal")]
    fn expect_anonymous_function_mut_panics_on_non_function_literal() {
        let mut module = compile(r#"pub fn main() { fn(value) { value }(1) }"#);
        *expect_anonymous_call_fun_mut(&mut module) = typed_int_expr(1);

        expect_anonymous_function_mut(&mut module);
    }
}
