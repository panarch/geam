use super::BlockEnvironment;
use crate::plan::execution::compiled::CompiledCallbacks;
use crate::runtime::captures::ExecutionDomain;
use crate::runtime::compiled::custom::CustomInput;
use crate::runtime::compiled::custom_loop::{
    BoolCallback, CustomList, CustomLoopValues, IntCallback,
};
use crate::runtime::integer::IntegerValue;
use std::mem;

impl BlockEnvironment {
    pub(in crate::runtime::graph) fn supports_custom_loop(
        &self,
        callbacks: &CompiledCallbacks,
        domain: ExecutionDomain,
    ) -> bool {
        self.values.int_functions.iter().all(|function| {
            callbacks.int(function.runtime_id()).is_some()
                && function
                    .capture_frame()
                    .domain()
                    .is_none_or(|origin| origin == domain)
        }) && self.values.bool_functions.iter().all(|function| {
            callbacks.bool(function.runtime_id()).is_some()
                && function
                    .capture_frame()
                    .domain()
                    .is_none_or(|origin| origin == domain)
        })
    }

    pub(in crate::runtime::graph) fn load_custom_loop(
        &mut self,
        values: &mut CustomLoopValues,
        callbacks: &CompiledCallbacks,
        domain: ExecutionDomain,
    ) -> bool {
        let Some(ints) = self
            .values
            .int_functions
            .iter()
            .cloned()
            .map(|value| IntCallback::bind(value, callbacks, domain))
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        let Some(bools) = self
            .values
            .bool_functions
            .iter()
            .cloned()
            .map(|value| BoolCallback::bind(value, callbacks, domain))
            .collect::<Option<Vec<_>>>()
        else {
            return false;
        };
        values.int_functions = ints;
        values.bool_functions = bools;
        if self.values.ints.iter().any(|value| value.small().is_none()) {
            return false;
        }
        values.ints.clear();
        values.ints.extend(
            self.values
                .ints
                .iter()
                .filter_map(IntegerValue::small)
                .map(i128::from),
        );
        self.values.ints.clear();
        values.bools.clear();
        mem::swap(&mut values.bools, &mut self.values.bools);
        values.customs.clear();
        values
            .customs
            .extend(self.values.customs.drain(..).map(CustomInput));
        values.custom_lists.clear();
        values
            .custom_lists
            .extend(self.values.custom_lists.drain(..).map(CustomList));
        self.values.int_functions.clear();
        self.values.bool_functions.clear();
        true
    }

    pub(in crate::runtime::graph) fn restore_custom_loop(&mut self, values: &mut CustomLoopValues) {
        self.values.ints.clear();
        self.values
            .ints
            .extend(values.ints.drain(..).map(IntegerValue::from));
        self.values.bools.clear();
        mem::swap(&mut self.values.bools, &mut values.bools);
        self.values
            .customs
            .extend(values.customs.drain(..).map(|value| value.0));
        self.values
            .custom_lists
            .extend(values.custom_lists.drain(..).map(|value| value.0));
        self.values
            .int_functions
            .extend(values.int_functions.drain(..).map(|value| value.value));
        self.values
            .bool_functions
            .extend(values.bool_functions.drain(..).map(|value| value.value));
    }
}

#[cfg(test)]
mod tests {
    use super::{BlockEnvironment, CompiledCallbacks, CustomLoopValues};
    use crate::plan::execution::function::{BoolFunctionFunctionId, IntFunctionFunctionId};
    use crate::runtime::CaptureStorage;
    use crate::runtime::HostCallOrigin;
    use crate::runtime::compiled::custom_loop::{BoolCallback, IntCallback};
    use crate::runtime::execution::Services;
    use crate::runtime::function::Execution;
    use crate::runtime::graph::RetainedValues;
    use std::num::NonZeroUsize;

    #[test]
    fn selection_keeps_uncompiled_and_foreign_source_callbacks_in_the_original_environment() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .build()
            .unwrap();
        let captures = CaptureStorage::default();
        let foreign = CaptureStorage::default();
        let services = Services::<crate::ExecutionPlan>::new(captures.clone());
        let context = services.context();
        let metadata = CompiledCallbacks::interpreted();

        let plan = crate::runtime::plan_src(
            "pub fn main() { let bias = 7 fn(value: Int) { value + bias } }",
        );
        let integer = runtime
            .block_on(
                Execution::new(
                    IntFunctionFunctionId(0),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )
                .drive(&plan, &context, NonZeroUsize::MAX),
            )
            .unwrap()
            .unwrap();
        assert_eq!(integer.capture_frame().domain(), Some(captures.domain()));
        assert_eq!(integer.captures().len(), 1);
        let mut environment = BlockEnvironment::from_retained(RetainedValues::empty());
        environment.push_int_function(integer.clone());
        for domain in [captures.domain(), foreign.domain()] {
            assert!(IntCallback::bind(integer.clone(), &metadata, domain).is_none());
            assert!(!environment.supports_custom_loop(&metadata, domain));
            let mut values = CustomLoopValues::default();
            assert!(!environment.load_custom_loop(&mut values, &metadata, domain));
            assert_eq!(environment.values.int_functions, vec![integer.clone()]);
            assert!(values.int_functions.is_empty() && values.bool_functions.is_empty());
        }

        let plan = crate::runtime::plan_src(
            "pub fn main() { let enabled = True fn(value: Bool) { value && enabled } }",
        );
        let boolean = runtime
            .block_on(
                Execution::new(
                    BoolFunctionFunctionId(0),
                    HostCallOrigin::Entry,
                    RetainedValues::empty(),
                )
                .drive(&plan, &context, NonZeroUsize::MAX),
            )
            .unwrap()
            .unwrap();
        assert_eq!(boolean.capture_frame().domain(), Some(captures.domain()));
        assert_eq!(boolean.captures().len(), 1);
        let mut environment = BlockEnvironment::from_retained(RetainedValues::empty());
        environment.push_bool_function(boolean.clone());
        for domain in [captures.domain(), foreign.domain()] {
            assert!(BoolCallback::bind(boolean.clone(), &metadata, domain).is_none());
            assert!(!environment.supports_custom_loop(&metadata, domain));
            let mut values = CustomLoopValues::default();
            assert!(!environment.load_custom_loop(&mut values, &metadata, domain));
            assert_eq!(environment.values.bool_functions, vec![boolean.clone()]);
            assert!(values.int_functions.is_empty() && values.bool_functions.is_empty());
        }

        // This checks binding and ownership lookup only. These metadata-only
        // entries are never admitted or executed as a prepared plan.
        use crate::plan::execution::compiled::CompiledCallback;
        use crate::runtime::compiled::tests::metadata_callback;
        let metadata = CompiledCallbacks {
            ints: vec![CompiledCallback {
                function: integer.runtime_id(),
                entry: 0,
                checkpoints: vec![].into(),
                returns: vec![].into(),
                run: metadata_callback,
            }]
            .into(),
            bools: vec![CompiledCallback {
                function: boolean.runtime_id(),
                entry: 0,
                checkpoints: vec![].into(),
                returns: vec![].into(),
                run: metadata_callback,
            }]
            .into(),
        };
        let mut environment = BlockEnvironment::from_retained(RetainedValues::empty());
        environment.push_int_function(integer.clone());
        environment.push_bool_function(boolean.clone());
        assert!(environment.supports_custom_loop(&metadata, captures.domain()));
        assert!(!environment.supports_custom_loop(&metadata, foreign.domain()));
        let integer_binding =
            IntCallback::bind(integer.clone(), &metadata, captures.domain()).unwrap();
        let boolean_binding =
            BoolCallback::bind(boolean.clone(), &metadata, captures.domain()).unwrap();
        let integer_clone = integer_binding.clone();
        let boolean_clone = boolean_binding.clone();
        drop(integer_binding);
        drop(boolean_binding);
        assert_eq!(integer_clone.value, integer);
        assert_eq!(boolean_clone.value, boolean);
        let mut environment = BlockEnvironment::from_retained(RetainedValues::empty());
        environment.push_bool_function(boolean);
        assert!(!environment.supports_custom_loop(&metadata, foreign.domain()));
    }
}
