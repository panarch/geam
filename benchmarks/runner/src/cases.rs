use clap::ValueEnum;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Case {
    pub(super) workload: Workload,
    pub(super) size: u64,
}

#[derive(Clone, Copy, Debug, Deserialize, Eq, Ord, PartialEq, PartialOrd, Serialize, ValueEnum)]
#[serde(rename_all = "snake_case")]
#[value(rename_all = "snake_case")]
pub(super) enum Workload {
    CallbackControl,
    CountOnes,
    CountOnesAssert,
    CountOnesEqual,
    OddNumsBetween,
    SlicePrefix,
    SliceSuffix,
    Arithmetic,
    CapturingFold,
    CaptureChain,
    CustomMatch,
    StringFields,
    StringPrefixes,
    StringGraphemes,
    StringGraphemesUnicode,
    BitChecksum,
    ParseSum,
}

impl Workload {
    pub(super) fn name(self) -> &'static str {
        match self {
            Self::CallbackControl => "callback_control",
            Self::CountOnes => "count_ones",
            Self::CountOnesAssert => "count_ones_assert",
            Self::CountOnesEqual => "count_ones_equal",
            Self::OddNumsBetween => "odd_nums_between",
            Self::SlicePrefix => "slice_prefix",
            Self::SliceSuffix => "slice_suffix",
            Self::Arithmetic => "arithmetic",
            Self::CapturingFold => "capturing_fold",
            Self::CaptureChain => "capture_chain",
            Self::CustomMatch => "custom_match",
            Self::StringFields => "string_fields",
            Self::StringPrefixes => "string_prefixes",
            Self::StringGraphemes => "string_graphemes",
            Self::StringGraphemesUnicode => "string_graphemes_unicode",
            Self::BitChecksum => "bit_checksum",
            Self::ParseSum => "parse_sum",
        }
    }
}

impl Case {
    pub(super) fn expected_checksum(self) -> i64 {
        let n = self.size as i64;
        match self.workload {
            Workload::CallbackControl => n,
            Workload::CountOnes | Workload::CountOnesAssert | Workload::CountOnesEqual => n / 3 + 1,
            Workload::OddNumsBetween => (n / 2).pow(2),
            Workload::SlicePrefix => n * (n - 1) / 2,
            Workload::SliceSuffix => (10_000 - n) * n + n * (n - 1) / 2,
            Workload::Arithmetic => (1..=n)
                .map(|x| if x % 2 == 0 { x * 3 + 1 } else { x * 2 - 1 })
                .sum(),
            Workload::CapturingFold => n * (n - 1) / 2 * 3 + n * 7,
            Workload::CaptureChain => n,
            Workload::CustomMatch => (0..n)
                .map(|x| match x % 3 {
                    0 => x,
                    1 => -x,
                    _ => 0,
                })
                .sum(),
            Workload::StringFields => n * 17,
            Workload::StringPrefixes | Workload::StringGraphemes => n,
            Workload::StringGraphemesUnicode => n * 2,
            Workload::BitChecksum => n * 30,
            Workload::ParseSum => n * 328,
        }
    }
}

pub(super) fn suite(filter: Option<Workload>) -> Vec<Case> {
    let mut cases = Vec::new();
    for workload in [
        Workload::CallbackControl,
        Workload::CountOnes,
        Workload::CountOnesAssert,
        Workload::CountOnesEqual,
        Workload::OddNumsBetween,
        Workload::SlicePrefix,
        Workload::SliceSuffix,
        Workload::Arithmetic,
        Workload::CapturingFold,
        Workload::CaptureChain,
        Workload::CustomMatch,
        Workload::StringFields,
        Workload::StringPrefixes,
        Workload::StringGraphemes,
        Workload::StringGraphemesUnicode,
        Workload::BitChecksum,
        Workload::ParseSum,
    ] {
        if filter.is_some_and(|selected| selected != workload) {
            continue;
        }
        let sizes: &[u64] = match workload {
            Workload::CallbackControl => &[1],
            Workload::CaptureChain => &[1, 9, 101, 201, 401, 801],
            Workload::StringPrefixes
            | Workload::StringGraphemes
            | Workload::StringGraphemesUnicode => &[0, 8, 1_024, 4_096, 16_384],
            Workload::CountOnes
            | Workload::CountOnesAssert
            | Workload::CountOnesEqual
            | Workload::OddNumsBetween
            | Workload::SlicePrefix
            | Workload::SliceSuffix => &[100, 1_000, 10_000],
            Workload::BitChecksum | Workload::ParseSum => &[100, 1_000, 4_000],
            _ => &[100, 1_000],
        };
        cases.extend(sizes.iter().map(|&size| Case { workload, size }));
    }
    cases
}

pub(super) fn select(names: &[String]) -> crate::Result<Vec<Case>> {
    let all = suite(None);
    if names.is_empty() {
        return Ok(all);
    }
    let mut selected = Vec::new();
    for name in names {
        let case = all
            .iter()
            .find(|case| format!("{}/{}", case.workload.name(), case.size) == *name)
            .copied()
            .ok_or_else(|| crate::invalid(format!("unknown maintained case: {name}")))?;
        if selected.contains(&case) {
            return Err(crate::invalid(format!("duplicate case: {name}")).into());
        }
        selected.push(case);
    }
    Ok(selected)
}

#[cfg(test)]
mod tests {
    use super::{Case, Workload, suite};

    #[test]
    fn independent_oracles_cover_every_workload_family() {
        for (workload, size, expected) in [
            (Workload::CallbackControl, 1, 1),
            (Workload::CountOnes, 6, 3),
            (Workload::CountOnesAssert, 6, 3),
            (Workload::CountOnesEqual, 6, 3),
            (Workload::OddNumsBetween, 6, 9),
            (Workload::SlicePrefix, 3, 3),
            (Workload::SliceSuffix, 3, 29_994),
            (Workload::Arithmetic, 4, 26),
            (Workload::CapturingFold, 3, 30),
            (Workload::CaptureChain, 1, 1),
            (Workload::CaptureChain, 9, 9),
            (Workload::CaptureChain, 801, 801),
            (Workload::CustomMatch, 6, -2),
            (Workload::StringFields, 1, 17),
            (Workload::StringPrefixes, 0, 0),
            (Workload::StringPrefixes, 8, 8),
            (Workload::StringGraphemes, 0, 0),
            (Workload::StringGraphemes, 8, 8),
            (Workload::StringGraphemesUnicode, 0, 0),
            (Workload::StringGraphemesUnicode, 8, 16),
            (Workload::BitChecksum, 1, 30),
            (Workload::ParseSum, 1, 328),
            (Workload::BitChecksum, 4000, 120000),
            (Workload::ParseSum, 4000, 1312000),
        ] {
            assert_eq!(Case { workload, size }.expected_checksum(), expected);
            assert_eq!(serde_json::to_value(workload).unwrap(), workload.name());
        }
        assert_eq!(suite(None).len(), 54);
        assert_eq!(
            suite(Some(Workload::CaptureChain)),
            [1, 9, 101, 201, 401, 801].map(|size| Case {
                workload: Workload::CaptureChain,
                size,
            })
        );
        for workload in [
            Workload::CountOnes,
            Workload::CountOnesAssert,
            Workload::CountOnesEqual,
        ] {
            assert_eq!(
                suite(Some(workload)),
                [100, 1_000, 10_000].map(|size| Case { workload, size })
            );
        }
    }
    #[test]
    fn exact_case_selection_preserves_order_and_rejects_unknown_or_duplicate_inputs() {
        assert_eq!(super::select(&[]).unwrap(), suite(None));
        assert_eq!(
            super::select(&["parse_sum/4000".into(), "callback_control/1".into()]).unwrap(),
            [
                Case {
                    workload: Workload::ParseSum,
                    size: 4000
                },
                Case {
                    workload: Workload::CallbackControl,
                    size: 1
                }
            ]
        );
        assert_eq!(
            super::select(&["parse_sum/999".into()])
                .unwrap_err()
                .to_string(),
            "unknown maintained case: parse_sum/999"
        );
        assert_eq!(
            super::select(&["parse_sum/4000".into(), "parse_sum/4000".into()])
                .unwrap_err()
                .to_string(),
            "duplicate case: parse_sum/4000"
        );
        let case = Case {
            workload: Workload::BitChecksum,
            size: 4000,
        };
        assert_eq!(
            serde_json::to_string(&case).unwrap(),
            r#"{"workload":"bit_checksum","size":4000}"#
        );
        assert_eq!(
            serde_json::from_str::<Case>(r#"{"workload":"bit_checksum","size":4000}"#).unwrap(),
            case
        );
    }
}
