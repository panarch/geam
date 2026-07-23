use super::super::super::FunctionLocal;
use super::{write_args, write_constant, write_function_call, write_projection};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::{
    ExplainFunctionId, function_function_label, list_function_label,
};
use crate::plan::execution::graph::ExplainLocal;
use crate::plan::execution::{
    BitArrayFunctionId, BitArrayListLocalId, CustomConstructorId, CustomFunctionId,
    CustomFunctionLocal, CustomListLocalId, CustomLocal, FloatListLocalId, FunctionFunctionId,
    FunctionFunctionLocal, FunctionListLocalId, FunctionReturnFamily, GenericCallableId,
    GenericFunctionLocal, IntFunctionId, IntListLocalId, IntLocalId, ListFunctionLocal,
    ListListLocalId, NeverFunctionLocal, NilFunctionId, NilListLocalId, ParamLocal,
    ParameterListListLocalId, ParameterListLocalId, StringFunctionId, StringListLocalId,
    StringLocalId, TupleFunctionId, TupleListLocalId, TupleLocalId, UtfCodepointFunctionId,
    UtfCodepointListLocalId, UtfCodepointLocalId,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FunctionTarget {
    Generic(GenericCallableId),
    Never(crate::plan::execution::NeverFunctionId),
    Int(IntFunctionId),
    Float(crate::plan::execution::FloatFunctionId),
    String(StringFunctionId),
    BitArray(BitArrayFunctionId),
    UtfCodepoint(UtfCodepointFunctionId),
    Custom(CustomFunctionId),
    Bool(crate::plan::execution::BoolFunctionId),
    Nil(NilFunctionId),
    Tuple(TupleFunctionId),
    List(crate::plan::execution::ListFunctionId),
    Function(FunctionFunctionId),
}

pub(crate) struct FunctionInstruction {
    type_: crate::plan::execution::FunctionType,
    family: FunctionReturnFamily,
    kind: FunctionInstructionKind,
}

pub(crate) enum FunctionCapture {
    Int {
        target: IntLocalId,
        source: IntLocalId,
    },
    Float {
        target: crate::plan::execution::FloatLocalId,
        source: crate::plan::execution::FloatLocalId,
    },
    String {
        target: StringLocalId,
        source: StringLocalId,
    },
    BitArray {
        target: crate::plan::execution::BitArrayLocalId,
        source: crate::plan::execution::BitArrayLocalId,
    },
    UtfCodepoint {
        target: UtfCodepointLocalId,
        source: UtfCodepointLocalId,
    },
    Custom {
        target: CustomLocal,
        source: CustomLocal,
    },
    Bool {
        target: crate::plan::execution::BoolLocalId,
        source: crate::plan::execution::BoolLocalId,
    },
    Nil {
        target: crate::plan::execution::NilLocalId,
        source: crate::plan::execution::NilLocalId,
    },
    Tuple {
        target: TupleLocalId,
        source: TupleLocalId,
    },
    ParameterList {
        target: ParameterListLocalId,
        source: ParameterListLocalId,
    },
    ParameterListList {
        target: ParameterListListLocalId,
        source: ParameterListListLocalId,
    },
    IntList {
        target: IntListLocalId,
        source: IntListLocalId,
    },
    StringList {
        target: StringListLocalId,
        source: StringListLocalId,
    },
    BitArrayList {
        target: BitArrayListLocalId,
        source: BitArrayListLocalId,
    },
    UtfCodepointList {
        target: UtfCodepointListLocalId,
        source: UtfCodepointListLocalId,
    },
    CustomList {
        target: CustomListLocalId,
        source: CustomListLocalId,
    },
    FloatList {
        target: FloatListLocalId,
        source: FloatListLocalId,
    },
    BoolList {
        target: crate::plan::execution::BoolListLocalId,
        source: crate::plan::execution::BoolListLocalId,
    },
    NilList {
        target: NilListLocalId,
        source: NilListLocalId,
    },
    TupleList {
        target: TupleListLocalId,
        source: TupleListLocalId,
    },
    ListList {
        target: ListListLocalId,
        source: ListListLocalId,
    },
    FunctionList {
        target: FunctionListLocalId,
        source: FunctionListLocalId,
    },
    IntFunction {
        target: crate::plan::execution::IntFunctionLocalId,
        source: crate::plan::execution::IntFunctionLocalId,
    },
    FloatFunction {
        target: crate::plan::execution::FloatFunctionLocalId,
        source: crate::plan::execution::FloatFunctionLocalId,
    },
    StringFunction {
        target: crate::plan::execution::StringFunctionLocalId,
        source: crate::plan::execution::StringFunctionLocalId,
    },
    BitArrayFunction {
        target: crate::plan::execution::BitArrayFunctionLocalId,
        source: crate::plan::execution::BitArrayFunctionLocalId,
    },
    UtfCodepointFunction {
        target: crate::plan::execution::UtfCodepointFunctionLocalId,
        source: crate::plan::execution::UtfCodepointFunctionLocalId,
    },
    GenericFunction {
        target: GenericFunctionLocal,
        source: GenericFunctionLocal,
    },
    NeverFunction {
        target: NeverFunctionLocal,
        source: NeverFunctionLocal,
    },
    CustomFunction {
        target: CustomFunctionLocal,
        source: CustomFunctionLocal,
    },
    BoolFunction {
        target: crate::plan::execution::BoolFunctionLocalId,
        source: crate::plan::execution::BoolFunctionLocalId,
    },
    NilFunction {
        target: crate::plan::execution::NilFunctionLocalId,
        source: crate::plan::execution::NilFunctionLocalId,
    },
    TupleFunction {
        target: crate::plan::execution::TupleFunctionLocalId,
        source: crate::plan::execution::TupleFunctionLocalId,
    },
    ListFunction {
        target: ListFunctionLocal,
        source: ListFunctionLocal,
    },
    FunctionFunction {
        target: FunctionFunctionLocal,
        source: FunctionFunctionLocal,
    },
}

impl Explain for FunctionInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("function[");
        context.push_str(&self.family().to_string());
        context.push_str("] ");
        match self.kind() {
            FunctionInstructionKind::Constant(id) => {
                write_constant(context.output(), "function", *id);
            }
            FunctionInstructionKind::Reference(target) => {
                context.push_str("reference ");
                context.write(target);
            }
            FunctionInstructionKind::Closure { target, captures } => {
                context.push_str("closure target=");
                context.write(target);
                context.push_str(" captures=");
                context.write_list(captures, |context, capture| context.write(capture));
            }
            FunctionInstructionKind::Constructor(constructor) => {
                context.push_str("constructor custom_type#");
                context.push_str(&constructor.type_id().index().to_string());
                context.push_str(".constructor#");
                context.push_str(&constructor.index().to_string());
            }
            FunctionInstructionKind::Call { function, args } => {
                context.push_str("call ");
                function_function_label(function).write(context.output());
                write_args(context.output(), args);
            }
            FunctionInstructionKind::FunctionCall { function, args } => {
                write_function_call(context.output(), "function_call", function, args);
            }
            FunctionInstructionKind::TupleIndex { tuple, index } => {
                write_projection(context.output(), "tuple_index", tuple, *index);
            }
            FunctionInstructionKind::CustomField { source, index } => {
                write_projection(context.output(), "custom_field", source, *index);
            }
            FunctionInstructionKind::ListIndex { list, index } => {
                write_projection(context.output(), "list_index", list, *index);
            }
        }
    }
}

impl Explain for FunctionTarget {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        match self {
            FunctionTarget::Generic(GenericCallableId::Function {
                template,
                substitution,
            }) => {
                context.push_str("template#");
                context.push_str(&template.to_string());
                context.push_str(" shapes=");
                context.write_list(substitution, |context, shape| {
                    context.push_str("shape#");
                    context.push_str(&shape.index().to_string());
                });
            }
            FunctionTarget::Generic(GenericCallableId::Constructor(constructor)) => {
                context.push_str("custom_type#");
                context.push_str(&constructor.type_id().index().to_string());
                context.push_str(".constructor#");
                context.push_str(&constructor.index().to_string());
            }
            FunctionTarget::Never(function) => function.label().write(context.output()),
            FunctionTarget::Int(function) => function.label().write(context.output()),
            FunctionTarget::Float(function) => function.label().write(context.output()),
            FunctionTarget::String(function) => function.label().write(context.output()),
            FunctionTarget::BitArray(function) => function.label().write(context.output()),
            FunctionTarget::UtfCodepoint(function) => function.label().write(context.output()),
            FunctionTarget::Custom(function) => function.label().write(context.output()),
            FunctionTarget::Bool(function) => function.label().write(context.output()),
            FunctionTarget::Nil(function) => function.label().write(context.output()),
            FunctionTarget::Tuple(function) => function.label().write(context.output()),
            FunctionTarget::List(function) => list_function_label(function).write(context.output()),
            FunctionTarget::Function(function) => {
                function_function_label(function).write(context.output());
            }
        }
    }
}

impl Explain for FunctionCapture {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            FunctionCapture::Int { target, source } => write_capture(output, target, source),
            FunctionCapture::Float { target, source } => write_capture(output, target, source),
            FunctionCapture::String { target, source } => write_capture(output, target, source),
            FunctionCapture::BitArray { target, source } => write_capture(output, target, source),
            FunctionCapture::UtfCodepoint { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::Custom { target, source } => write_capture(output, target, source),
            FunctionCapture::Bool { target, source } => write_capture(output, target, source),
            FunctionCapture::Nil { target, source } => write_capture(output, target, source),
            FunctionCapture::Tuple { target, source } => write_capture(output, target, source),
            FunctionCapture::ParameterList { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::ParameterListList { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::IntList { target, source } => write_capture(output, target, source),
            FunctionCapture::StringList { target, source } => write_capture(output, target, source),
            FunctionCapture::BitArrayList { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::UtfCodepointList { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::CustomList { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::FloatList { target, source } => write_capture(output, target, source),
            FunctionCapture::BoolList { target, source } => write_capture(output, target, source),
            FunctionCapture::NilList { target, source } => write_capture(output, target, source),
            FunctionCapture::TupleList { target, source } => write_capture(output, target, source),
            FunctionCapture::ListList { target, source } => write_capture(output, target, source),
            FunctionCapture::FunctionList { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::IntFunction { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::FloatFunction { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::StringFunction { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::BitArrayFunction { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::UtfCodepointFunction { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::GenericFunction { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::NeverFunction { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::CustomFunction { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::BoolFunction { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::NilFunction { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::TupleFunction { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::ListFunction { target, source } => {
                write_capture(output, target, source);
            }
            FunctionCapture::FunctionFunction { target, source } => {
                write_capture(output, target, source);
            }
        }
    }
}

fn write_capture<Target, Source>(output: &mut String, target: &Target, source: &Source)
where
    Target: ExplainLocal,
    Source: ExplainLocal,
{
    target.write_local(output);
    output.push_str("<-");
    source.write_local(output);
}

pub(crate) enum FunctionInstructionKind {
    Constant(crate::plan::execution::ConstantId<FunctionLocal>),
    Reference(FunctionTarget),
    Closure {
        target: FunctionTarget,
        captures: Box<[FunctionCapture]>,
    },
    Constructor(CustomConstructorId),
    Call {
        function: FunctionFunctionId,
        args: Box<[ParamLocal]>,
    },
    FunctionCall {
        function: FunctionFunctionLocal,
        args: Box<[ParamLocal]>,
    },
    TupleIndex {
        tuple: TupleLocalId,
        index: usize,
    },
    CustomField {
        source: CustomLocal,
        index: usize,
    },
    ListIndex {
        list: FunctionListLocalId,
        index: usize,
    },
}

impl FunctionInstruction {
    pub(in crate::plan::execution) fn new(
        type_: crate::plan::execution::FunctionType,
        family: FunctionReturnFamily,
        kind: FunctionInstructionKind,
    ) -> Self {
        Self {
            type_,
            family,
            kind,
        }
    }

    pub(crate) fn type_(&self) -> &crate::plan::execution::FunctionType {
        &self.type_
    }

    pub(crate) fn family(&self) -> FunctionReturnFamily {
        self.family
    }

    pub(crate) fn kind(&self) -> &FunctionInstructionKind {
        &self.kind
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::{TupleFunctionId, explain};

    #[test]
    fn writes_function_instruction_targets_calls_and_captures() {
        let source = r#"
fn identity(value: Int) { value }
fn returner(function: fn(Int) -> Int) { function }

pub fn main() {
  let captured = 1
  let reference = identity
  let closure = fn(value) { value + captured }
  let caller = returner
  let direct = returner(reference)
  let indirect = caller(reference)
  #(reference, closure, direct, indirect)
}
"#;
        let expected = concat!(
            "    %int#0:shape#0(Int) = int.value 1\n",
            "    %function.int#0:shape#1(fn(Int) -> Int) = function[Int] ",
            "reference int#0\n",
            "    %function.int#1:shape#1(fn(Int) -> Int) = function[Int] closure ",
            "target=int#1 captures=[%int#1<-%int#0]\n",
            "    %function.function#0:shape#2(fn(fn(Int) -> Int) -> fn(Int) -> Int) = ",
            "function[Function] reference function.int#0\n",
            "    %function.int#2:shape#1(fn(Int) -> Int) = function[Int] call ",
            "function.int#0 args=[%function.int#0]\n",
            "    %function.int#3:shape#1(fn(Int) -> Int) = function[Int] function_call ",
            "%function.function#0 args=[%function.int#0]\n",
            "    %tuple#0:shape#3(#(fn(Int) -> Int, fn(Int) -> Int, fn(Int) -> Int, ",
            "fn(Int) -> Int)) = tuple.value elements=[%function.int#0, ",
            "%function.int#1, %function.int#2, %function.int#3]\n",
        );

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let graph = plan.tuple_function(TupleFunctionId(0)).body().block_graph();
            let mut context = explain::ExplainContext::new(plan, output);
            for block in graph.blocks() {
                for instruction in block.instructions() {
                    context.write(instruction);
                }
            }
        });
    }
}
