use std::collections::{HashMap, HashSet};

use super::super::super::draft::pattern::{
    DraftBitArrayPatternSegment, DraftBitArrayPatternSizeExpr, DraftMatchPattern,
    DraftMatchPatternBinding,
};
use crate::plan::execution::graph::{MatchIntBindingId, MatchIntPatternBinding};

pub(super) struct IntBindings {
    required: HashSet<usize>,
    slots: HashMap<usize, MatchIntBindingId>,
}

impl IntBindings {
    pub(super) fn new(root: &DraftMatchPattern) -> Self {
        let mut required = HashSet::new();
        let mut patterns = vec![root];
        let mut sizes = Vec::new();
        while let Some(pattern) = patterns.pop() {
            match pattern {
                DraftMatchPattern::Tuple(elements) | DraftMatchPattern::List { elements, .. } => {
                    patterns.extend(elements)
                }
                DraftMatchPattern::Custom { fields, .. } => patterns.extend(fields),
                DraftMatchPattern::Alias { pattern, .. } => patterns.push(pattern),
                DraftMatchPattern::BitArray(pattern) => {
                    for segment in &pattern.segments {
                        match segment {
                            DraftBitArrayPatternSegment::Int { size, .. }
                            | DraftBitArrayPatternSegment::Float { size, .. } => {
                                sizes.push(&size.value);
                            }
                            DraftBitArrayPatternSegment::Bits { size, .. } => {
                                if let Some(size) = size {
                                    sizes.push(&size.value);
                                }
                            }
                            DraftBitArrayPatternSegment::String { .. }
                            | DraftBitArrayPatternSegment::UtfCodepoint { .. } => {}
                        }
                    }
                }
                DraftMatchPattern::Bind(_)
                | DraftMatchPattern::Discard
                | DraftMatchPattern::Int(_)
                | DraftMatchPattern::Float(_)
                | DraftMatchPattern::String(_)
                | DraftMatchPattern::Bool(_)
                | DraftMatchPattern::Nil
                | DraftMatchPattern::StringPrefix { .. } => {}
            }
        }
        while let Some(size) = sizes.pop() {
            match size {
                DraftBitArrayPatternSizeExpr::Binding(index) => {
                    required.insert(*index);
                }
                DraftBitArrayPatternSizeExpr::Add { left, right }
                | DraftBitArrayPatternSizeExpr::Subtract { left, right }
                | DraftBitArrayPatternSizeExpr::Multiply { left, right }
                | DraftBitArrayPatternSizeExpr::Divide { left, right }
                | DraftBitArrayPatternSizeExpr::Remainder { left, right } => {
                    sizes.push(right);
                    sizes.push(left);
                }
                DraftBitArrayPatternSizeExpr::Value(_) | DraftBitArrayPatternSizeExpr::Local(_) => {
                }
            }
        }
        Self {
            required,
            slots: HashMap::new(),
        }
    }

    pub(super) fn bind(&mut self, binding: DraftMatchPatternBinding) -> MatchIntPatternBinding {
        let size = if self.required.contains(&binding.index) {
            let slot = MatchIntBindingId::new(self.slots.len());
            self.slots.insert(binding.index, slot);
            Some(slot)
        } else {
            None
        };
        MatchIntPatternBinding {
            binding: super::freeze_binding(binding),
            size,
        }
    }

    pub(super) fn get(&self, index: usize) -> MatchIntBindingId {
        self.slots[&index]
    }
}
