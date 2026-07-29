use super::super::body::FunctionBody;
use super::{
    BitArrayFunctionId, BoolFunctionId, FloatFunctionId, IntFunctionId, NeverFunctionId,
    NilFunctionId, StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::function::FunctionBodyOwner;
use crate::plan::execution::graph::{
    BitArrayLocalId, BoolLocalId, CustomLocal, FloatLocalId, IntLocalId, NilLocalId, StringLocalId,
    TupleLocalId, UtfCodepointLocalId,
};
use crate::plan::execution::type_::CustomValueShape;
use std::convert::Infallible;

pub(crate) type IntFunctionBody =
    FunctionBody<IntLocalId, crate::plan::FunctionCallTarget<IntFunctionId>>;
pub(crate) type NeverFunctionBody =
    FunctionBody<Infallible, crate::plan::FunctionCallTarget<NeverFunctionId>>;
pub(crate) type FloatFunctionBody =
    FunctionBody<FloatLocalId, crate::plan::FunctionCallTarget<FloatFunctionId>>;
pub(crate) type StringFunctionBody =
    FunctionBody<StringLocalId, crate::plan::FunctionCallTarget<StringFunctionId>>;
pub(crate) type BitArrayFunctionBody =
    FunctionBody<BitArrayLocalId, crate::plan::FunctionCallTarget<BitArrayFunctionId>>;
pub(crate) type UtfCodepointFunctionBody =
    FunctionBody<UtfCodepointLocalId, crate::plan::FunctionCallTarget<UtfCodepointFunctionId>>;
pub(crate) type BoolFunctionBody =
    FunctionBody<BoolLocalId, crate::plan::FunctionCallTarget<BoolFunctionId>>;
pub(crate) type NilFunctionBody =
    FunctionBody<NilLocalId, crate::plan::FunctionCallTarget<NilFunctionId>>;
pub(crate) type TupleFunctionBody =
    FunctionBody<TupleLocalId, crate::plan::FunctionCallTarget<TupleFunctionId>>;

pub(crate) struct CustomFunctionBody {
    _signature_shape: CustomValueShape,
    _body_shape: CustomValueShape,
    body: FunctionBody<CustomLocal, crate::plan::FunctionCallTarget<usize>>,
}

impl CustomFunctionBody {
    pub(in crate::plan::execution) fn from_parts(
        signature_shape: CustomValueShape,
        body_shape: CustomValueShape,
        body: FunctionBody<CustomLocal, crate::plan::FunctionCallTarget<usize>>,
    ) -> Self {
        Self {
            _signature_shape: signature_shape,
            _body_shape: body_shape,
            body,
        }
    }

    #[cfg(test)]
    pub(crate) fn body_shape(&self) -> &CustomValueShape {
        &self._body_shape
    }

    #[cfg(test)]
    pub(crate) fn signature_shape(&self) -> &CustomValueShape {
        &self._signature_shape
    }

    #[cfg(test)]
    pub(crate) fn function_body(
        &self,
    ) -> &FunctionBody<CustomLocal, crate::plan::FunctionCallTarget<usize>> {
        &self.body
    }
}

impl FunctionBodyOwner for CustomFunctionBody {
    type Return = CustomLocal;
    type TailCall = crate::plan::FunctionCallTarget<usize>;

    fn function_body(&self) -> &FunctionBody<Self::Return, Self::TailCall> {
        &self.body
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{CustomFunctionBody, FunctionBodyOwner};
    use crate::plan::execution::explain;
    use crate::plan::execution::function::RuntimeFunctionId;

    #[test]
    fn exposes_the_custom_function_body() {
        let source = r#"
pub type Boxed { Boxed(Int) }
pub fn main() { Boxed(1) }
"#;

        explain::with_execution_plan(source, |plan| {
            let owner = custom_function_body(plan);
            let body = <CustomFunctionBody as FunctionBodyOwner>::function_body(owner);

            assert!(std::ptr::eq(body, owner.function_body()));
        });
    }

    #[test]
    #[should_panic(expected = "source should lower a custom-returning main function")]
    fn custom_function_body_shape_guard_is_visible() {
        explain::with_execution_plan("pub fn main() { 1 }", |plan| {
            custom_function_body(plan);
        });
    }

    fn custom_function_body(plan: &crate::plan::execution::ExecutionPlan) -> &CustomFunctionBody {
        let RuntimeFunctionId::Custom(function) = plan.main_runtime() else {
            panic!("source should lower a custom-returning main function");
        };
        plan.custom_function(function).body()
    }
}
