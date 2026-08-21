use super::super::body::ProfiledFunctionBody;
use super::{
    BitArrayListFunctionId, BoolListFunctionId, CustomListFunctionId, ExternalListFunctionId,
    FloatListFunctionId, FunctionListFunctionId, IntListFunctionId, ListListFunctionId,
    NilListFunctionId, ParameterListFunctionId, ParameterListListFunctionId, StringListFunctionId,
    TupleListFunctionId, UtfCodepointListFunctionId,
};
use crate::plan::execution::function::{ExecutionProfile, HostedExecutionGraph};
use crate::plan::execution::graph::{
    BitArrayListLocalId, BoolListLocalId, CustomListLocalId, ExternalListLocalId, FloatListLocalId,
    FunctionListLocalId, IntListLocalId, ListListLocalId, NilListLocalId, ParameterListListLocalId,
    ParameterListLocalId, StringListLocalId, TupleListLocalId, UtfCodepointListLocalId,
};

pub(crate) type ProfiledParameterListFunctionBody<Graph> = ProfiledFunctionBody<
    ParameterListLocalId,
    crate::plan::FunctionCallTarget<ParameterListFunctionId>,
    Graph,
>;
pub(crate) type ProfiledIntListFunctionBody<Graph> =
    ProfiledFunctionBody<IntListLocalId, crate::plan::FunctionCallTarget<IntListFunctionId>, Graph>;
pub(crate) type ProfiledFloatListFunctionBody<Graph> = ProfiledFunctionBody<
    FloatListLocalId,
    crate::plan::FunctionCallTarget<FloatListFunctionId>,
    Graph,
>;
pub(crate) type ProfiledStringListFunctionBody<Graph> = ProfiledFunctionBody<
    StringListLocalId,
    crate::plan::FunctionCallTarget<StringListFunctionId>,
    Graph,
>;
pub(crate) type ProfiledBitArrayListFunctionBody<Graph> = ProfiledFunctionBody<
    BitArrayListLocalId,
    crate::plan::FunctionCallTarget<BitArrayListFunctionId>,
    Graph,
>;
pub(crate) type ProfiledUtfCodepointListFunctionBody<Graph> = ProfiledFunctionBody<
    UtfCodepointListLocalId,
    crate::plan::FunctionCallTarget<UtfCodepointListFunctionId>,
    Graph,
>;
pub(crate) type ProfiledCustomListFunctionBody<Graph> = ProfiledFunctionBody<
    CustomListLocalId,
    crate::plan::FunctionCallTarget<CustomListFunctionId>,
    Graph,
>;
pub(crate) type ProfiledExternalListFunctionBody<Graph> = ProfiledFunctionBody<
    ExternalListLocalId,
    crate::plan::FunctionCallTarget<ExternalListFunctionId>,
    Graph,
>;
pub(crate) type ProfiledBoolListFunctionBody<Graph> = ProfiledFunctionBody<
    BoolListLocalId,
    crate::plan::FunctionCallTarget<BoolListFunctionId>,
    Graph,
>;
pub(crate) type ProfiledNilListFunctionBody<Graph> =
    ProfiledFunctionBody<NilListLocalId, crate::plan::FunctionCallTarget<NilListFunctionId>, Graph>;
pub(crate) type ProfiledTupleListFunctionBody<Graph> = ProfiledFunctionBody<
    TupleListLocalId,
    crate::plan::FunctionCallTarget<TupleListFunctionId>,
    Graph,
>;
pub(crate) type ProfiledParameterListListFunctionBody<Graph> = ProfiledFunctionBody<
    ParameterListListLocalId,
    crate::plan::FunctionCallTarget<ParameterListListFunctionId>,
    Graph,
>;
pub(crate) type ProfiledListListFunctionBody<Graph> = ProfiledFunctionBody<
    ListListLocalId,
    crate::plan::FunctionCallTarget<ListListFunctionId>,
    Graph,
>;
pub(crate) type ProfiledFunctionListFunctionBody<Graph> = ProfiledFunctionBody<
    FunctionListLocalId,
    crate::plan::FunctionCallTarget<FunctionListFunctionId>,
    Graph,
>;

pub(crate) type ParameterListFunctionBody = ProfiledParameterListFunctionBody<HostedExecutionGraph>;
pub(crate) type IntListFunctionBody = ProfiledIntListFunctionBody<HostedExecutionGraph>;
pub(crate) type FloatListFunctionBody = ProfiledFloatListFunctionBody<HostedExecutionGraph>;
pub(crate) type StringListFunctionBody = ProfiledStringListFunctionBody<HostedExecutionGraph>;
pub(crate) type BitArrayListFunctionBody = ProfiledBitArrayListFunctionBody<HostedExecutionGraph>;
pub(crate) type UtfCodepointListFunctionBody =
    ProfiledUtfCodepointListFunctionBody<HostedExecutionGraph>;
pub(crate) type CustomListFunctionBody = ProfiledCustomListFunctionBody<HostedExecutionGraph>;
pub(crate) type ExternalListFunctionBody = ProfiledExternalListFunctionBody<HostedExecutionGraph>;
pub(crate) type BoolListFunctionBody = ProfiledBoolListFunctionBody<HostedExecutionGraph>;
pub(crate) type NilListFunctionBody = ProfiledNilListFunctionBody<HostedExecutionGraph>;
pub(crate) type TupleListFunctionBody = ProfiledTupleListFunctionBody<HostedExecutionGraph>;
pub(crate) type ParameterListListFunctionBody =
    ProfiledParameterListListFunctionBody<HostedExecutionGraph>;
pub(crate) type ListListFunctionBody = ProfiledListListFunctionBody<HostedExecutionGraph>;
pub(crate) type FunctionListFunctionBody = ProfiledFunctionListFunctionBody<HostedExecutionGraph>;

pub(crate) type ExecutionParameterListFunctionBody<Profile> =
    ProfiledParameterListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionIntListFunctionBody<Profile> =
    ProfiledIntListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionFloatListFunctionBody<Profile> =
    ProfiledFloatListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionStringListFunctionBody<Profile> =
    ProfiledStringListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionBitArrayListFunctionBody<Profile> =
    ProfiledBitArrayListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionUtfCodepointListFunctionBody<Profile> =
    ProfiledUtfCodepointListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionCustomListFunctionBody<Profile> =
    ProfiledCustomListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionExternalListFunctionBody<Profile> =
    ProfiledExternalListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionBoolListFunctionBody<Profile> =
    ProfiledBoolListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionNilListFunctionBody<Profile> =
    ProfiledNilListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionTupleListFunctionBody<Profile> =
    ProfiledTupleListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionParameterListListFunctionBody<Profile> =
    ProfiledParameterListListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionListListFunctionBody<Profile> =
    ProfiledListListFunctionBody<<Profile as ExecutionProfile>::Graph>;
pub(crate) type ExecutionFunctionListFunctionBody<Profile> =
    ProfiledFunctionListFunctionBody<<Profile as ExecutionProfile>::Graph>;
