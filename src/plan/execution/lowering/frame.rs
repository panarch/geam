use super::id::list_function_local;
use crate::plan::module;

pub(super) fn frame_layout(layout: module::FrameLayout) -> super::super::FrameLayout {
    let parts = layout.into_parts();
    let _ = parts.nils;

    super::super::FrameLayout::from_parts(super::super::frame::FrameLayoutParts {
        ints: parts.ints,
        floats: parts.floats,
        strings: parts.strings,
        bools: parts.bools,
        tuples: parts.tuples,
        int_lists: parts.int_lists,
        string_lists: parts.string_lists,
        float_lists: parts.float_lists,
        bool_lists: parts.bool_lists,
        nil_lists: parts.nil_lists,
        tuple_lists: parts.tuple_lists,
        list_lists: parts.list_lists,
        function_lists: parts.function_lists,
        int_functions: parts.int_functions,
        float_functions: parts.float_functions,
        string_functions: parts.string_functions,
        bool_functions: parts.bool_functions,
        nil_functions: parts.nil_functions,
        tuple_functions: parts.tuple_functions,
        list_functions: parts
            .list_functions
            .into_iter()
            .map(list_function_local)
            .collect(),
        function_functions: parts.function_functions,
    })
}

#[cfg(test)]
mod tests {
    use super::super::super::{
        BoolListFunctionLocalId, ExecutionPlan, FloatListFunctionLocalId,
        FunctionListFunctionLocalId, IntFunctionId, IntListFunctionLocalId, ListFunctionLocal,
        ListListFunctionLocalId, NilListFunctionLocalId, StringListFunctionLocalId,
        TupleListFunctionLocalId,
    };
    use crate::plan::{FunctionType, ValueType};

    #[test]
    fn lowering_preserves_every_execution_frame_slot_family() {
        let source = r#"
fn all_slots(
  int: Int,
  float: Float,
  string: String,
  bool: Bool,
  nil: Nil,
  tuple: #(Int),
  int_list: List(Int),
  string_list: List(String),
  float_list: List(Float),
  bool_list: List(Bool),
  nil_list: List(Nil),
  tuple_list: List(#(Int)),
  list_list: List(List(Int)),
  function_list: List(fn() -> Int),
  int_function: fn() -> Int,
  float_function: fn() -> Float,
  string_function: fn() -> String,
  bool_function: fn() -> Bool,
  nil_function: fn() -> Nil,
  tuple_function: fn() -> #(Int),
  int_list_function: fn() -> List(Int),
  string_list_function: fn() -> List(String),
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
        assert_eq!(layout.bools(), 1);
        assert_eq!(layout.tuples(), 1);
        assert_eq!(layout.int_lists(), 1);
        assert_eq!(layout.string_lists(), 1);
        assert_eq!(layout.float_lists(), 1);
        assert_eq!(layout.bool_lists(), 1);
        assert_eq!(layout.nil_lists(), 1);
        assert_eq!(layout.tuple_lists(), &[vec![ValueType::Int]]);
        assert_eq!(layout.list_lists(), &[ValueType::Int]);
        assert_eq!(
            layout.function_lists(),
            &[FunctionType::new(Vec::new(), ValueType::Int)]
        );
        assert_eq!(layout.int_functions(), 1);
        assert_eq!(layout.float_functions(), 1);
        assert_eq!(layout.string_functions(), 1);
        assert_eq!(layout.bool_functions(), 1);
        assert_eq!(layout.nil_functions(), 1);
        assert_eq!(layout.tuple_functions(), 1);
        assert_eq!(layout.function_functions(), 1);

        let int_type = FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Int)));
        let string_type =
            FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::String)));
        let float_type = FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Float)));
        let bool_type = FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Bool)));
        let nil_type = FunctionType::new(Vec::new(), ValueType::List(Box::new(ValueType::Nil)));
        let tuple_type = FunctionType::new(
            Vec::new(),
            ValueType::List(Box::new(ValueType::Tuple(vec![ValueType::Int]))),
        );
        let list_type = FunctionType::new(
            Vec::new(),
            ValueType::List(Box::new(ValueType::List(Box::new(ValueType::Int)))),
        );
        let item_function_type = FunctionType::new(Vec::new(), ValueType::Int);
        let function_type = FunctionType::new(
            Vec::new(),
            ValueType::List(Box::new(ValueType::Function(Box::new(
                item_function_type.clone(),
            )))),
        );
        assert_eq!(
            layout.list_functions(),
            &[
                ListFunctionLocal::Int {
                    local: IntListFunctionLocalId(0),
                    type_: int_type,
                },
                ListFunctionLocal::String {
                    local: StringListFunctionLocalId(1),
                    type_: string_type,
                },
                ListFunctionLocal::Float {
                    local: FloatListFunctionLocalId(2),
                    type_: float_type,
                },
                ListFunctionLocal::Bool {
                    local: BoolListFunctionLocalId(3),
                    type_: bool_type,
                },
                ListFunctionLocal::Nil {
                    local: NilListFunctionLocalId(4),
                    type_: nil_type,
                },
                ListFunctionLocal::Tuple {
                    local: TupleListFunctionLocalId(5),
                    type_: tuple_type,
                    item_type: vec![ValueType::Int],
                },
                ListFunctionLocal::List {
                    local: ListListFunctionLocalId(6),
                    type_: list_type,
                    item_type: Box::new(ValueType::Int),
                },
                ListFunctionLocal::Function {
                    local: FunctionListFunctionLocalId(7),
                    type_: function_type,
                    item_type: Box::new(item_function_type),
                },
            ]
        );
    }
}
