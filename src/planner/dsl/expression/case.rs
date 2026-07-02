pub(super) mod bool_;
pub(super) mod int;
pub(super) mod string;

pub(crate) use bool_::*;
pub(crate) use int::*;
pub(crate) use string::*;

#[cfg(test)]
mod tests {
    use super::{bool_case_int, int_case_int, string_case_int};
    use crate::plan::IntExprKind;
    use crate::planner::dsl::expression::{bool_, int, string};

    #[test]
    fn case_facade_reexports_subject_family_helpers() {
        assert!(matches!(
            bool_case_int(bool_(true), int(1), int(0)).0.kind(),
            IntExprKind::BoolCase { .. },
        ));
        assert!(matches!(
            int_case_int(int(1), [(1, int(1))], int(0)).0.kind(),
            IntExprKind::IntCase { .. },
        ));
        assert!(matches!(
            string_case_int(string("key"), [("one", int(1))], int(0))
                .0
                .kind(),
            IntExprKind::StringCase { .. },
        ));
    }
}
