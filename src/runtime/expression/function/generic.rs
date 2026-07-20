use super::eval_custom_field_function;
use crate::plan::ValueType;
use crate::plan::execution::{
    ExecutionPlan, FunctionReturnFamily, GenericFunctionExpr, GenericFunctionExprKind,
    GenericFunctionType,
};
use crate::runtime::expression::{
    eval_bool_expr, eval_direct_call, eval_float_expr, eval_function_call, eval_int_expr,
    eval_panic_expr, eval_string_expr, project_function_list_expr, project_tuple_expr,
};
use crate::runtime::frame::Frame;
use crate::runtime::function;
use crate::runtime::state::RuntimeState;
use crate::runtime::{
    EvaluatedFunctionValueKind, EvaluatedGenericFunction, EvaluatedValue, ExecutionError,
    InvariantError,
};

pub(in crate::runtime) fn eval_generic_function_expr(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    expression: &GenericFunctionExpr,
) -> Result<EvaluatedGenericFunction, ExecutionError> {
    eval_generic_function_expr_kind(
        plan,
        state,
        frame,
        expression.generic_function_type(),
        expression.kind(),
    )
}

pub(in crate::runtime) fn eval_generic_function_expr_kind(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    type_: &GenericFunctionType,
    kind: &GenericFunctionExprKind,
) -> Result<EvaluatedGenericFunction, ExecutionError> {
    match kind {
        GenericFunctionExprKind::Constant(value) => {
            eval_generic_function_expr(plan, state, frame, plan.constant(*value))
        }
        GenericFunctionExprKind::Reference { target } => Ok(EvaluatedGenericFunction::reference(
            target.clone(),
            Vec::new(),
            Vec::new(),
            type_.to_function_type(),
        )),
        GenericFunctionExprKind::Constructor { target } => Ok(EvaluatedGenericFunction::closure(
            target.clone(),
            Vec::new(),
            Vec::new(),
            type_.to_function_type(),
        )),
        GenericFunctionExprKind::Closure { target, captures } => {
            Ok(EvaluatedGenericFunction::closure(
                target.clone(),
                Vec::new(),
                function::eval_capture_args(plan, state, frame, captures)?,
                type_.to_function_type(),
            ))
        }
        GenericFunctionExprKind::LocalGet { local } => Ok(frame.get_generic_function(local)),
        GenericFunctionExprKind::Call(call) => eval_direct_call(
            plan,
            state,
            frame,
            call,
            function::run_generic_function_returning_function_call,
        ),
        GenericFunctionExprKind::FunctionCall(call) => eval_function_call(
            plan,
            state,
            frame,
            call,
            function::run_generic_function_function_call,
        ),
        GenericFunctionExprKind::TupleIndex { tuple, index } => {
            let expected =
                ValueType::Function(Box::new(plan.function_type(&type_.to_function_type())));
            let value = project_tuple_expr(plan, state, frame, tuple, *index, expected.clone())?;
            let actual = value.value_type(plan);
            match value {
                EvaluatedValue::Function(function) => match function.kind() {
                    EvaluatedFunctionValueKind::Generic(value) if actual == expected => {
                        Ok(value.clone())
                    }
                    _ => Err(ExecutionError::Invariant(
                        InvariantError::TupleIndexFamilyMismatch { expected, actual },
                    )),
                },
                _ => Err(ExecutionError::Invariant(
                    InvariantError::TupleIndexFamilyMismatch { expected, actual },
                )),
            }
        }
        GenericFunctionExprKind::CustomField(access) => {
            let (constructor, expected, function) =
                eval_custom_field_function(plan, state, frame, access)?;
            let actual = ValueType::Function(Box::new(plan.function_type(function.type_())));
            match function.kind() {
                EvaluatedFunctionValueKind::Generic(value) if actual == expected => {
                    Ok(value.clone())
                }
                _ => {
                    let descriptor = plan.custom_constructor(constructor);
                    Err(ExecutionError::Invariant(
                        InvariantError::CustomFieldFamilyMismatch {
                            custom_type: plan.custom_value_type(constructor.type_id()),
                            constructor: descriptor.name().clone(),
                            field_index: access.index(),
                            expected,
                            actual,
                        },
                    ))
                }
            }
        }
        GenericFunctionExprKind::ListIndex { list, index } => {
            let type_ = plan.function_type(&type_.to_function_type());
            let function = project_function_list_expr(plan, state, frame, list, *index, &type_)?;
            match function.kind() {
                EvaluatedFunctionValueKind::Generic(value) => Ok(value.clone()),
                _ => Err(ExecutionError::Invariant(
                    InvariantError::FunctionReturnFamilyMismatch {
                        expected: FunctionReturnFamily::Generic,
                        actual: function.kind().family(),
                    },
                )),
            }
        }
        GenericFunctionExprKind::Panic(panic) => {
            eval_panic_expr(plan, state, frame, panic).map(|never| match never {})
        }
        GenericFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => {
            if eval_bool_expr(plan, state, frame, subject)? {
                eval_generic_function_expr_kind(plan, state, frame, type_, true_)
            } else {
                eval_generic_function_expr_kind(plan, state, frame, type_, false_)
            }
        }
        GenericFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_int_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_generic_function_expr_kind(plan, state, frame, type_, branch);
                }
            }
            eval_generic_function_expr_kind(plan, state, frame, type_, fallback)
        }
        GenericFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_string_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_generic_function_expr_kind(plan, state, frame, type_, branch);
                }
            }
            eval_generic_function_expr_kind(plan, state, frame, type_, fallback)
        }
        GenericFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => {
            let subject = eval_float_expr(plan, state, frame, subject)?;
            for (pattern, branch) in clauses {
                if pattern == &subject {
                    return eval_generic_function_expr_kind(plan, state, frame, type_, branch);
                }
            }
            eval_generic_function_expr_kind(plan, state, frame, type_, fallback)
        }
        GenericFunctionExprKind::Block { steps, return_ } => {
            function::execute_steps(plan, state, steps, frame)?;
            eval_generic_function_expr_kind(plan, state, frame, type_, return_)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::{
        CallArg, CallArgKind, CustomConstruction, CustomExpr, CustomExprKind, CustomFieldAccess,
        CustomLocalExpr, FunctionFunctionId, FunctionReturnFamily, GenericFunctionExpr,
        GenericFunctionExprKind, GenericFunctionFunctionId, ReturnBlock, ReturnGraph,
        RuntimeFunctionId, TypedListExprKind,
    };
    use crate::plan::{
        BoolExpr, CaptureArg, Expr, FloatExpr, FunctionExpr, FunctionListExpr, FunctionListItem,
        FunctionListLocalId, FunctionShape, FunctionTemplate, FunctionTemplateId,
        FunctionTemplateSignature, FunctionType, GenericExpr,
        GenericFunctionExpr as ModuleGenericFunctionExpr, GenericFunctionReturn,
        GenericFunctionType, GenericLocal, GenericLocalId, IntExpr, IntFunctionExpr, IntFunctionId,
        IntFunctionReference, IntLocalId, ModulePlan, PanicExpr, PanicSite, Param, ParamLocal,
        ParamSlot, ReturnExpr, Step, StringExpr, TupleExpr, TypeParameterId, TypeScheme,
        ValueShape, ValueType, monomorphic_function_instantiation,
    };
    use crate::runtime::expression::eval_generic_function_expr;
    use crate::runtime::frame::Frame;
    use crate::runtime::state::RuntimeState;
    use crate::runtime::{
        EvaluatedCustomValue, EvaluatedFunctionValue, EvaluatedFunctionValueKind,
        EvaluatedIntFunction, EvaluatedValue, ExecutionError, InvariantError, PanicKind,
    };

    #[test]
    fn generic_function_evaluation_preserves_the_symbolic_return_family() {
        let plan = crate::runtime::plan_src("pub fn main() { fn(value) { value } }");
        let function_id = main_generic_function_id(&plan);
        let function = plan.generic_function_function(&function_id);
        let expression = expression_return(function.return_().body());
        let mut state = RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        let value = eval_generic_function_expr(&plan, &mut state, &mut frame, expression)
            .expect("generic closure evaluation should succeed");

        assert_eq!(
            EvaluatedFunctionValue::from(value).kind().family(),
            FunctionReturnFamily::Generic,
        );
    }

    #[test]
    fn generic_constructor_function_evaluation_creates_fresh_instances() {
        let plan =
            crate::runtime::plan_src("pub type Box(value) { Box(value) }\npub fn main() { Box }");
        let function_id = main_generic_function_id(&plan);
        let function = plan.generic_function_function(&function_id);
        let expression = expression_return(function.return_().body());
        let mut state = RuntimeState::new();
        let mut frame = Frame::new(function.frame_layout(), &mut state);

        let first = eval_generic_function_expr(&plan, &mut state, &mut frame, expression)
            .expect("generic constructor evaluation should succeed");
        let second = eval_generic_function_expr(&plan, &mut state, &mut frame, expression)
            .expect("each generic constructor evaluation should succeed");

        assert_eq!(first.type_(), second.type_());
        assert_ne!(first, second);
    }

    #[test]
    fn module_expression_errors_propagate_through_generic_function_wrappers() {
        let parameter = TypeParameterId(0);
        let type_ = generic_function_type(parameter);
        let target_local = GenericLocal::new(GenericLocalId(0), parameter);
        let target_instantiation = FunctionTemplateSignature::new(
            FunctionTemplateId::new(1),
            TypeScheme::new(1),
            type_.shape(),
        )
        .identity_instantiation();
        let target = || {
            FunctionTemplate::from_signature(
                FunctionTemplateSignature::new(
                    FunctionTemplateId::new(1),
                    TypeScheme::new(1),
                    type_.shape(),
                ),
                "identity".into(),
                vec![Param::named_shape(
                    ParamLocal::generic(target_local),
                    "value".into(),
                    ValueShape::Parameter(parameter),
                )],
                vec![Step::evaluate(Expr::int(IntExpr::local_get(
                    IntLocalId(0),
                    "capture".into(),
                )))],
                ReturnExpr::generic_body(
                    parameter,
                    crate::plan::GenericReturn::expr(GenericExpr::local_get(
                        target_local,
                        "value".into(),
                    )),
                ),
            )
        };
        let panic = |message: &str| {
            PanicExpr::panic_at(
                Some(StringExpr::value(message.into())),
                PanicSite::unknown(),
            )
        };
        let fallback = || ModuleGenericFunctionExpr::panic(panic("fallback"), type_.clone());
        let expressions = [
            (
                ModuleGenericFunctionExpr::closure(
                    target_instantiation,
                    vec![ParamSlot::new(
                        ParamLocal::generic(target_local),
                        ValueShape::Parameter(parameter),
                    )],
                    vec![CaptureArg::int(
                        IntLocalId(0),
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
                        vec![ValueType::Function(Box::new(generic_public_function_type(
                            parameter,
                        )))],
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
                        ValueType::Function(Box::new(generic_public_function_type(parameter))),
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
                .expect("matching generic function branches should plan"),
                "bool subject",
            ),
            (
                ModuleGenericFunctionExpr::bool_case(
                    BoolExpr::not(BoolExpr::value(false)),
                    ModuleGenericFunctionExpr::panic(panic("true branch"), type_.clone()),
                    fallback(),
                )
                .expect("matching generic function branches should plan"),
                "true branch",
            ),
            (
                ModuleGenericFunctionExpr::bool_case(
                    BoolExpr::not(BoolExpr::value(true)),
                    fallback(),
                    ModuleGenericFunctionExpr::panic(panic("false branch"), type_.clone()),
                )
                .expect("matching generic function branches should plan"),
                "false branch",
            ),
            (
                ModuleGenericFunctionExpr::int_case(
                    IntExpr::panic(panic("int subject")),
                    Vec::new(),
                    fallback(),
                )
                .expect("matching generic function branches should plan"),
                "int subject",
            ),
            (
                ModuleGenericFunctionExpr::string_case(
                    StringExpr::panic(panic("string subject")),
                    Vec::new(),
                    fallback(),
                )
                .expect("matching generic function branches should plan"),
                "string subject",
            ),
            (
                ModuleGenericFunctionExpr::float_case(
                    FloatExpr::panic(panic("float subject")),
                    Vec::new(),
                    fallback(),
                )
                .expect("matching generic function branches should plan"),
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
                crate::run_main(&generic_function_plan(expression, vec![target()])),
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
    fn generic_function_custom_field_propagates_source_error() {
        let source = r#"
pub type Box(value) {
  Box(value: fn(value) -> value)
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
    fn generic_function_tuple_projection_reports_direct_mutated_family_mismatch() {
        let parameter = TypeParameterId(0);
        let type_ = generic_function_type(parameter);
        let expected = ValueType::Function(Box::new(generic_public_function_type(parameter)));
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
            crate::run_main(&generic_function_plan(expression, vec![target])),
            Err(ExecutionError::Invariant(
                InvariantError::TupleIndexFamilyMismatch {
                    expected: expected.clone(),
                    actual: ValueType::Function(Box::new(wrong_type)),
                }
            )),
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
            crate::run_main(&generic_function_plan(expression, Vec::new())),
            Err(ExecutionError::Invariant(
                InvariantError::TupleIndexFamilyMismatch {
                    expected,
                    actual: ValueType::Int,
                }
            )),
        );
    }

    #[test]
    fn generic_function_list_projection_reports_direct_mutated_return_family() {
        let parameter = TypeParameterId(0);
        let type_ = generic_function_type(parameter);
        let item_type = generic_public_function_type(parameter);
        let expression = ModuleGenericFunctionExpr::list_index(
            FunctionListExpr::local_get(
                FunctionListItem::new(item_type),
                FunctionListLocalId(0),
                "functions".into(),
            ),
            0,
            type_,
        );
        let plan = generic_function_plan(expression, Vec::new());
        let function_id = main_generic_function_id(&plan);
        let function = plan.generic_function_function(&function_id);
        let expression = expression_return(function.return_().body());
        let list = expect_function_list_projection(expression);
        let local = expect_function_list_local(list);
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
        let mut frame = Frame::new(function.frame_layout(), &mut state);
        frame.set_function_list(local, functions);

        assert_eq!(
            eval_generic_function_expr(&plan, &mut state, &mut frame, expression),
            Err(ExecutionError::Invariant(
                InvariantError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::Generic,
                    actual: FunctionReturnFamily::Int,
                }
            )),
        );
    }

    #[test]
    fn generic_function_custom_field_reports_direct_mutated_family_mismatch() {
        let plan = generic_custom_field_execution_plan();
        let main_id = main_generic_function_id(&plan);
        let main = plan.generic_function_function(&main_id);
        let (function, args) = expect_tail_call(main.return_().body());
        let binding = expect_custom_argument(&args[0]);
        let construction = expect_custom_construction(binding.value());
        let target = plan.generic_function_function(function);
        let expression = expression_return(target.return_().body());
        let access = expect_generic_custom_field(expression);
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
            eval_generic_function_expr(&plan, &mut state, &mut frame, expression),
            Err(ExecutionError::Invariant(
                InvariantError::CustomFieldFamilyMismatch {
                    custom_type: plan.custom_value_type(construction.constructor().type_id()),
                    constructor: descriptor.name().clone(),
                    field_index: access.index(),
                    expected: ValueType::Function(Box::new(generic_public_function_type(
                        TypeParameterId(0),
                    ))),
                    actual: ValueType::Function(Box::new(plan.function_type(&wrong_type))),
                }
            )),
        );

        let value = EvaluatedCustomValue::from_fields(
            construction.constructor(),
            vec![EvaluatedValue::Int(1.into())].into_boxed_slice(),
        );
        let mut frame = Frame::new(target.frame_layout(), &mut state);
        frame.set_custom(binding.local(), value);

        assert_eq!(
            eval_generic_function_expr(&plan, &mut state, &mut frame, expression),
            Err(ExecutionError::Invariant(
                InvariantError::CustomFieldFamilyMismatch {
                    custom_type: plan.custom_value_type(construction.constructor().type_id()),
                    constructor: descriptor.name().clone(),
                    field_index: access.index(),
                    expected: ValueType::Function(Box::new(generic_public_function_type(
                        TypeParameterId(0),
                    ))),
                    actual: ValueType::Int,
                }
            )),
        );
    }

    fn generic_function_type(parameter: TypeParameterId) -> GenericFunctionType {
        GenericFunctionType::new(vec![ValueShape::Parameter(parameter)], parameter)
    }

    fn generic_public_function_type(parameter: TypeParameterId) -> FunctionType {
        FunctionType::new(
            vec![ValueType::Parameter(parameter)],
            ValueType::Parameter(parameter),
        )
    }

    #[test]
    #[should_panic(expected = "expected a generic-function main")]
    fn generic_function_main_fixture_guard_rejects_int_main() {
        let plan = crate::runtime::plan_src("pub fn main() { 1 }");
        let _ = main_generic_function_id(&plan);
    }

    #[test]
    #[should_panic(expected = "expected an expression return body")]
    fn expression_return_fixture_guard_rejects_tail_call() {
        let plan = generic_custom_field_execution_plan();
        let main = plan.generic_function_function(&main_generic_function_id(&plan));
        let _ = expression_return(main.return_().body());
    }

    #[test]
    #[should_panic(expected = "expected a function-list projection")]
    fn function_list_projection_fixture_guard_rejects_closure() {
        let plan = crate::runtime::plan_src("pub fn main() { fn(value) { value } }");
        let main = plan.generic_function_function(&main_generic_function_id(&plan));
        let _ = expect_function_list_projection(expression_return(main.return_().body()));
    }

    #[test]
    #[should_panic(expected = "expected a function-list local")]
    fn function_list_local_fixture_guard_rejects_value() {
        let value_plan =
            crate::runtime::plan_src("fn identity(value) { value }\npub fn main() { [identity] }");
        let value_function =
            value_plan.function_list_function(value_plan.function_list_function_id(0));
        let value = expression_return(value_function.return_());

        let _ = expect_function_list_local(value);
    }

    #[test]
    #[should_panic(expected = "expected a tail-call return body")]
    fn tail_call_fixture_guard_rejects_expression_return() {
        let plan = crate::runtime::plan_src("pub fn main() { fn(value) { value } }");
        let main = plan.generic_function_function(&main_generic_function_id(&plan));
        let _ = expect_tail_call(main.return_().body());
    }

    #[test]
    #[should_panic(expected = "expected a custom call argument")]
    fn custom_argument_fixture_guard_rejects_int_argument() {
        let plan = crate::runtime::plan_src(
            "fn target(_value: Int) { fn(value) { value } }\npub fn main() { target(1) }",
        );
        let main = plan.generic_function_function(&main_generic_function_id(&plan));
        let (_, args) = expect_tail_call(main.return_().body());
        let _ = expect_custom_argument(&args[0]);
    }

    #[test]
    #[should_panic(expected = "expected a custom construction")]
    fn custom_construction_fixture_guard_rejects_local_get() {
        let plan = generic_custom_field_execution_plan();
        let main = plan.generic_function_function(&main_generic_function_id(&plan));
        let (function, _) = expect_tail_call(main.return_().body());
        let target = plan.generic_function_function(function);
        let access = expect_generic_custom_field(expression_return(target.return_().body()));
        let _ = expect_custom_construction(access.source());
    }

    #[test]
    #[should_panic(expected = "expected a generic custom-field projection")]
    fn custom_field_fixture_guard_rejects_closure() {
        let plan = crate::runtime::plan_src("pub fn main() { fn(value) { value } }");
        let main = plan.generic_function_function(&main_generic_function_id(&plan));
        let _ = expect_generic_custom_field(expression_return(main.return_().body()));
    }

    fn generic_function_plan(
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

    fn generic_custom_field_execution_plan() -> crate::ExecutionPlan {
        crate::runtime::plan_src(
            r#"
pub type Box(value) {
  Box(value: fn(value) -> value)
}

fn identity(value: value) {
  value
}

fn get(box: Box(value)) {
  box.value
}

pub fn main() {
  get(Box(identity))
}
"#,
        )
    }

    fn main_generic_function_id(plan: &crate::ExecutionPlan) -> GenericFunctionFunctionId {
        match plan.main_runtime() {
            RuntimeFunctionId::Function {
                id: FunctionFunctionId::Generic(id),
                ..
            } => id,
            _ => panic!("expected a generic-function main"),
        }
    }

    fn expression_return<Expression, Function>(
        graph: &ReturnGraph<Expression, Function>,
    ) -> &Expression {
        match graph.block(graph.entry()) {
            ReturnBlock::Return(expression) => expression,
            _ => panic!("expected an expression return body"),
        }
    }

    fn expect_function_list_projection(
        expression: &GenericFunctionExpr,
    ) -> &crate::plan::execution::FunctionListExpr {
        match expression.kind() {
            GenericFunctionExprKind::ListIndex { list, .. } => list,
            _ => panic!("expected a function-list projection"),
        }
    }

    fn expect_function_list_local(
        list: &crate::plan::execution::FunctionListExpr,
    ) -> crate::plan::execution::FunctionListLocalId {
        match list.kind() {
            TypedListExprKind::LocalGet { local } => *local,
            _ => panic!("expected a function-list local"),
        }
    }

    fn expect_tail_call(
        graph: &ReturnGraph<GenericFunctionExpr, GenericFunctionFunctionId>,
    ) -> (&GenericFunctionFunctionId, &[CallArg]) {
        match graph.block(graph.entry()) {
            ReturnBlock::TailCall { function, args } => (function, args),
            _ => panic!("expected a tail-call return body"),
        }
    }

    fn expect_custom_argument(argument: &CallArg) -> &CustomLocalExpr {
        match argument.kind() {
            CallArgKind::Custom(binding) => binding,
            _ => panic!("expected a custom call argument"),
        }
    }

    fn expect_custom_construction(expression: &CustomExpr) -> &CustomConstruction {
        match expression.kind() {
            CustomExprKind::Constructor(construction) => construction,
            _ => panic!("expected a custom construction"),
        }
    }

    fn expect_generic_custom_field(expression: &GenericFunctionExpr) -> &CustomFieldAccess {
        match expression.kind() {
            GenericFunctionExprKind::CustomField(access) => access,
            _ => panic!("expected a generic custom-field projection"),
        }
    }
}
