mod callable;
mod capture;
mod compound;
mod native;
mod nullary;

pub use compound::{CallCustom, CallTuple, CompoundField};
pub use native::{
    CustomNativeExecution, CustomNativeRequest, StringNativeExecution, StringNativeRequest,
    TupleNativeExecution, TupleNativeRequest,
};
pub(in crate::runtime) use native::{GeneratedNativePhase, GeneratedNativeState};
pub use nullary::CallNullary;

pub use callable::{
    BitArrayCallable, BoolCallable, FloatCallable, IntCallable, NilCallable, StringCallable,
    UtfCodepointCallable,
};

use crate::StringValue;
use crate::plan::HostCallSite;
use crate::plan::execution::compiled::{CallTarget, CompiledCheckpoint};
use crate::plan::execution::function::FunctionReturnFamily;
use crate::plan::execution::function::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BoolFunctionFunctionId, BoolFunctionId,
    CustomFunctionId, FloatFunctionFunctionId, FloatFunctionId, IntFunctionFunctionId,
    IntFunctionId, NilFunctionFunctionId, NilFunctionId, StringFunctionFunctionId,
    StringFunctionId, TupleFunctionId, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
};
use crate::runtime::CaptureStorage;
use crate::runtime::captures::Captures;
use crate::runtime::compiled::bit_array::BitArrayValues;
use crate::runtime::compiled::int_list::{IntList, IntListOps};
use crate::runtime::compiled::native_calls::{CallNativeFailure, CallNativeOps};
use crate::runtime::compiled::numeric::NumericValues;
use crate::runtime::compiled::primitive_list::{
    BitArrayList, BoolList, FloatList, NilList, PrimitiveListOps, StringList, UtfCodepointList,
};
use crate::runtime::compiled::string::StringValues;
use crate::runtime::evaluated::{EvaluatedBitArray, EvaluatedCapture};
use crate::runtime::graph::BlockEnvironment;
use crate::runtime::integer::IntegerValue;
use crate::runtime::state::list::RuntimeListStorage;

/// Canonical-boundary columns. Ordinary generated steps keep concrete locals
/// and carry only the actual result on completion.
#[derive(Default)]
pub struct CallValues {
    pub customs: Vec<CallCustom>,
    pub tuples: Vec<CallTuple>,
    pub ints: Vec<CallInteger>,
    pub bools: Vec<bool>,
    pub floats: Vec<f64>,
    pub strings: Vec<StringValue>,
    pub bit_arrays: Vec<CallBitArray>,
    pub utf_codepoints: Vec<char>,
    pub int_lists: Vec<IntList>,
    pub bool_lists: Vec<BoolList>,
    pub float_lists: Vec<FloatList>,
    pub string_lists: Vec<StringList>,
    pub bit_array_lists: Vec<BitArrayList>,
    pub utf_codepoint_lists: Vec<UtfCodepointList>,
    pub nil_lists: Vec<NilList>,
    pub int_functions: Vec<IntCallable>,
    pub bool_functions: Vec<BoolCallable>,
    pub float_functions: Vec<FloatCallable>,
    pub string_functions: Vec<StringCallable>,
    pub bit_array_functions: Vec<BitArrayCallable>,
    pub utf_codepoint_functions: Vec<UtfCodepointCallable>,
    pub nil_functions: Vec<NilCallable>,
}

/// Borrowed only while selecting or constructing an entry. A rejected entry
/// leaves every original canonical column untouched.
#[derive(Clone, Copy)]
pub struct CallInputs<'values>(pub(in crate::runtime) &'values BlockEnvironment);

impl<'values> CallInputs<'values> {
    pub(in crate::runtime) fn new(environment: &'values BlockEnvironment) -> Self {
        Self(environment)
    }
}

#[derive(Clone)]
pub struct CallInteger(pub(in crate::runtime) IntegerValue);

impl CallInteger {
    pub fn small(&self) -> Option<i128> {
        self.0.small().map(i128::from)
    }
}

impl From<i128> for CallInteger {
    fn from(value: i128) -> Self {
        Self(value.into())
    }
}

#[derive(Clone, PartialEq, Eq)]
pub struct CallBitArray(pub(in crate::runtime) EvaluatedBitArray);

impl From<crate::BitArrayValue> for CallBitArray {
    fn from(value: crate::BitArrayValue) -> Self {
        Self(EvaluatedBitArray::from_value(value))
    }
}

#[derive(Clone)]
pub struct CallCaptures(pub(in crate::runtime) Captures);

pub struct CallCaptureInputs<'captures>(pub(in crate::runtime) &'captures Captures);
pub struct CallCapture(EvaluatedCapture);

/// These borrows end before a callback, checkpoint, re-entry, or yield.
pub struct CallOps<'execution> {
    captures: &'execution CaptureStorage,
    numeric: &'execution mut NumericValues,
    strings: &'execution mut Option<Box<StringValues>>,
    bit_arrays: &'execution mut Option<Box<BitArrayValues>>,
    lists: IntListOps<'execution>,
    primitive_lists: PrimitiveListOps<'execution>,
    root_tail_entry: bool,
    synchronous_strings: &'execution [bool],
    synchronous_customs: &'execution [bool],
    synchronous_tuples: &'execution [bool],
}

pub trait CallExecution: Send {
    /// A cached execution owns no active locals or return entries. Restart
    /// selects the same typed entry as a fresh start, without borrowing inputs.
    fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool;
    fn retained_bytes(&self) -> usize;
    fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress;

    fn advance_native(
        self: Box<Self>,
        ops: &mut CallOps<'_>,
        budget: &mut usize,
        _native: &mut CallNativeOps<'_>,
    ) -> Result<Option<CallProgress>, CallNativeFailure> {
        Ok(Some(self.advance(ops, budget)))
    }
}

/// One completed generated workspace, local to an execution's graph storage.
/// Waiting and yielding executions remain owned by their active continuation.
#[derive(Default)]
pub struct CallStorage {
    idle: Option<Box<dyn CallExecution>>,
}

impl CallStorage {
    pub fn reuse(
        &mut self,
        target: CallTarget,
        point: usize,
        inputs: CallInputs<'_>,
    ) -> Option<Box<dyn CallExecution>> {
        let mut execution = self.idle.take()?;
        if execution.restart(target, point, inputs) {
            Some(execution)
        } else {
            self.idle = Some(execution);
            None
        }
    }

    pub(in crate::runtime) fn recycle(&mut self, execution: Box<dyn CallExecution>) {
        const MAX_RETAINED_BYTES: usize = 64 * 1024;
        if execution.retained_bytes() <= MAX_RETAINED_BYTES {
            self.idle = Some(execution);
        }
    }
}

pub type CallStart = fn(usize, CallInputs<'_>, &mut CallStorage) -> Option<Box<dyn CallExecution>>;
pub type CallResume<Value> = Box<dyn FnOnce(Value) -> Box<dyn CallExecution> + Send>;

/// A normal return carries only its actual typed result.
pub enum CallOutput {
    Custom(CallCustom),
    Tuple(CallTuple),
    Int(CallInteger),
    Bool(bool),
    IntFunction(IntCallable),
    BoolFunction(BoolCallable),
    Float(f64),
    String(StringValue),
    BitArray(CallBitArray),
    UtfCodepoint(char),
    Nil(()),
    FloatFunction(FloatCallable),
    StringFunction(StringCallable),
    BitArrayFunction(BitArrayCallable),
    UtfCodepointFunction(UtfCodepointCallable),
    NilFunction(NilCallable),
}

impl CallOutput {
    pub(in crate::runtime) fn family(&self) -> FunctionReturnFamily {
        match self {
            Self::Custom(_) => FunctionReturnFamily::Custom,
            Self::Tuple(_) => FunctionReturnFamily::Tuple,
            Self::Int(_) => FunctionReturnFamily::Int,
            Self::Float(_) => FunctionReturnFamily::Float,
            Self::String(_) => FunctionReturnFamily::String,
            Self::BitArray(_) => FunctionReturnFamily::BitArray,
            Self::UtfCodepoint(_) => FunctionReturnFamily::UtfCodepoint,
            Self::Bool(_) => FunctionReturnFamily::Bool,
            Self::Nil(()) => FunctionReturnFamily::Nil,
            Self::IntFunction(_)
            | Self::FloatFunction(_)
            | Self::StringFunction(_)
            | Self::BitArrayFunction(_)
            | Self::UtfCodepointFunction(_)
            | Self::BoolFunction(_)
            | Self::NilFunction(_) => FunctionReturnFamily::Function,
        }
    }
}

pub struct CallArguments {
    pub values: Box<CallValues>,
    pub captures: Option<CallCaptures>,
}

/// Single scalar input, before crossing a canonical argument boundary.
pub enum CallNativeInput {
    Int(CallInteger),
    Bool(bool),
}

impl CallNativeInput {
    pub fn arguments(self) -> CallArguments {
        let values = match self {
            Self::Int(value) => CallValues {
                ints: vec![value],
                ..CallValues::default()
            },
            Self::Bool(value) => CallValues {
                bools: vec![value],
                ..CallValues::default()
            },
        };
        CallArguments {
            values: Box::new(values),
            captures: None,
        }
    }
}

/// The full live columns are boxed only at canonical handoffs. Their width
/// does not enlarge the ordinary generated step or return representation.
pub enum CallProgress {
    StringNative(StringNativeRequest),
    CustomNative(CustomNativeRequest),
    TupleNative(TupleNativeRequest),
    Custom {
        function: CustomFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<CallCustom>,
    },
    Tuple {
        function: TupleFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<CallTuple>,
    },
    InterpretedCustom {
        function: CustomFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<CallCustom>,
    },
    InterpretedTuple {
        function: TupleFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<CallTuple>,
    },
    Yield(Box<dyn CallExecution>),
    Complete {
        output: CallOutput,
        execution: Box<dyn CallExecution>,
    },
    Interpreted {
        target: CallTarget,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
    },
    Int {
        function: IntFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<CallInteger>,
    },
    /// An owned scalar enters the original dispatcher without a full value pack.
    IntScalar {
        function: IntFunctionId,
        site: HostCallSite,
        input: CallNativeInput,
        resume: CallResume<CallInteger>,
    },
    Bool {
        function: BoolFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<bool>,
    },
    BoolScalar {
        function: BoolFunctionId,
        site: HostCallSite,
        input: CallNativeInput,
        resume: CallResume<bool>,
    },
    IntFunction {
        function: IntFunctionFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<IntCallable>,
    },
    BoolFunction {
        function: BoolFunctionFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<BoolCallable>,
    },
    Float {
        function: FloatFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<f64>,
    },
    String {
        function: StringFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<StringValue>,
    },
    BitArray {
        function: BitArrayFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<CallBitArray>,
    },
    UtfCodepoint {
        function: UtfCodepointFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<char>,
    },
    Nil {
        function: NilFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<()>,
    },
    FloatFunction {
        function: FloatFunctionFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<FloatCallable>,
    },
    StringFunction {
        function: StringFunctionFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<StringCallable>,
    },
    BitArrayFunction {
        function: BitArrayFunctionFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<BitArrayCallable>,
    },
    UtfCodepointFunction {
        function: UtfCodepointFunctionFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<UtfCodepointCallable>,
    },
    NilFunction {
        function: NilFunctionFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<NilCallable>,
    },
    InterpretedInt {
        function: IntFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<CallInteger>,
    },
    InterpretedBool {
        function: BoolFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<bool>,
    },
    InterpretedIntFunction {
        function: IntFunctionFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<IntCallable>,
    },
    InterpretedBoolFunction {
        function: BoolFunctionFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<BoolCallable>,
    },
    InterpretedFloat {
        function: FloatFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<f64>,
    },
    InterpretedString {
        function: StringFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<StringValue>,
    },
    InterpretedBitArray {
        function: BitArrayFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<CallBitArray>,
    },
    InterpretedUtfCodepoint {
        function: UtfCodepointFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<char>,
    },
    InterpretedNil {
        function: NilFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<()>,
    },
    InterpretedFloatFunction {
        function: FloatFunctionFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<FloatCallable>,
    },
    InterpretedStringFunction {
        function: StringFunctionFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<StringCallable>,
    },
    InterpretedBitArrayFunction {
        function: BitArrayFunctionFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<BitArrayCallable>,
    },
    InterpretedUtfCodepointFunction {
        function: UtfCodepointFunctionFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<UtfCodepointCallable>,
    },
    InterpretedNilFunction {
        function: NilFunctionFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: Box<CallValues>,
        resume: CallResume<NilCallable>,
    },
}

impl<'execution> CallOps<'execution> {
    pub(in crate::runtime) fn new(
        captures: &'execution CaptureStorage,
        numeric: &'execution mut NumericValues,
        lists: &'execution RuntimeListStorage,
        strings: &'execution mut Option<Box<StringValues>>,
        bit_arrays: &'execution mut Option<Box<BitArrayValues>>,
    ) -> Self {
        Self {
            captures,
            numeric,
            strings,
            bit_arrays,
            lists: IntListOps::new(lists),
            primitive_lists: PrimitiveListOps::new(lists),
            root_tail_entry: false,
            synchronous_strings: &[],
            synchronous_customs: &[],
            synchronous_tuples: &[],
        }
    }

    pub(in crate::runtime) fn with_root_tail_entry(mut self, charge: bool) -> Self {
        self.root_tail_entry = charge;
        self
    }

    pub(in crate::runtime) fn with_synchronous_strings(
        mut self,
        enabled: &'execution [bool],
    ) -> Self {
        self.synchronous_strings = enabled;
        self
    }

    pub(in crate::runtime) fn with_synchronous_compounds(
        mut self,
        customs: &'execution [bool],
        tuples: &'execution [bool],
    ) -> Self {
        self.synchronous_customs = customs;
        self.synchronous_tuples = tuples;
        self
    }
    pub fn supports_custom_native(&self, function: CustomFunctionId) -> bool {
        self.synchronous_customs
            .get(function.index)
            .copied()
            .unwrap_or(false)
    }
    pub fn supports_tuple_native(&self, function: TupleFunctionId) -> bool {
        self.synchronous_tuples
            .get(function.0)
            .copied()
            .unwrap_or(false)
    }

    pub fn supports_string_native(&self, function: StringFunctionId) -> bool {
        self.synchronous_strings
            .get(function.0)
            .copied()
            .unwrap_or(false)
    }

    /// A root source tail visits FunctionExecution::Entry. An ordinary callee
    /// tail stays within its activation and must not acquire that extra charge.
    pub fn root_tail_entry(&self) -> bool {
        self.root_tail_entry
    }

    /// Existing execution-owned numeric storage. Generated calls pass their
    /// entry tuple directly; this storage records only a yielded prefix or
    /// result of the shared structured numeric body.
    pub fn numeric(&mut self) -> &mut NumericValues {
        self.numeric
    }

    pub fn strings(
        &mut self,
        ints: &[i128],
        bools: &[bool],
        inputs: impl IntoIterator<Item = StringValue>,
    ) -> Box<StringValues> {
        let mut values = self.strings.take().unwrap_or_default();
        values.load_owned(ints, bools, inputs);
        values
    }

    pub fn bit_arrays(
        &mut self,
        ints: &[i128],
        bools: &[bool],
        inputs: impl IntoIterator<Item = CallBitArray>,
    ) -> Box<BitArrayValues> {
        let mut values = self.bit_arrays.take().unwrap_or_default();
        values.load_owned(ints, bools, inputs.into_iter().map(|input| input.0.value()));
        values
    }

    /// Return inactive scratch after completion or a real canonical handoff.
    /// Yielded kernels retain their own box and do not publish it to this cache.
    pub fn recycle_strings(&mut self, mut values: Box<StringValues>) {
        values.release_inputs();
        *self.strings = Some(values);
    }

    pub fn recycle_bit_arrays(&mut self, mut values: Box<BitArrayValues>) {
        values.release_inputs();
        *self.bit_arrays = Some(values);
    }

    pub fn lists(&self) -> &IntListOps<'_> {
        &self.lists
    }

    pub fn belongs_to_execution(&self, captures: &CallCaptureInputs<'_>) -> bool {
        captures.0.domain().is_none() || captures.0.domain() == Some(self.captures.domain())
    }

    pub fn primitive_lists(&self) -> &PrimitiveListOps<'_> {
        &self.primitive_lists
    }

    fn capture(&self, captures: Vec<CallCapture>) -> Captures {
        self.captures
            .capture(captures.into_iter().map(|capture| capture.0).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CallCapture, CallCaptureInputs, CallExecution, CallInputs, CallInteger, CallNativeInput,
        CallOps, CallProgress, CallStorage,
    };
    use crate::plan::execution::compiled::CallTarget;
    use crate::plan::execution::function::{BoolFunctionId, IntFunctionId};
    use crate::plan::execution::graph::{
        BoolFunctionLocalId, BoolLocalId, IntFunctionLocalId, IntListLocalId, IntLocalId,
    };
    use crate::plan::execution::type_::{FunctionType, ValueType};
    use crate::runtime::CaptureStorage;
    use crate::runtime::captures::Captures;
    use crate::runtime::compiled::numeric::NumericValues;
    use crate::runtime::evaluated::{EvaluatedCapture, EvaluatedCaptureKind};
    use crate::runtime::graph::{BlockEnvironment, RetainedValues};
    use crate::runtime::integer::IntegerValue;
    use crate::runtime::plan_src;
    use crate::runtime::state::list::RuntimeListStorage;
    use crate::{BitArrayValue, StringValue};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    #[test]
    fn completed_outputs_preserve_each_exact_return_family() {
        use super::{CallBitArray, CallOutput};
        use crate::plan::execution::function::{
            BitArrayFunctionId, FloatFunctionId, FunctionReturnFamily, NilFunctionId,
            StringFunctionId, UtfCodepointFunctionId,
        };
        let captures = CaptureStorage::default();
        let lists = RuntimeListStorage::default();
        let mut numeric = NumericValues::default();
        let mut strings = None;
        let mut bits = None;
        let ops = CallOps::new(&captures, &mut numeric, &lists, &mut strings, &mut bits);
        for (output, expected) in [
            (CallOutput::Int(42_i128.into()), FunctionReturnFamily::Int),
            (CallOutput::Bool(true), FunctionReturnFamily::Bool),
            (CallOutput::Float(4.5), FunctionReturnFamily::Float),
            (
                CallOutput::String("typed".into()),
                FunctionReturnFamily::String,
            ),
            (
                CallOutput::BitArray(CallBitArray::from(BitArrayValue::from_bytes(vec![7]))),
                FunctionReturnFamily::BitArray,
            ),
            (
                CallOutput::UtfCodepoint('λ'),
                FunctionReturnFamily::UtfCodepoint,
            ),
            (CallOutput::Nil(()), FunctionReturnFamily::Nil),
            (
                CallOutput::IntFunction(ops.int_reference(
                    IntFunctionId(3),
                    FunctionType::new(Vec::new(), ValueType::Int),
                )),
                FunctionReturnFamily::Function,
            ),
            (
                CallOutput::BoolFunction(ops.bool_reference(
                    BoolFunctionId(3),
                    FunctionType::new(Vec::new(), ValueType::Bool),
                )),
                FunctionReturnFamily::Function,
            ),
            (
                CallOutput::FloatFunction(ops.float_reference(
                    FloatFunctionId(3),
                    FunctionType::new(Vec::new(), ValueType::Float),
                )),
                FunctionReturnFamily::Function,
            ),
            (
                CallOutput::StringFunction(ops.string_reference(
                    StringFunctionId(3),
                    FunctionType::new(Vec::new(), ValueType::String),
                )),
                FunctionReturnFamily::Function,
            ),
            (
                CallOutput::BitArrayFunction(ops.bit_array_reference(
                    BitArrayFunctionId(3),
                    FunctionType::new(Vec::new(), ValueType::BitArray),
                )),
                FunctionReturnFamily::Function,
            ),
            (
                CallOutput::UtfCodepointFunction(ops.utf_codepoint_reference(
                    UtfCodepointFunctionId(3),
                    FunctionType::new(Vec::new(), ValueType::UtfCodepoint),
                )),
                FunctionReturnFamily::Function,
            ),
            (
                CallOutput::NilFunction(ops.nil_reference(
                    NilFunctionId(3),
                    FunctionType::new(Vec::new(), ValueType::Nil),
                )),
                FunctionReturnFamily::Function,
            ),
        ] {
            assert_eq!(output.family(), expected);
        }
    }

    struct IdleWorkspace {
        capacity: Vec<[u8; 1024]>,
        dropped: Arc<AtomicUsize>,
        input: Option<i128>,
    }

    #[test]
    fn range_kernels_keep_owned_scratch_across_yields_and_recycle_the_same_box() {
        let captures = CaptureStorage::default();
        let lists = RuntimeListStorage::default();
        let mut numeric = NumericValues::default();
        let mut strings = None;
        let mut bits = None;
        let raw = StringValue::from_bytes([b"tag:".as_slice(), &[0xff; 64]].concat());
        let backing = raw.as_ptr();
        let input = BitArrayValue::try_from_parts(vec![0xE5, 0x58], 13).unwrap();
        let (mut string_kernel, mut bit_kernel) = {
            let mut ops = CallOps::new(&captures, &mut numeric, &lists, &mut strings, &mut bits);
            (
                ops.strings(&[7], &[true], [raw]),
                ops.bit_arrays(&[9], &[false], [input.clone().into()]),
            )
        };
        let string_owner = std::ptr::from_ref(&*string_kernel);
        let bit_owner = std::ptr::from_ref(&*bit_kernel);
        string_kernel.strings[0] = string_kernel.strings[0].drop_prefix(4);
        bit_kernel.bit_arrays[0] = bit_kernel.bit_arrays[0].slice(2, 11).unwrap();
        assert!(strings.is_none());
        assert!(bits.is_none());

        // Another activation may use the ordinary cache while these kernels yield.
        {
            let mut ops = CallOps::new(&captures, &mut numeric, &lists, &mut strings, &mut bits);
            let other_strings = ops.strings(&[], &[], [StringValue::from("other")]);
            let other_bits = ops.bit_arrays(&[], &[], [BitArrayValue::from_bytes(vec![0]).into()]);
            ops.recycle_strings(other_strings);
            ops.recycle_bit_arrays(other_bits);
        }
        assert_eq!(string_kernel.ints, [7]);
        assert_eq!(string_kernel.bools, [true]);
        assert_eq!(bit_kernel.ints, [9]);
        assert_eq!(bit_kernel.bools, [false]);
        let result = string_kernel.finish(string_kernel.strings[0]);
        assert_eq!(result.as_ptr(), backing.wrapping_add(4));
        assert_eq!(result.as_bytes(), &[0xff; 64]);
        let result = bit_kernel.finish(bit_kernel.bit_arrays[0]);
        assert_eq!(result, input.bit_slice(2, 11).unwrap());

        let mut ops = CallOps::new(&captures, &mut numeric, &lists, &mut strings, &mut bits);
        ops.recycle_strings(string_kernel);
        ops.recycle_bit_arrays(bit_kernel);
        let reused_strings = ops.strings(&[], &[], []);
        let reused_bits = ops.bit_arrays(&[], &[], []);
        assert_eq!(std::ptr::from_ref(&*reused_strings), string_owner);
        assert_eq!(std::ptr::from_ref(&*reused_bits), bit_owner);
        assert!(reused_strings.strings.is_empty());
        assert!(reused_bits.bit_arrays.is_empty());
    }

    impl CallExecution for IdleWorkspace {
        fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
            if target != CallTarget::Int(IntFunctionId(2)) || point != 3 {
                return false;
            }
            self.input = inputs.int(0);
            self.input.is_some()
        }

        fn retained_bytes(&self) -> usize {
            std::mem::size_of::<Self>() + self.capacity.capacity() * 1024
        }

        fn advance(self: Box<Self>, _ops: &mut CallOps<'_>, _budget: &mut usize) -> CallProgress {
            CallProgress::Yield(self)
        }
    }

    impl Drop for IdleWorkspace {
        fn drop(&mut self) {
            self.dropped.fetch_add(1, Ordering::SeqCst);
        }
    }

    #[test]
    fn scalar_native_handoff_has_one_original_argument_and_no_captures() {
        for value in [7, i128::from(i64::MAX) + 1] {
            let arguments = CallNativeInput::Int(value.into()).arguments();
            assert_eq!(arguments.values.ints.len(), 1);
            assert_eq!(arguments.values.ints[0].0, IntegerValue::from(value));
            assert!(arguments.values.bools.is_empty());
            assert!(arguments.values.int_lists.is_empty());
            assert!(arguments.values.int_functions.is_empty());
            assert!(arguments.values.bool_functions.is_empty());
            assert!(arguments.captures.is_none());
        }
        for value in [false, true] {
            let arguments = CallNativeInput::Bool(value).arguments();
            assert_eq!(arguments.values.bools, [value]);
            assert!(arguments.values.ints.is_empty());
            assert!(arguments.values.int_lists.is_empty());
            assert!(arguments.values.int_functions.is_empty());
            assert!(arguments.values.bool_functions.is_empty());
            assert!(arguments.captures.is_none());
        }
    }

    #[test]
    fn idle_workspace_reuses_owned_capacity_keeps_rejected_inputs_and_bounds_retention() {
        let dropped = Arc::new(AtomicUsize::new(0));
        let mut cache = CallStorage::default();
        let ints = [IntegerValue::from(7)];
        let mut retained = RetainedValues::empty();
        retained.push_int(ints[0].clone());
        let environment = BlockEnvironment::from_retained(retained);
        let inputs = CallInputs::new(&environment);
        let target = CallTarget::Int(IntFunctionId(2));
        assert!(cache.reuse(target, 3, inputs).is_none());
        let idle = Box::new(IdleWorkspace {
            capacity: Vec::with_capacity(4),
            dropped: dropped.clone(),
            input: None,
        });
        let owner = std::ptr::from_ref(&*idle).cast::<()>();
        cache.recycle(idle);
        assert!(
            cache
                .reuse(CallTarget::Bool(BoolFunctionId(2)), 3, inputs)
                .is_none()
        );
        assert!(cache.reuse(target, 2, inputs).is_none());
        let big = [IntegerValue::from(1_i128 << 100)];
        let mut retained = RetainedValues::empty();
        retained.push_int(big[0].clone());
        let big_environment = BlockEnvironment::from_retained(retained);
        assert!(
            cache
                .reuse(target, 3, CallInputs::new(&big_environment))
                .is_none()
        );
        assert_eq!(dropped.load(Ordering::SeqCst), 0);
        assert_eq!(ints, [IntegerValue::from(7)]);
        assert_eq!(big, [IntegerValue::from(1_i128 << 100)]);
        let execution = cache.reuse(target, 3, inputs).unwrap();
        assert_eq!(std::ptr::from_ref(&*execution).cast::<()>(), owner);
        assert!(cache.idle.is_none());
        assert_eq!(
            execution.retained_bytes(),
            std::mem::size_of::<IdleWorkspace>() + 4096
        );
        let captures = CaptureStorage::default();
        let mut numeric = NumericValues::default();
        let mut string_scratch = None;
        let mut bit_scratch = None;
        let lists = RuntimeListStorage::default();
        let mut budget = 0;
        let progress = execution.advance(
            &mut CallOps::new(
                &captures,
                &mut numeric,
                &lists,
                &mut string_scratch,
                &mut bit_scratch,
            ),
            &mut budget,
        );
        drop(progress);
        assert_eq!(dropped.load(Ordering::SeqCst), 1);
        cache.recycle(Box::new(IdleWorkspace {
            capacity: Vec::with_capacity(4),
            dropped: dropped.clone(),
            input: None,
        }));
        cache.recycle(Box::new(IdleWorkspace {
            capacity: Vec::with_capacity(65),
            dropped: dropped.clone(),
            input: None,
        }));
        assert_eq!(dropped.load(Ordering::SeqCst), 2);
        assert_eq!(
            cache.idle.as_ref().unwrap().retained_bytes(),
            std::mem::size_of::<IdleWorkspace>() + 4096
        );
        cache.recycle(Box::new(IdleWorkspace {
            capacity: Vec::with_capacity(1),
            dropped: dropped.clone(),
            input: None,
        }));
        assert_eq!(dropped.load(Ordering::SeqCst), 3);
        drop(cache);
        assert_eq!(dropped.load(Ordering::SeqCst), 4);
    }

    #[test]
    fn call_integers_normalize_small_boundaries_and_preserve_promoted_results() {
        for value in [i128::from(i64::MIN), 0, i128::from(i64::MAX)] {
            let value = CallInteger::from(value);
            assert_eq!(value.small(), value.0.small().map(i128::from));
            assert!(value.small().is_some());
        }
        for value in [
            i128::from(i64::MIN) - 1,
            i128::from(i64::MAX) + 1,
            1_i128 << 100,
        ] {
            let result = CallInteger::from(value);
            assert_eq!(result.small(), None);
            assert_eq!(result.0, IntegerValue::from(value));
        }
    }

    #[test]
    fn canonical_callable_ops_keep_types_targets_identity_captures_and_execution_ownership() {
        let storage = CaptureStorage::default();
        let other_storage = storage.for_execution();
        let mut numeric = NumericValues::default();
        let mut string_scratch = None;
        let mut bit_scratch = None;
        let lists = RuntimeListStorage::default();
        let ops = CallOps::new(
            &storage,
            &mut numeric,
            &lists,
            &mut string_scratch,
            &mut bit_scratch,
        );
        let int_type = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let bool_type = FunctionType::new(vec![ValueType::Bool], ValueType::Bool);
        let int = ops.int_reference(IntFunctionId(2), int_type.clone());
        let boolean = ops.bool_reference(BoolFunctionId(3), bool_type.clone());
        assert_eq!(int.target(), IntFunctionId(2));
        assert_eq!(boolean.target(), BoolFunctionId(3));
        assert_eq!(
            int.0,
            ops.int_reference(IntFunctionId(2), int_type.clone()).0
        );
        assert_eq!(
            boolean.0,
            ops.bool_reference(BoolFunctionId(3), bool_type.clone()).0
        );
        assert!(ops.belongs_to_execution(&int.captures()));
        assert!(ops.belongs_to_execution(&boolean.captures()));
        assert!(ops.belongs_to_execution(&CallCaptureInputs(&Captures::default())));
        assert!(!ops.belongs_to_execution(&CallCaptureInputs(&other_storage.capture(Vec::new()))));
        let closure = ops.int_closure(
            IntFunctionId(4),
            int_type.clone(),
            vec![
                CallCapture::int(IntLocalId(1), 7),
                CallCapture::bool(BoolLocalId(0), true),
                CallCapture::int_function(IntFunctionLocalId(0), int.clone()),
                CallCapture::bool_function(BoolFunctionLocalId(0), boolean.clone()),
                CallCapture::int_list(
                    IntListLocalId(2),
                    ops.lists().value(
                        plan_src("pub fn main() { [1] }")
                            .int_list_function_id(0)
                            .type_id(),
                        &[3, 4],
                    ),
                ),
            ],
        );
        let captures = closure.captures();
        assert!(std::ptr::eq(captures.0, closure.0.capture_frame()));
        assert_eq!(captures.int(IntLocalId(1)), Some(7));
        assert_eq!(captures.bool(BoolLocalId(0)), Some(true));
        let list = captures.int_list(IntListLocalId(2)).unwrap();
        assert_eq!(ops.lists().index(&list, 0), Some(3));
        assert_eq!(list.len(), 2);
        assert!(captures.int_list(IntListLocalId(1)).is_none());
        assert_eq!(
            captures.int_function(IntFunctionLocalId(0)).unwrap().0,
            int.0
        );
        assert_eq!(
            captures.bool_function(BoolFunctionLocalId(0)).unwrap().0,
            boolean.0
        );
        assert_eq!(captures.int(IntLocalId(0)), None);
        assert_eq!(captures.bool(BoolLocalId(1)), None);
        assert!(captures.int_function(IntFunctionLocalId(1)).is_none());
        assert!(captures.bool_function(BoolFunctionLocalId(1)).is_none());
        assert_eq!(closure.0, closure.clone().0);
        assert_ne!(
            closure.0,
            ops.int_closure(IntFunctionId(4), int_type.clone(), Vec::new())
                .0
        );
        assert_eq!(closure.0.type_(), &int_type);
        let changed_type = FunctionType::new(Vec::new(), ValueType::Int);
        let retagged = closure.clone().with_type(changed_type.clone());
        assert_eq!(retagged.target(), closure.target());
        assert_eq!(retagged.0.type_(), &changed_type);
        assert_eq!(retagged.0.capture_frame(), closure.0.capture_frame());
        let bool_closure = ops.bool_closure(
            BoolFunctionId(5),
            bool_type.clone(),
            vec![CallCapture::bool(BoolLocalId(0), false)],
        );
        assert_eq!(bool_closure.captures().bool(BoolLocalId(0)), Some(false));
        assert_ne!(
            bool_closure.0,
            ops.bool_closure(BoolFunctionId(5), bool_type.clone(), Vec::new())
                .0
        );
        let retagged = bool_closure
            .clone()
            .with_type(FunctionType::new(Vec::new(), ValueType::Bool));
        assert_eq!(retagged.target(), bool_closure.target());
        assert_eq!(retagged.0.capture_frame(), bool_closure.0.capture_frame());
        let big = storage.capture(vec![EvaluatedCapture::from_kind(
            EvaluatedCaptureKind::Int {
                local: IntLocalId(0),
                value: (1_i128 << 100).into(),
            },
        )]);
        assert_eq!(CallCaptureInputs(&big).int(IntLocalId(0)), None);
        let mut ops = CallOps::new(
            &storage,
            &mut numeric,
            &lists,
            &mut string_scratch,
            &mut bit_scratch,
        );
        ops.numeric().ints.push(42);
        assert_eq!(numeric.ints, vec![42]);
        let retained = captures.retain();
        std::thread::spawn(move || {
            assert_eq!(&retained.0, closure.0.capture_frame());
        })
        .join()
        .unwrap();
    }
}
