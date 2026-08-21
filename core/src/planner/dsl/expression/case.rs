pub(super) mod bool_;
pub(super) mod float;
pub(super) mod int;
pub(super) mod string;

pub(crate) use bool_::*;
pub(crate) use float::*;
pub(crate) use int::*;
pub(crate) use string::*;

#[cfg(test)]
mod tests {
    use super::{bool_case_int, float_case_int, int_case_int, string_case_int};
    use crate::plan::IntExprKind;
    use crate::planner::dsl::expression::{bool_, float, int, string};
    use ecow::EcoString;
    use num_bigint::BigInt;

    #[test]
    fn case_facade_reexports_subject_family_helpers() {
        assert_eq!(
            bool_case_int(bool_(true), int(1), int(0)).0.kind(),
            &IntExprKind::BoolCase {
                subject: Box::new(bool_(true).into()),
                true_: Box::new(int(1).into()),
                false_: Box::new(int(0).into()),
            },
        );
        assert_eq!(
            int_case_int(int(1), [(1, int(1))], int(0)).0.kind(),
            &IntExprKind::IntCase {
                subject: Box::new(int(1).into()),
                clauses: vec![(BigInt::from(1), int(1).into())],
                fallback: Box::new(int(0).into()),
            },
        );
        assert_eq!(
            float_case_int(float(1.0), [(1.0, int(1))], int(0)).0.kind(),
            &IntExprKind::FloatCase {
                subject: Box::new(float(1.0).into()),
                clauses: vec![(1.0, int(1).into())],
                fallback: Box::new(int(0).into()),
            },
        );
        assert_eq!(
            string_case_int(string("key"), [("one", int(1))], int(0))
                .0
                .kind(),
            &IntExprKind::StringCase {
                subject: Box::new(string("key").into()),
                clauses: vec![(EcoString::from("one"), int(1).into())],
                fallback: Box::new(int(0).into()),
            },
        );
    }
}
