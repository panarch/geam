use super::{BlockEnvironment, RetainedValues};
use crate::runtime::compiled::calls::{CallArguments, CallInputs, CallOutput, CallValues};

impl BlockEnvironment {
    pub(in crate::runtime::graph) fn call_inputs(&self) -> CallInputs<'_> {
        CallInputs::new(
            &self.values.ints,
            &self.values.bools,
            &self.values.int_lists,
            &self.values.int_functions,
            &self.values.bool_functions,
        )
    }

    pub(in crate::runtime::graph) fn clear_call_values(&mut self) {
        self.values.ints.clear();
        self.values.bools.clear();
        self.values.int_lists.clear();
        self.values.int_functions.clear();
        self.values.bool_functions.clear();
    }

    /// Entry has cleared the columns without releasing capacity. A completed
    /// body needs one result slot, not a snapshot of every prior local.
    pub(in crate::runtime::graph) fn store_call_output(&mut self, output: CallOutput) {
        match output {
            CallOutput::Int(value) => self.values.ints.push(value.0),
            CallOutput::Bool(value) => self.values.bools.push(value),
            CallOutput::IntFunction(value) => self.values.int_functions.push(value.0),
            CallOutput::BoolFunction(value) => self.values.bool_functions.push(value.0),
        }
    }

    pub(in crate::runtime::graph) fn restore_call_values(&mut self, values: CallValues) {
        self.values.ints = values.ints.into_iter().map(|value| value.0).collect();
        self.values.bools = values.bools;
        self.values.int_lists = values.int_lists.into_iter().map(|value| value.0).collect();
        self.values.int_functions = values
            .int_functions
            .into_iter()
            .map(|value| value.0)
            .collect();
        self.values.bool_functions = values
            .bool_functions
            .into_iter()
            .map(|value| value.0)
            .collect();
    }
}

impl CallValues {
    pub(in crate::runtime::graph) fn into_retained(self) -> RetainedValues {
        let mut inputs = RetainedValues::empty();
        inputs.values.ints = self.ints.into_iter().map(|value| value.0).collect();
        inputs.values.bools = self.bools;
        inputs.values.int_lists = self.int_lists.into_iter().map(|value| value.0).collect();
        inputs.values.int_functions = self
            .int_functions
            .into_iter()
            .map(|value| value.0)
            .collect();
        inputs.values.bool_functions = self
            .bool_functions
            .into_iter()
            .map(|value| value.0)
            .collect();
        inputs
    }
}

impl CallArguments {
    pub(in crate::runtime::graph) fn into_retained(self) -> RetainedValues {
        let mut inputs = self.values.into_retained();
        if let Some(captures) = self.captures {
            inputs.append_captures(&captures.0);
        }
        inputs
    }
}

#[cfg(test)]
mod tests {
    use super::BlockEnvironment;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::IntListLocalId;
    use crate::plan::execution::type_::{FunctionType, ValueType};
    use crate::runtime::CaptureStorage;
    use crate::runtime::compiled::calls::{CallArguments, CallCapture, CallOps, CallValues};
    use crate::runtime::compiled::numeric::NumericValues;
    use crate::runtime::plan_src;
    use crate::runtime::state::list::RuntimeListStorage;

    #[test]
    fn list_columns_move_through_entry_restoration_and_callee_capture_order() {
        let plan = plan_src("pub fn main() { [1] }");
        let type_id = plan.int_list_function_id(0).type_id();
        let lists = RuntimeListStorage::default();
        let captures = CaptureStorage::default();
        let mut numeric = NumericValues::default();
        let ops = CallOps::new(&captures, &mut numeric, &lists);
        let argument = ops.lists().value(type_id, &[3, 4]);
        let captured = ops.lists().value(type_id, &[7, 8]);
        let values = CallValues {
            ints: vec![11_i128.into()],
            bools: vec![true],
            int_lists: vec![argument.clone()],
            ..CallValues::default()
        };
        let mut environment = BlockEnvironment::from_retained(values.into_retained());
        assert_eq!(
            environment.values.int_lists.as_slice(),
            std::slice::from_ref(&argument.0)
        );
        assert_eq!(environment.call_inputs().int_list(0).unwrap().0, argument.0);
        assert!(environment.call_inputs().int_list(1).is_none());
        let capacity = environment.values.int_lists.capacity();
        environment.clear_call_values();
        assert!(environment.values.int_lists.is_empty());
        assert_eq!(environment.values.int_lists.capacity(), capacity);
        environment.restore_call_values(CallValues {
            int_lists: vec![captured.clone(), argument.clone()],
            ..CallValues::default()
        });
        assert_eq!(
            environment.values.int_lists,
            [captured.0.clone(), argument.0.clone()]
        );
        assert!(environment.values.ints.is_empty());
        assert!(environment.values.bools.is_empty());
        let function = ops.int_closure(
            IntFunctionId(2),
            FunctionType::new(Vec::new(), ValueType::Int),
            vec![CallCapture::int_list(IntListLocalId(1), captured.clone())],
        );
        let callee = CallArguments {
            values: CallValues {
                int_lists: vec![argument.clone()],
                ..CallValues::default()
            },
            captures: Some(function.captures().retain()),
        }
        .into_retained();
        assert_eq!(callee.values.int_lists, [argument.0, captured.0]);
    }
}
