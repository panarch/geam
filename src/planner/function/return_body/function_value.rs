use crate::plan::{
    BitArrayFunctionExpr, BitArrayFunctionExprKind, BitArrayFunctionReturn, BoolFunctionExpr,
    BoolFunctionExprKind, BoolFunctionReturn, CustomFunctionExpr, CustomFunctionReturn,
    ExternalFunctionExpr, ExternalFunctionReturn, FloatFunctionExpr, FloatFunctionExprKind,
    FloatFunctionReturn, FunctionExpr, FunctionExprKind, FunctionFunctionExpr,
    FunctionFunctionReturn, GenericFunctionExpr, GenericFunctionExprKind, GenericFunctionReturn,
    IntFunctionExpr, IntFunctionExprKind, IntFunctionReturn, ListFunctionExpr,
    ListFunctionExprKind, ListFunctionReturn, NilFunctionExpr, NilFunctionExprKind,
    NilFunctionReturn, ReturnBody, ReturnExpr, StringFunctionExpr, StringFunctionExprKind,
    StringFunctionReturn, TupleFunctionExpr, TupleFunctionExprKind, TupleFunctionReturn,
    UtfCodepointFunctionExpr, UtfCodepointFunctionExprKind, UtfCodepointFunctionReturn,
};
pub(super) fn function_returning_function_expr(actual: FunctionExpr) -> ReturnExpr {
    let (shape, kind) = actual.into_parts();
    match kind {
        FunctionExprKind::Generic(actual) => {
            ReturnExpr::generic_function_shape_body(shape, generic_function_return(actual))
        }
        FunctionExprKind::Int(actual) => {
            ReturnExpr::int_function_shape_body(shape, int_function_return(actual))
        }
        FunctionExprKind::String(actual) => {
            ReturnExpr::string_function_shape_body(shape, string_function_return(actual))
        }
        FunctionExprKind::BitArray(actual) => {
            ReturnExpr::bit_array_function_shape_body(shape, bit_array_function_return(actual))
        }
        FunctionExprKind::UtfCodepoint(actual) => ReturnExpr::utf_codepoint_function_shape_body(
            shape,
            utf_codepoint_function_return(actual),
        ),
        FunctionExprKind::Custom(actual) => {
            ReturnExpr::custom_function_shape_body(shape, custom_function_return(actual))
        }
        FunctionExprKind::External(actual) => {
            ReturnExpr::external_function_shape_body(shape, external_function_return(actual))
        }
        FunctionExprKind::Float(actual) => {
            ReturnExpr::float_function_shape_body(shape, float_function_return(actual))
        }
        FunctionExprKind::Bool(actual) => {
            ReturnExpr::bool_function_shape_body(shape, bool_function_return(actual))
        }
        FunctionExprKind::Nil(actual) => {
            ReturnExpr::nil_function_shape_body(shape, nil_function_return(actual))
        }
        FunctionExprKind::Tuple(actual) => {
            ReturnExpr::tuple_function_shape_body(shape, tuple_function_return(actual))
        }
        FunctionExprKind::List(actual) => {
            let item_type = actual.return_item_type();
            ReturnExpr::list_function_shape_body(shape, item_type, list_function_return(actual))
        }
        FunctionExprKind::Function(actual) => {
            ReturnExpr::function_function_shape_body(shape, function_function_return(actual))
        }
    }
}

fn generic_function_return(expression: GenericFunctionExpr) -> GenericFunctionReturn {
    match expression.kind() {
        GenericFunctionExprKind::Call {
            function,
            args,
            site,
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
        GenericFunctionExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            generic_function_return((**true_).clone()),
            generic_function_return((**false_).clone()),
        ),
        GenericFunctionExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), generic_function_return(branch.clone())))
                .collect(),
            generic_function_return((**fallback).clone()),
        ),
        GenericFunctionExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), generic_function_return(branch.clone())))
                .collect(),
            generic_function_return((**fallback).clone()),
        ),
        GenericFunctionExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, generic_function_return(branch.clone())))
                .collect(),
            generic_function_return((**fallback).clone()),
        ),
        GenericFunctionExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), generic_function_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

fn custom_function_return(expression: CustomFunctionExpr) -> CustomFunctionReturn {
    CustomFunctionReturn::expr(expression)
}

fn external_function_return(expression: ExternalFunctionExpr) -> ExternalFunctionReturn {
    ExternalFunctionReturn::expr(expression)
}

fn int_function_return(expression: IntFunctionExpr) -> IntFunctionReturn {
    match expression.kind() {
        IntFunctionExprKind::Call {
            function,
            args,
            site,
            ..
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        StringFunctionExprKind::Call {
            function,
            args,
            site,
            ..
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        BitArrayFunctionExprKind::Call {
            function,
            args,
            site,
            ..
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        UtfCodepointFunctionExprKind::Call {
            function,
            args,
            site,
            ..
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        FloatFunctionExprKind::Call {
            function,
            args,
            site,
            ..
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        BoolFunctionExprKind::Call {
            function,
            args,
            site,
            ..
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        NilFunctionExprKind::Call {
            function,
            args,
            site,
            ..
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        TupleFunctionExprKind::Call {
            function,
            args,
            site,
            ..
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        ListFunctionExprKind::Call {
            function,
            args,
            site,
            ..
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        bit_array_function_return, bool_function_return, float_function_return,
        int_function_return, list_function_return, nil_function_return, string_function_return,
        tuple_function_return,
    };
    use crate::plan::{
        BitArrayFunctionExpr, BitArrayFunctionReference, BoolExpr, BoolFunctionExpr,
        BoolFunctionReference, FloatExpr, FloatFunctionExpr, FloatFunctionReference, FunctionShape,
        FunctionType, IntFunctionExpr, IntFunctionReference, ListFunctionExpr,
        ListFunctionReference, ReturnBody, StringExpr, StringFunctionExpr, StringFunctionReference,
        TupleFunctionExpr, TupleFunctionReference, ValueShape, ValueType,
        monomorphic_function_instantiation,
    };

    #[test]
    fn concrete_function_returns_preserve_recursive_case_and_block_shapes() {
        let int_reference = IntFunctionExpr::reference(IntFunctionReference::new(
            monomorphic_function_instantiation(
                0,
                FunctionShape::from_function_type(FunctionType::new(Vec::new(), ValueType::Int)),
            ),
        ));
        let int_float_case = IntFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, int_reference.clone())],
            int_reference.clone(),
        );
        let int_bool_case = IntFunctionExpr::bool_case(
            BoolExpr::value(true),
            int_float_case,
            int_reference.clone(),
        );
        assert_eq!(
            int_function_return(int_bool_case),
            ReturnBody::bool_case(
                BoolExpr::value(true),
                ReturnBody::float_case(
                    FloatExpr::value(1.0),
                    vec![(1.0, ReturnBody::expr(int_reference.clone()))],
                    ReturnBody::expr(int_reference.clone()),
                ),
                ReturnBody::expr(int_reference),
            ),
        );

        let string_reference = StringFunctionExpr::reference(StringFunctionReference::new(
            monomorphic_function_instantiation(
                1,
                FunctionShape::from_function_type(FunctionType::new(Vec::new(), ValueType::String)),
            ),
        ));
        let string_float_case = StringFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, string_reference.clone())],
            string_reference.clone(),
        );
        let string_bool_case = StringFunctionExpr::bool_case(
            BoolExpr::value(true),
            string_float_case,
            string_reference.clone(),
        );
        assert_eq!(
            string_function_return(StringFunctionExpr::block(Vec::new(), string_bool_case)),
            ReturnBody::block(
                Vec::new(),
                ReturnBody::bool_case(
                    BoolExpr::value(true),
                    ReturnBody::float_case(
                        FloatExpr::value(1.0),
                        vec![(1.0, ReturnBody::expr(string_reference.clone()))],
                        ReturnBody::expr(string_reference.clone()),
                    ),
                    ReturnBody::expr(string_reference),
                ),
            ),
        );

        let bit_array_reference = BitArrayFunctionExpr::reference(BitArrayFunctionReference::new(
            monomorphic_function_instantiation(
                2,
                FunctionShape::from_function_type(FunctionType::new(
                    Vec::new(),
                    ValueType::BitArray,
                )),
            ),
        ));
        assert_eq!(
            bit_array_function_return(BitArrayFunctionExpr::block(
                Vec::new(),
                bit_array_reference.clone(),
            )),
            ReturnBody::block(Vec::new(), ReturnBody::expr(bit_array_reference)),
        );

        let float_reference = FloatFunctionExpr::reference(FloatFunctionReference::new(
            monomorphic_function_instantiation(
                3,
                FunctionShape::from_function_type(FunctionType::new(Vec::new(), ValueType::Float)),
            ),
        ));
        let float_string_case = FloatFunctionExpr::string_case(
            StringExpr::value("value".into()),
            vec![("value".into(), float_reference.clone())],
            float_reference.clone(),
        );
        assert_eq!(
            float_function_return(FloatFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, float_string_case)],
                float_reference.clone(),
            ),),
            ReturnBody::float_case(
                FloatExpr::value(1.0),
                vec![(
                    1.0,
                    ReturnBody::string_case(
                        StringExpr::value("value".into()),
                        vec![("value".into(), ReturnBody::expr(float_reference.clone()))],
                        ReturnBody::expr(float_reference.clone()),
                    ),
                )],
                ReturnBody::expr(float_reference),
            ),
        );

        let bool_reference = BoolFunctionExpr::reference(BoolFunctionReference::new(
            monomorphic_function_instantiation(
                4,
                FunctionShape::from_function_type(FunctionType::new(Vec::new(), ValueType::Bool)),
            ),
        ));
        let bool_float_case = BoolFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, bool_reference.clone())],
            bool_reference.clone(),
        );
        let bool_bool_case = BoolFunctionExpr::bool_case(
            BoolExpr::value(true),
            bool_float_case,
            bool_reference.clone(),
        );
        assert_eq!(
            bool_function_return(BoolFunctionExpr::block(Vec::new(), bool_bool_case)),
            ReturnBody::block(
                Vec::new(),
                ReturnBody::bool_case(
                    BoolExpr::value(true),
                    ReturnBody::float_case(
                        FloatExpr::value(1.0),
                        vec![(1.0, ReturnBody::expr(bool_reference.clone()))],
                        ReturnBody::expr(bool_reference.clone()),
                    ),
                    ReturnBody::expr(bool_reference),
                ),
            ),
        );

        let nil_reference = crate::plan::NilFunctionExpr::reference(
            crate::plan::NilFunctionReference::new(monomorphic_function_instantiation(
                5,
                FunctionShape::from_function_type(FunctionType::new(Vec::new(), ValueType::Nil)),
            )),
        );
        let nil_float_case = crate::plan::NilFunctionExpr::float_case(
            FloatExpr::value(1.0),
            vec![(1.0, nil_reference.clone())],
            nil_reference.clone(),
        );
        let nil_bool_case = crate::plan::NilFunctionExpr::bool_case(
            BoolExpr::value(true),
            nil_float_case,
            nil_reference.clone(),
        );
        assert_eq!(
            nil_function_return(crate::plan::NilFunctionExpr::block(
                Vec::new(),
                nil_bool_case,
            )),
            ReturnBody::block(
                Vec::new(),
                ReturnBody::bool_case(
                    BoolExpr::value(true),
                    ReturnBody::float_case(
                        FloatExpr::value(1.0),
                        vec![(1.0, ReturnBody::expr(nil_reference.clone()))],
                        ReturnBody::expr(nil_reference.clone()),
                    ),
                    ReturnBody::expr(nil_reference),
                ),
            ),
        );

        let tuple_reference = TupleFunctionExpr::reference(TupleFunctionReference::new(
            monomorphic_function_instantiation(
                6,
                FunctionShape::new(
                    Vec::new(),
                    ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
                ),
            ),
        ));
        let tuple_string_case = TupleFunctionExpr::string_case(
            StringExpr::value("value".into()),
            vec![("value".into(), tuple_reference.clone())],
            tuple_reference.clone(),
        );
        assert_eq!(
            tuple_function_return(TupleFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, tuple_string_case)],
                tuple_reference.clone(),
            ),),
            ReturnBody::float_case(
                FloatExpr::value(1.0),
                vec![(
                    1.0,
                    ReturnBody::string_case(
                        StringExpr::value("value".into()),
                        vec![("value".into(), ReturnBody::expr(tuple_reference.clone()))],
                        ReturnBody::expr(tuple_reference.clone()),
                    ),
                )],
                ReturnBody::expr(tuple_reference),
            ),
        );
        let list_reference = ListFunctionExpr::reference(
            ListFunctionReference::new(monomorphic_function_instantiation(
                7,
                FunctionShape::new(Vec::new(), ValueShape::List(Box::new(ValueShape::Int))),
            )),
            ValueType::Int,
        );
        let list_string_case = ListFunctionExpr::string_case(
            StringExpr::value("value".into()),
            vec![("value".into(), list_reference.clone())],
            list_reference.clone(),
        );
        assert_eq!(
            list_function_return(ListFunctionExpr::float_case(
                FloatExpr::value(1.0),
                vec![(1.0, list_string_case)],
                list_reference.clone(),
            ),),
            ReturnBody::float_case(
                FloatExpr::value(1.0),
                vec![(
                    1.0,
                    ReturnBody::string_case(
                        StringExpr::value("value".into()),
                        vec![("value".into(), ReturnBody::expr(list_reference.clone()))],
                        ReturnBody::expr(list_reference.clone()),
                    ),
                )],
                ReturnBody::expr(list_reference),
            ),
        );
    }
}
