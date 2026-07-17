use super::{plan_expr, plan_expr_with_expected_source_stop_type, record_access};
use crate::plan::{
    CustomConstructor, CustomExpr, CustomLocalId, CustomType, Expr, Step, ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionType, InvalidRecordUpdateShapeReason, InvalidTypedAstReason, PlanError,
};
use ecow::EcoString;
use gleam_core::ast::{CallArg, ImplicitCallArgOrigin, TypedExpr};
use gleam_core::type_::error::VariableOrigin;
use gleam_core::type_::{Type, ValueConstructor, ValueConstructorVariant};
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    updated_record: TypedExpr,
    updated_record_assigned_name: Option<EcoString>,
    constructor: TypedExpr,
    arguments: Vec<CallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let source_type = updated_record.type_();
    let source_custom_type = custom_type(source_type.as_ref(), context)?;
    let implicit_target = implicit_target(
        &updated_record,
        updated_record_assigned_name,
        &source_custom_type,
    )?;
    let constructor = record_constructor(constructor, context)?;
    let result_type = custom_type(type_.as_ref(), context)?;
    if constructor.type_() != &result_type {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::Type,
            },
        });
    }
    let source = plan_expr(updated_record, context)?;
    let actual = source.value_type();
    let Some(source) = source.into_custom() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Custom,
                actual: InvalidExpressionType::from_value_type(actual),
            },
        });
    };
    if source.type_() != &source_custom_type {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::Type,
            },
        });
    }

    let local = context.define_internal_custom_local();
    let typed_local = crate::plan::CustomLocal::from_shape(local, source.shape().clone());
    let local_name = internal_local_name(local);
    let step = Step::let_custom(local, local_name.clone(), source);
    let local_get = CustomExpr::local_get(typed_local, local_name);
    let arguments = plan_arguments(
        arguments,
        &constructor,
        local_get,
        &implicit_target,
        context,
    )?;
    let construction =
        crate::plan::CustomConstruction::try_new(constructor, arguments).map_err(|_| {
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::RecordUpdateShape {
                    reason: InvalidRecordUpdateShapeReason::ArgumentCount,
                },
            }
        })?;
    context
        .custom_expr_from_construction(construction)
        .map(|result| Expr::custom(CustomExpr::block(vec![step], result)))
}

fn plan_arguments(
    arguments: Vec<CallArg<TypedExpr>>,
    constructor: &CustomConstructor,
    source: CustomExpr,
    implicit_target: &ImplicitTarget,
    context: &mut PlanContext<'_>,
) -> Result<Vec<Expr>, PlanError> {
    let mut planned = Vec::with_capacity(arguments.len());
    for (index, argument) in arguments.into_iter().enumerate() {
        let Some(field) = constructor.fields().get(index) else {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::RecordUpdateShape {
                    reason: InvalidRecordUpdateShapeReason::ArgumentCount,
                },
            });
        };
        if argument.label.as_ref() != field.label() {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::RecordUpdateShape {
                    reason: InvalidRecordUpdateShapeReason::ArgumentLabel,
                },
            });
        }
        let expression = match argument.implicit {
            None => plan_expr_with_expected_source_stop_type(
                argument.value,
                field.type_().clone(),
                context,
            )?,
            Some(ImplicitCallArgOrigin::RecordUpdate) => plan_implicit_argument(
                argument.value,
                index,
                field.label().cloned(),
                field.type_(),
                source.clone(),
                implicit_target,
                context,
            )?,
            Some(_) => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::RecordUpdateShape {
                        reason: InvalidRecordUpdateShapeReason::ImplicitArgumentOrigin,
                    },
                });
            }
        };
        if expression.value_type() != *field.type_() {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::from_value_type(field.type_().clone()),
                    actual: InvalidExpressionType::from_value_type(expression.value_type()),
                },
            });
        }
        planned.push(expression);
    }
    Ok(planned)
}

fn plan_implicit_argument(
    expression: TypedExpr,
    expected_index: usize,
    expected_label: Option<EcoString>,
    expected_type: &ValueType,
    source: CustomExpr,
    implicit_target: &ImplicitTarget,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let (type_, label, index, record) = match expression {
        TypedExpr::RecordAccess {
            type_,
            label,
            index,
            record,
            ..
        } => (type_, Some(label), index, *record),
        TypedExpr::PositionalAccess {
            type_,
            index,
            record,
            ..
        } => (type_, None, index, *record),
        _ => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::RecordUpdateShape {
                    reason: InvalidRecordUpdateShapeReason::ImplicitFieldAccess,
                },
            });
        }
    };
    if index != expected_index as u64
        || label != expected_label
        || context.value_type(type_.as_ref()) != *expected_type
    {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::ImplicitFieldAccess,
            },
        });
    }
    if !implicit_target.matches(&record, context) {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::ImplicitFieldTarget,
            },
        });
    }
    record_access::plan_from_expr(type_, label, index, Expr::custom(source), context)
}

fn record_constructor(
    expression: TypedExpr,
    context: &PlanContext<'_>,
) -> Result<CustomConstructor, PlanError> {
    let TypedExpr::Var { constructor, .. } = expression else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::Constructor,
            },
        });
    };
    let ValueConstructorVariant::Record { .. } = &constructor.variant else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::Constructor,
            },
        });
    };
    context.custom_constructor(&constructor)
}

fn custom_type(type_: &Type, context: &mut PlanContext<'_>) -> Result<CustomType, PlanError> {
    match context.value_type(type_) {
        ValueType::Custom(type_) => Ok(type_),
        _ => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::Type,
            },
        }),
    }
}

enum ImplicitTarget {
    OriginalVariable {
        name: EcoString,
        constructor: Box<ValueConstructor>,
    },
    GeneratedVariable {
        name: EcoString,
        type_: ValueType,
    },
}

impl ImplicitTarget {
    fn matches(&self, expression: &TypedExpr, context: &PlanContext<'_>) -> bool {
        let TypedExpr::Var {
            name, constructor, ..
        } = expression
        else {
            return false;
        };
        match self {
            Self::OriginalVariable {
                name: expected_name,
                constructor: expected_constructor,
            } => name == expected_name && constructor == expected_constructor.as_ref(),
            Self::GeneratedVariable {
                name: expected_name,
                type_: expected_type,
            } => {
                name == expected_name
                    && context
                        .value_shape_in_scope(constructor.type_.as_ref())
                        .value_type()
                        == *expected_type
                    && is_generated_local_variable(constructor)
            }
        }
    }
}

fn is_generated_local_variable(constructor: &ValueConstructor) -> bool {
    let ValueConstructorVariant::LocalVariable { origin, .. } = &constructor.variant else {
        return false;
    };
    origin == &VariableOrigin::generated()
}

fn implicit_target(
    updated_record: &TypedExpr,
    assigned_name: Option<EcoString>,
    source_type: &CustomType,
) -> Result<ImplicitTarget, PlanError> {
    match (updated_record, assigned_name) {
        (
            TypedExpr::Var {
                name, constructor, ..
            },
            None,
        ) => Ok(ImplicitTarget::OriginalVariable {
            name: name.clone(),
            constructor: Box::new(constructor.clone()),
        }),
        (TypedExpr::Var { .. }, Some(_)) | (_, None) => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape {
                reason: InvalidRecordUpdateShapeReason::BaseAssignment,
            },
        }),
        (_, Some(name)) => Ok(ImplicitTarget::GeneratedVariable {
            name,
            type_: ValueType::Custom(source_type.clone()),
        }),
    }
}

fn internal_local_name(local: CustomLocalId) -> EcoString {
    format!("<record:update:{}>", local.0).into()
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        CustomConstructor, CustomConstructorField, CustomExpr, CustomFieldAccess, CustomLocalId,
        CustomReturn, CustomType, CustomTypeName, Expr, IntExpr, ReturnExpr, Step, StringExpr,
        ValueType,
    };
    use crate::planner::support::{compile, dummy_span};
    use crate::planner::{
        InvalidExpressionType, InvalidRecordUpdateShapeReason, InvalidTypedAstReason, PlanError,
        plan_module,
    };
    use gleam_core::ast::{CallArg, ImplicitCallArgOrigin, Statement, TypedExpr, TypedStatement};
    use gleam_core::type_::error::{VariableDeclaration, VariableOrigin, VariableSyntax};
    use gleam_core::type_::{self, Type, ValueConstructor, ValueConstructorVariant};
    use std::sync::Arc;

    const SOURCE: &str = r#"
pub type Person {
  Person(name: String, age: Int)
}

pub fn main() {
  let person = Person(name: "Lucy", age: 30)
  Person(..person, age: 31)
}
"#;

    const NON_VARIABLE_SOURCE: &str = r#"
pub type Person {
  Person(name: String, age: Int)
}

fn identity(person: Person) {
  person
}

pub fn main() {
  let person = Person(name: "Lucy", age: 30)
  Person(..identity(person), age: 31)
}
"#;

    const POSITIONAL_SOURCE: &str = r#"
pub type Boxed(a) {
  Boxed(a, label: String)
}

pub fn main() {
  let boxed = Boxed(1, label: "one")
  Boxed(..boxed, label: "two")
}
"#;

    const ALL_FIELDS_SOURCE: &str = r#"
pub type Person {
  Person(name: String, age: Int)
}

pub fn main() {
  let person = Person(name: "Lucy", age: 30)
  Person(..person, name: "Mia", age: 31)
}
"#;

    fn invalid_shape(reason: InvalidRecordUpdateShapeReason) -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::RecordUpdateShape { reason },
        }
    }

    #[test]
    fn plan_record_update_binds_base_once_and_projects_existing_field() {
        let plan = plan_module(compile(SOURCE)).expect("record update should plan");
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Person".into()),
            Vec::new(),
        );
        let constructor = CustomConstructor::new(
            type_.clone(),
            "Person".into(),
            0,
            vec![
                CustomConstructorField::new(Some("name".into()), ValueType::String),
                CustomConstructorField::new(Some("age".into()), ValueType::Int),
            ],
        );
        let shape = crate::plan::CustomValueShape::new(
            type_.type_name().clone(),
            Vec::new(),
            crate::plan::CustomConstructorRefinement::Exact(0),
        );
        let source = CustomExpr::local_get(
            crate::plan::CustomLocal::from_shape(CustomLocalId(0), shape.clone()),
            "person".into(),
        );
        let local = CustomLocalId(1);
        let local_name = ecow::EcoString::from("<record:update:1>");
        let projected_name = Expr::string(StringExpr::custom_field(CustomFieldAccess::new(
            CustomExpr::local_get(
                crate::plan::CustomLocal::from_shape(local, shape.clone()),
                local_name.clone(),
            ),
            0,
            Some("name".into()),
        )));
        let updated = CustomExpr::from_construction(
            shape,
            crate::plan::CustomConstruction::try_new(
                constructor,
                vec![projected_name, Expr::int(IntExpr::value(31.into()))],
            )
            .expect("test record construction should be valid"),
        );

        assert_eq!(
            plan.main_function().return_(),
            &ReturnExpr::custom_body(CustomReturn::block(
                vec![Step::let_custom(local, local_name, source)],
                CustomReturn::expr(updated),
            )),
        );
    }

    #[test]
    fn plan_record_update_projects_positional_field() {
        let plan = plan_module(compile(POSITIONAL_SOURCE)).expect("record update should plan");
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            vec![ValueType::Int],
        );
        let constructor = CustomConstructor::new(
            type_.clone(),
            "Boxed".into(),
            0,
            vec![
                CustomConstructorField::new(None, ValueType::Int),
                CustomConstructorField::new(Some("label".into()), ValueType::String),
            ],
        );
        let shape = crate::plan::CustomValueShape::new(
            type_.type_name().clone(),
            vec![crate::plan::ValueShape::Int],
            crate::plan::CustomConstructorRefinement::Exact(0),
        );
        let source = CustomExpr::local_get(
            crate::plan::CustomLocal::from_shape(CustomLocalId(0), shape.clone()),
            "boxed".into(),
        );
        let local = CustomLocalId(1);
        let local_name = ecow::EcoString::from("<record:update:1>");
        let projected_value = Expr::int(IntExpr::custom_field(CustomFieldAccess::new(
            CustomExpr::local_get(
                crate::plan::CustomLocal::from_shape(local, shape.clone()),
                local_name.clone(),
            ),
            0,
            None,
        )));
        let updated = CustomExpr::from_construction(
            shape,
            crate::plan::CustomConstruction::try_new(
                constructor,
                vec![
                    projected_value,
                    Expr::string(StringExpr::value("two".into())),
                ],
            )
            .expect("test record construction should be valid"),
        );

        assert_eq!(
            plan.main_function().return_(),
            &ReturnExpr::custom_body(CustomReturn::block(
                vec![Step::let_custom(local, local_name, source)],
                CustomReturn::expr(updated),
            )),
        );
    }

    #[test]
    fn plan_record_update_evaluates_base_when_all_fields_are_explicit() {
        let plan = plan_module(compile(ALL_FIELDS_SOURCE)).expect("record update should plan");
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Person".into()),
            Vec::new(),
        );
        let constructor = CustomConstructor::new(
            type_.clone(),
            "Person".into(),
            0,
            vec![
                CustomConstructorField::new(Some("name".into()), ValueType::String),
                CustomConstructorField::new(Some("age".into()), ValueType::Int),
            ],
        );
        let shape = crate::plan::CustomValueShape::new(
            type_.type_name().clone(),
            Vec::new(),
            crate::plan::CustomConstructorRefinement::Exact(0),
        );
        let source = CustomExpr::local_get(
            crate::plan::CustomLocal::from_shape(CustomLocalId(0), shape.clone()),
            "person".into(),
        );
        let local = CustomLocalId(1);
        let local_name = ecow::EcoString::from("<record:update:1>");
        let updated = CustomExpr::from_construction(
            shape,
            crate::plan::CustomConstruction::try_new(
                constructor,
                vec![
                    Expr::string(StringExpr::value("Mia".into())),
                    Expr::int(IntExpr::value(31.into())),
                ],
            )
            .expect("test record construction should be valid"),
        );

        assert_eq!(
            plan.main_function().return_(),
            &ReturnExpr::custom_body(CustomReturn::block(
                vec![Step::let_custom(local, local_name, source)],
                CustomReturn::expr(updated),
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_base_assignment() {
        let mut assigned_variable = compile(SOURCE);
        let (_, _, assigned_name, _, _) =
            record_update_parts_mut(&mut assigned_variable.definitions.functions[0].body[1]);
        *assigned_name = Some("_record".into());
        assert_eq!(
            plan_module(assigned_variable),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::BaseAssignment,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_constructor_expression() {
        let mut non_record_constructor = compile(SOURCE);
        let (_, _, _, constructor, _) =
            record_update_parts_mut(&mut non_record_constructor.definitions.functions[0].body[1]);
        *constructor = TypedExpr::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: 1.into(),
            type_: type_::int(),
        };
        assert_eq!(
            plan_module(non_record_constructor),
            Err(invalid_shape(InvalidRecordUpdateShapeReason::Constructor)),
        );
    }

    #[test]
    fn reject_margin_record_update_non_custom_result_type() {
        let mut wrong_result_type = compile(SOURCE);
        let (type_, _, _, _, _) =
            record_update_parts_mut(&mut wrong_result_type.definitions.functions[0].body[1]);
        *type_ = type_::int();
        assert_eq!(
            plan_module(wrong_result_type),
            Err(invalid_shape(InvalidRecordUpdateShapeReason::Type)),
        );
    }

    #[test]
    fn reject_margin_record_update_mismatched_custom_result_type() {
        let mut mismatched_result_type = compile(SOURCE);
        let (type_, _, _, _, _) =
            record_update_parts_mut(&mut mismatched_result_type.definitions.functions[0].body[1]);
        *type_ = type_::result(type_::int(), type_::nil());
        assert_eq!(
            plan_module(mismatched_result_type),
            Err(invalid_shape(InvalidRecordUpdateShapeReason::Type)),
        );
    }

    #[test]
    fn reject_margin_record_update_non_constructor_variable() {
        let mut non_record_variant = compile(SOURCE);
        let (_, updated_record, _, constructor, _) =
            record_update_parts_mut(&mut non_record_variant.definitions.functions[0].body[1]);
        *constructor = updated_record.clone();
        assert_eq!(
            plan_module(non_record_variant),
            Err(invalid_shape(InvalidRecordUpdateShapeReason::Constructor)),
        );
    }

    #[test]
    fn reject_margin_record_update_non_custom_base_type() {
        let mut unsupported_base_type = compile(SOURCE);
        let (_, updated_record, _, _, _) =
            record_update_parts_mut(&mut unsupported_base_type.definitions.functions[0].body[1]);
        *updated_record = TypedExpr::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: 1.into(),
            type_: type_::int(),
        };
        assert_eq!(
            plan_module(unsupported_base_type),
            Err(invalid_shape(InvalidRecordUpdateShapeReason::Type)),
        );
    }

    #[test]
    fn reject_margin_record_update_invalid_base_expression() {
        let mut invalid_base_expression = compile(SOURCE);
        let (_, updated_record, assigned_name, _, _) =
            record_update_parts_mut(&mut invalid_base_expression.definitions.functions[0].body[1]);
        let type_ = updated_record.type_();
        *updated_record = TypedExpr::Invalid {
            location: dummy_span(),
            type_,
            extra_information: None,
        };
        *assigned_name = Some("_record".into());
        assert_eq!(
            plan_module(invalid_base_expression),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_record_update_base_expression_family() {
        let mut non_custom_local = compile(
            r#"
pub type Person { Person(name: String, age: Int) }
pub fn main() {
  let number = 1
  let person = Person(name: "Lucy", age: 30)
  Person(..person, age: 31)
}
"#,
        );
        let (_, updated_record, _, _, _) =
            record_update_parts_mut(&mut non_custom_local.definitions.functions[0].body[2]);
        let (name, _) = variable_parts_mut(updated_record);
        *name = "number".into();
        assert_eq!(
            plan_module(non_custom_local),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Custom,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_record_update_base_custom_type() {
        let mut wrong_custom_local = compile(
            r#"
pub type Person { Person(name: String, age: Int) }
pub type Other { Other(name: String, age: Int) }
pub fn main() {
  let other = Other(name: "Lucy", age: 30)
  let person = Person(name: "Lucy", age: 30)
  Person(..person, age: 31)
}
"#,
        );
        let (_, updated_record, _, _, _) =
            record_update_parts_mut(&mut wrong_custom_local.definitions.functions[0].body[2]);
        let (name, _) = variable_parts_mut(updated_record);
        *name = "other".into();
        assert_eq!(
            plan_module(wrong_custom_local),
            Err(invalid_shape(InvalidRecordUpdateShapeReason::Type)),
        );
    }

    #[test]
    fn reject_margin_record_update_argument_count() {
        let mut wrong_count = compile(SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_count.definitions.functions[0].body[1]);
        arguments.pop();
        assert_eq!(
            plan_module(wrong_count),
            Err(invalid_shape(InvalidRecordUpdateShapeReason::ArgumentCount,)),
        );

        let mut extra_argument = compile(SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut extra_argument.definitions.functions[0].body[1]);
        arguments.push(arguments[0].clone());
        assert_eq!(
            plan_module(extra_argument),
            Err(invalid_shape(InvalidRecordUpdateShapeReason::ArgumentCount,)),
        );
    }

    #[test]
    fn reject_margin_record_update_argument_label() {
        let mut wrong_label = compile(SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_label.definitions.functions[0].body[1]);
        arguments[0].label = Some("wrong".into());
        assert_eq!(
            plan_module(wrong_label),
            Err(invalid_shape(InvalidRecordUpdateShapeReason::ArgumentLabel,)),
        );
    }

    #[test]
    fn reject_margin_record_update_explicit_argument_type() {
        let mut wrong_explicit_type = compile(SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_explicit_type.definitions.functions[0].body[1]);
        arguments[1].value = TypedExpr::String {
            location: dummy_span(),
            value: "wrong".into(),
            type_: type_::string(),
        };
        assert_eq!(
            plan_module(wrong_explicit_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::String,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_record_update_invalid_explicit_argument() {
        let mut invalid_explicit_expression = compile(SOURCE);
        let (_, _, _, _, arguments) = record_update_parts_mut(
            &mut invalid_explicit_expression.definitions.functions[0].body[1],
        );
        arguments[1].value = TypedExpr::Invalid {
            location: dummy_span(),
            type_: type_::int(),
            extra_information: None,
        };
        assert_eq!(
            plan_module(invalid_explicit_expression),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: crate::planner::InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_record_update_implicit_argument_origin() {
        let mut wrong_origin = compile(SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_origin.definitions.functions[0].body[1]);
        arguments[0].implicit = Some(ImplicitCallArgOrigin::Pipe);
        assert_eq!(
            plan_module(wrong_origin),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ImplicitArgumentOrigin,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_implicit_argument_expression() {
        let mut wrong_expression = compile(SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_expression.definitions.functions[0].body[1]);
        arguments[0].value = TypedExpr::String {
            location: dummy_span(),
            value: "wrong".into(),
            type_: type_::string(),
        };
        assert_eq!(
            plan_module(wrong_expression),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ImplicitFieldAccess,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_implicit_field_index() {
        let mut wrong_index = compile(SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_index.definitions.functions[0].body[1]);
        let (_, _, index, _) = implicit_record_access_parts_mut(&mut arguments[0].value);
        *index = 1;
        assert_eq!(
            plan_module(wrong_index),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ImplicitFieldAccess,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_implicit_field_label() {
        let mut wrong_label = compile(SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_label.definitions.functions[0].body[1]);
        let (_, label, _, _) = implicit_record_access_parts_mut(&mut arguments[0].value);
        *label = "wrong".into();
        assert_eq!(
            plan_module(wrong_label),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ImplicitFieldAccess,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_implicit_field_type() {
        let mut wrong_type = compile(SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_type.definitions.functions[0].body[1]);
        let (field_type, _, _, _) = implicit_record_access_parts_mut(&mut arguments[0].value);
        *field_type = type_::int();
        assert_eq!(
            plan_module(wrong_type),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ImplicitFieldAccess,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_original_target_name() {
        let mut wrong_target = compile(SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_target.definitions.functions[0].body[1]);
        let (_, _, _, record) = implicit_record_access_parts_mut(&mut arguments[0].value);
        let (name, _) = variable_parts_mut(record);
        *name = "wrong".into();
        assert_eq!(
            plan_module(wrong_target),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ImplicitFieldTarget,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_original_target_constructor() {
        let mut wrong_original_constructor = compile(SOURCE);
        let (_, _, _, _, arguments) = record_update_parts_mut(
            &mut wrong_original_constructor.definitions.functions[0].body[1],
        );
        let (_, _, _, record) = implicit_record_access_parts_mut(&mut arguments[0].value);
        let (_, constructor) = variable_parts_mut(record);
        constructor.type_ = type_::int();
        assert_eq!(
            plan_module(wrong_original_constructor),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ImplicitFieldTarget,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_generated_target_name() {
        let mut wrong_generated_name = compile(NON_VARIABLE_SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_generated_name.definitions.functions[1].body[1]);
        let (_, _, _, record) = implicit_record_access_parts_mut(&mut arguments[0].value);
        let (name, _) = variable_parts_mut(record);
        *name = "wrong".into();
        assert_eq!(
            plan_module(wrong_generated_name),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ImplicitFieldTarget,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_generated_target_type() {
        let mut wrong_generated_type = compile(NON_VARIABLE_SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_generated_type.definitions.functions[1].body[1]);
        let (_, _, _, record) = implicit_record_access_parts_mut(&mut arguments[0].value);
        let (_, constructor) = variable_parts_mut(record);
        constructor.type_ = type_::int();
        assert_eq!(
            plan_module(wrong_generated_type),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ImplicitFieldTarget,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_generated_target_origin() {
        let mut wrong_generated_origin = compile(NON_VARIABLE_SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_generated_origin.definitions.functions[1].body[1]);
        let (_, _, _, record) = implicit_record_access_parts_mut(&mut arguments[0].value);
        let (_, constructor) = variable_parts_mut(record);
        constructor.variant = ValueConstructorVariant::LocalVariable {
            location: dummy_span(),
            origin: VariableOrigin {
                syntax: VariableSyntax::Variable("_record".into()),
                declaration: VariableDeclaration::LetPattern,
            },
        };
        assert_eq!(
            plan_module(wrong_generated_origin),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ImplicitFieldTarget,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_generated_target_variant() {
        let mut wrong_generated_variant = compile(NON_VARIABLE_SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut wrong_generated_variant.definitions.functions[1].body[1]);
        let (_, _, _, record) = implicit_record_access_parts_mut(&mut arguments[0].value);
        let (_, constructor) = variable_parts_mut(record);
        constructor.variant = ValueConstructorVariant::Record {
            name: "Person".into(),
            arity: 2,
            field_map: None,
            location: dummy_span(),
            module: "main".into(),
            variants_count: 1,
            variant_index: 0,
            documentation: None,
        };
        assert_eq!(
            plan_module(wrong_generated_variant),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ImplicitFieldTarget,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_non_variable_implicit_target() {
        let mut non_variable_target = compile(SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut non_variable_target.definitions.functions[0].body[1]);
        let (_, _, _, record) = implicit_record_access_parts_mut(&mut arguments[0].value);
        *record = TypedExpr::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: 1.into(),
            type_: type_::int(),
        };
        assert_eq!(
            plan_module(non_variable_target),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ImplicitFieldTarget,
            )),
        );
    }

    fn record_update_parts_mut(
        statement: &mut TypedStatement,
    ) -> (
        &mut Arc<Type>,
        &mut TypedExpr,
        &mut Option<ecow::EcoString>,
        &mut TypedExpr,
        &mut Vec<CallArg<TypedExpr>>,
    ) {
        let Statement::Expression(TypedExpr::RecordUpdate {
            type_,
            updated_record,
            updated_record_assigned_name,
            constructor,
            arguments,
            ..
        }) = statement
        else {
            panic!("statement should be a record update expression");
        };
        (
            type_,
            updated_record,
            updated_record_assigned_name,
            constructor,
            arguments,
        )
    }

    fn implicit_record_access_parts_mut(
        expression: &mut TypedExpr,
    ) -> (
        &mut Arc<Type>,
        &mut ecow::EcoString,
        &mut u64,
        &mut TypedExpr,
    ) {
        let TypedExpr::RecordAccess {
            type_,
            label,
            index,
            record,
            ..
        } = expression
        else {
            panic!("expression should be an implicit record access");
        };
        (type_, label, index, record)
    }

    fn variable_parts_mut(
        expression: &mut TypedExpr,
    ) -> (&mut ecow::EcoString, &mut ValueConstructor) {
        let TypedExpr::Var {
            name, constructor, ..
        } = expression
        else {
            panic!("expression should be a variable");
        };
        (name, constructor)
    }

    #[test]
    #[should_panic(expected = "statement should be a record update expression")]
    fn record_update_parts_mut_rejects_other_statements() {
        let mut module = compile(SOURCE);
        record_update_parts_mut(&mut module.definitions.functions[0].body[0]);
    }

    #[test]
    #[should_panic(expected = "expression should be an implicit record access")]
    fn implicit_record_access_parts_mut_rejects_other_expressions() {
        implicit_record_access_parts_mut(&mut TypedExpr::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: 1.into(),
            type_: type_::int(),
        });
    }

    #[test]
    #[should_panic(expected = "expression should be a variable")]
    fn variable_parts_mut_rejects_other_expressions() {
        variable_parts_mut(&mut TypedExpr::Int {
            location: dummy_span(),
            value: "1".into(),
            int_value: 1.into(),
            type_: type_::int(),
        });
    }
}
