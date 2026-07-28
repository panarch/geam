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
    HostedFunctionTarget,
};
use std::convert::Infallible;

pub(crate) trait ExecutionFunctionBody: FunctionBodyOwner {
    type HostValueTarget;
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
}

pub(crate) type ExecutionFunction<Profile, Body> = <Profile as ExecutionProfile>::Function<Body>;

pub(crate) type ExecutionHostTarget<Profile, Body> =
    <Profile as ExecutionProfile>::HostTarget<Body>;

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
    type HostTarget<Body: ExecutionFunctionBody> = HostedFunctionTarget<Body::HostValueTarget>;
    type Function<Body: ExecutionFunctionBody> =
        ValueFunctionEntry<Body, HostedFunctionTarget<Body::HostValueTarget>>;

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

impl<Body, Target> ExecutionFunctionEntry<Body>
    for ValueFunctionEntry<Body, HostedFunctionTarget<Target>>
{
    type HostTarget = HostedFunctionTarget<Target>;

    fn as_ref(&self) -> ExecutionFunctionRef<'_, Body, Self::HostTarget> {
        match self {
            ValueFunctionEntry::Graph(function) => ExecutionFunctionRef::Graph(function),
            ValueFunctionEntry::Host(target) => ExecutionFunctionRef::Host(target),
        }
    }
}

pub(crate) trait HostValueTarget {
    type Target;
}

impl<Body> ExecutionFunctionBody for Body
where
    Body: FunctionBodyOwner,
    Body::Return: HostValueTarget,
{
    type HostValueTarget = <Body::Return as HostValueTarget>::Target;
}

impl HostValueTarget for Infallible {
    type Target = Infallible;
}

impl HostValueTarget for IntLocalId {
    type Target = HostIntFunctionId;
}

impl HostValueTarget for FloatLocalId {
    type Target = HostFloatFunctionId;
}

impl HostValueTarget for StringLocalId {
    type Target = HostStringFunctionId;
}

impl HostValueTarget for BitArrayLocalId {
    type Target = HostBitArrayFunctionId;
}

impl HostValueTarget for UtfCodepointLocalId {
    type Target = HostUtfCodepointFunctionId;
}

impl HostValueTarget for BoolLocalId {
    type Target = HostBoolFunctionId;
}

impl HostValueTarget for NilLocalId {
    type Target = HostNilFunctionId;
}

impl HostValueTarget for CustomLocal {
    type Target = Infallible;
}

impl HostValueTarget for TupleLocalId {
    type Target = Infallible;
}

impl HostValueTarget for ParameterListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for IntListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for FloatListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for StringListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for BitArrayListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for UtfCodepointListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for CustomListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for BoolListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for NilListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for TupleListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for ParameterListListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for ListListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for FunctionListLocalId {
    type Target = Infallible;
}

impl HostValueTarget for IntFunctionLocalId {
    type Target = Infallible;
}

impl HostValueTarget for FloatFunctionLocalId {
    type Target = Infallible;
}

impl HostValueTarget for StringFunctionLocalId {
    type Target = Infallible;
}

impl HostValueTarget for BitArrayFunctionLocalId {
    type Target = Infallible;
}

impl HostValueTarget for UtfCodepointFunctionLocalId {
    type Target = Infallible;
}

impl HostValueTarget for GenericFunctionLocal {
    type Target = Infallible;
}

impl HostValueTarget for NeverFunctionLocal {
    type Target = Infallible;
}

impl HostValueTarget for CustomFunctionLocal {
    type Target = Infallible;
}

impl HostValueTarget for BoolFunctionLocalId {
    type Target = Infallible;
}

impl HostValueTarget for NilFunctionLocalId {
    type Target = Infallible;
}

impl HostValueTarget for TupleFunctionLocalId {
    type Target = Infallible;
}

impl HostValueTarget for ListFunctionLocal {
    type Target = Infallible;
}

impl HostValueTarget for FunctionFunctionLocal {
    type Target = Infallible;
}

#[cfg(test)]
mod tests {
    use super::{ExecutionFunction, ExecutionHostTarget, HostValueTarget, HostedExecutionProfile};
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
        HostNilFunctionId, HostStringFunctionId, HostUtfCodepointFunctionId, HostedFunctionTarget,
    };
    use std::any::TypeId;
    use std::convert::Infallible;

    type Hosted = HostedExecutionProfile<StatelessHostProfile>;

    #[test]
    fn maps_every_return_local_to_its_value_host_target() {
        assert_target::<IntLocalId, HostIntFunctionId>();
        assert_target::<FloatLocalId, HostFloatFunctionId>();
        assert_target::<StringLocalId, HostStringFunctionId>();
        assert_target::<BitArrayLocalId, HostBitArrayFunctionId>();
        assert_target::<UtfCodepointLocalId, HostUtfCodepointFunctionId>();
        assert_target::<BoolLocalId, HostBoolFunctionId>();
        assert_target::<NilLocalId, HostNilFunctionId>();
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
    fn maps_every_function_body_to_a_value_or_never_host_target() {
        assert_hosted::<NeverFunctionBody, Infallible>();
        assert_hosted::<IntFunctionBody, HostIntFunctionId>();
        assert_hosted::<FloatFunctionBody, HostFloatFunctionId>();
        assert_hosted::<StringFunctionBody, HostStringFunctionId>();
        assert_hosted::<BitArrayFunctionBody, HostBitArrayFunctionId>();
        assert_hosted::<UtfCodepointFunctionBody, HostUtfCodepointFunctionId>();
        assert_hosted::<CustomFunctionBody, Infallible>();
        assert_hosted::<BoolFunctionBody, HostBoolFunctionId>();
        assert_hosted::<NilFunctionBody, HostNilFunctionId>();
        assert_hosted::<TupleFunctionBody, Infallible>();
        assert_hosted::<ParameterListFunctionBody, Infallible>();
        assert_hosted::<IntListFunctionBody, Infallible>();
        assert_hosted::<FloatListFunctionBody, Infallible>();
        assert_hosted::<StringListFunctionBody, Infallible>();
        assert_hosted::<BitArrayListFunctionBody, Infallible>();
        assert_hosted::<UtfCodepointListFunctionBody, Infallible>();
        assert_hosted::<CustomListFunctionBody, Infallible>();
        assert_hosted::<BoolListFunctionBody, Infallible>();
        assert_hosted::<NilListFunctionBody, Infallible>();
        assert_hosted::<TupleListFunctionBody, Infallible>();
        assert_hosted::<ParameterListListFunctionBody, Infallible>();
        assert_hosted::<ListListFunctionBody, Infallible>();
        assert_hosted::<FunctionListFunctionBody, Infallible>();
        assert_hosted::<IntFunctionFunctionBody, Infallible>();
        assert_hosted::<FloatFunctionFunctionBody, Infallible>();
        assert_hosted::<StringFunctionFunctionBody, Infallible>();
        assert_hosted::<BitArrayFunctionFunctionBody, Infallible>();
        assert_hosted::<UtfCodepointFunctionFunctionBody, Infallible>();
        assert_hosted::<GenericFunctionFunctionBody, Infallible>();
        assert_hosted::<NeverFunctionFunctionBody, Infallible>();
        assert_hosted::<CustomFunctionFunctionBody, Infallible>();
        assert_hosted::<BoolFunctionFunctionBody, Infallible>();
        assert_hosted::<NilFunctionFunctionBody, Infallible>();
        assert_hosted::<TupleFunctionFunctionBody, Infallible>();
        assert_hosted::<ListFunctionFunctionBody, Infallible>();
        assert_hosted::<FunctionFunctionFunctionBody, Infallible>();
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
            ValueFunctionEntry<IntFunctionBody, HostedFunctionTarget<HostIntFunctionId>>,
        >();
        assert_same::<
            ExecutionHostTarget<Hosted, IntFunctionBody>,
            HostedFunctionTarget<HostIntFunctionId>,
        >();
        assert_same::<
            ExecutionFunction<Hosted, CustomFunctionBody>,
            ValueFunctionEntry<CustomFunctionBody, HostedFunctionTarget<Infallible>>,
        >();
        assert_same::<
            ExecutionHostTarget<Hosted, CustomFunctionBody>,
            HostedFunctionTarget<Infallible>,
        >();
    }

    fn assert_hosted<Body, ValueTarget>()
    where
        Body: FunctionBodyOwner + super::ExecutionFunctionBody + 'static,
        ValueTarget: 'static,
    {
        assert_same::<ExecutionHostTarget<Hosted, Body>, HostedFunctionTarget<ValueTarget>>();
    }

    fn assert_target<Return, Expected>()
    where
        Return: HostValueTarget,
        Return::Target: 'static,
        Expected: 'static,
    {
        assert_same::<Return::Target, Expected>();
    }

    fn assert_same<Actual: 'static, Expected: 'static>() {
        assert_eq!(TypeId::of::<Actual>(), TypeId::of::<Expected>());
    }
}
