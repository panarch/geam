use ecow::EcoString;
use geam::BitArrayValue;
use num_bigint::BigInt;

#[geam::provider(
    package = "example_value_types",
    modules = [scalars, tuples, lists, customs, results],
)]
pub struct Component;

#[geam::module(path = "example_value_types/scalars")]
mod scalars {
    use super::{BigInt, BitArrayValue, EcoString};

    #[geam::function]
    fn join(left: EcoString, right: EcoString) -> EcoString {
        format!("{left}:{right}").into()
    }

    #[geam::function]
    fn add(left: BigInt, right: BigInt) -> BigInt {
        left + right
    }

    #[geam::function]
    fn multiply(left: f64, right: f64) -> f64 {
        left * right
    }

    #[geam::function]
    fn keep_bits(value: BitArrayValue) -> BitArrayValue {
        value
    }

    #[geam::function]
    fn keep_codepoint(value: char) -> char {
        value
    }

    #[geam::function]
    fn invert(value: bool) -> bool {
        !value
    }

    #[geam::function]
    fn keep_nil(value: ()) -> () {
        value
    }
}

#[geam::module(path = "example_value_types/tuples")]
mod tuples {
    use super::{BigInt, EcoString};

    #[geam::function]
    fn wrap(value: EcoString) -> (EcoString,) {
        (value,)
    }

    #[geam::function]
    fn unwrap(value: (EcoString,)) -> EcoString {
        value.0
    }

    #[geam::function]
    fn swap(value: (EcoString, BigInt)) -> (BigInt, EcoString) {
        let (label, count) = value;
        (count, label)
    }

    #[geam::function]
    fn rotate(value: (EcoString, f64, bool)) -> (bool, EcoString, f64) {
        let (label, measurement, enabled) = value;
        (enabled, label, measurement)
    }

    #[geam::function]
    fn reassociate(value: (EcoString, (BigInt, bool))) -> ((EcoString, BigInt), bool) {
        let (label, (count, enabled)) = value;
        ((label, count), enabled)
    }
}

#[geam::module(path = "example_value_types/lists")]
mod lists {
    use super::{BigInt, EcoString};

    #[geam::function]
    fn length(values: geam::List<BigInt>) -> BigInt {
        values.len().into()
    }

    #[geam::function]
    fn first_or(values: geam::List<EcoString>, fallback: EcoString) -> EcoString {
        values.get(0).unwrap_or(fallback)
    }

    #[geam::function]
    fn identity(values: geam::List<BigInt>) -> geam::List<BigInt> {
        values
    }

    #[geam::function]
    fn reverse(values: geam::List<EcoString>) -> Vec<EcoString> {
        (0..values.len())
            .rev()
            .filter_map(|index| values.get(index))
            .collect()
    }

    #[geam::function]
    fn labels(values: geam::List<(EcoString, BigInt)>) -> Vec<EcoString> {
        (0..values.len())
            .filter_map(|index| values.get(index).map(|(label, _)| label))
            .collect()
    }
}

#[geam::module(path = "example_value_types/customs")]
mod customs {
    use super::{BigInt, EcoString};

    #[geam::custom(input = PriorityInput)]
    enum Priority {
        Low,
        Normal,
        High,
    }

    #[geam::custom(input = JobInput)]
    enum Job {
        Pending,
        Named(EcoString),
        Scheduled { label: EcoString, attempt: BigInt },
        Prioritized(Priority),
        Tags(Vec<EcoString>),
    }

    #[geam::function]
    fn low() -> Priority {
        Priority::Low
    }

    #[geam::function]
    fn normal() -> Priority {
        Priority::Normal
    }

    #[geam::function]
    fn high() -> Priority {
        Priority::High
    }

    #[geam::function]
    fn pending() -> Job {
        Job::Pending
    }

    #[geam::function]
    fn named(label: EcoString) -> Job {
        Job::Named(label)
    }

    #[geam::function]
    fn scheduled(label: EcoString, attempt: BigInt) -> Job {
        Job::Scheduled { label, attempt }
    }

    #[geam::function]
    fn prioritized() -> Job {
        Job::Prioritized(Priority::High)
    }

    #[geam::function]
    fn tagged(first: EcoString, second: EcoString) -> Job {
        Job::Tags(vec![first, second])
    }

    #[geam::function]
    fn describe(job: JobInput) -> EcoString {
        match job {
            JobInput::Pending => "pending".into(),
            JobInput::Named(label) => format!("named:{label}").into(),
            JobInput::Scheduled { label, attempt } => format!("scheduled:{label}:{attempt}").into(),
            JobInput::Prioritized(PriorityInput::Low) => "priority:low".into(),
            JobInput::Prioritized(PriorityInput::Normal) => "priority:normal".into(),
            JobInput::Prioritized(PriorityInput::High) => "priority:high".into(),
            JobInput::Tags(tags) => {
                let first = tags.get(0).unwrap_or_else(|| "empty".into());
                format!("tags:{}:{first}", tags.len()).into()
            }
        }
    }

    #[geam::function]
    fn first_priority(values: geam::List<PriorityInput>) -> EcoString {
        match values.get(0) {
            Some(PriorityInput::Low) => "low".into(),
            Some(PriorityInput::Normal) => "normal".into(),
            Some(PriorityInput::High) => "high".into(),
            None => "missing".into(),
        }
    }
}

#[geam::module(path = "example_value_types/results")]
mod results {
    use super::{BigInt, EcoString};

    #[geam::custom(input = ParseErrorInput)]
    enum ParseError {
        Empty,
        Invalid(EcoString),
    }

    #[geam::function]
    fn parse(value: EcoString) -> Result<BigInt, ParseError> {
        if value.is_empty() {
            Err(ParseError::Empty)
        } else {
            value
                .parse::<i64>()
                .map(BigInt::from)
                .map_err(|_| ParseError::Invalid(value))
        }
    }

    #[geam::function]
    fn describe(value: Result<BigInt, ParseErrorInput>) -> EcoString {
        match value {
            Ok(value) => format!("ok:{value}").into(),
            Err(ParseErrorInput::Empty) => "error:empty".into(),
            Err(ParseErrorInput::Invalid(value)) => format!("error:{value}").into(),
        }
    }

    #[geam::function]
    fn optional(value: BigInt, keep: bool) -> Option<(EcoString, BigInt)> {
        keep.then(|| ("kept".into(), value))
    }

    #[geam::function]
    fn describe_option(value: Option<(EcoString, BigInt)>) -> EcoString {
        value.map_or_else(
            || "none".into(),
            |(label, value)| format!("some:{label}:{value}").into(),
        )
    }

    #[geam::function]
    fn first(values: geam::List<Result<BigInt, ParseErrorInput>>) -> EcoString {
        values.get(0).map_or_else(|| "missing".into(), describe)
    }

    #[geam::function]
    fn samples() -> Vec<Result<BigInt, ParseError>> {
        vec![Ok(3.into()), Err(ParseError::Empty)]
    }
}
