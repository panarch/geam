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
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::CustomConstructorId;
use std::convert::Infallible;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionTarget {
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
pub enum ExternalFunctionTarget {
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
pub enum ExternalFunctionCallTarget {
    Function(ExternalFunctionFunctionId),
    ListFunction {
        id: ExternalListFunctionFunctionId,
        type_: crate::plan::execution::type_::FunctionType,
        list_type: crate::plan::execution::type_::ExternalListTypeId,
    },
}

#[derive(Clone)]
pub struct FunctionInstruction {
    pub type_: crate::plan::execution::type_::FunctionType,
    pub family: FunctionReturnFamily,
    pub kind: FunctionInstructionKind,
}

#[derive(Clone)]
pub struct ExternalFunctionInstruction {
    pub type_: crate::plan::execution::type_::FunctionType,
    pub family: FunctionReturnFamily,
    pub kind: ExternalFunctionInstructionKind,
}

pub trait ExternalFunctionInstructionView {
    fn instruction(&self) -> &ExternalFunctionInstruction;
}

#[derive(Clone)]
pub enum FunctionCapture {
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

#[derive(Clone)]
pub enum FunctionInstructionKind {
    Constant(crate::plan::execution::constant::ConstantId<FunctionLocal>),
    Reference(FunctionTarget),
    Closure {
        target: FunctionTarget,
        captures: Table<FunctionCapture>,
    },
    Constructor(CustomConstructorId),
    Call {
        function: ProfiledFunctionFunctionId<Infallible>,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: CoreFunctionFunctionLocal,
        args: Table<ParamLocal>,
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

#[derive(Clone)]
pub enum ExternalFunctionInstructionKind {
    Reference(ExternalFunctionTarget),
    Closure {
        target: ExternalFunctionTarget,
        captures: Table<FunctionCapture>,
    },
    Call {
        function: ExternalFunctionCallTarget,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: ExternalFunctionFunctionLocal,
        args: Table<ParamLocal>,
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

impl Emit for FunctionTarget {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Generic(field_0) => output.call("graph::FunctionTarget::Generic", &[field_0]),
            Self::Never(field_0) => output.call("graph::FunctionTarget::Never", &[field_0]),
            Self::Int(field_0) => output.call("graph::FunctionTarget::Int", &[field_0]),
            Self::Float(field_0) => output.call("graph::FunctionTarget::Float", &[field_0]),
            Self::String(field_0) => output.call("graph::FunctionTarget::String", &[field_0]),
            Self::BitArray(field_0) => output.call("graph::FunctionTarget::BitArray", &[field_0]),
            Self::UtfCodepoint(field_0) => {
                output.call("graph::FunctionTarget::UtfCodepoint", &[field_0])
            }
            Self::Custom(field_0) => output.call("graph::FunctionTarget::Custom", &[field_0]),
            Self::Bool(field_0) => output.call("graph::FunctionTarget::Bool", &[field_0]),
            Self::Nil(field_0) => output.call("graph::FunctionTarget::Nil", &[field_0]),
            Self::Tuple(field_0) => output.call("graph::FunctionTarget::Tuple", &[field_0]),
            Self::List(field_0) => output.call("graph::FunctionTarget::List", &[field_0]),
            Self::Function(field_0) => output.call("graph::FunctionTarget::Function", &[field_0]),
        }
    }
}

impl Emit for ExternalFunctionTarget {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value(field_0) => output.call("graph::ExternalFunctionTarget::Value", &[field_0]),
            Self::List(field_0) => output.call("graph::ExternalFunctionTarget::List", &[field_0]),
            Self::Function(field_0) => {
                output.call("graph::ExternalFunctionTarget::Function", &[field_0])
            }
            Self::ListFunction {
                id,
                type_,
                list_type,
            } => output.structure(
                "graph::ExternalFunctionTarget::ListFunction",
                &[("id", id), ("type_", type_), ("list_type", list_type)],
            ),
        }
    }
}

impl Emit for ExternalFunctionCallTarget {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Function(field_0) => {
                output.call("graph::ExternalFunctionCallTarget::Function", &[field_0])
            }
            Self::ListFunction {
                id,
                type_,
                list_type,
            } => output.structure(
                "graph::ExternalFunctionCallTarget::ListFunction",
                &[("id", id), ("type_", type_), ("list_type", list_type)],
            ),
        }
    }
}

impl Emit for FunctionInstruction {
    fn emit(&self, output: &mut Rust) {
        let Self {
            type_,
            family,
            kind,
        } = self;
        output.structure(
            "graph::FunctionInstruction",
            &[("type_", type_), ("family", family), ("kind", kind)],
        );
    }
}

impl Emit for ExternalFunctionInstruction {
    fn emit(&self, output: &mut Rust) {
        let Self {
            type_,
            family,
            kind,
        } = self;
        output.structure(
            "graph::ExternalFunctionInstruction",
            &[("type_", type_), ("family", family), ("kind", kind)],
        );
    }
}

impl Emit for FunctionCapture {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Int { target, source } => output.structure(
                "graph::FunctionCapture::Int",
                &[("target", target), ("source", source)],
            ),
            Self::Float { target, source } => output.structure(
                "graph::FunctionCapture::Float",
                &[("target", target), ("source", source)],
            ),
            Self::String { target, source } => output.structure(
                "graph::FunctionCapture::String",
                &[("target", target), ("source", source)],
            ),
            Self::BitArray { target, source } => output.structure(
                "graph::FunctionCapture::BitArray",
                &[("target", target), ("source", source)],
            ),
            Self::UtfCodepoint { target, source } => output.structure(
                "graph::FunctionCapture::UtfCodepoint",
                &[("target", target), ("source", source)],
            ),
            Self::Custom { target, source } => output.structure(
                "graph::FunctionCapture::Custom",
                &[("target", target), ("source", source)],
            ),
            Self::External { target, source } => output.structure(
                "graph::FunctionCapture::External",
                &[("target", target), ("source", source)],
            ),
            Self::Bool { target, source } => output.structure(
                "graph::FunctionCapture::Bool",
                &[("target", target), ("source", source)],
            ),
            Self::Nil { target, source } => output.structure(
                "graph::FunctionCapture::Nil",
                &[("target", target), ("source", source)],
            ),
            Self::Tuple { target, source } => output.structure(
                "graph::FunctionCapture::Tuple",
                &[("target", target), ("source", source)],
            ),
            Self::ParameterList { target, source } => output.structure(
                "graph::FunctionCapture::ParameterList",
                &[("target", target), ("source", source)],
            ),
            Self::ParameterListList { target, source } => output.structure(
                "graph::FunctionCapture::ParameterListList",
                &[("target", target), ("source", source)],
            ),
            Self::IntList { target, source } => output.structure(
                "graph::FunctionCapture::IntList",
                &[("target", target), ("source", source)],
            ),
            Self::StringList { target, source } => output.structure(
                "graph::FunctionCapture::StringList",
                &[("target", target), ("source", source)],
            ),
            Self::BitArrayList { target, source } => output.structure(
                "graph::FunctionCapture::BitArrayList",
                &[("target", target), ("source", source)],
            ),
            Self::UtfCodepointList { target, source } => output.structure(
                "graph::FunctionCapture::UtfCodepointList",
                &[("target", target), ("source", source)],
            ),
            Self::CustomList { target, source } => output.structure(
                "graph::FunctionCapture::CustomList",
                &[("target", target), ("source", source)],
            ),
            Self::ExternalList { target, source } => output.structure(
                "graph::FunctionCapture::ExternalList",
                &[("target", target), ("source", source)],
            ),
            Self::FloatList { target, source } => output.structure(
                "graph::FunctionCapture::FloatList",
                &[("target", target), ("source", source)],
            ),
            Self::BoolList { target, source } => output.structure(
                "graph::FunctionCapture::BoolList",
                &[("target", target), ("source", source)],
            ),
            Self::NilList { target, source } => output.structure(
                "graph::FunctionCapture::NilList",
                &[("target", target), ("source", source)],
            ),
            Self::TupleList { target, source } => output.structure(
                "graph::FunctionCapture::TupleList",
                &[("target", target), ("source", source)],
            ),
            Self::ListList { target, source } => output.structure(
                "graph::FunctionCapture::ListList",
                &[("target", target), ("source", source)],
            ),
            Self::FunctionList { target, source } => output.structure(
                "graph::FunctionCapture::FunctionList",
                &[("target", target), ("source", source)],
            ),
            Self::IntFunction { target, source } => output.structure(
                "graph::FunctionCapture::IntFunction",
                &[("target", target), ("source", source)],
            ),
            Self::FloatFunction { target, source } => output.structure(
                "graph::FunctionCapture::FloatFunction",
                &[("target", target), ("source", source)],
            ),
            Self::StringFunction { target, source } => output.structure(
                "graph::FunctionCapture::StringFunction",
                &[("target", target), ("source", source)],
            ),
            Self::BitArrayFunction { target, source } => output.structure(
                "graph::FunctionCapture::BitArrayFunction",
                &[("target", target), ("source", source)],
            ),
            Self::UtfCodepointFunction { target, source } => output.structure(
                "graph::FunctionCapture::UtfCodepointFunction",
                &[("target", target), ("source", source)],
            ),
            Self::GenericFunction { target, source } => output.structure(
                "graph::FunctionCapture::GenericFunction",
                &[("target", target), ("source", source)],
            ),
            Self::NeverFunction { target, source } => output.structure(
                "graph::FunctionCapture::NeverFunction",
                &[("target", target), ("source", source)],
            ),
            Self::CustomFunction { target, source } => output.structure(
                "graph::FunctionCapture::CustomFunction",
                &[("target", target), ("source", source)],
            ),
            Self::ExternalFunction { target, source } => output.structure(
                "graph::FunctionCapture::ExternalFunction",
                &[("target", target), ("source", source)],
            ),
            Self::BoolFunction { target, source } => output.structure(
                "graph::FunctionCapture::BoolFunction",
                &[("target", target), ("source", source)],
            ),
            Self::NilFunction { target, source } => output.structure(
                "graph::FunctionCapture::NilFunction",
                &[("target", target), ("source", source)],
            ),
            Self::TupleFunction { target, source } => output.structure(
                "graph::FunctionCapture::TupleFunction",
                &[("target", target), ("source", source)],
            ),
            Self::ListFunction { target, source } => output.structure(
                "graph::FunctionCapture::ListFunction",
                &[("target", target), ("source", source)],
            ),
            Self::FunctionFunction { target, source } => output.structure(
                "graph::FunctionCapture::FunctionFunction",
                &[("target", target), ("source", source)],
            ),
        }
    }
}

impl Emit for FunctionInstructionKind {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Constant(field_0) => {
                output.call("graph::FunctionInstructionKind::Constant", &[field_0])
            }
            Self::Reference(field_0) => {
                output.call("graph::FunctionInstructionKind::Reference", &[field_0])
            }
            Self::Closure { target, captures } => output.structure(
                "graph::FunctionInstructionKind::Closure",
                &[("target", target), ("captures", captures)],
            ),
            Self::Constructor(field_0) => {
                output.call("graph::FunctionInstructionKind::Constructor", &[field_0])
            }
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::FunctionInstructionKind::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::FunctionInstructionKind::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::FunctionInstructionKind::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::FunctionInstructionKind::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::FunctionInstructionKind::ListIndex",
                &[("list", list), ("index", index)],
            ),
        }
    }
}

impl Emit for ExternalFunctionInstructionKind {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Reference(field_0) => output.call(
                "graph::ExternalFunctionInstructionKind::Reference",
                &[field_0],
            ),
            Self::Closure { target, captures } => output.structure(
                "graph::ExternalFunctionInstructionKind::Closure",
                &[("target", target), ("captures", captures)],
            ),
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::ExternalFunctionInstructionKind::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::ExternalFunctionInstructionKind::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{FunctionCapture, FunctionTarget};
    use crate::plan::execution::function::{
        BitArrayFunctionId, BoolFunctionId, CustomFunctionId, FloatFunctionId, GenericCallableId,
        IntFunctionFunctionId, IntFunctionId, IntListFunctionId, ListFunctionId, NeverFunctionId,
        NilFunctionId, ProfiledFunctionFunctionId, StringFunctionId, TupleFunctionId,
        UtfCodepointFunctionId,
    };
    use crate::plan::execution::graph::{
        BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
        BoolListLocalId, BoolLocalId, CustomListLocalId, CustomLocal, CustomLocalId,
        ExternalListLocalId, ExternalLocal, ExternalLocalId, FloatFunctionLocalId,
        FloatListLocalId, FloatLocalId, FunctionListLocalId, IntFunctionLocalId, IntListLocalId,
        IntLocalId, ListListLocalId, NilFunctionLocalId, NilListLocalId, NilLocalId,
        ParameterListListLocalId, ParameterListLocalId, StringFunctionLocalId, StringListLocalId,
        StringLocalId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
        UtfCodepointFunctionLocalId, UtfCodepointListLocalId, UtfCodepointLocalId,
    };
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::{
        CustomConstructorId, CustomTypeId, CustomValueShape, CustomValueShapeId, ExternalTypeId,
        IntListTypeId, ListTypeId, ValueShapeId,
    };

    #[test]
    fn emits_callable_captures_with_their_complete_refined_type() {
        use crate::plan::execution::graph::{
            CoreFunctionFunctionLocal, CoreFunctionFunctionLocalId, CustomFunctionLocal,
            CustomFunctionLocalId, ExternalFunctionFunctionLocal, ExternalFunctionFunctionLocalId,
            ExternalFunctionLocal, ExternalFunctionLocalId, FunctionFunctionLocal,
            GenericFunctionLocal, GenericFunctionLocalId, IntListFunctionLocalId,
            ListFunctionLocal, NeverFunctionLocal, NeverFunctionLocalId,
        };
        use crate::plan::execution::type_::{
            CustomFunctionType, ExternalFunctionType, FunctionFunctionType, FunctionShape,
            FunctionType, GenericFunctionType, ValueType,
        };

        let symbolic = FunctionType::new(
            Vec::new(),
            ValueType::Parameter(crate::plan::TypeParameterId(0)),
        );
        let inner = FunctionType::new(Vec::new(), ValueType::Int);
        let cases = [
            (
                FunctionCapture::GenericFunction {
                    target: GenericFunctionLocal {
                        id: GenericFunctionLocalId(2),
                        type_: GenericFunctionType::from_shapes(
                            symbolic.clone(),
                            FunctionShape::new(ValueShapeId(3), symbolic.clone()),
                        ),
                    },
                    source: GenericFunctionLocal {
                        id: GenericFunctionLocalId(5),
                        type_: GenericFunctionType::from_shapes(
                            symbolic.clone(),
                            FunctionShape::new(ValueShapeId(3), symbolic.clone()),
                        ),
                    },
                },
                r#"
data::graph::FunctionCapture::GenericFunction {
    target: data::graph::GenericFunctionLocal {
        id: data::graph::GenericFunctionLocalId(2),
        type_: data::type_::GenericFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
            },
            shape: data::type_::FunctionShape {
                shape_id: data::type_::ValueShapeId(3),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                },
            },
        },
    },
    source: data::graph::GenericFunctionLocal {
        id: data::graph::GenericFunctionLocalId(5),
        type_: data::type_::GenericFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
            },
            shape: data::type_::FunctionShape {
                shape_id: data::type_::ValueShapeId(3),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                },
            },
        },
    },
}"#.trim_start_matches('\n'),
            ),
            (
                FunctionCapture::NeverFunction {
                    target: NeverFunctionLocal {
                        id: NeverFunctionLocalId(2),
                        type_: GenericFunctionType::from_shapes(
                            symbolic.clone(),
                            FunctionShape::new(ValueShapeId(3), symbolic.clone()),
                        ),
                    },
                    source: NeverFunctionLocal {
                        id: NeverFunctionLocalId(5),
                        type_: GenericFunctionType::from_shapes(
                            symbolic.clone(),
                            FunctionShape::new(ValueShapeId(3), symbolic.clone()),
                        ),
                    },
                },
                r#"
data::graph::FunctionCapture::NeverFunction {
    target: data::graph::NeverFunctionLocal {
        id: data::graph::NeverFunctionLocalId(2),
        type_: data::type_::GenericFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
            },
            shape: data::type_::FunctionShape {
                shape_id: data::type_::ValueShapeId(3),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                },
            },
        },
    },
    source: data::graph::NeverFunctionLocal {
        id: data::graph::NeverFunctionLocalId(5),
        type_: data::type_::GenericFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
            },
            shape: data::type_::FunctionShape {
                shape_id: data::type_::ValueShapeId(3),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Parameter(data::type_::parameter_id(0))),
                },
            },
        },
    },
}"#.trim_start_matches('\n'),
            ),
            (
                FunctionCapture::CustomFunction {
                    target: CustomFunctionLocal {
                        id: CustomFunctionLocalId(2),
                        type_: CustomFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::Custom(CustomTypeId(3))),
                            Vec::new(),
                            CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                        ),
                    },
                    source: CustomFunctionLocal {
                        id: CustomFunctionLocalId(5),
                        type_: CustomFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::Custom(CustomTypeId(3))),
                            Vec::new(),
                            CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                        ),
                    },
                },
                r#"
data::graph::FunctionCapture::CustomFunction {
    target: data::graph::CustomFunctionLocal {
        id: data::graph::CustomFunctionLocalId(2),
        type_: data::type_::CustomFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(3))),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::CustomValueShape {
                type_id: data::type_::CustomTypeId(3),
                shape_id: data::type_::CustomValueShapeId(4),
            },
        },
    },
    source: data::graph::CustomFunctionLocal {
        id: data::graph::CustomFunctionLocalId(5),
        type_: data::type_::CustomFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Custom(data::type_::CustomTypeId(3))),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::CustomValueShape {
                type_id: data::type_::CustomTypeId(3),
                shape_id: data::type_::CustomValueShapeId(4),
            },
        },
    },
}"#.trim_start_matches('\n'),
            ),
            (
                FunctionCapture::ExternalFunction {
                    target: ExternalFunctionLocal {
                        id: ExternalFunctionLocalId(2),
                        type_: ExternalFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::External(ExternalTypeId(3))),
                            Vec::new(),
                            ExternalTypeId(3),
                        ),
                    },
                    source: ExternalFunctionLocal {
                        id: ExternalFunctionLocalId(5),
                        type_: ExternalFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::External(ExternalTypeId(3))),
                            Vec::new(),
                            ExternalTypeId(3),
                        ),
                    },
                },
                r#"
data::graph::FunctionCapture::ExternalFunction {
    target: data::graph::ExternalFunctionLocal {
        id: data::graph::ExternalFunctionLocalId(2),
        type_: data::type_::ExternalFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(3))),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::ExternalTypeId(3),
        },
    },
    source: data::graph::ExternalFunctionLocal {
        id: data::graph::ExternalFunctionLocalId(5),
        type_: data::type_::ExternalFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(3))),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::ExternalTypeId(3),
        },
    },
}"#.trim_start_matches('\n'),
            ),
            (
                FunctionCapture::ListFunction {
                    target: ListFunctionLocal::Int {
                        local: IntListFunctionLocalId(2),
                        type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                        list_type: IntListTypeId::new(ListTypeId(3)),
                    },
                    source: ListFunctionLocal::Int {
                        local: IntListFunctionLocalId(5),
                        type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                        list_type: IntListTypeId::new(ListTypeId(3)),
                    },
                },
                r#"
data::graph::FunctionCapture::ListFunction {
    target: data::graph::ListFunctionLocal::Int {
        local: data::graph::IntListFunctionLocalId(2),
        type_: data::type_::FunctionType {
            arguments: data::Storage::Static(&[]),
            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3))),
        },
        list_type: data::type_::IntListTypeId {
            list_type: data::type_::ListTypeId(3),
        },
    },
    source: data::graph::ListFunctionLocal::Int {
        local: data::graph::IntListFunctionLocalId(5),
        type_: data::type_::FunctionType {
            arguments: data::Storage::Static(&[]),
            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3))),
        },
        list_type: data::type_::IntListTypeId {
            list_type: data::type_::ListTypeId(3),
        },
    },
}"#.trim_start_matches('\n'),
            ),
            (
                FunctionCapture::FunctionFunction {
                    target: FunctionFunctionLocal::Core(CoreFunctionFunctionLocal {
                        id: CoreFunctionFunctionLocalId(2),
                        type_: FunctionFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::Function(inner.clone())),
                            Vec::new(),
                            FunctionShape::new(ValueShapeId(7), inner.clone()),
                        ),
                    }),
                    source: FunctionFunctionLocal::Core(CoreFunctionFunctionLocal {
                        id: CoreFunctionFunctionLocalId(5),
                        type_: FunctionFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::Function(inner.clone())),
                            Vec::new(),
                            FunctionShape::new(ValueShapeId(7), inner.clone()),
                        ),
                    }),
                },
                r#"
data::graph::FunctionCapture::FunctionFunction {
    target: data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
        id: data::graph::CoreFunctionFunctionLocalId(2),
        type_: data::type_::FunctionFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                })),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::FunctionShape {
                shape_id: data::type_::ValueShapeId(7),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                },
            },
        },
    }),
    source: data::graph::FunctionFunctionLocal::Core(data::graph::CoreFunctionFunctionLocal {
        id: data::graph::CoreFunctionFunctionLocalId(5),
        type_: data::type_::FunctionFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                })),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::FunctionShape {
                shape_id: data::type_::ValueShapeId(7),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                },
            },
        },
    }),
}"#.trim_start_matches('\n'),
            ),
            (
                FunctionCapture::FunctionFunction {
                    target: FunctionFunctionLocal::External(ExternalFunctionFunctionLocal {
                        id: ExternalFunctionFunctionLocalId(2),
                        type_: FunctionFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::Function(inner.clone())),
                            Vec::new(),
                            FunctionShape::new(ValueShapeId(7), inner.clone()),
                        ),
                    }),
                    source: FunctionFunctionLocal::External(ExternalFunctionFunctionLocal {
                        id: ExternalFunctionFunctionLocalId(5),
                        type_: FunctionFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::Function(inner.clone())),
                            Vec::new(),
                            FunctionShape::new(ValueShapeId(7), inner.clone()),
                        ),
                    }),
                },
                r#"
data::graph::FunctionCapture::FunctionFunction {
    target: data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
        id: data::graph::ExternalFunctionFunctionLocalId(2),
        type_: data::type_::FunctionFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                })),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::FunctionShape {
                shape_id: data::type_::ValueShapeId(7),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                },
            },
        },
    }),
    source: data::graph::FunctionFunctionLocal::External(data::graph::ExternalFunctionFunctionLocal {
        id: data::graph::ExternalFunctionFunctionLocalId(5),
        type_: data::type_::FunctionFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                })),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::FunctionShape {
                shape_id: data::type_::ValueShapeId(7),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                },
            },
        },
    }),
}"#.trim_start_matches('\n'),
            ),
        ];
        for (capture, expected) in cases {
            assert_eq!(Rust::expression(&capture), expected);
        }
    }

    #[test]
    fn emits_function_value_construction_invocation_and_projection() {
        use super::{FunctionInstruction, FunctionInstructionKind};
        use crate::plan::execution::constant::ConstantId;
        use crate::plan::execution::function::FunctionReturnFamily;
        use crate::plan::execution::graph::{
            CoreFunctionFunctionLocal, CoreFunctionFunctionLocalId,
        };
        use crate::plan::execution::type_::{
            FunctionFunctionType, FunctionShape, FunctionType, ValueType,
        };
        use crate::plan::{HostCallSite, SourceSpan};

        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let inner = FunctionType::new(Vec::new(), ValueType::Int);
        let cases = [
            (
                FunctionInstructionKind::Constant(ConstantId::new(2)),
                r#"
data::graph::FunctionInstructionKind::Constant(data::constant::ConstantId {
    index: 2,
    value: ::core::marker::PhantomData,
})"#.trim_start_matches('\n'),
            ),
            (
                FunctionInstructionKind::Reference(FunctionTarget::Int(IntFunctionId(2))),
                "data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(2)))",
            ),
            (
                FunctionInstructionKind::Closure {
                    target: FunctionTarget::Int(IntFunctionId(2)),
                    captures: vec![FunctionCapture::Int {
                        target: IntLocalId(0),
                        source: IntLocalId(5),
                    }]
                    .into(),
                },
                r#"
data::graph::FunctionInstructionKind::Closure {
    target: data::graph::FunctionTarget::Int(data::function::IntFunctionId(2)),
    captures: data::Storage::Static(&[
        data::graph::FunctionCapture::Int {
            target: data::graph::IntLocalId(0),
            source: data::graph::IntLocalId(5),
        },
    ]),
}"#.trim_start_matches('\n'),
            ),
            (
                FunctionInstructionKind::Constructor(CustomConstructorId::new(CustomTypeId(3), 1)),
                r#"
data::graph::FunctionInstructionKind::Constructor(data::type_::CustomConstructorId {
    type_id: data::type_::CustomTypeId(3),
    index: 1,
})"#.trim_start_matches('\n'),
            ),
            (
                FunctionInstructionKind::Call {
                    function: ProfiledFunctionFunctionId::Int(IntFunctionFunctionId(2)),
                    args: Vec::new().into(),
                    site: site.clone(),
                },
                r#"
data::graph::FunctionInstructionKind::Call {
    function: data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(2)),
    args: data::Storage::Static(&[]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                FunctionInstructionKind::FunctionCall {
                    function: CoreFunctionFunctionLocal {
                        id: CoreFunctionFunctionLocalId(2),
                        type_: FunctionFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::Function(inner.clone())),
                            Vec::new(),
                            FunctionShape::new(ValueShapeId(7), inner.clone()),
                        ),
                    },
                    args: Vec::new().into(),
                    site,
                },
                r#"
data::graph::FunctionInstructionKind::FunctionCall {
    function: data::graph::CoreFunctionFunctionLocal {
        id: data::graph::CoreFunctionFunctionLocalId(2),
        type_: data::type_::FunctionFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                })),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::FunctionShape {
                shape_id: data::type_::ValueShapeId(7),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::Int),
                },
            },
        },
    },
    args: data::Storage::Static(&[]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                FunctionInstructionKind::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                r#"
data::graph::FunctionInstructionKind::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                FunctionInstructionKind::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                r#"
data::graph::FunctionInstructionKind::CustomField {
    source: data::graph::CustomLocal {
        id: data::graph::CustomLocalId(2),
        shape: data::type_::CustomValueShape {
            type_id: data::type_::CustomTypeId(3),
            shape_id: data::type_::CustomValueShapeId(4),
        },
    },
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                FunctionInstructionKind::ListIndex {
                    list: FunctionListLocalId(2),
                    index: 1,
                },
                r#"
data::graph::FunctionInstructionKind::ListIndex {
    list: data::graph::FunctionListLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
        ];
        for (kind, expected) in cases {
            assert_eq!(Rust::expression(&kind), expected);
        }
        let instruction = FunctionInstruction::new(
            inner,
            FunctionReturnFamily::Int,
            FunctionInstructionKind::Reference(FunctionTarget::Int(IntFunctionId(2))),
        );
        assert_eq!(
            Rust::expression(&instruction),
            r#"
data::graph::FunctionInstruction {
    type_: data::type_::FunctionType {
        arguments: data::Storage::Static(&[]),
        return_: data::Storage::Static(&data::type_::ValueType::Int),
    },
    family: data::function::FunctionReturnFamily::Int,
    kind: data::graph::FunctionInstructionKind::Reference(data::graph::FunctionTarget::Int(data::function::IntFunctionId(2))),
}"#.trim_start_matches('\n')
        );
    }

    #[test]
    fn emits_external_function_targets_and_completion_instructions() {
        use super::{
            ExternalFunctionCallTarget, ExternalFunctionInstruction,
            ExternalFunctionInstructionKind, ExternalFunctionTarget,
        };
        use crate::plan::execution::function::{
            ExternalFunctionFunctionId, ExternalFunctionId, ExternalListFunctionFunctionId,
            ExternalListFunctionId, FunctionReturnFamily, RuntimeFunctionFunctionTarget,
        };
        use crate::plan::execution::graph::{
            ExternalFunctionFunctionLocal, ExternalFunctionFunctionLocalId,
        };
        use crate::plan::execution::type_::{
            ExternalFunctionType, ExternalListTypeId, FunctionFunctionType, FunctionShape,
            FunctionType, ValueType,
        };
        use crate::plan::{HostCallSite, SourceSpan};

        let inner = FunctionType::new(Vec::new(), ValueType::External(ExternalTypeId(3)));
        let targets = [
            (
                ExternalFunctionTarget::Value(ExternalFunctionId::new(2, ExternalTypeId(3))),
                r#"
data::graph::ExternalFunctionTarget::Value(data::function::ExternalFunctionId {
    index: 2,
    return_type: data::type_::ExternalTypeId(3),
})"#.trim_start_matches('\n'),
            ),
            (
                ExternalFunctionTarget::List(ExternalListFunctionId::new(
                    2,
                    ExternalListTypeId::new(ListTypeId(3), ExternalTypeId(4)),
                )),
                r#"
data::graph::ExternalFunctionTarget::List(data::function::ExternalListFunctionId {
    index: 2,
    type_id: data::type_::ExternalListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ExternalTypeId(4),
    },
})"#.trim_start_matches('\n'),
            ),
            (
                ExternalFunctionTarget::Function(ExternalFunctionFunctionId {
                    index: 2,
                    type_: ExternalFunctionType::from_shapes(
                        inner.clone(),
                        Vec::new(),
                        ExternalTypeId(3),
                    ),
                }),
                r#"
data::graph::ExternalFunctionTarget::Function(data::function::ExternalFunctionFunctionId {
    index: 2,
    type_: data::type_::ExternalFunctionType {
        type_: data::type_::FunctionType {
            arguments: data::Storage::Static(&[]),
            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(3))),
        },
        arguments: data::Storage::Static(&[]),
        return_: data::type_::ExternalTypeId(3),
    },
})"#.trim_start_matches('\n'),
            ),
            (
                ExternalFunctionTarget::ListFunction {
                    id: ExternalListFunctionFunctionId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: ExternalListTypeId::new(ListTypeId(3), ExternalTypeId(4)),
                },
                r#"
data::graph::ExternalFunctionTarget::ListFunction {
    id: data::function::ExternalListFunctionFunctionId(2),
    type_: data::type_::FunctionType {
        arguments: data::Storage::Static(&[]),
        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3))),
    },
    list_type: data::type_::ExternalListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ExternalTypeId(4),
    },
}"#.trim_start_matches('\n'),
            ),
        ];
        for (target, expected) in targets {
            assert_eq!(Rust::expression(&target), expected);
        }
        let calls = [
            (
                ExternalFunctionCallTarget::Function(ExternalFunctionFunctionId {
                    index: 2,
                    type_: ExternalFunctionType::from_shapes(
                        inner.clone(),
                        Vec::new(),
                        ExternalTypeId(3),
                    ),
                }),
                r#"
data::graph::ExternalFunctionCallTarget::Function(data::function::ExternalFunctionFunctionId {
    index: 2,
    type_: data::type_::ExternalFunctionType {
        type_: data::type_::FunctionType {
            arguments: data::Storage::Static(&[]),
            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(3))),
        },
        arguments: data::Storage::Static(&[]),
        return_: data::type_::ExternalTypeId(3),
    },
})"#.trim_start_matches('\n'),
            ),
            (
                ExternalFunctionCallTarget::ListFunction {
                    id: ExternalListFunctionFunctionId(2),
                    type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                    list_type: ExternalListTypeId::new(ListTypeId(3), ExternalTypeId(4)),
                },
                r#"
data::graph::ExternalFunctionCallTarget::ListFunction {
    id: data::function::ExternalListFunctionFunctionId(2),
    type_: data::type_::FunctionType {
        arguments: data::Storage::Static(&[]),
        return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3))),
    },
    list_type: data::type_::ExternalListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ExternalTypeId(4),
    },
}"#.trim_start_matches('\n'),
            ),
        ];
        for (target, expected) in calls {
            assert_eq!(Rust::expression(&target), expected);
        }
        assert_eq!(
            Rust::expression(&RuntimeFunctionFunctionTarget::<crate::plan::execution::function::GenericFunctionFunctionId>::External(
                ExternalFunctionCallTarget::Function(ExternalFunctionFunctionId {
                    index: 2,
                    type_: ExternalFunctionType::from_shapes(
                        inner.clone(),
                        Vec::new(),
                        ExternalTypeId(3)
                    )
                })
            )),
            r#"
data::function::RuntimeFunctionFunctionTarget::External(data::graph::ExternalFunctionCallTarget::Function(data::function::ExternalFunctionFunctionId {
    index: 2,
    type_: data::type_::ExternalFunctionType {
        type_: data::type_::FunctionType {
            arguments: data::Storage::Static(&[]),
            return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(3))),
        },
        arguments: data::Storage::Static(&[]),
        return_: data::type_::ExternalTypeId(3),
    },
}))"#.trim_start_matches('\n')
        );
        let site = HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8));
        let kinds = [
            (
                ExternalFunctionInstructionKind::Reference(ExternalFunctionTarget::Value(
                    ExternalFunctionId::new(2, ExternalTypeId(3)),
                )),
                r#"
data::graph::ExternalFunctionInstructionKind::Reference(data::graph::ExternalFunctionTarget::Value(data::function::ExternalFunctionId {
    index: 2,
    return_type: data::type_::ExternalTypeId(3),
}))"#.trim_start_matches('\n'),
            ),
            (
                ExternalFunctionInstructionKind::Closure {
                    target: ExternalFunctionTarget::Value(ExternalFunctionId::new(
                        2,
                        ExternalTypeId(3),
                    )),
                    captures: vec![FunctionCapture::Int {
                        target: IntLocalId(0),
                        source: IntLocalId(5),
                    }]
                    .into(),
                },
                r#"
data::graph::ExternalFunctionInstructionKind::Closure {
    target: data::graph::ExternalFunctionTarget::Value(data::function::ExternalFunctionId {
        index: 2,
        return_type: data::type_::ExternalTypeId(3),
    }),
    captures: data::Storage::Static(&[
        data::graph::FunctionCapture::Int {
            target: data::graph::IntLocalId(0),
            source: data::graph::IntLocalId(5),
        },
    ]),
}"#.trim_start_matches('\n'),
            ),
            (
                ExternalFunctionInstructionKind::Call {
                    function: ExternalFunctionCallTarget::Function(ExternalFunctionFunctionId {
                        index: 2,
                        type_: ExternalFunctionType::from_shapes(
                            inner.clone(),
                            Vec::new(),
                            ExternalTypeId(3),
                        ),
                    }),
                    args: Vec::new().into(),
                    site: site.clone(),
                },
                r#"
data::graph::ExternalFunctionInstructionKind::Call {
    function: data::graph::ExternalFunctionCallTarget::Function(data::function::ExternalFunctionFunctionId {
        index: 2,
        type_: data::type_::ExternalFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(3))),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::ExternalTypeId(3),
        },
    }),
    args: data::Storage::Static(&[]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                ExternalFunctionInstructionKind::FunctionCall {
                    function: ExternalFunctionFunctionLocal {
                        id: ExternalFunctionFunctionLocalId(2),
                        type_: FunctionFunctionType::from_shapes(
                            FunctionType::new(Vec::new(), ValueType::Function(inner.clone())),
                            Vec::new(),
                            FunctionShape::new(ValueShapeId(7), inner.clone()),
                        ),
                    },
                    args: Vec::new().into(),
                    site,
                },
                r#"
data::graph::ExternalFunctionInstructionKind::FunctionCall {
    function: data::graph::ExternalFunctionFunctionLocal {
        id: data::graph::ExternalFunctionFunctionLocalId(2),
        type_: data::type_::FunctionFunctionType {
            type_: data::type_::FunctionType {
                arguments: data::Storage::Static(&[]),
                return_: data::Storage::Static(&data::type_::ValueType::Function(data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(3))),
                })),
            },
            arguments: data::Storage::Static(&[]),
            return_: data::type_::FunctionShape {
                shape_id: data::type_::ValueShapeId(7),
                type_: data::type_::FunctionType {
                    arguments: data::Storage::Static(&[]),
                    return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(3))),
                },
            },
        },
    },
    args: data::Storage::Static(&[]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
        ];
        for (kind, expected) in kinds {
            assert_eq!(Rust::expression(&kind), expected);
        }
        let instruction = ExternalFunctionInstruction::new(
            inner,
            FunctionReturnFamily::External,
            ExternalFunctionInstructionKind::Reference(ExternalFunctionTarget::Value(
                ExternalFunctionId::new(2, ExternalTypeId(3)),
            )),
        );
        assert_eq!(
            Rust::expression(&instruction),
            r#"
data::graph::ExternalFunctionInstruction {
    type_: data::type_::FunctionType {
        arguments: data::Storage::Static(&[]),
        return_: data::Storage::Static(&data::type_::ValueType::External(data::type_::ExternalTypeId(3))),
    },
    family: data::function::FunctionReturnFamily::External,
    kind: data::graph::ExternalFunctionInstructionKind::Reference(data::graph::ExternalFunctionTarget::Value(data::function::ExternalFunctionId {
        index: 2,
        return_type: data::type_::ExternalTypeId(3),
    })),
}"#.trim_start_matches('\n')
        );
    }

    #[test]
    fn emits_capture_source_and_destination_without_conflating_families() {
        let cases = [
            (
                FunctionCapture::Int {
                    target: IntLocalId(2),
                    source: IntLocalId(5),
                },
                r#"
data::graph::FunctionCapture::Int {
    target: data::graph::IntLocalId(2),
    source: data::graph::IntLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::Float {
                    target: FloatLocalId(2),
                    source: FloatLocalId(5),
                },
                r#"
data::graph::FunctionCapture::Float {
    target: data::graph::FloatLocalId(2),
    source: data::graph::FloatLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::String {
                    target: StringLocalId(2),
                    source: StringLocalId(5),
                },
                r#"
data::graph::FunctionCapture::String {
    target: data::graph::StringLocalId(2),
    source: data::graph::StringLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::BitArray {
                    target: BitArrayLocalId(2),
                    source: BitArrayLocalId(5),
                },
                r#"
data::graph::FunctionCapture::BitArray {
    target: data::graph::BitArrayLocalId(2),
    source: data::graph::BitArrayLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::UtfCodepoint {
                    target: UtfCodepointLocalId(2),
                    source: UtfCodepointLocalId(5),
                },
                r#"
data::graph::FunctionCapture::UtfCodepoint {
    target: data::graph::UtfCodepointLocalId(2),
    source: data::graph::UtfCodepointLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::Bool {
                    target: BoolLocalId(2),
                    source: BoolLocalId(5),
                },
                r#"
data::graph::FunctionCapture::Bool {
    target: data::graph::BoolLocalId(2),
    source: data::graph::BoolLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::Nil {
                    target: NilLocalId(2),
                    source: NilLocalId(5),
                },
                r#"
data::graph::FunctionCapture::Nil {
    target: data::graph::NilLocalId(2),
    source: data::graph::NilLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::Tuple {
                    target: TupleLocalId(2),
                    source: TupleLocalId(5),
                },
                r#"
data::graph::FunctionCapture::Tuple {
    target: data::graph::TupleLocalId(2),
    source: data::graph::TupleLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::ParameterList {
                    target: ParameterListLocalId(2),
                    source: ParameterListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::ParameterList {
    target: data::graph::ParameterListLocalId(2),
    source: data::graph::ParameterListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::ParameterListList {
                    target: ParameterListListLocalId(2),
                    source: ParameterListListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::ParameterListList {
    target: data::graph::ParameterListListLocalId(2),
    source: data::graph::ParameterListListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::IntList {
                    target: IntListLocalId(2),
                    source: IntListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::IntList {
    target: data::graph::IntListLocalId(2),
    source: data::graph::IntListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::StringList {
                    target: StringListLocalId(2),
                    source: StringListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::StringList {
    target: data::graph::StringListLocalId(2),
    source: data::graph::StringListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::BitArrayList {
                    target: BitArrayListLocalId(2),
                    source: BitArrayListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::BitArrayList {
    target: data::graph::BitArrayListLocalId(2),
    source: data::graph::BitArrayListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::UtfCodepointList {
                    target: UtfCodepointListLocalId(2),
                    source: UtfCodepointListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::UtfCodepointList {
    target: data::graph::UtfCodepointListLocalId(2),
    source: data::graph::UtfCodepointListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::CustomList {
                    target: CustomListLocalId(2),
                    source: CustomListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::CustomList {
    target: data::graph::CustomListLocalId(2),
    source: data::graph::CustomListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::ExternalList {
                    target: ExternalListLocalId(2),
                    source: ExternalListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::ExternalList {
    target: data::graph::ExternalListLocalId(2),
    source: data::graph::ExternalListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::FloatList {
                    target: FloatListLocalId(2),
                    source: FloatListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::FloatList {
    target: data::graph::FloatListLocalId(2),
    source: data::graph::FloatListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::BoolList {
                    target: BoolListLocalId(2),
                    source: BoolListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::BoolList {
    target: data::graph::BoolListLocalId(2),
    source: data::graph::BoolListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::NilList {
                    target: NilListLocalId(2),
                    source: NilListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::NilList {
    target: data::graph::NilListLocalId(2),
    source: data::graph::NilListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::TupleList {
                    target: TupleListLocalId(2),
                    source: TupleListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::TupleList {
    target: data::graph::TupleListLocalId(2),
    source: data::graph::TupleListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::ListList {
                    target: ListListLocalId(2),
                    source: ListListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::ListList {
    target: data::graph::ListListLocalId(2),
    source: data::graph::ListListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::FunctionList {
                    target: FunctionListLocalId(2),
                    source: FunctionListLocalId(5),
                },
                r#"
data::graph::FunctionCapture::FunctionList {
    target: data::graph::FunctionListLocalId(2),
    source: data::graph::FunctionListLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::IntFunction {
                    target: IntFunctionLocalId(2),
                    source: IntFunctionLocalId(5),
                },
                r#"
data::graph::FunctionCapture::IntFunction {
    target: data::graph::IntFunctionLocalId(2),
    source: data::graph::IntFunctionLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::FloatFunction {
                    target: FloatFunctionLocalId(2),
                    source: FloatFunctionLocalId(5),
                },
                r#"
data::graph::FunctionCapture::FloatFunction {
    target: data::graph::FloatFunctionLocalId(2),
    source: data::graph::FloatFunctionLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::StringFunction {
                    target: StringFunctionLocalId(2),
                    source: StringFunctionLocalId(5),
                },
                r#"
data::graph::FunctionCapture::StringFunction {
    target: data::graph::StringFunctionLocalId(2),
    source: data::graph::StringFunctionLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::BitArrayFunction {
                    target: BitArrayFunctionLocalId(2),
                    source: BitArrayFunctionLocalId(5),
                },
                r#"
data::graph::FunctionCapture::BitArrayFunction {
    target: data::graph::BitArrayFunctionLocalId(2),
    source: data::graph::BitArrayFunctionLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::UtfCodepointFunction {
                    target: UtfCodepointFunctionLocalId(2),
                    source: UtfCodepointFunctionLocalId(5),
                },
                r#"
data::graph::FunctionCapture::UtfCodepointFunction {
    target: data::graph::UtfCodepointFunctionLocalId(2),
    source: data::graph::UtfCodepointFunctionLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::BoolFunction {
                    target: BoolFunctionLocalId(2),
                    source: BoolFunctionLocalId(5),
                },
                r#"
data::graph::FunctionCapture::BoolFunction {
    target: data::graph::BoolFunctionLocalId(2),
    source: data::graph::BoolFunctionLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::NilFunction {
                    target: NilFunctionLocalId(2),
                    source: NilFunctionLocalId(5),
                },
                r#"
data::graph::FunctionCapture::NilFunction {
    target: data::graph::NilFunctionLocalId(2),
    source: data::graph::NilFunctionLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::TupleFunction {
                    target: TupleFunctionLocalId(2),
                    source: TupleFunctionLocalId(5),
                },
                r#"
data::graph::FunctionCapture::TupleFunction {
    target: data::graph::TupleFunctionLocalId(2),
    source: data::graph::TupleFunctionLocalId(5),
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::Custom {
                    target: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    source: CustomLocal::new(
                        CustomLocalId(5),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                },
                r#"
data::graph::FunctionCapture::Custom {
    target: data::graph::CustomLocal {
        id: data::graph::CustomLocalId(2),
        shape: data::type_::CustomValueShape {
            type_id: data::type_::CustomTypeId(3),
            shape_id: data::type_::CustomValueShapeId(4),
        },
    },
    source: data::graph::CustomLocal {
        id: data::graph::CustomLocalId(5),
        shape: data::type_::CustomValueShape {
            type_id: data::type_::CustomTypeId(3),
            shape_id: data::type_::CustomValueShapeId(4),
        },
    },
}"#
                .trim_start_matches('\n'),
            ),
            (
                FunctionCapture::External {
                    target: ExternalLocal::new(ExternalLocalId(2), ExternalTypeId(3)),
                    source: ExternalLocal::new(ExternalLocalId(5), ExternalTypeId(3)),
                },
                r#"
data::graph::FunctionCapture::External {
    target: data::graph::ExternalLocal {
        id: data::graph::ExternalLocalId(2),
        type_id: data::type_::ExternalTypeId(3),
    },
    source: data::graph::ExternalLocal {
        id: data::graph::ExternalLocalId(5),
        type_id: data::type_::ExternalTypeId(3),
    },
}"#
                .trim_start_matches('\n'),
            ),
        ];
        for (capture, expected) in cases {
            assert_eq!(Rust::expression(&capture), expected);
        }
    }

    #[test]
    fn emits_direct_symbolic_and_nested_function_targets() {
        let cases = [
            (
                FunctionTarget::Never(NeverFunctionId(2)),
                "data::graph::FunctionTarget::Never(data::function::NeverFunctionId(2))",
            ),
            (
                FunctionTarget::Int(IntFunctionId(2)),
                "data::graph::FunctionTarget::Int(data::function::IntFunctionId(2))",
            ),
            (
                FunctionTarget::Float(FloatFunctionId(2)),
                "data::graph::FunctionTarget::Float(data::function::FloatFunctionId(2))",
            ),
            (
                FunctionTarget::String(StringFunctionId(2)),
                "data::graph::FunctionTarget::String(data::function::StringFunctionId(2))",
            ),
            (
                FunctionTarget::BitArray(BitArrayFunctionId(2)),
                "data::graph::FunctionTarget::BitArray(data::function::BitArrayFunctionId(2))",
            ),
            (
                FunctionTarget::UtfCodepoint(UtfCodepointFunctionId(2)),
                "data::graph::FunctionTarget::UtfCodepoint(data::function::UtfCodepointFunctionId(2))",
            ),
            (
                FunctionTarget::Bool(BoolFunctionId(2)),
                "data::graph::FunctionTarget::Bool(data::function::BoolFunctionId(2))",
            ),
            (
                FunctionTarget::Nil(NilFunctionId(2)),
                "data::graph::FunctionTarget::Nil(data::function::NilFunctionId(2))",
            ),
            (
                FunctionTarget::Tuple(TupleFunctionId(2)),
                "data::graph::FunctionTarget::Tuple(data::function::TupleFunctionId(2))",
            ),
            (
                FunctionTarget::Generic(GenericCallableId::function(2, vec![ValueShapeId(3)])),
                r#"
data::graph::FunctionTarget::Generic(data::function::GenericCallableId::Function {
    template: 2,
    substitution: data::Storage::Static(&[
        data::type_::ValueShapeId(3),
    ]),
})"#.trim_start_matches('\n'),
            ),
            (
                FunctionTarget::Generic(GenericCallableId::constructor(CustomConstructorId::new(
                    CustomTypeId(3),
                    2,
                ))),
                r#"
data::graph::FunctionTarget::Generic(data::function::GenericCallableId::Constructor(data::type_::CustomConstructorId {
    type_id: data::type_::CustomTypeId(3),
    index: 2,
}))"#.trim_start_matches('\n'),
            ),
            (
                FunctionTarget::Custom(CustomFunctionId::new(
                    2,
                    CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                )),
                r#"
data::graph::FunctionTarget::Custom(data::function::CustomFunctionId {
    index: 2,
    return_shape: data::type_::CustomValueShape {
        type_id: data::type_::CustomTypeId(3),
        shape_id: data::type_::CustomValueShapeId(4),
    },
})"#.trim_start_matches('\n'),
            ),
            (
                FunctionTarget::List(ListFunctionId::Int(IntListFunctionId::new(
                    2,
                    IntListTypeId::new(ListTypeId(3)),
                ))),
                r#"
data::graph::FunctionTarget::List(data::function::ListFunctionId::Int(data::function::IntListFunctionId {
    index: 2,
    type_id: data::type_::IntListTypeId {
        list_type: data::type_::ListTypeId(3),
    },
}))"#.trim_start_matches('\n'),
            ),
            (
                FunctionTarget::Function(ProfiledFunctionFunctionId::Int(IntFunctionFunctionId(2))),
                "data::graph::FunctionTarget::Function(data::function::ProfiledFunctionFunctionId::Int(data::function::IntFunctionFunctionId(2)))",
            ),
        ];
        for (target, expected) in cases {
            assert_eq!(Rust::expression(&target), expected);
        }
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
                captures: vec![FunctionCapture::Int {
                    target: IntLocalId(1),
                    source: IntLocalId(0),
                }]
                .into(),
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
                args: vec![ParamLocal::Int(IntLocalId(2))].into(),
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
                        FunctionType::new(Vec::new(), ValueType::Function(function_type.clone())),
                        Vec::new(),
                        FunctionShape::new(ValueShapeId::new(0), function_type),
                    ),
                ),
                args: vec![ParamLocal::Int(IntLocalId(3))].into(),
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
