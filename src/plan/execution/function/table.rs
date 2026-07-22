use super::ExecutableFunction;
use super::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayFunctionReturn, BitArrayListFunctionId,
    BitArrayListReturn, BitArrayReturn, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionReturn,
    BoolListFunctionId, BoolListReturn, BoolReturn, CustomFunctionFunctionId, CustomFunctionId,
    CustomFunctionReturn, CustomListFunctionId, CustomListReturn, CustomReturn,
    FloatFunctionFunctionId, FloatFunctionId, FloatFunctionReturn, FloatListFunctionId,
    FloatListReturn, FloatReturn, FunctionFunctionFunctionId, FunctionFunctionReturn,
    FunctionListFunctionId, FunctionListReturn, GenericFunctionFunctionId, GenericFunctionReturn,
    IntFunctionFunctionId, IntFunctionId, IntFunctionReturn, IntListFunctionId, IntListReturn,
    IntReturn, ListFunctionFunctionId, ListFunctionReturn, ListListFunctionId, ListListReturn,
    NeverFunctionFunctionId, NeverFunctionId, NeverFunctionReturn, NeverReturn,
    NilFunctionFunctionId, NilFunctionId, NilFunctionReturn, NilListFunctionId, NilListReturn,
    NilReturn, ParameterListFunctionId, ParameterListListFunctionId, ParameterListListReturn,
    ParameterListReturn, StringFunctionFunctionId, StringFunctionId, StringFunctionReturn,
    StringListFunctionId, StringListReturn, StringReturn, TupleFunctionFunctionId, TupleFunctionId,
    TupleFunctionReturn, TupleListFunctionId, TupleListReturn, TupleReturn,
    UtfCodepointFunctionFunctionId, UtfCodepointFunctionId, UtfCodepointFunctionReturn,
    UtfCodepointListFunctionId, UtfCodepointListReturn, UtfCodepointReturn,
};

pub(in crate::plan::execution) struct FunctionTables {
    pub(in crate::plan::execution) never_functions: Vec<ExecutableFunction<NeverReturn>>,
    pub(in crate::plan::execution) int_functions: Vec<ExecutableFunction<IntReturn>>,
    pub(in crate::plan::execution) float_functions: Vec<ExecutableFunction<FloatReturn>>,
    pub(in crate::plan::execution) string_functions: Vec<ExecutableFunction<StringReturn>>,
    pub(in crate::plan::execution) bit_array_functions: Vec<ExecutableFunction<BitArrayReturn>>,
    pub(in crate::plan::execution) utf_codepoint_functions:
        Vec<ExecutableFunction<UtfCodepointReturn>>,
    pub(in crate::plan::execution) custom_functions: Vec<ExecutableFunction<CustomReturn>>,
    pub(in crate::plan::execution) bool_functions: Vec<ExecutableFunction<BoolReturn>>,
    pub(in crate::plan::execution) nil_functions: Vec<ExecutableFunction<NilReturn>>,
    pub(in crate::plan::execution) tuple_functions: Vec<ExecutableFunction<TupleReturn>>,
    pub(in crate::plan::execution) parameter_list_functions: Vec<(
        ParameterListFunctionId,
        ExecutableFunction<ParameterListReturn>,
    )>,
    pub(in crate::plan::execution) int_list_functions:
        Vec<(IntListFunctionId, ExecutableFunction<IntListReturn>)>,
    pub(in crate::plan::execution) string_list_functions:
        Vec<(StringListFunctionId, ExecutableFunction<StringListReturn>)>,
    pub(in crate::plan::execution) bit_array_list_functions: Vec<(
        BitArrayListFunctionId,
        ExecutableFunction<BitArrayListReturn>,
    )>,
    pub(in crate::plan::execution) utf_codepoint_list_functions: Vec<(
        UtfCodepointListFunctionId,
        ExecutableFunction<UtfCodepointListReturn>,
    )>,
    pub(in crate::plan::execution) custom_list_functions:
        Vec<(CustomListFunctionId, ExecutableFunction<CustomListReturn>)>,
    pub(in crate::plan::execution) float_list_functions:
        Vec<(FloatListFunctionId, ExecutableFunction<FloatListReturn>)>,
    pub(in crate::plan::execution) bool_list_functions:
        Vec<(BoolListFunctionId, ExecutableFunction<BoolListReturn>)>,
    pub(in crate::plan::execution) nil_list_functions:
        Vec<(NilListFunctionId, ExecutableFunction<NilListReturn>)>,
    pub(in crate::plan::execution) tuple_list_functions:
        Vec<(TupleListFunctionId, ExecutableFunction<TupleListReturn>)>,
    pub(in crate::plan::execution) parameter_list_list_functions: Vec<(
        ParameterListListFunctionId,
        ExecutableFunction<ParameterListListReturn>,
    )>,
    pub(in crate::plan::execution) list_list_functions:
        Vec<(ListListFunctionId, ExecutableFunction<ListListReturn>)>,
    pub(in crate::plan::execution) function_list_functions: Vec<(
        FunctionListFunctionId,
        ExecutableFunction<FunctionListReturn>,
    )>,
    pub(in crate::plan::execution) int_function_functions:
        Vec<ExecutableFunction<IntFunctionReturn>>,
    pub(in crate::plan::execution) float_function_functions:
        Vec<ExecutableFunction<FloatFunctionReturn>>,
    pub(in crate::plan::execution) string_function_functions:
        Vec<ExecutableFunction<StringFunctionReturn>>,
    pub(in crate::plan::execution) bit_array_function_functions:
        Vec<ExecutableFunction<BitArrayFunctionReturn>>,
    pub(in crate::plan::execution) utf_codepoint_function_functions:
        Vec<ExecutableFunction<UtfCodepointFunctionReturn>>,
    pub(in crate::plan::execution) custom_function_functions:
        Vec<ExecutableFunction<CustomFunctionReturn>>,
    pub(in crate::plan::execution) bool_function_functions:
        Vec<ExecutableFunction<BoolFunctionReturn>>,
    pub(in crate::plan::execution) nil_function_functions:
        Vec<ExecutableFunction<NilFunctionReturn>>,
    pub(in crate::plan::execution) tuple_function_functions:
        Vec<ExecutableFunction<TupleFunctionReturn>>,
    pub(in crate::plan::execution) generic_function_functions:
        Vec<ExecutableFunction<GenericFunctionReturn>>,
    pub(in crate::plan::execution) never_function_functions:
        Vec<ExecutableFunction<NeverFunctionReturn>>,
    pub(in crate::plan::execution) parameter_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) parameter_list_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) int_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) string_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) bit_array_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) utf_codepoint_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) custom_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) float_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) bool_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) nil_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) tuple_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) list_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) function_list_function_functions:
        Vec<ExecutableFunction<ListFunctionReturn>>,
    pub(in crate::plan::execution) function_function_functions:
        Vec<ExecutableFunction<FunctionFunctionReturn>>,
}

impl FunctionTables {
    pub(in crate::plan::execution) fn never_function(
        &self,
        id: NeverFunctionId,
    ) -> &ExecutableFunction<NeverReturn> {
        &self.never_functions[id.0]
    }

    pub(in crate::plan::execution) fn parameter_list_function(
        &self,
        id: ParameterListFunctionId,
    ) -> &ExecutableFunction<ParameterListReturn> {
        &self.parameter_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn parameter_list_list_function(
        &self,
        id: ParameterListListFunctionId,
    ) -> &ExecutableFunction<ParameterListListReturn> {
        &self.parameter_list_list_functions[id.index()].1
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn parameter_list_function_id(
        &self,
        index: usize,
    ) -> ParameterListFunctionId {
        self.parameter_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn parameter_list_list_function_id(
        &self,
        index: usize,
    ) -> ParameterListListFunctionId {
        self.parameter_list_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn int_list_function_id(
        &self,
        index: usize,
    ) -> IntListFunctionId {
        self.int_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn string_list_function_id(
        &self,
        index: usize,
    ) -> StringListFunctionId {
        self.string_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn bit_array_list_function_id(
        &self,
        index: usize,
    ) -> BitArrayListFunctionId {
        self.bit_array_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn utf_codepoint_list_function_id(
        &self,
        index: usize,
    ) -> UtfCodepointListFunctionId {
        self.utf_codepoint_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn custom_list_function_id(
        &self,
        index: usize,
    ) -> CustomListFunctionId {
        self.custom_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn float_list_function_id(
        &self,
        index: usize,
    ) -> FloatListFunctionId {
        self.float_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn bool_list_function_id(
        &self,
        index: usize,
    ) -> BoolListFunctionId {
        self.bool_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn nil_list_function_id(
        &self,
        index: usize,
    ) -> NilListFunctionId {
        self.nil_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn tuple_list_function_id(
        &self,
        index: usize,
    ) -> TupleListFunctionId {
        self.tuple_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn list_list_function_id(
        &self,
        index: usize,
    ) -> ListListFunctionId {
        self.list_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn function_list_function_id(
        &self,
        index: usize,
    ) -> FunctionListFunctionId {
        self.function_list_functions[index].0
    }

    pub(in crate::plan::execution) fn int_function(
        &self,
        id: IntFunctionId,
    ) -> &ExecutableFunction<IntReturn> {
        &self.int_functions[id.0]
    }

    pub(in crate::plan::execution) fn float_function(
        &self,
        id: FloatFunctionId,
    ) -> &ExecutableFunction<FloatReturn> {
        &self.float_functions[id.0]
    }

    pub(in crate::plan::execution) fn string_function(
        &self,
        id: StringFunctionId,
    ) -> &ExecutableFunction<StringReturn> {
        &self.string_functions[id.0]
    }

    pub(in crate::plan::execution) fn bit_array_function(
        &self,
        id: BitArrayFunctionId,
    ) -> &ExecutableFunction<BitArrayReturn> {
        &self.bit_array_functions[id.0]
    }

    pub(in crate::plan::execution) fn utf_codepoint_function(
        &self,
        id: UtfCodepointFunctionId,
    ) -> &ExecutableFunction<UtfCodepointReturn> {
        &self.utf_codepoint_functions[id.0]
    }

    pub(in crate::plan::execution) fn custom_function(
        &self,
        id: CustomFunctionId,
    ) -> &ExecutableFunction<CustomReturn> {
        &self.custom_functions[id.index()]
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn custom_function_id(&self, index: usize) -> CustomFunctionId {
        CustomFunctionId::new(
            index,
            *self.custom_functions[index].graph().signature_shape(),
        )
    }

    pub(in crate::plan::execution) fn bool_function(
        &self,
        id: BoolFunctionId,
    ) -> &ExecutableFunction<BoolReturn> {
        &self.bool_functions[id.0]
    }

    pub(in crate::plan::execution) fn nil_function(
        &self,
        id: NilFunctionId,
    ) -> &ExecutableFunction<NilReturn> {
        &self.nil_functions[id.0]
    }

    pub(in crate::plan::execution) fn tuple_function(
        &self,
        id: TupleFunctionId,
    ) -> &ExecutableFunction<TupleReturn> {
        &self.tuple_functions[id.0]
    }

    pub(in crate::plan::execution) fn int_list_function(
        &self,
        id: IntListFunctionId,
    ) -> &ExecutableFunction<IntListReturn> {
        &self.int_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn string_list_function(
        &self,
        id: StringListFunctionId,
    ) -> &ExecutableFunction<StringListReturn> {
        &self.string_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn bit_array_list_function(
        &self,
        id: BitArrayListFunctionId,
    ) -> &ExecutableFunction<BitArrayListReturn> {
        &self.bit_array_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn utf_codepoint_list_function(
        &self,
        id: UtfCodepointListFunctionId,
    ) -> &ExecutableFunction<UtfCodepointListReturn> {
        &self.utf_codepoint_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn custom_list_function(
        &self,
        id: CustomListFunctionId,
    ) -> &ExecutableFunction<CustomListReturn> {
        &self.custom_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn float_list_function(
        &self,
        id: FloatListFunctionId,
    ) -> &ExecutableFunction<FloatListReturn> {
        &self.float_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn bool_list_function(
        &self,
        id: BoolListFunctionId,
    ) -> &ExecutableFunction<BoolListReturn> {
        &self.bool_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn nil_list_function(
        &self,
        id: NilListFunctionId,
    ) -> &ExecutableFunction<NilListReturn> {
        &self.nil_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn tuple_list_function(
        &self,
        id: TupleListFunctionId,
    ) -> &ExecutableFunction<TupleListReturn> {
        &self.tuple_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn list_list_function(
        &self,
        id: ListListFunctionId,
    ) -> &ExecutableFunction<ListListReturn> {
        &self.list_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn function_list_function(
        &self,
        id: FunctionListFunctionId,
    ) -> &ExecutableFunction<FunctionListReturn> {
        &self.function_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &ExecutableFunction<IntFunctionReturn> {
        &self.int_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &ExecutableFunction<FloatFunctionReturn> {
        &self.float_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &ExecutableFunction<StringFunctionReturn> {
        &self.string_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn bit_array_function_function(
        &self,
        id: BitArrayFunctionFunctionId,
    ) -> &ExecutableFunction<BitArrayFunctionReturn> {
        &self.bit_array_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn utf_codepoint_function_function(
        &self,
        id: UtfCodepointFunctionFunctionId,
    ) -> &ExecutableFunction<UtfCodepointFunctionReturn> {
        &self.utf_codepoint_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn custom_function_function(
        &self,
        id: &CustomFunctionFunctionId,
    ) -> &ExecutableFunction<CustomFunctionReturn> {
        &self.custom_function_functions[id.index()]
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn function_function_function_id(
        &self,
        index: usize,
    ) -> FunctionFunctionFunctionId {
        FunctionFunctionFunctionId::new(
            index,
            self.function_function_functions[index]
                .graph()
                .type_()
                .clone(),
        )
    }

    pub(in crate::plan::execution) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &ExecutableFunction<BoolFunctionReturn> {
        &self.bool_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &ExecutableFunction<NilFunctionReturn> {
        &self.nil_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &ExecutableFunction<TupleFunctionReturn> {
        &self.tuple_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn generic_function_function(
        &self,
        id: &GenericFunctionFunctionId,
    ) -> &ExecutableFunction<GenericFunctionReturn> {
        &self.generic_function_functions[id.index()]
    }

    pub(in crate::plan::execution) fn never_function_function(
        &self,
        id: &NeverFunctionFunctionId,
    ) -> &ExecutableFunction<NeverFunctionReturn> {
        &self.never_function_functions[id.index()]
    }

    pub(in crate::plan::execution) fn list_function_function(
        &self,
        id: &ListFunctionFunctionId,
    ) -> &ExecutableFunction<ListFunctionReturn> {
        match id {
            ListFunctionFunctionId::Parameter { id, .. } => {
                &self.parameter_list_function_functions[id.0]
            }
            ListFunctionFunctionId::ParameterList { id, .. } => {
                &self.parameter_list_list_function_functions[id.0]
            }
            ListFunctionFunctionId::Int { id, .. } => &self.int_list_function_functions[id.0],
            ListFunctionFunctionId::String { id, .. } => &self.string_list_function_functions[id.0],
            ListFunctionFunctionId::BitArray { id, .. } => {
                &self.bit_array_list_function_functions[id.0]
            }
            ListFunctionFunctionId::UtfCodepoint { id, .. } => {
                &self.utf_codepoint_list_function_functions[id.0]
            }
            ListFunctionFunctionId::Custom { id, .. } => &self.custom_list_function_functions[id.0],
            ListFunctionFunctionId::Float { id, .. } => &self.float_list_function_functions[id.0],
            ListFunctionFunctionId::Bool { id, .. } => &self.bool_list_function_functions[id.0],
            ListFunctionFunctionId::Nil { id, .. } => &self.nil_list_function_functions[id.0],
            ListFunctionFunctionId::Tuple { id, .. } => &self.tuple_list_function_functions[id.0],
            ListFunctionFunctionId::List { id, .. } => &self.list_list_function_functions[id.0],
            ListFunctionFunctionId::Function { id, .. } => {
                &self.function_list_function_functions[id.0]
            }
        }
    }

    pub(in crate::plan::execution) fn function_function_function(
        &self,
        id: &FunctionFunctionFunctionId,
    ) -> &ExecutableFunction<FunctionFunctionReturn> {
        &self.function_function_functions[id.index()]
    }
}

#[cfg(test)]
mod tests {
    use crate::plan::{FunctionType, TypeParameterId, ValueType};
    use crate::runtime::{FunctionValue, Value, run_main};

    #[test]
    fn custom_list_function_function_uses_its_exact_runtime_table() {
        assert_eq!(
            run_main(&execution_plan(
                r#"
pub type Boxed { Boxed(Int) }

fn factory() -> fn() -> List(Boxed) {
  fn() { [Boxed(1)] }
}

pub fn main() {
  let assert [Boxed(value)] = factory()()
  value
}
"#,
            )),
            Ok(Value::Int(1.into())),
        );
    }

    #[test]
    fn list_function_function_tables_dispatch_every_item_family() {
        let cases = [
            (
                "fn factory() -> fn() -> List(value) { fn() { [] } } pub fn main() { factory() }",
                ValueType::Parameter(TypeParameterId(0)),
            ),
            (
                "fn factory() -> fn() -> List(List(value)) { fn() { [] } } pub fn main() { factory() }",
                ValueType::List(Box::new(ValueType::Parameter(TypeParameterId(0)))),
            ),
            (
                "pub fn main() -> fn() -> List(Int) { fn() { [] } }",
                ValueType::Int,
            ),
            (
                "pub fn main() -> fn() -> List(String) { fn() { [] } }",
                ValueType::String,
            ),
            (
                "pub fn main() -> fn() -> List(BitArray) { fn() { [] } }",
                ValueType::BitArray,
            ),
            (
                "pub fn main() -> fn() -> List(UtfCodepoint) { fn() { [] } }",
                ValueType::UtfCodepoint,
            ),
            (
                "pub fn main() -> fn() -> List(Float) { fn() { [] } }",
                ValueType::Float,
            ),
            (
                "pub fn main() -> fn() -> List(Bool) { fn() { [] } }",
                ValueType::Bool,
            ),
            (
                "pub fn main() -> fn() -> List(Nil) { fn() { [] } }",
                ValueType::Nil,
            ),
            (
                "pub fn main() -> fn() -> List(#(Int)) { fn() { [] } }",
                ValueType::Tuple(vec![ValueType::Int]),
            ),
            (
                "pub fn main() -> fn() -> List(List(Int)) { fn() { [] } }",
                ValueType::List(Box::new(ValueType::Int)),
            ),
            (
                "pub fn main() -> fn() -> List(fn() -> Int) { fn() { [] } }",
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ),
        ];

        for (source, item_type) in cases {
            let plan = execution_plan(source);
            let function = expect_function(run_main(&plan).expect("main should return a function"));

            assert_eq!(
                function.type_(),
                FunctionType::new(Vec::new(), ValueType::List(Box::new(item_type))),
            );
        }
    }

    #[test]
    #[should_panic(expected = "expected a function value")]
    fn function_value_fixture_guard_rejects_int_value() {
        let _ = expect_function(Value::Int(0.into()));
    }

    fn expect_function(value: Value) -> FunctionValue {
        match value {
            Value::Function(function) => function,
            _ => panic!("expected a function value"),
        }
    }

    fn execution_plan(source: &str) -> crate::ExecutionPlan {
        let typed = crate::compile_typed_module("main", "main.gleam", source)
            .expect("source should compile");
        let module_plan = crate::plan_module(typed).expect("source should plan");
        crate::ExecutionPlan::from_module_plan(module_plan)
    }
}
