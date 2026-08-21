use super::super::super::FunctionLocal;
use super::{write_args, write_constant, write_function_call, write_projection};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::FunctionLabelSource;
use crate::plan::execution::function::{
    BitArrayFunctionId, CustomFunctionId, ExecutionGraphProfile, ExternalFunctionFunctionId,
    ExternalFunctionId, ExternalListFunctionFunctionId, ExternalListFunctionId,
    FunctionReturnFamily, GenericCallableId, IntFunctionId, ListFunctionId, NilFunctionId,
    ProfiledFunctionFunctionId, StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::graph::LocalLabel;
use crate::plan::execution::graph::{
    BitArrayListLocalId, CoreFunctionFunctionLocal, CustomFunctionLocal, CustomListLocalId,
    CustomLocal, ExternalFunctionFunctionLocal, ExternalFunctionLocal, ExternalListLocalId,
    ExternalLocal, FloatListLocalId, FunctionFunctionLocal, FunctionListLocalId,
    GenericFunctionLocal, IntListLocalId, IntLocalId, ListFunctionLocal, ListListLocalId,
    NeverFunctionLocal, NilListLocalId, ParamLocal, ParameterListListLocalId, ParameterListLocalId,
    StringListLocalId, StringLocalId, TupleListLocalId, TupleLocalId, UtfCodepointListLocalId,
    UtfCodepointLocalId,
};
use crate::plan::execution::type_::CustomConstructorId;
use std::convert::Infallible;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum FunctionTarget {
    Generic(GenericCallableId),
    Never(crate::plan::execution::function::NeverFunctionId),
    Int(IntFunctionId),
    Float(crate::plan::execution::function::FloatFunctionId),
    String(StringFunctionId),
    BitArray(BitArrayFunctionId),
    UtfCodepoint(UtfCodepointFunctionId),
    Custom(CustomFunctionId),
    Bool(crate::plan::execution::function::BoolFunctionId),
    Nil(NilFunctionId),
    Tuple(TupleFunctionId),
    List(ListFunctionId),
    Function(ProfiledFunctionFunctionId<Infallible>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExternalFunctionTarget {
    Value(ExternalFunctionId),
    List(ExternalListFunctionId),
    Function(ExternalFunctionFunctionId),
    ListFunction {
        id: ExternalListFunctionFunctionId,
        type_: crate::plan::execution::type_::FunctionType,
        list_type: crate::plan::execution::type_::ExternalListTypeId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ExternalFunctionCallTarget {
    Function(ExternalFunctionFunctionId),
    ListFunction {
        id: ExternalListFunctionFunctionId,
        type_: crate::plan::execution::type_::FunctionType,
        list_type: crate::plan::execution::type_::ExternalListTypeId,
    },
}

pub(crate) struct FunctionInstruction {
    type_: crate::plan::execution::type_::FunctionType,
    family: FunctionReturnFamily,
    kind: FunctionInstructionKind,
}

pub(crate) struct ExternalFunctionInstruction {
    type_: crate::plan::execution::type_::FunctionType,
    family: FunctionReturnFamily,
    kind: ExternalFunctionInstructionKind,
}

pub(crate) trait ExternalFunctionInstructionView {
    fn instruction(&self) -> &ExternalFunctionInstruction;
}

pub(crate) enum FunctionCapture {
    Int {
        target: IntLocalId,
        source: IntLocalId,
    },
    Float {
        target: crate::plan::execution::graph::FloatLocalId,
        source: crate::plan::execution::graph::FloatLocalId,
    },
    String {
        target: StringLocalId,
        source: StringLocalId,
    },
    BitArray {
        target: crate::plan::execution::graph::BitArrayLocalId,
        source: crate::plan::execution::graph::BitArrayLocalId,
    },
    UtfCodepoint {
        target: UtfCodepointLocalId,
        source: UtfCodepointLocalId,
    },
    Custom {
        target: CustomLocal,
        source: CustomLocal,
    },
    External {
        target: ExternalLocal,
        source: ExternalLocal,
    },
    Bool {
        target: crate::plan::execution::graph::BoolLocalId,
        source: crate::plan::execution::graph::BoolLocalId,
    },
    Nil {
        target: crate::plan::execution::graph::NilLocalId,
        source: crate::plan::execution::graph::NilLocalId,
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
    ExternalList {
        target: ExternalListLocalId,
        source: ExternalListLocalId,
    },
    FloatList {
        target: FloatListLocalId,
        source: FloatListLocalId,
    },
    BoolList {
        target: crate::plan::execution::graph::BoolListLocalId,
        source: crate::plan::execution::graph::BoolListLocalId,
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
        target: crate::plan::execution::graph::IntFunctionLocalId,
        source: crate::plan::execution::graph::IntFunctionLocalId,
    },
    FloatFunction {
        target: crate::plan::execution::graph::FloatFunctionLocalId,
        source: crate::plan::execution::graph::FloatFunctionLocalId,
    },
    StringFunction {
        target: crate::plan::execution::graph::StringFunctionLocalId,
        source: crate::plan::execution::graph::StringFunctionLocalId,
    },
    BitArrayFunction {
        target: crate::plan::execution::graph::BitArrayFunctionLocalId,
        source: crate::plan::execution::graph::BitArrayFunctionLocalId,
    },
    UtfCodepointFunction {
        target: crate::plan::execution::graph::UtfCodepointFunctionLocalId,
        source: crate::plan::execution::graph::UtfCodepointFunctionLocalId,
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
    ExternalFunction {
        target: ExternalFunctionLocal,
        source: ExternalFunctionLocal,
    },
    BoolFunction {
        target: crate::plan::execution::graph::BoolFunctionLocalId,
        source: crate::plan::execution::graph::BoolFunctionLocalId,
    },
    NilFunction {
        target: crate::plan::execution::graph::NilFunctionLocalId,
        source: crate::plan::execution::graph::NilFunctionLocalId,
    },
    TupleFunction {
        target: crate::plan::execution::graph::TupleFunctionLocalId,
        source: crate::plan::execution::graph::TupleFunctionLocalId,
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
            FunctionInstructionKind::Call { function, args, .. } => {
                context.push_str("call ");
                function.function_label().write(context.output());
                write_args(context.output(), args);
            }
            FunctionInstructionKind::FunctionCall { function, args, .. } => {
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

impl Explain for ExternalFunctionInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        context.push_str("function[");
        context.push_str(&self.family().to_string());
        context.push_str("] ");
        match self.kind() {
            ExternalFunctionInstructionKind::Reference(target) => {
                context.push_str("reference ");
                context.write(target);
            }
            ExternalFunctionInstructionKind::Closure { target, captures } => {
                context.push_str("closure target=");
                context.write(target);
                context.push_str(" captures=");
                context.write_list(captures, |context, capture| context.write(capture));
            }
            ExternalFunctionInstructionKind::Call { function, args, .. } => {
                context.push_str("call ");
                function.function_label().write(context.output());
                write_args(context.output(), args);
            }
            ExternalFunctionInstructionKind::FunctionCall { function, args, .. } => {
                write_function_call(context.output(), "function_call", function, args);
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
            FunctionTarget::Never(function) => function.function_label().write(context.output()),
            FunctionTarget::Int(function) => function.function_label().write(context.output()),
            FunctionTarget::Float(function) => function.function_label().write(context.output()),
            FunctionTarget::String(function) => function.function_label().write(context.output()),
            FunctionTarget::BitArray(function) => function.function_label().write(context.output()),
            FunctionTarget::UtfCodepoint(function) => {
                function.function_label().write(context.output())
            }
            FunctionTarget::Custom(function) => function.function_label().write(context.output()),
            FunctionTarget::Bool(function) => function.function_label().write(context.output()),
            FunctionTarget::Nil(function) => function.function_label().write(context.output()),
            FunctionTarget::Tuple(function) => function.function_label().write(context.output()),
            FunctionTarget::List(function) => function.function_label().write(context.output()),
            FunctionTarget::Function(function) => {
                std::convert::Infallible::function_function(function)
                    .function_label()
                    .write(context.output())
            }
        }
    }
}

impl Explain for ExternalFunctionTarget {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        self.function_label().write(context.output());
    }
}

impl FunctionLabelSource for ExternalFunctionTarget {
    fn function_label(&self) -> crate::plan::execution::explain::FunctionLabel {
        match self {
            Self::Value(function) => function.function_label(),
            Self::List(function) => function.function_label(),
            Self::Function(function) => function.function_label(),
            Self::ListFunction { id, .. } => id.function_label(),
        }
    }
}

impl FunctionLabelSource for ExternalFunctionCallTarget {
    fn function_label(&self) -> crate::plan::execution::explain::FunctionLabel {
        match self {
            Self::Function(function) => function.function_label(),
            Self::ListFunction { id, .. } => id.function_label(),
        }
    }
}

impl ExternalFunctionCallTarget {
    pub(crate) fn runtime_id(&self) -> crate::plan::execution::function::FunctionFunctionId {
        match self {
            Self::Function(function) => {
                crate::plan::execution::function::FunctionFunctionId::External(function.clone())
            }
            Self::ListFunction {
                id,
                type_,
                list_type,
            } => crate::plan::execution::function::FunctionFunctionId::List(
                crate::plan::execution::function::ListFunctionFunctionId::External {
                    id: *id,
                    type_: type_.clone(),
                    list_type: *list_type,
                },
            ),
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
            FunctionCapture::External { target, source } => write_capture(output, target, source),
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
            FunctionCapture::ExternalList { target, source } => {
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
            FunctionCapture::ExternalFunction { target, source } => {
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
    Target: LocalLabel,
    Source: LocalLabel,
{
    target.write_local_label(output);
    output.push_str("<-");
    source.write_local_label(output);
}

pub(crate) enum FunctionInstructionKind {
    Constant(crate::plan::execution::constant::ConstantId<FunctionLocal>),
    Reference(FunctionTarget),
    Closure {
        target: FunctionTarget,
        captures: Box<[FunctionCapture]>,
    },
    Constructor(CustomConstructorId),
    Call {
        function: ProfiledFunctionFunctionId<Infallible>,
        args: Box<[ParamLocal]>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: CoreFunctionFunctionLocal,
        args: Box<[ParamLocal]>,
        site: crate::plan::HostCallSite,
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

pub(crate) enum ExternalFunctionInstructionKind {
    Reference(ExternalFunctionTarget),
    Closure {
        target: ExternalFunctionTarget,
        captures: Box<[FunctionCapture]>,
    },
    Call {
        function: ExternalFunctionCallTarget,
        args: Box<[ParamLocal]>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: ExternalFunctionFunctionLocal,
        args: Box<[ParamLocal]>,
        site: crate::plan::HostCallSite,
    },
}

impl FunctionInstruction {
    pub(in crate::plan::execution) fn new(
        type_: crate::plan::execution::type_::FunctionType,
        family: FunctionReturnFamily,
        kind: FunctionInstructionKind,
    ) -> Self {
        Self {
            type_,
            family,
            kind,
        }
    }

    pub(crate) fn type_(&self) -> &crate::plan::execution::type_::FunctionType {
        &self.type_
    }

    pub(crate) fn family(&self) -> FunctionReturnFamily {
        self.family
    }

    pub(crate) fn kind(&self) -> &FunctionInstructionKind {
        &self.kind
    }
}

impl ExternalFunctionInstruction {
    pub(in crate::plan::execution) fn new(
        type_: crate::plan::execution::type_::FunctionType,
        family: FunctionReturnFamily,
        kind: ExternalFunctionInstructionKind,
    ) -> Self {
        Self {
            type_,
            family,
            kind,
        }
    }

    pub(crate) fn type_(&self) -> &crate::plan::execution::type_::FunctionType {
        &self.type_
    }

    pub(crate) fn family(&self) -> FunctionReturnFamily {
        self.family
    }

    pub(crate) fn kind(&self) -> &ExternalFunctionInstructionKind {
        &self.kind
    }
}

impl ExternalFunctionInstructionView for ExternalFunctionInstruction {
    fn instruction(&self) -> &ExternalFunctionInstruction {
        self
    }
}

impl ExternalFunctionInstructionView for Infallible {
    fn instruction(&self) -> &ExternalFunctionInstruction {
        match *self {}
    }
}

#[cfg(test)]
mod external_function_instruction_view_tests {
    use super::{
        ExternalFunctionInstruction, ExternalFunctionInstructionKind,
        ExternalFunctionInstructionView, ExternalFunctionTarget,
    };
    use crate::plan::execution::function::{ExternalFunctionId, FunctionReturnFamily};
    use crate::plan::execution::type_::{ExternalTypeId, FunctionType, ValueType};
    use std::convert::Infallible;

    #[test]
    fn exposes_external_function_instruction_metadata() {
        let external_type = ExternalTypeId::new(0);
        let function_type = FunctionType::new(Vec::new(), ValueType::External(external_type));
        let instruction = ExternalFunctionInstruction::new(
            function_type.clone(),
            FunctionReturnFamily::External,
            ExternalFunctionInstructionKind::Reference(ExternalFunctionTarget::Value(
                ExternalFunctionId::new(1, external_type),
            )),
        );

        let viewed = instruction.instruction();

        assert!(std::ptr::eq(viewed, &instruction));
        assert_eq!(viewed.type_(), &function_type);
        assert_eq!(viewed.family(), FunctionReturnFamily::External);
        assert!(matches!(
            viewed.kind(),
            ExternalFunctionInstructionKind::Reference(ExternalFunctionTarget::Value(function))
                if function.index() == 1
        ));
    }

    #[test]
    fn plain_external_function_instruction_view_is_uninhabited() {
        fn assert_view<View: ExternalFunctionInstructionView>() {}

        assert_view::<Infallible>();
    }
}

#[cfg(test)]
mod function_instruction_explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::TupleFunctionId;

    #[test]
    fn writes_function_instruction_variants() {
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

#[cfg(test)]
mod external_function_instruction_explain_tests {
    use super::{
        ExternalFunctionCallTarget, ExternalFunctionInstruction, ExternalFunctionInstructionKind,
        ExternalFunctionTarget, FunctionCapture,
    };
    use crate::plan::HostCallSite;
    use crate::plan::execution::explain;
    use crate::plan::execution::function::{
        ExternalFunctionFunctionId, ExternalFunctionId, ExternalListFunctionId,
        FunctionReturnFamily,
    };
    use crate::plan::execution::graph::{
        ExternalFunctionFunctionLocal, ExternalFunctionFunctionLocalId, IntLocalId, ParamLocal,
    };
    use crate::plan::execution::type_::{
        ExternalFunctionType, ExternalListTypeId, ExternalTypeId, FunctionFunctionType,
        FunctionShape, FunctionType, ListTypeId, ValueShapeId, ValueType,
    };

    #[test]
    fn writes_external_function_reference() {
        let external_type = ExternalTypeId::new(0);
        let instruction = ExternalFunctionInstruction::new(
            FunctionType::new(Vec::new(), ValueType::External(external_type)),
            FunctionReturnFamily::External,
            ExternalFunctionInstructionKind::Reference(ExternalFunctionTarget::Value(
                ExternalFunctionId::new(1, external_type),
            )),
        );
        let expected = "function[External] reference external#1";

        assert_explanation(&instruction, expected);
    }

    #[test]
    fn writes_external_function_closure() {
        let external_type = ExternalTypeId::new(0);
        let list_type = ExternalListTypeId::new(ListTypeId::new(1), external_type);
        let instruction = ExternalFunctionInstruction::new(
            FunctionType::new(Vec::new(), ValueType::List(list_type.list_type())),
            FunctionReturnFamily::List,
            ExternalFunctionInstructionKind::Closure {
                target: ExternalFunctionTarget::List(ExternalListFunctionId::new(2, list_type)),
                captures: Box::new([FunctionCapture::Int {
                    target: IntLocalId(1),
                    source: IntLocalId(0),
                }]),
            },
        );
        let expected = "function[List] closure target=list.external#2 captures=[%int#1<-%int#0]";

        assert_explanation(&instruction, expected);
    }

    #[test]
    fn writes_external_function_call() {
        let external_type = ExternalTypeId::new(0);
        let function_type = FunctionType::new(Vec::new(), ValueType::External(external_type));
        let instruction = ExternalFunctionInstruction::new(
            function_type.clone(),
            FunctionReturnFamily::External,
            ExternalFunctionInstructionKind::Call {
                function: ExternalFunctionCallTarget::Function(ExternalFunctionFunctionId::new(
                    3,
                    ExternalFunctionType::from_shapes(function_type, Vec::new(), external_type),
                )),
                args: Box::new([ParamLocal::Int(IntLocalId(2))]),
                site: HostCallSite::unknown(),
            },
        );
        let expected = "function[External] call function.external#3 args=[%int#2]";

        assert_explanation(&instruction, expected);
    }

    #[test]
    fn writes_external_function_value_call() {
        let external_type = ExternalTypeId::new(0);
        let function_type = FunctionType::new(Vec::new(), ValueType::External(external_type));
        let instruction = ExternalFunctionInstruction::new(
            function_type.clone(),
            FunctionReturnFamily::External,
            ExternalFunctionInstructionKind::FunctionCall {
                function: ExternalFunctionFunctionLocal::new(
                    ExternalFunctionFunctionLocalId(4),
                    FunctionFunctionType::from_shapes(
                        FunctionType::new(
                            Vec::new(),
                            ValueType::Function(Box::new(function_type.clone())),
                        ),
                        Vec::new(),
                        FunctionShape::new(ValueShapeId::new(0), function_type),
                    ),
                ),
                args: Box::new([ParamLocal::Int(IntLocalId(3))]),
                site: HostCallSite::unknown(),
            },
        );
        let expected =
            "function[External] function_call %function.function.external#4 args=[%int#3]";

        assert_explanation(&instruction, expected);
    }

    fn assert_explanation(instruction: &ExternalFunctionInstruction, expected: &str) {
        explain::assert_rendered("pub fn main() { 1 }", expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(instruction);
        });
    }
}

#[cfg(test)]
mod function_target_explain_tests {
    use super::{ExternalFunctionTarget, FunctionTarget};
    use crate::plan::execution::explain;
    use crate::plan::execution::function::{
        ExternalFunctionFunctionId, ExternalFunctionId, ExternalListFunctionFunctionId,
        GenericCallableId, IntFunctionId,
    };
    use crate::plan::execution::type_::{
        ExternalFunctionType, ExternalListTypeId, ExternalTypeId, FunctionType, ListTypeId,
        ValueShapeId, ValueType,
    };

    #[test]
    fn writes_core_function_target() {
        let target = FunctionTarget::Int(IntFunctionId(2));
        let expected = "int#2";

        assert_explanation(&target, expected);
    }

    #[test]
    fn writes_generic_function_target() {
        let target =
            FunctionTarget::Generic(GenericCallableId::function(3, vec![ValueShapeId::new(4)]));
        let expected = "template#3 shapes=[shape#4]";

        assert_explanation(&target, expected);
    }

    #[test]
    fn writes_external_value_function_target() {
        let target =
            ExternalFunctionTarget::Value(ExternalFunctionId::new(5, ExternalTypeId::new(0)));
        let expected = "external#5";

        assert_explanation(&target, expected);
    }

    #[test]
    fn writes_external_function_function_target() {
        let external_type = ExternalTypeId::new(0);
        let function_type = FunctionType::new(Vec::new(), ValueType::External(external_type));
        let target = ExternalFunctionTarget::Function(ExternalFunctionFunctionId::new(
            6,
            ExternalFunctionType::from_shapes(function_type, Vec::new(), external_type),
        ));
        let expected = "function.external#6";

        assert_explanation(&target, expected);
    }

    #[test]
    fn writes_external_list_function_function_target() {
        let external_type = ExternalTypeId::new(0);
        let function_type = FunctionType::new(Vec::new(), ValueType::External(external_type));
        let target = ExternalFunctionTarget::ListFunction {
            id: ExternalListFunctionFunctionId(7),
            type_: function_type,
            list_type: ExternalListTypeId::new(ListTypeId::new(0), external_type),
        };
        let expected = "function.list.external#7";

        assert_explanation(&target, expected);
    }

    fn assert_explanation<Target>(target: &Target, expected: &str)
    where
        Target: crate::plan::execution::explain::Explain,
    {
        explain::assert_rendered("pub fn main() { 1 }", expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(target);
        });
    }
}

#[cfg(test)]
mod function_capture_explain_tests {
    use super::FunctionCapture;
    use crate::plan::execution::explain;
    use crate::plan::execution::graph::{
        ExternalFunctionLocal, ExternalFunctionLocalId, ExternalListLocalId, ExternalLocal,
        ExternalLocalId, IntLocalId,
    };
    use crate::plan::execution::type_::{
        ExternalFunctionType, ExternalTypeId, FunctionType, ValueType,
    };

    #[test]
    fn writes_int_function_capture() {
        let capture = FunctionCapture::Int {
            target: IntLocalId(1),
            source: IntLocalId(0),
        };
        let expected = "%int#1<-%int#0";

        assert_explanation(&capture, expected);
    }

    #[test]
    fn writes_external_function_capture() {
        let external_type = ExternalTypeId::new(0);
        let capture = FunctionCapture::External {
            target: ExternalLocal::new(ExternalLocalId(3), external_type),
            source: ExternalLocal::new(ExternalLocalId(2), external_type),
        };
        let expected = "%external#3<-%external#2";

        assert_explanation(&capture, expected);
    }

    #[test]
    fn writes_external_list_function_capture() {
        let capture = FunctionCapture::ExternalList {
            target: ExternalListLocalId(5),
            source: ExternalListLocalId(4),
        };
        let expected = "%list.external#5<-%list.external#4";

        assert_explanation(&capture, expected);
    }

    #[test]
    fn writes_external_function_function_capture() {
        let external_type = ExternalTypeId::new(0);
        let function_type = ExternalFunctionType::from_shapes(
            FunctionType::new(Vec::new(), ValueType::External(external_type)),
            Vec::new(),
            external_type,
        );
        let capture = FunctionCapture::ExternalFunction {
            target: ExternalFunctionLocal::new(ExternalFunctionLocalId(7), function_type.clone()),
            source: ExternalFunctionLocal::new(ExternalFunctionLocalId(6), function_type),
        };
        let expected = "%function.external#7<-%function.external#6";

        assert_explanation(&capture, expected);
    }

    fn assert_explanation(capture: &FunctionCapture, expected: &str) {
        explain::assert_rendered("pub fn main() { 1 }", expected, |plan, output| {
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(capture);
        });
    }
}
