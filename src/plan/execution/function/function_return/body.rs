use super::super::body::ProfiledFunctionBody;
use super::{
    BitArrayFunctionFunctionId, BoolFunctionFunctionId, ExternalListFunctionFunctionId,
    FloatFunctionFunctionId, GenericFunctionFunctionId, IntFunctionFunctionId,
    NeverFunctionFunctionId, NilFunctionFunctionId, ProfiledListFunctionFunctionId,
    StringFunctionFunctionId, TupleFunctionFunctionId, UtfCodepointFunctionFunctionId,
};
use crate::plan::FunctionCallTarget;
use crate::plan::execution::function::{
    ExecutionGraphProfile, ExecutionProfile, FunctionBodyOwner, HostedExecutionGraph,
};
use crate::plan::execution::graph::{
    BitArrayFunctionLocalId, BoolFunctionLocalId, CustomFunctionLocal, ExternalFunctionLocal,
    FloatFunctionLocalId, FunctionFunctionLocal, GenericFunctionLocal, IntFunctionLocalId,
    ListFunctionLocal, NeverFunctionLocal, NilFunctionLocalId, StringFunctionLocalId,
    TupleFunctionLocalId, UtfCodepointFunctionLocalId,
};
use crate::plan::execution::type_::{
    CustomFunctionType, ExternalFunctionType, FunctionFunctionType, FunctionShape,
};
use std::convert::Infallible;

pub(crate) type ProfiledIntFunctionFunctionBody<Graph> = TypedFunctionBody<
    ProfiledFunctionBody<IntFunctionLocalId, FunctionCallTarget<IntFunctionFunctionId>, Graph>,
>;
pub(crate) type ProfiledFloatFunctionFunctionBody<Graph> = TypedFunctionBody<
    ProfiledFunctionBody<FloatFunctionLocalId, FunctionCallTarget<FloatFunctionFunctionId>, Graph>,
>;
pub(crate) type ProfiledStringFunctionFunctionBody<Graph> = TypedFunctionBody<
    ProfiledFunctionBody<
        StringFunctionLocalId,
        FunctionCallTarget<StringFunctionFunctionId>,
        Graph,
    >,
>;
pub(crate) type ProfiledBitArrayFunctionFunctionBody<Graph> = TypedFunctionBody<
    ProfiledFunctionBody<
        BitArrayFunctionLocalId,
        FunctionCallTarget<BitArrayFunctionFunctionId>,
        Graph,
    >,
>;
pub(crate) type ProfiledUtfCodepointFunctionFunctionBody<Graph> = TypedFunctionBody<
    ProfiledFunctionBody<
        UtfCodepointFunctionLocalId,
        FunctionCallTarget<UtfCodepointFunctionFunctionId>,
        Graph,
    >,
>;
pub(crate) type ProfiledGenericFunctionFunctionBody<Graph> = TypedFunctionBody<
    ProfiledFunctionBody<
        GenericFunctionLocal,
        FunctionCallTarget<GenericFunctionFunctionId>,
        Graph,
    >,
>;
pub(crate) type ProfiledNeverFunctionFunctionBody<Graph> = TypedFunctionBody<
    ProfiledFunctionBody<NeverFunctionLocal, FunctionCallTarget<NeverFunctionFunctionId>, Graph>,
>;
pub(crate) type ProfiledBoolFunctionFunctionBody<Graph> = TypedFunctionBody<
    ProfiledFunctionBody<BoolFunctionLocalId, FunctionCallTarget<BoolFunctionFunctionId>, Graph>,
>;
pub(crate) type ProfiledNilFunctionFunctionBody<Graph> = TypedFunctionBody<
    ProfiledFunctionBody<NilFunctionLocalId, FunctionCallTarget<NilFunctionFunctionId>, Graph>,
>;
pub(crate) type ProfiledTupleFunctionFunctionBody<Graph> = TypedFunctionBody<
    ProfiledFunctionBody<TupleFunctionLocalId, FunctionCallTarget<TupleFunctionFunctionId>, Graph>,
>;
pub(crate) type ProfiledCoreListFunctionFunctionBody<Graph> = TypedFunctionBody<
    ProfiledFunctionBody<
        ListFunctionLocal,
        FunctionCallTarget<ProfiledListFunctionFunctionId<Infallible>>,
        Graph,
    >,
>;
pub(crate) type ProfiledExternalListFunctionFunctionBody<Graph> = TypedFunctionBody<
    ProfiledFunctionBody<
        ListFunctionLocal,
        FunctionCallTarget<ExternalListFunctionFunctionId>,
        Graph,
    >,
>;

pub(crate) type IntFunctionFunctionBody = ProfiledIntFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type FloatFunctionFunctionBody = ProfiledFloatFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type StringFunctionFunctionBody =
    ProfiledStringFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type BitArrayFunctionFunctionBody =
    ProfiledBitArrayFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type UtfCodepointFunctionFunctionBody =
    ProfiledUtfCodepointFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type GenericFunctionFunctionBody =
    ProfiledGenericFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type NeverFunctionFunctionBody = ProfiledNeverFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type BoolFunctionFunctionBody = ProfiledBoolFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type NilFunctionFunctionBody = ProfiledNilFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type TupleFunctionFunctionBody = ProfiledTupleFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type CoreListFunctionFunctionBody =
    ProfiledCoreListFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type ExternalListFunctionFunctionBody =
    ProfiledExternalListFunctionFunctionBody<HostedExecutionGraph>;

pub(crate) type ExecutionIntFunctionFunctionBody<Profile> =
    ProfiledIntFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionFloatFunctionFunctionBody<Profile> =
    ProfiledFloatFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionStringFunctionFunctionBody<Profile> =
    ProfiledStringFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionBitArrayFunctionFunctionBody<Profile> =
    ProfiledBitArrayFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionUtfCodepointFunctionFunctionBody<Profile> =
    ProfiledUtfCodepointFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionGenericFunctionFunctionBody<Profile> =
    ProfiledGenericFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionNeverFunctionFunctionBody<Profile> =
    ProfiledNeverFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionBoolFunctionFunctionBody<Profile> =
    ProfiledBoolFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionNilFunctionFunctionBody<Profile> =
    ProfiledNilFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionTupleFunctionFunctionBody<Profile> =
    ProfiledTupleFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionCoreListFunctionFunctionBody<Profile> =
    ProfiledCoreListFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionExternalListFunctionFunctionBody<Profile> =
    ProfiledExternalListFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;

pub(crate) struct ProfiledCustomFunctionFunctionBody<Graph: ExecutionGraphProfile> {
    _shape: FunctionShape,
    _type: CustomFunctionType,
    body: ProfiledFunctionBody<CustomFunctionLocal, FunctionCallTarget<usize>, Graph>,
}

pub(crate) struct ProfiledExternalFunctionFunctionBody<Graph: ExecutionGraphProfile> {
    _shape: FunctionShape,
    _type: ExternalFunctionType,
    body: ProfiledFunctionBody<ExternalFunctionLocal, FunctionCallTarget<usize>, Graph>,
}

pub(crate) struct ProfiledFunctionFunctionFunctionBody<Graph: ExecutionGraphProfile> {
    _shape: FunctionShape,
    _type: FunctionFunctionType,
    body: ProfiledFunctionBody<FunctionFunctionLocal, FunctionCallTarget<usize>, Graph>,
}

pub(crate) type CustomFunctionFunctionBody =
    ProfiledCustomFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type ExternalFunctionFunctionBody =
    ProfiledExternalFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type FunctionFunctionFunctionBody =
    ProfiledFunctionFunctionFunctionBody<HostedExecutionGraph>;
pub(crate) type ExecutionCustomFunctionFunctionBody<Profile> =
    ProfiledCustomFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionExternalFunctionFunctionBody<Profile> =
    ProfiledExternalFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionFunctionFunctionFunctionBody<Profile> =
    ProfiledFunctionFunctionFunctionBody<<Profile as ExecutionProfile>::Graph>;

pub(crate) struct TypedFunctionBody<Body> {
    _shape: FunctionShape,
    body: Body,
}

impl<Graph: ExecutionGraphProfile> ProfiledCustomFunctionFunctionBody<Graph> {
    pub(in crate::plan::execution) fn from_parts(
        shape: FunctionShape,
        type_: CustomFunctionType,
        body: ProfiledFunctionBody<CustomFunctionLocal, FunctionCallTarget<usize>, Graph>,
    ) -> Self {
        Self {
            _shape: shape,
            _type: type_,
            body,
        }
    }

    pub(in crate::plan::execution) fn into_parts(
        self,
    ) -> (
        FunctionShape,
        CustomFunctionType,
        ProfiledFunctionBody<CustomFunctionLocal, FunctionCallTarget<usize>, Graph>,
    ) {
        (self._shape, self._type, self.body)
    }

    #[cfg(test)]
    pub(crate) fn function_body(
        &self,
    ) -> &ProfiledFunctionBody<CustomFunctionLocal, FunctionCallTarget<usize>, Graph> {
        &self.body
    }
}

impl<Graph: ExecutionGraphProfile> ProfiledFunctionFunctionFunctionBody<Graph> {
    pub(in crate::plan::execution) fn from_parts(
        shape: FunctionShape,
        type_: FunctionFunctionType,
        body: ProfiledFunctionBody<FunctionFunctionLocal, FunctionCallTarget<usize>, Graph>,
    ) -> Self {
        Self {
            _shape: shape,
            _type: type_,
            body,
        }
    }

    pub(in crate::plan::execution) fn into_parts(
        self,
    ) -> (
        FunctionShape,
        FunctionFunctionType,
        ProfiledFunctionBody<FunctionFunctionLocal, FunctionCallTarget<usize>, Graph>,
    ) {
        (self._shape, self._type, self.body)
    }

    #[cfg(test)]
    pub(crate) fn type_(&self) -> &FunctionFunctionType {
        &self._type
    }

    #[cfg(test)]
    pub(crate) fn function_body(
        &self,
    ) -> &ProfiledFunctionBody<FunctionFunctionLocal, FunctionCallTarget<usize>, Graph> {
        &self.body
    }
}

impl<Graph: ExecutionGraphProfile> ProfiledExternalFunctionFunctionBody<Graph> {
    pub(in crate::plan::execution) fn from_parts(
        shape: FunctionShape,
        type_: ExternalFunctionType,
        body: ProfiledFunctionBody<ExternalFunctionLocal, FunctionCallTarget<usize>, Graph>,
    ) -> Self {
        Self {
            _shape: shape,
            _type: type_,
            body,
        }
    }
}

impl<Body> TypedFunctionBody<Body> {
    pub(in crate::plan::execution) fn new(shape: FunctionShape, body: Body) -> Self {
        Self {
            _shape: shape,
            body,
        }
    }

    pub(in crate::plan::execution) fn into_parts(self) -> (FunctionShape, Body) {
        (self._shape, self.body)
    }

    #[cfg(test)]
    pub(crate) fn function_body(&self) -> &Body {
        &self.body
    }
}

impl<Body> FunctionBodyOwner for TypedFunctionBody<Body>
where
    Body: FunctionBodyOwner,
{
    type Return = Body::Return;
    type TailCall = Body::TailCall;
    type Graph = Body::Graph;

    fn function_body(&self) -> &ProfiledFunctionBody<Self::Return, Self::TailCall, Self::Graph> {
        self.body.function_body()
    }
}

impl<Graph: ExecutionGraphProfile> FunctionBodyOwner for ProfiledCustomFunctionFunctionBody<Graph> {
    type Return = CustomFunctionLocal;
    type TailCall = FunctionCallTarget<usize>;
    type Graph = Graph;

    fn function_body(&self) -> &ProfiledFunctionBody<Self::Return, Self::TailCall, Self::Graph> {
        &self.body
    }
}

impl<Graph: ExecutionGraphProfile> FunctionBodyOwner
    for ProfiledExternalFunctionFunctionBody<Graph>
{
    type Return = ExternalFunctionLocal;
    type TailCall = FunctionCallTarget<usize>;
    type Graph = Graph;

    fn function_body(&self) -> &ProfiledFunctionBody<Self::Return, Self::TailCall, Self::Graph> {
        &self.body
    }
}

impl<Graph: ExecutionGraphProfile> FunctionBodyOwner
    for ProfiledFunctionFunctionFunctionBody<Graph>
{
    type Return = FunctionFunctionLocal;
    type TailCall = FunctionCallTarget<usize>;
    type Graph = Graph;

    fn function_body(&self) -> &ProfiledFunctionBody<Self::Return, Self::TailCall, Self::Graph> {
        &self.body
    }
}

#[cfg(test)]
mod explain_tests {
    use super::{FunctionBodyOwner, ProfiledExternalFunctionFunctionBody, TypedFunctionBody};
    use crate::plan::execution::explain;
    use crate::plan::execution::function::{
        FunctionExit, HostedExecutionGraph, ProfiledCoreRuntimeFunctionId, ProfiledFunctionBody,
        ProfiledFunctionFunctionId, ProfiledRuntimeFunctionId,
    };
    use crate::plan::execution::graph::{
        BlockGraphExitId, BlockId, ExternalFunctionLocal, ExternalFunctionLocalId, ProfiledBlock,
        ProfiledBlockGraph, Terminator,
    };
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::execution::type_::{
        ExternalFunctionType, ExternalTypeId, FunctionShape, FunctionType, ValueShapeId, ValueType,
    };
    use std::convert::Infallible;

    type FunctionFunctionId = ProfiledFunctionFunctionId<Infallible>;
    type CoreRuntimeFunctionId = ProfiledCoreRuntimeFunctionId<Infallible>;
    type RuntimeFunctionId = ProfiledRuntimeFunctionId<Infallible>;

    #[test]
    fn exposes_every_typed_function_return_body() {
        let sources = [
            "pub fn main() -> fn() -> Int { fn() { 1 } }",
            "pub fn main() -> fn() -> Float { fn() { 1.0 } }",
            "pub fn main() -> fn() -> String { fn() { \"one\" } }",
            "pub fn main() -> fn() -> BitArray { fn() { <<1>> } }",
            "pub fn main() -> fn() -> UtfCodepoint { fn() { panic } }",
            "pub fn main() -> fn(value) -> value { fn(value) { value } }",
            "pub fn main() -> fn() -> value { fn() { panic } }",
            "pub fn main() -> fn() -> Bool { fn() { True } }",
            "pub fn main() -> fn() -> Nil { fn() { Nil } }",
            "pub fn main() -> fn() -> #(Int) { fn() { #(1) } }",
            "pub fn main() -> fn() -> List(Int) { fn() { [] } }",
        ];

        for source in sources {
            assert_function_body_owner(source);
        }
    }

    #[test]
    fn exposes_custom_and_function_return_bodies() {
        let sources = [
            r#"
pub type Boxed { Boxed(Int) }
pub fn main() -> fn() -> Boxed { fn() { Boxed(1) } }
"#,
            "pub fn main() -> fn() -> fn() -> Int { fn() { fn() { 1 } } }",
        ];

        for source in sources {
            assert_function_body_owner(source);
        }
    }

    #[test]
    #[should_panic(expected = "source should lower a function-returning main function")]
    fn function_body_owner_shape_guard_is_visible() {
        assert_function_body_owner("pub fn main() { 1 }");
    }

    #[test]
    fn external_function_function_body_exposes_its_owned_body() {
        let external_type = ExternalTypeId::new(0);
        let function_type = FunctionType::new(Vec::new(), ValueType::External(external_type));
        let external_function_type =
            ExternalFunctionType::from_shapes(function_type.clone(), Vec::new(), external_type);
        let expected_return =
            ExternalFunctionLocal::new(ExternalFunctionLocalId(0), external_function_type.clone());
        let body = ProfiledFunctionBody::from_parts(
            ProfiledBlockGraph::from_parts(
                BlockId::new(0),
                vec![ProfiledBlock::<HostedExecutionGraph>::new(
                    Vec::new(),
                    Vec::new(),
                    Terminator::Exit(BlockGraphExitId::new(0)),
                )],
            ),
            vec![FunctionExit::Return(expected_return.clone())],
        );
        let owner = ProfiledExternalFunctionFunctionBody::from_parts(
            FunctionShape::new(ValueShapeId::new(0), function_type),
            external_function_type,
            body,
        );

        let actual = FunctionBodyOwner::function_body(&owner);

        assert!(std::ptr::eq(actual, &owner.body));
        assert_eq!(actual.block_graph().blocks().len(), 1);
    }

    fn assert_typed_body_owner<Body>(owner: &TypedFunctionBody<Body>)
    where
        Body: FunctionBodyOwner,
    {
        let actual = <TypedFunctionBody<Body> as FunctionBodyOwner>::function_body(owner);
        let expected = FunctionBodyOwner::function_body(owner.function_body());
        assert!(std::ptr::eq(actual, expected));
    }

    fn assert_function_body_owner(source: &str) {
        explain::with_execution_plan(source, |plan| {
            let RuntimeFunctionId::Core(CoreRuntimeFunctionId::Function { id, .. }) =
                RuntimeExecutionPlan::main_runtime(plan)
            else {
                panic!("source should lower a function-returning main function");
            };
            assert_function_body_id_owner(plan, id);
        });
    }

    fn assert_function_body_id_owner(plan: &crate::ExecutionPlan, id: FunctionFunctionId) {
        match id {
            FunctionFunctionId::Generic(id) => {
                assert_typed_body_owner(plan.generic_function_function(&id).body());
            }
            FunctionFunctionId::Never(id) => {
                assert_typed_body_owner(plan.never_function_function(&id).body());
            }
            FunctionFunctionId::Int(id) => {
                assert_typed_body_owner(plan.int_function_function(id).body());
            }
            FunctionFunctionId::Float(id) => {
                assert_typed_body_owner(plan.float_function_function(id).body());
            }
            FunctionFunctionId::String(id) => {
                assert_typed_body_owner(plan.string_function_function(id).body());
            }
            FunctionFunctionId::BitArray(id) => {
                assert_typed_body_owner(plan.bit_array_function_function(id).body());
            }
            FunctionFunctionId::UtfCodepoint(id) => {
                assert_typed_body_owner(plan.utf_codepoint_function_function(id).body());
            }
            FunctionFunctionId::Custom(id) => {
                let owner = plan.custom_function_function(&id).body();
                let actual = FunctionBodyOwner::function_body(owner);
                assert!(std::ptr::eq(actual, owner.function_body()));
            }
            FunctionFunctionId::Bool(id) => {
                assert_typed_body_owner(plan.bool_function_function(id).body());
            }
            FunctionFunctionId::Nil(id) => {
                assert_typed_body_owner(plan.nil_function_function(id).body());
            }
            FunctionFunctionId::Tuple(id) => {
                assert_typed_body_owner(plan.tuple_function_function(id).body());
            }
            FunctionFunctionId::List(id) => {
                assert_typed_body_owner(plan.core_list_function_function(&id).body());
            }
            FunctionFunctionId::Function(id) => {
                let owner = plan.function_function_function(&id).body();
                let actual = FunctionBodyOwner::function_body(owner);
                assert!(std::ptr::eq(actual, owner.function_body()));
            }
        }
    }
}
