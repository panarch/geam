use crate::plan::HostCallSite;
use crate::plan::execution::compiled::{CallTarget, CompiledCheckpoint};
use crate::plan::execution::function::{
    BoolFunctionFunctionId, BoolFunctionId, IntFunctionFunctionId, IntFunctionId,
};
use crate::plan::execution::graph::{
    BlockGraphExitId, BoolFunctionLocalId, BoolLocalId, IntFunctionLocalId, IntListLocalId,
    IntLocalId,
};
use crate::plan::execution::type_::FunctionType;
use crate::runtime::CaptureStorage;
use crate::runtime::captures::Captures;
use crate::runtime::compiled::int_list::{IntList, IntListOps};
use crate::runtime::compiled::numeric::NumericValues;
use crate::runtime::evaluated::{
    EvaluatedBoolFunction, EvaluatedCapture, EvaluatedCaptureKind, EvaluatedFunction,
    EvaluatedIntFunction, EvaluatedListCapture,
};
use crate::runtime::integer::IntegerValue;
use crate::runtime::state::list::{IntListValueId, RuntimeListStorage};

/// Opaque canonical values at generated/canonical boundaries. Inside a
/// generated body, Small integers and callable locals remain concrete Rust
/// values; these columns are used only to enter or leave that body.
#[derive(Default)]
pub struct CallValues {
    pub ints: Vec<CallInteger>,
    pub bools: Vec<bool>,
    pub int_lists: Vec<IntList>,
    pub int_functions: Vec<IntCallable>,
    pub bool_functions: Vec<BoolCallable>,
}

/// Borrowed canonical inputs used only while selecting and constructing a
/// generated entry. A rejected entry leaves the original columns untouched.
#[derive(Clone, Copy)]
pub struct CallInputs<'values> {
    ints: &'values [IntegerValue],
    bools: &'values [bool],
    int_lists: &'values [IntListValueId],
    int_functions: &'values [EvaluatedIntFunction],
    bool_functions: &'values [EvaluatedBoolFunction],
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

#[derive(Clone)]
pub struct IntCallable(pub(in crate::runtime) EvaluatedIntFunction);

#[derive(Clone)]
pub struct BoolCallable(pub(in crate::runtime) EvaluatedBoolFunction);

#[derive(Clone)]
pub struct CallCaptures(pub(in crate::runtime) Captures);

/// A callee's captures borrowed only while building its generated state.
/// Canonical suspension explicitly retains the original payload instead.
pub struct CallCaptureInputs<'captures>(&'captures Captures);

pub struct CallCapture(EvaluatedCapture);

/// Borrowed only for the current advance. Captures retain the existing
/// execution domain and release queue, never this borrow across a yield.
pub struct CallOps<'execution> {
    captures: &'execution CaptureStorage,
    numeric: &'execution mut NumericValues,
    lists: IntListOps<'execution>,
}

pub trait CallExecution: Send {
    /// A cached execution owns no active locals or return entries. Restart
    /// selects the same typed entry as a fresh start, without borrowing inputs.
    fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool;
    fn retained_bytes(&self) -> usize;
    fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress;
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

/// A normal generated Return carries only its actual result. Canonical
/// checkpoints still use CallValues to restore the complete live environment.
pub enum CallOutput {
    Int(CallInteger),
    Bool(bool),
    IntFunction(IntCallable),
    BoolFunction(BoolCallable),
}

pub struct CallArguments {
    pub values: CallValues,
    pub captures: Option<CallCaptures>,
}

/// Only a canonical boundary allocates a typed resume closure. Ordinary
/// generated calls use their generated, family-specific heap stacks.
pub enum CallProgress {
    Yield(Box<dyn CallExecution>),
    Complete {
        exit: BlockGraphExitId,
        output: CallOutput,
        execution: Box<dyn CallExecution>,
    },
    Interpreted {
        point: CompiledCheckpoint,
        values: CallValues,
    },
    Int {
        function: IntFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
        resume: CallResume<CallInteger>,
    },
    Bool {
        function: BoolFunctionId,
        site: HostCallSite,
        arguments: CallArguments,
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
    InterpretedInt {
        function: IntFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: CallValues,
        resume: CallResume<CallInteger>,
    },
    InterpretedBool {
        function: BoolFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: CallValues,
        resume: CallResume<bool>,
    },
    InterpretedIntFunction {
        function: IntFunctionFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: CallValues,
        resume: CallResume<IntCallable>,
    },
    InterpretedBoolFunction {
        function: BoolFunctionFunctionId,
        site: HostCallSite,
        point: CompiledCheckpoint,
        values: CallValues,
        resume: CallResume<BoolCallable>,
    },
}

impl<'values> CallInputs<'values> {
    pub(in crate::runtime) fn new(
        ints: &'values [IntegerValue],
        bools: &'values [bool],
        int_lists: &'values [IntListValueId],
        int_functions: &'values [EvaluatedIntFunction],
        bool_functions: &'values [EvaluatedBoolFunction],
    ) -> Self {
        Self {
            ints,
            bools,
            int_lists,
            int_functions,
            bool_functions,
        }
    }

    pub fn int(&self, index: usize) -> Option<i128> {
        self.ints.get(index)?.small().map(i128::from)
    }

    pub fn bool(&self, index: usize) -> Option<bool> {
        self.bools.get(index).copied()
    }

    pub fn int_list(&self, index: usize) -> Option<IntList> {
        self.int_lists.get(index).cloned().map(IntList)
    }

    pub fn int_function(&self, index: usize) -> Option<IntCallable> {
        self.int_functions.get(index).cloned().map(IntCallable)
    }

    pub fn bool_function(&self, index: usize) -> Option<BoolCallable> {
        self.bool_functions.get(index).cloned().map(BoolCallable)
    }

    pub fn int_function_target(&self, index: usize) -> Option<IntFunctionId> {
        self.int_functions
            .get(index)
            .map(EvaluatedFunction::runtime_id)
    }

    pub fn bool_function_target(&self, index: usize) -> Option<BoolFunctionId> {
        self.bool_functions
            .get(index)
            .map(EvaluatedFunction::runtime_id)
    }
}

impl IntCallable {
    pub fn with_type(self, type_: FunctionType) -> Self {
        Self(self.0.with_type(type_))
    }
    pub fn target(&self) -> IntFunctionId {
        self.0.runtime_id()
    }

    pub fn captures(&self) -> CallCaptureInputs<'_> {
        CallCaptureInputs(self.0.capture_frame())
    }
}

impl BoolCallable {
    pub fn with_type(self, type_: FunctionType) -> Self {
        Self(self.0.with_type(type_))
    }
    pub fn target(&self) -> BoolFunctionId {
        self.0.runtime_id()
    }

    pub fn captures(&self) -> CallCaptureInputs<'_> {
        CallCaptureInputs(self.0.capture_frame())
    }
}

impl CallCaptureInputs<'_> {
    pub fn retain(&self) -> CallCaptures {
        CallCaptures(self.0.clone())
    }

    pub fn int(&self, local: IntLocalId) -> Option<i128> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::Int {
                    local: target,
                    value,
                } if *target == local => value.small().map(i128::from),
                _ => None,
            })
    }

    pub fn bool(&self, local: BoolLocalId) -> Option<bool> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::Bool {
                    local: target,
                    value,
                } if *target == local => Some(*value),
                _ => None,
            })
    }

    pub fn int_list(&self, local: IntListLocalId) -> Option<IntList> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::List(EvaluatedListCapture::Int {
                    local: target,
                    value,
                }) if *target == local => Some(IntList(value.clone())),
                _ => None,
            })
    }

    pub fn int_function(&self, local: IntFunctionLocalId) -> Option<IntCallable> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::IntFunction {
                    local: target,
                    value,
                } if *target == local => Some(IntCallable(value.clone())),
                _ => None,
            })
    }

    pub fn bool_function(&self, local: BoolFunctionLocalId) -> Option<BoolCallable> {
        self.0
            .values()
            .iter()
            .find_map(|capture| match capture.kind() {
                EvaluatedCaptureKind::BoolFunction {
                    local: target,
                    value,
                } if *target == local => Some(BoolCallable(value.clone())),
                _ => None,
            })
    }
}

impl CallCapture {
    pub fn int(local: IntLocalId, value: i128) -> Self {
        Self(EvaluatedCapture::from_kind(EvaluatedCaptureKind::Int {
            local,
            value: value.into(),
        }))
    }

    pub fn bool(local: BoolLocalId, value: bool) -> Self {
        Self(EvaluatedCapture::from_kind(EvaluatedCaptureKind::Bool {
            local,
            value,
        }))
    }

    pub fn int_list(local: IntListLocalId, value: IntList) -> Self {
        Self(EvaluatedCapture::list(EvaluatedListCapture::Int {
            local,
            value: value.0,
        }))
    }

    pub fn int_function(local: IntFunctionLocalId, value: IntCallable) -> Self {
        Self(EvaluatedCapture::from_kind(
            EvaluatedCaptureKind::IntFunction {
                local,
                value: value.0,
            },
        ))
    }

    pub fn bool_function(local: BoolFunctionLocalId, value: BoolCallable) -> Self {
        Self(EvaluatedCapture::from_kind(
            EvaluatedCaptureKind::BoolFunction {
                local,
                value: value.0,
            },
        ))
    }
}

impl<'execution> CallOps<'execution> {
    pub(in crate::runtime) fn new(
        captures: &'execution CaptureStorage,
        numeric: &'execution mut NumericValues,
        lists: &'execution RuntimeListStorage,
    ) -> Self {
        Self {
            captures,
            numeric,
            lists: IntListOps::new(lists),
        }
    }

    /// Existing execution-owned numeric storage. Generated calls pass their
    /// entry tuple directly; this storage records only a yielded prefix or
    /// result of the shared structured numeric body.
    pub fn numeric(&mut self) -> &mut NumericValues {
        self.numeric
    }

    pub fn lists(&self) -> &IntListOps<'_> {
        &self.lists
    }

    pub fn belongs_to_execution(&self, captures: &CallCaptureInputs<'_>) -> bool {
        captures.0.domain().is_none() || captures.0.domain() == Some(self.captures.domain())
    }

    pub fn int_reference(&self, target: IntFunctionId, type_: FunctionType) -> IntCallable {
        IntCallable(EvaluatedFunction::reference(
            target,
            self.captures.capture(Vec::new()),
            type_,
        ))
    }

    pub fn bool_reference(&self, target: BoolFunctionId, type_: FunctionType) -> BoolCallable {
        BoolCallable(EvaluatedFunction::reference(
            target,
            self.captures.capture(Vec::new()),
            type_,
        ))
    }

    pub fn int_closure(
        &self,
        target: IntFunctionId,
        type_: FunctionType,
        captures: Vec<CallCapture>,
    ) -> IntCallable {
        IntCallable(EvaluatedFunction::closure(
            target,
            self.capture(captures),
            type_,
        ))
    }

    pub fn bool_closure(
        &self,
        target: BoolFunctionId,
        type_: FunctionType,
        captures: Vec<CallCapture>,
    ) -> BoolCallable {
        BoolCallable(EvaluatedFunction::closure(
            target,
            self.capture(captures),
            type_,
        ))
    }

    fn capture(&self, captures: Vec<CallCapture>) -> Captures {
        self.captures
            .capture(captures.into_iter().map(|capture| capture.0).collect())
    }
}

#[cfg(test)]
mod tests {
    use super::{
        CallCapture, CallCaptureInputs, CallExecution, CallInputs, CallInteger, CallOps,
        CallProgress, CallStorage,
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
    use crate::runtime::integer::IntegerValue;
    use crate::runtime::plan_src;
    use crate::runtime::state::list::RuntimeListStorage;
    use std::sync::Arc;
    use std::sync::atomic::{AtomicUsize, Ordering};

    struct IdleWorkspace {
        capacity: Vec<[u8; 1024]>,
        dropped: Arc<AtomicUsize>,
        input: Option<i128>,
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
    fn idle_workspace_reuses_owned_capacity_keeps_rejected_inputs_and_bounds_retention() {
        let dropped = Arc::new(AtomicUsize::new(0));
        let mut cache = CallStorage::default();
        let ints = [IntegerValue::from(7)];
        let inputs = CallInputs::new(&ints, &[], &[], &[], &[]);
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
        assert!(
            cache
                .reuse(target, 3, CallInputs::new(&big, &[], &[], &[], &[]))
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
        let lists = RuntimeListStorage::default();
        let mut budget = 0;
        let progress = execution.advance(
            &mut CallOps::new(&captures, &mut numeric, &lists),
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
    fn entry_inputs_borrow_original_columns_and_clone_only_selected_handles() {
        let storage = CaptureStorage::default();
        let mut numeric = NumericValues::default();
        let lists = RuntimeListStorage::default();
        let ops = CallOps::new(&storage, &mut numeric, &lists);
        let integer = ops.int_reference(
            IntFunctionId(2),
            FunctionType::new(vec![ValueType::Int], ValueType::Int),
        );
        let boolean = ops.bool_reference(
            BoolFunctionId(3),
            FunctionType::new(vec![ValueType::Bool], ValueType::Bool),
        );
        let ints = [IntegerValue::from(7), IntegerValue::from(1_i128 << 100)];
        let bools = [false, true];
        let int_functions = [integer.0];
        let bool_functions = [boolean.0];
        let plan = plan_src("pub fn main() { [1] }");
        let int_lists = [ops
            .lists()
            .value(plan.int_list_function_id(0).type_id(), &[3, 4])
            .0];
        let inputs = CallInputs::new(&ints, &bools, &int_lists, &int_functions, &bool_functions);
        assert!(std::ptr::eq(inputs.ints.as_ptr(), ints.as_ptr()));
        assert!(std::ptr::eq(inputs.bools.as_ptr(), bools.as_ptr()));
        assert!(std::ptr::eq(inputs.int_lists.as_ptr(), int_lists.as_ptr()));
        let selected_list = inputs.int_list(0).unwrap();
        assert_eq!(selected_list.0, int_lists[0]);
        assert!(std::ptr::eq(
            selected_list.0.values(),
            int_lists[0].values()
        ));
        assert_eq!(ops.lists().index(&selected_list, 1), Some(4));
        assert!(inputs.int_list(1).is_none());
        assert!(std::ptr::eq(
            inputs.int_functions.as_ptr(),
            int_functions.as_ptr()
        ));
        assert!(std::ptr::eq(
            inputs.bool_functions.as_ptr(),
            bool_functions.as_ptr()
        ));
        assert_eq!(inputs.int(0), Some(7));
        assert_eq!(inputs.int(1), None);
        assert_eq!(inputs.int(2), None);
        assert_eq!(inputs.bool(0), Some(false));
        assert_eq!(inputs.bool(1), Some(true));
        assert_eq!(inputs.bool(2), None);
        assert_eq!(inputs.int_function_target(0), Some(IntFunctionId(2)));
        assert_eq!(inputs.bool_function_target(0), Some(BoolFunctionId(3)));
        assert_eq!(inputs.int_function_target(1), None);
        assert_eq!(inputs.bool_function_target(1), None);
        assert_eq!(inputs.int_function(0).unwrap().0, int_functions[0]);
        assert_eq!(inputs.bool_function(0).unwrap().0, bool_functions[0]);
        assert!(inputs.int_function(1).is_none());
        assert!(inputs.bool_function(1).is_none());
        assert_eq!(
            ints,
            [IntegerValue::from(7), IntegerValue::from(1_i128 << 100)]
        );
        assert_eq!(bools, [false, true]);
    }

    #[test]
    fn canonical_callable_ops_keep_types_targets_identity_captures_and_execution_ownership() {
        let storage = CaptureStorage::default();
        let other_storage = storage.for_execution();
        let mut numeric = NumericValues::default();
        let lists = RuntimeListStorage::default();
        let ops = CallOps::new(&storage, &mut numeric, &lists);
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
        let mut ops = CallOps::new(&storage, &mut numeric, &lists);
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
