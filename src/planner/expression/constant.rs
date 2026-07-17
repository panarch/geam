use crate::plan::{
    BoolExpr, Expr, FloatExpr, FunctionExpr, IntExpr, ListExpr, NilExpr, StringExpr, TupleExpr,
    ValueType,
};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidExpressionType, InvalidTypedAstReason, PlanError,
    UnsupportedExpressionKind,
};
use gleam_core::ast::Constant;
use gleam_core::type_::{PRELUDE_MODULE_NAME, Type, ValueConstructor, ValueConstructorVariant};
use std::sync::Arc;

pub(in crate::planner::expression) fn plan(
    literal: Constant<Arc<Type>>,
    context: &PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match literal {
        Constant::Int { int_value, .. } => Ok(Expr::int(IntExpr::value(int_value))),
        Constant::Float { float_value, .. } => {
            Ok(Expr::float(FloatExpr::value(float_value.value())))
        }
        Constant::String { value, .. } => Ok(Expr::string(StringExpr::value(value))),
        Constant::StringConcatenation { left, right, .. } => {
            let left = plan_string(*left, context)?;
            let right = plan_string(*right, context)?;

            Ok(Expr::string(StringExpr::concatenate(left, right)))
        }
        Constant::Tuple {
            elements, type_, ..
        } => plan_tuple(elements, type_, context),
        Constant::List {
            elements,
            tail,
            type_,
            ..
        } => plan_list(elements, tail.map(|tail| *tail), type_, context),
        Constant::Var { constructor, .. } => {
            plan_var(constructor.map(|constructor| *constructor), context)
        }
        Constant::Record {
            arguments,
            record_constructor,
            ..
        } => plan_record(
            arguments,
            record_constructor.map(|constructor| *constructor),
            context,
        ),
        Constant::BitArray { segments, .. } => {
            super::bit_array::plan_constant(segments, context).map(Expr::bit_array)
        }
        Constant::RecordUpdate { .. } => {
            invalid_expression_shape(InvalidExpressionShapeKind::RecordUpdate)
        }
        Constant::Todo { .. } | Constant::Invalid { .. } => {
            invalid_expression_shape(InvalidExpressionShapeKind::Invalid)
        }
    }
}

fn plan_string(
    literal: Constant<Arc<Type>>,
    context: &PlanContext<'_>,
) -> Result<StringExpr, PlanError> {
    let expression = plan(literal, context)?;
    let actual = expression.value_type();
    match expression.into_string() {
        Some(expression) => Ok(expression),
        None => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::String,
                actual: InvalidExpressionType::from_value_type(actual),
            },
        }),
    }
}

fn plan_tuple(
    elements: Vec<Constant<Arc<Type>>>,
    type_: Arc<Type>,
    context: &PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let planned_elements = elements
        .into_iter()
        .map(|element| plan(element, context))
        .collect::<Result<Vec<_>, _>>()?;
    let actual_type = planned_elements
        .iter()
        .map(Expr::value_type)
        .collect::<Vec<_>>();
    let expected_type = match ValueType::from_gleam(type_.as_ref()) {
        Some(ValueType::Tuple(type_)) => type_,
        Some(actual) => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Tuple,
                    actual: InvalidExpressionType::from_value_type(actual),
                },
            });
        }
        None => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Tuple,
                    actual: InvalidExpressionType::Unsupported,
                },
            });
        }
    };

    if expected_type != actual_type {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::Tuple,
                actual: InvalidExpressionType::Tuple,
            },
        });
    }

    Ok(Expr::tuple(TupleExpr::value(
        planned_elements,
        expected_type,
    )))
}

fn plan_list(
    elements: Vec<Constant<Arc<Type>>>,
    tail: Option<Constant<Arc<Type>>>,
    type_: Arc<Type>,
    context: &PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let planned_elements = elements
        .into_iter()
        .map(|element| plan(element, context))
        .collect::<Result<Vec<_>, _>>()?;

    let Some(list_element_type) = type_.list_type() else {
        return match ValueType::from_gleam(type_.as_ref()) {
            Some(actual) => Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::from_value_type(actual),
                },
            }),
            None => Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Unsupported,
                },
            }),
        };
    };
    let expected_element_type = match ValueType::from_gleam(list_element_type.as_ref()) {
        Some(type_) => type_,
        None => {
            return Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::UnsupportedListElementType,
            });
        }
    };

    let Some(tail) = tail else {
        let list = match ListExpr::try_value(planned_elements, expected_element_type) {
            Ok(list) => list,
            Err(error) => {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::from_value_type(error.expected),
                        actual: InvalidExpressionType::from_value_type(error.actual),
                    },
                });
            }
        };
        return Ok(Expr::list(list));
    };
    let tail = plan(tail, context)?;
    let actual = tail.value_type();
    let Some(tail) = tail.into_list() else {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionType {
                expected: InvalidExpressionType::List,
                actual: InvalidExpressionType::from_value_type(actual),
            },
        });
    };
    let elements = match crate::plan::ListElements::from_exprs(
        expected_element_type.clone(),
        planned_elements,
    ) {
        Ok(elements) => elements,
        Err(error) => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::from_value_type(error.expected),
                    actual: InvalidExpressionType::from_value_type(error.actual),
                },
            });
        }
    };
    let elements = match crate::plan::ListSpreadElements::from_parts(elements, tail) {
        Ok(elements) => elements,
        Err(_) => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::List,
                },
            });
        }
    };
    Ok(Expr::list(ListExpr::from_spread_elements(elements)))
}

fn plan_var(
    constructor: Option<ValueConstructor>,
    context: &PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let Some(constructor) = constructor else {
        return invalid_expression_shape(InvalidExpressionShapeKind::Invalid);
    };

    match constructor.variant {
        ValueConstructorVariant::ModuleConstant {
            module, literal, ..
        } if module == *context.module_name => plan(literal, context),
        ValueConstructorVariant::ModuleFn {
            module,
            name,
            external_erlang,
            external_javascript,
            ..
        } if module == *context.module_name
            && external_erlang.is_none()
            && external_javascript.is_none() =>
        {
            let Some(function) = context.lookup_function(&name) else {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::UnknownLocal { name },
                });
            };

            Ok(Expr::function(FunctionExpr::reference(
                function.reference(function.signature.identity_instantiation()),
            )))
        }
        ValueConstructorVariant::ModuleConstant { .. }
        | ValueConstructorVariant::ModuleFn { .. } => {
            invalid_expression_shape(InvalidExpressionShapeKind::ModuleSelect)
        }
        ValueConstructorVariant::Record { .. } => plan_record_constructor(constructor, context),
        ValueConstructorVariant::LocalVariable { .. } => {
            invalid_expression_shape(InvalidExpressionShapeKind::Invalid)
        }
    }
}

fn plan_record(
    arguments: Option<Vec<gleam_core::ast::CallArg<Constant<Arc<Type>>>>>,
    constructor: Option<ValueConstructor>,
    context: &PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let Some(constructor) = constructor else {
        return invalid_expression_shape(InvalidExpressionShapeKind::RecordConstructor);
    };

    let ValueConstructorVariant::Record { .. } = &constructor.variant else {
        return invalid_expression_shape(InvalidExpressionShapeKind::Invalid);
    };
    let Some(arguments) = arguments else {
        return plan_record_constructor(constructor, context);
    };
    if arguments.is_empty()
        && matches!(
            &constructor.variant,
            ValueConstructorVariant::Record {
                name,
                module,
                arity: 0,
                ..
            } if module == PRELUDE_MODULE_NAME
                && matches!(name.as_str(), "True" | "False" | "Nil")
        )
    {
        return plan_record_constructor(constructor, context);
    }
    let constructor = context.custom_constructor(&constructor)?;
    let arguments = arguments
        .into_iter()
        .enumerate()
        .map(|(index, argument)| {
            let Some(field) = constructor.fields().get(index) else {
                return invalid_expression_shape(InvalidExpressionShapeKind::RecordConstructor);
            };
            if let Some(label) = &argument.label
                && field.label() != Some(label)
            {
                return invalid_expression_shape(InvalidExpressionShapeKind::RecordConstructor);
            }
            let argument = plan(argument.value, context)?;
            if argument.value_type() != *field.type_() {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionType {
                        expected: InvalidExpressionType::from_value_type(field.type_().clone()),
                        actual: InvalidExpressionType::from_value_type(argument.value_type()),
                    },
                });
            }
            Ok(argument)
        })
        .collect::<Result<Vec<_>, _>>()?;
    let construction =
        crate::plan::CustomConstruction::try_new(constructor, arguments).map_err(|_| {
            PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }
        })?;
    context
        .custom_expr_from_construction(construction)
        .map(Expr::custom)
}

fn plan_record_constructor(
    constructor: ValueConstructor,
    context: &PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let ValueConstructorVariant::Record {
        name,
        module,
        arity,
        ..
    } = &constructor.variant
    else {
        return invalid_expression_shape(InvalidExpressionShapeKind::Invalid);
    };
    if module == PRELUDE_MODULE_NAME && *arity == 0 {
        match name.as_str() {
            "True" => return Ok(Expr::bool(BoolExpr::value(true))),
            "False" => return Ok(Expr::bool(BoolExpr::value(false))),
            "Nil" => return Ok(Expr::nil(NilExpr::value())),
            _ => {
                return invalid_expression_shape(InvalidExpressionShapeKind::PreludeConstructor);
            }
        }
    }
    if module != context.module_name
        && !(module == PRELUDE_MODULE_NAME && matches!(name.as_str(), "Ok" | "Error"))
    {
        return invalid_expression_shape(InvalidExpressionShapeKind::RecordConstructor);
    }
    let shape = crate::plan::ValueShape::from_gleam(constructor.type_.as_ref()).ok_or(
        PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::GenericFunction,
        },
    )?;
    let constructor = context.custom_constructor(&constructor)?;
    if usize::from(*arity) != constructor.fields().len() {
        return invalid_expression_shape(InvalidExpressionShapeKind::RecordConstructor);
    }
    crate::plan::module::custom_constructor_expr(constructor)
        .with_shape(shape)
        .ok_or(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::RecordConstructor,
            },
        })
}

fn invalid_expression_shape(kind: InvalidExpressionShapeKind) -> Result<Expr, PlanError> {
    Err(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionShape { kind },
    })
}

#[cfg(test)]
mod tests {
    use super::{plan, plan_record, plan_record_constructor, plan_var};
    use crate::plan::{
        ConstantFunction, ConstantTemplate, ConstantTemplateId, ConstantTemplateSignature,
        ConstantTemplates, ConstantValue, CustomConstructor, CustomConstructorField, CustomExpr,
        CustomReturn, CustomType, CustomTypeName, Expr, FunctionReference, FunctionShape,
        FunctionTemplateId, FunctionTemplateSignature, IntLocalId, LocalId, ParamBinding,
        ParamLocal, ParamSlot, ReturnExpr, TypeScheme, ValueShape, ValueType,
        monomorphic_function_instantiation,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, FunctionParam, PlanContext};
    use crate::planner::dsl::{
        bool_, call_int_function, float, function, int, int_function_call_arg, int_function_ref,
        list, list_spread, local_int, module, nil, string, tuple,
    };
    use crate::planner::error::{
        InvalidCustomTypeReason, InvalidExpressionShapeKind, InvalidExpressionType,
        InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use ecow::EcoString;
    use gleam_core::analyse::Inferred;
    use gleam_core::ast::{
        BitArraySegment, Constant, Publicity, RecordBeingUpdated, Statement, TypedExpr,
    };
    use gleam_core::type_::error::VariableOrigin;
    use gleam_core::type_::{self, Deprecation, ValueConstructor, ValueConstructorVariant};
    use std::collections::HashMap;

    #[test]
    fn plan_constant_value_families() {
        let actual = plan_module(compile(
            r#"
const number = 1
const ratio = 1.5
const label = "ge" <> "am"
const pair = #(1, "one")
const truth = True
const falsehood = False
const nothing = Nil

pub fn main() {
  #(number, ratio, label, pair, truth, falsehood, nothing)
}
"#,
        ))
        .expect("source should plan");
        let pair_elements = vec![ValueShape::Int, ValueShape::String].into_boxed_slice();
        let left = ConstantValue::string("ge".into())
            .into_string()
            .expect("a String value has String storage");
        let right = ConstantValue::string("am".into())
            .into_string()
            .expect("a String value has String storage");
        let entries = vec![
            (
                ConstantTemplate::new(
                    ConstantTemplateSignature::nil(ConstantTemplateId(0), 0, TypeScheme::new(0)),
                    "nothing".into(),
                ),
                ConstantValue::nil(),
            ),
            (
                ConstantTemplate::new(
                    ConstantTemplateSignature::bool(ConstantTemplateId(1), 0, TypeScheme::new(0)),
                    "falsehood".into(),
                ),
                ConstantValue::bool(false),
            ),
            (
                ConstantTemplate::new(
                    ConstantTemplateSignature::bool(ConstantTemplateId(2), 1, TypeScheme::new(0)),
                    "truth".into(),
                ),
                ConstantValue::bool(true),
            ),
            (
                ConstantTemplate::new(
                    ConstantTemplateSignature::tuple(
                        ConstantTemplateId(3),
                        0,
                        TypeScheme::new(0),
                        pair_elements.clone(),
                    ),
                    "pair".into(),
                ),
                ConstantValue::tuple(
                    vec![ValueShape::Int, ValueShape::String].into_boxed_slice(),
                    vec![
                        ConstantValue::int(1.into()),
                        ConstantValue::string("one".into()),
                    ]
                    .into_boxed_slice(),
                ),
            ),
            (
                ConstantTemplate::new(
                    ConstantTemplateSignature::string(ConstantTemplateId(4), 0, TypeScheme::new(0)),
                    "label".into(),
                ),
                ConstantValue::string_concatenation(left, right),
            ),
            (
                ConstantTemplate::new(
                    ConstantTemplateSignature::float(ConstantTemplateId(5), 0, TypeScheme::new(0)),
                    "ratio".into(),
                ),
                ConstantValue::float(1.5),
            ),
            (
                ConstantTemplate::new(
                    ConstantTemplateSignature::int(ConstantTemplateId(6), 0, TypeScheme::new(0)),
                    "number".into(),
                ),
                ConstantValue::int(1.into()),
            ),
        ];
        let expected = module(
            "main",
            function(
                "main",
                tuple([
                    Expr::from(int(1)),
                    Expr::from(float(1.5)),
                    Expr::from(string("ge").concatenate(string("am"))),
                    Expr::from(tuple([Expr::from(int(1)), Expr::from(string("one"))])),
                    Expr::from(bool_(true)),
                    Expr::from(bool_(false)),
                    Expr::from(nil()),
                ]),
            ),
            [],
        )
        .with_constants(ConstantTemplates::from_entries(entries));

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_constant_compound_alias_and_list_spread() {
        let actual = plan_module(compile(
            r#"
const rest = [2, 3]
const values = [1, ..rest]

pub fn main() {
  values
}
"#,
        ))
        .expect("source should plan");
        let rest_signature = ConstantTemplateSignature::list(
            ConstantTemplateId(0),
            0,
            TypeScheme::new(0),
            ValueShape::Int,
        );
        let rest_instantiation = rest_signature
            .try_instantiate(Vec::new())
            .expect("a monomorphic constant signature should instantiate");
        let values_signature = ConstantTemplateSignature::list(
            ConstantTemplateId(1),
            1,
            TypeScheme::new(0),
            ValueShape::Int,
        );
        let rest_value = ConstantValue::try_list(
            ValueShape::Int,
            vec![ConstantValue::int(2.into()), ConstantValue::int(3.into())],
            None,
        )
        .expect("rest has matching Int list elements");
        let values_value = ConstantValue::try_list(
            ValueShape::Int,
            vec![ConstantValue::int(1.into())],
            Some(ConstantValue::reference(rest_instantiation)),
        )
        .expect("values has a matching Int list tail");
        let expected = module(
            "main",
            function(
                "main",
                list_spread(
                    [int(1)],
                    list([int(2), int(3)], ValueType::Int),
                    ValueType::Int,
                ),
            ),
            [],
        )
        .with_constants(ConstantTemplates::from_entries(vec![
            (
                ConstantTemplate::new(rest_signature, "rest".into()),
                rest_value,
            ),
            (
                ConstantTemplate::new(values_signature, "values".into()),
                values_value,
            ),
        ]));

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_constant_function_value_call() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

const f = add_one

pub fn main() {
  f(41)
}
"#,
        ))
        .expect("source should plan");
        let function_shape = FunctionShape::new(vec![ValueShape::Int], ValueShape::Int);
        let signature = ConstantTemplateSignature::function(
            ConstantTemplateId(0),
            0,
            TypeScheme::new(0),
            function_shape.clone(),
        );
        let value = ConstantValue::function(
            function_shape.clone(),
            ConstantFunction::Reference(FunctionReference::from_slots(
                monomorphic_function_instantiation(1, function_shape),
                vec![ParamSlot::from_local(ParamLocal::int(IntLocalId(0)))],
            )),
        );
        let expected = module(
            "main",
            function(
                "main",
                call_int_function(
                    int_function_ref(1, [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(0, int(41))],
                ),
            ),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
        )
        .with_constants(ConstantTemplates::from_entries(vec![(
            ConstantTemplate::new(signature, "f".into()),
            value,
        )]));

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_constant_segment_values_preserve_compound_and_reference_shapes() {
        assert_eq!(
            plan_constant_literal(Constant::StringConcatenation {
                location: dummy_span(),
                left: Box::new(Constant::String {
                    location: dummy_span(),
                    value: "ge".into(),
                }),
                right: Box::new(Constant::String {
                    location: dummy_span(),
                    value: "am".into(),
                }),
            }),
            Ok(Expr::from(string("ge").concatenate(string("am")))),
        );
        assert_eq!(
            plan_constant_literal(Constant::Tuple {
                location: dummy_span(),
                elements: vec![Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }],
                type_: type_::tuple(vec![type_::int()]),
            }),
            Ok(Expr::from(tuple([Expr::from(int(1))]))),
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: vec![Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }],
                type_: type_::list(type_::int()),
                tail: Some(Box::new(Constant::List {
                    location: dummy_span(),
                    elements: vec![Constant::Int {
                        location: dummy_span(),
                        value: "2".into(),
                        int_value: 2.into(),
                    }],
                    type_: type_::list(type_::int()),
                    tail: None,
                })),
            }),
            Ok(Expr::from(list_spread(
                [int(1)],
                list([int(2)], ValueType::Int),
                ValueType::Int,
            ))),
        );

        let mut constant_module = compile(
            r#"
const answer = 1
pub fn main() { answer }
"#,
        );
        let constant = main_var_constructor_mut(&mut constant_module).clone();
        assert_eq!(
            plan_constant_literal(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "answer".into(),
                constructor: Some(Box::new(constant)),
                type_: type_::int(),
            }),
            Ok(Expr::from(int(1))),
        );

        let function_shape = FunctionShape::new(vec![ValueShape::Int], ValueShape::Int);
        let mut function_module = function_constant_module();
        let constructor = constant_definition_alias_constructor_mut(&mut function_module).clone();
        let module_name = "main".into();
        let functions = HashMap::from([(
            "add_one".into(),
            FunctionInfo {
                signature: FunctionTemplateSignature::new(
                    FunctionTemplateId::new(0),
                    TypeScheme::new(0),
                    function_shape.clone(),
                ),
                type_parameters: crate::planner::type_parameter::TypeParameterScope::default(),
                return_shape: ValueShape::Int,
                params: vec![FunctionParam::new(
                    ParamLocal::int(IntLocalId(0)),
                    ValueShape::Int,
                    ParamBinding::Named("value".into()),
                    None,
                )],
            },
        )]);
        let mut anonymous_functions = AnonymousFunctions::default();
        let context = PlanContext::new(&module_name, &functions, &mut anonymous_functions);
        assert_eq!(
            plan_var(Some(constructor), &context),
            Ok(Expr::function(crate::plan::FunctionExpr::reference(
                FunctionReference::from_slots(
                    monomorphic_function_instantiation(0, function_shape),
                    vec![ParamSlot::from_local(ParamLocal::int(IntLocalId(0)))],
                ),
            ))),
        );
    }

    #[test]
    fn plan_constant_segment_record_preserves_concrete_result_payload() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous_functions = AnonymousFunctions::default();
        let context = PlanContext::new(&module_name, &functions, &mut anonymous_functions);
        let result_type = CustomType::new(
            CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
            vec![ValueType::Int, ValueType::String],
        );
        let mut constructor = record_constructor("Ok", "gleam", 1);
        constructor.type_ = type_::fn_(
            vec![type_::int()],
            type_::result(type_::int(), type_::string()),
        );

        assert_eq!(
            plan_record(
                Some(vec![gleam_core::ast::CallArg {
                    label: None,
                    location: dummy_span(),
                    value: Constant::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: 1.into(),
                    },
                    implicit: None,
                }]),
                Some(constructor),
                &context,
            ),
            Ok(Expr::custom(
                CustomExpr::try_constructor(
                    CustomConstructor::new(
                        result_type,
                        "Ok".into(),
                        0,
                        vec![CustomConstructorField::new(None, ValueType::Int)],
                    ),
                    vec![Expr::from(int(1))],
                )
                .expect("Result construction should match its descriptor"),
            )),
        );
    }

    #[test]
    fn reject_margin_constant_function_and_constructor_shapes() {
        let module_name = EcoString::from("main");
        let functions = HashMap::new();
        let mut anonymous_functions = AnonymousFunctions::default();
        let context = PlanContext::new(&module_name, &functions, &mut anonymous_functions);
        let mut function_module = function_constant_module();
        let constructor = constant_definition_alias_constructor_mut(&mut function_module).clone();

        assert_eq!(
            plan_var(Some(constructor.clone()), &context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal {
                    name: "add_one".into(),
                },
            }),
        );

        let mut external_module = function_constant_module();
        *module_fn_constant_alias_module_mut(&mut external_module) = "other".into();
        let external = constant_definition_alias_constructor_mut(&mut external_module).clone();
        assert_eq!(
            plan_var(Some(external), &context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::ModuleSelect,
                },
            }),
        );

        let mut generic_constructor = record_constructor("Boxed", "main", 0);
        generic_constructor.type_ = type_::generic_var(0);
        assert_eq!(
            plan_record_constructor(generic_constructor, &context),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::GenericFunction,
            }),
        );
    }

    #[test]
    fn plan_concrete_result_constant_value() {
        let plan = plan_module(compile(
            r#"
const result: Result(Int, Nil) = Ok(1)

pub fn main() {
  result
}
"#,
        ))
        .expect("concrete Result constant should plan");
        assert_eq!(
            plan.main_function().return_().value_type(),
            ValueType::Custom(crate::plan::CustomType::new(
                crate::plan::CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
                vec![ValueType::Int, ValueType::Nil],
            )),
        );
    }

    #[test]
    fn plan_zero_field_custom_constructor_constant() {
        let plan = plan_module(compile(
            r#"
pub type Token { Empty }
const empty = Empty
pub fn main() { empty }
"#,
        ))
        .expect("zero-field custom constructor constant should plan");
        let type_ = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Token".into()),
            Vec::new(),
        );
        let constructor = CustomConstructor::new(type_.clone(), "Empty".into(), 0, Vec::new());

        assert_eq!(
            plan.main_function().return_(),
            &ReturnExpr::custom_body(CustomReturn::expr(
                CustomExpr::try_constructor(constructor, Vec::new())
                    .expect("test custom construction should be valid"),
            ),),
        );
    }

    #[test]
    fn plan_frontend_normalized_record_update_constant() {
        let plan = plan_module(compile(
            r#"
pub type Person {
  Person(name: String, age: Int)
}

const lucy = Person(name: "Lucy", age: 30)
const older_lucy = Person(..lucy, age: 31)

pub fn main() { older_lucy }
"#,
        ))
        .expect("frontend-normalized constructor constant should plan");
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

        assert_eq!(
            plan.main_function().return_(),
            &ReturnExpr::custom_body(CustomReturn::expr(
                CustomExpr::try_constructor(
                    constructor,
                    vec![Expr::from(string("Lucy")), Expr::from(int(31))],
                )
                .expect("test custom construction should be valid"),
            ),),
        );
    }

    #[test]
    fn reject_profile_polymorphic_custom_constructor_constant() {
        assert_eq!(
            plan_module(compile(
                r#"
pub type Boxed(value) {
  Boxed(value)
}

const make = Boxed

pub fn main() {
  make
  1
}
"#,
            )),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::GenericFunction,
            }),
        );
    }

    #[test]
    fn reject_margin_invalid_constant_shapes() {
        let invalid = || Constant::Invalid {
            location: dummy_span(),
            type_: type_::int(),
            extra_information: None,
        };
        let invalid_error = Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        });

        assert_eq!(
            plan_constant_literal(Constant::StringConcatenation {
                location: dummy_span(),
                left: Box::new(invalid()),
                right: Box::new(Constant::String {
                    location: dummy_span(),
                    value: "right".into(),
                }),
            }),
            invalid_error,
        );
        assert_eq!(
            plan_constant_literal(Constant::Tuple {
                location: dummy_span(),
                elements: vec![invalid()],
                type_: type_::tuple(vec![type_::int()]),
            }),
            invalid_error,
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: vec![invalid()],
                type_: type_::list(type_::int()),
                tail: None,
            }),
            invalid_error,
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: Vec::new(),
                type_: type_::list(type_::int()),
                tail: Some(Box::new(invalid())),
            }),
            invalid_error,
        );
        assert_eq!(
            plan_constant_literal(Constant::BitArray {
                location: dummy_span(),
                segments: vec![BitArraySegment {
                    location: dummy_span(),
                    value: Box::new(invalid()),
                    options: Vec::new(),
                    type_: type_::int(),
                }],
            }),
            invalid_error,
        );

        assert_eq!(
            plan_constant_literal(Constant::Tuple {
                location: dummy_span(),
                elements: Vec::new(),
                type_: type_::int(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Tuple,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::Tuple {
                location: dummy_span(),
                elements: Vec::new(),
                type_: type_::tuple(vec![type_::generic_var(0)]),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Tuple,
                    actual: InvalidExpressionType::Unsupported,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::Tuple {
                location: dummy_span(),
                elements: vec![Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }],
                type_: type_::tuple(vec![type_::string()]),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Tuple,
                    actual: InvalidExpressionType::Tuple,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: Vec::new(),
                type_: type_::int(),
                tail: None,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: Vec::new(),
                type_: type_::generic_var(0),
                tail: None,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Unsupported,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: Vec::new(),
                type_: type_::list(type_::generic_var(0)),
                tail: None,
            }),
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::UnsupportedListElementType,
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: vec![Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }],
                type_: type_::list(type_::float()),
                tail: None,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Float,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: vec![Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }],
                type_: type_::list(type_::bool()),
                tail: None,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Bool,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: vec![Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }],
                type_: type_::list(type_::nil()),
                tail: None,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Nil,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: vec![Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }],
                type_: type_::list(type_::fn_(vec![type_::int()], type_::int())),
                tail: None,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Function,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: vec![Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }],
                type_: type_::list(type_::string()),
                tail: Some(Box::new(Constant::List {
                    location: dummy_span(),
                    elements: vec![Constant::String {
                        location: dummy_span(),
                        value: "tail".into(),
                    }],
                    type_: type_::list(type_::string()),
                    tail: None,
                })),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::Invalid {
                location: dummy_span(),
                type_: type_::int(),
                extra_information: None,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::StringConcatenation {
                location: dummy_span(),
                left: Box::new(Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }),
                right: Box::new(Constant::String {
                    location: dummy_span(),
                    value: "right".into(),
                }),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::StringConcatenation {
                location: dummy_span(),
                left: Box::new(Constant::String {
                    location: dummy_span(),
                    value: "left".into(),
                }),
                right: Box::new(Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: Vec::new(),
                type_: type_::list(type_::int()),
                tail: Some(Box::new(Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                })),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::List {
                location: dummy_span(),
                elements: Vec::new(),
                type_: type_::list(type_::int()),
                tail: Some(Box::new(Constant::List {
                    location: dummy_span(),
                    elements: vec![Constant::String {
                        location: dummy_span(),
                        value: "one".into(),
                    }],
                    type_: type_::list(type_::string()),
                    tail: None,
                })),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::List,
                    actual: InvalidExpressionType::List,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "missing".into(),
                constructor: None,
                type_: type_::int(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "local".into(),
                constructor: Some(Box::new(ValueConstructor::local_variable(
                    dummy_span(),
                    VariableOrigin::generated(),
                    type_::int(),
                ))),
                type_: type_::int(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "Other".into(),
                constructor: Some(Box::new(record_constructor("Other", "gleam", 0))),
                type_: type_::bool(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::PreludeConstructor,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "Boxed".into(),
                constructor: Some(Box::new(record_constructor("Boxed", "main", 0))),
                type_: type_::int(),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Boxed".into(),
                    reason: InvalidCustomTypeReason::ConstructorType,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::Record {
                location: dummy_span(),
                module: None,
                name: "Boxed".into(),
                arguments: None,
                type_: type_::int(),
                field_map: Inferred::Unknown,
                record_constructor: None,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::Record {
                location: dummy_span(),
                module: None,
                name: "Boxed".into(),
                arguments: Some(vec![gleam_core::ast::CallArg {
                    label: None,
                    location: dummy_span(),
                    value: Constant::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: 1.into(),
                    },
                    implicit: None,
                }]),
                type_: type_::int(),
                field_map: Inferred::Unknown,
                record_constructor: Some(Box::new(record_constructor("Boxed", "main", 1))),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Boxed".into(),
                    reason: InvalidCustomTypeReason::ConstructorType,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::Record {
                location: dummy_span(),
                module: None,
                name: "True".into(),
                arguments: Some(vec![gleam_core::ast::CallArg {
                    label: None,
                    location: dummy_span(),
                    value: Constant::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: 1.into(),
                    },
                    implicit: None,
                }]),
                type_: type_::bool(),
                field_map: Inferred::Unknown,
                record_constructor: Some(Box::new(record_constructor("True", "gleam", 0))),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "True".into(),
                    reason: InvalidCustomTypeReason::ConstructorType,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::Record {
                location: dummy_span(),
                module: None,
                name: "Broken".into(),
                arguments: None,
                type_: type_::int(),
                field_map: Inferred::Unknown,
                record_constructor: Some(Box::new(ValueConstructor::local_variable(
                    dummy_span(),
                    VariableOrigin::generated(),
                    type_::int(),
                ))),
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::Todo {
                location: dummy_span(),
                type_: type_::int(),
                message: None,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
        assert_eq!(
            plan_constant_literal(Constant::RecordUpdate {
                location: dummy_span(),
                constructor_location: dummy_span(),
                module: None,
                name: "Boxed".into(),
                record: RecordBeingUpdated {
                    base: Box::new(Constant::Int {
                        location: dummy_span(),
                        value: "1".into(),
                        int_value: 1.into(),
                    }),
                    location: dummy_span(),
                },
                arguments: Vec::new(),
                type_: type_::int(),
                field_map: Inferred::Unknown,
            }),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordUpdate,
                },
            }),
        );
    }

    #[test]
    fn constant_prelude_record_constructor_values_preserve_concrete_result_type() {
        let result_type = crate::plan::CustomType::new(
            crate::plan::CustomTypeName::new("".into(), "gleam".into(), "Result".into()),
            vec![ValueType::Int, ValueType::Nil],
        );
        let mut constructor = record_constructor("Ok", "gleam", 1);
        constructor.type_ = type_::fn_(
            vec![type_::int()],
            type_::result(type_::int(), type_::nil()),
        );
        assert_eq!(
            plan_constant_literal(Constant::Var {
                location: dummy_span(),
                module: None,
                name: "Ok".into(),
                constructor: Some(Box::new(constructor)),
                type_: type_::fn_(
                    vec![type_::int()],
                    type_::result(type_::int(), type_::nil())
                ),
            }),
            Ok(Expr::function(crate::plan::FunctionExpr::custom(
                crate::plan::CustomFunctionExpr::constructor(crate::plan::CustomConstructor::new(
                    result_type,
                    "Ok".into(),
                    0,
                    vec![crate::plan::CustomConstructorField::new(
                        None,
                        ValueType::Int
                    )],
                )),
            ))),
        );
    }

    #[test]
    fn constant_record_constructor_margins_preserve_exact_shape_failures() {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous_functions = AnonymousFunctions::default();
        let context = PlanContext::new(&module_name, &functions, &mut anonymous_functions);

        let mut true_constructor = record_constructor("True", "gleam", 0);
        true_constructor.type_ = type_::bool();
        assert_eq!(
            plan_record(Some(Vec::new()), Some(true_constructor), &context),
            Ok(Expr::bool(crate::plan::BoolExpr::value(true))),
        );
        let mut false_constructor = record_constructor("False", "gleam", 0);
        false_constructor.type_ = type_::bool();
        assert_eq!(
            plan_record(Some(Vec::new()), Some(false_constructor), &context),
            Ok(Expr::bool(crate::plan::BoolExpr::value(false))),
        );
        let mut nil_constructor = record_constructor("Nil", "gleam", 0);
        nil_constructor.type_ = type_::nil();
        assert_eq!(
            plan_record(Some(Vec::new()), Some(nil_constructor), &context),
            Ok(Expr::nil(crate::plan::NilExpr::value())),
        );

        let result_constructor = || {
            let mut constructor = record_constructor("Ok", "gleam", 1);
            constructor.type_ = type_::fn_(
                vec![type_::int()],
                type_::result(type_::int(), type_::string()),
            );
            constructor
        };
        let int_argument = |label| gleam_core::ast::CallArg {
            label,
            location: dummy_span(),
            value: Constant::Int {
                location: dummy_span(),
                value: "1".into(),
                int_value: 1.into(),
            },
            implicit: None,
        };
        assert_eq!(
            plan_record(Some(Vec::new()), Some(result_constructor()), &context,),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
        assert_eq!(
            plan_record(
                Some(vec![int_argument(None), int_argument(None)]),
                Some(result_constructor()),
                &context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
        let mut extra_argument_constructor = record_constructor("Ok", "gleam", 2);
        extra_argument_constructor.type_ = type_::fn_(
            vec![type_::int()],
            type_::result(type_::int(), type_::string()),
        );
        assert_eq!(
            plan_record(
                Some(vec![int_argument(None), int_argument(None)]),
                Some(extra_argument_constructor),
                &context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
        assert_eq!(
            plan_record(
                Some(vec![int_argument(Some("wrong".into()))]),
                Some(result_constructor()),
                &context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
        assert_eq!(
            plan_record(
                Some(vec![gleam_core::ast::CallArg {
                    label: None,
                    location: dummy_span(),
                    value: Constant::String {
                        location: dummy_span(),
                        value: "wrong".into(),
                    },
                    implicit: None,
                }]),
                Some(result_constructor()),
                &context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::Int,
                    actual: InvalidExpressionType::String,
                },
            }),
        );
        assert_eq!(
            plan_record(
                Some(vec![gleam_core::ast::CallArg {
                    label: None,
                    location: dummy_span(),
                    value: Constant::Invalid {
                        location: dummy_span(),
                        type_: type_::int(),
                        extra_information: None,
                    },
                    implicit: None,
                }]),
                Some(result_constructor()),
                &context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );

        let local = ValueConstructor::local_variable(
            dummy_span(),
            VariableOrigin::generated(),
            type_::int(),
        );
        assert_eq!(
            plan_record_constructor(local, &context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
        assert_eq!(
            plan_record_constructor(record_constructor("External", "other", 0), &context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
        assert_eq!(
            plan_record_constructor(record_constructor("Ok", "other", 0), &context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
        assert_eq!(
            plan_record_constructor(record_constructor("External", "gleam", 1), &context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
        let mut mismatched_result_constructor = record_constructor("Ok", "gleam", 2);
        mismatched_result_constructor.type_ = type_::fn_(
            vec![type_::int()],
            type_::result(type_::int(), type_::string()),
        );
        assert_eq!(
            plan_record_constructor(mismatched_result_constructor, &context),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_constant_record_descriptor_argument_count() {
        let mut module = compile(
            r#"
pub type Pair { Pair(Int, Int) }
const pair = Pair(1, 2)
pub fn main() { pair }
"#,
        );
        let custom_type = module.definitions.constants[0].type_.clone();
        let mut constructor = record_constructor("Pair", "main", 1);
        constructor.type_ = type_::fn_(vec![type_::int(), type_::int()], custom_type.clone());
        *module.definitions.constants[0].value = Constant::Record {
            location: dummy_span(),
            module: None,
            name: "Pair".into(),
            arguments: Some(vec![gleam_core::ast::CallArg {
                label: None,
                location: dummy_span(),
                value: Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                },
                implicit: None,
            }]),
            type_: custom_type,
            field_map: Inferred::Unknown,
            record_constructor: Some(Box::new(constructor)),
        };

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_constant_module_constructor_shapes() {
        let mut missing_function = function_constant_module();
        missing_function.definitions.functions.retain(|function| {
            function
                .name
                .as_ref()
                .is_some_and(|(_, name)| name == "main")
        });
        assert_eq!(
            plan_module(missing_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal {
                    name: "add_one".into(),
                },
            }),
        );

        let mut non_current_function = function_constant_module();
        let module = module_fn_constant_alias_module_mut(&mut non_current_function);
        *module = "other".into();
        assert_eq!(
            plan_module(non_current_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::ModuleSelect,
                },
            }),
        );

        let mut non_current_constant = compile(
            r#"
const answer = 1

pub fn main() {
  answer
}
"#,
        );
        let module = module_constant_module_mut(&mut non_current_constant);
        *module = "other".into();
        assert_eq!(
            plan_module(non_current_constant),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::ModuleSelect,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_module_constant_literal_shapes() {
        assert_eq!(
            plan_module(module_with_constant_value(Constant::StringConcatenation {
                location: dummy_span(),
                left: Box::new(Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }),
                right: Box::new(Constant::String {
                    location: dummy_span(),
                    value: "right".into(),
                }),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_module(module_with_constant_value(Constant::StringConcatenation {
                location: dummy_span(),
                left: Box::new(Constant::String {
                    location: dummy_span(),
                    value: "left".into(),
                }),
                right: Box::new(Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: 1.into(),
                }),
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_module(module_with_constant_value(Constant::Invalid {
                location: dummy_span(),
                type_: type_::int(),
                extra_information: None,
            })),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    #[test]
    fn module_fn_constant_alias_module_mut_panics_on_constant_alias() {
        let result = std::panic::catch_unwind(|| {
            let mut module = compile(
                r#"
const answer = 1
const f = answer

pub fn main() {
  f
}
"#,
            );

            module_fn_constant_alias_module_mut(&mut module);
        });

        assert!(result.is_err());
    }

    #[test]
    #[should_panic(expected = "expected constant definition alias")]
    fn constant_definition_alias_fixture_guard_rejects_value_literal() {
        let mut module = compile(
            r#"
const f = 1

pub fn main() {
  f
}
"#,
        );

        constant_definition_alias_constructor_mut(&mut module);
    }

    #[test]
    fn module_constant_module_mut_panics_on_function_reference() {
        let result = std::panic::catch_unwind(|| {
            let mut module = compile(
                r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  add_one
}
"#,
            );

            module_constant_module_mut(&mut module);
        });

        assert!(result.is_err());
    }

    #[test]
    fn main_var_constructor_mut_panics_on_multiple_statements() {
        let result = std::panic::catch_unwind(|| {
            let mut module = compile(
                r#"
pub fn main() {
  let value = 1
  value
}
"#,
            );

            main_var_constructor_mut(&mut module);
        });

        assert!(result.is_err());
    }

    #[test]
    fn main_var_constructor_mut_panics_on_non_variable_expression() {
        let result = std::panic::catch_unwind(|| {
            let mut module = compile(
                r#"
pub fn main() {
  1
}
"#,
            );

            main_var_constructor_mut(&mut module);
        });

        assert!(result.is_err());
    }

    fn plan_constant_literal(
        literal: Constant<std::sync::Arc<type_::Type>>,
    ) -> Result<Expr, PlanError> {
        let module_name = "main".into();
        let functions = HashMap::new();
        let mut anonymous_functions = AnonymousFunctions::default();
        let context = PlanContext::new(&module_name, &functions, &mut anonymous_functions);

        plan(literal, &context)
    }

    fn record_constructor(name: &str, module: &str, arity: u16) -> ValueConstructor {
        ValueConstructor {
            publicity: Publicity::Private,
            deprecation: Deprecation::NotDeprecated,
            variant: ValueConstructorVariant::Record {
                name: name.into(),
                arity,
                field_map: None,
                location: dummy_span(),
                module: module.into(),
                variants_count: 1,
                variant_index: 0,
                documentation: None,
            },
            type_: type_::int(),
        }
    }

    fn function_constant_module() -> gleam_core::ast::TypedModule {
        compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

const f = add_one

pub fn main() {
  f
}
"#,
        )
    }

    fn module_with_constant_value(
        literal: Constant<std::sync::Arc<type_::Type>>,
    ) -> gleam_core::ast::TypedModule {
        let mut module = compile(
            r#"
const value = 1

pub fn main() {
  value
}
"#,
        );
        *module.definitions.constants[0].value = literal;

        module
    }

    fn module_fn_constant_alias_module_mut(
        module: &mut gleam_core::ast::TypedModule,
    ) -> &mut ecow::EcoString {
        let ValueConstructorVariant::ModuleFn { module, .. } =
            &mut constant_definition_alias_constructor_mut(module).variant
        else {
            panic!("expected module function constant alias");
        };

        module
    }

    fn module_constant_module_mut(
        module: &mut gleam_core::ast::TypedModule,
    ) -> &mut ecow::EcoString {
        let ValueConstructorVariant::ModuleConstant { module, .. } =
            &mut main_var_constructor_mut(module).variant
        else {
            panic!("expected module constant constructor");
        };

        module
    }

    fn constant_definition_alias_constructor_mut(
        module: &mut gleam_core::ast::TypedModule,
    ) -> &mut ValueConstructor {
        let constant = module
            .definitions
            .constants
            .iter_mut()
            .find(|constant| constant.name == "f")
            .expect("f constant should exist");
        let Constant::Var {
            constructor: Some(constructor),
            ..
        } = constant.value.as_mut()
        else {
            panic!("expected constant definition alias");
        };

        constructor
    }

    fn main_var_constructor_mut(
        module: &mut gleam_core::ast::TypedModule,
    ) -> &mut ValueConstructor {
        let function = module
            .definitions
            .functions
            .iter_mut()
            .find(|function| {
                function
                    .name
                    .as_ref()
                    .is_some_and(|(_, name)| name == "main")
            })
            .expect("main function should exist");
        let [Statement::Expression(expression)] = function.body.as_mut_slice() else {
            panic!("expected single expression statement");
        };
        let TypedExpr::Var { constructor, .. } = expression else {
            panic!("expected variable expression");
        };

        constructor
    }
}
