use super::frame::FrameLayout;
use crate::plan::{
    BoolExpr, BoolFunctionExpr, BoolFunctionFunctionId, BoolFunctionId, BoolFunctionLocalId,
    BoolFunctionValue, BoolLocalId, CallArg, CaptureArg, Expr, FunctionExpr, FunctionFunctionExpr,
    FunctionFunctionFunctionId, FunctionFunctionId, FunctionFunctionLocalId, FunctionType, IntExpr,
    IntFunctionExpr, IntFunctionFunctionId, IntFunctionId, IntFunctionLocalId, IntFunctionValue,
    IntLocalId, NilExpr, NilFunctionExpr, NilFunctionFunctionId, NilFunctionId, NilFunctionLocalId,
    NilFunctionValue, NilLocalId, ParamLocal, ReturnBody, ReturnExpr, Step, StringExpr,
    StringFunctionExpr, StringFunctionFunctionId, StringFunctionId, StringFunctionLocalId,
    StringFunctionValue, StringLocalId, ValueType,
};

#[test]
fn frame_layout_derived_surface_is_covered() {
    let layout = FrameLayout::default();
    let cloned = clone_value(&layout);

    assert_eq!(layout, cloned);
    assert_eq!(
        format!("{layout:?}"),
        "FrameLayout { ints: 0, strings: 0, bools: 0, nils: 0, int_functions: 0, string_functions: 0, bool_functions: 0, nil_functions: 0, function_functions: 0 }",
    );
}

fn clone_value<T: Clone>(value: &T) -> T {
    value.clone()
}

#[test]
fn frame_layout_includes_local_ids() {
    let mut layout = FrameLayout::default();

    layout.include_local(&ParamLocal::int(IntLocalId(1)));
    layout.include_local(&ParamLocal::string(StringLocalId(2)));
    layout.include_local(&ParamLocal::bool(BoolLocalId(3)));
    layout.include_local(&ParamLocal::nil(NilLocalId(4)));
    layout.include_int_function(IntFunctionLocalId(5));
    layout.include_nil_function(NilFunctionLocalId(6));

    assert_eq!(layout.ints(), 2);
    assert_eq!(layout.strings(), 3);
    assert_eq!(layout.bools(), 4);
    assert_eq!(layout.nils(), 5);
    assert_eq!(layout.int_functions(), 6);
    assert_eq!(layout.nil_functions(), 7);
}

#[test]
fn frame_layout_includes_function_expression_nested_locals() {
    let nested_block = IntFunctionExpr::block(
        vec![Step::evaluate(Expr::int(IntExpr::local_get(
            IntLocalId(4),
            "value".into(),
        )))],
        int_function_expr(),
    );
    let nested_case = IntFunctionExpr::int_case(
        IntExpr::local_get(IntLocalId(3), "subject".into()),
        vec![(1.into(), nested_block)],
        int_function_expr(),
    );
    let function_case = IntFunctionExpr::bool_case(
        BoolExpr::local_get(BoolLocalId(2), "flag".into()),
        int_function_expr(),
        nested_case,
    );
    let steps = vec![Step::evaluate(Expr::function(FunctionExpr::int(
        IntFunctionExpr::block(
            vec![Step::evaluate(Expr::function(FunctionExpr::int(
                function_case,
            )))],
            int_function_expr(),
        ),
    )))];
    let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

    let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

    assert_eq!(layout.ints(), 5);
    assert_eq!(layout.bools(), 3);
}

#[test]
fn frame_layout_includes_function_local_storage() {
    let steps = vec![Step::let_int_function(
        IntFunctionLocalId(1),
        "f".into(),
        IntFunctionExpr::local_get(
            IntFunctionLocalId(2),
            "g".into(),
            int_function_expr().type_().clone(),
        ),
    )];
    let return_ = ReturnExpr::int(
        IntFunctionId(0),
        IntExpr::function_call(
            IntFunctionExpr::local_get(
                IntFunctionLocalId(3),
                "h".into(),
                int_function_expr().type_().clone(),
            ),
            Vec::new(),
        ),
    );

    let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

    assert_eq!(layout.int_functions(), 4);
}

#[test]
fn frame_layout_includes_return_body_locals() {
    let return_ = ReturnExpr::int_body(
        IntFunctionId(0),
        ReturnBody::block(
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(1),
                "step".into(),
            )))],
            ReturnBody::bool_case(
                BoolExpr::local_get(BoolLocalId(2), "flag".into()),
                ReturnBody::int_case(
                    IntExpr::local_get(IntLocalId(3), "subject".into()),
                    vec![(
                        1.into(),
                        ReturnBody::tail_call(
                            IntFunctionId(1),
                            vec![CallArg::int(
                                IntLocalId(4),
                                IntExpr::local_get(IntLocalId(5), "arg".into()),
                            )],
                        ),
                    )],
                    ReturnBody::expr(IntExpr::local_get(IntLocalId(6), "fallback".into())),
                ),
                ReturnBody::expr(IntExpr::local_get(IntLocalId(7), "false".into())),
            ),
        ),
    );

    let layout = FrameLayout::from_function_parts(&[], &[], &return_);

    assert_eq!(layout.ints(), 8);
    assert_eq!(layout.bools(), 3);

    let function_return = ReturnExpr::int_function_body(
        IntFunctionFunctionId(0),
        int_function_expr().type_().clone(),
        ReturnBody::block(
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(8),
                "function_step".into(),
            )))],
            ReturnBody::bool_case(
                BoolExpr::local_get(BoolLocalId(4), "function_flag".into()),
                ReturnBody::int_case(
                    IntExpr::local_get(IntLocalId(9), "function_subject".into()),
                    vec![(
                        1.into(),
                        ReturnBody::tail_call(IntFunctionFunctionId(1), Vec::new()),
                    )],
                    ReturnBody::expr(IntFunctionExpr::local_get(
                        IntFunctionLocalId(6),
                        "function_fallback".into(),
                        int_function_expr().type_().clone(),
                    )),
                ),
                ReturnBody::expr(IntFunctionExpr::local_get(
                    IntFunctionLocalId(7),
                    "function_false".into(),
                    int_function_expr().type_().clone(),
                )),
            ),
        ),
    );

    let layout = FrameLayout::from_function_parts(&[], &[], &function_return);

    assert_eq!(layout.ints(), 10);
    assert_eq!(layout.bools(), 5);
    assert_eq!(layout.int_functions(), 8);

    let string_function_return = ReturnExpr::string_function_body(
        StringFunctionFunctionId(0),
        string_function_expr().type_().clone(),
        ReturnBody::block(
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(10),
                "string_function_step".into(),
            )))],
            ReturnBody::bool_case(
                BoolExpr::local_get(BoolLocalId(5), "string_function_flag".into()),
                ReturnBody::int_case(
                    IntExpr::local_get(IntLocalId(14), "string_function_subject".into()),
                    vec![(
                        1.into(),
                        ReturnBody::tail_call(StringFunctionFunctionId(1), Vec::new()),
                    )],
                    ReturnBody::expr(StringFunctionExpr::local_get(
                        StringFunctionLocalId(4),
                        "string_function_true".into(),
                        string_function_expr().type_().clone(),
                    )),
                ),
                ReturnBody::expr(StringFunctionExpr::local_get(
                    StringFunctionLocalId(5),
                    "string_function_false".into(),
                    string_function_expr().type_().clone(),
                )),
            ),
        ),
    );
    let layout = FrameLayout::from_function_parts(&[], &[], &string_function_return);
    assert_eq!(layout.ints(), 15);
    assert_eq!(layout.bools(), 6);
    assert_eq!(layout.string_functions(), 6);

    let bool_function_return = ReturnExpr::bool_function_body(
        BoolFunctionFunctionId(0),
        bool_function_expr().type_().clone(),
        ReturnBody::block(
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(11),
                "bool_function_step".into(),
            )))],
            ReturnBody::bool_case(
                BoolExpr::local_get(BoolLocalId(6), "bool_function_flag".into()),
                ReturnBody::int_case(
                    IntExpr::local_get(IntLocalId(15), "bool_function_subject".into()),
                    vec![(
                        1.into(),
                        ReturnBody::tail_call(BoolFunctionFunctionId(1), Vec::new()),
                    )],
                    ReturnBody::expr(BoolFunctionExpr::local_get(
                        BoolFunctionLocalId(4),
                        "bool_function_true".into(),
                        bool_function_expr().type_().clone(),
                    )),
                ),
                ReturnBody::expr(BoolFunctionExpr::local_get(
                    BoolFunctionLocalId(5),
                    "bool_function_false".into(),
                    bool_function_expr().type_().clone(),
                )),
            ),
        ),
    );
    let layout = FrameLayout::from_function_parts(&[], &[], &bool_function_return);
    assert_eq!(layout.ints(), 16);
    assert_eq!(layout.bools(), 7);
    assert_eq!(layout.bool_functions(), 6);

    let nil_function_return = ReturnExpr::nil_function_body(
        NilFunctionFunctionId(0),
        nil_function_expr().type_().clone(),
        ReturnBody::block(
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(12),
                "nil_function_step".into(),
            )))],
            ReturnBody::bool_case(
                BoolExpr::local_get(BoolLocalId(7), "nil_function_flag".into()),
                ReturnBody::int_case(
                    IntExpr::local_get(IntLocalId(16), "nil_function_subject".into()),
                    vec![(
                        1.into(),
                        ReturnBody::tail_call(NilFunctionFunctionId(1), Vec::new()),
                    )],
                    ReturnBody::expr(NilFunctionExpr::local_get(
                        NilFunctionLocalId(4),
                        "nil_function_true".into(),
                        nil_function_expr().type_().clone(),
                    )),
                ),
                ReturnBody::expr(NilFunctionExpr::local_get(
                    NilFunctionLocalId(5),
                    "nil_function_false".into(),
                    nil_function_expr().type_().clone(),
                )),
            ),
        ),
    );
    let layout = FrameLayout::from_function_parts(&[], &[], &nil_function_return);
    assert_eq!(layout.ints(), 17);
    assert_eq!(layout.bools(), 8);
    assert_eq!(layout.nil_functions(), 6);

    let function_function_type = int_function_expr().type_().clone();
    let function_function_return = ReturnExpr::function_function_body(
        FunctionFunctionFunctionId(0),
        function_function_type.clone(),
        ReturnBody::block(
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(13),
                "function_function_step".into(),
            )))],
            ReturnBody::bool_case(
                BoolExpr::local_get(BoolLocalId(8), "function_function_flag".into()),
                ReturnBody::int_case(
                    IntExpr::local_get(IntLocalId(26), "function_function_subject".into()),
                    vec![(
                        1.into(),
                        ReturnBody::tail_call(FunctionFunctionFunctionId(1), Vec::new()),
                    )],
                    ReturnBody::expr(FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(4),
                        "function_function_fallback".into(),
                        function_function_type.clone(),
                    )),
                ),
                ReturnBody::expr(FunctionFunctionExpr::local_get(
                    FunctionFunctionLocalId(5),
                    "function_function_false".into(),
                    function_function_type,
                )),
            ),
        ),
    );
    let layout = FrameLayout::from_function_parts(&[], &[], &function_function_return);
    assert_eq!(layout.ints(), 27);
    assert_eq!(layout.bools(), 9);
    assert_eq!(layout.function_functions(), 6);
}

#[test]
fn frame_layout_includes_primitive_return_body_families() {
    let string_return = ReturnExpr::string_body(
        StringFunctionId(0),
        ReturnBody::block(
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(20),
                "string_step".into(),
            )))],
            ReturnBody::bool_case(
                BoolExpr::local_get(BoolLocalId(20), "string_flag".into()),
                ReturnBody::int_case(
                    IntExpr::local_get(IntLocalId(21), "string_subject".into()),
                    vec![(
                        1.into(),
                        ReturnBody::expr(StringExpr::local_get(
                            StringLocalId(20),
                            "string_hit".into(),
                        )),
                    )],
                    ReturnBody::expr(StringExpr::local_get(
                        StringLocalId(21),
                        "string_fallback".into(),
                    )),
                ),
                ReturnBody::expr(StringExpr::local_get(
                    StringLocalId(22),
                    "string_false".into(),
                )),
            ),
        ),
    );
    let layout = FrameLayout::from_function_parts(&[], &[], &string_return);
    assert_eq!(layout.ints(), 22);
    assert_eq!(layout.bools(), 21);
    assert_eq!(layout.strings(), 23);

    let bool_return = ReturnExpr::bool_body(
        BoolFunctionId(0),
        ReturnBody::block(
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(22),
                "bool_step".into(),
            )))],
            ReturnBody::bool_case(
                BoolExpr::local_get(BoolLocalId(21), "bool_flag".into()),
                ReturnBody::int_case(
                    IntExpr::local_get(IntLocalId(23), "bool_subject".into()),
                    vec![(
                        1.into(),
                        ReturnBody::expr(BoolExpr::local_get(BoolLocalId(22), "bool_hit".into())),
                    )],
                    ReturnBody::expr(BoolExpr::local_get(BoolLocalId(23), "bool_fallback".into())),
                ),
                ReturnBody::expr(BoolExpr::local_get(BoolLocalId(24), "bool_false".into())),
            ),
        ),
    );
    let layout = FrameLayout::from_function_parts(&[], &[], &bool_return);
    assert_eq!(layout.ints(), 24);
    assert_eq!(layout.bools(), 25);

    let nil_return = ReturnExpr::nil_body(
        NilFunctionId(0),
        ReturnBody::block(
            vec![Step::evaluate(Expr::int(IntExpr::local_get(
                IntLocalId(24),
                "nil_step".into(),
            )))],
            ReturnBody::bool_case(
                BoolExpr::local_get(BoolLocalId(25), "nil_flag".into()),
                ReturnBody::int_case(
                    IntExpr::local_get(IntLocalId(25), "nil_subject".into()),
                    vec![(
                        1.into(),
                        ReturnBody::expr(NilExpr::local_get(NilLocalId(20), "nil_hit".into())),
                    )],
                    ReturnBody::expr(NilExpr::local_get(NilLocalId(21), "nil_fallback".into())),
                ),
                ReturnBody::expr(NilExpr::local_get(NilLocalId(22), "nil_false".into())),
            ),
        ),
    );
    let layout = FrameLayout::from_function_parts(&[], &[], &nil_return);
    assert_eq!(layout.ints(), 26);
    assert_eq!(layout.bools(), 26);
    assert_eq!(layout.nils(), 23);
}

#[test]
fn frame_layout_includes_function_expression_return_families() {
    let steps = vec![
        Step::evaluate(Expr::function(FunctionExpr::string(
            StringFunctionExpr::local_get(
                StringFunctionLocalId(1),
                "string".into(),
                string_function_expr().type_().clone(),
            ),
        ))),
        Step::evaluate(Expr::function(FunctionExpr::bool(
            BoolFunctionExpr::local_get(
                BoolFunctionLocalId(2),
                "bool".into(),
                bool_function_expr().type_().clone(),
            ),
        ))),
        Step::evaluate(Expr::function(FunctionExpr::nil(
            NilFunctionExpr::local_get(
                NilFunctionLocalId(3),
                "nil".into(),
                nil_function_expr().type_().clone(),
            ),
        ))),
    ];
    let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

    let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

    assert_eq!(layout.string_functions(), 2);
    assert_eq!(layout.bool_functions(), 3);
    assert_eq!(layout.nil_functions(), 4);
}

#[test]
fn frame_layout_includes_function_expression_call_and_closure_families() {
    let string_type = string_function_expr().type_().clone();
    let bool_type = bool_function_expr().type_().clone();
    let nil_type = nil_function_expr().type_().clone();
    let function_type = function_returning_int_function_type();

    let string_callee_type = FunctionType::new(
        vec![ValueType::Int],
        ValueType::Function(Box::new(string_type.clone())),
    );
    let bool_callee_type = FunctionType::new(
        vec![ValueType::Int],
        ValueType::Function(Box::new(bool_type.clone())),
    );
    let nil_callee_type = FunctionType::new(
        vec![ValueType::Int],
        ValueType::Function(Box::new(nil_type.clone())),
    );
    let function_callee_type = FunctionType::new(
        vec![ValueType::Int],
        ValueType::Function(Box::new(function_type.clone())),
    );

    let steps = vec![
        Step::evaluate(Expr::function(FunctionExpr::string(
            StringFunctionExpr::closure(
                StringFunctionId(1),
                Vec::new(),
                vec![CaptureArg::int(
                    IntLocalId(0),
                    IntExpr::local_get(IntLocalId(30), "string_closure_capture".into()),
                )],
                string_type.clone(),
            ),
        ))),
        Step::evaluate(Expr::function(FunctionExpr::string(
            StringFunctionExpr::function_call(
                FunctionFunctionExpr::local_get(
                    FunctionFunctionLocalId(20),
                    "string_callee".into(),
                    string_callee_type,
                ),
                vec![CallArg::int(
                    IntLocalId(0),
                    IntExpr::local_get(IntLocalId(31), "string_call_arg".into()),
                )],
                string_type,
            ),
        ))),
        Step::evaluate(Expr::function(FunctionExpr::bool(
            BoolFunctionExpr::closure(
                BoolFunctionId(1),
                Vec::new(),
                vec![CaptureArg::int(
                    IntLocalId(0),
                    IntExpr::local_get(IntLocalId(32), "bool_closure_capture".into()),
                )],
                bool_type.clone(),
            ),
        ))),
        Step::evaluate(Expr::function(FunctionExpr::bool(
            BoolFunctionExpr::function_call(
                FunctionFunctionExpr::local_get(
                    FunctionFunctionLocalId(21),
                    "bool_callee".into(),
                    bool_callee_type,
                ),
                vec![CallArg::int(
                    IntLocalId(0),
                    IntExpr::local_get(IntLocalId(33), "bool_call_arg".into()),
                )],
                bool_type,
            ),
        ))),
        Step::evaluate(Expr::function(FunctionExpr::nil(NilFunctionExpr::closure(
            NilFunctionId(1),
            Vec::new(),
            vec![CaptureArg::int(
                IntLocalId(0),
                IntExpr::local_get(IntLocalId(34), "nil_closure_capture".into()),
            )],
            nil_type.clone(),
        )))),
        Step::evaluate(Expr::function(FunctionExpr::nil(
            NilFunctionExpr::function_call(
                FunctionFunctionExpr::local_get(
                    FunctionFunctionLocalId(22),
                    "nil_callee".into(),
                    nil_callee_type,
                ),
                vec![CallArg::int(
                    IntLocalId(0),
                    IntExpr::local_get(IntLocalId(35), "nil_call_arg".into()),
                )],
                nil_type,
            ),
        ))),
        Step::evaluate(Expr::function(FunctionExpr::function(
            FunctionFunctionExpr::closure(
                FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                Vec::new(),
                vec![CaptureArg::int(
                    IntLocalId(0),
                    IntExpr::local_get(IntLocalId(36), "function_closure_capture".into()),
                )],
                function_type.clone(),
                int_function_expr().type_().clone(),
            ),
        ))),
        Step::evaluate(Expr::function(FunctionExpr::function(
            FunctionFunctionExpr::function_call(
                FunctionFunctionExpr::local_get(
                    FunctionFunctionLocalId(23),
                    "function_callee".into(),
                    function_callee_type,
                ),
                vec![CallArg::int(
                    IntLocalId(0),
                    IntExpr::local_get(IntLocalId(37), "function_call_arg".into()),
                )],
                function_type,
            ),
        ))),
    ];
    let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

    let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

    assert_eq!(layout.ints(), 38);
    assert_eq!(layout.function_functions(), 24);
}

#[test]
fn frame_layout_includes_step_and_function_expression_families() {
    let steps = vec![
        Step::let_int(
            IntLocalId(6),
            "number".into(),
            IntExpr::bool_case(
                BoolExpr::local_get(BoolLocalId(5), "use_true".into()),
                IntExpr::int_case(
                    IntExpr::local_get(IntLocalId(4), "subject".into()),
                    vec![(1.into(), IntExpr::local_get(IntLocalId(3), "hit".into()))],
                    IntExpr::local_get(IntLocalId(2), "miss".into()),
                ),
                IntExpr::local_get(IntLocalId(1), "false_branch".into()),
            ),
        ),
        Step::let_string(
            StringLocalId(1),
            "text".into(),
            StringExpr::block(
                Vec::new(),
                StringExpr::call(crate::plan::StringFunctionId(0), Vec::new()),
            ),
        ),
        Step::let_bool(
            BoolLocalId(1),
            "flag".into(),
            BoolExpr::block(
                Vec::new(),
                BoolExpr::equal(
                    Expr::int(IntExpr::value(1.into())),
                    Expr::int(IntExpr::value(1.into())),
                ),
            ),
        ),
        Step::let_nil(
            NilLocalId(1),
            "none".into(),
            NilExpr::block(
                Vec::new(),
                NilExpr::call(crate::plan::NilFunctionId(0), Vec::new()),
            ),
        ),
        Step::let_string_function(
            StringFunctionLocalId(2),
            "string_fn".into(),
            StringFunctionExpr::bool_case(
                BoolExpr::value(true),
                StringFunctionExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), string_function_expr())],
                    string_function_expr(),
                ),
                StringFunctionExpr::block(Vec::new(), string_function_expr()),
            ),
        ),
        Step::let_bool_function(
            BoolFunctionLocalId(2),
            "bool_fn".into(),
            BoolFunctionExpr::bool_case(
                BoolExpr::value(true),
                BoolFunctionExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), bool_function_expr())],
                    bool_function_expr(),
                ),
                BoolFunctionExpr::block(Vec::new(), bool_function_expr()),
            ),
        ),
        Step::let_nil_function(
            NilFunctionLocalId(2),
            "nil_fn".into(),
            NilFunctionExpr::bool_case(
                BoolExpr::value(true),
                NilFunctionExpr::int_case(
                    IntExpr::value(1.into()),
                    vec![(1.into(), nil_function_expr())],
                    nil_function_expr(),
                ),
                NilFunctionExpr::block(Vec::new(), nil_function_expr()),
            ),
        ),
        Step::evaluate(Expr::string(StringExpr::function_call(
            StringFunctionExpr::local_get(
                StringFunctionLocalId(3),
                "string_fn".into(),
                string_function_expr().type_().clone(),
            ),
            Vec::new(),
        ))),
        Step::evaluate(Expr::bool(BoolExpr::function_call(
            BoolFunctionExpr::local_get(
                BoolFunctionLocalId(3),
                "bool_fn".into(),
                bool_function_expr().type_().clone(),
            ),
            Vec::new(),
        ))),
        Step::evaluate(Expr::nil(NilExpr::function_call(
            NilFunctionExpr::local_get(
                NilFunctionLocalId(3),
                "nil_fn".into(),
                nil_function_expr().type_().clone(),
            ),
            Vec::new(),
        ))),
        Step::evaluate(Expr::int(IntExpr::negate(IntExpr::value(1.into())))),
    ];
    let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

    let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

    assert_eq!(layout.ints(), 7);
    assert_eq!(layout.strings(), 2);
    assert_eq!(layout.bools(), 6);
    assert_eq!(layout.nils(), 2);
    assert_eq!(layout.string_functions(), 4);
    assert_eq!(layout.bool_functions(), 4);
    assert_eq!(layout.nil_functions(), 4);
}

#[test]
fn frame_layout_includes_function_arg_and_capture_families() {
    let returning_function_type = function_returning_int_function_type();
    let steps = vec![
        Step::evaluate(Expr::int(IntExpr::call(
            IntFunctionId(0),
            vec![
                CallArg::string_function(
                    StringFunctionLocalId(1),
                    StringFunctionExpr::local_get(
                        StringFunctionLocalId(7),
                        "string_function_arg".into(),
                        string_function_expr().type_().clone(),
                    ),
                ),
                CallArg::bool_function(
                    BoolFunctionLocalId(1),
                    BoolFunctionExpr::local_get(
                        BoolFunctionLocalId(8),
                        "bool_function_arg".into(),
                        bool_function_expr().type_().clone(),
                    ),
                ),
                CallArg::nil_function(
                    NilFunctionLocalId(1),
                    NilFunctionExpr::local_get(
                        NilFunctionLocalId(9),
                        "nil_function_arg".into(),
                        nil_function_expr().type_().clone(),
                    ),
                ),
                CallArg::function_function(
                    FunctionFunctionLocalId(1),
                    FunctionFunctionExpr::local_get(
                        FunctionFunctionLocalId(10),
                        "function_function_arg".into(),
                        returning_function_type.clone(),
                    ),
                ),
            ],
        ))),
        Step::evaluate(Expr::function(FunctionExpr::function(
            FunctionFunctionExpr::closure(
                FunctionFunctionId::Int(IntFunctionFunctionId(0)),
                Vec::new(),
                vec![
                    CaptureArg::string(
                        StringLocalId(3),
                        StringExpr::local_get(StringLocalId(15), "string_capture".into()),
                    ),
                    CaptureArg::bool(
                        BoolLocalId(3),
                        BoolExpr::local_get(BoolLocalId(16), "bool_capture".into()),
                    ),
                    CaptureArg::nil(
                        NilLocalId(3),
                        NilExpr::local_get(NilLocalId(17), "nil_capture".into()),
                    ),
                    CaptureArg::int_function(
                        IntFunctionLocalId(2),
                        IntFunctionExpr::local_get(
                            IntFunctionLocalId(18),
                            "int_function_capture".into(),
                            int_function_expr().type_().clone(),
                        ),
                    ),
                    CaptureArg::string_function(
                        StringFunctionLocalId(2),
                        StringFunctionExpr::local_get(
                            StringFunctionLocalId(11),
                            "string_function_capture".into(),
                            string_function_expr().type_().clone(),
                        ),
                    ),
                    CaptureArg::bool_function(
                        BoolFunctionLocalId(2),
                        BoolFunctionExpr::local_get(
                            BoolFunctionLocalId(12),
                            "bool_function_capture".into(),
                            bool_function_expr().type_().clone(),
                        ),
                    ),
                    CaptureArg::nil_function(
                        NilFunctionLocalId(2),
                        NilFunctionExpr::local_get(
                            NilFunctionLocalId(13),
                            "nil_function_capture".into(),
                            nil_function_expr().type_().clone(),
                        ),
                    ),
                    CaptureArg::function_function(
                        FunctionFunctionLocalId(2),
                        FunctionFunctionExpr::local_get(
                            FunctionFunctionLocalId(14),
                            "function_function_capture".into(),
                            returning_function_type.clone(),
                        ),
                    ),
                ],
                returning_function_type.clone(),
                int_function_expr().type_().clone(),
            ),
        ))),
    ];
    let return_ = ReturnExpr::int(IntFunctionId(1), IntExpr::value(0.into()));

    let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

    assert_eq!(layout.string_functions(), 12);
    assert_eq!(layout.bool_functions(), 13);
    assert_eq!(layout.nil_functions(), 14);
    assert_eq!(layout.function_functions(), 15);
    assert_eq!(layout.strings(), 16);
    assert_eq!(layout.bools(), 17);
    assert_eq!(layout.nils(), 18);
    assert_eq!(layout.int_functions(), 19);
}

#[test]
fn frame_layout_includes_bool_operator_families() {
    let steps = vec![Step::evaluate(Expr::bool(BoolExpr::and(
        BoolExpr::and(
            BoolExpr::lte_int(
                IntExpr::local_get(IntLocalId(1), "lte_left".into()),
                IntExpr::local_get(IntLocalId(2), "lte_right".into()),
            ),
            BoolExpr::gt_int(
                IntExpr::local_get(IntLocalId(3), "gt_left".into()),
                IntExpr::local_get(IntLocalId(4), "gt_right".into()),
            ),
        ),
        BoolExpr::and(
            BoolExpr::gte_int(
                IntExpr::local_get(IntLocalId(5), "gte_left".into()),
                IntExpr::local_get(IntLocalId(6), "gte_right".into()),
            ),
            BoolExpr::not_equal(
                Expr::int(IntExpr::local_get(IntLocalId(7), "not_equal_left".into())),
                Expr::int(IntExpr::local_get(IntLocalId(8), "not_equal_right".into())),
            ),
        ),
    )))];
    let return_ = ReturnExpr::int(IntFunctionId(0), IntExpr::value(0.into()));

    let layout = FrameLayout::from_function_parts(&[], &steps, &return_);

    assert_eq!(layout.ints(), 9);
}

fn int_function_expr() -> IntFunctionExpr {
    IntFunctionExpr::value(IntFunctionValue::new(
        IntFunctionId(0),
        vec![ParamLocal::int(IntLocalId(0))],
    ))
}

fn string_function_expr() -> StringFunctionExpr {
    StringFunctionExpr::value(StringFunctionValue::new(
        crate::plan::StringFunctionId(0),
        vec![ParamLocal::string(StringLocalId(0))],
    ))
}

fn bool_function_expr() -> BoolFunctionExpr {
    BoolFunctionExpr::value(BoolFunctionValue::new(
        crate::plan::BoolFunctionId(0),
        vec![ParamLocal::bool(BoolLocalId(0))],
    ))
}

fn nil_function_expr() -> NilFunctionExpr {
    NilFunctionExpr::value(NilFunctionValue::new(
        crate::plan::NilFunctionId(0),
        vec![ParamLocal::nil(NilLocalId(0))],
    ))
}

fn function_returning_int_function_type() -> FunctionType {
    FunctionType::new(
        Vec::new(),
        ValueType::Function(Box::new(int_function_expr().type_().clone())),
    )
}
