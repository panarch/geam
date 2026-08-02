use ecow::EcoString;
use num_bigint::BigInt;

use super::{EvaluatedBitArray, EvaluatedCustomValue, EvaluatedExternalValue, EvaluatedValue};
use super::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedCustomFunction,
    EvaluatedExternalFunction, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedGenericFunction, EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNeverFunction,
    EvaluatedNilFunction, EvaluatedStringFunction, EvaluatedTupleFunction,
    EvaluatedUtfCodepointFunction,
};
use crate::plan::execution::graph::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolLocalId,
    CustomFunctionLocal, CustomLocal, ExternalFunctionLocal, ExternalLocal, FloatFunctionLocalId,
    FloatLocalId, FunctionFunctionLocal, GenericFunctionLocal, IntFunctionLocalId, IntLocalId,
    ListFunctionLocal, NeverFunctionLocal, NilFunctionLocalId, NilLocalId, StringFunctionLocalId,
    StringLocalId, TupleFunctionLocalId, TupleLocalId, UtfCodepointFunctionLocalId,
    UtfCodepointLocalId,
};
use crate::runtime::state::list::ExternalListValueId;

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) struct EvaluatedCapture {
    kind: EvaluatedCaptureKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedCaptureKind {
    Int {
        local: IntLocalId,
        value: BigInt,
    },
    Float {
        local: FloatLocalId,
        value: f64,
    },
    String {
        local: StringLocalId,
        value: EcoString,
    },
    BitArray {
        local: BitArrayLocalId,
        value: EvaluatedBitArray,
    },
    UtfCodepoint {
        local: UtfCodepointLocalId,
        value: char,
    },
    Custom {
        local: CustomLocal,
        value: EvaluatedCustomValue,
    },
    External {
        local: ExternalLocal,
        value: EvaluatedExternalValue,
    },
    Bool {
        local: BoolLocalId,
        value: bool,
    },
    Nil {
        local: NilLocalId,
    },
    Tuple {
        local: TupleLocalId,
        value: Vec<EvaluatedValue>,
    },
    List(EvaluatedListCapture),
    IntFunction {
        local: IntFunctionLocalId,
        value: EvaluatedIntFunction,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: EvaluatedFloatFunction,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: EvaluatedStringFunction,
    },
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        value: EvaluatedBitArrayFunction,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        value: EvaluatedUtfCodepointFunction,
    },
    CustomFunction {
        local: CustomFunctionLocal,
        value: EvaluatedCustomFunction,
    },
    ExternalFunction {
        local: ExternalFunctionLocal,
        value: EvaluatedExternalFunction,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: EvaluatedBoolFunction,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: EvaluatedNilFunction,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: EvaluatedTupleFunction,
    },
    ListFunction {
        local: ListFunctionLocal,
        value: EvaluatedListFunction,
    },
    FunctionFunction {
        local: FunctionFunctionLocal,
        value: EvaluatedFunctionFunction,
    },
    GenericFunction {
        local: GenericFunctionLocal,
        value: EvaluatedGenericFunction,
    },
    NeverFunction {
        local: NeverFunctionLocal,
        value: EvaluatedNeverFunction,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedListCapture {
    Parameter {
        local: crate::plan::execution::graph::ParameterListLocalId,
        value: crate::runtime::state::list::ParameterListValueId,
    },
    ParameterList {
        local: crate::plan::execution::graph::ParameterListListLocalId,
        value: crate::runtime::state::list::ParameterListListValueId,
    },
    Int {
        local: crate::plan::execution::graph::IntListLocalId,
        value: crate::runtime::state::list::IntListValueId,
    },
    String {
        local: crate::plan::execution::graph::StringListLocalId,
        value: crate::runtime::state::list::StringListValueId,
    },
    BitArray {
        local: crate::plan::execution::graph::BitArrayListLocalId,
        value: crate::runtime::state::list::BitArrayListValueId,
    },
    UtfCodepoint {
        local: crate::plan::execution::graph::UtfCodepointListLocalId,
        value: crate::runtime::state::list::UtfCodepointListValueId,
    },
    Custom {
        local: crate::plan::execution::graph::CustomListLocalId,
        value: crate::runtime::state::list::CustomListValueId,
    },
    External {
        local: crate::plan::execution::graph::ExternalListLocalId,
        value: ExternalListValueId,
    },
    Float {
        local: crate::plan::execution::graph::FloatListLocalId,
        value: crate::runtime::state::list::FloatListValueId,
    },
    Bool {
        local: crate::plan::execution::graph::BoolListLocalId,
        value: crate::runtime::state::list::BoolListValueId,
    },
    Nil {
        local: crate::plan::execution::graph::NilListLocalId,
        value: crate::runtime::state::list::NilListValueId,
    },
    Tuple {
        local: crate::plan::execution::graph::TupleListLocalId,
        value: crate::runtime::state::list::TupleListValueId,
    },
    List {
        local: crate::plan::execution::graph::ListListLocalId,
        value: crate::runtime::state::list::ListListValueId,
    },
    Function {
        local: crate::plan::execution::graph::FunctionListLocalId,
        value: crate::runtime::state::list::FunctionListValueId,
    },
}

impl EvaluatedCapture {
    pub(in crate::runtime) fn from_kind(kind: EvaluatedCaptureKind) -> Self {
        Self { kind }
    }

    pub(in crate::runtime) fn kind(&self) -> &EvaluatedCaptureKind {
        &self.kind
    }

    pub(in crate::runtime) fn int(local: IntLocalId, value: BigInt) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Int { local, value })
    }

    pub(in crate::runtime) fn float(local: FloatLocalId, value: f64) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Float { local, value })
    }

    pub(in crate::runtime) fn string(local: StringLocalId, value: EcoString) -> Self {
        Self::from_kind(EvaluatedCaptureKind::String { local, value })
    }

    pub(in crate::runtime) fn bit_array(local: BitArrayLocalId, value: EvaluatedBitArray) -> Self {
        Self::from_kind(EvaluatedCaptureKind::BitArray { local, value })
    }

    pub(in crate::runtime) fn utf_codepoint(local: UtfCodepointLocalId, value: char) -> Self {
        Self::from_kind(EvaluatedCaptureKind::UtfCodepoint { local, value })
    }

    pub(in crate::runtime) fn custom(local: CustomLocal, value: EvaluatedCustomValue) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Custom { local, value })
    }

    pub(in crate::runtime) fn external(
        local: ExternalLocal,
        value: EvaluatedExternalValue,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::External { local, value })
    }

    pub(in crate::runtime) fn bool(local: BoolLocalId, value: bool) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Bool { local, value })
    }

    pub(in crate::runtime) fn nil(local: NilLocalId) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Nil { local })
    }

    pub(in crate::runtime) fn tuple(local: TupleLocalId, value: Vec<EvaluatedValue>) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Tuple { local, value })
    }

    pub(in crate::runtime) fn list(value: EvaluatedListCapture) -> Self {
        Self::from_kind(EvaluatedCaptureKind::List(value))
    }

    pub(in crate::runtime) fn int_function(
        local: IntFunctionLocalId,
        value: EvaluatedIntFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::IntFunction { local, value })
    }

    pub(in crate::runtime) fn generic_function(
        local: GenericFunctionLocal,
        value: EvaluatedGenericFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::GenericFunction { local, value })
    }

    pub(in crate::runtime) fn never_function(
        local: NeverFunctionLocal,
        value: EvaluatedNeverFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::NeverFunction { local, value })
    }

    pub(in crate::runtime) fn float_function(
        local: FloatFunctionLocalId,
        value: EvaluatedFloatFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::FloatFunction { local, value })
    }

    pub(in crate::runtime) fn string_function(
        local: StringFunctionLocalId,
        value: EvaluatedStringFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::StringFunction { local, value })
    }

    pub(in crate::runtime) fn bit_array_function(
        local: BitArrayFunctionLocalId,
        value: EvaluatedBitArrayFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::BitArrayFunction { local, value })
    }

    pub(in crate::runtime) fn utf_codepoint_function(
        local: UtfCodepointFunctionLocalId,
        value: EvaluatedUtfCodepointFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::UtfCodepointFunction { local, value })
    }

    pub(in crate::runtime) fn custom_function(
        local: CustomFunctionLocal,
        value: EvaluatedCustomFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::CustomFunction { local, value })
    }

    pub(in crate::runtime) fn external_function(
        local: ExternalFunctionLocal,
        value: EvaluatedExternalFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::ExternalFunction { local, value })
    }

    pub(in crate::runtime) fn bool_function(
        local: BoolFunctionLocalId,
        value: EvaluatedBoolFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::BoolFunction { local, value })
    }

    pub(in crate::runtime) fn nil_function(
        local: NilFunctionLocalId,
        value: EvaluatedNilFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::NilFunction { local, value })
    }

    pub(in crate::runtime) fn tuple_function(
        local: TupleFunctionLocalId,
        value: EvaluatedTupleFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::TupleFunction { local, value })
    }

    pub(in crate::runtime) fn list_function(
        local: ListFunctionLocal,
        value: EvaluatedListFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::ListFunction { local, value })
    }

    pub(in crate::runtime) fn function_function(
        local: FunctionFunctionLocal,
        value: EvaluatedFunctionFunction,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::FunctionFunction { local, value })
    }
}

#[cfg(test)]
mod tests {
    use super::{EvaluatedCapture, EvaluatedCaptureKind, EvaluatedListCapture};
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::{IntFunctionLocalId, IntListLocalId, IntLocalId};
    use crate::runtime::evaluated::EvaluatedIntFunction;
    use crate::runtime::state::RuntimeState;

    #[test]
    fn capture_constructors_preserve_scalar_list_and_function_kinds() {
        let plan = crate::runtime::plan_src(
            r#"
fn ints() -> List(Int) { [] }

pub fn main() {
  let _ = ints
  0
}
"#,
        );
        let mut echo = Vec::new();
        let mut state = RuntimeState::new(&mut echo);
        let list_value = state
            .lists_mut()
            .int(plan.int_list_function_id(0).type_id(), vec![1.into()]);
        let function_value = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Int,
            ),
        );

        assert_eq!(
            EvaluatedCapture::int(IntLocalId(0), 1.into()).kind(),
            &EvaluatedCaptureKind::Int {
                local: IntLocalId(0),
                value: 1.into(),
            },
        );
        assert_eq!(
            EvaluatedCapture::list(EvaluatedListCapture::Int {
                local: IntListLocalId(0),
                value: list_value.clone(),
            })
            .kind(),
            &EvaluatedCaptureKind::List(EvaluatedListCapture::Int {
                local: IntListLocalId(0),
                value: list_value,
            }),
        );
        assert_eq!(
            EvaluatedCapture::int_function(IntFunctionLocalId(0), function_value.clone()).kind(),
            &EvaluatedCaptureKind::IntFunction {
                local: IntFunctionLocalId(0),
                value: function_value,
            },
        );
    }
}
