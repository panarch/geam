use super::scoped::StoredRuntimeValue;
use crate::host::{HostRetainedType, HostRetainedValue, HostTypeParameter, RetainedAbi};
use crate::runtime::BorrowedValue;
use crate::runtime::evaluated::{EvaluatedBitArray, EvaluatedValue};
use crate::runtime::integer::IntegerValue;
use crate::{BitArrayValue, StringValue};
use num_bigint::BigInt;
use std::marker::PhantomData;

impl<const INDEX: usize> HostRetainedType for HostTypeParameter<INDEX> {
    type Retained = HostRetainedValue<Self>;
}

impl<const INDEX: usize> RetainedAbi<HostRetainedValue<Self>> for HostTypeParameter<INDEX> {
    fn read(value: &StoredRuntimeValue) -> HostRetainedValue<Self> {
        HostRetainedValue {
            value: value.clone_retained(),
            type_: PhantomData,
        }
    }

    fn store(value: HostRetainedValue<Self>, _input: &StoredRuntimeValue) -> StoredRuntimeValue {
        value.value
    }
}

macro_rules! scalar {
    ($type:ty, $variant:ident, $reader:ident, $read:expr, $write:expr) => {
        impl HostRetainedType for $type {
            type Retained = Self;
        }

        impl RetainedAbi<Self> for $type {
            fn read(value: &StoredRuntimeValue) -> Self {
                ($read)(BorrowedValue::from_stored(value).$reader())
            }

            fn store(value: Self, input: &StoredRuntimeValue) -> StoredRuntimeValue {
                StoredRuntimeValue::new(EvaluatedValue::$variant(($write)(value)), input.metadata())
            }
        }
    };
}

scalar!(
    BigInt,
    Int,
    int,
    |v: &IntegerValue| v.bigint().into_owned(),
    |v: BigInt| v.into()
);
scalar!(f64, Float, float, |v| v, |v| v);
scalar!(
    StringValue,
    String,
    string,
    |v: &StringValue| v.clone(),
    |v| v
);
scalar!(
    BitArrayValue,
    BitArray,
    bit_array,
    |v: &BitArrayValue| v.clone(),
    EvaluatedBitArray::from_value
);
scalar!(char, UtfCodepoint, utf_codepoint, |v| v, |v| v);
scalar!(bool, Bool, bool, |v| v, |v| v);

impl HostRetainedType for () {
    type Retained = Self;
}

impl RetainedAbi<Self> for () {
    fn read(_value: &StoredRuntimeValue) -> Self {}

    fn store((): Self, input: &StoredRuntimeValue) -> StoredRuntimeValue {
        StoredRuntimeValue::new(EvaluatedValue::Nil, input.metadata())
    }
}

#[cfg(test)]
mod tests {
    use super::{HostRetainedType, HostTypeParameter, RetainedAbi, StoredRuntimeValue};
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::runtime::BorrowedValue;
    use crate::runtime::evaluated::{EvaluatedBitArray, EvaluatedValue};
    use crate::{BitArrayValue, StringValue};
    use num_bigint::BigInt;

    #[test]
    fn retained_scalar_layouts_round_trip_without_a_scoped_store() {
        let plan = crate::runtime::plan_src("pub fn main() { Nil }");
        let nil = StoredRuntimeValue::new(EvaluatedValue::Nil, plan.value_metadata());
        macro_rules! round_trip {
            ($type:ty, $value:expr, $reader:ident) => {{
                let value: $type = $value;
                let expected: $type = $value;
                let stored = <$type as RetainedAbi<$type>>::store(value, &nil);
                assert_eq!(<$type as RetainedAbi<$type>>::read(&stored), expected);
                let _ = BorrowedValue::from_stored(&stored).$reader();
            }};
        }
        round_trip!(BigInt, BigInt::from(7), int);
        round_trip!(BigInt, BigInt::from(i128::MAX) + 1, int);
        round_trip!(f64, 1.25, float);
        round_trip!(StringValue, "hello".into(), string);
        round_trip!(
            BitArrayValue,
            BitArrayValue::try_from_parts(vec![0xb7], 5).unwrap(),
            bit_array
        );
        round_trip!(char, 'λ', utf_codepoint);
        round_trip!(bool, true, bool);
        assert_eq!(<() as RetainedAbi<()>>::read(&nil), ());
        let stored = <() as RetainedAbi<()>>::store((), &nil);
        assert_eq!(<() as RetainedAbi<()>>::read(&stored), ());
    }

    #[test]
    fn retained_float_bits_and_shared_scalar_owners_survive_storage_and_send() {
        let plan = crate::runtime::plan_src("pub fn main() { Nil }");
        let nil = StoredRuntimeValue::new(EvaluatedValue::Nil, plan.value_metadata());
        for original in [-0.0, f64::INFINITY, f64::from_bits(0x7ff8_0000_0000_0042)] {
            let stored = <f64 as RetainedAbi<f64>>::store(original, &nil);
            assert_eq!(
                <f64 as RetainedAbi<f64>>::read(&stored).to_bits(),
                original.to_bits()
            );
        }
        let string: StringValue = "λ shared".repeat(4096).into();
        let bits = BitArrayValue::try_from_parts(vec![0xb7, 0xc8], 13).unwrap();
        let original_string = string.clone();
        let original_bits = bits.clone();
        let stored_string = <StringValue as RetainedAbi<StringValue>>::store(string, &nil);
        let stored_bits = <BitArrayValue as RetainedAbi<BitArrayValue>>::store(bits, &nil);
        drop(plan);
        drop(nil);
        let (string, bits) = std::thread::spawn(move || {
            (
                <StringValue as RetainedAbi<StringValue>>::read(&stored_string),
                <BitArrayValue as RetainedAbi<BitArrayValue>>::read(&stored_bits),
            )
        })
        .join()
        .unwrap();
        assert_eq!(string, original_string);
        assert_eq!(string.as_str().as_ptr(), original_string.as_str().as_ptr());
        assert_eq!(bits, original_bits);
        assert_eq!(bits.bytes().as_ptr(), original_bits.bytes().as_ptr());
        assert_eq!(bits.bit_len(), 13);
    }

    #[test]
    fn opaque_retention_preserves_the_original_owner_and_can_move_between_threads() {
        type Opaque = HostTypeParameter<0>;
        let plan = crate::runtime::plan_src("pub fn main() { Nil }");
        let value = StoredRuntimeValue::new(
            EvaluatedValue::Tuple(vec![
                EvaluatedValue::String("retained".into()),
                EvaluatedValue::BitArray(EvaluatedBitArray::from_value(BitArrayValue::from_bytes(
                    vec![7],
                ))),
            ]),
            plan.value_metadata(),
        );
        let identity = std::ptr::from_ref(value.value());
        let argument =
            <Opaque as RetainedAbi<<Opaque as HostRetainedType>::Retained>>::read(&value);
        assert_eq!(std::ptr::from_ref(argument.value.value()), identity);
        drop(plan);
        let returned = std::thread::spawn(move || {
            <Opaque as RetainedAbi<<Opaque as HostRetainedType>::Retained>>::store(argument, &value)
        })
        .join()
        .unwrap();
        assert_eq!(std::ptr::from_ref(returned.value()), identity);
        assert_eq!(
            BorrowedValue::from_stored(&returned)
                .tuple_item(0)
                .string()
                .as_str(),
            "retained"
        );
        assert_eq!(
            BorrowedValue::from_stored(&returned)
                .tuple_item(1)
                .bit_array()
                .bytes(),
            [7]
        );
    }
}
