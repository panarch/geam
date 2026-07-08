use crate::plan::{
    BoolExpr, BoolFunctionExpr, Expr, ExprKind, FloatExpr, FloatFunctionExpr, FunctionExpr,
    FunctionExprKind, FunctionFunctionExpr, IntExpr, IntFunctionExpr, ListExpr, ListFunctionExpr,
    NilExpr, NilFunctionExpr, Step, StringExpr, StringFunctionExpr, TupleExpr, TupleFunctionExpr,
    ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use crate::planner::statement::plan_non_empty_steps_and_return;
use gleam_core::ast::TypedStatement;
use vec1::Vec1;

pub(super) fn plan(
    statements: Vec1<TypedStatement>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    context.with_local_scope(|context| {
        let planned = plan_non_empty_steps_and_return(statements, context, None)?;
        Ok(block_expr(planned.steps, planned.return_))
    })
}

pub(super) fn plan_with_expected_source_stop_type(
    statements: Vec1<TypedStatement>,
    expected: &ValueType,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    context.with_local_scope(|context| {
        let planned = plan_non_empty_steps_and_return(statements, context, Some(expected))?;
        Ok(block_expr(planned.steps, planned.return_))
    })
}

pub(super) fn block_expr(steps: Vec<Step>, return_: Expr) -> Expr {
    match return_.into_kind() {
        ExprKind::Int(return_) => Expr::int(IntExpr::block(steps, return_)),
        ExprKind::String(return_) => Expr::string(StringExpr::block(steps, return_)),
        ExprKind::Float(return_) => Expr::float(FloatExpr::block(steps, return_)),
        ExprKind::Bool(return_) => Expr::bool(BoolExpr::block(steps, return_)),
        ExprKind::Nil(return_) => Expr::nil(NilExpr::block(steps, return_)),
        ExprKind::Tuple(return_) => Expr::tuple(TupleExpr::block(steps, return_)),
        ExprKind::List(return_) => Expr::list(ListExpr::block(steps, return_)),
        ExprKind::Function(return_) => match return_.into_kind() {
            FunctionExprKind::Int(return_) => {
                Expr::function(FunctionExpr::int(IntFunctionExpr::block(steps, return_)))
            }
            FunctionExprKind::String(return_) => Expr::function(FunctionExpr::string(
                StringFunctionExpr::block(steps, return_),
            )),
            FunctionExprKind::Float(return_) => Expr::function(FunctionExpr::float(
                FloatFunctionExpr::block(steps, return_),
            )),
            FunctionExprKind::Bool(return_) => {
                Expr::function(FunctionExpr::bool(BoolFunctionExpr::block(steps, return_)))
            }
            FunctionExprKind::Nil(return_) => {
                Expr::function(FunctionExpr::nil(NilFunctionExpr::block(steps, return_)))
            }
            FunctionExprKind::Tuple(return_) => Expr::function(FunctionExpr::tuple(
                TupleFunctionExpr::block(steps, return_),
            )),
            FunctionExprKind::List(return_) => {
                Expr::function(FunctionExpr::list(ListFunctionExpr::block(steps, return_)))
            }
            FunctionExprKind::Function(return_) => Expr::function(FunctionExpr::function(
                FunctionFunctionExpr::block(steps, return_),
            )),
        },
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        FunctionFunctionId, FunctionType, IntFunctionFunctionId, IntFunctionId, LocalId,
        RuntimeFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        block_function, block_int, bool_, bool_return_block, bool_return_expr, evaluate_step,
        float, float_return_block, float_return_expr, function, function_ref, int,
        int_return_block, int_return_expr, let_int_step, let_nil_step, list, list_return_block,
        list_return_expr, local_bool, local_float, local_int, local_nil, local_string, module, nil,
        nil_return_block, nil_return_expr, return_list, string, string_return_block,
        string_return_expr,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, expect_plan_error};
    use crate::planner::{PlanError, UnsupportedExpressionKind};

    #[test]
    fn plan_block_return_values() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  { "geam" }
}

pub fn bool_main() {
  { True }
}

pub fn float_main() {
  { 1.5 }
}

pub fn nil_main() {
  { Nil }
}

pub fn list_main() {
  { [1] }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                string_return_block([], string_return_expr(string("geam"))),
            ),
            [
                function(
                    "bool_main",
                    bool_return_block([], bool_return_expr(bool_(true))),
                ),
                function(
                    "float_main",
                    float_return_block([], float_return_expr(float(1.5))),
                ),
                function("nil_main", nil_return_block([], nil_return_expr(nil()))),
                function(
                    "list_main",
                    return_list(
                        ValueType::Int,
                        list_return_block([], list_return_expr(list([int(1)], ValueType::Int))),
                    ),
                ),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_valued_block_expression_statement() {
        let actual = plan_module(compile(
            r#"
fn identity(value: Int) {
  value
}

fn string_identity(value: String) {
  value
}

fn float_identity(value: Float) {
  value
}

fn bool_identity(value: Bool) {
  value
}

fn nil_identity(value: Nil) {
  value
}

fn get_identity() {
  identity
}

fn values(value: Int) {
  [value]
}

pub fn main() {
  { identity }
  { string_identity }
  { float_identity }
  { bool_identity }
  { nil_identity }
  { get_identity }
  { values }
  1
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1))
                .evaluate(block_function(
                    vec![],
                    function_ref(
                        RuntimeFunctionId::Int(IntFunctionId(1)),
                        [LocalId::Int(crate::plan::IntLocalId(0))],
                    ),
                ))
                .evaluate(block_function(
                    vec![],
                    function_ref(
                        RuntimeFunctionId::String(crate::plan::StringFunctionId(0)),
                        [LocalId::String(crate::plan::StringLocalId(0))],
                    ),
                ))
                .evaluate(block_function(
                    vec![],
                    function_ref(
                        RuntimeFunctionId::Float(crate::plan::FloatFunctionId(0)),
                        [LocalId::Float(crate::plan::FloatLocalId(0))],
                    ),
                ))
                .evaluate(block_function(
                    vec![],
                    function_ref(
                        RuntimeFunctionId::Bool(crate::plan::BoolFunctionId(0)),
                        [LocalId::Bool(crate::plan::BoolLocalId(0))],
                    ),
                ))
                .evaluate(block_function(
                    vec![],
                    function_ref(
                        RuntimeFunctionId::Nil(crate::plan::NilFunctionId(0)),
                        [LocalId::Nil(crate::plan::NilLocalId(0))],
                    ),
                ))
                .evaluate(block_function(
                    vec![],
                    function_ref(
                        RuntimeFunctionId::Function {
                            id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                            return_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                        },
                        Vec::<LocalId>::new(),
                    ),
                ))
                .evaluate(block_function(
                    vec![],
                    function_ref(
                        RuntimeFunctionId::List(crate::plan::ListFunctionId::from_item_type(
                            0,
                            crate::plan::ValueType::Int,
                        )),
                        [LocalId::Int(crate::plan::IntLocalId(0))],
                    ),
                )),
            [
                function("identity", local_int(0, "value")).param_int(0, "value"),
                function("string_identity", local_string(0, "value")).param_string(0, "value"),
                function("float_identity", local_float(0, "value")).param_float(0, "value"),
                function("bool_identity", local_bool(0, "value")).param_bool(0, "value"),
                function("nil_identity", local_nil(0, "value")).param_nil(0, "value"),
                function(
                    "get_identity",
                    function_ref(
                        RuntimeFunctionId::Int(IntFunctionId(1)),
                        [LocalId::Int(crate::plan::IntLocalId(0))],
                    ),
                ),
                function("values", list([local_int(0, "value")], ValueType::Int))
                    .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_block_expression_statement_steps() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  {
    1
    2
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block([evaluate_step(int(1))], int_return_expr(int(2))),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_block_let_binding() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  {
    let x = 1
    x + 2
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "x", int(1))],
                    int_return_expr(local_int(0, "x").add_int(int(2))),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_block_result_binding() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let y = {
    let x = 1
    x + 2
  }
  y
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(1, "y")).let_int(
                1,
                "y",
                block_int(
                    [let_int_step(0, "x", int(1))],
                    local_int(0, "x").add_int(int(2)),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_nested_block_scope() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  {
    let x = 1
    {
      let y = 2
      x + y
    }
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                int_return_block(
                    [let_int_step(0, "x", int(1))],
                    int_return_block(
                        [let_int_step(1, "y", int(2))],
                        int_return_expr(local_int(0, "x").add_int(local_int(1, "y"))),
                    ),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_block_shadowing_does_not_leak() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let x = 1
  {
    let x = 2
    x + 1
  }
  x
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", local_int(0, "x"))
                .let_int(0, "x", int(1))
                .evaluate(block_int(
                    [let_int_step(1, "x", int(2))],
                    local_int(1, "x").add_int(int(1)),
                )),
            [],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_unsupported_expression_inside_block() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  {
    echo 1
    1
  }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            },
        );
    }

    #[test]
    fn reject_profile_unsupported_expression_inside_function_valued_block() {
        assert_eq!(
            expect_plan_error(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  {
    echo 1
    add_one
  }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::Echo,
            },
        );
    }

    #[test]
    fn plan_block_nil_let_binding() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  {
    let x = Nil
    x
  }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function(
                "main",
                nil_return_block(
                    [let_nil_step(0, "x", nil())],
                    nil_return_expr(local_nil(0, "x")),
                ),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }
}
