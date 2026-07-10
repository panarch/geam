use super::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, eval_string_expr,
    project_tuple_list_expr,
};
use crate::plan::execution::ExecutionPlan;
use crate::plan::{TupleExpr, TupleExprKind, Value, ValueType};
use crate::runtime::ExecutionError;
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_tuple_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &TupleExpr,
) -> Result<Vec<Value>, ExecutionError> {
    match expression.kind() {
        TupleExprKind::Value(elements) => {
            let mut values = Vec::with_capacity(elements.len());
            for element in elements {
                values.push(super::eval_expr(plan, frame, element)?);
            }
            Ok(values)
        }
        TupleExprKind::LocalGet { local, .. } => Ok(frame.get_tuple(*local)),
        TupleExprKind::Call { function, args } => {
            function::run_tuple_call(plan, *function, args, frame)
        }
        TupleExprKind::FunctionCall { function, args } => {
            function::run_tuple_function_call(plan, function, args, frame)
        }
        TupleExprKind::TupleIndex { tuple, index } => {
            let expected = ValueType::Tuple(expression.type_().to_vec());
            match project_tuple_expr(plan, frame, tuple, *index, expected.clone())? {
                Value::Tuple(values) => Ok(values),
                other => Err(ExecutionError::tuple_index_family_mismatch(
                    expected,
                    other.value_type(),
                )),
            }
        }
        TupleExprKind::ListIndex { list, index } => {
            project_tuple_list_expr(plan, frame, list, *index, expression.type_())
        }
        TupleExprKind::Panic(panic) => eval_panic_expr(plan, frame, panic),
        TupleExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_tuple_expr(plan, frame, true_)
            } else {
                eval_tuple_expr(plan, frame, false_)
            }
        }
        TupleExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_tuple_expr(plan, frame, branch);
                }
            }
            eval_tuple_expr(plan, frame, fallback)
        }
        TupleExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_tuple_expr(plan, frame, branch);
                }
            }
            eval_tuple_expr(plan, frame, fallback)
        }
        TupleExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_tuple_expr(plan, frame, branch);
                }
            }
            eval_tuple_expr(plan, frame, fallback)
        }
        TupleExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_tuple_expr(plan, frame, return_)
        }
    }
}

pub(in crate::runtime) fn project_tuple_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    tuple: &TupleExpr,
    index: usize,
    expected: ValueType,
) -> Result<Value, ExecutionError> {
    let values = eval_tuple_expr(plan, frame, tuple)?;
    let Some(value) = values.get(index).cloned() else {
        return Err(ExecutionError::tuple_index_family_mismatch(
            expected,
            ValueType::Tuple(values.iter().map(Value::value_type).collect()),
        ));
    };
    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::{eval_tuple_expr, project_tuple_expr};
    use crate::plan::execution::ExecutionPlan;
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FunctionId, FunctionPlan, IntExpr, IntFunctionId, ListExpr,
        PanicExpr, PanicSite, ReturnExpr, Step, StringExpr, TupleExpr, TupleFunctionId, Value,
        ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, PanicKind};
    use crate::runtime::{int, run_src};

    #[test]
    fn eval_tuple_panic_returns_error() {
        let plan = crate::runtime::plan_src("pub fn main() { #(1) }");
        let mut frame = Frame::default();

        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::panic(
                    PanicExpr::panic_at(None, PanicSite::unknown()),
                    vec![ValueType::Int],
                ),
            ),
            Err(ExecutionError::source_panic(
                None,
                PanicKind::Panic,
                None,
                PanicSite::unknown()
            )),
        );
    }

    #[test]
    fn eval_tuple_expr_source_paths() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  #(1, "one")
}
"#,
            ),
            tuple(vec![int(1), string("one")]),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let pair = #(1, "one")
  pair
}
"#,
            ),
            tuple(vec![int(1), string("one")]),
        );

        assert_eq!(
            run_src(
                r#"
fn pair() {
  #(1, "one")
}

pub fn main() {
  pair()
}
"#,
            ),
            tuple(vec![int(1), string("one")]),
        );

        assert_eq!(
            run_src(
                r#"
fn pair() {
  #(1, "one")
}

pub fn main() {
  let f = pair
  f()
}
"#,
            ),
            tuple(vec![int(1), string("one")]),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  let nested = #(#(1, "one"), 2)
  nested.0
}
"#,
            ),
            tuple(vec![int(1), string("one")]),
        );
    }

    #[test]
    fn eval_tuple_expr_case_and_block_paths() {
        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case True {
    True -> #(1, "hit")
    False -> #(2, "miss")
  }
}
"#,
            ),
            tuple(vec![int(1), string("hit")]),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case 2 {
    1 -> #(1, "hit")
    _ -> #(2, "miss")
  }
}
"#,
            ),
            tuple(vec![int(2), string("miss")]),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case "hit" {
    "hit" -> #(1, "hit")
    _ -> #(2, "miss")
  }
}
"#,
            ),
            tuple(vec![int(1), string("hit")]),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  case 1.0 {
    2.0 -> #(1, "hit")
    _ -> #(2, "miss")
  }
}
"#,
            ),
            tuple(vec![int(2), string("miss")]),
        );

        assert_eq!(
            run_src(
                r#"
pub fn main() {
  {
    let value = 1
    #(value, "block")
  }
}
"#,
            ),
            tuple(vec![int(1), string("block")]),
        );
    }

    #[test]
    fn eval_tuple_expr_direct_case_and_block_paths() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::call(TupleFunctionId(0), Vec::new(), tuple_type()),
            ),
            Ok(vec![int(1), string("hit")]),
        );
        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::bool_case(BoolExpr::value(true), tuple_expr(1, "hit"), other_tuple(),),
            ),
            Ok(vec![int(1), string("hit")]),
        );
        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::bool_case(BoolExpr::value(false), other_tuple(), tuple_expr(1, "hit"),),
            ),
            Ok(vec![int(1), string("hit")]),
        );
        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), tuple_expr(1, "hit"))],
                    other_tuple(),
                ),
            ),
            Ok(vec![int(1), string("hit")]),
        );
        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::int_case(
                    IntExpr::value(2.into()),
                    vec![(1.into(), other_tuple())],
                    tuple_expr(1, "hit"),
                ),
            ),
            Ok(vec![int(1), string("hit")]),
        );
        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::string_case(
                    StringExpr::value("hit".into()),
                    vec![("hit".into(), tuple_expr(1, "hit"))],
                    other_tuple(),
                ),
            ),
            Ok(vec![int(1), string("hit")]),
        );
        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::string_case(
                    StringExpr::value("miss".into()),
                    vec![("hit".into(), other_tuple())],
                    tuple_expr(1, "hit"),
                ),
            ),
            Ok(vec![int(1), string("hit")]),
        );
        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, tuple_expr(1, "hit"))],
                    other_tuple(),
                ),
            ),
            Ok(vec![int(1), string("hit")]),
        );
        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::float_case(
                    FloatExpr::value(2.0),
                    vec![(1.0, other_tuple())],
                    tuple_expr(1, "hit"),
                ),
            ),
            Ok(vec![int(1), string("hit")]),
        );
        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::value(0.into())))],
                    tuple_expr(1, "hit"),
                ),
            ),
            Ok(vec![int(1), string("hit")]),
        );
    }

    #[test]
    fn tuple_projection_invariant_errors() {
        let plan = plan();
        let mut frame = Frame::default();
        let tuple_expr = TupleExpr::value(
            vec![Expr::int(IntExpr::value(1.into()))],
            vec![ValueType::Int],
        );

        assert_eq!(
            project_tuple_expr(&plan, &mut frame, &tuple_expr, 1, ValueType::String),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::String,
                ValueType::Tuple(vec![ValueType::Int]),
            )),
        );

        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::tuple_index(tuple_expr, 0, vec![ValueType::String]),
            ),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Tuple(vec![ValueType::String]),
                ValueType::Int,
            )),
        );
    }

    #[test]
    fn list_projection_invariant_errors() {
        let plan = plan();
        let mut frame = Frame::default();
        let tuple_type = vec![ValueType::Int, ValueType::String];
        let list = ListExpr::value(
            vec![Expr::tuple(TupleExpr::value(
                vec![
                    Expr::int(IntExpr::value(1.into())),
                    Expr::string(StringExpr::value("one".into())),
                ],
                tuple_type.clone(),
            ))],
            ValueType::Tuple(tuple_type.clone()),
        );

        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::list_index(list, 0, tuple_type.clone()),
            ),
            Ok(vec![Value::Int(1.into()), Value::String("one".into())]),
        );

        let list = ListExpr::value(
            vec![Expr::tuple(TupleExpr::value(
                vec![
                    Expr::int(IntExpr::value(1.into())),
                    Expr::string(StringExpr::value("one".into())),
                ],
                tuple_type.clone(),
            ))],
            ValueType::Tuple(tuple_type.clone()),
        );
        assert_eq!(
            eval_tuple_expr(
                &plan,
                &mut frame,
                &TupleExpr::list_index(list, 1, tuple_type.clone()),
            ),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Tuple(tuple_type.clone()),
                1,
                1,
            )),
        );
    }

    #[test]
    fn eval_tuple_expr_propagates_operand_errors() {
        let plan = plan();
        let mut frame = Frame::default();

        for (expression, expected) in [
            (
                TupleExpr::value(vec![Expr::int(error_int_expr())], vec![ValueType::Int]),
                ValueType::Int,
            ),
            (
                TupleExpr::bool_case(error_bool_expr(), tuple_expr(1, "hit"), other_tuple()),
                ValueType::Bool,
            ),
            (
                TupleExpr::int_case(
                    error_int_expr(),
                    vec![(1.into(), tuple_expr(1, "hit"))],
                    other_tuple(),
                ),
                ValueType::Int,
            ),
            (
                TupleExpr::string_case(
                    error_string_expr(),
                    vec![("hit".into(), tuple_expr(1, "hit"))],
                    other_tuple(),
                ),
                ValueType::String,
            ),
            (
                TupleExpr::float_case(
                    error_float_expr(),
                    vec![(1.0, tuple_expr(1, "hit"))],
                    other_tuple(),
                ),
                ValueType::Float,
            ),
            (
                TupleExpr::block(
                    vec![Step::evaluate(Expr::bool(error_bool_expr()))],
                    tuple_expr(1, "hit"),
                ),
                ValueType::Bool,
            ),
        ] {
            assert_eq!(
                eval_tuple_expr(&plan, &mut frame, &expression),
                Err(tuple_index_error(expected)),
            );
        }

        assert_eq!(
            eval_tuple_expr(&plan, &mut frame, &error_tuple_expr()),
            Err(tuple_index_error(ValueType::Tuple(tuple_type()))),
        );
        assert_eq!(
            project_tuple_expr(&plan, &mut frame, &error_tuple_expr(), 0, ValueType::Int),
            Err(tuple_index_error(ValueType::Tuple(tuple_type()))),
        );
    }

    fn error_int_expr() -> IntExpr {
        IntExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_bool_expr() -> BoolExpr {
        BoolExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_string_expr() -> StringExpr {
        StringExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_float_expr() -> FloatExpr {
        FloatExpr::tuple_index(empty_tuple(), 0)
    }

    fn error_tuple_expr() -> TupleExpr {
        TupleExpr::tuple_index(empty_tuple(), 0, tuple_type())
    }

    fn empty_tuple() -> TupleExpr {
        TupleExpr::value(Vec::new(), Vec::new())
    }

    fn tuple_index_error(expected: ValueType) -> ExecutionError {
        ExecutionError::tuple_index_family_mismatch(expected, ValueType::Tuple(Vec::new()))
    }

    fn tuple(values: Vec<Value>) -> Value {
        Value::Tuple(values)
    }

    fn string(value: &str) -> Value {
        Value::String(value.into())
    }

    fn tuple_expr(int_value: i64, string_value: &str) -> TupleExpr {
        TupleExpr::value(
            vec![
                Expr::int(IntExpr::value(int_value.into())),
                Expr::string(StringExpr::value(string_value.into())),
            ],
            tuple_type(),
        )
    }

    fn other_tuple() -> TupleExpr {
        tuple_expr(2, "miss")
    }

    fn tuple_type() -> Vec<ValueType> {
        vec![ValueType::Int, ValueType::String]
    }

    fn plan() -> ExecutionPlan {
        ExecutionPlan::from_module_plan(crate::plan::ModulePlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into())),
            ),
            vec![FunctionPlan::new(
                FunctionId::new(1),
                "tuple_value".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::tuple(TupleFunctionId(0), tuple_expr(1, "hit")),
            )],
        ))
    }
}
