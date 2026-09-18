use ecow::EcoString;
use std::borrow::Borrow;
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter};
use std::hash::{Hash, Hasher};
use std::ops::{Deref, Range};

/// An immutable Gleam string with shared substring storage.
///
/// Clones and larger slices share their original allocation. A retained slice
/// can therefore keep a larger string alive. Use [`Self::detached`] when the
/// visible text needs independent storage.
#[derive(Clone, Default)]
pub struct StringValue {
    backing: EcoString,
    range: Range<usize>,
}

impl StringValue {
    /// Creates an empty string without an allocation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Borrows only the visible UTF-8 text.
    pub fn as_str(&self) -> &str {
        &self.backing[self.range.clone()]
    }

    /// Selects a UTF-8 byte range, returning `None` for invalid bounds.
    pub fn get(&self, range: Range<usize>) -> Option<Self> {
        self.as_str().get(range.clone())?;
        Some(self.slice(range))
    }

    /// Selects a byte range without copying larger substrings.
    ///
    /// Empty and inline-sized substrings do not retain the original allocation.
    /// Like string indexing, this requires ordered, in-bounds UTF-8 boundaries.
    /// Use [`Self::get`] when accepting unchecked byte positions.
    ///
    /// # Panics
    ///
    /// Panics when the range is not a valid UTF-8 slice of this string.
    pub fn slice(&self, range: Range<usize>) -> Self {
        let text = &self.as_str()[range.clone()];
        if text.len() <= EcoString::INLINE_LIMIT {
            Self::from(text)
        } else {
            Self {
                backing: self.backing.clone(),
                range: self.range.start + range.start..self.range.start + range.end,
            }
        }
    }

    /// Copies the visible text without retaining its original allocation.
    pub fn detached(&self) -> Self {
        Self::from(self.as_str())
    }

    /// Returns flat text, reusing storage only when the whole backing is visible.
    pub fn into_ecostring(self) -> EcoString {
        if self.range == (0..self.backing.len()) {
            self.backing
        } else {
            EcoString::from(self.as_str())
        }
    }
}

impl From<EcoString> for StringValue {
    fn from(backing: EcoString) -> Self {
        let end = backing.len();
        Self {
            backing,
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

impl Deref for StringValue {
    type Target = str;

    fn deref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<str> for StringValue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl Borrow<str> for StringValue {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl PartialEq for StringValue {
    fn eq(&self, other: &Self) -> bool {
        self.as_str() == other.as_str()
    }
}

impl Eq for StringValue {}

impl PartialEq<str> for StringValue {
    fn eq(&self, other: &str) -> bool {
        self.as_str() == other
    }
}

impl PartialEq<&str> for StringValue {
    fn eq(&self, other: &&str) -> bool {
        self.as_str() == *other
    }
}

impl PartialOrd for StringValue {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for StringValue {
    fn cmp(&self, other: &Self) -> Ordering {
        self.as_str().cmp(other.as_str())
    }
}

impl Hash for StringValue {
    fn hash<Hasher_: Hasher>(&self, state: &mut Hasher_) {
        self.as_str().hash(state);
    }
}

impl Debug for StringValue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Debug::fmt(self.as_str(), formatter)
    }
}

impl Display for StringValue {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self.as_str(), formatter)
    }
}

#[cfg(test)]
mod tests {
    use super::StringValue;
    use ecow::EcoString;
    use std::borrow::Borrow;
    use std::cmp::Ordering;
    use std::collections::{BTreeSet, HashMap, hash_map::DefaultHasher};
    use std::hash::{Hash, Hasher};

    #[test]
    fn constructs_and_borrows_visible_text() {
        assert_eq!(StringValue::new().as_str(), "");
        assert_eq!(StringValue::default().len(), 0);
        let owned = StringValue::from(String::from("text"));
        assert_eq!(owned.as_ref(), "text");
        assert_eq!(Borrow::<str>::borrow(&owned), "text");
        assert_eq!(&*owned, "text");
        assert_eq!(owned.len(), 4);
        assert!(!owned.is_empty());
        assert!(StringValue::new().is_empty());
    }

    #[test]
    fn nested_views_share_one_backing_after_the_original_is_dropped() {
        let original = StringValue::from("begin:abcdefghijklmnopqrstuvwxyz:end");
        let backing = original.backing.as_ptr();
        let middle = original.slice(6..32);
        let nested = middle.slice(2..22);
        let full = nested.clone();
        assert_eq!(middle.as_str(), "abcdefghijklmnopqrstuvwxyz");
        assert_eq!(nested.as_str(), "cdefghijklmnopqrstuv");
        assert_eq!(middle.backing.as_ptr(), backing);
        assert_eq!(nested.backing.as_ptr(), backing);
        assert_eq!(full.backing.as_ptr(), backing);
        assert_eq!(nested.range, 8..28);
        drop(original);
        drop(middle);
        assert_eq!(nested, full);
        assert_eq!(full.as_str(), "cdefghijklmnopqrstuv");
    }

    #[test]
    fn empty_and_inline_slices_release_the_original_backing() {
        let original = StringValue::from("abcdefghijklmnopqrstuvwxyz");
        let empty = original.slice(9..9);
        let inline = original.slice(2..2 + EcoString::INLINE_LIMIT);
        assert_eq!(empty.backing, "");
        assert_eq!(empty.range, 0..0);
        assert_eq!(
            inline.backing.as_str(),
            &original[2..2 + EcoString::INLINE_LIMIT]
        );
        assert_eq!(inline.range, 0..EcoString::INLINE_LIMIT);
        assert_ne!(inline.backing.as_ptr(), original.backing.as_ptr());
    }

    #[test]
    fn checks_unicode_ranges_and_extreme_positions() {
        let original = StringValue::from("_a\u{1f44d}\u{1f3fd}e\u{301}_");
        let visible = original.slice(1..original.len() - 1);
        assert_eq!(
            visible.get(1..9).map(|value| value.to_string()),
            Some("\u{1f44d}\u{1f3fd}".into())
        );
        assert_eq!(visible.get(2..3), None);
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
    fn indexing_requires_valid_utf8_boundaries() {
        StringValue::from("\u{1f44d}").slice(1..2);
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
        same.as_str().hash(&mut right);
        assert_eq!(left.finish(), right.finish());
        assert_eq!(
            HashMap::from([(visible.clone(), 7)]).get(same.as_str()),
            Some(&7)
        );
        assert_eq!(BTreeSet::from([visible, same]).len(), 1);
    }

    #[test]
    fn copies_explicitly_and_moves_only_complete_backing() {
        let original = StringValue::from("prefix:abcdefghijklmnopqrstuvwxyz:suffix");
        let visible = original.slice(7..33);
        let detached = visible.detached();
        let flat = visible.clone().into_ecostring();
        assert_eq!(detached.as_str(), "abcdefghijklmnopqrstuvwxyz");
        assert_eq!(detached.backing.len(), 26);
        assert_ne!(detached.backing.as_ptr(), original.backing.as_ptr());
        assert_eq!(flat, "abcdefghijklmnopqrstuvwxyz");
        assert_ne!(flat.as_ptr(), original.backing.as_ptr());
        let pointer = original.backing.as_ptr();
        let whole = original.into_ecostring();
        assert_eq!(whole.as_ptr(), pointer);
        assert_eq!(
            StringValue::from(whole).as_str(),
            "prefix:abcdefghijklmnopqrstuvwxyz:suffix"
        );
    }

    #[test]
    fn ecostring_mutation_does_not_change_published_ranges() {
        let mut original = EcoString::from("abcdefghijklmnopqrstuvwxyz");
        let owner = StringValue::from(original.clone());
        let view = owner.slice(1..25);
        original.push_str("changed");
        original.clear();
        assert_eq!(owner.as_str(), "abcdefghijklmnopqrstuvwxyz");
        assert_eq!(view.as_str(), "bcdefghijklmnopqrstuvwxy");
    }

    #[test]
    fn branching_and_sequential_ranges_do_not_copy_large_payloads() {
        let original = StringValue::from("x".repeat(16_384));
        let pointer = original.backing.as_ptr();
        let branches: Vec<_> = (0..128)
            .map(|start| original.slice(start..start + 4096))
            .collect();
        for branch in &branches {
            assert_eq!(branch.backing.as_ptr(), pointer);
            assert_eq!(branch.len(), 4096);
        }
        let mut remaining = original;
        while remaining.len() > EcoString::INLINE_LIMIT + 1 {
            remaining = remaining.slice(1..remaining.len());
            assert_eq!(remaining.backing.as_ptr(), pointer);
        }
        let tail = remaining.slice(1..remaining.len());
        assert_eq!(tail.backing.len(), EcoString::INLINE_LIMIT);
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
                assert_eq!(views.last().map(StringValue::as_str), Some("x"));
                drop(views);
            })
            .expect("thread should start")
            .join()
            .expect("flat range ownership should release on a bounded stack");
    }
}
