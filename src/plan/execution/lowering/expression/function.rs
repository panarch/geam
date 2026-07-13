mod bit_array;
mod bool;
mod float;
mod int;
mod list;
mod nil;
mod returning_function;
mod string;
mod tuple;

pub(in crate::plan::execution::lowering) use bit_array::bit_array_function_expr;
pub(in crate::plan::execution::lowering) use bool::bool_function_expr;
pub(in crate::plan::execution::lowering) use float::float_function_expr;
pub(in crate::plan::execution::lowering) use int::int_function_expr;
pub(in crate::plan::execution::lowering) use list::list_function_expr;
pub(in crate::plan::execution::lowering) use nil::nil_function_expr;
pub(in crate::plan::execution::lowering) use returning_function::function_function_expr;
pub(in crate::plan::execution::lowering) use string::string_function_expr;
pub(in crate::plan::execution::lowering) use tuple::tuple_function_expr;

use crate::plan::{execution, module};

fn function_reference<ModuleFunction, ExecutionFunction>(
    reference: module::TypedFunctionReference<ModuleFunction>,
    context: &mut super::super::LoweringContext,
    lower_function: impl FnOnce(ModuleFunction, &mut super::super::LoweringContext) -> ExecutionFunction,
) -> execution::FunctionReference<ExecutionFunction> {
    let (function, params) = reference.into_parts();
    execution::FunctionReference::new(
        lower_function(function, context),
        params
            .into_iter()
            .map(|param| crate::plan::execution::lowering::param::param_local(param, context))
            .collect(),
    )
}

fn closure_template<ModuleFunction, ExecutionFunction>(
    function: ModuleFunction,
    params: Vec<module::ParamLocal>,
    captures: Vec<module::CaptureArg>,
    context: &mut super::super::LoweringContext,
    lower_function: impl FnOnce(ModuleFunction, &mut super::super::LoweringContext) -> ExecutionFunction,
) -> execution::ClosureTemplate<ExecutionFunction> {
    execution::ClosureTemplate::new(
        lower_function(function, context),
        params
            .into_iter()
            .map(|param| crate::plan::execution::lowering::param::param_local(param, context))
            .collect(),
        super::capture_args(captures, context),
    )
}

pub(in crate::plan::execution::lowering) fn function_expr(
    expression: module::FunctionExpr,
    context: &mut super::super::LoweringContext,
) -> execution::FunctionExpr {
    execution::FunctionExpr::from_kind(match expression.into_kind() {
        module::FunctionExprKind::Int(expression) => {
            execution::FunctionExprKind::Int(int_function_expr(expression, context))
        }
        module::FunctionExprKind::String(expression) => {
            execution::FunctionExprKind::String(string_function_expr(expression, context))
        }
        module::FunctionExprKind::BitArray(expression) => {
            execution::FunctionExprKind::BitArray(bit_array_function_expr(expression, context))
        }
        module::FunctionExprKind::Float(expression) => {
            execution::FunctionExprKind::Float(float_function_expr(expression, context))
        }
        module::FunctionExprKind::Bool(expression) => {
            execution::FunctionExprKind::Bool(bool_function_expr(expression, context))
        }
        module::FunctionExprKind::Nil(expression) => {
            execution::FunctionExprKind::Nil(nil_function_expr(expression, context))
        }
        module::FunctionExprKind::Tuple(expression) => {
            execution::FunctionExprKind::Tuple(tuple_function_expr(expression, context))
        }
        module::FunctionExprKind::List(expression) => {
            execution::FunctionExprKind::List(list_function_expr(expression, context))
        }
        module::FunctionExprKind::Function(expression) => {
            execution::FunctionExprKind::Function(function_function_expr(expression, context))
        }
    })
}

#[cfg(test)]
mod tests {
    use super::super::super::super::{
        CallArg, CallArgKind, CaptureArg, CaptureArgKind, ClosureTemplate, ExecutionPlan,
        FunctionReference, IntExpr, IntExprKind, IntFunctionExpr, IntFunctionExprKind,
        IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId, IntLocalId, ParamLocal,
        ReturnBody, ReturnBodyKind, Step, StepKind, StringExpr, StringExprKind, StringLocalId,
    };
    use num_bigint::BigInt;

    #[test]
    fn lowering_separates_int_function_reference_and_closure_lifecycles() {
        let plan = reference_closure_execution_plan();
        let main = plan.int_function(IntFunctionId(0));

        assert_eq!(main.steps().len(), 3);

        let (captured_local, captured_value) = expect_int_binding(&main.steps()[0]);
        assert_eq!(captured_local, IntLocalId(0));
        assert_eq!(expect_int_value(captured_value), &BigInt::from(1));

        let (reference_local, reference_value) = expect_int_function_binding(&main.steps()[1]);
        assert_eq!(reference_local, IntFunctionLocalId(0));
        let reference = expect_int_function_reference(reference_value);
        assert_eq!(reference.function(), &IntFunctionId(1));
        assert_eq!(reference.params(), &[ParamLocal::Int(IntLocalId(0))]);

        let (closure_local, closure_value) = expect_int_function_binding(&main.steps()[2]);
        assert_eq!(closure_local, IntFunctionLocalId(1));
        let closure = expect_int_function_closure(closure_value);
        assert_eq!(closure.function(), &IntFunctionId(2));
        assert_eq!(closure.params(), &[ParamLocal::Int(IntLocalId(0))]);
        assert_eq!(closure.captures().len(), 1);
        let (capture_local, capture_value) = expect_int_capture(&closure.captures()[0]);
        assert_eq!(capture_local, IntLocalId(1));
        assert_eq!(expect_int_local_get(capture_value), IntLocalId(0));

        let returned = expect_expression_return(main.return_());
        let (reference_call, reference_args) = expect_int_function_call(returned);
        assert_eq!(
            expect_int_function_local_get(reference_call),
            IntFunctionLocalId(0)
        );
        assert_eq!(reference_args.len(), 1);
        let (reference_arg_local, reference_argument) = expect_int_call_arg(&reference_args[0]);
        assert_eq!(reference_arg_local, IntLocalId(0));
        let (closure_call, closure_args) = expect_int_function_call(reference_argument);
        assert_eq!(
            expect_int_function_local_get(closure_call),
            IntFunctionLocalId(1)
        );
        assert_eq!(closure_args.len(), 1);
        let (closure_arg_local, closure_argument) = expect_int_call_arg(&closure_args[0]);
        assert_eq!(closure_arg_local, IntLocalId(0));
        assert_eq!(expect_int_value(closure_argument), &BigInt::from(40));
    }

    #[test]
    #[should_panic(expected = "expected an Int binding step")]
    fn int_binding_fixture_guard_rejects_function_binding() {
        let plan = reference_closure_execution_plan();
        let _ = expect_int_binding(&plan.int_function(IntFunctionId(0)).steps()[1]);
    }

    #[test]
    #[should_panic(expected = "expected an Int function binding step")]
    fn int_function_binding_fixture_guard_rejects_int_binding() {
        let plan = reference_closure_execution_plan();
        let _ = expect_int_function_binding(&plan.int_function(IntFunctionId(0)).steps()[0]);
    }

    #[test]
    #[should_panic(expected = "expected an Int function reference")]
    fn int_function_reference_fixture_guard_rejects_closure() {
        let plan = reference_closure_execution_plan();
        let (_, expression) =
            expect_int_function_binding(&plan.int_function(IntFunctionId(0)).steps()[2]);
        let _ = expect_int_function_reference(expression);
    }

    #[test]
    #[should_panic(expected = "expected an Int function closure")]
    fn int_function_closure_fixture_guard_rejects_reference() {
        let plan = reference_closure_execution_plan();
        let (_, expression) =
            expect_int_function_binding(&plan.int_function(IntFunctionId(0)).steps()[1]);
        let _ = expect_int_function_closure(expression);
    }

    #[test]
    #[should_panic(expected = "expected an Int capture")]
    fn int_capture_fixture_guard_rejects_function_capture() {
        let source = r#"
pub fn main() {
  let captured = fn() { 1 }
  let closure = fn() { captured() }
  closure()
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);
        let (_, closure_expression) =
            expect_int_function_binding(&plan.int_function(IntFunctionId(0)).steps()[1]);
        let closure = expect_int_function_closure(closure_expression);

        let _ = expect_int_capture(&closure.captures()[0]);
    }

    #[test]
    #[should_panic(expected = "expected an Int function call")]
    fn int_function_call_fixture_guard_rejects_value() {
        let plan = reference_closure_execution_plan();
        let (_, value) = expect_int_binding(&plan.int_function(IntFunctionId(0)).steps()[0]);
        let _ = expect_int_function_call(value);
    }

    #[test]
    #[should_panic(expected = "expected an Int call argument")]
    fn int_call_argument_fixture_guard_rejects_string_argument() {
        let argument = CallArg::from_kind(CallArgKind::String {
            local: StringLocalId(0),
            value: StringExpr::from_kind(StringExprKind::Value("value".into())),
        });

        let _ = expect_int_call_arg(&argument);
    }

    #[test]
    #[should_panic(expected = "expected an Int function local get")]
    fn int_function_local_get_fixture_guard_rejects_reference() {
        let plan = reference_closure_execution_plan();
        let (_, reference) =
            expect_int_function_binding(&plan.int_function(IntFunctionId(0)).steps()[1]);
        let _ = expect_int_function_local_get(reference);
    }

    #[test]
    #[should_panic(expected = "expected an Int local get")]
    fn int_local_get_fixture_guard_rejects_value() {
        let plan = reference_closure_execution_plan();
        let (_, value) = expect_int_binding(&plan.int_function(IntFunctionId(0)).steps()[0]);
        let _ = expect_int_local_get(value);
    }

    #[test]
    #[should_panic(expected = "expected an Int value")]
    fn int_value_fixture_guard_rejects_local_get() {
        let plan = reference_closure_execution_plan();
        let (_, closure_expression) =
            expect_int_function_binding(&plan.int_function(IntFunctionId(0)).steps()[2]);
        let capture = expect_int_function_closure(closure_expression)
            .captures()
            .first()
            .expect("fixture should contain one capture");
        let (_, value) = expect_int_capture(capture);
        let _ = expect_int_value(value);
    }

    #[test]
    #[should_panic(expected = "expected an expression return body")]
    fn int_expression_return_fixture_guard_rejects_case_return() {
        let source = r#"
pub fn main() {
  case True {
    True -> 1
    False -> 0
  }
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        let _ = expect_expression_return(plan.int_function(IntFunctionId(0)).return_());
    }

    #[test]
    #[should_panic(expected = "expected an expression return body")]
    fn int_function_expression_return_fixture_guard_rejects_case_return() {
        let source = r#"
fn identity(value: Int) { value }

fn choose() {
  case True {
    True -> identity
    False -> identity
  }
}

pub fn main() { choose()(1) }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);

        let _ = expect_expression_return(
            plan.int_function_function(IntFunctionFunctionId(0))
                .return_(),
        );
    }

    fn reference_closure_execution_plan() -> ExecutionPlan {
        let source = r#"
fn identity(value: Int) { value }

pub fn main() {
  let captured = 1
  let reference = identity
  let closure = fn(value) { value + captured }
  reference(closure(40))
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        ExecutionPlan::from_module_plan(module_plan)
    }

    fn expect_int_binding(step: &Step) -> (IntLocalId, &IntExpr) {
        match step.kind() {
            StepKind::LetInt { local, value } => (*local, value),
            _ => panic!("expected an Int binding step"),
        }
    }

    fn expect_int_function_binding(step: &Step) -> (IntFunctionLocalId, &IntFunctionExpr) {
        match step.kind() {
            StepKind::LetIntFunction { local, value } => (*local, value),
            _ => panic!("expected an Int function binding step"),
        }
    }

    fn expect_int_function_reference(
        expression: &IntFunctionExpr,
    ) -> &FunctionReference<IntFunctionId> {
        match expression.kind() {
            IntFunctionExprKind::Reference(reference) => reference,
            _ => panic!("expected an Int function reference"),
        }
    }

    fn expect_int_function_closure(
        expression: &IntFunctionExpr,
    ) -> &ClosureTemplate<IntFunctionId> {
        match expression.kind() {
            IntFunctionExprKind::Closure(closure) => closure,
            _ => panic!("expected an Int function closure"),
        }
    }

    fn expect_int_capture(capture: &CaptureArg) -> (IntLocalId, &IntExpr) {
        match capture.kind() {
            CaptureArgKind::Int { local, value } => (*local, value),
            _ => panic!("expected an Int capture"),
        }
    }

    fn expect_int_function_call(expression: &IntExpr) -> (&IntFunctionExpr, &[CallArg]) {
        match expression.kind() {
            IntExprKind::FunctionCall { function, args } => (function, args),
            _ => panic!("expected an Int function call"),
        }
    }

    fn expect_int_call_arg(argument: &CallArg) -> (IntLocalId, &IntExpr) {
        match argument.kind() {
            CallArgKind::Int { local, value } => (*local, value),
            _ => panic!("expected an Int call argument"),
        }
    }

    fn expect_int_function_local_get(expression: &IntFunctionExpr) -> IntFunctionLocalId {
        match expression.kind() {
            IntFunctionExprKind::LocalGet { local } => *local,
            _ => panic!("expected an Int function local get"),
        }
    }

    fn expect_int_local_get(expression: &IntExpr) -> IntLocalId {
        match expression.kind() {
            IntExprKind::LocalGet { local } => *local,
            _ => panic!("expected an Int local get"),
        }
    }

    fn expect_int_value(expression: &IntExpr) -> &BigInt {
        match expression.kind() {
            IntExprKind::Value(value) => value,
            _ => panic!("expected an Int value"),
        }
    }

    fn expect_expression_return<Expression, Function>(
        body: &ReturnBody<Expression, Function>,
    ) -> &Expression {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => expression,
            _ => panic!("expected an expression return body"),
        }
    }
}
