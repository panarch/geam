use super::super::super::graph::MatchPatternBinding;

pub(in super::super) fn write_binding(output: &mut String, binding: &MatchPatternBinding) {
    output.push_str("binding#");
    output.push_str(&binding.index().to_string());
}

#[cfg(test)]
mod tests {
    use crate::plan::execution::graph::MatchPatternBinding;

    #[test]
    fn writes_match_binding_index() {
        super::super::super::assert_written("binding#3", |output| {
            super::write_binding(output, &MatchPatternBinding::new(3));
        });
    }
}
