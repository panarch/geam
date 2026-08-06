use crate::plan::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CustomExpr, CustomFunctionExpr,
    Expr, ExternalExpr, ExternalFunctionExpr, FloatExpr, FloatFunctionExpr, FunctionExpr,
    FunctionFunctionExpr, GenericFunctionExpr, IntExpr, IntFunctionExpr, ListExpr,
    ListFunctionExpr, LocalId, NilExpr, NilFunctionExpr, StringExpr, StringFunctionExpr, TupleExpr,
    TupleFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
};
use crate::planner::context::{
    FunctionLocalBinding, ModuleFunctionTarget, PlanContext, ResolvedLocal,
};
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidModuleReferenceReason, InvalidTypedAstReason, PlanError,
};
use crate::planner::expression::record_constructor::ResolvedRecordConstructor;
use ecow::EcoString;
use gleam_core::type_::{
    ModuleValueConstructor, PRELUDE_MODULE_NAME, ValueConstructor, ValueConstructorVariant,
};

pub(super) fn plan_var(
    name: EcoString,
    constructor: ValueConstructor,
    constructor_shape: crate::plan::ValueShape,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match constructor.variant {
        ValueConstructorVariant::LocalVariable { .. } => {
            let expression = match context.resolve_local(&name)? {
                ResolvedLocal::Primitive(local) => local_get(local, name),
                ResolvedLocal::Custom(local) => Expr::custom(CustomExpr::local_get(local, name)),
                ResolvedLocal::External(local) => {
                    Expr::external(ExternalExpr::local_get(local, name))
                }
                ResolvedLocal::Tuple { local, shape } => {
                    let type_ = shape
                        .iter()
                        .map(crate::plan::ValueShape::value_type)
                        .collect();
                    Expr::tuple(TupleExpr::local_get(local, name, type_).with_shape(shape))
                }
                ResolvedLocal::List { local, item_shape } => {
                    Expr::list(ListExpr::local_get(local, name).with_item_shape(item_shape))
                }
                ResolvedLocal::Function { binding, shape } => {
                    function_local_get(binding, name, shape)?
                }
            };

            Ok(expression)
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
            if module == PRELUDE_MODULE_NAME && !matches!(name.as_str(), "Ok" | "Error") {
                return Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::ExpressionShape {
                        kind: InvalidExpressionShapeKind::PreludeConstructor,
                    },
                });
            }
            let shape = constructor_shape;
            let constructor = context.custom_constructor(&constructor)?;
            ResolvedRecordConstructor::direct(module.clone(), name.clone(), constructor)
                .plan_reference(shape)
        }
        ValueConstructorVariant::ModuleFn {
            module,
            name,
            external_erlang,
            external_javascript,
            ..
        } => {
            let target = ModuleFunctionTarget::direct(
                module,
                name,
                external_erlang.is_some() || external_javascript.is_some(),
            );
            plan_function_reference(target, constructor_shape, context)
        }
        ValueConstructorVariant::ModuleConstant { module, name, .. } => {
            context.module_constant_expr(&module, &name, &constructor_shape)
        }
    }
}

pub(super) fn plan_module_select(
    module_name: EcoString,
    label: EcoString,
    constructor: ModuleValueConstructor,
    shape: crate::plan::ValueShape,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match constructor {
        ModuleValueConstructor::Fn {
            module,
            name,
            external_erlang,
            external_javascript,
            ..
        } => {
            let target = ModuleFunctionTarget::selected(
                context,
                module_name,
                label,
                module,
                name,
                external_erlang.is_some() || external_javascript.is_some(),
            )?;
            plan_function_reference(target, shape, context)
        }
        ModuleValueConstructor::Constant { .. } => {
            context.module_constant_expr(&module_name, &label, &shape)
        }
        ModuleValueConstructor::Record {
            name,
            variant_index,
            arity,
            type_,
            ..
        } => ResolvedRecordConstructor::selected(
            context,
            module_name,
            label,
            name,
            type_,
            usize::from(variant_index),
            usize::from(arity),
        )?
        .plan_reference(shape),
    }
}

fn plan_function_reference(
    target: ModuleFunctionTarget,
    constructor_shape: crate::plan::ValueShape,
    context: &PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let target = target.validate_external(context)?;
    let function = context.module_function(&target)?;
    let shape = target.function_shape(constructor_shape)?;
    let instantiation = target.instantiate_reference(&function, &shape)?;
    let reference = function.reference(instantiation);

    FunctionExpr::reference(reference)
        .with_shape(shape)
        .map(Expr::function)
        .ok_or_else(|| PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ModuleReference {
                module: target.module().clone(),
                name: target.name().clone(),
                reason: InvalidModuleReferenceReason::FunctionReferenceShape,
            },
        })
}

fn function_local_get(
    binding: FunctionLocalBinding,
    name: EcoString,
    shape: crate::plan::FunctionShape,
) -> Result<Expr, PlanError> {
    let expression = match binding {
        FunctionLocalBinding::Generic(local) => {
            FunctionExpr::generic(GenericFunctionExpr::local_get(local, name))
        }
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
        FunctionLocalBinding::External(local) => {
            FunctionExpr::external(ExternalFunctionExpr::local_get(local, name))
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
                kind: InvalidExpressionShapeKind::VariableFunctionLocalShape,
            },
        })
}

fn local_get(local: LocalId, name: EcoString) -> Expr {
    match local {
        LocalId::Generic(local) => Expr::generic(crate::plan::GenericExpr::local_get(local, name)),
        LocalId::Int(local) => Expr::int(IntExpr::local_get(local, name)),
        LocalId::Float(local) => Expr::float(FloatExpr::local_get(local, name)),
        LocalId::String(local) => Expr::string(StringExpr::local_get(local, name)),
        LocalId::BitArray(local) => Expr::bit_array(BitArrayExpr::local_get(local, name)),
        LocalId::UtfCodepoint(local) => {
            Expr::utf_codepoint(UtfCodepointExpr::local_get(local, name))
        }
        LocalId::Bool(local) => Expr::bool(BoolExpr::local_get(local, name)),
        LocalId::Nil(local) => Expr::nil(NilExpr::local_get(local, name)),
    }
}

#[cfg(test)]
mod tests {
    use super::super::{module_returning_typed_expr, typed_int_expr, typed_prelude_constructor};
    use crate::plan::{
        FunctionShape, FunctionType, IntFunctionId, IntFunctionLocalId, IntLocalId, LocalId,
        ParamLocal, RuntimeFunctionId, ValueShape, ValueType,
    };
    use crate::planner::context::{AnonymousFunctions, FunctionInfo, PlanContext};
    use crate::planner::dsl::{
        bool_, call_int_function_at, function, function_ref, host_call_site, int,
        int_function_call_arg, local_bool, local_float, local_int, local_int_function, local_nil,
        local_string, module, nil,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use crate::planner::{
        InvalidCustomTypeReason, InvalidExpressionShapeKind, InvalidModuleReferenceReason,
        InvalidTypedAstReason, PlanError,
    };
    use ecow::EcoString;
    use gleam_core::ast::{Publicity, Statement, TypedExpr, TypedStatement};
    use gleam_core::type_::{
        self, Deprecation, ModuleValueConstructor, ValueConstructor, ValueConstructorVariant,
    };
    use std::collections::HashMap;

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
        let source = r#"
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
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
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
                    call_int_function_at(
                        local_int_function(0, "function", [LocalId::Int(IntLocalId(0))]),
                        [int_function_call_arg(local_int(0, "value"))],
                        host_call_site(source, "apply", "function(value)"),
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
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::MissingFunction,
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
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
                },
            }),
        );

        let mut external_function = compile(
            r#"
@external(erlang, "external", "identity")
fn identity(value: Int) -> Int

pub fn main() {
  identity
}
"#,
        );
        external_function.definitions.functions.retain(|function| {
            function
                .name
                .as_ref()
                .is_some_and(|(_, name)| name == "main")
        });
        assert_eq!(
            plan_module(external_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::ExternalFunction,
                },
            }),
        );

        let mut invalid_function_shape = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
}
"#,
        );
        let (_, constructor) = expect_var_mut(expect_expression_statement_mut(
            &mut invalid_function_shape.definitions.functions[1].body[0],
        ));
        constructor.type_ = type_::int();
        assert_eq!(
            plan_module(invalid_function_shape),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::FunctionType,
                },
            }),
        );

        let mut function_return_shape_mismatch = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
}
"#,
        );
        let (_, constructor) = expect_var_mut(expect_expression_statement_mut(
            &mut function_return_shape_mismatch.definitions.functions[1].body[0],
        ));
        constructor.type_ = type_::fn_(vec![type_::int()], type_::string());
        assert_eq!(
            plan_module(function_return_shape_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::FunctionInstantiation,
                },
            }),
        );

        let mut missing_constant = compile(
            r#"
const answer = 1

pub fn main() {
  answer
}
"#,
        );
        missing_constant.definitions.constants.clear();
        assert_eq!(
            plan_module(missing_constant),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "answer".into(),
                    reason: InvalidModuleReferenceReason::MissingConstant,
                },
            }),
        );

        let mut incompatible_constant_instantiation = compile(
            r#"
const values = []

pub fn main() {
  values
}
"#,
        );
        let (_, constructor) = expect_var_mut(expect_expression_statement_mut(
            &mut incompatible_constant_instantiation.definitions.functions[0].body[0],
        ));
        constructor.type_ = type_::int();
        assert_eq!(
            plan_module(incompatible_constant_instantiation),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "values".into(),
                    reason: InvalidModuleReferenceReason::ConstantInstantiation,
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
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Boxed".into(),
                    reason: Box::new(InvalidCustomTypeReason::MissingDefinition),
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
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Boxed".into(),
                    reason: Box::new(InvalidCustomTypeReason::ConstructorArity {
                        expected: 1,
                        actual: 0,
                    }),
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
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "Boxed".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
                },
            }),
        );
    }

    #[test]
    fn module_select_values_preserve_constructor_and_function_ownership() {
        let constructor_source = r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed
}
"#;
        let expected_constructor = plan_module(compile(constructor_source));
        let mut module_constructor = compile(constructor_source);
        replace_main_value_with_module_select(&mut module_constructor);
        assert_eq!(plan_module(module_constructor), expected_constructor);

        let mut unlinked_constructor = compile(constructor_source);
        let (_, module_name, _, _) =
            replace_main_value_with_module_select(&mut unlinked_constructor);
        *module_name = "other".into();
        assert_eq!(
            plan_module(unlinked_constructor),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "Boxed".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
                },
            }),
        );

        let mut arity_mismatch = compile(constructor_source);
        let (_, _, _, constructor) = replace_main_value_with_module_select(&mut arity_mismatch);
        let (arity, _) = module_record_fields(constructor);
        *arity = 0;
        assert_eq!(
            plan_module(arity_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "geam".into(),
                    module: "main".into(),
                    name: "Boxed".into(),
                    reason: Box::new(InvalidCustomTypeReason::ConstructorArity {
                        expected: 1,
                        actual: 0,
                    }),
                },
            }),
        );

        let mut constructor_type_mismatch = compile(constructor_source);
        let (_, _, _, constructor) =
            replace_main_value_with_module_select(&mut constructor_type_mismatch);
        let (_, type_) = module_record_fields(constructor);
        *type_ = type_::int();
        assert_eq!(
            plan_module(constructor_type_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::CustomType {
                    package: "".into(),
                    module: "main".into(),
                    name: "Boxed".into(),
                    reason: Box::new(InvalidCustomTypeReason::ConstructorType {
                        actual: crate::plan::ValueType::Int,
                    }),
                },
            }),
        );

        let mut outer_shape_mismatch = compile(constructor_source);
        let (type_, _, _, _) = replace_main_value_with_module_select(&mut outer_shape_mismatch);
        *type_ = type_::int();
        assert_eq!(
            plan_module(outer_shape_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "Boxed".into(),
                    reason: InvalidModuleReferenceReason::RecordConstructorResultShape,
                },
            }),
        );

        let mut constructor_label_mismatch = compile(constructor_source);
        let (_, _, label, _) =
            replace_main_value_with_module_select(&mut constructor_label_mismatch);
        *label = "Other".into();
        assert_eq!(
            plan_module(constructor_label_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "Other".into(),
                    reason: InvalidModuleReferenceReason::RecordConstructorName {
                        actual: "Boxed".into(),
                    },
                },
            }),
        );

        let function_source = r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
}
"#;
        let expected_function = plan_module(compile(function_source));
        let mut module_function = compile(function_source);
        replace_main_value_with_module_select(&mut module_function);
        assert_eq!(plan_module(module_function), expected_function);

        let mut outer_module_mismatch = compile(function_source);
        let (_, module_name, _, _) =
            replace_main_value_with_module_select(&mut outer_module_mismatch);
        *module_name = "other".into();
        assert_eq!(
            plan_module(outer_module_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
                },
            }),
        );

        let mut target_module_mismatch = compile(function_source);
        let (_, _, _, constructor) =
            replace_main_value_with_module_select(&mut target_module_mismatch);
        *module_function_module(constructor) = "other".into();
        assert_eq!(
            plan_module(target_module_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::FunctionModule {
                        actual: "other".into(),
                    },
                },
            }),
        );

        let mut function_label_mismatch = compile(function_source);
        let (_, _, label, _) = replace_main_value_with_module_select(&mut function_label_mismatch);
        *label = "other".into();
        assert_eq!(
            plan_module(function_label_mismatch),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "other".into(),
                    reason: InvalidModuleReferenceReason::FunctionName {
                        actual: "identity".into(),
                    },
                },
            }),
        );

        let mut external_function = compile(function_source);
        let (_, _, _, constructor) = replace_main_value_with_module_select(&mut external_function);
        let external_erlang = module_function_external_erlang(constructor);
        *external_erlang = Some(("external".into(), "identity".into()));
        assert_eq!(
            plan_module(external_function),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "main".into(),
                    name: "identity".into(),
                    reason: InvalidModuleReferenceReason::ExternalFunction,
                },
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected variable expression")]
    fn module_select_value_helper_rejects_literals() {
        let mut module = compile("pub fn main() { 1 }");

        replace_main_value_with_module_select(&mut module);
    }

    #[test]
    #[should_panic(expected = "expected module value constructor")]
    fn module_select_value_helper_rejects_locals() {
        let mut module = compile(
            r#"
pub fn main() {
  let value = 1
  value
}
"#,
        );
        module.definitions.functions[0].body.remove(0);

        replace_main_value_with_module_select(&mut module);
    }

    #[test]
    #[should_panic(expected = "expected expression statement")]
    fn module_select_value_helper_rejects_assignments() {
        let mut module = compile(
            r#"
pub fn main() {
  let value = 1
  value
}
"#,
        );

        replace_main_value_with_module_select(&mut module);
    }

    #[test]
    #[should_panic(expected = "expected module record constructor")]
    fn module_record_fields_rejects_function_constructor() {
        let mut module = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity
}
"#,
        );
        let (_, _, _, constructor) = replace_main_value_with_module_select(&mut module);

        module_record_fields(constructor);
    }

    #[test]
    #[should_panic(expected = "expected module function constructor")]
    fn module_function_external_rejects_record_constructor() {
        let mut module = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed
}
"#,
        );
        let (_, _, _, constructor) = replace_main_value_with_module_select(&mut module);

        module_function_external_erlang(constructor);
    }

    #[test]
    #[should_panic(expected = "expected module function constructor")]
    fn module_function_module_rejects_record_constructor() {
        let mut module = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed
}
"#,
        );
        let (_, _, _, constructor) = replace_main_value_with_module_select(&mut module);

        module_function_module(constructor);
    }

    #[test]
    #[should_panic(expected = "expected module select expression")]
    fn module_select_parts_rejects_literals() {
        let mut expression = typed_int_expr(1);

        module_select_parts(&mut expression);
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
    fn reject_margin_function_local_shape_mismatch_propagates_from_var_owner() {
        let module_name = EcoString::from("main");
        let functions = HashMap::<EcoString, FunctionInfo>::new();
        let mut anonymous = AnonymousFunctions::default();
        let mut context = PlanContext::new(&module_name, &functions, &mut anonymous);
        let function_type = FunctionType::new(Vec::new(), ValueType::Int);
        context.define_int_function_local_shape(
            "f".into(),
            function_type,
            FunctionShape::new(Vec::new(), ValueShape::String),
        );
        let constructor = ValueConstructor::local_variable(
            dummy_span(),
            type_::error::VariableOrigin::generated(),
            type_::fn_(Vec::new(), type_::int()),
        );

        assert_eq!(
            super::plan_var(
                "f".into(),
                constructor,
                ValueShape::Function(Box::new(FunctionShape::new(Vec::new(), ValueShape::Int,))),
                &mut context,
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::VariableFunctionLocalShape,
                },
            }),
        );
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

    fn replace_main_value_with_module_select(
        module: &mut gleam_core::ast::TypedModule,
    ) -> (
        &mut std::sync::Arc<type_::Type>,
        &mut EcoString,
        &mut EcoString,
        &mut ModuleValueConstructor,
    ) {
        let main = module
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
        let Statement::Expression(expression) = &mut main.body[0] else {
            panic!("expected expression statement");
        };
        let TypedExpr::Var {
            location,
            name,
            constructor,
        } = std::mem::replace(expression, typed_int_expr(0))
        else {
            panic!("expected variable expression");
        };
        let type_ = constructor.type_.clone();
        let module_constructor = match constructor.variant {
            ValueConstructorVariant::Record {
                name,
                module,
                variant_index,
                arity,
                field_map,
                location,
                documentation,
                ..
            } => (
                module,
                ModuleValueConstructor::Record {
                    name,
                    variant_index,
                    arity,
                    type_: type_.clone(),
                    field_map,
                    location,
                    documentation,
                },
            ),
            ValueConstructorVariant::ModuleFn {
                location,
                module,
                name,
                external_erlang,
                external_javascript,
                field_map,
                documentation,
                purity,
                ..
            } => (
                module.clone(),
                ModuleValueConstructor::Fn {
                    location,
                    module,
                    name,
                    external_erlang,
                    external_javascript,
                    field_map,
                    documentation,
                    purity,
                },
            ),
            ValueConstructorVariant::ModuleConstant { .. }
            | ValueConstructorVariant::LocalVariable { .. } => {
                panic!("expected module value constructor");
            }
        };
        let (module_name, module_constructor) = module_constructor;
        *expression = TypedExpr::ModuleSelect {
            location,
            field_start: 0,
            type_,
            label: name,
            module_name: module_name.clone(),
            module_alias: module_name,
            constructor: module_constructor,
        };
        module_select_parts(expression)
    }

    fn module_select_parts(
        expression: &mut TypedExpr,
    ) -> (
        &mut std::sync::Arc<type_::Type>,
        &mut EcoString,
        &mut EcoString,
        &mut ModuleValueConstructor,
    ) {
        match expression {
            TypedExpr::ModuleSelect {
                type_,
                module_name,
                label,
                constructor,
                ..
            } => (type_, module_name, label, constructor),
            _ => panic!("expected module select expression"),
        }
    }

    fn module_record_fields(
        constructor: &mut ModuleValueConstructor,
    ) -> (&mut u16, &mut std::sync::Arc<type_::Type>) {
        match constructor {
            ModuleValueConstructor::Record { arity, type_, .. } => (arity, type_),
            _ => panic!("expected module record constructor"),
        }
    }

    fn module_function_external_erlang(
        constructor: &mut ModuleValueConstructor,
    ) -> &mut Option<(EcoString, EcoString)> {
        match constructor {
            ModuleValueConstructor::Fn {
                external_erlang, ..
            } => external_erlang,
            _ => panic!("expected module function constructor"),
        }
    }

    fn module_function_module(constructor: &mut ModuleValueConstructor) -> &mut EcoString {
        match constructor {
            ModuleValueConstructor::Fn { module, .. } => module,
            _ => panic!("expected module function constructor"),
        }
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
}
