use super::{expression_type, invalid_expression_type, plan_expr};
use crate::plan::{Expr, ValueShape};
use crate::planner::context::PlanContext;
#[cfg(not(target_pointer_width = "64"))]
use crate::planner::error::InvalidTypedAstReason;
use crate::planner::error::{InvalidExpressionType, PlanError};
use ecow::EcoString;
use gleam_core::ast::TypedExpr;
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    label: EcoString,
    index: u64,
    record: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let record_type = record.type_();
    let record = plan_expr(record, context)?;
    plan_from_expr(type_, Some(label), index, record_type, record, context)
}

pub(super) fn plan_from_expr(
    type_: Arc<Type>,
    label: Option<EcoString>,
    index: u64,
    record_type: Arc<Type>,
    record: Expr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    #[cfg(target_pointer_width = "64")]
    let index = index as usize;
    #[cfg(not(target_pointer_width = "64"))]
    let index = usize::try_from(index).map_err(|_| PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionType {
            expected: InvalidExpressionType::Custom,
            actual: InvalidExpressionType::Custom,
        },
    })?;

    let actual = expression_type(&record);
    let record = record
        .into_custom()
        .ok_or_else(|| invalid_expression_type(InvalidExpressionType::Custom, actual))?;
    let expected_shape = ValueShape::from_gleam(type_.as_ref()).ok_or_else(|| {
        invalid_expression_type(
            InvalidExpressionType::Unsupported,
            InvalidExpressionType::Custom,
        )
    })?;
    let expected = expected_shape.value_type();
    let (access, source_shape) =
        context.custom_field_access(record, record_type.as_ref(), index, label, &expected)?;
    let resolved_shape = source_shape.refine(&expected_shape).ok_or_else(|| {
        super::invalid_expression_type_for_value(expected.clone(), source_shape.value_type())
    })?;

    Ok(Expr::custom_field_shape(access, resolved_shape))
}

#[cfg(test)]
#[allow(clippy::arc_with_non_send_sync)]
mod tests {
    use super::{plan, plan_from_expr};
    use crate::plan::{
        CustomConstructorDefinition, CustomConstructorRefinement, CustomExpr,
        CustomFieldDefinition, CustomLocal, CustomLocalId, CustomType, CustomTypeDefinition,
        CustomTypeName, CustomTypeParameterId, CustomTypePublicity, CustomTypeTemplate,
        CustomValueShape, Expr, FunctionShape, FunctionType, IntExpr, ValueShape, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, PlanContext};
    use crate::planner::support::dummy_span;
    use crate::planner::{
        InvalidCustomTypeReason, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    };
    use ecow::EcoString;
    use gleam_core::ast::Publicity;
    use gleam_core::ast::TypedExpr;
    use gleam_core::type_::{self, Type};
    use std::collections::HashMap;
    use std::sync::Arc;

    #[test]
    fn record_access_rejects_invalid_expression_and_result_types() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let definition = generic_definition(&module);
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new_with_custom_types(
            &module,
            &functions,
            std::slice::from_ref(&definition),
            &mut anonymous,
        );
        let custom_type = generic_type(&module, vec![ValueType::Int]);
        let gleam_type = generic_gleam_type(&module, vec![type_::int()], None);

        assert_eq!(
            plan(
                type_::int(),
                "value".into(),
                0,
                TypedExpr::Invalid {
                    location: dummy_span(),
                    type_: gleam_type.clone(),
                    extra_information: None,
                },
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
        assert_eq!(
            plan_from_expr(
                type_::int(),
                Some("value".into()),
                0,
                gleam_type.clone(),
                Expr::int(IntExpr::value(1.into())),
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Custom,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_from_expr(
                type_::generic_var(99),
                Some("value".into()),
                0,
                gleam_type,
                custom_local(custom_type),
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Unsupported,
                    actual: InvalidExpressionType::Custom,
                },
            }),
        );
        let missing = CustomType::new(
            CustomTypeName::new("geam".into(), module.clone(), "Missing".into()),
            Vec::new(),
        );
        assert_eq!(
            plan_from_expr(
                type_::int(),
                Some("value".into()),
                0,
                Arc::new(Type::Named {
                    publicity: Publicity::Private,
                    package: "geam".into(),
                    module: module.clone(),
                    name: "Missing".into(),
                    arguments: Vec::new(),
                    inferred_variant: None,
                }),
                custom_local(missing.clone()),
                &mut context,
            ),
            Err(custom_error(
                &missing,
                InvalidCustomTypeReason::UnknownDefinition,
            )),
        );
    }

    #[test]
    fn record_access_rejects_invalid_custom_metadata() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let definition = generic_definition(&module);
        let mut anonymous = AnonymousFunctions::default();
        let context = PlanContext::new_with_custom_types(
            &module,
            &functions,
            std::slice::from_ref(&definition),
            &mut anonymous,
        );
        let generic_int = generic_type(&module, vec![ValueType::Int]);
        let generic_without_arguments = generic_type(&module, Vec::new());
        let missing = CustomType::new(
            CustomTypeName::new("geam".into(), module.clone(), "Missing".into()),
            Vec::new(),
        );

        assert_eq!(
            context.custom_field_access(
                custom_local_expr(missing.clone()),
                generic_gleam_type(&module, Vec::new(), None).as_ref(),
                0,
                Some("value".into()),
                &ValueType::Int,
            ),
            Err(custom_error(
                &missing,
                InvalidCustomTypeReason::UnknownDefinition,
            )),
        );
        assert_eq!(
            context.custom_field_access(
                custom_local_expr(generic_without_arguments.clone()),
                generic_gleam_type(&module, Vec::new(), None).as_ref(),
                0,
                Some("value".into()),
                &ValueType::Int,
            ),
            Err(custom_error(
                &generic_without_arguments,
                InvalidCustomTypeReason::TypeArgumentCount,
            )),
        );
        assert_eq!(
            context.custom_field_access(
                custom_local_expr(generic_int.clone()),
                generic_gleam_type(&module, vec![type_::int()], Some(1)).as_ref(),
                0,
                Some("value".into()),
                &ValueType::Int,
            ),
            Err(custom_error(
                &generic_int,
                InvalidCustomTypeReason::ConstructorIndex,
            )),
        );
        assert_eq!(
            context.custom_field_access(
                custom_local_expr(generic_int.clone()),
                generic_gleam_type(&module, vec![type_::int()], None).as_ref(),
                1,
                Some("value".into()),
                &ValueType::Int,
            ),
            Err(custom_error(
                &generic_int,
                InvalidCustomTypeReason::FieldIndex,
            )),
        );
        assert_eq!(
            context.custom_field_access(
                custom_local_expr(generic_int.clone()),
                generic_gleam_type(&module, vec![type_::int()], None).as_ref(),
                0,
                Some("wrong".into()),
                &ValueType::Int,
            ),
            Err(custom_error(
                &generic_int,
                InvalidCustomTypeReason::FieldLabel,
            )),
        );
        assert_eq!(
            context.custom_field_access(
                custom_local_expr(generic_int.clone()),
                generic_gleam_type(&module, vec![type_::int()], None).as_ref(),
                0,
                Some("value".into()),
                &ValueType::String,
            ),
            Err(custom_error(
                &generic_int,
                InvalidCustomTypeReason::FieldType,
            )),
        );

        let broken_definition = CustomTypeDefinition::new(
            CustomTypeName::new("geam".into(), module.clone(), "Broken".into()),
            CustomTypePublicity::Private,
            false,
            vec![CustomTypeParameterId(0)],
            vec![CustomConstructorDefinition::new(
                "Broken".into(),
                0,
                vec![CustomFieldDefinition::new(
                    Some("value".into()),
                    CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                )],
            )],
        );
        let broken = CustomType::new(broken_definition.name().clone(), vec![ValueType::Int]);
        let mut anonymous = AnonymousFunctions::default();
        let broken_context = PlanContext::new_with_custom_types(
            &module,
            &functions,
            std::slice::from_ref(&broken_definition),
            &mut anonymous,
        );
        assert_eq!(
            broken_context.custom_field_access(
                custom_local_expr(broken.clone()),
                Arc::new(Type::Named {
                    publicity: Publicity::Private,
                    package: "geam".into(),
                    module: module.clone(),
                    name: "Broken".into(),
                    arguments: vec![type_::int()],
                    inferred_variant: None,
                })
                .as_ref(),
                0,
                Some("value".into()),
                &ValueType::Int,
            ),
            Err(custom_error(
                &broken,
                InvalidCustomTypeReason::ParameterType,
            )),
        );

        let partially_broken_definition = CustomTypeDefinition::new(
            CustomTypeName::new("geam".into(), module.clone(), "PartiallyBroken".into()),
            CustomTypePublicity::Private,
            false,
            vec![CustomTypeParameterId(0)],
            vec![CustomConstructorDefinition::new(
                "PartiallyBroken".into(),
                0,
                vec![
                    CustomFieldDefinition::new(
                        Some("value".into()),
                        CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                    ),
                    CustomFieldDefinition::new(
                        Some("invalid".into()),
                        CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                    ),
                ],
            )],
        );
        let partially_broken = CustomType::new(
            partially_broken_definition.name().clone(),
            vec![ValueType::Int],
        );
        let mut anonymous = AnonymousFunctions::default();
        let partially_broken_context = PlanContext::new_with_custom_types(
            &module,
            &functions,
            std::slice::from_ref(&partially_broken_definition),
            &mut anonymous,
        );
        assert_eq!(
            partially_broken_context.custom_field_access(
                custom_local_expr(partially_broken.clone()),
                Arc::new(Type::Named {
                    publicity: Publicity::Private,
                    package: "geam".into(),
                    module: module.clone(),
                    name: "PartiallyBroken".into(),
                    arguments: vec![type_::int()],
                    inferred_variant: None,
                })
                .as_ref(),
                0,
                Some("value".into()),
                &ValueType::Int,
            ),
            Err(custom_error(
                &partially_broken,
                InvalidCustomTypeReason::ParameterType,
            )),
        );
    }

    #[test]
    fn record_access_rejects_empty_and_incompatible_shared_field_shapes() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let empty_name = CustomTypeName::new("geam".into(), module.clone(), "Empty".into());
        let shared_name = CustomTypeName::new("geam".into(), module.clone(), "Shared".into());
        let definitions = vec![
            CustomTypeDefinition::new(
                empty_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                Vec::new(),
            ),
            CustomTypeDefinition::new(
                shared_name.clone(),
                CustomTypePublicity::Private,
                false,
                vec![CustomTypeParameterId(0), CustomTypeParameterId(1)],
                vec![
                    CustomConstructorDefinition::new(
                        "First".into(),
                        0,
                        vec![CustomFieldDefinition::new(
                            Some("value".into()),
                            CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                        )],
                    ),
                    CustomConstructorDefinition::new(
                        "Second".into(),
                        1,
                        vec![CustomFieldDefinition::new(
                            Some("value".into()),
                            CustomTypeTemplate::Parameter(CustomTypeParameterId(1)),
                        )],
                    ),
                ],
            ),
        ];
        let mut anonymous = AnonymousFunctions::default();
        let context =
            PlanContext::new_with_custom_types(&module, &functions, &definitions, &mut anonymous);

        let empty = CustomType::new(empty_name.clone(), Vec::new());
        let empty_source = Arc::new(Type::Named {
            publicity: Publicity::Private,
            package: "geam".into(),
            module: module.clone(),
            name: empty_name.name().clone(),
            arguments: Vec::new(),
            inferred_variant: None,
        });
        assert_eq!(
            context.custom_field_access(
                custom_local_expr(empty.clone()),
                empty_source.as_ref(),
                0,
                Some("value".into()),
                &ValueType::Int,
            ),
            Err(custom_error(&empty, InvalidCustomTypeReason::FieldIndex,)),
        );

        let choice = CustomType::new(
            CustomTypeName::new("geam".into(), module.clone(), "Choice".into()),
            Vec::new(),
        );
        let function_type =
            FunctionType::new(vec![ValueType::Custom(choice.clone())], ValueType::Int);
        let function_shape = |constructor| {
            ValueShape::Function(Box::new(FunctionShape::new(
                vec![ValueShape::Custom(CustomValueShape::new(
                    choice.type_name().clone(),
                    Vec::new(),
                    CustomConstructorRefinement::Exact(constructor),
                ))],
                ValueShape::Int,
            )))
        };
        let shared_shape = CustomValueShape::new(
            shared_name.clone(),
            vec![function_shape(0), function_shape(1)],
            CustomConstructorRefinement::Any,
        );
        let shared = shared_shape.type_().clone();
        let source = CustomExpr::local_get(
            CustomLocal::from_shape(CustomLocalId(0), shared_shape),
            "shared".into(),
        );
        let shared_source = Arc::new(Type::Named {
            publicity: Publicity::Private,
            package: "geam".into(),
            module: module.clone(),
            name: shared_name.name().clone(),
            arguments: vec![
                type_::fn_(vec![named_custom_type(&choice)], type_::int()),
                type_::fn_(vec![named_custom_type(&choice)], type_::int()),
            ],
            inferred_variant: None,
        });
        assert_eq!(
            context.custom_field_access(
                source,
                shared_source.as_ref(),
                0,
                Some("value".into()),
                &ValueType::Function(Box::new(function_type)),
            ),
            Err(custom_error(&shared, InvalidCustomTypeReason::FieldType,)),
        );
    }

    #[test]
    fn record_access_rejects_conflicting_result_refinements() {
        let module = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let choice_name = CustomTypeName::new("geam".into(), module.clone(), "Choice".into());
        let definitions = vec![
            generic_definition(&module),
            CustomTypeDefinition::new(
                choice_name.clone(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![
                    CustomConstructorDefinition::new("First".into(), 0, Vec::new()),
                    CustomConstructorDefinition::new("Second".into(), 1, Vec::new()),
                ],
            ),
        ];
        let choice = CustomType::new(choice_name.clone(), Vec::new());
        let first_shape = CustomValueShape::new(
            choice_name,
            Vec::new(),
            CustomConstructorRefinement::Exact(0),
        );
        let source_shape = CustomValueShape::new(
            generic_type(&module, vec![ValueType::Custom(choice.clone())])
                .type_name()
                .clone(),
            vec![ValueShape::Custom(first_shape)],
            CustomConstructorRefinement::Exact(0),
        );
        let source = Expr::custom(CustomExpr::local_get(
            CustomLocal::from_shape(CustomLocalId(0), source_shape),
            "record".into(),
        ));
        let record_type = generic_gleam_type(
            &module,
            vec![Arc::new(Type::Named {
                publicity: Publicity::Private,
                package: "geam".into(),
                module: module.clone(),
                name: "Choice".into(),
                arguments: Vec::new(),
                inferred_variant: Some(0),
            })],
            Some(0),
        );
        let expected = Arc::new(Type::Named {
            publicity: Publicity::Private,
            package: "geam".into(),
            module: module.clone(),
            name: "Choice".into(),
            arguments: Vec::new(),
            inferred_variant: Some(1),
        });
        let mut anonymous = AnonymousFunctions::default();
        let mut context =
            PlanContext::new_with_custom_types(&module, &functions, &definitions, &mut anonymous);

        assert_eq!(
            plan_from_expr(
                expected,
                Some("value".into()),
                0,
                record_type,
                source,
                &mut context,
            ),
            Err(super::super::invalid_expression_type_for_value(
                ValueType::Custom(choice.clone()),
                ValueType::Custom(choice),
            )),
        );
    }

    fn generic_definition(module: &EcoString) -> CustomTypeDefinition {
        CustomTypeDefinition::new(
            CustomTypeName::new("geam".into(), module.clone(), "Generic".into()),
            CustomTypePublicity::Private,
            false,
            vec![CustomTypeParameterId(0)],
            vec![CustomConstructorDefinition::new(
                "Generic".into(),
                0,
                vec![CustomFieldDefinition::new(
                    Some("value".into()),
                    CustomTypeTemplate::Parameter(CustomTypeParameterId(0)),
                )],
            )],
        )
    }

    fn generic_type(module: &EcoString, arguments: Vec<ValueType>) -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), module.clone(), "Generic".into()),
            arguments,
        )
    }

    fn generic_gleam_type(
        module: &EcoString,
        arguments: Vec<Arc<Type>>,
        inferred_variant: Option<u16>,
    ) -> Arc<Type> {
        Arc::new(Type::Named {
            publicity: Publicity::Private,
            package: "geam".into(),
            module: module.clone(),
            name: "Generic".into(),
            arguments,
            inferred_variant,
        })
    }

    fn named_custom_type(type_: &CustomType) -> Arc<Type> {
        Arc::new(Type::Named {
            publicity: Publicity::Private,
            package: type_.type_name().package().clone(),
            module: type_.type_name().module().clone(),
            name: type_.type_name().name().clone(),
            arguments: Vec::new(),
            inferred_variant: None,
        })
    }

    fn custom_local(type_: CustomType) -> Expr {
        Expr::custom(custom_local_expr(type_))
    }

    fn custom_local_expr(type_: CustomType) -> CustomExpr {
        CustomExpr::local_get(
            crate::plan::CustomLocal::new(CustomLocalId(0), type_),
            "value".into(),
        )
    }

    fn custom_error(type_: &CustomType, reason: InvalidCustomTypeReason) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::CustomType {
                name: type_.type_name().name().clone(),
                reason,
            },
        }
    }
}
