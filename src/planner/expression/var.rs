use crate::plan::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CustomExpr, CustomFunctionExpr,
    Expr, FloatExpr, FloatFunctionExpr, FunctionExpr, FunctionFunctionExpr, IntExpr,
    IntFunctionExpr, ListExpr, ListFunctionExpr, LocalId, NilExpr, NilFunctionExpr, StringExpr,
    StringFunctionExpr, TupleExpr, TupleFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
    ValueType,
};
use crate::planner::context::{FunctionLocalBinding, PlanContext};
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError, UnsupportedExpressionKind,
};
use ecow::EcoString;
use gleam_core::type_::{PRELUDE_MODULE_NAME, ValueConstructor, ValueConstructorVariant};

pub(super) fn plan_var(
    name: EcoString,
    constructor: ValueConstructor,
    context: &PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match constructor.variant {
        ValueConstructorVariant::LocalVariable { .. } => {
            if let Some((local, type_)) = context.lookup_local(&name) {
                return local_get(local, name, type_);
            }
            if let Some((local, shape)) = context.lookup_tuple_local(&name) {
                let type_ = shape
                    .iter()
                    .map(crate::plan::ValueShape::value_type)
                    .collect();
                return Ok(Expr::tuple(
                    TupleExpr::local_get(local, name, type_).with_shape(shape),
                ));
            }
            if let Some(local) = context.lookup_custom_local(&name) {
                return Ok(Expr::custom(CustomExpr::local_get(local, name)));
            }
            if let Some((local, item_shape)) = context.lookup_list_local(&name) {
                return Ok(Expr::list(
                    ListExpr::local_get(local, name).with_item_shape(item_shape),
                ));
            }
            if let Some((binding, shape)) = context.lookup_function_local(&name) {
                return function_local_get(binding, name, shape);
            }

            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal { name },
            })
        }
        ValueConstructorVariant::Record {
            ref name,
            ref module,
            arity,
            ..
        } => {
            if module == PRELUDE_MODULE_NAME && arity == 0 {
                match name.as_str() {
                    "True" => return Ok(Expr::bool(BoolExpr::value(true))),
                    "False" => return Ok(Expr::bool(BoolExpr::value(false))),
                    "Nil" => return Ok(Expr::nil(NilExpr::value())),
                    _ => {}
                }
            }

            if module == context.module_name
                || (module == PRELUDE_MODULE_NAME && matches!(name.as_str(), "Ok" | "Error"))
            {
                let shape = crate::plan::ValueShape::from_gleam(constructor.type_.as_ref()).ok_or(
                    PlanError::UnsupportedExpression {
                        kind: UnsupportedExpressionKind::GenericFunction,
                    },
                )?;
                let constructor = context.custom_constructor(&constructor)?;
                if usize::from(arity) != constructor.fields().len() {
                    return Err(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::ExpressionShape {
                            kind: InvalidExpressionShapeKind::RecordConstructor,
                        },
                    });
                }
                return crate::plan::module::custom_constructor_expr(constructor)
                    .with_shape(shape)
                    .ok_or(PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::ExpressionShape {
                            kind: InvalidExpressionShapeKind::RecordConstructor,
                        },
                    });
            }

            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: if module == PRELUDE_MODULE_NAME {
                        InvalidExpressionShapeKind::PreludeConstructor
                    } else {
                        InvalidExpressionShapeKind::ModuleSelect
                    },
                },
            })
        }
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
            let function =
                context
                    .lookup_function(&name)
                    .ok_or_else(|| PlanError::InvalidTypedAst {
                        reason: InvalidTypedAstReason::UnknownLocal { name: name.clone() },
                    })?;

            let shape = function.shape();
            let reference = function.reference();

            FunctionExpr::reference(reference)
                .with_resolved_shape(shape)
                .map(Expr::function)
                .ok_or(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::Invalid,
                    },
                })
        }
        ValueConstructorVariant::ModuleFn { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::ModuleSelect,
            },
        }),
        ValueConstructorVariant::ModuleConstant {
            module, literal, ..
        } if module == *context.module_name => super::constant::plan(literal, context),
        ValueConstructorVariant::ModuleConstant { .. } => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::ModuleSelect,
            },
        }),
    }
}

fn function_local_get(
    binding: FunctionLocalBinding,
    name: EcoString,
    shape: crate::plan::FunctionShape,
) -> Result<Expr, PlanError> {
    let expression = match binding {
        FunctionLocalBinding::Int { local, type_ } => {
            FunctionExpr::int(IntFunctionExpr::local_get(local, name, type_))
        }
        FunctionLocalBinding::String { local, type_ } => {
            FunctionExpr::string(StringFunctionExpr::local_get(local, name, type_))
        }
        FunctionLocalBinding::BitArray { local, type_ } => {
            FunctionExpr::bit_array(BitArrayFunctionExpr::local_get(local, name, type_))
        }
        FunctionLocalBinding::UtfCodepoint { local, type_ } => {
            FunctionExpr::utf_codepoint(UtfCodepointFunctionExpr::local_get(local, name, type_))
        }
        FunctionLocalBinding::Custom(local) => {
            FunctionExpr::custom(CustomFunctionExpr::local_get(local, name))
        }
        FunctionLocalBinding::Float { local, type_ } => {
            FunctionExpr::float(FloatFunctionExpr::local_get(local, name, type_))
        }
        FunctionLocalBinding::Bool { local, type_ } => {
            FunctionExpr::bool(BoolFunctionExpr::local_get(local, name, type_))
        }
        FunctionLocalBinding::Nil { local, type_ } => {
            FunctionExpr::nil(NilFunctionExpr::local_get(local, name, type_))
        }
        FunctionLocalBinding::Tuple { local, type_ } => {
            FunctionExpr::tuple(TupleFunctionExpr::local_get(local, name, type_))
        }
        FunctionLocalBinding::List(local) => {
            FunctionExpr::list(ListFunctionExpr::local_get(local, name))
        }
        FunctionLocalBinding::Function(local) => {
            FunctionExpr::function(FunctionFunctionExpr::local_get(local, name))
        }
    };
    expression
        .with_resolved_shape(shape)
        .map(Expr::function)
        .ok_or(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        })
}

fn local_get(local: LocalId, name: EcoString, type_: ValueType) -> Result<Expr, PlanError> {
    match (local, type_) {
        (LocalId::Int(local), ValueType::Int) => Ok(Expr::int(IntExpr::local_get(local, name))),
        (LocalId::Float(local), ValueType::Float) => {
            Ok(Expr::float(FloatExpr::local_get(local, name)))
        }
        (LocalId::String(local), ValueType::String) => {
            Ok(Expr::string(StringExpr::local_get(local, name)))
        }
        (LocalId::BitArray(local), ValueType::BitArray) => {
            Ok(Expr::bit_array(BitArrayExpr::local_get(local, name)))
        }
        (LocalId::UtfCodepoint(local), ValueType::UtfCodepoint) => Ok(Expr::utf_codepoint(
            UtfCodepointExpr::local_get(local, name),
        )),
        (LocalId::Bool(local), ValueType::Bool) => Ok(Expr::bool(BoolExpr::local_get(local, name))),
        (LocalId::Nil(local), ValueType::Nil) => Ok(Expr::nil(NilExpr::local_get(local, name))),
        _ => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::Invalid,
            },
        }),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{module_returning_typed_expr, typed_int_expr, typed_prelude_constructor};
    use crate::plan::{
        FunctionType, IntFunctionId, IntFunctionLocalId, IntLocalId, LocalId, ParamLocal,
        RuntimeFunctionId, ValueType,
    };
    use crate::planner::dsl::{
        bool_, call_int_function, function, function_ref, int, int_function_call_arg, local_bool,
        local_float, local_int, local_int_function, local_nil, local_string, module, nil,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use crate::planner::{
        InvalidCustomTypeReason, InvalidExpressionShapeKind, InvalidTypedAstReason, PlanError,
        UnsupportedExpressionKind,
    };
    use ecow::EcoString;
    use gleam_core::ast::{Publicity, Statement, TypedExpr, TypedStatement};
    use gleam_core::type_::{self, Deprecation, ValueConstructor, ValueConstructorVariant};

    #[test]
    fn plan_local_variables() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  1
}

pub fn int_id(value: Int) {
  value
}

pub fn string_id(value: String) {
  value
}

pub fn float_id(value: Float) {
  value
}

pub fn bool_id(value: Bool) {
  value
}

pub fn nil_id(value: Nil) {
  value
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1)),
            [
                function("int_id", local_int(0, "value")).param_int(0, "value"),
                function("string_id", local_string(0, "value")).param_string(0, "value"),
                function("float_id", local_float(0, "value")).param_float(0, "value"),
                function("bool_id", local_bool(0, "value")).param_bool(0, "value"),
                function("nil_id", local_nil(0, "value")).param_nil(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_bool_and_nil_constructors() {
        let actual = plan_module(compile(
            r#"
pub fn truth() {
  True
}

pub fn falsehood() {
  False
}

pub fn main() {
  Nil
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", nil()),
            [
                function("truth", bool_(true)),
                function("falsehood", bool_(false)),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_top_level_function_reference_expression() {
        let actual = plan_module(compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
  1
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1)).evaluate(function_ref(
                RuntimeFunctionId::Int(IntFunctionId(1)),
                [LocalId::Int(IntLocalId(0))],
            )),
            [function("identity", local_int(0, "value")).param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_top_level_function_reference_with_function_argument() {
        let actual = plan_module(compile(
            r#"
fn add_one(value: Int) {
  value + 1
}

fn apply(function: fn(Int) -> Int, value: Int) {
  function(value)
}

pub fn main() {
  apply
  1
}
"#,
        ))
        .expect("source should plan");
        let expected = module(
            "main",
            function("main", int(1)).evaluate(function_ref(
                RuntimeFunctionId::Int(IntFunctionId(2)),
                [
                    ParamLocal::int_function(
                        IntFunctionLocalId(0),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    ),
                    ParamLocal::int(IntLocalId(0)),
                ],
            )),
            [
                function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value"),
                function(
                    "apply",
                    call_int_function(
                        local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                        [int_function_call_arg(0, local_int(0, "value"))],
                    ),
                )
                .param_int_function(0, "function", [ValueType::Int])
                .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_value_constructor_variants() {
        let mut unbound_local = compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        );
        let variable = unbound_local.definitions.functions[0].body.remove(1);
        unbound_local.definitions.functions[0].body = vec![variable];
        assert_eq!(
            plan_module(unbound_local),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal { name: "x".into() },
            }),
        );

        let mut missing_current_function = compile(
            r#"
pub fn main() {
  identity
  1
}

fn identity(value: Int) {
  value
}
"#,
        );
        let identity_index = missing_current_function
            .definitions
            .functions
            .iter()
            .position(|function| {
                function
                    .name
                    .as_ref()
                    .is_some_and(|(_, name)| name == "identity")
            })
            .expect("identity function should exist");
        missing_current_function
            .definitions
            .functions
            .remove(identity_index);
        assert_eq!(
            plan_module(missing_current_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal {
                    name: "identity".into(),
                },
            }),
        );

        let mut non_current_function = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
  1
}
"#,
        );
        let main = non_current_function
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
        let module =
            expect_module_fn_module_mut(expect_expression_statement_mut(&mut main.body[0]));
        *module = "other".into();
        assert_eq!(
            plan_module(non_current_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::ModuleSelect,
                },
            }),
        );

        let mut record_constructor = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed
  1
}
"#,
        );
        record_constructor.definitions.custom_types.clear();
        assert_eq!(
            plan_module(record_constructor),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    name: "Boxed".into(),
                    reason: InvalidCustomTypeReason::UnknownDefinition,
                },
            }),
        );

        let mut constructor_arity_mismatch = compile(
            r#"
pub type Boxed { Boxed(Int) }
pub fn main() { Boxed }
"#,
        );
        let (_, constructor) = expect_var_mut(expect_expression_statement_mut(
            &mut constructor_arity_mismatch.definitions.functions[0].body[0],
        ));
        constructor.variant = ValueConstructorVariant::Record {
            name: "Boxed".into(),
            arity: 0,
            field_map: None,
            location: dummy_span(),
            module: "main".into(),
            variants_count: 1,
            variant_index: 0,
            documentation: None,
        };
        assert_eq!(
            plan_module(constructor_arity_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::RecordConstructor,
                },
            }),
        );

        assert_eq!(
            plan_module(module_returning_typed_expr(typed_prelude_constructor(
                "Other",
                type_::bool(),
            ))),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::PreludeConstructor,
                },
            }),
        );

        assert_eq!(
            plan_module(module_returning_typed_expr(typed_record_constructor(
                "Boxed",
                type_::result(type_::int(), type_::nil()),
                0,
                "other",
            ))),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::ModuleSelect,
                },
            }),
        );
    }

    #[test]
    fn prelude_record_constructor_values_preserve_concrete_function_type() {
        let function_type = type_::fn_(
            vec![type_::int()],
            type_::result(type_::int(), type_::nil()),
        );
        let mut module = module_returning_typed_expr(typed_record_constructor(
            "Ok",
            function_type.clone(),
            1,
            type_::PRELUDE_MODULE_NAME,
        ));
        module.definitions.functions[0].return_type = function_type;
        let plan = plan_module(module).expect("concrete Result constructor should plan");

        assert_eq!(
            plan.main_function().return_().value_type(),
            ValueType::Function(Box::new(FunctionType::new(
                vec![ValueType::Int],
                ValueType::Custom(crate::plan::CustomType::new(
                    crate::plan::CustomTypeName::new("".into(), "gleam".into(), "Result".into(),),
                    vec![ValueType::Int, ValueType::Nil],
                )),
            ))),
        );
    }

    #[test]
    fn reject_profile_polymorphic_custom_constructor_value() {
        assert_eq!(
            plan_module(compile(
                r#"
pub type Boxed(value) {
  Boxed(value)
}

pub fn main() {
  let make = Boxed
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
    fn reject_margin_local_type_shape_mismatch() {
        assert_eq!(
            super::local_get(
                LocalId::Int(IntLocalId(0)),
                "value".into(),
                ValueType::String,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::Invalid,
                },
            }),
        );
    }

    fn expect_expression_statement_mut(statement: &mut TypedStatement) -> &mut TypedExpr {
        let Statement::Expression(expression) = statement else {
            panic!("expected expression statement");
        };
        expression
    }

    fn expect_module_fn_module_mut(expression: &mut TypedExpr) -> &mut ecow::EcoString {
        let (_, constructor) = expect_var_mut(expression);
        let type_::ValueConstructorVariant::ModuleFn { module, .. } = &mut constructor.variant
        else {
            panic!("expected module function constructor");
        };
        module
    }

    fn expect_var_mut(
        expression: &mut TypedExpr,
    ) -> (&mut EcoString, &mut type_::ValueConstructor) {
        let TypedExpr::Var {
            name, constructor, ..
        } = expression
        else {
            panic!("expected variable expression");
        };
        (name, constructor)
    }

    fn typed_record_constructor(
        name: &str,
        type_: std::sync::Arc<type_::Type>,
        arity: u16,
        module: &str,
    ) -> TypedExpr {
        TypedExpr::Var {
            location: dummy_span(),
            name: name.into(),
            constructor: ValueConstructor {
                publicity: Publicity::Private,
                deprecation: Deprecation::NotDeprecated,
                type_,
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
            },
        }
    }

    #[test]
    #[should_panic(expected = "expected expression statement")]
    fn expect_expression_statement_mut_panics_on_assignment() {
        let mut module = compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        );

        expect_expression_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    #[test]
    #[should_panic(expected = "expected variable expression")]
    fn expect_module_fn_module_mut_panics_on_int() {
        let mut expression = typed_int_expr(1);

        expect_module_fn_module_mut(&mut expression);
    }

    #[test]
    #[should_panic(expected = "expected module function constructor")]
    fn expect_module_fn_module_mut_panics_on_prelude_constructor() {
        let mut expression = typed_prelude_constructor("True", type_::bool());

        expect_module_fn_module_mut(&mut expression);
    }
}
