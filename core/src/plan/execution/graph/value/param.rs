use super::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId,
    CustomFunctionLocal, CustomLocal, ExternalFunctionLocal, ExternalLocal, FloatFunctionLocalId,
    FloatLocalId, FunctionFunctionLocal, GenericFunctionLocal, IntFunctionLocalId, IntLocalId,
    ListFunctionLocal, ListLocal, NeverFunctionLocal, NilFunctionLocalId, NilLocalId,
    StringFunctionLocalId, StringLocalId, TupleFunctionLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointLocalId,
};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::LocalLabel;
use crate::plan::execution::prepared::rust::{Emit, Rust};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::{FunctionType, ValueShapeId, ValueType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamSlot {
    pub local: ParamLocal,
    pub shape: ValueShapeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParamLocal {
    Int(IntLocalId),
    Float(FloatLocalId),
    String(StringLocalId),
    BitArray(BitArrayLocalId),
    UtfCodepoint(UtfCodepointLocalId),
    Custom(CustomLocal),
    External(ExternalLocal),
    Bool(BoolLocalId),
    Nil(NilLocalId),
    Tuple {
        local: TupleLocalId,
        type_: Table<ValueType>,
    },
    List(ListLocal),
    IntFunction {
        local: IntFunctionLocalId,
        type_: FunctionType,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        type_: FunctionType,
    },
    StringFunction {
        local: StringFunctionLocalId,
        type_: FunctionType,
    },
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        type_: FunctionType,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        type_: FunctionType,
    },
    GenericFunction(GenericFunctionLocal),
    NeverFunction(NeverFunctionLocal),
    CustomFunction(CustomFunctionLocal),
    ExternalFunction(ExternalFunctionLocal),
    BoolFunction {
        local: BoolFunctionLocalId,
        type_: FunctionType,
    },
    NilFunction {
        local: NilFunctionLocalId,
        type_: FunctionType,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        type_: FunctionType,
    },
    ListFunction(ListFunctionLocal),
    FunctionFunction(FunctionFunctionLocal),
}

impl ParamSlot {
    pub(in crate::plan::execution) fn new(local: ParamLocal, shape: ValueShapeId) -> Self {
        Self { local, shape }
    }

    pub(crate) fn local(&self) -> &ParamLocal {
        &self.local
    }

    pub(crate) fn shape(&self) -> ValueShapeId {
        self.shape
    }
}

impl Explain for ParamSlot {
    fn write_explanation(&self, context: &mut ExplainContext<'_, '_>) {
        self.local().write_local_label(context.output());
        context.push_str(":shape#");
        context.push_str(&self.shape().index().to_string());
        context.push('(');
        let type_ = context.shape_value_type(self.shape());
        context.write(&type_);
        context.push(')');
    }
}

impl Emit for ParamSlot {
    fn emit(&self, output: &mut Rust) {
        let Self { local, shape } = self;
        output.structure("graph::ParamSlot", &[("local", local), ("shape", shape)]);
    }
}

impl Emit for ParamLocal {
    fn emit(&self, output: &mut Rust) {
        match self {
            Self::Int(field_0) => output.call("graph::ParamLocal::Int", &[field_0]),
            Self::Float(field_0) => output.call("graph::ParamLocal::Float", &[field_0]),
            Self::String(field_0) => output.call("graph::ParamLocal::String", &[field_0]),
            Self::BitArray(field_0) => output.call("graph::ParamLocal::BitArray", &[field_0]),
            Self::UtfCodepoint(field_0) => {
                output.call("graph::ParamLocal::UtfCodepoint", &[field_0])
            }
            Self::Custom(field_0) => output.call("graph::ParamLocal::Custom", &[field_0]),
            Self::External(field_0) => output.call("graph::ParamLocal::External", &[field_0]),
            Self::Bool(field_0) => output.call("graph::ParamLocal::Bool", &[field_0]),
            Self::Nil(field_0) => output.call("graph::ParamLocal::Nil", &[field_0]),
            Self::Tuple { local, type_ } => output.structure(
                "graph::ParamLocal::Tuple",
                &[("local", local), ("type_", type_)],
            ),
            Self::List(field_0) => output.call("graph::ParamLocal::List", &[field_0]),
            Self::IntFunction { local, type_ } => output.structure(
                "graph::ParamLocal::IntFunction",
                &[("local", local), ("type_", type_)],
            ),
            Self::FloatFunction { local, type_ } => output.structure(
                "graph::ParamLocal::FloatFunction",
                &[("local", local), ("type_", type_)],
            ),
            Self::StringFunction { local, type_ } => output.structure(
                "graph::ParamLocal::StringFunction",
                &[("local", local), ("type_", type_)],
            ),
            Self::BitArrayFunction { local, type_ } => output.structure(
                "graph::ParamLocal::BitArrayFunction",
                &[("local", local), ("type_", type_)],
            ),
            Self::UtfCodepointFunction { local, type_ } => output.structure(
                "graph::ParamLocal::UtfCodepointFunction",
                &[("local", local), ("type_", type_)],
            ),
            Self::GenericFunction(field_0) => {
                output.call("graph::ParamLocal::GenericFunction", &[field_0])
            }
            Self::NeverFunction(field_0) => {
                output.call("graph::ParamLocal::NeverFunction", &[field_0])
            }
            Self::CustomFunction(field_0) => {
                output.call("graph::ParamLocal::CustomFunction", &[field_0])
            }
            Self::ExternalFunction(field_0) => {
                output.call("graph::ParamLocal::ExternalFunction", &[field_0])
            }
            Self::BoolFunction { local, type_ } => output.structure(
                "graph::ParamLocal::BoolFunction",
                &[("local", local), ("type_", type_)],
            ),
            Self::NilFunction { local, type_ } => output.structure(
                "graph::ParamLocal::NilFunction",
                &[("local", local), ("type_", type_)],
            ),
            Self::TupleFunction { local, type_ } => output.structure(
                "graph::ParamLocal::TupleFunction",
                &[("local", local), ("type_", type_)],
            ),
            Self::ListFunction(field_0) => {
                output.call("graph::ParamLocal::ListFunction", &[field_0])
            }
            Self::FunctionFunction(field_0) => {
                output.call("graph::ParamLocal::FunctionFunction", &[field_0])
            }
        }
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::explain;
    use crate::plan::execution::function::IntFunctionId;

    #[test]
    fn writes_slot_from_a_lowered_instruction() {
        let source = "pub fn main() { 1 }";
        let expected = "%int#0:shape#0(Int)";

        assert_explanation(source, expected);
    }

    fn assert_explanation(source: &str, expected: &str) {
        explain::assert_rendered(source, expected, |plan, output| {
            let instruction = &plan
                .int_function(IntFunctionId(0))
                .body()
                .block_graph()
                .blocks()
                .next()
                .unwrap()
                .instructions()[0];
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(instruction.output());
        });
    }
}
