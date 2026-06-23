use crate::plan::{BoolExpr, Expr, IntExpr, LocalId, NilExpr, StringExpr};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
};
use ecow::EcoString;
use gleam_core::type_::{PRELUDE_MODULE_NAME, ValueConstructor, ValueConstructorVariant};

pub(super) fn plan_var(
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
            "True" => Ok(Expr::bool(BoolExpr::value(true))),
            "False" => Ok(Expr::bool(BoolExpr::value(false))),
            "Nil" => Ok(Expr::nil(NilExpr::value())),
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

fn local_get(local: LocalId, name: EcoString) -> Expr {
    match local {
        LocalId::Int(local) => Expr::int(IntExpr::local_get(local, name)),
        LocalId::String(local) => Expr::string(StringExpr::local_get(local, name)),
        LocalId::Bool(local) => Expr::bool(BoolExpr::local_get(local, name)),
        LocalId::Nil(local) => Expr::nil(NilExpr::local_get(local, name)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{module_returning_typed_expr, typed_prelude_constructor};
    use crate::planner::dsl::{
        bool_, function, int, local_bool, local_int, local_nil, local_string, module, nil,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, expect_plan_error};
    use crate::planner::{
        InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
    };
    use gleam_core::type_;

    #[test]
    fn plan_local_variables() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1
}

pub fn int_id(value: Int) {
  value
}

pub fn string_id(value: String) {
  value
}

pub fn bool_id(value: Bool) {
  value
}

pub fn nil_id(value: Nil) {
  value
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1)),
            [
                function("int_id", local_int(0, "value")).param_int(0, "value"),
                function("string_id", local_string(0, "value")).param_string(0, "value"),
                function("bool_id", local_bool(0, "value")).param_bool(0, "value"),
                function("nil_id", local_nil(0, "value")).param_nil(0, "value"),
            ],
        );

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
        let expected = module(
            "main",
            function("main", nil()),
            [
                function("truth", bool_(true)),
                function("falsehood", bool_(false)),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_function_reference_expression() {
        assert_eq!(
            expect_plan_error(
                r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::FunctionReference,
            },
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
            plan_module(module_returning_typed_expr(typed_prelude_constructor(
                "Other",
                type_::bool(),
            ))),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::PreludeConstructor,
                },
            }),
        );
    }
}
