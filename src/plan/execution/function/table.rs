mod host;

use super::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BitArrayListFunctionId, BoolFunctionFunctionId,
    BoolFunctionId, BoolListFunctionId, CustomFunctionFunctionId, CustomFunctionId,
    CustomListFunctionId, ExecutionBitArrayFunctionBody, ExecutionBitArrayFunctionFunctionBody,
    ExecutionBitArrayListFunctionBody, ExecutionBoolFunctionBody,
    ExecutionBoolFunctionFunctionBody, ExecutionBoolListFunctionBody,
    ExecutionCoreListFunctionFunctionBody, ExecutionCustomFunctionBody,
    ExecutionCustomFunctionFunctionBody, ExecutionCustomListFunctionBody,
    ExecutionExternalFunctionBody, ExecutionExternalFunctionFunctionBody,
    ExecutionExternalListFunctionBody, ExecutionExternalListFunctionFunctionBody,
    ExecutionFloatFunctionBody, ExecutionFloatFunctionFunctionBody, ExecutionFloatListFunctionBody,
    ExecutionFunctionFunctionFunctionBody, ExecutionFunctionListFunctionBody,
    ExecutionGenericFunctionFunctionBody, ExecutionIntFunctionBody,
    ExecutionIntFunctionFunctionBody, ExecutionIntListFunctionBody, ExecutionListListFunctionBody,
    ExecutionNeverFunctionFunctionBody, ExecutionNilFunctionBody, ExecutionNilFunctionFunctionBody,
    ExecutionNilListFunctionBody, ExecutionParameterListFunctionBody,
    ExecutionParameterListListFunctionBody, ExecutionStringFunctionBody,
    ExecutionStringFunctionFunctionBody, ExecutionStringListFunctionBody,
    ExecutionTupleFunctionBody, ExecutionTupleFunctionFunctionBody, ExecutionTupleListFunctionBody,
    ExecutionUtfCodepointFunctionBody, ExecutionUtfCodepointFunctionFunctionBody,
    ExecutionUtfCodepointListFunctionBody, ExternalFunctionFunctionId, ExternalFunctionId,
    ExternalListFunctionFunctionId, ExternalListFunctionId, FloatFunctionFunctionId,
    FloatFunctionId, FloatListFunctionId, FunctionFunctionFunctionId, FunctionListFunctionId,
    GenericFunctionFunctionId, IntFunctionFunctionId, IntFunctionId, IntListFunctionId,
    ListListFunctionId, NeverFunctionFunctionId, NeverFunctionId, NilFunctionFunctionId,
    NilFunctionId, NilListFunctionId, ParameterListFunctionId, ParameterListListFunctionId,
    ProfiledListFunctionFunctionId, StringFunctionFunctionId, StringFunctionId,
    StringListFunctionId, TupleFunctionFunctionId, TupleFunctionId, TupleListFunctionId,
    UtfCodepointFunctionFunctionId, UtfCodepointFunctionId, UtfCodepointListFunctionId,
};
use super::{
    ExecutableFunction, ExecutionFunction, ExecutionNeverFunction, ExecutionProfile,
    FunctionFunctionTables, ListFunctionTables, ValueFunctionTables,
};
use crate::plan::execution::explain::{Explain, ExplainContext, FunctionLabel};
use crate::plan::execution::function::{
    FunctionBodyOwner, FunctionLabelSource, TailCallLabelIndex,
};
use crate::plan::execution::graph::LocalLabel;
use std::convert::Infallible;

pub(in crate::plan::execution) use host::HostedFunctionTablesExplanation;

pub(in crate::plan::execution) struct FunctionTables<Profile: ExecutionProfile> {
    pub(in crate::plan::execution) value_returns: ValueFunctionTables<Profile>,
    pub(in crate::plan::execution) list_returns: ListFunctionTables<Profile>,
    pub(in crate::plan::execution) function_returns: FunctionFunctionTables<Profile>,
}

impl Explain for FunctionTables<Infallible> {
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
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalFunctionId:
        FunctionLabelSource,
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalListFunctionId:
        FunctionLabelSource,
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalFunctionFunctionId:
        FunctionLabelSource,
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalListFunctionFunctionId:
        FunctionLabelSource,
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalInstruction:
        Explain,
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalListInstruction:
        Explain,
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalFunctionInstruction:
        Explain,
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
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalFunctionId:
        FunctionLabelSource,
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalListFunctionId:
        FunctionLabelSource,
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalFunctionFunctionId:
        FunctionLabelSource,
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalListFunctionFunctionId:
        FunctionLabelSource,
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalInstruction:
        Explain,
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalListInstruction:
        Explain,
    <Body::Graph as crate::plan::execution::function::ExecutionGraphProfile>::ExternalFunctionInstruction:
        Explain,
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

impl<Profile: ExecutionProfile> FunctionTables<Profile> {
    pub(in crate::plan::execution) fn never_function(
        &self,
        id: NeverFunctionId,
    ) -> &ExecutionNeverFunction<Profile> {
        &self.value_returns.never_functions[id.0]
    }

    pub(in crate::plan::execution) fn parameter_list_function(
        &self,
        id: ParameterListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionParameterListFunctionBody<Profile>> {
        &self.list_returns.parameter_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn parameter_list_list_function(
        &self,
        id: ParameterListListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionParameterListListFunctionBody<Profile>> {
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
    ) -> &ExecutionFunction<Profile, ExecutionIntFunctionBody<Profile>> {
        &self.value_returns.int_functions[id.0]
    }

    pub(in crate::plan::execution) fn float_function(
        &self,
        id: FloatFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionFloatFunctionBody<Profile>> {
        &self.value_returns.float_functions[id.0]
    }

    pub(in crate::plan::execution) fn string_function(
        &self,
        id: StringFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionStringFunctionBody<Profile>> {
        &self.value_returns.string_functions[id.0]
    }

    pub(in crate::plan::execution) fn bit_array_function(
        &self,
        id: BitArrayFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionBitArrayFunctionBody<Profile>> {
        &self.value_returns.bit_array_functions[id.0]
    }

    pub(in crate::plan::execution) fn utf_codepoint_function(
        &self,
        id: UtfCodepointFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionUtfCodepointFunctionBody<Profile>> {
        &self.value_returns.utf_codepoint_functions[id.0]
    }

    pub(in crate::plan::execution) fn custom_function(
        &self,
        id: CustomFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionCustomFunctionBody<Profile>> {
        &self.value_returns.custom_functions[id.index()]
    }

    pub(in crate::plan::execution) fn external_function(
        &self,
        id: ExternalFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionExternalFunctionBody<Profile>> {
        &self.value_returns.external_functions[id.index()]
    }

    pub(in crate::plan::execution) fn bool_function(
        &self,
        id: BoolFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionBoolFunctionBody<Profile>> {
        &self.value_returns.bool_functions[id.0]
    }

    pub(in crate::plan::execution) fn nil_function(
        &self,
        id: NilFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionNilFunctionBody<Profile>> {
        &self.value_returns.nil_functions[id.0]
    }

    pub(in crate::plan::execution) fn tuple_function(
        &self,
        id: TupleFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionTupleFunctionBody<Profile>> {
        &self.value_returns.tuple_functions[id.0]
    }

    pub(in crate::plan::execution) fn int_list_function(
        &self,
        id: IntListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionIntListFunctionBody<Profile>> {
        &self.list_returns.int_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn string_list_function(
        &self,
        id: StringListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionStringListFunctionBody<Profile>> {
        &self.list_returns.string_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn bit_array_list_function(
        &self,
        id: BitArrayListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionBitArrayListFunctionBody<Profile>> {
        &self.list_returns.bit_array_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn utf_codepoint_list_function(
        &self,
        id: UtfCodepointListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionUtfCodepointListFunctionBody<Profile>> {
        &self.list_returns.utf_codepoint_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn custom_list_function(
        &self,
        id: CustomListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionCustomListFunctionBody<Profile>> {
        &self.list_returns.custom_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn external_list_function(
        &self,
        id: ExternalListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionExternalListFunctionBody<Profile>> {
        &self.list_returns.external_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn float_list_function(
        &self,
        id: FloatListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionFloatListFunctionBody<Profile>> {
        &self.list_returns.float_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn bool_list_function(
        &self,
        id: BoolListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionBoolListFunctionBody<Profile>> {
        &self.list_returns.bool_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn nil_list_function(
        &self,
        id: NilListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionNilListFunctionBody<Profile>> {
        &self.list_returns.nil_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn tuple_list_function(
        &self,
        id: TupleListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionTupleListFunctionBody<Profile>> {
        &self.list_returns.tuple_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn list_list_function(
        &self,
        id: ListListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionListListFunctionBody<Profile>> {
        &self.list_returns.list_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn function_list_function(
        &self,
        id: FunctionListFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionFunctionListFunctionBody<Profile>> {
        &self.list_returns.function_list_functions[id.index()].1
    }

    pub(in crate::plan::execution) fn int_function_function(
        &self,
        id: IntFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionIntFunctionFunctionBody<Profile>> {
        &self.function_returns.int_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn float_function_function(
        &self,
        id: FloatFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionFloatFunctionFunctionBody<Profile>> {
        &self.function_returns.float_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn string_function_function(
        &self,
        id: StringFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionStringFunctionFunctionBody<Profile>> {
        &self.function_returns.string_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn bit_array_function_function(
        &self,
        id: BitArrayFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionBitArrayFunctionFunctionBody<Profile>> {
        &self.function_returns.bit_array_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn utf_codepoint_function_function(
        &self,
        id: UtfCodepointFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionUtfCodepointFunctionFunctionBody<Profile>> {
        &self.function_returns.utf_codepoint_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn custom_function_function(
        &self,
        id: &CustomFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionCustomFunctionFunctionBody<Profile>> {
        &self.function_returns.custom_function_functions[id.index()]
    }

    pub(in crate::plan::execution) fn external_function_function(
        &self,
        id: &ExternalFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionExternalFunctionFunctionBody<Profile>> {
        &self.function_returns.external_function_functions[id.index()]
    }

    pub(in crate::plan::execution) fn bool_function_function(
        &self,
        id: BoolFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionBoolFunctionFunctionBody<Profile>> {
        &self.function_returns.bool_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn nil_function_function(
        &self,
        id: NilFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionNilFunctionFunctionBody<Profile>> {
        &self.function_returns.nil_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn tuple_function_function(
        &self,
        id: TupleFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionTupleFunctionFunctionBody<Profile>> {
        &self.function_returns.tuple_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn generic_function_function(
        &self,
        id: &GenericFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionGenericFunctionFunctionBody<Profile>> {
        &self.function_returns.generic_function_functions[id.index()]
    }

    pub(in crate::plan::execution) fn never_function_function(
        &self,
        id: &NeverFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionNeverFunctionFunctionBody<Profile>> {
        &self.function_returns.never_function_functions[id.index()]
    }

    pub(in crate::plan::execution) fn core_list_function_function(
        &self,
        id: &ProfiledListFunctionFunctionId<Infallible>,
    ) -> &ExecutionFunction<Profile, ExecutionCoreListFunctionFunctionBody<Profile>> {
        match id {
            ProfiledListFunctionFunctionId::Parameter { id, .. } => {
                &self.function_returns.parameter_list_function_functions[id.0]
            }
            ProfiledListFunctionFunctionId::ParameterList { id, .. } => {
                &self.function_returns.parameter_list_list_function_functions[id.0]
            }
            ProfiledListFunctionFunctionId::Int { id, .. } => {
                &self.function_returns.int_list_function_functions[id.0]
            }
            ProfiledListFunctionFunctionId::String { id, .. } => {
                &self.function_returns.string_list_function_functions[id.0]
            }
            ProfiledListFunctionFunctionId::BitArray { id, .. } => {
                &self.function_returns.bit_array_list_function_functions[id.0]
            }
            ProfiledListFunctionFunctionId::UtfCodepoint { id, .. } => {
                &self.function_returns.utf_codepoint_list_function_functions[id.0]
            }
            ProfiledListFunctionFunctionId::Custom { id, .. } => {
                &self.function_returns.custom_list_function_functions[id.0]
            }
            ProfiledListFunctionFunctionId::External { id, .. } => match *id {},
            ProfiledListFunctionFunctionId::Float { id, .. } => {
                &self.function_returns.float_list_function_functions[id.0]
            }
            ProfiledListFunctionFunctionId::Bool { id, .. } => {
                &self.function_returns.bool_list_function_functions[id.0]
            }
            ProfiledListFunctionFunctionId::Nil { id, .. } => {
                &self.function_returns.nil_list_function_functions[id.0]
            }
            ProfiledListFunctionFunctionId::Tuple { id, .. } => {
                &self.function_returns.tuple_list_function_functions[id.0]
            }
            ProfiledListFunctionFunctionId::List { id, .. } => {
                &self.function_returns.list_list_function_functions[id.0]
            }
            ProfiledListFunctionFunctionId::Function { id, .. } => {
                &self.function_returns.function_list_function_functions[id.0]
            }
        }
    }

    pub(in crate::plan::execution) fn external_list_function_function(
        &self,
        id: ExternalListFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionExternalListFunctionFunctionBody<Profile>> {
        &self.function_returns.external_list_function_functions[id.0]
    }

    pub(in crate::plan::execution) fn function_function_function(
        &self,
        id: &FunctionFunctionFunctionId,
    ) -> &ExecutionFunction<Profile, ExecutionFunctionFunctionFunctionBody<Profile>> {
        &self.function_returns.function_function_functions[id.index()]
    }
}

#[cfg(test)]
impl FunctionTables<Infallible> {
    pub(in crate::plan::execution) fn custom_function_id(&self, index: usize) -> CustomFunctionId {
        let function = &self.value_returns.custom_functions[index];
        CustomFunctionId::new(index, *function.body().signature_shape())
    }

    pub(in crate::plan::execution) fn function_function_function_id(
        &self,
        index: usize,
    ) -> FunctionFunctionFunctionId {
        let function = &self.function_returns.function_function_functions[index];
        FunctionFunctionFunctionId::new(index, function.body().type_().clone())
    }
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
