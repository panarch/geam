use crate::plan::{BoolExpr, Expr, ExprKind, FunctionExpr, IntExpr, NilExpr, Step, StringExpr};
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
        let planned = plan_non_empty_steps_and_return(statements, context)?;

        Ok(block_expr(planned.steps, planned.return_))
    })
}

pub(super) fn block_expr(steps: Vec<Step>, return_: Expr) -> Expr {
    match return_.into_kind() {
        ExprKind::Int(return_) => Expr::int(IntExpr::block(steps, return_)),
        ExprKind::String(return_) => Expr::string(StringExpr::block(steps, return_)),
        ExprKind::Bool(return_) => Expr::bool(BoolExpr::block(steps, return_)),
        ExprKind::Nil(return_) => Expr::nil(NilExpr::block(steps, return_)),
        ExprKind::Function(return_) => Expr::function(FunctionExpr::block(steps, return_)),
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{IntFunctionId, LocalId, RuntimeFunctionId};
    use crate::planner::dsl::{
        block_bool, block_function, block_int, block_nil, block_string, bool_, evaluate_step,
        function, function_ref, int, let_int_step, let_nil_step, local_int, local_nil, module, nil,
        string,
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

pub fn nil_main() {
  { Nil }
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", block_string([], string("geam"))),
            [
                function("bool_main", block_bool([], bool_(true))),
                function("nil_main", block_nil([], nil())),
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

pub fn main() {
  { identity }
  1
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1)).evaluate(block_function(
                [],
                function_ref(
                    RuntimeFunctionId::Int(IntFunctionId(1)),
                    [LocalId::Int(crate::plan::IntLocalId(0))],
                ),
            )),
            [function("identity", local_int(0, "value")).param_int(0, "value")],
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
            function("main", block_int([evaluate_step(int(1))], int(2))),
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
                block_int(
                    [let_int_step(0, "x", int(1))],
                    block_int(
                        [let_int_step(1, "y", int(2))],
                        local_int(0, "x").add_int(local_int(1, "y")),
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
    [1]
    1
  }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::List,
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
                block_nil([let_nil_step(0, "x", nil())], local_nil(0, "x")),
            ),
            [],
        );

        assert_eq!(actual, expected);
    }
}
