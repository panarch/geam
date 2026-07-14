mod argument;
mod direct;
mod function_value;
mod implicit;

use crate::plan::{CustomExpr, Expr};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidCallShapeReason, InvalidTypedAstReason, InvalidUseShapeReason, PlanError,
};
use ecow::EcoString;
use gleam_core::ast::{CallArg as GleamCallArg, TypedExpr};
use gleam_core::type_::{PRELUDE_MODULE_NAME, Type, ValueConstructorVariant};
use std::sync::Arc;

pub(super) fn plan_call(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    if arguments.iter().any(|argument| argument.implicit.is_some()) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CallShape {
                reason: InvalidCallShapeReason::ImplicitArguments,
            },
        });
    }

    plan_call_expression(type_, fun, arguments, context, None)
}

pub(super) fn plan_use_call(
    call: TypedExpr,
    use_assignment_count: usize,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    implicit::plan_use_call(call, use_assignment_count, context)
}

pub(super) fn plan_pipeline_direct_call(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    implicit::plan_pipeline_direct_call(type_, fun, arguments, context)
}

pub(super) fn plan_pipeline_hole_call(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    implicit::plan_pipeline_hole_call(type_, fun, arguments, context)
}

struct CaptureSubstitution {
    name: EcoString,
    value: Expr,
}

fn invalid_use_shape(reason: InvalidUseShapeReason) -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::UseShape { reason },
    }
}

fn is_capture_local(expression: &TypedExpr, capture_name: &EcoString) -> bool {
    matches!(
        expression,
        TypedExpr::Var {
            name,
            constructor,
            ..
        } if name == capture_name
            && matches!(
                constructor.variant,
                ValueConstructorVariant::LocalVariable { .. }
            )
    )
}

fn plan_call_expression(
    type_: Arc<Type>,
    fun: TypedExpr,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Expr, PlanError> {
    if let TypedExpr::Var { constructor, .. } = &fun {
        match &constructor.variant {
            ValueConstructorVariant::ModuleFn {
                module,
                name,
                external_erlang,
                external_javascript,
                ..
            } if module == context.module_name
                && external_erlang.is_none()
                && external_javascript.is_none() =>
            {
                let function = context
                    .lookup_function(name)
                    .ok_or(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::CallShape {
                            reason: InvalidCallShapeReason::MissingCurrentModuleFunction,
                        },
                    })?;
                return direct::plan_direct_function_call(
                    type_, function, arguments, context, capture,
                );
            }
            ValueConstructorVariant::ModuleConstant { literal, .. }
                if literal.type_().fn_types().is_none() =>
            {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::ModuleConstant,
                    },
                });
            }
            ValueConstructorVariant::ModuleConstant { .. } => {}
            ValueConstructorVariant::Record { module, arity, .. }
                if module == context.module_name || module == PRELUDE_MODULE_NAME =>
            {
                let constructor = context.custom_constructor(constructor)?;
                if usize::from(*arity) != arguments.len() {
                    return Err(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::CallShape {
                            reason: InvalidCallShapeReason::FunctionCallArityMismatch,
                        },
                    });
                }
                let arguments = argument::plan_custom_constructor_args(
                    arguments,
                    &constructor,
                    context,
                    capture,
                )?;
                return Ok(Expr::custom(CustomExpr::constructor(
                    constructor,
                    arguments,
                )));
            }
            ValueConstructorVariant::Record { .. } => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::RecordConstructor,
                    },
                });
            }
            ValueConstructorVariant::ModuleFn { .. } => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::CallShape {
                        reason: InvalidCallShapeReason::NonCurrentModuleFunction,
                    },
                });
            }
            ValueConstructorVariant::LocalVariable { .. } => {}
        }
    }

    function_value::plan_function_value_call(type_, fun, arguments, context, capture)
}

#[cfg(test)]
mod support {
    use crate::planner::support::compile;
    use gleam_core::ast::{CallArg, Statement, TypedExpr, TypedStatement};
    use gleam_core::type_::{self, ValueConstructor};

    pub(super) fn expect_call_statement_mut(
        statement: &mut TypedStatement,
    ) -> (
        &mut std::sync::Arc<type_::Type>,
        &mut TypedExpr,
        &mut Vec<CallArg<TypedExpr>>,
    ) {
        match statement {
            Statement::Expression(expression) => expect_call_expression_mut(expression),
            _ => panic!("expected call expression statement"),
        }
    }

    pub(super) fn expect_call_expression_mut(
        expression: &mut TypedExpr,
    ) -> (
        &mut std::sync::Arc<type_::Type>,
        &mut TypedExpr,
        &mut Vec<CallArg<TypedExpr>>,
    ) {
        match expression {
            TypedExpr::Call {
                type_,
                fun,
                arguments,
                ..
            } => (type_, fun.as_mut(), arguments),
            _ => panic!("expected call expression statement"),
        }
    }

    pub(super) fn expect_var_constructor_mut(expression: &mut TypedExpr) -> &mut ValueConstructor {
        match expression {
            TypedExpr::Var { constructor, .. } => constructor,
            _ => panic!("expected variable expression"),
        }
    }

    #[test]
    #[should_panic(expected = "expected call expression statement")]
    fn expect_call_statement_mut_panics_on_expression() {
        let mut module = crate::planner::support::compile_minimal_module();

        expect_call_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    #[test]
    #[should_panic(expected = "expected call expression statement")]
    fn expect_call_statement_mut_panics_on_assignment() {
        let mut module = compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        );

        expect_call_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    #[test]
    #[should_panic(expected = "expected variable expression")]
    fn expect_var_constructor_mut_panics_on_int() {
        let mut expression = super::super::typed_int_expr(1);

        expect_var_constructor_mut(&mut expression);
    }
}

#[cfg(test)]
mod tests {
    use super::super::{module_returning_typed_expr, typed_prelude_constructor, typed_string_expr};
    use super::support::{expect_call_statement_mut, expect_var_constructor_mut};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use crate::planner::{
        InvalidCallShapeReason, InvalidCustomTypeReason, InvalidTypedAstReason, PlanError,
        UnsupportedExpressionKind,
    };
    use gleam_core::ast::{ImplicitCallArgOrigin, Statement, TypedExpr, TypedModule};
    use gleam_core::type_::{self, ValueConstructorVariant, error::VariableOrigin};

    #[test]
    fn reject_profile_polymorphic_result_constructor_call() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  Ok(1)
  1
}
"#,
            )),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::GenericFunction,
            }),
        );
    }

    #[test]
    fn reject_margin_module_constant_call_shape() {
        reject_margin_module_constant_call(compile(
            r#"
const answer = 1

pub fn main() {
  answer
}
"#,
        ));
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
    fn reject_margin_call_entry_and_callee_shapes() {
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

        let mut local_variable_callee = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, fun, _) =
            expect_call_statement_mut(&mut local_variable_callee.definitions.functions[1].body[0]);
        let constructor = expect_var_constructor_mut(fun);
        constructor.variant = ValueConstructorVariant::LocalVariable {
            location: dummy_span(),
            origin: VariableOrigin::generated(),
        };
        assert_eq!(
            plan_module(local_variable_callee),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal {
                    name: "identity".into(),
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

        reject_margin_non_local_module_fn_call(compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        ));

        let mut record_constructor_call = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed(1)
  1
}
"#,
        );
        record_constructor_call.definitions.custom_types.clear();
        assert_eq!(
            plan_module(record_constructor_call),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Boxed".into(),
                    reason: InvalidCustomTypeReason::UnknownDefinition,
                },
            }),
        );

        let mut constructor_arity_mismatch = compile(
            r#"
pub type Boxed { Boxed(Int) }
pub fn main() { Boxed(1) }
"#,
        );
        let (_, _, arguments) = expect_call_statement_mut(
            &mut constructor_arity_mismatch.definitions.functions[0].body[0],
        );
        arguments.clear();
        assert_eq!(
            plan_module(constructor_arity_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::FunctionCallArityMismatch,
                },
            }),
        );

        let mut labelled_constructor_argument = compile(
            r#"
pub type Boxed { Boxed(value: Int) }
pub fn main() { Boxed(value: 1) }
"#,
        );
        let (_, _, arguments) = expect_call_statement_mut(
            &mut labelled_constructor_argument.definitions.functions[0].body[0],
        );
        arguments[0].label = Some("other".into());
        assert_eq!(
            plan_module(labelled_constructor_argument),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::LabelledArguments,
                },
            }),
        );

        let mut constructor_field_type_mismatch = compile(
            r#"
pub type Boxed { Boxed(Int) }
pub fn main() { Boxed(1) }
"#,
        );
        let (_, _, arguments) = expect_call_statement_mut(
            &mut constructor_field_type_mismatch.definitions.functions[0].body[0],
        );
        arguments[0].value = typed_string_expr("wrong");
        assert_eq!(
            plan_module(constructor_field_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: crate::planner::InvalidExpressionType::Int,
                    actual: crate::planner::InvalidExpressionType::String,
                },
            }),
        );

        let mut external_record_constructor = compile(
            r#"
pub type Boxed { Boxed(Int) }
pub fn main() { Boxed(1) }
"#,
        );
        let (_, fun, _) = expect_call_statement_mut(
            &mut external_record_constructor.definitions.functions[0].body[0],
        );
        let constructor = expect_var_constructor_mut(fun);
        constructor.variant = ValueConstructorVariant::Record {
            name: "Boxed".into(),
            arity: 1,
            field_map: None,
            location: dummy_span(),
            module: "other".into(),
            variants_count: 1,
            variant_index: 0,
            documentation: None,
        };
        assert_eq!(
            plan_module(external_record_constructor),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::RecordConstructor,
                },
            }),
        );

        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::Call {
                location: dummy_span(),
                type_: type_::bool(),
                fun: Box::new(typed_prelude_constructor("True", type_::bool())),
                arguments: Vec::new(),
                open_parenthesis: Some(0),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "True".into(),
                    reason: InvalidCustomTypeReason::ConstructorType,
                },
            }),
        );
    }

    fn reject_margin_module_constant_call(mut module_constant_call: TypedModule) {
        module_constant_call.definitions.constants.clear();
        let statement = module_constant_call.definitions.functions[0].body.remove(0);
        let module_constant = match statement {
            Statement::Expression(module_constant) => module_constant,
            _ => panic!("expected expression statement"),
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

    fn reject_margin_non_local_module_fn_call(mut non_local_module_fn: TypedModule) {
        let function = non_local_module_fn
            .definitions
            .functions
            .last_mut()
            .expect("expected test module to have a function");
        let (_, fun, _) = expect_call_statement_mut(&mut function.body[0]);
        let constructor = expect_var_constructor_mut(fun);
        let module = match &mut constructor.variant {
            ValueConstructorVariant::ModuleFn { module, .. } => module,
            _ => panic!("expected module function constructor"),
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
}
