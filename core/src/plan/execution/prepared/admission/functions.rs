use super::body::{self, BodyError, Tail};
use super::call::{CallError, Target};
use super::catalog::{Catalog, Function};
use super::instruction::Instructions;
use super::local::Output;
use super::operand::Operand;
use super::type_::Types;
use crate::plan::execution::function::{
    ExecutionBitArrayFunctionBody, ExecutionBitArrayFunctionFunctionBody,
    ExecutionBitArrayListFunctionBody, ExecutionBoolFunctionBody,
    ExecutionBoolFunctionFunctionBody, ExecutionBoolListFunctionBody,
    ExecutionCoreListFunctionFunctionBody, ExecutionCustomFunctionBody,
    ExecutionCustomFunctionFunctionBody, ExecutionCustomListFunctionBody,
    ExecutionExternalFunctionBody, ExecutionExternalFunctionFunctionBody,
    ExecutionExternalListFunctionBody, ExecutionExternalListFunctionFunctionBody,
    ExecutionFloatFunctionBody, ExecutionFloatFunctionFunctionBody, ExecutionFloatListFunctionBody,
    ExecutionFunctionBody, ExecutionFunctionEntry, ExecutionFunctionFunctionFunctionBody,
    ExecutionFunctionListFunctionBody, ExecutionFunctionRef, ExecutionGenericFunctionFunctionBody,
    ExecutionGraphProfile, ExecutionIntFunctionBody, ExecutionIntFunctionFunctionBody,
    ExecutionIntListFunctionBody, ExecutionListListFunctionBody, ExecutionNeverFunctionBody,
    ExecutionNeverFunctionFunctionBody, ExecutionNilFunctionBody, ExecutionNilFunctionFunctionBody,
    ExecutionNilListFunctionBody, ExecutionParameterListFunctionBody,
    ExecutionParameterListListFunctionBody, ExecutionProfile, ExecutionStringFunctionBody,
    ExecutionStringFunctionFunctionBody, ExecutionStringListFunctionBody,
    ExecutionTupleFunctionBody, ExecutionTupleFunctionFunctionBody, ExecutionTupleListFunctionBody,
    ExecutionUtfCodepointFunctionBody, ExecutionUtfCodepointFunctionFunctionBody,
    ExecutionUtfCodepointListFunctionBody, FunctionFunctionTables, FunctionTableFamily,
    FunctionTables, ListFunctionTables, ValueFunctionTables,
};
use crate::plan::execution::graph::ExternalListInstructionView;
use std::convert::Infallible;

pub(super) trait Hosts<Profile: ExecutionProfile> {
    type Error;
    fn tables(&self, context: &Instructions<'_, '_, Profile::Graph>) -> Result<(), Self::Error>;
    fn function<Body: ExecutionFunctionBody>(
        &self,
        target: &Profile::HostTarget<Body>,
        contract: &Function<'_>,
        context: &Instructions<'_, '_, Profile::Graph>,
    ) -> Result<(), Self::Error>
    where
        Body::Return: Output;
    fn never(
        &self,
        target: &Profile::NeverHostTarget,
        contract: &Function<'_>,
        context: &Instructions<'_, '_, Profile::Graph>,
    ) -> Result<(), Self::Error>;
}

impl Hosts<Infallible> for InfallibleHosts {
    type Error = Infallible;
    fn tables(&self, _context: &Instructions<'_, '_, Infallible>) -> Result<(), Infallible> {
        Ok(())
    }
    fn function<Body: ExecutionFunctionBody>(
        &self,
        target: &Infallible,
        _contract: &Function<'_>,
        _context: &Instructions<'_, '_, Infallible>,
    ) -> Result<(), Infallible> {
        match *target {}
    }
    fn never(
        &self,
        target: &Infallible,
        _contract: &Function<'_>,
        _context: &Instructions<'_, '_, Infallible>,
    ) -> Result<(), Infallible> {
        match *target {}
    }
}

pub(super) struct InfallibleHosts;

#[derive(Debug, PartialEq, Eq)]
pub(super) struct FunctionError<HostError> {
    pub family: FunctionTableFamily,
    pub index: usize,
    pub kind: FunctionErrorKind<HostError>,
}

#[derive(Debug, PartialEq, Eq)]
pub(super) enum FunctionErrorKind<HostError> {
    Body(BodyError),
    Host(HostError),
    ListIdentity,
    ListType(CallError),
}

pub(super) fn all<'data, Profile: ExecutionProfile, Host: Hosts<Profile>>(
    tables: &'data FunctionTables<Profile>,
    context: &Instructions<'_, 'data, Profile::Graph>,
    hosts: &Host,
) -> Result<(), FunctionError<Host::Error>>
where
    <Profile::Graph as ExecutionGraphProfile>::ExternalFunctionId: Target,
    <Profile::Graph as ExecutionGraphProfile>::ExternalListFunctionId: Target,
    <<Profile::Graph as ExecutionGraphProfile>::ExternalListInstruction as ExternalListInstructionView>::FunctionLocal: Operand,
{
    let FunctionTables {
        value_returns,
        list_returns,
        function_returns,
    } = tables;
    let ValueFunctionTables {
        never_functions,
        int_functions,
        float_functions,
        string_functions,
        bit_array_functions,
        utf_codepoint_functions,
        custom_functions,
        external_functions,
        bool_functions,
        nil_functions,
        tuple_functions,
    } = value_returns;
    let ListFunctionTables {
        parameter_list_functions,
        int_list_functions,
        string_list_functions,
        bit_array_list_functions,
        utf_codepoint_list_functions,
        custom_list_functions,
        external_list_functions,
        float_list_functions,
        bool_list_functions,
        nil_list_functions,
        tuple_list_functions,
        parameter_list_list_functions,
        list_list_functions,
        function_list_functions,
    } = list_returns;
    let FunctionFunctionTables {
        int_function_functions,
        float_function_functions,
        string_function_functions,
        bit_array_function_functions,
        utf_codepoint_function_functions,
        custom_function_functions,
        external_function_functions,
        bool_function_functions,
        nil_function_functions,
        tuple_function_functions,
        generic_function_functions,
        never_function_functions,
        parameter_list_function_functions,
        parameter_list_list_function_functions,
        int_list_function_functions,
        string_list_function_functions,
        bit_array_list_function_functions,
        utf_codepoint_list_function_functions,
        custom_list_function_functions,
        external_list_function_functions,
        float_list_function_functions,
        bool_list_function_functions,
        nil_list_function_functions,
        tuple_list_function_functions,
        list_list_function_functions,
        function_list_function_functions,
        function_function_functions,
    } = function_returns;
    for (index, (value, contract)) in never_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::Never))
        .enumerate()
    {
        let family = FunctionTableFamily::Never;
        entry::<ExecutionNeverFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| hosts.never(target, contract, context),
        )?;
    }
    for (index, (value, contract)) in int_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::Int))
        .enumerate()
    {
        let family = FunctionTableFamily::Int;
        entry::<ExecutionIntFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionIntFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, (value, contract)) in float_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::Float))
        .enumerate()
    {
        let family = FunctionTableFamily::Float;
        entry::<ExecutionFloatFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionFloatFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, (value, contract)) in string_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::String))
        .enumerate()
    {
        let family = FunctionTableFamily::String;
        entry::<ExecutionStringFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionStringFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, (value, contract)) in bit_array_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::BitArray))
        .enumerate()
    {
        let family = FunctionTableFamily::BitArray;
        entry::<ExecutionBitArrayFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionBitArrayFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, (value, contract)) in utf_codepoint_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::UtfCodepoint))
        .enumerate()
    {
        let family = FunctionTableFamily::UtfCodepoint;
        entry::<ExecutionUtfCodepointFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionUtfCodepointFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in custom_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::Custom))
        .enumerate()
    {
        let family = FunctionTableFamily::Custom;
        entry::<ExecutionCustomFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCustomFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, (value, contract)) in external_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::External))
        .enumerate()
    {
        let family = FunctionTableFamily::External;
        entry::<ExecutionExternalFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionExternalFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, (value, contract)) in bool_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::Bool))
        .enumerate()
    {
        let family = FunctionTableFamily::Bool;
        entry::<ExecutionBoolFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionBoolFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, (value, contract)) in nil_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::Nil))
        .enumerate()
    {
        let family = FunctionTableFamily::Nil;
        entry::<ExecutionNilFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionNilFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, (value, contract)) in tuple_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::Tuple))
        .enumerate()
    {
        let family = FunctionTableFamily::Tuple;
        entry::<ExecutionTupleFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionTupleFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, ((id, value), contract)) in parameter_list_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::ParameterList))
        .enumerate()
    {
        let family = FunctionTableFamily::ParameterList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionParameterListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionParameterListFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, ((id, value), contract)) in int_list_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::IntList))
        .enumerate()
    {
        let family = FunctionTableFamily::IntList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionIntListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionIntListFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, ((id, value), contract)) in string_list_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::StringList))
        .enumerate()
    {
        let family = FunctionTableFamily::StringList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionStringListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts
                    .function::<ExecutionStringListFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, ((id, value), contract)) in bit_array_list_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::BitArrayList))
        .enumerate()
    {
        let family = FunctionTableFamily::BitArrayList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionBitArrayListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionBitArrayListFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, ((id, value), contract)) in utf_codepoint_list_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::UtfCodepointList),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::UtfCodepointList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionUtfCodepointListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionUtfCodepointListFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, ((id, value), contract)) in custom_list_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::CustomList))
        .enumerate()
    {
        let family = FunctionTableFamily::CustomList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionCustomListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts
                    .function::<ExecutionCustomListFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, ((id, value), contract)) in external_list_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::ExternalList))
        .enumerate()
    {
        let family = FunctionTableFamily::ExternalList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionExternalListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionExternalListFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, ((id, value), contract)) in float_list_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::FloatList))
        .enumerate()
    {
        let family = FunctionTableFamily::FloatList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionFloatListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionFloatListFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, ((id, value), contract)) in bool_list_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::BoolList))
        .enumerate()
    {
        let family = FunctionTableFamily::BoolList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionBoolListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionBoolListFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, ((id, value), contract)) in nil_list_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::NilList))
        .enumerate()
    {
        let family = FunctionTableFamily::NilList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionNilListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionNilListFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, ((id, value), contract)) in tuple_list_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::TupleList))
        .enumerate()
    {
        let family = FunctionTableFamily::TupleList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionTupleListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionTupleListFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, ((id, value), contract)) in parameter_list_list_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::ParameterListList),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::ParameterListList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionParameterListListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionParameterListListFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, ((id, value), contract)) in list_list_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::ListList))
        .enumerate()
    {
        let family = FunctionTableFamily::ListList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionListListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionListListFunctionBody<Profile>>(target, contract, context)
            },
        )?;
    }
    for (index, ((id, value), contract)) in function_list_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::FunctionList))
        .enumerate()
    {
        let family = FunctionTableFamily::FunctionList;
        list::<Host::Error>(
            id,
            id.index(),
            family,
            index,
            context.catalog,
            context.types,
        )?;
        entry::<ExecutionFunctionListFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionFunctionListFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in int_function_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::IntFunction))
        .enumerate()
    {
        let family = FunctionTableFamily::IntFunction;
        entry::<ExecutionIntFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionIntFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in float_function_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::FloatFunction))
        .enumerate()
    {
        let family = FunctionTableFamily::FloatFunction;
        entry::<ExecutionFloatFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionFloatFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in string_function_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::StringFunction))
        .enumerate()
    {
        let family = FunctionTableFamily::StringFunction;
        entry::<ExecutionStringFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionStringFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in bit_array_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::BitArrayFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::BitArrayFunction;
        entry::<ExecutionBitArrayFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionBitArrayFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in utf_codepoint_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::UtfCodepointFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::UtfCodepointFunction;
        entry::<ExecutionUtfCodepointFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionUtfCodepointFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in custom_function_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::CustomFunction))
        .enumerate()
    {
        let family = FunctionTableFamily::CustomFunction;
        entry::<ExecutionCustomFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCustomFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in external_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::ExternalFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::ExternalFunction;
        entry::<ExecutionExternalFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionExternalFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in bool_function_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::BoolFunction))
        .enumerate()
    {
        let family = FunctionTableFamily::BoolFunction;
        entry::<ExecutionBoolFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionBoolFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in nil_function_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::NilFunction))
        .enumerate()
    {
        let family = FunctionTableFamily::NilFunction;
        entry::<ExecutionNilFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionNilFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in tuple_function_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::TupleFunction))
        .enumerate()
    {
        let family = FunctionTableFamily::TupleFunction;
        entry::<ExecutionTupleFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionTupleFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in generic_function_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::GenericFunction))
        .enumerate()
    {
        let family = FunctionTableFamily::GenericFunction;
        entry::<ExecutionGenericFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionGenericFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in never_function_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::NeverFunction))
        .enumerate()
    {
        let family = FunctionTableFamily::NeverFunction;
        entry::<ExecutionNeverFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionNeverFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in parameter_list_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::ParameterListFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::ParameterListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in parameter_list_list_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::ParameterListListFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::ParameterListListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in int_list_function_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::IntListFunction))
        .enumerate()
    {
        let family = FunctionTableFamily::IntListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in string_list_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::StringListFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::StringListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in bit_array_list_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::BitArrayListFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::BitArrayListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in utf_codepoint_list_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::UtfCodepointListFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::UtfCodepointListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in custom_list_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::CustomListFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::CustomListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in external_list_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::ExternalListFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::ExternalListFunction;
        entry::<ExecutionExternalListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionExternalListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in float_list_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::FloatListFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::FloatListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in bool_list_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::BoolListFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::BoolListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in nil_list_function_functions
        .iter()
        .zip(context.catalog.family(FunctionTableFamily::NilListFunction))
        .enumerate()
    {
        let family = FunctionTableFamily::NilListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in tuple_list_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::TupleListFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::TupleListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in list_list_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::ListListFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::ListListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in function_list_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::FunctionListFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::FunctionListFunction;
        entry::<ExecutionCoreListFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionCoreListFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    for (index, (value, contract)) in function_function_functions
        .iter()
        .zip(
            context
                .catalog
                .family(FunctionTableFamily::FunctionFunction),
        )
        .enumerate()
    {
        let family = FunctionTableFamily::FunctionFunction;
        entry::<ExecutionFunctionFunctionFunctionBody<Profile>, _, _>(
            value,
            family,
            index,
            &contract,
            context,
            |target, contract| {
                hosts.function::<ExecutionFunctionFunctionFunctionBody<Profile>>(
                    target, contract, context,
                )
            },
        )?;
    }
    Ok(())
}

fn list<HostError>(
    target: &dyn Target,
    stored_index: usize,
    family: FunctionTableFamily,
    index: usize,
    catalog: &Catalog<'_>,
    types: &Types<'_>,
) -> Result<(), FunctionError<HostError>> {
    if stored_index != index {
        return Err(FunctionError {
            family,
            index,
            kind: FunctionErrorKind::ListIdentity,
        });
    }
    target
        .resolve(catalog, types)
        .map(|_| ())
        .map_err(|error| FunctionError {
            family,
            index,
            kind: FunctionErrorKind::ListType(error),
        })
}

fn entry<'data, Body, Entry, HostError>(
    value: &'data Entry,
    family: FunctionTableFamily,
    index: usize,
    contract: &Function<'data>,
    context: &Instructions<'_, 'data, Body::Graph>,
    host: impl FnOnce(&Entry::HostTarget, &Function<'data>) -> Result<(), HostError>,
) -> Result<(), FunctionError<HostError>>
where
    Body: body::Contract + 'data,
    Body::Return: Operand,
    Body::TailCall: Tail,
    Entry: ExecutionFunctionEntry<Body>,
    <Body::Graph as ExecutionGraphProfile>::ExternalFunctionId: Target,
    <Body::Graph as ExecutionGraphProfile>::ExternalListFunctionId: Target,
    <<Body::Graph as ExecutionGraphProfile>::ExternalListInstruction as ExternalListInstructionView>::FunctionLocal: Operand,
{
    match value.as_ref() {
        ExecutionFunctionRef::Graph(value) => {
            body::function(value.body(), value.entry(), family, contract, context).map_err(
                |error| FunctionError {
                    family,
                    index,
                    kind: FunctionErrorKind::Body(error),
                },
            )
        }
        ExecutionFunctionRef::Host(target) => {
            host(target, contract).map_err(|error| FunctionError {
                family,
                index,
                kind: FunctionErrorKind::Host(error),
            })
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::hosts::NativeFunctions;
    use super::super::tests::{lowered_native, native_hosts};
    use super::super::{catalog::Catalog, constant, source::Sources, type_::Types};
    use super::{
        CallError, FunctionError, FunctionErrorKind, FunctionTableFamily, InfallibleHosts,
        Instructions, all, list,
    };
    use crate::plan::execution::host::HostedExecutionProfile;

    #[test]
    fn each_list_table_rejects_a_row_stored_under_a_different_identity() {
        use super::super::tests::owned_mut;
        use super::ListFunctionTables;
        type Mutation = fn(&mut ListFunctionTables<HostedExecutionProfile>);
        let cases: &[(FunctionTableFamily, &str, Mutation)] = &[
            (FunctionTableFamily::ParameterList, "[]", |tables| {
                owned_mut(&mut tables.parameter_list_functions)[0].0.index = 99
            }),
            (FunctionTableFamily::IntList, "[42]", |tables| {
                owned_mut(&mut tables.int_list_functions)[0].0.index = 99
            }),
            (FunctionTableFamily::StringList, r#"["text"]"#, |tables| {
                owned_mut(&mut tables.string_list_functions)[0].0.index = 99
            }),
            (FunctionTableFamily::BitArrayList, "[<<42>>]", |tables| {
                owned_mut(&mut tables.bit_array_list_functions)[0].0.index = 99
            }),
            (
                FunctionTableFamily::UtfCodepointList,
                "[point()]",
                |tables| {
                    owned_mut(&mut tables.utf_codepoint_list_functions)[0]
                        .0
                        .index = 99
                },
            ),
            (FunctionTableFamily::CustomList, "[Box(42)]", |tables| {
                owned_mut(&mut tables.custom_list_functions)[0].0.index = 99
            }),
            (FunctionTableFamily::ExternalList, "[key()]", |tables| {
                owned_mut(&mut tables.external_list_functions)[0].0.index = 99
            }),
            (FunctionTableFamily::FloatList, "[1.5]", |tables| {
                owned_mut(&mut tables.float_list_functions)[0].0.index = 99
            }),
            (FunctionTableFamily::BoolList, "[True]", |tables| {
                owned_mut(&mut tables.bool_list_functions)[0].0.index = 99
            }),
            (FunctionTableFamily::NilList, "[Nil]", |tables| {
                owned_mut(&mut tables.nil_list_functions)[0].0.index = 99
            }),
            (FunctionTableFamily::TupleList, "[#(42, True)]", |tables| {
                owned_mut(&mut tables.tuple_list_functions)[0].0.index = 99
            }),
            (FunctionTableFamily::ParameterListList, "[[]]", |tables| {
                owned_mut(&mut tables.parameter_list_list_functions)[0]
                    .0
                    .index = 99
            }),
            (FunctionTableFamily::ListList, "[[42]]", |tables| {
                owned_mut(&mut tables.list_list_functions)[0].0.index = 99
            }),
            (
                FunctionTableFamily::FunctionList,
                "[fn(x) { x + 1 }]",
                |tables| owned_mut(&mut tables.function_list_functions)[0].0.index = 99,
            ),
        ];
        for (family, expression, mutate) in cases {
            let source = format!(
                "pub type Box(a) {{ Box(a) }} pub type Key \
                 @external(erlang, \"native\", \"key\") fn key() -> Key \
                 fn point() {{ let assert <<point:utf8_codepoint>> = <<65>> point }} pub fn main() {{ {expression} }}"
            );
            let (mut program, values, nevers) = lowered_native(&source);
            mutate(&mut owned_mut(&mut program.functions).list_returns);
            let common = &program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let catalog =
                Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
            let sources = Sources::admit(common.root, &common.modules).unwrap();
            let context = Instructions {
                types: &types,
                catalog: &catalog,
                sources: &sources,
                constants: &common.constants,
            };
            assert_eq!(
                all(
                    &program.functions,
                    &context,
                    &NativeFunctions::new(&values, &nevers, native_hosts()).unwrap()
                ),
                Err(FunctionError {
                    family: *family,
                    index: 0,
                    kind: FunctionErrorKind::ListIdentity
                })
            );
        }
    }

    #[test]
    fn native_entry_links_are_validated_before_the_runtime_owner_exists() {
        use super::super::{
            hosts::{NativeError, NativeFunctions},
            tests::owned_mut,
        };
        use crate::host::{HostProviderModule, HostProviderSet, StatelessHostProfile};
        use crate::plan::execution::function::ValueFunctionEntry;
        use crate::plan::execution::host::HostedFunctionTarget;
        let hosts = || {
            HostProviderSet::<StatelessHostProfile>::from_providers([HostProviderModule::new(
                "app", "main",
            )
            .unwrap()
            .with_function("native", |value: num_bigint::BigInt| value)
            .unwrap()])
            .unwrap()
        };
        let typed = crate::compile_typed_host_program(
            "app",
            "main",
            [crate::PackageSource::new(
                "app",
                Vec::<&str>::new(),
                [crate::ModuleSource::new(
                    "main",
                    "src/main.gleam",
                    r#"
@external(erlang, "native", "native")
fn native(value: Int) -> Int
pub fn main() { native(42) }
"#,
                )],
            )],
            hosts(),
        )
        .unwrap();
        let (mut program, functions) = crate::plan::execution::lowering::lower_hosted(
            crate::plan_host_program(typed).unwrap(),
        )
        .unwrap();
        let (values, nevers) = functions.into_metadata();
        let values = values
            .into_vec()
            .into_iter()
            .map(|value| std::sync::Arc::try_unwrap(value).ok().unwrap())
            .collect::<Vec<_>>();
        assert!(nevers.is_empty());
        let corrupted = owned_mut(
            &mut owned_mut(&mut program.functions)
                .value_returns
                .int_functions,
        )
        .iter_mut()
        .enumerate()
        .filter_map(|(index, entry)| match entry {
            ValueFunctionEntry::Host(HostedFunctionTarget::Value(target)) => {
                target.index = 99;
                Some(index)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
        assert_eq!(corrupted.len(), 1);
        let common = &program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &program.functions, &types).unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        let linked = NativeFunctions::new(&values, &[], hosts()).unwrap();
        assert_eq!(
            all(&program.functions, &context, &linked),
            Err(FunctionError {
                family: FunctionTableFamily::Int,
                index: corrupted[0],
                kind: FunctionErrorKind::Host(NativeError::MissingValue(99))
            })
        );
    }

    #[test]
    fn list_table_identity_and_stored_type_have_distinct_admission_errors() {
        use crate::plan::execution::function::IntListFunctionId;
        use crate::plan::execution::type_::{IntListTypeId, ListTypeId};
        use std::convert::Infallible;

        let typed =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { [42] }")
                .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let table = &plan.program.functions.list_returns.int_list_functions;
        assert_eq!(table.len(), 1);
        let (id, _) = table.iter().next().unwrap();
        assert_eq!(
            list::<Infallible>(
                id,
                id.index(),
                FunctionTableFamily::IntList,
                0,
                &catalog,
                &types
            ),
            Ok(())
        );
        assert_eq!(
            list::<Infallible>(id, 1, FunctionTableFamily::IntList, 0, &catalog, &types),
            Err(FunctionError {
                family: FunctionTableFamily::IntList,
                index: 0,
                kind: FunctionErrorKind::ListIdentity
            })
        );
        let wrong = IntListFunctionId {
            index: 0,
            type_id: IntListTypeId::new(ListTypeId(99)),
        };
        assert_eq!(
            list::<Infallible>(&wrong, 0, FunctionTableFamily::IntList, 0, &catalog, &types),
            Err(FunctionError {
                family: FunctionTableFamily::IntList,
                index: 0,
                kind: FunctionErrorKind::ListType(CallError::Type(
                    super::super::type_::TypeError::MissingList { index: 99 }
                )),
            })
        );
    }

    #[test]
    fn preserves_refinements_through_aliases_projections_and_exhaustive_matches() {
        for source in [
            r#"
type Item { Empty One(Int) Pair(left: Int, right: Int) }
fn read(item: Item, fallback: Item) {
  case item, fallback {
    Pair(left, right) as pair, _ if left > 0 -> {
      let captured = fn() {
        case pair {
          Pair(a, b) -> a + b
          _ -> 0
        }
      }
      captured()
    }
    One(n), _ | _, One(n) -> n
    _, _ -> 0
  }
}
pub fn main() { read(Pair(20, 22), Empty) }
"#,
            r#"
type Maybe(a) { Some(a) None }
fn read(value: Maybe(Maybe(Int))) {
  case value {
    None -> 0
    Some(None) -> 0
    Some(Some(n)) -> n
  }
}
pub fn main() { read(Some(Some(42))) }
"#,
            r#"
type Maybe(a) { Some(a) None }
fn read(values: List(Int), maybe: Maybe(Int)) {
  case values, maybe {
    [], _ -> 0
    [_, ..], None -> 0
    [first, ..], Some(second) -> first + second
  }
}
pub fn main() { read([20], Some(22)) }
"#,
            r#"
type Box(a) { Box(a) }
pub fn main() {
  let assert [_, ..tail] = [Box(0), Box(42)]
  let assert [Box(n)] = tail
  n
}
"#,
            r#"
type Item { Empty Pair(left: Int, right: Int) }
fn read(value: #(Item, List(String))) {
  case value {
    #(Pair(a, b), ["prefix:" <> _, ..]) -> a + b
    _ -> 0
  }
}
pub fn main() { read(#(Pair(20, 22), ["prefix:value"])) }
"#,
        ] {
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            assert_admitted_plan(&plan);
            assert_eq!(
                crate::run_main(&plan, &mut Vec::new()).unwrap(),
                crate::Value::Int(42.into()),
                "{source}",
            );
        }
    }

    #[test]
    fn admits_declared_constructors_without_materialized_rows_in_divergent_functions() {
        let source = r#"
type Box(a) { Box(a) }
fn diverge(_value: Int) -> Box(a) { Box(panic) }
fn handoff() -> fn(Int) -> Box(a) { diverge }
pub fn main() { let _ = handoff() 42 }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        assert!(
            plan.program
                .common
                .custom_types
                .types
                .iter()
                .any(|descriptor| {
                    descriptor.constructor_count == 1 && descriptor.constructors.is_empty()
                })
        );
        assert_admitted_plan(&plan);
        assert_eq!(
            crate::run_main(&plan, &mut Vec::new()).unwrap(),
            crate::Value::Int(42.into()),
        );
    }
    #[test]
    fn checks_every_lowered_table_in_complete_plain_programs() {
        for source in [
            "pub fn main() { #(42, 1.5, \"text\", <<42>>, True, Nil) }",
            "fn id(x) { x } pub fn main() { #(id(42), id(1.5), id(\"text\"), id(<<42>>), id(True), id(Nil), id(#(42, True))) }",
            "fn id(x) { x } pub fn main() { #(id([42]), id([1.5]), id([\"text\"]), id([<<42>>]), id([True]), id([Nil]), id([#(42, True)]), id([[42]]), id([fn(x) { x + 1 }])) }",
            "fn id(x) { x } pub type Box(a) { Box(a) } pub fn main() { #(id(Box(42)), id([Box(42)]), id([]), id([[]])) }",
            "fn id(x) { x } pub fn main() { #(id(fn() { 42 }), id(fn() { 1.5 }), id(fn() { \"text\" }), id(fn() { <<42>> }), id(fn() { True }), id(fn() { Nil }), id(fn() { #(42, True) })) }",
            "fn id(x) { x } pub fn main() { #(id(fn() { [42] }), id(fn() { [1.5] }), id(fn() { [\"text\"] }), id(fn() { [<<42>>] }), id(fn() { [True] }), id(fn() { [Nil] }), id(fn() { [#(42, True)] }), id(fn() { [[42]] }), id(fn() { [fn(x) { x }] })) }",
            "fn id(x) { x } pub type Box(a) { Box(a) } pub fn main() { #(id(fn() { Box(42) }), id(fn() { [Box(42)] }), id(fn() { [] }), id(fn() { [[]] }), id(fn() { fn(x) { x } }), id(fn(x) { x }), id(fn() { panic })) }",
            "fn choose(x) { case x { 0 -> 42 _ -> choose(x - 1) } } pub fn main() { choose(4) }",
            "pub type Choice { First Second(String) Third(Int) } fn read(x) { case x { First -> 0 Second(_) -> 1 Third(n) -> n } } pub fn main() { read(Third(42)) }",
            "pub type Box(a) { Box(a) } fn read(x) { let assert Box([n, ..]) as original = x #(n, original) } pub fn main() { read(Box([42])) }",
            "fn read(x) { let assert [first, ..rest] as original = x #(first, rest, original) } pub fn main() { read([42, 43]) }",
            "fn read(x) { let assert <<size, value:size(size)>> = x value } pub fn main() { read(<<8, 42>>) }",
            "fn read(x) { let assert \"a\" <> rest = x case rest { \"b\" -> 42 _ -> 0 } } pub fn main() { read(\"ab\") }",
            "const i = 42 const f = 1.5 const s = \"text\" const b = <<42>> const t = #(i, f, s, b, True, Nil) pub fn main() { t }",
            "const xs = [42] const ys = [[42]] const zs = [#(42, True)] pub fn main() { #(xs, ys, zs) }",
            "pub type Box(a) { Box(a) } const box = Box(42) const boxes = [box] pub fn main() { #(box, boxes) }",
            "fn stop(x: Int) -> a { panic } pub fn main() -> Int { let f = stop let _ = f(42) 0 }",
            "fn make(offset) { fn(value) { offset + value } } pub fn main() { make(20)(22) }",
            "pub fn main() { echo 42 }",
        ] {
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            assert_admitted_plan(&plan);
        }
    }

    #[test]
    fn every_function_family_rejects_a_capture_contract_absent_from_its_body() {
        use super::super::block::BlockError;
        use super::BodyError;
        use crate::plan::execution::function::FunctionCatalog;
        use crate::plan::execution::graph::{IntLocalId, ParamLocal, ParamSlot};
        use crate::plan::execution::type_::{ValueShapeId, ValueType};

        let cases = [
            (FunctionTableFamily::Never, r#"panic"#),
            (FunctionTableFamily::Int, r#"42"#),
            (FunctionTableFamily::Float, r#"1.5"#),
            (FunctionTableFamily::String, r#""text""#),
            (FunctionTableFamily::BitArray, r#"<<42>>"#),
            (
                FunctionTableFamily::UtfCodepoint,
                r#"{ let assert <<value:utf8_codepoint>> = <<97>> value }"#,
            ),
            (FunctionTableFamily::Custom, r#"Box(42)"#),
            (FunctionTableFamily::External, r#"key()"#),
            (FunctionTableFamily::Bool, r#"True"#),
            (FunctionTableFamily::Nil, r#"Nil"#),
            (FunctionTableFamily::Tuple, r#"#(42, True)"#),
            (FunctionTableFamily::ParameterList, r#"[]"#),
            (FunctionTableFamily::IntList, r#"[42]"#),
            (FunctionTableFamily::StringList, r#"["text"]"#),
            (FunctionTableFamily::BitArrayList, r#"[<<42>>]"#),
            (FunctionTableFamily::UtfCodepointList, r#"[codepoint()]"#),
            (FunctionTableFamily::CustomList, r#"[Box(42)]"#),
            (FunctionTableFamily::ExternalList, r#"[key()]"#),
            (FunctionTableFamily::FloatList, r#"[1.5]"#),
            (FunctionTableFamily::BoolList, r#"[True]"#),
            (FunctionTableFamily::NilList, r#"[Nil]"#),
            (FunctionTableFamily::TupleList, r#"[#(42, True)]"#),
            (FunctionTableFamily::ParameterListList, r#"[[]]"#),
            (FunctionTableFamily::ListList, r#"[[42]]"#),
            (FunctionTableFamily::FunctionList, r#"[fn(x) { x + 1 }]"#),
            (FunctionTableFamily::IntFunction, r#"fn() { 42 }"#),
            (FunctionTableFamily::FloatFunction, r#"fn() { 1.5 }"#),
            (FunctionTableFamily::StringFunction, r#"fn() { "text" }"#),
            (FunctionTableFamily::BitArrayFunction, r#"fn() { <<42>> }"#),
            (
                FunctionTableFamily::UtfCodepointFunction,
                r#"fn() { codepoint() }"#,
            ),
            (FunctionTableFamily::CustomFunction, r#"fn() { Box(42) }"#),
            (FunctionTableFamily::ExternalFunction, r#"fn() { key() }"#),
            (FunctionTableFamily::BoolFunction, r#"fn() { True }"#),
            (FunctionTableFamily::NilFunction, r#"fn() { Nil }"#),
            (
                FunctionTableFamily::TupleFunction,
                r#"fn() { #(42, True) }"#,
            ),
            (FunctionTableFamily::GenericFunction, r#"fn(x) { x }"#),
            (FunctionTableFamily::NeverFunction, r#"fn() { panic }"#),
            (FunctionTableFamily::ParameterListFunction, r#"fn() { [] }"#),
            (
                FunctionTableFamily::ParameterListListFunction,
                r#"fn() { [[]] }"#,
            ),
            (FunctionTableFamily::IntListFunction, r#"fn() { [42] }"#),
            (
                FunctionTableFamily::StringListFunction,
                r#"fn() { ["text"] }"#,
            ),
            (
                FunctionTableFamily::BitArrayListFunction,
                r#"fn() { [<<42>>] }"#,
            ),
            (
                FunctionTableFamily::UtfCodepointListFunction,
                r#"fn() { [codepoint()] }"#,
            ),
            (
                FunctionTableFamily::CustomListFunction,
                r#"fn() { [Box(42)] }"#,
            ),
            (
                FunctionTableFamily::ExternalListFunction,
                r#"fn() { [key()] }"#,
            ),
            (FunctionTableFamily::FloatListFunction, r#"fn() { [1.5] }"#),
            (FunctionTableFamily::BoolListFunction, r#"fn() { [True] }"#),
            (FunctionTableFamily::NilListFunction, r#"fn() { [Nil] }"#),
            (
                FunctionTableFamily::TupleListFunction,
                r#"fn() { [#(42, True)] }"#,
            ),
            (FunctionTableFamily::ListListFunction, r#"fn() { [[42]] }"#),
            (
                FunctionTableFamily::FunctionListFunction,
                r#"fn() { [fn(x) { x + 1 }] }"#,
            ),
            (
                FunctionTableFamily::FunctionFunction,
                r#"fn() { fn(x) { x + 1 } }"#,
            ),
        ];
        for (family, expression) in cases {
            let source = format!(
                "pub type Box(a) {{ Box(a) }} pub type Key \
                 @external(erlang, \"native\", \"key\") fn key() -> Key \
                 fn codepoint() {{ let assert <<value:utf8_codepoint>> = <<97>> value }} \
                 pub fn main() {{ echo 42 {expression} }}"
            );
            let (program, values, nevers) = lowered_native(&source);
            let common = &program.common;
            let types = Types::admit(
                &common.list_types,
                &common.custom_types,
                &common.external_types,
                &common.value_shapes,
            )
            .unwrap();
            let original = &common.function_parameters;
            let sources = Sources::admit(common.root, &common.modules).unwrap();
            let linked = NativeFunctions::new(&values, &nevers, native_hosts()).unwrap();
            {
                let catalog = Catalog::admit(original, &program.functions, &types).unwrap();
                let context = Instructions {
                    types: &types,
                    catalog: &catalog,
                    sources: &sources,
                    constants: &common.constants,
                };
                assert_eq!(all(&program.functions, &context, &linked), Ok(()));
            }
            let range = original.families[family as usize].clone();
            assert_eq!(
                range.len(),
                if family == FunctionTableFamily::External {
                    2
                } else {
                    1
                },
                "{family:?}: {source}"
            );
            let int_shape = ValueShapeId(
                common
                    .value_shapes
                    .shape_types
                    .iter()
                    .position(|type_| type_ == &ValueType::Int)
                    .unwrap(),
            );
            let mut contracts = original.functions.to_vec();
            assert!(contracts[range.start].captures.is_empty(), "{family:?}");
            contracts[range.start].captures =
                vec![ParamSlot::new(ParamLocal::Int(IntLocalId(0)), int_shape)].into();
            let changed = FunctionCatalog {
                families: original.families.clone(),
                functions: contracts.into(),
                parameters: original.parameters.clone(),
            };
            let catalog = Catalog::admit(&changed, &program.functions, &types).unwrap();
            let context = Instructions {
                types: &types,
                catalog: &catalog,
                sources: &sources,
                constants: &common.constants,
            };
            assert_eq!(
                all(&program.functions, &context, &linked),
                Err(FunctionError {
                    family,
                    index: 0,
                    kind: FunctionErrorKind::Body(BodyError::Block(BlockError::CaptureCount {
                        expected: 1,
                        found: 0
                    })),
                }),
                "{family:?}: {source}",
            );
        }
    }

    fn assert_admitted_plan(plan: &crate::ExecutionPlan) {
        let common = &plan.program.common;
        let types = Types::admit(
            &common.list_types,
            &common.custom_types,
            &common.external_types,
            &common.value_shapes,
        )
        .unwrap();
        let catalog =
            Catalog::admit(&common.function_parameters, &plan.program.functions, &types).unwrap();
        let sources = Sources::admit(common.root, &common.modules).unwrap();
        let context = Instructions {
            types: &types,
            catalog: &catalog,
            sources: &sources,
            constants: &common.constants,
        };
        assert_eq!(
            all(&plan.program.functions, &context, &InfallibleHosts),
            Ok(())
        );
        assert_eq!(constant::all(&common.constants, &context), Ok(()));
    }
}
