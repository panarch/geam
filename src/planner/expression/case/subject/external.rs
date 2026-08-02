use super::super::super::plan_expr_with_expected_source_stop_shape;
use super::super::invalid_case_shape;
use super::{CaseClause, OrderedCaseClauseInput};
use crate::plan::{
    BoolExpr, Expr, ExprKind, ExternalExpr, ExternalLocal, ExternalValueShape, Step,
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
    shape: ExternalValueShape,
    clauses: Vec<CaseClause>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let subject = plan_expr_with_expected_source_stop_shape(
        subject,
        crate::plan::ValueShape::External(shape.clone()),
        context,
    )?;
    let return_shape = context.value_shape(type_.as_ref());
    let ExprKind::External(subject) = subject.into_kind() else {
        return Err(invalid_case_shape(
            InvalidCaseShapeReason::PatternTypeMismatch,
        ));
    };
    let (subject_step, subject) = bind_subject(subject, shape.clone(), context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let (pattern, reachable, exhaustive_remainder) = pattern.into_parts();
            let bound_names = plan_pattern(pattern, &shape, context)?;
            ordered_clauses.push(super::plan_ordered_case_clause(
                OrderedCaseClauseInput {
                    case_type: type_.as_ref(),
                    return_shape: &return_shape,
                    then: clause.then.clone(),
                    branch_bindings: super::branch_bindings(&bound_names, subject.clone()),
                    guard: clause.guard.clone(),
                    match_condition: BoolExpr::value(true),
                    is_total: clause.guard.is_none(),
                    reachable,
                    exhaustive_remainder,
                },
                context,
            )?);
        }
    }

    let case = super::ordered_case_expr(ordered_clauses)?;
    Ok(super::case_subject_block(subject_step, case))
}

fn plan_pattern(
    pattern: Pattern<Arc<Type>>,
    shape: &ExternalValueShape,
    context: &mut PlanContext<'_>,
) -> Result<Vec<EcoString>, PlanError> {
    match pattern {
        Pattern::Variable { name, type_, .. }
            if context.value_shape(type_.as_ref())
                == crate::plan::ValueShape::External(shape.clone()) =>
        {
            Ok(vec![name])
        }
        Pattern::Discard { type_, .. }
            if context.value_shape(type_.as_ref())
                == crate::plan::ValueShape::External(shape.clone()) =>
        {
            Ok(Vec::new())
        }
        Pattern::Assign { name, pattern, .. } => {
            let mut names = plan_pattern(*pattern, shape, context)?;
            names.push(name);
            Ok(names)
        }
        Pattern::Invalid { .. } => Err(invalid_case_shape(InvalidCaseShapeReason::InvalidPattern)),
        Pattern::Variable { .. }
        | Pattern::Discard { .. }
        | Pattern::Int { .. }
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

fn bind_subject(
    subject: ExternalExpr,
    shape: ExternalValueShape,
    context: &mut PlanContext<'_>,
) -> (Step, Expr) {
    let local = context.define_internal_external_local();
    let local = ExternalLocal::from_shape(local, shape);
    let name: EcoString = format!("<case:external:{}>", local.id().0).into();
    (
        Step::let_external(local.clone(), name.clone(), subject),
        Expr::external(ExternalExpr::local_get(local, name)),
    )
}

#[cfg(test)]
mod tests {
    use crate::host::{
        ExternalTestProfile, ExternalTestRunState, HostCall, HostCallCompletion, HostCallError,
        HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType, HostProvider,
        HostProviderModule, HostProviderSet,
    };
    use crate::plan::{ExternalTypeName, ExternalValueShape};
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::support::dummy_span;
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError,
    };
    use crate::{ModuleSource, PackageSource};
    use ecow::EcoString;
    use gleam_core::ast::{Pattern, TypedExpr};
    use gleam_core::type_::{self, error::VariableOrigin};
    use num_bigint::BigInt;
    use std::collections::HashMap;

    struct TokenSchema;

    struct TokenProvider;

    type HostToken = HostExternalType<TokenSchema>;

    impl HostExternalSchema for TokenSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Token";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalStorage<TokenSchema> for ExternalTestProfile {
        type Payload = ();

        fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
            &stores.units
        }

        fn equal(_: &Self::Payload, _: &Self::Payload) -> bool {
            true
        }

        fn inspect(_: &Self::Payload) -> EcoString {
            "Token".into()
        }
    }

    impl HostProvider<ExternalTestProfile> for TokenProvider {
        type State = ();

        fn project(state: &mut ExternalTestRunState) -> &mut Self::State {
            &mut state.provider
        }
    }

    fn new_token<'call>(
        mut call: HostCall<'call, ExternalTestProfile, TokenProvider, HostToken>,
    ) -> Result<HostCallCompletion<'call, HostToken>, HostCallError> {
        let _ = call.state();
        let token = call.create_external(());
        Ok(call.return_value(token))
    }

    #[test]
    fn external_case_rejects_malformed_subject_and_pattern_shapes() {
        let module = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module, &functions, &mut anonymous);
        let shape = ExternalValueShape::new(
            ExternalTypeName::new("application".into(), "main".into(), "Token".into()),
            Vec::new(),
        );

        assert_eq!(
            super::plan(
                type_::int(),
                TypedExpr::Int {
                    location: dummy_span(),
                    type_: type_::int(),
                    value: "1".into(),
                    int_value: BigInt::from(1),
                },
                shape.clone(),
                Vec::new(),
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
        assert_eq!(
            super::plan(
                type_::int(),
                TypedExpr::Invalid {
                    location: dummy_span(),
                    type_: type_::int(),
                    extra_information: None,
                },
                shape.clone(),
                Vec::new(),
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
        assert_eq!(
            super::plan_pattern(
                Pattern::Invalid {
                    location: dummy_span(),
                    type_: type_::int(),
                },
                &shape,
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );
        assert_eq!(
            super::plan(
                type_::int(),
                TypedExpr::Panic {
                    location: dummy_span(),
                    type_: type_::int(),
                    message: None,
                },
                shape.clone(),
                Vec::new(),
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::MissingFallbackPattern,
                },
            }),
        );
        assert_eq!(
            super::plan(
                type_::int(),
                TypedExpr::Panic {
                    location: dummy_span(),
                    type_: type_::int(),
                    message: None,
                },
                shape.clone(),
                vec![super::CaseClause {
                    pattern: Pattern::Assign {
                        location: dummy_span(),
                        name: "selected".into(),
                        pattern: Box::new(Pattern::Invalid {
                            location: dummy_span(),
                            type_: type_::int(),
                        }),
                    },
                    alternative_patterns: Vec::new(),
                    guard: None,
                    reachable: true,
                    exhaustive_remainder: false,
                    then: TypedExpr::Int {
                        location: dummy_span(),
                        type_: type_::int(),
                        value: "1".into(),
                        int_value: BigInt::from(1),
                    },
                }],
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::InvalidPattern,
                },
            }),
        );
        assert_eq!(
            super::plan_pattern(
                Pattern::Variable {
                    location: dummy_span(),
                    name: "value".into(),
                    type_: type_::bool(),
                    origin: VariableOrigin::generated(),
                },
                &shape,
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CaseShape {
                    reason: InvalidCaseShapeReason::PatternTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn external_case_binds_and_returns_its_exact_subject() {
        let source = r#"
@external(erlang, "host", "Token")
pub type Token

@external(erlang, "host", "new_token")
fn new_token() -> Token

fn identity(value: Token) -> Token {
  case value {
    selected -> selected
  }
}

fn discard(value: Token) -> Token {
  case value {
    _ -> value
  }
}

fn alias(value: Token) -> Token {
  case value {
    selected as whole -> {
      let _ = selected
      whole
    }
  }
}

pub fn main() {
  let token = new_token()
  #(
    identity(token),
    discard(token),
    alias(token),
    token == new_token(),
  )
}
"#;
        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_external_type::<TokenSchema>()
            .expect("external type should be valid")
            .with_scoped_function::<TokenProvider, (), HostToken, _>("new_token", new_token)
            .expect("external constructor should be valid");
        let typed = crate::compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new("main", "src/main.gleam", source)],
            )],
            HostProviderSet::with_providers(
                Vec::<crate::HostModule<ExternalTestProfile>>::new(),
                [provider],
            )
            .expect("provider module should be unique"),
        )
        .expect("external source should compile");
        let plan = crate::plan_host_program(typed).expect("external case should plan");
        let execution =
            crate::HostedExecution::try_from_module_plan(plan).expect("external case should seal");
        let returned = execution
            .run_main(&mut ExternalTestRunState::default(), &mut Vec::new())
            .expect("external case should execute");

        assert_eq!(
            returned.inspect().to_string(),
            "#(Token, Token, Token, True)",
        );
    }

    #[test]
    fn external_case_propagates_subject_and_branch_errors() {
        for source in [
            r#"
@external(erlang, "host", "Token")
pub type Token

fn identity(value: Token) -> Token {
  case { <<1:native>> value } {
    selected -> selected
  }
}

pub fn main() { 0 }
"#,
            r#"
@external(erlang, "host", "Token")
pub type Token

fn identity(value: Token) -> Token {
  case value {
    selected -> { <<1:native>> selected }
  }
}

pub fn main() { 0 }
"#,
        ] {
            let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
                .expect("provider module should be valid")
                .with_external_type::<TokenSchema>()
                .expect("external type should be valid");
            let typed = crate::compile_typed_host_program(
                "application",
                "main",
                [PackageSource::new(
                    "application",
                    Vec::<&str>::new(),
                    [ModuleSource::new("main", "src/main.gleam", source)],
                )],
                HostProviderSet::with_providers(
                    Vec::<crate::HostModule<ExternalTestProfile>>::new(),
                    [provider],
                )
                .expect("provider module should be unique"),
            )
            .expect("external source should compile");

            let error = crate::plan_host_program(typed)
                .err()
                .expect("external case should preserve the child planning error");
            assert_eq!(
                error,
                PlanError::UnsupportedBitArraySegment {
                    reason: crate::planner::UnsupportedBitArraySegmentReason::NativeEndianness,
                },
            );
        }
    }
}
