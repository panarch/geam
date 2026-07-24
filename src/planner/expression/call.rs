mod argument;
mod direct;
mod function_value;
mod implicit;

use crate::plan::Expr;
use crate::planner::context::{ModuleFunctionTarget, PlanContext};
use crate::planner::error::{
    InvalidCallShapeReason, InvalidModuleReferenceReason, InvalidTypedAstReason,
    InvalidUseShapeReason, PlanError,
};
use ecow::EcoString;
use gleam_core::ast::{CallArg as GleamCallArg, TypedExpr};
use gleam_core::type_::{ModuleValueConstructor, Type, ValueConstructorVariant};
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
            } => {
                let target = ModuleFunctionTarget::direct(
                    module.clone(),
                    name.clone(),
                    external_erlang.is_some() || external_javascript.is_some(),
                )
                .validate_external()?;
                let function = context.module_function(&target)?;
                return direct::plan_direct_function_call(
                    type_, function, arguments, context, capture,
                );
            }
            ValueConstructorVariant::ModuleConstant {
                module,
                name,
                literal,
                ..
            } => {
                let _linked_module = context.resolve_module_reference(module, name)?;
                if literal.type_().fn_types().is_none() {
                    return Err(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::ModuleReference {
                            module: module.clone(),
                            name: name.clone(),
                            reason: InvalidModuleReferenceReason::NonCallableConstant,
                        },
                    });
                }
            }
            ValueConstructorVariant::Record { .. } => {
                let constructor = context.custom_constructor(constructor)?;
                return plan_custom_constructor_call(constructor, arguments, context, capture);
            }
            ValueConstructorVariant::LocalVariable { .. } => {}
        }
    }
    if let TypedExpr::ModuleSelect {
        module_name,
        label,
        constructor,
        ..
    } = &fun
    {
        match constructor {
            ModuleValueConstructor::Fn {
                module,
                name,
                external_erlang,
                external_javascript,
                ..
            } => {
                let target = ModuleFunctionTarget::selected(
                    context,
                    module_name.clone(),
                    label.clone(),
                    module.clone(),
                    name.clone(),
                    external_erlang.is_some() || external_javascript.is_some(),
                )?
                .validate_external()?;
                let function = context.module_function(&target)?;
                return direct::plan_direct_function_call(
                    type_, function, arguments, context, capture,
                );
            }
            ModuleValueConstructor::Record {
                name,
                variant_index,
                arity,
                type_,
                ..
            } => {
                let _linked_module = context.resolve_module_reference(module_name, label)?;
                if name != label {
                    return Err(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::ModuleReference {
                            module: module_name.clone(),
                            name: label.clone(),
                            reason: InvalidModuleReferenceReason::RecordConstructorName {
                                actual: name.clone(),
                            },
                        },
                    });
                }
                let constructor = context.module_custom_constructor(
                    type_.as_ref(),
                    name.clone(),
                    module_name,
                    usize::from(*variant_index),
                    usize::from(*arity),
                )?;
                return plan_custom_constructor_call(constructor, arguments, context, capture);
            }
            ModuleValueConstructor::Constant { literal, .. } => {
                let _linked_module = context.resolve_module_reference(module_name, label)?;
                if literal.type_().fn_types().is_none() {
                    return Err(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::ModuleReference {
                            module: module_name.clone(),
                            name: label.clone(),
                            reason: InvalidModuleReferenceReason::NonCallableConstant,
                        },
                    });
                }
            }
        }
    }

    function_value::plan_function_value_call(type_, fun, arguments, context, capture)
}

fn plan_custom_constructor_call(
    constructor: crate::plan::CustomConstructor,
    arguments: Vec<GleamCallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
    capture: Option<&CaptureSubstitution>,
) -> Result<Expr, PlanError> {
    let arguments =
        argument::plan_custom_constructor_args(arguments, &constructor, context, capture)?;
    let construction =
        crate::plan::CustomConstruction::try_new(constructor, arguments).map_err(|error| {
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::RecordConstructorArgumentCount {
                        expected: error.expected,
                        actual: error.actual,
                    },
                },
            }
        })?;
    context
        .custom_expr_from_construction(construction)
        .map(Expr::custom)
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
    use crate::planner::support::{compile, dummy_span};
    use crate::planner::{
        InvalidCallShapeReason, InvalidCustomTypeReason, InvalidModuleReferenceReason,
        InvalidTypedAstReason, PlanError,
    };
    use crate::planner::{plan_module, plan_program};
    use crate::{ModuleSource, compile_typed_program};
    use gleam_core::ast::{
        Constant, ImplicitCallArgOrigin, Publicity, Statement, TypedExpr, TypedModule,
    };
    use gleam_core::type_::{
        self, ModuleValueConstructor, ValueConstructorVariant, error::VariableOrigin,
    };

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
    fn qualified_function_constant_call_matches_unqualified_import_plan() {
        let dependency = r#"
fn add_one(value: Int) {
  value + 1
}

pub const operation = add_one
"#;
        let qualified = compile_typed_program(
            "main",
            [
                ModuleSource::new("operation", "operation.gleam", dependency),
                ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
import operation

pub fn main() {
  operation.operation(41)
}
"#,
                ),
            ],
        )
        .expect("qualified function constant program should compile");
        let qualified =
            plan_program(qualified).expect("qualified function constant call should plan");
        let unqualified = compile_typed_program(
            "main",
            [
                ModuleSource::new("operation", "operation.gleam", dependency),
                ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
import operation.{operation}

pub fn main() {
  operation(41)
}
"#,
                ),
            ],
        )
        .expect("unqualified function constant program should compile");
        let unqualified =
            plan_program(unqualified).expect("unqualified function constant call should plan");

        assert_eq!(
            qualified.main_function().return_(),
            unqualified.main_function().return_(),
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
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::MissingFunction,
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
                    reason: InvalidCallShapeReason::RecordConstructorArgumentCount {
                        expected: 1,
                        actual: 0,
                    },
                },
            }),
        );

        let mut constructor_descriptor_arity_mismatch = compile(
            r#"
pub type Boxed { Boxed(Int) }
pub fn main() { Boxed(1) }
"#,
        );
        let (_, function, arguments) = expect_call_statement_mut(
            &mut constructor_descriptor_arity_mismatch.definitions.functions[0].body[0],
        );
        let constructor = expect_var_constructor_mut(function);
        constructor.variant = ValueConstructorVariant::Record {
            name: "Boxed".into(),
            arity: 0,
            field_map: None,
            location: dummy_span(),
            module: "main".into(),
            variants_count: 1,
            variant_index: 0,
            documentation: None,
        };
        arguments.clear();
        assert_eq!(
            plan_module(constructor_descriptor_arity_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Boxed".into(),
                    reason: InvalidCustomTypeReason::ConstructorArity,
                },
            }),
        );

        let mut extra_constructor_argument = compile(
            r#"
pub type Boxed { Boxed(Int) }
pub fn main() { Boxed(1) }
"#,
        );
        let (_, _, arguments) = expect_call_statement_mut(
            &mut extra_constructor_argument.definitions.functions[0].body[0],
        );
        arguments.push(arguments[0].clone());
        assert_eq!(
            plan_module(extra_constructor_argument),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::RecordConstructorArgumentCount {
                        expected: 1,
                        actual: 2,
                    },
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
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "Boxed".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
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

    #[test]
    fn module_select_calls_preserve_constructors_and_reject_invalid_targets() {
        let source = r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed(1)
}
"#;
        let expected = plan_module(compile(source));
        let mut qualified_constructor = compile(source);
        let (_, function, _) =
            expect_call_statement_mut(&mut qualified_constructor.definitions.functions[0].body[0]);
        *function = module_select_record("main", 1);
        assert_eq!(plan_module(qualified_constructor), expected);

        let mut unlinked_constructor = compile(source);
        let (_, function, _) =
            expect_call_statement_mut(&mut unlinked_constructor.definitions.functions[0].body[0]);
        *function = module_select_record("other", 1);
        assert_eq!(
            plan_module(unlinked_constructor),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "Boxed".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
                },
            }),
        );

        let mut constructor_label_mismatch = compile(source);
        let (_, function, _) = expect_call_statement_mut(
            &mut constructor_label_mismatch.definitions.functions[0].body[0],
        );
        *function = module_select_record_with_label("main", "Other", 1);
        assert_eq!(
            plan_module(constructor_label_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "Other".into(),
                    reason: InvalidModuleReferenceReason::RecordConstructorName {
                        actual: "Boxed".into(),
                    },
                },
            }),
        );

        let mut constructor_arity_mismatch = compile(source);
        let (_, function, _) = expect_call_statement_mut(
            &mut constructor_arity_mismatch.definitions.functions[0].body[0],
        );
        *function = module_select_record("main", 0);
        assert_eq!(
            plan_module(constructor_arity_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Boxed".into(),
                    reason: InvalidCustomTypeReason::ConstructorArity,
                },
            }),
        );

        let mut argument_count_mismatch = compile(source);
        let (_, function, arguments) = expect_call_statement_mut(
            &mut argument_count_mismatch.definitions.functions[0].body[0],
        );
        *function = module_select_record("main", 1);
        arguments.clear();
        assert_eq!(
            plan_module(argument_count_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CallShape {
                    reason: InvalidCallShapeReason::RecordConstructorArgumentCount {
                        expected: 1,
                        actual: 0,
                    },
                },
            }),
        );

        let mut constructor_type_mismatch = compile(source);
        let (_, function, _) = expect_call_statement_mut(
            &mut constructor_type_mismatch.definitions.functions[0].body[0],
        );
        *function = module_select_record_with_type("main", "Boxed", 1, type_::int());
        assert_eq!(
            plan_module(constructor_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Boxed".into(),
                    reason: InvalidCustomTypeReason::ConstructorType,
                },
            }),
        );

        let mut constructor_argument_type_mismatch = compile(source);
        let (_, function, arguments) = expect_call_statement_mut(
            &mut constructor_argument_type_mismatch.definitions.functions[0].body[0],
        );
        *function = module_select_record("main", 1);
        arguments[0].value = typed_string_expr("wrong");
        assert_eq!(
            plan_module(constructor_argument_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: crate::planner::InvalidExpressionType::Int,
                    actual: crate::planner::InvalidExpressionType::String,
                },
            }),
        );

        let mut unlinked_function = function_call_module();
        replace_function_callee_with_module_select(
            &mut unlinked_function,
            "other",
            "missing",
            "other",
            "missing",
            None,
            None,
        );
        assert_eq!(
            plan_module(unlinked_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "missing".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
                },
            }),
        );

        let mut missing_function = function_call_module();
        replace_function_callee_with_module_select(
            &mut missing_function,
            "main",
            "missing",
            "main",
            "missing",
            None,
            None,
        );
        assert_eq!(
            plan_module(missing_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "missing".into(),
                    reason: InvalidModuleReferenceReason::MissingFunction,
                },
            }),
        );

        let mut function_module_mismatch = function_call_module();
        replace_function_callee_with_module_select(
            &mut function_module_mismatch,
            "main",
            "identity",
            "other",
            "identity",
            None,
            None,
        );
        assert_eq!(
            plan_module(function_module_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::FunctionModule {
                        actual: "other".into(),
                    },
                },
            }),
        );

        let mut function_label_mismatch = function_call_module();
        replace_function_callee_with_module_select(
            &mut function_label_mismatch,
            "main",
            "other",
            "main",
            "identity",
            None,
            None,
        );
        assert_eq!(
            plan_module(function_label_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "other".into(),
                    reason: InvalidModuleReferenceReason::FunctionName {
                        actual: "identity".into(),
                    },
                },
            }),
        );

        let mut external_function = function_call_module();
        replace_function_callee_with_module_select(
            &mut external_function,
            "main",
            "identity",
            "main",
            "identity",
            Some(("external".into(), "identity".into())),
            None,
        );
        assert_eq!(
            plan_module(external_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::ExternalFunction,
                },
            }),
        );

        let mut javascript_external_function = function_call_module();
        replace_function_callee_with_module_select(
            &mut javascript_external_function,
            "main",
            "identity",
            "main",
            "identity",
            None,
            Some(("external".into(), "identity".into())),
        );
        assert_eq!(
            plan_module(javascript_external_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::ExternalFunction,
                },
            }),
        );

        let mut direct_external_function = compile(
            r#"
@external(erlang, "external", "identity")
fn identity(value: Int) -> Int

pub fn main() {
  identity(1)
}
"#,
        );
        direct_external_function
            .definitions
            .functions
            .retain(|function| {
                function
                    .name
                    .as_ref()
                    .is_some_and(|(_, name)| name == "main")
            });
        assert_eq!(
            plan_module(direct_external_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::ExternalFunction,
                },
            }),
        );

        let mut direct_unlinked_constant = compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

const operation = add_one

pub fn main() {
  operation(1)
}
"#,
        );
        direct_unlinked_constant.name = "other".into();
        direct_unlinked_constant.definitions.constants.clear();
        direct_unlinked_constant
            .definitions
            .functions
            .retain(|function| {
                function
                    .name
                    .as_ref()
                    .is_some_and(|(_, name)| name == "main")
            });
        assert_eq!(
            plan_module(direct_unlinked_constant),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "operation".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
                },
            }),
        );

        let constant_call = TypedExpr::Call {
            location: dummy_span(),
            type_: type_::int(),
            fun: Box::new(TypedExpr::ModuleSelect {
                location: dummy_span(),
                field_start: 0,
                type_: type_::int(),
                label: "answer".into(),
                module_name: "main".into(),
                module_alias: "main".into(),
                constructor: ModuleValueConstructor::Constant {
                    literal: Constant::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: 1.into(),
                    },
                    location: dummy_span(),
                    documentation: None,
                },
            }),
            arguments: Vec::new(),
            open_parenthesis: Some(0),
        };
        assert_eq!(
            plan_module(module_returning_typed_expr(constant_call)),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "answer".into(),
                    reason: InvalidModuleReferenceReason::NonCallableConstant,
                },
            }),
        );

        let unlinked_constant_call = TypedExpr::Call {
            location: dummy_span(),
            type_: type_::int(),
            fun: Box::new(TypedExpr::ModuleSelect {
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
                        int_value: 1.into(),
                    },
                    location: dummy_span(),
                    documentation: None,
                },
            }),
            arguments: Vec::new(),
            open_parenthesis: Some(0),
        };
        assert_eq!(
            plan_module(module_returning_typed_expr(unlinked_constant_call)),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "answer".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
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
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "answer".into(),
                    reason: InvalidModuleReferenceReason::NonCallableConstant,
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
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
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
    #[should_panic(expected = "expected module function callee")]
    fn module_select_function_helper_rejects_function_value_calls() {
        let mut module = compile(
            r#"
fn identity(value: Int) {
  value
}

fn provider() {
  identity
}

pub fn main() {
  provider()(1)
}
"#,
        );

        replace_function_callee_with_module_select(
            &mut module,
            "main",
            "identity",
            "main",
            "identity",
            None,
            None,
        );
    }

    #[test]
    #[should_panic(expected = "expected module function constructor")]
    fn module_select_function_helper_rejects_record_constructors() {
        let mut module = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed(1)
}
"#,
        );

        replace_function_callee_with_module_select(
            &mut module,
            "main",
            "identity",
            "main",
            "identity",
            None,
            None,
        );
    }

    fn module_select_record(module: &str, arity: u16) -> TypedExpr {
        module_select_record_with_label(module, "Boxed", arity)
    }

    fn module_select_record_with_label(module: &str, label: &str, arity: u16) -> TypedExpr {
        let custom_type = type_::named("geam", module, "Boxed", Publicity::Public, Vec::new());
        module_select_record_with_type(
            module,
            label,
            arity,
            type_::fn_(vec![type_::int()], custom_type),
        )
    }

    fn module_select_record_with_type(
        module: &str,
        label: &str,
        arity: u16,
        constructor_type: std::sync::Arc<type_::Type>,
    ) -> TypedExpr {
        TypedExpr::ModuleSelect {
            location: dummy_span(),
            field_start: 0,
            type_: constructor_type.clone(),
            label: label.into(),
            module_name: module.into(),
            module_alias: module.into(),
            constructor: ModuleValueConstructor::Record {
                name: "Boxed".into(),
                variant_index: 0,
                arity,
                type_: constructor_type,
                field_map: None,
                location: dummy_span(),
                documentation: None,
            },
        }
    }

    fn function_call_module() -> TypedModule {
        compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        )
    }

    fn replace_function_callee_with_module_select(
        module: &mut TypedModule,
        module_name: &str,
        label: &str,
        target_module: &str,
        target_name: &str,
        external_erlang: Option<(ecow::EcoString, ecow::EcoString)>,
        external_javascript: Option<(ecow::EcoString, ecow::EcoString)>,
    ) {
        let main = module
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
        let (_, function, _) = expect_call_statement_mut(&mut main.body[0]);
        let TypedExpr::Var {
            location,
            name: _,
            constructor,
        } = std::mem::replace(function, super::super::typed_int_expr(0))
        else {
            panic!("expected module function callee");
        };
        let type_ = constructor.type_.clone();
        let ValueConstructorVariant::ModuleFn {
            location: definition_location,
            field_map,
            external_javascript: _,
            documentation,
            purity,
            ..
        } = constructor.variant
        else {
            panic!("expected module function constructor");
        };

        *function = TypedExpr::ModuleSelect {
            location,
            field_start: 0,
            type_,
            label: label.into(),
            module_name: module_name.into(),
            module_alias: module_name.into(),
            constructor: ModuleValueConstructor::Fn {
                location: definition_location,
                module: target_module.into(),
                name: target_name.into(),
                external_erlang,
                external_javascript,
                field_map,
                documentation,
                purity,
            },
        };
    }
}
