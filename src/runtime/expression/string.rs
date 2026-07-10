use super::{
    eval_bool_expr, eval_float_expr, eval_int_expr, eval_panic_expr, project_string_list_expr,
    project_tuple_expr,
};
use crate::plan::ValueType;
use crate::plan::execution::ExecutionPlan;
use crate::plan::execution::{StringExpr, StringExprKind};
use crate::runtime::ExecutionError;
use crate::runtime::Value;
use crate::runtime::frame::Frame;
use crate::runtime::function;
use ecow::EcoString;

pub(in crate::runtime) fn eval_string_expr(
    plan: &ExecutionPlan,
    frame: &mut Frame,
    expression: &StringExpr,
) -> Result<EcoString, ExecutionError> {
    match expression.kind() {
        StringExprKind::Value(value) => Ok(value.clone()),
        StringExprKind::LocalGet { local, .. } => Ok(frame.get_string(*local)),
        StringExprKind::Call { function, args } => {
            function::run_string_call(plan, *function, args, frame)
        }
        StringExprKind::FunctionCall { function, args } => {
            function::run_string_function_call(plan, function, args, frame)
        }
        StringExprKind::TupleIndex { tuple, index } => {
            match project_tuple_expr(plan, frame, tuple, *index, ValueType::String)? {
                Value::String(value) => Ok(value),
                other => Err(ExecutionError::tuple_index_family_mismatch(
                    ValueType::String,
                    other.value_type(),
                )),
            }
        }
        StringExprKind::ListIndex { list, index } => {
            project_string_list_expr(plan, frame, list, *index)
        }
        StringExprKind::Panic(panic) => {
            eval_panic_expr(plan, frame, panic).map(|never| match never {})
        }
        StringExprKind::Concatenate { left, right } => Ok(format!(
            "{}{}",
            eval_string_expr(plan, frame, left)?,
            eval_string_expr(plan, frame, right)?,
        )
        .into()),
        StringExprKind::DropPrefix { value, prefix } => Ok(eval_string_expr(plan, frame, value)?
            .strip_prefix(prefix.as_str())
            .unwrap_or("")
            .into()),
        StringExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, frame, subject)? {
                eval_string_expr(plan, frame, true_)
            } else {
                eval_string_expr(plan, frame, false_)
            }
        }
        StringExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_expr(plan, frame, branch);
                }
            }
            eval_string_expr(plan, frame, fallback)
        }
        StringExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_expr(plan, frame, branch);
                }
            }
            eval_string_expr(plan, frame, fallback)
        }
        StringExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_string_expr(plan, frame, branch);
                }
            }
            eval_string_expr(plan, frame, fallback)
        }
        StringExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, steps, frame)?;
            eval_string_expr(plan, frame, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FunctionId, FunctionPlan, IntExpr, ModulePlan, PanicExpr,
        PanicSite, ReturnExpr, Step, StringExpr, StringFunctionId, TupleExpr, ValueType,
    };
    use crate::runtime::{ExecutionError, run_main};

    #[test]
    fn source_string_expression_variants_evaluate_exact_values() {
        let source = r#"
fn suffix(value: String) -> String { value <> "!" }

pub fn main() {
  let local = "local"
  let function = suffix
  #(
    local,
    suffix("call"),
    function("function"),
    #("tuple").0,
    case ["list"] { [value] -> value _ -> "missing" },
    "left" <> "right",
    case "prefix-rest" { "prefix-" <> rest -> rest _ -> "missing" },
    case True { True -> "true" False -> "false" },
    case False { True -> "true" False -> "false" },
    case 1 { 1 -> "one" _ -> "other" },
    case 2 { 1 -> "one" _ -> "other" },
    case "one" { "one" -> "match" _ -> "other" },
    case "two" { "one" -> "match" _ -> "other" },
    case 1.0 { 1.0 -> "match" _ -> "other" },
    case 2.0 { 1.0 -> "match" _ -> "other" },
    { let _ = 0 "block" },
  )
}
"#;

        assert_eq!(
            crate::runtime::run_src(source),
            crate::runtime::Value::Tuple(
                [
                    "local",
                    "call!",
                    "function!",
                    "tuple",
                    "list",
                    "leftright",
                    "rest",
                    "true",
                    "false",
                    "one",
                    "other",
                    "match",
                    "other",
                    "match",
                    "other",
                    "block",
                ]
                .into_iter()
                .map(|value| crate::runtime::Value::String(value.into()))
                .collect(),
            ),
        );
    }

    #[test]
    fn source_operand_errors_propagate_through_string_expressions() {
        let expressions = [
            "fail_string() <> \"suffix\"",
            "\"prefix\" <> fail_string()",
            "case fail_bool() { True -> \"true\" False -> \"false\" }",
            "case fail_int() { 0 -> \"zero\" _ -> \"other\" }",
            "case fail_string() { \"zero\" -> \"zero\" _ -> \"other\" }",
            "case fail_float() { 0.0 -> \"zero\" _ -> \"other\" }",
            "{ let _ = fail_bool() \"value\" }",
            "{ let function = fail_string function() }",
        ];

        for expression in expressions {
            let source = format!(
                r#"
fn fail_bool() -> Bool {{ panic }}
fn fail_int() -> Int {{ panic }}
fn fail_string() -> String {{ panic }}
fn fail_float() -> Float {{ panic }}
pub fn main() -> String {{ {expression} }}
"#,
            );

            assert_eq!(
                crate::runtime::run_src_error(&source).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    #[test]
    fn module_expression_errors_propagate_through_string_wrappers() {
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let expressions = [
            StringExpr::tuple_index(TupleExpr::panic(panic(), vec![ValueType::String]), 0),
            StringExpr::drop_prefix(StringExpr::panic(panic()), "prefix".into()),
            StringExpr::bool_case(
                BoolExpr::panic(panic()),
                StringExpr::value("true".into()),
                StringExpr::value("false".into()),
            ),
            StringExpr::int_case(
                IntExpr::panic(panic()),
                Vec::new(),
                StringExpr::value("fallback".into()),
            ),
            StringExpr::string_case(
                StringExpr::panic(panic()),
                Vec::new(),
                StringExpr::value("fallback".into()),
            ),
            StringExpr::float_case(
                FloatExpr::panic(panic()),
                Vec::new(),
                StringExpr::value("fallback".into()),
            ),
            StringExpr::block(
                vec![Step::evaluate(Expr::bool(BoolExpr::panic(panic())))],
                StringExpr::value("value".into()),
            ),
        ];

        for expression in expressions {
            assert_eq!(
                run_module_string_expression(expression).to_string(),
                "panic: `panic` expression evaluated.",
            );
        }
    }

    fn run_module_string_expression(expression: StringExpr) -> ExecutionError {
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::string(StringFunctionId(0), expression),
        );
        let module = ModulePlan::new("main".into(), main, Vec::new());
        let plan = crate::ExecutionPlan::from_module_plan(module);

        run_main(&plan).expect_err("module expression should fail at runtime")
    }
}
