use super::{HostFunctionId, HostNeverFunctionId, HostedFunction};
use crate::host::{
    AsyncHostCallError, AsyncHostCallErrorKind, AsyncHostCallback, AsyncHostFunctionCallback,
    AsyncHostFunctionKind, HostProfile, OwnedHostCallback, OwnedHostFunctionImplementation,
    ScopedAsyncHostCallback,
};
use crate::plan::execution::function::{
    ExecutionBitArrayFunctionBody, ExecutionBoolFunctionBody, ExecutionFloatFunctionBody,
    ExecutionIntFunctionBody, ExecutionNilFunctionBody, ExecutionStringFunctionBody,
    ExecutionUtfCodepointFunctionBody,
};
use crate::{BitArrayValue, HostFailure};
use ecow::EcoString;
use num_bigint::BigInt;
use std::sync::Arc;

pub(crate) enum ResumableHostCallback<Profile: HostProfile, Return> {
    Immediate(OwnedHostCallback<Profile, Return>),
    AsyncOwned(Arc<AsyncHostCallback<Return>>),
    AsyncScoped(Arc<ScopedAsyncHostCallback<Profile, Return>>),
}

type AsyncHostedFunction<Profile, Return> = HostedFunction<ResumableHostCallback<Profile, Return>>;
type AsyncHostedNeverFunction<Profile> =
    HostedFunction<OwnedHostCallback<Profile, std::convert::Infallible>>;

pub(crate) type AsyncHostedExternalFunction<Profile> = HostedFunction<
    Arc<ScopedAsyncHostCallback<Profile, crate::runtime::TransferExternalPayloadLease>>,
>;

pub(crate) struct AsyncHostFunctionTables<Profile: HostProfile> {
    ints: Box<[AsyncHostedFunction<Profile, BigInt>]>,
    floats: Box<[AsyncHostedFunction<Profile, f64>]>,
    strings: Box<[AsyncHostedFunction<Profile, EcoString>]>,
    bit_arrays: Box<[AsyncHostedFunction<Profile, BitArrayValue>]>,
    utf_codepoints: Box<[AsyncHostedFunction<Profile, char>]>,
    bools: Box<[AsyncHostedFunction<Profile, bool>]>,
    nils: Box<[AsyncHostedFunction<Profile, ()>]>,
    externals: Box<[AsyncHostedExternalFunction<Profile>]>,
    nevers: Box<[AsyncHostedNeverFunction<Profile>]>,
}

pub(crate) struct AsyncHostFunctionTableBuilder<Profile: HostProfile> {
    ints: Vec<AsyncHostedFunction<Profile, BigInt>>,
    floats: Vec<AsyncHostedFunction<Profile, f64>>,
    strings: Vec<AsyncHostedFunction<Profile, EcoString>>,
    bit_arrays: Vec<AsyncHostedFunction<Profile, BitArrayValue>>,
    utf_codepoints: Vec<AsyncHostedFunction<Profile, char>>,
    bools: Vec<AsyncHostedFunction<Profile, bool>>,
    nils: Vec<AsyncHostedFunction<Profile, ()>>,
    externals: Vec<AsyncHostedExternalFunction<Profile>>,
    nevers: Vec<AsyncHostedNeverFunction<Profile>>,
}

#[derive(Clone, Copy)]
pub(crate) enum AsyncHostFunctionIndex {
    Int(usize),
    Float(usize),
    String(usize),
    BitArray(usize),
    UtfCodepoint(usize),
    Bool(usize),
    Nil(usize),
    External {
        index: usize,
        type_: crate::plan::execution::type_::ExternalTypeId,
    },
}

#[derive(Clone, Copy)]
pub(crate) enum ImmediateHostFunctionIndex {
    Value(AsyncHostFunctionIndex),
    Never(usize),
}

impl<Profile: HostProfile> AsyncHostFunctionTableBuilder<Profile> {
    pub(in crate::plan::execution) fn new() -> Self {
        Self {
            ints: Vec::new(),
            floats: Vec::new(),
            strings: Vec::new(),
            bit_arrays: Vec::new(),
            utf_codepoints: Vec::new(),
            bools: Vec::new(),
            nils: Vec::new(),
            externals: Vec::new(),
            nevers: Vec::new(),
        }
    }

    pub(in crate::plan::execution) fn push_async(
        &mut self,
        metadata: super::HostedFunctionMetadata,
        implementation: &AsyncHostFunctionKind<Profile>,
    ) -> AsyncHostFunctionIndex {
        macro_rules! push_async {
            ($field:ident, $function:expr, $index:ident) => {{
                let index = self.$field.len();
                let callback = match $function {
                    AsyncHostFunctionCallback::Owned(function) => {
                        ResumableHostCallback::AsyncOwned(Arc::clone(function))
                    }
                    AsyncHostFunctionCallback::Scoped(function) => {
                        ResumableHostCallback::AsyncScoped(Arc::clone(function))
                    }
                };
                self.$field.push(HostedFunction::new(metadata, callback));
                AsyncHostFunctionIndex::$index(index)
            }};
        }

        match implementation {
            AsyncHostFunctionKind::Int(function) => push_async!(ints, function, Int),
            AsyncHostFunctionKind::Float(function) => push_async!(floats, function, Float),
            AsyncHostFunctionKind::String(function) => push_async!(strings, function, String),
            AsyncHostFunctionKind::BitArray(function) => {
                push_async!(bit_arrays, function, BitArray)
            }
            AsyncHostFunctionKind::UtfCodepoint(function) => {
                push_async!(utf_codepoints, function, UtfCodepoint)
            }
            AsyncHostFunctionKind::Bool(function) => push_async!(bools, function, Bool),
            AsyncHostFunctionKind::Nil(function) => push_async!(nils, function, Nil),
            AsyncHostFunctionKind::External(function) => {
                let index = self.externals.len();
                let type_ = metadata
                    .constructions()
                    .external(metadata.signature().return_());
                self.externals
                    .push(HostedFunction::new(metadata, Arc::clone(function)));
                AsyncHostFunctionIndex::External { index, type_ }
            }
        }
    }

    pub(in crate::plan::execution) fn push_immediate(
        &mut self,
        metadata: super::HostedFunctionMetadata,
        implementation: &OwnedHostFunctionImplementation<Profile>,
    ) -> ImmediateHostFunctionIndex {
        macro_rules! push_value {
            ($field:ident, $function:expr, $index:ident) => {{
                let index = self.$field.len();
                self.$field.push(HostedFunction::new(
                    metadata,
                    ResumableHostCallback::Immediate($function.clone()),
                ));
                ImmediateHostFunctionIndex::Value(AsyncHostFunctionIndex::$index(index))
            }};
        }

        match implementation {
            OwnedHostFunctionImplementation::Never(function) => {
                let index = self.nevers.len();
                self.nevers
                    .push(HostedFunction::new(metadata, function.clone()));
                ImmediateHostFunctionIndex::Never(index)
            }
            OwnedHostFunctionImplementation::Int(function) => {
                push_value!(ints, function, Int)
            }
            OwnedHostFunctionImplementation::Float(function) => {
                push_value!(floats, function, Float)
            }
            OwnedHostFunctionImplementation::String(function) => {
                push_value!(strings, function, String)
            }
            OwnedHostFunctionImplementation::BitArray(function) => {
                push_value!(bit_arrays, function, BitArray)
            }
            OwnedHostFunctionImplementation::UtfCodepoint(function) => {
                push_value!(utf_codepoints, function, UtfCodepoint)
            }
            OwnedHostFunctionImplementation::Bool(function) => {
                push_value!(bools, function, Bool)
            }
            OwnedHostFunctionImplementation::Nil(function) => {
                push_value!(nils, function, Nil)
            }
        }
    }

    pub(in crate::plan::execution) fn finish(self) -> AsyncHostFunctionTables<Profile> {
        AsyncHostFunctionTables {
            ints: self.ints.into_boxed_slice(),
            floats: self.floats.into_boxed_slice(),
            strings: self.strings.into_boxed_slice(),
            bit_arrays: self.bit_arrays.into_boxed_slice(),
            utf_codepoints: self.utf_codepoints.into_boxed_slice(),
            bools: self.bools.into_boxed_slice(),
            nils: self.nils.into_boxed_slice(),
            externals: self.externals.into_boxed_slice(),
            nevers: self.nevers.into_boxed_slice(),
        }
    }
}

macro_rules! async_table_access {
    ($method:ident, $body:ty, $return:ty, $field:ident) => {
        pub(crate) fn $method(
            &self,
            id: &HostFunctionId<$body>,
        ) -> &AsyncHostedFunction<Profile, $return> {
            &self.$field[id.index()]
        }
    };
}

impl<Profile: HostProfile> AsyncHostFunctionTables<Profile> {
    pub(crate) fn external(
        &self,
        id: &HostFunctionId<
            crate::plan::execution::function::ExecutionExternalFunctionBody<
                super::AsyncHostedExecutionProfile,
            >,
        >,
    ) -> &HostedFunction<
        Arc<ScopedAsyncHostCallback<Profile, crate::runtime::TransferExternalPayloadLease>>,
    > {
        &self.externals[id.index()]
    }
    async_table_access!(
        int,
        ExecutionIntFunctionBody<super::AsyncHostedExecutionProfile>,
        BigInt,
        ints
    );
    async_table_access!(
        float,
        ExecutionFloatFunctionBody<super::AsyncHostedExecutionProfile>,
        f64,
        floats
    );
    async_table_access!(
        string,
        ExecutionStringFunctionBody<super::AsyncHostedExecutionProfile>,
        EcoString,
        strings
    );
    async_table_access!(
        bit_array,
        ExecutionBitArrayFunctionBody<super::AsyncHostedExecutionProfile>,
        BitArrayValue,
        bit_arrays
    );
    async_table_access!(
        utf_codepoint,
        ExecutionUtfCodepointFunctionBody<super::AsyncHostedExecutionProfile>,
        char,
        utf_codepoints
    );
    async_table_access!(
        bool_,
        ExecutionBoolFunctionBody<super::AsyncHostedExecutionProfile>,
        bool,
        bools
    );
    async_table_access!(
        nil,
        ExecutionNilFunctionBody<super::AsyncHostedExecutionProfile>,
        (),
        nils
    );

    pub(crate) fn never(&self, id: HostNeverFunctionId) -> &AsyncHostedNeverFunction<Profile> {
        &self.nevers[id.index()]
    }
}

pub(crate) fn call_resumable_never<Profile: HostProfile>(
    function: &AsyncHostedNeverFunction<Profile>,
    state: &mut Profile::RunState,
    arguments: &dyn crate::host::HostCallArguments,
) -> Result<std::convert::Infallible, HostFailure> {
    function.implementation().call(state, arguments)
}

pub(crate) fn async_host_failure(
    error: AsyncHostCallError,
) -> Result<HostFailure, crate::runtime::TransferExecutionError> {
    match error.into_kind() {
        AsyncHostCallErrorKind::Failure(failure) => Ok(failure),
        AsyncHostCallErrorKind::Nested(error) => Err(error),
    }
}
