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
use crate::runtime::{LocalValues, RuntimeValueProfile};

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) struct EvaluatedCapture<Profile: RuntimeValueProfile = LocalValues> {
    kind: EvaluatedCaptureKind<Profile>,
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedCaptureKind<Profile: RuntimeValueProfile = LocalValues> {
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
        value: EvaluatedCustomValue<Profile>,
    },
    External {
        local: ExternalLocal,
        value: EvaluatedExternalValue<Profile>,
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
        value: Vec<EvaluatedValue<Profile>>,
    },
    List(EvaluatedListCapture<Profile>),
    IntFunction {
        local: IntFunctionLocalId,
        value: EvaluatedIntFunction<Profile>,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        value: EvaluatedFloatFunction<Profile>,
    },
    StringFunction {
        local: StringFunctionLocalId,
        value: EvaluatedStringFunction<Profile>,
    },
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        value: EvaluatedBitArrayFunction<Profile>,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        value: EvaluatedUtfCodepointFunction<Profile>,
    },
    CustomFunction {
        local: CustomFunctionLocal,
        value: EvaluatedCustomFunction<Profile>,
    },
    ExternalFunction {
        local: ExternalFunctionLocal,
        value: EvaluatedExternalFunction<Profile>,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        value: EvaluatedBoolFunction<Profile>,
    },
    NilFunction {
        local: NilFunctionLocalId,
        value: EvaluatedNilFunction<Profile>,
    },
    TupleFunction {
        local: TupleFunctionLocalId,
        value: EvaluatedTupleFunction<Profile>,
    },
    ListFunction {
        local: ListFunctionLocal,
        value: EvaluatedListFunction<Profile>,
    },
    FunctionFunction {
        local: FunctionFunctionLocal,
        value: EvaluatedFunctionFunction<Profile>,
    },
    GenericFunction {
        local: GenericFunctionLocal,
        value: EvaluatedGenericFunction<Profile>,
    },
    NeverFunction {
        local: NeverFunctionLocal,
        value: EvaluatedNeverFunction<Profile>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub(in crate::runtime) enum EvaluatedListCapture<Profile: RuntimeValueProfile = LocalValues> {
    Parameter {
        local: crate::plan::execution::graph::ParameterListLocalId,
        value: crate::runtime::state::list::ParameterListValueId<Profile>,
    },
    ParameterList {
        local: crate::plan::execution::graph::ParameterListListLocalId,
        value: crate::runtime::state::list::ParameterListListValueId<Profile>,
    },
    Int {
        local: crate::plan::execution::graph::IntListLocalId,
        value: crate::runtime::state::list::IntListValueId<Profile>,
    },
    String {
        local: crate::plan::execution::graph::StringListLocalId,
        value: crate::runtime::state::list::StringListValueId<Profile>,
    },
    BitArray {
        local: crate::plan::execution::graph::BitArrayListLocalId,
        value: crate::runtime::state::list::BitArrayListValueId<Profile>,
    },
    UtfCodepoint {
        local: crate::plan::execution::graph::UtfCodepointListLocalId,
        value: crate::runtime::state::list::UtfCodepointListValueId<Profile>,
    },
    Custom {
        local: crate::plan::execution::graph::CustomListLocalId,
        value: crate::runtime::state::list::CustomListValueId<Profile>,
    },
    External {
        local: crate::plan::execution::graph::ExternalListLocalId,
        value: ExternalListValueId<Profile>,
    },
    Float {
        local: crate::plan::execution::graph::FloatListLocalId,
        value: crate::runtime::state::list::FloatListValueId<Profile>,
    },
    Bool {
        local: crate::plan::execution::graph::BoolListLocalId,
        value: crate::runtime::state::list::BoolListValueId<Profile>,
    },
    Nil {
        local: crate::plan::execution::graph::NilListLocalId,
        value: crate::runtime::state::list::NilListValueId<Profile>,
    },
    Tuple {
        local: crate::plan::execution::graph::TupleListLocalId,
        value: crate::runtime::state::list::TupleListValueId<Profile>,
    },
    List {
        local: crate::plan::execution::graph::ListListLocalId,
        value: crate::runtime::state::list::ListListValueId<Profile>,
    },
    Function {
        local: crate::plan::execution::graph::FunctionListLocalId,
        value: crate::runtime::state::list::FunctionListValueId<Profile>,
    },
}

impl<Profile: RuntimeValueProfile> EvaluatedCapture<Profile> {
    pub(in crate::runtime) fn from_kind(kind: EvaluatedCaptureKind<Profile>) -> Self {
        Self { kind }
    }

    pub(in crate::runtime) fn kind(&self) -> &EvaluatedCaptureKind<Profile> {
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

    pub(in crate::runtime) fn custom(
        local: CustomLocal,
        value: EvaluatedCustomValue<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Custom { local, value })
    }

    pub(in crate::runtime) fn external(
        local: ExternalLocal,
        value: EvaluatedExternalValue<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::External { local, value })
    }

    pub(in crate::runtime) fn bool(local: BoolLocalId, value: bool) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Bool { local, value })
    }

    pub(in crate::runtime) fn nil(local: NilLocalId) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Nil { local })
    }

    pub(in crate::runtime) fn tuple(
        local: TupleLocalId,
        value: Vec<EvaluatedValue<Profile>>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::Tuple { local, value })
    }

    pub(in crate::runtime) fn list(value: EvaluatedListCapture<Profile>) -> Self {
        Self::from_kind(EvaluatedCaptureKind::List(value))
    }

    pub(in crate::runtime) fn int_function(
        local: IntFunctionLocalId,
        value: EvaluatedIntFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::IntFunction { local, value })
    }

    pub(in crate::runtime) fn float_function(
        local: FloatFunctionLocalId,
        value: EvaluatedFloatFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::FloatFunction { local, value })
    }

    pub(in crate::runtime) fn string_function(
        local: StringFunctionLocalId,
        value: EvaluatedStringFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::StringFunction { local, value })
    }

    pub(in crate::runtime) fn bit_array_function(
        local: BitArrayFunctionLocalId,
        value: EvaluatedBitArrayFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::BitArrayFunction { local, value })
    }

    pub(in crate::runtime) fn utf_codepoint_function(
        local: UtfCodepointFunctionLocalId,
        value: EvaluatedUtfCodepointFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::UtfCodepointFunction { local, value })
    }

    pub(in crate::runtime) fn custom_function(
        local: CustomFunctionLocal,
        value: EvaluatedCustomFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::CustomFunction { local, value })
    }

    pub(in crate::runtime) fn external_function(
        local: ExternalFunctionLocal,
        value: EvaluatedExternalFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::ExternalFunction { local, value })
    }

    pub(in crate::runtime) fn bool_function(
        local: BoolFunctionLocalId,
        value: EvaluatedBoolFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::BoolFunction { local, value })
    }

    pub(in crate::runtime) fn nil_function(
        local: NilFunctionLocalId,
        value: EvaluatedNilFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::NilFunction { local, value })
    }

    pub(in crate::runtime) fn tuple_function(
        local: TupleFunctionLocalId,
        value: EvaluatedTupleFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::TupleFunction { local, value })
    }

    pub(in crate::runtime) fn list_function(
        local: ListFunctionLocal,
        value: EvaluatedListFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::ListFunction { local, value })
    }

    pub(in crate::runtime) fn function_function(
        local: FunctionFunctionLocal,
        value: EvaluatedFunctionFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::FunctionFunction { local, value })
    }

    pub(in crate::runtime) fn generic_function(
        local: GenericFunctionLocal,
        value: EvaluatedGenericFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::GenericFunction { local, value })
    }

    pub(in crate::runtime) fn never_function(
        local: NeverFunctionLocal,
        value: EvaluatedNeverFunction<Profile>,
    ) -> Self {
        Self::from_kind(EvaluatedCaptureKind::NeverFunction { local, value })
    }
}

#[cfg(test)]
mod tests {
    use super::{EvaluatedCapture, EvaluatedCaptureKind, EvaluatedListCapture};
    use crate::plan::execution::function::IntFunctionId;
    use crate::plan::execution::graph::{IntFunctionLocalId, IntListLocalId, IntLocalId};
    use crate::runtime::LocalValues;
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
        let function_value: EvaluatedIntFunction = EvaluatedIntFunction::reference(
            IntFunctionId(0),
            Vec::new(),
            Vec::new(),
            crate::plan::execution::type_::FunctionType::new(
                Vec::new(),
                crate::plan::execution::type_::ValueType::Int,
            ),
        );

        assert_eq!(
            EvaluatedCapture::<LocalValues>::int(IntLocalId(0), 1.into()).kind(),
            &EvaluatedCaptureKind::Int {
                local: IntLocalId(0),
                value: 1.into(),
            },
        );
        assert_eq!(
            EvaluatedCapture::<LocalValues>::list(EvaluatedListCapture::Int {
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
            EvaluatedCapture::<LocalValues>::int_function(
                IntFunctionLocalId(0),
                function_value.clone(),
            )
            .kind(),
            &EvaluatedCaptureKind::IntFunction {
                local: IntFunctionLocalId(0),
                value: function_value,
            },
        );
    }
}
