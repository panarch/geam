use super::id::list_function_local;
use crate::plan::module;

pub(super) fn frame_layout(
    layout: module::FrameLayout,
    context: &mut super::LoweringContext,
) -> super::super::FrameLayout {
    let parts = layout.into_parts();
    let _ = parts.nils;

    super::super::FrameLayout::from_slots(super::super::frame::FrameSlots {
        ints: parts.ints,
        floats: parts.floats,
        strings: parts.strings,
        bit_arrays: parts.bit_arrays,
        bools: parts.bools,
        tuples: parts.tuples,
        int_lists: (0..parts.int_lists)
            .map(|_| context.int_list_type())
            .collect(),
        string_lists: (0..parts.string_lists)
            .map(|_| context.string_list_type())
            .collect(),
        bit_array_lists: (0..parts.bit_array_lists)
            .map(|_| context.bit_array_list_type())
            .collect(),
        float_lists: (0..parts.float_lists)
            .map(|_| context.float_list_type())
            .collect(),
        bool_lists: (0..parts.bool_lists)
            .map(|_| context.bool_list_type())
            .collect(),
        nil_lists: (0..parts.nil_lists)
            .map(|_| context.nil_list_type())
            .collect(),
        tuple_lists: parts
            .tuple_lists
            .into_iter()
            .map(|item| context.tuple_list_type(item))
            .collect(),
        list_lists: parts
            .list_lists
            .into_iter()
            .map(|item| context.list_list_type(item))
            .collect(),
        function_lists: parts
            .function_lists
            .into_iter()
            .map(|item| context.function_list_type(item))
            .collect(),
        int_functions: parts.int_functions,
        float_functions: parts.float_functions,
        string_functions: parts.string_functions,
        bit_array_functions: parts.bit_array_functions,
        bool_functions: parts.bool_functions,
        nil_functions: parts.nil_functions,
        tuple_functions: parts.tuple_functions,
        list_functions: parts
            .list_functions
            .into_iter()
            .map(|local| list_function_local(local, context))
            .collect(),
        function_functions: parts.function_functions,
    })
}

#[cfg(test)]
mod tests {
    use super::super::super::{ExecutionPlan, IntFunctionId};
    use crate::plan::{FunctionType, ValueType};

    #[test]
    fn lowering_preserves_every_execution_frame_slot_family() {
        let source = r#"
fn all_slots(
  int: Int,
  float: Float,
  string: String,
  bit_array: BitArray,
  bool: Bool,
  nil: Nil,
  tuple: #(Int),
  int_list: List(Int),
  string_list: List(String),
  bit_array_list: List(BitArray),
  float_list: List(Float),
  bool_list: List(Bool),
  nil_list: List(Nil),
  tuple_list: List(#(Int)),
  list_list: List(List(Int)),
  function_list: List(fn() -> Int),
  int_function: fn() -> Int,
  float_function: fn() -> Float,
  string_function: fn() -> String,
  bit_array_function: fn() -> BitArray,
  bool_function: fn() -> Bool,
  nil_function: fn() -> Nil,
  tuple_function: fn() -> #(Int),
  int_list_function: fn() -> List(Int),
  string_list_function: fn() -> List(String),
  bit_array_list_function: fn() -> List(BitArray),
  float_list_function: fn() -> List(Float),
  bool_list_function: fn() -> List(Bool),
  nil_list_function: fn() -> List(Nil),
  tuple_list_function: fn() -> List(#(Int)),
  list_list_function: fn() -> List(List(Int)),
  function_list_function: fn() -> List(fn() -> Int),
  function_function: fn() -> fn() -> Int,
) {
  int
}

pub fn main() { 0 }
"#;
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        let plan = ExecutionPlan::from_module_plan(module_plan);
        let layout = plan.int_function(IntFunctionId(1)).frame_layout();

        assert_eq!(layout.ints(), 1);
        assert_eq!(layout.floats(), 1);
        assert_eq!(layout.strings(), 1);
        assert_eq!(layout.bit_arrays(), 1);
        assert_eq!(layout.bools(), 1);
        assert_eq!(layout.tuples(), 1);
        assert_eq!(layout.int_lists().len(), 1);
        assert_eq!(layout.string_lists().len(), 1);
        assert_eq!(layout.bit_array_lists().len(), 1);
        assert_eq!(layout.float_lists().len(), 1);
        assert_eq!(layout.bool_lists().len(), 1);
        assert_eq!(layout.nil_lists().len(), 1);
        assert_eq!(layout.tuple_lists().len(), 1);
        assert_eq!(layout.list_lists().len(), 1);
        assert_eq!(layout.function_lists().len(), 1);
        assert_eq!(
            plan.tuple_list_item_type(layout.tuple_lists()[0]),
            vec![ValueType::Int]
        );
        assert_eq!(
            plan.nested_list_item_type(layout.list_lists()[0]),
            ValueType::Int
        );
        assert_eq!(
            plan.function_list_item_type(layout.function_lists()[0]),
            FunctionType::new(Vec::new(), ValueType::Int)
        );
        assert_eq!(layout.int_functions(), 1);
        assert_eq!(layout.float_functions(), 1);
        assert_eq!(layout.string_functions(), 1);
        assert_eq!(layout.bit_array_functions(), 1);
        assert_eq!(layout.bool_functions(), 1);
        assert_eq!(layout.nil_functions(), 1);
        assert_eq!(layout.tuple_functions(), 1);
        assert_eq!(layout.function_functions(), 1);

        let item_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let expected_returns = [
            ValueType::List(Box::new(ValueType::Int)),
            ValueType::List(Box::new(ValueType::String)),
            ValueType::List(Box::new(ValueType::BitArray)),
            ValueType::List(Box::new(ValueType::Float)),
            ValueType::List(Box::new(ValueType::Bool)),
            ValueType::List(Box::new(ValueType::Nil)),
            ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
            ValueType::List(Box::new(ValueType::Function(Box::new(item_function_type)))),
        ];
        let list_functions = layout.list_functions();
        assert_eq!(list_functions.len(), expected_returns.len());
        for (local, expected_return) in list_functions.iter().zip(expected_returns) {
            assert_eq!(
                plan.function_type(local.type_()),
                FunctionType::new(Vec::new(), expected_return.clone())
            );
            assert_eq!(plan.list_value_type(local.list_type()), expected_return);
        }
    }
}
