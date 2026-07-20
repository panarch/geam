use super::eval_custom_field_function;
use crate::plan::ValueType;
use crate::plan::execution::{
    ExecutionPlan, FunctionReturnFamily, GenericFunctionType, NeverFunctionExpr,
    NeverFunctionExprKind,
};
use crate::runtime::expression::{
    eval_bool_expr, eval_direct_call, eval_float_expr, eval_function_call, eval_int_expr,
    eval_panic_expr, eval_string_expr, project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedFunctionValueKind, EvaluatedNeverFunction, EvaluatedValue, ExecutionError,
};

pub(in crate::runtime) fn eval_never_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &NeverFunctionExpr,
) -> Result<EvaluatedNeverFunction, ExecutionError> {
    eval_never_function_expr_kind(plan, state, frame, expression.type_(), expression.kind())
}

pub(in crate::runtime) fn eval_never_function_expr_kind(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    type_: &GenericFunctionType,
    kind: &NeverFunctionExprKind,
) -> Result<EvaluatedNeverFunction, ExecutionError> {
    match kind {
        NeverFunctionExprKind::Constant(value) => {
            eval_never_function_expr(plan, state, frame, plan.constant(*value))
        }
        NeverFunctionExprKind::Reference(reference) => Ok(EvaluatedNeverFunction::reference(
            *reference.function(),
            reference.param_locals(),
            Vec::new(),
            type_.to_function_type(),
        )),
        NeverFunctionExprKind::Closure(closure) => Ok(EvaluatedNeverFunction::closure(
            *closure.function(),
            closure.param_locals(),
            function::eval_capture_args(plan, state, frame, closure.captures())?,
            type_.to_function_type(),
        )),
        NeverFunctionExprKind::LocalGet { local } => Ok(frame.get_never_function(local)),
        NeverFunctionExprKind::Call(call) => eval_direct_call(
            plan,
            state,
            frame,
            call,
            function::run_never_function_returning_function_call,
        ),
        NeverFunctionExprKind::FunctionCall(call) => eval_function_call(
            plan,
            state,
            frame,
            call,
            function::run_never_function_function_call,
        ),
        NeverFunctionExprKind::TupleIndex { tuple, index } => {
            let expected =
                ValueType::Function(Box::new(plan.function_type(&type_.to_function_type())));
            let value = project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type(plan);
            match value {
                EvaluatedValue::Function(function) => match function.kind() {
                    EvaluatedFunctionValueKind::Never(value) if actual == expected => {
                        Ok(value.clone())
                    }
                    _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
                },
                _ => Err(ExecutionError::TupleIndexFamilyMismatch { expected, actual }),
            }
        }
        NeverFunctionExprKind::CustomField(access) => {
            let (constructor, expected, function) =
                eval_custom_field_function(plan, state, frame, access)?;
            let actual = ValueType::Function(Box::new(plan.function_type(function.type_())));
            match function.kind() {
                EvaluatedFunctionValueKind::Never(value) if actual == expected => Ok(value.clone()),
                _ => {
                    let descriptor = plan.custom_constructor(constructor);
                    Err(ExecutionError::CustomFieldFamilyMismatch {
                        custom_type: plan.custom_value_type(constructor.type_id()),
                        constructor: descriptor.name().clone(),
                        field_index: access.index(),
                        expected,
                        actual,
                    })
                }
            }
        }
        NeverFunctionExprKind::ListIndex { list, index } => {
            let type_ = plan.function_type(&type_.to_function_type());
            let function = project_function_list_expr(plan, state, frame, list, *index, &type_)?;
            match function.kind() {
                EvaluatedFunctionValueKind::Never(value) => Ok(value.clone()),
                _ => Err(ExecutionError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::Never,
                    actual: function.kind().family(),
                }),
            }
        }
        NeverFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        NeverFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_never_function_expr_kind(plan, state, frame, type_, true_)
            } else {
                eval_never_function_expr_kind(plan, state, frame, type_, false_)
            }
        }
        NeverFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_never_function_expr_kind(plan, state, frame, type_, branch);
                }
            }
            eval_never_function_expr_kind(plan, state, frame, type_, fallback)
        }
        NeverFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_never_function_expr_kind(plan, state, frame, type_, branch);
                }
            }
            eval_never_function_expr_kind(plan, state, frame, type_, fallback)
        }
        NeverFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_never_function_expr_kind(plan, state, frame, type_, branch);
                }
            }
            eval_never_function_expr_kind(plan, state, frame, type_, fallback)
        }
        NeverFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_never_function_expr_kind(plan, state, frame, type_, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{
        CallArg, CallArgKind, CustomConstruction, CustomExpr, CustomExprKind, CustomFieldAccess,
        CustomLocalExpr, FunctionReturnFamily, NeverFunctionExpr, NeverFunctionExprKind,
        NeverFunctionFunctionId, ReturnBody, ReturnBodyKind, RuntimeFunctionId, TypedListExprKind,
    };
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionExpr,
        FunctionListExpr as ModuleFunctionListExpr, FunctionListItem, FunctionListLocalId,
        FunctionShape, FunctionTemplate, FunctionTemplateId, FunctionTemplateSignature,
        FunctionType, GenericExpr, GenericFunctionExpr as ModuleGenericFunctionExpr,
        GenericFunctionReturn, GenericFunctionType, IntExpr, IntFunctionExpr, IntFunctionId,
        IntFunctionReference, IntLocalId, ModulePlan, PanicExpr, PanicSite, Param, ParamLocal,
        ParamSlot, ReturnExpr, Step, StringExpr, TupleExpr, TypeParameterId, TypeScheme,
        ValueShape, ValueType, monomorphic_function_instantiation,
    };
    use crate::runtime::evaluated::{
        EvaluatedFunctionValue, EvaluatedFunctionValueKind, EvaluatedIntFunction, EvaluatedValue,
    };
    use crate::runtime::expression::eval_never_function_expr;
    use crate::runtime::frame::Frame;
    use crate::runtime::state::RuntimeState;
    use crate::runtime::{EvaluatedCustomValue, ExecutionError, PanicKind};

    #[test]
    fn module_expression_errors_propagate_through_never_function_wrappers() {
        let parameter = TypeParameterId(0);
        let type_ = GenericFunctionType::new(vec![ValueShape::Int], parameter);
        let target_instantiation = FunctionTemplateSignature::new(
            FunctionTemplateId::new(1),
            TypeScheme::new(1),
            type_.shape(),
        )
        .identity_instantiation();
        let panic = |message: &str| {
            PanicExpr::panic_at(
                Some(StringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let target = || {
            FunctionTemplate::from_signature(
                FunctionTemplateSignature::new(
                    FunctionTemplateId::new(1),
                    TypeScheme::new(1),
                    type_.shape(),
                ),
                "diverge".into(),
                vec![Param::named_shape(
                    ParamLocal::int(IntLocalId(0)),
                    "value".into(),
                    ValueShape::Int,
                )],
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(1),
                    "capture".into(),
                )))],
                ReturnExpr::generic_body(
                    parameter,
                    crate::plan::GenericReturn::expr(GenericExpr::panic(
                        parameter,
                        panic("target"),
                    )),
                ),
            )
        };
        let fallback = || ModuleGenericFunctionExpr::panic(panic("fallback"), type_.clone());
        let function_type =
            FunctionType::new(vec![ValueType::Int], ValueType::Parameter(parameter));
        let expressions = [
            (
                ModuleGenericFunctionExpr::closure(
                    target_instantiation,
                    vec![ParamSlot::new(
                        ParamLocal::int(IntLocalId(0)),
                        ValueShape::Int,
                    )],
                    vec![CaptureArg::int(
                        IntLocalId(1),
                        IntExpr::panic(panic("capture")),
                    )],
                    type_.clone(),
                ),
                "capture",
            ),
            (
                ModuleGenericFunctionExpr::tuple_index(
                    TupleExpr::panic(
                        panic("tuple"),
                        vec![ValueType::Function(Box::new(function_type.clone()))],
                    ),
                    0,
                    type_.clone(),
                ),
                "tuple",
            ),
            (
                ModuleGenericFunctionExpr::list_index(
                    super::super::expect_function_list(crate::plan::ListExpr::panic(
                        panic("list"),
                        ValueType::Function(Box::new(function_type)),
                    )),
                    0,
                    type_.clone(),
                ),
                "list",
            ),
            (
                ModuleGenericFunctionExpr::bool_case(
                    BoolExpr::panic(panic("bool subject")),
                    fallback(),
                    fallback(),
                )
                .expect("matching never function branches should plan"),
                "bool subject",
            ),
            (
                ModuleGenericFunctionExpr::int_case(
                    IntExpr::panic(panic("int subject")),
                    Vec::new(),
                    fallback(),
                )
                .expect("matching never function branches should plan"),
                "int subject",
            ),
            (
                ModuleGenericFunctionExpr::string_case(
                    StringExpr::panic(panic("string subject")),
                    Vec::new(),
                    fallback(),
                )
                .expect("matching never function branches should plan"),
                "string subject",
            ),
            (
                ModuleGenericFunctionExpr::float_case(
                    FloatExpr::panic(panic("float subject")),
                    Vec::new(),
                    fallback(),
                )
                .expect("matching never function branches should plan"),
                "float subject",
            ),
            (
                ModuleGenericFunctionExpr::block(
                    vec![Step::evaluate(Expr::int(IntExpr::panic(panic("step"))))],
                    fallback(),
                ),
                "step",
            ),
        ];

        for (expression, message) in expressions {
            assert_eq!(
                crate::run_main(&never_function_plan(expression, vec![target()])),
                Err(ExecutionError::source_panic(
                    None,
                    PanicKind::Panic,
                    Some(message.into()),
                    PanicSite::unknown(),
                )),
            );
        }
    }

    #[test]
    fn never_function_custom_field_propagates_source_error() {
        let source = r#"
pub type Box(value) {
  Box(value: fn(Int) -> value)
}

fn fail() -> Box(value) {
  panic as "custom field"
}

pub fn main() {
  fail().value
}
"#;

        assert_eq!(
            crate::runtime::run_src_error(source).to_string(),
            "panic: custom field",
        );
    }

    #[test]
    fn never_function_list_projection_reports_direct_mutated_return_family() {
        let parameter = TypeParameterId(0);
        let type_ = GenericFunctionType::new(vec![ValueShape::Int], parameter);
        let expression = ModuleGenericFunctionExpr::list_index(
            ModuleFunctionListExpr::local_get(
                FunctionListItem::new(FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Parameter(parameter),
                )),
                FunctionListLocalId(0),
                "functions".into(),
            ),
            0,
            type_,
        );
        let function_shape = expression.shape();
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::new(0),
            TypeScheme::new(1),
            FunctionShape::new(
                Vec::new(),
                ValueShape::Function(Box::new(function_shape.clone())),
            ),
        );
        let main = FunctionTemplate::from_signature(
            signature,
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::generic_function_shape_body(
                function_shape.clone(),
                GenericFunctionReturn::expr(expression),
            ),
        );
        let plan = crate::ExecutionPlan::from_module_plan(ModulePlan::new(
            "main".into(),
            main,
            Vec::new(),
        ));
        let main_id = main_never_function_id(&plan);
        let main = plan.never_function_function(&main_id);
        let expression = expression_return(main.return_().body());
        let list = function_list_projection(expression);
        let local = function_list_local(list);
        let wrong = EvaluatedIntFunction::reference(
            crate::plan::execution::IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::FunctionType::new(
                Vec::new(),
                crate::plan::execution::ValueType::Int,
            ),
        );
        let mut state = RuntimeState::new();
        let functions = state.function(
            list.item().type_id(),
            vec![EvaluatedFunctionValue::from_kind(
                EvaluatedFunctionValueKind::Int(wrong),
            )],
        );
        let mut frame = Frame::new(main.frame_layout(), &mut state);
        frame.set_function_list(local, functions);

        assert_eq!(
            eval_never_function_expr(&plan, &mut state, &mut frame, expression),
            Err(ExecutionError::FunctionReturnFamilyMismatch {
                expected: FunctionReturnFamily::Never,
                actual: FunctionReturnFamily::Int,
            }),
        );
    }

    #[test]
    fn never_function_custom_field_reports_direct_mutated_family_mismatch() {
        let plan = crate::runtime::plan_src(
            r#"
pub type Box(value) {
  Box(value: fn(Int) -> value)
}

fn diverge(_value: Int) -> value { panic }
fn get(box: Box(value)) { box.value }
pub fn main() { get(Box(diverge)) }
"#,
        );
        let main_id = main_never_function_id(&plan);
        let main = plan.never_function_function(&main_id);
        let (function, args) = tail_call(main.return_().body());
        let binding = custom_argument(&args[0]);
        let construction = custom_construction(binding.value());
        let target = plan.never_function_function(function);
        let expression = expression_return(target.return_().body());
        let access = custom_field_projection(expression);
        let wrong_type = crate::plan::execution::FunctionType::new(
            Vec::new(),
            crate::plan::execution::ValueType::Int,
        );
        let wrong = EvaluatedIntFunction::reference(
            crate::plan::execution::IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            wrong_type.clone(),
        );
        let value = EvaluatedCustomValue::from_fields(
            construction.constructor(),
            vec![EvaluatedValue::Function(EvaluatedFunctionValue::from_kind(
                EvaluatedFunctionValueKind::Int(wrong),
            ))]
            .into_boxed_slice(),
        );
        let mut state = RuntimeState::new();
        let mut frame = Frame::new(target.frame_layout(), &mut state);
        frame.set_custom(binding.local(), value);
        let descriptor = plan.custom_constructor(construction.constructor());

        assert_eq!(
            eval_never_function_expr(&plan, &mut state, &mut frame, expression),
            Err(ExecutionError::CustomFieldFamilyMismatch {
                custom_type: plan.custom_value_type(construction.constructor().type_id()),
                constructor: descriptor.name().clone(),
                field_index: access.index(),
                expected: ValueType::Function(Box::new(crate::plan::FunctionType::new(
                    vec![ValueType::Int],
                    ValueType::Parameter(TypeParameterId(0)),
                ))),
                actual: ValueType::Function(Box::new(plan.function_type(&wrong_type))),
            }),
        );
    }

    #[test]
    fn never_function_tuple_projection_reports_direct_mutated_family_mismatches() {
        let parameter = TypeParameterId(0);
        let type_ = GenericFunctionType::new(vec![ValueShape::Int], parameter);
        let expected_type =
            FunctionType::new(vec![ValueType::Int], ValueType::Parameter(parameter));
        let expected = ValueType::Function(Box::new(expected_type));
        let wrong_type = FunctionType::new(Vec::new(), ValueType::Int);
        let wrong_function =
            FunctionExpr::int(IntFunctionExpr::reference(IntFunctionReference::new(
                monomorphic_function_instantiation(
                    1,
                    FunctionShape::from_function_type(wrong_type.clone()),
                ),
                Vec::new(),
            )));
        let expression = ModuleGenericFunctionExpr::tuple_index(
            TupleExpr::value(vec![Expr::function(wrong_function)], vec![expected.clone()]),
            0,
            type_.clone(),
        );
        let target = FunctionTemplate::new(
            FunctionTemplateId::new(1),
            "target".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into())),
        );

        assert_eq!(
            crate::run_main(&never_function_plan(expression, vec![target])),
            Err(ExecutionError::TupleIndexFamilyMismatch {
                expected: expected.clone(),
                actual: ValueType::Function(Box::new(wrong_type)),
            }),
        );

        let expression = ModuleGenericFunctionExpr::tuple_index(
            TupleExpr::value(
                vec![Expr::int(IntExpr::value(1.into()))],
                vec![expected.clone()],
            ),
            0,
            type_,
        );
        assert_eq!(
            crate::run_main(&never_function_plan(expression, Vec::new())),
            Err(ExecutionError::TupleIndexFamilyMismatch {
                expected,
                actual: ValueType::Int,
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected a never-function main")]
    fn never_function_main_fixture_guard_rejects_int_main() {
        let plan = crate::runtime::plan_src("pub fn main() { 1 }");
        let _ = main_never_function_id(&plan);
    }

    #[test]
    #[should_panic(expected = "expected an expression return body")]
    fn expression_return_fixture_guard_rejects_tail_call() {
        let plan = custom_field_execution_plan();
        let main = plan.never_function_function(&main_never_function_id(&plan));
        let _ = expression_return(main.return_().body());
    }

    #[test]
    #[should_panic(expected = "expected a function-list projection")]
    fn function_list_projection_fixture_guard_rejects_reference() {
        let plan = crate::runtime::plan_src(
            "fn diverge(_value: Int) -> value { panic }\npub fn main() { diverge }",
        );
        let main = plan.never_function_function(&main_never_function_id(&plan));
        let _ = function_list_projection(expression_return(main.return_().body()));
    }

    #[test]
    #[should_panic(expected = "expected a function-list local")]
    fn function_list_local_fixture_guard_rejects_value() {
        let plan = crate::runtime::plan_src(
            "fn diverge(_value: Int) -> value { panic }\npub fn main() { [diverge] }",
        );
        let function = plan.function_list_function(plan.function_list_function_id(0));
        let list = expression_return(function.return_());
        let _ = function_list_local(list);
    }

    #[test]
    #[should_panic(expected = "expected a tail-call return body")]
    fn tail_call_fixture_guard_rejects_expression_return() {
        let plan = crate::runtime::plan_src(
            "fn diverge(_value: Int) -> value { panic }\npub fn main() { diverge }",
        );
        let main = plan.never_function_function(&main_never_function_id(&plan));
        let _ = tail_call(main.return_().body());
    }

    #[test]
    #[should_panic(expected = "expected a custom call argument")]
    fn custom_argument_fixture_guard_rejects_int_argument() {
        let plan = crate::runtime::plan_src(
            "fn diverge(_value: Int) -> value { panic }\nfn target(_value: Int) { diverge }\npub fn main() { target(1) }",
        );
        let main = plan.never_function_function(&main_never_function_id(&plan));
        let (_, args) = tail_call(main.return_().body());
        let _ = custom_argument(&args[0]);
    }

    #[test]
    #[should_panic(expected = "expected a custom construction")]
    fn custom_construction_fixture_guard_rejects_local_get() {
        let plan = custom_field_execution_plan();
        let main = plan.never_function_function(&main_never_function_id(&plan));
        let (function, _) = tail_call(main.return_().body());
        let target = plan.never_function_function(function);
        let access = custom_field_projection(expression_return(target.return_().body()));
        let _ = custom_construction(access.source());
    }

    #[test]
    #[should_panic(expected = "expected a never custom-field projection")]
    fn custom_field_fixture_guard_rejects_reference() {
        let plan = crate::runtime::plan_src(
            "fn diverge(_value: Int) -> value { panic }\npub fn main() { diverge }",
        );
        let main = plan.never_function_function(&main_never_function_id(&plan));
        let _ = custom_field_projection(expression_return(main.return_().body()));
    }

    fn custom_field_execution_plan() -> crate::ExecutionPlan {
        crate::runtime::plan_src(
            r#"
pub type Box(value) {
  Box(value: fn(Int) -> value)
}

fn diverge(_value: Int) -> value { panic }
fn get(box: Box(value)) { box.value }
pub fn main() { get(Box(diverge)) }
"#,
        )
    }

    fn never_function_plan(
        expression: ModuleGenericFunctionExpr,
        functions: Vec<FunctionTemplate>,
    ) -> crate::ExecutionPlan {
        let function_shape = expression.shape();
        let main_shape = FunctionShape::new(
            Vec::new(),
            ValueShape::Function(Box::new(function_shape.clone())),
        );
        let signature = FunctionTemplateSignature::new(
            FunctionTemplateId::new(0),
            TypeScheme::new(1),
            main_shape,
        );
        let main = FunctionTemplate::from_signature(
            signature,
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::generic_function_shape_body(
                function_shape.clone(),
                GenericFunctionReturn::expr(expression),
            ),
        );
        let module = ModulePlan::new("main".into(), main, functions);

        crate::ExecutionPlan::from_module_plan(module)
    }

    fn main_never_function_id(
        plan: &crate::ExecutionPlan,
    ) -> crate::plan::execution::NeverFunctionFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::Function {
                id: crate::plan::execution::FunctionFunctionId::Never(id),
                ..
            } => id,
            _ => panic!("expected a never-function main"),
        }
    }

    fn expression_return<Expression, Function>(
        body: &crate::plan::execution::ReturnBody<Expression, Function>,
    ) -> &Expression {
        match body.kind() {
            ReturnBodyKind::Expr(expression) => expression,
            _ => panic!("expected an expression return body"),
        }
    }

    fn function_list_projection(
        expression: &NeverFunctionExpr,
    ) -> &crate::plan::execution::FunctionListExpr {
        match expression.kind() {
            NeverFunctionExprKind::ListIndex { list, .. } => list,
            _ => panic!("expected a function-list projection"),
        }
    }

    fn function_list_local(
        list: &crate::plan::execution::FunctionListExpr,
    ) -> crate::plan::execution::FunctionListLocalId {
        match list.kind() {
            TypedListExprKind::LocalGet { local } => *local,
            _ => panic!("expected a function-list local"),
        }
    }

    fn tail_call(
        body: &ReturnBody<NeverFunctionExpr, NeverFunctionFunctionId>,
    ) -> (&NeverFunctionFunctionId, &[CallArg]) {
        match body.kind() {
            ReturnBodyKind::TailCall { function, args } => (function, args),
            _ => panic!("expected a tail-call return body"),
        }
    }

    fn custom_argument(argument: &CallArg) -> &CustomLocalExpr {
        match argument.kind() {
            CallArgKind::Custom(binding) => binding,
            _ => panic!("expected a custom call argument"),
        }
    }

    fn custom_construction(expression: &CustomExpr) -> &CustomConstruction {
        match expression.kind() {
            CustomExprKind::Constructor(construction) => construction,
            _ => panic!("expected a custom construction"),
        }
    }

    fn custom_field_projection(expression: &NeverFunctionExpr) -> &CustomFieldAccess {
        match expression.kind() {
            NeverFunctionExprKind::CustomField(access) => access,
            _ => panic!("expected a never custom-field projection"),
        }
    }
}
