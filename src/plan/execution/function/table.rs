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
use super::{ExecutableFunction, FunctionFunctionTables, ListFunctionTables, ValueFunctionTables};

pub(in crate::plan::execution) struct FunctionTables {
    pub(in crate::plan::execution) value_returns: ValueFunctionTables,
    pub(in crate::plan::execution) list_returns: ListFunctionTables,
    pub(in crate::plan::execution) function_returns: FunctionFunctionTables,
}

impl FunctionTables {
    pub(in crate::plan::execution) fn never_function(
        &self,
        id: NeverFunctionId,
    ) -> &ExecutableFunction<NeverReturn> {
        &self.value_returns.never_functions[id.0]
    }

    pub(in crate::plan::execution) fn parameter_list_function(
        &self,
        id: ParameterListFunctionId,
    ) -> &ExecutableFunction<ParameterListReturn> {
        &self.list_returns.parameter_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn parameter_list_list_function(
        &self,
        id: ParameterListListFunctionId,
    ) -> &ExecutableFunction<ParameterListListReturn> {
        &self.list_returns.parameter_list_list_functions[id.index()].1
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn parameter_list_function_id(
        &self,
        index: usize,
    ) -> ParameterListFunctionId {
        self.list_returns.parameter_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn parameter_list_list_function_id(
        &self,
        index: usize,
    ) -> ParameterListListFunctionId {
        self.list_returns.parameter_list_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn int_list_function_id(
        &self,
        index: usize,
    ) -> IntListFunctionId {
        self.list_returns.int_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn string_list_function_id(
        &self,
        index: usize,
    ) -> StringListFunctionId {
        self.list_returns.string_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn bit_array_list_function_id(
        &self,
        index: usize,
    ) -> BitArrayListFunctionId {
        self.list_returns.bit_array_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn utf_codepoint_list_function_id(
        &self,
        index: usize,
    ) -> UtfCodepointListFunctionId {
        self.list_returns.utf_codepoint_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn custom_list_function_id(
        &self,
        index: usize,
    ) -> CustomListFunctionId {
        self.list_returns.custom_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn float_list_function_id(
        &self,
        index: usize,
    ) -> FloatListFunctionId {
        self.list_returns.float_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn bool_list_function_id(
        &self,
        index: usize,
    ) -> BoolListFunctionId {
        self.list_returns.bool_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn nil_list_function_id(
        &self,
        index: usize,
    ) -> NilListFunctionId {
        self.list_returns.nil_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn tuple_list_function_id(
        &self,
        index: usize,
    ) -> TupleListFunctionId {
        self.list_returns.tuple_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn list_list_function_id(
        &self,
        index: usize,
    ) -> ListListFunctionId {
        self.list_returns.list_list_functions[index].0
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn function_list_function_id(
        &self,
        index: usize,
    ) -> FunctionListFunctionId {
        self.list_returns.function_list_functions[index].0
    }

    pub(in crate::plan::execution) fn int_function(
        &self,
        id: IntFunctionId,
    ) -> &ExecutableFunction<IntReturn> {
        &self.value_returns.int_functions[id.0]
    }

    pub(in crate::plan::execution) fn float_function(
        &self,
        id: FloatFunctionId,
    ) -> &ExecutableFunction<FloatReturn> {
        &self.value_returns.float_functions[id.0]
    }

    pub(in crate::plan::execution) fn string_function(
        &self,
        id: StringFunctionId,
    ) -> &ExecutableFunction<StringReturn> {
        &self.value_returns.string_functions[id.0]
    }

    pub(in crate::plan::execution) fn bit_array_function(
        &self,
        id: BitArrayFunctionId,
    ) -> &ExecutableFunction<BitArrayReturn> {
        &self.value_returns.bit_array_functions[id.0]
    }

    pub(in crate::plan::execution) fn utf_codepoint_function(
        &self,
        id: UtfCodepointFunctionId,
    ) -> &ExecutableFunction<UtfCodepointReturn> {
        &self.value_returns.utf_codepoint_functions[id.0]
    }

    pub(in crate::plan::execution) fn custom_function(
        &self,
        id: CustomFunctionId,
    ) -> &ExecutableFunction<CustomReturn> {
        &self.value_returns.custom_functions[id.index()]
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn custom_function_id(&self, index: usize) -> CustomFunctionId {
        CustomFunctionId::new(
            index,
            *self.value_returns.custom_functions[index]
                .graph()
                .signature_shape(),
        )
    }

    pub(in crate::plan::execution) fn bool_function(
        &self,
        id: BoolFunctionId,
    ) -> &ExecutableFunction<BoolReturn> {
        &self.value_returns.bool_functions[id.0]
    }

    pub(in crate::plan::execution) fn nil_function(
        &self,
        id: NilFunctionId,
    ) -> &ExecutableFunction<NilReturn> {
        &self.value_returns.nil_functions[id.0]
    }

    pub(in crate::plan::execution) fn tuple_function(
        &self,
        id: TupleFunctionId,
    ) -> &ExecutableFunction<TupleReturn> {
        &self.value_returns.tuple_functions[id.0]
    }

    pub(in crate::plan::execution) fn int_list_function(
        &self,
        id: IntListFunctionId,
    ) -> &ExecutableFunction<IntListReturn> {
        &self.list_returns.int_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn string_list_function(
        &self,
        id: StringListFunctionId,
    ) -> &ExecutableFunction<StringListReturn> {
        &self.list_returns.string_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn bit_array_list_function(
        &self,
        id: BitArrayListFunctionId,
    ) -> &ExecutableFunction<BitArrayListReturn> {
        &self.list_returns.bit_array_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn utf_codepoint_list_function(
        &self,
        id: UtfCodepointListFunctionId,
    ) -> &ExecutableFunction<UtfCodepointListReturn> {
        &self.list_returns.utf_codepoint_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn custom_list_function(
        &self,
        id: CustomListFunctionId,
    ) -> &ExecutableFunction<CustomListReturn> {
        &self.list_returns.custom_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn float_list_function(
        &self,
        id: FloatListFunctionId,
    ) -> &ExecutableFunction<FloatListReturn> {
        &self.list_returns.float_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn bool_list_function(
        &self,
        id: BoolListFunctionId,
    ) -> &ExecutableFunction<BoolListReturn> {
        &self.list_returns.bool_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn nil_list_function(
        &self,
        id: NilListFunctionId,
    ) -> &ExecutableFunction<NilListReturn> {
        &self.list_returns.nil_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn tuple_list_function(
        &self,
        id: TupleListFunctionId,
    ) -> &ExecutableFunction<TupleListReturn> {
        &self.list_returns.tuple_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn list_list_function(
        &self,
        id: ListListFunctionId,
    ) -> &ExecutableFunction<ListListReturn> {
        &self.list_returns.list_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn function_list_function(
        &self,
        id: FunctionListFunctionId,
    ) -> &ExecutableFunction<FunctionListReturn> {
        &self.list_returns.function_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &ExecutableFunction<IntFunctionReturn> {
        &self.function_returns.int_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &ExecutableFunction<FloatFunctionReturn> {
        &self.function_returns.float_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &ExecutableFunction<StringFunctionReturn> {
        &self.function_returns.string_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn bit_array_function_function(
        &self,
        id: BitArrayFunctionFunctionId,
    ) -> &ExecutableFunction<BitArrayFunctionReturn> {
        &self.function_returns.bit_array_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn utf_codepoint_function_function(
        &self,
        id: UtfCodepointFunctionFunctionId,
    ) -> &ExecutableFunction<UtfCodepointFunctionReturn> {
        &self.function_returns.utf_codepoint_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn custom_function_function(
        &self,
        id: &CustomFunctionFunctionId,
    ) -> &ExecutableFunction<CustomFunctionReturn> {
        &self.function_returns.custom_function_functions[id.index()]
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn function_function_function_id(
        &self,
        index: usize,
    ) -> FunctionFunctionFunctionId {
        FunctionFunctionFunctionId::new(
            index,
            self.function_returns.function_function_functions[index]
                .graph()
                .type_()
                .clone(),
        )
    }

    pub(in crate::plan::execution) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &ExecutableFunction<BoolFunctionReturn> {
        &self.function_returns.bool_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &ExecutableFunction<NilFunctionReturn> {
        &self.function_returns.nil_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &ExecutableFunction<TupleFunctionReturn> {
        &self.function_returns.tuple_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn generic_function_function(
        &self,
        id: &GenericFunctionFunctionId,
    ) -> &ExecutableFunction<GenericFunctionReturn> {
        &self.function_returns.generic_function_functions[id.index()]
    }

    pub(in crate::plan::execution) fn never_function_function(
        &self,
        id: &NeverFunctionFunctionId,
    ) -> &ExecutableFunction<NeverFunctionReturn> {
        &self.function_returns.never_function_functions[id.index()]
    }

    pub(in crate::plan::execution) fn list_function_function(
        &self,
        id: &ListFunctionFunctionId,
    ) -> &ExecutableFunction<ListFunctionReturn> {
        match id {
            ListFunctionFunctionId::Parameter { id, .. } => {
                &self.function_returns.parameter_list_function_functions[id.0]
            }
            ListFunctionFunctionId::ParameterList { id, .. } => {
                &self.function_returns.parameter_list_list_function_functions[id.0]
            }
            ListFunctionFunctionId::Int { id, .. } => {
                &self.function_returns.int_list_function_functions[id.0]
            }
            ListFunctionFunctionId::String { id, .. } => {
                &self.function_returns.string_list_function_functions[id.0]
            }
            ListFunctionFunctionId::BitArray { id, .. } => {
                &self.function_returns.bit_array_list_function_functions[id.0]
            }
            ListFunctionFunctionId::UtfCodepoint { id, .. } => {
                &self.function_returns.utf_codepoint_list_function_functions[id.0]
            }
            ListFunctionFunctionId::Custom { id, .. } => {
                &self.function_returns.custom_list_function_functions[id.0]
            }
            ListFunctionFunctionId::Float { id, .. } => {
                &self.function_returns.float_list_function_functions[id.0]
            }
            ListFunctionFunctionId::Bool { id, .. } => {
                &self.function_returns.bool_list_function_functions[id.0]
            }
            ListFunctionFunctionId::Nil { id, .. } => {
                &self.function_returns.nil_list_function_functions[id.0]
            }
            ListFunctionFunctionId::Tuple { id, .. } => {
                &self.function_returns.tuple_list_function_functions[id.0]
            }
            ListFunctionFunctionId::List { id, .. } => {
                &self.function_returns.list_list_function_functions[id.0]
            }
            ListFunctionFunctionId::Function { id, .. } => {
                &self.function_returns.function_list_function_functions[id.0]
            }
        }
    }

    pub(in crate::plan::execution) fn function_function_function(
        &self,
        id: &FunctionFunctionFunctionId,
    ) -> &ExecutableFunction<FunctionFunctionReturn> {
        &self.function_returns.function_function_functions[id.index()]
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
