use std::collections::BTreeSet;

// A backward obligation accepts every length at or above minimum, plus lengths
// already excluded by later branches. Normalize its upper end for cycle checks.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct Length {
    minimum: usize,
    excluded: BTreeSet<usize>,
}

impl Length {
    pub(super) fn new(minimum: usize) -> Self {
        Self {
            minimum,
            excluded: BTreeSet::new(),
        }
    }

    pub(super) fn minimum(&self) -> usize {
        self.minimum
    }

    pub(super) fn accepts(&self, length: usize) -> bool {
        length >= self.minimum || self.excluded.contains(&length)
    }

    pub(super) fn excluding(&self, length: usize) -> Self {
        let mut result = self.clone();
        if length < result.minimum {
            result.excluded.insert(length);
            while result.minimum > 0 && result.excluded.remove(&(result.minimum - 1)) {
                result.minimum -= 1;
            }
        }
        result
    }

    pub(super) fn after_prepend(&self, count: usize) -> Self {
        Self {
            minimum: self.minimum.saturating_sub(count),
            excluded: self
                .excluded
                .iter()
                .filter_map(|length| length.checked_sub(count))
                .collect(),
        }
    }

    pub(super) fn before_drop(&self, count: usize) -> Option<Self> {
        Some(Self {
            minimum: self.minimum.checked_add(count)?,
            excluded: self.excluded.iter().map(|length| length + count).collect(),
        })
    }

    pub(super) fn covers(&self, required: &Self) -> bool {
        self.minimum >= required.minimum
            && self.excluded.iter().all(|length| required.accepts(*length))
    }
}

#[cfg(test)]
mod tests {
    use super::Length;
    use std::collections::BTreeSet;

    #[test]
    fn exclusions_accumulate_in_any_order_without_skipping_an_unproved_length() {
        for order in [[0, 1, 2], [2, 0, 1], [1, 2, 0]] {
            let mut required = Length::new(3);
            for (index, length) in order.into_iter().enumerate() {
                required = required.excluding(length);
                for candidate in 0..5 {
                    assert_eq!(
                        required.accepts(candidate),
                        candidate >= 3 || order[..=index].contains(&candidate)
                    );
                }
            }
            assert_eq!(required, Length::new(0));
        }
        assert_eq!(Length::new(0).excluding(0), Length::new(0));
        assert_eq!(Length::new(2).excluding(2), Length::new(2));
        assert_eq!(
            Length::new(3).excluding(1).excluding(1),
            Length::new(3).excluding(1)
        );
    }

    #[test]
    fn list_construction_translates_the_remaining_obligation() {
        let required = Length::new(5).excluding(1).excluding(3);
        assert_eq!(required.after_prepend(2), Length::new(3).excluding(1));
        assert_eq!(required.after_prepend(5), Length::new(0));
        assert_eq!(required.after_prepend(6), Length::new(0));
        assert_eq!(
            required.before_drop(2),
            Some(Length {
                minimum: 7,
                excluded: BTreeSet::from([3, 5])
            })
        );
        assert_eq!(Length::new(usize::MAX).before_drop(1), None);
    }

    #[test]
    fn cycles_must_preserve_every_remaining_length_requirement() {
        let cases = [
            (Length::new(2), Length::new(1), true),
            (Length::new(1), Length::new(2), false),
            (Length::new(3).excluding(0), Length::new(3), false),
            (Length::new(3), Length::new(3).excluding(0), true),
            (Length::new(4).excluding(2), Length::new(2), true),
            (
                Length::new(3).excluding(0),
                Length::new(3).excluding(0),
                true,
            ),
        ];
        for (known, required, expected) in cases {
            assert_eq!(known.covers(&required), expected);
        }
    }
}
