use super::{
    BitArray, Bool, Float, Function, FunctionFunction, Int, IntFunction, List, ListFunction, Nil,
    String, TupleFunction, UtfCodepoint,
};
use crate::plan::{
    BitArrayExpr, BitArrayFunctionExpr, BoolExpr, BoolFunctionExpr, CustomFunctionExpr, FloatExpr,
    FloatFunctionExpr, FunctionExpr, FunctionExprKind, FunctionFunctionExpr, GenericFunctionExpr,
    IntExpr, IntFunctionExpr, ListExpr, ListFunctionExpr, NilExpr, NilFunctionExpr, Step,
    StringExpr, StringFunctionExpr, TupleFunctionExpr, UtfCodepointExpr, UtfCodepointFunctionExpr,
};

pub(crate) fn block_int(steps: impl IntoIterator<Item = Step>, return_: Int) -> Int {
    Int(IntExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_string(steps: impl IntoIterator<Item = Step>, return_: String) -> String {
    String(StringExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_bit_array(
    steps: impl IntoIterator<Item = Step>,
    return_: BitArray,
) -> BitArray {
    BitArray(BitArrayExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_utf_codepoint(
    steps: impl IntoIterator<Item = Step>,
    return_: UtfCodepoint,
) -> UtfCodepoint {
    UtfCodepoint(UtfCodepointExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_float(steps: impl IntoIterator<Item = Step>, return_: Float) -> Float {
    Float(FloatExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_bool(steps: impl IntoIterator<Item = Step>, return_: Bool) -> Bool {
    Bool(BoolExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_nil(steps: impl IntoIterator<Item = Step>, return_: Nil) -> Nil {
    Nil(NilExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_list(steps: impl IntoIterator<Item = Step>, return_: List) -> List {
    List(ListExpr::block(steps.into_iter().collect(), return_.into()))
}

pub(crate) fn block_function(steps: Vec<Step>, return_: Function) -> Function {
    Function(match FunctionExpr::from(return_).into_kind() {
        FunctionExprKind::Generic(return_) => {
            FunctionExpr::generic(GenericFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Int(return_) => FunctionExpr::int(IntFunctionExpr::block(steps, return_)),
        FunctionExprKind::String(return_) => {
            FunctionExpr::string(StringFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::BitArray(return_) => {
            FunctionExpr::bit_array(BitArrayFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::UtfCodepoint(return_) => {
            FunctionExpr::utf_codepoint(UtfCodepointFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Custom(return_) => {
            FunctionExpr::custom(CustomFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Float(return_) => {
            FunctionExpr::float(FloatFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Bool(return_) => {
            FunctionExpr::bool(BoolFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Nil(return_) => FunctionExpr::nil(NilFunctionExpr::block(steps, return_)),
        FunctionExprKind::Tuple(return_) => {
            FunctionExpr::tuple(TupleFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::List(return_) => {
            FunctionExpr::list(ListFunctionExpr::block(steps, return_))
        }
        FunctionExprKind::Function(return_) => {
            FunctionExpr::function(FunctionFunctionExpr::block(steps, return_))
        }
    })
}

pub(crate) fn block_tuple_function(
    steps: impl IntoIterator<Item = Step>,
    return_: TupleFunction,
) -> TupleFunction {
    TupleFunction(TupleFunctionExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_list_function(
    steps: impl IntoIterator<Item = Step>,
    return_: ListFunction,
) -> ListFunction {
    ListFunction(ListFunctionExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_function_function(
    steps: impl IntoIterator<Item = Step>,
    return_: FunctionFunction,
) -> FunctionFunction {
    FunctionFunction(FunctionFunctionExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

pub(crate) fn block_int_function(
    steps: impl IntoIterator<Item = Step>,
    return_: IntFunction,
) -> IntFunction {
    IntFunction(IntFunctionExpr::block(
        steps.into_iter().collect(),
        return_.into(),
    ))
}

#[cfg(test)]
mod tests {
    use super::{
        block_bit_array, block_bool, block_float, block_function, block_function_function,
        block_int, block_int_function, block_list, block_list_function, block_nil, block_string,
        block_tuple_function, block_utf_codepoint,
    };
    use crate::plan::{
        BitArrayExpr, BoolExpr, CustomFunctionExpr, CustomFunctionReference, CustomType,
        CustomTypeName, CustomValueShape, FloatExpr, FunctionExpr, FunctionFunctionExpr,
        FunctionFunctionId, FunctionShape, FunctionType, GenericFunctionExpr,
        GenericFunctionReference, GenericFunctionType, IntExpr, IntFunctionExpr,
        IntFunctionFunctionId, ListExpr, ListFunctionExpr, NilExpr, ParamLocal, StringExpr,
        TupleFunctionExpr, TypeParameterId, UtfCodepointExpr, ValueShape, ValueType,
        monomorphic_function_instantiation,
    };
    use crate::planner::dsl::expression::{
        Function, bit_array, bit_array_function_ref, bool_, bool_function_ref, float,
        float_function_ref, function_function_ref, int, int_function_ref, let_bit_array_step,
        let_bool_step, let_int_step, let_nil_step, let_string_step, list, list_function_ref,
        local_bit_array, local_bool, local_int, local_nil, local_string, local_utf_codepoint, nil,
        nil_function_ref, string, string_function_ref, tuple_function_ref,
        utf_codepoint_function_ref,
    };

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }

    #[test]
    fn primitive_block_helpers_build_block_shapes() {
        assert_eq!(
            block_int([let_int_step(0, "x", int(1))], local_int(0, "x")).0,
            IntExpr::block(vec![let_int_step(0, "x", int(1))], local_int(0, "x").into()),
        );
        assert_eq!(
            block_string([let_string_step(0, "x", string("a"))], local_string(0, "x")).0,
            StringExpr::block(
                vec![let_string_step(0, "x", string("a"))],
                local_string(0, "x").into(),
            ),
        );
        assert_eq!(
            block_bit_array(
                [let_bit_array_step(0, "x", bit_array([]))],
                local_bit_array(0, "x"),
            )
            .0,
            BitArrayExpr::block(
                vec![let_bit_array_step(0, "x", bit_array([]))],
                local_bit_array(0, "x").into(),
            ),
        );
        assert_eq!(
            block_utf_codepoint(Vec::new(), local_utf_codepoint(0, "codepoint")).0,
            UtfCodepointExpr::block(Vec::new(), local_utf_codepoint(0, "codepoint").into(),),
        );
        assert_eq!(
            block_float([], float(1.0)).0,
            FloatExpr::block(Vec::new(), float(1.0).into()),
        );
        assert_eq!(
            block_bool([let_bool_step(0, "x", bool_(true))], local_bool(0, "x")).0,
            BoolExpr::block(
                vec![let_bool_step(0, "x", bool_(true))],
                local_bool(0, "x").into(),
            ),
        );
        assert_eq!(
            block_nil([let_nil_step(0, "x", nil())], local_nil(0, "x")).0,
            NilExpr::block(vec![let_nil_step(0, "x", nil())], local_nil(0, "x").into()),
        );
        assert_eq!(
            block_list([], list([int(1)], ValueType::Int)).0,
            ListExpr::block(Vec::new(), list([int(1)], ValueType::Int).into()),
        );
    }

    #[test]
    fn function_block_helpers_preserve_return_family() {
        let parameter = TypeParameterId(0);
        let generic_type = GenericFunctionType::new(Vec::new(), parameter);
        let generic_shape = generic_type.shape();
        let generic = GenericFunctionExpr::reference(
            GenericFunctionReference::new(
                monomorphic_function_instantiation(0, generic_shape),
                Vec::new(),
            ),
            generic_type,
        );
        let custom_shape = CustomValueShape::any(custom_type());
        let custom = CustomFunctionExpr::reference(
            CustomFunctionReference::new(
                monomorphic_function_instantiation(
                    0,
                    FunctionShape::new(Vec::new(), ValueShape::Custom(custom_shape.clone())),
                ),
                Vec::new(),
            ),
            custom_shape,
        );
        let returned_function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let expressions = vec![
            FunctionExpr::generic(generic),
            FunctionExpr::from(int_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionExpr::string(string_function_ref(0, Vec::<ParamLocal>::new()).0),
            FunctionExpr::from(bit_array_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionExpr::from(utf_codepoint_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionExpr::custom(custom),
            FunctionExpr::from(float_function_ref(0, Vec::<ParamLocal>::new())),
            FunctionExpr::bool(bool_function_ref(0, Vec::<ParamLocal>::new()).0),
            FunctionExpr::nil(nil_function_ref(0, Vec::<ParamLocal>::new()).0),
            FunctionExpr::from(tuple_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                [ValueType::Int],
            )),
            FunctionExpr::from(list_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                ValueType::Int,
            )),
            FunctionExpr::from(function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::<ParamLocal>::new(),
                returned_function_type.clone(),
            )),
        ];
        for expression in expressions {
            assert_eq!(
                FunctionExpr::from(block_function(Vec::new(), Function(expression.clone()))),
                FunctionExpr::block(Vec::new(), expression),
            );
        }

        assert_eq!(
            block_int_function(
                [],
                int_function_ref(0, [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]),
            )
            .0,
            IntFunctionExpr::block(
                Vec::new(),
                int_function_ref(0, [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]).into(),
            ),
        );
        assert_eq!(
            block_tuple_function(
                [],
                tuple_function_ref(
                    0,
                    [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                    [ValueType::Int, ValueType::String],
                ),
            )
            .0,
            TupleFunctionExpr::block(
                Vec::new(),
                tuple_function_ref(
                    0,
                    [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))],
                    [ValueType::Int, ValueType::String],
                )
                .into(),
            ),
        );
        assert_eq!(
            block_list_function(
                [],
                list_function_ref(0, Vec::<ParamLocal>::new(), ValueType::Int)
            )
            .0,
            ListFunctionExpr::block(
                Vec::new(),
                list_function_ref(0, Vec::<ParamLocal>::new(), ValueType::Int).into(),
            ),
        );
        assert_eq!(
            block_function_function(
                [],
                function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type.clone(),
                ),
            )
            .0,
            FunctionFunctionExpr::block(
                Vec::new(),
                function_function_ref(
                    FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                    Vec::<ParamLocal>::new(),
                    returned_function_type,
                )
                .into(),
            ),
        );
    }
}
