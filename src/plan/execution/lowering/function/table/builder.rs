use super::LoweredFunction;
use super::family::{CoreListFunctionFunctionSignature, CoreListFunctionReturn};
use crate::plan::execution::function::{
    BitArrayFunctionBody, BitArrayFunctionFunctionBody, BitArrayListFunctionBody,
    BitArrayListFunctionId, BoolFunctionBody, BoolFunctionFunctionBody, BoolListFunctionBody,
    BoolListFunctionId, CoreListFunctionFunctionBody, CustomFunctionBody,
    CustomFunctionFunctionBody, CustomListFunctionBody, CustomListFunctionId, ExternalFunctionBody,
    ExternalFunctionFunctionBody, ExternalListFunctionBody, ExternalListFunctionFunctionBody,
    ExternalListFunctionId, FloatFunctionBody, FloatFunctionFunctionBody, FloatListFunctionBody,
    FloatListFunctionId, FunctionFunctionFunctionBody, FunctionListFunctionBody,
    FunctionListFunctionId, GenericFunctionFunctionBody, IntFunctionBody, IntFunctionFunctionBody,
    IntListFunctionBody, IntListFunctionId, ListListFunctionBody, ListListFunctionId,
    NeverFunctionBody, NeverFunctionFunctionBody, NilFunctionBody, NilFunctionFunctionBody,
    NilListFunctionBody, NilListFunctionId, ParameterListFunctionBody, ParameterListFunctionId,
    ParameterListListFunctionBody, ParameterListListFunctionId, StringFunctionBody,
    StringFunctionFunctionBody, StringListFunctionBody, StringListFunctionId, TupleFunctionBody,
    TupleFunctionFunctionBody, TupleListFunctionBody, TupleListFunctionId,
    UtfCodepointFunctionBody, UtfCodepointFunctionFunctionBody, UtfCodepointListFunctionBody,
    UtfCodepointListFunctionId,
};

#[derive(Default)]
pub(in crate::plan::execution::lowering) struct FunctionTableBuilder {
    pub(in crate::plan::execution::lowering::function) never_functions:
        Vec<(usize, LoweredFunction<NeverFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) int_functions:
        Vec<(usize, LoweredFunction<IntFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) float_functions:
        Vec<(usize, LoweredFunction<FloatFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) string_functions:
        Vec<(usize, LoweredFunction<StringFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) bit_array_functions:
        Vec<(usize, LoweredFunction<BitArrayFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) utf_codepoint_functions:
        Vec<(usize, LoweredFunction<UtfCodepointFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) custom_functions:
        Vec<(usize, LoweredFunction<CustomFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) external_functions:
        Vec<(usize, LoweredFunction<ExternalFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) bool_functions:
        Vec<(usize, LoweredFunction<BoolFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) nil_functions:
        Vec<(usize, LoweredFunction<NilFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) tuple_functions:
        Vec<(usize, LoweredFunction<TupleFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) parameter_list_functions: Vec<(
        ParameterListFunctionId,
        LoweredFunction<ParameterListFunctionBody>,
    )>,
    pub(in crate::plan::execution::lowering::function) int_list_functions:
        Vec<(IntListFunctionId, LoweredFunction<IntListFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) string_list_functions: Vec<(
        StringListFunctionId,
        LoweredFunction<StringListFunctionBody>,
    )>,
    pub(in crate::plan::execution::lowering::function) bit_array_list_functions: Vec<(
        BitArrayListFunctionId,
        LoweredFunction<BitArrayListFunctionBody>,
    )>,
    pub(in crate::plan::execution::lowering::function) utf_codepoint_list_functions: Vec<(
        UtfCodepointListFunctionId,
        LoweredFunction<UtfCodepointListFunctionBody>,
    )>,
    pub(in crate::plan::execution::lowering::function) custom_list_functions: Vec<(
        CustomListFunctionId,
        LoweredFunction<CustomListFunctionBody>,
    )>,
    pub(in crate::plan::execution::lowering::function) external_list_functions: Vec<(
        ExternalListFunctionId,
        LoweredFunction<ExternalListFunctionBody>,
    )>,
    pub(in crate::plan::execution::lowering::function) float_list_functions:
        Vec<(FloatListFunctionId, LoweredFunction<FloatListFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) bool_list_functions:
        Vec<(BoolListFunctionId, LoweredFunction<BoolListFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) nil_list_functions:
        Vec<(NilListFunctionId, LoweredFunction<NilListFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) tuple_list_functions:
        Vec<(TupleListFunctionId, LoweredFunction<TupleListFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) parameter_list_list_functions: Vec<(
        ParameterListListFunctionId,
        LoweredFunction<ParameterListListFunctionBody>,
    )>,
    pub(in crate::plan::execution::lowering::function) list_list_functions:
        Vec<(ListListFunctionId, LoweredFunction<ListListFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) function_list_functions: Vec<(
        FunctionListFunctionId,
        LoweredFunction<FunctionListFunctionBody>,
    )>,
    pub(in crate::plan::execution::lowering::function) int_function_functions:
        Vec<(usize, LoweredFunction<IntFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) float_function_functions:
        Vec<(usize, LoweredFunction<FloatFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) string_function_functions:
        Vec<(usize, LoweredFunction<StringFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) bit_array_function_functions:
        Vec<(usize, LoweredFunction<BitArrayFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) utf_codepoint_function_functions:
        Vec<(usize, LoweredFunction<UtfCodepointFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) custom_function_functions:
        Vec<(usize, LoweredFunction<CustomFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) external_function_functions:
        Vec<(usize, LoweredFunction<ExternalFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) bool_function_functions:
        Vec<(usize, LoweredFunction<BoolFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) nil_function_functions:
        Vec<(usize, LoweredFunction<NilFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) tuple_function_functions:
        Vec<(usize, LoweredFunction<TupleFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) generic_function_functions:
        Vec<(usize, LoweredFunction<GenericFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) never_function_functions:
        Vec<(usize, LoweredFunction<NeverFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) parameter_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) parameter_list_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) int_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) string_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) bit_array_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) utf_codepoint_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) custom_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) external_list_function_functions:
        Vec<(usize, LoweredFunction<ExternalListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) float_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) bool_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) nil_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) tuple_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) list_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) function_list_function_functions:
        Vec<(usize, LoweredFunction<CoreListFunctionFunctionBody>)>,
    pub(in crate::plan::execution::lowering::function) function_function_functions:
        Vec<(usize, LoweredFunction<FunctionFunctionFunctionBody>)>,
}

pub(in crate::plan::execution::lowering::function) fn push_core_list_function_function(
    functions: &mut FunctionTableBuilder,
    index: usize,
    signature: &CoreListFunctionFunctionSignature,
    function: LoweredFunction<CoreListFunctionFunctionBody>,
) {
    match signature.return_ {
        CoreListFunctionReturn::Parameter(_) => functions
            .parameter_list_function_functions
            .push((index, function)),
        CoreListFunctionReturn::ParameterList(_) => functions
            .parameter_list_list_function_functions
            .push((index, function)),
        CoreListFunctionReturn::Int(_) => functions
            .int_list_function_functions
            .push((index, function)),
        CoreListFunctionReturn::String(_) => {
            functions
                .string_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::BitArray(_) => {
            functions
                .bit_array_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::UtfCodepoint(_) => {
            functions
                .utf_codepoint_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::Custom(_) => {
            functions
                .custom_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::Float(_) => {
            functions
                .float_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::Bool(_) => {
            functions
                .bool_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::Nil(_) => {
            functions
                .nil_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::Tuple(_) => {
            functions
                .tuple_list_function_functions
                .push((index, function));
        }
        CoreListFunctionReturn::List(_) => functions
            .list_list_function_functions
            .push((index, function)),
        CoreListFunctionReturn::Function(_) => {
            functions
                .function_list_function_functions
                .push((index, function));
        }
    }
}

pub(in crate::plan::execution::lowering::function) fn push_external_list_function_function(
    functions: &mut FunctionTableBuilder,
    index: usize,
    function: LoweredFunction<ExternalListFunctionFunctionBody>,
) {
    functions
        .external_list_function_functions
        .push((index, function));
}

#[cfg(test)]
mod tests {
    use super::{FunctionTableBuilder, push_core_list_function_function};
    use crate::plan::FunctionTemplateId;
    use crate::plan::execution::function::CoreListFunctionFunctionBody;
    use crate::plan::execution::lowering::function::table::family::{
        CoreListFunctionFunctionSignature, CoreListFunctionReturn,
    };
    use crate::plan::execution::lowering::function::table::{
        LoweredFunction, LoweredSpecialization,
    };
    use crate::plan::execution::lowering::specialization::{Representability, SpecializationKey};
    use crate::plan::execution::type_::{FunctionType, IntListTypeId, ListTypeId, ValueType};

    #[test]
    fn routes_core_list_function_functions_to_their_return_family_bucket() {
        let signature = CoreListFunctionFunctionSignature {
            type_: FunctionType::new(Vec::new(), ValueType::Int),
            return_: CoreListFunctionReturn::Int(IntListTypeId::new(ListTypeId::new(0))),
        };
        let specialization = SpecializationKey::monomorphic(FunctionTemplateId::new(4));
        let function: LoweredFunction<CoreListFunctionFunctionBody> = LoweredSpecialization {
            specialization: specialization.clone(),
            value: Representability::Uninhabited,
        };
        let mut functions = FunctionTableBuilder::default();

        push_core_list_function_function(&mut functions, 7, &signature, function);

        assert_eq!(functions.int_list_function_functions.len(), 1);
        let (index, function) = &functions.int_list_function_functions[0];
        assert_eq!(*index, 7);
        assert_eq!(function.specialization, specialization);
        assert_eq!(
            std::mem::discriminant(&function.value),
            std::mem::discriminant(&Representability::Uninhabited),
        );
    }
}
