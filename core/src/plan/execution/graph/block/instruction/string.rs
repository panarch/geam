use super::{
    write_binary, write_call, write_constant, write_function_call, write_literal, write_projection,
};
use crate::plan::Text;
use crate::plan::execution::constant::ConstantId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::StringFunctionId;
use crate::plan::execution::graph::LocalLabel;
use crate::plan::execution::graph::{
    CustomLocal, ParamLocal, StringFunctionLocalId, StringListLocalId, StringLocalId, TupleLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;

#[derive(Clone)]
pub enum StringInstruction {
    Value(Text),
    Constant(ConstantId<StringLocalId>),
    Call {
        function: StringFunctionId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: StringFunctionLocalId,
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
        list: StringListLocalId,
        index: usize,
    },
    Concatenate {
        left: StringLocalId,
        right: StringLocalId,
    },
    DropPrefix {
        value: StringLocalId,
        prefix: Text,
    },
}

impl Explain for StringInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            StringInstruction::Value(value) => {
                write_literal(output, "string.value", &format!("{value:?}"));
            }
            StringInstruction::Constant(id) => write_constant(output, "string", *id),
            StringInstruction::Call { function, args, .. } => {
                write_call(output, "string.call", function, args);
            }
            StringInstruction::FunctionCall { function, args, .. } => {
                write_function_call(output, "string.function_call", function, args);
            }
            StringInstruction::TupleIndex { tuple, index } => {
                write_projection(output, "string.tuple_index", tuple, *index);
            }
            StringInstruction::CustomField { source, index } => {
                write_projection(output, "string.custom_field", source, *index);
            }
            StringInstruction::ListIndex { list, index } => {
                write_projection(output, "string.list_index", list, *index);
            }
            StringInstruction::Concatenate { left, right } => {
                write_binary(output, "string.concatenate", left, right);
            }
            StringInstruction::DropPrefix { value, prefix } => {
                output.push_str("string.drop_prefix ");
                value.write_local_label(output);
                output.push_str(" prefix=");
                output.push_str(&format!("{prefix:?}"));
            }
        }
    }
}

impl Emit for StringInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value(field_0) => output.call("graph::StringInstruction::Value", &[field_0]),
            Self::Constant(field_0) => {
                output.call("graph::StringInstruction::Constant", &[field_0])
            }
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::StringInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::StringInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::StringInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::StringInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::StringInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
            Self::Concatenate { left, right } => output.structure(
                "graph::StringInstruction::Concatenate",
                &[("left", left), ("right", right)],
            ),
            Self::DropPrefix { value, prefix } => output.structure(
                "graph::StringInstruction::DropPrefix",
                &[("value", value), ("prefix", prefix)],
            ),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::StringInstruction;
    use crate::plan::execution::constant::ConstantId;
    use crate::plan::execution::function::StringFunctionId;
    use crate::plan::execution::graph::{
        CustomLocal, CustomLocalId, ParamLocal, StringFunctionLocalId, StringListLocalId,
        StringLocalId, TupleLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{CustomTypeId, CustomValueShape, CustomValueShapeId};
    use crate::plan::{HostCallSite, SourceSpan};

    #[test]
    fn emits_every_string_instruction_with_its_operands_and_source_site() {
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let cases = [
            (
                StringInstruction::Value("line\n\"quoted\"".into()),
                "data::graph::StringInstruction::Value(data::Text::Static(\"line\\n\\\"quoted\\\"\"))",
            ),
            (
                StringInstruction::Constant(ConstantId::new(3)),
                r#"
data::graph::StringInstruction::Constant(data::constant::ConstantId {
    index: 3,
    value: ::core::marker::PhantomData,
})"#.trim_start_matches('\n'),
            ),
            (
                StringInstruction::Call {
                    function: StringFunctionId(2),
                    args: vec![ParamLocal::String(StringLocalId(5))].into(),
                    site: site.clone(),
                },
                r#"
data::graph::StringInstruction::Call {
    function: data::function::StringFunctionId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::String(data::graph::StringLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                StringInstruction::FunctionCall {
                    function: StringFunctionLocalId(2),
                    args: vec![ParamLocal::String(StringLocalId(5))].into(),
                    site,
                },
                r#"
data::graph::StringInstruction::FunctionCall {
    function: data::graph::StringFunctionLocalId(2),
    args: data::Storage::Static(&[
        data::graph::ParamLocal::String(data::graph::StringLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                StringInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                r#"
data::graph::StringInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                StringInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                r#"
data::graph::StringInstruction::CustomField {
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
                StringInstruction::ListIndex {
                    list: StringListLocalId(2),
                    index: 1,
                },
                r#"
data::graph::StringInstruction::ListIndex {
    list: data::graph::StringListLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                StringInstruction::Concatenate {
                    left: StringLocalId(2),
                    right: StringLocalId(5),
                },
                r#"
data::graph::StringInstruction::Concatenate {
    left: data::graph::StringLocalId(2),
    right: data::graph::StringLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                StringInstruction::DropPrefix {
                    value: StringLocalId(2),
                    prefix: "pre".into(),
                },
                r#"
data::graph::StringInstruction::DropPrefix {
    value: data::graph::StringLocalId(2),
    prefix: data::Text::Static("pre"),
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
    use crate::plan::execution::function::TupleFunctionId;
    use crate::plan::execution::graph::ProfiledInstructionKind;

    #[test]
    fn writes_string_values_and_concatenation() {
        let source = r#"
pub fn main() {
  let prefix = "pre"
  #(prefix <> "fix")
}
"#;
        let expected =
            "string.value \"pre\" | string.value \"fix\" | string.concatenate %string#0 %string#1";

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_string_constants_calls_and_projections() {
        let source = r#"
const saved = "saved"

pub type Holder {
  Holder(value: String)
}

fn string_value(value: String) { value }
fn string_values(values: List(String)) { values }

pub fn main() {
  let function = string_value
  let values = string_values(["list"])
  let selected = case values {
    [value, ..] -> value
    _ -> ""
  }
  let tuple = #("tuple")
  let holder = Holder("record")
  let text = string_value("prefix-tail")
  let suffix = case text {
    "prefix-" <> rest -> rest
    _ -> ""
  }
  #(
    saved,
    string_value("call"),
    function("function"),
    tuple.0,
    holder.value,
    selected,
    suffix,
  )
}
"#;
        let expected = concat!(
            "string.value \"list\" | string.list_index %list.string#0 index=0 | ",
            "string.value \"tuple\" | string.value \"record\" | ",
            "string.value \"prefix-tail\" | string.call string#0 args=[%string#3] | ",
            "string.drop_prefix %string#1 prefix=\"prefix-\" | constant.string#0 | ",
            "string.value \"call\" | string.call string#0 args=[%string#3] | ",
            "string.value \"function\" | ",
            "string.function_call %function.string#0 args=[%string#5] | ",
            "string.tuple_index %tuple#0 index=0 | ",
            "string.custom_field %custom#0 index=0 | string.value \"\" | string.value \"\"",
        );

        assert_explanation(source, expected);
    }

    fn write_separator(output: &mut String, first: &mut bool) {
        if *first {
            *first = false;
        } else {
            output.push_str(" | ");
        }
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan.tuple_function(TupleFunctionId(0)).body().block_graph();
            let mut first = true;
            for instruction in graph.blocks().flat_map(|block| block.instructions()) {
                if let ProfiledInstructionKind::String(instruction) = instruction.kind() {
                    write_separator(output, &mut first);
                    let mut context = explain::ExplainContext::new(plan, output);
                    context.write(instruction);
                }
            }
        });
    }
}
