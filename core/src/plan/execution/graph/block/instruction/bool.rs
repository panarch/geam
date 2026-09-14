use super::{
    write_binary, write_call, write_constant, write_function_call, write_length, write_literal,
    write_projection, write_unary,
};
use crate::plan::Text;
use crate::plan::execution::constant::ConstantId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::BoolFunctionId;
use crate::plan::execution::graph::LocalLabel;
use crate::plan::execution::graph::{
    BoolFunctionLocalId, BoolListLocalId, BoolLocalId, CustomLocal, FloatLocalId, IntLocalId,
    ListLocal, ParamLocal, StringLocalId, TupleLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub enum BoolInstruction {
    Value(bool),
    Constant(ConstantId<BoolLocalId>),
    Call {
        function: BoolFunctionId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: BoolFunctionLocalId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    TupleIndex {
        tuple: TupleLocalId,
        index: usize,
    },
    CustomField {
        source: CustomLocal,
        index: usize,
    },
    ListIndex {
        list: BoolListLocalId,
        index: usize,
    },
    Not(BoolLocalId),
    LtInt {
        left: IntLocalId,
        right: IntLocalId,
    },
    LtEqInt {
        left: IntLocalId,
        right: IntLocalId,
    },
    GtInt {
        left: IntLocalId,
        right: IntLocalId,
    },
    GtEqInt {
        left: IntLocalId,
        right: IntLocalId,
    },
    LtFloat {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    LtEqFloat {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    GtFloat {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    GtEqFloat {
        left: FloatLocalId,
        right: FloatLocalId,
    },
    Equal {
        left: ParamLocal,
        right: ParamLocal,
    },
    NotEqual {
        left: ParamLocal,
        right: ParamLocal,
    },
    StringStartsWith {
        value: StringLocalId,
        prefix: Text,
    },
    ListLengthEquals {
        value: ListLocal,
        length: usize,
    },
    ListLengthAtLeast {
        value: ListLocal,
        length: usize,
    },
}

impl Explain for BoolInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            BoolInstruction::Value(value) => {
                write_literal(output, "bool.value", if *value { "True" } else { "False" });
            }
            BoolInstruction::Constant(id) => write_constant(output, "bool", *id),
            BoolInstruction::Call { function, args, .. } => {
                write_call(output, "bool.call", function, args);
            }
            BoolInstruction::FunctionCall { function, args, .. } => {
                write_function_call(output, "bool.function_call", function, args);
            }
            BoolInstruction::TupleIndex { tuple, index } => {
                write_projection(output, "bool.tuple_index", tuple, *index);
            }
            BoolInstruction::CustomField { source, index } => {
                write_projection(output, "bool.custom_field", source, *index);
            }
            BoolInstruction::ListIndex { list, index } => {
                write_projection(output, "bool.list_index", list, *index);
            }
            BoolInstruction::Not(value) => write_unary(output, "bool.not", value),
            BoolInstruction::LtInt { left, right } => {
                write_binary(output, "bool.lt_int", left, right)
            }
            BoolInstruction::LtEqInt { left, right } => {
                write_binary(output, "bool.lte_int", left, right);
            }
            BoolInstruction::GtInt { left, right } => {
                write_binary(output, "bool.gt_int", left, right)
            }
            BoolInstruction::GtEqInt { left, right } => {
                write_binary(output, "bool.gte_int", left, right);
            }
            BoolInstruction::LtFloat { left, right } => {
                write_binary(output, "bool.lt_float", left, right);
            }
            BoolInstruction::LtEqFloat { left, right } => {
                write_binary(output, "bool.lte_float", left, right);
            }
            BoolInstruction::GtFloat { left, right } => {
                write_binary(output, "bool.gt_float", left, right);
            }
            BoolInstruction::GtEqFloat { left, right } => {
                write_binary(output, "bool.gte_float", left, right);
            }
            BoolInstruction::Equal { left, right } => {
                write_binary(output, "bool.equal", left, right)
            }
            BoolInstruction::NotEqual { left, right } => {
                write_binary(output, "bool.not_equal", left, right);
            }
            BoolInstruction::StringStartsWith { value, prefix } => {
                output.push_str("bool.string_starts_with ");
                value.write_local_label(output);
                output.push_str(" prefix=");
                output.push_str(&format!("{prefix:?}"));
            }
            BoolInstruction::ListLengthEquals { value, length } => {
                write_length(output, "bool.list_length_equals", value, *length);
            }
            BoolInstruction::ListLengthAtLeast { value, length } => {
                write_length(output, "bool.list_length_at_least", value, *length);
            }
        }
    }
}

impl Emit for BoolInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value(field_0) => output.call("graph::BoolInstruction::Value", &[field_0]),
            Self::Constant(field_0) => output.call("graph::BoolInstruction::Constant", &[field_0]),
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::BoolInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::BoolInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::BoolInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::BoolInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::BoolInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
            Self::Not(field_0) => output.call("graph::BoolInstruction::Not", &[field_0]),
            Self::LtInt { left, right } => output.structure(
                "graph::BoolInstruction::LtInt",
                &[("left", left), ("right", right)],
            ),
            Self::LtEqInt { left, right } => output.structure(
                "graph::BoolInstruction::LtEqInt",
                &[("left", left), ("right", right)],
            ),
            Self::GtInt { left, right } => output.structure(
                "graph::BoolInstruction::GtInt",
                &[("left", left), ("right", right)],
            ),
            Self::GtEqInt { left, right } => output.structure(
                "graph::BoolInstruction::GtEqInt",
                &[("left", left), ("right", right)],
            ),
            Self::LtFloat { left, right } => output.structure(
                "graph::BoolInstruction::LtFloat",
                &[("left", left), ("right", right)],
            ),
            Self::LtEqFloat { left, right } => output.structure(
                "graph::BoolInstruction::LtEqFloat",
                &[("left", left), ("right", right)],
            ),
            Self::GtFloat { left, right } => output.structure(
                "graph::BoolInstruction::GtFloat",
                &[("left", left), ("right", right)],
            ),
            Self::GtEqFloat { left, right } => output.structure(
                "graph::BoolInstruction::GtEqFloat",
                &[("left", left), ("right", right)],
            ),
            Self::Equal { left, right } => output.structure(
                "graph::BoolInstruction::Equal",
                &[("left", left), ("right", right)],
            ),
            Self::NotEqual { left, right } => output.structure(
                "graph::BoolInstruction::NotEqual",
                &[("left", left), ("right", right)],
            ),
            Self::StringStartsWith { value, prefix } => output.structure(
                "graph::BoolInstruction::StringStartsWith",
                &[("value", value), ("prefix", prefix)],
            ),
            Self::ListLengthEquals { value, length } => output.structure(
                "graph::BoolInstruction::ListLengthEquals",
                &[("value", value), ("length", length)],
            ),
            Self::ListLengthAtLeast { value, length } => output.structure(
                "graph::BoolInstruction::ListLengthAtLeast",
                &[("value", value), ("length", length)],
            ),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::BoolInstruction;
    use crate::plan::execution::constant::ConstantId;
    use crate::plan::execution::function::BoolFunctionId;
    use crate::plan::execution::graph::{
        BoolFunctionLocalId, BoolListLocalId, BoolLocalId, CustomLocal, CustomLocalId,
        FloatLocalId, IntLocalId, ListLocal, ParamLocal, StringLocalId, TupleLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{
        BoolListTypeId, CustomTypeId, CustomValueShape, CustomValueShapeId, ListTypeId,
    };
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn emits_every_boolean_instruction_with_its_operands_and_source_site() {
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let cases = [
            (
                BoolInstruction::Value(true),
                "data::graph::BoolInstruction::Value(true)",
            ),
            (
                BoolInstruction::Constant(ConstantId::new(3)),
                r#"
data::graph::BoolInstruction::Constant(data::constant::ConstantId {
    index: 3,
    value: ::core::marker::PhantomData,
})"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::Call {
                    function: BoolFunctionId(2),
                    args: vec![ParamLocal::Bool(BoolLocalId(5))].into(),
                    site: site.clone(),
                },
                r#"
data::graph::BoolInstruction::Call {
    function: data::function::BoolFunctionId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::FunctionCall {
                    function: BoolFunctionLocalId(2),
                    args: vec![ParamLocal::Bool(BoolLocalId(5))].into(),
                    site,
                },
                r#"
data::graph::BoolInstruction::FunctionCall {
    function: data::graph::BoolFunctionLocalId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Bool(data::graph::BoolLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                r#"
data::graph::BoolInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                r#"
data::graph::BoolInstruction::CustomField {
    source: data::graph::CustomLocal {
        id: data::graph::CustomLocalId(2),
        shape: data::type_::CustomValueShape {
            type_id: data::type_::CustomTypeId(3),
            shape_id: data::type_::CustomValueShapeId(4),
        },
    },
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::ListIndex {
                    list: BoolListLocalId(2),
                    index: 1,
                },
                r#"
data::graph::BoolInstruction::ListIndex {
    list: data::graph::BoolListLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::Not(BoolLocalId(2)),
                "data::graph::BoolInstruction::Not(data::graph::BoolLocalId(2))",
            ),
            (
                BoolInstruction::LtInt {
                    left: IntLocalId(2),
                    right: IntLocalId(5),
                },
                r#"
data::graph::BoolInstruction::LtInt {
    left: data::graph::IntLocalId(2),
    right: data::graph::IntLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::LtEqInt {
                    left: IntLocalId(2),
                    right: IntLocalId(5),
                },
                r#"
data::graph::BoolInstruction::LtEqInt {
    left: data::graph::IntLocalId(2),
    right: data::graph::IntLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::GtInt {
                    left: IntLocalId(2),
                    right: IntLocalId(5),
                },
                r#"
data::graph::BoolInstruction::GtInt {
    left: data::graph::IntLocalId(2),
    right: data::graph::IntLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::GtEqInt {
                    left: IntLocalId(2),
                    right: IntLocalId(5),
                },
                r#"
data::graph::BoolInstruction::GtEqInt {
    left: data::graph::IntLocalId(2),
    right: data::graph::IntLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::LtFloat {
                    left: FloatLocalId(2),
                    right: FloatLocalId(5),
                },
                r#"
data::graph::BoolInstruction::LtFloat {
    left: data::graph::FloatLocalId(2),
    right: data::graph::FloatLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::LtEqFloat {
                    left: FloatLocalId(2),
                    right: FloatLocalId(5),
                },
                r#"
data::graph::BoolInstruction::LtEqFloat {
    left: data::graph::FloatLocalId(2),
    right: data::graph::FloatLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::GtFloat {
                    left: FloatLocalId(2),
                    right: FloatLocalId(5),
                },
                r#"
data::graph::BoolInstruction::GtFloat {
    left: data::graph::FloatLocalId(2),
    right: data::graph::FloatLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::GtEqFloat {
                    left: FloatLocalId(2),
                    right: FloatLocalId(5),
                },
                r#"
data::graph::BoolInstruction::GtEqFloat {
    left: data::graph::FloatLocalId(2),
    right: data::graph::FloatLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::Equal {
                    left: ParamLocal::Bool(BoolLocalId(2)),
                    right: ParamLocal::Bool(BoolLocalId(5)),
                },
                r#"
data::graph::BoolInstruction::Equal {
    left: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
    right: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(5)),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::NotEqual {
                    left: ParamLocal::Bool(BoolLocalId(2)),
                    right: ParamLocal::Bool(BoolLocalId(5)),
                },
                r#"
data::graph::BoolInstruction::NotEqual {
    left: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
    right: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(5)),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::StringStartsWith {
                    value: StringLocalId(2),
                    prefix: "pre".into(),
                },
                r#"
data::graph::BoolInstruction::StringStartsWith {
    value: data::graph::StringLocalId(2),
    prefix: data::Text::Static("pre"),
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::ListLengthEquals {
                    value: ListLocal::Bool {
                        local: BoolListLocalId(2),
                        type_id: BoolListTypeId::new(ListTypeId(4)),
                    },
                    length: 3,
                },
                r#"
data::graph::BoolInstruction::ListLengthEquals {
    value: data::graph::ListLocal::Bool {
        local: data::graph::BoolListLocalId(2),
        type_id: data::type_::BoolListTypeId {
            list_type: data::type_::ListTypeId(4),
        },
    },
    length: 3,
}"#.trim_start_matches('\n'),
            ),
            (
                BoolInstruction::ListLengthAtLeast {
                    value: ListLocal::Bool {
                        local: BoolListLocalId(2),
                        type_id: BoolListTypeId::new(ListTypeId(4)),
                    },
                    length: 3,
                },
                r#"
data::graph::BoolInstruction::ListLengthAtLeast {
    value: data::graph::ListLocal::Bool {
        local: data::graph::BoolListLocalId(2),
        type_id: data::type_::BoolListTypeId {
            list_type: data::type_::ListTypeId(4),
        },
    },
    length: 3,
}"#.trim_start_matches('\n'),
            ),
        ];
        for (instruction, expected) in cases {
            assert_eq!(Rust::expression(&instruction), expected);
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::BoolFunctionId;
    use crate::plan::execution::graph::ProfiledInstructionKind;

    #[test]
    fn writes_bool_instruction_grammar() {
        let source = r#"
pub fn main() {
  let integer = 1
  let float = 1.0
  let values = [1]
  !True
  && integer < 2
  && integer <= 2
  && integer > 0
  && integer >= 0
  && float <. 2.0
  && float <=. 2.0
  && float >. 0.0
  && float >=. 0.0
  && integer == 1
  && integer != 2
  && values == [1]
}
"#;
        let expected = concat!(
            "bool.value True | bool.not %bool#0 | bool.lt_int %int#0 %int#1 | ",
            "bool.lte_int %int#0 %int#1 | bool.gt_int %int#0 %int#1 | ",
            "bool.gte_int %int#0 %int#1 | bool.lt_float %float#0 %float#1 | ",
            "bool.lte_float %float#0 %float#1 | bool.gt_float %float#0 %float#1 | ",
            "bool.gte_float %float#0 %float#1 | bool.equal %int#0 %int#1 | ",
            "bool.not_equal %int#0 %int#1 | bool.equal %list.int#0 %list.int#1 | ",
            "bool.value True | bool.value False",
        );

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_bool_constants_calls_projections_and_pattern_checks() {
        let source = r#"
const saved = True

pub type Flag {
  Flag(value: Bool)
}

fn bool_value(value: Bool) { value }
fn bool_values(values: List(Bool)) { values }
fn string_value(value: String) { value }

pub fn main() {
  let function = bool_value
  let values = bool_values([True])
  let selected = case values {
    [value, ..] -> value
    _ -> False
  }
  let exact = case values {
    [value] -> value
    _ -> False
  }
  let tuple = #(True)
  let record = Flag(True)
  let text = string_value("prefix-tail")
  let prefix = case text {
    "prefix-" <> _ -> True
    _ -> False
  }

  saved
  && bool_value(True)
  && function(True)
  && tuple.0
  && record.value
  && selected
  && exact
  && prefix
}
"#;
        let expected = concat!(
            "bool.value True | bool.list_length_at_least %list.bool#1 length=1 | ",
            "bool.list_index %list.bool#0 index=0 | bool.value True | bool.value True | ",
            "bool.list_length_equals %list.bool#0 length=1 | ",
            "bool.list_index %list.bool#0 index=0 | bool.value True | bool.value True | ",
            "bool.value True | bool.value True | ",
            "bool.string_starts_with %string#1 prefix=\"prefix-\" | bool.value True | ",
            "bool.value True | constant.bool#0 | bool.value True | ",
            "bool.call bool#1 args=[%bool#3] | bool.value True | ",
            "bool.function_call %function.bool#0 args=[%bool#3] | ",
            "bool.tuple_index %tuple#0 index=0 | bool.custom_field %custom#0 index=0 | ",
            "bool.value True | bool.value False | bool.value False | bool.value False | ",
            "bool.value False | bool.value False | bool.value False | bool.value False | ",
            "bool.value False | bool.value False",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan.bool_function(BoolFunctionId(0)).body().block_graph();
            let mut first = true;
            for instruction in graph.blocks().flat_map(|block| block.instructions()) {
                if let ProfiledInstructionKind::Bool(instruction) = instruction.kind() {
                    if first {
                        first = false;
                    } else {
                        output.push_str(" | ");
                    }
                    let mut context = explain::ExplainContext::new(plan, output);
                    context.write(instruction);
                }
            }
        });
    }
}
