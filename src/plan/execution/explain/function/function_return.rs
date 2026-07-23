use super::super::super::{ExecutionPlan, FunctionTables};
use super::table::write_table;

pub(super) fn write_function_return_tables(
    output: &mut String,
    plan: &ExecutionPlan,
    functions: &FunctionTables,
) {
    write_table(
        output,
        plan,
        "function.int",
        &functions.int_function_functions,
    );
    write_table(
        output,
        plan,
        "function.float",
        &functions.float_function_functions,
    );
    write_table(
        output,
        plan,
        "function.string",
        &functions.string_function_functions,
    );
    write_table(
        output,
        plan,
        "function.bit_array",
        &functions.bit_array_function_functions,
    );
    write_table(
        output,
        plan,
        "function.utf_codepoint",
        &functions.utf_codepoint_function_functions,
    );
    write_table(
        output,
        plan,
        "function.custom",
        &functions.custom_function_functions,
    );
    write_table(
        output,
        plan,
        "function.bool",
        &functions.bool_function_functions,
    );
    write_table(
        output,
        plan,
        "function.nil",
        &functions.nil_function_functions,
    );
    write_table(
        output,
        plan,
        "function.tuple",
        &functions.tuple_function_functions,
    );
    write_table(
        output,
        plan,
        "function.generic",
        &functions.generic_function_functions,
    );
    write_table(
        output,
        plan,
        "function.never",
        &functions.never_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.parameter",
        &functions.parameter_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.parameter_list",
        &functions.parameter_list_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.int",
        &functions.int_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.string",
        &functions.string_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.bit_array",
        &functions.bit_array_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.utf_codepoint",
        &functions.utf_codepoint_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.custom",
        &functions.custom_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.float",
        &functions.float_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.bool",
        &functions.bool_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.nil",
        &functions.nil_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.tuple",
        &functions.tuple_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.list",
        &functions.list_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.list.function",
        &functions.function_list_function_functions,
    );
    write_table(
        output,
        plan,
        "function.function",
        &functions.function_function_functions,
    );
}

#[cfg(test)]
mod tests {
    #[test]
    fn writes_function_return_families_in_storage_order() {
        assert_explanation(
            r#"
fn int_function() -> fn() -> Int { fn() { 1 } }
fn first_float() -> Float { 1.0 }
fn second_float() -> Float { 2.0 }
fn float_function(value: Int) -> fn() -> Float {
  {
    case value {
      0 -> first_float
      _ -> second_float
    }
  }
}
fn string_value() -> String { "value" }
fn string_function() -> fn() -> String {
  {
    let selected = string_value
    selected
  }
}
fn first_list() -> List(Int) { [] }
fn second_list() -> List(Int) { [] }
fn list_function(value: Int) -> fn() -> List(Int) {
  {
    case value {
      0 -> first_list
      _ -> second_list
    }
  }
}
fn first_tuple() -> #(Int) { #(1) }
fn second_tuple() -> #(Int) { #(2) }
fn tuple_function(value: Int) -> fn() -> #(Int) {
  {
    case value {
      0 -> first_tuple
      _ -> second_tuple
    }
  }
}
pub fn main() -> fn() -> Bool {
  let _ = #(
    int_function(),
    float_function(0),
    string_function(),
    list_function(0),
    tuple_function(0),
  )
  fn() { True }
}
"#,
            concat!(
                "\nfunction function.int#0\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %function.int#0:shape#1(fn() -> Int) = function[Int] closure ",
                "target=int#0 captures=[]\n",
                "    return %function.int#0\n",
                "\nfunction function.float#0\n",
                "  entry b0 params=[%int#0:shape#0(Int)] captures=[]\n",
                "  block b0 params=[%int#0:shape#0(Int)]\n",
                "    switch.int %int#0 clauses=[0->b1()] fallback=b2()\n",
                "  block b1 params=[]\n",
                "    %function.float#0:shape#3(fn() -> Float) = function[Float] reference ",
                "float#0\n",
                "    return %function.float#0\n",
                "  block b2 params=[]\n",
                "    %function.float#0:shape#3(fn() -> Float) = function[Float] reference ",
                "float#1\n",
                "    return %function.float#0\n",
                "\nfunction function.string#0\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %function.string#0:shape#5(fn() -> String) = function[String] reference ",
                "string#0\n",
                "    return %function.string#0\n",
                "\nfunction function.bool#0\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %function.int#0:shape#1(fn() -> Int) = function[Int] call ",
                "function.int#0 args=[]\n",
                "    %int#0:shape#0(Int) = int.value 0\n",
                "    %function.float#0:shape#3(fn() -> Float) = function[Float] call ",
                "function.float#0 args=[%int#0]\n",
                "    %function.string#0:shape#5(fn() -> String) = function[String] call ",
                "function.string#0 args=[]\n",
                "    %int#1:shape#0(Int) = int.value 0\n",
                "    %function.list.int#0:shape#7(fn() -> list_type#0) = function[List] ",
                "call function.list.int#0 args=[%int#1]\n",
                "    %int#2:shape#0(Int) = int.value 0\n",
                "    %function.tuple#0:shape#9(fn() -> #(Int)) = function[Tuple] call ",
                "function.tuple#0 args=[%int#2]\n",
                "    %tuple#0:shape#10(#(fn() -> Int, fn() -> Float, fn() -> String, ",
                "fn() -> list_type#0, fn() -> #(Int))) = tuple.value elements=[%function.int#0, ",
                "%function.float#0, %function.string#0, %function.list.int#0, ",
                "%function.tuple#0]\n",
                "    %function.bool#0:shape#12(fn() -> Bool) = function[Bool] closure ",
                "target=bool#0 captures=[]\n",
                "    return %function.bool#0\n",
                "\nfunction function.tuple#0\n",
                "  entry b0 params=[%int#0:shape#0(Int)] captures=[]\n",
                "  block b0 params=[%int#0:shape#0(Int)]\n",
                "    switch.int %int#0 clauses=[0->b1()] fallback=b2()\n",
                "  block b1 params=[]\n",
                "    %function.tuple#0:shape#9(fn() -> #(Int)) = function[Tuple] reference ",
                "tuple#0\n",
                "    return %function.tuple#0\n",
                "  block b2 params=[]\n",
                "    %function.tuple#0:shape#9(fn() -> #(Int)) = function[Tuple] reference ",
                "tuple#1\n",
                "    return %function.tuple#0\n",
                "\nfunction function.list.int#0\n",
                "  entry b0 params=[%int#0:shape#0(Int)] captures=[]\n",
                "  block b0 params=[%int#0:shape#0(Int)]\n",
                "    switch.int %int#0 clauses=[0->b1()] fallback=b2()\n",
                "  block b1 params=[]\n",
                "    %function.list.int#0:shape#7(fn() -> list_type#0) = function[List] ",
                "reference list.int#0\n",
                "    return %function.list.int#0\n",
                "  block b2 params=[]\n",
                "    %function.list.int#0:shape#7(fn() -> list_type#0) = function[List] ",
                "reference list.int#1\n",
                "    return %function.list.int#0\n",
            ),
        );
    }

    #[test]
    fn writes_function_return_call_and_block_control_flow() {
        assert_explanation(
            r#"
fn bit_array_value() -> BitArray { <<>> }
fn bit_array_block() -> fn() -> BitArray {
  {
    let selected = bit_array_value
    selected
  }
}
fn bit_array_tail() -> fn() -> BitArray { bit_array_tail() }

fn utf_codepoint_value(value: UtfCodepoint) -> UtfCodepoint { value }
fn utf_codepoint_block() -> fn(UtfCodepoint) -> UtfCodepoint {
  {
    let selected = utf_codepoint_value
    selected
  }
}
fn utf_codepoint_tail() -> fn(UtfCodepoint) -> UtfCodepoint {
  utf_codepoint_tail()
}

fn float_tail() -> fn() -> Float { float_tail() }

fn bool_value() -> Bool { True }
fn bool_block() -> fn() -> Bool {
  {
    let selected = bool_value
    selected
  }
}

fn nil_value() -> Nil { Nil }

pub fn main() -> fn() -> Nil {
  let _ = #(
    bit_array_block(),
    bit_array_tail,
    utf_codepoint_block(),
    utf_codepoint_tail,
    float_tail,
    bool_block(),
  )
  {
    let selected = nil_value
    selected
  }
}
"#,
            concat!(
                "\nfunction function.float#0\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    tail function.float#0 args=[]\n",
                "\nfunction function.bit_array#0\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %function.bit_array#0:shape#1(fn() -> BitArray) = function[BitArray] ",
                "reference bit_array#0\n",
                "    return %function.bit_array#0\n",
                "\nfunction function.bit_array#1\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    tail function.bit_array#1 args=[]\n",
                "\nfunction function.utf_codepoint#0\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %function.utf_codepoint#0:shape#3(fn(UtfCodepoint) -> UtfCodepoint) = ",
                "function[UtfCodepoint] reference utf_codepoint#0\n",
                "    return %function.utf_codepoint#0\n",
                "\nfunction function.utf_codepoint#1\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    tail function.utf_codepoint#1 args=[]\n",
                "\nfunction function.bool#0\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %function.bool#0:shape#10(fn() -> Bool) = function[Bool] reference bool#0\n",
                "    return %function.bool#0\n",
                "\nfunction function.nil#0\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %function.bit_array#0:shape#1(fn() -> BitArray) = function[BitArray] ",
                "call function.bit_array#0 args=[]\n",
                "    %function.function#0:shape#6(fn() -> fn() -> BitArray) = ",
                "function[Function] reference function.bit_array#1\n",
                "    %function.utf_codepoint#0:shape#3(fn(UtfCodepoint) -> UtfCodepoint) = ",
                "function[UtfCodepoint] call function.utf_codepoint#0 args=[]\n",
                "    %function.function#1:shape#7(fn() -> fn(UtfCodepoint) -> UtfCodepoint) = ",
                "function[Function] reference function.utf_codepoint#1\n",
                "    %function.function#2:shape#8(fn() -> fn() -> Float) = ",
                "function[Function] reference function.float#0\n",
                "    %function.bool#0:shape#10(fn() -> Bool) = function[Bool] call ",
                "function.bool#0 args=[]\n",
                "    %tuple#0:shape#11(#(fn() -> BitArray, fn() -> fn() -> BitArray, ",
                "fn(UtfCodepoint) -> UtfCodepoint, fn() -> fn(UtfCodepoint) -> UtfCodepoint, ",
                "fn() -> fn() -> Float, fn() -> Bool)) = tuple.value ",
                "elements=[%function.bit_array#0, %function.function#0, ",
                "%function.utf_codepoint#0, %function.function#1, %function.function#2, ",
                "%function.bool#0]\n",
                "    %function.nil#0:shape#13(fn() -> Nil) = function[Nil] reference nil#0\n",
                "    return %function.nil#0\n",
            ),
        );
    }

    fn assert_explanation(source: &str, expected: &str) {
        super::super::super::assert_rendered(source, expected, |plan, output| {
            super::write_function_return_tables(output, plan, &plan.functions);
        });
    }
}
