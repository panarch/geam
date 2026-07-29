use super::{ExecutableFunction, FunctionBodyOwner, ValueFunctionEntry};
use crate::host::HostProfile;
use crate::plan::execution::host::{HostedExecutionProfile, HostedFunctionTarget};
use std::convert::Infallible;

pub(crate) trait ExecutionProfile {
    type RunState;
    type HostTarget<Body: ExecutionFunctionBody>;
    type Function<Body: ExecutionFunctionBody>: ExecutionFunctionEntry<Body, HostTarget = Self::HostTarget<Body>>;

    fn graph<Body: ExecutionFunctionBody>(
        function: ExecutableFunction<Body>,
    ) -> Self::Function<Body>;
}

pub(crate) trait ExecutionFunctionBody: FunctionBodyOwner {}

pub(crate) trait ExecutionFunctionEntry<Body> {
    type HostTarget;

    fn as_ref(&self) -> ExecutionFunctionRef<'_, Body, Self::HostTarget>;
}

pub(crate) enum ExecutionFunctionRef<'function, Body, HostTarget> {
    Graph(&'function ExecutableFunction<Body>),
    Host(&'function HostTarget),
}

pub(crate) type ExecutionFunction<Profile, Body> = <Profile as ExecutionProfile>::Function<Body>;

pub(crate) type ExecutionHostTarget<Profile, Body> =
    <Profile as ExecutionProfile>::HostTarget<Body>;

impl<Body> ExecutionFunctionBody for Body where Body: FunctionBodyOwner {}

impl ExecutionProfile for Infallible {
    type RunState = ();
    type HostTarget<Body: ExecutionFunctionBody> = Infallible;
    type Function<Body: ExecutionFunctionBody> = ExecutableFunction<Body>;

    fn graph<Body: ExecutionFunctionBody>(
        function: ExecutableFunction<Body>,
    ) -> Self::Function<Body> {
        function
    }
}

impl<Profile: HostProfile> ExecutionProfile for HostedExecutionProfile<Profile> {
    type RunState = Profile::RunState;
    type HostTarget<Body: ExecutionFunctionBody> = HostedFunctionTarget<Body>;
    type Function<Body: ExecutionFunctionBody> =
        ValueFunctionEntry<Body, HostedFunctionTarget<Body>>;

    fn graph<Body: ExecutionFunctionBody>(
        function: ExecutableFunction<Body>,
    ) -> Self::Function<Body> {
        ValueFunctionEntry::graph(function)
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
#[cfg(test)]
mod tests {
    use super::{ExecutionFunction, ExecutionHostTarget, HostedExecutionProfile};
    use crate::StatelessHostProfile;
    use crate::plan::execution::function::{
        BitArrayFunctionBody, BitArrayFunctionFunctionBody, BitArrayListFunctionBody,
        BoolFunctionBody, BoolFunctionFunctionBody, BoolListFunctionBody, CustomFunctionBody,
        CustomFunctionFunctionBody, CustomListFunctionBody, ExecutableFunction, FloatFunctionBody,
        FloatFunctionFunctionBody, FloatListFunctionBody, FunctionBodyOwner,
        FunctionFunctionFunctionBody, FunctionListFunctionBody, GenericFunctionFunctionBody,
        IntFunctionBody, IntFunctionFunctionBody, IntListFunctionBody, ListFunctionFunctionBody,
        ListListFunctionBody, NeverFunctionBody, NeverFunctionFunctionBody, NilFunctionBody,
        NilFunctionFunctionBody, NilListFunctionBody, ParameterListFunctionBody,
        ParameterListListFunctionBody, StringFunctionBody, StringFunctionFunctionBody,
        StringListFunctionBody, TupleFunctionBody, TupleFunctionFunctionBody,
        TupleListFunctionBody, UtfCodepointFunctionBody, UtfCodepointFunctionFunctionBody,
        UtfCodepointListFunctionBody, ValueFunctionEntry,
    };
    use crate::plan::execution::host::HostedFunctionTarget;
    use std::any::TypeId;
    use std::convert::Infallible;

    type Hosted = HostedExecutionProfile<StatelessHostProfile>;

    #[test]
    fn maps_every_function_body_to_its_typed_host_target() {
        assert_hosted::<NeverFunctionBody>();
        assert_hosted::<IntFunctionBody>();
        assert_hosted::<FloatFunctionBody>();
        assert_hosted::<StringFunctionBody>();
        assert_hosted::<BitArrayFunctionBody>();
        assert_hosted::<UtfCodepointFunctionBody>();
        assert_hosted::<CustomFunctionBody>();
        assert_hosted::<BoolFunctionBody>();
        assert_hosted::<NilFunctionBody>();
        assert_hosted::<TupleFunctionBody>();
        assert_hosted::<ParameterListFunctionBody>();
        assert_hosted::<IntListFunctionBody>();
        assert_hosted::<FloatListFunctionBody>();
        assert_hosted::<StringListFunctionBody>();
        assert_hosted::<BitArrayListFunctionBody>();
        assert_hosted::<UtfCodepointListFunctionBody>();
        assert_hosted::<CustomListFunctionBody>();
        assert_hosted::<BoolListFunctionBody>();
        assert_hosted::<NilListFunctionBody>();
        assert_hosted::<TupleListFunctionBody>();
        assert_hosted::<ParameterListListFunctionBody>();
        assert_hosted::<ListListFunctionBody>();
        assert_hosted::<FunctionListFunctionBody>();
        assert_hosted::<IntFunctionFunctionBody>();
        assert_hosted::<FloatFunctionFunctionBody>();
        assert_hosted::<StringFunctionFunctionBody>();
        assert_hosted::<BitArrayFunctionFunctionBody>();
        assert_hosted::<UtfCodepointFunctionFunctionBody>();
        assert_hosted::<GenericFunctionFunctionBody>();
        assert_hosted::<NeverFunctionFunctionBody>();
        assert_hosted::<CustomFunctionFunctionBody>();
        assert_hosted::<BoolFunctionFunctionBody>();
        assert_hosted::<NilFunctionFunctionBody>();
        assert_hosted::<TupleFunctionFunctionBody>();
        assert_hosted::<ListFunctionFunctionBody>();
        assert_hosted::<FunctionFunctionFunctionBody>();
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
            ValueFunctionEntry<IntFunctionBody, HostedFunctionTarget<IntFunctionBody>>,
        >();
        assert_same::<
            ExecutionHostTarget<Hosted, IntFunctionBody>,
            HostedFunctionTarget<IntFunctionBody>,
        >();
        assert_same::<
            ExecutionFunction<Hosted, CustomFunctionBody>,
            ValueFunctionEntry<CustomFunctionBody, HostedFunctionTarget<CustomFunctionBody>>,
        >();
        assert_same::<
            ExecutionHostTarget<Hosted, CustomFunctionBody>,
            HostedFunctionTarget<CustomFunctionBody>,
        >();
    }

    fn assert_hosted<Body>()
    where
        Body: FunctionBodyOwner + 'static,
        HostedFunctionTarget<Body>: 'static,
    {
        assert_same::<ExecutionHostTarget<Hosted, Body>, HostedFunctionTarget<Body>>();
    }

    fn assert_same<Actual: 'static, Expected: 'static>() {
        assert_eq!(TypeId::of::<Actual>(), TypeId::of::<Expected>());
    }
}
