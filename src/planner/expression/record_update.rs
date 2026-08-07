mod validation;

use crate::plan::{CustomExpr, CustomLocalId, Expr, Step};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use ecow::EcoString;
use gleam_core::ast::{CallArg, TypedExpr};
use gleam_core::type_::Type;
use std::sync::Arc;

pub(super) fn plan(
    type_: Arc<Type>,
    updated_record: TypedExpr,
    updated_record_assigned_name: Option<EcoString>,
    constructor: TypedExpr,
    arguments: Vec<CallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let validated = validation::validate(
        type_,
        updated_record,
        updated_record_assigned_name,
        constructor,
        arguments,
        context,
    )?;
    let (source, constructor, arguments) = validated.into_parts();

    let local = context.define_internal_custom_local();
    let typed_local = crate::plan::CustomLocal::from_shape(local, source.shape().clone());
    let local_name = internal_local_name(local);
    let step = Step::let_custom(local, local_name.clone(), source);
    let local_get = CustomExpr::local_get(typed_local, local_name);
    let arguments = arguments.plan(local_get, context)?;
    let construction = crate::plan::CustomConstruction::from_validated(constructor, arguments);
    context
        .custom_expr_from_construction(construction)
        .map(|result| Expr::custom(CustomExpr::block(vec![step], result)))
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
        InvalidCustomTypeReason, InvalidRecordUpdateShapeReason, InvalidTypedAstReason, PlanError,
        RecordUpdateArgumentOrigin, plan_module, plan_program,
    };
    use crate::{ModuleSource, compile_typed_program};
    use gleam_core::ast::{CallArg, ImplicitCallArgOrigin, Statement, TypedExpr, TypedStatement};
    use gleam_core::type_::error::{VariableDeclaration, VariableOrigin, VariableSyntax};
    use gleam_core::type_::{
        self, ModuleValueConstructor, Type, ValueConstructor, ValueConstructorVariant,
    };
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

    fn person_type() -> ValueType {
        ValueType::Custom(CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Person".into()),
            Vec::new(),
        ))
    }

    fn other_type() -> ValueType {
        ValueType::Custom(CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Other".into()),
            Vec::new(),
        ))
    }

    fn result_type() -> ValueType {
        ValueType::Custom(CustomType::new(
            CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
            vec![ValueType::Int, ValueType::Nil],
        ))
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
            crate::plan::CustomConstruction::from_validated(
                constructor,
                vec![projected_name, Expr::int(IntExpr::value(31.into()))],
            ),
        );

        assert_eq!(
            plan.main_function().return_(),
            &ReturnExpr::custom_body(CustomReturn::block(
                vec![Step::let_custom(local, local_name, source)],
                CustomReturn::with_signature_shape(
                    crate::plan::CustomValueShape::any(type_),
                    updated,
                ),
            )),
        );
    }

    #[test]
    fn qualified_record_update_constructor_matches_unqualified_import_plan() {
        let dependency = r#"
pub type Person {
  Person(name: String, age: Int)
}
"#;
        let qualified = compile_typed_program(
            "main",
            [
                ModuleSource::new("model", "model.gleam", dependency),
                ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
import model

pub fn main() {
  let person = model.Person(name: "Lucy", age: 30)
  model.Person(..person, age: 31)
}
"#,
                ),
            ],
        )
        .expect("qualified record update program should compile");
        let qualified = plan_program(qualified).expect("qualified record update should plan");
        let unqualified = compile_typed_program(
            "main",
            [
                ModuleSource::new("model", "model.gleam", dependency),
                ModuleSource::new(
                    "main",
                    "main.gleam",
                    r#"
import model.{Person}

pub fn main() {
  let person = Person(name: "Lucy", age: 30)
  Person(..person, age: 31)
}
"#,
                ),
            ],
        )
        .expect("unqualified record update program should compile");
        let unqualified = plan_program(unqualified).expect("unqualified record update should plan");

        assert_eq!(
            qualified.main_function().return_(),
            unqualified.main_function().return_(),
        );
    }

    #[test]
    fn reject_margin_record_update_module_select_label_mismatch() {
        let mut module = compile(SOURCE);
        let (_, _, _, constructor, _) =
            record_update_parts_mut(&mut module.definitions.functions[0].body[1]);
        let constructor_type = constructor.type_();
        *constructor = TypedExpr::ModuleSelect {
            location: dummy_span(),
            field_start: 0,
            type_: constructor_type.clone(),
            label: "Other".into(),
            module_name: "main".into(),
            module_alias: "main".into(),
            constructor: ModuleValueConstructor::Record {
                name: "Person".into(),
                variant_index: 0,
                arity: 2,
                type_: constructor_type,
                field_map: None,
                location: dummy_span(),
                documentation: None,
            },
        };

        assert_eq!(
            plan_module(module),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ConstructorName {
                    expected: "Other".into(),
                    actual: "Person".into(),
                },
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_direct_constructor_name_mismatch() {
        let mut module = compile(SOURCE);
        let (_, _, _, constructor, _) =
            record_update_parts_mut(&mut module.definitions.functions[0].body[1]);
        let (name, _) = variable_parts_mut(constructor);
        *name = "Other".into();

        assert_eq!(
            plan_module(module),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ConstructorName {
                    expected: "Other".into(),
                    actual: "Person".into(),
                },
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_module_select_constructor_metadata() {
        let mut variable_type_mismatch = compile(SOURCE);
        let (_, _, _, constructor, _) =
            record_update_parts_mut(&mut variable_type_mismatch.definitions.functions[0].body[1]);
        let (_, constructor) = variable_parts_mut(constructor);
        constructor.type_ = type_::int();
        assert_eq!(
            plan_module(variable_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "".into(),
                    module: "main".into(),
                    name: "Person".into(),
                    reason: Box::new(InvalidCustomTypeReason::ConstructorType {
                        actual: ValueType::Int,
                    }),
                },
            }),
        );

        let mut module_constructor = compile(SOURCE);
        let (_, _, _, constructor, _) =
            record_update_parts_mut(&mut module_constructor.definitions.functions[0].body[1]);
        let constructor_type = constructor.type_();
        *constructor = TypedExpr::ModuleSelect {
            location: dummy_span(),
            field_start: 0,
            type_: constructor_type.clone(),
            label: "Person".into(),
            module_name: "main".into(),
            module_alias: "main".into(),
            constructor: ModuleValueConstructor::Record {
                name: "Person".into(),
                variant_index: 0,
                arity: 1,
                type_: constructor_type,
                field_map: None,
                location: dummy_span(),
                documentation: None,
            },
        };
        assert_eq!(
            plan_module(module_constructor),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Person".into(),
                    reason: Box::new(InvalidCustomTypeReason::ConstructorArity {
                        expected: 2,
                        actual: 1,
                    }),
                },
            }),
        );

        let mut module_mismatch = compile(SOURCE);
        let (_, _, _, constructor, _) =
            record_update_parts_mut(&mut module_mismatch.definitions.functions[0].body[1]);
        let constructor_type = constructor.type_();
        *constructor = TypedExpr::ModuleSelect {
            location: dummy_span(),
            field_start: 0,
            type_: constructor_type.clone(),
            label: "Person".into(),
            module_name: "other".into(),
            module_alias: "other".into(),
            constructor: ModuleValueConstructor::Record {
                name: "Person".into(),
                variant_index: 0,
                arity: 2,
                type_: constructor_type,
                field_map: None,
                location: dummy_span(),
                documentation: None,
            },
        };
        assert_eq!(
            plan_module(module_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Person".into(),
                    reason: Box::new(InvalidCustomTypeReason::ConstructorModule {
                        expected: "main".into(),
                        actual: "other".into(),
                    }),
                },
            }),
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
            crate::plan::CustomConstruction::from_validated(
                constructor,
                vec![
                    projected_value,
                    Expr::string(StringExpr::value("two".into())),
                ],
            ),
        );

        assert_eq!(
            plan.main_function().return_(),
            &ReturnExpr::custom_body(CustomReturn::block(
                vec![Step::let_custom(local, local_name, source)],
                CustomReturn::with_signature_shape(
                    crate::plan::CustomValueShape::any(type_),
                    updated,
                ),
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
            crate::plan::CustomConstruction::from_validated(
                constructor,
                vec![
                    Expr::string(StringExpr::value("Mia".into())),
                    Expr::int(IntExpr::value(31.into())),
                ],
            ),
        );

        assert_eq!(
            plan.main_function().return_(),
            &ReturnExpr::custom_body(CustomReturn::block(
                vec![Step::let_custom(local, local_name, source)],
                CustomReturn::with_signature_shape(
                    crate::plan::CustomValueShape::any(type_),
                    updated,
                ),
            )),
        );
    }

    #[test]
    fn plan_record_update_changes_generic_result_type() {
        let plan = plan_module(compile(
            r#"
pub type Pair(first, second) {
  Pair(first: first, second: second)
}

fn replace_first(pair: Pair(first, second), value: replacement) -> Pair(replacement, second) {
  Pair(..pair, first: value)
}

pub fn main() {
  replace_first(Pair(first: 1, second: True), "one")
}
"#,
        ))
        .expect("generic type-changing record update should plan");
        let replace_first = plan
            .functions()
            .iter()
            .find(|function| function.name() == "replace_first")
            .expect("replace_first should be planned");
        let pair_type = |first, second| {
            ValueType::Custom(CustomType::new(
                CustomTypeName::new("geam".into(), "main".into(), "Pair".into()),
                vec![ValueType::Parameter(first), ValueType::Parameter(second)],
            ))
        };

        assert_eq!(
            replace_first.params()[0].shape().value_type(),
            pair_type(
                crate::plan::TypeParameterId(2),
                crate::plan::TypeParameterId(1),
            ),
        );
        assert_eq!(
            replace_first.params()[1].shape().value_type(),
            ValueType::Parameter(crate::plan::TypeParameterId(0)),
        );
        assert_eq!(
            replace_first.return_().value_type(),
            pair_type(
                crate::plan::TypeParameterId(0),
                crate::plan::TypeParameterId(1),
            ),
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
                InvalidRecordUpdateShapeReason::BaseAssignment {
                    requires_assignment: false,
                    has_assignment: true,
                }
            )),
        );

        let mut unassigned_expression = compile(NON_VARIABLE_SOURCE);
        let (_, _, assigned_name, _, _) =
            record_update_parts_mut(&mut unassigned_expression.definitions.functions[1].body[1]);
        *assigned_name = None;
        assert_eq!(
            plan_module(unassigned_expression),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::BaseAssignment {
                    requires_assignment: true,
                    has_assignment: false,
                }
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
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ConstructorExpression,
            )),
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
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ConstructorResultType {
                    expected: person_type(),
                    actual: ValueType::Int,
                },
            )),
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
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ConstructorResultType {
                    expected: person_type(),
                    actual: result_type(),
                },
            )),
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
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ConstructorKind,
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_non_constructor_module_select() {
        let mut non_record_module_select = compile(SOURCE);
        let (_, _, _, constructor, _) =
            record_update_parts_mut(&mut non_record_module_select.definitions.functions[0].body[1]);
        let type_ = constructor.type_();
        *constructor = TypedExpr::ModuleSelect {
            location: dummy_span(),
            field_start: 0,
            type_,
            label: "value".into(),
            module_name: "main".into(),
            module_alias: "main".into(),
            constructor: ModuleValueConstructor::Constant {
                literal: gleam_core::ast::Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                },
                location: dummy_span(),
                documentation: None,
            },
        };
        assert_eq!(
            plan_module(non_record_module_select),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ConstructorKind,
            )),
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
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::UpdatedSourceFamily {
                    actual: ValueType::Int,
                },
            )),
        );
    }

    #[test]
    fn reject_margin_record_update_invalid_base_expression() {
        let mut invalid_base_expression = compile(NON_VARIABLE_SOURCE);
        let (_, updated_record, _, _, _) =
            record_update_parts_mut(&mut invalid_base_expression.definitions.functions[1].body[1]);
        let type_ = updated_record.type_();
        *updated_record = TypedExpr::Invalid {
            location: dummy_span(),
            type_,
            extra_information: None,
        };
        assert_eq!(
            plan_module(invalid_base_expression),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::InvalidExpressionNode,
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
        let (_, updated_record, _, _, arguments) =
            record_update_parts_mut(&mut non_custom_local.definitions.functions[0].body[2]);
        let (name, _) = variable_parts_mut(updated_record);
        *name = "number".into();
        let (_, _, _, target) = implicit_record_access_parts_mut(&mut arguments[0].value);
        let (name, _) = variable_parts_mut(target);
        *name = "number".into();
        assert_eq!(
            plan_module(non_custom_local),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::UpdatedSourceType {
                    expected: person_type(),
                    actual: ValueType::Int,
                },
            )),
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
        let (_, updated_record, _, _, arguments) =
            record_update_parts_mut(&mut wrong_custom_local.definitions.functions[0].body[2]);
        let (name, _) = variable_parts_mut(updated_record);
        *name = "other".into();
        let (_, _, _, target) = implicit_record_access_parts_mut(&mut arguments[0].value);
        let (name, _) = variable_parts_mut(target);
        *name = "other".into();
        assert_eq!(
            plan_module(wrong_custom_local),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::UpdatedSourceType {
                    expected: person_type(),
                    actual: other_type(),
                },
            )),
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
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ArgumentCount {
                    expected: 2,
                    actual: 1,
                }
            )),
        );

        let mut extra_argument = compile(SOURCE);
        let (_, _, _, _, arguments) =
            record_update_parts_mut(&mut extra_argument.definitions.functions[0].body[1]);
        arguments.push(arguments[0].clone());
        assert_eq!(
            plan_module(extra_argument),
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ArgumentCount {
                    expected: 2,
                    actual: 3,
                }
            )),
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
            Err(invalid_shape(
                InvalidRecordUpdateShapeReason::ArgumentLabel {
                    index: 0,
                    expected: Some("name".into()),
                    actual: Some("wrong".into()),
                }
            )),
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
                reason: InvalidTypedAstReason::ExpressionValueTypeMismatch {
                    expected: ValueType::Int,
                    actual: ValueType::String,
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
                reason: InvalidTypedAstReason::InvalidExpressionNode,
            }),
        );
    }

    #[test]
    fn reject_margin_record_update_implicit_argument_origin() {
        for (origin, expected) in [
            (
                ImplicitCallArgOrigin::IncorrectArityUse,
                RecordUpdateArgumentOrigin::IncorrectArityUse,
            ),
            (
                ImplicitCallArgOrigin::PatternFieldSpread,
                RecordUpdateArgumentOrigin::PatternFieldSpread,
            ),
            (
                ImplicitCallArgOrigin::Pipe,
                RecordUpdateArgumentOrigin::Pipe,
            ),
            (ImplicitCallArgOrigin::Use, RecordUpdateArgumentOrigin::Use),
        ] {
            let mut wrong_origin = compile(SOURCE);
            let (_, _, _, _, arguments) =
                record_update_parts_mut(&mut wrong_origin.definitions.functions[0].body[1]);
            arguments[0].implicit = Some(origin);
            assert_eq!(
                plan_module(wrong_origin),
                Err(invalid_shape(
                    InvalidRecordUpdateShapeReason::ImplicitArgumentOrigin {
                        index: 0,
                        actual: expected,
                    },
                )),
            );
        }
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
                InvalidRecordUpdateShapeReason::ImplicitFieldExpression { argument: 0 },
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
                InvalidRecordUpdateShapeReason::ImplicitFieldIndex {
                    argument: 0,
                    expected: 0,
                    actual: 1,
                }
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
                InvalidRecordUpdateShapeReason::ImplicitFieldLabel {
                    argument: 0,
                    expected: Some("name".into()),
                    actual: Some("wrong".into()),
                }
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
                InvalidRecordUpdateShapeReason::ImplicitFieldType {
                    argument: 0,
                    expected: ValueType::String,
                    actual: ValueType::Int,
                }
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
                InvalidRecordUpdateShapeReason::ImplicitTargetName {
                    argument: 0,
                    expected: "person".into(),
                    actual: "wrong".into(),
                }
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
                InvalidRecordUpdateShapeReason::ImplicitOriginalTargetConstructor { argument: 0 },
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
                InvalidRecordUpdateShapeReason::ImplicitTargetName {
                    argument: 0,
                    expected: "_record".into(),
                    actual: "wrong".into(),
                }
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
                InvalidRecordUpdateShapeReason::ImplicitGeneratedTargetType {
                    argument: 0,
                    expected: person_type(),
                    actual: ValueType::Int,
                },
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
                InvalidRecordUpdateShapeReason::ImplicitGeneratedTargetOrigin { argument: 0 },
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
                InvalidRecordUpdateShapeReason::ImplicitGeneratedTargetKind { argument: 0 },
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
                InvalidRecordUpdateShapeReason::ImplicitTargetExpression { argument: 0 },
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
