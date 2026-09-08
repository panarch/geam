use crate::runtime::EvaluatedValue;
use crate::runtime::graph::RetainedValues;

pub(crate) struct RetainedInputs {
    retained: RetainedValues,
}

pub(crate) struct CallbackInputs {
    arguments: Vec<EvaluatedValue>,
}

impl RetainedInputs {
    pub(crate) fn empty() -> Self {
        Self {
            retained: RetainedValues::empty(),
        }
    }

    pub(in crate::runtime) fn push_value(&mut self, value: EvaluatedValue) {
        self.retained.push_evaluated(value);
    }

    pub(crate) fn push_input(&mut self, input: crate::runtime::EmbeddingInput) {
        self.push_value(input.into_value());
    }

    pub(in crate::runtime) fn into_retained(self) -> RetainedValues {
        self.retained
    }
}

impl CallbackInputs {
    pub(crate) fn new() -> Self {
        Self {
            arguments: Vec::new(),
        }
    }

    pub(in crate::runtime) fn push_value(&mut self, value: EvaluatedValue) {
        self.arguments.push(value);
    }

    pub(in crate::runtime) fn into_arguments(self) -> Box<[EvaluatedValue]> {
        self.arguments.into_boxed_slice()
    }
}

#[cfg(test)]
mod tests {
    use super::{CallbackInputs, EvaluatedValue, RetainedInputs};
    use crate::host::{HostCallArguments, HostParameterLayout};
    use crate::runtime::EmbeddingInputValue;
    use ecow::EcoString;
    use num_bigint::BigInt;

    #[test]
    fn callback_arguments_preserve_source_order_and_family_local_storage() {
        let mut inputs = CallbackInputs::new();
        inputs.push_value(EvaluatedValue::String("first".into()));
        inputs.push_value(EvaluatedValue::Int(42.into()));
        inputs.push_value(EvaluatedValue::Nil);
        inputs.push_value(EvaluatedValue::String("last".into()));
        let arguments = inputs.into_arguments();
        assert_eq!(
            arguments.as_ref(),
            &[
                EvaluatedValue::String("first".into()),
                EvaluatedValue::Int(42.into()),
                EvaluatedValue::Nil,
                EvaluatedValue::String("last".into()),
            ]
        );
        let mut layout = HostParameterLayout::default();
        let first = layout.register::<EcoString>();
        let number = layout.register::<BigInt>();
        let nil = layout.register::<()>();
        let last = layout.register::<EcoString>();
        let mut direct = RetainedInputs::empty();
        direct.push_input(EcoString::from("first").into_input());
        direct.push_input(BigInt::from(42).into_input());
        direct.push_input(().into_input());
        direct.push_input(EcoString::from("last").into_input());
        let direct = direct.into_retained();
        assert_eq!(direct.string(first), "first");
        assert_eq!(direct.int(number), BigInt::from(42));
        assert_eq!(direct.nil(nil), ());
        assert_eq!(direct.string(last), "last");
    }

    #[test]
    fn zero_arity_callback_owns_an_empty_source_argument_sequence() {
        let arguments = CallbackInputs::new().into_arguments();
        assert!(arguments.is_empty());
    }
}
