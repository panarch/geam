use super::format::{write_binary, write_integer_binary, write_length, write_unary};
use crate::plan::Text;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::{
    BoolLocalId, FloatLocalId, IntegerOperand, ListLocal, LocalLabel, ParamLocal, StringLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};

#[derive(Clone)]
pub enum BoolTest {
    Not(BoolLocalId),
    EqualInt {
        left: IntegerOperand,
        right: IntegerOperand,
    },
    NotEqualInt {
        left: IntegerOperand,
        right: IntegerOperand,
    },
    LtInt {
        left: IntegerOperand,
        right: IntegerOperand,
    },
    LtEqInt {
        left: IntegerOperand,
        right: IntegerOperand,
    },
    GtInt {
        left: IntegerOperand,
        right: IntegerOperand,
    },
    GtEqInt {
        left: IntegerOperand,
        right: IntegerOperand,
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

impl Explain for BoolTest {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            Self::Not(value) => write_unary(output, "bool.not", value),
            Self::EqualInt { left, right } => {
                write_integer_binary(output, "bool.equal_int", left, right)
            }
            Self::NotEqualInt { left, right } => {
                write_integer_binary(output, "bool.not_equal_int", left, right)
            }
            Self::LtInt { left, right } => write_integer_binary(output, "bool.lt_int", left, right),
            Self::LtEqInt { left, right } => {
                write_integer_binary(output, "bool.lte_int", left, right);
            }
            Self::GtInt { left, right } => write_integer_binary(output, "bool.gt_int", left, right),
            Self::GtEqInt { left, right } => {
                write_integer_binary(output, "bool.gte_int", left, right);
            }
            Self::LtFloat { left, right } => {
                write_binary(output, "bool.lt_float", left, right);
            }
            Self::LtEqFloat { left, right } => {
                write_binary(output, "bool.lte_float", left, right);
            }
            Self::GtFloat { left, right } => {
                write_binary(output, "bool.gt_float", left, right);
            }
            Self::GtEqFloat { left, right } => {
                write_binary(output, "bool.gte_float", left, right);
            }
            Self::Equal { left, right } => write_binary(output, "bool.equal", left, right),
            Self::NotEqual { left, right } => {
                write_binary(output, "bool.not_equal", left, right);
            }
            Self::StringStartsWith { value, prefix } => {
                output.push_str("bool.string_starts_with ");
                value.write_local_label(output);
                output.push_str(" prefix=");
                output.push_str(&format!("{prefix:?}"));
            }
            Self::ListLengthEquals { value, length } => {
                write_length(output, "bool.list_length_equals", value, *length);
            }
            Self::ListLengthAtLeast { value, length } => {
                write_length(output, "bool.list_length_at_least", value, *length);
            }
        }
    }
}

impl Emit for BoolTest {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Not(field_0) => output.call("graph::BoolTest::Not", &[field_0]),
            Self::EqualInt { left, right } => output.structure(
                "graph::BoolTest::EqualInt",
                &[("left", left), ("right", right)],
            ),
            Self::NotEqualInt { left, right } => output.structure(
                "graph::BoolTest::NotEqualInt",
                &[("left", left), ("right", right)],
            ),
            Self::LtInt { left, right } => output.structure(
                "graph::BoolTest::LtInt",
                &[("left", left), ("right", right)],
            ),
            Self::LtEqInt { left, right } => output.structure(
                "graph::BoolTest::LtEqInt",
                &[("left", left), ("right", right)],
            ),
            Self::GtInt { left, right } => output.structure(
                "graph::BoolTest::GtInt",
                &[("left", left), ("right", right)],
            ),
            Self::GtEqInt { left, right } => output.structure(
                "graph::BoolTest::GtEqInt",
                &[("left", left), ("right", right)],
            ),
            Self::LtFloat { left, right } => output.structure(
                "graph::BoolTest::LtFloat",
                &[("left", left), ("right", right)],
            ),
            Self::LtEqFloat { left, right } => output.structure(
                "graph::BoolTest::LtEqFloat",
                &[("left", left), ("right", right)],
            ),
            Self::GtFloat { left, right } => output.structure(
                "graph::BoolTest::GtFloat",
                &[("left", left), ("right", right)],
            ),
            Self::GtEqFloat { left, right } => output.structure(
                "graph::BoolTest::GtEqFloat",
                &[("left", left), ("right", right)],
            ),
            Self::Equal { left, right } => output.structure(
                "graph::BoolTest::Equal",
                &[("left", left), ("right", right)],
            ),
            Self::NotEqual { left, right } => output.structure(
                "graph::BoolTest::NotEqual",
                &[("left", left), ("right", right)],
            ),
            Self::StringStartsWith { value, prefix } => output.structure(
                "graph::BoolTest::StringStartsWith",
                &[("value", value), ("prefix", prefix)],
            ),
            Self::ListLengthEquals { value, length } => output.structure(
                "graph::BoolTest::ListLengthEquals",
                &[("value", value), ("length", length)],
            ),
            Self::ListLengthAtLeast { value, length } => output.structure(
                "graph::BoolTest::ListLengthAtLeast",
                &[("value", value), ("length", length)],
            ),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::BoolTest;
    use crate::plan::execution::graph::{
        BoolListLocalId, BoolLocalId, FloatLocalId, IntLocalId, IntegerOperand, ListLocal,
        ParamLocal, StringLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{BoolListTypeId, ListTypeId};

    #[test]
    fn emits_every_pure_boolean_test_with_its_typed_operands() {
        let cases = [
            (
                BoolTest::LtInt {
                    left: IntegerOperand::Local(IntLocalId(2)),
                    right: IntegerOperand::Immediate(i64::MAX),
                },
                "data::graph::BoolTest::LtInt {\n    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),\n    right: data::graph::IntegerOperand::Immediate(9223372036854775807),\n}",
            ),
            (
                BoolTest::Not(BoolLocalId(2)),
                "data::graph::BoolTest::Not(data::graph::BoolLocalId(2))",
            ),
            (
                BoolTest::EqualInt {
                    left: IntegerOperand::Local(IntLocalId(2)),
                    right: IntegerOperand::Local(IntLocalId(5)),
                },
                r#"
data::graph::BoolTest::EqualInt {
    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
    right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(5)),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::NotEqualInt {
                    left: IntegerOperand::Local(IntLocalId(2)),
                    right: IntegerOperand::Local(IntLocalId(5)),
                },
                r#"
data::graph::BoolTest::NotEqualInt {
    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
    right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(5)),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::LtInt {
                    left: IntegerOperand::Local(IntLocalId(2)),
                    right: IntegerOperand::Local(IntLocalId(5)),
                },
                r#"
data::graph::BoolTest::LtInt {
    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
    right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(5)),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::LtEqInt {
                    left: IntegerOperand::Local(IntLocalId(2)),
                    right: IntegerOperand::Local(IntLocalId(5)),
                },
                r#"
data::graph::BoolTest::LtEqInt {
    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
    right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(5)),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::GtInt {
                    left: IntegerOperand::Local(IntLocalId(2)),
                    right: IntegerOperand::Local(IntLocalId(5)),
                },
                r#"
data::graph::BoolTest::GtInt {
    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
    right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(5)),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::GtEqInt {
                    left: IntegerOperand::Local(IntLocalId(2)),
                    right: IntegerOperand::Local(IntLocalId(5)),
                },
                r#"
data::graph::BoolTest::GtEqInt {
    left: data::graph::IntegerOperand::Local(data::graph::IntLocalId(2)),
    right: data::graph::IntegerOperand::Local(data::graph::IntLocalId(5)),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::LtFloat {
                    left: FloatLocalId(2),
                    right: FloatLocalId(5),
                },
                r#"
data::graph::BoolTest::LtFloat {
    left: data::graph::FloatLocalId(2),
    right: data::graph::FloatLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::LtEqFloat {
                    left: FloatLocalId(2),
                    right: FloatLocalId(5),
                },
                r#"
data::graph::BoolTest::LtEqFloat {
    left: data::graph::FloatLocalId(2),
    right: data::graph::FloatLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::GtFloat {
                    left: FloatLocalId(2),
                    right: FloatLocalId(5),
                },
                r#"
data::graph::BoolTest::GtFloat {
    left: data::graph::FloatLocalId(2),
    right: data::graph::FloatLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::GtEqFloat {
                    left: FloatLocalId(2),
                    right: FloatLocalId(5),
                },
                r#"
data::graph::BoolTest::GtEqFloat {
    left: data::graph::FloatLocalId(2),
    right: data::graph::FloatLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::Equal {
                    left: ParamLocal::Bool(BoolLocalId(2)),
                    right: ParamLocal::Bool(BoolLocalId(5)),
                },
                r#"
data::graph::BoolTest::Equal {
    left: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
    right: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(5)),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::NotEqual {
                    left: ParamLocal::Bool(BoolLocalId(2)),
                    right: ParamLocal::Bool(BoolLocalId(5)),
                },
                r#"
data::graph::BoolTest::NotEqual {
    left: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(2)),
    right: data::graph::ParamLocal::Bool(data::graph::BoolLocalId(5)),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::StringStartsWith {
                    value: StringLocalId(2),
                    prefix: "pre".into(),
                },
                r#"
data::graph::BoolTest::StringStartsWith {
    value: data::graph::StringLocalId(2),
    prefix: data::Text::Static("pre"),
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::ListLengthEquals {
                    value: ListLocal::Bool {
                        local: BoolListLocalId(2),
                        type_id: BoolListTypeId::new(ListTypeId(4)),
                    },
                    length: 3,
                },
                r#"
data::graph::BoolTest::ListLengthEquals {
    value: data::graph::ListLocal::Bool {
        local: data::graph::BoolListLocalId(2),
        type_id: data::type_::BoolListTypeId {
            list_type: data::type_::ListTypeId(4),
        },
    },
    length: 3,
}"#
                .trim_start_matches('\n'),
            ),
            (
                BoolTest::ListLengthAtLeast {
                    value: ListLocal::Bool {
                        local: BoolListLocalId(2),
                        type_id: BoolListTypeId::new(ListTypeId(4)),
                    },
                    length: 3,
                },
                r#"
data::graph::BoolTest::ListLengthAtLeast {
    value: data::graph::ListLocal::Bool {
        local: data::graph::BoolListLocalId(2),
        type_id: data::type_::BoolListTypeId {
            list_type: data::type_::ListTypeId(4),
        },
    },
    length: 3,
}"#
                .trim_start_matches('\n'),
            ),
        ];
        for (value, expected) in cases {
            assert_eq!(Rust::expression(&value), expected);
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use super::BoolTest;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::BoolFunctionId;
    use crate::plan::execution::graph::{
        IntListLocalId, ListLocal, ProfiledInstructionKind, StringLocalId, Terminator,
    };
    use crate::plan::execution::type_::{IntListTypeId, ListTypeId};

    #[test]
    fn writes_bool_test_grammar() {
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
  && values != [2]
}
"#;
        let expected = concat!(
            "bool.value True | bool.not %bool#0 | bool.lt_int %int#0 2 | ",
            "bool.lte_int %int#0 2 | bool.gt_int %int#0 0 | ",
            "bool.gte_int %int#0 0 | bool.lt_float %float#0 %float#1 | ",
            "bool.lte_float %float#0 %float#1 | bool.gt_float %float#0 %float#1 | ",
            "bool.gte_float %float#0 %float#1 | bool.equal_int %int#0 1 | ",
            "bool.not_equal_int %int#0 2 | bool.equal %list.int#0 %list.int#1 | ",
            "bool.not_equal %list.int#0 %list.int#1 | ",
            "bool.value True | bool.value False",
        );

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_prefix_and_length_test_operands_exactly() {
        let cases = [
            (
                BoolTest::StringStartsWith {
                    value: StringLocalId(2),
                    prefix: "x\"\\\n".into(),
                },
                "bool.string_starts_with %string#2 prefix=\"x\\\"\\\\\\n\"",
            ),
            (
                BoolTest::ListLengthEquals {
                    value: ListLocal::Int {
                        local: IntListLocalId(2),
                        type_id: IntListTypeId::new(ListTypeId(3)),
                    },
                    length: 4,
                },
                "bool.list_length_equals %list.int#2 length=4",
            ),
            (
                BoolTest::ListLengthAtLeast {
                    value: ListLocal::Int {
                        local: IntListLocalId(2),
                        type_id: IntListTypeId::new(ListTypeId(3)),
                    },
                    length: 4,
                },
                "bool.list_length_at_least %list.int#2 length=4",
            ),
        ];
        explain::with_execution_plan("pub fn main() { True }", |plan| {
            for (test, expected) in cases {
                let mut actual = String::new();
                let mut context = explain::ExplainContext::new(plan, &mut actual);
                context.write(&test);
                assert_eq!(actual, expected);
            }
        });
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan.bool_function(BoolFunctionId(0)).body().block_graph();
            let mut parts = Vec::new();
            for block in graph.blocks() {
                for instruction in block.instructions() {
                    if let ProfiledInstructionKind::Bool(instruction) = instruction.kind() {
                        let mut part = String::new();
                        let mut context = explain::ExplainContext::new(plan, &mut part);
                        context.write(instruction);
                        parts.push(part);
                    }
                }
                if let Terminator::TestBranch(branch) = block.terminator() {
                    let mut part = String::new();
                    let mut context = explain::ExplainContext::new(plan, &mut part);
                    context.write(&branch.test);
                    parts.push(part);
                }
            }
            output.push_str(&parts.join(" | "));
        });
    }
}
