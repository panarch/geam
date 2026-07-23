use super::{MatchPattern, MatchPatternBinding};

pub(crate) struct MatchPatternList {
    elements: Box<[MatchPattern]>,
    tail: Option<MatchPatternListTail>,
}

pub(crate) enum MatchPatternListTail {
    Ignore,
    Bind(MatchPatternBinding),
}

impl MatchPatternList {
    pub(in crate::plan::execution) fn new(
        elements: Vec<MatchPattern>,
        tail: Option<MatchPatternListTail>,
    ) -> Self {
        Self {
            elements: elements.into_boxed_slice(),
            tail,
        }
    }

    pub(crate) fn elements(&self) -> &[MatchPattern] {
        &self.elements
    }

    pub(crate) fn tail(&self) -> Option<&MatchPatternListTail> {
        self.tail.as_ref()
    }
}
