use crate::plan::{BinOp, Expr, FunctionRef, Value};
use crate::planner::context::PlanContext;
use crate::planner::error::PlanError;
use ecow::EcoString;
use gleam_core::ast::{BinOp as GleamBinOp, TypedExpr};
use gleam_core::type_::{PRELUDE_MODULE_NAME, ValueConstructor, ValueConstructorVariant};

pub(super) fn plan_expr(
    expression: TypedExpr,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match expression {
        TypedExpr::Int { int_value, .. } => Ok(Expr::Value(Value::Int(int_value))),
        TypedExpr::String { value, .. } => Ok(Expr::Value(Value::String(value))),
        TypedExpr::Var {
            constructor, name, ..
        } => plan_var(name, constructor, context),
        TypedExpr::Call { fun, arguments, .. } => plan_call(*fun, arguments, context),
        TypedExpr::BinOp {
            operator,
            left,
            right,
            ..
        } => Ok(Expr::BinOp {
            op: plan_bin_op(operator)?,
            left: Box::new(plan_expr(*left, context)?),
            right: Box::new(plan_expr(*right, context)?),
        }),
        TypedExpr::NegateInt { value, .. } => {
            Ok(Expr::NegateInt(Box::new(plan_expr(*value, context)?)))
        }
        TypedExpr::NegateBool { value, .. } => {
            Ok(Expr::NegateBool(Box::new(plan_expr(*value, context)?)))
        }
        TypedExpr::Float { .. } => Err(PlanError::UnsupportedExpression { kind: "float" }),
        TypedExpr::Block { .. } => Err(PlanError::UnsupportedExpression { kind: "block" }),
        TypedExpr::Pipeline { .. } => Err(PlanError::UnsupportedExpression { kind: "pipeline" }),
        TypedExpr::Fn { .. } => Err(PlanError::UnsupportedExpression {
            kind: "anonymous function",
        }),
        TypedExpr::List { .. } => Err(PlanError::UnsupportedExpression { kind: "list" }),
        TypedExpr::Case { .. } => Err(PlanError::UnsupportedExpression { kind: "case" }),
        TypedExpr::RecordAccess { .. } => Err(PlanError::UnsupportedExpression {
            kind: "record access",
        }),
        TypedExpr::PositionalAccess { .. } => Err(PlanError::UnsupportedExpression {
            kind: "positional access",
        }),
        TypedExpr::ModuleSelect { .. } => Err(PlanError::UnsupportedExpression {
            kind: "module select",
        }),
        TypedExpr::Tuple { .. } => Err(PlanError::UnsupportedExpression { kind: "tuple" }),
        TypedExpr::TupleIndex { .. } => Err(PlanError::UnsupportedExpression {
            kind: "tuple index",
        }),
        TypedExpr::Todo { .. } => Err(PlanError::UnsupportedExpression { kind: "todo" }),
        TypedExpr::Panic { .. } => Err(PlanError::UnsupportedExpression { kind: "panic" }),
        TypedExpr::Echo { .. } => Err(PlanError::UnsupportedExpression { kind: "echo" }),
        TypedExpr::BitArray { .. } => Err(PlanError::UnsupportedExpression { kind: "bit array" }),
        TypedExpr::RecordUpdate { .. } => Err(PlanError::UnsupportedExpression {
            kind: "record update",
        }),
        TypedExpr::Invalid { .. } => Err(PlanError::UnsupportedExpression { kind: "invalid" }),
    }
}

fn plan_var(
    name: EcoString,
    constructor: ValueConstructor,
    context: &PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match constructor.variant {
        ValueConstructorVariant::LocalVariable { .. } => {
            let local = context
                .lookup_local(&name)
                .ok_or_else(|| PlanError::UnknownLocal { name: name.clone() })?;
            Ok(Expr::LocalGet { local, name })
        }
        ValueConstructorVariant::Record {
            name,
            module,
            arity,
            ..
        } if arity == 0 && module == PRELUDE_MODULE_NAME => match name.as_str() {
            "True" => Ok(Expr::Value(Value::Bool(true))),
            "False" => Ok(Expr::Value(Value::Bool(false))),
            "Nil" => Ok(Expr::Value(Value::Nil)),
            _ => Err(PlanError::UnsupportedExpression {
                kind: "prelude constructor",
            }),
        },
        ValueConstructorVariant::ModuleFn { .. } => Err(PlanError::UnsupportedExpression {
            kind: "function reference",
        }),
        ValueConstructorVariant::ModuleConstant { .. } => Err(PlanError::UnsupportedExpression {
            kind: "module constant",
        }),
        ValueConstructorVariant::Record { .. } => Err(PlanError::UnsupportedExpression {
            kind: "record constructor",
        }),
    }
}

fn plan_call(
    fun: TypedExpr,
    arguments: Vec<gleam_core::ast::CallArg<TypedExpr>>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    if arguments.iter().any(|argument| argument.label.is_some()) {
        return Err(PlanError::UnsupportedCall {
            reason: "labelled call arguments are not supported",
        });
    }

    if arguments.iter().any(|argument| argument.implicit.is_some()) {
        return Err(PlanError::UnsupportedCall {
            reason: "implicit call arguments are not supported",
        });
    }

    let function = plan_function_ref(fun, context)?;
    let args = arguments
        .into_iter()
        .map(|argument| plan_expr(argument.value, context))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(Expr::Call { function, args })
}

fn plan_function_ref(
    expression: TypedExpr,
    context: &PlanContext<'_>,
) -> Result<FunctionRef, PlanError> {
    let TypedExpr::Var { constructor, .. } = expression else {
        return Err(PlanError::UnsupportedCall {
            reason: "only direct local function calls are supported",
        });
    };

    match constructor.variant {
        ValueConstructorVariant::ModuleFn {
            module,
            name,
            external_erlang,
            external_javascript,
            ..
        } if module == *context.module_name
            && context.function_names.contains(&name)
            && external_erlang.is_none()
            && external_javascript.is_none() =>
        {
            Ok(FunctionRef::Local(name))
        }
        ValueConstructorVariant::ModuleFn { .. } => Err(PlanError::UnsupportedCall {
            reason: "only current-module functions are supported",
        }),
        ValueConstructorVariant::LocalVariable { .. } => Err(PlanError::UnsupportedCall {
            reason: "calling local function values is not supported",
        }),
        ValueConstructorVariant::ModuleConstant { .. } => Err(PlanError::UnsupportedCall {
            reason: "calling module constants is not supported",
        }),
        ValueConstructorVariant::Record { .. } => Err(PlanError::UnsupportedCall {
            reason: "calling record constructors is not supported",
        }),
    }
}

fn plan_bin_op(operator: GleamBinOp) -> Result<BinOp, PlanError> {
    match operator {
        GleamBinOp::AddInt => Ok(BinOp::AddInt),
        GleamBinOp::SubInt => Ok(BinOp::SubInt),
        GleamBinOp::MultInt => Ok(BinOp::MultInt),
        GleamBinOp::LtInt => Ok(BinOp::LtInt),
        GleamBinOp::LtEqInt => Ok(BinOp::LtEqInt),
        GleamBinOp::GtInt => Ok(BinOp::GtInt),
        GleamBinOp::GtEqInt => Ok(BinOp::GtEqInt),
        GleamBinOp::Eq => Ok(BinOp::Eq),
        GleamBinOp::NotEq => Ok(BinOp::NotEq),
        GleamBinOp::Concatenate => Ok(BinOp::Concatenate),
        GleamBinOp::And => Err(PlanError::UnsupportedBinOp { operator: "and" }),
        GleamBinOp::Or => Err(PlanError::UnsupportedBinOp { operator: "or" }),
        GleamBinOp::LtFloat => Err(PlanError::UnsupportedBinOp {
            operator: "lt float",
        }),
        GleamBinOp::LtEqFloat => Err(PlanError::UnsupportedBinOp {
            operator: "lte float",
        }),
        GleamBinOp::GtEqFloat => Err(PlanError::UnsupportedBinOp {
            operator: "gte float",
        }),
        GleamBinOp::GtFloat => Err(PlanError::UnsupportedBinOp {
            operator: "gt float",
        }),
        GleamBinOp::AddFloat => Err(PlanError::UnsupportedBinOp {
            operator: "add float",
        }),
        GleamBinOp::SubFloat => Err(PlanError::UnsupportedBinOp {
            operator: "sub float",
        }),
        GleamBinOp::MultFloat => Err(PlanError::UnsupportedBinOp {
            operator: "mult float",
        }),
        GleamBinOp::DivInt => Err(PlanError::UnsupportedBinOp {
            operator: "div int",
        }),
        GleamBinOp::DivFloat => Err(PlanError::UnsupportedBinOp {
            operator: "div float",
        }),
        GleamBinOp::RemainderInt => Err(PlanError::UnsupportedBinOp {
            operator: "remainder int",
        }),
    }
}

#[cfg(test)]
mod tests {
    use crate::planner::PlanError;
    use crate::planner::dsl::{bool_, function, int, local, module, nil, string};
    use crate::planner::plan_module;
    use crate::planner::support::{compile, compile_minimal_module, dummy_span, expect_plan_error};
    use gleam_core::ast::Publicity;
    use gleam_core::ast::{
        BinOp as GleamBinOp, CallArg, Constant, ImplicitCallArgOrigin, Statement, TypedExpr,
        TypedModule, TypedStatement,
    };
    use gleam_core::type_::{
        self, Deprecation, ModuleValueConstructor, PRELUDE_MODULE_NAME, ValueConstructor,
        ValueConstructorVariant, error::VariableOrigin,
    };
    use num_bigint::BigInt;

    #[test]
    fn plan_string_concatenation() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  "hello, " <> "geam"
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main")
            .function(function("main").return_(string("hello, ").concatenate(string("geam"))))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_integer_comparisons() {
        let actual = plan_module(compile(
            r#"
pub fn lt() {
  1 < 2
}

pub fn lte() {
  1 <= 2
}

pub fn gt() {
  2 > 1
}

pub fn gte() {
  2 >= 1
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main")
            .function(function("lt").return_(int(1).lt_int(int(2))))
            .function(function("lte").return_(int(1).lte_int(int(2))))
            .function(function("gt").return_(int(2).gt_int(int(1))))
            .function(function("gte").return_(int(2).gte_int(int(1))))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_negation_expressions() {
        let actual = plan_module(compile(
            r#"
pub fn negate(value: Int) {
  -value
}

pub fn invert(value: Bool) {
  !value
}
"#,
        ))
        .expect("source should plan");
        let expected = module("main")
            .function(
                function("negate")
                    .param("value")
                    .return_(local("value").negate_int()),
            )
            .function(
                function("invert")
                    .param("value")
                    .return_(local("value").negate_bool()),
            )
            .build();

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
        let expected = module("main")
            .function(function("truth").return_(bool_(true)))
            .function(function("falsehood").return_(bool_(false)))
            .function(function("main").return_(nil()))
            .build();

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_profile_pipeline_expression() {
        assert_eq!(
            expect_plan_error(
                r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  1 |> identity
}
"#,
            ),
            PlanError::UnsupportedExpression { kind: "pipeline" },
        );
    }

    #[test]
    fn reject_profile_list_expression() {
        assert_eq!(
            expect_plan_error(
                r#"
pub fn main() {
  [1, 2, 3]
}
"#,
            ),
            PlanError::UnsupportedExpression { kind: "list" },
        );
    }

    #[test]
    fn reject_profile_expression_variants() {
        let cases = [
            (
                r#"pub fn main() { 1.0 }"#,
                PlanError::UnsupportedExpression { kind: "float" },
            ),
            (
                r#"pub fn main() { { 1 } }"#,
                PlanError::UnsupportedExpression { kind: "block" },
            ),
            (
                r#"pub fn main() { fn(x) { x } }"#,
                PlanError::UnsupportedExpression {
                    kind: "anonymous function",
                },
            ),
            (
                r#"pub fn main() { case 1 { 1 -> 2 _ -> 3 } }"#,
                PlanError::UnsupportedExpression { kind: "case" },
            ),
            (
                r#"pub fn main() { #(1, 2) }"#,
                PlanError::UnsupportedExpression { kind: "tuple" },
            ),
            (
                r#"pub fn main() { #(1, 2).0 }"#,
                PlanError::UnsupportedExpression {
                    kind: "tuple index",
                },
            ),
            (
                r#"pub fn main() { todo }"#,
                PlanError::UnsupportedExpression { kind: "todo" },
            ),
            (
                r#"pub fn main() { panic }"#,
                PlanError::UnsupportedExpression { kind: "panic" },
            ),
            (
                r#"pub fn main() { echo 1 }"#,
                PlanError::UnsupportedExpression { kind: "echo" },
            ),
            (
                r#"pub fn main() { <<1>> }"#,
                PlanError::UnsupportedExpression { kind: "bit array" },
            ),
            (
                r#"fn identity(value: Int) { value } pub fn main() { identity }"#,
                PlanError::UnsupportedExpression {
                    kind: "function reference",
                },
            ),
        ];

        for (src, expected) in cases {
            assert_eq!(expect_plan_error(src), expected);
        }
    }

    #[test]
    fn reject_margin_expression_variants() {
        let synthetic_cases = [
            (
                module_returning_typed_expr(TypedExpr::PositionalAccess {
                    location: dummy_span(),
                    type_: type_::int(),
                    index: 0,
                    record: Box::new(typed_int_expr(1)),
                }),
                PlanError::UnsupportedExpression {
                    kind: "positional access",
                },
            ),
            (
                module_returning_typed_expr(TypedExpr::ModuleSelect {
                    location: dummy_span(),
                    field_start: 0,
                    type_: type_::int(),
                    label: "answer".into(),
                    module_name: "other".into(),
                    module_alias: "other".into(),
                    constructor: ModuleValueConstructor::Constant {
                        literal: Constant::Int {
                            location: dummy_span(),
                            value: "1".into(),
                            int_value: BigInt::from(1),
                        },
                        location: dummy_span(),
                        documentation: None,
                    },
                }),
                PlanError::UnsupportedExpression {
                    kind: "module select",
                },
            ),
            (
                module_returning_typed_expr(TypedExpr::Invalid {
                    location: dummy_span(),
                    type_: type_::int(),
                    extra_information: None,
                }),
                PlanError::UnsupportedExpression { kind: "invalid" },
            ),
        ];

        for (module, expected) in synthetic_cases {
            assert_eq!(plan_module(module), Err(expected));
        }

        let mut record_access = compile(
            r#"
pub type Boxed {
  Boxed(value: Int)
}

pub fn main() {
  Boxed(1).value
}
"#,
        );
        record_access.definitions.custom_types.clear();
        assert_eq!(
            plan_module(record_access),
            Err(PlanError::UnsupportedExpression {
                kind: "record access",
            }),
        );

        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::RecordUpdate {
                location: dummy_span(),
                spread_start: 0,
                type_: type_::int(),
                updated_record: Box::new(typed_int_expr(1)),
                updated_record_assigned_name: None,
                constructor: Box::new(typed_int_expr(1)),
                arguments: Vec::new(),
            })),
            Err(PlanError::UnsupportedExpression {
                kind: "record update",
            }),
        );
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
            Err(PlanError::UnknownLocal { name: "x".into() }),
        );

        let mut module_constant = compile(
            r#"
const answer = 1

pub fn main() {
  answer
}
"#,
        );
        module_constant.definitions.constants.clear();
        assert_eq!(
            plan_module(module_constant),
            Err(PlanError::UnsupportedExpression {
                kind: "module constant",
            }),
        );

        let mut record_constructor = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed
}
"#,
        );
        record_constructor.definitions.custom_types.clear();
        assert_eq!(
            plan_module(record_constructor),
            Err(PlanError::UnsupportedExpression {
                kind: "record constructor",
            }),
        );

        assert_eq!(
            plan_module(module_returning_typed_expr(TypedExpr::Var {
                location: dummy_span(),
                name: "Other".into(),
                constructor: ValueConstructor {
                    publicity: Publicity::Private,
                    deprecation: Deprecation::NotDeprecated,
                    type_: type_::bool(),
                    variant: ValueConstructorVariant::Record {
                        name: "Other".into(),
                        arity: 0,
                        field_map: None,
                        location: dummy_span(),
                        module: PRELUDE_MODULE_NAME.into(),
                        variants_count: 1,
                        variant_index: 0,
                        documentation: None,
                    },
                },
            })),
            Err(PlanError::UnsupportedExpression {
                kind: "prelude constructor",
            }),
        );

        let mut local_variable_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (fun, _) =
            expect_call_statement_mut(&mut local_variable_call.definitions.functions[1].body[0]);
        let constructor = expect_var_constructor_mut(fun);
        constructor.variant = ValueConstructorVariant::LocalVariable {
            location: dummy_span(),
            origin: VariableOrigin::generated(),
        };
        assert_eq!(
            plan_module(local_variable_call),
            Err(PlanError::UnsupportedCall {
                reason: "calling local function values is not supported",
            }),
        );

        let module_constant_call = compile(
            r#"
const answer = 1

pub fn main() {
  answer
}
"#,
        );
        reject_margin_module_constant_call(module_constant_call);
    }

    fn reject_margin_module_constant_call(mut module_constant_call: TypedModule) {
        module_constant_call.definitions.constants.clear();
        let statement = module_constant_call.definitions.functions[0].body.remove(0);
        let Statement::Expression(module_constant) = statement else {
            panic!("expected expression statement");
        };
        module_constant_call.definitions.functions[0].body =
            vec![Statement::Expression(TypedExpr::Call {
                location: dummy_span(),
                type_: type_::int(),
                fun: Box::new(module_constant),
                arguments: Vec::new(),
                open_parenthesis: Some(0),
            })];
        assert_eq!(
            plan_module(module_constant_call),
            Err(PlanError::UnsupportedCall {
                reason: "calling module constants is not supported",
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected expression statement")]
    fn reject_margin_module_constant_call_panics_on_assignment_statement() {
        reject_margin_module_constant_call(compile(
            r#"
pub fn main() {
  let x = 1
  x
}
"#,
        ));
    }

    #[test]
    fn reject_margin_call_shapes() {
        let mut labelled_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, arguments) =
            expect_call_statement_mut(&mut labelled_call.definitions.functions[1].body[0]);
        arguments[0].label = Some("value".into());
        assert_eq!(
            plan_module(labelled_call),
            Err(PlanError::UnsupportedCall {
                reason: "labelled call arguments are not supported",
            }),
        );

        let mut implicit_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (_, arguments) =
            expect_call_statement_mut(&mut implicit_call.definitions.functions[1].body[0]);
        arguments[0].implicit = Some(ImplicitCallArgOrigin::Pipe);
        assert_eq!(
            plan_module(implicit_call),
            Err(PlanError::UnsupportedCall {
                reason: "implicit call arguments are not supported",
            }),
        );

        let mut non_direct_call = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        let (fun, _) =
            expect_call_statement_mut(&mut non_direct_call.definitions.functions[1].body[0]);
        *fun = typed_int_expr(1);
        assert_eq!(
            plan_module(non_direct_call),
            Err(PlanError::UnsupportedCall {
                reason: "only direct local function calls are supported",
            }),
        );

        let non_local_module_fn = compile(
            r#"
fn identity(value: Int) {
  value
}

pub fn main() {
  identity(1)
}
"#,
        );
        reject_margin_non_local_module_fn_call(non_local_module_fn);

        let mut record_constructor_call = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed(1)
}
"#,
        );
        record_constructor_call.definitions.custom_types.clear();
        assert_eq!(
            plan_module(record_constructor_call),
            Err(PlanError::UnsupportedCall {
                reason: "calling record constructors is not supported",
            }),
        );
    }

    fn reject_margin_non_local_module_fn_call(mut non_local_module_fn: TypedModule) {
        let function = non_local_module_fn
            .definitions
            .functions
            .last_mut()
            .expect("expected test module to have a function");
        let (fun, _) = expect_call_statement_mut(&mut function.body[0]);
        let constructor = expect_var_constructor_mut(fun);
        let ValueConstructorVariant::ModuleFn { module, .. } = &mut constructor.variant else {
            panic!("expected module function constructor");
        };
        *module = "other".into();
        assert_eq!(
            plan_module(non_local_module_fn),
            Err(PlanError::UnsupportedCall {
                reason: "only current-module functions are supported",
            }),
        );
    }

    #[test]
    #[should_panic(expected = "expected module function constructor")]
    fn reject_margin_non_local_module_fn_call_panics_on_record_constructor() {
        let record_constructor_call = compile(
            r#"
pub type Boxed {
  Boxed(Int)
}

pub fn main() {
  Boxed(1)
}
"#,
        );
        reject_margin_non_local_module_fn_call(record_constructor_call);
    }

    #[test]
    fn reject_margin_binary_operators() {
        let cases = [
            (GleamBinOp::And, "and"),
            (GleamBinOp::Or, "or"),
            (GleamBinOp::LtFloat, "lt float"),
            (GleamBinOp::LtEqFloat, "lte float"),
            (GleamBinOp::GtEqFloat, "gte float"),
            (GleamBinOp::GtFloat, "gt float"),
            (GleamBinOp::AddFloat, "add float"),
            (GleamBinOp::SubFloat, "sub float"),
            (GleamBinOp::MultFloat, "mult float"),
            (GleamBinOp::DivInt, "div int"),
            (GleamBinOp::DivFloat, "div float"),
            (GleamBinOp::RemainderInt, "remainder int"),
        ];

        for (operator, expected) in cases {
            assert_eq!(
                plan_module(module_returning_typed_expr(TypedExpr::BinOp {
                    location: dummy_span(),
                    type_: type_::int(),
                    operator,
                    operator_start: 0,
                    left: Box::new(typed_int_expr(1)),
                    right: Box::new(typed_int_expr(2)),
                })),
                Err(PlanError::UnsupportedBinOp { operator: expected }),
            );
        }
    }

    fn module_returning_typed_expr(expression: TypedExpr) -> TypedModule {
        let mut module = compile_minimal_module();
        module.definitions.functions[0].body = vec![Statement::Expression(expression)];
        module
    }

    fn expect_call_statement_mut(
        statement: &mut TypedStatement,
    ) -> (&mut TypedExpr, &mut Vec<CallArg<TypedExpr>>) {
        let Statement::Expression(TypedExpr::Call { fun, arguments, .. }) = statement else {
            panic!("expected call expression statement");
        };
        (fun.as_mut(), arguments)
    }

    #[test]
    #[should_panic(expected = "expected call expression statement")]
    fn expect_call_statement_mut_panics_on_expression() {
        let mut module = compile_minimal_module();

        expect_call_statement_mut(&mut module.definitions.functions[0].body[0]);
    }

    fn expect_var_constructor_mut(expression: &mut TypedExpr) -> &mut ValueConstructor {
        let TypedExpr::Var { constructor, .. } = expression else {
            panic!("expected variable expression");
        };
        constructor
    }

    #[test]
    #[should_panic(expected = "expected variable expression")]
    fn expect_var_constructor_mut_panics_on_int() {
        let mut expression = typed_int_expr(1);

        expect_var_constructor_mut(&mut expression);
    }

    fn typed_int_expr(value: i64) -> TypedExpr {
        TypedExpr::Int {
            location: dummy_span(),
            type_: type_::int(),
            value: value.to_string().into(),
            int_value: BigInt::from(value),
        }
    }
}
