mod function_return;
mod list_return;
mod table;
mod value_return;

use self::function_return::write_function_return_tables;
use self::list_return::write_list_return_tables;
use self::value_return::write_value_return_tables;
use super::super::{ExecutionPlan, FunctionTables};

pub(super) fn write_function_tables(
    output: &mut String,
    plan: &ExecutionPlan,
    functions: &FunctionTables,
) {
    write_value_return_tables(output, plan, functions);
    write_list_return_tables(output, plan, functions);
    write_function_return_tables(output, plan, functions);
}

#[cfg(test)]
mod tests {
    #[test]
    fn writes_value_list_and_function_return_groups_in_order() {
        let source = r#"
fn value() -> Int { 1 }
fn values(value: Int) -> List(Int) {
  {
    case value {
      0 -> []
      _ -> []
    }
  }
}
fn callable() -> fn() -> Int { fn() { 1 } }

pub fn main() {
  let provider = values
  let _ = #(value(), provider(0), callable())
  0
}
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = crate::ExecutionPlan::from_module_plan(module_plan);
        let mut output = String::new();

        super::write_function_tables(&mut output, &plan, &plan.functions);

        assert_eq!(
            output,
            concat!(
                "\nfunction int#0\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %function.list.int#0:shape#2(fn(Int) -> list_type#0) = ",
                "function[List] reference list.int#0\n",
                "    %int#0:shape#0(Int) = int.call int#1 args=[]\n",
                "    %int#1:shape#0(Int) = int.value 0\n",
                "    %list.int#0:shape#1(list_type#0) = list.int[type#0] function_call ",
                "%function.list.int#0 args=[%int#1]\n",
                "    %function.int#0:shape#3(fn() -> Int) = function[Int] call ",
                "function.int#0 args=[]\n",
                "    %tuple#0:shape#4(#(Int, list_type#0, fn() -> Int)) = tuple.value ",
                "elements=[%int#0, %list.int#0, %function.int#0]\n",
                "    %int#2:shape#0(Int) = int.value 0\n",
                "    return %int#2\n",
                "\nfunction int#1\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %int#0:shape#0(Int) = int.value 1\n",
                "    return %int#0\n",
                "\nfunction int#2\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %int#0:shape#0(Int) = int.value 1\n",
                "    return %int#0\n",
                "\nfunction list.int#0\n",
                "  entry b0 params=[%int#0:shape#0(Int)] captures=[]\n",
                "  block b0 params=[%int#0:shape#0(Int)]\n",
                "    switch.int %int#0 clauses=[0->b1()] fallback=b2()\n",
                "  block b1 params=[]\n",
                "    %list.int#0:shape#1(list_type#0) = list.int[type#0] value ",
                "elements=[]\n",
                "    return %list.int#0\n",
                "  block b2 params=[]\n",
                "    %list.int#0:shape#1(list_type#0) = list.int[type#0] value ",
                "elements=[]\n",
                "    return %list.int#0\n",
                "\nfunction function.int#0\n",
                "  entry b0 params=[] captures=[]\n",
                "  block b0 params=[]\n",
                "    %function.int#0:shape#3(fn() -> Int) = function[Int] closure ",
                "target=int#2 captures=[]\n",
                "    return %function.int#0\n",
            ),
        );
    }
}
