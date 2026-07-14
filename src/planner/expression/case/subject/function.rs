use super::super::super::plan_expr_with_expected_source_stop_type;
use super::super::invalid_case_shape;
use super::{CaseClause, OrderedCaseClauseInput, case_return_type};
use crate::plan::{
    BitArrayFunctionExpr, BoolExpr, CustomFunctionExpr, Expr, ExprKind, FunctionExpr,
    FunctionExprKind, FunctionFunctionExpr, FunctionFunctionLocalId, FunctionType, IntFunctionExpr,
    IntFunctionLocalId, Step, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{InvalidCaseShapeReason, PlanError};
use ecow::EcoString;
use gleam_core::ast::{Pattern, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    subject: TypedExpr,
    subject_type: FunctionType,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject_value_type = ValueType::Function(Box::new(subject_type));
    let subject =
        plan_expr_with_expected_source_stop_type(subject, subject_value_type.clone(), context)?;
    let return_type = case_return_type(type_.as_ref())?;

    let ExprKind::Function(subject) = subject.into_kind() else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    };
    let (subject_step, subject) = bind_function_case_subject(subject, context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let pattern = plan_function_case_pattern(pattern, &subject_value_type)?;
            let bindings = super::branch_bindings(pattern.bound_names(), subject.clone());
            let is_total = clause.guard.is_none();
            ordered_clauses.push(super::plan_ordered_case_clause(
                OrderedCaseClauseInput {
                    case_type: type_.as_ref(),
                    return_type: &return_type,
                    then: clause.then.clone(),
                    branch_bindings: bindings,
                    guard: clause.guard.clone(),
                    match_condition: BoolExpr::value(true),
                    is_total,
                },
                context,
            )?);
        }
    }

    super::ordered_case_expr(ordered_clauses)
        .map(|case| super::case_subject_block(subject_step, case))
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct FunctionCasePattern {
    bound_names: Vec<EcoString>,
}

impl FunctionCasePattern {
    fn bound_names(&self) -> &[EcoString] {
        &self.bound_names
    }

    fn add_bound_name(&mut self, name: EcoString) {
        self.bound_names.push(name);
    }
}

fn plan_function_case_pattern(
    pattern: Pattern<Arc<Type>>,
    subject_type: &ValueType,
) -> Result<FunctionCasePattern, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. } if matches_type(type_.as_ref(), subject_type) => {
            Ok(FunctionCasePattern {
                bound_names: vec![name],
            })
        }
        Pattern::Variable { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Discard { type_, .. } if matches_type(type_.as_ref(), subject_type) => {
            Ok(FunctionCasePattern {
                bound_names: Vec::new(),
            })
        }
        Pattern::Discard { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
        Pattern::Assign { name, pattern, .. } => {
            let mut pattern = plan_function_case_pattern(*pattern, subject_type)?;
            pattern.add_bound_name(name);
            Ok(pattern)
        }
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. } => Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        )),
    }
}

fn matches_type(type_: &Type, subject_type: &ValueType) -> bool {
    ValueType::from_gleam(type_) == Some(subject_type.clone())
}

fn bind_function_case_subject(
    subject: FunctionExpr,
    context: &mut PlanContext<'_>,
) -> (Step, Expr) {
    match subject.into_kind() {
        FunctionExprKind::Int(subject) => {
            let local = context.define_internal_int_function_local();
            let name = internal_int_function_case_subject_name(local);
            let type_ = subject.type_().clone();
            (
                Step::let_int_function(local, name.clone(), subject),
                Expr::function(FunctionExpr::int(IntFunctionExpr::local_get(
                    local, name, type_,
                ))),
            )
        }
        FunctionExprKind::String(subject) => {
            let local = context.define_internal_string_function_local();
            let name = internal_string_function_case_subject_name(local);
            let type_ = subject.type_().clone();
            (
                Step::let_string_function(local, name.clone(), subject),
                Expr::function(FunctionExpr::string(
                    crate::plan::StringFunctionExpr::local_get(local, name, type_),
                )),
            )
        }
        FunctionExprKind::BitArray(subject) => {
            let local = context.define_internal_bit_array_function_local();
            let name = internal_bit_array_function_case_subject_name(local);
            let type_ = subject.type_().clone();
            (
                Step::let_bit_array_function(local, name.clone(), subject),
                Expr::function(FunctionExpr::bit_array(BitArrayFunctionExpr::local_get(
                    local, name, type_,
                ))),
            )
        }
        FunctionExprKind::UtfCodepoint(subject) => {
            let local = context.define_internal_utf_codepoint_function_local();
            let name = internal_utf_codepoint_function_case_subject_name(local);
            let type_ = subject.type_().clone();
            (
                Step::let_utf_codepoint_function(local, name.clone(), subject),
                Expr::function(FunctionExpr::utf_codepoint(
                    crate::plan::UtfCodepointFunctionExpr::local_get(local, name, type_),
                )),
            )
        }
        FunctionExprKind::Custom(subject) => {
            let local = context.define_internal_custom_function_local();
            let name = internal_custom_function_case_subject_name(local);
            let type_ = subject.type_().clone();
            (
                Step::let_custom_function(local, name.clone(), subject),
                Expr::function(FunctionExpr::custom(CustomFunctionExpr::local_get(
                    local, name, type_,
                ))),
            )
        }
        FunctionExprKind::Float(subject) => {
            let local = context.define_internal_float_function_local();
            let name = internal_float_function_case_subject_name(local);
            let type_ = subject.type_().clone();
            (
                Step::let_float_function(local, name.clone(), subject),
                Expr::function(FunctionExpr::float(
                    crate::plan::FloatFunctionExpr::local_get(local, name, type_),
                )),
            )
        }
        FunctionExprKind::Bool(subject) => {
            let local = context.define_internal_bool_function_local();
            let name = internal_bool_function_case_subject_name(local);
            let type_ = subject.type_().clone();
            (
                Step::let_bool_function(local, name.clone(), subject),
                Expr::function(FunctionExpr::bool(
                    crate::plan::BoolFunctionExpr::local_get(local, name, type_),
                )),
            )
        }
        FunctionExprKind::Nil(subject) => {
            let local = context.define_internal_nil_function_local();
            let name = internal_nil_function_case_subject_name(local);
            let type_ = subject.type_().clone();
            (
                Step::let_nil_function(local, name.clone(), subject),
                Expr::function(FunctionExpr::nil(crate::plan::NilFunctionExpr::local_get(
                    local, name, type_,
                ))),
            )
        }
        FunctionExprKind::Tuple(subject) => {
            let local = context.define_internal_tuple_function_local();
            let name = internal_tuple_function_case_subject_name(local);
            let type_ = subject.type_().clone();
            (
                Step::let_tuple_function(local, name.clone(), subject),
                Expr::function(FunctionExpr::tuple(
                    crate::plan::TupleFunctionExpr::local_get(local, name, type_),
                )),
            )
        }
        FunctionExprKind::List(subject) => {
            let type_ = subject.type_().clone();
            let local =
                context.define_internal_list_function_local(type_, subject.return_item_type());
            let name = internal_list_function_case_subject_name(&local);
            (
                Step::let_list_function(local.clone(), name.clone(), subject),
                Expr::function(FunctionExpr::list(
                    crate::plan::ListFunctionExpr::local_get(local, name),
                )),
            )
        }
        FunctionExprKind::Function(subject) => {
            let local = context.define_internal_function_function_local();
            let name = internal_function_function_case_subject_name(local);
            let type_ = subject.type_().clone();
            (
                Step::let_function_function(local, name.clone(), subject),
                Expr::function(FunctionExpr::function(FunctionFunctionExpr::local_get(
                    local, name, type_,
                ))),
            )
        }
    }
}

fn internal_int_function_case_subject_name(local: IntFunctionLocalId) -> EcoString {
    format!("<case:int_function:{}>", local.0).into()
}

fn internal_string_function_case_subject_name(
    local: crate::plan::StringFunctionLocalId,
) -> EcoString {
    format!("<case:string_function:{}>", local.0).into()
}

fn internal_bit_array_function_case_subject_name(
    local: crate::plan::BitArrayFunctionLocalId,
) -> EcoString {
    format!("<case:bit_array_function:{}>", local.0).into()
}

fn internal_utf_codepoint_function_case_subject_name(
    local: crate::plan::UtfCodepointFunctionLocalId,
) -> EcoString {
    format!("<case:utf_codepoint_function:{}>", local.0).into()
}

fn internal_custom_function_case_subject_name(
    local: crate::plan::CustomFunctionLocalId,
) -> EcoString {
    format!("<case:custom_function:{}>", local.0).into()
}

fn internal_float_function_case_subject_name(
    local: crate::plan::FloatFunctionLocalId,
) -> EcoString {
    format!("<case:float_function:{}>", local.0).into()
}

fn internal_bool_function_case_subject_name(local: crate::plan::BoolFunctionLocalId) -> EcoString {
    format!("<case:bool_function:{}>", local.0).into()
}

fn internal_nil_function_case_subject_name(local: crate::plan::NilFunctionLocalId) -> EcoString {
    format!("<case:nil_function:{}>", local.0).into()
}

fn internal_tuple_function_case_subject_name(
    local: crate::plan::TupleFunctionLocalId,
) -> EcoString {
    format!("<case:tuple_function:{}>", local.0).into()
}

fn internal_list_function_case_subject_name(local: &crate::plan::ListFunctionLocal) -> EcoString {
    format!("<case:list_function:{}>", local.index()).into()
}

fn internal_function_function_case_subject_name(local: FunctionFunctionLocalId) -> EcoString {
    format!("<case:function_function:{}>", local.0).into()
}

#[cfg(test)]
mod tests {
    use super::bind_function_case_subject;
    use crate::plan::{
        BitArrayFunctionExpr, BitArrayFunctionLocalId, CustomFunctionExpr, CustomFunctionLocalId,
        CustomType, CustomTypeName, Expr, FunctionExpr, FunctionFunctionExpr,
        FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionLocalId, FunctionType,
        IntLocalId, LocalId, Step, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, PlanContext};
    use crate::planner::dsl::{
        bit_array_function_ref, bool_function_ref, call_int_function, float_function_ref, function,
        function_function_ref, int, int_function_call_arg, int_function_ref, int_return_block,
        int_return_expr, let_bool_function_step, let_int_function_step, let_nil_function_step,
        let_string_function_step, list_function_ref, local_int, local_int_function, module,
        nil_function_ref, string_function_ref, tuple_function_ref, utf_codepoint_function_ref,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{dummy_span, expect_plan_error};
    use crate::planner::{InvalidCaseShapeReason, InvalidTypedAstReason, PlanError};
    use ecow::EcoString;
    use gleam_core::type_::error::VariableOrigin;
    use std::collections::HashMap;

    #[test]
    fn plan_function_subject_binds_internal_subject_once() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case add_one {
    f -> f(41)
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
                    [let_int_function_step(
                        0,
                        "<case:int_function:0>",
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                    )],
                    int_return_block(
                        [let_int_function_step(
                            1,
                            "f",
                            local_int_function(0, "<case:int_function:0>", [ValueType::Int]),
                        )],
                        int_return_expr(call_int_function(
                            local_int_function(1, "f", [ValueType::Int]),
                            [int_function_call_arg(0, int(41))],
                        )),
                    ),
                ),
            ),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_subject_alias_binds_inner_then_alias_after_single_subject_eval() {
        let actual = plan_module(crate::planner::support::compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case add_one {
    f as alias -> alias(41)
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
                    [let_int_function_step(
                        0,
                        "<case:int_function:0>",
                        int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                    )],
                    int_return_block(
                        [
                            let_int_function_step(
                                1,
                                "f",
                                local_int_function(0, "<case:int_function:0>", [ValueType::Int]),
                            ),
                            let_int_function_step(
                                2,
                                "alias",
                                local_int_function(0, "<case:int_function:0>", [ValueType::Int]),
                            ),
                        ],
                        int_return_expr(call_int_function(
                            local_int_function(2, "alias", [ValueType::Int]),
                            [int_function_call_arg(0, int(41))],
                        )),
                    ),
                ),
            ),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn bind_function_case_subject_preserves_return_family_and_internal_name() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let int_param = LocalId::Int(IntLocalId(0));
        let empty_function_type = FunctionType::new(Vec::new(), ValueType::Int);

        assert_eq!(
            bind_function_case_subject(int_function_ref(0, [int_param]).into(), &mut context),
            (
                let_int_function_step(0, "<case:int_function:0>", int_function_ref(0, [int_param])),
                Expr::from(local_int_function(
                    0,
                    "<case:int_function:0>",
                    [ValueType::Int],
                )),
            ),
        );
        assert_eq!(
            bind_function_case_subject(
                Expr::from(string_function_ref(0, Vec::<LocalId>::new()))
                    .into_function()
                    .expect("string function expr"),
                &mut context,
            ),
            (
                let_string_function_step(
                    0,
                    "<case:string_function:0>",
                    string_function_ref(0, Vec::<LocalId>::new()),
                ),
                Expr::function(FunctionExpr::string(
                    crate::plan::StringFunctionExpr::local_get(
                        crate::plan::StringFunctionLocalId(0),
                        "<case:string_function:0>".into(),
                        FunctionType::new(Vec::new(), ValueType::String),
                    ),
                )),
            ),
        );
        assert_eq!(
            bind_function_case_subject(
                bit_array_function_ref(0, Vec::<LocalId>::new()).into(),
                &mut context,
            ),
            (
                Step::let_bit_array_function(
                    BitArrayFunctionLocalId(0),
                    "<case:bit_array_function:0>".into(),
                    bit_array_function_ref(0, Vec::<LocalId>::new()).into(),
                ),
                Expr::function(FunctionExpr::bit_array(BitArrayFunctionExpr::local_get(
                    BitArrayFunctionLocalId(0),
                    "<case:bit_array_function:0>".into(),
                    FunctionType::new(Vec::new(), ValueType::BitArray),
                ))),
            ),
        );
        assert_eq!(
            bind_function_case_subject(
                utf_codepoint_function_ref(0, Vec::<LocalId>::new()).into(),
                &mut context,
            ),
            (
                Step::let_utf_codepoint_function(
                    crate::plan::UtfCodepointFunctionLocalId(0),
                    "<case:utf_codepoint_function:0>".into(),
                    utf_codepoint_function_ref(0, Vec::<LocalId>::new()).into(),
                ),
                Expr::function(FunctionExpr::utf_codepoint(
                    crate::plan::UtfCodepointFunctionExpr::local_get(
                        crate::plan::UtfCodepointFunctionLocalId(0),
                        "<case:utf_codepoint_function:0>".into(),
                        FunctionType::new(Vec::new(), ValueType::UtfCodepoint),
                    ),
                )),
            ),
        );
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        let custom_function_type =
            FunctionType::new(Vec::new(), ValueType::Custom(custom_type.clone()));
        let custom_subject = CustomFunctionExpr::local_get(
            CustomFunctionLocalId(7),
            "source".into(),
            custom_function_type.clone(),
        );
        assert_eq!(
            bind_function_case_subject(FunctionExpr::custom(custom_subject.clone()), &mut context,),
            (
                Step::let_custom_function(
                    CustomFunctionLocalId(0),
                    "<case:custom_function:0>".into(),
                    custom_subject,
                ),
                Expr::function(FunctionExpr::custom(CustomFunctionExpr::local_get(
                    CustomFunctionLocalId(0),
                    "<case:custom_function:0>".into(),
                    custom_function_type,
                ))),
            ),
        );
        assert_eq!(
            bind_function_case_subject(
                float_function_ref(0, Vec::<LocalId>::new()).into(),
                &mut context,
            ),
            (
                Step::let_float_function(
                    crate::plan::FloatFunctionLocalId(0),
                    "<case:float_function:0>".into(),
                    float_function_ref(0, Vec::<LocalId>::new()).into(),
                ),
                Expr::function(FunctionExpr::float(
                    crate::plan::FloatFunctionExpr::local_get(
                        crate::plan::FloatFunctionLocalId(0),
                        "<case:float_function:0>".into(),
                        FunctionType::new(Vec::new(), ValueType::Float),
                    ),
                )),
            ),
        );
        assert_eq!(
            bind_function_case_subject(
                Expr::from(bool_function_ref(0, Vec::<LocalId>::new()))
                    .into_function()
                    .expect("bool function expr"),
                &mut context,
            ),
            (
                let_bool_function_step(
                    0,
                    "<case:bool_function:0>",
                    bool_function_ref(0, Vec::<LocalId>::new()),
                ),
                Expr::function(FunctionExpr::bool(
                    crate::plan::BoolFunctionExpr::local_get(
                        crate::plan::BoolFunctionLocalId(0),
                        "<case:bool_function:0>".into(),
                        FunctionType::new(Vec::new(), ValueType::Bool),
                    ),
                )),
            ),
        );
        assert_eq!(
            bind_function_case_subject(
                Expr::from(nil_function_ref(0, Vec::<LocalId>::new()))
                    .into_function()
                    .expect("nil function expr"),
                &mut context,
            ),
            (
                let_nil_function_step(
                    0,
                    "<case:nil_function:0>",
                    nil_function_ref(0, Vec::<LocalId>::new()),
                ),
                Expr::function(FunctionExpr::nil(crate::plan::NilFunctionExpr::local_get(
                    crate::plan::NilFunctionLocalId(0),
                    "<case:nil_function:0>".into(),
                    FunctionType::new(Vec::new(), ValueType::Nil),
                ))),
            ),
        );
        assert_eq!(
            bind_function_case_subject(
                tuple_function_ref(0, Vec::<LocalId>::new(), [ValueType::Int]).into(),
                &mut context,
            ),
            (
                Step::let_tuple_function(
                    crate::plan::TupleFunctionLocalId(0),
                    "<case:tuple_function:0>".into(),
                    tuple_function_ref(0, Vec::<LocalId>::new(), [ValueType::Int]).into(),
                ),
                Expr::function(FunctionExpr::tuple(
                    crate::plan::TupleFunctionExpr::local_get(
                        crate::plan::TupleFunctionLocalId(0),
                        "<case:tuple_function:0>".into(),
                        FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int])),
                    ),
                )),
            ),
        );
        assert_eq!(
            bind_function_case_subject(
                list_function_ref(0, Vec::<LocalId>::new(), ValueType::Int).into(),
                &mut context,
            ),
            (
                Step::let_list_function(
                    crate::plan::ListFunctionLocal::from_item_type(
                        0,
                        crate::plan::FunctionType::new(
                            Vec::new(),
                            crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                        ),
                        crate::plan::ValueType::Int,
                    ),
                    "<case:list_function:0>".into(),
                    list_function_ref(0, Vec::<LocalId>::new(), ValueType::Int).into(),
                ),
                Expr::function(FunctionExpr::list(
                    crate::plan::ListFunctionExpr::local_get(
                        crate::plan::ListFunctionLocal::from_item_type(
                            0,
                            crate::plan::FunctionType::new(
                                Vec::new(),
                                crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                            ),
                            crate::plan::ValueType::Int,
                        ),
                        "<case:list_function:0>".into()
                    ),
                )),
            ),
        );
        let function_function_type = FunctionType::new(
            Vec::new(),
            ValueType::Function(Box::new(empty_function_type.clone())),
        );
        assert_eq!(
            bind_function_case_subject(
                function_function_ref(
                    FunctionFunctionId::Function(FunctionFunctionFunctionId(0)),
                    Vec::<LocalId>::new(),
                    empty_function_type.clone(),
                )
                .into(),
                &mut context,
            ),
            (
                Step::let_function_function(
                    FunctionFunctionLocalId(0),
                    "<case:function_function:0>".into(),
                    function_function_ref(
                        FunctionFunctionId::Function(FunctionFunctionFunctionId(0)),
                        Vec::<LocalId>::new(),
                        empty_function_type,
                    )
                    .into(),
                ),
                Expr::function(FunctionExpr::function(FunctionFunctionExpr::local_get(
                    FunctionFunctionLocalId(0),
                    "<case:function_function:0>".into(),
                    function_function_type,
                ))),
            ),
        );
    }

    #[test]
    fn reject_profile_function_subject_expression_errors_before_case_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case echo add_one {
    _ -> 0
  }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: crate::planner::UnsupportedExpressionKind::Echo,
            },
        );
    }

    #[test]
    fn reject_profile_function_subject_branch_errors_during_clause_lowering() {
        assert_eq!(
            expect_plan_error(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case add_one {
    _ -> echo 0
  }
}
"#,
            ),
            PlanError::UnsupportedExpression {
                kind: crate::planner::UnsupportedExpressionKind::Echo,
            },
        );
    }

    #[test]
    fn reject_margin_function_subject_case_shapes() {
        let mut unsupported_case_type = crate::planner::support::compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case add_one {
    _ -> 1
  }
}
"#,
        );
        let (case_type, _, _) = super::super::super::expect_case_statement_mut(
            &mut unsupported_case_type.definitions.functions[1].body[0],
        );
        *case_type = super::super::invalid_case_return_type();
        assert_eq!(
            plan_module(unsupported_case_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::BranchReturnTypeMismatch,
                },
            }),
        );

        let mut empty_pattern = crate::planner::support::compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case add_one {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut empty_pattern.definitions.functions[1].body[0],
        );
        clauses[0].pattern.clear();
        assert_eq!(
            plan_module(empty_pattern),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternSubjectCountMismatch,
                },
            }),
        );

        let mut pattern_type_mismatch = crate::planner::support::compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case add_one {
    _ -> 1
  }
}
"#,
        );
        let (_, _, clauses) = super::super::super::expect_case_statement_mut(
            &mut pattern_type_mismatch.definitions.functions[1].body[0],
        );
        clauses[0].pattern[0] = gleam_core::ast::Pattern::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: num_bigint::BigInt::from(1),
        };
        assert_eq!(
            plan_module(pattern_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );

        let mut subject_expression_family_mismatch = crate::planner::support::compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  case add_one {
    _ -> 1
  }
}
"#,
        );
        let (_, subjects, _) = super::super::super::expect_case_statement_mut(
            &mut subject_expression_family_mismatch.definitions.functions[1].body[0],
        );
        subjects[0] = gleam_core::ast::TypedExpr::Int {
            location: dummy_span(),
            type_: gleam_core::type_::fn_(vec![gleam_core::type_::int()], gleam_core::type_::int()),
            value: "1".into(),
            int_value: num_bigint::BigInt::from(1),
        };
        assert_eq!(
            plan_module(subject_expression_family_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_function_case_pattern_mismatched_and_invalid_shapes() {
        let function_type = ValueType::Function(Box::new(FunctionType::new(
            vec![ValueType::Int],
            ValueType::Int,
        )));
        assert_eq!(
            super::plan_function_case_pattern(
                gleam_core::ast::Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: gleam_core::type_::int(),
                    origin: VariableOrigin::generated(),
                },
                &function_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_function_case_pattern(
                gleam_core::ast::Pattern::Assign {
                    location: dummy_span(),
                    name: "alias".into(),
                    pattern: Box::new(gleam_core::ast::Pattern::Variable {
                        location: dummy_span(),
                        name: "value".into(),
                        type_: gleam_core::type_::int(),
                        origin: VariableOrigin::generated(),
                    }),
                },
                &function_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_function_case_pattern(
                gleam_core::ast::Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: gleam_core::type_::int(),
                },
                &function_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_function_case_pattern(
                gleam_core::ast::Pattern::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: num_bigint::BigInt::from(1),
                },
                &function_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan_function_case_pattern(
                gleam_core::ast::Pattern::Invalid {
                    location: dummy_span(),
                    type_: gleam_core::type_::fn_(
                        vec![gleam_core::type_::int()],
                        gleam_core::type_::int(),
                    ),
                },
                &function_type,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );
    }
}
