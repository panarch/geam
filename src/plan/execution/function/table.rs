mod host;

use super::{
    BitArrayFunctionBody, BitArrayFunctionFunctionBody, BitArrayFunctionFunctionId,
    BitArrayFunctionId, BitArrayListFunctionBody, BitArrayListFunctionId, BoolFunctionBody,
    BoolFunctionFunctionBody, BoolFunctionFunctionId, BoolFunctionId, BoolListFunctionBody,
    BoolListFunctionId, CustomFunctionBody, CustomFunctionFunctionBody, CustomFunctionFunctionId,
    CustomFunctionId, CustomListFunctionBody, CustomListFunctionId, FloatFunctionBody,
    FloatFunctionFunctionBody, FloatFunctionFunctionId, FloatFunctionId, FloatListFunctionBody,
    FloatListFunctionId, FunctionFunctionFunctionBody, FunctionFunctionFunctionId,
    FunctionListFunctionBody, FunctionListFunctionId, GenericFunctionFunctionBody,
    GenericFunctionFunctionId, IntFunctionBody, IntFunctionFunctionBody, IntFunctionFunctionId,
    IntFunctionId, IntListFunctionBody, IntListFunctionId, ListFunctionFunctionBody,
    ListFunctionFunctionId, ListListFunctionBody, ListListFunctionId, NeverFunctionBody,
    NeverFunctionFunctionBody, NeverFunctionFunctionId, NeverFunctionId, NilFunctionBody,
    NilFunctionFunctionBody, NilFunctionFunctionId, NilFunctionId, NilListFunctionBody,
    NilListFunctionId, ParameterListFunctionBody, ParameterListFunctionId,
    ParameterListListFunctionBody, ParameterListListFunctionId, StringFunctionBody,
    StringFunctionFunctionBody, StringFunctionFunctionId, StringFunctionId, StringListFunctionBody,
    StringListFunctionId, TupleFunctionBody, TupleFunctionFunctionBody, TupleFunctionFunctionId,
    TupleFunctionId, TupleListFunctionBody, TupleListFunctionId, UtfCodepointFunctionBody,
    UtfCodepointFunctionFunctionBody, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
    UtfCodepointListFunctionBody, UtfCodepointListFunctionId,
};
use super::{ExecutableFunction, FunctionFunctionTables, ListFunctionTables, ValueFunctionTables};
use crate::plan::execution::explain::{Explain, ExplainContext, FunctionLabel};
use crate::plan::execution::function::{FunctionBodyOwner, TailCallLabelIndex};
use crate::plan::execution::graph::LocalLabel;

pub(in crate::plan::execution) use host::HostedFunctionTablesExplanation;

pub(in crate::plan::execution) struct FunctionTables<IntFunction, BoolFunction> {
    pub(in crate::plan::execution) value_returns: ValueFunctionTables<IntFunction, BoolFunction>,
    pub(in crate::plan::execution) list_returns: ListFunctionTables,
    pub(in crate::plan::execution) function_returns: FunctionFunctionTables,
}

impl Explain
    for FunctionTables<ExecutableFunction<IntFunctionBody>, ExecutableFunction<BoolFunctionBody>>
{
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.write(&self.value_returns);
        context.write(&self.list_returns);
        context.write(&self.function_returns);
    }
}

pub(in crate::plan::execution::function) fn write_table<'a, Body, Functions>(
    context: &mut ExplainContext<'_, '_>,
    family: &'static str,
    functions: Functions,
) where
    Body: FunctionBodyOwner + 'a,
    Body::Return: LocalLabel,
    Body::TailCall: TailCallLabelIndex,
    Functions: IntoIterator<Item = &'a ExecutableFunction<Body>>,
{
    for (index, function) in functions.into_iter().enumerate() {
        write_function(context, family, index, function);
    }
}

fn write_function<Body>(
    context: &mut ExplainContext<'_, '_>,
    family: &'static str,
    index: usize,
    function: &ExecutableFunction<Body>,
) where
    Body: FunctionBodyOwner,
    Body::Return: LocalLabel,
    Body::TailCall: TailCallLabelIndex,
{
    context.push_str("\nfunction ");
    FunctionLabel::new(family, index).write(context.output());
    context.push('\n');
    let body = function.body().function_body();
    body.write_explanation(
        context,
        family,
        function.entry().params(body),
        function.entry().captures(body),
    );
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;

    #[test]
    fn writes_value_list_and_function_return_groups_in_order() {
        let source = r#"
fn ints() -> List(Int) { [] }
fn callable() -> fn() -> Int { fn() { 1 } }

pub fn main() {
  let _ = #(ints(), callable())
  0
}
"#;
        let expected = concat!(
            "\nfunction int#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %list.int#0:shape#1(list_type#0) = list.int[type#0] call ",
            "list.int#0 args=[]\n",
            "    %function.int#0:shape#2(fn() -> Int) = function[Int] call ",
            "function.int#0 args=[]\n",
            "    %tuple#0:shape#3(#(list_type#0, fn() -> Int)) = tuple.value ",
            "elements=[%list.int#0, %function.int#0]\n",
            "    %int#0:shape#0(Int) = int.value 0\n",
            "    return %int#0\n",
            "\nfunction int#1\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %int#0:shape#0(Int) = int.value 1\n",
            "    return %int#0\n",
            "\nfunction list.int#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %list.int#0:shape#1(list_type#0) = list.int[type#0] value ",
            "elements=[]\n",
            "    return %list.int#0\n",
            "\nfunction function.int#0\n",
            "  entry b0 params=[] captures=[]\n",
            "  block b0 params=[]\n",
            "    %function.int#0:shape#2(fn() -> Int) = function[Int] closure ",
            "target=int#1 captures=[]\n",
            "    return %function.int#0\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&plan.program.functions);
        });
    }
}

impl<IntFunction, BoolFunction> FunctionTables<IntFunction, BoolFunction> {
    pub(in crate::plan::execution) fn never_function(
        &self,
        id: NeverFunctionId,
    ) -> &ExecutableFunction<NeverFunctionBody> {
        &self.value_returns.never_functions[id.0]
    }

    pub(in crate::plan::execution) fn parameter_list_function(
        &self,
        id: ParameterListFunctionId,
    ) -> &ExecutableFunction<ParameterListFunctionBody> {
        &self.list_returns.parameter_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn parameter_list_list_function(
        &self,
        id: ParameterListListFunctionId,
    ) -> &ExecutableFunction<ParameterListListFunctionBody> {
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

    pub(in crate::plan::execution) fn int_function(&self, id: IntFunctionId) -> &IntFunction {
        &self.value_returns.int_functions[id.0]
    }

    pub(in crate::plan::execution) fn float_function(
        &self,
        id: FloatFunctionId,
    ) -> &ExecutableFunction<FloatFunctionBody> {
        &self.value_returns.float_functions[id.0]
    }

    pub(in crate::plan::execution) fn string_function(
        &self,
        id: StringFunctionId,
    ) -> &ExecutableFunction<StringFunctionBody> {
        &self.value_returns.string_functions[id.0]
    }

    pub(in crate::plan::execution) fn bit_array_function(
        &self,
        id: BitArrayFunctionId,
    ) -> &ExecutableFunction<BitArrayFunctionBody> {
        &self.value_returns.bit_array_functions[id.0]
    }

    pub(in crate::plan::execution) fn utf_codepoint_function(
        &self,
        id: UtfCodepointFunctionId,
    ) -> &ExecutableFunction<UtfCodepointFunctionBody> {
        &self.value_returns.utf_codepoint_functions[id.0]
    }

    pub(in crate::plan::execution) fn custom_function(
        &self,
        id: CustomFunctionId,
    ) -> &ExecutableFunction<CustomFunctionBody> {
        &self.value_returns.custom_functions[id.index()]
    }

    #[cfg(test)]
    pub(in crate::plan::execution) fn custom_function_id(&self, index: usize) -> CustomFunctionId {
        CustomFunctionId::new(
            index,
            *self.value_returns.custom_functions[index]
                .body()
                .signature_shape(),
        )
    }

    pub(in crate::plan::execution) fn bool_function(&self, id: BoolFunctionId) -> &BoolFunction {
        &self.value_returns.bool_functions[id.0]
    }

    pub(in crate::plan::execution) fn nil_function(
        &self,
        id: NilFunctionId,
    ) -> &ExecutableFunction<NilFunctionBody> {
        &self.value_returns.nil_functions[id.0]
    }

    pub(in crate::plan::execution) fn tuple_function(
        &self,
        id: TupleFunctionId,
    ) -> &ExecutableFunction<TupleFunctionBody> {
        &self.value_returns.tuple_functions[id.0]
    }

    pub(in crate::plan::execution) fn int_list_function(
        &self,
        id: IntListFunctionId,
    ) -> &ExecutableFunction<IntListFunctionBody> {
        &self.list_returns.int_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn string_list_function(
        &self,
        id: StringListFunctionId,
    ) -> &ExecutableFunction<StringListFunctionBody> {
        &self.list_returns.string_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn bit_array_list_function(
        &self,
        id: BitArrayListFunctionId,
    ) -> &ExecutableFunction<BitArrayListFunctionBody> {
        &self.list_returns.bit_array_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn utf_codepoint_list_function(
        &self,
        id: UtfCodepointListFunctionId,
    ) -> &ExecutableFunction<UtfCodepointListFunctionBody> {
        &self.list_returns.utf_codepoint_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn custom_list_function(
        &self,
        id: CustomListFunctionId,
    ) -> &ExecutableFunction<CustomListFunctionBody> {
        &self.list_returns.custom_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn float_list_function(
        &self,
        id: FloatListFunctionId,
    ) -> &ExecutableFunction<FloatListFunctionBody> {
        &self.list_returns.float_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn bool_list_function(
        &self,
        id: BoolListFunctionId,
    ) -> &ExecutableFunction<BoolListFunctionBody> {
        &self.list_returns.bool_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn nil_list_function(
        &self,
        id: NilListFunctionId,
    ) -> &ExecutableFunction<NilListFunctionBody> {
        &self.list_returns.nil_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn tuple_list_function(
        &self,
        id: TupleListFunctionId,
    ) -> &ExecutableFunction<TupleListFunctionBody> {
        &self.list_returns.tuple_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn list_list_function(
        &self,
        id: ListListFunctionId,
    ) -> &ExecutableFunction<ListListFunctionBody> {
        &self.list_returns.list_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn function_list_function(
        &self,
        id: FunctionListFunctionId,
    ) -> &ExecutableFunction<FunctionListFunctionBody> {
        &self.list_returns.function_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &ExecutableFunction<IntFunctionFunctionBody> {
        &self.function_returns.int_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &ExecutableFunction<FloatFunctionFunctionBody> {
        &self.function_returns.float_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &ExecutableFunction<StringFunctionFunctionBody> {
        &self.function_returns.string_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn bit_array_function_function(
        &self,
        id: BitArrayFunctionFunctionId,
    ) -> &ExecutableFunction<BitArrayFunctionFunctionBody> {
        &self.function_returns.bit_array_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn utf_codepoint_function_function(
        &self,
        id: UtfCodepointFunctionFunctionId,
    ) -> &ExecutableFunction<UtfCodepointFunctionFunctionBody> {
        &self.function_returns.utf_codepoint_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn custom_function_function(
        &self,
        id: &CustomFunctionFunctionId,
    ) -> &ExecutableFunction<CustomFunctionFunctionBody> {
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
                .body()
                .type_()
                .clone(),
        )
    }

    pub(in crate::plan::execution) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &ExecutableFunction<BoolFunctionFunctionBody> {
        &self.function_returns.bool_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &ExecutableFunction<NilFunctionFunctionBody> {
        &self.function_returns.nil_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &ExecutableFunction<TupleFunctionFunctionBody> {
        &self.function_returns.tuple_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn generic_function_function(
        &self,
        id: &GenericFunctionFunctionId,
    ) -> &ExecutableFunction<GenericFunctionFunctionBody> {
        &self.function_returns.generic_function_functions[id.index()]
    }

    pub(in crate::plan::execution) fn never_function_function(
        &self,
        id: &NeverFunctionFunctionId,
    ) -> &ExecutableFunction<NeverFunctionFunctionBody> {
        &self.function_returns.never_function_functions[id.index()]
    }

    pub(in crate::plan::execution) fn list_function_function(
        &self,
        id: &ListFunctionFunctionId,
    ) -> &ExecutableFunction<ListFunctionFunctionBody> {
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
    ) -> &ExecutableFunction<FunctionFunctionFunctionBody> {
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
            run_main(
                &execution_plan(
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
                ),
                &mut Vec::new()
            ),
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
            let function = expect_function(
                run_main(&plan, &mut Vec::new()).expect("main should return a function"),
            );

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
