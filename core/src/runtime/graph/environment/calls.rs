use super::{BlockEnvironment, RetainedValues};
use crate::runtime::compiled::calls::{CallArguments, CallInputs, CallOutput, CallValues};

impl BlockEnvironment {
    pub(in crate::runtime::graph) fn call_inputs(&self) -> CallInputs<'_> {
        CallInputs::new(
            &self.values.ints,
            &self.values.bools,
            &self.values.int_functions,
            &self.values.bool_functions,
        )
    }

    pub(in crate::runtime::graph) fn clear_call_values(&mut self) {
        self.values.ints.clear();
        self.values.bools.clear();
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
