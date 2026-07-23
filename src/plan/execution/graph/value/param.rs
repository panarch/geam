use super::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId,
    CustomFunctionLocal, CustomLocal, FloatFunctionLocalId, FloatLocalId, FunctionFunctionLocal,
    GenericFunctionLocal, IntFunctionLocalId, IntLocalId, ListFunctionLocal, ListLocal,
    NeverFunctionLocal, NilFunctionLocalId, NilLocalId, StringFunctionLocalId, StringLocalId,
    TupleFunctionLocalId, TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointLocalId,
};
use crate::plan::execution::explain::{Explain, ExplainContext};
use crate::plan::execution::graph::ExplainLocal;
use crate::plan::execution::{FunctionType, ValueShapeId, ValueType};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct ParamSlot {
    local: ParamLocal,
    shape: ValueShapeId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum ParamLocal {
    Int(IntLocalId),
    Float(FloatLocalId),
    String(StringLocalId),
    BitArray(BitArrayLocalId),
    UtfCodepoint(UtfCodepointLocalId),
    Custom(CustomLocal),
    Bool(BoolLocalId),
    Nil(NilLocalId),
    Tuple {
        local: TupleLocalId,
        type_: Vec<ValueType>,
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
        self.local().write_local(context.output());
        context.push_str(":shape#");
        context.push_str(&self.shape().index().to_string());
        context.push('(');
        let type_ = context.plan().shape_value_type(self.shape());
        context.write(&type_);
        context.push(')');
    }
}

#[cfg(test)]
mod explain_tests {
    use crate::plan::execution::{IntFunctionId, explain};

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
                .blocks()[0]
                .instructions()[0];
            let mut context = explain::ExplainContext::new(plan, output);
            context.write(instruction.output());
        });
    }
}
