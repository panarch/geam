use super::{ExecutableFunction, FunctionBodyOwner, ValueFunctionEntry};
use crate::host::HostProfile;
use crate::plan::execution::graph::{
    BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
    BoolListLocalId, BoolLocalId, CustomFunctionLocal, CustomListLocalId, CustomLocal,
    FloatFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionFunctionLocal,
    FunctionListLocalId, GenericFunctionLocal, IntFunctionLocalId, IntListLocalId, IntLocalId,
    ListFunctionLocal, ListListLocalId, NeverFunctionLocal, NilFunctionLocalId, NilListLocalId,
    NilLocalId, ParameterListListLocalId, ParameterListLocalId, StringFunctionLocalId,
    StringListLocalId, StringLocalId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
    UtfCodepointFunctionLocalId, UtfCodepointListLocalId, UtfCodepointLocalId,
};
use crate::plan::execution::host::{
    HostBitArrayFunctionId, HostBoolFunctionId, HostFloatFunctionId, HostIntFunctionId,
    HostNilFunctionId, HostStringFunctionId, HostUtfCodepointFunctionId, HostedExecutionProfile,
};
use std::convert::Infallible;

pub(crate) trait ExecutionFunctionBody: FunctionBodyOwner {
    type HostTarget;
}

pub(crate) enum ExecutionFunctionRef<'function, Body, HostTarget> {
    Graph(&'function ExecutableFunction<Body>),
    Host(&'function HostTarget),
}

pub(crate) trait ExecutionFunctionEntry<Body> {
    type HostTarget;

    fn as_ref(&self) -> ExecutionFunctionRef<'_, Body, Self::HostTarget>;
}

pub(crate) trait ExecutionProfile {
    type RunState;
    type HostTarget<Body: ExecutionFunctionBody>;
    type Function<Body: ExecutionFunctionBody>: ExecutionFunctionEntry<Body, HostTarget = Self::HostTarget<Body>>;

    fn graph<Body: ExecutionFunctionBody>(
        function: ExecutableFunction<Body>,
    ) -> Self::Function<Body>;

    fn graph_only<Body>(function: &Self::Function<Body>) -> &ExecutableFunction<Body>
    where
        Body: ExecutionFunctionBody<HostTarget = Infallible>;
}

pub(crate) type ExecutionFunction<Profile, Body> = <Profile as ExecutionProfile>::Function<Body>;

pub(crate) type ExecutionHostTarget<Profile, Body> =
    <Profile as ExecutionProfile>::HostTarget<Body>;

pub(crate) fn graph_function<Profile, Body>(
    function: &ExecutionFunction<Profile, Body>,
) -> &ExecutableFunction<Body>
where
    Profile: ExecutionProfile,
    Body: ExecutionFunctionBody<HostTarget = Infallible>,
{
    Profile::graph_only(function)
}

pub(crate) trait HostTarget {
    type Target;
}

impl<Body> ExecutionFunctionBody for Body
where
    Body: FunctionBodyOwner,
    Body::Return: HostTarget,
{
    type HostTarget = <Body::Return as HostTarget>::Target;
}

impl ExecutionProfile for Infallible {
    type RunState = ();
    type HostTarget<Body: ExecutionFunctionBody> = Infallible;
    type Function<Body: ExecutionFunctionBody> = ExecutableFunction<Body>;

    fn graph<Body: ExecutionFunctionBody>(
        function: ExecutableFunction<Body>,
    ) -> Self::Function<Body> {
        function
    }

    fn graph_only<Body>(function: &Self::Function<Body>) -> &ExecutableFunction<Body>
    where
        Body: ExecutionFunctionBody<HostTarget = Infallible>,
    {
        function
    }
}

impl<Profile: HostProfile> ExecutionProfile for HostedExecutionProfile<Profile> {
    type RunState = Profile::RunState;
    type HostTarget<Body: ExecutionFunctionBody> = Body::HostTarget;
    type Function<Body: ExecutionFunctionBody> = ValueFunctionEntry<Body, Body::HostTarget>;

    fn graph<Body: ExecutionFunctionBody>(
        function: ExecutableFunction<Body>,
    ) -> Self::Function<Body> {
        ValueFunctionEntry::graph(function)
    }

    fn graph_only<Body>(function: &Self::Function<Body>) -> &ExecutableFunction<Body>
    where
        Body: ExecutionFunctionBody<HostTarget = Infallible>,
    {
        match function {
            ValueFunctionEntry::Graph(function) => function,
            ValueFunctionEntry::Host(target) => match *target {},
        }
    }
}

impl<Body> ExecutionFunctionEntry<Body> for ExecutableFunction<Body> {
    type HostTarget = Infallible;

    fn as_ref(&self) -> ExecutionFunctionRef<'_, Body, Self::HostTarget> {
        ExecutionFunctionRef::Graph(self)
    }
}

impl<Body, HostTarget> ExecutionFunctionEntry<Body> for ValueFunctionEntry<Body, HostTarget> {
    type HostTarget = HostTarget;

    fn as_ref(&self) -> ExecutionFunctionRef<'_, Body, Self::HostTarget> {
        match self {
            ValueFunctionEntry::Graph(function) => ExecutionFunctionRef::Graph(function),
            ValueFunctionEntry::Host(target) => ExecutionFunctionRef::Host(target),
        }
    }
}

impl HostTarget for Infallible {
    type Target = Infallible;
}

impl HostTarget for IntLocalId {
    type Target = HostIntFunctionId;
}

impl HostTarget for FloatLocalId {
    type Target = HostFloatFunctionId;
}

impl HostTarget for StringLocalId {
    type Target = HostStringFunctionId;
}

impl HostTarget for BitArrayLocalId {
    type Target = HostBitArrayFunctionId;
}

impl HostTarget for UtfCodepointLocalId {
    type Target = HostUtfCodepointFunctionId;
}

impl HostTarget for BoolLocalId {
    type Target = HostBoolFunctionId;
}

impl HostTarget for NilLocalId {
    type Target = HostNilFunctionId;
}

impl HostTarget for CustomLocal {
    type Target = Infallible;
}

impl HostTarget for TupleLocalId {
    type Target = Infallible;
}

impl HostTarget for ParameterListLocalId {
    type Target = Infallible;
}

impl HostTarget for IntListLocalId {
    type Target = Infallible;
}

impl HostTarget for FloatListLocalId {
    type Target = Infallible;
}

impl HostTarget for StringListLocalId {
    type Target = Infallible;
}

impl HostTarget for BitArrayListLocalId {
    type Target = Infallible;
}

impl HostTarget for UtfCodepointListLocalId {
    type Target = Infallible;
}

impl HostTarget for CustomListLocalId {
    type Target = Infallible;
}

impl HostTarget for BoolListLocalId {
    type Target = Infallible;
}

impl HostTarget for NilListLocalId {
    type Target = Infallible;
}

impl HostTarget for TupleListLocalId {
    type Target = Infallible;
}

impl HostTarget for ParameterListListLocalId {
    type Target = Infallible;
}

impl HostTarget for ListListLocalId {
    type Target = Infallible;
}

impl HostTarget for FunctionListLocalId {
    type Target = Infallible;
}

impl HostTarget for IntFunctionLocalId {
    type Target = Infallible;
}

impl HostTarget for FloatFunctionLocalId {
    type Target = Infallible;
}

impl HostTarget for StringFunctionLocalId {
    type Target = Infallible;
}

impl HostTarget for BitArrayFunctionLocalId {
    type Target = Infallible;
}

impl HostTarget for UtfCodepointFunctionLocalId {
    type Target = Infallible;
}

impl HostTarget for GenericFunctionLocal {
    type Target = Infallible;
}

impl HostTarget for NeverFunctionLocal {
    type Target = Infallible;
}

impl HostTarget for CustomFunctionLocal {
    type Target = Infallible;
}

impl HostTarget for BoolFunctionLocalId {
    type Target = Infallible;
}

impl HostTarget for NilFunctionLocalId {
    type Target = Infallible;
}

impl HostTarget for TupleFunctionLocalId {
    type Target = Infallible;
}

impl HostTarget for ListFunctionLocal {
    type Target = Infallible;
}

impl HostTarget for FunctionFunctionLocal {
    type Target = Infallible;
}

#[cfg(test)]
mod tests {
    use super::{ExecutionFunction, ExecutionHostTarget, HostTarget, HostedExecutionProfile};
    use crate::StatelessHostProfile;
    use crate::plan::execution::function::{
        CustomFunctionBody, ExecutableFunction, IntFunctionBody, ValueFunctionEntry,
    };
    use crate::plan::execution::graph::{
        BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolFunctionLocalId,
        BoolListLocalId, BoolLocalId, CustomFunctionLocal, CustomListLocalId, CustomLocal,
        FloatFunctionLocalId, FloatListLocalId, FloatLocalId, FunctionFunctionLocal,
        FunctionListLocalId, GenericFunctionLocal, IntFunctionLocalId, IntListLocalId, IntLocalId,
        ListFunctionLocal, ListListLocalId, NeverFunctionLocal, NilFunctionLocalId, NilListLocalId,
        NilLocalId, ParameterListListLocalId, ParameterListLocalId, StringFunctionLocalId,
        StringListLocalId, StringLocalId, TupleFunctionLocalId, TupleListLocalId, TupleLocalId,
        UtfCodepointFunctionLocalId, UtfCodepointListLocalId, UtfCodepointLocalId,
    };
    use crate::plan::execution::host::{
        HostBitArrayFunctionId, HostBoolFunctionId, HostFloatFunctionId, HostIntFunctionId,
        HostNilFunctionId, HostStringFunctionId, HostUtfCodepointFunctionId,
    };
    use std::any::TypeId;
    use std::convert::Infallible;

    type Hosted = HostedExecutionProfile<StatelessHostProfile>;

    #[test]
    fn maps_scalar_return_locals_to_typed_host_targets() {
        assert_target::<IntLocalId, HostIntFunctionId>();
        assert_target::<FloatLocalId, HostFloatFunctionId>();
        assert_target::<StringLocalId, HostStringFunctionId>();
        assert_target::<BitArrayLocalId, HostBitArrayFunctionId>();
        assert_target::<UtfCodepointLocalId, HostUtfCodepointFunctionId>();
        assert_target::<BoolLocalId, HostBoolFunctionId>();
        assert_target::<NilLocalId, HostNilFunctionId>();
    }

    #[test]
    fn keeps_non_scalar_return_locals_graph_only() {
        assert_target::<Infallible, Infallible>();
        assert_target::<CustomLocal, Infallible>();
        assert_target::<TupleLocalId, Infallible>();
        assert_target::<ParameterListLocalId, Infallible>();
        assert_target::<IntListLocalId, Infallible>();
        assert_target::<FloatListLocalId, Infallible>();
        assert_target::<StringListLocalId, Infallible>();
        assert_target::<BitArrayListLocalId, Infallible>();
        assert_target::<UtfCodepointListLocalId, Infallible>();
        assert_target::<CustomListLocalId, Infallible>();
        assert_target::<BoolListLocalId, Infallible>();
        assert_target::<NilListLocalId, Infallible>();
        assert_target::<TupleListLocalId, Infallible>();
        assert_target::<ParameterListListLocalId, Infallible>();
        assert_target::<ListListLocalId, Infallible>();
        assert_target::<FunctionListLocalId, Infallible>();
        assert_target::<IntFunctionLocalId, Infallible>();
        assert_target::<FloatFunctionLocalId, Infallible>();
        assert_target::<StringFunctionLocalId, Infallible>();
        assert_target::<BitArrayFunctionLocalId, Infallible>();
        assert_target::<UtfCodepointFunctionLocalId, Infallible>();
        assert_target::<GenericFunctionLocal, Infallible>();
        assert_target::<NeverFunctionLocal, Infallible>();
        assert_target::<CustomFunctionLocal, Infallible>();
        assert_target::<BoolFunctionLocalId, Infallible>();
        assert_target::<NilFunctionLocalId, Infallible>();
        assert_target::<TupleFunctionLocalId, Infallible>();
        assert_target::<ListFunctionLocal, Infallible>();
        assert_target::<FunctionFunctionLocal, Infallible>();
    }

    #[test]
    fn maps_plain_and_hosted_function_entries_through_one_profile() {
        assert_same::<
            ExecutionFunction<Infallible, IntFunctionBody>,
            ExecutableFunction<IntFunctionBody>,
        >();
        assert_same::<ExecutionHostTarget<Infallible, IntFunctionBody>, Infallible>();
        assert_same::<
            ExecutionFunction<Hosted, IntFunctionBody>,
            ValueFunctionEntry<IntFunctionBody, HostIntFunctionId>,
        >();
        assert_same::<ExecutionHostTarget<Hosted, IntFunctionBody>, HostIntFunctionId>();
        assert_same::<
            ExecutionFunction<Hosted, CustomFunctionBody>,
            ValueFunctionEntry<CustomFunctionBody, Infallible>,
        >();
        assert_same::<ExecutionHostTarget<Hosted, CustomFunctionBody>, Infallible>();
    }

    fn assert_target<Return, Expected>()
    where
        Return: HostTarget,
        Return::Target: 'static,
        Expected: 'static,
    {
        assert_eq!(TypeId::of::<Return::Target>(), TypeId::of::<Expected>());
    }

    fn assert_same<Actual: 'static, Expected: 'static>() {
        assert_eq!(TypeId::of::<Actual>(), TypeId::of::<Expected>());
    }
}
