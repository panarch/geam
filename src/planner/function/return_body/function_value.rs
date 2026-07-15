use crate::plan::{
    BitArrayFunctionExpr, BitArrayFunctionExprKind, BitArrayFunctionReturn, BoolFunctionExpr,
    BoolFunctionExprKind, BoolFunctionReturn, CustomFunctionExpr, CustomFunctionReturn,
    FloatFunctionExpr, FloatFunctionExprKind, FloatFunctionReturn, FunctionExpr, FunctionExprKind,
    FunctionFunctionExpr, FunctionFunctionId, FunctionFunctionReturn, IntFunctionExpr,
    IntFunctionExprKind, IntFunctionReturn, ListFunctionExpr, ListFunctionExprKind,
    ListFunctionReturn, NilFunctionExpr, NilFunctionExprKind, NilFunctionReturn, ReturnBody,
    ReturnExpr, StringFunctionExpr, StringFunctionExprKind, StringFunctionReturn,
    TupleFunctionExpr, TupleFunctionExprKind, TupleFunctionReturn, UtfCodepointFunctionExpr,
    UtfCodepointFunctionExprKind, UtfCodepointFunctionReturn,
};
use crate::planner::error::{InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError};
use ecow::EcoString;

pub(super) fn function_returning_function_expr(
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
        (FunctionFunctionId::BitArray(runtime_id), FunctionExprKind::BitArray(actual)) => {
            let type_ = actual.type_().clone();
            Ok(ReturnExpr::bit_array_function_body(
                runtime_id,
                type_,
                bit_array_function_return(actual),
            ))
        }
        (FunctionFunctionId::UtfCodepoint(runtime_id), FunctionExprKind::UtfCodepoint(actual)) => {
            let type_ = actual.type_().clone();
            Ok(ReturnExpr::utf_codepoint_function_body(
                runtime_id,
                type_,
                utf_codepoint_function_return(actual),
            ))
        }
        (FunctionFunctionId::Custom(runtime_id), FunctionExprKind::Custom(actual)) => Ok(
            ReturnExpr::custom_function_body(runtime_id.index(), custom_function_return(actual)),
        ),
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
        (FunctionFunctionId::List(runtime_id), FunctionExprKind::List(actual)) => Ok(
            ReturnExpr::list_function_body(runtime_id, list_function_return(actual)),
        ),
        (FunctionFunctionId::Function(runtime_id), FunctionExprKind::Function(actual)) => {
            Ok(ReturnExpr::function_function_body(
                runtime_id.index(),
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

fn custom_function_return(expression: CustomFunctionExpr) -> CustomFunctionReturn {
    CustomFunctionReturn::expr(expression)
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

fn bit_array_function_return(expression: BitArrayFunctionExpr) -> BitArrayFunctionReturn {
    match expression.kind() {
        BitArrayFunctionExprKind::Call { function, args, .. } => {
            ReturnBody::tail_call(*function, args.clone())
        }
        BitArrayFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            bit_array_function_return((**true_).clone()),
            bit_array_function_return((**false_).clone()),
        ),
        BitArrayFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), bit_array_function_return(branch.clone())))
                .collect(),
            bit_array_function_return((**fallback).clone()),
        ),
        BitArrayFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), bit_array_function_return(branch.clone())))
                .collect(),
            bit_array_function_return((**fallback).clone()),
        ),
        BitArrayFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, bit_array_function_return(branch.clone())))
                .collect(),
            bit_array_function_return((**fallback).clone()),
        ),
        BitArrayFunctionExprKind::Block { steps, return_ } => ReturnBody::block(
            steps.clone(),
            bit_array_function_return((**return_).clone()),
        ),
        _ => ReturnBody::expr(expression),
    }
}

fn utf_codepoint_function_return(
    expression: UtfCodepointFunctionExpr,
) -> UtfCodepointFunctionReturn {
    match expression.kind() {
        UtfCodepointFunctionExprKind::Call { function, args, .. } => {
            ReturnBody::tail_call(*function, args.clone())
        }
        UtfCodepointFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            utf_codepoint_function_return((**true_).clone()),
            utf_codepoint_function_return((**false_).clone()),
        ),
        UtfCodepointFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| {
                    (value.clone(), utf_codepoint_function_return(branch.clone()))
                })
                .collect(),
            utf_codepoint_function_return((**fallback).clone()),
        ),
        UtfCodepointFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| {
                    (value.clone(), utf_codepoint_function_return(branch.clone()))
                })
                .collect(),
            utf_codepoint_function_return((**fallback).clone()),
        ),
        UtfCodepointFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, utf_codepoint_function_return(branch.clone())))
                .collect(),
            utf_codepoint_function_return((**fallback).clone()),
        ),
        UtfCodepointFunctionExprKind::Block { steps, return_ } => ReturnBody::block(
            steps.clone(),
            utf_codepoint_function_return((**return_).clone()),
        ),
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

fn list_function_return(expression: ListFunctionExpr) -> ListFunctionReturn {
    match expression.kind() {
        ListFunctionExprKind::Call { function, args, .. } => {
            ReturnBody::tail_call(function.clone(), args.clone())
        }
        ListFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            list_function_return((**true_).clone()),
            list_function_return((**false_).clone()),
        ),
        ListFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), list_function_return(branch.clone())))
                .collect(),
            list_function_return((**fallback).clone()),
        ),
        ListFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), list_function_return(branch.clone())))
                .collect(),
            list_function_return((**fallback).clone()),
        ),
        ListFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, list_function_return(branch.clone())))
                .collect(),
            list_function_return((**fallback).clone()),
        ),
        ListFunctionExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), list_function_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

fn function_function_return(expression: FunctionFunctionExpr) -> FunctionFunctionReturn {
    FunctionFunctionReturn::expr(expression)
}

#[cfg(test)]
mod tests {
    use super::{
        bit_array_function_return, custom_function_return, float_function_return,
        function_returning_function_expr, list_function_return,
    };
    use crate::plan::{
        BitArrayFunctionExpr, BitArrayFunctionFunctionId, BitArrayFunctionId,
        BitArrayFunctionReference, BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId,
        BoolFunctionId, BoolFunctionReference, CustomFunctionExpr, CustomFunctionFunctionId,
        CustomFunctionId, CustomFunctionReference, CustomFunctionType, CustomType, CustomTypeName,
        FloatExpr, FloatFunctionExpr, FloatFunctionFunctionId, FloatFunctionId,
        FloatFunctionReference, FunctionExpr, FunctionFunctionExpr, FunctionFunctionFunctionId,
        FunctionFunctionId, FunctionFunctionReference, FunctionFunctionReturn,
        FunctionFunctionType, FunctionType, IntExpr, IntFunctionExpr, IntFunctionFunctionId,
        IntFunctionId, IntFunctionReference, IntLocalId, ListFunctionExpr, ListFunctionFunctionId,
        ListFunctionId, ListFunctionReference, NilFunctionExpr, NilFunctionFunctionId,
        NilFunctionId, NilFunctionReference, ParamLocal, ReturnBody, ReturnExpr, StringExpr,
        StringFunctionExpr, StringFunctionFunctionId, StringFunctionId, StringFunctionReference,
        TupleFunctionExpr, TupleFunctionFunctionId, TupleFunctionId, TupleFunctionReference,
        ValueType,
    };
    use crate::planner::{InvalidFunctionShapeReason, InvalidTypedAstReason, PlanError};
    use num_bigint::BigInt;

    #[test]
    fn function_returning_function_expr_rejects_family_mismatch() {
        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
                FunctionExpr::int(IntFunctionExpr::reference(IntFunctionReference::new(
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
    fn function_returning_function_expr_preserves_return_families() {
        let function_return_type = FunctionType::new(Vec::new(), ValueType::Int);

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                FunctionExpr::int(IntFunctionExpr::reference(IntFunctionReference::new(
                    IntFunctionId(0),
                    Vec::new(),
                ))),
            ),
            Ok(ReturnExpr::int_function(
                IntFunctionFunctionId(0),
                IntFunctionExpr::reference(IntFunctionReference::new(IntFunctionId(0), Vec::new())),
            )),
        );

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::String(StringFunctionFunctionId(0)),
                FunctionExpr::string(StringFunctionExpr::reference(StringFunctionReference::new(
                    StringFunctionId(0),
                    Vec::new(),
                ))),
            ),
            Ok(ReturnExpr::string_function(
                StringFunctionFunctionId(0),
                StringFunctionExpr::reference(StringFunctionReference::new(
                    StringFunctionId(0),
                    Vec::new(),
                )),
            )),
        );

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::BitArray(BitArrayFunctionFunctionId(0)),
                FunctionExpr::bit_array(BitArrayFunctionExpr::reference(
                    BitArrayFunctionReference::new(BitArrayFunctionId(0), Vec::new()),
                )),
            ),
            Ok(ReturnExpr::bit_array_function(
                BitArrayFunctionFunctionId(0),
                BitArrayFunctionExpr::reference(BitArrayFunctionReference::new(
                    BitArrayFunctionId(0),
                    Vec::new(),
                )),
            )),
        );

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
                FunctionExpr::float(FloatFunctionExpr::reference(FloatFunctionReference::new(
                    FloatFunctionId(0),
                    Vec::new(),
                ))),
            ),
            Ok(ReturnExpr::float_function(
                FloatFunctionFunctionId(0),
                FloatFunctionExpr::reference(FloatFunctionReference::new(
                    FloatFunctionId(0),
                    Vec::new(),
                )),
            )),
        );

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::Bool(BoolFunctionFunctionId(0)),
                FunctionExpr::bool(BoolFunctionExpr::reference(BoolFunctionReference::new(
                    BoolFunctionId(0),
                    Vec::new(),
                ))),
            ),
            Ok(ReturnExpr::bool_function(
                BoolFunctionFunctionId(0),
                BoolFunctionExpr::reference(BoolFunctionReference::new(
                    BoolFunctionId(0),
                    Vec::new()
                )),
            )),
        );

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::Nil(NilFunctionFunctionId(0)),
                FunctionExpr::nil(NilFunctionExpr::reference(NilFunctionReference::new(
                    NilFunctionId(0),
                    Vec::new(),
                ))),
            ),
            Ok(ReturnExpr::nil_function(
                NilFunctionFunctionId(0),
                NilFunctionExpr::reference(NilFunctionReference::new(NilFunctionId(0), Vec::new())),
            )),
        );

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::Tuple(TupleFunctionFunctionId(0)),
                FunctionExpr::tuple(TupleFunctionExpr::reference(
                    TupleFunctionReference::new(TupleFunctionId(0), Vec::new()),
                    vec![ValueType::Float],
                )),
            ),
            Ok(ReturnExpr::tuple_function(
                TupleFunctionFunctionId(0),
                TupleFunctionExpr::reference(
                    TupleFunctionReference::new(TupleFunctionId(0), Vec::new()),
                    vec![ValueType::Float],
                ),
            )),
        );

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::List(ListFunctionFunctionId::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                    ),
                    crate::plan::ValueType::Int
                )),
                FunctionExpr::list(ListFunctionExpr::reference(ListFunctionReference::new(
                    ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                    Vec::new()
                ))),
            ),
            Ok(ReturnExpr::list_function(
                ListFunctionFunctionId::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                    ),
                    crate::plan::ValueType::Int
                ),
                ListFunctionExpr::reference(ListFunctionReference::new(
                    ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
                    Vec::new()
                )),
            )),
        );

        assert_eq!(
            function_returning_function_expr(
                &"main".into(),
                FunctionFunctionId::Function(FunctionFunctionFunctionId::new(
                    0,
                    FunctionFunctionType::new(Vec::new(), function_return_type.clone()),
                )),
                FunctionExpr::function(FunctionFunctionExpr::reference(
                    FunctionFunctionReference::new(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::new(),
                    ),
                    function_return_type.clone(),
                )),
            ),
            Ok(ReturnExpr::function_function(
                0,
                FunctionFunctionExpr::reference(
                    FunctionFunctionReference::new(
                        FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                        Vec::new(),
                    ),
                    function_return_type,
                ),
            )),
        );
    }

    #[test]
    fn function_value_returns_preserve_float_case_return_body_shape() {
        assert_eq!(
            function_returning_function_expr(
                &"int_function".into(),
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                FunctionExpr::int(int_function_float_case()),
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
            function_returning_function_expr(
                &"string_function".into(),
                FunctionFunctionId::String(StringFunctionFunctionId(0)),
                FunctionExpr::string(string_function_float_case()),
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
            function_returning_function_expr(
                &"float_function".into(),
                FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
                FunctionExpr::float(float_function_float_case()),
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
            function_returning_function_expr(
                &"bool_function".into(),
                FunctionFunctionId::Bool(BoolFunctionFunctionId(0)),
                FunctionExpr::bool(bool_function_float_case()),
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
            function_returning_function_expr(
                &"nil_function".into(),
                FunctionFunctionId::Nil(NilFunctionFunctionId(0)),
                FunctionExpr::nil(nil_function_float_case()),
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
        assert_eq!(
            function_returning_function_expr(
                &"tuple_function".into(),
                FunctionFunctionId::Tuple(TupleFunctionFunctionId(0)),
                FunctionExpr::tuple(tuple_function_float_case()),
            ),
            Ok(ReturnExpr::tuple_function_body(
                TupleFunctionFunctionId(0),
                tuple_function_type(),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(tuple_function_value()))],
                    ReturnBody::expr(tuple_function_value()),
                ),
            )),
        );
        assert_eq!(
            function_returning_function_expr(
                &"list_function".into(),
                FunctionFunctionId::List(ListFunctionFunctionId::from_item_type(
                    0,
                    list_function_type(),
                    crate::plan::ValueType::Int,
                )),
                FunctionExpr::list(list_function_float_case()),
            ),
            Ok(ReturnExpr::list_function_body(
                ListFunctionFunctionId::from_item_type(
                    0,
                    list_function_type(),
                    crate::plan::ValueType::Int
                ),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(list_function_value()))],
                    ReturnBody::expr(list_function_value()),
                ),
            )),
        );
        assert_eq!(
            function_returning_function_expr(
                &"function_function".into(),
                FunctionFunctionId::Function(FunctionFunctionFunctionId::new(
                    0,
                    FunctionFunctionType::new(Vec::new(), float_function_type()),
                )),
                FunctionExpr::function(function_function_float_case()),
            ),
            Ok(ReturnExpr::function_function_body(
                0,
                FunctionFunctionReturn::expr(function_function_float_case()),
            )),
        );
    }

    #[test]
    fn custom_function_return_preserves_tail_case_and_block_shapes() {
        let value = custom_function_value();
        assert_eq!(
            custom_function_return(CustomFunctionExpr::call(
                CustomFunctionFunctionId::new(0, custom_function_type()),
                Vec::new(),
            ))
            .into_parts(),
            (custom_function_type(), ReturnBody::tail_call(0, Vec::new()),),
        );
        assert_eq!(
            custom_function_return(CustomFunctionExpr::bool_case(
                BoolExpr::value(true),
                value.clone(),
                value.clone(),
            ))
            .into_parts(),
            (
                custom_function_type(),
                ReturnBody::bool_case(
                    BoolExpr::value(true),
                    ReturnBody::expr(value.clone().into_parts().1),
                    ReturnBody::expr(value.clone().into_parts().1),
                ),
            ),
        );
        assert_eq!(
            custom_function_return(CustomFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(1.into(), value.clone())],
                value.clone(),
            ))
            .into_parts(),
            (
                custom_function_type(),
                ReturnBody::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), ReturnBody::expr(value.clone().into_parts().1))],
                    ReturnBody::expr(value.clone().into_parts().1),
                ),
            ),
        );
        assert_eq!(
            custom_function_return(CustomFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), value.clone())],
                value.clone(),
            ))
            .into_parts(),
            (
                custom_function_type(),
                ReturnBody::string_case(
                    StringExpr::value("one".into()),
                    vec![("one".into(), ReturnBody::expr(value.clone().into_parts().1),)],
                    ReturnBody::expr(value.clone().into_parts().1),
                ),
            ),
        );
        assert_eq!(
            custom_function_return(CustomFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, value.clone())],
                value.clone(),
            ))
            .into_parts(),
            (
                custom_function_type(),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(value.clone().into_parts().1))],
                    ReturnBody::expr(value.clone().into_parts().1),
                ),
            ),
        );
        assert_eq!(
            custom_function_return(CustomFunctionExpr::block(
                vec![crate::plan::Step::evaluate(crate::plan::Expr::int(
                    IntExpr::value(1.into()),
                ))],
                value.clone(),
            ))
            .into_parts(),
            (
                custom_function_type(),
                ReturnBody::block(
                    vec![crate::plan::Step::evaluate(crate::plan::Expr::int(
                        IntExpr::value(1.into()),
                    ))],
                    ReturnBody::expr(value.into_parts().1),
                ),
            ),
        );
    }

    #[test]
    fn float_function_return_preserves_tail_and_case_return_body_shapes() {
        assert_eq!(
            float_function_return(FloatFunctionExpr::call(
                FloatFunctionFunctionId(0),
                Vec::new(),
                float_function_type(),
            )),
            ReturnBody::tail_call(FloatFunctionFunctionId(0), Vec::new()),
        );
        assert_eq!(
            float_function_return(FloatFunctionExpr::bool_case(
                BoolExpr::value(true),
                float_function_value(),
                float_function_value(),
            )),
            ReturnBody::bool_case(
                BoolExpr::value(true),
                ReturnBody::expr(float_function_value()),
                ReturnBody::expr(float_function_value()),
            ),
        );
        assert_eq!(
            float_function_return(FloatFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), float_function_value())],
                float_function_value(),
            )),
            ReturnBody::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), ReturnBody::expr(float_function_value()))],
                ReturnBody::expr(float_function_value()),
            ),
        );
        assert_eq!(
            float_function_return(FloatFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), float_function_value())],
                float_function_value(),
            )),
            ReturnBody::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), ReturnBody::expr(float_function_value()))],
                ReturnBody::expr(float_function_value()),
            ),
        );
        assert_eq!(
            float_function_return(FloatFunctionExpr::block(
                vec![crate::plan::Step::evaluate(crate::plan::Expr::float(
                    FloatExpr::value(1.0),
                ))],
                float_function_value(),
            )),
            ReturnBody::block(
                vec![crate::plan::Step::evaluate(crate::plan::Expr::float(
                    FloatExpr::value(1.0),
                ))],
                ReturnBody::expr(float_function_value()),
            ),
        );
    }

    #[test]
    fn bit_array_function_return_preserves_tail_and_block_return_body_shapes() {
        let type_ = FunctionType::new(Vec::new(), ValueType::BitArray);
        let value = BitArrayFunctionExpr::reference(BitArrayFunctionReference::new(
            BitArrayFunctionId(0),
            Vec::new(),
        ));
        assert_eq!(
            bit_array_function_return(BitArrayFunctionExpr::call(
                BitArrayFunctionFunctionId(0),
                Vec::new(),
                type_.clone(),
            )),
            ReturnBody::tail_call(BitArrayFunctionFunctionId(0), Vec::new()),
        );
        let step = crate::plan::Step::evaluate(crate::plan::Expr::int(IntExpr::value(1.into())));
        assert_eq!(
            bit_array_function_return(BitArrayFunctionExpr::block(
                vec![step.clone()],
                value.clone(),
            )),
            ReturnBody::block(vec![step], ReturnBody::expr(value.clone())),
        );
        assert_eq!(
            bit_array_function_return(BitArrayFunctionExpr::bool_case(
                BoolExpr::value(true),
                value.clone(),
                value.clone(),
            )),
            ReturnBody::bool_case(
                BoolExpr::value(true),
                ReturnBody::expr(value.clone()),
                ReturnBody::expr(value.clone()),
            ),
        );
        assert_eq!(
            bit_array_function_return(BitArrayFunctionExpr::int_case(
                IntExpr::value(1.into()),
                vec![(BigInt::from(1), value.clone())],
                value.clone(),
            )),
            ReturnBody::int_case(
                IntExpr::value(1.into()),
                vec![(BigInt::from(1), ReturnBody::expr(value.clone()))],
                ReturnBody::expr(value.clone()),
            ),
        );
        assert_eq!(
            bit_array_function_return(BitArrayFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), value.clone())],
                value.clone(),
            )),
            ReturnBody::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), ReturnBody::expr(value.clone()))],
                ReturnBody::expr(value.clone()),
            ),
        );
        assert_eq!(
            bit_array_function_return(BitArrayFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, value.clone())],
                value.clone(),
            )),
            ReturnBody::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, ReturnBody::expr(value.clone()))],
                ReturnBody::expr(value),
            ),
        );
    }

    #[test]
    fn list_function_return_preserves_tail_and_case_return_body_shapes() {
        assert_eq!(
            list_function_return(ListFunctionExpr::call(
                ListFunctionFunctionId::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                    ),
                    crate::plan::ValueType::Int
                ),
                Vec::new()
            )),
            ReturnBody::tail_call(
                ListFunctionFunctionId::from_item_type(
                    0,
                    crate::plan::FunctionType::new(
                        Vec::new(),
                        crate::plan::ValueType::List(Box::new(crate::plan::ValueType::Int))
                    ),
                    crate::plan::ValueType::Int
                ),
                Vec::new()
            ),
        );
        assert_eq!(
            list_function_return(ListFunctionExpr::bool_case(
                BoolExpr::value(true),
                list_function_value(),
                list_function_value(),
            )),
            ReturnBody::bool_case(
                BoolExpr::value(true),
                ReturnBody::expr(list_function_value()),
                ReturnBody::expr(list_function_value()),
            ),
        );
        assert_eq!(
            list_function_return(ListFunctionExpr::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), list_function_value())],
                list_function_value(),
            )),
            ReturnBody::int_case(
                IntExpr::value(BigInt::from(1)),
                vec![(BigInt::from(1), ReturnBody::expr(list_function_value()))],
                ReturnBody::expr(list_function_value()),
            ),
        );
        assert_eq!(
            list_function_return(ListFunctionExpr::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), list_function_value())],
                list_function_value(),
            )),
            ReturnBody::string_case(
                StringExpr::value("one".into()),
                vec![("one".into(), ReturnBody::expr(list_function_value()))],
                ReturnBody::expr(list_function_value()),
            ),
        );
        assert_eq!(
            list_function_return(ListFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, list_function_value())],
                list_function_value(),
            )),
            ReturnBody::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, ReturnBody::expr(list_function_value()))],
                ReturnBody::expr(list_function_value()),
            ),
        );
        assert_eq!(
            list_function_return(ListFunctionExpr::block(
                vec![crate::plan::Step::evaluate(crate::plan::Expr::float(
                    FloatExpr::value(1.0),
                ))],
                list_function_value(),
            )),
            ReturnBody::block(
                vec![crate::plan::Step::evaluate(crate::plan::Expr::float(
                    FloatExpr::value(1.0),
                ))],
                ReturnBody::expr(list_function_value()),
            ),
        );
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

    fn tuple_function_float_case() -> TupleFunctionExpr {
        TupleFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, tuple_function_value())],
            tuple_function_value(),
        )
    }

    fn list_function_float_case() -> ListFunctionExpr {
        ListFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, list_function_value())],
            list_function_value(),
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

    fn tuple_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Float],
            ValueType::Tuple(vec![ValueType::Float]),
        )
    }

    fn list_function_type() -> FunctionType {
        FunctionType::new(
            vec![ValueType::Float],
            ValueType::List(Box::new(ValueType::Int)),
        )
    }

    fn int_function_value() -> IntFunctionExpr {
        IntFunctionExpr::reference(IntFunctionReference::new(
            IntFunctionId(0),
            vec![ParamLocal::int(IntLocalId(0))],
        ))
    }

    fn string_function_value() -> StringFunctionExpr {
        StringFunctionExpr::reference(StringFunctionReference::new(
            StringFunctionId(0),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        ))
    }

    fn float_function_value() -> FloatFunctionExpr {
        FloatFunctionExpr::reference(FloatFunctionReference::new(
            FloatFunctionId(0),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        ))
    }

    fn bool_function_value() -> BoolFunctionExpr {
        BoolFunctionExpr::reference(BoolFunctionReference::new(
            BoolFunctionId(0),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        ))
    }

    fn nil_function_value() -> NilFunctionExpr {
        NilFunctionExpr::reference(NilFunctionReference::new(
            NilFunctionId(0),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        ))
    }

    fn tuple_function_value() -> TupleFunctionExpr {
        TupleFunctionExpr::reference(
            TupleFunctionReference::new(
                TupleFunctionId(0),
                vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
            ),
            vec![ValueType::Float],
        )
    }

    fn list_function_value() -> ListFunctionExpr {
        ListFunctionExpr::reference(ListFunctionReference::new(
            ListFunctionId::from_item_type(0, crate::plan::ValueType::Int),
            vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
        ))
    }

    fn function_function_value() -> FunctionFunctionExpr {
        FunctionFunctionExpr::reference(
            FunctionFunctionReference::new(
                FunctionFunctionId::Float(FloatFunctionFunctionId(0)),
                Vec::new(),
            ),
            float_function_type(),
        )
    }

    fn custom_function_type() -> CustomFunctionType {
        CustomFunctionType::new(vec![ValueType::Float], custom_type())
    }

    fn custom_function_value() -> CustomFunctionExpr {
        CustomFunctionExpr::reference(
            CustomFunctionReference::new(
                CustomFunctionId(0),
                vec![ParamLocal::float(crate::plan::FloatLocalId(0))],
            ),
            custom_type(),
        )
    }

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }
}
