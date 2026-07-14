use super::{
    BitArray, BitArrayFunction, Bool, BoolFunction, Float, FloatFunction, Int, IntFunction, List,
    Nil, NilFunction, String, StringFunction, Tuple, UtfCodepoint, UtfCodepointFunction,
};
use crate::plan::{
    BitArrayFunctionLocalId, BitArrayLocalId, BoolFunctionLocalId, BoolListLocalId, BoolLocalId,
    CustomListLocalId, Expr, FloatFunctionLocalId, FloatListLocalId, FloatLocalId,
    FunctionListLocalId, IntFunctionLocalId, IntListLocalId, IntLocalId, ListExpr, ListListLocalId,
    ListLocalExpr, NilFunctionLocalId, NilListLocalId, NilLocalId, Step, StringFunctionLocalId,
    StringListLocalId, StringLocalId, TupleListLocalId, TupleLocalId, UtfCodepointFunctionLocalId,
    UtfCodepointLocalId,
};
use ecow::EcoString;

pub(crate) fn let_int_step(local: usize, name: impl Into<EcoString>, value: Int) -> Step {
    Step::let_int(IntLocalId(local), name.into(), value.into())
}

pub(crate) fn let_string_step(local: usize, name: impl Into<EcoString>, value: String) -> Step {
    Step::let_string(StringLocalId(local), name.into(), value.into())
}

pub(crate) fn let_bit_array_step(
    local: usize,
    name: impl Into<EcoString>,
    value: BitArray,
) -> Step {
    Step::let_bit_array(BitArrayLocalId(local), name.into(), value.into())
}

pub(crate) fn let_utf_codepoint_step(
    local: usize,
    name: impl Into<EcoString>,
    value: UtfCodepoint,
) -> Step {
    Step::let_utf_codepoint(UtfCodepointLocalId(local), name.into(), value.into())
}

pub(crate) fn let_float_step(local: usize, name: impl Into<EcoString>, value: Float) -> Step {
    Step::let_float(FloatLocalId(local), name.into(), value.into())
}

pub(crate) fn let_bool_step(local: usize, name: impl Into<EcoString>, value: Bool) -> Step {
    Step::let_bool(BoolLocalId(local), name.into(), value.into())
}

pub(crate) fn let_nil_step(local: usize, name: impl Into<EcoString>, value: Nil) -> Step {
    Step::let_nil(NilLocalId(local), name.into(), value.into())
}

pub(crate) fn let_tuple_step(local: usize, name: impl Into<EcoString>, value: Tuple) -> Step {
    Step::let_tuple(TupleLocalId(local), name.into(), value.into())
}

pub(crate) fn let_list_step(local: usize, name: impl Into<EcoString>, value: List) -> Step {
    let value = match value.0 {
        ListExpr::Int(value) => ListLocalExpr::Int {
            local: IntListLocalId(local),
            value,
        },
        ListExpr::String(value) => ListLocalExpr::String {
            local: StringListLocalId(local),
            value,
        },
        ListExpr::BitArray(value) => ListLocalExpr::BitArray {
            local: crate::plan::BitArrayListLocalId(local),
            value,
        },
        ListExpr::UtfCodepoint(value) => ListLocalExpr::UtfCodepoint {
            local: crate::plan::UtfCodepointListLocalId(local),
            value,
        },
        ListExpr::Custom(value) => ListLocalExpr::Custom {
            local: CustomListLocalId(local),
            item_type: value.item().item_type(),
            value,
        },
        ListExpr::Float(value) => ListLocalExpr::Float {
            local: FloatListLocalId(local),
            value,
        },
        ListExpr::Bool(value) => ListLocalExpr::Bool {
            local: BoolListLocalId(local),
            value,
        },
        ListExpr::Nil(value) => ListLocalExpr::Nil {
            local: NilListLocalId(local),
            value,
        },
        ListExpr::Tuple(value) => ListLocalExpr::Tuple {
            local: TupleListLocalId(local),
            item_type: value.item().item_type(),
            value,
        },
        ListExpr::List(value) => ListLocalExpr::List {
            local: ListListLocalId(local),
            item_type: value.item().item_type(),
            value,
        },
        ListExpr::Function(value) => ListLocalExpr::Function {
            local: FunctionListLocalId(local),
            item_type: value.item().item_type(),
            value,
        },
    };

    Step::let_list_expr(name.into(), value)
}

pub(crate) fn let_int_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: IntFunction,
) -> Step {
    Step::let_int_function(IntFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn let_string_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: StringFunction,
) -> Step {
    Step::let_string_function(StringFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn let_bit_array_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: BitArrayFunction,
) -> Step {
    Step::let_bit_array_function(BitArrayFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn let_utf_codepoint_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: UtfCodepointFunction,
) -> Step {
    Step::let_utf_codepoint_function(
        UtfCodepointFunctionLocalId(local),
        name.into(),
        value.into(),
    )
}

pub(crate) fn let_float_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: FloatFunction,
) -> Step {
    Step::let_float_function(FloatFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn let_bool_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: BoolFunction,
) -> Step {
    Step::let_bool_function(BoolFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn let_nil_function_step(
    local: usize,
    name: impl Into<EcoString>,
    value: NilFunction,
) -> Step {
    Step::let_nil_function(NilFunctionLocalId(local), name.into(), value.into())
}

pub(crate) fn evaluate_step(value: impl Into<Expr>) -> Step {
    Step::evaluate(value.into())
}

#[cfg(test)]
mod tests {
    use super::{
        evaluate_step, let_bit_array_function_step, let_bit_array_step, let_bool_function_step,
        let_bool_step, let_float_function_step, let_float_step, let_int_function_step,
        let_int_step, let_list_step, let_nil_function_step, let_nil_step, let_string_function_step,
        let_string_step, let_tuple_step, let_utf_codepoint_function_step, let_utf_codepoint_step,
    };
    use crate::plan::{
        BitArrayExpr, BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId,
        BoolFunctionLocalId, BoolListLocalId, BoolLocalId, CustomListLocalId, CustomType,
        CustomTypeName, Expr, FloatFunctionLocalId, FloatListLocalId, FloatLocalId,
        FunctionListLocalId, FunctionType, IntFunctionLocalId, IntListLocalId, IntLocalId,
        ListExpr, ListListLocalId, ListLocalExpr, NilFunctionLocalId, NilListLocalId, NilLocalId,
        Step, StringFunctionLocalId, StringListLocalId, StringLocalId, TupleListLocalId,
        TupleLocalId, UtfCodepointFunctionLocalId, UtfCodepointListLocalId, UtfCodepointLocalId,
        ValueType,
    };
    use crate::planner::dsl::expression::{
        bit_array, bit_array_function_ref, bool_, bool_function_ref, float, float_function_ref,
        int, int_function_ref, list, local_utf_codepoint, nil, nil_function_ref, string,
        string_function_ref, tuple, utf_codepoint_function_ref,
    };

    #[test]
    fn step_helpers_build_step_shapes() {
        let custom_type = CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        );
        assert_eq!(
            let_int_step(0, "x", int(1)),
            Step::let_int(IntLocalId(0), "x".into(), int(1).into()),
        );
        assert_eq!(
            let_string_step(1, "name", string("a")),
            Step::let_string(StringLocalId(1), "name".into(), string("a").into()),
        );
        assert_eq!(
            let_bit_array_step(14, "bits", bit_array([])),
            Step::let_bit_array(
                BitArrayLocalId(14),
                "bits".into(),
                BitArrayExpr::value(Vec::new()),
            ),
        );
        assert_eq!(
            let_utf_codepoint_step(15, "codepoint", local_utf_codepoint(0, "source")),
            Step::let_utf_codepoint(
                UtfCodepointLocalId(15),
                "codepoint".into(),
                local_utf_codepoint(0, "source").into(),
            ),
        );
        assert_eq!(
            let_float_step(2, "ratio", float(1.0)),
            Step::let_float(FloatLocalId(2), "ratio".into(), float(1.0).into()),
        );
        assert_eq!(
            let_bool_step(3, "ok", bool_(true)),
            Step::let_bool(BoolLocalId(3), "ok".into(), bool_(true).into()),
        );
        assert_eq!(
            let_nil_step(4, "done", nil()),
            Step::let_nil(NilLocalId(4), "done".into(), nil().into()),
        );
        assert_eq!(
            let_tuple_step(
                5,
                "pair",
                tuple([Expr::from(int(1)), Expr::from(string("one"))])
            ),
            Step::let_tuple(
                TupleLocalId(5),
                "pair".into(),
                tuple([Expr::from(int(1)), Expr::from(string("one"))]).into(),
            ),
        );
        assert_eq!(
            let_list_step(6, "values", list([int(1)], ValueType::Int)),
            Step::let_list_expr(
                "values".into(),
                ListLocalExpr::Int {
                    local: IntListLocalId(6),
                    value: ListExpr::from(list([int(1)], ValueType::Int))
                        .into_int()
                        .expect("expected int list"),
                },
            ),
        );
        assert_eq!(
            let_list_step(7, "names", list([string("a")], ValueType::String)),
            Step::let_list_expr(
                "names".into(),
                ListLocalExpr::String {
                    local: StringListLocalId(7),
                    value: ListExpr::from(list([string("a")], ValueType::String))
                        .into_string()
                        .expect("expected string list"),
                },
            ),
        );
        assert_eq!(
            let_list_step(14, "bits", list([bit_array([])], ValueType::BitArray)),
            Step::let_list_expr(
                "bits".into(),
                ListLocalExpr::BitArray {
                    local: BitArrayListLocalId(14),
                    value: ListExpr::from(list([bit_array([])], ValueType::BitArray))
                        .into_bit_array()
                        .expect("expected bit array list"),
                },
            ),
        );
        assert_eq!(
            let_list_step(
                15,
                "codepoints",
                list([local_utf_codepoint(0, "source")], ValueType::UtfCodepoint,),
            ),
            Step::let_list_expr(
                "codepoints".into(),
                ListLocalExpr::UtfCodepoint {
                    local: UtfCodepointListLocalId(15),
                    value: ListExpr::from(list(
                        [local_utf_codepoint(0, "source")],
                        ValueType::UtfCodepoint,
                    ))
                    .into_utf_codepoint()
                    .expect("expected UTF codepoint list"),
                },
            ),
        );
        assert_eq!(
            let_list_step(
                14,
                "customs",
                list(Vec::<Expr>::new(), ValueType::Custom(custom_type.clone())),
            ),
            Step::let_list_expr(
                "customs".into(),
                ListLocalExpr::Custom {
                    local: CustomListLocalId(14),
                    item_type: custom_type.clone(),
                    value: ListExpr::from(
                        list(Vec::<Expr>::new(), ValueType::Custom(custom_type),)
                    )
                    .into_custom()
                    .expect("expected custom list"),
                },
            ),
        );
        assert_eq!(
            let_list_step(8, "ratios", list([float(1.5)], ValueType::Float)),
            Step::let_list_expr(
                "ratios".into(),
                ListLocalExpr::Float {
                    local: FloatListLocalId(8),
                    value: ListExpr::from(list([float(1.5)], ValueType::Float))
                        .into_float()
                        .expect("expected float list"),
                },
            ),
        );
        assert_eq!(
            let_list_step(9, "flags", list([bool_(true)], ValueType::Bool)),
            Step::let_list_expr(
                "flags".into(),
                ListLocalExpr::Bool {
                    local: BoolListLocalId(9),
                    value: ListExpr::from(list([bool_(true)], ValueType::Bool))
                        .into_bool()
                        .expect("expected bool list"),
                },
            ),
        );
        assert_eq!(
            let_list_step(10, "nils", list([nil()], ValueType::Nil)),
            Step::let_list_expr(
                "nils".into(),
                ListLocalExpr::Nil {
                    local: NilListLocalId(10),
                    value: ListExpr::from(list([nil()], ValueType::Nil))
                        .into_nil()
                        .expect("expected nil list"),
                },
            ),
        );
        assert_eq!(
            let_list_step(
                11,
                "pairs",
                list(
                    [tuple([Expr::from(int(1)), Expr::from(string("one"))])],
                    ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
                ),
            ),
            Step::let_list_expr(
                "pairs".into(),
                ListLocalExpr::Tuple {
                    local: TupleListLocalId(11),
                    item_type: vec![ValueType::Int, ValueType::String],
                    value: ListExpr::from(list(
                        [tuple([Expr::from(int(1)), Expr::from(string("one"))])],
                        ValueType::Tuple(vec![ValueType::Int, ValueType::String]),
                    ))
                    .into_tuple()
                    .expect("expected tuple list"),
                },
            ),
        );
        assert_eq!(
            let_list_step(
                12,
                "nested",
                list(
                    [list([int(1)], ValueType::Int)],
                    ValueType::List(Box::new(ValueType::Int)),
                ),
            ),
            Step::let_list_expr(
                "nested".into(),
                ListLocalExpr::List {
                    local: ListListLocalId(12),
                    item_type: Box::new(ValueType::Int),
                    value: ListExpr::from(list(
                        [list([int(1)], ValueType::Int)],
                        ValueType::List(Box::new(ValueType::Int)),
                    ))
                    .into_list()
                    .expect("expected nested list"),
                },
            ),
        );
        let function_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        assert_eq!(
            let_list_step(
                13,
                "functions",
                list(
                    [int_function_ref(
                        0,
                        [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]
                    )],
                    ValueType::Function(Box::new(function_type.clone())),
                ),
            ),
            Step::let_list_expr(
                "functions".into(),
                ListLocalExpr::Function {
                    local: FunctionListLocalId(13),
                    item_type: function_type.clone(),
                    value: ListExpr::from(list(
                        [int_function_ref(
                            0,
                            [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]
                        )],
                        ValueType::Function(Box::new(function_type)),
                    ))
                    .into_function()
                    .expect("expected function list"),
                },
            ),
        );
        assert_eq!(
            let_int_function_step(
                0,
                "f",
                int_function_ref(0, [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]),
            ),
            Step::let_int_function(
                IntFunctionLocalId(0),
                "f".into(),
                int_function_ref(0, [crate::plan::LocalId::Int(crate::plan::IntLocalId(0))]).into(),
            ),
        );
        assert_eq!(
            let_string_function_step(
                1,
                "f",
                string_function_ref(
                    0,
                    [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                ),
            ),
            Step::let_string_function(
                StringFunctionLocalId(1),
                "f".into(),
                string_function_ref(
                    0,
                    [crate::plan::LocalId::String(crate::plan::StringLocalId(0))],
                )
                .into(),
            ),
        );
        assert_eq!(
            let_bit_array_function_step(
                5,
                "f",
                bit_array_function_ref(
                    0,
                    [crate::plan::LocalId::BitArray(
                        crate::plan::BitArrayLocalId(0)
                    )],
                ),
            ),
            Step::let_bit_array_function(
                BitArrayFunctionLocalId(5),
                "f".into(),
                bit_array_function_ref(
                    0,
                    [crate::plan::LocalId::BitArray(
                        crate::plan::BitArrayLocalId(0)
                    )],
                )
                .into(),
            ),
        );
        assert_eq!(
            let_utf_codepoint_function_step(
                6,
                "f",
                utf_codepoint_function_ref(
                    0,
                    [crate::plan::LocalId::UtfCodepoint(
                        crate::plan::UtfCodepointLocalId(0),
                    )],
                ),
            ),
            Step::let_utf_codepoint_function(
                UtfCodepointFunctionLocalId(6),
                "f".into(),
                utf_codepoint_function_ref(
                    0,
                    [crate::plan::LocalId::UtfCodepoint(
                        crate::plan::UtfCodepointLocalId(0),
                    )],
                )
                .into(),
            ),
        );
        assert_eq!(
            let_float_function_step(
                2,
                "f",
                float_function_ref(
                    0,
                    [crate::plan::LocalId::Float(crate::plan::FloatLocalId(0))]
                ),
            ),
            Step::let_float_function(
                FloatFunctionLocalId(2),
                "f".into(),
                float_function_ref(
                    0,
                    [crate::plan::LocalId::Float(crate::plan::FloatLocalId(0))]
                )
                .into(),
            ),
        );
        assert_eq!(
            let_bool_function_step(
                3,
                "f",
                bool_function_ref(0, [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))]),
            ),
            Step::let_bool_function(
                BoolFunctionLocalId(3),
                "f".into(),
                bool_function_ref(0, [crate::plan::LocalId::Bool(crate::plan::BoolLocalId(0))])
                    .into(),
            ),
        );
        assert_eq!(
            let_nil_function_step(
                4,
                "f",
                nil_function_ref(0, [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))]),
            ),
            Step::let_nil_function(
                NilFunctionLocalId(4),
                "f".into(),
                nil_function_ref(0, [crate::plan::LocalId::Nil(crate::plan::NilLocalId(0))]).into(),
            ),
        );
        assert_eq!(evaluate_step(int(1)), Step::evaluate(Expr::from(int(1))));
    }
}
