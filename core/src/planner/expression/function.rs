mod free_variables;

use crate::plan::{CaptureArg, Expr, FunctionExpr, FunctionShape, FunctionType, ValueShape};
use crate::planner::context::PlanContext;
use crate::planner::error::{
    InvalidExpressionShapeKind, InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError,
};
use crate::planner::function::{anonymous_function_plan, plan_anonymous_function_body};
use crate::planner::module::function_params_in;
use crate::planner::type_parameter::TypeParameterScope;
use gleam_compiler_core::ast::{
    CAPTURE_VARIABLE, CallArg as GleamCallArg, FunctionLiteralKind, Statement, TypedArg, TypedExpr,
    TypedStatement,
};
use gleam_compiler_core::type_::{Type, ValueConstructorVariant};
use std::sync::Arc;
use vec1::Vec1;

pub(super) fn plan_anonymous(
    type_: Arc<Type>,
    kind: FunctionLiteralKind,
    arguments: Vec<TypedArg>,
    body: Vec1<TypedStatement>,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    match kind {
        FunctionLiteralKind::Anonymous { .. } => {}
        FunctionLiteralKind::Capture { .. } => {
            validate_capture_literal(&arguments, &body)?;
        }
        FunctionLiteralKind::Use { .. } => {
            return Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::FunctionLiteralKind,
                },
            });
        }
    }

    let mut type_parameters = context.type_parameters().clone();
    let error_name = context.anonymous_function_error_name();
    let function_shape = anonymous_function_shape(
        error_name.clone(),
        context.value_shape_with_parameters(type_.as_ref(), &mut type_parameters),
    )?;
    let function_type = function_shape.type_();
    let params = function_params_in(
        error_name.clone(),
        &arguments,
        &mut type_parameters,
        &|name| context.is_external_type(name),
    )?;
    validate_argument_types(&error_name, &function_type, &params)?;
    plan_anonymous_with_valid_arguments(
        function_shape,
        params,
        arguments,
        body,
        type_parameters,
        context,
    )
}

fn validate_capture_literal(
    arguments: &[TypedArg],
    body: &Vec1<TypedStatement>,
) -> Result<(), PlanError> {
    let valid = match (arguments, body.as_slice()) {
        (
            [argument],
            [
                Statement::Expression(TypedExpr::Call {
                    arguments: call_arguments,
                    ..
                }),
            ],
        ) => {
            argument.get_variable_name().map(|name| name.as_str()) == Some(CAPTURE_VARIABLE)
                && count_capture_literal_arguments(call_arguments) == 1
        }
        _ => false,
    };
    if !valid {
        return Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::FunctionCaptureLiteral,
            },
        });
    }
    Ok(())
}

fn count_capture_literal_arguments(arguments: &[GleamCallArg<TypedExpr>]) -> usize {
    arguments
        .iter()
        .filter(|argument| is_capture_literal_local(&argument.value))
        .count()
}

fn is_capture_literal_local(expression: &TypedExpr) -> bool {
    matches!(
        expression,
        TypedExpr::Var {
            name,
            constructor,
            ..
        } if name.as_str() == CAPTURE_VARIABLE
            && matches!(
                constructor.variant,
                ValueConstructorVariant::LocalVariable { .. }
            )
    )
}

fn anonymous_function_shape(
    name: ecow::EcoString,
    shape: ValueShape,
) -> Result<FunctionShape, PlanError> {
    match shape {
        ValueShape::Function(shape) => Ok(*shape),
        actual => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name,
                reason: InvalidFunctionShapeReason::ExpressionType {
                    actual: actual.value_type(),
                },
            },
        }),
    }
}

fn validate_argument_types(
    name: &ecow::EcoString,
    type_: &FunctionType,
    params: &[crate::planner::context::FunctionParam],
) -> Result<(), PlanError> {
    let actual = params
        .iter()
        .map(|param| param.local().value_type())
        .collect::<Vec<_>>();

    if actual == type_.argument_types() {
        Ok(())
    } else {
        Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: name.clone(),
                reason: InvalidFunctionShapeReason::ArgumentTypeMismatch,
            },
        })
    }
}

fn plan_anonymous_with_valid_arguments(
    function_shape: FunctionShape,
    params: Vec<crate::planner::context::FunctionParam>,
    arguments: Vec<TypedArg>,
    body: Vec1<TypedStatement>,
    type_parameters: TypeParameterScope,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let free_names = free_variables::anonymous_free_variables(&arguments, &body);
    let captures = context.capture_bindings(&free_names)?;
    plan_anonymous_with_captures(
        function_shape,
        params,
        captures,
        body,
        type_parameters,
        context,
    )
}

fn plan_anonymous_with_captures(
    function_shape: FunctionShape,
    params: Vec<crate::planner::context::FunctionParam>,
    captures: Vec<crate::planner::context::CaptureBinding>,
    body: Vec1<TypedStatement>,
    type_parameters: TypeParameterScope,
    context: &mut PlanContext<'_>,
) -> Result<Expr, PlanError> {
    let return_shape = function_shape.return_shape().clone();
    let name = context.reserve_anonymous_function_name();
    let (name, info) = context.allocate_anonymous_function_shape(
        name,
        return_shape.clone(),
        params.clone(),
        type_parameters.clone(),
    );

    let planned = {
        let mut body_context = context.anonymous_function_context(name.clone(), type_parameters);
        plan_anonymous_function_body(
            &name,
            &return_shape,
            &params,
            captures,
            body,
            &mut body_context,
        )
    };

    let planned = planned?;
    let instantiation = info.signature.identity_instantiation();
    let (function, captures) = anonymous_function_plan(info, name, planned);
    let value = closure_expr(instantiation, captures, &function_shape);
    context.push_anonymous_function(function);
    Ok(Expr::function(
        value.resolve_constructed_shape(function_shape),
    ))
}

fn closure_expr(
    function: crate::plan::FunctionInstantiation,
    captures: Vec<CaptureArg>,
    shape: &FunctionShape,
) -> FunctionExpr {
    let type_ = shape.type_();
    match shape.return_shape() {
        ValueShape::Parameter(parameter) => {
            FunctionExpr::generic(crate::plan::GenericFunctionExpr::closure(
                function,
                captures,
                crate::plan::GenericFunctionType::new(shape.argument_shapes().to_vec(), *parameter),
            ))
        }
        ValueShape::Int => FunctionExpr::int(crate::plan::IntFunctionExpr::closure(
            function, captures, type_,
        )),
        ValueShape::String => FunctionExpr::string(crate::plan::StringFunctionExpr::closure(
            function, captures, type_,
        )),
        ValueShape::BitArray => FunctionExpr::bit_array(
            crate::plan::BitArrayFunctionExpr::closure(function, captures, type_),
        ),
        ValueShape::UtfCodepoint => FunctionExpr::utf_codepoint(
            crate::plan::UtfCodepointFunctionExpr::closure(function, captures, type_),
        ),
        ValueShape::Custom(return_shape) => {
            FunctionExpr::custom(crate::plan::CustomFunctionExpr::closure(
                function,
                captures,
                crate::plan::CustomFunctionType::from_shapes(
                    shape.argument_shapes().to_vec(),
                    return_shape.clone(),
                ),
            ))
        }
        ValueShape::External(return_shape) => {
            FunctionExpr::external(crate::plan::ExternalFunctionExpr::closure(
                function,
                captures,
                crate::plan::ExternalFunctionType::from_shapes(
                    shape.argument_shapes().to_vec(),
                    return_shape.clone(),
                ),
            ))
        }
        ValueShape::Float => FunctionExpr::float(crate::plan::FloatFunctionExpr::closure(
            function, captures, type_,
        )),
        ValueShape::Bool => FunctionExpr::bool(crate::plan::BoolFunctionExpr::closure(
            function, captures, type_,
        )),
        ValueShape::Nil => FunctionExpr::nil(crate::plan::NilFunctionExpr::closure(
            function, captures, type_,
        )),
        ValueShape::Tuple(return_shape) => {
            FunctionExpr::tuple(crate::plan::TupleFunctionExpr::closure(
                function,
                captures,
                type_,
                return_shape.iter().map(ValueShape::value_type).collect(),
            ))
        }
        ValueShape::List(item) => FunctionExpr::list(crate::plan::ListFunctionExpr::closure(
            function,
            captures,
            item.value_type(),
        )),
        ValueShape::Function(return_shape) => {
            FunctionExpr::function(crate::plan::FunctionFunctionExpr::closure(
                function,
                captures,
                crate::plan::FunctionFunctionType::from_shapes(
                    shape.argument_shapes().to_vec(),
                    (**return_shape).clone(),
                ),
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{
        Expr, ExternalTypeName, ExternalValueShape, FunctionFunctionId, FunctionShape,
        FunctionType, IntExpr, IntFunctionFunctionId, IntFunctionId, IntLocalId, LocalId,
        PanicExpr, PanicSite, ParamLocal, ReturnExpr, SourceSpan, StringExpr, TupleLocalId,
        ValueShape, ValueType,
    };
    use crate::planner::dsl::{
        call_int_function_at, capture_int, capture_tuple, function, function_function_closure,
        host_call_site, int, int_arg, int_function_call_arg, int_function_closure,
        int_return_tail_call_at, let_int_function_step, let_int_step, let_tuple_step, local_int,
        local_int_function, local_tuple, module_with_anonymous, string, tuple,
        tuple_function_closure,
    };
    use crate::planner::error::{
        InvalidExpressionShapeKind, InvalidFunctionShapeReason, InvalidModuleReferenceReason,
        InvalidPipelineShapeReason, InvalidTypedAstReason, PlanError,
    };
    use crate::planner::plan_module;
    use crate::planner::support::{compile, dummy_span};
    use gleam_compiler_core::ast::{
        ArgNames, CAPTURE_VARIABLE, CallArg as GleamCallArg, Constant, FunctionLiteralKind,
        PipelineAssignmentKind, Statement, TypedArg, TypedExpr, TypedModule,
        TypedPipelineAssignment, TypedStatement,
    };
    use gleam_compiler_core::type_::ModuleValueConstructor;

    #[test]
    fn plan_non_capturing_anonymous_function() {
        let source = r#"
pub fn main() {
  let add_one = fn(value) { value + 1 }
  add_one(41)
}
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let add_one = int_function_closure(
            1,
            [LocalId::Int(IntLocalId(0))],
            Vec::<crate::plan::CaptureArg>::new(),
        );
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function_at(
                    local_int_function(0, "add_one", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(int(41))],
                    host_call_site(source, "main", "add_one(41)"),
                ),
            )
            .step(let_int_function_step(0, "add_one", add_one)),
            [],
            [
                function("<anonymous:0>", local_int(0, "value").add_int(int(1)))
                    .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_anonymous_function_discard_argument() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  fn(_: Int) { 1 }
  42
}
"#,
        ))
        .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function("main", int(42)).evaluate(int_function_closure(
                1,
                [LocalId::Int(IntLocalId(0))],
                Vec::<crate::plan::CaptureArg>::new(),
            )),
            [],
            [function("<anonymous:0>", int(1)).discard_int_param(0)],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_anonymous_function_referencing_top_level_function() {
        let source = r#"
fn add_one(value: Int) {
  value + 1
}

pub fn main() {
  let wrapped = fn(value) { add_one(value) }
  wrapped(41)
}
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let wrapped = int_function_closure(
            2,
            [LocalId::Int(IntLocalId(0))],
            Vec::<crate::plan::CaptureArg>::new(),
        );
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function_at(
                    local_int_function(0, "wrapped", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(int(41))],
                    host_call_site(source, "main", "wrapped(41)"),
                ),
            )
            .step(let_int_function_step(0, "wrapped", wrapped)),
            [function("add_one", local_int(0, "value").add_int(int(1))).param_int(0, "value")],
            [function(
                "<anonymous:0>",
                int_return_tail_call_at(
                    1,
                    [int_arg(local_int(0, "value"))],
                    host_call_site(source, "<anonymous:0>", "add_one(value)"),
                ),
            )
            .param_int(0, "value")],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_main_returning_anonymous_function() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  fn(value) { value + 1 }
}
"#,
        ))
        .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                int_function_closure(
                    1,
                    [LocalId::Int(IntLocalId(0))],
                    Vec::<crate::plan::CaptureArg>::new(),
                ),
            ),
            [],
            [
                function("<anonymous:0>", local_int(0, "value").add_int(int(1)))
                    .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_nested_anonymous_function_reserves_outer_name_before_body() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  fn() { fn(value) { value + 1 } }
}
"#,
        ))
        .expect("source should plan");
        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                function_function_closure(
                    FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                    Vec::<ParamLocal>::new(),
                    Vec::<crate::plan::CaptureArg>::new(),
                    returned_function_type.clone(),
                ),
            ),
            [],
            [
                function(
                    "<anonymous:0>",
                    int_function_closure(
                        2,
                        [LocalId::Int(IntLocalId(0))],
                        Vec::<crate::plan::CaptureArg>::new(),
                    ),
                ),
                function("<anonymous:1>", local_int(0, "value").add_int(int(1)))
                    .param_int(0, "value"),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_nested_anonymous_function_panic_site_uses_reserved_outer_name() {
        let actual = plan_module(compile(
            r#"
pub fn main() -> Int {
  let outer = fn() {
    fn() { 1 }
    panic as "outer"
  }
  outer()
}
"#,
        ))
        .expect("source should plan");
        let anonymous_functions = actual.anonymous_functions();
        let outer_function = &anonymous_functions[0];
        let inner_function = &anonymous_functions[1];

        assert_eq!(inner_function.name(), "<anonymous:1>");
        assert_eq!(outer_function.name(), "<anonymous:0>");

        assert_eq!(
            outer_function.return_(),
            &ReturnExpr::int(
                IntFunctionId(1),
                IntExpr::panic(PanicExpr::panic_at(
                    Some(StringExpr::value("outer".into())),
                    PanicSite::new(
                        "main".into(),
                        "<anonymous:0>".into(),
                        SourceSpan::new(64, 80)
                    ),
                )),
            ),
        );
    }

    #[test]
    fn plan_capturing_anonymous_function() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let value = 1
  fn() { value }
  1
}
"#,
        ))
        .expect("source should plan");
        let expected = module_with_anonymous(
            "main",
            function("main", int(1))
                .step(let_int_step(0, "value", int(1)))
                .evaluate(int_function_closure(
                    1,
                    Vec::<LocalId>::new(),
                    [capture_int(0)],
                )),
            [],
            [function("<anonymous:0>", local_int(0, "value")).capture(
                crate::plan::ParamSlot::from_local(ParamLocal::int(IntLocalId(0))),
            )],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_nested_capturing_anonymous_function() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let value = 1
  fn() { fn() { value } }
  1
}
"#,
        ))
        .expect("source should plan");
        let returned_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let expected = module_with_anonymous(
            "main",
            function("main", int(1))
                .step(let_int_step(0, "value", int(1)))
                .evaluate(function_function_closure(
                    FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                    Vec::<ParamLocal>::new(),
                    [capture_int(0)],
                    returned_function_type.clone(),
                )),
            [],
            [
                function(
                    "<anonymous:0>",
                    int_function_closure(2, Vec::<LocalId>::new(), [capture_int(0)]),
                )
                .capture(crate::plan::ParamSlot::from_local(ParamLocal::int(
                    IntLocalId(0),
                ))),
                function("<anonymous:1>", local_int(0, "value")).capture(
                    crate::plan::ParamSlot::from_local(ParamLocal::int(IntLocalId(0))),
                ),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn closure_expr_preserves_tuple_return_family() {
        let return_type = vec![ValueType::Int];
        let shape = FunctionShape::from_function_type(FunctionType::new(
            Vec::new(),
            ValueType::Tuple(return_type.clone()),
        ));
        let expression = super::closure_expr(
            crate::plan::monomorphic_function_instantiation(0, shape.clone()),
            Vec::new(),
            &shape,
        );

        assert_eq!(
            expression.type_(),
            FunctionType::new(Vec::new(), ValueType::Tuple(return_type)),
        );
    }

    #[test]
    fn plan_tuple_returning_capturing_anonymous_function() {
        let actual = plan_module(compile(
            r#"
pub fn main() {
  let pair = #(1, "one")
  fn() { pair }
}
"#,
        ))
        .expect("source should plan");
        let pair_type = [ValueType::Int, ValueType::String];
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                tuple_function_closure(
                    1,
                    Vec::<LocalId>::new(),
                    [capture_tuple(0, pair_type.clone())],
                    pair_type.clone(),
                ),
            )
            .step(let_tuple_step(
                0,
                "pair",
                tuple([Expr::from(int(1)), Expr::from(string("one"))]),
            )),
            [],
            [
                function("<anonymous:0>", local_tuple(0, "pair", pair_type.clone())).capture(
                    crate::plan::ParamSlot::from_local(ParamLocal::tuple(
                        TupleLocalId(0),
                        pair_type.to_vec(),
                    )),
                ),
            ],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_capture_literal() {
        let source = r#"
fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  let add_one = add(1, _)
  add_one(41)
}
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let add_one = int_function_closure(
            2,
            [LocalId::Int(IntLocalId(0))],
            Vec::<crate::plan::CaptureArg>::new(),
        );
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function_at(
                    local_int_function(0, "add_one", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(int(41))],
                    host_call_site(source, "main", "add_one(41)"),
                ),
            )
            .step(let_int_function_step(0, "add_one", add_one)),
            [
                function("add", local_int(0, "left").add_int(local_int(1, "right")))
                    .param_int(0, "left")
                    .param_int(1, "right"),
            ],
            [function(
                "<anonymous:0>",
                int_return_tail_call_at(
                    1,
                    [int_arg(int(1)), int_arg(local_int(0, CAPTURE_VARIABLE))],
                    host_call_site(source, "<anonymous:0>", "add(1, _)"),
                ),
            )
            .param_int(0, CAPTURE_VARIABLE)],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_capture_labelled_argument() {
        let source = r#"
fn add(to base: Int, value amount: Int) {
  base + amount
}

pub fn main() {
  let add_one = add(to: 1, value: _)
  add_one(41)
}
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let add_one = int_function_closure(
            2,
            [LocalId::Int(IntLocalId(0))],
            Vec::<crate::plan::CaptureArg>::new(),
        );
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function_at(
                    local_int_function(0, "add_one", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(int(41))],
                    host_call_site(source, "main", "add_one(41)"),
                ),
            )
            .step(let_int_function_step(0, "add_one", add_one)),
            [
                function("add", local_int(0, "base").add_int(local_int(1, "amount")))
                    .param_int(0, "base")
                    .param_int(1, "amount"),
            ],
            [function(
                "<anonymous:0>",
                int_return_tail_call_at(
                    1,
                    [int_arg(int(1)), int_arg(local_int(0, CAPTURE_VARIABLE))],
                    host_call_site(source, "<anonymous:0>", "add(to: 1, value: _)"),
                ),
            )
            .param_int(0, CAPTURE_VARIABLE)],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn plan_function_capture_literal_with_closure_capture() {
        let source = r#"
fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  let base = 1
  let add_base = add(base, _)
  add_base(41)
}
"#;
        let actual = plan_module(compile(source)).expect("source should plan");
        let add_base = int_function_closure(2, [LocalId::Int(IntLocalId(0))], [capture_int(0)]);
        let expected = module_with_anonymous(
            "main",
            function(
                "main",
                call_int_function_at(
                    local_int_function(0, "add_base", [LocalId::Int(IntLocalId(0))]),
                    [int_function_call_arg(int(41))],
                    host_call_site(source, "main", "add_base(41)"),
                ),
            )
            .step(let_int_step(0, "base", int(1)))
            .step(let_int_function_step(0, "add_base", add_base)),
            [
                function("add", local_int(0, "left").add_int(local_int(1, "right")))
                    .param_int(0, "left")
                    .param_int(1, "right"),
            ],
            [function(
                "<anonymous:0>",
                int_return_tail_call_at(
                    1,
                    [
                        int_arg(local_int(1, "base")),
                        int_arg(local_int(0, CAPTURE_VARIABLE)),
                    ],
                    host_call_site(source, "<anonymous:0>", "add(base, _)"),
                ),
            )
            .param_int(0, CAPTURE_VARIABLE)
            .capture(crate::plan::ParamSlot::from_local(ParamLocal::int(
                IntLocalId(1),
            )))],
        );

        assert_eq!(actual, expected);
    }

    #[test]
    fn reject_margin_non_function_literal_type() {
        for (type_, actual) in [
            (gleam_compiler_core::type_::int(), ValueType::Int),
            (gleam_compiler_core::type_::string(), ValueType::String),
            (gleam_compiler_core::type_::bit_array(), ValueType::BitArray),
            (utf_codepoint_type(), ValueType::UtfCodepoint),
            (gleam_compiler_core::type_::float(), ValueType::Float),
            (gleam_compiler_core::type_::bool(), ValueType::Bool),
            (gleam_compiler_core::type_::nil(), ValueType::Nil),
            (
                gleam_compiler_core::type_::tuple(vec![gleam_compiler_core::type_::int()]),
                ValueType::Tuple(vec![ValueType::Int]),
            ),
        ] {
            let mut module = anonymous_function_module();
            let (expression_type, _, _) = anonymous_function_expression_mut(&mut module);
            *expression_type = type_;

            assert_eq!(
                plan_module(module),
                Err(PlanError::InvalidTypedAst {
                    reason: InvalidTypedAstReason::FunctionShape {
                        name: "<anonymous:0>".into(),
                        reason: InvalidFunctionShapeReason::ExpressionType { actual },
                    },
                }),
            );
        }

        let external = ExternalValueShape::new(
            ExternalTypeName::new("geam".into(), "main".into(), "Token".into()),
            Vec::new(),
        );
        assert_eq!(
            super::anonymous_function_shape(
                "<anonymous:0>".into(),
                ValueShape::External(external.clone())
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous:0>".into(),
                    reason: InvalidFunctionShapeReason::ExpressionType {
                        actual: ValueType::External(external.type_().clone()),
                    },
                },
            }),
        );
    }

    fn utf_codepoint_type() -> std::sync::Arc<gleam_compiler_core::type_::Type> {
        let module = compile(
            r#"
fn identity(value: UtfCodepoint) -> UtfCodepoint { value }
pub fn main() { 0 }
"#,
        );
        module.definitions.functions[0].arguments[0].type_.clone()
    }

    #[test]
    fn reject_margin_anonymous_function_argument_type_mismatch() {
        let mut module = anonymous_function_module();
        let (_, arguments, _) = anonymous_function_expression_mut(&mut module);
        arguments[0].type_ = gleam_compiler_core::type_::string();

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous:0>".into(),
                    reason: InvalidFunctionShapeReason::ArgumentTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_anonymous_function_param_shape_error_propagates() {
        let mut module = anonymous_function_module();
        let (_, arguments, _) = anonymous_function_expression_mut(&mut module);
        arguments[0] = labelled_arg();

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous:0>".into(),
                    reason: InvalidFunctionShapeReason::LabelledArgument,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_anonymous_function_return_type_mismatch() {
        let mut module = anonymous_function_module();
        let (type_, _, _) = anonymous_function_expression_mut(&mut module);
        *type_ = gleam_compiler_core::type_::fn_(
            vec![gleam_compiler_core::type_::int()],
            gleam_compiler_core::type_::string(),
        );

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous:0>".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_non_function_literal_types() {
        let mut module = anonymous_function_module();
        let (type_, _, _) = anonymous_function_expression_mut(&mut module);
        *type_ = gleam_compiler_core::type_::list(gleam_compiler_core::type_::int());

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous:0>".into(),
                    reason: InvalidFunctionShapeReason::ExpressionType {
                        actual: ValueType::List(Box::new(ValueType::Int)),
                    },
                },
            }),
        );

        let mut invalid_shape = anonymous_function_module();
        let (type_, _, _) = anonymous_function_expression_mut(&mut invalid_shape);
        *type_ = gleam_compiler_core::type_::result(
            gleam_compiler_core::type_::int(),
            gleam_compiler_core::type_::nil(),
        );

        assert_eq!(
            plan_module(invalid_shape),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous:0>".into(),
                    reason: InvalidFunctionShapeReason::ExpressionType {
                        actual: ValueType::Custom(crate::plan::CustomType::new(
                            crate::plan::CustomTypeName::new(
                                "".into(),
                                "gleam".into(),
                                "Result".into(),
                            ),
                            vec![ValueType::Int, ValueType::Nil],
                        )),
                    },
                },
            }),
        );

        let mut invalid_type = anonymous_function_module();
        let (type_, _, _) = anonymous_function_expression_mut(&mut invalid_type);
        *type_ = gleam_compiler_core::type_::generic_var(99);

        assert_eq!(
            plan_module(invalid_type),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "<anonymous:0>".into(),
                    reason: InvalidFunctionShapeReason::ExpressionType {
                        actual: ValueType::Parameter(crate::plan::TypeParameterId(0)),
                    },
                },
            }),
        );
    }

    #[test]
    fn reject_margin_anonymous_function_unknown_capture() {
        let mut module = compile(
            r#"
pub fn main() {
  let value = 1
  fn() { value }
  1
}
"#,
        );
        module.definitions.functions[0].body.remove(0);

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::UnknownLocal {
                    name: "value".into(),
                },
            }),
        );
    }

    #[test]
    fn reject_margin_anonymous_function_module_select_body() {
        let mut module = anonymous_function_module();
        let body = anonymous_function_body_mut(&mut module);
        body[0] = Statement::Expression(TypedExpr::ModuleSelect {
            location: dummy_span(),
            field_start: 0,
            type_: gleam_compiler_core::type_::int(),
            label: "answer".into(),
            module_name: "other".into(),
            module_alias: "other".into(),
            constructor: ModuleValueConstructor::Constant {
                literal: Constant::Int {
                    location: dummy_span(),
                    value: "1".into(),
                    int_value: num_bigint::BigInt::from(1),
                },
                location: dummy_span(),
                documentation: None,
            },
        });

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ModuleReference {
                    module: "other".into(),
                    name: "answer".into(),
                    reason: InvalidModuleReferenceReason::UnlinkedModule,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_anonymous_function_pipeline_intermediate_shape() {
        let mut module = anonymous_function_module();
        let body = anonymous_function_body_mut(&mut module);
        body[0] = Statement::Expression(TypedExpr::Pipeline {
            location: dummy_span(),
            first_value: TypedPipelineAssignment {
                location: dummy_span(),
                name: "pipe_0".into(),
                value: Box::new(super::super::typed_int_expr(1)),
            },
            assignments: vec![(
                TypedPipelineAssignment {
                    location: dummy_span(),
                    name: "pipe_1".into(),
                    value: Box::new(super::super::typed_int_expr(2)),
                },
                PipelineAssignmentKind::FirstArgument {
                    second_argument: None,
                },
            )],
            finally: Box::new(super::super::typed_int_expr(3)),
            finally_kind: PipelineAssignmentKind::FirstArgument {
                second_argument: None,
            },
        });

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::PipelineShape {
                    reason: InvalidPipelineShapeReason::NonCallStep,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_use_function_literal_expression_kind() {
        let mut module = anonymous_function_module();
        let (_, _, kind) = anonymous_function_expression_mut(&mut module);
        *kind = FunctionLiteralKind::Use {
            location: dummy_span(),
        };

        assert_eq!(
            plan_module(module),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::ExpressionShape {
                    kind: InvalidExpressionShapeKind::FunctionLiteralKind,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_function_capture_literal_argument_shape() {
        for mutate in [
            |arguments: &mut Vec<TypedArg>, _: &mut vec1::Vec1<TypedStatement>| {
                arguments.push(arguments[0].clone());
            },
            |arguments: &mut Vec<TypedArg>, _: &mut vec1::Vec1<TypedStatement>| {
                arguments[0].names = ArgNames::Named {
                    name: "other".into(),
                    location: dummy_span(),
                };
            },
        ] {
            let mut module = function_capture_literal_module();
            let (arguments, body, _) = function_capture_literal_expression_mut(&mut module);
            mutate(arguments, body);

            assert_eq!(
                plan_module(module),
                Err(invalid_function_capture_literal_shape()),
            );
        }
    }

    #[test]
    fn reject_margin_function_capture_literal_body_shape() {
        for mutate in [
            |_: &mut Vec<TypedArg>, body: &mut vec1::Vec1<TypedStatement>| {
                body[0] = Statement::Expression(super::super::typed_int_expr(1));
            },
            |_: &mut Vec<TypedArg>, body: &mut vec1::Vec1<TypedStatement>| {
                let arguments = function_capture_literal_body_call_args_mut(body);
                let capture_index = capture_argument_index(arguments);
                arguments[capture_index].value = super::super::typed_int_expr(1);
            },
            |_: &mut Vec<TypedArg>, body: &mut vec1::Vec1<TypedStatement>| {
                let arguments = function_capture_literal_body_call_args_mut(body);
                let capture = arguments[capture_argument_index(arguments)].clone();
                arguments.push(capture);
            },
            |_: &mut Vec<TypedArg>, body: &mut vec1::Vec1<TypedStatement>| {
                let arguments = function_capture_literal_body_call_args_mut(body);
                let capture_index = capture_argument_index(arguments);
                arguments[capture_index].value = TypedExpr::Var {
                    location: dummy_span(),
                    name: CAPTURE_VARIABLE.into(),
                    constructor: gleam_compiler_core::type_::ValueConstructor {
                        publicity: gleam_compiler_core::ast::Publicity::Private,
                        deprecation: gleam_compiler_core::type_::Deprecation::NotDeprecated,
                        type_: gleam_compiler_core::type_::int(),
                        variant: gleam_compiler_core::type_::ValueConstructorVariant::Record {
                            name: "Capture".into(),
                            arity: 1,
                            field_map: None,
                            location: dummy_span(),
                            module: "main".into(),
                            variants_count: 1,
                            variant_index: 0,
                            documentation: None,
                        },
                    },
                };
            },
        ] {
            let mut module = function_capture_literal_module();
            let (arguments, body, _) = function_capture_literal_expression_mut(&mut module);
            mutate(arguments, body);

            assert_eq!(
                plan_module(module),
                Err(invalid_function_capture_literal_shape()),
            );
        }
    }

    #[test]
    #[should_panic(expected = "expected function capture literal expression statement")]
    fn function_capture_literal_expression_mut_panics_on_non_function_statement() {
        let mut module = compile(r#"pub fn main() { 1 }"#);

        let _ = function_capture_literal_expression_mut(&mut module);
    }

    #[test]
    #[should_panic(expected = "expected function capture literal call body")]
    fn function_capture_literal_body_call_args_mut_panics_on_non_call_body() {
        let mut module = function_capture_literal_module();
        let (_, body, _) = function_capture_literal_expression_mut(&mut module);
        body[0] = Statement::Expression(super::super::typed_int_expr(1));

        let _ = function_capture_literal_body_call_args_mut(body);
    }

    #[test]
    #[should_panic(expected = "expected anonymous function expression statement")]
    fn anonymous_function_expression_mut_panics_on_non_function_statement() {
        let mut module = compile(r#"pub fn main() { 1 }"#);

        let _ = anonymous_function_expression_mut(&mut module);
    }

    #[test]
    #[should_panic(expected = "expected anonymous function expression statement")]
    fn anonymous_function_body_mut_panics_on_non_function_statement() {
        let mut module = compile(r#"pub fn main() { 1 }"#);

        let _ = anonymous_function_body_mut(&mut module);
    }

    fn anonymous_function_module() -> TypedModule {
        compile("pub fn main() {\n  fn(value) { value + 1 }\n  1\n}\n")
    }

    fn function_capture_literal_module() -> TypedModule {
        compile(
            r#"
fn add(left: Int, right: Int) {
  left + right
}

pub fn main() {
  add(1, _)
}
"#,
        )
    }

    fn anonymous_function_body_mut(module: &mut TypedModule) -> &mut vec1::Vec1<TypedStatement> {
        let Statement::Expression(TypedExpr::Fn { body, .. }) =
            &mut module.definitions.functions[0].body[0]
        else {
            panic!("expected anonymous function expression statement");
        };

        body
    }

    fn anonymous_function_expression_mut(
        module: &mut TypedModule,
    ) -> (
        &mut std::sync::Arc<gleam_compiler_core::type_::Type>,
        &mut Vec<TypedArg>,
        &mut FunctionLiteralKind,
    ) {
        let Statement::Expression(TypedExpr::Fn {
            type_,
            arguments,
            kind,
            ..
        }) = &mut module.definitions.functions[0].body[0]
        else {
            panic!("expected anonymous function expression statement");
        };

        (type_, arguments, kind)
    }

    fn function_capture_literal_expression_mut(
        module: &mut TypedModule,
    ) -> (
        &mut Vec<TypedArg>,
        &mut vec1::Vec1<TypedStatement>,
        &mut FunctionLiteralKind,
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
            .expect("expected main function");

        let Statement::Expression(TypedExpr::Fn {
            arguments,
            body,
            kind,
            ..
        }) = &mut main.body[0]
        else {
            panic!("expected function capture literal expression statement");
        };

        (arguments, body, kind)
    }

    fn function_capture_literal_body_call_args_mut(
        body: &mut vec1::Vec1<TypedStatement>,
    ) -> &mut Vec<GleamCallArg<TypedExpr>> {
        let (_, arguments) = function_capture_literal_body_call_parts_mut(body);

        arguments
    }

    fn function_capture_literal_body_call_parts_mut(
        body: &mut vec1::Vec1<TypedStatement>,
    ) -> (&mut Box<TypedExpr>, &mut Vec<GleamCallArg<TypedExpr>>) {
        assert_eq!(
            body.len(),
            1,
            "expected single capture literal body statement"
        );
        let Statement::Expression(TypedExpr::Call { fun, arguments, .. }) = &mut body[0] else {
            panic!("expected function capture literal call body");
        };

        (fun, arguments)
    }

    fn capture_argument_index(arguments: &[GleamCallArg<TypedExpr>]) -> usize {
        arguments
            .iter()
            .position(|argument: &GleamCallArg<TypedExpr>| argument.is_capture_hole())
            .expect("expected capture literal argument")
    }

    fn invalid_function_capture_literal_shape() -> PlanError {
        PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::ExpressionShape {
                kind: InvalidExpressionShapeKind::FunctionCaptureLiteral,
            },
        }
    }

    fn labelled_arg() -> TypedArg {
        TypedArg {
            names: ArgNames::NamedLabelled {
                label: "label".into(),
                label_location: dummy_span(),
                name: "value".into(),
                name_location: dummy_span(),
            },
            location: dummy_span(),
            annotation: None,
            type_: gleam_compiler_core::type_::int(),
        }
    }
}
