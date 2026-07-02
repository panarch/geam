use crate::plan::{
    BoolExpr, BoolExprKind, BoolFunctionExpr, BoolFunctionExprKind, BoolFunctionReturn, BoolReturn,
    Expr, ExprKind, FloatExpr, FloatExprKind, FloatFunctionExpr, FloatFunctionExprKind,
    FloatFunctionReturn, FloatReturn, FunctionExpr, FunctionExprKind, FunctionFunctionExpr,
    FunctionFunctionExprKind, FunctionFunctionId, FunctionFunctionReturn, IntExpr, IntExprKind,
    IntFunctionExpr, IntFunctionExprKind, IntFunctionReturn, IntReturn, NilExpr, NilExprKind,
    NilFunctionExpr, NilFunctionExprKind, NilFunctionReturn, NilReturn, ReturnBody, ReturnExpr,
    RuntimeFunctionId, StringExpr, StringExprKind, StringFunctionExpr, StringFunctionExprKind,
    StringFunctionReturn, StringReturn, TupleExpr, TupleExprKind, TupleFunctionExpr,
    TupleFunctionExprKind, TupleFunctionReturn, TupleReturn, ValueType,
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
        (ValueType::Float, RuntimeFunctionId::Float(runtime_id), ExprKind::Float(actual)) => {
            Ok(ReturnExpr::float_body(*runtime_id, float_return(actual)))
        }
        (ValueType::Bool, RuntimeFunctionId::Bool(runtime_id), ExprKind::Bool(actual)) => {
            Ok(ReturnExpr::bool_body(*runtime_id, bool_return(actual)))
        }
        (ValueType::Nil, RuntimeFunctionId::Nil(runtime_id), ExprKind::Nil(actual)) => {
            Ok(ReturnExpr::nil_body(*runtime_id, nil_return(actual)))
        }
        (
            ValueType::Tuple(expected),
            RuntimeFunctionId::Tuple { id, return_type },
            ExprKind::Tuple(actual),
        ) if expected == actual.type_() && expected == return_type => Ok(ReturnExpr::tuple_body(
            *id,
            expected.clone(),
            tuple_return(actual),
        )),
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
        (FunctionFunctionId::Float(runtime_id), FunctionExprKind::Float(actual)) => {
            let type_ = actual.type_().clone();
            Ok(ReturnExpr::float_function_body(
                runtime_id,
                type_,
                float_function_return(actual),
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
        (FunctionFunctionId::Tuple(runtime_id), FunctionExprKind::Tuple(actual)) => {
            let type_ = actual.type_().clone();
            Ok(ReturnExpr::tuple_function_body(
                runtime_id,
                type_,
                tuple_function_return(actual),
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

fn float_return(expression: FloatExpr) -> FloatReturn {
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

fn tuple_return(expression: TupleExpr) -> TupleReturn {
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
        IntFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, int_function_return(branch.clone())))
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
        StringFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, string_function_return(branch.clone())))
                .collect(),
            string_function_return((**fallback).clone()),
        ),
        StringFunctionExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), string_function_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

fn float_function_return(expression: FloatFunctionExpr) -> FloatFunctionReturn {
    match expression.kind() {
        FloatFunctionExprKind::Call { function, args, .. } => {
            ReturnBody::tail_call(*function, args.clone())
        }
        FloatFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            float_function_return((**true_).clone()),
            float_function_return((**false_).clone()),
        ),
        FloatFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), float_function_return(branch.clone())))
                .collect(),
            float_function_return((**fallback).clone()),
        ),
        FloatFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), float_function_return(branch.clone())))
                .collect(),
            float_function_return((**fallback).clone()),
        ),
        FloatFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, float_function_return(branch.clone())))
                .collect(),
            float_function_return((**fallback).clone()),
        ),
        FloatFunctionExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), float_function_return((**return_).clone()))
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
        BoolFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, bool_function_return(branch.clone())))
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
        NilFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, nil_function_return(branch.clone())))
                .collect(),
            nil_function_return((**fallback).clone()),
        ),
        NilFunctionExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), nil_function_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

fn tuple_function_return(expression: TupleFunctionExpr) -> TupleFunctionReturn {
    match expression.kind() {
        TupleFunctionExprKind::Call { function, args, .. } => {
            ReturnBody::tail_call(*function, args.clone())
        }
        TupleFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            tuple_function_return((**true_).clone()),
            tuple_function_return((**false_).clone()),
        ),
        TupleFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), tuple_function_return(branch.clone())))
                .collect(),
            tuple_function_return((**fallback).clone()),
        ),
        TupleFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), tuple_function_return(branch.clone())))
                .collect(),
            tuple_function_return((**fallback).clone()),
        ),
        TupleFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, tuple_function_return(branch.clone())))
                .collect(),
            tuple_function_return((**fallback).clone()),
        ),
        TupleFunctionExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), tuple_function_return((**return_).clone()))
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
        FunctionFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, function_function_return(branch.clone())))
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
        BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionValue,
        Expr, FloatExpr, FloatFunctionExpr, FloatFunctionFunctionId, FloatFunctionId,
        FloatFunctionValue, FunctionExpr, FunctionFunctionExpr, FunctionFunctionFunctionId,
        FunctionFunctionId, FunctionFunctionValue, FunctionType, IntExpr, IntFunctionExpr,
        IntFunctionFunctionId, IntFunctionId, IntFunctionValue, IntLocalId, NilExpr,
        NilFunctionExpr, NilFunctionFunctionId, NilFunctionId, NilFunctionValue, ParamLocal,
        ReturnBody, ReturnExpr, RuntimeFunctionId, StringExpr, StringFunctionExpr,
        StringFunctionFunctionId, StringFunctionId, StringFunctionValue, ValueType,
    };
    use crate::planner::{InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError};
    use num_bigint::BigInt;

    #[test]
    fn reject_margin_function_return_family_mismatch() {
        assert_eq!(
            function_return_expr(
                &"main".into(),
                &ValueType::Float,
                &RuntimeFunctionId::Int(IntFunctionId(0)),
                Expr::float(FloatExpr::value(1.0)),
            ),
            Err(PlanError::InvalidTypedAst {
                reason: InvalidTypedAstReason::FunctionShape {
                    name: "main".into(),
                    reason: InvalidFunctionShapeReason::ReturnTypeMismatch,
                },
            }),
        );

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

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
                FunctionExpr::int(IntFunctionExpr::value(IntFunctionValue::new(
                    IntFunctionId(0),
                    Vec::new(),
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

    #[test]
    fn function_return_expr_preserves_float_case_return_body_shape() {
        assert_eq!(
            function_return_expr(
                &"int_value".into(),
                &ValueType::Int,
                &RuntimeFunctionId::Int(IntFunctionId(0)),
                Expr::int(int_float_case()),
            ),
            Ok(ReturnExpr::int_body(
                IntFunctionId(0),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(IntExpr::value(BigInt::from(1))))],
                    ReturnBody::expr(IntExpr::value(BigInt::from(0))),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"string_value".into(),
                &ValueType::String,
                &RuntimeFunctionId::String(StringFunctionId(0)),
                Expr::string(string_float_case()),
            ),
            Ok(ReturnExpr::string_body(
                StringFunctionId(0),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(StringExpr::value("one".into())))],
                    ReturnBody::expr(StringExpr::value("zero".into())),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"float_value".into(),
                &ValueType::Float,
                &RuntimeFunctionId::Float(FloatFunctionId(0)),
                Expr::float(float_float_case()),
            ),
            Ok(ReturnExpr::float_body(
                FloatFunctionId(0),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(FloatExpr::value(1.0)))],
                    ReturnBody::expr(FloatExpr::value(0.0)),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"bool_value".into(),
                &ValueType::Bool,
                &RuntimeFunctionId::Bool(BoolFunctionId(0)),
                Expr::bool(bool_float_case()),
            ),
            Ok(ReturnExpr::bool_body(
                BoolFunctionId(0),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(BoolExpr::value(true)))],
                    ReturnBody::expr(BoolExpr::value(false)),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"nil_value".into(),
                &ValueType::Nil,
                &RuntimeFunctionId::Nil(NilFunctionId(0)),
                Expr::nil(nil_float_case()),
            ),
            Ok(ReturnExpr::nil_body(
                NilFunctionId(0),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(NilExpr::value()))],
                    ReturnBody::expr(NilExpr::value()),
                ),
            )),
        );
    }

    #[test]
    fn function_returning_function_expr_preserves_float_case_return_body_shape() {
        assert_eq!(
            function_return_expr(
                &"int_function".into(),
                &ValueType::Function(Box::new(int_function_type())),
                &RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    return_type: int_function_type(),
                },
                Expr::function(FunctionExpr::int(int_function_float_case())),
            ),
            Ok(ReturnExpr::int_function_body(
                IntFunctionFunctionId(0),
                int_function_type(),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(int_function_value()))],
                    ReturnBody::expr(int_function_value()),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"string_function".into(),
                &ValueType::Function(Box::new(string_function_type())),
                &RuntimeFunctionId::Function {
                    id: FunctionFunctionId::String(StringFunctionFunctionId(0)),
                    return_type: string_function_type(),
                },
                Expr::function(FunctionExpr::string(string_function_float_case())),
            ),
            Ok(ReturnExpr::string_function_body(
                StringFunctionFunctionId(0),
                string_function_type(),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(string_function_value()))],
                    ReturnBody::expr(string_function_value()),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"float_function".into(),
                &ValueType::Function(Box::new(float_function_type())),
                &RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
                    return_type: float_function_type(),
                },
                Expr::function(FunctionExpr::float(float_function_float_case())),
            ),
            Ok(ReturnExpr::float_function_body(
                FloatFunctionFunctionId(0),
                float_function_type(),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(float_function_value()))],
                    ReturnBody::expr(float_function_value()),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"bool_function".into(),
                &ValueType::Function(Box::new(bool_function_type())),
                &RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Bool(BoolFunctionFunctionId(0)),
                    return_type: bool_function_type(),
                },
                Expr::function(FunctionExpr::bool(bool_function_float_case())),
            ),
            Ok(ReturnExpr::bool_function_body(
                BoolFunctionFunctionId(0),
                bool_function_type(),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(bool_function_value()))],
                    ReturnBody::expr(bool_function_value()),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"nil_function".into(),
                &ValueType::Function(Box::new(nil_function_type())),
                &RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Nil(NilFunctionFunctionId(0)),
                    return_type: nil_function_type(),
                },
                Expr::function(FunctionExpr::nil(nil_function_float_case())),
            ),
            Ok(ReturnExpr::nil_function_body(
                NilFunctionFunctionId(0),
                nil_function_type(),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(nil_function_value()))],
                    ReturnBody::expr(nil_function_value()),
                ),
            )),
        );
        let value_type = ValueType::Function(Box::new(float_function_type()));
        assert_eq!(
            function_return_expr(
                &"function_function".into(),
                &ValueType::Function(Box::new(FunctionType::new(Vec::new(), value_type.clone(),))),
                &RuntimeFunctionId::Function {
                    id: FunctionFunctionId::Function(FunctionFunctionFunctionId(0)),
                    return_type: FunctionType::new(Vec::new(), value_type),
                },
                Expr::function(FunctionExpr::function(function_function_float_case())),
            ),
            Ok(ReturnExpr::function_function_body(
                FunctionFunctionFunctionId(0),
                FunctionType::new(
                    Vec::new(),
                    ValueType::Function(Box::new(float_function_type())),
                ),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(function_function_value()))],
                    ReturnBody::expr(function_function_value()),
                ),
            )),
        );
    }

    #[test]
    fn function_return_expr_preserves_float_return_body_subject_cases() {
        assert_eq!(
            function_return_expr(
                &"float_tail_call".into(),
                &ValueType::Float,
                &RuntimeFunctionId::Float(FloatFunctionId(0)),
                Expr::float(FloatExpr::call(FloatFunctionId(1), Vec::new())),
            ),
            Ok(ReturnExpr::float_body(
                FloatFunctionId(0),
                ReturnBody::tail_call(FloatFunctionId(1), Vec::new()),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"float_bool_case".into(),
                &ValueType::Float,
                &RuntimeFunctionId::Float(FloatFunctionId(0)),
                Expr::float(FloatExpr::bool_case(
                    BoolExpr::value(true),
                    FloatExpr::value(1.0),
                    FloatExpr::value(0.0),
                )),
            ),
            Ok(ReturnExpr::float_body(
                FloatFunctionId(0),
                ReturnBody::bool_case(
                    BoolExpr::value(true),
                    ReturnBody::expr(FloatExpr::value(1.0)),
                    ReturnBody::expr(FloatExpr::value(0.0)),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"float_int_case".into(),
                &ValueType::Float,
                &RuntimeFunctionId::Float(FloatFunctionId(0)),
                Expr::float(FloatExpr::int_case(
                    IntExpr::value(BigInt::from(1)),
                    vec![(BigInt::from(1), FloatExpr::value(1.0))],
                    FloatExpr::value(0.0),
                )),
            ),
            Ok(ReturnExpr::float_body(
                FloatFunctionId(0),
                ReturnBody::int_case(
                    IntExpr::value(BigInt::from(1)),
                    vec![(BigInt::from(1), ReturnBody::expr(FloatExpr::value(1.0)),)],
                    ReturnBody::expr(FloatExpr::value(0.0)),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"float_string_case".into(),
                &ValueType::Float,
                &RuntimeFunctionId::Float(FloatFunctionId(0)),
                Expr::float(FloatExpr::string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), FloatExpr::value(1.0))],
                    FloatExpr::value(0.0),
                )),
            ),
            Ok(ReturnExpr::float_body(
                FloatFunctionId(0),
                ReturnBody::string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), ReturnBody::expr(FloatExpr::value(1.0)))],
                    ReturnBody::expr(FloatExpr::value(0.0)),
                ),
            )),
        );
    }

    #[test]
    fn function_return_expr_preserves_string_case_return_body_shapes() {
        assert_eq!(
            function_return_expr(
                &"string_string_case".into(),
                &ValueType::String,
                &RuntimeFunctionId::String(StringFunctionId(0)),
                Expr::string(StringExpr::string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), StringExpr::value("hit".into()))],
                    StringExpr::value("fallback".into()),
                )),
            ),
            Ok(ReturnExpr::string_body(
                StringFunctionId(0),
                ReturnBody::string_case(
                    StringExpr::value("one".into()),
                    vec![(
                        "one".into(),
                        ReturnBody::expr(StringExpr::value("hit".into()))
                    )],
                    ReturnBody::expr(StringExpr::value("fallback".into())),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"bool_string_case".into(),
                &ValueType::Bool,
                &RuntimeFunctionId::Bool(BoolFunctionId(0)),
                Expr::bool(BoolExpr::string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), BoolExpr::value(true))],
                    BoolExpr::value(false),
                )),
            ),
            Ok(ReturnExpr::bool_body(
                BoolFunctionId(0),
                ReturnBody::string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), ReturnBody::expr(BoolExpr::value(true)))],
                    ReturnBody::expr(BoolExpr::value(false)),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"nil_string_case".into(),
                &ValueType::Nil,
                &RuntimeFunctionId::Nil(NilFunctionId(0)),
                Expr::nil(NilExpr::string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), NilExpr::value())],
                    NilExpr::value(),
                )),
            ),
            Ok(ReturnExpr::nil_body(
                NilFunctionId(0),
                ReturnBody::string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), ReturnBody::expr(NilExpr::value()))],
                    ReturnBody::expr(NilExpr::value()),
                ),
            )),
        );
    }

    #[test]
    fn function_return_expr_preserves_float_function_return_body_shapes() {
        let value_type = ValueType::Function(Box::new(float_function_type()));
        let runtime_id = RuntimeFunctionId::Function {
            id: FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
            return_type: float_function_type(),
        };

        assert_eq!(
            function_return_expr(
                &"float_function_tail_call".into(),
                &value_type,
                &runtime_id,
                Expr::function(FunctionExpr::float(FloatFunctionExpr::call(
                    FloatFunctionFunctionId(0),
                    Vec::new(),
                    float_function_type(),
                ))),
            ),
            Ok(ReturnExpr::float_function_body(
                FloatFunctionFunctionId(0),
                float_function_type(),
                ReturnBody::tail_call(FloatFunctionFunctionId(0), Vec::new()),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"float_function_bool_case".into(),
                &value_type,
                &runtime_id,
                Expr::function(FunctionExpr::float(FloatFunctionExpr::bool_case(
                    BoolExpr::value(true),
                    float_function_value(),
                    float_function_value(),
                ))),
            ),
            Ok(ReturnExpr::float_function_body(
                FloatFunctionFunctionId(0),
                float_function_type(),
                ReturnBody::bool_case(
                    BoolExpr::value(true),
                    ReturnBody::expr(float_function_value()),
                    ReturnBody::expr(float_function_value()),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"float_function_int_case".into(),
                &value_type,
                &runtime_id,
                Expr::function(FunctionExpr::float(FloatFunctionExpr::int_case(
                    IntExpr::value(BigInt::from(1)),
                    vec![(BigInt::from(1), float_function_value())],
                    float_function_value(),
                ))),
            ),
            Ok(ReturnExpr::float_function_body(
                FloatFunctionFunctionId(0),
                float_function_type(),
                ReturnBody::int_case(
                    IntExpr::value(BigInt::from(1)),
                    vec![(BigInt::from(1), ReturnBody::expr(float_function_value()),)],
                    ReturnBody::expr(float_function_value()),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"float_function_string_case".into(),
                &value_type,
                &runtime_id,
                Expr::function(FunctionExpr::float(FloatFunctionExpr::string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), float_function_value())],
                    float_function_value(),
                ))),
            ),
            Ok(ReturnExpr::float_function_body(
                FloatFunctionFunctionId(0),
                float_function_type(),
                ReturnBody::string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), ReturnBody::expr(float_function_value()))],
                    ReturnBody::expr(float_function_value()),
                ),
            )),
        );
        assert_eq!(
            function_return_expr(
                &"float_function_block".into(),
                &value_type,
                &runtime_id,
                Expr::function(FunctionExpr::float(FloatFunctionExpr::block(
                    vec![crate::plan::Step::evaluate(Expr::float(FloatExpr::value(
                        1.0,
                    )))],
                    float_function_value(),
                ))),
            ),
            Ok(ReturnExpr::float_function_body(
                FloatFunctionFunctionId(0),
                float_function_type(),
                ReturnBody::block(
                    vec![crate::plan::Step::evaluate(Expr::float(FloatExpr::value(
                        1.0,
                    )))],
                    ReturnBody::expr(float_function_value()),
                ),
            )),
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

    fn int_function_float_case() -> IntFunctionExpr {
        IntFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, int_function_value())],
            int_function_value(),
        )
    }

    fn string_function_float_case() -> StringFunctionExpr {
        StringFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, string_function_value())],
            string_function_value(),
        )
    }

    fn float_function_float_case() -> FloatFunctionExpr {
        FloatFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, float_function_value())],
            float_function_value(),
        )
    }

    fn bool_function_float_case() -> BoolFunctionExpr {
        BoolFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, bool_function_value())],
            bool_function_value(),
        )
    }

    fn nil_function_float_case() -> NilFunctionExpr {
        NilFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, nil_function_value())],
            nil_function_value(),
        )
    }

    fn function_function_float_case() -> FunctionFunctionExpr {
        FunctionFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, function_function_value())],
            function_function_value(),
        )
    }

    fn int_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Int], ValueType::Int)
    }

    fn string_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Float], ValueType::String)
    }

    fn float_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Float], ValueType::Float)
    }

    fn bool_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Float], ValueType::Bool)
    }

    fn nil_function_type() -> FunctionType {
        FunctionType::new(vec![ValueType::Float], ValueType::Nil)
    }

    fn int_function_value() -> IntFunctionExpr {
        IntFunctionExpr::value(IntFunctionValue::new(
            IntFunctionId(0),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }

    fn string_function_value() -> StringFunctionExpr {
        StringFunctionExpr::value(StringFunctionValue::new(
            StringFunctionId(0),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        ))
    }

    fn float_function_value() -> FloatFunctionExpr {
        FloatFunctionExpr::value(FloatFunctionValue::new(
            FloatFunctionId(0),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        ))
    }

    fn bool_function_value() -> BoolFunctionExpr {
        BoolFunctionExpr::value(BoolFunctionValue::new(
            BoolFunctionId(0),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        ))
    }

    fn nil_function_value() -> NilFunctionExpr {
        NilFunctionExpr::value(NilFunctionValue::new(
            NilFunctionId(0),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        ))
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::value(FunctionFunctionValue::new(
            FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
            Vec::new(),
            float_function_type(),
        ))
    }
}
