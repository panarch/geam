use ecow::EcoString;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::Range;
use std::str::Utf8Error;
use std::sync::Arc;

/// An immutable Gleam String which preserves its exact bytes.
///
/// Text constructors retain EcoString's inline and shared storage. Native
/// providers can use [`Self::from_bytes`] for arbitrary bytes, and request
/// validated UTF-8 separately through [`Self::as_str`]. Clones and larger
/// slices share their allocation; [`Self::detached`] releases a larger parent.
#[derive(Clone)]
pub struct StringValue {
    backing: StringBacking,
    range: Range<usize>,
}

#[derive(Clone)]
enum StringBacking {
    Text(EcoString),
    Bytes(Arc<Vec<u8>>),
}

impl StringValue {
    /// Creates an empty string without an allocation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Takes ownership of the bytes without UTF-8 validation or a payload copy.
    pub fn from_bytes(bytes: Vec<u8>) -> Self {
        if bytes.is_empty() {
            return Self::new();
        }
        let end = bytes.len();
        Self {
            backing: StringBacking::Bytes(Arc::new(bytes)),
            range: 0..end,
        }
    }

    /// Borrows the exact visible bytes, including invalid UTF-8.
    pub fn as_bytes(&self) -> &[u8] {
        &self.backing.bytes()[self.range.clone()]
    }

    /// Borrows the visible text when those bytes form valid UTF-8.
    ///
    /// Known text ranges need only boundary checks. Raw ranges are validated
    /// when requested; their backing's other bytes do not affect the result.
    pub fn as_str(&self) -> Result<&str, Utf8Error> {
        match self.known_text() {
            Some(text) => Ok(text),
            None => std::str::from_utf8(self.as_bytes()),
        }
    }

    /// Returns the visible byte length.
    pub fn len(&self) -> usize {
        self.range.end - self.range.start
    }

    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }

    pub fn as_ptr(&self) -> *const u8 {
        self.as_bytes().as_ptr()
    }

    /// Compares a prefix without interpreting either value as text.
    pub fn starts_with(&self, prefix: &[u8]) -> bool {
        self.as_bytes().starts_with(prefix)
    }

    /// Compares a suffix without interpreting either value as text.
    pub fn ends_with(&self, suffix: &[u8]) -> bool {
        self.as_bytes().ends_with(suffix)
    }

    /// Concatenates the exact bytes. Known text keeps EcoString's inline path.
    pub fn concat(&self, other: &Self) -> Self {
        match (self.known_text(), other.known_text()) {
            (Some(left), Some(right)) => {
                let mut text = EcoString::with_capacity(left.len() + right.len());
                text.push_str(left);
                text.push_str(right);
                text.into()
            }
            _ => {
                let mut bytes = Vec::with_capacity(self.len() + other.len());
                bytes.extend_from_slice(self.as_bytes());
                bytes.extend_from_slice(other.as_bytes());
                Self::from_bytes(bytes)
            }
        }
    }

    /// Selects an ordered, in-bounds byte range, including partial codepoints.
    pub fn get(&self, range: Range<usize>) -> Option<Self> {
        self.as_bytes().get(range.clone())?;
        Some(self.slice(range))
    }

    /// Selects a byte range without copying larger substrings.
    ///
    /// Empty and small substrings do not retain the original allocation.
    /// This requires ordered, in-bounds byte positions, not UTF-8 boundaries.
    /// Use [`Self::get`] when accepting unchecked byte positions.
    ///
    /// # Panics
    ///
    /// Panics when the byte range is not ordered or is out of bounds.
    pub fn slice(&self, range: Range<usize>) -> Self {
        let bytes = &self.as_bytes()[range.clone()];
        if bytes.len() <= EcoString::INLINE_LIMIT {
            match self.known_text().and_then(|text| text.get(range)) {
                Some(text) => Self::from(text),
                None => Self::from_bytes(bytes.to_vec()),
            }
        } else {
            Self {
                backing: self.backing.clone(),
                range: self.range.start + range.start..self.range.start + range.end,
            }
        }
    }

    /// The last restored range can move the backing instead of retaining it.
    pub(in crate::runtime) fn into_slice(mut self, range: Range<usize>) -> Self {
        let bytes = &self.as_bytes()[range.clone()];
        if bytes.len() <= EcoString::INLINE_LIMIT {
            self.slice(range)
        } else {
            self.range = self.range.start + range.start..self.range.start + range.end;
            self
        }
    }

    /// Copies the visible bytes without retaining the original allocation.
    pub fn detached(&self) -> Self {
        match self.known_text() {
            Some(text) => Self::from(text),
            None => Self::from_bytes(self.as_bytes().to_vec()),
        }
    }

    /// Returns flat text, reusing storage only when the whole backing is visible.
    pub fn into_ecostring(self) -> Result<EcoString, Utf8Error> {
        match self {
            Self {
                backing: StringBacking::Text(text),
                range,
            } if range == (0..text.len()) => Ok(text),
            value => value.as_str().map(EcoString::from),
        }
    }

    fn known_text(&self) -> Option<&str> {
        match &self.backing {
            StringBacking::Text(text) => text.get(self.range.clone()),
            StringBacking::Bytes(_) => None,
        }
    }

    fn fmt_bytes(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        formatter.write_str("<<")?;
        for (index, byte) in self.as_bytes().iter().enumerate() {
            if index > 0 {
                formatter.write_str(", ")?;
            }
            Display::fmt(byte, formatter)?;
        }
        formatter.write_str(">>")
    }
}

impl StringBacking {
    fn bytes(&self) -> &[u8] {
        match self {
            Self::Text(text) => text.as_bytes(),
            Self::Bytes(bytes) => bytes,
        }
    }
}

impl Default for StringValue {
    fn default() -> Self {
        EcoString::new().into()
    }
}

impl From<EcoString> for StringValue {
    fn from(backing: EcoString) -> Self {
        let end = backing.len();
        Self {
            backing: StringBacking::Text(backing),
            range: 0..end,
        }
    }
}

impl From<&str> for StringValue {
    fn from(value: &str) -> Self {
        Self::from(EcoString::from(value))
    }
}

impl From<String> for StringValue {
    fn from(value: String) -> Self {
        Self::from(EcoString::from(value))
    }
}

impl AsRef<[u8]> for StringValue {
    fn as_ref(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl Borrow<[u8]> for StringValue {
    fn borrow(&self) -> &[u8] {
        self.as_bytes()
    }
}

impl PartialEq for StringValue {
    fn eq(&self, other: &Self) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl Eq for StringValue {}

impl PartialEq<str> for StringValue {
    fn eq(&self, other: &str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialEq<&str> for StringValue {
    fn eq(&self, other: &&str) -> bool {
        self.as_bytes() == other.as_bytes()
    }
}

impl PartialOrd for StringValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StringValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_bytes().cmp(other.as_bytes())
    }
}

impl Hash for StringValue {
    fn hash<Hasher_: Hasher>(&self, state: &mut Hasher_) {
        self.as_bytes().hash(state);
    }
}

impl Debug for StringValue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Ok(text) => Debug::fmt(text, formatter),
            Err(_) => self.fmt_bytes(formatter),
        }
    }
}

impl Display for StringValue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        match self.as_str() {
            Ok(text) => Display::fmt(text, formatter),
            Err(_) => self.fmt_bytes(formatter),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{StringBacking, StringValue};
    use ecow::EcoString;
    use std::borrow::Borrow;
    use std::cmp::Ordering;
    use std::collections::{BTreeSet, HashMap, hash_map::DefaultHasher};
    use std::hash::{Hash, Hasher};
    use std::mem::discriminant;
    use std::sync::Arc;

    #[test]
    fn constructs_and_borrows_visible_text() {
        assert_eq!(StringValue::new().as_str(), Ok(""));
        assert_eq!(StringValue::default().len(), 0);
        let owned = StringValue::from(String::from("text"));
        assert_eq!(owned.as_ref(), b"text");
        assert_eq!(Borrow::<[u8]>::borrow(&owned), b"text");
        assert_eq!(owned.as_bytes(), b"text");
        assert_eq!(owned.len(), 4);
        assert!(!owned.is_empty());
        assert!(StringValue::new().is_empty());
    }

    #[test]
    fn nested_views_share_one_backing_after_the_original_is_dropped() {
        let original = StringValue::from("begin:abcdefghijklmnopqrstuvwxyz:end");
        let backing = original.backing.bytes().as_ptr();
        let middle = original.slice(6..32);
        let nested = middle.slice(2..22);
        let full = nested.clone();
        assert_eq!(middle.as_str(), Ok("abcdefghijklmnopqrstuvwxyz"));
        assert_eq!(nested.as_str(), Ok("cdefghijklmnopqrstuv"));
        assert_eq!(middle.backing.bytes().as_ptr(), backing);
        assert_eq!(nested.backing.bytes().as_ptr(), backing);
        assert_eq!(full.backing.bytes().as_ptr(), backing);
        assert_eq!(nested.range, 8..28);
        drop(original);
        drop(middle);
        assert_eq!(nested, full);
        assert_eq!(full.as_str(), Ok("cdefghijklmnopqrstuv"));
    }

    #[test]
    fn empty_and_inline_slices_release_the_original_backing() {
        let original = StringValue::from("abcdefghijklmnopqrstuvwxyz");
        let empty = original.slice(9..9);
        let inline = original.slice(2..2 + EcoString::INLINE_LIMIT);
        assert_eq!(empty.backing.bytes(), b"");
        assert_eq!(empty.range, 0..0);
        assert_eq!(
            inline.backing.bytes(),
            &original.as_bytes()[2..2 + EcoString::INLINE_LIMIT]
        );
        assert_eq!(inline.range, 0..EcoString::INLINE_LIMIT);
        assert_ne!(
            inline.backing.bytes().as_ptr(),
            original.backing.bytes().as_ptr()
        );
    }

    #[test]
    fn consuming_nested_slices_move_large_backings_and_detach_small_ranges() {
        let original = StringValue::from("prefix:가나다abcdefghijklmnopqrstuvwxyz:end");
        let backing = original.backing.bytes().as_ptr();
        let end = original.len() - 4;
        let visible = original.into_slice(7..end);
        let length = visible.len();
        let suffix = visible.into_slice(9..length);
        assert_eq!(suffix.as_str(), Ok("abcdefghijklmnopqrstuvwxyz"));
        assert_eq!(suffix.backing.bytes().as_ptr(), backing);
        assert_eq!(suffix.range, 16..42);
        let inline = suffix.clone().into_slice(2..2 + EcoString::INLINE_LIMIT);
        assert_eq!(
            inline.as_bytes(),
            &suffix.as_bytes()[2..2 + EcoString::INLINE_LIMIT]
        );
        assert_eq!(inline.range, 0..EcoString::INLINE_LIMIT);
        assert_ne!(inline.backing.bytes().as_ptr(), backing);
        let empty = suffix.into_slice(9..9);
        assert_eq!(empty.backing.bytes(), b"");
        assert_eq!(empty.range, 0..0);
    }

    #[test]
    fn checks_unicode_ranges_and_extreme_positions() {
        let original = StringValue::from("_a\u{1f44d}\u{1f3fd}e\u{301}_");
        let visible = original.slice(1..original.len() - 1);
        assert_eq!(
            visible.get(1..9).map(|value| value.to_string()),
            Some("\u{1f44d}\u{1f3fd}".into())
        );
        assert_eq!(visible.get(2..3).unwrap().as_bytes(), &[0x9f]);
        assert_eq!(visible.get(std::ops::Range { start: 4, end: 2 }), None);
        assert_eq!(visible.get(0..usize::MAX), None);
        assert_eq!(visible.get(usize::MAX..usize::MAX), None);
        assert_eq!(
            visible.get(visible.len()..visible.len()),
            Some(StringValue::new())
        );
        assert_eq!(visible.get(0..visible.len()), Some(visible.clone()));
    }

    #[test]
    #[should_panic]
    fn slicing_requires_ordered_in_bounds_positions() {
        StringValue::from("\u{1f44d}").slice(1..5);
    }

    #[test]
    fn visible_content_owns_comparison_hashing_and_formatting() {
        let original = StringValue::from("hidden:abcdefghijklmnop\n\"\\:hidden");
        let visible = original.slice(7..26);
        let same = StringValue::from("abcdefghijklmnop\n\"\\");
        let different = original.slice(8..27);
        assert_eq!(visible, same);
        assert_ne!(visible, different);
        assert_eq!(visible.partial_cmp(&same), Some(Ordering::Equal));
        assert_eq!(visible.cmp(&different), Ordering::Less);
        assert!(visible.eq("abcdefghijklmnop\n\"\\"));
        assert_eq!(visible, "abcdefghijklmnop\n\"\\");
        assert_eq!(format!("{visible:?}"), "\"abcdefghijklmnop\\n\\\"\\\\\"");
        assert_eq!(format!("{visible}"), "abcdefghijklmnop\n\"\\");
        let mut left = DefaultHasher::new();
        let mut right = DefaultHasher::new();
        visible.hash(&mut left);
        same.as_bytes().hash(&mut right);
        assert_eq!(left.finish(), right.finish());
        assert_eq!(
            HashMap::from([(visible.clone(), 7)]).get(same.as_bytes()),
            Some(&7)
        );
        assert_eq!(BTreeSet::from([visible, same]).len(), 1);
    }

    #[test]
    fn copies_explicitly_and_moves_only_complete_backing() {
        let original = StringValue::from("prefix:abcdefghijklmnopqrstuvwxyz:suffix");
        let visible = original.slice(7..33);
        let detached = visible.detached();
        let flat = visible.clone().into_ecostring().unwrap();
        assert_eq!(detached.as_str(), Ok("abcdefghijklmnopqrstuvwxyz"));
        assert_eq!(detached.backing.bytes().len(), 26);
        assert_ne!(
            detached.backing.bytes().as_ptr(),
            original.backing.bytes().as_ptr()
        );
        assert_eq!(flat, "abcdefghijklmnopqrstuvwxyz");
        assert_ne!(flat.as_ptr(), original.backing.bytes().as_ptr());
        let pointer = original.backing.bytes().as_ptr();
        let whole = original.into_ecostring().unwrap();
        assert_eq!(whole.as_ptr(), pointer);
        assert_eq!(
            StringValue::from(whole).as_str(),
            Ok("prefix:abcdefghijklmnopqrstuvwxyz:suffix")
        );
    }

    #[test]
    fn ecostring_mutation_does_not_change_published_ranges() {
        let mut original = EcoString::from("abcdefghijklmnopqrstuvwxyz");
        let owner = StringValue::from(original.clone());
        let view = owner.slice(1..25);
        original.push_str("changed");
        original.clear();
        assert_eq!(owner.as_str(), Ok("abcdefghijklmnopqrstuvwxyz"));
        assert_eq!(view.as_str(), Ok("bcdefghijklmnopqrstuvwxy"));
    }

    #[test]
    fn branching_and_sequential_ranges_do_not_copy_large_payloads() {
        let original = StringValue::from("x".repeat(16_384));
        let pointer = original.backing.bytes().as_ptr();
        let branches: Vec<_> = (0..128)
            .map(|start| original.slice(start..start + 4096))
            .collect();
        for branch in &branches {
            assert_eq!(branch.backing.bytes().as_ptr(), pointer);
            assert_eq!(branch.len(), 4096);
        }
        let mut remaining = original;
        while remaining.len() > EcoString::INLINE_LIMIT + 1 {
            remaining = remaining.slice(1..remaining.len());
            assert_eq!(remaining.backing.bytes().as_ptr(), pointer);
        }
        let tail = remaining.slice(1..remaining.len());
        assert_eq!(tail.backing.bytes().len(), EcoString::INLINE_LIMIT);
        assert_eq!(tail.len(), EcoString::INLINE_LIMIT);
    }

    #[test]
    fn retained_views_transfer_and_release_without_a_recursive_chain() {
        fn send_sync<Value: Send + Sync>() {}
        send_sync::<StringValue>();
        let original = StringValue::from("x".repeat(20_000));
        let views: Vec<_> = (0..20_000)
            .map(|start| original.slice(start..original.len()))
            .collect();
        drop(original);
        std::thread::Builder::new()
            .stack_size(256 * 1024)
            .spawn(move || {
                assert_eq!(views[0].len(), 20_000);
                assert_eq!(views.last().map(StringValue::as_str), Some(Ok("x")));
                drop(views);
            })
            .expect("thread should start")
            .join()
            .expect("flat range ownership should release on a bounded stack");
    }

    #[test]
    fn raw_construction_moves_bytes_and_visible_ranges_decide_text_validity() {
        let bytes = vec![0xff, 0, 0x80, b'a', 0xc3, 0xa9, 0xc3];
        let pointer = bytes.as_ptr();
        let raw = StringValue::from_bytes(bytes);
        assert_eq!(raw.as_ptr(), pointer);
        assert_eq!(raw.as_bytes(), &[0xff, 0, 0x80, b'a', 0xc3, 0xa9, 0xc3]);
        assert!(raw.as_str().is_err());
        assert_eq!(raw.slice(3..6).as_str(), Ok("aé"));
        assert!(raw.slice(6..7).into_ecostring().is_err());
        assert_eq!(raw.slice(3..6).into_ecostring(), Ok(EcoString::from("aé")));
        assert_eq!(StringValue::from_bytes(Vec::new()).as_str(), Ok(""));
        let text = StringValue::from("é");
        let first = text.slice(0..1);
        let second = text.slice(1..2);
        assert!(first.as_str().is_err());
        assert!(second.as_str().is_err());
        assert_eq!(first.concat(&second).as_str(), Ok("é"));
        assert_eq!(first.concat(&second).as_bytes(), text.as_bytes());
        assert_eq!(text.concat(&"!".into()), "é!");
        assert!(raw.starts_with(&[0xff, 0]));
        assert!(raw.ends_with(&[0xc3]));
        assert_eq!(raw.concat(&StringValue::new()), raw);
        assert_eq!(StringValue::new().concat(&raw), raw);
    }

    #[test]
    fn raw_views_share_large_allocations_and_detach_small_ranges() {
        let original = StringValue::from_bytes(vec![0xff; 4096]);
        let pointer = original.as_ptr();
        let middle = original.slice(100..4000);
        let nested = middle.slice(100..3000);
        assert_eq!(nested.as_ptr(), pointer.wrapping_add(200));
        drop(original);
        drop(middle);
        let backing = raw_backing(&nested);
        assert_eq!(Arc::strong_count(backing), 1);
        let moved = nested.into_slice(100..2000);
        assert_eq!(moved.as_ptr(), pointer.wrapping_add(300));
        let backing = raw_backing(&moved);
        assert_eq!(Arc::strong_count(backing), 1);
        let detached = moved.detached();
        assert_eq!(detached, moved);
        assert_ne!(detached.as_ptr(), moved.as_ptr());
        let small = moved.clone().into_slice(0..3);
        assert_eq!(small.as_bytes(), &[0xff; 3]);
        assert_ne!(small.as_ptr(), moved.as_ptr());
        assert_eq!(moved.slice(10..10), StringValue::new());
    }

    fn raw_backing(value: &StringValue) -> &Arc<Vec<u8>> {
        let StringBacking::Bytes(backing) = &value.backing else {
            panic!("expected raw String backing")
        };
        backing
    }

    #[test]
    #[should_panic(expected = "expected raw String backing")]
    fn raw_storage_assertion_rejects_text_backing() {
        raw_backing(&StringValue::from("text"));
    }

    #[test]
    fn byte_borrow_hash_and_diagnostic_display_ignore_the_backing_kind() {
        let text = StringValue::from("é\0");
        let raw = StringValue::from_bytes(vec![0xc3, 0xa9, 0]);
        assert_eq!(text, raw);
        assert_eq!(HashMap::from([(text, 7)]).get(raw.as_bytes()), Some(&7));
        let invalid = StringValue::from_bytes(vec![0, 128, 255, 195]);
        assert_eq!(format!("{invalid:?}"), "<<0, 128, 255, 195>>");
        assert_eq!(format!("{invalid}"), "<<0, 128, 255, 195>>");
        assert_eq!(format!("{raw:?}"), "\"é\\0\"");
        assert!(invalid < StringValue::from_bytes(vec![0, 128, 255, 196]));
    }
    #[test]
    fn larger_text_ranges_can_hold_partial_codepoints_without_losing_aliases() {
        let text = StringValue::from("éabcdefghijklmnopqrstuvwxyz");
        let pointer = text.as_ptr();
        let partial = text.slice(1..text.len());
        assert_eq!(partial.as_ptr(), pointer.wrapping_add(1));
        assert_eq!(discriminant(&partial.backing), discriminant(&text.backing));
        assert!(partial.as_str().is_err());
        assert_eq!(
            partial.slice(1..partial.len()).as_str(),
            Ok("abcdefghijklmnopqrstuvwxyz")
        );
        let detached = partial.detached();
        assert_eq!(detached.as_bytes(), partial.as_bytes());
        assert_ne!(detached.as_ptr(), partial.as_ptr());
        let moved = partial.into_slice(1..27);
        drop(text);
        assert_eq!(moved.as_str(), Ok("abcdefghijklmnopqrstuvwxyz"));
        assert_eq!(moved.as_ptr(), pointer.wrapping_add(2));
    }

    struct BoundedWriter {
        limit: usize,
        output: String,
    }

    impl std::fmt::Write for BoundedWriter {
        fn write_str(&mut self, text: &str) -> std::fmt::Result {
            if self.output.len() + text.len() > self.limit {
                return Err(std::fmt::Error);
            }
            self.output.push_str(text);
            Ok(())
        }
    }

    #[test]
    fn raw_diagnostics_propagate_writer_failures_without_panicking_or_mutating_bytes() {
        use std::fmt::Write;
        let raw = StringValue::from_bytes(vec![255, 0]);
        for limit in [0, 2, 5, 7, 8, 10] {
            for debug in [false, true] {
                let mut writer = BoundedWriter {
                    limit,
                    output: String::new(),
                };
                let result = if debug {
                    write!(writer, "{raw:?}")
                } else {
                    write!(writer, "{raw}")
                };
                if limit < 10 {
                    assert_eq!(result, Err(std::fmt::Error));
                } else {
                    assert_eq!(result, Ok(()));
                    assert_eq!(writer.output, "<<255, 0>>");
                }
                assert_eq!(raw.as_bytes(), &[255, 0]);
            }
        }
    }
}
