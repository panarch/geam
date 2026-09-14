use super::super::super::{FunctionLocal, StoredListLocal};
use super::{write_call, write_constant, write_function_call, write_projection};
use crate::plan::execution::constant::ConstantId;
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::function::FunctionLabelSource;
use crate::plan::execution::function::{
    BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, FloatListFunctionId,
    FunctionListFunctionId, IntListFunctionId, ListListFunctionId, NilListFunctionId,
    ParameterListFunctionId, ParameterListListFunctionId, StringListFunctionId,
    TupleListFunctionId, UtfCodepointListFunctionId,
};
use crate::plan::execution::graph::LocalLabel;
use crate::plan::execution::graph::{
    BitArrayListLocalId, BoolListLocalId, CustomListLocalId, CustomLocal, ExternalListLocalId,
    ExternalLocal, FloatListLocalId, FloatLocalId, FunctionListLocalId, IntListLocalId, IntLocalId,
    ListFunctionLocal, ListListLocalId, NilListLocalId, ParamLocal, ParameterListListLocalId,
    ParameterListLocalId, StringListLocalId, StringLocalId, TupleListLocalId, TupleLocalId,
    UtfCodepointListLocalId, UtfCodepointLocalId,
};
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, CustomListTypeId, ExternalListTypeId, FloatListTypeId,
    FunctionListTypeId, IntListTypeId, ListListTypeId, NilListTypeId, ParameterListListTypeId,
    ParameterListTypeId, StringListTypeId, TupleListTypeId, UtfCodepointListTypeId,
};

#[derive(Clone)]
pub enum ParameterListInstruction {
    Empty,
    Constant(ConstantId<ParameterListLocalId>),
    Call {
        function: ParameterListFunctionId,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: ListFunctionLocal,
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
        list: ParameterListListLocalId,
        index: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypedListInstruction<Element: 'static, Local, Function, FunctionLocal = ListFunctionLocal>
{
    Value(Table<Element>),
    Constant(ConstantId<Local>),
    Spread {
        elements: Table<Element>,
        tail: Local,
    },
    Call {
        function: Function,
        args: Table<ParamLocal>,
        site: crate::plan::HostCallSite,
    },
    FunctionCall {
        function: FunctionLocal,
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
        list: ListListLocalId,
        index: usize,
    },
    DropFirst {
        list: Local,
        count: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternalListInstruction {
    pub type_id: ExternalListTypeId,
    pub instruction: TypedListInstruction<
        ExternalLocal,
        ExternalListLocalId,
        crate::plan::execution::function::ExternalListFunctionId,
        crate::plan::execution::graph::ExternalListFunctionLocalId,
    >,
}

pub trait ExternalListInstructionView {
    type Function;
    type FunctionLocal;

    fn type_id(&self) -> ExternalListTypeId;

    fn instruction(
        &self,
    ) -> &TypedListInstruction<
        ExternalLocal,
        ExternalListLocalId,
        Self::Function,
        Self::FunctionLocal,
    >;
}

#[derive(Clone)]
pub enum ListInstruction {
    Parameter(ParameterListTypeId, ParameterListInstruction),
    ParameterList(
        ParameterListListTypeId,
        TypedListInstruction<
            ParameterListLocalId,
            ParameterListListLocalId,
            ParameterListListFunctionId,
        >,
    ),
    Int(
        IntListTypeId,
        TypedListInstruction<IntLocalId, IntListLocalId, IntListFunctionId>,
    ),
    String(
        StringListTypeId,
        TypedListInstruction<StringLocalId, StringListLocalId, StringListFunctionId>,
    ),
    BitArray(
        BitArrayListTypeId,
        TypedListInstruction<
            crate::plan::execution::graph::BitArrayLocalId,
            BitArrayListLocalId,
            BitArrayListFunctionId,
        >,
    ),
    UtfCodepoint(
        UtfCodepointListTypeId,
        TypedListInstruction<
            UtfCodepointLocalId,
            UtfCodepointListLocalId,
            UtfCodepointListFunctionId,
        >,
    ),
    Custom(
        CustomListTypeId,
        TypedListInstruction<CustomLocal, CustomListLocalId, CustomListFunctionId>,
    ),
    Float(
        FloatListTypeId,
        TypedListInstruction<FloatLocalId, FloatListLocalId, FloatListFunctionId>,
    ),
    Bool(
        BoolListTypeId,
        TypedListInstruction<
            crate::plan::execution::graph::BoolLocalId,
            BoolListLocalId,
            BoolListFunctionId,
        >,
    ),
    Nil(
        NilListTypeId,
        TypedListInstruction<
            crate::plan::execution::graph::NilLocalId,
            NilListLocalId,
            NilListFunctionId,
        >,
    ),
    Tuple(
        TupleListTypeId,
        TypedListInstruction<TupleLocalId, TupleListLocalId, TupleListFunctionId>,
    ),
    List(
        ListListTypeId,
        TypedListInstruction<StoredListLocal, ListListLocalId, ListListFunctionId>,
    ),
    Function(
        FunctionListTypeId,
        TypedListInstruction<FunctionLocal, FunctionListLocalId, FunctionListFunctionId>,
    ),
}

impl Explain for ListInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        let output = context.output();
        match self {
            Self::Parameter(type_id, instruction) => {
                output.push_str("list.parameter[type#");
                output.push_str(&type_id.list_type().index().to_string());
                output.push_str("] ");
                write_parameter(output, instruction);
            }
            Self::ParameterList(type_id, instruction) => write_typed(
                output,
                "parameter_list",
                type_id.list_type().index(),
                instruction,
            ),
            Self::Int(type_id, instruction) => {
                write_typed(output, "int", type_id.list_type().index(), instruction);
            }
            Self::String(type_id, instruction) => {
                write_typed(output, "string", type_id.list_type().index(), instruction);
            }
            Self::BitArray(type_id, instruction) => {
                write_typed(
                    output,
                    "bit_array",
                    type_id.list_type().index(),
                    instruction,
                );
            }
            Self::UtfCodepoint(type_id, instruction) => write_typed(
                output,
                "utf_codepoint",
                type_id.list_type().index(),
                instruction,
            ),
            Self::Custom(type_id, instruction) => {
                write_typed(output, "custom", type_id.list_type().index(), instruction);
            }
            Self::Float(type_id, instruction) => {
                write_typed(output, "float", type_id.list_type().index(), instruction);
            }
            Self::Bool(type_id, instruction) => {
                write_typed(output, "bool", type_id.list_type().index(), instruction);
            }
            Self::Nil(type_id, instruction) => {
                write_typed(output, "nil", type_id.list_type().index(), instruction);
            }
            Self::Tuple(type_id, instruction) => {
                write_typed(output, "tuple", type_id.list_type().index(), instruction);
            }
            Self::List(type_id, instruction) => {
                write_typed(output, "list", type_id.list_type().index(), instruction);
            }
            Self::Function(type_id, instruction) => {
                write_typed(output, "function", type_id.list_type().index(), instruction);
            }
        }
    }
}

impl ExternalListInstruction {
    pub(in crate::plan::execution) fn new(
        type_id: ExternalListTypeId,
        instruction: TypedListInstruction<
            ExternalLocal,
            ExternalListLocalId,
            crate::plan::execution::function::ExternalListFunctionId,
            crate::plan::execution::graph::ExternalListFunctionLocalId,
        >,
    ) -> Self {
        Self {
            type_id,
            instruction,
        }
    }
}

impl ExternalListInstructionView for ExternalListInstruction {
    type Function = crate::plan::execution::function::ExternalListFunctionId;
    type FunctionLocal = crate::plan::execution::graph::ExternalListFunctionLocalId;

    fn type_id(&self) -> ExternalListTypeId {
        self.type_id
    }

    fn instruction(
        &self,
    ) -> &TypedListInstruction<
        ExternalLocal,
        ExternalListLocalId,
        Self::Function,
        Self::FunctionLocal,
    > {
        &self.instruction
    }
}

impl ExternalListInstructionView for std::convert::Infallible {
    type Function = std::convert::Infallible;
    type FunctionLocal = std::convert::Infallible;

    fn type_id(&self) -> ExternalListTypeId {
        match *self {}
    }

    fn instruction(
        &self,
    ) -> &TypedListInstruction<
        ExternalLocal,
        ExternalListLocalId,
        Self::Function,
        Self::FunctionLocal,
    > {
        match *self {}
    }
}

impl Explain for ExternalListInstruction {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        write_typed(
            context.output(),
            "external",
            self.type_id.list_type().index(),
            &self.instruction,
        );
    }
}

fn write_parameter(output: &mut String, instruction: &ParameterListInstruction) {
    match instruction {
        ParameterListInstruction::Empty => output.push_str("empty"),
        ParameterListInstruction::Constant(id) => write_constant(output, "list.parameter", *id),
        ParameterListInstruction::Call { function, args, .. } => {
            write_call(output, "call", function, args);
        }
        ParameterListInstruction::FunctionCall { function, args, .. } => {
            write_function_call(output, "function_call", function, args);
        }
        ParameterListInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "tuple_index", tuple, *index);
        }
        ParameterListInstruction::CustomField { source, index } => {
            write_projection(output, "custom_field", source, *index);
        }
        ParameterListInstruction::ListIndex { list, index } => {
            write_projection(output, "list_index", list, *index);
        }
    }
}

fn write_typed<Element, Local, Function, FunctionLocal>(
    output: &mut String,
    family: &'static str,
    type_id: usize,
    instruction: &TypedListInstruction<Element, Local, Function, FunctionLocal>,
) where
    Element: LocalLabel,
    Local: LocalLabel,
    Function: FunctionLabelSource,
    FunctionLocal: LocalLabel,
{
    output.push_str("list.");
    output.push_str(family);
    output.push_str("[type#");
    output.push_str(&type_id.to_string());
    output.push_str("] ");
    match instruction {
        TypedListInstruction::Value(elements) => {
            output.push_str("value elements=");
            write_list_values(output, elements);
        }
        TypedListInstruction::Constant(id) => {
            write_constant(output, &format!("list.{family}"), *id);
        }
        TypedListInstruction::Spread { elements, tail } => {
            output.push_str("spread elements=");
            write_list_values(output, elements);
            output.push_str(" tail=");
            tail.write_local_label(output);
        }
        TypedListInstruction::Call { function, args, .. } => {
            write_call(output, "call", function, args);
        }
        TypedListInstruction::FunctionCall { function, args, .. } => {
            write_function_call(output, "function_call", function, args);
        }
        TypedListInstruction::TupleIndex { tuple, index } => {
            write_projection(output, "tuple_index", tuple, *index);
        }
        TypedListInstruction::CustomField { source, index } => {
            write_projection(output, "custom_field", source, *index);
        }
        TypedListInstruction::ListIndex { list, index } => {
            write_projection(output, "list_index", list, *index);
        }
        TypedListInstruction::DropFirst { list, count } => {
            output.push_str("drop_first ");
            list.write_local_label(output);
            output.push_str(" count=");
            output.push_str(&count.to_string());
        }
    }
}

fn write_list_values<Value: LocalLabel>(output: &mut String, values: &[Value]) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        value.write_local_label(output);
    }
    output.push(']');
}

impl Emit for ParameterListInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Empty => output.path("graph::ParameterListInstruction::Empty"),
            Self::Constant(field_0) => {
                output.call("graph::ParameterListInstruction::Constant", &[field_0])
            }
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::ParameterListInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::ParameterListInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::ParameterListInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::ParameterListInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::ParameterListInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
        }
    }
}

impl<Element: 'static, Local, Function, FunctionLocal> Emit
    for TypedListInstruction<Element, Local, Function, FunctionLocal>
where
    Table<Element>: Emit,
    ConstantId<Local>: Emit,
    Local: Emit,
    Function: Emit,
    FunctionLocal: Emit,
{
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Value(field_0) => output.call("graph::TypedListInstruction::Value", &[field_0]),
            Self::Constant(field_0) => {
                output.call("graph::TypedListInstruction::Constant", &[field_0])
            }
            Self::Spread { elements, tail } => output.structure(
                "graph::TypedListInstruction::Spread",
                &[("elements", elements), ("tail", tail)],
            ),
            Self::Call {
                function,
                args,
                site,
            } => output.structure(
                "graph::TypedListInstruction::Call",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::FunctionCall {
                function,
                args,
                site,
            } => output.structure(
                "graph::TypedListInstruction::FunctionCall",
                &[("function", function), ("args", args), ("site", site)],
            ),
            Self::TupleIndex { tuple, index } => output.structure(
                "graph::TypedListInstruction::TupleIndex",
                &[("tuple", tuple), ("index", index)],
            ),
            Self::CustomField { source, index } => output.structure(
                "graph::TypedListInstruction::CustomField",
                &[("source", source), ("index", index)],
            ),
            Self::ListIndex { list, index } => output.structure(
                "graph::TypedListInstruction::ListIndex",
                &[("list", list), ("index", index)],
            ),
            Self::DropFirst { list, count } => output.structure(
                "graph::TypedListInstruction::DropFirst",
                &[("list", list), ("count", count)],
            ),
        }
    }
}

impl Emit for ExternalListInstruction {
    fn emit(&self, output: &mut Rust) {
        let Self {
            type_id,
            instruction,
        } = self;
        output.structure(
            "graph::ExternalListInstruction",
            &[("type_id", type_id), ("instruction", instruction)],
        );
    }
}

impl Emit for ListInstruction {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Parameter(field_0, field_1) => {
                output.call("graph::ListInstruction::Parameter", &[field_0, field_1])
            }
            Self::ParameterList(field_0, field_1) => {
                output.call("graph::ListInstruction::ParameterList", &[field_0, field_1])
            }
            Self::Int(field_0, field_1) => {
                output.call("graph::ListInstruction::Int", &[field_0, field_1])
            }
            Self::String(field_0, field_1) => {
                output.call("graph::ListInstruction::String", &[field_0, field_1])
            }
            Self::BitArray(field_0, field_1) => {
                output.call("graph::ListInstruction::BitArray", &[field_0, field_1])
            }
            Self::UtfCodepoint(field_0, field_1) => {
                output.call("graph::ListInstruction::UtfCodepoint", &[field_0, field_1])
            }
            Self::Custom(field_0, field_1) => {
                output.call("graph::ListInstruction::Custom", &[field_0, field_1])
            }
            Self::Float(field_0, field_1) => {
                output.call("graph::ListInstruction::Float", &[field_0, field_1])
            }
            Self::Bool(field_0, field_1) => {
                output.call("graph::ListInstruction::Bool", &[field_0, field_1])
            }
            Self::Nil(field_0, field_1) => {
                output.call("graph::ListInstruction::Nil", &[field_0, field_1])
            }
            Self::Tuple(field_0, field_1) => {
                output.call("graph::ListInstruction::Tuple", &[field_0, field_1])
            }
            Self::List(field_0, field_1) => {
                output.call("graph::ListInstruction::List", &[field_0, field_1])
            }
            Self::Function(field_0, field_1) => {
                output.call("graph::ListInstruction::Function", &[field_0, field_1])
            }
        }
    }
}

#[cfg(test)]
mod emission_tests {
    use super::{
        ExternalListInstruction, ListInstruction, ParameterListInstruction, TypedListInstruction,
    };
    use crate::plan::execution::graph::{ExternalLocal, ExternalLocalId};
    use crate::plan::execution::prepared::rust::Rust;
    use crate::plan::execution::type_::list::{FunctionItemTypeId, TupleItemTypeId};
    use crate::plan::execution::type_::{
        BitArrayListTypeId, BoolListTypeId, CustomListTypeId, CustomTypeId, ExternalListTypeId,
        ExternalTypeId, FloatListTypeId, FunctionListTypeId, IntListTypeId, ListListTypeId,
        ListTypeId, NilListTypeId, ParameterListListTypeId, ParameterListTypeId, StringListTypeId,
        TupleListTypeId, UtfCodepointListTypeId,
    };

    #[test]
    fn emits_parameter_list_operations_without_concrete_element_storage() {
        use crate::plan::execution::constant::ConstantId;
        use crate::plan::execution::function::ParameterListFunctionId;
        use crate::plan::execution::graph::{
            CustomLocal, CustomLocalId, ListFunctionLocal, ParamLocal,
            ParameterListFunctionLocalId, ParameterListListLocalId, TupleLocalId,
        };
        use crate::plan::execution::type_::{
            CustomValueShape, CustomValueShapeId, FunctionType, ValueType,
        };
        use crate::plan::{HostCallSite, SourceSpan};

        let cases = [
            (
                ParameterListInstruction::Empty,
                "data::graph::ParameterListInstruction::Empty",
            ),
            (
                ParameterListInstruction::Constant(ConstantId::new(2)),
                r#"
data::graph::ParameterListInstruction::Constant(data::constant::ConstantId {
    index: 2,
    value: ::core::marker::PhantomData,
})"#.trim_start_matches('\n'),
            ),
            (
                ParameterListInstruction::Call {
                    function: ParameterListFunctionId::new(
                        2,
                        ParameterListTypeId::new(ListTypeId(3), crate::plan::TypeParameterId(4)),
                    ),
                    args: Vec::<ParamLocal>::new().into(),
                    site: HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8)),
                },
                r#"
data::graph::ParameterListInstruction::Call {
    function: data::function::ParameterListFunctionId {
        index: 2,
        type_id: data::type_::ParameterListTypeId {
            list_type: data::type_::ListTypeId(3),
            item: data::type_::parameter_id(4),
        },
    },
    args: data::Storage::Static(&[]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                ParameterListInstruction::FunctionCall {
                    function: ListFunctionLocal::Parameter {
                        local: ParameterListFunctionLocalId(2),
                        type_: FunctionType::new(Vec::new(), ValueType::List(ListTypeId(3))),
                        list_type: ParameterListTypeId::new(
                            ListTypeId(3),
                            crate::plan::TypeParameterId(4),
                        ),
                    },
                    args: Vec::new().into(),
                    site: HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8)),
                },
                r#"
data::graph::ParameterListInstruction::FunctionCall {
    function: data::graph::ListFunctionLocal::Parameter {
        local: data::graph::ParameterListFunctionLocalId(2),
        type_: data::type_::FunctionType {
            arguments: data::Storage::Static(&[]),
            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3))),
        },
        list_type: data::type_::ParameterListTypeId {
            list_type: data::type_::ListTypeId(3),
            item: data::type_::parameter_id(4),
        },
    },
    args: data::Storage::Static(&[]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                ParameterListInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                r#"
data::graph::ParameterListInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                ParameterListInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                r#"
data::graph::ParameterListInstruction::CustomField {
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
                ParameterListInstruction::ListIndex {
                    list: ParameterListListLocalId(2),
                    index: 1,
                },
                r#"
data::graph::ParameterListInstruction::ListIndex {
    list: data::graph::ParameterListListLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
        ];
        for (instruction, expected) in cases {
            assert_eq!(Rust::expression(&instruction), expected);
        }
    }

    #[test]
    fn emits_typed_list_construction_calls_and_projections() {
        use crate::plan::execution::constant::ConstantId;
        use crate::plan::execution::function::IntListFunctionId;
        use crate::plan::execution::graph::{
            CustomLocal, CustomLocalId, IntListFunctionLocalId, IntListLocalId, IntLocalId,
            ListFunctionLocal, ListListLocalId, ParamLocal, TupleLocalId,
        };
        use crate::plan::execution::type_::{
            CustomValueShape, CustomValueShapeId, FunctionType, ValueType,
        };
        use crate::plan::{HostCallSite, SourceSpan};

        let cases: [(
            TypedListInstruction<IntLocalId, IntListLocalId, IntListFunctionId>,
            &str,
        ); 9] = [
            (
                TypedListInstruction::Value(vec![IntLocalId(2)].into()),
                r#"
data::graph::TypedListInstruction::Value(data::Storage::Static(&[
    data::graph::IntLocalId(2),
]))"#.trim_start_matches('\n'),
            ),
            (
                TypedListInstruction::Constant(ConstantId::new(2)),
                r#"
data::graph::TypedListInstruction::Constant(data::constant::ConstantId {
    index: 2,
    value: ::core::marker::PhantomData,
})"#.trim_start_matches('\n'),
            ),
            (
                TypedListInstruction::Spread {
                    elements: vec![IntLocalId(2)].into(),
                    tail: IntListLocalId(5),
                },
                r#"
data::graph::TypedListInstruction::Spread {
    elements: data::Storage::Static(&[
        data::graph::IntLocalId(2),
    ]),
    tail: data::graph::IntListLocalId(5),
}"#.trim_start_matches('\n'),
            ),
            (
                TypedListInstruction::Call {
                    function: IntListFunctionId::new(2, IntListTypeId::new(ListTypeId(3))),
                    args: vec![ParamLocal::Int(IntLocalId(5))].into(),
                    site: HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8)),
                },
                r#"
data::graph::TypedListInstruction::Call {
    function: data::function::IntListFunctionId {
        index: 2,
        type_id: data::type_::IntListTypeId {
            list_type: data::type_::ListTypeId(3),
        },
    },
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                TypedListInstruction::FunctionCall {
                    function: ListFunctionLocal::Int {
                        local: IntListFunctionLocalId(2),
                        type_: FunctionType::new(
                            vec![ValueType::Int],
                            ValueType::List(ListTypeId(3)),
                        ),
                        list_type: IntListTypeId::new(ListTypeId(3)),
                    },
                    args: vec![ParamLocal::Int(IntLocalId(5))].into(),
                    site: HostCallSite::new("example".into(), "main".into(), SourceSpan::new(3, 8)),
                },
                r#"
data::graph::TypedListInstruction::FunctionCall {
    function: data::graph::ListFunctionLocal::Int {
        local: data::graph::IntListFunctionLocalId(2),
        type_: data::type_::FunctionType {
            arguments: data::Storage::Static(&[
                data::type_::ValueType::Int,
            ]),
            return_: data::Storage::Static(&data::type_::ValueType::List(data::type_::ListTypeId(3))),
        },
        list_type: data::type_::IntListTypeId {
            list_type: data::type_::ListTypeId(3),
        },
    },
    args: data::Storage::Static(&[
        data::graph::ParamLocal::Int(data::graph::IntLocalId(5)),
    ]),
    site: data::source::HostCallSite::from_static("example", "main", data::source::SourceSpan::new(3, 8)),
}"#.trim_start_matches('\n'),
            ),
            (
                TypedListInstruction::TupleIndex {
                    tuple: TupleLocalId(2),
                    index: 1,
                },
                r#"
data::graph::TypedListInstruction::TupleIndex {
    tuple: data::graph::TupleLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                TypedListInstruction::CustomField {
                    source: CustomLocal::new(
                        CustomLocalId(2),
                        CustomValueShape::new(CustomTypeId(3), CustomValueShapeId(4)),
                    ),
                    index: 1,
                },
                r#"
data::graph::TypedListInstruction::CustomField {
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
                TypedListInstruction::ListIndex {
                    list: ListListLocalId(2),
                    index: 1,
                },
                r#"
data::graph::TypedListInstruction::ListIndex {
    list: data::graph::ListListLocalId(2),
    index: 1,
}"#.trim_start_matches('\n'),
            ),
            (
                TypedListInstruction::DropFirst {
                    list: IntListLocalId(2),
                    count: 1,
                },
                r#"
data::graph::TypedListInstruction::DropFirst {
    list: data::graph::IntListLocalId(2),
    count: 1,
}"#.trim_start_matches('\n'),
            ),
        ];
        for (instruction, expected) in cases {
            assert_eq!(Rust::expression(&instruction), expected);
        }
    }

    #[test]
    fn emits_every_list_storage_family_and_its_type_metadata() {
        let cases = [
            (
                ListInstruction::Int(
                    IntListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::Value(Vec::new().into()),
                ),
                r#"
data::graph::ListInstruction::Int(data::type_::IntListTypeId {
    list_type: data::type_::ListTypeId(3),
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#
                    .trim_start_matches('\n'),
            ),
            (
                ListInstruction::String(
                    StringListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::Value(Vec::new().into()),
                ),
                r#"
data::graph::ListInstruction::String(data::type_::StringListTypeId {
    list_type: data::type_::ListTypeId(3),
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#
                    .trim_start_matches('\n'),
            ),
            (
                ListInstruction::BitArray(
                    BitArrayListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::Value(Vec::new().into()),
                ),
                r#"
data::graph::ListInstruction::BitArray(data::type_::BitArrayListTypeId {
    list_type: data::type_::ListTypeId(3),
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#
                    .trim_start_matches('\n'),
            ),
            (
                ListInstruction::UtfCodepoint(
                    UtfCodepointListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::Value(Vec::new().into()),
                ),
                r#"
data::graph::ListInstruction::UtfCodepoint(data::type_::UtfCodepointListTypeId {
    list_type: data::type_::ListTypeId(3),
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#
                    .trim_start_matches('\n'),
            ),
            (
                ListInstruction::Float(
                    FloatListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::Value(Vec::new().into()),
                ),
                r#"
data::graph::ListInstruction::Float(data::type_::FloatListTypeId {
    list_type: data::type_::ListTypeId(3),
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#
                    .trim_start_matches('\n'),
            ),
            (
                ListInstruction::Bool(
                    BoolListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::Value(Vec::new().into()),
                ),
                r#"
data::graph::ListInstruction::Bool(data::type_::BoolListTypeId {
    list_type: data::type_::ListTypeId(3),
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#
                    .trim_start_matches('\n'),
            ),
            (
                ListInstruction::Nil(
                    NilListTypeId::new(ListTypeId(3)),
                    TypedListInstruction::Value(Vec::new().into()),
                ),
                r#"
data::graph::ListInstruction::Nil(data::type_::NilListTypeId {
    list_type: data::type_::ListTypeId(3),
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#
                    .trim_start_matches('\n'),
            ),
            (
                ListInstruction::Parameter(
                    ParameterListTypeId::new(ListTypeId(3), crate::plan::TypeParameterId(4)),
                    ParameterListInstruction::Empty,
                ),
                r#"
data::graph::ListInstruction::Parameter(data::type_::ParameterListTypeId {
    list_type: data::type_::ListTypeId(3),
    item: data::type_::parameter_id(4),
}, data::graph::ParameterListInstruction::Empty)"#
                    .trim_start_matches('\n'),
            ),
            (
                ListInstruction::ParameterList(
                    ParameterListListTypeId::new(
                        ListTypeId(3),
                        ParameterListTypeId::new(ListTypeId(4), crate::plan::TypeParameterId(5)),
                    ),
                    TypedListInstruction::Value(Vec::new().into()),
                ),
                r#"
data::graph::ListInstruction::ParameterList(data::type_::ParameterListListTypeId {
    list_type: data::type_::ListTypeId(3),
    item_type: data::type_::ParameterListTypeId {
        list_type: data::type_::ListTypeId(4),
        item: data::type_::parameter_id(5),
    },
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#
                    .trim_start_matches('\n'),
            ),
            (
                ListInstruction::Custom(
                    CustomListTypeId::new(ListTypeId(3), CustomTypeId(4)),
                    TypedListInstruction::Value(Vec::new().into()),
                ),
                r#"
data::graph::ListInstruction::Custom(data::type_::CustomListTypeId {
    list_type: data::type_::ListTypeId(3),
    item_type: data::type_::CustomTypeId(4),
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#
                    .trim_start_matches('\n'),
            ),
            (
                ListInstruction::Tuple(
                    TupleListTypeId {
                        list_type: ListTypeId(3),
                        item_type: TupleItemTypeId(4),
                    },
                    TypedListInstruction::Value(Vec::new().into()),
                ),
                r#"
data::graph::ListInstruction::Tuple(data::type_::TupleListTypeId {
    list_type: data::type_::ListTypeId(3),
    item_type: data::type_::TupleItemTypeId(4),
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#
                    .trim_start_matches('\n'),
            ),
            (
                ListInstruction::List(
                    ListListTypeId::new(ListTypeId(3), ListTypeId(4)),
                    TypedListInstruction::Value(Vec::new().into()),
                ),
                r#"
data::graph::ListInstruction::List(data::type_::ListListTypeId {
    list_type: data::type_::ListTypeId(3),
    item_type: data::type_::ListTypeId(4),
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#
                    .trim_start_matches('\n'),
            ),
            (
                ListInstruction::Function(
                    FunctionListTypeId {
                        list_type: ListTypeId(3),
                        item_type: FunctionItemTypeId(4),
                    },
                    TypedListInstruction::Value(Vec::new().into()),
                ),
                r#"
data::graph::ListInstruction::Function(data::type_::FunctionListTypeId {
    list_type: data::type_::ListTypeId(3),
    item_type: data::type_::FunctionItemTypeId(4),
}, data::graph::TypedListInstruction::Value(data::Storage::Static(&[])))"#
                    .trim_start_matches('\n'),
            ),
        ];
        for (instruction, expected) in cases {
            assert_eq!(Rust::expression(&instruction), expected);
        }
        let external = ExternalListInstruction::new(
            ExternalListTypeId::new(ListTypeId(3), ExternalTypeId(4)),
            TypedListInstruction::Value(
                vec![ExternalLocal::new(ExternalLocalId(2), ExternalTypeId(4))].into(),
            ),
        );
        assert_eq!(
            Rust::expression(&external),
            r#"
data::graph::ExternalListInstruction {
    type_id: data::type_::ExternalListTypeId {
        list_type: data::type_::ListTypeId(3),
        item_type: data::type_::ExternalTypeId(4),
    },
    instruction: data::graph::TypedListInstruction::Value(data::Storage::Static(&[
        data::graph::ExternalLocal {
            id: data::graph::ExternalLocalId(2),
            type_id: data::type_::ExternalTypeId(4),
        },
    ])),
}"#
            .trim_start_matches('\n')
        );
    }
}

#[cfg(test)]
mod external_list_view_tests {
    use super::{ExternalListInstruction, ExternalListInstructionView, TypedListInstruction};
    use crate::plan::execution::graph::{ExternalLocal, ExternalLocalId};
    use crate::plan::execution::type_::{ExternalListTypeId, ExternalTypeId, ListTypeId};

    #[test]
    fn exposes_external_list_type_and_instruction() {
        let external_type = ExternalTypeId::new(3);
        let list_type = ExternalListTypeId::new(ListTypeId::new(7), external_type);
        let instruction = ExternalListInstruction::new(
            list_type,
            TypedListInstruction::Value(
                vec![ExternalLocal::new(ExternalLocalId(2), external_type)].into(),
            ),
        );

        assert_eq!(instruction.type_id(), list_type);
        assert_eq!(
            instruction.instruction(),
            &TypedListInstruction::Value(
                vec![ExternalLocal::new(ExternalLocalId(2), external_type)].into(),
            ),
        );
    }

    #[test]
    fn plain_external_list_instruction_view_is_uninhabited() {
        fn assert_view<View>()
        where
            View: ExternalListInstructionView<
                    Function = std::convert::Infallible,
                    FunctionLocal = std::convert::Infallible,
                >,
        {
        }

        assert_view::<std::convert::Infallible>();
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{ExternalListInstruction, TypedListInstruction};
    use crate::plan::execution::explain;
    use crate::plan::execution::function::TupleFunctionId;
    use crate::plan::execution::graph::{ExternalLocal, ExternalLocalId};
    use crate::plan::execution::type_::{ExternalListTypeId, ExternalTypeId, ListTypeId};

    #[test]
    fn writes_list_instruction_grammar() {
        let source = r#"
pub fn main() {
  let tail = [3]
  let values = [1, 2, ..tail]
  let assert [_, ..rest] = values
  #([], values, rest)
}
"#;
        let expected = concat!(
            "    %int#0:shape#0(Int) = int.value 3\n",
            "    %list.int#0:shape#1(list_type#1) = list.int[type#1] value ",
            "elements=[%int#0]\n",
            "    %int#1:shape#0(Int) = int.value 1\n",
            "    %int#2:shape#0(Int) = int.value 2\n",
            "    %list.int#1:shape#1(list_type#1) = list.int[type#1] spread ",
            "elements=[%int#1, %int#2] tail=%list.int#0\n",
            "    %list.parameter#0:shape#3(list_type#0) = list.parameter[type#0] empty\n",
            "    %tuple#0:shape#4(#(list_type#0, list_type#1, list_type#1)) = ",
            "tuple.value elements=[%list.parameter#0, %list.int#1, %list.int#0]\n",
        );

        assert_explanation(source, expected);
    }

    #[test]
    fn writes_external_list_instruction_grammar() {
        let source = "pub fn main() { 1 }";
        let expected = "list.external[type#7] value elements=[%external#2]";

        explain::assert_rendered(source, expected, |plan, output| {
            let external_type = ExternalTypeId::new(3);
            let instruction = ExternalListInstruction::new(
                ExternalListTypeId::new(ListTypeId::new(7), external_type),
                TypedListInstruction::Value(
                    vec![ExternalLocal::new(ExternalLocalId(2), external_type)].into(),
                ),
            );
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(&instruction);
        });
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
