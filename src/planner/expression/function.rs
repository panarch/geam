mod free_variables;

use crate::plan::{
    CaptureArg, Expr, FunctionExpr, FunctionType, ParamLocal, RuntimeFunctionId, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidExpressionType, InvalidFunctionShapeReason,
    InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
};
use crate::planner::function::{anonymous_function_plan, plan_anonymous_function_body};
use crate::planner::module::{ParamLabelPolicy, function_params};
use gleam_core::ast::{
    CAPTURE_VARIABLE, CallArg as GleamCallArg, FunctionLiteralKind, Statement, TypedArg, TypedExpr,
    TypedStatement,
};
use gleam_core::type_::{Type, ValueConstructorVariant};
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
            validate_capture_literal(&arguments, &body)?;
        }
        FunctionLiteralKind::Use { .. } => {
            return Err(invalid_function_literal_kind_error());
        }
    }

    let function_type = anonymous_function_type(type_.as_ref())?;
    let error_name = context.anonymous_function_error_name();
    let params = function_params(error_name.clone(), &arguments, ParamLabelPolicy::Reject)?;
    validate_argument_types(&error_name, &function_type, &params)?;
    plan_anonymous_with_valid_arguments(function_type, params, arguments, body, context)
}

fn plan_anonymous_with_valid_arguments(
    function_type: FunctionType,
    params: Vec<crate::planner::context::FunctionParam>,
    arguments: Vec<TypedArg>,
    body: Vec1<TypedStatement>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let free_names = free_variables::anonymous_free_variables(&arguments, &body);
    let captures = context.capture_bindings(&free_names)?;
    plan_anonymous_with_captures(function_type, params, captures, body, context)
}

fn plan_anonymous_with_captures(
    function_type: FunctionType,
    params: Vec<crate::planner::context::FunctionParam>,
    captures: Vec<crate::planner::context::CaptureBinding>,
    body: Vec1<TypedStatement>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let return_type = function_type.return_().clone();
    let runtime_id = context.allocate_anonymous_runtime_id(&return_type);
    let name = context.reserve_anonymous_function_name();

    let planned = {
        let mut body_context = context.anonymous_function_context(name.clone());
        plan_anonymous_function_body(
            &name,
            &return_type,
            &runtime_id,
            &params,
            captures,
            body,
            &mut body_context,
        )
    };

    let planned = planned?;
    let (name, info) = context.allocate_anonymous_function(name, return_type, params, runtime_id);
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

fn validate_capture_literal(
    arguments: &[TypedArg],
    body: &Vec1<TypedStatement>,
) -> Result<(), PlanError> {
    let [argument] = arguments else {
        return Err(invalid_capture_literal_shape());
    };

    if argument.get_variable_name().map(|name| name.as_str()) != Some(CAPTURE_VARIABLE) {
        return Err(invalid_capture_literal_shape());
    }

    let [Statement::Expression(TypedExpr::Call { arguments, .. })] = body.as_slice() else {
        return Err(invalid_capture_literal_shape());
    };

    if count_capture_literal_arguments(arguments) == 1 {
        Ok(())
    } else {
        Err(invalid_capture_literal_shape())
    }
}

fn count_capture_literal_arguments(arguments: &[GleamCallArg<TypedExpr>]) -> usize {
    arguments
        .iter()
        .filter(|argument| is_capture_literal_local(&argument.value))
        .count()
}

fn is_capture_literal_local(expression: &TypedExpr) -> bool {
    matches!(
        expression,
        TypedExpr::Var {
            name,
            constructor,
            ..
        } if name.as_str() == CAPTURE_VARIABLE
            && matches!(
                constructor.variant,
                ValueConstructorVariant::LocalVariable { .. }
            )
    )
}

fn invalid_function_literal_kind_error() -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionShape {
            kind: InvalidExpressionShapeKind::Invalid,
        },
    }
}

fn invalid_capture_literal_shape() -> PlanError {
    PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionShape {
            kind: InvalidExpressionShapeKind::FunctionCaptureLiteral,
        },
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
        RuntimeFunctionId::Float(runtime_id) => FunctionExpr::float(
            crate::plan::FloatFunctionExpr::closure(*runtime_id, params, captures, type_),
        ),
        RuntimeFunctionId::Bool(runtime_id) => FunctionExpr::bool(
            crate::plan::BoolFunctionExpr::closure(*runtime_id, params, captures, type_),
        ),
        RuntimeFunctionId::Nil(runtime_id) => FunctionExpr::nil(
            crate::plan::NilFunctionExpr::closure(*runtime_id, params, captures, type_),
        ),
        RuntimeFunctionId::Tuple { id, return_type } => {
            FunctionExpr::tuple(crate::plan::TupleFunctionExpr::closure(
                *id,
                params,
                captures,
                type_,
                return_type.clone(),
            ))
        }
        RuntimeFunctionId::List(id) => FunctionExpr::list(crate::plan::ListFunctionExpr::closure(
            id.clone(),
            params,
            captures,
        )),
        RuntimeFunctionId::Function { id, return_type } => {
            FunctionExpr::function(crate::plan::FunctionFunctionExpr::closure(
                id.clone(),
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
        Some(ValueType::Float) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Function,
                actual: InvalidExpressionType::Float,
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
        Some(ValueType::Tuple(_)) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Function,
                actual: InvalidExpressionType::Tuple,
            },
        }),
        Some(ValueType::List(_)) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Function,
                actual: InvalidExpressionType::List,
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
        Expr, FunctionFunctionId, FunctionType, IntExpr, IntFunctionFunctionId, IntFunctionId,
        IntLocalId, LocalId, PanicExpr, PanicSite, ParamLocal, ReturnExpr, RuntimeFunctionId,
        SourceSpan, StringExpr, TupleFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        call_int_function, capture_int, capture_tuple, function, function_function_closure,
        function_function_ref, function_ref, int, int_arg, int_function_call_arg,
        int_function_closure, int_function_ref, int_return_tail_call, let_int_function_step,
        let_int_step, let_tuple_step, local_int, local_int_function, local_tuple,
        module_with_anonymous, string, tuple, tuple_function_closure,
    };
    use crate::planner::error::{
        InvalidExpressionShapeKind, InvalidExpressionType, InvalidFunctionShapeReason,
        InvalidPipelineShapeReason, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
        UnsupportedPatternKind,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use gleam_core::ast::{
        ArgNames, CAPTURE_VARIABLE, CallArg as GleamCallArg, Constant, FunctionLiteralKind,
        PipelineAssignmentKind, Statement, TypedArg, TypedExpr, TypedModule,
        TypedPipelineAssignment, TypedStatement,
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
    fn plan_nested_anonymous_function_reserves_outer_name_before_body() {
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
                function("<anonymous:1>", local_int(0, "value").add_int(int(1)))
                    .param_int(0, "value"),
                function(
                    "<anonymous:0>",
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
    fn plan_nested_anonymous_function_panic_site_uses_reserved_outer_name() {
        let actual = plan_module(compile(
            r#"
pub fn main() -> Int {
  let outer = fn() {
    fn() { 1 }
    panic as "outer"
  }
  outer()
}
"#,
        ))
        .expect("source should plan");
        let anonymous_functions = actual.anonymous_functions();
        let inner_function = &anonymous_functions[0];
        let outer_function = &anonymous_functions[1];

        assert_eq!(inner_function.name(), "<anonymous:1>");
        assert_eq!(outer_function.name(), "<anonymous:0>");

        assert_eq!(
            outer_function.return_(),
            &ReturnExpr::int(
                IntFunctionId(1),
                IntExpr::panic(PanicExpr::panic_at(
                    Some(StringExpr::value("outer".into())),
                    PanicSite::new(
                        "main".into(),
                        "<anonymous:0>".into(),
                        SourceSpan::new(64, 80)
                    ),
                )),
            ),
        );
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
                function("<anonymous:1>", local_int(0, "value")),
                function(
                    "<anonymous:0>",
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
    fn closure_expr_preserves_tuple_return_family() {
        let return_type = vec![ValueType::Int];
        let expression = super::closure_expr(
            &RuntimeFunctionId::Tuple {
                id: TupleFunctionId(0),
                return_type: return_type.clone(),
            },
            Vec::<ParamLocal>::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Tuple(return_type.clone())),
        );

        assert_eq!(
            expression.type_(),
            &FunctionType::new(Vec::new(), ValueType::Tuple(return_type)),
        );
    }

    #[test]
    fn plan_tuple_returning_capturing_anonymous_function() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let pair = #(1, "one")
  fn() { pair }
}
"#,
        ))
        .expect("source should plan");
        let pair_type = [ValueType::Int, ValueType::String];
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                tuple_function_closure(
                    0,
                    Vec::<LocalId>::new(),
                    [capture_tuple(0, local_tuple(0, "pair", pair_type.clone()))],
                    pair_type.clone(),
                ),
            )
            .step(let_tuple_step(
                0,
                "pair",
                tuple([Expr::from(int(1)), Expr::from(string("one"))]),
            )),
            [],
            [function("<anonymous:0>", local_tuple(0, "pair", pair_type))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_capture_literal() {
        let actual = plan_module(compile(
            r#"
fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  let add_one = add(1, _)
  add_one(41)
}
"#,
        ))
        .expect("source should plan");
        let add_one = int_function_ref(2, [LocalId::Int(IntLocalId(0))]);
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
            [
                function("add", local_int(0, "left").add_int(local_int(1, "right")))
                    .param_int(0, "left")
                    .param_int(1, "right"),
            ],
            [function(
                "<anonymous:0>",
                int_return_tail_call(
                    1,
                    [
                        int_arg(0, int(1)),
                        int_arg(1, local_int(0, CAPTURE_VARIABLE)),
                    ],
                ),
            )
            .param_int(0, CAPTURE_VARIABLE)],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_capture_labelled_argument() {
        let actual = plan_module(compile(
            r#"
fn add(to base: Int, value amount: Int) {
  base + amount
}

pub fn main() {
  let add_one = add(to: 1, value: _)
  add_one(41)
}
"#,
        ))
        .expect("source should plan");
        let add_one = int_function_ref(2, [LocalId::Int(IntLocalId(0))]);
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
            [
                function("add", local_int(0, "base").add_int(local_int(1, "amount")))
                    .param_int(0, "base")
                    .param_int(1, "amount"),
            ],
            [function(
                "<anonymous:0>",
                int_return_tail_call(
                    1,
                    [
                        int_arg(0, int(1)),
                        int_arg(1, local_int(0, CAPTURE_VARIABLE)),
                    ],
                ),
            )
            .param_int(0, CAPTURE_VARIABLE)],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_capture_literal_with_closure_capture() {
        let actual = plan_module(compile(
            r#"
fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  let base = 1
  let add_base = add(base, _)
  add_base(41)
}
"#,
        ))
        .expect("source should plan");
        let add_base = int_function_closure(
            2,
            [LocalId::Int(IntLocalId(0))],
            [capture_int(1, local_int(0, "base"))],
        );
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function(
                    local_int_function(0, "add_base", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(0, int(41))],
                ),
            )
            .step(let_int_step(0, "base", int(1)))
            .step(let_int_function_step(0, "add_base", add_base)),
            [
                function("add", local_int(0, "left").add_int(local_int(1, "right")))
                    .param_int(0, "left")
                    .param_int(1, "right"),
            ],
            [function(
                "<anonymous:0>",
                int_return_tail_call(
                    1,
                    [
                        int_arg(0, local_int(1, "base")),
                        int_arg(1, local_int(0, CAPTURE_VARIABLE)),
                    ],
                ),
            )
            .param_int(0, CAPTURE_VARIABLE)],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_unsupported_anonymous_function_type() {
        assert_eq!(
            plan_module(compile(
                r#"
pub fn main() {
  fn() { <<>> }
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
            (gleam_core::type_::float(), InvalidExpressionType::Float),
            (gleam_core::type_::bool(), InvalidExpressionType::Bool),
            (gleam_core::type_::nil(), InvalidExpressionType::Nil),
            (
                gleam_core::type_::tuple(vec![gleam_core::type_::int()]),
                InvalidExpressionType::Tuple,
            ),
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
    fn reject_margin_anonymous_function_param_shape_error_propagates() {
        let mut module = anonymous_function_module();
        let (_, arguments, _) = anonymous_function_expression_mut(&mut module);
        arguments[0] = labelled_arg();

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous:0>".into(),
                    reason: InvalidFunctionShapeReason::LabelledArgument,
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
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Function,
                    actual: InvalidExpressionType::List,
                },
            }),
        );

        let mut invalid_shape = anonymous_function_module();
        let (type_, _, _) = anonymous_function_expression_mut(&mut invalid_shape);
        *type_ = gleam_core::type_::bit_array();

        assert_eq!(
            plan_module(invalid_shape),
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
    fn reject_profile_anonymous_function_let_assert_constructor_pattern() {
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
            Err(PlanError::UnsupportedPattern {
                kind: UnsupportedPatternKind::Constructor,
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
    fn reject_margin_function_capture_literal_argument_shape() {
        for mutate in [
            |arguments: &mut Vec<TypedArg>, _: &mut vec1::Vec1<TypedStatement>| {
                arguments.push(arguments[0].clone());
            },
            |arguments: &mut Vec<TypedArg>, _: &mut vec1::Vec1<TypedStatement>| {
                arguments[0].names = ArgNames::Named {
                    name: "other".into(),
                    location: dummy_span(),
                };
            },
        ] {
            let mut module = function_capture_literal_module();
            let (arguments, body, _) = function_capture_literal_expression_mut(&mut module);
            mutate(arguments, body);

            assert_eq!(
                plan_module(module),
                Err(invalid_function_capture_literal_shape()),
            );
        }
    }

    #[test]
    fn reject_margin_function_capture_literal_body_shape() {
        for mutate in [
            |_: &mut Vec<TypedArg>, body: &mut vec1::Vec1<TypedStatement>| {
                body[0] = Statement::Expression(super::super::typed_int_expr(1));
            },
            |_: &mut Vec<TypedArg>, body: &mut vec1::Vec1<TypedStatement>| {
                let arguments = function_capture_literal_body_call_args_mut(body);
                let capture_index = capture_argument_index(arguments);
                arguments[capture_index].value = super::super::typed_int_expr(1);
            },
            |_: &mut Vec<TypedArg>, body: &mut vec1::Vec1<TypedStatement>| {
                let arguments = function_capture_literal_body_call_args_mut(body);
                let capture = arguments[capture_argument_index(arguments)].clone();
                arguments.push(capture);
            },
            |_: &mut Vec<TypedArg>, body: &mut vec1::Vec1<TypedStatement>| {
                let arguments = function_capture_literal_body_call_args_mut(body);
                let capture_index = capture_argument_index(arguments);
                arguments[capture_index].value = TypedExpr::Var {
                    location: dummy_span(),
                    name: CAPTURE_VARIABLE.into(),
                    constructor: gleam_core::type_::ValueConstructor {
                        publicity: gleam_core::ast::Publicity::Private,
                        deprecation: gleam_core::type_::Deprecation::NotDeprecated,
                        type_: gleam_core::type_::int(),
                        variant: gleam_core::type_::ValueConstructorVariant::Record {
                            name: "Capture".into(),
                            arity: 1,
                            field_map: None,
                            location: dummy_span(),
                            module: "main".into(),
                            variants_count: 1,
                            variant_index: 0,
                            documentation: None,
                        },
                    },
                };
            },
        ] {
            let mut module = function_capture_literal_module();
            let (arguments, body, _) = function_capture_literal_expression_mut(&mut module);
            mutate(arguments, body);

            assert_eq!(
                plan_module(module),
                Err(invalid_function_capture_literal_shape()),
            );
        }
    }

    #[test]
    #[should_panic(expected = "expected function capture literal expression statement")]
    fn function_capture_literal_expression_mut_panics_on_non_function_statement() {
        let mut module = compile(r#"pub fn main() { 1 }"#);

        let _ = function_capture_literal_expression_mut(&mut module);
    }

    #[test]
    #[should_panic(expected = "expected function capture literal call body")]
    fn function_capture_literal_body_call_args_mut_panics_on_non_call_body() {
        let mut module = function_capture_literal_module();
        let (_, body, _) = function_capture_literal_expression_mut(&mut module);
        body[0] = Statement::Expression(super::super::typed_int_expr(1));

        let _ = function_capture_literal_body_call_args_mut(body);
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

    fn function_capture_literal_module() -> TypedModule {
        compile(
            r#"
fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  add(1, _)
}
"#,
        )
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

    fn function_capture_literal_expression_mut(
        module: &mut TypedModule,
    ) -> (
        &mut Vec<TypedArg>,
        &mut vec1::Vec1<TypedStatement>,
        &mut FunctionLiteralKind,
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
            .expect("expected main function");

        let Statement::Expression(TypedExpr::Fn {
            arguments,
            body,
            kind,
            ..
        }) = &mut main.body[0]
        else {
            panic!("expected function capture literal expression statement");
        };

        (arguments, body, kind)
    }

    fn function_capture_literal_body_call_args_mut(
        body: &mut vec1::Vec1<TypedStatement>,
    ) -> &mut Vec<GleamCallArg<TypedExpr>> {
        let (_, arguments) = function_capture_literal_body_call_parts_mut(body);

        arguments
    }

    fn function_capture_literal_body_call_parts_mut(
        body: &mut vec1::Vec1<TypedStatement>,
    ) -> (&mut Box<TypedExpr>, &mut Vec<GleamCallArg<TypedExpr>>) {
        assert_eq!(
            body.len(),
            1,
            "expected single capture literal body statement"
        );
        let Statement::Expression(TypedExpr::Call { fun, arguments, .. }) = &mut body[0] else {
            panic!("expected function capture literal call body");
        };

        (fun, arguments)
    }

    fn capture_argument_index(arguments: &[GleamCallArg<TypedExpr>]) -> usize {
        arguments
            .iter()
            .position(|argument: &GleamCallArg<TypedExpr>| argument.is_capture_hole())
            .expect("expected capture literal argument")
    }

    fn invalid_function_capture_literal_shape() -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::FunctionCaptureLiteral,
            },
        }
    }

    fn labelled_arg() -> TypedArg {
        TypedArg {
            names: ArgNames::NamedLabelled {
                label: "label".into(),
                label_location: dummy_span(),
                name: "value".into(),
                name_location: dummy_span(),
            },
            location: dummy_span(),
            annotation: None,
            type_: gleam_core::type_::int(),
        }
    }
}
