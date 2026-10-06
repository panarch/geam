mod list;

use super::function::EvaluatedFunctionIdentity;
use crate::StringValue;
use crate::plan::execution::function::RuntimeListFunctionId;
use crate::runtime::evaluated::{
    EvaluatedBitArray, EvaluatedBitArrayFunction, EvaluatedBoolFunction,
    EvaluatedCoreFunctionFunction, EvaluatedCustomFunction, EvaluatedCustomValue,
    EvaluatedExternalFunction, EvaluatedExternalFunctionFunction, EvaluatedExternalListFunction,
    EvaluatedExternalValue, EvaluatedFloatFunction, EvaluatedFunctionFunction,
    EvaluatedFunctionValue, EvaluatedFunctionValueKind, EvaluatedGenericFunction,
    EvaluatedIntFunction, EvaluatedListFunction, EvaluatedNeverFunction, EvaluatedNilFunction,
    EvaluatedStringFunction, EvaluatedTupleFunction, EvaluatedUtfCodepointFunction, EvaluatedValue,
};
use crate::runtime::integer::IntegerValue;
use crate::runtime::state::list::ParameterListValueId;
use crate::runtime::{StoredRuntimeValue, captures::Captures};
pub(in crate::runtime) use list::EvaluatedListRef;

// Read-only operations borrow the owner; retained results acquire an owned value.
pub(in crate::runtime) enum EvaluatedValueRef<'value> {
    Int(&'value IntegerValue),
    Float(f64),
    String(&'value StringValue),
    BitArray(&'value EvaluatedBitArray),
    UtfCodepoint(char),
    Custom(&'value EvaluatedCustomValue),
    External(&'value EvaluatedExternalValue),
    Bool(bool),
    Nil,
    Tuple(&'value [EvaluatedValue]),
    ParameterList(ParameterListValueId),
    List(EvaluatedListRef<'value>),
    Function(EvaluatedFunctionRef<'value>),
}

pub(in crate::runtime) enum EvaluatedFunctionRef<'value> {
    Generic(&'value EvaluatedGenericFunction),
    Never(&'value EvaluatedNeverFunction),
    Int(&'value EvaluatedIntFunction),
    Float(&'value EvaluatedFloatFunction),
    String(&'value EvaluatedStringFunction),
    BitArray(&'value EvaluatedBitArrayFunction),
    UtfCodepoint(&'value EvaluatedUtfCodepointFunction),
    Custom(&'value EvaluatedCustomFunction),
    External(&'value EvaluatedExternalFunction),
    Bool(&'value EvaluatedBoolFunction),
    Nil(&'value EvaluatedNilFunction),
    Tuple(&'value EvaluatedTupleFunction),
    List(&'value EvaluatedListFunction),
    ExternalList(&'value EvaluatedExternalListFunction),
    CoreFunction(&'value EvaluatedCoreFunctionFunction),
    ExternalFunction(&'value EvaluatedExternalFunctionFunction),
}

impl<'value> EvaluatedFunctionRef<'value> {
    fn retained_parts(
        &self,
    ) -> (
        &'value EvaluatedFunctionIdentity,
        Option<&'value StoredRuntimeValue>,
        &'value Captures,
    ) {
        match self {
            Self::Generic(value) => value.retained_parts(),
            Self::Never(value) => value.retained_parts(),
            Self::Int(value) => value.retained_parts(),
            Self::Float(value) => value.retained_parts(),
            Self::String(value) => value.retained_parts(),
            Self::BitArray(value) => value.retained_parts(),
            Self::UtfCodepoint(value) => value.retained_parts(),
            Self::External(value) => value.retained_parts(),
            Self::Bool(value) => value.retained_parts(),
            Self::Nil(value) => value.retained_parts(),
            Self::Tuple(value) => value.retained_parts(),
            Self::List(value) => value.retained_parts(),
            Self::ExternalList(value) => value.retained_parts(),
            Self::CoreFunction(value) => value.retained_parts(),
            Self::ExternalFunction(value) => value.retained_parts(),
            Self::Custom(value) => match value {
                EvaluatedCustomFunction::Function(value) => value.retained_parts(),
                EvaluatedCustomFunction::Constructor(value) => value.retained_parts(),
            },
        }
    }

    pub(in crate::runtime) fn identity(&self) -> &'value EvaluatedFunctionIdentity {
        self.retained_parts().0
    }

    pub(in crate::runtime) fn native_source(&self) -> Option<&'value StoredRuntimeValue> {
        self.retained_parts().1
    }

    pub(in crate::runtime) fn capture_frame(&self) -> &'value Captures {
        self.retained_parts().2
    }
}

impl EvaluatedValueRef<'_> {
    pub(in crate::runtime) fn retain(&self) -> EvaluatedValue {
        match self {
            Self::Int(value) => EvaluatedValue::Int((*value).clone()),
            Self::Float(value) => EvaluatedValue::Float(*value),
            Self::String(value) => EvaluatedValue::String((*value).clone()),
            Self::BitArray(value) => EvaluatedValue::BitArray((*value).clone()),
            Self::UtfCodepoint(value) => EvaluatedValue::UtfCodepoint(*value),
            Self::Custom(value) => EvaluatedValue::Custom((*value).clone()),
            Self::External(value) => EvaluatedValue::External((*value).clone()),
            Self::Bool(value) => EvaluatedValue::Bool(*value),
            Self::Nil => EvaluatedValue::Nil,
            Self::Tuple(value) => EvaluatedValue::Tuple(value.to_vec()),
            Self::ParameterList(value) => EvaluatedValue::ParameterList(*value),
            Self::List(value) => value.retain(),
            Self::Function(value) => EvaluatedValue::Function(value.retain()),
        }
    }
}

impl<'value> From<&'value EvaluatedValue> for EvaluatedValueRef<'value> {
    fn from(value: &'value EvaluatedValue) -> Self {
        match value {
            EvaluatedValue::Int(value) => Self::Int(value),
            EvaluatedValue::Float(value) => Self::Float(*value),
            EvaluatedValue::String(value) => Self::String(value),
            EvaluatedValue::BitArray(value) => Self::BitArray(value),
            EvaluatedValue::UtfCodepoint(value) => Self::UtfCodepoint(*value),
            EvaluatedValue::Custom(value) => Self::Custom(value),
            EvaluatedValue::External(value) => Self::External(value),
            EvaluatedValue::Bool(value) => Self::Bool(*value),
            EvaluatedValue::Nil => Self::Nil,
            EvaluatedValue::Tuple(value) => Self::Tuple(value),
            EvaluatedValue::ParameterList(value) => Self::ParameterList(*value),
            EvaluatedValue::List(value) => Self::List(EvaluatedListRef::from(value)),
            EvaluatedValue::Function(value) => Self::Function(EvaluatedFunctionRef::from(value)),
        }
    }
}

impl EvaluatedFunctionRef<'_> {
    fn retain(&self) -> EvaluatedFunctionValue {
        match self {
            Self::Generic(value) => (*value).clone().into(),
            Self::Never(value) => (*value).clone().into(),
            Self::Int(value) => (*value).clone().into(),
            Self::Float(value) => (*value).clone().into(),
            Self::String(value) => (*value).clone().into(),
            Self::BitArray(value) => (*value).clone().into(),
            Self::UtfCodepoint(value) => (*value).clone().into(),
            Self::Custom(value) => (*value).clone().into(),
            Self::External(value) => (*value).clone().into(),
            Self::Bool(value) => (*value).clone().into(),
            Self::Nil(value) => (*value).clone().into(),
            Self::Tuple(value) => (*value).clone().into(),
            Self::List(value) => (*value).clone().into(),
            Self::ExternalList(value) => (*value)
                .clone()
                .map_runtime_id(RuntimeListFunctionId::External)
                .into(),
            Self::CoreFunction(value) => EvaluatedFunctionFunction::Core((*value).clone()).into(),
            Self::ExternalFunction(value) => {
                EvaluatedFunctionFunction::External((*value).clone()).into()
            }
        }
    }
}

impl<'value> From<&'value EvaluatedFunctionValue> for EvaluatedFunctionRef<'value> {
    fn from(value: &'value EvaluatedFunctionValue) -> Self {
        match value.kind() {
            EvaluatedFunctionValueKind::Generic(value) => Self::Generic(value),
            EvaluatedFunctionValueKind::Never(value) => Self::Never(value),
            EvaluatedFunctionValueKind::Int(value) => Self::Int(value),
            EvaluatedFunctionValueKind::Float(value) => Self::Float(value),
            EvaluatedFunctionValueKind::String(value) => Self::String(value),
            EvaluatedFunctionValueKind::BitArray(value) => Self::BitArray(value),
            EvaluatedFunctionValueKind::UtfCodepoint(value) => Self::UtfCodepoint(value),
            EvaluatedFunctionValueKind::Custom(value) => Self::Custom(value),
            EvaluatedFunctionValueKind::External(value) => Self::External(value),
            EvaluatedFunctionValueKind::Bool(value) => Self::Bool(value),
            EvaluatedFunctionValueKind::Nil(value) => Self::Nil(value),
            EvaluatedFunctionValueKind::Tuple(value) => Self::Tuple(value),
            EvaluatedFunctionValueKind::List(value) => Self::List(value),
            EvaluatedFunctionValueKind::Function(EvaluatedFunctionFunction::Core(value)) => {
                Self::CoreFunction(value)
            }
            EvaluatedFunctionValueKind::Function(EvaluatedFunctionFunction::External(value)) => {
                Self::ExternalFunction(value)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EvaluatedValueRef;
    use crate::runtime::borrowed::BorrowedValue;
    use crate::runtime::evaluated::EvaluatedValue;
    use num_bigint::BigInt;
    use std::ptr;

    #[test]
    fn tuple_inspection_borrows_fields_and_only_retained_bindings_outlive_the_owner() {
        let fields = vec![
            EvaluatedValue::Int((BigInt::from(1_u64) << 200_u32).into()),
            EvaluatedValue::String("an independently retained string".into()),
        ];
        assert!(matches!(
            EvaluatedValueRef::from(&fields[0]),
            EvaluatedValueRef::Int(value) if ptr::eq(value, BorrowedValue::from_value(&fields[0]).int())
        ));
        assert!(matches!(
            EvaluatedValueRef::from(&fields[1]),
            EvaluatedValueRef::String(value)
                if ptr::eq(value, BorrowedValue::from_value(&fields[1]).string())
        ));
        let original_fields = fields.as_ptr();
        let original = EvaluatedValue::Tuple(fields);
        assert!(matches!(
            EvaluatedValueRef::from(&original),
            EvaluatedValueRef::Tuple(fields) if ptr::eq(fields.as_ptr(), original_fields)
        ));
        let retained = EvaluatedValueRef::from(&original).retain();
        drop(original);
        assert_eq!(
            retained,
            EvaluatedValue::Tuple(vec![
                EvaluatedValue::Int((BigInt::from(1_u64) << 200_u32).into()),
                EvaluatedValue::String("an independently retained string".into()),
            ])
        );
    }
}
