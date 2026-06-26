use crate::plan::{
    BoolExpr, BoolFunctionExpr, Expr, FunctionExpr, IntExpr, IntFunctionExpr, LocalId, NilExpr,
    NilFunctionExpr, StringExpr, StringFunctionExpr, ValueType,
};
use crate::planner::context::{FunctionLocalBinding, PlanContext};
use crate::planner::error::{InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError};
use ecow::EcoString;
use gleam_core::type_::{PRELUDE_MODULE_NAME, ValueConstructor, ValueConstructorVariant};

pub(super) fn plan_var(
    name: EcoString,
    constructor: ValueConstructor,
    context: &PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match constructor.variant {
        ValueConstructorVariant::LocalVariable { .. } => {
            if let Some((local, type_)) = context.lookup_local(&name) {
                return local_get(local, name, type_);
            }
            if let Some(binding) = context.lookup_function_local(&name) {
                return Ok(function_local_get(binding, name));
            }

            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal { name },
            })
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
            let function =
                context
                    .lookup_function(&name)
                    .ok_or_else(|| PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::UnknownLocal { name: name.clone() },
                    })?;

            let value = function.value();

            Ok(Expr::function(FunctionExpr::value(value)))
        }
        ValueConstructorVariant::ModuleFn { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::ModuleSelect,
            },
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

fn function_local_get(binding: FunctionLocalBinding, name: EcoString) -> Expr {
    match binding {
        FunctionLocalBinding::Int { local, type_ } => Expr::function(FunctionExpr::int(
            IntFunctionExpr::local_get(local, name, type_),
        )),
        FunctionLocalBinding::String { local, type_ } => Expr::function(FunctionExpr::string(
            StringFunctionExpr::local_get(local, name, type_),
        )),
        FunctionLocalBinding::Bool { local, type_ } => Expr::function(FunctionExpr::bool(
            BoolFunctionExpr::local_get(local, name, type_),
        )),
        FunctionLocalBinding::Nil { local, type_ } => Expr::function(FunctionExpr::nil(
            NilFunctionExpr::local_get(local, name, type_),
        )),
    }
}

fn local_get(local: LocalId, name: EcoString, type_: ValueType) -> Result<Expr, PlanError> {
    match (local, type_) {
        (LocalId::Int(local), ValueType::Int) => Ok(Expr::int(IntExpr::local_get(local, name))),
        (LocalId::String(local), ValueType::String) => {
            Ok(Expr::string(StringExpr::local_get(local, name)))
        }
        (LocalId::Bool(local), ValueType::Bool) => Ok(Expr::bool(BoolExpr::local_get(local, name))),
        (LocalId::Nil(local), ValueType::Nil) => Ok(Expr::nil(NilExpr::local_get(local, name))),
        _ => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{module_returning_typed_expr, typed_int_expr, typed_prelude_constructor};
    use crate::plan::{
        FunctionType, IntFunctionId, IntFunctionLocalId, IntLocalId, LocalId, ParamLocal,
        RuntimeFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        bool_, call_int_function, function, function_ref, int, int_function_call_arg, local_bool,
        local_int, local_int_function, local_nil, local_string, module, nil,
    };
    use crate::planner::plan_module;
    use crate::planner::support::compile;
    use crate::planner::{InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError};
    use ecow::EcoString;
    use gleam_core::ast::{Statement, TypedExpr, TypedStatement};
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
    fn plan_top_level_function_reference_expression() {
        let actual = plan_module(compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
  1
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1)).evaluate(function_ref(
                RuntimeFunctionId::Int(IntFunctionId(1)),
                [LocalId::Int(IntLocalId(0))],
            )),
            [function("identity", local_int(0, "value")).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_top_level_function_reference_with_function_argument() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn apply(function: fn(Int) -> Int, value: Int) {
  function(value)
}

pub fn main() {
  apply
  1
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1)).evaluate(function_ref(
                RuntimeFunctionId::Int(IntFunctionId(2)),
                [
                    ParamLocal::int_function(
                        IntFunctionLocalId(0),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    ),
                    ParamLocal::int(IntLocalId(0)),
                ],
            )),
            [
                function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value"),
                function(
                    "apply",
                    call_int_function(
                        local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                        [int_function_call_arg(local_int(0, "value"))],
                    ),
                )
                .param_int_function(0, "function", [ValueType::Int])
                .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
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

        let mut missing_current_function = compile(
            r#"
pub fn main() {
  identity
  1
}

fn identity(value: Int) {
  value
}
"#,
        );
        let identity_index = missing_current_function
            .definitions
            .functions
            .iter()
            .position(|function| {
                function
                    .name
                    .as_ref()
                    .is_some_and(|(_, name)| name == "identity")
            })
            .expect("identity function should exist");
        missing_current_function
            .definitions
            .functions
            .remove(identity_index);
        assert_eq!(
            plan_module(missing_current_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal {
                    name: "identity".into(),
                },
            }),
        );

        let mut non_current_function = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
  1
}
"#,
        );
        let main = non_current_function
            .definitions
            .functions
            .iter_mut()
            .find(|function| {
                function
                    .name
                    .as_ref()
                    .is_some_and(|(_, name)| name == "main")
            })
            .expect("main function should exist");
        let module =
            expect_module_fn_module_mut(expect_expression_statement_mut(&mut main.body[0]));
        *module = "other".into();
        assert_eq!(
            plan_module(non_current_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::ModuleSelect,
                },
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
  1
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

    #[test]
    fn reject_margin_local_type_shape_mismatch() {
        assert_eq!(
            super::local_get(
                LocalId::Int(IntLocalId(0)),
                "value".into(),
                ValueType::String,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    fn expect_expression_statement_mut(statement: &mut TypedStatement) -> &mut TypedExpr {
        let Statement::Expression(expression) = statement else {
            panic!("expected expression statement");
        };
        expression
    }

    fn expect_module_fn_module_mut(expression: &mut TypedExpr) -> &mut ecow::EcoString {
        let (_, constructor) = expect_var_mut(expression);
        let type_::ValueConstructorVariant::ModuleFn { module, .. } = &mut constructor.variant
        else {
            panic!("expected module function constructor");
        };
        module
    }

    fn expect_var_mut(
        expression: &mut TypedExpr,
    ) -> (&mut EcoString, &mut type_::ValueConstructor) {
        let TypedExpr::Var {
            name, constructor, ..
        } = expression
        else {
            panic!("expected variable expression");
        };
        (name, constructor)
    }

    #[test]
    #[should_panic(expected = "expected expression statement")]
    fn expect_expression_statement_mut_panics_on_assignment() {
        let mut module = compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        );

        expect_expression_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    #[test]
    #[should_panic(expected = "expected variable expression")]
    fn expect_module_fn_module_mut_panics_on_int() {
        let mut expression = typed_int_expr(1);

        expect_module_fn_module_mut(&mut expression);
    }

    #[test]
    #[should_panic(expected = "expected module function constructor")]
    fn expect_module_fn_module_mut_panics_on_prelude_constructor() {
        let mut expression = typed_prelude_constructor("True", type_::bool());

        expect_module_fn_module_mut(&mut expression);
    }
}
