use super::super::super::graph::{
    BoolInstruction, CustomInstruction, FloatInstruction, IntInstruction, NilInstruction,
    StringInstruction, TupleInstruction, UtfCodepointInstruction,
};
use super::super::value::{ExplainLocal, write_locals};
use super::operand::{
    write_binary, write_call, write_constant, write_function_call, write_length, write_literal,
    write_projection, write_unary,
};

pub(super) fn write_int(output: &mut String, instruction: &IntInstruction) {
    match instruction {
        IntInstruction::Value(value) => write_literal(output, "int.value", &value.to_string()),
        IntInstruction::Constant(id) => write_constant(output, "int", *id),
        IntInstruction::Call { function, args } => write_call(output, "int.call", function, args),
        IntInstruction::FunctionCall { function, args } => {
            write_function_call(output, "int.function_call", function, args);
        }
        IntInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "int.tuple_index", tuple, *index);
        }
        IntInstruction::CustomField { source, index } => {
            write_projection(output, "int.custom_field", source, *index);
        }
        IntInstruction::ListIndex { list, index } => {
            write_projection(output, "int.list_index", list, *index);
        }
        IntInstruction::Add { left, right } => write_binary(output, "int.add", left, right),
        IntInstruction::Sub { left, right } => write_binary(output, "int.sub", left, right),
        IntInstruction::Mult { left, right } => write_binary(output, "int.mult", left, right),
        IntInstruction::Div { left, right } => write_binary(output, "int.div", left, right),
        IntInstruction::Remainder { left, right } => {
            write_binary(output, "int.remainder", left, right);
        }
        IntInstruction::Negate(value) => write_unary(output, "int.negate", value),
    }
}

pub(super) fn write_float(output: &mut String, instruction: &FloatInstruction) {
    match instruction {
        FloatInstruction::Value(value) => {
            write_literal(output, "float.value", &format!("{value:?}"));
        }
        FloatInstruction::Constant(id) => write_constant(output, "float", *id),
        FloatInstruction::Call { function, args } => {
            write_call(output, "float.call", function, args);
        }
        FloatInstruction::FunctionCall { function, args } => {
            write_function_call(output, "float.function_call", function, args);
        }
        FloatInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "float.tuple_index", tuple, *index);
        }
        FloatInstruction::CustomField { source, index } => {
            write_projection(output, "float.custom_field", source, *index);
        }
        FloatInstruction::ListIndex { list, index } => {
            write_projection(output, "float.list_index", list, *index);
        }
        FloatInstruction::Add { left, right } => write_binary(output, "float.add", left, right),
        FloatInstruction::Sub { left, right } => write_binary(output, "float.sub", left, right),
        FloatInstruction::Mult { left, right } => {
            write_binary(output, "float.mult", left, right);
        }
        FloatInstruction::Div { left, right } => write_binary(output, "float.div", left, right),
    }
}

pub(super) fn write_string(output: &mut String, instruction: &StringInstruction) {
    match instruction {
        StringInstruction::Value(value) => {
            write_literal(output, "string.value", &format!("{value:?}"));
        }
        StringInstruction::Constant(id) => write_constant(output, "string", *id),
        StringInstruction::Call { function, args } => {
            write_call(output, "string.call", function, args);
        }
        StringInstruction::FunctionCall { function, args } => {
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
            value.write_local(output);
            output.push_str(" prefix=");
            output.push_str(&format!("{prefix:?}"));
        }
    }
}

pub(super) fn write_utf_codepoint(output: &mut String, instruction: &UtfCodepointInstruction) {
    match instruction {
        UtfCodepointInstruction::Call { function, args } => {
            write_call(output, "utf_codepoint.call", function, args);
        }
        UtfCodepointInstruction::FunctionCall { function, args } => {
            write_function_call(output, "utf_codepoint.function_call", function, args);
        }
        UtfCodepointInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "utf_codepoint.tuple_index", tuple, *index);
        }
        UtfCodepointInstruction::CustomField { source, index } => {
            write_projection(output, "utf_codepoint.custom_field", source, *index);
        }
        UtfCodepointInstruction::ListIndex { list, index } => {
            write_projection(output, "utf_codepoint.list_index", list, *index);
        }
    }
}

pub(super) fn write_custom(output: &mut String, instruction: &CustomInstruction) {
    match instruction {
        CustomInstruction::Construct {
            constructor,
            fields,
        } => {
            output.push_str("custom.construct custom_type#");
            output.push_str(&constructor.type_id().index().to_string());
            output.push_str(".constructor#");
            output.push_str(&constructor.index().to_string());
            output.push_str(" fields=");
            write_locals(output, fields);
        }
        CustomInstruction::Constant(id) => write_constant(output, "custom", *id),
        CustomInstruction::Call { function, args } => {
            write_call(output, "custom.call", function, args);
        }
        CustomInstruction::FunctionCall { function, args } => {
            write_function_call(output, "custom.function_call", function, args);
        }
        CustomInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "custom.tuple_index", tuple, *index);
        }
        CustomInstruction::CustomField { source, index } => {
            write_projection(output, "custom.custom_field", source, *index);
        }
        CustomInstruction::ListIndex { list, index } => {
            write_projection(output, "custom.list_index", list, *index);
        }
    }
}

pub(super) fn write_bool(output: &mut String, instruction: &BoolInstruction) {
    match instruction {
        BoolInstruction::Value(value) => {
            write_literal(output, "bool.value", if *value { "True" } else { "False" });
        }
        BoolInstruction::Constant(id) => write_constant(output, "bool", *id),
        BoolInstruction::Call { function, args } => {
            write_call(output, "bool.call", function, args);
        }
        BoolInstruction::FunctionCall { function, args } => {
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
        BoolInstruction::LtInt { left, right } => write_binary(output, "bool.lt_int", left, right),
        BoolInstruction::LtEqInt { left, right } => {
            write_binary(output, "bool.lte_int", left, right);
        }
        BoolInstruction::GtInt { left, right } => write_binary(output, "bool.gt_int", left, right),
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
        BoolInstruction::Equal { left, right } => write_binary(output, "bool.equal", left, right),
        BoolInstruction::NotEqual { left, right } => {
            write_binary(output, "bool.not_equal", left, right);
        }
        BoolInstruction::StringStartsWith { value, prefix } => {
            output.push_str("bool.string_starts_with ");
            value.write_local(output);
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

pub(super) fn write_nil(output: &mut String, instruction: &NilInstruction) {
    match instruction {
        NilInstruction::Value => output.push_str("nil.value"),
        NilInstruction::Constant(id) => write_constant(output, "nil", *id),
        NilInstruction::Call { function, args } => write_call(output, "nil.call", function, args),
        NilInstruction::FunctionCall { function, args } => {
            write_function_call(output, "nil.function_call", function, args);
        }
        NilInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "nil.tuple_index", tuple, *index);
        }
        NilInstruction::CustomField { source, index } => {
            write_projection(output, "nil.custom_field", source, *index);
        }
        NilInstruction::ListIndex { list, index } => {
            write_projection(output, "nil.list_index", list, *index);
        }
    }
}

pub(super) fn write_tuple(output: &mut String, instruction: &TupleInstruction) {
    match instruction {
        TupleInstruction::Value(elements) => {
            output.push_str("tuple.value elements=");
            write_locals(output, elements);
        }
        TupleInstruction::Constant(id) => write_constant(output, "tuple", *id),
        TupleInstruction::Call { function, args } => {
            write_call(output, "tuple.call", function, args);
        }
        TupleInstruction::FunctionCall { function, args } => {
            write_function_call(output, "tuple.function_call", function, args);
        }
        TupleInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "tuple.tuple_index", tuple, *index);
        }
        TupleInstruction::CustomField { source, index } => {
            write_projection(output, "tuple.custom_field", source, *index);
        }
        TupleInstruction::ListIndex { list, index } => {
            write_projection(output, "tuple.list_index", list, *index);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::TupleFunctionId;

    #[test]
    fn writes_value_instruction_grammar() {
        let source = r#"
pub fn main() {
  let integer = 1
  let float = 1.5
  let string = "a"
  let boolean = True
  let nil = Nil
  let tuple = #(integer)
  #(
    integer + 2,
    -integer,
    float +. 2.0,
    string <> "b",
    !boolean,
    nil,
    tuple,
  )
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let graph = plan.tuple_function(TupleFunctionId(0)).graph();
        let mut output = String::new();

        for instruction in graph.blocks()[0].instructions() {
            super::super::write_instruction(&mut output, &plan, instruction);
        }

        assert_eq!(
            output,
            concat!(
                "    %int#0:shape#0(Int) = int.value 1\n",
                "    %float#0:shape#1(Float) = float.value 1.5\n",
                "    %string#0:shape#2(String) = string.value \"a\"\n",
                "    %bool#0:shape#3(Bool) = bool.value True\n",
                "    %nil#0:shape#4(Nil) = nil.value\n",
                "    %tuple#0:shape#5(#(Int)) = tuple.value elements=[%int#0]\n",
                "    %int#1:shape#0(Int) = int.value 2\n",
                "    %int#2:shape#0(Int) = int.add %int#0 %int#1\n",
                "    %int#3:shape#0(Int) = int.negate %int#0\n",
                "    %float#1:shape#1(Float) = float.value 2.0\n",
                "    %float#2:shape#1(Float) = float.add %float#0 %float#1\n",
                "    %string#1:shape#2(String) = string.value \"b\"\n",
                "    %string#2:shape#2(String) = string.concatenate %string#0 %string#1\n",
                "    %bool#1:shape#3(Bool) = bool.not %bool#0\n",
                "    %tuple#1:shape#6(#(Int, Int, Float, String, Bool, Nil, #(Int))) = ",
                "tuple.value elements=[%int#2, %int#3, %float#2, %string#2, %bool#1, ",
                "%nil#0, %tuple#0]\n",
            ),
        );
    }
}
