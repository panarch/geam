use super::{
    eval_bool_expr, eval_int_expr, eval_panic_expr, eval_string_expr, project_float_list_expr,
    project_tuple_expr,
};
use crate::plan::execution::ExecutionPlan;
use crate::plan::{FloatExpr, FloatExprKind, Value, ValueType};
use crate::runtime::ExecutionError;
use crate::runtime::frame::Frame;
use crate::runtime::function;

pub(in crate::runtime) fn eval_float_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &FloatExpr,
) -> Result<f64, ExecutionError> {
    match expression.kind() {
        FloatExprKind::Value(value) => Ok(*value),
        FloatExprKind::LocalGet { local, .. } => Ok(frame.get_float(*local)),
        FloatExprKind::Call { function, args } => {
            function::run_float_call(plan, *function, args, frame)
        }
        FloatExprKind::FunctionCall { function, args } => {
            function::run_float_function_call(plan, function, args, frame)
        }
        FloatExprKind::TupleIndex { tuple, index } => {
            match project_tuple_expr(plan, frame, tuple, *index, ValueType::Float)? {
                Value::Float(value) => Ok(value),
                other => Err(ExecutionError::tuple_index_family_mismatch(
                    ValueType::Float,
                    other.value_type(),
                )),
            }
        }
        FloatExprKind::ListIndex { list, index } => {
            project_float_list_expr(plan, frame, list, *index)
        }
        FloatExprKind::Panic(panic) => eval_panic_expr(plan, frame, panic),
        FloatExprKind::Add { left, right } => {
            Ok(eval_float_expr(plan, frame, left)? + eval_float_expr(plan, frame, right)?)
        }
        FloatExprKind::Sub { left, right } => {
            Ok(eval_float_expr(plan, frame, left)? - eval_float_expr(plan, frame, right)?)
        }
        FloatExprKind::Mult { left, right } => {
            Ok(eval_float_expr(plan, frame, left)? * eval_float_expr(plan, frame, right)?)
        }
        FloatExprKind::Div { left, right } => Ok(eval_div_float(
            eval_float_expr(plan, frame, left)?,
            eval_float_expr(plan, frame, right)?,
        )),
        FloatExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_float_expr(plan, frame, true_)
            } else {
                eval_float_expr(plan, frame, false_)
            }
        }
        FloatExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_float_expr(plan, frame, branch);
                }
            }
            eval_float_expr(plan, frame, fallback)
        }
        FloatExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_float_expr(plan, frame, branch);
                }
            }
            eval_float_expr(plan, frame, fallback)
        }
        FloatExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_float_expr(plan, frame, branch);
                }
            }
            eval_float_expr(plan, frame, fallback)
        }
        FloatExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_float_expr(plan, frame, return_)
        }
    }
}

fn eval_div_float(left: f64, right: f64) -> f64 {
    // Geam normalizes Gleam float division by zero instead of exposing raw Rust
    // f64 infinities or NaN.
    if right == 0.0 { 0.0 } else { left / right }
}

#[cfg(test)]
mod tests {
    use super::{eval_div_float, eval_float_expr};
    use crate::plan::execution::ExecutionPlan;
    use crate::plan::{
        BoolExpr, BoolFunctionExpr, Expr, FloatExpr, FloatFunctionExpr, FloatFunctionId,
        FloatLocalId, FrameLayout, FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionValue,
        FunctionId, FunctionPlan, FunctionReturnFamily, FunctionType, IntExpr, IntFunctionExpr,
        ListExpr, PanicExpr, PanicSite, ReturnExpr, Step, StringExpr, StringFunctionExpr,
        StringFunctionFunctionId, TupleExpr, ValueType,
    };
    use crate::runtime::frame::Frame;
    use crate::runtime::{ExecutionError, PanicKind};

    #[test]
    fn tuple_index_family_mismatch_returns_error() {
        let plan = crate::runtime::plan_src("pub fn main() { 1.0 }");
        let mut frame = Frame::default();
        let tuple = TupleExpr::value(
            vec![Expr::float(FloatExpr::value(1.5))],
            vec![ValueType::Float],
        );

        assert_eq!(
            eval_float_expr(&plan, &mut frame, &FloatExpr::tuple_index(tuple, 0)),
            Ok(1.5),
        );

        let tuple = TupleExpr::value(
            vec![Expr::string(StringExpr::value("one".into()))],
            vec![ValueType::String],
        );

        assert_eq!(
            eval_float_expr(&plan, &mut frame, &FloatExpr::tuple_index(tuple, 0)),
            Err(ExecutionError::tuple_index_family_mismatch(
                ValueType::Float,
                ValueType::String,
            )),
        );
    }

    #[test]
    fn list_projection_invariant_errors() {
        let plan = crate::runtime::plan_src("pub fn main() { 1.0 }");
        let mut frame = Frame::default();
        let list = ListExpr::value(vec![Expr::float(FloatExpr::value(1.5))], ValueType::Float);

        assert_eq!(
            eval_float_expr(&plan, &mut frame, &FloatExpr::list_index(list, 0)),
            Ok(1.5),
        );

        let list = ListExpr::value(vec![Expr::float(FloatExpr::value(1.5))], ValueType::Float);
        assert_eq!(
            eval_float_expr(&plan, &mut frame, &FloatExpr::list_index(list, 1)),
            Err(ExecutionError::list_index_out_of_bounds(
                ValueType::Float,
                1,
                1,
            )),
        );
    }

    #[test]
    fn eval_float_panic_returns_error() {
        let plan = crate::runtime::plan_src("pub fn main() { 1.0 }");
        let mut frame = Frame::default();

        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::panic(PanicExpr::panic_at(None, PanicSite::unknown())),
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
    fn eval_float_division_by_zero_returns_zero() {
        assert_eq!(eval_div_float(1.5, 0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(eval_div_float(1.5, -0.0).to_bits(), 0.0f64.to_bits());
        assert_eq!(eval_div_float(3.0, 2.0), 1.5);
    }

    #[test]
    fn eval_float_values_locals_calls_and_operators() {
        let plan = plan();
        let mut frame = Frame::new(FrameLayout::from_function_parts(
            &[],
            &[],
            &ReturnExpr::float(
                FloatFunctionId(9),
                FloatExpr::local_get(FloatLocalId(0), "value".into()),
            ),
        ));
        frame.set_float(FloatLocalId(0), 1.5);

        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::local_get(FloatLocalId(0), "value".into()),
            ),
            Ok(1.5),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::call(FloatFunctionId(0), Vec::new())
            ),
            Ok(3.5),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::add(FloatExpr::value(1.0), FloatExpr::value(2.0)),
            ),
            Ok(3.0),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::sub(FloatExpr::value(5.0), FloatExpr::value(2.0)),
            ),
            Ok(3.0),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::mult(FloatExpr::value(1.5), FloatExpr::value(2.0)),
            ),
            Ok(3.0),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::div(FloatExpr::value(6.0), FloatExpr::value(2.0)),
            ),
            Ok(3.0),
        );
    }

    #[test]
    fn eval_float_case_and_block_branches() {
        let plan = plan();
        let mut frame = Frame::default();

        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::bool_case(
                    BoolExpr::value(true),
                    FloatExpr::value(1.0),
                    FloatExpr::value(2.0),
                ),
            ),
            Ok(1.0),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::bool_case(
                    BoolExpr::value(false),
                    FloatExpr::value(1.0),
                    FloatExpr::value(2.0),
                ),
            ),
            Ok(2.0),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), FloatExpr::value(1.0))],
                    FloatExpr::value(0.0),
                ),
            ),
            Ok(1.0),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::int_case(
                    IntExpr::value(2.into()),
                    vec![(1.into(), FloatExpr::value(1.0))],
                    FloatExpr::value(0.0),
                ),
            ),
            Ok(0.0),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::string_case(
                    StringExpr::value("hit".into()),
                    vec![("hit".into(), FloatExpr::value(1.0))],
                    FloatExpr::value(0.0),
                ),
            ),
            Ok(1.0),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::string_case(
                    StringExpr::value("miss".into()),
                    vec![("hit".into(), FloatExpr::value(1.0))],
                    FloatExpr::value(0.0),
                ),
            ),
            Ok(0.0),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, FloatExpr::value(1.0))],
                    FloatExpr::value(0.0),
                ),
            ),
            Ok(1.0),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::float_case(
                    FloatExpr::value(2.0),
                    vec![(1.0, FloatExpr::value(1.0))],
                    FloatExpr::value(0.0),
                ),
            ),
            Ok(0.0),
        );
        assert_eq!(
            eval_float_expr(
                &plan,
                &mut frame,
                &FloatExpr::block(
                    vec![Step::evaluate(Expr::float(FloatExpr::value(1.0)))],
                    FloatExpr::value(2.0),
                ),
            ),
            Ok(2.0),
        );
    }

    #[test]
    fn eval_float_expr_propagates_operand_errors() {
        let execution_plan = plan();
        let mut frame = Frame::default();

        assert_float_error(FloatExpr::add(error_float_expr(), FloatExpr::value(1.0)));
        assert_float_error(FloatExpr::add(FloatExpr::value(1.0), error_float_expr()));
        assert_float_error(FloatExpr::sub(error_float_expr(), FloatExpr::value(1.0)));
        assert_float_error(FloatExpr::sub(FloatExpr::value(1.0), error_float_expr()));
        assert_float_error(FloatExpr::mult(error_float_expr(), FloatExpr::value(1.0)));
        assert_float_error(FloatExpr::mult(FloatExpr::value(1.0), error_float_expr()));
        assert_float_error(FloatExpr::div(error_float_expr(), FloatExpr::value(1.0)));
        assert_float_error(FloatExpr::div(FloatExpr::value(1.0), error_float_expr()));

        assert_eq!(
            eval_float_expr(
                &execution_plan,
                &mut frame,
                &FloatExpr::bool_case(
                    error_bool_expr(),
                    FloatExpr::value(1.0),
                    FloatExpr::value(0.0),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Bool,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_float_expr(
                &execution_plan,
                &mut frame,
                &FloatExpr::int_case(
                    error_int_expr(),
                    vec![(1.into(), FloatExpr::value(1.0))],
                    FloatExpr::value(0.0),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::Int,
                FunctionReturnFamily::String,
            )),
        );
        assert_eq!(
            eval_float_expr(
                &execution_plan,
                &mut frame,
                &FloatExpr::string_case(
                    error_string_expr(),
                    vec![("hit".into(), FloatExpr::value(1.0))],
                    FloatExpr::value(0.0),
                ),
            ),
            Err(function_return_family_error_value(
                FunctionReturnFamily::String,
                FunctionReturnFamily::Int,
            )),
        );
        assert_float_error(FloatExpr::float_case(
            error_float_expr(),
            vec![(1.0, FloatExpr::value(1.0))],
            FloatExpr::value(0.0),
        ));
        assert_float_error(FloatExpr::block(
            vec![Step::evaluate(Expr::float(error_float_expr()))],
            FloatExpr::value(1.0),
        ));
        assert_float_error(FloatExpr::block(Vec::new(), error_float_expr()));

        fn assert_float_error(expression: FloatExpr) {
            let plan = plan();
            let mut frame = Frame::default();

            assert_eq!(
                eval_float_expr(&plan, &mut frame, &expression),
                Err(function_return_family_error_value(
                    FunctionReturnFamily::Float,
                    FunctionReturnFamily::String,
                )),
            );
        }
    }

    fn function_return_family_error_value(
        expected: FunctionReturnFamily,
        actual: FunctionReturnFamily,
    ) -> ExecutionError {
        ExecutionError::function_return_family_mismatch(expected, actual)
    }

    fn error_bool_expr() -> BoolExpr {
        BoolExpr::function_call(
            BoolFunctionExpr::function_call(
                function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Bool),
            ),
            Vec::new(),
        )
    }

    fn error_int_expr() -> IntExpr {
        IntExpr::function_call(
            IntFunctionExpr::function_call(
                function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Int),
            ),
            Vec::new(),
        )
    }

    fn error_string_expr() -> StringExpr {
        StringExpr::function_call(
            StringFunctionExpr::function_call(
                FunctionFunctionExpr::value(FunctionFunctionValue::new(
                    FunctionFunctionId::Int(crate::plan::IntFunctionFunctionId(0)),
                    Vec::new(),
                    FunctionType::new(Vec::new(), ValueType::Int),
                )),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::String),
            ),
            Vec::new(),
        )
    }

    fn error_float_expr() -> FloatExpr {
        FloatExpr::function_call(
            FloatFunctionExpr::function_call(
                function_function_expr(),
                Vec::new(),
                FunctionType::new(Vec::new(), ValueType::Float),
            ),
            Vec::new(),
        )
    }

    fn function_function_expr() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::String(StringFunctionFunctionId(0)),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::String),
        ))
    }

    fn plan() -> ExecutionPlan {
        ExecutionPlan::from_module_plan(crate::plan::ModulePlan::new(
            "main".into(),
            FunctionPlan::new(
                FunctionId::new(0),
                "main".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::int(crate::plan::IntFunctionId(0), IntExpr::value(0.into())),
            ),
            vec![FunctionPlan::new(
                FunctionId::new(1),
                "float_value".into(),
                Vec::new(),
                Vec::new(),
                ReturnExpr::float(FloatFunctionId(0), FloatExpr::value(3.5)),
            )],
        ))
    }
}
