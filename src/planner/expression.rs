mod block;
mod call;
mod case;
mod function;
mod operator;
mod pipeline;
mod var;

use crate::plan::{
    BoolExpr, Expr, FloatExpr, FunctionExpr, FunctionFunctionExpr, FunctionType, IntExpr,
    StringExpr, TupleExpr, ValueType,
};
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
        TypedExpr::Float { float_value, .. } => {
            Ok(Expr::float(FloatExpr::value(float_value.value())))
        }
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
        TypedExpr::Block { statements, .. } => block::plan(statements, context),
        TypedExpr::Tuple {
            type_, elements, ..
        } => plan_tuple(type_, elements, context),
        TypedExpr::TupleIndex {
            type_,
            index,
            tuple,
            ..
        } => plan_tuple_index(type_, index, *tuple, context),
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

fn plan_tuple(
    type_: std::sync::Arc<gleam_core::type_::Type>,
    elements: Vec<TypedExpr>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let planned_elements = elements
        .into_iter()
        .map(|element| plan_expr(element, context))
        .collect::<Result<Vec<_>, _>>()?;
    let actual_type = planned_elements
        .iter()
        .map(Expr::value_type)
        .collect::<Vec<_>>();
    let expected_type = match ValueType::from_gleam(type_.as_ref()) {
        Some(ValueType::Tuple(type_)) => type_,
        Some(actual) => {
            return Err(invalid_expression_type_for_value(
                ValueType::Tuple(actual_type),
                actual,
            ));
        }
        None => {
            return Err(invalid_expression_type(
                InvalidExpressionType::Tuple,
                InvalidExpressionType::Unsupported,
            ));
        }
    };

    if expected_type != actual_type {
        return Err(invalid_expression_type_for_value(
            ValueType::Tuple(expected_type),
            ValueType::Tuple(actual_type),
        ));
    }

    Ok(Expr::tuple(TupleExpr::value(
        planned_elements,
        expected_type,
    )))
}

fn plan_tuple_index(
    type_: std::sync::Arc<gleam_core::type_::Type>,
    index: u64,
    tuple: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    #[cfg(target_pointer_width = "64")]
    let index = index as usize;
    #[cfg(not(target_pointer_width = "64"))]
    let index = usize::try_from(index).map_err(|_| PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::Tuple,
            actual: InvalidExpressionType::Tuple,
        },
    })?;
    let tuple = plan_expr(tuple, context)?;
    let actual = expression_type(&tuple);
    let tuple = tuple
        .into_tuple()
        .ok_or_else(|| invalid_expression_type(InvalidExpressionType::Tuple, actual))?;
    let expected = ValueType::from_gleam(type_.as_ref()).ok_or_else(|| {
        invalid_expression_type(
            InvalidExpressionType::Unsupported,
            InvalidExpressionType::Tuple,
        )
    })?;
    let actual = tuple.type_().get(index).cloned().ok_or_else(|| {
        invalid_expression_type_for_value(expected.clone(), ValueType::Tuple(vec![]))
    })?;
    if actual != expected {
        return Err(invalid_expression_type_for_value(expected.clone(), actual));
    }

    tuple_index_expr(tuple, index, expected)
}

fn tuple_index_expr(
    tuple: TupleExpr,
    index: usize,
    return_type: ValueType,
) -> Result<Expr, PlanError> {
    match return_type {
        ValueType::Int => Ok(Expr::int(IntExpr::tuple_index(tuple, index))),
        ValueType::String => Ok(Expr::string(StringExpr::tuple_index(tuple, index))),
        ValueType::Float => Ok(Expr::float(FloatExpr::tuple_index(tuple, index))),
        ValueType::Bool => Ok(Expr::bool(BoolExpr::tuple_index(tuple, index))),
        ValueType::Nil => Ok(Expr::nil(crate::plan::NilExpr::tuple_index(tuple, index))),
        ValueType::Tuple(type_) => Ok(Expr::tuple(TupleExpr::tuple_index(tuple, index, type_))),
        ValueType::Function(type_) => Ok(tuple_index_function_expr(tuple, index, *type_)),
    }
}

fn tuple_index_function_expr(tuple: TupleExpr, index: usize, type_: FunctionType) -> Expr {
    match type_.return_() {
        ValueType::Int => Expr::function(FunctionExpr::int(
            crate::plan::IntFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::String => Expr::function(FunctionExpr::string(
            crate::plan::StringFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::Float => Expr::function(FunctionExpr::float(
            crate::plan::FloatFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::Bool => Expr::function(FunctionExpr::bool(
            crate::plan::BoolFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::Nil => Expr::function(FunctionExpr::nil(
            crate::plan::NilFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::Tuple(_) => Expr::function(FunctionExpr::tuple(
            crate::plan::TupleFunctionExpr::tuple_index(tuple, index, type_),
        )),
        ValueType::Function(_) => Expr::function(FunctionExpr::function(
            FunctionFunctionExpr::tuple_index(tuple, index, type_),
        )),
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

fn plan_float_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<FloatExpr, PlanError> {
    let expression = plan_expr(expression, context)?;
    let actual = expression_type(&expression);
    expression
        .into_float()
        .ok_or_else(|| invalid_expression_type(InvalidExpressionType::Float, actual))
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
        ValueType::Float => InvalidExpressionType::Float,
        ValueType::Bool => InvalidExpressionType::Bool,
        ValueType::Nil => InvalidExpressionType::Nil,
        ValueType::Tuple(_) => InvalidExpressionType::Tuple,
        ValueType::Function(_) => InvalidExpressionType::Function,
    }
}

fn value_type_expression_type(type_: ValueType) -> InvalidExpressionType {
    match type_ {
        ValueType::Int => InvalidExpressionType::Int,
        ValueType::String => InvalidExpressionType::String,
        ValueType::Float => InvalidExpressionType::Float,
        ValueType::Bool => InvalidExpressionType::Bool,
        ValueType::Nil => InvalidExpressionType::Nil,
        ValueType::Tuple(_) => InvalidExpressionType::Tuple,
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
pub(in crate::planner::expression) fn typed_tuple_expr(
    type_: std::sync::Arc<gleam_core::type_::Type>,
    elements: Vec<TypedExpr>,
) -> TypedExpr {
    TypedExpr::Tuple {
        location: crate::planner::support::dummy_span(),
        type_,
        elements,
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
        expression_type, invalid_expression_type, invalid_expression_type_for_value,
        module_returning_typed_expr, typed_int_expr, typed_string_expr, typed_tuple_expr,
    };
    use crate::plan::{
        BoolLocalId, Expr, FunctionExpr, FunctionFunctionId, FunctionType, FunctionValue,
        IntFunctionFunctionId, IntLocalId, NilFunctionId, NilLocalId, ParamLocal,
        RuntimeFunctionId, StringLocalId, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::dsl::{
        bool_, bool_function_ref, float, float_function_ref, function, function_function_ref, int,
        int_function_ref, let_tuple_step, local_bool, local_float, local_int, local_nil,
        local_string, local_tuple, module, nil, nil_function_ref, string, string_function_ref,
        tuple, tuple_function_ref,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span, expect_plan_error};
    use crate::planner::{
        InvalidExpressionShapeKind, InvalidExpressionType, InvalidTypedAstReason, PlanError,
        UnsupportedExpressionKind,
    };
    use gleam_core::ast::{Constant, TypedExpr};
    use gleam_core::type_::{self, ModuleValueConstructor};
    use num_bigint::BigInt;
    use std::collections::HashMap;

    #[test]
    fn reject_profile_expression_variants() {
        let cases = [
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
    fn reject_margin_plan_expr_error_propagates_through_typed_expression_helpers() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        let expected = PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        };

        assert_eq!(
            super::plan_int_expr(invalid_expr(type_::int()), &mut context),
            Err(expected.clone()),
        );
        assert_eq!(
            super::plan_string_expr(invalid_expr(type_::string()), &mut context),
            Err(expected.clone()),
        );
        assert_eq!(
            super::plan_float_expr(invalid_expr(type_::float()), &mut context),
            Err(expected.clone()),
        );
        assert_eq!(
            super::plan_bool_expr(invalid_expr(type_::bool()), &mut context),
            Err(expected),
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
        assert_eq!(
            invalid_expression_type_for_value(ValueType::Float, ValueType::Int),
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Float,
                    actual: InvalidExpressionType::Int,
                },
            },
        );
    }

    #[test]
    fn plan_tuple_index_result_families() {
        assert_tuple_index_plan(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let values = #(True, Nil, add_one)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [
                            ValueType::Bool,
                            ValueType::Nil,
                            ValueType::Function(Box::new(int_to_int_type())),
                        ],
                    )
                    .index_bool(0),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([
                        Expr::from(bool_(true)),
                        Expr::from(nil()),
                        Expr::from(int_function_ref(0, [ParamLocal::int(IntLocalId(0))])),
                    ]),
                )),
                [
                    function("add_one", local_int(0, "value").add_int(int(1)))
                        .param_int(0, "value"),
                ],
            ),
        );

        assert_tuple_index_plan(
            r#"
pub fn main() {
  let values = #("ok")
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(0, "values", [ValueType::String]).index_string(0),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(string("ok"))]),
                )),
                [],
            ),
        );

        assert_tuple_index_plan(
            r#"
pub fn main() {
  let values = #(1.5)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(0, "values", [ValueType::Float]).index_float(0),
                )
                .step(let_tuple_step(0, "values", tuple([Expr::from(float(1.5))]))),
                [],
            ),
        );

        assert_tuple_index_plan(
            r#"
pub fn main() {
  let values = #(#(1))
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(0, "values", [ValueType::Tuple(vec![ValueType::Int])])
                        .index_tuple(0, [ValueType::Int]),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(tuple([Expr::from(int(1))]))]),
                )),
                [],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let values = #(add_one)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [ValueType::Function(Box::new(int_to_int_type()))],
                    )
                    .index_int_function(0, [ValueType::Int]),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(int_function_ref(
                        0,
                        [ParamLocal::int(IntLocalId(0))],
                    ))]),
                )),
                [
                    function("add_one", local_int(0, "value").add_int(int(1)))
                        .param_int(0, "value"),
                ],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn text(value: String) {
  value
}

pub fn main() {
  let values = #(text)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [ValueType::Function(Box::new(string_to_string_type()))],
                    )
                    .index_string_function(0, [ValueType::String]),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(string_function_ref(
                        0,
                        [ParamLocal::string(StringLocalId(0))],
                    ))]),
                )),
                [function("text", local_string(0, "value")).param_string(0, "value")],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn number(value: Float) {
  value
}

pub fn main() {
  let values = #(number)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [ValueType::Function(Box::new(float_to_float_type()))],
                    )
                    .index_float_function(0, [ValueType::Float]),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(float_function_ref(
                        0,
                        [ParamLocal::float(crate::plan::FloatLocalId(0))],
                    ))]),
                )),
                [function("number", local_float(0, "value")).param_float(0, "value")],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn flag(value: Bool) {
  value
}

pub fn main() {
  let values = #(flag)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [ValueType::Function(Box::new(bool_to_bool_type()))],
                    )
                    .index_bool_function(0, [ValueType::Bool]),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(bool_function_ref(
                        0,
                        [ParamLocal::bool(BoolLocalId(0))],
                    ))]),
                )),
                [function("flag", local_bool(0, "value")).param_bool(0, "value")],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn unit(value: Nil) {
  value
}

pub fn main() {
  let values = #(unit)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [ValueType::Function(Box::new(nil_to_nil_type()))],
                    )
                    .index_nil_function(0, [ValueType::Nil]),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(nil_function_ref(
                        0,
                        [ParamLocal::nil(NilLocalId(0))],
                    ))]),
                )),
                [function("unit", local_nil(0, "value")).param_nil(0, "value")],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn tuple(value: Int) {
  #(value)
}

pub fn main() {
  let values = #(tuple)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(
                        0,
                        "values",
                        [ValueType::Function(Box::new(int_to_tuple_type()))],
                    )
                    .index_tuple_function(
                        0,
                        [ValueType::Int],
                        [ValueType::Int],
                    ),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(tuple_function_ref(
                        0,
                        [ParamLocal::int(IntLocalId(0))],
                        [ValueType::Int],
                    ))]),
                )),
                [
                    function("tuple", tuple([Expr::from(local_int(0, "value"))]))
                        .param_int(0, "value"),
                ],
            ),
        );

        assert_tuple_index_plan(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn get() {
  add_one
}

pub fn main() {
  let values = #(get)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(0, "values", [ValueType::Function(Box::new(getter_type()))])
                        .index_function_function(0, Vec::<ValueType>::new(), int_to_int_type()),
                )
                .step(let_tuple_step(
                    0,
                    "values",
                    tuple([Expr::from(function_function_ref(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::<ParamLocal>::new(),
                        int_to_int_type(),
                    ))]),
                )),
                [
                    function("add_one", local_int(0, "value").add_int(int(1)))
                        .param_int(0, "value"),
                    function("get", int_function_ref(0, [ParamLocal::int(IntLocalId(0))])),
                ],
            ),
        );

        assert_tuple_index_plan(
            r#"
pub fn main() {
  let values = #(Nil)
  values.0
}
"#,
            module(
                "main",
                function(
                    "main",
                    local_tuple(0, "values", [ValueType::Nil]).index_nil(0),
                )
                .step(let_tuple_step(0, "values", tuple([Expr::from(nil())]))),
                [],
            ),
        );
    }

    fn assert_tuple_index_plan(src: &str, expected: crate::plan::ExecutionPlan) {
        let actual = plan_module(compile(src)).expect("source should plan");

        assert_eq!(actual, expected);
    }

    fn int_to_int_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn string_to_string_type() -> FunctionType {
        FunctionType::new(vec![ValueType::String], ValueType::String)
    }

    fn float_to_float_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Float], ValueType::Float)
    }

    fn bool_to_bool_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Bool], ValueType::Bool)
    }

    fn nil_to_nil_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Nil], ValueType::Nil)
    }

    fn int_to_tuple_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Tuple(vec![ValueType::Int]))
    }

    fn getter_type() -> FunctionType {
        FunctionType::new(Vec::new(), ValueType::Function(Box::new(int_to_int_type())))
    }

    #[test]
    fn reject_margin_tuple_expression_shapes() {
        let tuple_int = type_::tuple(vec![type_::int()]);
        let cases = [
            (
                TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::int(),
                    elements: vec![typed_int_expr(1)],
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::tuple(vec![type_::list(type_::int())]),
                    elements: vec![typed_int_expr(1)],
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual: InvalidExpressionType::Unsupported,
                    },
                },
            ),
            (
                TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::tuple(vec![type_::string()]),
                    elements: vec![typed_int_expr(1)],
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual: InvalidExpressionType::Tuple,
                    },
                },
            ),
            (
                TypedExpr::Tuple {
                    location: dummy_span(),
                    type_: type_::tuple(vec![type_::int()]),
                    elements: vec![invalid_expr(type_::int())],
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                },
            ),
            (
                TypedExpr::TupleIndex {
                    location: dummy_span(),
                    type_: type_::int(),
                    index: 0,
                    tuple: Box::new(typed_int_expr(1)),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Tuple,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::TupleIndex {
                    location: dummy_span(),
                    type_: type_::list(type_::int()),
                    index: 0,
                    tuple: Box::new(typed_tuple_expr(tuple_int.clone(), vec![typed_int_expr(1)])),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Unsupported,
                        actual: InvalidExpressionType::Tuple,
                    },
                },
            ),
            (
                TypedExpr::TupleIndex {
                    location: dummy_span(),
                    type_: type_::int(),
                    index: 1,
                    tuple: Box::new(typed_tuple_expr(tuple_int.clone(), vec![typed_int_expr(1)])),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::Int,
                        actual: InvalidExpressionType::Tuple,
                    },
                },
            ),
            (
                TypedExpr::TupleIndex {
                    location: dummy_span(),
                    type_: type_::string(),
                    index: 0,
                    tuple: Box::new(typed_tuple_expr(tuple_int, vec![typed_int_expr(1)])),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::String,
                        actual: InvalidExpressionType::Int,
                    },
                },
            ),
            (
                TypedExpr::TupleIndex {
                    location: dummy_span(),
                    type_: type_::int(),
                    index: 0,
                    tuple: Box::new(invalid_expr(type_::tuple(vec![type_::int()]))),
                },
                PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                },
            ),
        ];

        for (expression, expected) in cases {
            assert_eq!(
                plan_module(module_returning_typed_expr(expression)),
                Err(expected)
            );
        }
    }

    #[test]
    fn reject_margin_float_expression_type_direct() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);

        assert_eq!(
            super::plan_float_expr(typed_string_expr("not float"), &mut context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Float,
                    actual: InvalidExpressionType::String,
                },
            }),
        );
    }

    fn invalid_expr(type_: std::sync::Arc<type_::Type>) -> TypedExpr {
        TypedExpr::Invalid {
            location: dummy_span(),
            type_,
            extra_information: None,
        }
    }
}
