use crate::plan::{
    BoolExpr, BoolExprKind, BoolReturn, FloatExpr, FloatExprKind, FloatReturn, IntExpr,
    IntExprKind, IntReturn, ListExpr, ListExprKind, ListReturn, NilExpr, NilExprKind, NilReturn,
    ReturnBody, StringExpr, StringExprKind, StringReturn, TupleExpr, TupleExprKind, TupleReturn,
};

pub(super) fn int_return(expression: IntExpr) -> IntReturn {
    match expression.kind() {
        IntExprKind::Call { function, args } => ReturnBody::tail_call(*function, args.clone()),
        IntExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            int_return((**true_).clone()),
            int_return((**false_).clone()),
        ),
        IntExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), int_return(branch.clone())))
                .collect(),
            int_return((**fallback).clone()),
        ),
        IntExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), int_return(branch.clone())))
                .collect(),
            int_return((**fallback).clone()),
        ),
        IntExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, int_return(branch.clone())))
                .collect(),
            int_return((**fallback).clone()),
        ),
        IntExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), int_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

pub(super) fn string_return(expression: StringExpr) -> StringReturn {
    match expression.kind() {
        StringExprKind::Call { function, args } => ReturnBody::tail_call(*function, args.clone()),
        StringExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            string_return((**true_).clone()),
            string_return((**false_).clone()),
        ),
        StringExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), string_return(branch.clone())))
                .collect(),
            string_return((**fallback).clone()),
        ),
        StringExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), string_return(branch.clone())))
                .collect(),
            string_return((**fallback).clone()),
        ),
        StringExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, string_return(branch.clone())))
                .collect(),
            string_return((**fallback).clone()),
        ),
        StringExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), string_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

pub(super) fn bool_return(expression: BoolExpr) -> BoolReturn {
    match expression.kind() {
        BoolExprKind::Call { function, args } => ReturnBody::tail_call(*function, args.clone()),
        BoolExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            bool_return((**true_).clone()),
            bool_return((**false_).clone()),
        ),
        BoolExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), bool_return(branch.clone())))
                .collect(),
            bool_return((**fallback).clone()),
        ),
        BoolExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), bool_return(branch.clone())))
                .collect(),
            bool_return((**fallback).clone()),
        ),
        BoolExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, bool_return(branch.clone())))
                .collect(),
            bool_return((**fallback).clone()),
        ),
        BoolExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), bool_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

pub(super) fn nil_return(expression: NilExpr) -> NilReturn {
    match expression.kind() {
        NilExprKind::Call { function, args } => ReturnBody::tail_call(*function, args.clone()),
        NilExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            nil_return((**true_).clone()),
            nil_return((**false_).clone()),
        ),
        NilExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), nil_return(branch.clone())))
                .collect(),
            nil_return((**fallback).clone()),
        ),
        NilExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), nil_return(branch.clone())))
                .collect(),
            nil_return((**fallback).clone()),
        ),
        NilExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, nil_return(branch.clone())))
                .collect(),
            nil_return((**fallback).clone()),
        ),
        NilExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), nil_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

pub(super) fn float_return(expression: FloatExpr) -> FloatReturn {
    match expression.kind() {
        FloatExprKind::Call { function, args } => ReturnBody::tail_call(*function, args.clone()),
        FloatExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            float_return((**true_).clone()),
            float_return((**false_).clone()),
        ),
        FloatExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), float_return(branch.clone())))
                .collect(),
            float_return((**fallback).clone()),
        ),
        FloatExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), float_return(branch.clone())))
                .collect(),
            float_return((**fallback).clone()),
        ),
        FloatExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, float_return(branch.clone())))
                .collect(),
            float_return((**fallback).clone()),
        ),
        FloatExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), float_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

pub(super) fn tuple_return(expression: TupleExpr) -> TupleReturn {
    match expression.kind() {
        TupleExprKind::Call { function, args } => ReturnBody::tail_call(*function, args.clone()),
        TupleExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            tuple_return((**true_).clone()),
            tuple_return((**false_).clone()),
        ),
        TupleExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), tuple_return(branch.clone())))
                .collect(),
            tuple_return((**fallback).clone()),
        ),
        TupleExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), tuple_return(branch.clone())))
                .collect(),
            tuple_return((**fallback).clone()),
        ),
        TupleExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, tuple_return(branch.clone())))
                .collect(),
            tuple_return((**fallback).clone()),
        ),
        TupleExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), tuple_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

pub(super) fn list_return(expression: ListExpr) -> ListReturn {
    match expression.kind() {
        ListExprKind::Call { function, args } => ReturnBody::tail_call(*function, args.clone()),
        ListExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            list_return((**true_).clone()),
            list_return((**false_).clone()),
        ),
        ListExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), list_return(branch.clone())))
                .collect(),
            list_return((**fallback).clone()),
        ),
        ListExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), list_return(branch.clone())))
                .collect(),
            list_return((**fallback).clone()),
        ),
        ListExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, list_return(branch.clone())))
                .collect(),
            list_return((**fallback).clone()),
        ),
        ListExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), list_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        bool_return, float_return, int_return, list_return, nil_return, string_return, tuple_return,
    };
    use crate::plan::{
        BoolExpr, Expr, FloatExpr, FloatFunctionId, IntExpr, ListExpr, NilExpr, ReturnBody,
        StringExpr, TupleExpr, ValueType,
    };
    use num_bigint::BigInt;

    #[test]
    fn primitive_returns_preserve_float_case_return_body_shape() {
        assert_eq!(
            int_return(int_float_case()),
            ReturnBody::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, ReturnBody::expr(IntExpr::value(BigInt::from(1))))],
                ReturnBody::expr(IntExpr::value(BigInt::from(0))),
            ),
        );
        assert_eq!(
            string_return(string_float_case()),
            ReturnBody::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, ReturnBody::expr(StringExpr::value("one".into())))],
                ReturnBody::expr(StringExpr::value("zero".into())),
            ),
        );
        assert_eq!(
            float_return(float_float_case()),
            ReturnBody::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, ReturnBody::expr(FloatExpr::value(1.0)))],
                ReturnBody::expr(FloatExpr::value(0.0)),
            ),
        );
        assert_eq!(
            bool_return(bool_float_case()),
            ReturnBody::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, ReturnBody::expr(BoolExpr::value(true)))],
                ReturnBody::expr(BoolExpr::value(false)),
            ),
        );
        assert_eq!(
            nil_return(nil_float_case()),
            ReturnBody::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, ReturnBody::expr(NilExpr::value()))],
                ReturnBody::expr(NilExpr::value()),
            ),
        );
        assert_eq!(
            tuple_return(tuple_float_case()),
            ReturnBody::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, ReturnBody::expr(tuple_value()))],
                ReturnBody::expr(tuple_value()),
            ),
        );
        assert_eq!(
            list_return(list_float_case()),
            ReturnBody::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, ReturnBody::expr(list_value()))],
                ReturnBody::expr(list_value()),
            ),
        );
    }

    #[test]
    fn float_return_preserves_tail_and_case_return_body_shapes() {
        assert_eq!(
            float_return(FloatExpr::call(FloatFunctionId(1), Vec::new())),
            ReturnBody::tail_call(FloatFunctionId(1), Vec::new()),
        );
        assert_eq!(
            float_return(FloatExpr::bool_case(
                BoolExpr::value(true),
                FloatExpr::value(1.0),
                FloatExpr::value(0.0),
            )),
            ReturnBody::bool_case(
                BoolExpr::value(true),
                ReturnBody::expr(FloatExpr::value(1.0)),
                ReturnBody::expr(FloatExpr::value(0.0)),
            ),
        );
        assert_eq!(
            float_return(FloatExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), FloatExpr::value(1.0))],
                FloatExpr::value(0.0),
            )),
            ReturnBody::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), ReturnBody::expr(FloatExpr::value(1.0)))],
                ReturnBody::expr(FloatExpr::value(0.0)),
            ),
        );
        assert_eq!(
            float_return(FloatExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), FloatExpr::value(1.0))],
                FloatExpr::value(0.0),
            )),
            ReturnBody::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), ReturnBody::expr(FloatExpr::value(1.0)))],
                ReturnBody::expr(FloatExpr::value(0.0)),
            ),
        );
    }

    #[test]
    fn primitive_returns_preserve_string_case_return_body_shapes() {
        assert_eq!(
            string_return(StringExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), StringExpr::value("hit".into()))],
                StringExpr::value("fallback".into()),
            )),
            ReturnBody::string_case(
                StringExpr::value("one".into()),
                vec![(
                    "one".into(),
                    ReturnBody::expr(StringExpr::value("hit".into()))
                )],
                ReturnBody::expr(StringExpr::value("fallback".into())),
            ),
        );
        assert_eq!(
            bool_return(BoolExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), BoolExpr::value(true))],
                BoolExpr::value(false),
            )),
            ReturnBody::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), ReturnBody::expr(BoolExpr::value(true)))],
                ReturnBody::expr(BoolExpr::value(false)),
            ),
        );
        assert_eq!(
            nil_return(NilExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), NilExpr::value())],
                NilExpr::value(),
            )),
            ReturnBody::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), ReturnBody::expr(NilExpr::value()))],
                ReturnBody::expr(NilExpr::value()),
            ),
        );
    }

    fn int_float_case() -> IntExpr {
        IntExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, IntExpr::value(BigInt::from(1)))],
            IntExpr::value(BigInt::from(0)),
        )
    }

    fn string_float_case() -> StringExpr {
        StringExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, StringExpr::value("one".into()))],
            StringExpr::value("zero".into()),
        )
    }

    fn float_float_case() -> FloatExpr {
        FloatExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, FloatExpr::value(1.0))],
            FloatExpr::value(0.0),
        )
    }

    fn bool_float_case() -> BoolExpr {
        BoolExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, BoolExpr::value(true))],
            BoolExpr::value(false),
        )
    }

    fn nil_float_case() -> NilExpr {
        NilExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, NilExpr::value())],
            NilExpr::value(),
        )
    }

    fn tuple_float_case() -> TupleExpr {
        TupleExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, tuple_value())],
            tuple_value(),
        )
    }

    fn list_float_case() -> ListExpr {
        ListExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, list_value())],
            list_value(),
        )
    }

    fn tuple_value() -> TupleExpr {
        TupleExpr::value(
            vec![Expr::float(FloatExpr::value(1.0))],
            vec![ValueType::Float],
        )
    }

    fn list_value() -> ListExpr {
        ListExpr::value(vec![Expr::float(FloatExpr::value(1.0))], ValueType::Float)
    }
}
