use super::BlockValues;
use crate::plan::execution::function::{ListFunctionId, RuntimeListFunctionId};
use crate::plan::execution::graph::StorageFamily;
use crate::runtime::evaluated::{
    EvaluatedFunctionFunction, EvaluatedFunctionValue, EvaluatedFunctionValueKind,
    EvaluatedListFunction, EvaluatedValue,
};
use crate::runtime::state::list::ListValueId;

pub(super) struct ColumnWrite {
    pub(super) family: StorageFamily,
    pub(super) growth: usize,
}

impl BlockValues {
    pub(super) fn write_evaluated(&mut self, value: EvaluatedValue) -> Option<ColumnWrite> {
        Some(match value {
            EvaluatedValue::Int(value) => push_column(&mut self.ints, value, StorageFamily::Int),
            EvaluatedValue::Float(value) => {
                push_column(&mut self.floats, value, StorageFamily::Float)
            }
            EvaluatedValue::String(value) => {
                push_column(&mut self.strings, value, StorageFamily::String)
            }
            EvaluatedValue::BitArray(value) => {
                push_column(&mut self.bit_arrays, value, StorageFamily::BitArray)
            }
            EvaluatedValue::UtfCodepoint(value) => {
                push_column(&mut self.utf_codepoints, value, StorageFamily::UtfCodepoint)
            }
            EvaluatedValue::Custom(value) => {
                push_column(&mut self.customs, value, StorageFamily::Custom)
            }
            EvaluatedValue::External(value) => {
                push_column(&mut self.externals, value, StorageFamily::External)
            }
            EvaluatedValue::Bool(value) => push_column(&mut self.bools, value, StorageFamily::Bool),
            EvaluatedValue::Tuple(value) => {
                push_column(&mut self.tuples, value, StorageFamily::Tuple)
            }
            EvaluatedValue::ParameterList(value) => push_column(
                &mut self.parameter_lists,
                value,
                StorageFamily::ParameterList,
            ),
            EvaluatedValue::Nil => return None,
            EvaluatedValue::List(value) => self.write_list(value.into_value()),
            EvaluatedValue::Function(value) => self.write_function(value),
        })
    }

    pub(super) fn write_list(&mut self, value: ListValueId) -> ColumnWrite {
        match value {
            ListValueId::Parameter(value) => push_column(
                &mut self.parameter_lists,
                value,
                StorageFamily::ParameterList,
            ),
            ListValueId::Int(value) => {
                push_column(&mut self.int_lists, value, StorageFamily::IntList)
            }
            ListValueId::String(value) => {
                push_column(&mut self.string_lists, value, StorageFamily::StringList)
            }
            ListValueId::BitArray(value) => push_column(
                &mut self.bit_array_lists,
                value,
                StorageFamily::BitArrayList,
            ),
            ListValueId::UtfCodepoint(value) => push_column(
                &mut self.utf_codepoint_lists,
                value,
                StorageFamily::UtfCodepointList,
            ),
            ListValueId::Custom(value) => {
                push_column(&mut self.custom_lists, value, StorageFamily::CustomList)
            }
            ListValueId::External(value) => {
                push_column(&mut self.external_lists, value, StorageFamily::ExternalList)
            }
            ListValueId::Float(value) => {
                push_column(&mut self.float_lists, value, StorageFamily::FloatList)
            }
            ListValueId::Bool(value) => {
                push_column(&mut self.bool_lists, value, StorageFamily::BoolList)
            }
            ListValueId::Nil(value) => {
                push_column(&mut self.nil_lists, value, StorageFamily::NilList)
            }
            ListValueId::Tuple(value) => {
                push_column(&mut self.tuple_lists, value, StorageFamily::TupleList)
            }
            ListValueId::ParameterList(value) => push_column(
                &mut self.parameter_list_lists,
                value,
                StorageFamily::ParameterListList,
            ),
            ListValueId::List(value) => {
                push_column(&mut self.list_lists, value, StorageFamily::ListList)
            }
            ListValueId::Function(value) => {
                push_column(&mut self.function_lists, value, StorageFamily::FunctionList)
            }
        }
    }

    pub(super) fn write_function(&mut self, value: EvaluatedFunctionValue) -> ColumnWrite {
        match value.into_kind() {
            EvaluatedFunctionValueKind::Generic(value) => push_column(
                &mut self.generic_functions,
                value,
                StorageFamily::GenericFunction,
            ),
            EvaluatedFunctionValueKind::Never(value) => push_column(
                &mut self.never_functions,
                value,
                StorageFamily::NeverFunction,
            ),
            EvaluatedFunctionValueKind::Int(value) => {
                push_column(&mut self.int_functions, value, StorageFamily::IntFunction)
            }
            EvaluatedFunctionValueKind::Float(value) => push_column(
                &mut self.float_functions,
                value,
                StorageFamily::FloatFunction,
            ),
            EvaluatedFunctionValueKind::String(value) => push_column(
                &mut self.string_functions,
                value,
                StorageFamily::StringFunction,
            ),
            EvaluatedFunctionValueKind::BitArray(value) => push_column(
                &mut self.bit_array_functions,
                value,
                StorageFamily::BitArrayFunction,
            ),
            EvaluatedFunctionValueKind::UtfCodepoint(value) => push_column(
                &mut self.utf_codepoint_functions,
                value,
                StorageFamily::UtfCodepointFunction,
            ),
            EvaluatedFunctionValueKind::Custom(value) => push_column(
                &mut self.custom_functions,
                value,
                StorageFamily::CustomFunction,
            ),
            EvaluatedFunctionValueKind::External(value) => push_column(
                &mut self.external_functions,
                value,
                StorageFamily::ExternalFunction,
            ),
            EvaluatedFunctionValueKind::Bool(value) => {
                push_column(&mut self.bool_functions, value, StorageFamily::BoolFunction)
            }
            EvaluatedFunctionValueKind::Nil(value) => {
                push_column(&mut self.nil_functions, value, StorageFamily::NilFunction)
            }
            EvaluatedFunctionValueKind::Tuple(value) => push_column(
                &mut self.tuple_functions,
                value,
                StorageFamily::TupleFunction,
            ),
            EvaluatedFunctionValueKind::List(value) => self.write_list_function(value),
            EvaluatedFunctionValueKind::Function(EvaluatedFunctionFunction::Core(value)) => {
                push_column(
                    &mut self.core_function_functions,
                    value,
                    StorageFamily::CoreFunctionFunction,
                )
            }
            EvaluatedFunctionValueKind::Function(EvaluatedFunctionFunction::External(value)) => {
                push_column(
                    &mut self.external_function_functions,
                    value,
                    StorageFamily::ExternalFunctionFunction,
                )
            }
        }
    }

    pub(super) fn write_list_function(&mut self, value: EvaluatedListFunction) -> ColumnWrite {
        match value.runtime_id() {
            RuntimeListFunctionId::Core(function) => match function {
                ListFunctionId::Parameter(_) => push_column(
                    &mut self.parameter_list_functions,
                    value,
                    StorageFamily::ParameterListFunction,
                ),
                ListFunctionId::ParameterList(_) => push_column(
                    &mut self.parameter_list_list_functions,
                    value,
                    StorageFamily::ParameterListListFunction,
                ),
                ListFunctionId::Int(_) => push_column(
                    &mut self.int_list_functions,
                    value,
                    StorageFamily::IntListFunction,
                ),
                ListFunctionId::String(_) => push_column(
                    &mut self.string_list_functions,
                    value,
                    StorageFamily::StringListFunction,
                ),
                ListFunctionId::BitArray(_) => push_column(
                    &mut self.bit_array_list_functions,
                    value,
                    StorageFamily::BitArrayListFunction,
                ),
                ListFunctionId::UtfCodepoint(_) => push_column(
                    &mut self.utf_codepoint_list_functions,
                    value,
                    StorageFamily::UtfCodepointListFunction,
                ),
                ListFunctionId::Custom(_) => push_column(
                    &mut self.custom_list_functions,
                    value,
                    StorageFamily::CustomListFunction,
                ),
                ListFunctionId::Float(_) => push_column(
                    &mut self.float_list_functions,
                    value,
                    StorageFamily::FloatListFunction,
                ),
                ListFunctionId::Bool(_) => push_column(
                    &mut self.bool_list_functions,
                    value,
                    StorageFamily::BoolListFunction,
                ),
                ListFunctionId::Nil(_) => push_column(
                    &mut self.nil_list_functions,
                    value,
                    StorageFamily::NilListFunction,
                ),
                ListFunctionId::Tuple(_) => push_column(
                    &mut self.tuple_list_functions,
                    value,
                    StorageFamily::TupleListFunction,
                ),
                ListFunctionId::List(_) => push_column(
                    &mut self.list_list_functions,
                    value,
                    StorageFamily::ListListFunction,
                ),
                ListFunctionId::Function(_) => push_column(
                    &mut self.function_list_functions,
                    value,
                    StorageFamily::FunctionListFunction,
                ),
            },
            RuntimeListFunctionId::External(function) => push_column(
                &mut self.external_list_functions,
                value.map_runtime_id(|_| function),
                StorageFamily::ExternalListFunction,
            ),
        }
    }
}

fn push_column<Value>(column: &mut Vec<Value>, value: Value, family: StorageFamily) -> ColumnWrite {
    let capacity = column.capacity();
    column.push(value);
    ColumnWrite {
        family,
        growth: (column.capacity() - capacity) * size_of::<Value>(),
    }
}
