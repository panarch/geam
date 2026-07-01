use crate::plan::{
    CaptureArg, Expr, FunctionExpr, FunctionType, ParamLocal, RuntimeFunctionId, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidExpressionType, InvalidFunctionShapeReason,
    InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
};
use crate::planner::function::{anonymous_function_plan, plan_anonymous_function_body};
use crate::planner::module::function_params;
use ecow::EcoString;
use gleam_core::ast::{
    FunctionLiteralKind, Pattern, Statement, TypedArg, TypedExpr, TypedPipelineAssignment,
    TypedStatement,
};
use gleam_core::type_::{Type, ValueConstructorVariant};
use std::collections::HashSet;
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
        FunctionLiteralKind::Capture { .. } | FunctionLiteralKind::Use { .. } => {
            return Err(function_literal_kind_error(kind));
        }
    }

    let function_type = anonymous_function_type(type_.as_ref())?;
    let error_name = context.anonymous_function_error_name();
    let params = function_params(error_name.clone(), &arguments)?;
    validate_argument_types(&error_name, &function_type, &params).and_then(|()| {
        plan_anonymous_with_valid_arguments(
            function_type,
            error_name,
            params,
            arguments,
            body,
            context,
        )
    })
}

pub(super) fn plan_use_callback(
    type_: Arc<Type>,
    arguments: Vec<TypedArg>,
    body: Vec1<TypedStatement>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let function_type = anonymous_function_type(type_.as_ref())?;
    let error_name = context.anonymous_function_error_name();
    let params = function_params(error_name.clone(), &arguments)?;
    validate_argument_types(&error_name, &function_type, &params).and_then(|()| {
        plan_anonymous_with_valid_arguments(
            function_type,
            error_name,
            params,
            arguments,
            body,
            context,
        )
    })
}

fn plan_anonymous_with_valid_arguments(
    function_type: FunctionType,
    error_name: EcoString,
    params: Vec<crate::planner::context::FunctionParam>,
    arguments: Vec<TypedArg>,
    body: Vec1<TypedStatement>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let free_names = anonymous_free_variables(&arguments, &body);
    context.capture_bindings(&free_names).and_then(|captures| {
        plan_anonymous_with_captures(function_type, error_name, params, captures, body, context)
    })
}

fn plan_anonymous_with_captures(
    function_type: FunctionType,
    error_name: EcoString,
    params: Vec<crate::planner::context::FunctionParam>,
    captures: Vec<crate::planner::context::CaptureBinding>,
    body: Vec1<TypedStatement>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let return_type = function_type.return_().clone();
    let runtime_id = context.allocate_anonymous_runtime_id(&return_type);

    let planned = {
        let mut body_context = context.anonymous_function_context();
        plan_anonymous_function_body(
            &error_name,
            &return_type,
            &runtime_id,
            &params,
            captures,
            body,
            &mut body_context,
        )
    };

    let planned = planned?;
    let (name, info) = context.allocate_anonymous_function(return_type, params, runtime_id);
    let value = if planned.captures.is_empty() {
        FunctionExpr::value(info.value())
    } else {
        closure_expr(
            &info.runtime_id,
            info.param_locals(),
            planned.captures.clone(),
            function_type,
        )
    };
    let function = anonymous_function_plan(info, name, planned);
    context.push_anonymous_function(function);
    Ok(Expr::function(value))
}

fn function_literal_kind_error(kind: FunctionLiteralKind) -> PlanError {
    let invalid = PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionShape {
            kind: InvalidExpressionShapeKind::Invalid,
        },
    };

    kind.is_capture()
        .then(function_capture_literal_error)
        .map_or(invalid, |error| error)
}

fn function_capture_literal_error() -> PlanError {
    PlanError::UnsupportedExpression {
        kind: UnsupportedExpressionKind::FunctionCaptureLiteral,
    }
}

fn closure_expr(
    runtime_id: &RuntimeFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureArg>,
    type_: FunctionType,
) -> FunctionExpr {
    match runtime_id {
        RuntimeFunctionId::Int(runtime_id) => FunctionExpr::int(
            crate::plan::IntFunctionExpr::closure(*runtime_id, params, captures, type_),
        ),
        RuntimeFunctionId::String(runtime_id) => FunctionExpr::string(
            crate::plan::StringFunctionExpr::closure(*runtime_id, params, captures, type_),
        ),
        RuntimeFunctionId::Bool(runtime_id) => FunctionExpr::bool(
            crate::plan::BoolFunctionExpr::closure(*runtime_id, params, captures, type_),
        ),
        RuntimeFunctionId::Nil(runtime_id) => FunctionExpr::nil(
            crate::plan::NilFunctionExpr::closure(*runtime_id, params, captures, type_),
        ),
        RuntimeFunctionId::Function { id, return_type } => {
            FunctionExpr::function(crate::plan::FunctionFunctionExpr::closure(
                *id,
                params,
                captures,
                type_,
                return_type.clone(),
            ))
        }
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
        None => Err(anonymous_function_type_error(type_)),
    }
}

fn anonymous_function_type_error(type_: &Type) -> PlanError {
    match type_.fn_types() {
        Some(_) => PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::UnsupportedFunctionLiteralType,
        },
        None => PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        },
    }
}

fn anonymous_free_variables(arguments: &[TypedArg], body: &Vec1<TypedStatement>) -> Vec<EcoString> {
    let mut bound = HashSet::new();
    for argument in arguments {
        bound.extend(argument.get_variable_name().cloned());
    }

    let mut free = FreeVariables::new();
    collect_statements(body.as_slice(), &mut bound, &mut free);
    free.names
}

struct FreeVariables {
    names: Vec<EcoString>,
    seen: HashSet<EcoString>,
}

impl FreeVariables {
    fn new() -> Self {
        Self {
            names: Vec::new(),
            seen: HashSet::new(),
        }
    }

    fn record(&mut self, name: &EcoString, bound: &HashSet<EcoString>) {
        if !bound.contains(name) && self.seen.insert(name.clone()) {
            self.names.push(name.clone());
        }
    }
}

fn collect_statements(
    statements: &[TypedStatement],
    bound: &mut HashSet<EcoString>,
    free: &mut FreeVariables,
) {
    for statement in statements {
        collect_statement(statement, bound, free);
    }
}

fn collect_statement(
    statement: &TypedStatement,
    bound: &mut HashSet<EcoString>,
    free: &mut FreeVariables,
) {
    match statement {
        Statement::Expression(expression) => collect_expr(expression, bound, free),
        Statement::Assignment(assignment) => {
            collect_expr(&assignment.value, bound, free);
            collect_variable_pattern_bound_name(&assignment.pattern, bound);
        }
        Statement::Use(use_) => {
            collect_expr(&use_.call, bound, free);
        }
        Statement::Assert(_) => {}
    }
}

fn collect_expr(expression: &TypedExpr, bound: &mut HashSet<EcoString>, free: &mut FreeVariables) {
    match expression {
        TypedExpr::Int { .. }
        | TypedExpr::Float { .. }
        | TypedExpr::String { .. }
        | TypedExpr::Invalid { .. } => {}
        TypedExpr::Var {
            name, constructor, ..
        } => {
            if matches!(
                constructor.variant,
                ValueConstructorVariant::LocalVariable { .. }
            ) {
                free.record(name, bound);
            }
        }
        TypedExpr::Block { statements, .. } => {
            let mut block_bound = bound.clone();
            collect_statements(statements.as_slice(), &mut block_bound, free);
        }
        TypedExpr::Pipeline {
            first_value,
            assignments,
            finally,
            ..
        } => {
            let mut pipeline_bound = bound.clone();
            collect_pipeline_assignment(first_value, &mut pipeline_bound, free);
            for (assignment, _) in assignments {
                collect_pipeline_assignment(assignment, &mut pipeline_bound, free);
            }
            collect_expr(finally, &mut pipeline_bound, free);
        }
        TypedExpr::Fn {
            arguments, body, ..
        } => {
            for name in anonymous_free_variables(arguments, body) {
                free.record(&name, bound);
            }
        }
        TypedExpr::Call { fun, arguments, .. } => {
            collect_expr(fun, bound, free);
            for argument in arguments {
                collect_expr(&argument.value, bound, free);
            }
        }
        TypedExpr::BinOp { left, right, .. } => {
            collect_expr(left, bound, free);
            collect_expr(right, bound, free);
        }
        TypedExpr::Case {
            subjects, clauses, ..
        } => {
            for subject in subjects {
                collect_expr(subject, bound, free);
            }
            for clause in clauses {
                let mut branch_bound = bound.clone();
                for pattern in &clause.pattern {
                    collect_variable_pattern_bound_name(pattern, &mut branch_bound);
                }
                collect_expr(&clause.then, &mut branch_bound, free);
            }
        }
        TypedExpr::NegateBool { value, .. } | TypedExpr::NegateInt { value, .. } => {
            collect_expr(value, bound, free);
        }
        TypedExpr::List { .. }
        | TypedExpr::RecordAccess { .. }
        | TypedExpr::PositionalAccess { .. }
        | TypedExpr::Tuple { .. }
        | TypedExpr::TupleIndex { .. }
        | TypedExpr::Todo { .. }
        | TypedExpr::Panic { .. }
        | TypedExpr::Echo { .. }
        | TypedExpr::BitArray { .. }
        | TypedExpr::RecordUpdate { .. }
        | TypedExpr::ModuleSelect { .. } => {}
    }
}

fn collect_pipeline_assignment(
    assignment: &TypedPipelineAssignment,
    bound: &mut HashSet<EcoString>,
    free: &mut FreeVariables,
) {
    collect_expr(&assignment.value, bound, free);
    bound.insert(assignment.name.clone());
}

fn collect_variable_pattern_bound_name(
    pattern: &Pattern<Arc<Type>>,
    bound: &mut HashSet<EcoString>,
) {
    match pattern {
        Pattern::Variable { name, .. } => {
            bound.insert(name.clone());
        }
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::Assign { .. }
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::BitArraySize(_)
        | Pattern::Discard { .. }
        | Pattern::Invalid { .. } => {}
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
        call_int_function, capture_int, function, function_function_closure, function_function_ref,
        function_ref, int, int_arg, int_function_call_arg, int_function_closure, int_function_ref,
        int_return_tail_call, let_int_function_step, let_int_step, local_int, local_int_function,
        module_with_anonymous,
    };
    use crate::planner::error::{
        InvalidExpressionShapeKind, InvalidExpressionType, InvalidFunctionShapeReason,
        InvalidPipelineShapeReason, InvalidTypedAstReason, PlanError, UnsupportedAssignmentKind,
        UnsupportedExpressionKind, UnsupportedStatementKind,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use gleam_core::ast::{
        Constant, FunctionLiteralKind, PipelineAssignmentKind, Statement, TypedArg, TypedExpr,
        TypedModule, TypedPipelineAssignment, TypedStatement,
    };
    use gleam_core::type_::ModuleValueConstructor;

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
        let add_one = int_function_ref(1, [LocalId::Int(IntLocalId(0))]);
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function(
                    local_int_function(0, "add_one", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(0, int(41))],
                ),
            )
            .step(let_int_function_step(0, "add_one", add_one)),
            [],
            [
                function("<anonymous:0>", local_int(0, "value").add_int(int(1)))
                    .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_anonymous_function_discard_argument() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  fn(_: Int) { 1 }
  42
}
"#,
        ))
        .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function("main", int(42)).evaluate(int_function_ref(1, [LocalId::Int(IntLocalId(0))])),
            [],
            [function("<anonymous:0>", int(1)).discard_int_param(0)],
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
        let wrapped = int_function_ref(2, [LocalId::Int(IntLocalId(0))]);
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function(
                    local_int_function(0, "wrapped", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(0, int(41))],
                ),
            )
            .step(let_int_function_step(0, "wrapped", wrapped)),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
            [function(
                "<anonymous:0>",
                int_return_tail_call(1, [int_arg(0, local_int(0, "value"))]),
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
    fn plan_capturing_anonymous_function() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let value = 1
  fn() { value }
  1
}
"#,
        ))
        .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function("main", int(1))
                .step(let_int_step(0, "value", int(1)))
                .evaluate(int_function_closure(
                    1,
                    Vec::<LocalId>::new(),
                    [capture_int(0, local_int(0, "value"))],
                )),
            [],
            [function("<anonymous:0>", local_int(0, "value"))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_nested_capturing_anonymous_function() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let value = 1
  fn() { fn() { value } }
  1
}
"#,
        ))
        .expect("source should plan");
        let returned_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let expected = module_with_anonymous(
            "main",
            function("main", int(1))
                .step(let_int_step(0, "value", int(1)))
                .evaluate(function_function_closure(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    [capture_int(0, local_int(0, "value"))],
                    returned_function_type.clone(),
                )),
            [],
            [
                function("<anonymous:0>", local_int(0, "value")),
                function(
                    "<anonymous:1>",
                    int_function_closure(
                        1,
                        Vec::<LocalId>::new(),
                        [capture_int(0, local_int(0, "value"))],
                    ),
                ),
            ],
        );

        assert_eq!(actual, expected);
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
    fn reject_margin_anonymous_function_unknown_capture() {
        let mut module = compile(
            r#"
pub fn main() {
  let value = 1
  fn() { value }
  1
}
"#,
        );
        module.definitions.functions[0].body.remove(0);

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal {
                    name: "value".into(),
                },
            }),
        );
    }

    #[test]
    fn reject_margin_anonymous_function_module_select_body() {
        let mut module = anonymous_function_module();
        let body = anonymous_function_body_mut(&mut module);
        body[0] = Statement::Expression(TypedExpr::ModuleSelect {
            location: dummy_span(),
            field_start: 0,
            type_: gleam_core::type_::int(),
            label: "answer".into(),
            module_name: "other".into(),
            module_alias: "other".into(),
            constructor: ModuleValueConstructor::Constant {
                literal: Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: num_bigint::BigInt::from(1),
                },
                location: dummy_span(),
                documentation: None,
            },
        });

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::ModuleSelect,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_anonymous_function_pipeline_intermediate_shape() {
        let mut module = anonymous_function_module();
        let body = anonymous_function_body_mut(&mut module);
        body[0] = Statement::Expression(TypedExpr::Pipeline {
            location: dummy_span(),
            first_value: TypedPipelineAssignment {
                location: dummy_span(),
                name: "pipe_0".into(),
                value: Box::new(super::super::typed_int_expr(1)),
            },
            assignments: vec![(
                TypedPipelineAssignment {
                    location: dummy_span(),
                    name: "pipe_1".into(),
                    value: Box::new(super::super::typed_int_expr(2)),
                },
                PipelineAssignmentKind::FirstArgument {
                    second_argument: None,
                },
            )],
            finally: Box::new(super::super::typed_int_expr(3)),
            finally_kind: PipelineAssignmentKind::FirstArgument {
                second_argument: None,
            },
        });

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PipelineShape {
                    reason: InvalidPipelineShapeReason::NonCallStep,
                },
            }),
        );
    }

    #[test]
    fn reject_profile_anonymous_function_assert_statement() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  fn() {
    assert True
    1
  }
  1
}
"#,
            )),
            Err(PlanError::UnsupportedStatement {
                kind: UnsupportedStatementKind::Assert,
            }),
        );
    }

    #[test]
    fn reject_profile_anonymous_function_let_assert_assignment() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  fn() {
    let assert True = True
    1
  }
  1
}
"#,
            )),
            Err(PlanError::UnsupportedAssignment {
                kind: UnsupportedAssignmentKind::LetAssert,
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

    #[test]
    #[should_panic(expected = "expected anonymous function expression statement")]
    fn anonymous_function_body_mut_panics_on_non_function_statement() {
        let mut module = compile(r#"pub fn main() { 1 }"#);

        let _ = anonymous_function_body_mut(&mut module);
    }

    fn anonymous_function_module() -> TypedModule {
        compile("pub fn main() {\n  fn(value) { value + 1 }\n  1\n}\n")
    }

    fn anonymous_function_body_mut(module: &mut TypedModule) -> &mut vec1::Vec1<TypedStatement> {
        let Statement::Expression(TypedExpr::Fn { body, .. }) =
            &mut module.definitions.functions[0].body[0]
        else {
            panic!("expected anonymous function expression statement");
        };

        body
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
