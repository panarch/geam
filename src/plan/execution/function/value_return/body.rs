use super::super::body::ProfiledFunctionBody;
use super::{
    BitArrayFunctionId, BoolFunctionId, FloatFunctionId, IntFunctionId, NeverFunctionId,
    NilFunctionId, StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::function::{
    ExecutionGraphProfile, ExecutionProfile, FunctionBodyOwner, HostedExecutionGraph,
};
use crate::plan::execution::graph::{
    BitArrayLocalId, BoolLocalId, CustomLocal, ExternalLocal, FloatLocalId, IntLocalId, NilLocalId,
    StringLocalId, TupleLocalId, UtfCodepointLocalId,
};
use crate::plan::execution::type_::{CustomValueShape, ExternalTypeId};
use std::convert::Infallible;

pub(crate) type ProfiledIntFunctionBody<Graph> =
    ProfiledFunctionBody<IntLocalId, crate::plan::FunctionCallTarget<IntFunctionId>, Graph>;
pub(crate) type ProfiledNeverFunctionBody<Graph> =
    ProfiledFunctionBody<Infallible, crate::plan::FunctionCallTarget<NeverFunctionId>, Graph>;
pub(crate) type ProfiledFloatFunctionBody<Graph> =
    ProfiledFunctionBody<FloatLocalId, crate::plan::FunctionCallTarget<FloatFunctionId>, Graph>;
pub(crate) type ProfiledStringFunctionBody<Graph> =
    ProfiledFunctionBody<StringLocalId, crate::plan::FunctionCallTarget<StringFunctionId>, Graph>;
pub(crate) type ProfiledBitArrayFunctionBody<Graph> = ProfiledFunctionBody<
    BitArrayLocalId,
    crate::plan::FunctionCallTarget<BitArrayFunctionId>,
    Graph,
>;
pub(crate) type ProfiledUtfCodepointFunctionBody<Graph> = ProfiledFunctionBody<
    UtfCodepointLocalId,
    crate::plan::FunctionCallTarget<UtfCodepointFunctionId>,
    Graph,
>;
pub(crate) type ProfiledBoolFunctionBody<Graph> =
    ProfiledFunctionBody<BoolLocalId, crate::plan::FunctionCallTarget<BoolFunctionId>, Graph>;
pub(crate) type ProfiledNilFunctionBody<Graph> =
    ProfiledFunctionBody<NilLocalId, crate::plan::FunctionCallTarget<NilFunctionId>, Graph>;
pub(crate) type ProfiledTupleFunctionBody<Graph> =
    ProfiledFunctionBody<TupleLocalId, crate::plan::FunctionCallTarget<TupleFunctionId>, Graph>;

pub(crate) type IntFunctionBody = ProfiledIntFunctionBody<HostedExecutionGraph>;
pub(crate) type NeverFunctionBody = ProfiledNeverFunctionBody<HostedExecutionGraph>;
pub(crate) type FloatFunctionBody = ProfiledFloatFunctionBody<HostedExecutionGraph>;
pub(crate) type StringFunctionBody = ProfiledStringFunctionBody<HostedExecutionGraph>;
pub(crate) type BitArrayFunctionBody = ProfiledBitArrayFunctionBody<HostedExecutionGraph>;
pub(crate) type UtfCodepointFunctionBody = ProfiledUtfCodepointFunctionBody<HostedExecutionGraph>;
pub(crate) type BoolFunctionBody = ProfiledBoolFunctionBody<HostedExecutionGraph>;
pub(crate) type NilFunctionBody = ProfiledNilFunctionBody<HostedExecutionGraph>;
pub(crate) type TupleFunctionBody = ProfiledTupleFunctionBody<HostedExecutionGraph>;

pub(crate) type ExecutionIntFunctionBody<Profile> =
    ProfiledIntFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionNeverFunctionBody<Profile> =
    ProfiledNeverFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionFloatFunctionBody<Profile> =
    ProfiledFloatFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionStringFunctionBody<Profile> =
    ProfiledStringFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionBitArrayFunctionBody<Profile> =
    ProfiledBitArrayFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionUtfCodepointFunctionBody<Profile> =
    ProfiledUtfCodepointFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionBoolFunctionBody<Profile> =
    ProfiledBoolFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionNilFunctionBody<Profile> =
    ProfiledNilFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionTupleFunctionBody<Profile> =
    ProfiledTupleFunctionBody<<Profile as ExecutionProfile>::Graph>;

pub(crate) struct ProfiledCustomFunctionBody<Graph: ExecutionGraphProfile> {
    _signature_shape: CustomValueShape,
    _body_shape: CustomValueShape,
    body: ProfiledFunctionBody<CustomLocal, crate::plan::FunctionCallTarget<usize>, Graph>,
}

pub(crate) struct ProfiledExternalFunctionBody<Graph: ExecutionGraphProfile> {
    _signature_type: ExternalTypeId,
    _body_type: ExternalTypeId,
    body: ProfiledFunctionBody<ExternalLocal, crate::plan::FunctionCallTarget<usize>, Graph>,
}

pub(crate) type CustomFunctionBody = ProfiledCustomFunctionBody<HostedExecutionGraph>;
pub(crate) type ExternalFunctionBody = ProfiledExternalFunctionBody<HostedExecutionGraph>;
pub(crate) type ExecutionCustomFunctionBody<Profile> =
    ProfiledCustomFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionExternalFunctionBody<Profile> =
    ProfiledExternalFunctionBody<<Profile as ExecutionProfile>::Graph>;

impl<Graph: ExecutionGraphProfile> ProfiledCustomFunctionBody<Graph> {
    pub(in crate::plan::execution) fn from_parts(
        signature_shape: CustomValueShape,
        body_shape: CustomValueShape,
        body: ProfiledFunctionBody<CustomLocal, crate::plan::FunctionCallTarget<usize>, Graph>,
    ) -> Self {
        Self {
            _signature_shape: signature_shape,
            _body_shape: body_shape,
            body,
        }
    }

    pub(in crate::plan::execution) fn into_parts(
        self,
    ) -> (
        CustomValueShape,
        CustomValueShape,
        ProfiledFunctionBody<CustomLocal, crate::plan::FunctionCallTarget<usize>, Graph>,
    ) {
        (self._signature_shape, self._body_shape, self.body)
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
    ) -> &ProfiledFunctionBody<CustomLocal, crate::plan::FunctionCallTarget<usize>, Graph> {
        &self.body
    }
}

impl<Graph: ExecutionGraphProfile> FunctionBodyOwner for ProfiledCustomFunctionBody<Graph> {
    type Return = CustomLocal;
    type TailCall = crate::plan::FunctionCallTarget<usize>;
    type Graph = Graph;

    fn function_body(&self) -> &ProfiledFunctionBody<Self::Return, Self::TailCall, Self::Graph> {
        &self.body
    }
}

impl<Graph: ExecutionGraphProfile> ProfiledExternalFunctionBody<Graph> {
    pub(in crate::plan::execution) fn from_parts(
        signature_type: ExternalTypeId,
        body_type: ExternalTypeId,
        body: ProfiledFunctionBody<ExternalLocal, crate::plan::FunctionCallTarget<usize>, Graph>,
    ) -> Self {
        Self {
            _signature_type: signature_type,
            _body_type: body_type,
            body,
        }
    }
}

impl<Graph: ExecutionGraphProfile> FunctionBodyOwner for ProfiledExternalFunctionBody<Graph> {
    type Return = ExternalLocal;
    type TailCall = crate::plan::FunctionCallTarget<usize>;
    type Graph = Graph;

    fn function_body(&self) -> &ProfiledFunctionBody<Self::Return, Self::TailCall, Self::Graph> {
        &self.body
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{ExecutionCustomFunctionBody, FunctionBodyOwner, ProfiledExternalFunctionBody};
    use crate::plan::execution::explain;
    use crate::plan::execution::function::{
        CoreRuntimeFunctionId, FunctionExit, HostedExecutionGraph, ProfiledFunctionBody,
        RuntimeFunctionId,
    };
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockId, ExternalLocal, ExternalLocalId, ProfiledBlock,
        ProfiledBlockGraph, Terminator,
    };
    use crate::plan::execution::type_::ExternalTypeId;

    #[test]
    fn exposes_the_custom_function_body() {
        let source = r#"
pub type Boxed { Boxed(Int) }
pub fn main() { Boxed(1) }
"#;

        explain::with_execution_plan(source, |plan| {
            let owner = custom_function_body(plan);
            let body = FunctionBodyOwner::function_body(owner);

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

    #[test]
    fn external_function_body_exposes_its_owned_body() {
        let type_ = ExternalTypeId::new(0);
        let expected_return = ExternalLocal::new(ExternalLocalId(0), type_);
        let body = ProfiledFunctionBody::from_parts(
            ProfiledBlockGraph::from_parts(
                BlockId::new(0),
                vec![ProfiledBlock::<HostedExecutionGraph>::new(
                    Vec::new(),
                    Vec::new(),
                    Terminator::Exit(BlockGraphExitId::new(0)),
                )],
            ),
            vec![FunctionExit::Return(expected_return)],
        );
        let owner = ProfiledExternalFunctionBody::from_parts(type_, type_, body);

        let actual = FunctionBodyOwner::function_body(&owner);

        assert!(std::ptr::eq(actual, &owner.body));
        assert_eq!(actual.block_graph().blocks().len(), 1);
    }

    fn custom_function_body(
        plan: &crate::plan::execution::ExecutionPlan,
    ) -> &ExecutionCustomFunctionBody<std::convert::Infallible> {
        let RuntimeFunctionId::Core(CoreRuntimeFunctionId::Custom(function)) = plan.main_runtime()
        else {
            panic!("source should lower a custom-returning main function");
        };
        plan.custom_function(function).body()
    }
}
