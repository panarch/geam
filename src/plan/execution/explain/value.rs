use super::super::graph::FunctionLocal;
use super::super::{
    BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
    BoolListLocalId, BoolLocalId, CustomFunctionLocal, CustomListLocalId, CustomLocal,
    ExecutionPlan, FloatFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionFunctionLocal,
    FunctionListLocalId, GenericFunctionLocal, IntFunctionLocalId, IntListLocalId, IntLocalId,
    ListFunctionLocal, ListListLocalId, ListLocal, NeverFunctionLocal, NilFunctionLocalId,
    NilListLocalId, NilLocalId, ParamLocal, ParamSlot, ParameterListListLocalId,
    ParameterListLocalId, StoredListLocal, StringFunctionLocalId, StringListLocalId, StringLocalId,
    TupleFunctionLocalId, TupleListLocalId, TupleLocalId, UtfCodepointFunctionLocalId,
    UtfCodepointListLocalId, UtfCodepointLocalId, ValueType,
};
use std::convert::Infallible;

pub(super) trait ExplainLocal {
    fn write_local(&self, output: &mut String);
}

macro_rules! indexed_local {
    ($type_:ty, $family:literal) => {
        impl ExplainLocal for $type_ {
            fn write_local(&self, output: &mut String) {
                write_indexed(output, $family, self.0);
            }
        }
    };
}

indexed_local!(IntLocalId, "int");
indexed_local!(FloatLocalId, "float");
indexed_local!(StringLocalId, "string");
indexed_local!(BitArrayLocalId, "bit_array");
indexed_local!(UtfCodepointLocalId, "utf_codepoint");
indexed_local!(BoolLocalId, "bool");
indexed_local!(NilLocalId, "nil");
indexed_local!(TupleLocalId, "tuple");

indexed_local!(ParameterListLocalId, "list.parameter");
indexed_local!(ParameterListListLocalId, "list.parameter_list");
indexed_local!(IntListLocalId, "list.int");
indexed_local!(StringListLocalId, "list.string");
indexed_local!(BitArrayListLocalId, "list.bit_array");
indexed_local!(UtfCodepointListLocalId, "list.utf_codepoint");
indexed_local!(CustomListLocalId, "list.custom");
indexed_local!(FloatListLocalId, "list.float");
indexed_local!(BoolListLocalId, "list.bool");
indexed_local!(NilListLocalId, "list.nil");
indexed_local!(TupleListLocalId, "list.tuple");
indexed_local!(ListListLocalId, "list.list");
indexed_local!(FunctionListLocalId, "list.function");

indexed_local!(IntFunctionLocalId, "function.int");
indexed_local!(FloatFunctionLocalId, "function.float");
indexed_local!(StringFunctionLocalId, "function.string");
indexed_local!(BitArrayFunctionLocalId, "function.bit_array");
indexed_local!(UtfCodepointFunctionLocalId, "function.utf_codepoint");
indexed_local!(BoolFunctionLocalId, "function.bool");
indexed_local!(NilFunctionLocalId, "function.nil");
indexed_local!(TupleFunctionLocalId, "function.tuple");

impl ExplainLocal for CustomLocal {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "custom", self.id().0);
    }
}

impl ExplainLocal for GenericFunctionLocal {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.generic", self.id().0);
    }
}

impl ExplainLocal for NeverFunctionLocal {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.never", self.id().0);
    }
}

impl ExplainLocal for CustomFunctionLocal {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.custom", self.id().0);
    }
}

impl ExplainLocal for FunctionFunctionLocal {
    fn write_local(&self, output: &mut String) {
        write_indexed(output, "function.function", self.id().0);
    }
}

impl ExplainLocal for ListLocal {
    fn write_local(&self, output: &mut String) {
        match self {
            Self::Parameter { local, .. } => local.write_local(output),
            Self::ParameterList { local, .. } => local.write_local(output),
            Self::Int { local, .. } => local.write_local(output),
            Self::String { local, .. } => local.write_local(output),
            Self::BitArray { local, .. } => local.write_local(output),
            Self::UtfCodepoint { local, .. } => local.write_local(output),
            Self::Custom { local, .. } => local.write_local(output),
            Self::Float { local, .. } => local.write_local(output),
            Self::Bool { local, .. } => local.write_local(output),
            Self::Nil { local, .. } => local.write_local(output),
            Self::Tuple { local, .. } => local.write_local(output),
            Self::List { local, .. } => local.write_local(output),
            Self::Function { local, .. } => local.write_local(output),
        }
    }
}

impl ExplainLocal for StoredListLocal {
    fn write_local(&self, output: &mut String) {
        match self {
            Self::ParameterList(local) => local.write_local(output),
            Self::Int(local) => local.write_local(output),
            Self::String(local) => local.write_local(output),
            Self::BitArray(local) => local.write_local(output),
            Self::UtfCodepoint(local) => local.write_local(output),
            Self::Custom(local) => local.write_local(output),
            Self::Float(local) => local.write_local(output),
            Self::Bool(local) => local.write_local(output),
            Self::Nil(local) => local.write_local(output),
            Self::Tuple(local) => local.write_local(output),
            Self::List(local) => local.write_local(output),
            Self::Function(local) => local.write_local(output),
        }
    }
}

impl ExplainLocal for ListFunctionLocal {
    fn write_local(&self, output: &mut String) {
        match self {
            Self::Parameter { local, .. } => {
                write_indexed(output, "function.list.parameter", local.0)
            }
            Self::ParameterList { local, .. } => {
                write_indexed(output, "function.list.parameter_list", local.0)
            }
            Self::Int { local, .. } => write_indexed(output, "function.list.int", local.0),
            Self::String { local, .. } => write_indexed(output, "function.list.string", local.0),
            Self::BitArray { local, .. } => {
                write_indexed(output, "function.list.bit_array", local.0)
            }
            Self::UtfCodepoint { local, .. } => {
                write_indexed(output, "function.list.utf_codepoint", local.0)
            }
            Self::Custom { local, .. } => write_indexed(output, "function.list.custom", local.0),
            Self::Float { local, .. } => write_indexed(output, "function.list.float", local.0),
            Self::Bool { local, .. } => write_indexed(output, "function.list.bool", local.0),
            Self::Nil { local, .. } => write_indexed(output, "function.list.nil", local.0),
            Self::Tuple { local, .. } => write_indexed(output, "function.list.tuple", local.0),
            Self::List { local, .. } => write_indexed(output, "function.list.list", local.0),
            Self::Function { local, .. } => {
                write_indexed(output, "function.list.function", local.0)
            }
        }
    }
}

impl ExplainLocal for FunctionLocal {
    fn write_local(&self, output: &mut String) {
        match self {
            Self::Generic(local) => local.write_local(output),
            Self::Never(local) => local.write_local(output),
            Self::Int(local) => local.write_local(output),
            Self::Float(local) => local.write_local(output),
            Self::String(local) => local.write_local(output),
            Self::BitArray(local) => local.write_local(output),
            Self::UtfCodepoint(local) => local.write_local(output),
            Self::Custom(local) => local.write_local(output),
            Self::Bool(local) => local.write_local(output),
            Self::Nil(local) => local.write_local(output),
            Self::Tuple(local) => local.write_local(output),
            Self::List(local) => local.write_local(output),
            Self::Function(local) => local.write_local(output),
        }
    }
}

impl ExplainLocal for Infallible {
    fn write_local(&self, _output: &mut String) {
        match *self {}
    }
}

impl ExplainLocal for ParamLocal {
    fn write_local(&self, output: &mut String) {
        match self {
            Self::Int(local) => local.write_local(output),
            Self::Float(local) => local.write_local(output),
            Self::String(local) => local.write_local(output),
            Self::BitArray(local) => local.write_local(output),
            Self::UtfCodepoint(local) => local.write_local(output),
            Self::Custom(local) => local.write_local(output),
            Self::Bool(local) => local.write_local(output),
            Self::Nil(local) => local.write_local(output),
            Self::Tuple { local, .. } => local.write_local(output),
            Self::List(local) => local.write_local(output),
            Self::IntFunction { local, .. } => local.write_local(output),
            Self::FloatFunction { local, .. } => local.write_local(output),
            Self::StringFunction { local, .. } => local.write_local(output),
            Self::BitArrayFunction { local, .. } => local.write_local(output),
            Self::UtfCodepointFunction { local, .. } => local.write_local(output),
            Self::GenericFunction(local) => local.write_local(output),
            Self::NeverFunction(local) => local.write_local(output),
            Self::CustomFunction(local) => local.write_local(output),
            Self::BoolFunction { local, .. } => local.write_local(output),
            Self::NilFunction { local, .. } => local.write_local(output),
            Self::TupleFunction { local, .. } => local.write_local(output),
            Self::ListFunction(local) => local.write_local(output),
            Self::FunctionFunction(local) => local.write_local(output),
        }
    }
}

pub(super) fn write_slot(output: &mut String, plan: &ExecutionPlan, slot: &ParamSlot) {
    slot.local().write_local(output);
    output.push(':');
    output.push_str("shape#");
    output.push_str(&slot.shape().index().to_string());
    output.push('(');
    write_type(output, &plan.shape_value_type(slot.shape()));
    output.push(')');
}

pub(super) fn write_slots(output: &mut String, plan: &ExecutionPlan, slots: &[ParamSlot]) {
    write_list(output, slots, |output, slot| write_slot(output, plan, slot));
}

pub(super) fn write_locals(output: &mut String, locals: &[ParamLocal]) {
    write_list(output, locals, |output, local| local.write_local(output));
}

pub(super) fn write_list<Value>(
    output: &mut String,
    values: &[Value],
    mut write_value: impl FnMut(&mut String, &Value),
) {
    output.push('[');
    for (index, value) in values.iter().enumerate() {
        if index > 0 {
            output.push_str(", ");
        }
        write_value(output, value);
    }
    output.push(']');
}

pub(super) fn write_type(output: &mut String, type_: &ValueType) {
    match type_ {
        ValueType::Parameter(parameter) => {
            output.push_str("param#");
            output.push_str(&parameter.index().to_string());
        }
        ValueType::Int => output.push_str("Int"),
        ValueType::Float => output.push_str("Float"),
        ValueType::String => output.push_str("String"),
        ValueType::BitArray => output.push_str("BitArray"),
        ValueType::UtfCodepoint => output.push_str("UtfCodepoint"),
        ValueType::Bool => output.push_str("Bool"),
        ValueType::Nil => output.push_str("Nil"),
        ValueType::Tuple(elements) => {
            output.push_str("#(");
            for (index, element) in elements.iter().enumerate() {
                if index > 0 {
                    output.push_str(", ");
                }
                write_type(output, element);
            }
            output.push(')');
        }
        ValueType::List(id) => {
            output.push_str("list_type#");
            output.push_str(&id.index().to_string());
        }
        ValueType::Function(type_) => {
            output.push_str("fn(");
            for (index, argument) in type_.argument_types().iter().enumerate() {
                if index > 0 {
                    output.push_str(", ");
                }
                write_type(output, argument);
            }
            output.push_str(") -> ");
            write_type(output, type_.return_());
        }
        ValueType::Custom(id) => {
            output.push_str("custom_type#");
            output.push_str(&id.index().to_string());
        }
    }
}

fn write_indexed(output: &mut String, family: &str, index: usize) {
    output.push('%');
    output.push_str(family);
    output.push('#');
    output.push_str(&index.to_string());
}
