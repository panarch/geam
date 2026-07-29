use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use std::sync::Arc;
use thiserror::Error;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BitArrayValue {
    bits: Arc<BitVec<u8, Msb0>>,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[error("bit length {bit_len} exceeds the {available_bits} bits supplied")]
pub struct BitArrayValueLengthError {
    pub bit_len: usize,
    pub available_bits: usize,
}

impl BitArrayValue {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        Self {
            bits: Arc::new(BitVec::from_vec(bytes)),
        }
    }

    pub fn try_from_parts(
        bytes: Vec<u8>,
        bit_len: usize,
    ) -> Result<Self, BitArrayValueLengthError> {
        let available_bits = bytes.len().saturating_mul(8);
        if bit_len > available_bits {
            return Err(BitArrayValueLengthError {
                bit_len,
                available_bits,
            });
        }

        let mut bits = BitVec::from_vec(bytes);
        bits.truncate(bit_len);
        let remaining = bit_len % 8;
        if let Some(last) = bits.as_raw_mut_slice().last_mut()
            && remaining != 0
        {
            *last &= u8::MAX << (8 - remaining);
        }
        Ok(Self {
            bits: Arc::new(bits),
        })
    }

    pub fn bytes(&self) -> &[u8] {
        self.bits.as_raw_slice()
    }

    pub fn bit_len(&self) -> usize {
        self.bits.len()
    }

    pub(crate) fn bits(&self) -> &bitvec::slice::BitSlice<u8, Msb0> {
        &self.bits
    }

    pub(crate) fn from_evaluated(bits: Arc<BitVec<u8, Msb0>>) -> Self {
        Self { bits }
    }
}

#[cfg(test)]
mod tests {
    use super::{BitArrayValue, BitArrayValueLengthError};
    use std::sync::Arc;

    #[test]
    fn aligned_bytes_preserve_all_bits() {
        let value = BitArrayValue::from_bytes(vec![0xa5, 0xff]);

        assert_eq!(value.bytes(), &[0xa5, 0xff]);
        assert_eq!(value.bit_len(), 16);
    }

    #[test]
    fn clones_share_the_immutable_bit_storage() {
        let value = BitArrayValue::from_bytes(vec![0xa5]);
        let clone = value.clone();

        assert!(Arc::ptr_eq(&value.bits, &clone.bits));
        assert_send_sync::<BitArrayValue>();
    }

    #[test]
    fn checked_parts_preserve_unaligned_logical_bits() {
        let left = BitArrayValue::try_from_parts(vec![0b1011_1111], 4)
            .expect("four supplied bits should be valid");
        let right = BitArrayValue::try_from_parts(vec![0b1011_0000], 4)
            .expect("four supplied bits should be valid");

        assert_eq!(left, right);
        assert_eq!(left.bytes(), &[0b1011_0000]);
        assert_eq!(left.bit_len(), 4);
    }

    #[test]
    fn checked_parts_preserve_empty_and_aligned_values() {
        assert_eq!(
            BitArrayValue::try_from_parts(Vec::new(), 0),
            Ok(BitArrayValue::from_bytes(Vec::new())),
        );
        assert_eq!(
            BitArrayValue::try_from_parts(vec![0xa5], 8),
            Ok(BitArrayValue::from_bytes(vec![0xa5])),
        );
    }

    #[test]
    fn checked_parts_reject_bit_length_beyond_supplied_bytes() {
        assert_eq!(
            BitArrayValue::try_from_parts(vec![0], 9),
            Err(BitArrayValueLengthError {
                bit_len: 9,
                available_bits: 8,
            }),
        );
    }

    fn assert_send_sync<Value: Send + Sync>() {}
}
