use crate::plan::{
    BoolExpr, BoolExprKind, BoolFunctionExpr, BoolFunctionExprKind, BoolFunctionReturn, BoolReturn,
    Expr, ExprKind, FunctionExpr, FunctionExprKind, FunctionFunctionExpr, FunctionFunctionExprKind,
    FunctionFunctionId, FunctionFunctionReturn, IntExpr, IntExprKind, IntFunctionExpr,
    IntFunctionExprKind, IntFunctionReturn, IntReturn, NilExpr, NilExprKind, NilFunctionExpr,
    NilFunctionExprKind, NilFunctionReturn, NilReturn, ReturnBody, ReturnExpr, RuntimeFunctionId,
    StringExpr, StringExprKind, StringFunctionExpr, StringFunctionExprKind, StringFunctionReturn,
    StringReturn, ValueType,
};
use crate::planner::error::{InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError};
use ecow::EcoString;

pub(super) fn function_return_expr(
    name: &EcoString,
    expected: &ValueType,
    runtime_id: &RuntimeFunctionId,
    actual: Expr,
) -> Result<ReturnExpr, PlanError> {
    match (expected, runtime_id, actual.into_kind()) {
        (ValueType::Int, RuntimeFunctionId::Int(runtime_id), ExprKind::Int(actual)) => {
            Ok(ReturnExpr::int_body(*runtime_id, int_return(actual)))
        }
        (ValueType::String, RuntimeFunctionId::String(runtime_id), ExprKind::String(actual)) => {
            Ok(ReturnExpr::string_body(*runtime_id, string_return(actual)))
        }
        (ValueType::Bool, RuntimeFunctionId::Bool(runtime_id), ExprKind::Bool(actual)) => {
            Ok(ReturnExpr::bool_body(*runtime_id, bool_return(actual)))
        }
        (ValueType::Nil, RuntimeFunctionId::Nil(runtime_id), ExprKind::Nil(actual)) => {
            Ok(ReturnExpr::nil_body(*runtime_id, nil_return(actual)))
        }
        (
            ValueType::Function(expected),
            RuntimeFunctionId::Function { id, return_type },
            ExprKind::Function(actual),
        ) if expected.as_ref() == actual.type_() && expected.as_ref() == return_type => {
            function_returning_function_expr(name, *id, actual)
        }
        _ => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: name.clone(),
                reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
            },
        }),
    }
}

fn function_returning_function_expr(
    name: &EcoString,
    runtime_id: FunctionFunctionId,
    actual: FunctionExpr,
) -> Result<ReturnExpr, PlanError> {
    match (runtime_id, actual.into_kind()) {
        (FunctionFunctionId::Int(runtime_id), FunctionExprKind::Int(actual)) => {
            let type_ = actual.type_().clone();
            Ok(ReturnExpr::int_function_body(
                runtime_id,
                type_,
                int_function_return(actual),
            ))
        }
        (FunctionFunctionId::String(runtime_id), FunctionExprKind::String(actual)) => {
            let type_ = actual.type_().clone();
            Ok(ReturnExpr::string_function_body(
                runtime_id,
                type_,
                string_function_return(actual),
            ))
        }
        (FunctionFunctionId::Bool(runtime_id), FunctionExprKind::Bool(actual)) => {
            let type_ = actual.type_().clone();
            Ok(ReturnExpr::bool_function_body(
                runtime_id,
                type_,
                bool_function_return(actual),
            ))
        }
        (FunctionFunctionId::Nil(runtime_id), FunctionExprKind::Nil(actual)) => {
            let type_ = actual.type_().clone();
            Ok(ReturnExpr::nil_function_body(
                runtime_id,
                type_,
                nil_function_return(actual),
            ))
        }
        (FunctionFunctionId::Function(runtime_id), FunctionExprKind::Function(actual)) => {
            let type_ = actual.type_().clone();
            Ok(ReturnExpr::function_function_body(
                runtime_id,
                type_,
                function_function_return(actual),
            ))
        }
        _ => Err(PlanError::InvalidTypedAst {
            reason: InvalidTypedAstReason::FunctionShape {
                name: name.clone(),
                reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
            },
        }),
    }
}

fn int_return(expression: IntExpr) -> IntReturn {
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
        IntExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), int_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

fn string_return(expression: StringExpr) -> StringReturn {
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
        StringExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), string_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

fn bool_return(expression: BoolExpr) -> BoolReturn {
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
        BoolExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), bool_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

fn nil_return(expression: NilExpr) -> NilReturn {
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
        NilExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), nil_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

fn int_function_return(expression: IntFunctionExpr) -> IntFunctionReturn {
    match expression.kind() {
        IntFunctionExprKind::Call { function, args, .. } => {
            ReturnBody::tail_call(*function, args.clone())
        }
        IntFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            int_function_return((**true_).clone()),
            int_function_return((**false_).clone()),
        ),
        IntFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), int_function_return(branch.clone())))
                .collect(),
            int_function_return((**fallback).clone()),
        ),
        IntFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), int_function_return(branch.clone())))
                .collect(),
            int_function_return((**fallback).clone()),
        ),
        IntFunctionExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), int_function_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

fn string_function_return(expression: StringFunctionExpr) -> StringFunctionReturn {
    match expression.kind() {
        StringFunctionExprKind::Call { function, args, .. } => {
            ReturnBody::tail_call(*function, args.clone())
        }
        StringFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            string_function_return((**true_).clone()),
            string_function_return((**false_).clone()),
        ),
        StringFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), string_function_return(branch.clone())))
                .collect(),
            string_function_return((**fallback).clone()),
        ),
        StringFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), string_function_return(branch.clone())))
                .collect(),
            string_function_return((**fallback).clone()),
        ),
        StringFunctionExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), string_function_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

fn bool_function_return(expression: BoolFunctionExpr) -> BoolFunctionReturn {
    match expression.kind() {
        BoolFunctionExprKind::Call { function, args, .. } => {
            ReturnBody::tail_call(*function, args.clone())
        }
        BoolFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            bool_function_return((**true_).clone()),
            bool_function_return((**false_).clone()),
        ),
        BoolFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), bool_function_return(branch.clone())))
                .collect(),
            bool_function_return((**fallback).clone()),
        ),
        BoolFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), bool_function_return(branch.clone())))
                .collect(),
            bool_function_return((**fallback).clone()),
        ),
        BoolFunctionExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), bool_function_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

fn nil_function_return(expression: NilFunctionExpr) -> NilFunctionReturn {
    match expression.kind() {
        NilFunctionExprKind::Call { function, args, .. } => {
            ReturnBody::tail_call(*function, args.clone())
        }
        NilFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            nil_function_return((**true_).clone()),
            nil_function_return((**false_).clone()),
        ),
        NilFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), nil_function_return(branch.clone())))
                .collect(),
            nil_function_return((**fallback).clone()),
        ),
        NilFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), nil_function_return(branch.clone())))
                .collect(),
            nil_function_return((**fallback).clone()),
        ),
        NilFunctionExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), nil_function_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

fn function_function_return(expression: FunctionFunctionExpr) -> FunctionFunctionReturn {
    match expression.kind() {
        FunctionFunctionExprKind::Call { function, args, .. } => {
            ReturnBody::tail_call(*function, args.clone())
        }
        FunctionFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            function_function_return((**true_).clone()),
            function_function_return((**false_).clone()),
        ),
        FunctionFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), function_function_return(branch.clone())))
                .collect(),
            function_function_return((**fallback).clone()),
        ),
        FunctionFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), function_function_return(branch.clone())))
                .collect(),
            function_function_return((**fallback).clone()),
        ),
        FunctionFunctionExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), function_function_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

#[cfg(test)]
mod tests {
    use super::{function_return_expr, function_returning_function_expr};
    use crate::plan::{
        BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionValue, Expr,
        FunctionExpr, FunctionFunctionExpr, FunctionFunctionFunctionId, FunctionFunctionId,
        FunctionFunctionValue, FunctionType, IntFunctionExpr, IntFunctionFunctionId, IntFunctionId,
        IntFunctionValue, IntLocalId, NilFunctionExpr, NilFunctionFunctionId, NilFunctionId,
        NilFunctionValue, ParamLocal, ReturnExpr, RuntimeFunctionId, StringFunctionExpr,
        StringFunctionFunctionId, StringFunctionId, StringFunctionValue, ValueType,
    };
    use crate::planner::{InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError};

    #[test]
    fn reject_margin_function_return_family_mismatch() {
        assert_eq!(
            function_return_expr(
                &"main".into(),
                &ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
                &RuntimeFunctionId::Function {
                    id: FunctionFunctionId::String(StringFunctionFunctionId(0)),
                    return_type: FunctionType::new(Vec::new(), ValueType::Int),
                },
                Expr::function(FunctionExpr::int(IntFunctionExpr::value(
                    IntFunctionValue::new(IntFunctionId(0), Vec::new()),
                ))),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn reject_margin_function_return_type_metadata_mismatch() {
        let expected = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            function_return_expr(
                &"main".into(),
                &ValueType::Function(Box::new(expected.clone())),
                &RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: expected,
                },
                Expr::function(FunctionExpr::int(IntFunctionExpr::value(
                    IntFunctionValue::new(IntFunctionId(0), vec![ParamLocal::int(IntLocalId(0))]),
                ))),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );

        let expected = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        assert_eq!(
            function_return_expr(
                &"main".into(),
                &ValueType::Function(Box::new(expected)),
                &RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: FunctionType::new(vec![ValueType::Int], ValueType::Int),
                },
                Expr::function(FunctionExpr::int(IntFunctionExpr::value(
                    IntFunctionValue::new(IntFunctionId(0), Vec::new()),
                ))),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );
    }

    #[test]
    fn function_returning_function_expr_preserves_return_families() {
        let function_return_type = FunctionType::new(Vec::new(), ValueType::Int);

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::String(StringFunctionFunctionId(0)),
                FunctionExpr::string(StringFunctionExpr::value(StringFunctionValue::new(
                    StringFunctionId(0),
                    Vec::new(),
                ))),
            ),
            Ok(ReturnExpr::string_function(
                StringFunctionFunctionId(0),
                StringFunctionExpr::value(StringFunctionValue::new(
                    StringFunctionId(0),
                    Vec::new(),
                )),
            )),
        );

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::Bool(BoolFunctionFunctionId(0)),
                FunctionExpr::bool(BoolFunctionExpr::value(BoolFunctionValue::new(
                    BoolFunctionId(0),
                    Vec::new(),
                ))),
            ),
            Ok(ReturnExpr::bool_function(
                BoolFunctionFunctionId(0),
                BoolFunctionExpr::value(BoolFunctionValue::new(BoolFunctionId(0), Vec::new())),
            )),
        );

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::Nil(NilFunctionFunctionId(0)),
                FunctionExpr::nil(NilFunctionExpr::value(NilFunctionValue::new(
                    NilFunctionId(0),
                    Vec::new(),
                ))),
            ),
            Ok(ReturnExpr::nil_function(
                NilFunctionFunctionId(0),
                NilFunctionExpr::value(NilFunctionValue::new(NilFunctionId(0), Vec::new())),
            )),
        );

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::Function(FunctionFunctionFunctionId(0)),
                FunctionExpr::function(FunctionFunctionExpr::value(FunctionFunctionValue::new(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                    function_return_type.clone(),
                ))),
            ),
            Ok(ReturnExpr::function_function(
                FunctionFunctionFunctionId(0),
                FunctionFunctionExpr::value(FunctionFunctionValue::new(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::new(),
                    function_return_type,
                )),
            )),
        );
    }
}
