use crate::plan::execution::{CustomConstructorId, CustomFieldAccess, ExecutionPlan};
use crate::runtime::ExecutionError;
use crate::runtime::evaluated::EvaluatedValue;
use crate::runtime::frame::Frame;
use crate::runtime::state::RuntimeState;

pub(in crate::runtime) fn eval_custom_field(
    plan: &ExecutionPlan,
    state: &mut RuntimeState,
    frame: &mut Frame,
    access: &CustomFieldAccess,
) -> Result<(CustomConstructorId, EvaluatedValue), ExecutionError> {
    let value = super::eval_custom_expr(plan, state, frame, access.source())?;
    let field = value.fields()[access.index()].clone();
    Ok((value.constructor(), field))
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        CustomConstructor, CustomConstructorDefinition, CustomConstructorField, CustomExpr,
        CustomFieldAccess, CustomFieldDefinition, CustomType, CustomTypeDefinition, CustomTypeName,
        CustomTypePublicity, CustomTypeTemplate, Expr, FunctionExpr, FunctionFunctionExpr,
        FunctionFunctionId, FunctionFunctionReference, FunctionPlan, FunctionReference,
        FunctionType, IntExpr, IntFunctionFunctionId, IntFunctionId, IntLocalId, ListExpr,
        ModulePlan, ParamLocal, ReturnBody, ReturnExpr, RuntimeFunctionId, StringExpr,
        StringFunctionId, TupleExpr, ValueType,
    };
    use crate::plan::{
        CustomFunctionExpr, CustomFunctionId, CustomFunctionReference, FunctionId, TupleFunctionId,
    };
    use crate::runtime::{
        BitArrayValue, CustomFieldValue, CustomValue, ExecutionError, ListValue, Value,
    };

    #[test]
    fn typed_custom_field_projections_succeed_for_every_value_and_function_family() {
        let plan = crate::runtime::plan_src(include_str!(
            "../../../tests/fixtures/execution/expressions/record_access_families.gleam"
        ));
        let inner_type = CustomType::new(inner_name(), Vec::new());
        let inner = || {
            Value::Custom(CustomValue::from_evaluated(
                inner_type.clone(),
                "Inner".into(),
                0,
                vec![CustomFieldValue::from_evaluated(None, Value::Int(2.into()))],
            ))
        };

        assert_eq!(
            crate::run_main(&plan),
            Ok(Value::Tuple(vec![
                Value::Int(1.into()),
                Value::Float(1.5),
                Value::String("one".into()),
                Value::BitArray(BitArrayValue::from_bytes(vec![1])),
                Value::UtfCodepoint('A'),
                inner(),
                Value::Bool(true),
                Value::Nil,
                Value::Tuple(vec![Value::Int(3.into())]),
                Value::List(ListValue::int(vec![4.into()])),
                Value::Int(6.into()),
                Value::Float(1.5),
                Value::String("one".into()),
                Value::BitArray(BitArrayValue::from_bytes(vec![1])),
                Value::UtfCodepoint('A'),
                inner(),
                Value::Bool(true),
                Value::Nil,
                Value::Tuple(vec![Value::Int(3.into())]),
                Value::List(ListValue::int(vec![4.into()])),
                Value::Int(7.into()),
            ])),
        );
    }

    #[test]
    fn custom_field_projection_propagates_source_invariants() {
        let field = CustomConstructorField::new(Some("value".into()), ValueType::String);
        let boxed = CustomConstructor::new(boxed_type(), "Boxed".into(), 0, vec![field.clone()]);
        let source_error_access = CustomFieldAccess::new(
            CustomExpr::try_constructor(
                boxed.clone(),
                vec![Expr::string(StringExpr::tuple_index(
                    TupleExpr::value(
                        vec![Expr::int(IntExpr::value(1.into()))],
                        vec![ValueType::String],
                    ),
                    0,
                ))],
            )
            .expect("test custom construction should be valid"),
            0,
            Some("value".into()),
        );
        assert_eq!(
            run_field_projection_module(
                Expr::string(StringExpr::custom_field(source_error_access)),
                vec![CustomTypeDefinition::new(
                    boxed_name(),
                    CustomTypePublicity::Private,
                    false,
                    Vec::new(),
                    vec![CustomConstructorDefinition::new(
                        "Boxed".into(),
                        0,
                        vec![CustomFieldDefinition::new(
                            Some("value".into()),
                            CustomTypeTemplate::String,
                        )],
                    )],
                )],
            )
            .expect_err("direct-mutated tuple field should fail"),
            ExecutionError::TupleIndexFamilyMismatch {
                expected: ValueType::String,
                actual: ValueType::Int,
            },
        );
    }

    #[test]
    fn typed_custom_field_projections_report_every_family_mismatch() {
        let inner_type = CustomType::new(inner_name(), Vec::new());
        let int_function = FunctionType::new(Vec::new(), ValueType::Int);
        let expected_types = vec![
            ValueType::Int,
            ValueType::String,
            ValueType::BitArray,
            ValueType::UtfCodepoint,
            ValueType::Custom(inner_type.clone()),
            ValueType::Float,
            ValueType::Bool,
            ValueType::Nil,
            ValueType::Tuple(vec![ValueType::Int]),
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::List(Box::new(ValueType::String)),
            ValueType::List(Box::new(ValueType::BitArray)),
            ValueType::List(Box::new(ValueType::UtfCodepoint)),
            ValueType::List(Box::new(ValueType::Custom(inner_type.clone()))),
            ValueType::List(Box::new(ValueType::Float)),
            ValueType::List(Box::new(ValueType::Bool)),
            ValueType::List(Box::new(ValueType::Nil)),
            ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::String)))),
            ValueType::List(Box::new(ValueType::Function(Box::new(
                int_function.clone(),
            )))),
            ValueType::Function(Box::new(int_function.clone())),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::String))),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::BitArray))),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::UtfCodepoint,
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Custom(inner_type),
            ))),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Float))),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
            ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Nil))),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Tuple(vec![ValueType::Int]),
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::List(Box::new(ValueType::Int)),
            ))),
            ValueType::Function(Box::new(FunctionType::new(
                Vec::new(),
                ValueType::Function(Box::new(int_function)),
            ))),
        ];

        for expected in expected_types {
            let (actual, actual_value) = match &expected {
                ValueType::Int => (
                    ValueType::String,
                    Expr::string(StringExpr::value("wrong".into())),
                ),
                ValueType::List(item) if item.as_ref() == &ValueType::Int => (
                    ValueType::List(Box::new(ValueType::String)),
                    Expr::list(ListExpr::value(Vec::new(), ValueType::String)),
                ),
                ValueType::List(_) => (
                    ValueType::List(Box::new(ValueType::Int)),
                    Expr::list(ListExpr::value(Vec::new(), ValueType::Int)),
                ),
                ValueType::Function(function) if function.return_() == &ValueType::Int => (
                    ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::String))),
                    Expr::function(FunctionExpr::reference(FunctionReference::new(
                        RuntimeFunctionId::String(StringFunctionId(0)),
                        Vec::new(),
                    ))),
                ),
                ValueType::Function(_) => (
                    ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
                    Expr::function(FunctionExpr::reference(FunctionReference::new(
                        RuntimeFunctionId::Int(IntFunctionId(0)),
                        Vec::new(),
                    ))),
                ),
                _ => (ValueType::Int, Expr::int(IntExpr::value(1.into()))),
            };
            let constructor = CustomConstructor::new(
                boxed_type(),
                "Boxed".into(),
                0,
                vec![CustomConstructorField::new(
                    Some("value".into()),
                    expected.clone(),
                )],
            );
            let access = CustomFieldAccess::new(
                CustomExpr::try_constructor(constructor.clone(), vec![actual_value])
                    .expect("test custom construction should be valid"),
                0,
                Some("value".into()),
            );
            assert_eq!(
                run_field_projection_module(
                    Expr::custom_field_shape(
                        access,
                        crate::plan::ValueShape::from_value_type(expected.clone()),
                    ),
                    field_projection_definitions(&expected),
                ),
                Err(ExecutionError::CustomFieldFamilyMismatch {
                    custom_type: boxed_type(),
                    constructor: "Boxed".into(),
                    field_index: 0,
                    expected: expected.clone(),
                    actual,
                }),
            );
            let (outer_actual, outer_actual_type) = match &expected {
                ValueType::List(_) | ValueType::Function(_) => {
                    (Expr::int(IntExpr::value(1.into())), ValueType::Int)
                }
                _ => {
                    let function_type = FunctionType::new(Vec::new(), ValueType::Int);
                    (
                        Expr::function(FunctionExpr::reference(FunctionReference::new(
                            RuntimeFunctionId::Int(IntFunctionId(0)),
                            Vec::new(),
                        ))),
                        ValueType::Function(Box::new(function_type)),
                    )
                }
            };
            let constructor = CustomConstructor::new(
                boxed_type(),
                "Boxed".into(),
                0,
                vec![CustomConstructorField::new(
                    Some("value".into()),
                    expected.clone(),
                )],
            );
            let access = CustomFieldAccess::new(
                CustomExpr::try_constructor(constructor.clone(), vec![outer_actual])
                    .expect("test custom construction should be valid"),
                0,
                Some("value".into()),
            );
            assert_eq!(
                run_field_projection_module(
                    Expr::custom_field_shape(
                        access,
                        crate::plan::ValueShape::from_value_type(expected.clone()),
                    ),
                    field_projection_definitions(&expected),
                ),
                Err(ExecutionError::CustomFieldFamilyMismatch {
                    custom_type: boxed_type(),
                    constructor: "Boxed".into(),
                    field_index: 0,
                    expected: expected.clone(),
                    actual: outer_actual_type,
                }),
            );
            let access = CustomFieldAccess::new(
                CustomExpr::tuple_index_shape(
                    TupleExpr::value(
                        vec![Expr::int(IntExpr::value(1.into()))],
                        vec![ValueType::Custom(boxed_type())],
                    ),
                    0,
                    crate::plan::CustomValueShape::any(boxed_type()),
                ),
                0,
                Some("value".into()),
            );
            assert_eq!(
                run_field_projection_module(
                    Expr::custom_field_shape(
                        access,
                        crate::plan::ValueShape::from_value_type(expected.clone()),
                    ),
                    field_projection_definitions(&expected),
                ),
                Err(ExecutionError::TupleIndexFamilyMismatch {
                    expected: ValueType::Custom(boxed_type()),
                    actual: ValueType::Int,
                }),
            );
        }
    }

    #[test]
    fn custom_function_field_projection_rejects_same_family_signature_mismatch() {
        let inner_type = CustomType::new(inner_name(), Vec::new());
        let expected_function =
            FunctionType::new(Vec::new(), ValueType::Custom(inner_type.clone()));
        let actual_function =
            FunctionType::new(vec![ValueType::Int], ValueType::Custom(inner_type.clone()));
        let constructor = CustomConstructor::new(
            boxed_type(),
            "Boxed".into(),
            0,
            vec![CustomConstructorField::new(
                Some("value".into()),
                ValueType::Function(Box::new(expected_function.clone())),
            )],
        );
        let actual_value = Expr::function(FunctionExpr::custom(CustomFunctionExpr::reference(
            CustomFunctionReference::new(
                CustomFunctionId::new(0, inner_type),
                vec![ParamLocal::int(IntLocalId(0))],
            ),
        )));
        let access = CustomFieldAccess::new(
            CustomExpr::try_constructor(constructor.clone(), vec![actual_value])
                .expect("test custom construction should be valid"),
            0,
            Some("value".into()),
        );

        assert_eq!(
            run_field_projection_module(
                Expr::custom_field_shape(
                    access,
                    crate::plan::ValueShape::from_value_type(ValueType::Function(Box::new(
                        expected_function.clone(),
                    ))),
                ),
                field_projection_definitions(&ValueType::Function(Box::new(
                    expected_function.clone(),
                ))),
            ),
            Err(ExecutionError::CustomFieldFamilyMismatch {
                custom_type: boxed_type(),
                constructor: "Boxed".into(),
                field_index: 0,
                expected: ValueType::Function(Box::new(expected_function)),
                actual: ValueType::Function(Box::new(actual_function)),
            }),
        );
    }

    #[test]
    fn function_function_field_projection_rejects_same_family_signature_mismatch() {
        let returned = FunctionType::new(Vec::new(), ValueType::Int);
        let expected_function =
            FunctionType::new(Vec::new(), ValueType::Function(Box::new(returned.clone())));
        let actual_function = FunctionType::new(
            vec![ValueType::Int],
            ValueType::Function(Box::new(returned.clone())),
        );
        let constructor = CustomConstructor::new(
            boxed_type(),
            "Boxed".into(),
            0,
            vec![CustomConstructorField::new(
                Some("value".into()),
                ValueType::Function(Box::new(expected_function.clone())),
            )],
        );
        let actual_value = Expr::function(FunctionExpr::function(FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                vec![ParamLocal::int(IntLocalId(0))],
            ),
            returned,
        )));
        let access = CustomFieldAccess::new(
            CustomExpr::try_constructor(constructor.clone(), vec![actual_value])
                .expect("test custom construction should be valid"),
            0,
            Some("value".into()),
        );

        assert_eq!(
            run_field_projection_module(
                Expr::custom_field_shape(
                    access,
                    crate::plan::ValueShape::from_value_type(ValueType::Function(Box::new(
                        expected_function.clone(),
                    ))),
                ),
                field_projection_definitions(&ValueType::Function(Box::new(
                    expected_function.clone(),
                ))),
            ),
            Err(ExecutionError::CustomFieldFamilyMismatch {
                custom_type: boxed_type(),
                constructor: "Boxed".into(),
                field_index: 0,
                expected: ValueType::Function(Box::new(expected_function)),
                actual: ValueType::Function(Box::new(actual_function)),
            }),
        );
    }

    fn field_projection_definitions(expected: &ValueType) -> Vec<CustomTypeDefinition> {
        vec![
            CustomTypeDefinition::new(
                inner_name(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Inner".into(),
                    0,
                    Vec::new(),
                )],
            ),
            CustomTypeDefinition::new(
                boxed_name(),
                CustomTypePublicity::Private,
                false,
                Vec::new(),
                vec![CustomConstructorDefinition::new(
                    "Boxed".into(),
                    0,
                    vec![CustomFieldDefinition::new(
                        Some("value".into()),
                        custom_type_template(expected),
                    )],
                )],
            ),
        ]
    }

    fn run_field_projection_module(
        expression: Expr,
        definitions: Vec<CustomTypeDefinition>,
    ) -> Result<Value, ExecutionError> {
        let return_type = expression.value_type();
        let main = FunctionPlan::new(
            FunctionId::new(0),
            "main".into(),
            Vec::new(),
            Vec::new(),
            ReturnExpr::tuple_body(
                TupleFunctionId(0),
                vec![return_type.clone()],
                ReturnBody::expr(TupleExpr::value(vec![expression], vec![return_type])),
            ),
        );
        let module =
            ModulePlan::new("main".into(), main, Vec::new()).with_custom_types(definitions);
        let plan = crate::ExecutionPlan::from_module_plan(module);

        crate::run_main(&plan)
    }

    fn custom_type_template(type_: &ValueType) -> CustomTypeTemplate {
        match type_ {
            ValueType::Int => CustomTypeTemplate::Int,
            ValueType::Float => CustomTypeTemplate::Float,
            ValueType::String => CustomTypeTemplate::String,
            ValueType::BitArray => CustomTypeTemplate::BitArray,
            ValueType::UtfCodepoint => CustomTypeTemplate::UtfCodepoint,
            ValueType::Bool => CustomTypeTemplate::Bool,
            ValueType::Nil => CustomTypeTemplate::Nil,
            ValueType::Tuple(elements) => {
                CustomTypeTemplate::Tuple(elements.iter().map(custom_type_template).collect())
            }
            ValueType::List(item) => CustomTypeTemplate::List(Box::new(custom_type_template(item))),
            ValueType::Function(function) => CustomTypeTemplate::Function {
                arguments: function
                    .argument_types()
                    .iter()
                    .map(custom_type_template)
                    .collect(),
                return_: Box::new(custom_type_template(function.return_())),
            },
            ValueType::Custom(custom) => CustomTypeTemplate::Custom {
                name: custom.type_name().clone(),
                arguments: custom
                    .arguments()
                    .iter()
                    .map(custom_type_template)
                    .collect(),
            },
        }
    }

    fn boxed_name() -> CustomTypeName {
        CustomTypeName::new("geam".into(), "main".into(), "Boxed".into())
    }

    fn boxed_type() -> CustomType {
        CustomType::new(boxed_name(), Vec::new())
    }

    fn inner_name() -> CustomTypeName {
        CustomTypeName::new("geam".into(), "main".into(), "Inner".into())
    }
}
