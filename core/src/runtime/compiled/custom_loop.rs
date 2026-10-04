use super::CompiledProgress;
use super::custom::{CustomInput, CustomValues};
use crate::plan::execution::compiled::CompiledCallbacks;
use crate::plan::execution::type_::CustomListTypeId;
use crate::runtime::captures::ExecutionDomain;
use crate::runtime::evaluated::{
    EvaluatedBoolFunction, EvaluatedCapture, EvaluatedCaptureKind, EvaluatedIntFunction,
};
use crate::runtime::integer::IntegerValue;
use crate::runtime::state::list::{CustomListValueId, RuntimeListStorage};

pub type CustomLoopKernel =
    fn(usize, &mut CustomLoopValues, &CustomListOps<'_>, &mut usize) -> CustomLoopProgress;

pub type CallbackKernel<Value> =
    fn(&CallbackInputs<'_>, &mut CustomValues, &mut usize) -> CallbackProgress<Value>;

pub enum CustomLoopProgress {
    Caller(CompiledProgress),
    Call {
        point: usize,
        progress: CallbackStop,
    },
}

/// A completion contains the typed result directly. Only interrupted calls
/// populate the canonical callee prefix in CustomValues.
pub enum CallbackProgress<Value> {
    Complete(Value),
    Stopped(CallbackStop),
}

pub enum CallbackStop {
    Yield(usize),
    Interpreted(usize),
    Entry,
}

#[derive(Default)]
pub struct CustomLoopValues {
    pub ints: Vec<i128>,
    pub bools: Vec<bool>,
    pub customs: Vec<CustomInput>,
    pub custom_lists: Vec<CustomList>,
    pub int_functions: Vec<IntCallback>,
    pub bool_functions: Vec<BoolCallback>,
    pub callee: CustomValues,
}

#[derive(Clone)]
pub struct CustomList(pub(in crate::runtime) CustomListValueId);

pub struct CustomListOps<'runtime> {
    storage: &'runtime RuntimeListStorage,
}

pub struct CallbackArguments<'call> {
    pub ints: &'call [i128],
    pub bools: &'call [bool],
    pub customs: &'call [&'call CustomInput],
}

pub struct CallbackInputs<'call> {
    arguments: &'call CallbackArguments<'call>,
    captures: &'call CallbackCaptures,
}

#[derive(Clone)]
pub struct IntCallback {
    pub(in crate::runtime) value: EvaluatedIntFunction,
    run: CallbackKernel<i128>,
    pub(in crate::runtime) binding: usize,
    captures: CallbackCaptures,
}

#[derive(Clone)]
pub struct BoolCallback {
    pub(in crate::runtime) value: EvaluatedBoolFunction,
    run: CallbackKernel<bool>,
    pub(in crate::runtime) binding: usize,
    captures: CallbackCaptures,
}

impl CustomList {
    pub fn len(&self) -> usize {
        self.0.values().len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'runtime> CustomListOps<'runtime> {
    pub(in crate::runtime) fn new(storage: &'runtime RuntimeListStorage) -> Self {
        Self { storage }
    }
    pub fn index(&self, value: &CustomList, index: usize) -> Option<CustomInput> {
        self.storage
            .custom_values(&value.0)
            .get(index)
            .cloned()
            .map(CustomInput)
    }
    pub fn tail(&self, value: &CustomList, type_id: CustomListTypeId, count: usize) -> CustomList {
        CustomList(self.storage.tail_custom(type_id, &value.0, count))
    }
}

impl CallbackInputs<'_> {
    pub fn integer(&self, index: usize) -> Option<i128> {
        if let Some(value) = self.arguments.ints.get(index) {
            Some(*value)
        } else {
            self.captures.ints[index - self.arguments.ints.len()]
                .small()
                .map(i128::from)
        }
    }
    pub fn boolean(&self, index: usize) -> bool {
        if let Some(value) = self.arguments.bools.get(index) {
            *value
        } else {
            self.captures.bools[index - self.arguments.bools.len()]
        }
    }
    pub fn custom(&self, index: usize) -> CustomInput {
        if let Some(value) = self.arguments.customs.get(index) {
            (**value).clone()
        } else {
            self.captures.customs[index - self.arguments.customs.len()].clone()
        }
    }
}

impl IntCallback {
    pub(in crate::runtime) fn bind(
        value: EvaluatedIntFunction,
        compiled: &CompiledCallbacks,
        domain: ExecutionDomain,
    ) -> Option<Self> {
        if value
            .capture_frame()
            .domain()
            .is_some_and(|origin| origin != domain)
        {
            return None;
        }
        let binding = compiled
            .ints
            .binary_search_by_key(&value.runtime_id().0, |entry| entry.function.0)
            .ok()?;
        let run = compiled.ints[binding].run;
        let captures = capture_values(value.captures());
        Some(Self {
            value,
            run,
            captures,
            binding,
        })
    }
    pub fn call(
        &self,
        args: &CallbackArguments<'_>,
        values: &mut CustomValues,
        budget: &mut usize,
    ) -> CallbackProgress<i128> {
        (self.run)(
            &CallbackInputs {
                arguments: args,
                captures: &self.captures,
            },
            values,
            budget,
        )
    }
}

impl BoolCallback {
    pub(in crate::runtime) fn bind(
        value: EvaluatedBoolFunction,
        compiled: &CompiledCallbacks,
        domain: ExecutionDomain,
    ) -> Option<Self> {
        if value
            .capture_frame()
            .domain()
            .is_some_and(|origin| origin != domain)
        {
            return None;
        }
        let binding = compiled
            .bools
            .binary_search_by_key(&value.runtime_id().0, |entry| entry.function.0)
            .ok()?;
        let run = compiled.bools[binding].run;
        let captures = capture_values(value.captures());
        Some(Self {
            value,
            run,
            captures,
            binding,
        })
    }
    pub fn call(
        &self,
        args: &CallbackArguments<'_>,
        values: &mut CustomValues,
        budget: &mut usize,
    ) -> CallbackProgress<bool> {
        (self.run)(
            &CallbackInputs {
                arguments: args,
                captures: &self.captures,
            },
            values,
            budget,
        )
    }
}

#[derive(Default, Clone)]
struct CallbackCaptures {
    ints: Vec<IntegerValue>,
    bools: Vec<bool>,
    customs: Vec<CustomInput>,
}

// Metadata admits only these entry columns. Keep the original function's full
// capture owner for canonical resumption and project the columns read by the
// generated leaf; this is not another capture-layout validation boundary.
fn capture_values(captures: &[EvaluatedCapture]) -> CallbackCaptures {
    let mut values = CallbackCaptures::default();
    for capture in captures {
        match capture.kind() {
            EvaluatedCaptureKind::Int { value, .. } => values.ints.push(value.clone()),
            EvaluatedCaptureKind::Bool { value, .. } => values.bools.push(*value),
            EvaluatedCaptureKind::Custom { value, .. } => {
                values.customs.push(CustomInput(value.clone()))
            }
            _ => {}
        }
    }
    values
}

#[cfg(test)]
mod tests {
    use super::{CallbackArguments, CallbackCaptures, CallbackInputs, CustomInput, capture_values};
    use crate::plan::execution::graph::{
        BoolLocalId, CustomLocal, CustomLocalId, IntLocalId, StringLocalId,
    };
    use crate::plan::execution::type_::{
        CustomConstructorId, CustomTypeId, CustomValueShape, CustomValueShapeId,
    };
    use crate::runtime::compiled::custom::CustomValues;
    use crate::runtime::compiled::tests::metadata_callback;
    use crate::runtime::evaluated::{EvaluatedCapture, EvaluatedCustomValue, EvaluatedValue};
    use num_bigint::BigInt;

    #[test]
    fn inputs_borrow_arguments_then_read_captures_in_each_typed_column() {
        let constructor = CustomConstructorId {
            type_id: CustomTypeId(0),
            index: 0,
        };
        let argument = CustomInput(EvaluatedCustomValue::from_fields(
            constructor,
            vec![EvaluatedValue::Int(42.into())].into_boxed_slice(),
        ));
        let capture = EvaluatedCustomValue::from_fields(
            constructor,
            vec![EvaluatedValue::Int(9.into())].into_boxed_slice(),
        );
        let big: BigInt = BigInt::from(1) << 140;
        let captures = capture_values(&[
            EvaluatedCapture::int(IntLocalId(1), big.into()),
            EvaluatedCapture::bool(BoolLocalId(1), true),
            EvaluatedCapture::custom(
                CustomLocal {
                    id: CustomLocalId(1),
                    shape: CustomValueShape {
                        type_id: CustomTypeId(0),
                        shape_id: CustomValueShapeId(0),
                    },
                },
                capture.clone(),
            ),
            EvaluatedCapture::int(IntLocalId(2), 11.into()),
        ]);
        let arguments = CallbackArguments {
            ints: &[7],
            bools: &[false],
            customs: &[&argument],
        };
        let inputs = CallbackInputs {
            arguments: &arguments,
            captures: &captures,
        };
        assert_eq!(inputs.integer(0), Some(7));
        assert_eq!(inputs.integer(1), None);
        assert_eq!(inputs.integer(2), Some(11));
        assert!(!inputs.boolean(0));
        assert!(inputs.boolean(1));
        assert!(std::ptr::eq(
            inputs.custom(0).0.fields(),
            argument.0.fields()
        ));
        let retained = inputs.custom(1);
        assert!(std::ptr::eq(retained.0.fields(), capture.fields()));
        drop(captures);
        drop(capture);
        assert_eq!(retained.integer(0), Some(9));
        let unrelated = capture_values(&[EvaluatedCapture::string(
            StringLocalId(0),
            "outside the generated columns".into(),
        )]);
        assert!(unrelated.ints.is_empty());
        assert!(unrelated.bools.is_empty());
        assert!(unrelated.customs.is_empty());
    }

    #[test]
    #[should_panic(expected = "metadata fixture must not execute a callback kernel")]
    fn integer_metadata_fixture_rejects_execution() {
        let args = CallbackArguments {
            ints: &[],
            bools: &[],
            customs: &[],
        };
        let captures = CallbackCaptures::default();
        metadata_callback::<i128>(
            &CallbackInputs {
                arguments: &args,
                captures: &captures,
            },
            &mut CustomValues::default(),
            &mut 1,
        );
    }

    #[test]
    #[should_panic(expected = "metadata fixture must not execute a callback kernel")]
    fn boolean_metadata_fixture_rejects_execution() {
        let args = CallbackArguments {
            ints: &[],
            bools: &[],
            customs: &[],
        };
        let captures = CallbackCaptures::default();
        metadata_callback::<bool>(
            &CallbackInputs {
                arguments: &args,
                captures: &captures,
            },
            &mut CustomValues::default(),
            &mut 1,
        );
    }
}
