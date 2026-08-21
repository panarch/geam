use super::super::super::{
    conversion::expect_expression, plan_expr_with_expected_source_stop_shape,
};
use super::{CaseClause, OrderedCaseClauseInput};
use crate::plan::{BoolExpr, Expr, ExternalExpr, ExternalLocal, ExternalValueShape, Step};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use ecow::EcoString;
use gleam_compiler_core::ast::{Pattern, TypedExpr};
use gleam_compiler_core::type_::Type;
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
    let subject: ExternalExpr = expect_expression(subject)?;
    let (subject_step, subject) = bind_subject(subject, shape.clone(), context);
    let mut ordered_clauses = Vec::new();
    for clause in clauses {
        for pattern in clause.patterns() {
            let (pattern, reachable, exhaustive_remainder) = pattern.into_parts();
            let bound_names = plan_pattern(pattern, &shape, context)?;
            ordered_clauses.push(super::plan_ordered_case_clause(
                OrderedCaseClauseInput {
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
        ref pattern @ Pattern::Variable { ref name, .. } => {
            crate::planner::pattern::validate_pattern(
                pattern,
                &crate::plan::ValueShape::External(shape.clone()),
                context,
            )?;
            Ok(vec![name.clone()])
        }
        ref pattern @ Pattern::Discard { .. } => {
            crate::planner::pattern::validate_pattern(
                pattern,
                &crate::plan::ValueShape::External(shape.clone()),
                context,
            )?;
            Ok(Vec::new())
        }
        Pattern::Assign { name, pattern, .. } => {
            let mut names = plan_pattern(*pattern, shape, context)?;
            names.push(name);
            Ok(names)
        }
        pattern @ (Pattern::Int { .. }
        | Pattern::Float { .. }
        | Pattern::String { .. }
        | Pattern::BitArraySize(_)
        | Pattern::List { .. }
        | Pattern::Constructor { .. }
        | Pattern::Tuple { .. }
        | Pattern::BitArray { .. }
        | Pattern::StringPrefix { .. }
        | Pattern::Invalid { .. }) => Err(crate::planner::pattern::unexpected_pattern(
            &pattern,
            &crate::plan::ValueShape::External(shape.clone()),
            context,
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
        HostExternalBinding, HostExternalSchema, HostExternalStorage, HostExternalStore,
        HostExternalType, HostProvider, HostProviderModule, HostProviderSet,
    };
    use crate::plan::{ExternalTypeName, ExternalValueShape, ValueType};
    use crate::planner::context::{AnonymousFunctions, PlanContext};
    use crate::planner::support::dummy_span;
    use crate::planner::{
        InvalidCaseShapeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    };
    use crate::{ModuleSource, PackageSource};
    use ecow::EcoString;
    use gleam_compiler_core::ast::{Pattern, TypedExpr};
    use gleam_compiler_core::type_::{self, error::VariableOrigin};
    use num_bigint::BigInt;
    use std::collections::HashMap;

    struct TokenSchema;

    struct TokenProvider;

    struct TokenStorage;

    type HostToken = HostExternalType<TokenSchema>;

    impl HostExternalSchema for TokenSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Token";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalStorage<ExternalTestProfile, TokenSchema> for TokenStorage {
        type Payload = ();

        fn store(
            stores: &<ExternalTestProfile as crate::HostProfile>::ExternalStores,
        ) -> &HostExternalStore<Self::Payload> {
            &stores.units
        }

        fn source_equal(
            _: &crate::host::HostExternalEquality<'_>,
            _: &Self::Payload,
            _: &Self::Payload,
        ) -> bool {
            true
        }

        fn source_hash(_: &crate::host::HostExternalHashing<'_>, _: &Self::Payload) -> u64 {
            0
        }

        fn inspect(_: &crate::host::HostExternalInspection<'_>, _: &Self::Payload) -> EcoString {
            "Token".into()
        }
    }

    impl HostProvider<ExternalTestProfile> for TokenProvider {
        type State = ();

        fn project(state: &mut ExternalTestRunState) -> &mut Self::State {
            &mut state.provider
        }
    }

    impl HostExternalBinding<ExternalTestProfile, TokenSchema> for TokenProvider {
        type Storage = TokenStorage;
    }

    fn new_token<'call>(
        mut call: HostCall<'call, ExternalTestProfile, TokenProvider, HostToken>,
    ) -> Result<HostCallCompletion<'call, HostToken>, HostCallError> {
        let _ = call.state();
        let token = call.create_external(());
        Ok(call.return_value(token))
    }

    #[test]
    fn token_fixture_source_hash_is_exact() {
        let retained_hash = |_: &crate::runtime::StoredRuntimeValue| 7;
        let hashing = crate::host::HostExternalHashing::new(&retained_hash);

        assert_eq!(
            <TokenStorage as HostExternalStorage<ExternalTestProfile, TokenSchema>>::source_hash(
                &hashing,
                &(),
            ),
            0,
        );
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
            Err(super::super::expression_type_mismatch(
                InvalidExpressionType::External,
                InvalidExpressionType::Int,
            )),
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
                reason: InvalidTypedAstReason::InvalidExpressionNode,
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
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
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
                reason: InvalidTypedAstReason::PatternShape {
                    reason: crate::planner::InvalidPatternShapeReason::InvalidNode,
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
            Err(super::super::pattern_type_mismatch(
                ValueType::External(shape.type_().clone()),
                ValueType::Bool,
            )),
        );
        assert_eq!(
            super::plan_pattern(
                Pattern::Discard {
                    location: dummy_span(),
                    name: "_".into(),
                    type_: type_::bool(),
                },
                &shape,
                &mut context,
            ),
            Err(super::super::pattern_type_mismatch(
                ValueType::External(shape.type_().clone()),
                ValueType::Bool,
            )),
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
            .with_external_type::<TokenProvider, TokenSchema>()
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
                .with_external_type::<TokenProvider, TokenSchema>()
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
