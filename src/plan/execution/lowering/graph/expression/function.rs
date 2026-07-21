mod bit_array;
mod bool;
mod custom;
mod float;
mod generic;
mod int;
mod list;
mod never;
mod nil;
mod returning_function;
mod string;
mod tuple;
mod utf_codepoint;

pub(in crate::plan::execution::lowering) use bit_array::bit_array_function_expr;
pub(in crate::plan::execution::lowering) use bool::bool_function_expr;
pub(in crate::plan::execution::lowering) use custom::{
    custom_function_expr, custom_function_expr_kind, generic_custom_function_expr,
};
pub(in crate::plan::execution::lowering) use float::float_function_expr;
pub(in crate::plan::execution::lowering) use generic::executable_function_expr as generic_executable_function_expr;
pub(in crate::plan::execution::lowering) use generic::{
    generic_bit_array_function_expr, generic_bool_function_expr, generic_float_function_expr,
    generic_int_function_expr, generic_nil_function_expr, generic_string_function_expr,
    generic_tuple_function_expr, generic_utf_codepoint_function_expr,
    symbolic_bit_array_function_expr, symbolic_bool_function_expr,
    symbolic_custom_function_expr_kind, symbolic_float_function_expr,
    symbolic_function_function_expr_kind, symbolic_generic_function_expr,
    symbolic_int_function_expr, symbolic_list_function_expr, symbolic_nil_function_expr,
    symbolic_string_function_expr, symbolic_tuple_function_expr,
    symbolic_utf_codepoint_function_expr,
};
pub(in crate::plan::execution::lowering) use int::int_function_expr;
pub(in crate::plan::execution::lowering) use list::{
    generic_list_function_expr, list_function_expr,
};
pub(in crate::plan::execution::lowering) use never::generic_function_expr as generic_never_function_expr;
pub(in crate::plan::execution::lowering) use never::{
    custom_function_expr as custom_never_function_expr,
    custom_function_kind as custom_never_function_expr_kind,
    tuple_function_expr as tuple_never_function_expr,
};
pub(in crate::plan::execution::lowering) use nil::nil_function_expr;
pub(in crate::plan::execution::lowering) use returning_function::{
    function_function_expr, function_function_expr_kind, generic_function_function_expr,
};
pub(in crate::plan::execution::lowering) use string::string_function_expr;
pub(in crate::plan::execution::lowering) use tuple::tuple_function_expr;
pub(in crate::plan::execution::lowering) use utf_codepoint::utf_codepoint_function_expr;

use super::{capture_args, panic_expr};
use crate::plan::execution::graph::FunctionTarget;
use crate::plan::execution::lowering::graph::{DraftCursor, DraftFlow, DraftFunction, DraftGraph};
use crate::plan::execution::lowering::specialization::{
    FunctionRepresentation, Representability, SpecializedFunctionShape,
};
use crate::plan::module;

pub(in crate::plan::execution::lowering) fn function_expr(
    expression: &module::FunctionExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunction>> {
    let shape = context.concrete_function_shape(expression.shape());
    match context.function_representation(&shape) {
        FunctionRepresentation::Symbolic => {
            generic::symbolic_function_expr(expression, &shape, cursor, graph, context)
        }
        FunctionRepresentation::Never(proof) => {
            never::function_expr(expression, &proof, cursor, graph, context)
        }
        FunctionRepresentation::Executable(return_) => match expression.kind() {
            module::FunctionExprKind::Generic(expression) => generic::executable_function_expr(
                expression, &shape, &return_, cursor, graph, context,
            ),
            module::FunctionExprKind::Int(expression) => {
                int_function_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(|value| value.value().clone()))
            }
            module::FunctionExprKind::Float(expression) => {
                float_function_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(|value| value.value().clone()))
            }
            module::FunctionExprKind::String(expression) => {
                string_function_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(|value| value.value().clone()))
            }
            module::FunctionExprKind::BitArray(expression) => {
                bit_array_function_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(|value| value.value().clone()))
            }
            module::FunctionExprKind::UtfCodepoint(expression) => {
                utf_codepoint_function_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(|value| value.value().clone()))
            }
            module::FunctionExprKind::Custom(expression) => {
                custom_function_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(|value| value.value().clone()))
            }
            module::FunctionExprKind::Bool(expression) => {
                bool_function_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(|value| value.value().clone()))
            }
            module::FunctionExprKind::Nil(expression) => {
                nil_function_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(|value| value.value().clone()))
            }
            module::FunctionExprKind::Tuple(expression) => {
                tuple_function_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(|value| value.value().clone()))
            }
            module::FunctionExprKind::List(expression) => {
                list_function_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(|value| value.value().clone()))
            }
            module::FunctionExprKind::Function(expression) => {
                function_function_expr(expression, cursor, graph, context)
                    .map(|flow| flow.map(|value| value.value().clone()))
            }
        },
    }
}

pub(in crate::plan::execution::lowering) fn generic_function_expr(
    expression: &module::GenericFunctionExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunction>> {
    let shape = context.concrete_function_shape(&expression.shape());
    match context.function_representation(&shape) {
        FunctionRepresentation::Symbolic => {
            generic::symbolic_generic_function_expr(expression, &shape, cursor, graph, context)
        }
        FunctionRepresentation::Never(_) => {
            never::generic_function_expr(expression, &shape, cursor, graph, context)
                .map(|flow| flow.map(|value| value.value().clone()))
        }
        FunctionRepresentation::Executable(return_) => {
            generic::executable_function_expr(expression, &shape, &return_, cursor, graph, context)
        }
    }
}

pub(in crate::plan::execution::lowering) fn evaluated_function_function_expr(
    expression: &module::FunctionFunctionExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<()>> {
    let shape = context.concrete_function_shape(&crate::plan::FunctionShape::from_function_type(
        expression.function_function_type().to_function_type(),
    ));
    generic::symbolic_function_function_expr_kind(expression.kind(), &shape, cursor, graph, context)
        .map(|flow| flow.map(|_| ()))
}

pub(in crate::plan::execution::lowering) fn evaluated_generic_function_expr(
    expression: &module::GenericFunctionExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<()>> {
    let shape = context.concrete_function_shape(&expression.shape());
    generic::symbolic_generic_function_expr(expression, &shape, cursor, graph, context)
        .map(|flow| flow.map(|_| ()))
}

fn closure(
    function: &module::FunctionInstantiation,
    captures: &[module::CaptureArg],
    shape: SpecializedFunctionShape,
    target: FunctionTarget,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunction>> {
    use super::super::instruction::DraftFunctionInstruction as I;

    let captures = capture_args(function, captures, &cursor, context);
    let mut cursor = cursor;
    let value = graph.function_instruction(&mut cursor, shape, I::Closure { target, captures });
    Representability::Inhabited(DraftFlow::value(cursor, value))
}

fn reference(
    shape: SpecializedFunctionShape,
    target: FunctionTarget,
    mut cursor: DraftCursor,
    graph: &mut DraftGraph,
) -> DraftFlow<DraftFunction> {
    use super::super::instruction::DraftFunctionInstruction as I;

    let value = graph.function_instruction(&mut cursor, shape, I::Reference(target));
    DraftFlow::value(cursor, value)
}

fn source_stop(
    value: &module::PanicExpr,
    cursor: DraftCursor,
    graph: &mut DraftGraph,
    context: &mut super::super::super::LoweringContext,
) -> Representability<DraftFlow<DraftFunction>> {
    panic_expr(value, cursor, graph, context).map(|_| DraftFlow::Diverged)
}

#[cfg(test)]
mod tests {
    use crate::Value;
    use crate::plan::execution::lowering::graph::{DraftFlow, DraftGraphBuilder, DraftValueRef};
    use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};
    use crate::plan::{
        BitArrayFunctionExpr, BoolFunctionExpr, CustomConstructorDefinition, CustomFunctionExpr,
        CustomFunctionType, CustomType, CustomTypeDefinition, CustomTypeName, CustomTypePublicity,
        FloatFunctionExpr, FunctionExpr, FunctionFunctionExpr, FunctionFunctionType, FunctionShape,
        FunctionTemplateId, FunctionType, IntFunctionExpr, IntFunctionReference, ListExpr,
        ListFunctionExpr, NilFunctionExpr, PanicExpr, PanicSite, StringFunctionExpr,
        TupleFunctionExpr, UtfCodepointFunctionExpr, ValueShape, ValueType,
    };

    #[derive(Debug, PartialEq, Eq)]
    enum FlowOutcome {
        Uninhabited,
        Diverged,
        Value,
    }

    fn flow_outcome<T>(flow: Representability<DraftFlow<T>>) -> FlowOutcome {
        match flow {
            Representability::Uninhabited => FlowOutcome::Uninhabited,
            Representability::Inhabited(DraftFlow::Diverged) => FlowOutcome::Diverged,
            Representability::Inhabited(DraftFlow::Value { .. }) => FlowOutcome::Value,
        }
    }

    struct FunctionFamily {
        return_type: &'static str,
        value: &'static str,
        assertion: &'static str,
    }

    const FUNCTION_FAMILIES: &[FunctionFamily] = &[
        FunctionFamily {
            return_type: "Int",
            value: "1",
            assertion: "selected() == 1",
        },
        FunctionFamily {
            return_type: "Float",
            value: "1.5",
            assertion: "selected() == 1.5",
        },
        FunctionFamily {
            return_type: "String",
            value: "\"one\"",
            assertion: "selected() == \"one\"",
        },
        FunctionFamily {
            return_type: "BitArray",
            value: "<<1>>",
            assertion: "selected() == <<1>>",
        },
        FunctionFamily {
            return_type: "UtfCodepoint",
            value: "codepoint()",
            assertion: "selected() == codepoint()",
        },
        FunctionFamily {
            return_type: "Marker",
            value: "Marker(1)",
            assertion: "selected() == Marker(1)",
        },
        FunctionFamily {
            return_type: "Bool",
            value: "True",
            assertion: "selected() == True",
        },
        FunctionFamily {
            return_type: "Nil",
            value: "Nil",
            assertion: "selected() == Nil",
        },
        FunctionFamily {
            return_type: "#(Int)",
            value: "#(1)",
            assertion: "selected() == #(1)",
        },
        FunctionFamily {
            return_type: "List(Int)",
            value: "[1]",
            assertion: "selected() == [1]",
        },
        FunctionFamily {
            return_type: "fn() -> Int",
            value: "fn() { 1 }",
            assertion: "selected()() == 1",
        },
    ];

    #[test]
    fn every_callable_family_lowers_each_function_expression_owner() {
        for family in FUNCTION_FAMILIES {
            let expressions = [
                "target".to_owned(),
                format!("fn() {{ {} }}", family.value),
                "{ let local = target local }".to_owned(),
                "provider()".to_owned(),
                "{ let callable = provider callable() }".to_owned(),
                "#(target).0".to_owned(),
                "Holder(selected: target).selected".to_owned(),
                "case [target] { [selected] -> selected _ -> target }".to_owned(),
                "case True { True -> target False -> target }".to_owned(),
                "case 1 { 1 -> target _ -> target }".to_owned(),
                "case \"selected\" { \"selected\" -> target _ -> target }".to_owned(),
                "case 1.0 { 1.0 -> target _ -> target }".to_owned(),
                "{ let _ = Nil target }".to_owned(),
                "selected_constant".to_owned(),
            ];

            for expression in expressions {
                let source = source(family, &expression);
                assert_eq!(
                    crate::run_main(&execution_plan(&source)),
                    Ok(Value::Bool(true)),
                    "failed callable family {} expression {expression}",
                    family.return_type,
                );
            }
        }
    }

    #[test]
    fn every_callable_family_preserves_its_source_stop() {
        for family in FUNCTION_FAMILIES {
            let source = format!(
                r#"
pub type Marker {{ Marker(Int) }}

fn codepoint() -> UtfCodepoint {{
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}}

fn selected() -> fn() -> {return_type} {{ panic as "selected" }}

pub fn main() {{
  let selected = selected()
  {assertion}
}}
"#,
                return_type = family.return_type,
                assertion = family.assertion,
            );
            let error = crate::run_main(&execution_plan(&source)).unwrap_err();
            assert_eq!(error.to_string(), "panic: selected");
        }
    }

    #[test]
    fn every_callable_family_stops_when_an_owner_source_diverges() {
        for family in FUNCTION_FAMILIES {
            let expressions = [
                "provider(panic as \"source\")",
                "{ panic as \"source\" }(0)",
                "{ let callable = provider callable(panic as \"source\") }",
                "{ let callable = fail_provider() callable(0) }",
                "#(panic as \"source\", target).1",
                "Holder(selected: panic as \"source\").selected",
                "{ panic as \"source\" }[0]",
                "case [panic as \"source\"] { [selected] -> selected _ -> target }",
                "{ let failed: Int = panic as \"source\" let _ = failed target }",
            ];

            for expression in expressions {
                let source = diverging_source(family, expression);
                let error = crate::run_main(&execution_plan(&source)).unwrap_err();
                assert_eq!(
                    error.to_string(),
                    "panic: source",
                    "failed callable family {} expression {expression}",
                    family.return_type,
                );
            }
        }
    }

    #[test]
    fn symbolic_callable_families_stop_when_an_owner_source_diverges() {
        let targets = [
            "fn target(_value) { 1 }",
            "fn target(_value) { 1.5 }",
            "fn target(_value) { \"value\" }",
            "fn target(_value) { <<1>> }",
            "fn target(_value) { codepoint() }",
            "fn target(_value) { True }",
            "fn target(_value) { Nil }",
            "fn target(_value) { #(1) }",
            "fn target(value) { Boxed(value) }",
            "fn target(value) { [value] }",
            "fn target(value) { fn() { value } }",
            "fn target(_value: Int) -> value { panic as \"target\" }",
        ];
        let expressions = [
            "provider(panic as \"source\")",
            "{ let callable = provider(0) callable(panic as \"source\") }",
            "{ let callable = fail_provider() callable(0) }",
            "#(panic as \"source\", target).1",
            "Holder(selected: panic as \"source\").selected",
            "[panic as \"source\"][0]",
            "{ let failed: Int = panic as \"source\" let _ = failed target }",
        ];

        for target in targets {
            for expression in expressions {
                let source = symbolic_diverging_source(target, expression);
                let error = crate::run_main(&execution_plan(&source)).unwrap_err();
                assert_eq!(
                    error.to_string(),
                    "panic: source",
                    "failed symbolic target `{target}` expression `{expression}`",
                );
            }
        }
    }

    #[test]
    fn planner_generated_list_projections_stop_before_every_callable_family_output() {
        let custom_name = CustomTypeName::new("geam".into(), "main".into(), "Marker".into());
        let custom_type = CustomType::new(custom_name.clone(), Vec::new());
        let custom_definition = CustomTypeDefinition::new(
            custom_name,
            CustomTypePublicity::Private,
            false,
            Vec::new(),
            vec![CustomConstructorDefinition::new(
                "Marker".into(),
                0,
                Vec::new(),
            )],
        );
        let int_type = FunctionType::new(Vec::new(), ValueType::Int);
        let float_type = FunctionType::new(Vec::new(), ValueType::Float);
        let string_type = FunctionType::new(Vec::new(), ValueType::String);
        let bit_array_type = FunctionType::new(Vec::new(), ValueType::BitArray);
        let utf_codepoint_type = FunctionType::new(Vec::new(), ValueType::UtfCodepoint);
        let custom_function_type = CustomFunctionType::new(Vec::new(), custom_type);
        let bool_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let nil_type = FunctionType::new(Vec::new(), ValueType::Nil);
        let tuple_type = FunctionType::new(Vec::new(), ValueType::Tuple(vec![ValueType::Int]));
        let list_type = FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int)));
        let function_function_type = FunctionFunctionType::new(Vec::new(), int_type.clone());
        let panic = || PanicExpr::panic_at(None, PanicSite::unknown());
        let expressions = vec![
            FunctionExpr::int(IntFunctionExpr::list_index(
                ListExpr::panic(panic(), ValueType::Function(Box::new(int_type.clone())))
                    .into_function()
                    .expect("an Int-function item should create a function list"),
                0,
                int_type.clone(),
            )),
            FunctionExpr::float(FloatFunctionExpr::list_index(
                ListExpr::panic(panic(), ValueType::Function(Box::new(float_type.clone())))
                    .into_function()
                    .expect("a Float-function item should create a function list"),
                0,
                float_type,
            )),
            FunctionExpr::string(StringFunctionExpr::list_index(
                ListExpr::panic(panic(), ValueType::Function(Box::new(string_type.clone())))
                    .into_function()
                    .expect("a String-function item should create a function list"),
                0,
                string_type,
            )),
            FunctionExpr::bit_array(BitArrayFunctionExpr::list_index(
                ListExpr::panic(
                    panic(),
                    ValueType::Function(Box::new(bit_array_type.clone())),
                )
                .into_function()
                .expect("a BitArray-function item should create a function list"),
                0,
                bit_array_type,
            )),
            FunctionExpr::utf_codepoint(UtfCodepointFunctionExpr::list_index(
                ListExpr::panic(
                    panic(),
                    ValueType::Function(Box::new(utf_codepoint_type.clone())),
                )
                .into_function()
                .expect("a UtfCodepoint-function item should create a function list"),
                0,
                utf_codepoint_type,
            )),
            FunctionExpr::custom(CustomFunctionExpr::list_index(
                ListExpr::panic(
                    panic(),
                    ValueType::Function(Box::new(custom_function_type.to_function_type())),
                )
                .into_function()
                .expect("a custom-function item should create a function list"),
                0,
                custom_function_type,
            )),
            FunctionExpr::bool(BoolFunctionExpr::list_index(
                ListExpr::panic(panic(), ValueType::Function(Box::new(bool_type.clone())))
                    .into_function()
                    .expect("a Bool-function item should create a function list"),
                0,
                bool_type,
            )),
            FunctionExpr::nil(NilFunctionExpr::list_index(
                ListExpr::panic(panic(), ValueType::Function(Box::new(nil_type.clone())))
                    .into_function()
                    .expect("a Nil-function item should create a function list"),
                0,
                nil_type,
            )),
            FunctionExpr::tuple(TupleFunctionExpr::list_index(
                ListExpr::panic(panic(), ValueType::Function(Box::new(tuple_type.clone())))
                    .into_function()
                    .expect("a tuple-function item should create a function list"),
                0,
                tuple_type,
            )),
            FunctionExpr::list(ListFunctionExpr::list_index(
                ListExpr::panic(panic(), ValueType::Function(Box::new(list_type.clone())))
                    .into_function()
                    .expect("a list-function item should create a function list"),
                0,
                list_type,
                ValueType::Int,
            )),
            FunctionExpr::function(FunctionFunctionExpr::list_index(
                ListExpr::panic(
                    panic(),
                    ValueType::Function(Box::new(function_function_type.to_function_type())),
                )
                .into_function()
                .expect("a function-function item should create a function list"),
                0,
                function_function_type,
            )),
        ];
        let mut context = crate::plan::execution::lowering::test_support::lowering_context(vec![
            custom_definition,
        ]);
        let (mut graph, cursor) =
            DraftGraphBuilder::<DraftValueRef, ()>::new(Vec::new(), Vec::new());
        let reference = FunctionExpr::int(IntFunctionExpr::reference(IntFunctionReference::new(
            crate::plan::monomorphic_function_instantiation(
                0,
                FunctionShape::new(Vec::new(), ValueShape::Int),
            ),
        )));
        assert_eq!(
            flow_outcome(super::function_expr(
                &reference,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Value,
        );

        context
            .erased_specializations
            .insert(SpecializationKey::monomorphic(FunctionTemplateId::new(0)));
        let cursor = graph.empty_block(Default::default());
        assert_eq!(
            flow_outcome(super::function_expr(
                &reference,
                cursor,
                &mut graph,
                &mut context,
            )),
            FlowOutcome::Uninhabited,
        );
        context.erased_specializations.clear();

        for expression in expressions {
            let cursor = graph.empty_block(Default::default());
            assert_eq!(
                flow_outcome(super::function_expr(
                    &expression,
                    cursor,
                    &mut graph,
                    &mut context,
                )),
                FlowOutcome::Diverged,
            );
        }
    }

    fn source(family: &FunctionFamily, expression: &str) -> String {
        format!(
            r#"
pub type Marker {{ Marker(Int) }}
pub type Holder(value) {{ Holder(selected: value) }}

fn codepoint() -> UtfCodepoint {{
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}}

fn target() -> {return_type} {{ {value} }}
fn provider() -> fn() -> {return_type} {{ target }}
const selected_constant = target

pub fn main() {{
  let selected: fn() -> {return_type} = {expression}
  {assertion}
}}
"#,
            return_type = family.return_type,
            value = family.value,
            assertion = family.assertion,
        )
    }

    fn diverging_source(family: &FunctionFamily, expression: &str) -> String {
        format!(
            r#"
pub type Marker {{ Marker(Int) }}
pub type Holder(value) {{ Holder(selected: value) }}

fn codepoint() -> UtfCodepoint {{
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}}

fn target() -> {return_type} {{ {value} }}
fn provider(_value: Int) -> fn() -> {return_type} {{ target }}
fn fail_provider() -> fn(Int) -> fn() -> {return_type} {{ panic as "source" }}

pub fn main() {{
  let selected: fn() -> {return_type} = {expression}
  {assertion}
}}
"#,
            return_type = family.return_type,
            value = family.value,
            assertion = family.assertion,
        )
    }

    fn symbolic_diverging_source(target: &str, expression: &str) -> String {
        format!(
            r#"
pub type Boxed(value) {{ Boxed(value) }}
pub type Holder(value) {{ Holder(selected: value) }}

fn codepoint() -> UtfCodepoint {{
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}}

{target}
fn provider(_value: Int) {{ target }}
fn fail_provider() {{ panic as "source" }}

pub fn main() {{
  let _ = {expression}
  0
}}
"#,
        )
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module)
    }
}
