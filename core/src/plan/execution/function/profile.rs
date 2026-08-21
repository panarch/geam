use super::{ExecutableFunction, FunctionBodyOwner, ValueFunctionEntry};
use crate::plan::execution::function::{
    ExternalFunctionFunctionId, ExternalFunctionId, ExternalListFunctionFunctionId,
    ExternalListFunctionId, FunctionFunctionId, FunctionLabelSource, ListFunctionFunctionId,
    ProfiledFunctionFunctionId, ProfiledListFunctionFunctionId, ProfiledListFunctionId,
    RuntimeFunctionFunctionTarget, RuntimeListFunctionId, TailCallLabelIndex,
};
use crate::plan::execution::graph::{
    ExternalFunctionInstruction, ExternalFunctionInstructionView, ExternalInstruction,
    ExternalInstructionView, ExternalListInstruction, ExternalListInstructionView,
};
use crate::plan::execution::host::{
    HostNeverFunctionId, HostedExecutionProfile, HostedFunctionTarget,
};
use std::convert::Infallible;
use std::fmt::Debug;

#[cfg(test)]
use crate::plan::execution::function::{CoreRuntimeFunctionId, ProfiledCoreRuntimeFunctionId};

pub(crate) trait ExecutionProfile {
    type Graph: ExecutionGraphProfile;
    type HostTarget<Body: ExecutionFunctionBody>;
    type Function<Body: ExecutionFunctionBody>: ExecutionFunctionEntry<Body, HostTarget = Self::HostTarget<Body>>;
    type NeverHostTarget;
    type NeverFunction: ExecutionFunctionEntry<
            super::ExecutionNeverFunctionBody<Self>,
            HostTarget = Self::NeverHostTarget,
        >;

    fn graph<Body: ExecutionFunctionBody>(
        function: ExecutableFunction<Body>,
    ) -> Self::Function<Body>;

    fn never_graph(
        function: ExecutableFunction<super::ExecutionNeverFunctionBody<Self>>,
    ) -> Self::NeverFunction;
}

pub(crate) trait ExecutionGraphProfile: Sized + Debug + Clone + PartialEq + Eq {
    type ExternalFunctionId: Debug + Clone + PartialEq + Eq;
    type ExternalListFunctionId: Debug + Clone + PartialEq + Eq;
    type ExternalFunctionFunctionId: Debug + Clone + PartialEq + Eq;
    type ExternalListFunctionFunctionId: Debug + Clone + PartialEq + Eq;
    type RuntimeFunctionFunctionId: Debug + Clone + PartialEq + Eq;
    type ExternalInstruction: ExternalInstructionView<Function = Self::ExternalFunctionId>;
    type ExternalListInstruction: ExternalListInstructionView<
        Function = Self::ExternalListFunctionId,
    >;
    type ExternalFunctionInstruction: ExternalFunctionInstructionView;

    fn external_function(id: &Self::ExternalFunctionId) -> ExternalFunctionId;

    fn list_function(id: &ProfiledListFunctionId<Self>) -> RuntimeListFunctionId;

    fn function_function(id: &ProfiledFunctionFunctionId<Self>) -> FunctionFunctionId;

    fn list_function_function(id: &ProfiledListFunctionFunctionId<Self>) -> ListFunctionFunctionId;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct HostedExecutionGraph;

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
pub(crate) type ExecutionNeverFunction<Profile> = <Profile as ExecutionProfile>::NeverFunction;
pub(crate) type ExecutionNeverHostTarget<Profile> = <Profile as ExecutionProfile>::NeverHostTarget;

impl<Body> ExecutionFunctionBody for Body where Body: FunctionBodyOwner {}

impl ExecutionProfile for Infallible {
    type Graph = Infallible;
    type HostTarget<Body: ExecutionFunctionBody> = Infallible;
    type Function<Body: ExecutionFunctionBody> = ExecutableFunction<Body>;
    type NeverHostTarget = Infallible;
    type NeverFunction = ExecutableFunction<super::ExecutionNeverFunctionBody<Self>>;

    fn graph<Body: ExecutionFunctionBody>(
        function: ExecutableFunction<Body>,
    ) -> Self::Function<Body> {
        function
    }

    fn never_graph(
        function: ExecutableFunction<super::ExecutionNeverFunctionBody<Self>>,
    ) -> Self::NeverFunction {
        function
    }
}

impl ExecutionProfile for HostedExecutionProfile {
    type Graph = HostedExecutionGraph;
    type HostTarget<Body: ExecutionFunctionBody> = HostedFunctionTarget<Body>;
    type Function<Body: ExecutionFunctionBody> =
        ValueFunctionEntry<Body, HostedFunctionTarget<Body>>;
    type NeverHostTarget = HostNeverFunctionId;
    type NeverFunction =
        ValueFunctionEntry<super::ExecutionNeverFunctionBody<Self>, HostNeverFunctionId>;

    fn graph<Body: ExecutionFunctionBody>(
        function: ExecutableFunction<Body>,
    ) -> Self::Function<Body> {
        ValueFunctionEntry::graph(function)
    }

    fn never_graph(
        function: ExecutableFunction<super::ExecutionNeverFunctionBody<Self>>,
    ) -> Self::NeverFunction {
        ValueFunctionEntry::graph(function)
    }
}

impl ExecutionGraphProfile for Infallible {
    type ExternalFunctionId = Infallible;
    type ExternalListFunctionId = Infallible;
    type ExternalFunctionFunctionId = Infallible;
    type ExternalListFunctionFunctionId = Infallible;
    type RuntimeFunctionFunctionId = ProfiledFunctionFunctionId<Infallible>;
    type ExternalInstruction = Infallible;
    type ExternalListInstruction = Infallible;
    type ExternalFunctionInstruction = Infallible;

    fn external_function(id: &Self::ExternalFunctionId) -> ExternalFunctionId {
        match *id {}
    }

    fn list_function(id: &ProfiledListFunctionId<Self>) -> RuntimeListFunctionId {
        use ProfiledListFunctionId as F;

        match id {
            F::Core(id) => RuntimeListFunctionId::Core(id.clone()),
            F::External(id) => match *id {},
        }
    }

    fn function_function(id: &ProfiledFunctionFunctionId<Self>) -> FunctionFunctionId {
        use ProfiledFunctionFunctionId as F;

        match id {
            F::Generic(id) => FunctionFunctionId::Generic(id.clone()),
            F::Never(id) => FunctionFunctionId::Never(id.clone()),
            F::Int(id) => FunctionFunctionId::Int(*id),
            F::Float(id) => FunctionFunctionId::Float(*id),
            F::String(id) => FunctionFunctionId::String(*id),
            F::BitArray(id) => FunctionFunctionId::BitArray(*id),
            F::UtfCodepoint(id) => FunctionFunctionId::UtfCodepoint(*id),
            F::Custom(id) => FunctionFunctionId::Custom(id.clone()),
            F::External(id) => match *id {},
            F::Bool(id) => FunctionFunctionId::Bool(*id),
            F::Nil(id) => FunctionFunctionId::Nil(*id),
            F::Tuple(id) => FunctionFunctionId::Tuple(*id),
            F::List(id) => FunctionFunctionId::List(Self::list_function_function(id)),
            F::Function(id) => FunctionFunctionId::Function(id.clone()),
        }
    }

    fn list_function_function(id: &ProfiledListFunctionFunctionId<Self>) -> ListFunctionFunctionId {
        use ProfiledListFunctionFunctionId as F;

        match id {
            F::Parameter {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::Parameter {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
            F::ParameterList {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::ParameterList {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
            F::Int {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::Int {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
            F::String {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::String {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
            F::BitArray {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::BitArray {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
            F::UtfCodepoint {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::UtfCodepoint {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
            F::Custom {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::Custom {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
            F::External { id, .. } => match *id {},
            F::Float {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::Float {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
            F::Bool {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::Bool {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
            F::Nil {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::Nil {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
            F::Tuple {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::Tuple {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
            F::List {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::List {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
            F::Function {
                id,
                type_,
                list_type,
            } => ListFunctionFunctionId::Function {
                id: *id,
                type_: type_.clone(),
                list_type: *list_type,
            },
        }
    }
}

impl ExecutionGraphProfile for HostedExecutionGraph {
    type ExternalFunctionId = ExternalFunctionId;
    type ExternalListFunctionId = ExternalListFunctionId;
    type ExternalFunctionFunctionId = ExternalFunctionFunctionId;
    type ExternalListFunctionFunctionId = ExternalListFunctionFunctionId;
    type RuntimeFunctionFunctionId = RuntimeFunctionFunctionTarget;
    type ExternalInstruction = ExternalInstruction;
    type ExternalListInstruction = ExternalListInstruction;
    type ExternalFunctionInstruction = ExternalFunctionInstruction;

    fn external_function(id: &Self::ExternalFunctionId) -> ExternalFunctionId {
        *id
    }

    fn list_function(id: &ProfiledListFunctionId<Self>) -> RuntimeListFunctionId {
        id.clone()
    }

    fn function_function(id: &ProfiledFunctionFunctionId<Self>) -> FunctionFunctionId {
        id.clone()
    }

    fn list_function_function(id: &ProfiledListFunctionFunctionId<Self>) -> ListFunctionFunctionId {
        id.clone()
    }
}

impl FunctionLabelSource for Infallible {
    fn function_label(&self) -> crate::plan::execution::explain::FunctionLabel {
        match *self {}
    }
}

impl TailCallLabelIndex for Infallible {
    fn tail_call_label_index(&self) -> usize {
        match *self {}
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
pub(super) fn plain_core_runtime_function_id(
    id: &ProfiledCoreRuntimeFunctionId<Infallible>,
) -> CoreRuntimeFunctionId {
    use ProfiledCoreRuntimeFunctionId as F;

    match id {
        F::Never(id) => CoreRuntimeFunctionId::Never(*id),
        F::Int(id) => CoreRuntimeFunctionId::Int(*id),
        F::Float(id) => CoreRuntimeFunctionId::Float(*id),
        F::String(id) => CoreRuntimeFunctionId::String(*id),
        F::BitArray(id) => CoreRuntimeFunctionId::BitArray(*id),
        F::UtfCodepoint(id) => CoreRuntimeFunctionId::UtfCodepoint(*id),
        F::Custom(id) => CoreRuntimeFunctionId::Custom(*id),
        F::Bool(id) => CoreRuntimeFunctionId::Bool(*id),
        F::Nil(id) => CoreRuntimeFunctionId::Nil(*id),
        F::Tuple { id, return_type } => CoreRuntimeFunctionId::Tuple {
            id: *id,
            return_type: return_type.clone(),
        },
        F::List(id) => CoreRuntimeFunctionId::List(Infallible::list_function(id)),
        F::Function { id, return_type } => CoreRuntimeFunctionId::Function {
            id: RuntimeFunctionFunctionTarget::Core(id.clone()),
            return_type: return_type.clone(),
        },
    }
}
#[cfg(test)]
mod tests {
    use super::{
        ExecutionFunction, ExecutionGraphProfile, ExecutionHostTarget, HostedExecutionGraph,
        HostedExecutionProfile,
    };
    use crate::plan::execution::function::{
        BitArrayFunctionBody, BitArrayFunctionFunctionBody, BitArrayListFunctionBody,
        BoolFunctionBody, BoolFunctionFunctionBody, BoolListFunctionBody,
        CoreListFunctionFunctionBody, CustomFunctionBody, CustomFunctionFunctionBody,
        CustomListFunctionBody, ExecutableFunction, ExternalFunctionBody,
        ExternalFunctionFunctionBody, ExternalFunctionFunctionId, ExternalFunctionId,
        ExternalListFunctionBody, ExternalListFunctionFunctionBody, ExternalListFunctionFunctionId,
        ExternalListFunctionId, FloatFunctionBody, FloatFunctionFunctionBody,
        FloatListFunctionBody, FunctionBodyOwner, FunctionFunctionFunctionBody, FunctionFunctionId,
        FunctionListFunctionBody, GenericFunctionFunctionBody, IntFunctionBody,
        IntFunctionFunctionBody, IntListFunctionBody, ListFunctionFunctionId, ListListFunctionBody,
        NeverFunctionBody, NeverFunctionFunctionBody, NilFunctionBody, NilFunctionFunctionBody,
        NilListFunctionBody, ParameterListFunctionBody, ParameterListListFunctionBody,
        ProfiledFunctionFunctionId, ProfiledListFunctionFunctionId, ProfiledListFunctionId,
        RuntimeListFunctionId, StringFunctionBody, StringFunctionFunctionBody,
        StringListFunctionBody, TupleFunctionBody, TupleFunctionFunctionBody,
        TupleListFunctionBody, UtfCodepointFunctionBody, UtfCodepointFunctionFunctionBody,
        UtfCodepointListFunctionBody, ValueFunctionEntry,
    };
    use crate::plan::execution::host::{HostNeverFunctionId, HostedFunctionTarget};
    use crate::plan::execution::type_::{
        ExternalFunctionType, ExternalListTypeId, ExternalTypeId, FunctionType, ListTypeId,
        ValueType,
    };
    use std::any::TypeId;
    use std::convert::Infallible;

    type Hosted = HostedExecutionProfile;

    #[test]
    fn maps_every_function_body_to_its_typed_host_target() {
        assert_hosted::<IntFunctionBody>();
        assert_hosted::<FloatFunctionBody>();
        assert_hosted::<StringFunctionBody>();
        assert_hosted::<BitArrayFunctionBody>();
        assert_hosted::<UtfCodepointFunctionBody>();
        assert_hosted::<CustomFunctionBody>();
        assert_hosted::<ExternalFunctionBody>();
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
        assert_hosted::<ExternalListFunctionBody>();
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
        assert_hosted::<ExternalFunctionFunctionBody>();
        assert_hosted::<BoolFunctionFunctionBody>();
        assert_hosted::<NilFunctionFunctionBody>();
        assert_hosted::<TupleFunctionFunctionBody>();
        assert_hosted::<CoreListFunctionFunctionBody>();
        assert_hosted::<ExternalListFunctionFunctionBody>();
        assert_hosted::<FunctionFunctionFunctionBody>();
        assert_same::<
            super::ExecutionNeverFunction<Hosted>,
            ValueFunctionEntry<NeverFunctionBody, HostNeverFunctionId>,
        >();
        assert_same::<super::ExecutionNeverHostTarget<Hosted>, HostNeverFunctionId>();
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

    #[test]
    fn resolves_external_function_ids_through_the_hosted_graph_profile() {
        let external_type = ExternalTypeId::new(0);
        let list_type = ExternalListTypeId::new(ListTypeId::new(1), external_type);
        let function_type = FunctionType::new(Vec::new(), ValueType::External(external_type));
        let external_function_type =
            ExternalFunctionType::from_shapes(function_type.clone(), Vec::new(), external_type);
        let function = ExternalFunctionId::new(2, external_type);
        let list_function = ExternalListFunctionId::new(3, list_type);
        let function_function = ExternalFunctionFunctionId::new(4, external_function_type.clone());
        let list_function_function = ExternalListFunctionFunctionId(5);

        assert_eq!(
            <HostedExecutionGraph as ExecutionGraphProfile>::external_function(&function),
            function,
        );
        assert_eq!(
            <HostedExecutionGraph as ExecutionGraphProfile>::list_function(
                &ProfiledListFunctionId::External(list_function),
            ),
            RuntimeListFunctionId::External(list_function),
        );
        assert_eq!(
            <HostedExecutionGraph as ExecutionGraphProfile>::function_function(
                &ProfiledFunctionFunctionId::External(function_function.clone()),
            ),
            FunctionFunctionId::External(function_function.clone()),
        );
        assert_eq!(
            <HostedExecutionGraph as ExecutionGraphProfile>::list_function_function(
                &ProfiledListFunctionFunctionId::External {
                    id: list_function_function,
                    type_: function_type,
                    list_type,
                },
            ),
            ListFunctionFunctionId::External {
                id: list_function_function,
                type_: FunctionType::new(Vec::new(), ValueType::External(external_type)),
                list_type,
            },
        );
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
