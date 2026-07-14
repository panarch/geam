use super::{invalid_expression_type, invalid_expression_type_for_value};
use crate::plan::{
    BoolExpr, CustomExpr, CustomFunctionExpr, Expr, FloatExpr, FunctionExpr, IntExpr, ListExpr,
    NilExpr, StringExpr, TupleExpr, ValueType,
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
        Constant::RecordUpdate { .. } => Err(PlanError::UnsupportedExpression {
            kind: UnsupportedExpressionKind::RecordUpdate,
        }),
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
        None => Err(invalid_expression_type_for_value(ValueType::String, actual)),
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
            return Err(invalid_expression_type_for_value(
                ValueType::Tuple(actual_type),
                actual,
            ));
        }
        None => {
            return Err(invalid_expression_type(
                InvalidExpressionType::Tuple,
                InvalidExpressionType::Unsupported,
            ));
        }
    };

    if expected_type != actual_type {
        return Err(invalid_expression_type_for_value(
            ValueType::Tuple(expected_type),
            ValueType::Tuple(actual_type),
        ));
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
            Some(actual) => Err(invalid_expression_type_for_value(
                ValueType::List(Box::new(ValueType::Nil)),
                actual,
            )),
            None => Err(invalid_expression_type(
                InvalidExpressionType::List,
                InvalidExpressionType::Unsupported,
            )),
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
                return Err(invalid_expression_type_for_value(
                    error.expected,
                    error.actual,
                ));
            }
        };
        return Ok(Expr::list(list));
    };
    let tail = plan(tail, context)?;
    let actual = tail.value_type();
    let Some(tail) = tail.into_list() else {
        return Err(invalid_expression_type_for_value(
            ValueType::List(Box::new(expected_element_type.clone())),
            actual,
        ));
    };
    let elements = match crate::plan::ListElements::from_exprs(
        expected_element_type.clone(),
        planned_elements,
    ) {
        Ok(elements) => elements,
        Err(error) => {
            return Err(invalid_expression_type_for_value(
                error.expected,
                error.actual,
            ));
        }
    };
    let elements = match crate::plan::ListSpreadElements::from_parts(elements, tail) {
        Ok(elements) => elements,
        Err(error) => {
            return Err(invalid_expression_type_for_value(
                ValueType::List(Box::new(error.expected)),
                ValueType::List(Box::new(error.actual)),
            ));
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
                function.reference(),
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

    if !matches!(constructor.variant, ValueConstructorVariant::Record { .. }) {
        return invalid_expression_shape(InvalidExpressionShapeKind::Invalid);
    }
    let Some(arguments) = arguments else {
        return plan_record_constructor(constructor, context);
    };
    if arguments.is_empty() {
        return plan_record_constructor(constructor, context);
    }
    let constructor = context.custom_constructor(&constructor)?;
    if arguments.len() != constructor.fields().len() {
        return invalid_expression_shape(InvalidExpressionShapeKind::RecordConstructor);
    }
    let arguments = arguments
        .into_iter()
        .zip(constructor.fields())
        .map(|(argument, field)| {
            if let Some(label) = &argument.label
                && field.label() != Some(label)
            {
                return invalid_expression_shape(InvalidExpressionShapeKind::RecordConstructor);
            }
            let argument = plan(argument.value, context)?;
            if argument.value_type() != *field.type_() {
                return Err(invalid_expression_type_for_value(
                    field.type_().clone(),
                    argument.value_type(),
                ));
            }
            Ok(argument)
        })
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Expr::custom(CustomExpr::constructor(
        constructor,
        arguments,
    )))
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
    let constructor = context.custom_constructor(&constructor)?;
    if constructor.fields().is_empty() {
        Ok(Expr::custom(CustomExpr::constructor(
            constructor,
            Vec::new(),
        )))
    } else {
        Ok(Expr::function(FunctionExpr::custom(
            CustomFunctionExpr::constructor(constructor),
        )))
    }
}

fn invalid_expression_shape(kind: InvalidExpressionShapeKind) -> Result<Expr, PlanError> {
    Err(PlanError::InvalidTypedAst {
        reason: InvalidTypedAstReason::ExpressionShape { kind },
    })
}

#[cfg(test)]
mod tests {
    use super::{plan, plan_record, plan_record_constructor};
    use crate::plan::{
        CustomConstructor, CustomConstructorField, CustomExpr, CustomFunctionId, CustomReturn,
        CustomType, CustomTypeName, Expr, IntLocalId, LocalId, ReturnExpr, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, PlanContext};
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
        );

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
        );

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
        );

        assert_eq!(actual, expected);
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
            &ReturnExpr::custom_body(
                CustomFunctionId(0),
                type_,
                CustomReturn::expr(CustomExpr::constructor(constructor, Vec::new())),
            ),
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
            &ReturnExpr::custom_body(
                CustomFunctionId(0),
                type_,
                CustomReturn::expr(CustomExpr::constructor(
                    constructor,
                    vec![Expr::from(string("Lucy")), Expr::from(int(31))],
                )),
            ),
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
            Err(PlanError::UnsupportedExpression {
                kind: UnsupportedExpressionKind::RecordUpdate,
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
            plan_module(module_with_main_constant_literal(
                Constant::StringConcatenation {
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
                }
            )),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_module(module_with_main_constant_literal(
                Constant::StringConcatenation {
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
                }
            )),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionType {
                    expected: InvalidExpressionType::String,
                    actual: InvalidExpressionType::Int,
                },
            }),
        );
        assert_eq!(
            plan_module(module_with_main_constant_literal(Constant::Invalid {
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
    fn main_module_constant_literal_mut_panics_on_function_reference() {
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

            main_module_constant_literal_mut(&mut module);
        });

        assert!(result.is_err());
    }

    #[test]
    fn constant_alias_constructor_mut_panics_on_function_reference() {
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

            constant_alias_constructor_mut(&mut module);
        });

        assert!(result.is_err());
    }

    #[test]
    fn constant_alias_constructor_mut_panics_on_value_literal() {
        let result = std::panic::catch_unwind(|| {
            let mut module = compile(
                r#"
const answer = 1

pub fn main() {
  answer
}
"#,
            );

            constant_alias_constructor_mut(&mut module);
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

    fn module_with_main_constant_literal(
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
        *main_module_constant_literal_mut(&mut module) = literal;

        module
    }

    fn module_fn_constant_alias_module_mut(
        module: &mut gleam_core::ast::TypedModule,
    ) -> &mut ecow::EcoString {
        let ValueConstructorVariant::ModuleFn { module, .. } =
            &mut constant_alias_constructor_mut(module).variant
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

    fn main_module_constant_literal_mut(
        module: &mut gleam_core::ast::TypedModule,
    ) -> &mut Constant<std::sync::Arc<type_::Type>> {
        let ValueConstructorVariant::ModuleConstant { literal, .. } =
            &mut main_var_constructor_mut(module).variant
        else {
            panic!("expected module constant constructor");
        };

        literal
    }

    fn constant_alias_constructor_mut(
        module: &mut gleam_core::ast::TypedModule,
    ) -> &mut ValueConstructor {
        let ValueConstructorVariant::ModuleConstant { literal, .. } =
            &mut main_var_constructor_mut(module).variant
        else {
            panic!("expected module constant constructor");
        };
        let Constant::Var {
            constructor: Some(constructor),
            ..
        } = literal
        else {
            panic!("expected constant alias");
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
