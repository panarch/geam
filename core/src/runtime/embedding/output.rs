use super::list::EmbeddingList;
use crate::runtime::EvaluatedCustomValue;
use crate::runtime::evaluated::{EvaluatedExternalValue, EvaluatedFunctionValue, EvaluatedValue};
use crate::runtime::state::list::{ParameterListValueId, StoredListValueId};
use ecow::EcoString;
use num_bigint::BigInt;

pub(crate) struct EmbeddingOutput {
    ints: Vec<BigInt>,
    floats: Vec<f64>,
    strings: Vec<EcoString>,
    bit_arrays: Vec<crate::BitArrayValue>,
    utf_codepoints: Vec<char>,
    variants: Vec<usize>,
    _externals: Vec<EvaluatedExternalValue>,
    bools: Vec<bool>,
    _parameter_lists: Vec<ParameterListValueId>,
    lists: Vec<StoredListValueId>,
    _functions: Vec<EvaluatedFunctionValue>,
}

impl EmbeddingOutput {
    pub(super) fn from_value(value: EvaluatedValue) -> Self {
        let mut output = Self::empty();
        output.push_reversed(value);
        output
    }

    pub(super) fn from_tuple(values: Vec<EvaluatedValue>) -> Self {
        let mut output = Self::empty();
        for value in values.into_iter().rev() {
            output.push_reversed(value);
        }
        output
    }

    pub(super) fn from_custom(value: EvaluatedCustomValue) -> Self {
        let mut output = Self::empty();
        output.push_reversed(EvaluatedValue::Custom(value));
        output
    }

    pub(crate) fn take_int(&mut self) -> BigInt {
        take_last(&mut self.ints)
    }

    pub(crate) fn take_float(&mut self) -> f64 {
        take_last(&mut self.floats)
    }

    pub(crate) fn take_string(&mut self) -> EcoString {
        take_last(&mut self.strings)
    }

    pub(crate) fn take_bit_array(&mut self) -> crate::BitArrayValue {
        take_last(&mut self.bit_arrays)
    }

    pub(crate) fn take_utf_codepoint(&mut self) -> char {
        take_last(&mut self.utf_codepoints)
    }

    pub(crate) fn take_variant(&mut self) -> usize {
        take_last(&mut self.variants)
    }

    pub(crate) fn take_bool(&mut self) -> bool {
        take_last(&mut self.bools)
    }

    pub(crate) fn take_nil(&mut self) {}

    pub(crate) fn take_list(&mut self) -> EmbeddingList {
        EmbeddingList::new(take_last(&mut self.lists))
    }

    fn empty() -> Self {
        Self {
            ints: Vec::new(),
            floats: Vec::new(),
            strings: Vec::new(),
            bit_arrays: Vec::new(),
            utf_codepoints: Vec::new(),
            variants: Vec::new(),
            _externals: Vec::new(),
            bools: Vec::new(),
            _parameter_lists: Vec::new(),
            lists: Vec::new(),
            _functions: Vec::new(),
        }
    }

    fn push_reversed(&mut self, value: EvaluatedValue) {
        match value {
            EvaluatedValue::Int(value) => self.ints.push(value),
            EvaluatedValue::Float(value) => self.floats.push(value),
            EvaluatedValue::String(value) => self.strings.push(value),
            EvaluatedValue::BitArray(value) => self.bit_arrays.push(value.into_value()),
            EvaluatedValue::UtfCodepoint(value) => self.utf_codepoints.push(value),
            EvaluatedValue::Custom(value) => {
                let (constructor, fields) = value.into_fields();
                for field in fields.into_vec().into_iter().rev() {
                    self.push_reversed(field);
                }
                self.variants.push(constructor.index());
            }
            EvaluatedValue::External(value) => self._externals.push(value),
            EvaluatedValue::Bool(value) => self.bools.push(value),
            EvaluatedValue::Nil => {}
            EvaluatedValue::Tuple(values) => {
                for value in values.into_iter().rev() {
                    self.push_reversed(value);
                }
            }
            EvaluatedValue::ParameterList(value) => self._parameter_lists.push(value),
            EvaluatedValue::List(value) => self.lists.push(value),
            EvaluatedValue::Function(value) => self._functions.push(value),
        }
    }
}

fn take_last<Value>(values: &mut Vec<Value>) -> Value {
    values.swap_remove(values.len() - 1)
}

#[cfg(test)]
mod tests {
    use super::EmbeddingOutput;
    use crate::host::HostExternalStore;
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::type_::{ExternalTypeId, FunctionType, ValueType};
    use crate::runtime::evaluated::{
        EvaluatedExternalValue, EvaluatedFunctionValue, EvaluatedIntFunction, EvaluatedValue,
    };
    use crate::runtime::state::RuntimeState;
    use crate::runtime::state::list::{ListValueId, ParameterListValueId};

    #[test]
    fn retains_non_data_runtime_families_without_reinterpreting_them() {
        let plan = crate::runtime::plan_src(
            r#"
fn ints() -> List(Int) { [] }
fn parameters(values: List(value)) { values }

pub fn main() {
  let _ = ints
  let _ = parameters
  0
}
"#,
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let list = state
            .lists_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let parameter = ParameterListValueId::new(plan.parameter_list_function_id(0).type_id());
        let store = HostExternalStore::default();
        let external = store.insert(
            crate::host::HostStoredValue::<num_bigint::BigInt>::new(
                crate::runtime::StoredRuntimeValue::test_int(7.into()),
            ),
            |context, left, right| context.stored_values_equal(left, right),
            |context, value| context.stored_value_hash(value),
            |context, value| context.inspect_stored_value(value),
        );
        let external = EvaluatedExternalValue::new(ExternalTypeId::new(0), external);
        let stored_equal =
            |left: &crate::runtime::StoredRuntimeValue,
             right: &crate::runtime::StoredRuntimeValue| left.value() == right.value();
        let stored_hash = |_: &crate::runtime::StoredRuntimeValue| 7;
        let stored_inspect = |_: &crate::runtime::StoredRuntimeValue| "stored".into();
        let equality = crate::host::HostExternalEquality::new(&stored_equal);
        let hashing = crate::host::HostExternalHashing::new(&stored_hash);
        let inspection = crate::host::HostExternalInspection::new(&stored_inspect);
        assert!(external.source_equal(&equality, &external));
        assert_eq!(external.source_hash(&hashing), 7);
        assert_eq!(external.lease().inspection(&inspection), "stored");
        let function = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            FunctionType::new(Vec::new(), ValueType::Int),
        );
        let mut output = EmbeddingOutput::empty();

        output.push_reversed(EvaluatedValue::External(external));
        output.push_reversed(EvaluatedValue::ParameterList(parameter));
        output.push_reversed(EvaluatedValue::from(ListValueId::Int(list)));
        output.push_reversed(EvaluatedValue::Function(EvaluatedFunctionValue::from(
            function,
        )));

        assert_eq!(output._externals.len(), 1);
        assert_eq!(output._parameter_lists.len(), 1);
        assert_eq!(output.lists.len(), 1);
        assert_eq!(output._functions.len(), 1);
    }
}
