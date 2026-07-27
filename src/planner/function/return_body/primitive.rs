use crate::plan::{
    BitArrayExpr, BitArrayExprKind, BitArrayReturn, BoolExpr, BoolExprKind, BoolReturn, CustomExpr,
    CustomReturn, FloatExpr, FloatExprKind, FloatReturn, GenericExpr, GenericExprKind,
    GenericReturn, IntExpr, IntExprKind, IntReturn, ListItem, NilExpr, NilExprKind, NilReturn,
    ReturnBody, StringExpr, StringExprKind, StringReturn, TupleExpr, TupleExprKind, TupleReturn,
    TypedListExpr, TypedListReturnKind, UtfCodepointExpr, UtfCodepointExprKind, UtfCodepointReturn,
};

pub(super) fn custom_return(
    signature_shape: crate::plan::CustomValueShape,
    expression: CustomExpr,
) -> CustomReturn {
    CustomReturn::with_signature_shape(signature_shape, expression)
}

pub(super) fn generic_return(expression: GenericExpr) -> GenericReturn {
    match expression.kind() {
        GenericExprKind::Call {
            function,
            args,
            site,
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
        GenericExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            generic_return((**true_).clone()),
            generic_return((**false_).clone()),
        ),
        GenericExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), generic_return(branch.clone())))
                .collect(),
            generic_return((**fallback).clone()),
        ),
        GenericExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), generic_return(branch.clone())))
                .collect(),
            generic_return((**fallback).clone()),
        ),
        GenericExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, generic_return(branch.clone())))
                .collect(),
            generic_return((**fallback).clone()),
        ),
        GenericExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), generic_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

#[cfg(test)]
use crate::plan::{ListExpr, ListReturn};

pub(super) fn int_return(expression: IntExpr) -> IntReturn {
    match expression.kind() {
        IntExprKind::Call {
            function,
            args,
            site,
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        StringExprKind::Call {
            function,
            args,
            site,
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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

pub(super) fn bit_array_return(expression: BitArrayExpr) -> BitArrayReturn {
    match expression.kind() {
        BitArrayExprKind::Call {
            function,
            args,
            site,
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
        BitArrayExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            bit_array_return((**true_).clone()),
            bit_array_return((**false_).clone()),
        ),
        BitArrayExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), bit_array_return(branch.clone())))
                .collect(),
            bit_array_return((**fallback).clone()),
        ),
        BitArrayExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), bit_array_return(branch.clone())))
                .collect(),
            bit_array_return((**fallback).clone()),
        ),
        BitArrayExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, bit_array_return(branch.clone())))
                .collect(),
            bit_array_return((**fallback).clone()),
        ),
        BitArrayExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), bit_array_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

pub(super) fn utf_codepoint_return(expression: UtfCodepointExpr) -> UtfCodepointReturn {
    match expression.kind() {
        UtfCodepointExprKind::Call {
            function,
            args,
            site,
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
        UtfCodepointExprKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            (**subject).clone(),
            utf_codepoint_return((**true_).clone()),
            utf_codepoint_return((**false_).clone()),
        ),
        UtfCodepointExprKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), utf_codepoint_return(branch.clone())))
                .collect(),
            utf_codepoint_return((**fallback).clone()),
        ),
        UtfCodepointExprKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (value.clone(), utf_codepoint_return(branch.clone())))
                .collect(),
            utf_codepoint_return((**fallback).clone()),
        ),
        UtfCodepointExprKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            (**subject).clone(),
            clauses
                .iter()
                .map(|(value, branch)| (*value, utf_codepoint_return(branch.clone())))
                .collect(),
            utf_codepoint_return((**fallback).clone()),
        ),
        UtfCodepointExprKind::Block { steps, return_ } => {
            ReturnBody::block(steps.clone(), utf_codepoint_return((**return_).clone()))
        }
        _ => ReturnBody::expr(expression),
    }
}

pub(super) fn bool_return(expression: BoolExpr) -> BoolReturn {
    match expression.kind() {
        BoolExprKind::Call {
            function,
            args,
            site,
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        NilExprKind::Call {
            function,
            args,
            site,
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        FloatExprKind::Call {
            function,
            args,
            site,
        } => ReturnBody::tail_call(
            crate::plan::FunctionCallTarget::new(function.clone(), site.clone()),
            args.clone(),
        ),
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
        TupleExprKind::Call { function, args } => {
            ReturnBody::tail_call(function.clone(), args.clone())
        }
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

#[cfg(test)]
pub(super) fn list_return(expression: ListExpr) -> ListReturn {
    match expression {
        ListExpr::Generic(expression) => ListReturn::Generic {
            item_parameter: expression.item().parameter(),
            body: typed_list_return_body(expression),
        },
        ListExpr::Int(expression) => ListReturn::Int(typed_list_return_body(expression)),
        ListExpr::String(expression) => ListReturn::String(typed_list_return_body(expression)),
        ListExpr::BitArray(expression) => ListReturn::BitArray(typed_list_return_body(expression)),
        ListExpr::UtfCodepoint(expression) => {
            ListReturn::UtfCodepoint(typed_list_return_body(expression))
        }
        ListExpr::Custom(expression) => ListReturn::Custom {
            item_type: expression.item().item_type(),
            body: typed_list_return_body(expression),
        },
        ListExpr::Float(expression) => ListReturn::Float(typed_list_return_body(expression)),
        ListExpr::Bool(expression) => ListReturn::Bool(typed_list_return_body(expression)),
        ListExpr::Nil(expression) => ListReturn::Nil(typed_list_return_body(expression)),
        ListExpr::Tuple(expression) => ListReturn::Tuple {
            item_type: expression.item().item_type(),
            body: typed_list_return_body(expression),
        },
        ListExpr::ParameterList(expression) => ListReturn::ParameterList {
            item_parameter: expression.item().parameter(),
            body: typed_list_return_body(expression),
        },
        ListExpr::List(expression) => ListReturn::List {
            item_shape: expression.item().item_shape().clone(),
            body: typed_list_return_body(expression),
        },
        ListExpr::Function(expression) => ListReturn::Function {
            item_type: expression.item().item_type(),
            body: typed_list_return_body(expression),
        },
    }
}

pub(super) fn typed_list_return_body<Item: ListItem>(
    expression: TypedListExpr<Item>,
) -> ReturnBody<TypedListExpr<Item>, Item::Function> {
    match expression.into_return_kind() {
        TypedListReturnKind::Call { function, args } => ReturnBody::tail_call(function, args),
        TypedListReturnKind::BoolCase {
            subject,
            true_,
            false_,
        } => ReturnBody::bool_case(
            subject,
            typed_list_return_body(true_),
            typed_list_return_body(false_),
        ),
        TypedListReturnKind::IntCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::int_case(
            subject,
            clauses
                .into_iter()
                .map(|(value, branch)| (value, typed_list_return_body(branch)))
                .collect(),
            typed_list_return_body(fallback),
        ),
        TypedListReturnKind::StringCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::string_case(
            subject,
            clauses
                .into_iter()
                .map(|(value, branch)| (value, typed_list_return_body(branch)))
                .collect(),
            typed_list_return_body(fallback),
        ),
        TypedListReturnKind::FloatCase {
            subject,
            clauses,
            fallback,
        } => ReturnBody::float_case(
            subject,
            clauses
                .into_iter()
                .map(|(value, branch)| (value, typed_list_return_body(branch)))
                .collect(),
            typed_list_return_body(fallback),
        ),
        TypedListReturnKind::Block { steps, return_ } => {
            ReturnBody::block(steps, typed_list_return_body(return_))
        }
        TypedListReturnKind::Expr(expression) => ReturnBody::expr(expression),
    }
}

#[cfg(test)]
mod tests {
    use super::{
        bool_return, float_return, int_return, list_return, nil_return, string_return, tuple_return,
    };
    use crate::plan::{
        BoolExpr, CustomType, CustomTypeName, Expr, FloatExpr, FunctionType, IntExpr,
        ListCaseBranches, ListExpr, ListReturn, NilExpr, ReturnBody, Step, StringExpr, TupleExpr,
        TypeParameterId, ValueType,
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
            ListReturn::try_float_case(
                FloatExpr::value(1.0),
                vec![(1.0, ListReturn::expr(list_value()))],
                ListReturn::expr(list_value()),
            )
            .expect("float list case branches should share an item family"),
        );
    }

    #[test]
    fn tuple_return_preserves_block_return_body_shape() {
        let step = Step::evaluate(Expr::int(IntExpr::value(0.into())));
        let value = tuple_value();

        assert_eq!(
            tuple_return(TupleExpr::block(vec![step.clone()], value.clone())),
            ReturnBody::block(vec![step], ReturnBody::expr(value)),
        );
    }

    #[test]
    fn list_return_preserves_every_item_family() {
        let parameter = TypeParameterId(0);
        assert_eq!(
            list_return(ListExpr::value(Vec::new(), ValueType::Parameter(parameter),)),
            ListReturn::expr(ListExpr::value(Vec::new(), ValueType::Parameter(parameter),)),
        );
        assert_eq!(
            list_return(ListExpr::value(Vec::new(), ValueType::Int)),
            ListReturn::expr(ListExpr::value(Vec::new(), ValueType::Int)),
        );
        assert_eq!(
            list_return(ListExpr::value(Vec::new(), ValueType::String)),
            ListReturn::expr(ListExpr::value(Vec::new(), ValueType::String)),
        );
        assert_eq!(
            list_return(ListExpr::value(Vec::new(), ValueType::BitArray)),
            ListReturn::expr(ListExpr::value(Vec::new(), ValueType::BitArray)),
        );
        assert_eq!(
            list_return(ListExpr::value(Vec::new(), ValueType::UtfCodepoint)),
            ListReturn::expr(ListExpr::value(Vec::new(), ValueType::UtfCodepoint)),
        );
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        assert_eq!(
            list_return(ListExpr::value(
                Vec::new(),
                ValueType::Custom(custom_type.clone()),
            )),
            ListReturn::expr(ListExpr::value(Vec::new(), ValueType::Custom(custom_type),)),
        );
        assert_eq!(
            list_return(ListExpr::value(Vec::new(), ValueType::Bool)),
            ListReturn::expr(ListExpr::value(Vec::new(), ValueType::Bool)),
        );
        assert_eq!(
            list_return(ListExpr::value(Vec::new(), ValueType::Nil)),
            ListReturn::expr(ListExpr::value(Vec::new(), ValueType::Nil)),
        );
        assert_eq!(
            list_return(ListExpr::value(
                Vec::new(),
                ValueType::Tuple(vec![ValueType::Int])
            )),
            ListReturn::expr(ListExpr::value(
                Vec::new(),
                ValueType::Tuple(vec![ValueType::Int])
            )),
        );
        assert_eq!(
            list_return(ListExpr::value(
                Vec::new(),
                ValueType::List(Box::new(ValueType::Parameter(parameter))),
            )),
            ListReturn::expr(ListExpr::value(
                Vec::new(),
                ValueType::List(Box::new(ValueType::Parameter(parameter))),
            )),
        );
        assert_eq!(
            list_return(ListExpr::value(
                Vec::new(),
                ValueType::List(Box::new(ValueType::String)),
            )),
            ListReturn::expr(ListExpr::value(
                Vec::new(),
                ValueType::List(Box::new(ValueType::String)),
            )),
        );
        assert_eq!(
            list_return(ListExpr::value(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
            )),
            ListReturn::expr(ListExpr::value(
                Vec::new(),
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Bool))),
            )),
        );
    }

    #[test]
    fn float_return_preserves_tail_and_case_return_body_shapes() {
        let function = crate::plan::monomorphic_function_instantiation(
            1,
            crate::plan::FunctionShape::new(Vec::new(), crate::plan::ValueShape::Float),
        );
        assert_eq!(
            float_return(FloatExpr::call(function.clone(), Vec::new())),
            ReturnBody::tail_call(function, Vec::new()),
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
            ListCaseBranches::from_exprs(vec![(1.0, list_value())], list_value())
                .expect("list case branches"),
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
