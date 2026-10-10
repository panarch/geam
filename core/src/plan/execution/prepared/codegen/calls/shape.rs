use super::super::int_list::{IntListInstruction, IntListTest};
use super::super::shape::{
    CompiledBoolean, CompiledInstruction, CompiledShape, CompiledTerminator, CompiledTest,
    KernelKind, NumericComparison, NumericInteger,
};
use super::super::string::{StringOperation, StringTest};
use super::nullary::{CallTypes, CustomLocalShape};
use crate::plan::HostCallSite;
use crate::plan::execution::compiled::{
    CallContract, CallContractTarget, CallTarget, CompiledCheckpoint, CreationContract,
    NativeLoopContract, ReturnContract, TailContract,
};
use crate::plan::execution::function::{
    BitArrayFunctionFunctionId, BitArrayFunctionId, BoolFunctionFunctionId, BoolFunctionId,
    CustomFunctionId, ExecutionFunctionEntry, ExecutionFunctionRef, ExecutionGraphProfile,
    ExecutionProfile, FloatFunctionFunctionId, FloatFunctionId, FunctionBodyOwner, FunctionExit,
    FunctionTables, IntFunctionFunctionId, IntFunctionId, NilFunctionFunctionId, NilFunctionId,
    ProfiledFunctionBody, ProfiledFunctionFunctionId, StringFunctionFunctionId, StringFunctionId,
    TupleFunctionId, UtfCodepointFunctionFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::graph::{
    ArithmeticRegion, BitArrayFunctionLocalId, BitArrayInstruction, BitArrayListLocalId,
    BitArrayLocalId, BlockId, BoolFunctionLocalId, BoolInstruction, BoolListLocalId, BoolLocalId,
    BoolTest, CustomInstruction, Edge, FloatFunctionLocalId, FloatInstruction, FloatListLocalId,
    FloatLocalId, FunctionCapture, FunctionInstructionKind, FunctionTarget, IntFunctionLocalId,
    IntInstruction, IntListLocalId, IntLocalId, IntegerLiteral, IntegerOperand, ListInstruction,
    ListLocal, NilFunctionLocalId, NilInstruction, NilListLocalId, NilLocalId, ParamLocal,
    ProfiledInstruction, ProfiledInstructionKind, StringFunctionLocalId, StringInstruction,
    StringListLocalId, StringLocalId, Terminator, TupleInstruction, TupleLocalId,
    TypedListInstruction, UtfCodepointFunctionLocalId, UtfCodepointInstruction,
    UtfCodepointListLocalId, UtfCodepointLocalId,
};
use crate::plan::execution::storage::Table;
use crate::plan::execution::type_::{
    BitArrayListTypeId, BoolListTypeId, FloatListTypeId, FunctionType, IntListTypeId,
    NilListTypeId, StringListTypeId, UtfCodepointListTypeId, ValueType,
};
use std::collections::BTreeMap;

pub(in crate::plan::execution::prepared) struct CallProgram<'graph, Graph: ExecutionGraphProfile> {
    pub(super) functions: Vec<CallFunction<'graph, Graph>>,
}

pub(super) struct CallFunction<'graph, Graph: ExecutionGraphProfile> {
    pub(super) target: CallTarget,
    pub(super) shape: CallShape<'graph>,
    pub(super) kernel: Option<CompiledShape<'graph, Graph>>,
    pub(super) kernel_returns: Vec<KernelReturn>,
    pub(super) native_loop: bool,
}

pub(super) enum KernelReturn {
    Int(IntLocalId),
    Bool(BoolLocalId),
    String(StringLocalId),
    BitArray(BitArrayLocalId),
}

impl<'graph, Graph: ExecutionGraphProfile> CallProgram<'graph, Graph> {
    pub(in crate::plan::execution::prepared) fn inspect<
        Profile: ExecutionProfile<Graph = Graph>,
    >(
        functions: &'graph FunctionTables<Profile>,
        custom_types: &crate::plan::execution::type_::CustomTypeTable,
        value_shapes: &crate::plan::execution::type_::ValueShapeTable,
    ) -> Self {
        let types = CallTypes {
            custom_types,
            value_shapes,
        };
        let mut selected = Vec::new();
        for (index, entry) in functions.value_returns.int_functions.iter().enumerate() {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let kernel = CallShape::callee_kernel(function.body())
                && let Some(shape) = kernel
                    .as_ref()
                    .filter(|kernel| kernel.kind != KernelKind::Numeric)
                    .map(|kernel| {
                        CallShape::from_kernel(
                            function.body(),
                            function.entry().parameter_count,
                            kernel,
                            |local| CallLocal::Int(*local),
                        )
                    })
                    .or_else(|| {
                        CallShape::inspect(
                            &types,
                            function.body(),
                            function.entry().parameter_count,
                            false,
                            |value, local| matches!(local, CallLocal::Int(id) if id == value),
                            |target| CallTarget::Int(*target.function()),
                            |target| target.site().clone(),
                        )
                    })
            {
                selected.push(CallFunction {
                    target: CallTarget::Int(IntFunctionId(index)),
                    native_loop: NativeLoopContract::inspect(function.body().block_graph())
                        .is_some(),
                    kernel,
                    kernel_returns: function
                        .body()
                        .exits
                        .iter()
                        .filter_map(|exit| match exit {
                            FunctionExit::Return(local) => Some(KernelReturn::Int(*local)),
                            FunctionExit::TailCall { .. } => None,
                        })
                        .collect(),
                    shape,
                });
            }
        }
        for (index, entry) in functions.value_returns.bool_functions.iter().enumerate() {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let kernel = CallShape::callee_kernel(function.body())
                && let Some(shape) = kernel
                    .as_ref()
                    .filter(|kernel| kernel.kind != KernelKind::Numeric)
                    .map(|kernel| {
                        CallShape::from_kernel(
                            function.body(),
                            function.entry().parameter_count,
                            kernel,
                            |local| CallLocal::Bool(*local),
                        )
                    })
                    .or_else(|| {
                        CallShape::inspect(
                            &types,
                            function.body(),
                            function.entry().parameter_count,
                            false,
                            |value, local| matches!(local, CallLocal::Bool(id) if id == value),
                            |target| CallTarget::Bool(*target.function()),
                            |target| target.site().clone(),
                        )
                    })
            {
                selected.push(CallFunction {
                    target: CallTarget::Bool(BoolFunctionId(index)),
                    native_loop: NativeLoopContract::inspect(function.body().block_graph())
                        .is_some(),
                    kernel,
                    kernel_returns: function
                        .body()
                        .exits
                        .iter()
                        .filter_map(|exit| match exit {
                            FunctionExit::Return(local) => Some(KernelReturn::Bool(*local)),
                            FunctionExit::TailCall { .. } => None,
                        })
                        .collect(),
                    shape,
                });
            }
        }
        for (index, entry) in functions
            .function_returns
            .int_function_functions
            .iter()
            .enumerate()
        {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    &types,
                    FunctionBodyOwner::function_body(function.body()),
                    function.entry().parameter_count,
                    true,
                    |value, local| matches!(local, CallLocal::IntFunction { local: id, .. } if id == value),
                    |target| CallTarget::IntFunction(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::IntFunction(IntFunctionFunctionId(index)),
                    native_loop: false,
                    shape,
                    kernel: None,
                    kernel_returns: Vec::new(),
                });
            }
        }
        for (index, entry) in functions
            .function_returns
            .bool_function_functions
            .iter()
            .enumerate()
        {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    &types,
                    FunctionBodyOwner::function_body(function.body()),
                    function.entry().parameter_count,
                    true,
                    |value, local| matches!(local, CallLocal::BoolFunction { local: id, .. } if id == value),
                    |target| CallTarget::BoolFunction(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::BoolFunction(BoolFunctionFunctionId(index)),
                    native_loop: false,
                    shape,
                    kernel: None,
                    kernel_returns: Vec::new(),
                });
            }
        }
        for (index, entry) in functions.value_returns.float_functions.iter().enumerate() {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    &types,
                    function.body(),
                    function.entry().parameter_count,
                    false,
                    |value, local| matches!(local, CallLocal::Float(id) if id == value),
                    |target| CallTarget::Float(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::Float(FloatFunctionId(index)),
                    native_loop: NativeLoopContract::inspect(function.body().block_graph())
                        .is_some(),
                    shape,
                    kernel: None,
                    kernel_returns: Vec::new(),
                });
            }
        }
        for (index, entry) in functions
            .function_returns
            .float_function_functions
            .iter()
            .enumerate()
        {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    &types,
                    FunctionBodyOwner::function_body(function.body()),
                    function.entry().parameter_count,
                    true,
                    |value, local| matches!(local, CallLocal::FloatFunction { local: id, .. } if id == value),
                    |target| CallTarget::FloatFunction(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::FloatFunction(FloatFunctionFunctionId(index)),
                    native_loop: false,
                    shape,
                    kernel: None,
                    kernel_returns: Vec::new(),
                });
            }
        }
        for (index, entry) in functions.value_returns.string_functions.iter().enumerate() {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let kernel = CallShape::callee_kernel(function.body())
                && let Some(shape) = kernel
                    .as_ref()
                    .filter(|kernel| kernel.kind != KernelKind::Numeric)
                    .map(|kernel| {
                        CallShape::from_kernel(
                            function.body(),
                            function.entry().parameter_count,
                            kernel,
                            |local| CallLocal::String(*local),
                        )
                    })
                    .or_else(|| {
                        CallShape::inspect(
                            &types,
                            function.body(),
                            function.entry().parameter_count,
                            false,
                            |value, local| matches!(local, CallLocal::String(id) if id == value),
                            |target| CallTarget::String(*target.function()),
                            |target| target.site().clone(),
                        )
                    })
            {
                selected.push(CallFunction {
                    target: CallTarget::String(StringFunctionId(index)),
                    native_loop: NativeLoopContract::inspect(function.body().block_graph())
                        .is_some(),
                    kernel,
                    kernel_returns: function
                        .body()
                        .exits
                        .iter()
                        .filter_map(|exit| match exit {
                            FunctionExit::Return(local) => Some(KernelReturn::String(*local)),
                            FunctionExit::TailCall { .. } => None,
                        })
                        .collect(),
                    shape,
                });
            }
        }
        for (index, entry) in functions
            .function_returns
            .string_function_functions
            .iter()
            .enumerate()
        {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    &types,
                    FunctionBodyOwner::function_body(function.body()),
                    function.entry().parameter_count,
                    true,
                    |value, local| matches!(local, CallLocal::StringFunction { local: id, .. } if id == value),
                    |target| CallTarget::StringFunction(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::StringFunction(StringFunctionFunctionId(index)),
                    native_loop: false,
                    shape,
                    kernel: None,
                    kernel_returns: Vec::new(),
                });
            }
        }
        for (index, entry) in functions
            .value_returns
            .bit_array_functions
            .iter()
            .enumerate()
        {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let kernel = CallShape::callee_kernel(function.body())
                && let Some(shape) = kernel
                    .as_ref()
                    .filter(|kernel| kernel.kind != KernelKind::Numeric)
                    .map(|kernel| {
                        CallShape::from_kernel(
                            function.body(),
                            function.entry().parameter_count,
                            kernel,
                            |local| CallLocal::BitArray(*local),
                        )
                    })
                    .or_else(|| {
                        CallShape::inspect(
                            &types,
                            function.body(),
                            function.entry().parameter_count,
                            false,
                            |value, local| matches!(local, CallLocal::BitArray(id) if id == value),
                            |target| CallTarget::BitArray(*target.function()),
                            |target| target.site().clone(),
                        )
                    })
            {
                selected.push(CallFunction {
                    target: CallTarget::BitArray(BitArrayFunctionId(index)),
                    native_loop: NativeLoopContract::inspect(function.body().block_graph())
                        .is_some(),
                    kernel,
                    kernel_returns: function
                        .body()
                        .exits
                        .iter()
                        .filter_map(|exit| match exit {
                            FunctionExit::Return(local) => Some(KernelReturn::BitArray(*local)),
                            FunctionExit::TailCall { .. } => None,
                        })
                        .collect(),
                    shape,
                });
            }
        }
        for (index, entry) in functions
            .function_returns
            .bit_array_function_functions
            .iter()
            .enumerate()
        {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    &types,
                    FunctionBodyOwner::function_body(function.body()),
                    function.entry().parameter_count,
                    true,
                    |value, local| matches!(local, CallLocal::BitArrayFunction { local: id, .. } if id == value),
                    |target| CallTarget::BitArrayFunction(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::BitArrayFunction(BitArrayFunctionFunctionId(index)),
                    native_loop: false,
                    shape,
                    kernel: None,
                    kernel_returns: Vec::new(),
                });
            }
        }
        for (index, entry) in functions
            .value_returns
            .utf_codepoint_functions
            .iter()
            .enumerate()
        {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    &types,
                    function.body(),
                    function.entry().parameter_count,
                    false,
                    |value, local| matches!(local, CallLocal::UtfCodepoint(id) if id == value),
                    |target| CallTarget::UtfCodepoint(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::UtfCodepoint(UtfCodepointFunctionId(index)),
                    native_loop: NativeLoopContract::inspect(function.body().block_graph())
                        .is_some(),
                    shape,
                    kernel: None,
                    kernel_returns: Vec::new(),
                });
            }
        }
        for (index, entry) in functions
            .function_returns
            .utf_codepoint_function_functions
            .iter()
            .enumerate()
        {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    &types,
                    FunctionBodyOwner::function_body(function.body()),
                    function.entry().parameter_count,
                    true,
                    |value, local| matches!(local, CallLocal::UtfCodepointFunction { local: id, .. } if id == value),
                    |target| CallTarget::UtfCodepointFunction(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::UtfCodepointFunction(UtfCodepointFunctionFunctionId(index)),
                    native_loop: false,
                    shape,
                    kernel: None,
                    kernel_returns: Vec::new(),
                });
            }
        }
        for (index, entry) in functions.value_returns.nil_functions.iter().enumerate() {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    &types,
                    function.body(),
                    function.entry().parameter_count,
                    false,
                    |value, local| matches!(local, CallLocal::Nil(id) if id == value),
                    |target| CallTarget::Nil(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::Nil(NilFunctionId(index)),
                    native_loop: NativeLoopContract::inspect(function.body().block_graph())
                        .is_some(),
                    shape,
                    kernel: None,
                    kernel_returns: Vec::new(),
                });
            }
        }
        for (index, entry) in functions
            .function_returns
            .nil_function_functions
            .iter()
            .enumerate()
        {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    &types,
                    FunctionBodyOwner::function_body(function.body()),
                    function.entry().parameter_count,
                    true,
                    |value, local| matches!(local, CallLocal::NilFunction { local: id, .. } if id == value),
                    |target| CallTarget::NilFunction(*target.function()),
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::NilFunction(NilFunctionFunctionId(index)),
                    native_loop: false,
                    shape,
                    kernel: None,
                    kernel_returns: Vec::new(),
                });
            }
        }
        for (index, entry) in functions.value_returns.custom_functions.iter().enumerate() {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref()
                && let Some(shape) = CallShape::inspect(
                    &types,
                    &function.body().body,
                    function.entry().parameter_count,
                    false,
                    |value, local| local.canonical() == ParamLocal::Custom(*value),
                    |target| {
                        CallTarget::Custom(CustomFunctionId::new(
                            *target.function(),
                            function.body()._signature_shape,
                        ))
                    },
                    |target| target.site().clone(),
                )
            {
                selected.push(CallFunction {
                    target: CallTarget::Custom(CustomFunctionId::new(
                        index,
                        function.body()._signature_shape,
                    )),
                    native_loop: false,
                    shape,
                    kernel: None,
                    kernel_returns: Vec::new(),
                });
            }
        }
        for (index, entry) in functions.value_returns.tuple_functions.iter().enumerate() {
            if let ExecutionFunctionRef::Graph(function) = entry.as_ref() {
                let body = function.body();
                if let Some(shape) = CallShape::inspect(
                    &types,
                    body,
                    function.entry().parameter_count,
                    false,
                    |value, local| matches!(local, CallLocal::Tuple { local: id, .. } if id == value),
                    |target| CallTarget::Tuple(*target.function()),
                    |target| target.site().clone(),
                ) {
                    selected.push(CallFunction {
                        target: CallTarget::Tuple(TupleFunctionId(index)),
                        native_loop: false,
                        shape,
                        kernel: None,
                        kernel_returns: Vec::new(),
                    });
                }
            }
        }
        // A call-free prefix that immediately leaves this engine adds a
        // boundary instead of connecting computation. Keep the established
        // numeric leaf adapter; let other incomplete leaves use their original
        // executor, including the dedicated String and BitArray kernels.
        selected.retain(|function| {
            let complete = function
                .shape
                .points
                .iter()
                .all(|point| !matches!(point, CallPoint::Interpreted));
            // Compound source construction is outside this connection. Do not
            // introduce a new partial compound-return entry solely because its
            // prefix happens to contain a callable or scalar call.
            (!matches!(
                function.target,
                CallTarget::Custom(_) | CallTarget::Tuple(_)
            ) || complete)
                && (function.shape.root || function.kernel.is_some() || complete)
        });
        selected.sort_by_key(|function| function.target.key());
        let compound_native = |target: CallTarget| match target {
            CallTarget::Custom(id) => matches!(
                functions.value_returns.custom_functions[id.index].as_ref(),
                ExecutionFunctionRef::Host(_)
            ),
            CallTarget::Tuple(id) => matches!(
                functions.value_returns.tuple_functions[id.0].as_ref(),
                ExecutionFunctionRef::Host(_)
            ),
            _ => false,
        };
        for function in &mut selected {
            function.shape.root |= function.shape.tails.iter().any(|tail| {
                if compound_native(tail.target) && tail.args.iter().all(CallLocal::native_argument)
                {
                    return true;
                }
                let CallTarget::String(target) = tail.target else {
                    return false;
                };
                matches!(
                    functions.value_returns.string_functions[target.0].as_ref(),
                    ExecutionFunctionRef::Host(_)
                ) && tail.args.iter().all(CallLocal::native_argument)
            });
        }
        let scalar_native = |call: &CallInvocation| {
            matches!(
                call.args.as_slice(),
                [CallLocal::Int(_) | CallLocal::Bool(_)]
            ) && match call.target {
                CallContractTarget::Static(CallTarget::Int(target)) => matches!(
                    functions.value_returns.int_functions[target.0].as_ref(),
                    ExecutionFunctionRef::Host(_)
                ),
                CallContractTarget::Static(CallTarget::Bool(target)) => matches!(
                    functions.value_returns.bool_functions[target.0].as_ref(),
                    ExecutionFunctionRef::Host(_)
                ),
                _ => false,
            }
        };
        // A scalar wrapper is not useful when every call immediately leaves
        // this engine. Preserve complete call-free callees, callable bodies
        // and supported Native connections. Non-root wrappers need an actual
        // connection too, even when another useful root keeps this program.
        loop {
            let keep = selected
                .iter()
                .map(|function| {
                    (!function.shape.root
                        && function.shape.calls.is_empty()
                        && function.shape.tails.is_empty())
                        || function.kernel.is_some()
                        || !function.shape.creations.is_empty()
                        || function.shape.calls.iter().any(|call| {
                            scalar_native(call) || matches!(call.target, CallContractTarget::Static(target) if compound_native(target) && call.args.iter().all(CallLocal::native_argument)) ||
                            selected.iter().any(|callee| {
                                let target_matches = match call.target {
                                    CallContractTarget::Static(target) => callee.target == target,
                                    _ => true,
                                };
                                target_matches && callee.accepts_call(call)
                            }) || matches!(call.target, CallContractTarget::Static(CallTarget::String(target))
                                if matches!(functions.value_returns.string_functions[target.0].as_ref(), ExecutionFunctionRef::Host(_))
                                && call.args.iter().all(CallLocal::native_argument))
                        })
                        || function.shape.locals.iter().flatten().any(callable_local)
                        || function.shape.tails.iter().any(|tail| {
                            (compound_native(tail.target) && tail.args.iter().all(CallLocal::native_argument)) || selected.iter().any(|callee| {
                                callee.target == tail.target
                                    && callee.matches_parameters(&tail.args)
                            }) || matches!(tail.target, CallTarget::String(target)
                                if matches!(functions.value_returns.string_functions[target.0].as_ref(), ExecutionFunctionRef::Host(_))
                                && tail.args.iter().all(CallLocal::native_argument))
                        })
                })
                .collect::<Vec<_>>();
            if keep.iter().all(|keep| *keep) {
                break;
            }
            let mut index = 0;
            selected.retain(|_| {
                let retain = keep[index];
                index += 1;
                retain
            });
        }
        let roots = selected
            .iter()
            .map(|function| {
                function.shape.root
                    && match function.shape.points[function.shape.entry()] {
                        CallPoint::Call(index) => {
                            let call = &function.shape.calls[index];
                            matches!(call.target, CallContractTarget::Static(_))
                                || selected.iter().any(|callee| callee.accepts_call(call))
                        }
                        _ => true,
                    }
            })
            .collect::<Vec<_>>();
        for (function, root) in selected.iter_mut().zip(roots) {
            function.shape.root = root;
        }
        if !selected.iter().any(|function| function.shape.root) {
            selected.clear();
        }
        Self {
            functions: selected,
        }
    }

    pub(in crate::plan::execution::prepared) fn shapes(
        &self,
    ) -> impl Iterator<Item = (CallTarget, &CallShape<'graph>)> {
        self.functions
            .iter()
            .map(|function| (function.target, &function.shape))
    }
}

impl<Graph: ExecutionGraphProfile> CallFunction<'_, Graph> {
    pub(super) fn accepts_call(&self, call: &CallInvocation) -> bool {
        matches!(
            (&call.output, self.target),
            (CallLocal::Int(_), CallTarget::Int(_))
                | (CallLocal::Bool(_), CallTarget::Bool(_))
                | (CallLocal::IntFunction { .. }, CallTarget::IntFunction(_))
                | (CallLocal::BoolFunction { .. }, CallTarget::BoolFunction(_))
                | (CallLocal::Float(_), CallTarget::Float(_))
                | (
                    CallLocal::FloatFunction { .. },
                    CallTarget::FloatFunction(_)
                )
                | (
                    CallLocal::Custom(_) | CallLocal::Nullary(_),
                    CallTarget::Custom(_)
                )
                | (CallLocal::Tuple { .. }, CallTarget::Tuple(_))
                | (CallLocal::String(_), CallTarget::String(_))
                | (
                    CallLocal::StringFunction { .. },
                    CallTarget::StringFunction(_)
                )
                | (CallLocal::BitArray(_), CallTarget::BitArray(_))
                | (
                    CallLocal::BitArrayFunction { .. },
                    CallTarget::BitArrayFunction(_)
                )
                | (CallLocal::UtfCodepoint(_), CallTarget::UtfCodepoint(_))
                | (
                    CallLocal::UtfCodepointFunction { .. },
                    CallTarget::UtfCodepointFunction(_)
                )
                | (CallLocal::Nil(_), CallTarget::Nil(_))
                | (CallLocal::NilFunction { .. }, CallTarget::NilFunction(_))
        ) && self.matches_parameters(&call.args)
    }

    pub(super) fn matches_parameters(&self, args: &[CallLocal]) -> bool {
        let entry = &self.shape.locals[self.shape.entry()];
        self.shape.parameter_count == args.len()
            && entry
                .iter()
                .take(self.shape.parameter_count)
                .zip(args)
                .all(|(parameter, argument)| parameter.same_type(argument))
    }
}

/// Only supported locals enter the renderer. Canonical metadata is materialized
/// from these preparation-local views, never interpreted as a second graph.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum CallLocal {
    Nullary(CustomLocalShape),
    Custom(CustomLocalShape),
    Tuple {
        local: TupleLocalId,
        type_: Table<ValueType>,
    },
    Int(IntLocalId),
    IntList {
        local: IntListLocalId,
        type_id: IntListTypeId,
    },
    IntFunction {
        local: IntFunctionLocalId,
        type_: FunctionType,
    },
    Bool(BoolLocalId),
    BoolList {
        local: BoolListLocalId,
        type_id: BoolListTypeId,
    },
    BoolFunction {
        local: BoolFunctionLocalId,
        type_: FunctionType,
    },
    Float(FloatLocalId),
    FloatList {
        local: FloatListLocalId,
        type_id: FloatListTypeId,
    },
    FloatFunction {
        local: FloatFunctionLocalId,
        type_: FunctionType,
    },
    String(StringLocalId),
    StringList {
        local: StringListLocalId,
        type_id: StringListTypeId,
    },
    StringFunction {
        local: StringFunctionLocalId,
        type_: FunctionType,
    },
    BitArray(BitArrayLocalId),
    BitArrayList {
        local: BitArrayListLocalId,
        type_id: BitArrayListTypeId,
    },
    BitArrayFunction {
        local: BitArrayFunctionLocalId,
        type_: FunctionType,
    },
    UtfCodepoint(UtfCodepointLocalId),
    UtfCodepointList {
        local: UtfCodepointListLocalId,
        type_id: UtfCodepointListTypeId,
    },
    UtfCodepointFunction {
        local: UtfCodepointFunctionLocalId,
        type_: FunctionType,
    },
    Nil(NilLocalId),
    NilList {
        local: NilListLocalId,
        type_id: NilListTypeId,
    },
    NilFunction {
        local: NilFunctionLocalId,
        type_: FunctionType,
    },
}

impl CallLocal {
    pub(super) fn native_argument(&self) -> bool {
        matches!(
            self,
            Self::Int(_)
                | Self::Float(_)
                | Self::String(_)
                | Self::BitArray(_)
                | Self::UtfCodepoint(_)
                | Self::Bool(_)
                | Self::Nil(_)
                | Self::Nullary(_)
        )
    }

    pub(super) fn same_type(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Nullary(left), Self::Nullary(right))
            | (Self::Custom(left), Self::Custom(right))
            | (Self::Custom(left), Self::Nullary(right)) => left.accepts(right),
            (Self::Tuple { type_: left, .. }, Self::Tuple { type_: right, .. }) => left == right,
            (Self::Int(_), Self::Int(_))
            | (Self::Bool(_), Self::Bool(_))
            | (Self::Float(_), Self::Float(_))
            | (Self::String(_), Self::String(_))
            | (Self::BitArray(_), Self::BitArray(_))
            | (Self::UtfCodepoint(_), Self::UtfCodepoint(_))
            | (Self::Nil(_), Self::Nil(_)) => true,
            (Self::IntList { type_id: left, .. }, Self::IntList { type_id: right, .. }) => {
                left == right
            }
            (Self::BoolList { type_id: left, .. }, Self::BoolList { type_id: right, .. }) => {
                left == right
            }
            (Self::FloatList { type_id: left, .. }, Self::FloatList { type_id: right, .. }) => {
                left == right
            }
            (Self::StringList { type_id: left, .. }, Self::StringList { type_id: right, .. }) => {
                left == right
            }
            (
                Self::BitArrayList { type_id: left, .. },
                Self::BitArrayList { type_id: right, .. },
            ) => left == right,
            (
                Self::UtfCodepointList { type_id: left, .. },
                Self::UtfCodepointList { type_id: right, .. },
            ) => left == right,
            (Self::NilList { type_id: left, .. }, Self::NilList { type_id: right, .. }) => {
                left == right
            }
            (Self::IntFunction { type_: left, .. }, Self::IntFunction { type_: right, .. })
            | (Self::BoolFunction { type_: left, .. }, Self::BoolFunction { type_: right, .. })
            | (Self::FloatFunction { type_: left, .. }, Self::FloatFunction { type_: right, .. })
            | (
                Self::StringFunction { type_: left, .. },
                Self::StringFunction { type_: right, .. },
            )
            | (
                Self::BitArrayFunction { type_: left, .. },
                Self::BitArrayFunction { type_: right, .. },
            )
            | (
                Self::UtfCodepointFunction { type_: left, .. },
                Self::UtfCodepointFunction { type_: right, .. },
            )
            | (Self::NilFunction { type_: left, .. }, Self::NilFunction { type_: right, .. }) => {
                left == right
            }
            _ => false,
        }
    }

    pub(super) fn inspect(local: &ParamLocal) -> Option<Self> {
        Some(match local {
            ParamLocal::Tuple { local, type_ } => Self::Tuple {
                local: *local,
                type_: type_.clone(),
            },
            ParamLocal::Int(local) => Self::Int(*local),
            ParamLocal::List(ListLocal::Int { local, type_id }) => Self::IntList {
                local: *local,
                type_id: *type_id,
            },
            ParamLocal::IntFunction { local, type_ } => Self::IntFunction {
                local: *local,
                type_: type_.clone(),
            },
            ParamLocal::Bool(local) => Self::Bool(*local),
            ParamLocal::List(ListLocal::Bool { local, type_id }) => Self::BoolList {
                local: *local,
                type_id: *type_id,
            },
            ParamLocal::BoolFunction { local, type_ } => Self::BoolFunction {
                local: *local,
                type_: type_.clone(),
            },
            ParamLocal::Float(local) => Self::Float(*local),
            ParamLocal::List(ListLocal::Float { local, type_id }) => Self::FloatList {
                local: *local,
                type_id: *type_id,
            },
            ParamLocal::FloatFunction { local, type_ } => Self::FloatFunction {
                local: *local,
                type_: type_.clone(),
            },
            ParamLocal::String(local) => Self::String(*local),
            ParamLocal::List(ListLocal::String { local, type_id }) => Self::StringList {
                local: *local,
                type_id: *type_id,
            },
            ParamLocal::StringFunction { local, type_ } => Self::StringFunction {
                local: *local,
                type_: type_.clone(),
            },
            ParamLocal::BitArray(local) => Self::BitArray(*local),
            ParamLocal::List(ListLocal::BitArray { local, type_id }) => Self::BitArrayList {
                local: *local,
                type_id: *type_id,
            },
            ParamLocal::BitArrayFunction { local, type_ } => Self::BitArrayFunction {
                local: *local,
                type_: type_.clone(),
            },
            ParamLocal::UtfCodepoint(local) => Self::UtfCodepoint(*local),
            ParamLocal::List(ListLocal::UtfCodepoint { local, type_id }) => {
                Self::UtfCodepointList {
                    local: *local,
                    type_id: *type_id,
                }
            }
            ParamLocal::UtfCodepointFunction { local, type_ } => Self::UtfCodepointFunction {
                local: *local,
                type_: type_.clone(),
            },
            ParamLocal::Nil(local) => Self::Nil(*local),
            ParamLocal::List(ListLocal::Nil { local, type_id }) => Self::NilList {
                local: *local,
                type_id: *type_id,
            },
            ParamLocal::NilFunction { local, type_ } => Self::NilFunction {
                local: *local,
                type_: type_.clone(),
            },
            _ => return None,
        })
    }

    pub(super) fn canonical(&self) -> ParamLocal {
        match self {
            Self::Nullary(value) | Self::Custom(value) => ParamLocal::Custom(value.local),
            Self::Tuple { local, type_ } => ParamLocal::Tuple {
                local: *local,
                type_: type_.clone(),
            },
            Self::Int(local) => ParamLocal::Int(*local),
            Self::IntList { local, type_id } => ParamLocal::List(ListLocal::Int {
                local: *local,
                type_id: *type_id,
            }),
            Self::IntFunction { local, type_ } => ParamLocal::IntFunction {
                local: *local,
                type_: type_.clone(),
            },
            Self::Bool(local) => ParamLocal::Bool(*local),
            Self::BoolList { local, type_id } => ParamLocal::List(ListLocal::Bool {
                local: *local,
                type_id: *type_id,
            }),
            Self::BoolFunction { local, type_ } => ParamLocal::BoolFunction {
                local: *local,
                type_: type_.clone(),
            },
            Self::Float(local) => ParamLocal::Float(*local),
            Self::FloatList { local, type_id } => ParamLocal::List(ListLocal::Float {
                local: *local,
                type_id: *type_id,
            }),
            Self::FloatFunction { local, type_ } => ParamLocal::FloatFunction {
                local: *local,
                type_: type_.clone(),
            },
            Self::String(local) => ParamLocal::String(*local),
            Self::StringList { local, type_id } => ParamLocal::List(ListLocal::String {
                local: *local,
                type_id: *type_id,
            }),
            Self::StringFunction { local, type_ } => ParamLocal::StringFunction {
                local: *local,
                type_: type_.clone(),
            },
            Self::BitArray(local) => ParamLocal::BitArray(*local),
            Self::BitArrayList { local, type_id } => ParamLocal::List(ListLocal::BitArray {
                local: *local,
                type_id: *type_id,
            }),
            Self::BitArrayFunction { local, type_ } => ParamLocal::BitArrayFunction {
                local: *local,
                type_: type_.clone(),
            },
            Self::UtfCodepoint(local) => ParamLocal::UtfCodepoint(*local),
            Self::UtfCodepointList { local, type_id } => {
                ParamLocal::List(ListLocal::UtfCodepoint {
                    local: *local,
                    type_id: *type_id,
                })
            }
            Self::UtfCodepointFunction { local, type_ } => ParamLocal::UtfCodepointFunction {
                local: *local,
                type_: type_.clone(),
            },
            Self::Nil(local) => ParamLocal::Nil(*local),
            Self::NilList { local, type_id } => ParamLocal::List(ListLocal::Nil {
                local: *local,
                type_id: *type_id,
            }),
            Self::NilFunction { local, type_ } => ParamLocal::NilFunction {
                local: *local,
                type_: type_.clone(),
            },
        }
    }
}

pub(super) struct CallInvocation {
    pub point: usize,
    pub output: CallLocal,
    pub target: CallContractTarget,
    pub args: Vec<CallLocal>,
    pub site: HostCallSite,
}

impl CallInvocation {
    pub(super) fn contract(&self) -> CallContract {
        CallContract {
            point: self.point,
            output: self.output.canonical(),
            target: self.target.clone(),
            args: self.args.iter().map(CallLocal::canonical).collect(),
            site: self.site.clone(),
        }
    }
}

pub(super) enum CallableTarget {
    Int(IntFunctionId),
    Bool(BoolFunctionId),
    Float(FloatFunctionId),
    String(StringFunctionId),
    BitArray(BitArrayFunctionId),
    UtfCodepoint(UtfCodepointFunctionId),
    Nil(NilFunctionId),
}

pub(super) enum Capture {
    Int {
        target: IntLocalId,
        source: IntLocalId,
    },
    IntList {
        target: IntListLocalId,
        source: IntListLocalId,
    },
    IntFunction {
        target: IntFunctionLocalId,
        source: IntFunctionLocalId,
    },
    Bool {
        target: BoolLocalId,
        source: BoolLocalId,
    },
    BoolList {
        target: BoolListLocalId,
        source: BoolListLocalId,
    },
    BoolFunction {
        target: BoolFunctionLocalId,
        source: BoolFunctionLocalId,
    },
    Float {
        target: FloatLocalId,
        source: FloatLocalId,
    },
    FloatList {
        target: FloatListLocalId,
        source: FloatListLocalId,
    },
    FloatFunction {
        target: FloatFunctionLocalId,
        source: FloatFunctionLocalId,
    },
    String {
        target: StringLocalId,
        source: StringLocalId,
    },
    StringList {
        target: StringListLocalId,
        source: StringListLocalId,
    },
    StringFunction {
        target: StringFunctionLocalId,
        source: StringFunctionLocalId,
    },
    BitArray {
        target: BitArrayLocalId,
        source: BitArrayLocalId,
    },
    BitArrayList {
        target: BitArrayListLocalId,
        source: BitArrayListLocalId,
    },
    BitArrayFunction {
        target: BitArrayFunctionLocalId,
        source: BitArrayFunctionLocalId,
    },
    UtfCodepoint {
        target: UtfCodepointLocalId,
        source: UtfCodepointLocalId,
    },
    UtfCodepointList {
        target: UtfCodepointListLocalId,
        source: UtfCodepointListLocalId,
    },
    UtfCodepointFunction {
        target: UtfCodepointFunctionLocalId,
        source: UtfCodepointFunctionLocalId,
    },
    Nil {
        target: NilLocalId,
        source: NilLocalId,
    },
    NilList {
        target: NilListLocalId,
        source: NilListLocalId,
    },
    NilFunction {
        target: NilFunctionLocalId,
        source: NilFunctionLocalId,
    },
}

impl Capture {
    fn inspect(capture: &FunctionCapture) -> Option<Self> {
        Some(match capture {
            FunctionCapture::Int { target, source } => Self::Int {
                target: *target,
                source: *source,
            },
            FunctionCapture::IntList { target, source } => Self::IntList {
                target: *target,
                source: *source,
            },
            FunctionCapture::IntFunction { target, source } => Self::IntFunction {
                target: *target,
                source: *source,
            },
            FunctionCapture::Bool { target, source } => Self::Bool {
                target: *target,
                source: *source,
            },
            FunctionCapture::BoolList { target, source } => Self::BoolList {
                target: *target,
                source: *source,
            },
            FunctionCapture::BoolFunction { target, source } => Self::BoolFunction {
                target: *target,
                source: *source,
            },
            FunctionCapture::Float { target, source } => Self::Float {
                target: *target,
                source: *source,
            },
            FunctionCapture::FloatList { target, source } => Self::FloatList {
                target: *target,
                source: *source,
            },
            FunctionCapture::FloatFunction { target, source } => Self::FloatFunction {
                target: *target,
                source: *source,
            },
            FunctionCapture::String { target, source } => Self::String {
                target: *target,
                source: *source,
            },
            FunctionCapture::StringList { target, source } => Self::StringList {
                target: *target,
                source: *source,
            },
            FunctionCapture::StringFunction { target, source } => Self::StringFunction {
                target: *target,
                source: *source,
            },
            FunctionCapture::BitArray { target, source } => Self::BitArray {
                target: *target,
                source: *source,
            },
            FunctionCapture::BitArrayList { target, source } => Self::BitArrayList {
                target: *target,
                source: *source,
            },
            FunctionCapture::BitArrayFunction { target, source } => Self::BitArrayFunction {
                target: *target,
                source: *source,
            },
            FunctionCapture::UtfCodepoint { target, source } => Self::UtfCodepoint {
                target: *target,
                source: *source,
            },
            FunctionCapture::UtfCodepointList { target, source } => Self::UtfCodepointList {
                target: *target,
                source: *source,
            },
            FunctionCapture::UtfCodepointFunction { target, source } => {
                Self::UtfCodepointFunction {
                    target: *target,
                    source: *source,
                }
            }
            FunctionCapture::Nil { target, source } => Self::Nil {
                target: *target,
                source: *source,
            },
            FunctionCapture::NilList { target, source } => Self::NilList {
                target: *target,
                source: *source,
            },
            FunctionCapture::NilFunction { target, source } => Self::NilFunction {
                target: *target,
                source: *source,
            },
            _ => return None,
        })
    }

    fn canonical(&self) -> FunctionCapture {
        match self {
            Self::Int { target, source } => FunctionCapture::Int {
                target: *target,
                source: *source,
            },
            Self::IntList { target, source } => FunctionCapture::IntList {
                target: *target,
                source: *source,
            },
            Self::IntFunction { target, source } => FunctionCapture::IntFunction {
                target: *target,
                source: *source,
            },
            Self::Bool { target, source } => FunctionCapture::Bool {
                target: *target,
                source: *source,
            },
            Self::BoolList { target, source } => FunctionCapture::BoolList {
                target: *target,
                source: *source,
            },
            Self::BoolFunction { target, source } => FunctionCapture::BoolFunction {
                target: *target,
                source: *source,
            },
            Self::Float { target, source } => FunctionCapture::Float {
                target: *target,
                source: *source,
            },
            Self::FloatList { target, source } => FunctionCapture::FloatList {
                target: *target,
                source: *source,
            },
            Self::FloatFunction { target, source } => FunctionCapture::FloatFunction {
                target: *target,
                source: *source,
            },
            Self::String { target, source } => FunctionCapture::String {
                target: *target,
                source: *source,
            },
            Self::StringList { target, source } => FunctionCapture::StringList {
                target: *target,
                source: *source,
            },
            Self::StringFunction { target, source } => FunctionCapture::StringFunction {
                target: *target,
                source: *source,
            },
            Self::BitArray { target, source } => FunctionCapture::BitArray {
                target: *target,
                source: *source,
            },
            Self::BitArrayList { target, source } => FunctionCapture::BitArrayList {
                target: *target,
                source: *source,
            },
            Self::BitArrayFunction { target, source } => FunctionCapture::BitArrayFunction {
                target: *target,
                source: *source,
            },
            Self::UtfCodepoint { target, source } => FunctionCapture::UtfCodepoint {
                target: *target,
                source: *source,
            },
            Self::UtfCodepointList { target, source } => FunctionCapture::UtfCodepointList {
                target: *target,
                source: *source,
            },
            Self::UtfCodepointFunction { target, source } => {
                FunctionCapture::UtfCodepointFunction {
                    target: *target,
                    source: *source,
                }
            }
            Self::Nil { target, source } => FunctionCapture::Nil {
                target: *target,
                source: *source,
            },
            Self::NilList { target, source } => FunctionCapture::NilList {
                target: *target,
                source: *source,
            },
            Self::NilFunction { target, source } => FunctionCapture::NilFunction {
                target: *target,
                source: *source,
            },
        }
    }
}

pub(super) struct CallCreation {
    pub point: usize,
    pub output: CallLocal,
    pub target: CallableTarget,
    pub type_: FunctionType,
    pub reference: bool,
    pub captures: Vec<Capture>,
}

impl CallCreation {
    pub(super) fn contract(&self) -> CreationContract {
        CreationContract {
            point: self.point,
            output: self.output.canonical(),
            target: match self.target {
                CallableTarget::Int(id) => FunctionTarget::Int(id),
                CallableTarget::Bool(id) => FunctionTarget::Bool(id),
                CallableTarget::Float(id) => FunctionTarget::Float(id),
                CallableTarget::String(id) => FunctionTarget::String(id),
                CallableTarget::BitArray(id) => FunctionTarget::BitArray(id),
                CallableTarget::UtfCodepoint(id) => FunctionTarget::UtfCodepoint(id),
                CallableTarget::Nil(id) => FunctionTarget::Nil(id),
            },
            type_: self.type_.clone(),
            reference: self.reference,
            captures: self.captures.iter().map(Capture::canonical).collect(),
        }
    }
}

pub(super) struct CallReturn {
    pub point: usize,
    pub value: CallLocal,
}

impl CallReturn {
    pub(super) fn contract(&self) -> ReturnContract {
        ReturnContract {
            point: self.point,
            value: self.value.canonical(),
        }
    }
}

pub(super) struct CallTail {
    pub point: usize,
    pub target: CallTarget,
    pub args: Vec<CallLocal>,
    pub site: HostCallSite,
}

impl CallTail {
    pub(super) fn contract(&self) -> TailContract {
        TailContract {
            point: self.point,
            target: self.target,
            args: self.args.iter().map(CallLocal::canonical).collect(),
            site: self.site.clone(),
        }
    }
}

/// Phase-local views of the frozen graph. The executable sidecar stores only
/// its admission contracts; generated Rust owns the actual typed call states.
pub(in crate::plan::execution::prepared) struct CallShape<'graph> {
    pub root: bool,
    pub parameter_count: usize,
    entry: usize,
    pub checkpoints: Vec<CompiledCheckpoint>,
    pub(super) locals: Vec<Vec<CallLocal>>,
    pub(super) calls: Vec<CallInvocation>,
    pub(super) creations: Vec<CallCreation>,
    pub(super) returns: Vec<CallReturn>,
    pub(super) tails: Vec<CallTail>,
    pub(super) points: Vec<CallPoint<'graph>>,
    pub(super) starts: BTreeMap<usize, usize>,
}

pub(super) enum CallPoint<'graph> {
    Scalar(CallScalar<'graph>),
    Call(usize),
    Create(usize),
    Terminator(CallTerminator<'graph>),
    Return(usize),
    Tail(usize),
    Interpreted,
}

impl CallPoint<'_> {
    pub(super) fn is_inline(&self) -> bool {
        matches!(self, Self::Scalar(_) | Self::Create(_))
    }
}

impl<'graph> CallShape<'graph> {
    fn callee_kernel<Return, Tail, Graph: ExecutionGraphProfile>(
        body: &'graph ProfiledFunctionBody<Return, Tail, Graph>,
    ) -> Option<CompiledShape<'graph, Graph>> {
        if !body
            .exits
            .iter()
            .all(|exit| matches!(exit, FunctionExit::Return(_)))
        {
            return None;
        }
        let kernel = CompiledShape::inspect(body).or_else(|| CompiledShape::inspect_bits(body))?;
        if !matches!(
            kernel.kind,
            KernelKind::Numeric | KernelKind::String | KernelKind::BitArray
        ) {
            return None;
        }
        // Identity and literal-only leaves use direct locals. Creating range
        // scratch for those leaves adds ownership work without a range operation.
        if kernel.kind != KernelKind::Numeric
            && !kernel.blocks.values().any(|block| {
                matches!(
                    block.terminator,
                    CompiledTerminator::StringSwitch { .. }
                        | CompiledTerminator::StringMatch(_)
                        | CompiledTerminator::BitArray(_)
                        | CompiledTerminator::Test {
                            test: CompiledTest::String(_),
                            ..
                        }
                ) || block.instructions.iter().any(|instruction| {
                    matches!(
                        instruction,
                        CompiledInstruction::Boolean(
                            _,
                            CompiledBoolean::Test(CompiledTest::String(_))
                        )
                    )
                })
            })
        {
            return None;
        }
        // Every canonical instruction must be covered. In particular a partial
        // String prefix ending at a native call is not a call-free callee.
        if kernel.blocks.len() != kernel.graph.blocks().len()
            || kernel.blocks.iter().any(|(&block, compiled)| {
                compiled.instructions.len()
                    != kernel.graph.block(BlockId(block)).instructions().len()
            })
        {
            return None;
        }
        Some(kernel)
    }

    fn from_kernel<Return, Tail, Graph: ExecutionGraphProfile>(
        body: &'graph ProfiledFunctionBody<Return, Tail, Graph>,
        parameter_count: usize,
        kernel: &CompiledShape<'graph, Graph>,
        return_local: impl Fn(&Return) -> CallLocal,
    ) -> Self {
        let mut shape = Self {
            root: false,
            parameter_count,
            entry: kernel.start(kernel.graph.entry()),
            checkpoints: kernel.checkpoints.clone(),
            locals: Vec::new(),
            calls: Vec::new(),
            creations: Vec::new(),
            returns: Vec::new(),
            tails: Vec::new(),
            points: Vec::new(),
            starts: kernel.starts.clone(),
        };
        for (point, checkpoint) in kernel.checkpoints.iter().enumerate() {
            let block = kernel.graph.block(checkpoint.block);
            let locals = block
                .params()
                .iter()
                .map(|slot| slot.local())
                .chain(
                    block.instructions()[..checkpoint.instruction]
                        .iter()
                        .flat_map(|instruction| instruction.outputs().map(|slot| slot.local())),
                )
                // The complete kernel already admits every parameter and
                // preceding output in the scalar or IntList families accepted
                // here. Preserve source layout order without a second admission.
                .filter_map(CallLocal::inspect)
                .collect();
            shape.locals.push(locals);
            let action = if checkpoint.instruction == block.instructions().len()
                && let Terminator::Exit(exit) = block.terminator()
                && let FunctionExit::Return(value) = body.exit(*exit)
            {
                let index = shape.returns.len();
                shape.returns.push(CallReturn {
                    point,
                    value: return_local(value),
                });
                CallPoint::Return(index)
            } else {
                CallPoint::Interpreted
            };
            shape.points.push(action);
        }
        shape
    }

    fn inspect<Return, Tail, Graph: ExecutionGraphProfile>(
        types: &CallTypes<'_>,
        body: &'graph ProfiledFunctionBody<Return, Tail, Graph>,
        parameter_count: usize,
        returning_callable: bool,
        is_return: impl Fn(&Return, &CallLocal) -> bool,
        tail_target: impl Fn(&Tail) -> CallTarget,
        tail_site: impl Fn(&Tail) -> HostCallSite,
    ) -> Option<Self> {
        let graph = body.block_graph().as_view();
        let mut shape = Self {
            root: returning_callable,
            parameter_count,
            entry: 0,
            checkpoints: Vec::new(),
            locals: Vec::new(),
            calls: Vec::new(),
            creations: Vec::new(),
            returns: Vec::new(),
            tails: Vec::new(),
            points: Vec::new(),
            starts: BTreeMap::new(),
        };
        for (index, block) in graph.blocks().enumerate() {
            let Some(mut locals) = block
                .params()
                .iter()
                .map(|slot| types.local(slot.local()))
                .collect::<Option<Vec<_>>>()
            else {
                continue;
            };
            shape.root |= locals.iter().any(callable_local);
            shape.starts.insert(index, shape.points.len());
            let mut complete = true;
            for (instruction_index, instruction) in block.instructions().iter().enumerate() {
                let point = shape.push_point(BlockId(index), instruction_index, &locals);
                let Some(instruction) = inspect_instruction(instruction, point, &mut shape, types)
                else {
                    shape.points.push(CallPoint::Interpreted);
                    complete = false;
                    break;
                };
                shape.points.push(instruction);
                locals.extend(
                    block.instructions()[instruction_index]
                        .outputs()
                        .filter_map(|slot| types.local(slot.local())),
                );
            }
            if complete {
                let point = shape.push_point(BlockId(index), block.instructions().len(), &locals);
                let terminator = match block.terminator() {
                    Terminator::Exit(exit) => match body.exit(*exit) {
                        FunctionExit::Return(value) => {
                            let index = shape.returns.len();
                            // A complete block has classified the unique local
                            // named by this admitted return. Retain that local's
                            // metadata instead of performing another admission.
                            shape.returns.extend(
                                locals
                                    .iter()
                                    .filter(|local| is_return(value, local))
                                    .cloned()
                                    .map(|value| CallReturn { point, value }),
                            );
                            CallPoint::Return(index)
                        }
                        FunctionExit::TailCall { function, args, .. } => {
                            // A complete block has already admitted these
                            // locals through its parameters or instructions.
                            let args = args.iter().filter_map(|local| types.local(local)).collect();
                            let index = shape.tails.len();
                            shape.tails.push(CallTail {
                                point,
                                target: tail_target(function),
                                args,
                                site: tail_site(function),
                            });
                            CallPoint::Tail(index)
                        }
                    },
                    Terminator::Match(matcher) => super::matching::CompoundMatch::inspect(
                        matcher,
                        types,
                        &locals,
                        graph.block(matcher.success().target()).params(),
                    )
                    .map(|matcher| CallPoint::Terminator(CallTerminator::Match(Box::new(matcher))))
                    .unwrap_or(CallPoint::Interpreted),
                    terminator => CallTerminator::inspect(terminator)
                        .map(CallPoint::Terminator)
                        .unwrap_or(CallPoint::Interpreted),
                };
                shape.points.push(terminator);
            }
        }
        // A generated transfer needs every destination column. A block with
        // unsupported parameters was deliberately left to the canonical
        // graph; its incoming edge must stay there too.
        let declined = shape
            .points
            .iter()
            .enumerate()
            .filter_map(|(point, operation)| {
                let CallPoint::Terminator(terminator) = operation else {
                    return None;
                };
                let supported = terminator.accepts_edges(|edge| {
                    shape.starts.contains_key(&edge.target().index())
                        && edge
                            .args()
                            .iter()
                            .all(|argument| types.local(argument).is_some())
                });
                (!supported).then_some(point)
            })
            .collect::<Vec<_>>();
        for point in declined {
            shape.points[point] = CallPoint::Interpreted;
        }
        let entry = *shape.starts.get(&graph.entry().index())?;
        // Existing call-free List bodies keep their dedicated IntList kernel.
        // A List-owning caller is selected only when calls or callable values
        // make connecting it to this engine useful.
        if matches!(shape.points[entry], CallPoint::Interpreted)
            || (!shape.root && shape.has_lists())
        {
            return None;
        }
        if shape.locals[entry]
            .iter()
            .skip(parameter_count)
            .any(|local| matches!(local, CallLocal::Nullary(_)))
        {
            return None;
        }
        shape.entry = entry;
        Some(shape)
    }

    pub(in crate::plan::execution::prepared) fn entry(&self) -> usize {
        self.entry
    }

    fn has_lists(&self) -> bool {
        self.locals.iter().flatten().any(|local| {
            matches!(
                local,
                CallLocal::IntList { .. }
                    | CallLocal::BoolList { .. }
                    | CallLocal::FloatList { .. }
                    | CallLocal::StringList { .. }
                    | CallLocal::BitArrayList { .. }
                    | CallLocal::UtfCodepointList { .. }
                    | CallLocal::NilList { .. }
            )
        })
    }

    pub(in crate::plan::execution::prepared) fn local_contracts(&self) -> Vec<Vec<ParamLocal>> {
        self.locals
            .iter()
            .map(|locals| locals.iter().map(CallLocal::canonical).collect())
            .collect()
    }
    pub(in crate::plan::execution::prepared) fn call_contracts(&self) -> Vec<CallContract> {
        self.calls.iter().map(CallInvocation::contract).collect()
    }
    pub(in crate::plan::execution::prepared) fn creation_contracts(&self) -> Vec<CreationContract> {
        self.creations.iter().map(CallCreation::contract).collect()
    }
    pub(in crate::plan::execution::prepared) fn return_contracts(&self) -> Vec<ReturnContract> {
        self.returns.iter().map(CallReturn::contract).collect()
    }
    pub(in crate::plan::execution::prepared) fn tail_contracts(&self) -> Vec<TailContract> {
        self.tails.iter().map(CallTail::contract).collect()
    }

    fn push_point(&mut self, block: BlockId, instruction: usize, locals: &[CallLocal]) -> usize {
        let point = self.checkpoints.len();
        self.checkpoints.push(CompiledCheckpoint {
            block,
            instruction,
            ints: locals
                .iter()
                .filter(|local| matches!(local, CallLocal::Int(_)))
                .count(),
            bools: locals
                .iter()
                .filter(|local| matches!(local, CallLocal::Bool(_)))
                .count(),
            bit_arrays: locals
                .iter()
                .filter(|local| matches!(local, CallLocal::BitArray(_)))
                .count(),
            int_lists: locals
                .iter()
                .filter(|local| matches!(local, CallLocal::IntList { .. }))
                .count(),
            strings: locals
                .iter()
                .filter(|local| matches!(local, CallLocal::String(_)))
                .count(),
            customs: locals
                .iter()
                .filter(|local| matches!(local, CallLocal::Nullary(_) | CallLocal::Custom(_)))
                .count(),
            custom_lists: 0,
            int_functions: locals
                .iter()
                .filter(|local| matches!(local, CallLocal::IntFunction { .. }))
                .count(),
            bool_functions: locals
                .iter()
                .filter(|local| matches!(local, CallLocal::BoolFunction { .. }))
                .count(),
        });
        self.locals.push(locals.to_vec());
        point
    }
}

pub(super) enum CallScalar<'graph> {
    CompoundField {
        output: CallLocal,
        source: CallLocal,
        index: usize,
        custom: bool,
        read: super::matching::FieldRead,
    },
    Nullary {
        output: crate::plan::execution::graph::CustomLocalId,
        constructor: crate::plan::execution::type_::CustomConstructorId,
    },
    Integer(IntLocalId, NumericInteger<'graph>),
    Boolean(BoolLocalId, CallBoolean),
    IntList(IntListInstruction<'graph>),
    Float(FloatLocalId, FloatOperation),
    String(StringLocalId, StringOperation<'graph>),
    Nil,
    List {
        output: PrimitiveListLocal,
        operation: PrimitiveListOperation,
    },
    Index {
        output: CallLocal,
        list: PrimitiveListLocal,
        index: usize,
    },
    Region {
        region: &'graph ArithmeticRegion,
        outputs: Vec<IntLocalId>,
    },
}

/// Only the six additional primitive list channels reach these operations.
/// Their exact local and list type remain paired throughout preparation.
#[derive(Clone)]
pub(super) enum PrimitiveListLocal {
    Bool {
        local: BoolListLocalId,
        type_id: BoolListTypeId,
    },
    Float {
        local: FloatListLocalId,
        type_id: FloatListTypeId,
    },
    String {
        local: StringListLocalId,
        type_id: StringListTypeId,
    },
    BitArray {
        local: BitArrayListLocalId,
        type_id: BitArrayListTypeId,
    },
    UtfCodepoint {
        local: UtfCodepointListLocalId,
        type_id: UtfCodepointListTypeId,
    },
    Nil {
        local: NilListLocalId,
        type_id: NilListTypeId,
    },
}

impl PrimitiveListLocal {
    pub(super) fn canonical(&self) -> CallLocal {
        match self {
            Self::Bool { local, type_id } => CallLocal::BoolList {
                local: *local,
                type_id: *type_id,
            },
            Self::Float { local, type_id } => CallLocal::FloatList {
                local: *local,
                type_id: *type_id,
            },
            Self::String { local, type_id } => CallLocal::StringList {
                local: *local,
                type_id: *type_id,
            },
            Self::BitArray { local, type_id } => CallLocal::BitArrayList {
                local: *local,
                type_id: *type_id,
            },
            Self::UtfCodepoint { local, type_id } => CallLocal::UtfCodepointList {
                local: *local,
                type_id: *type_id,
            },
            Self::Nil { local, type_id } => CallLocal::NilList {
                local: *local,
                type_id: *type_id,
            },
        }
    }
}

pub(super) enum FloatOperation {
    Value(f64),
    Add(FloatLocalId, FloatLocalId),
    Subtract(FloatLocalId, FloatLocalId),
    Multiply(FloatLocalId, FloatLocalId),
    Divide(FloatLocalId, FloatLocalId),
}

impl FloatOperation {
    fn inspect(instruction: &FloatInstruction) -> Option<Self> {
        Some(match instruction {
            FloatInstruction::Value(value) => Self::Value(*value),
            FloatInstruction::Add { left, right } => Self::Add(*left, *right),
            FloatInstruction::Sub { left, right } => Self::Subtract(*left, *right),
            FloatInstruction::Mult { left, right } => Self::Multiply(*left, *right),
            FloatInstruction::Div { left, right } => Self::Divide(*left, *right),
            _ => return None,
        })
    }
}

/// A preparation-local projection of Value, Spread, and DropFirst. Exact
/// typed local IDs remain attached; runtime storage is never generalized.
pub(super) enum PrimitiveListOperation {
    Value(Vec<CallLocal>),
    Spread {
        elements: Vec<CallLocal>,
        tail: CallLocal,
    },
    Tail {
        list: CallLocal,
        count: usize,
    },
}

impl PrimitiveListOperation {
    fn inspect<Element: 'static, Local: Copy, Function>(
        instruction: &TypedListInstruction<Element, Local, Function>,
        element: impl Fn(&Element) -> CallLocal,
        local: impl Fn(Local) -> CallLocal,
    ) -> Option<Self> {
        Some(match instruction {
            TypedListInstruction::Value(elements) => {
                Self::Value(elements.iter().map(element).collect())
            }
            TypedListInstruction::Spread { elements, tail } => Self::Spread {
                elements: elements.iter().map(element).collect(),
                tail: local(*tail),
            },
            TypedListInstruction::DropFirst { list, count } => Self::Tail {
                list: local(*list),
                count: *count,
            },
            _ => return None,
        })
    }
}

pub(super) enum CallBoolean {
    Value(bool),
    Test(CallTest),
}

pub(super) enum FloatComparison {
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

pub(super) enum CallTest {
    Not(BoolLocalId),
    Compare(NumericComparison, IntegerOperand, IntegerOperand),
    IntList(IntListTest),
    FloatCompare(FloatComparison, FloatLocalId, FloatLocalId),
    Equal {
        left: CallLocal,
        right: CallLocal,
        negate: bool,
    },
    Length {
        list: CallLocal,
        length: usize,
        at_least: bool,
    },
    StringPrefix {
        value: StringLocalId,
        prefix: String,
    },
}

impl CallTest {
    fn from_compiled(test: CompiledTest<'_>) -> Option<Self> {
        match test {
            CompiledTest::Not(local) => Some(Self::Not(local)),
            CompiledTest::Compare(comparison, left, right) => {
                Some(Self::Compare(comparison, left, right))
            }
            CompiledTest::IntList(test) => Some(Self::IntList(test)),
            CompiledTest::String(StringTest::Prefix { value, prefix }) => {
                Some(Self::StringPrefix {
                    value,
                    prefix: prefix.to_owned(),
                })
            }
            CompiledTest::String(StringTest::Equal {
                left,
                right,
                negate,
            }) => Some(Self::Equal {
                left: CallLocal::String(left),
                right: CallLocal::String(right),
                negate,
            }),
            CompiledTest::BoolEqual {
                left,
                right,
                negate,
            } => Some(Self::Equal {
                left: CallLocal::Bool(left),
                right: CallLocal::Bool(right),
                negate,
            }),
            CompiledTest::CustomListLength { .. } => None,
        }
    }

    fn inspect(test: &BoolTest) -> Option<Self> {
        if let Some(test) = IntListTest::inspect(test) {
            return Some(Self::IntList(test));
        }
        Some(match test {
            BoolTest::Not(local) => Self::Not(*local),
            BoolTest::EqualInt { left, right } => {
                Self::Compare(NumericComparison::Equal, *left, *right)
            }
            BoolTest::NotEqualInt { left, right } => {
                Self::Compare(NumericComparison::NotEqual, *left, *right)
            }
            BoolTest::LtInt { left, right } => {
                Self::Compare(NumericComparison::Less, *left, *right)
            }
            BoolTest::LtEqInt { left, right } => {
                Self::Compare(NumericComparison::LessEqual, *left, *right)
            }
            BoolTest::GtInt { left, right } => {
                Self::Compare(NumericComparison::Greater, *left, *right)
            }
            BoolTest::GtEqInt { left, right } => {
                Self::Compare(NumericComparison::GreaterEqual, *left, *right)
            }
            BoolTest::LtFloat { left, right } => {
                Self::FloatCompare(FloatComparison::Less, *left, *right)
            }
            BoolTest::LtEqFloat { left, right } => {
                Self::FloatCompare(FloatComparison::LessEqual, *left, *right)
            }
            BoolTest::GtFloat { left, right } => {
                Self::FloatCompare(FloatComparison::Greater, *left, *right)
            }
            BoolTest::GtEqFloat { left, right } => {
                Self::FloatCompare(FloatComparison::GreaterEqual, *left, *right)
            }
            BoolTest::Equal { left, right } | BoolTest::NotEqual { left, right } => {
                let left = CallLocal::inspect(left)?;
                let right = CallLocal::inspect(right)?;
                if !left.same_type(&right) || callable_local(&left) {
                    return None;
                }
                Self::Equal {
                    left,
                    right,
                    negate: matches!(test, BoolTest::NotEqual { .. }),
                }
            }
            BoolTest::ListLengthEquals { value, length }
            | BoolTest::ListLengthAtLeast { value, length } => Self::Length {
                list: CallLocal::inspect(&ParamLocal::List(value.clone()))?,
                length: *length,
                at_least: matches!(test, BoolTest::ListLengthAtLeast { .. }),
            },
            BoolTest::StringStartsWith { value, prefix } => Self::StringPrefix {
                value: *value,
                prefix: prefix.as_str().to_owned(),
            },
        })
    }
}

impl<'graph> CallScalar<'graph> {
    fn inspect<Graph: ExecutionGraphProfile>(
        instruction: &'graph ProfiledInstruction<Graph>,
    ) -> Option<Self> {
        if let Some(compiled) = CompiledInstruction::inspect(instruction, KernelKind::Numeric)
            && let Some(scalar) = Self::from_compiled(compiled)
        {
            return Some(scalar);
        }
        let instruction = instruction.value()?;
        let output = CallLocal::inspect(instruction.output().local())?;
        Some(match (instruction.kind(), output) {
            (ProfiledInstructionKind::Float(value), CallLocal::Float(output)) => {
                Self::Float(output, FloatOperation::inspect(value)?)
            }
            (ProfiledInstructionKind::String(value), CallLocal::String(output)) => {
                Self::String(output, StringOperation::inspect(value)?)
            }
            (ProfiledInstructionKind::Nil(NilInstruction::Value), CallLocal::Nil(_)) => Self::Nil,
            (
                ProfiledInstructionKind::Bool(BoolInstruction::Test(test)),
                CallLocal::Bool(output),
            ) => Self::Boolean(output, CallBoolean::Test(CallTest::inspect(test)?)),
            (
                ProfiledInstructionKind::List(ListInstruction::Bool(type_id, value)),
                CallLocal::BoolList {
                    local,
                    type_id: output_type,
                },
            ) => Self::List {
                output: PrimitiveListLocal::Bool {
                    local,
                    type_id: output_type,
                },
                operation: PrimitiveListOperation::inspect(
                    value,
                    |local| CallLocal::Bool(*local),
                    |local| CallLocal::BoolList {
                        local,
                        type_id: *type_id,
                    },
                )?,
            },
            (
                ProfiledInstructionKind::List(ListInstruction::Float(type_id, value)),
                CallLocal::FloatList {
                    local,
                    type_id: output_type,
                },
            ) => Self::List {
                output: PrimitiveListLocal::Float {
                    local,
                    type_id: output_type,
                },
                operation: PrimitiveListOperation::inspect(
                    value,
                    |local| CallLocal::Float(*local),
                    |local| CallLocal::FloatList {
                        local,
                        type_id: *type_id,
                    },
                )?,
            },
            (
                ProfiledInstructionKind::List(ListInstruction::String(type_id, value)),
                CallLocal::StringList {
                    local,
                    type_id: output_type,
                },
            ) => Self::List {
                output: PrimitiveListLocal::String {
                    local,
                    type_id: output_type,
                },
                operation: PrimitiveListOperation::inspect(
                    value,
                    |local| CallLocal::String(*local),
                    |local| CallLocal::StringList {
                        local,
                        type_id: *type_id,
                    },
                )?,
            },
            (
                ProfiledInstructionKind::List(ListInstruction::BitArray(type_id, value)),
                CallLocal::BitArrayList {
                    local,
                    type_id: output_type,
                },
            ) => Self::List {
                output: PrimitiveListLocal::BitArray {
                    local,
                    type_id: output_type,
                },
                operation: PrimitiveListOperation::inspect(
                    value,
                    |local| CallLocal::BitArray(*local),
                    |local| CallLocal::BitArrayList {
                        local,
                        type_id: *type_id,
                    },
                )?,
            },
            (
                ProfiledInstructionKind::List(ListInstruction::UtfCodepoint(type_id, value)),
                CallLocal::UtfCodepointList {
                    local,
                    type_id: output_type,
                },
            ) => Self::List {
                output: PrimitiveListLocal::UtfCodepoint {
                    local,
                    type_id: output_type,
                },
                operation: PrimitiveListOperation::inspect(
                    value,
                    |local| CallLocal::UtfCodepoint(*local),
                    |local| CallLocal::UtfCodepointList {
                        local,
                        type_id: *type_id,
                    },
                )?,
            },
            (
                ProfiledInstructionKind::List(ListInstruction::Nil(type_id, value)),
                CallLocal::NilList {
                    local,
                    type_id: output_type,
                },
            ) => Self::List {
                output: PrimitiveListLocal::Nil {
                    local,
                    type_id: output_type,
                },
                operation: PrimitiveListOperation::inspect(
                    value,
                    |local| CallLocal::Nil(*local),
                    |local| CallLocal::NilList {
                        local,
                        type_id: *type_id,
                    },
                )?,
            },
            _ => return None,
        })
    }

    fn from_compiled(instruction: CompiledInstruction<'graph>) -> Option<Self> {
        match instruction {
            CompiledInstruction::Integer(output, value) => Some(Self::Integer(output, value)),
            CompiledInstruction::Boolean(output, CompiledBoolean::Value(value)) => {
                Some(Self::Boolean(output, CallBoolean::Value(value)))
            }
            CompiledInstruction::Boolean(output, CompiledBoolean::Test(test)) => Some(
                Self::Boolean(output, CallBoolean::Test(CallTest::from_compiled(test)?)),
            ),
            CompiledInstruction::Region { region, outputs } => {
                Some(Self::Region { region, outputs })
            }
            CompiledInstruction::IntList(instruction) => Some(Self::IntList(instruction)),
            CompiledInstruction::String(output, value) => Some(Self::String(output, value)),
            CompiledInstruction::CustomField(_) | CompiledInstruction::CustomLoop(_) => None,
        }
    }
}

pub(super) enum CallTerminator<'graph> {
    Match(Box<super::matching::CompoundMatch<'graph>>),
    Jump(&'graph Edge),
    Boolean {
        subject: BoolLocalId,
        true_: &'graph Edge,
        false_: &'graph Edge,
    },
    Test {
        test: CallTest,
        true_: &'graph Edge,
        false_: &'graph Edge,
    },
    Switch {
        subject: IntLocalId,
        clauses: &'graph [(IntegerLiteral, Edge)],
        fallback: &'graph Edge,
    },
}

impl<'graph> CallTerminator<'graph> {
    pub(super) fn enters(&self, block: BlockId) -> bool {
        let enters = |edge: &Edge| edge.target() == block;
        match self {
            Self::Match(matcher) => matcher.enters(block),
            Self::Jump(edge) => enters(edge),
            Self::Boolean { true_, false_, .. } | Self::Test { true_, false_, .. } => {
                enters(true_) || enters(false_)
            }
            Self::Switch {
                clauses, fallback, ..
            } => clauses.iter().any(|(_, edge)| enters(edge)) || enters(fallback),
        }
    }

    fn accepts_edges(&self, mut accepts: impl FnMut(&Edge) -> bool) -> bool {
        match self {
            Self::Match(matcher) => matcher.accepts_edges(accepts),
            Self::Jump(edge) => accepts(edge),
            Self::Boolean { true_, false_, .. } | Self::Test { true_, false_, .. } => {
                accepts(true_) && accepts(false_)
            }
            Self::Switch {
                clauses, fallback, ..
            } => clauses.iter().all(|(_, edge)| accepts(edge)) && accepts(fallback),
        }
    }

    fn inspect(terminator: &'graph Terminator) -> Option<Self> {
        Some(match terminator {
            Terminator::Jump(jump) => Self::Jump(&jump.edge),
            Terminator::BoolBranch(branch) => Self::Boolean {
                subject: branch.subject,
                true_: &branch.true_,
                false_: &branch.false_,
            },
            Terminator::TestBranch(branch) => Self::Test {
                test: CallTest::inspect(&branch.test)?,
                true_: &branch.true_,
                false_: &branch.false_,
            },
            Terminator::IntSwitch(switch)
                if switch
                    .clauses
                    .iter()
                    .all(|(literal, _)| small_literal(literal)) =>
            {
                Self::Switch {
                    subject: switch.subject,
                    clauses: &switch.clauses,
                    fallback: &switch.fallback,
                }
            }
            _ => return None,
        })
    }
}

fn small_literal(literal: &IntegerLiteral) -> bool {
    i64::try_from(literal.materialize()).is_ok()
}

fn callable_local(local: &CallLocal) -> bool {
    matches!(
        local,
        CallLocal::IntFunction { .. }
            | CallLocal::BoolFunction { .. }
            | CallLocal::FloatFunction { .. }
            | CallLocal::StringFunction { .. }
            | CallLocal::BitArrayFunction { .. }
            | CallLocal::UtfCodepointFunction { .. }
            | CallLocal::NilFunction { .. }
    )
}

fn inspect_instruction<'graph, Graph: ExecutionGraphProfile>(
    instruction: &'graph ProfiledInstruction<Graph>,
    point: usize,
    shape: &mut CallShape<'graph>,
    types: &CallTypes<'_>,
) -> Option<CallPoint<'graph>> {
    // Scalar classification already establishes its typed outputs. Other
    // instructions have one output, inspected once below.
    if let Some(scalar) = CallScalar::inspect(instruction) {
        return Some(CallPoint::Scalar(scalar));
    }
    let instruction = instruction.value()?;
    let output = types.local(instruction.output().local())?;
    if let (
        ProfiledInstructionKind::Custom(CustomInstruction::Construct { constructor, .. }),
        CallLocal::Nullary(value),
    ) = (instruction.kind(), &output)
    {
        // The graph's typed Construct is already admitted. Projecting its
        // output as Nullary proves that this constructor has no fields.
        return Some(CallPoint::Scalar(CallScalar::Nullary {
            output: value.local.id,
            constructor: *constructor,
        }));
    }
    if let Some(read) = super::matching::FieldRead::inspect(&output) {
        match instruction.kind() {
            ProfiledInstructionKind::Int(IntInstruction::TupleIndex { tuple, index })
            | ProfiledInstructionKind::Float(FloatInstruction::TupleIndex { tuple, index })
            | ProfiledInstructionKind::Bool(BoolInstruction::TupleIndex { tuple, index })
            | ProfiledInstructionKind::String(StringInstruction::TupleIndex { tuple, index })
            | ProfiledInstructionKind::BitArray(BitArrayInstruction::TupleIndex { tuple, index })
            | ProfiledInstructionKind::UtfCodepoint(UtfCodepointInstruction::TupleIndex {
                tuple,
                index,
            })
            | ProfiledInstructionKind::Nil(NilInstruction::TupleIndex { tuple, index })
            | ProfiledInstructionKind::Custom(CustomInstruction::TupleIndex { tuple, index })
            | ProfiledInstructionKind::Tuple(TupleInstruction::TupleIndex { tuple, index }) => {
                // Freezing and prepared admission allocate Tuple IDs in
                // family order. Every preceding Tuple is already in this
                // classified prefix, so the source is an indexed projection.
                let tuples = shape.locals[point]
                    .iter()
                    .filter(|local| matches!(local, CallLocal::Tuple { .. }))
                    .collect::<Vec<_>>();
                let source = tuples[tuple.0].clone();
                return Some(CallPoint::Scalar(CallScalar::CompoundField {
                    output,
                    source,
                    index: *index,
                    custom: false,
                    read,
                }));
            }
            ProfiledInstructionKind::Int(IntInstruction::CustomField { source, index })
            | ProfiledInstructionKind::Float(FloatInstruction::CustomField { source, index })
            | ProfiledInstructionKind::Bool(BoolInstruction::CustomField { source, index })
            | ProfiledInstructionKind::String(StringInstruction::CustomField { source, index })
            | ProfiledInstructionKind::BitArray(BitArrayInstruction::CustomField {
                source,
                index,
            })
            | ProfiledInstructionKind::UtfCodepoint(UtfCodepointInstruction::CustomField {
                source,
                index,
            })
            | ProfiledInstructionKind::Nil(NilInstruction::CustomField { source, index })
            | ProfiledInstructionKind::Custom(CustomInstruction::CustomField { source, index })
            | ProfiledInstructionKind::Tuple(TupleInstruction::CustomField { source, index }) => {
                // Every Custom in this complete prefix was classified at its
                // parameter or defining instruction, including Nullary values.
                let customs = shape.locals[point]
                    .iter()
                    .filter(|local| matches!(local, CallLocal::Custom(_) | CallLocal::Nullary(_)))
                    .collect::<Vec<_>>();
                let source = customs[source.id.0].clone();
                // Admitted CustomField has a real field. CallTypes therefore
                // cannot project its source as the fieldless Nullary carrier.
                return Some(CallPoint::Scalar(CallScalar::CompoundField {
                    output,
                    source,
                    index: *index,
                    custom: true,
                    read,
                }));
            }
            _ => {}
        }
    }
    match instruction.kind() {
        ProfiledInstructionKind::Bool(BoolInstruction::ListIndex { list, index }) => {
            let list = shape.locals[point].iter().find_map(|local| match local {
                CallLocal::BoolList { local, type_id } if local == list => {
                    Some(PrimitiveListLocal::Bool {
                        local: *local,
                        type_id: *type_id,
                    })
                }
                _ => None,
            })?;
            return Some(CallPoint::Scalar(CallScalar::Index {
                output,
                list,
                index: *index,
            }));
        }
        ProfiledInstructionKind::Float(FloatInstruction::ListIndex { list, index }) => {
            let list = shape.locals[point].iter().find_map(|local| match local {
                CallLocal::FloatList { local, type_id } if local == list => {
                    Some(PrimitiveListLocal::Float {
                        local: *local,
                        type_id: *type_id,
                    })
                }
                _ => None,
            })?;
            return Some(CallPoint::Scalar(CallScalar::Index {
                output,
                list,
                index: *index,
            }));
        }
        ProfiledInstructionKind::String(StringInstruction::ListIndex { list, index }) => {
            let list = shape.locals[point].iter().find_map(|local| match local {
                CallLocal::StringList { local, type_id } if local == list => {
                    Some(PrimitiveListLocal::String {
                        local: *local,
                        type_id: *type_id,
                    })
                }
                _ => None,
            })?;
            return Some(CallPoint::Scalar(CallScalar::Index {
                output,
                list,
                index: *index,
            }));
        }
        ProfiledInstructionKind::BitArray(BitArrayInstruction::ListIndex { list, index }) => {
            let list = shape.locals[point].iter().find_map(|local| match local {
                CallLocal::BitArrayList { local, type_id } if local == list => {
                    Some(PrimitiveListLocal::BitArray {
                        local: *local,
                        type_id: *type_id,
                    })
                }
                _ => None,
            })?;
            return Some(CallPoint::Scalar(CallScalar::Index {
                output,
                list,
                index: *index,
            }));
        }
        ProfiledInstructionKind::UtfCodepoint(UtfCodepointInstruction::ListIndex {
            list,
            index,
        }) => {
            let list = shape.locals[point].iter().find_map(|local| match local {
                CallLocal::UtfCodepointList { local, type_id } if local == list => {
                    Some(PrimitiveListLocal::UtfCodepoint {
                        local: *local,
                        type_id: *type_id,
                    })
                }
                _ => None,
            })?;
            return Some(CallPoint::Scalar(CallScalar::Index {
                output,
                list,
                index: *index,
            }));
        }
        ProfiledInstructionKind::Nil(NilInstruction::ListIndex { list, index }) => {
            let list = shape.locals[point].iter().find_map(|local| match local {
                CallLocal::NilList { local, type_id } if local == list => {
                    Some(PrimitiveListLocal::Nil {
                        local: *local,
                        type_id: *type_id,
                    })
                }
                _ => None,
            })?;
            return Some(CallPoint::Scalar(CallScalar::Index {
                output,
                list,
                index: *index,
            }));
        }
        _ => {}
    }
    let (target, args, site) = match instruction.kind() {
        ProfiledInstructionKind::Custom(CustomInstruction::Call {
            function,
            args,
            site,
        }) => (
            CallContractTarget::Static(CallTarget::Custom(*function)),
            args,
            site,
        ),
        ProfiledInstructionKind::Tuple(TupleInstruction::Call {
            function,
            args,
            site,
        }) => (
            CallContractTarget::Static(CallTarget::Tuple(*function)),
            args,
            site,
        ),
        ProfiledInstructionKind::Int(IntInstruction::Call {
            function,
            args,
            site,
        }) => (
            CallContractTarget::Static(CallTarget::Int(*function)),
            args,
            site,
        ),
        ProfiledInstructionKind::Int(IntInstruction::FunctionCall {
            function,
            args,
            site,
        }) => (CallContractTarget::IntValue(*function), args, site),
        ProfiledInstructionKind::Bool(BoolInstruction::Call {
            function,
            args,
            site,
        }) => (
            CallContractTarget::Static(CallTarget::Bool(*function)),
            args,
            site,
        ),
        ProfiledInstructionKind::Bool(BoolInstruction::FunctionCall {
            function,
            args,
            site,
        }) => (CallContractTarget::BoolValue(*function), args, site),
        ProfiledInstructionKind::Float(FloatInstruction::Call {
            function,
            args,
            site,
        }) => (
            CallContractTarget::Static(CallTarget::Float(*function)),
            args,
            site,
        ),
        ProfiledInstructionKind::Float(FloatInstruction::FunctionCall {
            function,
            args,
            site,
        }) => (CallContractTarget::FloatValue(*function), args, site),
        ProfiledInstructionKind::String(StringInstruction::Call {
            function,
            args,
            site,
        }) => (
            CallContractTarget::Static(CallTarget::String(*function)),
            args,
            site,
        ),
        ProfiledInstructionKind::String(StringInstruction::FunctionCall {
            function,
            args,
            site,
        }) => (CallContractTarget::StringValue(*function), args, site),
        ProfiledInstructionKind::BitArray(BitArrayInstruction::Call {
            function,
            args,
            site,
        }) => (
            CallContractTarget::Static(CallTarget::BitArray(*function)),
            args,
            site,
        ),
        ProfiledInstructionKind::BitArray(BitArrayInstruction::FunctionCall {
            function,
            args,
            site,
        }) => (CallContractTarget::BitArrayValue(*function), args, site),
        ProfiledInstructionKind::UtfCodepoint(UtfCodepointInstruction::Call {
            function,
            args,
            site,
        }) => (
            CallContractTarget::Static(CallTarget::UtfCodepoint(*function)),
            args,
            site,
        ),
        ProfiledInstructionKind::UtfCodepoint(UtfCodepointInstruction::FunctionCall {
            function,
            args,
            site,
        }) => (CallContractTarget::UtfCodepointValue(*function), args, site),
        ProfiledInstructionKind::Nil(NilInstruction::Call {
            function,
            args,
            site,
        }) => (
            CallContractTarget::Static(CallTarget::Nil(*function)),
            args,
            site,
        ),
        ProfiledInstructionKind::Nil(NilInstruction::FunctionCall {
            function,
            args,
            site,
        }) => (CallContractTarget::NilValue(*function), args, site),
        ProfiledInstructionKind::Function(function) => match function.kind() {
            FunctionInstructionKind::Call {
                function: ProfiledFunctionFunctionId::Int(function),
                args,
                site,
            } => (
                CallContractTarget::Static(CallTarget::IntFunction(*function)),
                args,
                site,
            ),
            FunctionInstructionKind::Call {
                function: ProfiledFunctionFunctionId::Bool(function),
                args,
                site,
            } => (
                CallContractTarget::Static(CallTarget::BoolFunction(*function)),
                args,
                site,
            ),
            FunctionInstructionKind::Call {
                function: ProfiledFunctionFunctionId::Float(function),
                args,
                site,
            } => (
                CallContractTarget::Static(CallTarget::FloatFunction(*function)),
                args,
                site,
            ),
            FunctionInstructionKind::Call {
                function: ProfiledFunctionFunctionId::String(function),
                args,
                site,
            } => (
                CallContractTarget::Static(CallTarget::StringFunction(*function)),
                args,
                site,
            ),
            FunctionInstructionKind::Call {
                function: ProfiledFunctionFunctionId::BitArray(function),
                args,
                site,
            } => (
                CallContractTarget::Static(CallTarget::BitArrayFunction(*function)),
                args,
                site,
            ),
            FunctionInstructionKind::Call {
                function: ProfiledFunctionFunctionId::UtfCodepoint(function),
                args,
                site,
            } => (
                CallContractTarget::Static(CallTarget::UtfCodepointFunction(*function)),
                args,
                site,
            ),
            FunctionInstructionKind::Call {
                function: ProfiledFunctionFunctionId::Nil(function),
                args,
                site,
            } => (
                CallContractTarget::Static(CallTarget::NilFunction(*function)),
                args,
                site,
            ),
            kind => {
                let target = match kind {
                    FunctionInstructionKind::Reference(FunctionTarget::Int(id))
                    | FunctionInstructionKind::Closure {
                        target: FunctionTarget::Int(id),
                        ..
                    } => CallableTarget::Int(*id),
                    FunctionInstructionKind::Reference(FunctionTarget::Bool(id))
                    | FunctionInstructionKind::Closure {
                        target: FunctionTarget::Bool(id),
                        ..
                    } => CallableTarget::Bool(*id),
                    FunctionInstructionKind::Reference(FunctionTarget::Float(id))
                    | FunctionInstructionKind::Closure {
                        target: FunctionTarget::Float(id),
                        ..
                    } => CallableTarget::Float(*id),
                    FunctionInstructionKind::Reference(FunctionTarget::String(id))
                    | FunctionInstructionKind::Closure {
                        target: FunctionTarget::String(id),
                        ..
                    } => CallableTarget::String(*id),
                    FunctionInstructionKind::Reference(FunctionTarget::BitArray(id))
                    | FunctionInstructionKind::Closure {
                        target: FunctionTarget::BitArray(id),
                        ..
                    } => CallableTarget::BitArray(*id),
                    FunctionInstructionKind::Reference(FunctionTarget::UtfCodepoint(id))
                    | FunctionInstructionKind::Closure {
                        target: FunctionTarget::UtfCodepoint(id),
                        ..
                    } => CallableTarget::UtfCodepoint(*id),
                    FunctionInstructionKind::Reference(FunctionTarget::Nil(id))
                    | FunctionInstructionKind::Closure {
                        target: FunctionTarget::Nil(id),
                        ..
                    } => CallableTarget::Nil(*id),
                    _ => return None,
                };
                let captures = match function.kind() {
                    FunctionInstructionKind::Closure { captures, .. } => captures
                        .iter()
                        .map(Capture::inspect)
                        .collect::<Option<Vec<_>>>()?,
                    _ => Vec::new(),
                };
                let index = shape.creations.len();
                shape.creations.push(CallCreation {
                    point,
                    output: output.clone(),
                    target,
                    type_: function.type_().clone(),
                    reference: matches!(function.kind(), FunctionInstructionKind::Reference(_)),
                    captures,
                });
                shape.root = true;
                return Some(CallPoint::Create(index));
            }
        },
        _ => return None,
    };
    // All inputs belong to the already classified parameters or preceding
    // outputs of this block, just as for a tail call above.
    let args = args
        .iter()
        .filter_map(|argument| types.local(argument))
        .collect();
    let index = shape.calls.len();
    shape.calls.push(CallInvocation {
        point,
        output: output.clone(),
        target,
        args,
        site: site.clone(),
    });
    shape.root = true;
    Some(CallPoint::Call(index))
}

#[cfg(test)]
mod tests {
    use super::super::local::local_name;
    use super::super::{Code, test_expression, write_scalar};
    use super::KernelKind;
    use super::{
        CallLocal, CallPoint, CallProgram, CallScalar, CallTarget, CallTest, CallableTarget,
        Capture, CompiledBoolean, CompiledInstruction, CompiledTest, inspect_instruction,
    };
    use crate::StatelessHostProfile;
    use crate::plan::execution::HostedProgram;
    use crate::plan::execution::compiled::{CallContractTarget, CompiledCheckpoint};
    use crate::plan::execution::function::{
        BoolFunctionId, ExecutionFunctionEntry, ExecutionFunctionRef, ExecutionIntFunctionBody,
        FunctionReturnFamily, IntFunctionFunctionId, IntFunctionId, TupleFunctionId,
    };
    use crate::plan::execution::graph::{
        BlockId, BoolFunctionLocalId, BoolInstruction, BoolLocalId, BoolTest, CustomInstruction,
        CustomListLocalId, CustomLocalId, FunctionCapture, FunctionInstructionKind,
        IntFunctionLocalId, IntListLocalId, IntLocalId, ListInstruction, ListLocal, ParamLocal,
        ParamSlot, ProfiledInstruction, ProfiledInstructionKind, StringLocalId, TupleLocalId,
        TypedListInstruction,
    };
    use crate::plan::execution::host::HostedExecutionProfile;
    use crate::plan::execution::prepared::codegen::custom::CustomField;
    use crate::plan::execution::prepared::codegen::custom_loop::CustomLoopInstruction;
    use crate::plan::execution::prepared::codegen::int_list::IntListTest;
    use crate::plan::execution::prepared::codegen::string::{StringOperation, StringTest};
    use crate::plan::execution::type_::{
        CustomListTypeId, CustomTypeId, FunctionType, IntListTypeId, ListTypeId, ValueShapeId,
        ValueType,
    };
    use num_bigint::BigInt;
    use std::convert::Infallible;

    #[test]
    fn ordinary_compound_calls_keep_their_existing_graph_callees() {
        let source = r#"
pub type Box { Box(value: Int) }
fn custom_identity(value: Box) { value }
fn tuple_identity(value: #(Int, Int)) { value }
fn make_pair(value: Int) { #(value, value) }
fn custom_number(value: Box) {
  let returned = custom_identity(value)
  returned.value
}
fn tuple_number(value: #(Int, Int)) {
  let returned = tuple_identity(value)
  returned.0 + returned.1
}
pub fn main() { custom_number(Box(40)) + tuple_number(make_pair(1)) }
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            crate::HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let calls = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let compound_calls = calls
            .functions
            .iter()
            .flat_map(|function| &function.shape.calls)
            .map(|call| (call.site.function(), &call.target))
            .collect::<Vec<_>>();
        let custom_callee = calls
            .functions
            .iter()
            .find(|function| function.target.family() == FunctionReturnFamily::Custom)
            .unwrap()
            .target;
        assert_eq!(custom_callee.index(), 0);
        assert_eq!(
            calls
                .functions
                .iter()
                .filter(|function| function.target.family() == FunctionReturnFamily::Tuple)
                .map(|function| function.target)
                .collect::<Vec<_>>(),
            [CallTarget::Tuple(TupleFunctionId(1))]
        );
        assert!(
            compound_calls
                == [
                    ("custom_number", &CallContractTarget::Static(custom_callee)),
                    (
                        "tuple_number",
                        &CallContractTarget::Static(CallTarget::Tuple(TupleFunctionId(1)))
                    ),
                ]
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::Int(42.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn native_tuple_connections_accept_primitive_inputs_and_keep_tuple_inputs_canonical() {
        use crate::{
            HostCall, HostCallCompletion, HostCallError, HostProvider, HostProviderModule,
            HostProviderSet, HostTuple, HostTupleType, HostTypeList, HostTypeListEnd,
        };
        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        type Elements = HostTypeList<BigInt, HostTypeList<BigInt, HostTypeListEnd>>;
        type Pair = HostTupleType<Elements>;
        fn primitive<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, Pair>,
            value: BigInt,
        ) -> Result<HostCallCompletion<'call, Pair>, HostCallError> {
            assert_eq!(call.state(), &());
            Ok(call.return_tuple((value.clone(), (value, ()))))
        }
        fn compound<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, Pair>,
            value: HostTuple<'call, Elements>,
        ) -> Result<HostCallCompletion<'call, Pair>, HostCallError> {
            assert_eq!(call.state(), &());
            Ok(call.return_value(value))
        }
        let source = r#"
@external(erlang, "example", "primitive")
fn primitive(value: Int) -> #(Int, Int)
@external(erlang, "example", "compound")
fn compound(value: #(Int, Int)) -> #(Int, Int)
fn simple(value: Int) { let pair = primitive(value) pair.0 + pair.1 }
fn boxed(value: #(Int, Int)) { let pair = compound(value) pair.0 + pair.1 }
pub fn main() { simple(20) + boxed(#(1, 1)) }
"#;
        let provider = HostProviderModule::<StatelessHostProfile>::new("example", "example")
            .unwrap()
            .with_scoped_function::<Provider, (BigInt,), Pair, _>("primitive", primitive)
            .unwrap()
            .with_scoped_function::<Provider, (Pair,), Pair, _>("compound", compound)
            .unwrap();
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            HostProviderSet::from_providers([provider]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let calls = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        assert_eq!(
            calls
                .functions
                .iter()
                .map(|function| function.target)
                .collect::<Vec<_>>(),
            [
                CallTarget::Int(IntFunctionId(0)),
                CallTarget::Int(IntFunctionId(1)),
            ]
        );
        assert!(
            calls
                .functions
                .iter()
                .flat_map(|function| &function.shape.calls)
                .map(|call| (call.site.function(), &call.target))
                .collect::<Vec<_>>()
                == [
                    (
                        "main",
                        &CallContractTarget::Static(CallTarget::Int(IntFunctionId(1))),
                    ),
                    (
                        "simple",
                        &CallContractTarget::Static(CallTarget::Tuple(TupleFunctionId(0))),
                    ),
                ]
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::Int(42.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn source_compound_fields_preserve_the_selected_owner_and_field_position() {
        let source = r#"
pub type Inner { Inner(Int) }
pub type Record {
  Record(number: Int, float: Float, flag: Bool, text: String, bits: BitArray,
    inner: Inner, nested: #(Int, Int), point: UtfCodepoint, nothing: Nil)
}
fn identity(value: Int) { value }
fn keep_float(value: Float) { value }
fn keep_bool(value: Bool) { value }
fn keep_string(value: String) { value }
fn keep_bits(value: BitArray) { value }
fn keep_inner(value: Inner) { value }
fn keep_pair(value: #(Int, Int)) { value }
fn keep_point(value: UtfCodepoint) { value }
fn keep_nil(value: Nil) { value }
fn read(record: Record, pair: #(Int, Float, Bool, String, BitArray, Inner, #(Int, Int), UtfCodepoint, Nil)) {
  let _ = keep_float(record.float)
  let _ = keep_float(pair.1)
  let _ = keep_bool(record.flag)
  let _ = keep_bool(pair.2)
  let _ = keep_string(record.text)
  let _ = keep_string(pair.3)
  let _ = keep_bits(record.bits)
  let _ = keep_bits(pair.4)
  let _ = keep_inner(record.inner)
  let _ = keep_inner(pair.5)
  let _ = keep_pair(record.nested)
  let _ = keep_pair(pair.6)
  let _ = keep_point(record.point)
  let _ = keep_point(pair.7)
  let _ = keep_nil(record.nothing)
  let _ = keep_nil(pair.8)
  identity(record.number + pair.0) + 1
}
pub fn main() {
  let assert <<point:utf8_codepoint>> = <<"a">>
  read(Record(40, 1.5, True, "kept", <<7:8>>, Inner(7), #(7, 8), point, Nil),
    #(2, 2.5, False, "other", <<8:8>>, Inner(8), #(8, 9), point, Nil))
}
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            crate::HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let types = super::CallTypes {
            custom_types: &plan.program.common.custom_types,
            value_shapes: &plan.program.common.value_shapes,
        };
        let calls = CallProgram::inspect(
            &plan.program.functions,
            types.custom_types,
            types.value_shapes,
        );
        let read = calls
            .functions
            .iter()
            .find(|function| function.shape.parameter_count == 2)
            .unwrap();
        let fields = read
            .shape
            .points
            .iter()
            .enumerate()
            .filter_map(|(point, operation)| match operation {
                CallPoint::Scalar(CallScalar::CompoundField {
                    source,
                    index,
                    custom,
                    ..
                }) => Some((point, source.canonical(), *index, *custom)),
                _ => None,
            })
            .collect::<Vec<_>>();
        assert_eq!(
            fields
                .iter()
                .map(|(_, _, index, custom)| (*index, *custom))
                .collect::<Vec<_>>(),
            [
                (1, true),
                (1, false),
                (2, true),
                (2, false),
                (3, true),
                (3, false),
                (4, true),
                (4, false),
                (5, true),
                (5, false),
                (6, true),
                (6, false),
                (7, true),
                (7, false),
                (8, true),
                (8, false),
                (0, true),
                (0, false),
            ]
        );
        let inputs = &read.shape.locals[read.shape.entry()];
        assert_eq!(inputs.len(), 2);
        for (_, source, _, custom) in fields {
            assert_eq!(source, inputs[usize::from(!custom)].canonical());
        }
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::Int(43.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn a_refined_constructor_keeps_its_sparse_join_transfer_canonical() {
        let source = r#"
pub type Choice { First Second Unused }
fn identity(value: Int) { value }
fn choose(value: Result(Choice, Nil), flag: Bool) {
  let before = identity(40)
  case value {
    Ok(First as narrow) -> {
      let selected = case flag { True -> narrow False -> Second }
      before + case selected { First -> 1 Second -> 2 _ -> 0 }
    }
    Error(Nil) -> 0
    _ -> -1
  }
}
pub fn main() { choose(Ok(First), True) + choose(Ok(First), False) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let calls = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let choose = calls
            .functions
            .iter()
            .find(|function| function.shape.parameter_count == 2)
            .unwrap();
        let body = match plan.program.functions.value_returns.int_functions[choose.target.index()]
            .as_ref()
        {
            ExecutionFunctionRef::Graph(function) => function.body(),
            ExecutionFunctionRef::Host(never) => match *never {},
        };
        let graph = body.block_graph().as_view();
        // The True arm carries Exact(First), but its join needs Any(Choice).
        // Unused is absent from the sparse catalog, so the join and its
        // incoming ordinary transfer belong to the original graph.
        assert_eq!(choose.shape.starts.get(&3), None);
        let canonical_transfers = choose
            .shape
            .points
            .iter()
            .enumerate()
            .filter(|(_, operation)| matches!(operation, CallPoint::Interpreted))
            .filter_map(|(point, _)| {
                let checkpoint = &choose.shape.checkpoints[point];
                let block = graph.block(checkpoint.block);
                (checkpoint.instruction == block.instructions().len())
                    .then(|| super::CallTerminator::inspect(block.terminator()))
                    .flatten()
                    .map(|_| (checkpoint.block, checkpoint.instruction))
            })
            .collect::<Vec<_>>();
        assert_eq!(canonical_transfers, [(BlockId(2), 0)]);
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(83.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn a_sparse_compound_destination_keeps_the_incoming_transfer_canonical() {
        let source = r#"
pub type Choice(a) { Empty Filled(a) }
fn fallback(value: Choice(Int)) { echo value 0 }
fn choose(value: Choice(Int)) { case value { Empty -> 42 rest -> fallback(rest) } }
pub fn main() { choose(Empty) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        // The source argument retains its exact Empty refinement; the fallback
        // destination needs the incomplete Any constructor set. Its incoming
        // match therefore stays canonical, including at the entry checkpoint.
        let calls = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        assert!(calls.functions.is_empty());
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(42.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn captured_nullary_values_keep_their_original_callable_owner() {
        let source = r#"
pub type Tag { First Second }
fn make(tag: Tag) { fn() { case tag { First -> 1 Second -> 2 } } }
pub fn main() { let first = make(First) let second = make(Second) first() + second() }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let calls = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        assert_eq!(
            calls
                .functions
                .iter()
                .map(|function| function.target)
                .collect::<Vec<_>>(),
            [CallTarget::Int(IntFunctionId(0))]
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(3.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn a_nullary_capture_stays_canonical_even_when_its_first_call_is_supported() {
        let source = r#"
pub type Tag { First Second }
fn consume(tag: Tag) { let _ = tag 7 }
fn make(tag: Tag) { fn() { consume(tag) } }
pub fn main() { let first = make(First) let second = make(Second) first() + second() }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let calls = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        assert_eq!(
            calls
                .functions
                .iter()
                .map(|function| function.target)
                .collect::<Vec<_>>(),
            [
                CallTarget::Int(IntFunctionId(0)),
                CallTarget::Int(IntFunctionId(2))
            ]
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(14.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn primitive_function_factories_leave_compound_parameters_and_captures_canonical() {
        for family in [
            "Int",
            "Float",
            "Bool",
            "Nil",
            "UtfCodepoint",
            "String",
            "BitArray",
        ] {
            let source = format!(
                r#"
fn make(pair: #(List({family}), List({family}))) -> fn() -> {family} {{
  fn() {{
    case pair.0 {{
      [value, ..] -> value
      [] -> panic as "no value"
    }}
  }}
}}
pub fn main() {{
  let calculate = make(#([], []))
  calculate()
}}
"#
            );
            let typed = crate::compile_typed_host_program(
                "example",
                "example",
                [crate::PackageSource::new(
                    "example",
                    Vec::<String>::new(),
                    [crate::ModuleSource::new(
                        "example",
                        "src/example.gleam",
                        source.clone(),
                    )],
                )],
                crate::HostProviderSet::<crate::StatelessHostProfile>::new([]).unwrap(),
            )
            .unwrap();
            let mut hosted = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let (plan, _, _) = hosted.parts_mut();
            let plan = &**plan;
            assert!(
                CallProgram::inspect(
                    &plan.program.functions,
                    &plan.program.common.custom_types,
                    &plan.program.common.value_shapes
                )
                .functions
                .is_empty(),
                "{family}"
            );
            let mut echo = Vec::new();
            let panic = source_panic(
                crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap_err(),
            );
            assert_eq!(panic.kind(), crate::PanicKind::Panic);
            assert_eq!(
                panic.message(),
                &crate::PanicMessage::Explicit("no value".into())
            );
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn nonempty_primitive_lists_preserve_elements_and_spread_family() {
        for (family, value) in [
            ("Float", "1.5"),
            ("Bool", "True"),
            ("Nil", "Nil"),
            ("String", "\"kept\""),
            ("BitArray", "<<7:8>>"),
            (
                "UtfCodepoint",
                "case <<97:8>> { <<point:utf8_codepoint>> -> point _ -> panic }",
            ),
        ] {
            let source = format!(
                r#"
fn consume(values: List({family})) -> Nil {{ let _ = values Nil }}
fn build(value: {family}, tail: List({family})) -> Nil {{
  consume([value])
  consume([value, ..tail])
  case tail {{ [] -> Nil [_, ..rest] -> consume(rest) }}
}}
pub fn main() {{ let value = {value} build(value, [value]) }}
"#
            );
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", &source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            let function = plan
                .program
                .functions
                .value_returns
                .nil_functions
                .iter()
                .find(|entry| entry.entry.parameter_count == 2)
                .unwrap();
            let function = match function.as_ref() {
                ExecutionFunctionRef::Graph(function) => function,
                ExecutionFunctionRef::Host(never) => match *never {},
            };
            let types = super::CallTypes {
                custom_types: &plan.program.common.custom_types,
                value_shapes: &plan.program.common.value_shapes,
            };
            // This owner classifies the valid graph before program-wide
            // pruning; the call-free List consumer need not be selected.
            let shape = super::CallShape::inspect(
                &types,
                function.body(),
                2,
                false,
                |value, local| matches!(local, CallLocal::Nil(id) if id == value),
                |target| CallTarget::Nil(*target.function()),
                |target| target.site().clone(),
            )
            .unwrap();
            let operations = shape
                .points
                .iter()
                .filter_map(|point| {
                    if let CallPoint::Scalar(CallScalar::List { output, operation }) = point {
                        Some((output, operation))
                    } else {
                        None
                    }
                })
                .collect::<Vec<_>>();
            assert_eq!(operations.len(), 3, "{family}");
            let mut value_count = 0;
            let mut spread_count = 0;
            let mut tail_count = 0;
            for (output, operation) in operations {
                match operation {
                    super::PrimitiveListOperation::Value(elements) => {
                        assert_eq!(elements.len(), 1);
                        value_count += 1;
                    }
                    super::PrimitiveListOperation::Spread { elements, tail } => {
                        assert_eq!(elements.len(), 1);
                        assert!(output.canonical().same_type(tail));
                        spread_count += 1;
                    }
                    super::PrimitiveListOperation::Tail { list, count } => {
                        assert!(output.canonical().same_type(list));
                        assert_eq!(*count, 1);
                        tail_count += 1;
                    }
                }
            }
            assert_eq!((value_count, spread_count, tail_count), (1, 1, 1));
            let mut echo = Vec::new();
            assert_eq!(
                crate::run_main(&plan, &mut echo).unwrap(),
                crate::Value::Nil
            );
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn primitive_index_projection_requires_the_candidate_to_own_its_list_input() {
        use crate::{HostProviderModule, HostProviderSet, StatelessHostProfile};
        for (family, value) in [
            ("Float", "1.5"),
            ("Bool", "True"),
            ("Nil", "Nil"),
            ("String", "\"kept\""),
            ("BitArray", "<<7:8>>"),
            (
                "UtfCodepoint",
                "case <<97:8>> { <<point:utf8_codepoint>> -> point _ -> panic }",
            ),
        ] {
            let source = format!(
                r#"
@external(erlang, "example", "native")
fn native() -> Int
fn consume(value: {family}) -> Int {{ let _ = value 42 }}
fn head(values: List({family}), needle: {family}, calculate: fn({family}) -> Int) -> Int {{
  case values {{
    [value, ..] if value == needle -> calculate(value)
    _ -> native()
  }}
}}
pub fn main() {{ let value = {value} head([value], value, consume) + native() }}
"#
            );
            let typed = crate::compile_typed_host_program(
                "example",
                "example",
                [crate::PackageSource::new(
                    "example",
                    Vec::<String>::new(),
                    [crate::ModuleSource::new(
                        "example",
                        "src/example.gleam",
                        source,
                    )],
                )],
                HostProviderSet::from_providers([HostProviderModule::<StatelessHostProfile>::new(
                    "example", "example",
                )
                .unwrap()
                .with_function::<(), BigInt, _>("native", || BigInt::from(0))
                .unwrap()])
                .unwrap(),
            )
            .unwrap();
            let mut hosted = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let (plan, _, _) = hosted.parts_mut();
            let plan = &**plan;
            let mut heads = plan
                .program
                .functions
                .value_returns
                .int_functions
                .iter()
                .filter_map(|entry| match entry.as_ref() {
                    ExecutionFunctionRef::Graph(function) => function
                        .entry()
                        .params(function.body())
                        .iter()
                        .any(|slot| matches!(slot.local(), ParamLocal::List(_)))
                        .then_some(function),
                    ExecutionFunctionRef::Host(_) => None,
                })
                .collect::<Vec<_>>();
            assert_eq!(heads.len(), 1);
            let head = heads.pop().unwrap();
            let graph = head.body().block_graph().as_view();
            let types = super::CallTypes {
                custom_types: &plan.program.common.custom_types,
                value_shapes: &plan.program.common.value_shapes,
            };
            let mut candidate = super::CallShape::inspect(
                &types,
                head.body(),
                3,
                false,
                |value, local| matches!(local, CallLocal::Int(id) if id == value),
                |target| CallTarget::Int(*target.function()),
                |target| target.site().clone(),
            )
            .unwrap();
            let indexed = graph.blocks().enumerate().flat_map(|(block_index, block)| block.instructions().iter().enumerate().map(move |(index, instruction)| (BlockId(block_index), index, instruction)))
                .find(|(_, _, instruction)| matches!(instruction.value().map(|value| value.kind()),
                    Some(ProfiledInstructionKind::Bool(BoolInstruction::ListIndex { .. }) | ProfiledInstructionKind::Float(crate::plan::execution::graph::FloatInstruction::ListIndex { .. })
                        | ProfiledInstructionKind::String(crate::plan::execution::graph::StringInstruction::ListIndex { .. })
                        | ProfiledInstructionKind::BitArray(crate::plan::execution::graph::BitArrayInstruction::ListIndex { .. })
                        | ProfiledInstructionKind::UtfCodepoint(crate::plan::execution::graph::UtfCodepointInstruction::ListIndex { .. })
                        | ProfiledInstructionKind::Nil(crate::plan::execution::graph::NilInstruction::ListIndex { .. })))).unwrap();
            let point = candidate
                .checkpoints
                .iter()
                .position(|point| point.block == indexed.0 && point.instruction == indexed.1)
                .unwrap();
            assert!(
                inspect_instruction(indexed.2, point, &mut candidate, &types).is_some(),
                "{family}"
            );
            // A candidate projection with no scoped list must decline. Only
            // this open classification context changes; the accepted plan stays intact.
            candidate.locals[point].retain(|local| {
                !matches!(
                    local,
                    CallLocal::BoolList { .. }
                        | CallLocal::FloatList { .. }
                        | CallLocal::StringList { .. }
                        | CallLocal::BitArrayList { .. }
                        | CallLocal::UtfCodepointList { .. }
                        | CallLocal::NilList { .. }
                )
            });
            assert!(
                inspect_instruction(indexed.2, point, &mut candidate, &types).is_none(),
                "{family}"
            );
            let mut echo = Vec::new();
            assert_eq!(
                crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
                crate::Value::Int(42.into())
            );
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn primitive_list_tuple_projections_stay_with_the_compound_owner() {
        for family in ["Float", "Bool", "Nil", "UtfCodepoint", "String", "BitArray"] {
            let source = format!(
                r#"
fn project(pair: #(List({family}))) -> Bool {{
  case pair.0 {{ [] -> False _ -> True }}
}}
pub fn main() {{ project(#([])) }}
"#
            );
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", &source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            let function = match plan.program.functions.value_returns.bool_functions[1].as_ref() {
                ExecutionFunctionRef::Graph(function) => function,
                ExecutionFunctionRef::Host(never) => match *never {},
            };
            assert_eq!(function.entry().parameter_count, 1);
            let graph = function.body().block_graph().as_view();
            let projection = graph
                .blocks()
                .flat_map(|block| block.instructions())
                .find(|instruction| {
                    instruction
                        .outputs()
                        .any(|slot| matches!(slot.local(), ParamLocal::List(_)))
                })
                .unwrap();
            let projected_family =
                projected_list_family(projection.value().map(|value| value.kind()));
            assert_eq!(projected_family, family);
            assert!(CallScalar::inspect(projection).is_none());
            assert!(
                CallProgram::inspect(
                    &plan.program.functions,
                    &plan.program.common.custom_types,
                    &plan.program.common.value_shapes
                )
                .functions
                .is_empty()
            );
            let mut echo = Vec::new();
            assert_eq!(
                crate::run_main(&plan, &mut echo).unwrap(),
                crate::Value::Bool(false)
            );
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn hosted_primitive_calls_keep_static_dynamic_and_captured_targets_in_their_families() {
        use crate::{HostProviderSet, StatelessHostProfile};
        for (family, value, selected_functions) in [
            ("Int", "7", 5),
            ("Bool", "True", 5),
            ("Float", "1.5", 5),
            ("String", "\"kept\"", 5),
            ("BitArray", "<<5:3>>", 4),
            ("Nil", "Nil", 5),
            (
                "UtfCodepoint",
                "case <<97:8>> { <<point:utf8_codepoint>> -> point _ -> panic }",
                5,
            ),
        ] {
            let source = format!(
                r#"
fn identity(value: {family}) -> {family} {{ value }}
fn apply(value: {family}, callback: fn({family}) -> {family}) -> {family} {{
  let first = identity(value)
  callback(first)
}}
fn factory(value: {family}) -> fn({family}) -> {family} {{ fn(_) {{ identity(value) }} }}
pub fn main() {{
  let value = {value}
  let callback = identity
  let first = identity(value)
  let calculate = factory(first)
  let second = apply(first, callback)
  let _ = calculate(second)
  42
}}
"#
            );
            let typed = crate::compile_typed_host_program(
                "example",
                "example",
                [crate::PackageSource::new(
                    "example",
                    Vec::<String>::new(),
                    [crate::ModuleSource::new(
                        "example",
                        "src/example.gleam",
                        source,
                    )],
                )],
                HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
            )
            .unwrap();
            let mut hosted = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let (plan, _, _) = hosted.parts_mut();
            let calls = CallProgram::inspect(
                &plan.program.functions,
                &plan.program.common.custom_types,
                &plan.program.common.value_shapes,
            );
            // The bit construction keeps this main entry canonical;
            // all four typed callable bodies remain selected in every case.
            assert_eq!(calls.functions.len(), selected_functions, "{family}");
            let mut echo = Vec::new();
            assert_eq!(
                crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
                crate::Value::Int(42.into())
            );
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn native_string_wrappers_with_list_arguments_keep_their_canonical_owner() {
        use crate::{
            HostCall, HostCallCompletion, HostCallError, HostList, HostListType, HostProvider,
            HostProviderModule, HostProviderSet, StatelessHostProfile, StringValue,
        };
        struct Provider;
        impl HostProvider<StatelessHostProfile> for Provider {
            type State = ();
            fn project(state: &mut ()) -> &mut () {
                state
            }
        }
        fn native<'call>(
            mut call: HostCall<'call, StatelessHostProfile, Provider, StringValue>,
            values: HostList<'call, StringValue>,
        ) -> Result<HostCallCompletion<'call, StringValue>, HostCallError> {
            let _ = call.state();
            assert_eq!(call.list_len(values), 1);
            Ok(call.return_value("kept".into()))
        }
        let source = r#"
@external(erlang, "example", "native")
fn native(values: List(String)) -> String
fn ordinary(values: List(String)) -> String { let result = native(values) result <> "" }
fn tail(values: List(String)) -> String { let _ = native(values) native(values) }
fn connected(value: Int) { let keep = fn(value) { value } keep(value) }
pub fn main() { #(connected(7), ordinary(["input"]), tail(["input"])) }
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            HostProviderSet::from_providers([HostProviderModule::<StatelessHostProfile>::new(
                "example", "example",
            )
            .unwrap()
            .with_scoped_function::<Provider, (HostListType<StringValue>,), StringValue, _>(
                "native", native,
            )
            .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let calls = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        assert!(!calls.functions.is_empty());
        assert!(
            calls
                .functions
                .iter()
                .all(|function| !matches!(function.target, CallTarget::String(_)))
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::Tuple(vec![
                crate::Value::Int(7.into()),
                crate::Value::String("kept".into()),
                crate::Value::String("kept".into())
            ])
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn string_wrappers_decline_calls_to_a_canonical_compound_callee() {
        use crate::HostProviderSet;
        let source = r#"
fn canonical(value: String) -> String {
  echo value
  let pair = #(value)
  case pair { #(kept) -> kept }
}
fn ordinary(value: String) -> String { let result = canonical(value) result <> "" }
fn tail(value: String) -> String { canonical(value) }
fn connected(value: Int) { let keep = fn(value) { value } keep(value) }
pub fn main() { #(connected(7), ordinary("first"), tail("second")) }
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let calls = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        assert!(!calls.functions.is_empty());
        assert!(
            calls
                .functions
                .iter()
                .all(|function| !matches!(function.target, CallTarget::String(_)))
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::Tuple(vec![
                crate::Value::Int(7.into()),
                crate::Value::String("first".into()),
                crate::Value::String("second".into()),
            ])
        );
        assert_eq!(
            echo.iter().map(|output| output.value()).collect::<Vec<_>>(),
            [
                &crate::Value::String("first".into()),
                &crate::Value::String("second".into()),
            ]
        );
    }

    #[test]
    fn a_string_predicate_in_a_callee_keeps_its_complete_string_kernel() {
        use crate::{HostProviderSet, StatelessHostProfile};
        let source = r#"
fn inspect(value: String) -> Int { let _ = value == "kept" 42 }
pub fn main() { let value = "kept" let calculate = fn() { inspect(value) } calculate() }
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let calls = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let kernels = calls
            .functions
            .iter()
            .filter_map(|function| function.kernel.as_ref())
            .collect::<Vec<_>>();
        assert_eq!(kernels.len(), 1);
        assert_eq!(kernels[0].kind, KernelKind::String);
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::Int(42.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn scalar_comparison_projection_rejects_compound_mismatched_and_callable_locals() {
        let custom_list = ParamLocal::List(ListLocal::Custom {
            local: CustomListLocalId(7),
            type_id: CustomListTypeId {
                list_type: ListTypeId(11),
                item_type: CustomTypeId(4),
            },
        });
        let compound = ParamLocal::Tuple {
            local: TupleLocalId(0),
            type_: vec![ValueType::Int].into(),
        };
        let function = ParamLocal::IntFunction {
            local: IntFunctionLocalId(0),
            type_: FunctionType::new(Vec::new(), ValueType::Int),
        };
        for (left, right) in [
            (custom_list.clone(), custom_list.clone()),
            (ParamLocal::Int(IntLocalId(0)), custom_list),
            (compound.clone(), ParamLocal::Int(IntLocalId(0))),
            (ParamLocal::Int(IntLocalId(0)), compound),
            (
                ParamLocal::String(StringLocalId(0)),
                ParamLocal::Bool(BoolLocalId(0)),
            ),
            (function.clone(), function),
        ] {
            assert!(
                CallTest::inspect(&BoolTest::Equal {
                    left: left.clone(),
                    right: right.clone()
                })
                .is_none()
            );
            assert!(CallTest::inspect(&BoolTest::NotEqual { left, right }).is_none());
        }
    }

    #[test]
    fn list_index_projection_keeps_the_selected_parameter_and_requires_its_live_local() {
        for family in ["Float", "Bool", "Nil", "UtfCodepoint", "String", "BitArray"] {
            let source = format!(
                r#"
fn touch(value: Nil) -> Nil {{ value }}
fn read_two(first: List({family}), second: List({family})) -> Nil {{
  touch(Nil)
  case second {{
    [] -> Nil
    [value, ..] -> {{
      echo value
      case first {{
        [] -> Nil
        [other, ..] -> {{ echo other Nil }}
      }}
    }}
  }}
}}
pub fn main() {{ read_two([], []) }}
"#
            );
            let typed =
                crate::compile_typed_module("example", "src/example.gleam", &source).unwrap();
            let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
            let mut program = CallProgram::inspect(
                &plan.program.functions,
                &plan.program.common.custom_types,
                &plan.program.common.value_shapes,
            );
            let function = program
                .functions
                .iter_mut()
                .find(|function| function.shape.parameter_count == 2)
                .unwrap();
            let body = match plan.program.functions.value_returns.nil_functions
                [function.target.index()]
            .as_ref()
            {
                ExecutionFunctionRef::Graph(function) => function.body(),
                ExecutionFunctionRef::Host(never) => match *never {},
            };
            let (point, expected) = function
                .shape
                .points
                .iter()
                .enumerate()
                .find_map(|(point, operation)| match operation {
                    CallPoint::Scalar(CallScalar::Index { list, .. })
                        if local_name(&list.canonical()).ends_with('1') =>
                    {
                        Some((point, list.canonical()))
                    }
                    _ => None,
                })
                .unwrap();
            let checkpoint = function.shape.checkpoints[point];
            let instruction = &body
                .block_graph()
                .as_view()
                .block(checkpoint.block)
                .instructions()[checkpoint.instruction];
            let projected = inspect_instruction(
                instruction,
                point,
                &mut function.shape,
                &super::CallTypes {
                    custom_types: &plan.program.common.custom_types,
                    value_shapes: &plan.program.common.value_shapes,
                },
            )
            .unwrap();
            let list = indexed_list(projected);
            assert_eq!(list.canonical(), expected, "{family}");
            // The owner must decline an index whose source is unavailable at
            // this checkpoint instead of substituting another list parameter.
            function.shape.locals[point].clear();
            assert!(
                inspect_instruction(
                    instruction,
                    point,
                    &mut function.shape,
                    &super::CallTypes {
                        custom_types: &plan.program.common.custom_types,
                        value_shapes: &plan.program.common.value_shapes
                    }
                )
                .is_none()
            );
            let mut echo = Vec::new();
            assert_eq!(
                crate::run_main(&plan, &mut echo).unwrap(),
                crate::Value::Nil
            );
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn list_locals_and_captures_preserve_exact_canonical_identity() {
        let type_id = IntListTypeId {
            list_type: ListTypeId(3),
        };
        let canonical = ParamLocal::List(ListLocal::Int {
            local: IntListLocalId(2),
            type_id,
        });
        let projected = CallLocal::inspect(&canonical).unwrap();
        assert_eq!(projected.canonical(), canonical);
        assert!(CallLocal::inspect(&canonical).is_some());
        assert!(projected.same_type(&CallLocal::IntList {
            local: IntListLocalId(7),
            type_id
        }));
        assert!(!projected.same_type(&CallLocal::IntList {
            local: IntListLocalId(2),
            type_id: IntListTypeId {
                list_type: ListTypeId(4)
            },
        }));
        assert!(!projected.same_type(&CallLocal::Int(IntLocalId(2))));
        let capture = FunctionCapture::IntList {
            target: IntListLocalId(1),
            source: IntListLocalId(2),
        };
        assert!(Capture::inspect(&capture).unwrap().canonical() == capture);
    }

    #[test]
    fn a_wide_arithmetic_region_after_a_call_keeps_its_canonical_big_integer_result() {
        let source = r#"
fn identity(value: Int) { value }
pub fn main() {
  let calculate = identity
  let value = calculate(1099511627776)
  value * value * value * value
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let main = match plan.program.functions.value_returns.int_functions[0].as_ref() {
            ExecutionFunctionRef::Graph(body) => body,
            ExecutionFunctionRef::Host(never) => match *never {},
        };
        let graph = main.body().block_graph().as_view();
        let (region, arithmetic) = graph
            .blocks()
            .flat_map(|block| block.instructions())
            .find_map(|instruction| match instruction {
                ProfiledInstruction::IntegerRegion(arithmetic) => Some((instruction, arithmetic)),
                _ => None,
            })
            .unwrap();
        assert!(!arithmetic.native);
        let mut program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let main = program
            .functions
            .iter_mut()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(0)))
            .unwrap();
        assert!(
            inspect_instruction(
                region,
                0,
                &mut main.shape,
                &super::CallTypes {
                    custom_types: &plan.program.common.custom_types,
                    value_shapes: &plan.program.common.value_shapes
                }
            )
            .is_none()
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(BigInt::from(1) << 160_usize)
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn a_hosted_wide_arithmetic_region_after_a_call_keeps_its_canonical_big_integer_result() {
        use crate::HostProviderSet;
        let source = r#"
fn identity(value: Int) { value }
pub fn main() {
  let calculate = identity
  let value = calculate(1099511627776)
  value * value * value * value
}
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let graph = hosted_main_body(plan).block_graph().as_view();
        let (region, arithmetic) = graph
            .blocks()
            .flat_map(|block| block.instructions())
            .find_map(|instruction| match instruction {
                ProfiledInstruction::IntegerRegion(arithmetic) => Some((instruction, arithmetic)),
                _ => None,
            })
            .unwrap();
        assert!(!arithmetic.native);
        let mut program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let main = program
            .functions
            .iter_mut()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(0)))
            .unwrap();
        assert!(
            inspect_instruction(
                region,
                0,
                &mut main.shape,
                &super::CallTypes {
                    custom_types: &plan.program.common.custom_types,
                    value_shapes: &plan.program.common.value_shapes
                }
            )
            .is_none()
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::Int(BigInt::from(1) << 160_usize)
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn a_native_arithmetic_region_after_a_call_preserves_its_integer_output() {
        let source = r#"
fn identity(value: Int) { value }
pub fn main() {
  let calculate = identity
  let value = calculate(4)
  value * 2 + 3
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let main = match plan.program.functions.value_returns.int_functions[0].as_ref() {
            ExecutionFunctionRef::Graph(body) => body,
            ExecutionFunctionRef::Host(never) => match *never {},
        };
        let graph = main.body().block_graph().as_view();
        let (instruction, arithmetic) = graph
            .blocks()
            .flat_map(|block| block.instructions())
            .find_map(|instruction| match instruction {
                ProfiledInstruction::IntegerRegion(arithmetic) => Some((instruction, arithmetic)),
                _ => None,
            })
            .unwrap();
        assert!(arithmetic.native);
        assert_eq!(arithmetic.inputs.as_ref(), &[IntLocalId(1)]);
        assert!(matches!(
            CallScalar::inspect(instruction),
            Some(CallScalar::Region { outputs, .. }) if outputs.as_slice() == [IntLocalId(2)]
        ));
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(11.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn each_supported_capture_projection_preserves_its_family_and_both_local_ids() {
        use crate::plan::execution::graph::{
            BitArrayFunctionLocalId, BitArrayListLocalId, BitArrayLocalId, BoolListLocalId,
            FloatFunctionLocalId, FloatListLocalId, FloatLocalId, IntListLocalId,
            NilFunctionLocalId, NilListLocalId, NilLocalId, StringFunctionLocalId,
            StringListLocalId, StringLocalId, UtfCodepointFunctionLocalId, UtfCodepointListLocalId,
            UtfCodepointLocalId,
        };
        for capture in [
            FunctionCapture::Int {
                target: IntLocalId(2),
                source: IntLocalId(7),
            },
            FunctionCapture::Bool {
                target: BoolLocalId(3),
                source: BoolLocalId(8),
            },
            FunctionCapture::IntFunction {
                target: IntFunctionLocalId(4),
                source: IntFunctionLocalId(9),
            },
            FunctionCapture::BoolFunction {
                target: BoolFunctionLocalId(5),
                source: BoolFunctionLocalId(10),
            },
            FunctionCapture::IntList {
                target: IntListLocalId(2),
                source: IntListLocalId(7),
            },
            FunctionCapture::BoolList {
                target: BoolListLocalId(2),
                source: BoolListLocalId(7),
            },
            FunctionCapture::Float {
                target: FloatLocalId(2),
                source: FloatLocalId(7),
            },
            FunctionCapture::FloatList {
                target: FloatListLocalId(2),
                source: FloatListLocalId(7),
            },
            FunctionCapture::FloatFunction {
                target: FloatFunctionLocalId(2),
                source: FloatFunctionLocalId(7),
            },
            FunctionCapture::String {
                target: StringLocalId(2),
                source: StringLocalId(7),
            },
            FunctionCapture::StringList {
                target: StringListLocalId(2),
                source: StringListLocalId(7),
            },
            FunctionCapture::StringFunction {
                target: StringFunctionLocalId(2),
                source: StringFunctionLocalId(7),
            },
            FunctionCapture::BitArray {
                target: BitArrayLocalId(2),
                source: BitArrayLocalId(7),
            },
            FunctionCapture::BitArrayList {
                target: BitArrayListLocalId(2),
                source: BitArrayListLocalId(7),
            },
            FunctionCapture::BitArrayFunction {
                target: BitArrayFunctionLocalId(2),
                source: BitArrayFunctionLocalId(7),
            },
            FunctionCapture::UtfCodepoint {
                target: UtfCodepointLocalId(2),
                source: UtfCodepointLocalId(7),
            },
            FunctionCapture::UtfCodepointList {
                target: UtfCodepointListLocalId(2),
                source: UtfCodepointListLocalId(7),
            },
            FunctionCapture::UtfCodepointFunction {
                target: UtfCodepointFunctionLocalId(2),
                source: UtfCodepointFunctionLocalId(7),
            },
            FunctionCapture::Nil {
                target: NilLocalId(2),
                source: NilLocalId(7),
            },
            FunctionCapture::NilList {
                target: NilListLocalId(2),
                source: NilListLocalId(7),
            },
            FunctionCapture::NilFunction {
                target: NilFunctionLocalId(2),
                source: NilFunctionLocalId(7),
            },
        ] {
            assert!(Capture::inspect(&capture).unwrap().canonical() == capture);
        }
    }

    #[test]
    fn a_dynamic_function_producer_remains_canonical_without_changing_its_supported_template() {
        use crate::{HostProviderSet, StatelessHostProfile};
        let source = r#"
fn identity(value: Int) { value }
fn factory() { identity }
pub fn main() { let make = factory let calculate = make() calculate(7) }
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let main = hosted_main_body(plan);
        let graph = main.block_graph().as_view();
        let block = graph.blocks().next().unwrap();
        let mut program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let factory = program
            .functions
            .iter_mut()
            .find(|function| function.target == CallTarget::IntFunction(IntFunctionFunctionId(0)))
            .unwrap();
        let creation_count = factory.shape.creations.len();
        let invocation = block
            .instructions()
            .iter()
            .find(|instruction| {
                matches!(
                    instruction.value().map(|value| value.kind()),
                    Some(ProfiledInstructionKind::Function(function))
                        if matches!(function.kind(), FunctionInstructionKind::FunctionCall { .. })
                )
            })
            .unwrap();
        assert!(
            inspect_instruction(
                invocation,
                0,
                &mut factory.shape,
                &super::CallTypes {
                    custom_types: &plan.program.common.custom_types,
                    value_shapes: &plan.program.common.value_shapes
                }
            )
            .is_none()
        );
        assert_eq!(factory.shape.creations.len(), creation_count);
        assert!(factory.shape.calls.is_empty());
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::Int(7.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn a_compound_parameter_keeps_each_function_producer_canonical() {
        use crate::{HostProviderSet, StatelessHostProfile};
        for (source, expected) in [
            (
                r#"
fn identity(value: Int) { value }
fn factory(label: #(String)) { fn(value: Int) { case label { #("x") -> identity(value) _ -> 0 } } }
pub fn main() {
  let forward = identity
  let value = forward(7)
  let calculate = factory(#("x"))
  calculate(value)
}
"#,
                crate::Value::Int(7.into()),
            ),
            (
                r#"
fn predicate(flag: Bool) { flag }
fn negative(value: Int) { value < 0 }
fn negate(flag: Bool) { let selected = predicate(flag) !selected }
fn factory(label: #(String)) { fn(flag: Bool) { case label { #("x") -> predicate(flag) _ -> False } } }
pub fn main() {
  let compare = negative
  let less = compare(-1)
  let calculate = negate
  let flag = calculate(less)
  let selected = factory(#("x"))
  selected(flag)
}
"#,
                crate::Value::Bool(false),
            ),
        ] {
            let typed = crate::compile_typed_host_program(
                "example",
                "example",
                [crate::PackageSource::new(
                    "example",
                    Vec::<String>::new(),
                    [crate::ModuleSource::new(
                        "example",
                        "src/example.gleam",
                        source,
                    )],
                )],
                HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
            )
            .unwrap();
            let mut hosted = crate::HostedExecution::try_from_module_plan(
                crate::plan_host_program(typed).unwrap(),
            )
            .unwrap();
            let (plan, _, _) = hosted.parts_mut();
            let program = CallProgram::inspect(
                &plan.program.functions,
                &plan.program.common.custom_types,
                &plan.program.common.value_shapes,
            );
            assert!(!program.functions.is_empty());
            assert!(program.shapes().all(|(target, _)| !matches!(
                target,
                CallTarget::IntFunction(_) | CallTarget::BoolFunction(_)
            )));
            let mut echo = Vec::new();
            assert_eq!(
                crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
                expected
            );
            assert!(echo.is_empty());
        }
    }

    #[test]
    fn tuple_captures_remain_with_the_canonical_capture_owner() {
        assert!(
            Capture::inspect(&FunctionCapture::Tuple {
                source: TupleLocalId(2),
                target: TupleLocalId(3),
            })
            .is_none()
        );
    }

    #[test]
    fn a_compound_capture_declines_creation_before_publishing_a_call_contract() {
        use crate::{HostProviderSet, StatelessHostProfile};
        let source = r#"
fn identity(value: Int) { value }
pub fn main() {
  let calculate = identity
  let value = calculate(7)
  let label = #("x")
  let captured = fn(value) { case label { #("x") -> value _ -> 0 } }
  captured(value)
}
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let main = hosted_main_body(plan);
        let graph = main.block_graph().as_view();
        let closure = graph.blocks().flat_map(|block| block.instructions()).find(|instruction| matches!(instruction.value().map(|value| value.kind()), Some(ProfiledInstructionKind::Function(function)) if matches!(function.kind(), FunctionInstructionKind::Closure { .. }))).unwrap();
        let mut program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let main = program
            .functions
            .iter_mut()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(0)))
            .unwrap();
        let creation_count = main.shape.creations.len();
        assert!(
            inspect_instruction(
                closure,
                0,
                &mut main.shape,
                &super::CallTypes {
                    custom_types: &plan.program.common.custom_types,
                    value_shapes: &plan.program.common.value_shapes
                }
            )
            .is_none()
        );
        assert_eq!(main.shape.creations.len(), creation_count);
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::Int(7.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn scalar_call_classification_preserves_a_typed_tuple_argument() {
        use crate::{HostProviderSet, StatelessHostProfile};
        let source = r#"
fn identity(value: Int) { value }
fn labelled(label: #(String)) { case label { #("x") -> 7 _ -> 0 } }
fn apply(label: #(String)) {
  let calculate = identity
  let value = calculate(1)
  labelled(label) + value
}
pub fn main() { apply(#("x")) }
"#;
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let (plan, _, _) = hosted.parts_mut();
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let tuple_calls = program
            .functions
            .iter()
            .flat_map(|function| &function.shape.calls)
            .filter(|call| {
                call.args
                    .iter()
                    .any(|argument| matches!(argument, CallLocal::Tuple { .. }))
            })
            .collect::<Vec<_>>();
        assert_eq!(tuple_calls.len(), 1);
        assert_eq!(tuple_calls[0].site.function(), "apply");
        assert_eq!(
            tuple_calls[0]
                .args
                .iter()
                .map(CallLocal::canonical)
                .collect::<Vec<_>>(),
            vec![ParamLocal::Tuple {
                local: TupleLocalId(0),
                type_: vec![ValueType::String].into(),
            }]
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::Int(8.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn custom_list_length_values_remain_canonical_in_scalar_calls() {
        let list = ListLocal::Custom {
            local: CustomListLocalId(7),
            type_id: CustomListTypeId {
                list_type: ListTypeId(11),
                item_type: CustomTypeId(4),
            },
        };
        for (test, at_least) in [
            (
                BoolTest::ListLengthEquals {
                    value: list.clone(),
                    length: 3,
                },
                false,
            ),
            (
                BoolTest::ListLengthAtLeast {
                    value: list.clone(),
                    length: 3,
                },
                true,
            ),
        ] {
            let instruction = ProfiledInstruction::<Infallible>::new(
                ParamSlot::new(ParamLocal::Bool(BoolLocalId(2)), ValueShapeId(3)),
                ProfiledInstructionKind::Bool(BoolInstruction::Test(test)),
            );
            let compiled = CompiledInstruction::inspect(&instruction, KernelKind::Numeric).unwrap();
            assert!(matches!(
                compiled,
                CompiledInstruction::Boolean(
                    BoolLocalId(2),
                    CompiledBoolean::Test(CompiledTest::CustomListLength {
                        list: CustomListLocalId(7),
                        length: 3,
                        at_least: projected_at_least,
                    }),
                ) if projected_at_least == at_least
            ));
            assert!(CallScalar::inspect(&instruction).is_none());
        }
    }

    #[test]
    fn custom_construction_keeps_the_supported_call_prefix_and_canonical_field_result() {
        let source = r#"
pub type Item { Item(value: Int) }
fn identity(value: Int) { value }
pub fn main() {
  let calculate = identity
  let value = calculate(7)
  let item = Item(value)
  item.value
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let main = match plan.program.functions.value_returns.int_functions[0].as_ref() {
            ExecutionFunctionRef::Graph(body) => body,
            ExecutionFunctionRef::Host(never) => match *never {},
        };
        let graph = main.body().block_graph().as_view();
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let main = program
            .functions
            .iter()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(0)))
            .unwrap();
        assert_eq!((main.shape.calls.len(), main.shape.creations.len()), (1, 1));
        let interpreted = main
            .shape
            .points
            .iter()
            .zip(&main.shape.checkpoints)
            .filter_map(|(point, checkpoint)| match point {
                CallPoint::Interpreted => Some((checkpoint.block, checkpoint.instruction)),
                _ => None,
            })
            .collect::<Vec<_>>();
        let constructions = graph
            .blocks()
            .enumerate()
            .flat_map(|(block_index, block)| {
                block.instructions().iter().enumerate().filter_map(
                    move |(instruction_index, instruction)| match instruction
                        .value()
                        .map(|value| value.kind())
                    {
                        Some(ProfiledInstructionKind::Custom(CustomInstruction::Construct {
                            ..
                        })) => Some((BlockId(block_index), instruction_index)),
                        _ => None,
                    },
                )
            })
            .collect::<Vec<_>>();
        assert_eq!(constructions.len(), 1);
        assert_eq!(interpreted, constructions);
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(7.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn list_comparison_projects_exact_locals_without_duplicating_a_pure_list_body() {
        let source = r#"
fn identity(value: Int) { value }
fn same(left: List(Int), right: List(Int)) { left == right }
pub fn main() { let calculate = identity let value = calculate(1) same([value], [2]) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let same = match plan.program.functions.value_returns.bool_functions[1].as_ref() {
            ExecutionFunctionRef::Graph(body) => body,
            ExecutionFunctionRef::Host(never) => match *never {},
        };
        let graph = same.body().block_graph().as_view();
        let instruction = &graph.blocks().next().unwrap().instructions()[0];
        assert_eq!(
            CallScalar::inspect(instruction).map(|scalar| {
                let mut code = Code::default();
                write_scalar(&mut code, &scalar);
                code.as_str().to_owned()
            }),
            Some("let bool0 = ops.lists().equal(&int_list0, &int_list1);\n".into())
        );
        let mut program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let main = program
            .functions
            .iter_mut()
            .find(|function| function.target == CallTarget::Bool(BoolFunctionId(0)))
            .unwrap();
        let call_count = main.shape.calls.len();
        assert!(
            inspect_instruction(
                instruction,
                0,
                &mut main.shape,
                &super::CallTypes {
                    custom_types: &plan.program.common.custom_types,
                    value_shapes: &plan.program.common.value_shapes
                }
            )
            .is_some()
        );
        assert_eq!(main.shape.calls.len(), call_count);
        assert!(
            !program
                .functions
                .iter()
                .any(|function| { function.target == CallTarget::Bool(BoolFunctionId(1)) })
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Bool(false)
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn scalar_projection_excludes_operations_owned_by_other_compiled_kernels() {
        for instruction in [
            CompiledInstruction::CustomField(CustomField::Integer {
                output: IntLocalId(0),
                source: CustomLocalId(0),
                index: 0,
            }),
            CompiledInstruction::CustomLoop(CustomLoopInstruction::Index {
                output: CustomLocalId(0),
                list: CustomListLocalId(0),
                index: 0,
            }),
        ] {
            assert!(CallScalar::from_compiled(instruction).is_none());
        }
    }

    #[test]
    fn scalar_call_projection_preserves_string_captures_and_supported_tests() {
        let capture = FunctionCapture::String {
            target: StringLocalId(0),
            source: StringLocalId(1),
        };
        assert!(Capture::inspect(&capture).unwrap().canonical() == capture);
        assert!(CallLocal::inspect(&ParamLocal::String(StringLocalId(1))).is_some());
        let string = CallScalar::from_compiled(CompiledInstruction::String(
            StringLocalId(0),
            StringOperation::Literal("text"),
        ))
        .unwrap();
        let mut code = Code::default();
        write_scalar(&mut code, &string);
        assert_eq!(
            code.as_str(),
            "let string0 = StringValue::from(\"text\");\n"
        );
        for (test, expected) in [
            (
                CompiledTest::IntList(IntListTest::Length {
                    list: IntListLocalId(0),
                    length: 2,
                    at_least: false,
                }),
                "int_list0.len() == 2",
            ),
            (
                CompiledTest::String(StringTest::Prefix {
                    value: StringLocalId(0),
                    prefix: "prefix",
                }),
                "string0.starts_with(\"prefix\".as_bytes())",
            ),
            (
                CompiledTest::String(StringTest::Equal {
                    left: StringLocalId(0),
                    right: StringLocalId(1),
                    negate: false,
                }),
                "string0 == string1",
            ),
            (
                CompiledTest::String(StringTest::Equal {
                    left: StringLocalId(0),
                    right: StringLocalId(1),
                    negate: true,
                }),
                "string0 != string1",
            ),
            (
                CompiledTest::BoolEqual {
                    left: BoolLocalId(0),
                    right: BoolLocalId(1),
                    negate: false,
                },
                "bool0 == bool1",
            ),
        ] {
            assert_eq!(
                test_expression(&CallTest::from_compiled(test).unwrap()),
                expected
            );
        }
        assert!(
            CallTest::from_compiled(CompiledTest::CustomListLength {
                list: CustomListLocalId(0),
                length: 2,
                at_least: true
            })
            .is_none()
        );
    }

    #[test]
    fn closure_and_nested_call_views_record_exact_canonical_locals_and_destinations() {
        let source = r#"
fn identity(value: Int) { value }
fn make(offset: Int) { fn(value) { identity(value) + offset } }
pub fn main() {
  let calculate = make(7)
  calculate(3) + 1
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let main = program
            .shapes()
            .find(|(target, _)| *target == CallTarget::Int(IntFunctionId(0)))
            .unwrap()
            .1;
        assert!(main.root);
        assert_eq!(main.entry(), 0);
        assert_eq!(main.parameter_count, 0);
        assert_eq!(
            main.checkpoints,
            (0..=5)
                .zip([0, 1, 1, 2, 3, 4])
                .map(|(instruction, ints)| CompiledCheckpoint {
                    block: BlockId(0),
                    instruction,
                    ints,
                    bools: 0,
                    bit_arrays: 0,
                    int_lists: 0,
                    strings: 0,
                    customs: 0,
                    custom_lists: 0,
                    int_functions: usize::from(instruction >= 2),
                    bool_functions: 0,
                })
                .collect::<Vec<_>>()
        );
        let calls = main.call_contracts();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].point, 1);
        assert!(
            calls[0].target
                == CallContractTarget::Static(CallTarget::IntFunction(IntFunctionFunctionId(0)))
        );
        assert_eq!(calls[0].args.as_ref(), &[ParamLocal::Int(IntLocalId(0))]);
        assert_eq!(
            calls[0].output,
            ParamLocal::IntFunction {
                local: IntFunctionLocalId(0),
                type_: FunctionType::new(vec![ValueType::Int], ValueType::Int),
            }
        );
        assert_eq!(calls[1].point, 3);
        assert!(calls[1].target == CallContractTarget::IntValue(IntFunctionLocalId(0)));
        assert_eq!(calls[1].output, ParamLocal::Int(IntLocalId(2)));
        assert_eq!(calls[1].args.as_ref(), &[ParamLocal::Int(IntLocalId(1))]);
        assert_eq!(main.return_contracts()[0].point, 5);
        assert_eq!(
            main.return_contracts()[0].value,
            ParamLocal::Int(IntLocalId(3))
        );
        let maker = program
            .shapes()
            .find(|(target, _)| *target == CallTarget::IntFunction(IntFunctionFunctionId(0)))
            .unwrap()
            .1;
        assert!(maker.root);
        assert!(
            maker.creation_contracts()[0].captures.as_ref()
                == [FunctionCapture::Int {
                    target: IntLocalId(1),
                    source: IntLocalId(0)
                }]
        );
        assert!(!maker.creation_contracts()[0].reference);
        let anonymous = program
            .shapes()
            .find(|(_, shape)| {
                shape.parameter_count == 1
                    && shape.locals[0]
                        == [CallLocal::Int(IntLocalId(0)), CallLocal::Int(IntLocalId(1))]
            })
            .unwrap()
            .1;
        assert!(anonymous.root);
        assert!(
            anonymous.call_contracts()[0].target
                == CallContractTarget::Static(CallTarget::Int(IntFunctionId(2)))
        );
        assert_eq!(
            anonymous.call_contracts()[0].output,
            ParamLocal::Int(IntLocalId(2))
        );
        assert_eq!(
            anonymous.return_contracts()[0].value,
            ParamLocal::Int(IntLocalId(3))
        );
        let identity = program
            .shapes()
            .find(|(target, _)| *target == CallTarget::Int(IntFunctionId(2)))
            .unwrap()
            .1;
        assert!(!identity.root);
        assert_eq!(identity.points.len(), 1);
        assert_eq!(identity.returns[0].point, 0);
        assert_eq!(identity.returns[0].value, CallLocal::Int(IntLocalId(0)));
    }

    #[test]
    fn unsupported_dynamic_entry_keeps_its_shared_body_without_selecting_a_root() {
        let source = r#"
fn apply(calculate: fn(Int) -> Int, value: Int) { calculate(value) }
pub fn main() {
  let calculate = fn(value) {
    case [value] {
      [first, ..] -> first
      [] -> 0
    }

  }
  apply(calculate, 7)
}
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let main = program
            .functions
            .iter()
            .find(|function| function.target == CallTarget::Int(IntFunctionId(0)))
            .unwrap();
        assert!(main.shape.root);
        let apply = program
            .functions
            .iter()
            .find(|function| function.target == main.shape.tails[0].target)
            .unwrap();
        assert!(!apply.shape.root);
        assert_eq!(apply.shape.parameter_count, 2);
        assert_eq!(apply.shape.calls.len(), 1);
        let call = &apply.shape.calls[0];
        assert!(call.target == CallContractTarget::IntValue(IntFunctionLocalId(0)));
        assert_eq!(call.args, [CallLocal::Int(IntLocalId(0))]);
        assert!(
            !program
                .functions
                .iter()
                .any(|callee| callee.accepts_call(call))
        );
    }

    #[test]
    fn scalar_wrappers_without_a_connected_callee_keep_the_original_bit_executor() {
        let source = r#"
fn checksum(input: BitArray, total: Int) -> Int {
  case input {
    <<value:8, rest:bits>> -> checksum(rest, total + value)
    <<>> -> total
    _ -> panic as "incomplete byte"
  }
}
fn wrapper(input: BitArray) { checksum(input, 0) }
pub fn main() { wrapper(<<1, 2, 3>>) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        assert!(program.functions.is_empty());
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(6.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn an_unconnected_bit_callback_stays_canonical_beside_a_generated_dynamic_caller() {
        let source = r#"
fn checksum(input: BitArray, total: Int) -> Int {
  case input {
    <<value:8, rest:bits>> -> checksum(rest, total + value)
    <<>> -> total
    _ -> panic as "incomplete byte"
  }
}
fn apply(calculate: fn() -> Int) -> Int { calculate() }
pub fn calculate(input: BitArray) -> Int {
  let checksum = fn() { checksum(input, 0) }
  let constant = fn() { 7 }
  apply(checksum) + apply(constant)
}
pub fn main() { calculate(<<1, 2, 3>>) }
"#;
        let typed = crate::compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let caller = program
            .functions
            .iter()
            .find(|function| function.shape.creations.len() == 2)
            .unwrap();
        assert!(caller.shape.root);
        let checksum = int_callable_target(&caller.shape.creations[0].target);
        let constant = int_callable_target(&caller.shape.creations[1].target);
        assert!(
            !program
                .functions
                .iter()
                .any(|function| function.target == CallTarget::Int(checksum))
        );
        assert!(
            program
                .functions
                .iter()
                .any(|function| function.target == CallTarget::Int(constant))
        );
        let mut echo = Vec::new();
        assert_eq!(
            crate::run_main(&plan, &mut echo).unwrap(),
            crate::Value::Int(13.into())
        );
        assert!(echo.is_empty());
    }

    #[test]
    fn unsupported_suffix_keeps_its_exact_prefix_and_scalar_only_programs_need_no_call_sidecar() {
        let typed = crate::compile_typed_module(
            "example",
            "src/example.gleam",
            "pub fn main() { let add = fn(value) { value + 1 } echo add(7) add(8) }",
        )
        .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        let program = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        let main = program
            .shapes()
            .find(|(target, _)| *target == CallTarget::Int(IntFunctionId(0)))
            .unwrap()
            .1;
        assert!(main.root);
        // Echo ends this supported prefix. Later canonical blocks may still
        // have their own supported entries; they do not move this boundary.
        assert_eq!(main.checkpoints[3].block, BlockId(0));
        assert_eq!(main.checkpoints[3].instruction, 3);
        assert_eq!(main.checkpoints[3].ints, 2);
        let typed =
            crate::compile_typed_module("example", "src/example.gleam", "pub fn main() { 7 }")
                .unwrap();
        let plan = crate::ExecutionPlan::from_module_plan(crate::plan_module(typed).unwrap());
        assert_eq!(
            CallProgram::inspect(
                &plan.program.functions,
                &plan.program.common.custom_types,
                &plan.program.common.value_shapes
            )
            .shapes()
            .count(),
            0
        );
    }
    fn hosted_main_body(
        plan: &HostedProgram<StatelessHostProfile>,
    ) -> &ExecutionIntFunctionBody<HostedExecutionProfile> {
        match plan.program.functions.value_returns.int_functions[0].as_ref() {
            ExecutionFunctionRef::Graph(entry) => entry.body(),
            ExecutionFunctionRef::Host(_) => panic!("fixture main must own its source graph"),
        }
    }

    #[test]
    fn hosted_main_fixture_rejects_a_native_entry_after_its_public_execution() {
        use crate::{HostProviderModule, HostProviderSet, StatelessHostProfile};
        use std::panic::{AssertUnwindSafe, catch_unwind};
        let source = "@external(erlang, \"example\", \"main\") pub fn main() -> Int";
        let typed = crate::compile_typed_host_program(
            "example",
            "example",
            [crate::PackageSource::new(
                "example",
                Vec::<String>::new(),
                [crate::ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    source,
                )],
            )],
            HostProviderSet::from_providers([HostProviderModule::<StatelessHostProfile>::new(
                "example", "example",
            )
            .unwrap()
            .with_function::<(), BigInt, _>("main", || 42.into())
            .unwrap()])
            .unwrap(),
        )
        .unwrap();
        let mut hosted =
            crate::HostedExecution::try_from_module_plan(crate::plan_host_program(typed).unwrap())
                .unwrap();
        let mut echo = Vec::new();
        assert_eq!(
            crate::execution_fixture::run(&mut hosted, &mut (), &mut echo).unwrap(),
            crate::Value::Int(42.into())
        );
        assert!(echo.is_empty());
        let (plan, _, _) = hosted.parts_mut();
        assert!(catch_unwind(AssertUnwindSafe(|| hosted_main_body(plan))).is_err());
    }

    fn source_panic(error: crate::ExecutionError) -> crate::Panic<crate::PanicValue> {
        match error {
            crate::ExecutionError::Panic(panic) => panic,
            _ => panic!("fixture execution must preserve the source panic"),
        }
    }

    fn projected_list_family(
        kind: Option<&ProfiledInstructionKind<std::convert::Infallible>>,
    ) -> &'static str {
        match kind {
            Some(ProfiledInstructionKind::List(ListInstruction::Float(
                _,
                TypedListInstruction::TupleIndex { index: 0, .. },
            ))) => "Float",
            Some(ProfiledInstructionKind::List(ListInstruction::Bool(
                _,
                TypedListInstruction::TupleIndex { index: 0, .. },
            ))) => "Bool",
            Some(ProfiledInstructionKind::List(ListInstruction::Nil(
                _,
                TypedListInstruction::TupleIndex { index: 0, .. },
            ))) => "Nil",
            Some(ProfiledInstructionKind::List(ListInstruction::UtfCodepoint(
                _,
                TypedListInstruction::TupleIndex { index: 0, .. },
            ))) => "UtfCodepoint",
            Some(ProfiledInstructionKind::List(ListInstruction::String(
                _,
                TypedListInstruction::TupleIndex { index: 0, .. },
            ))) => "String",
            Some(ProfiledInstructionKind::List(ListInstruction::BitArray(
                _,
                TypedListInstruction::TupleIndex { index: 0, .. },
            ))) => "BitArray",
            _ => panic!("fixture instruction must project a typed primitive list"),
        }
    }

    fn indexed_list(point: CallPoint<'_>) -> super::PrimitiveListLocal {
        match point {
            CallPoint::Scalar(CallScalar::Index { list, .. }) => list,
            _ => panic!("fixture instruction must index a primitive list"),
        }
    }

    fn int_callable_target(target: &CallableTarget) -> IntFunctionId {
        match target {
            CallableTarget::Int(target) => *target,
            _ => panic!("fixture closure must return Int"),
        }
    }

    #[test]
    fn primitive_fixture_guards_reject_other_failure_instruction_and_callable_shapes() {
        use crate::plan::execution::function::FunctionReturnFamily;
        use std::panic::{AssertUnwindSafe, catch_unwind};
        assert!(
            catch_unwind(|| source_panic(crate::ExecutionError::Invariant(
                crate::InvariantError::FunctionReturnFamilyMismatch {
                    expected: FunctionReturnFamily::Int,
                    actual: FunctionReturnFamily::Bool
                }
            )))
            .is_err()
        );
        assert!(catch_unwind(|| projected_list_family(None)).is_err());
        assert!(catch_unwind(AssertUnwindSafe(|| indexed_list(CallPoint::Interpreted))).is_err());
        assert!(
            catch_unwind(|| int_callable_target(&CallableTarget::Bool(BoolFunctionId(0)))).is_err()
        );
    }

    #[test]
    fn boolean_test_projection_preserves_comparison_and_length_grammar() {
        use crate::plan::execution::graph::{BoolListLocalId, FloatLocalId, IntegerOperand};
        use crate::plan::execution::type_::BoolListTypeId;
        let left = IntegerOperand::Local(IntLocalId(2));
        let right = IntegerOperand::Local(IntLocalId(3));
        for (test, expected) in [
            (BoolTest::Not(BoolLocalId(2)), "!bool2"),
            (BoolTest::EqualInt { left, right }, "int2 == int3"),
            (BoolTest::NotEqualInt { left, right }, "int2 != int3"),
            (BoolTest::LtInt { left, right }, "int2 < int3"),
            (BoolTest::LtEqInt { left, right }, "int2 <= int3"),
            (BoolTest::GtInt { left, right }, "int2 > int3"),
            (BoolTest::GtEqInt { left, right }, "int2 >= int3"),
            (
                BoolTest::LtFloat {
                    left: FloatLocalId(2),
                    right: FloatLocalId(3),
                },
                "f64::lt(&float2, &float3)",
            ),
            (
                BoolTest::LtEqFloat {
                    left: FloatLocalId(2),
                    right: FloatLocalId(3),
                },
                "f64::le(&float2, &float3)",
            ),
            (
                BoolTest::GtFloat {
                    left: FloatLocalId(2),
                    right: FloatLocalId(3),
                },
                "f64::gt(&float2, &float3)",
            ),
            (
                BoolTest::GtEqFloat {
                    left: FloatLocalId(2),
                    right: FloatLocalId(3),
                },
                "f64::ge(&float2, &float3)",
            ),
            (
                BoolTest::StringStartsWith {
                    value: StringLocalId(2),
                    prefix: "tag:".into(),
                },
                r#"string2.starts_with("tag:".as_bytes())"#,
            ),
            (
                BoolTest::Equal {
                    left: ParamLocal::Bool(BoolLocalId(2)),
                    right: ParamLocal::Bool(BoolLocalId(3)),
                },
                "bool2 == bool3",
            ),
            (
                BoolTest::NotEqual {
                    left: ParamLocal::Bool(BoolLocalId(2)),
                    right: ParamLocal::Bool(BoolLocalId(3)),
                },
                "bool2 != bool3",
            ),
        ] {
            assert_eq!(
                test_expression(&CallTest::inspect(&test).unwrap()),
                expected
            );
        }
        for test in [
            BoolTest::ListLengthEquals {
                value: ListLocal::Tuple {
                    local: crate::plan::execution::graph::TupleListLocalId(0),
                    type_id: crate::plan::execution::type_::TupleListTypeId {
                        list_type: ListTypeId(0),
                        item_type: crate::plan::execution::type_::list::TupleItemTypeId(0),
                    },
                },
                length: 1,
            },
            BoolTest::ListLengthAtLeast {
                value: ListLocal::Tuple {
                    local: crate::plan::execution::graph::TupleListLocalId(0),
                    type_id: crate::plan::execution::type_::TupleListTypeId {
                        list_type: ListTypeId(0),
                        item_type: crate::plan::execution::type_::list::TupleItemTypeId(0),
                    },
                },
                length: 1,
            },
        ] {
            assert!(CallTest::inspect(&test).is_none());
        }
        let list = ListLocal::Bool {
            local: BoolListLocalId(1),
            type_id: BoolListTypeId {
                list_type: ListTypeId(0),
            },
        };
        for (test, at_least) in [
            (
                BoolTest::ListLengthEquals {
                    value: list.clone(),
                    length: 3,
                },
                false,
            ),
            (
                BoolTest::ListLengthAtLeast {
                    value: list,
                    length: 3,
                },
                true,
            ),
        ] {
            let projected = CallTest::inspect(&test).unwrap();
            assert!(
                matches!(projected, CallTest::Length { length: 3, at_least: value, .. } if value == at_least)
            );
        }
    }

    #[test]
    fn integer_inequality_keeps_the_requested_local_operands() {
        let test = CallTest::inspect(&BoolTest::NotEqualInt {
            left: crate::plan::execution::graph::IntegerOperand::Local(IntLocalId(2)),
            right: crate::plan::execution::graph::IntegerOperand::Local(IntLocalId(3)),
        })
        .unwrap();
        assert_eq!(test_expression(&test), "int2 != int3");
    }

    #[test]
    fn unconnected_boolean_wrappers_keep_canonical_execution_and_echo_order() {
        use crate::{ExecutionPlan, Value, compile_typed_module, plan_module, run_main};

        let source = r#"
fn accepted() -> Bool { echo "accepted" True }
pub fn verify() -> Bool {
  let assert True = accepted()
  accepted()
}
pub fn main() { let _ = verify() Nil }
"#;
        let typed = compile_typed_module("example", "src/example.gleam", source).unwrap();
        let plan = ExecutionPlan::from_module_plan(plan_module(typed).unwrap());
        let calls = CallProgram::inspect(
            &plan.program.functions,
            &plan.program.common.custom_types,
            &plan.program.common.value_shapes,
        );
        assert!(calls.functions.is_empty());
        let mut echo = Vec::new();
        assert_eq!(run_main(&plan, &mut echo).unwrap(), Value::Nil);
        assert_eq!(
            echo.iter().map(|output| output.value()).collect::<Vec<_>>(),
            [
                &Value::String("accepted".into()),
                &Value::String("accepted".into())
            ]
        );
    }
}
