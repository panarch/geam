mod conversion;
mod function;
mod primitive;

use crate::plan::{
    BitArrayFunctionReturn, BitArrayReturn, BoolFunctionReturn, BoolReturn, CustomFunctionReturn,
    CustomReturn, FloatFunctionReturn, FloatReturn, FunctionFunctionReturn, FunctionShape,
    FunctionType, GenericFunctionReturn, IntFunctionReturn, IntReturn, ListFunctionReturn,
    ListReturn, NilFunctionReturn, NilReturn, ReturnExpr, StringFunctionReturn, StringReturn,
    TupleFunctionReturn, TupleReturn, UtfCodepointFunctionReturn, UtfCodepointReturn, ValueType,
};

pub(crate) use function::*;
pub(crate) use primitive::*;

fn tail_call_instantiation(
    template: usize,
    args: &[crate::plan::CallArg],
    return_shape: crate::plan::ValueShape,
) -> crate::plan::FunctionInstantiation {
    crate::plan::monomorphic_function_instantiation(
        template,
        crate::plan::FunctionShape::new(
            args.iter()
                .map(crate::plan::CallArg::parameter_shape)
                .collect(),
            return_shape,
        ),
    )
}

#[derive(Debug, PartialEq)]
pub(crate) enum FunctionReturn {
    Int(IntReturn),
    String(StringReturn),
    BitArray(BitArrayReturn),
    UtfCodepoint(UtfCodepointReturn),
    Custom(CustomReturn),
    Float(FloatReturn),
    Bool(BoolReturn),
    Nil(NilReturn),
    Tuple {
        type_: Vec<ValueType>,
        body: TupleReturn,
    },
    List(ListReturn),
    GenericFunction {
        shape: FunctionShape,
        body: GenericFunctionReturn,
    },
    IntFunction {
        type_: FunctionType,
        body: IntFunctionReturn,
    },
    StringFunction {
        type_: FunctionType,
        body: StringFunctionReturn,
    },
    BitArrayFunction {
        type_: FunctionType,
        body: BitArrayFunctionReturn,
    },
    UtfCodepointFunction {
        type_: FunctionType,
        body: UtfCodepointFunctionReturn,
    },
    CustomFunction(CustomFunctionReturn),
    FloatFunction {
        type_: FunctionType,
        body: FloatFunctionReturn,
    },
    BoolFunction {
        type_: FunctionType,
        body: BoolFunctionReturn,
    },
    NilFunction {
        type_: FunctionType,
        body: NilFunctionReturn,
    },
    TupleFunction {
        type_: FunctionType,
        body: TupleFunctionReturn,
    },
    ListFunction {
        item_type: ValueType,
        type_: FunctionType,
        body: ListFunctionReturn,
    },
    FunctionFunction(FunctionFunctionReturn),
}

impl FunctionReturn {
    pub(super) fn build(self) -> ReturnExpr {
        match self {
            Self::Int(body) => ReturnExpr::int_body(body),
            Self::String(body) => ReturnExpr::string_body(body),
            Self::BitArray(body) => ReturnExpr::bit_array_body(body),
            Self::UtfCodepoint(body) => ReturnExpr::utf_codepoint_body(body),
            Self::Custom(body) => ReturnExpr::custom_body(body),
            Self::Float(body) => ReturnExpr::float_body(body),
            Self::Bool(body) => ReturnExpr::bool_body(body),
            Self::Nil(body) => ReturnExpr::nil_body(body),
            Self::Tuple { type_, body } => ReturnExpr::tuple_body(type_, body),
            Self::List(ListReturn::Generic {
                item_parameter,
                body,
            }) => ReturnExpr::generic_list_body(item_parameter, body),
            Self::List(ListReturn::Int(body)) => ReturnExpr::int_list_body(body),
            Self::List(ListReturn::String(body)) => ReturnExpr::string_list_body(body),
            Self::List(ListReturn::BitArray(body)) => ReturnExpr::bit_array_list_body(body),
            Self::List(ListReturn::UtfCodepoint(body)) => ReturnExpr::utf_codepoint_list_body(body),
            Self::List(ListReturn::Custom { item_type, body }) => {
                ReturnExpr::custom_list_body(item_type, body)
            }
            Self::List(ListReturn::Float(body)) => ReturnExpr::float_list_body(body),
            Self::List(ListReturn::Bool(body)) => ReturnExpr::bool_list_body(body),
            Self::List(ListReturn::Nil(body)) => ReturnExpr::nil_list_body(body),
            Self::List(ListReturn::Tuple { item_type, body }) => {
                ReturnExpr::tuple_list_body(item_type, body)
            }
            Self::List(ListReturn::ParameterList {
                item_parameter,
                body,
            }) => ReturnExpr::parameter_list_list_body(item_parameter, body),
            Self::List(ListReturn::List { item_shape, body }) => {
                ReturnExpr::list_list_body(item_shape, body)
            }
            Self::List(ListReturn::Function { item_type, body }) => {
                ReturnExpr::function_list_body(item_type, body)
            }
            Self::GenericFunction { shape, body } => {
                ReturnExpr::generic_function_shape_body(shape, body)
            }
            Self::IntFunction { type_, body } => ReturnExpr::int_function_shape_body(
                crate::plan::FunctionShape::from_function_type(type_),
                body,
            ),
            Self::StringFunction { type_, body } => ReturnExpr::string_function_shape_body(
                crate::plan::FunctionShape::from_function_type(type_),
                body,
            ),
            Self::BitArrayFunction { type_, body } => ReturnExpr::bit_array_function_shape_body(
                crate::plan::FunctionShape::from_function_type(type_),
                body,
            ),
            Self::UtfCodepointFunction { type_, body } => {
                ReturnExpr::utf_codepoint_function_shape_body(
                    crate::plan::FunctionShape::from_function_type(type_),
                    body,
                )
            }
            Self::CustomFunction(body) => ReturnExpr::custom_function_shape_body(
                crate::plan::FunctionShape::new(
                    body.type_().argument_shapes().to_vec(),
                    crate::plan::ValueShape::Custom(body.type_().return_().clone()),
                ),
                body,
            ),
            Self::FloatFunction { type_, body } => ReturnExpr::float_function_shape_body(
                crate::plan::FunctionShape::from_function_type(type_),
                body,
            ),
            Self::BoolFunction { type_, body } => ReturnExpr::bool_function_shape_body(
                crate::plan::FunctionShape::from_function_type(type_),
                body,
            ),
            Self::NilFunction { type_, body } => ReturnExpr::nil_function_shape_body(
                crate::plan::FunctionShape::from_function_type(type_),
                body,
            ),
            Self::TupleFunction { type_, body } => ReturnExpr::tuple_function_shape_body(
                crate::plan::FunctionShape::from_function_type(type_),
                body,
            ),
            Self::ListFunction {
                item_type,
                type_,
                body,
            } => ReturnExpr::list_function_shape_body(
                crate::plan::FunctionShape::from_function_type(type_),
                item_type,
                body,
            ),
            Self::FunctionFunction(body) => ReturnExpr::function_function_shape_body(
                crate::plan::FunctionShape::from_function_type(body.type_().to_function_type()),
                body,
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FunctionReturn;
    use crate::plan::module::{CustomListReturn, GenericListReturn, ParameterListListReturn};
    use crate::plan::{
        CustomExpr, CustomFunctionExpr, CustomFunctionLocalId, CustomFunctionReturn, CustomLocalId,
        CustomReturn, CustomType, CustomTypeName, Expr, FunctionFunctionId, FunctionFunctionReturn,
        FunctionShape, FunctionType, GenericFunctionExpr, GenericFunctionReference,
        GenericFunctionReturn, GenericFunctionType, IntFunctionFunctionId, ParamLocal, ReturnBody,
        ReturnExpr, TypeParameterId, ValueShape, ValueType,
    };
    use crate::planner::dsl::expression::{
        bit_array, bit_array_function_ref, bool_, bool_function_ref, float, float_function_ref,
        function_function_ref, int, int_function_ref, list, list_function_ref, local_utf_codepoint,
        nil, nil_function_ref, string, string_function_ref, tuple, tuple_function_ref,
        utf_codepoint_function_ref,
    };

    fn custom_type() -> CustomType {
        CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        )
    }

    #[test]
    fn function_return_build_preserves_every_return_family() {
        assert_eq!(
            FunctionReturn::from(int(1)).build(),
            ReturnExpr::int_body(ReturnBody::expr(int(1).into())),
        );
        assert_eq!(
            FunctionReturn::from(string("value")).build(),
            ReturnExpr::string_body(ReturnBody::expr(string("value").into())),
        );
        assert_eq!(
            FunctionReturn::from(bit_array([])).build(),
            ReturnExpr::bit_array_body(ReturnBody::expr(bit_array([]).into())),
        );
        assert_eq!(
            FunctionReturn::from(local_utf_codepoint(0, "codepoint")).build(),
            ReturnExpr::utf_codepoint_body(ReturnBody::expr(
                local_utf_codepoint(0, "codepoint").into(),
            )),
        );
        assert_eq!(
            FunctionReturn::from(float(1.5)).build(),
            ReturnExpr::float_body(ReturnBody::expr(float(1.5).into())),
        );
        assert_eq!(
            FunctionReturn::from(bool_(true)).build(),
            ReturnExpr::bool_body(ReturnBody::expr(bool_(true).into())),
        );
        assert_eq!(
            FunctionReturn::from(nil()).build(),
            ReturnExpr::nil_body(ReturnBody::expr(nil().into())),
        );
        assert_eq!(
            FunctionReturn::from(tuple([Expr::from(int(1))])).build(),
            ReturnExpr::tuple_body(
                vec![ValueType::Int],
                ReturnBody::expr(tuple([Expr::from(int(1))]).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(list(
                Vec::<Expr>::new(),
                ValueType::Parameter(TypeParameterId(0)),
            ))
            .build(),
            ReturnExpr::generic_list_body(
                TypeParameterId(0),
                GenericListReturn::expr(
                    crate::plan::ListExpr::from(list(
                        Vec::<Expr>::new(),
                        ValueType::Parameter(TypeParameterId(0)),
                    ))
                    .into_generic()
                    .expect("expression should be List(parameter)"),
                ),
            ),
        );
        assert_eq!(
            FunctionReturn::from(list([int(1)], ValueType::Int)).build(),
            ReturnExpr::int_list_body(crate::plan::IntListReturn::expr(
                crate::plan::ListExpr::from(list([int(1)], ValueType::Int))
                    .into_int()
                    .expect("expression should be List(Int)"),
            ),),
        );
        assert_eq!(
            FunctionReturn::from(list(Vec::<Expr>::new(), ValueType::String)).build(),
            ReturnExpr::string_list_body(crate::plan::StringListReturn::expr(
                crate::plan::ListExpr::from(list(Vec::<Expr>::new(), ValueType::String))
                    .into_string()
                    .expect("expression should be List(String)"),
            ),),
        );
        assert_eq!(
            FunctionReturn::from(list(Vec::<Expr>::new(), ValueType::BitArray)).build(),
            ReturnExpr::bit_array_list_body(crate::plan::BitArrayListReturn::expr(
                crate::plan::ListExpr::from(list(Vec::<Expr>::new(), ValueType::BitArray))
                    .into_bit_array()
                    .expect("expression should be List(BitArray)"),
            ),),
        );
        assert_eq!(
            FunctionReturn::from(list(Vec::<Expr>::new(), ValueType::UtfCodepoint)).build(),
            ReturnExpr::utf_codepoint_list_body(crate::plan::UtfCodepointListReturn::expr(
                crate::plan::ListExpr::from(list(Vec::<Expr>::new(), ValueType::UtfCodepoint,))
                    .into_utf_codepoint()
                    .expect("expression should be List(UtfCodepoint)"),
            ),),
        );
        assert_eq!(
            FunctionReturn::from(list(Vec::<Expr>::new(), ValueType::Float)).build(),
            ReturnExpr::float_list_body(crate::plan::FloatListReturn::expr(
                crate::plan::ListExpr::from(list(Vec::<Expr>::new(), ValueType::Float))
                    .into_float()
                    .expect("expression should be List(Float)"),
            ),),
        );
        assert_eq!(
            FunctionReturn::from(list(Vec::<Expr>::new(), ValueType::Bool)).build(),
            ReturnExpr::bool_list_body(crate::plan::BoolListReturn::expr(
                crate::plan::ListExpr::from(list(Vec::<Expr>::new(), ValueType::Bool))
                    .into_bool()
                    .expect("expression should be List(Bool)"),
            ),),
        );
        assert_eq!(
            FunctionReturn::from(list(Vec::<Expr>::new(), ValueType::Nil)).build(),
            ReturnExpr::nil_list_body(crate::plan::NilListReturn::expr(
                crate::plan::ListExpr::from(list(Vec::<Expr>::new(), ValueType::Nil))
                    .into_nil()
                    .expect("expression should be List(Nil)"),
            ),),
        );

        let tuple_item = vec![ValueType::Int];
        assert_eq!(
            FunctionReturn::from(list(
                Vec::<Expr>::new(),
                ValueType::Tuple(tuple_item.clone())
            ))
            .build(),
            ReturnExpr::tuple_list_body(
                tuple_item.clone(),
                crate::plan::TupleListReturn::expr(
                    crate::plan::ListExpr::from(list(
                        Vec::<Expr>::new(),
                        ValueType::Tuple(tuple_item)
                    ))
                    .into_tuple()
                    .expect("expression should be List(Tuple)"),
                ),
            ),
        );

        let list_item = Box::new(ValueType::Int);
        assert_eq!(
            FunctionReturn::from(list(Vec::<Expr>::new(), ValueType::List(list_item.clone())))
                .build(),
            ReturnExpr::list_list_body(
                crate::plan::ValueStorageShape::Int,
                crate::plan::ListListReturn::expr(
                    crate::plan::ListExpr::from(list(
                        Vec::<Expr>::new(),
                        ValueType::List(list_item)
                    ))
                    .into_list()
                    .expect("expression should be List(List)"),
                ),
            ),
        );

        let parameter = TypeParameterId(0);
        let parameter_list_item = Box::new(ValueType::Parameter(parameter));
        assert_eq!(
            FunctionReturn::from(list(
                Vec::<Expr>::new(),
                ValueType::List(parameter_list_item.clone()),
            ))
            .build(),
            ReturnExpr::parameter_list_list_body(
                parameter,
                ParameterListListReturn::expr(
                    crate::plan::ListExpr::from(list(
                        Vec::<Expr>::new(),
                        ValueType::List(parameter_list_item),
                    ))
                    .into_parameter_list()
                    .expect("expression should be List(List(parameter))"),
                ),
            ),
        );

        let function_item = FunctionType::new(Vec::new(), ValueType::Int);
        assert_eq!(
            FunctionReturn::from(list(
                Vec::<Expr>::new(),
                ValueType::Function(Box::new(function_item.clone())),
            ))
            .build(),
            ReturnExpr::function_list_body(
                function_item.clone(),
                crate::plan::FunctionListReturn::expr(
                    crate::plan::ListExpr::from(list(
                        Vec::<Expr>::new(),
                        ValueType::Function(Box::new(function_item)),
                    ))
                    .into_function()
                    .expect("expression should be List(Function)"),
                ),
            ),
        );

        assert_eq!(
            FunctionReturn::from(int_function_ref(0, Vec::<ParamLocal>::new())).build(),
            ReturnExpr::int_function_shape_body(
                FunctionShape::new(Vec::new(), ValueShape::Int),
                ReturnBody::expr(int_function_ref(0, Vec::<ParamLocal>::new()).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(string_function_ref(0, Vec::<ParamLocal>::new())).build(),
            ReturnExpr::string_function_shape_body(
                FunctionShape::new(Vec::new(), ValueShape::String),
                ReturnBody::expr(string_function_ref(0, Vec::<ParamLocal>::new()).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(bit_array_function_ref(0, Vec::<ParamLocal>::new())).build(),
            ReturnExpr::bit_array_function_shape_body(
                FunctionShape::new(Vec::new(), ValueShape::BitArray),
                ReturnBody::expr(bit_array_function_ref(0, Vec::<ParamLocal>::new()).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(utf_codepoint_function_ref(0, Vec::<ParamLocal>::new(),)).build(),
            ReturnExpr::utf_codepoint_function_shape_body(
                FunctionShape::new(Vec::new(), ValueShape::UtfCodepoint),
                ReturnBody::expr(utf_codepoint_function_ref(0, Vec::<ParamLocal>::new()).into(),),
            ),
        );
        assert_eq!(
            FunctionReturn::from(float_function_ref(0, Vec::<ParamLocal>::new())).build(),
            ReturnExpr::float_function_shape_body(
                FunctionShape::new(Vec::new(), ValueShape::Float),
                ReturnBody::expr(float_function_ref(0, Vec::<ParamLocal>::new()).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(bool_function_ref(0, Vec::<ParamLocal>::new())).build(),
            ReturnExpr::bool_function_shape_body(
                FunctionShape::new(Vec::new(), ValueShape::Bool),
                ReturnBody::expr(bool_function_ref(0, Vec::<ParamLocal>::new()).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(nil_function_ref(0, Vec::<ParamLocal>::new())).build(),
            ReturnExpr::nil_function_shape_body(
                FunctionShape::new(Vec::new(), ValueShape::Nil),
                ReturnBody::expr(nil_function_ref(0, Vec::<ParamLocal>::new()).into()),
            ),
        );
        assert_eq!(
            FunctionReturn::from(tuple_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                [ValueType::Int],
            ))
            .build(),
            ReturnExpr::tuple_function_shape_body(
                FunctionShape::new(
                    Vec::new(),
                    ValueShape::Tuple(vec![ValueShape::Int].into_boxed_slice()),
                ),
                ReturnBody::expr(
                    tuple_function_ref(0, Vec::<ParamLocal>::new(), [ValueType::Int]).into(),
                ),
            ),
        );
        assert_eq!(
            FunctionReturn::from(list_function_ref(
                0,
                Vec::<ParamLocal>::new(),
                ValueType::Int
            ))
            .build(),
            ReturnExpr::list_function_shape_body(
                FunctionShape::new(Vec::new(), ValueShape::List(Box::new(ValueShape::Int)),),
                ValueType::Int,
                ReturnBody::expr(
                    list_function_ref(0, Vec::<ParamLocal>::new(), ValueType::Int).into(),
                ),
            ),
        );
        assert_eq!(
            FunctionReturn::from(function_function_ref(
                FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                Vec::<ParamLocal>::new(),
                FunctionType::new(vec![ValueType::Int], ValueType::Int),
            ))
            .build(),
            ReturnExpr::function_function_shape_body(
                FunctionShape::new(
                    Vec::new(),
                    ValueShape::Function(Box::new(FunctionShape::new(
                        vec![ValueShape::Int],
                        ValueShape::Int,
                    ))),
                ),
                FunctionFunctionReturn::expr(
                    function_function_ref(
                        FunctionFunctionId::Int(IntFunctionFunctionId(1)),
                        Vec::<ParamLocal>::new(),
                        FunctionType::new(vec![ValueType::Int], ValueType::Int),
                    )
                    .into(),
                ),
            ),
        );

        let generic_type = GenericFunctionType::new(Vec::new(), TypeParameterId(0));
        let generic_shape = generic_type.shape();
        let generic_function = GenericFunctionExpr::reference(
            GenericFunctionReference::new(crate::plan::monomorphic_function_instantiation(
                2,
                generic_shape.clone(),
            )),
            generic_type,
        );
        assert_eq!(
            FunctionReturn::GenericFunction {
                shape: generic_shape.clone(),
                body: GenericFunctionReturn::expr(generic_function.clone()),
            }
            .build(),
            ReturnExpr::generic_function_shape_body(
                generic_shape,
                GenericFunctionReturn::expr(generic_function),
            ),
        );
    }

    #[test]
    fn function_return_build_preserves_custom_metadata() {
        let custom = CustomExpr::local_get(
            crate::plan::CustomLocal::new(CustomLocalId(0), custom_type()),
            "value".into(),
        );
        assert_eq!(
            FunctionReturn::Custom(CustomReturn::expr(custom.clone())).build(),
            ReturnExpr::custom_body(CustomReturn::expr(custom)),
        );

        let custom_list =
            crate::plan::ListExpr::value(Vec::new(), ValueType::Custom(custom_type()))
                .into_custom()
                .expect("expected custom list");
        assert_eq!(
            FunctionReturn::List(crate::plan::ListReturn::Custom {
                item_type: custom_type(),
                body: CustomListReturn::expr(custom_list.clone()),
            })
            .build(),
            ReturnExpr::custom_list_body(custom_type(), CustomListReturn::expr(custom_list),),
        );

        let function_type = crate::plan::CustomFunctionType::new(Vec::new(), custom_type());
        let custom_function = CustomFunctionExpr::local_get(
            crate::plan::CustomFunctionLocal::new(CustomFunctionLocalId(0), function_type.clone()),
            "function".into(),
        );
        assert_eq!(
            FunctionReturn::CustomFunction(CustomFunctionReturn::expr(custom_function.clone()))
                .build(),
            ReturnExpr::custom_function_shape_body(
                FunctionShape::new(
                    Vec::new(),
                    ValueShape::Custom(crate::plan::CustomValueShape::any(custom_type())),
                ),
                CustomFunctionReturn::expr(custom_function),
            ),
        );
    }
}
