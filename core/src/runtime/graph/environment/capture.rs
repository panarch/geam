use super::{BlockEnvironment, BlockValues, RetainedValues};
use crate::plan::execution::graph::{
    FunctionFunctionLocal, ListFunctionLocal, ListLocal, ParamLocal, ParamSlot,
};
use crate::runtime::evaluated::{
    EvaluatedCapture, EvaluatedFunctionFunction, EvaluatedListCapture, EvaluatedListFunction,
};

impl RetainedValues {
    // Inputs were pushed in declaration order. Taking each column from the end
    // moves owned payloads in O(captures), including multiple values in one family.
    pub(in crate::runtime) fn into_captures(
        mut self,
        slots: &[ParamSlot],
    ) -> Vec<EvaluatedCapture> {
        let mut captures = slots
            .iter()
            .rev()
            .map(|slot| self.values.take_capture(slot.local()))
            .collect::<Vec<_>>();
        captures.reverse();
        captures
    }
}

impl BlockEnvironment {
    // The sealed capture slots form each column's suffix. The caller consumes
    // them in reverse declaration order after reading the ordinary arguments.
    pub(in crate::runtime) fn take_capture_value(
        &mut self,
        local: &ParamLocal,
    ) -> crate::runtime::EvaluatedValue {
        self.values.take_capture(local).into_value()
    }
}

impl BlockValues {
    fn take_capture(&mut self, local: &ParamLocal) -> EvaluatedCapture {
        match local {
            ParamLocal::Int(local) => EvaluatedCapture::int(*local, take_last(&mut self.ints)),
            ParamLocal::Float(local) => {
                EvaluatedCapture::float(*local, take_last(&mut self.floats))
            }
            ParamLocal::String(local) => {
                EvaluatedCapture::string(*local, take_last(&mut self.strings))
            }
            ParamLocal::BitArray(local) => {
                EvaluatedCapture::bit_array(*local, take_last(&mut self.bit_arrays))
            }
            ParamLocal::UtfCodepoint(local) => {
                EvaluatedCapture::utf_codepoint(*local, take_last(&mut self.utf_codepoints))
            }
            ParamLocal::Custom(local) => {
                EvaluatedCapture::custom(*local, take_last(&mut self.customs))
            }
            ParamLocal::External(local) => {
                EvaluatedCapture::external(*local, take_last(&mut self.externals))
            }
            ParamLocal::Bool(local) => EvaluatedCapture::bool(*local, take_last(&mut self.bools)),
            ParamLocal::Nil(local) => EvaluatedCapture::nil(*local),
            ParamLocal::Tuple { local, .. } => {
                EvaluatedCapture::tuple(*local, take_last(&mut self.tuples))
            }
            ParamLocal::List(local) => EvaluatedCapture::list(self.take_list_capture(local)),
            ParamLocal::IntFunction { local, .. } => {
                EvaluatedCapture::int_function(*local, take_last(&mut self.int_functions))
            }
            ParamLocal::FloatFunction { local, .. } => {
                EvaluatedCapture::float_function(*local, take_last(&mut self.float_functions))
            }
            ParamLocal::StringFunction { local, .. } => {
                EvaluatedCapture::string_function(*local, take_last(&mut self.string_functions))
            }
            ParamLocal::BitArrayFunction { local, .. } => EvaluatedCapture::bit_array_function(
                *local,
                take_last(&mut self.bit_array_functions),
            ),
            ParamLocal::UtfCodepointFunction { local, .. } => {
                EvaluatedCapture::utf_codepoint_function(
                    *local,
                    take_last(&mut self.utf_codepoint_functions),
                )
            }
            ParamLocal::BoolFunction { local, .. } => {
                EvaluatedCapture::bool_function(*local, take_last(&mut self.bool_functions))
            }
            ParamLocal::NilFunction { local, .. } => {
                EvaluatedCapture::nil_function(*local, take_last(&mut self.nil_functions))
            }
            ParamLocal::TupleFunction { local, .. } => {
                EvaluatedCapture::tuple_function(*local, take_last(&mut self.tuple_functions))
            }
            ParamLocal::CustomFunction(local) => EvaluatedCapture::custom_function(
                local.clone(),
                take_last(&mut self.custom_functions),
            ),
            ParamLocal::ExternalFunction(local) => EvaluatedCapture::external_function(
                local.clone(),
                take_last(&mut self.external_functions),
            ),
            ParamLocal::GenericFunction(local) => EvaluatedCapture::generic_function(
                local.clone(),
                take_last(&mut self.generic_functions),
            ),
            ParamLocal::NeverFunction(local) => EvaluatedCapture::never_function(
                local.clone(),
                take_last(&mut self.never_functions),
            ),
            ParamLocal::ListFunction(local) => {
                EvaluatedCapture::list_function(local.clone(), self.take_list_function(local))
            }
            ParamLocal::FunctionFunction(local) => {
                let value = match local {
                    FunctionFunctionLocal::Core(_) => EvaluatedFunctionFunction::Core(take_last(
                        &mut self.core_function_functions,
                    )),
                    FunctionFunctionLocal::External(_) => EvaluatedFunctionFunction::External(
                        take_last(&mut self.external_function_functions),
                    ),
                };
                EvaluatedCapture::function_function(local.clone(), value)
            }
        }
    }

    fn take_list_capture(&mut self, local: &ListLocal) -> EvaluatedListCapture {
        match local {
            ListLocal::Parameter { local, .. } => EvaluatedListCapture::Parameter {
                local: *local,
                value: take_last(&mut self.parameter_lists),
            },
            ListLocal::ParameterList { local, .. } => EvaluatedListCapture::ParameterList {
                local: *local,
                value: take_last(&mut self.parameter_list_lists),
            },
            ListLocal::Int { local, .. } => EvaluatedListCapture::Int {
                local: *local,
                value: take_last(&mut self.int_lists),
            },
            ListLocal::String { local, .. } => EvaluatedListCapture::String {
                local: *local,
                value: take_last(&mut self.string_lists),
            },
            ListLocal::BitArray { local, .. } => EvaluatedListCapture::BitArray {
                local: *local,
                value: take_last(&mut self.bit_array_lists),
            },
            ListLocal::UtfCodepoint { local, .. } => EvaluatedListCapture::UtfCodepoint {
                local: *local,
                value: take_last(&mut self.utf_codepoint_lists),
            },
            ListLocal::Custom { local, .. } => EvaluatedListCapture::Custom {
                local: *local,
                value: take_last(&mut self.custom_lists),
            },
            ListLocal::External { local, .. } => EvaluatedListCapture::External {
                local: *local,
                value: take_last(&mut self.external_lists),
            },
            ListLocal::Float { local, .. } => EvaluatedListCapture::Float {
                local: *local,
                value: take_last(&mut self.float_lists),
            },
            ListLocal::Bool { local, .. } => EvaluatedListCapture::Bool {
                local: *local,
                value: take_last(&mut self.bool_lists),
            },
            ListLocal::Nil { local, .. } => EvaluatedListCapture::Nil {
                local: *local,
                value: take_last(&mut self.nil_lists),
            },
            ListLocal::Tuple { local, .. } => EvaluatedListCapture::Tuple {
                local: *local,
                value: take_last(&mut self.tuple_lists),
            },
            ListLocal::List { local, .. } => EvaluatedListCapture::List {
                local: *local,
                value: take_last(&mut self.list_lists),
            },
            ListLocal::Function { local, .. } => EvaluatedListCapture::Function {
                local: *local,
                value: take_last(&mut self.function_lists),
            },
        }
    }

    fn take_list_function(&mut self, local: &ListFunctionLocal) -> EvaluatedListFunction {
        match local {
            ListFunctionLocal::Parameter { .. } => take_last(&mut self.parameter_list_functions),
            ListFunctionLocal::ParameterList { .. } => {
                take_last(&mut self.parameter_list_list_functions)
            }
            ListFunctionLocal::Int { .. } => take_last(&mut self.int_list_functions),
            ListFunctionLocal::String { .. } => take_last(&mut self.string_list_functions),
            ListFunctionLocal::BitArray { .. } => take_last(&mut self.bit_array_list_functions),
            ListFunctionLocal::UtfCodepoint { .. } => {
                take_last(&mut self.utf_codepoint_list_functions)
            }
            ListFunctionLocal::Custom { .. } => take_last(&mut self.custom_list_functions),
            ListFunctionLocal::External { .. } => take_last(&mut self.external_list_functions)
                .map_runtime_id(crate::plan::execution::function::RuntimeListFunctionId::External),
            ListFunctionLocal::Float { .. } => take_last(&mut self.float_list_functions),
            ListFunctionLocal::Bool { .. } => take_last(&mut self.bool_list_functions),
            ListFunctionLocal::Nil { .. } => take_last(&mut self.nil_list_functions),
            ListFunctionLocal::Tuple { .. } => take_last(&mut self.tuple_list_functions),
            ListFunctionLocal::List { .. } => take_last(&mut self.list_list_functions),
            ListFunctionLocal::Function { .. } => take_last(&mut self.function_list_functions),
        }
    }
}

fn take_last<Value>(values: &mut Vec<Value>) -> Value {
    values.remove(values.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::{BlockEnvironment, ParamLocal, RetainedValues};
    use crate::plan::execution::graph::TupleLocalId;
    use crate::plan::execution::type_::ValueType;
    use crate::runtime::EvaluatedValue;

    fn tuple_capture(value: &EvaluatedValue) -> &[EvaluatedValue] {
        let EvaluatedValue::Tuple(values) = value else {
            panic!("expected tuple capture")
        };
        values
    }

    #[test]
    #[should_panic(expected = "expected tuple capture")]
    fn tuple_capture_observation_rejects_a_scalar() {
        tuple_capture(&EvaluatedValue::Nil);
    }

    #[test]
    fn native_capture_handoff_moves_nested_tuple_buffers_without_copying_arguments() {
        let first = vec![EvaluatedValue::Tuple(vec![EvaluatedValue::Int(20.into())])];
        let second = vec![EvaluatedValue::Tuple(vec![EvaluatedValue::Int(22.into())])];
        let pointers =
            [&first, &second].map(|tuple| (tuple.as_ptr(), tuple_capture(&tuple[0]).as_ptr()));
        let mut inputs = RetainedValues::empty();
        inputs.push_evaluated(EvaluatedValue::Tuple(vec![EvaluatedValue::Bool(true)]));
        inputs.push_evaluated(EvaluatedValue::Tuple(first));
        inputs.push_evaluated(EvaluatedValue::Tuple(second));
        let mut environment = BlockEnvironment::from_retained(inputs);
        for (index, value) in [(2, 22), (1, 20)] {
            let tuple = environment.take_capture_value(&ParamLocal::Tuple {
                local: TupleLocalId(index),
                type_: vec![ValueType::Tuple(vec![ValueType::Int].into())].into(),
            });
            let tuple = tuple_capture(&tuple);
            let nested = tuple_capture(&tuple[0]);
            assert_eq!((tuple.as_ptr(), nested.as_ptr()), pointers[index - 1]);
            assert_eq!(nested, &[EvaluatedValue::Int(value.into())]);
        }
        assert_eq!(
            environment.tuple(TupleLocalId(0)),
            [EvaluatedValue::Bool(true)]
        );
        assert_eq!(environment.values.tuples.len(), 1);
    }
}
