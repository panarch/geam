use bitvec::order::Msb0;
use bitvec::vec::BitVec;
use bitvec::view::BitView;
use std::fmt::{self, Debug, Formatter};
use std::sync::Arc;
use thiserror::Error;

#[derive(Clone)]
pub struct BitArrayValue {
    bytes: Arc<[u8]>,
    byte_offset: usize,
    bit_len: usize,
}

#[derive(Debug, Error, Clone, Copy, PartialEq, Eq)]
#[error("bit length {bit_len} exceeds the {available_bits} bits supplied")]
pub struct BitArrayValueLengthError {
    pub bit_len: usize,
    pub available_bits: usize,
}

impl BitArrayValue {
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        let bit_len = bytes.len().saturating_mul(8);
        Self {
            bytes: bytes.into(),
            byte_offset: 0,
            bit_len,
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

        let mut bytes = bytes;
        bytes.truncate(bit_len.div_ceil(8));
        let remaining = bit_len % 8;
        if let Some(last) = bytes.last_mut()
            && remaining != 0
        {
            *last &= u8::MAX << (8 - remaining);
        }
        Ok(Self {
            bytes: bytes.into(),
            byte_offset: 0,
            bit_len,
        })
    }

    pub fn bytes(&self) -> &[u8] {
        let byte_len = self.bit_len.div_ceil(8);
        &self.bytes[self.byte_offset..self.byte_offset + byte_len]
    }

    pub fn bit_len(&self) -> usize {
        self.bit_len
    }

    pub(crate) fn bits(&self) -> &bitvec::slice::BitSlice<u8, Msb0> {
        &self.bytes().view_bits::<Msb0>()[..self.bit_len]
    }

    pub(crate) fn from_evaluated(mut bits: BitVec<u8, Msb0>) -> Self {
        let bit_len = bits.len();
        bits.force_align();
        bits.set_uninitialized(false);
        Self {
            bytes: bits.into_vec().into(),
            byte_offset: 0,
            bit_len,
        }
    }

    pub(crate) fn byte_slice(&self, start: usize, length: usize) -> Option<Self> {
        if !self.bit_len.is_multiple_of(8) {
            return None;
        }
        let end = start.checked_add(length)?;
        if end > self.bit_len / 8 {
            return None;
        }
        Some(Self {
            bytes: Arc::clone(&self.bytes),
            byte_offset: self.byte_offset + start,
            bit_len: length * 8,
        })
    }

    pub(crate) fn pad_to_bytes(&self) -> Self {
        Self {
            bytes: Arc::clone(&self.bytes),
            byte_offset: self.byte_offset,
            bit_len: self.bit_len.div_ceil(8) * 8,
        }
    }
}

impl Debug for BitArrayValue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("BitArrayValue")
            .field("bytes", &self.bytes())
            .field("bit_len", &self.bit_len)
            .finish()
    }
}

impl PartialEq for BitArrayValue {
    fn eq(&self, other: &Self) -> bool {
        self.bits() == other.bits()
    }
}

impl Eq for BitArrayValue {}

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

        assert!(Arc::ptr_eq(&value.bytes, &clone.bytes));
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

    #[test]
    fn byte_slices_and_padding_share_the_backing_storage() {
        let value = BitArrayValue::from_bytes(vec![1, 2, 3, 4]);
        let slice = value
            .byte_slice(1, 2)
            .expect("aligned in-bounds byte slice should exist");
        let unaligned = BitArrayValue::try_from_parts(vec![0b1010_0000], 4)
            .expect("four supplied bits should be valid");
        let padded = unaligned.pad_to_bytes();

        assert_eq!(slice.bytes(), &[2, 3]);
        assert_eq!(slice.bit_len(), 16);
        assert!(Arc::ptr_eq(&value.bytes, &slice.bytes));
        assert_eq!(padded.bytes(), &[0b1010_0000]);
        assert_eq!(padded.bit_len(), 8);
        assert!(Arc::ptr_eq(&unaligned.bytes, &padded.bytes));
        assert_eq!(unaligned.byte_slice(0, 0), None);
        assert_eq!(value.byte_slice(usize::MAX, 1), None);
        assert_eq!(value.byte_slice(3, 2), None);
    }

    #[test]
    fn equality_and_debug_use_only_the_logical_range() {
        let value = BitArrayValue::from_bytes(vec![1, 2, 3]);
        let slice = value
            .byte_slice(1, 1)
            .expect("middle byte should be sliceable");

        assert_eq!(slice, BitArrayValue::from_bytes(vec![2]));
        assert_eq!(
            format!("{slice:?}"),
            "BitArrayValue { bytes: [2], bit_len: 8 }",
        );
    }

    fn assert_send_sync<Value: Send + Sync>() {}
}
