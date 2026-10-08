use super::{CallCapture, CallCaptureInputs, CallOps};
use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, FloatFunctionId, IntFunctionId, NilFunctionId,
    StringFunctionId, UtfCodepointFunctionId,
};
use crate::plan::execution::type_::FunctionType;
use crate::runtime::evaluated::{
    EvaluatedBitArrayFunction, EvaluatedBoolFunction, EvaluatedFloatFunction, EvaluatedFunction,
    EvaluatedIntFunction, EvaluatedNilFunction, EvaluatedStringFunction,
    EvaluatedUtfCodepointFunction,
};

#[derive(Clone)]
pub struct IntCallable(pub(in crate::runtime) EvaluatedIntFunction);

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

#[derive(Clone)]
pub struct BoolCallable(pub(in crate::runtime) EvaluatedBoolFunction);

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

#[derive(Clone)]
pub struct FloatCallable(pub(in crate::runtime) EvaluatedFloatFunction);

impl FloatCallable {
    pub fn with_type(self, type_: FunctionType) -> Self {
        Self(self.0.with_type(type_))
    }

    pub fn target(&self) -> FloatFunctionId {
        self.0.runtime_id()
    }

    pub fn captures(&self) -> CallCaptureInputs<'_> {
        CallCaptureInputs(self.0.capture_frame())
    }
}

#[derive(Clone)]
pub struct StringCallable(pub(in crate::runtime) EvaluatedStringFunction);

impl StringCallable {
    pub fn with_type(self, type_: FunctionType) -> Self {
        Self(self.0.with_type(type_))
    }

    pub fn target(&self) -> StringFunctionId {
        self.0.runtime_id()
    }

    pub fn captures(&self) -> CallCaptureInputs<'_> {
        CallCaptureInputs(self.0.capture_frame())
    }
}

#[derive(Clone)]
pub struct BitArrayCallable(pub(in crate::runtime) EvaluatedBitArrayFunction);

impl BitArrayCallable {
    pub fn with_type(self, type_: FunctionType) -> Self {
        Self(self.0.with_type(type_))
    }

    pub fn target(&self) -> BitArrayFunctionId {
        self.0.runtime_id()
    }

    pub fn captures(&self) -> CallCaptureInputs<'_> {
        CallCaptureInputs(self.0.capture_frame())
    }
}

#[derive(Clone)]
pub struct UtfCodepointCallable(pub(in crate::runtime) EvaluatedUtfCodepointFunction);

impl UtfCodepointCallable {
    pub fn with_type(self, type_: FunctionType) -> Self {
        Self(self.0.with_type(type_))
    }

    pub fn target(&self) -> UtfCodepointFunctionId {
        self.0.runtime_id()
    }

    pub fn captures(&self) -> CallCaptureInputs<'_> {
        CallCaptureInputs(self.0.capture_frame())
    }
}

#[derive(Clone)]
pub struct NilCallable(pub(in crate::runtime) EvaluatedNilFunction);

impl NilCallable {
    pub fn with_type(self, type_: FunctionType) -> Self {
        Self(self.0.with_type(type_))
    }

    pub fn target(&self) -> NilFunctionId {
        self.0.runtime_id()
    }

    pub fn captures(&self) -> CallCaptureInputs<'_> {
        CallCaptureInputs(self.0.capture_frame())
    }
}

impl CallOps<'_> {
    pub fn int_reference(&self, target: IntFunctionId, type_: FunctionType) -> IntCallable {
        IntCallable(EvaluatedFunction::reference(
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

    pub fn bool_reference(&self, target: BoolFunctionId, type_: FunctionType) -> BoolCallable {
        BoolCallable(EvaluatedFunction::reference(
            target,
            self.captures.capture(Vec::new()),
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

    pub fn float_reference(&self, target: FloatFunctionId, type_: FunctionType) -> FloatCallable {
        FloatCallable(EvaluatedFunction::reference(
            target,
            self.captures.capture(Vec::new()),
            type_,
        ))
    }

    pub fn float_closure(
        &self,
        target: FloatFunctionId,
        type_: FunctionType,
        captures: Vec<CallCapture>,
    ) -> FloatCallable {
        FloatCallable(EvaluatedFunction::closure(
            target,
            self.capture(captures),
            type_,
        ))
    }

    pub fn string_reference(
        &self,
        target: StringFunctionId,
        type_: FunctionType,
    ) -> StringCallable {
        StringCallable(EvaluatedFunction::reference(
            target,
            self.captures.capture(Vec::new()),
            type_,
        ))
    }

    pub fn string_closure(
        &self,
        target: StringFunctionId,
        type_: FunctionType,
        captures: Vec<CallCapture>,
    ) -> StringCallable {
        StringCallable(EvaluatedFunction::closure(
            target,
            self.capture(captures),
            type_,
        ))
    }

    pub fn bit_array_reference(
        &self,
        target: BitArrayFunctionId,
        type_: FunctionType,
    ) -> BitArrayCallable {
        BitArrayCallable(EvaluatedFunction::reference(
            target,
            self.captures.capture(Vec::new()),
            type_,
        ))
    }

    pub fn bit_array_closure(
        &self,
        target: BitArrayFunctionId,
        type_: FunctionType,
        captures: Vec<CallCapture>,
    ) -> BitArrayCallable {
        BitArrayCallable(EvaluatedFunction::closure(
            target,
            self.capture(captures),
            type_,
        ))
    }

    pub fn utf_codepoint_reference(
        &self,
        target: UtfCodepointFunctionId,
        type_: FunctionType,
    ) -> UtfCodepointCallable {
        UtfCodepointCallable(EvaluatedFunction::reference(
            target,
            self.captures.capture(Vec::new()),
            type_,
        ))
    }

    pub fn utf_codepoint_closure(
        &self,
        target: UtfCodepointFunctionId,
        type_: FunctionType,
        captures: Vec<CallCapture>,
    ) -> UtfCodepointCallable {
        UtfCodepointCallable(EvaluatedFunction::closure(
            target,
            self.capture(captures),
            type_,
        ))
    }

    pub fn nil_reference(&self, target: NilFunctionId, type_: FunctionType) -> NilCallable {
        NilCallable(EvaluatedFunction::reference(
            target,
            self.captures.capture(Vec::new()),
            type_,
        ))
    }

    pub fn nil_closure(
        &self,
        target: NilFunctionId,
        type_: FunctionType,
        captures: Vec<CallCapture>,
    ) -> NilCallable {
        NilCallable(EvaluatedFunction::closure(
            target,
            self.capture(captures),
            type_,
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::CallOps;
    use crate::plan::execution::function::{
        BitArrayFunctionId, BoolFunctionId, FloatFunctionId, IntFunctionId, NilFunctionId,
        StringFunctionId, UtfCodepointFunctionId,
    };
    use crate::plan::execution::graph::IntLocalId;
    use crate::plan::execution::type_::{FunctionType, ValueType};
    use crate::runtime::CaptureStorage;
    use crate::runtime::compiled::calls::CallCapture;
    use crate::runtime::compiled::numeric::NumericValues;
    use crate::runtime::state::list::RuntimeListStorage;

    #[test]
    fn int_callable_preserves_signature_target_capture_identity_and_domain() {
        let storage = CaptureStorage::default();
        let other = storage.for_execution();
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
        let signature = FunctionType::new(vec![ValueType::Int], ValueType::Int);
        let reference = ops.int_reference(IntFunctionId(2), signature.clone());
        assert_eq!(reference.target(), IntFunctionId(2));
        assert!(reference.captures().0.values().is_empty());
        assert!(ops.belongs_to_execution(&reference.captures()));
        let closure = ops.int_closure(
            IntFunctionId(3),
            signature.clone(),
            vec![CallCapture::int(IntLocalId(1), 7)],
        );
        assert_eq!(closure.target(), IntFunctionId(3));
        assert_eq!(closure.captures().int(IntLocalId(1)), Some(7));
        assert!(ops.belongs_to_execution(&closure.captures()));
        assert_eq!(closure.0, closure.clone().0);
        assert_ne!(
            closure.0,
            ops.int_closure(
                IntFunctionId(3),
                signature,
                vec![CallCapture::int(IntLocalId(1), 7)]
            )
            .0
        );
        let retagged_type = FunctionType::new(Vec::new(), ValueType::Int);
        let retagged = closure.clone().with_type(retagged_type.clone());
        assert_eq!(retagged.0.type_(), &retagged_type);
        assert_eq!(retagged.target(), closure.target());
        assert_eq!(retagged.0.capture_frame(), closure.0.capture_frame());
        let mut other_numeric = NumericValues::default();
        let mut other_string_scratch = None;
        let mut other_bit_scratch = None;
        let other_ops = CallOps::new(
            &other,
            &mut other_numeric,
            &lists,
            &mut other_string_scratch,
            &mut other_bit_scratch,
        );
        assert!(!other_ops.belongs_to_execution(&closure.captures()));
        let retained = closure.captures().retain();
        drop(closure);
        assert_eq!(retained.0.values().len(), 1);
    }

    #[test]
    fn bool_callable_preserves_signature_target_capture_identity_and_domain() {
        let storage = CaptureStorage::default();
        let other = storage.for_execution();
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
        let signature = FunctionType::new(vec![ValueType::Int], ValueType::Bool);
        let reference = ops.bool_reference(BoolFunctionId(2), signature.clone());
        assert_eq!(reference.target(), BoolFunctionId(2));
        assert!(reference.captures().0.values().is_empty());
        assert!(ops.belongs_to_execution(&reference.captures()));
        let closure = ops.bool_closure(
            BoolFunctionId(3),
            signature.clone(),
            vec![CallCapture::int(IntLocalId(1), 7)],
        );
        assert_eq!(closure.target(), BoolFunctionId(3));
        assert_eq!(closure.captures().int(IntLocalId(1)), Some(7));
        assert!(ops.belongs_to_execution(&closure.captures()));
        assert_eq!(closure.0, closure.clone().0);
        assert_ne!(
            closure.0,
            ops.bool_closure(
                BoolFunctionId(3),
                signature,
                vec![CallCapture::int(IntLocalId(1), 7)]
            )
            .0
        );
        let retagged_type = FunctionType::new(Vec::new(), ValueType::Bool);
        let retagged = closure.clone().with_type(retagged_type.clone());
        assert_eq!(retagged.0.type_(), &retagged_type);
        assert_eq!(retagged.target(), closure.target());
        assert_eq!(retagged.0.capture_frame(), closure.0.capture_frame());
        let mut other_numeric = NumericValues::default();
        let mut other_string_scratch = None;
        let mut other_bit_scratch = None;
        let other_ops = CallOps::new(
            &other,
            &mut other_numeric,
            &lists,
            &mut other_string_scratch,
            &mut other_bit_scratch,
        );
        assert!(!other_ops.belongs_to_execution(&closure.captures()));
        let retained = closure.captures().retain();
        drop(closure);
        assert_eq!(retained.0.values().len(), 1);
    }

    #[test]
    fn float_callable_preserves_signature_target_capture_identity_and_domain() {
        let storage = CaptureStorage::default();
        let other = storage.for_execution();
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
        let signature = FunctionType::new(vec![ValueType::Int], ValueType::Float);
        let reference = ops.float_reference(FloatFunctionId(2), signature.clone());
        assert_eq!(reference.target(), FloatFunctionId(2));
        assert!(reference.captures().0.values().is_empty());
        assert!(ops.belongs_to_execution(&reference.captures()));
        let closure = ops.float_closure(
            FloatFunctionId(3),
            signature.clone(),
            vec![CallCapture::int(IntLocalId(1), 7)],
        );
        assert_eq!(closure.target(), FloatFunctionId(3));
        assert_eq!(closure.captures().int(IntLocalId(1)), Some(7));
        assert!(ops.belongs_to_execution(&closure.captures()));
        assert_eq!(closure.0, closure.clone().0);
        assert_ne!(
            closure.0,
            ops.float_closure(
                FloatFunctionId(3),
                signature,
                vec![CallCapture::int(IntLocalId(1), 7)]
            )
            .0
        );
        let retagged_type = FunctionType::new(Vec::new(), ValueType::Float);
        let retagged = closure.clone().with_type(retagged_type.clone());
        assert_eq!(retagged.0.type_(), &retagged_type);
        assert_eq!(retagged.target(), closure.target());
        assert_eq!(retagged.0.capture_frame(), closure.0.capture_frame());
        let mut other_numeric = NumericValues::default();
        let mut other_string_scratch = None;
        let mut other_bit_scratch = None;
        let other_ops = CallOps::new(
            &other,
            &mut other_numeric,
            &lists,
            &mut other_string_scratch,
            &mut other_bit_scratch,
        );
        assert!(!other_ops.belongs_to_execution(&closure.captures()));
        let retained = closure.captures().retain();
        drop(closure);
        assert_eq!(retained.0.values().len(), 1);
    }

    #[test]
    fn string_callable_preserves_signature_target_capture_identity_and_domain() {
        let storage = CaptureStorage::default();
        let other = storage.for_execution();
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
        let signature = FunctionType::new(vec![ValueType::Int], ValueType::String);
        let reference = ops.string_reference(StringFunctionId(2), signature.clone());
        assert_eq!(reference.target(), StringFunctionId(2));
        assert!(reference.captures().0.values().is_empty());
        assert!(ops.belongs_to_execution(&reference.captures()));
        let closure = ops.string_closure(
            StringFunctionId(3),
            signature.clone(),
            vec![CallCapture::int(IntLocalId(1), 7)],
        );
        assert_eq!(closure.target(), StringFunctionId(3));
        assert_eq!(closure.captures().int(IntLocalId(1)), Some(7));
        assert!(ops.belongs_to_execution(&closure.captures()));
        assert_eq!(closure.0, closure.clone().0);
        assert_ne!(
            closure.0,
            ops.string_closure(
                StringFunctionId(3),
                signature,
                vec![CallCapture::int(IntLocalId(1), 7)]
            )
            .0
        );
        let retagged_type = FunctionType::new(Vec::new(), ValueType::String);
        let retagged = closure.clone().with_type(retagged_type.clone());
        assert_eq!(retagged.0.type_(), &retagged_type);
        assert_eq!(retagged.target(), closure.target());
        assert_eq!(retagged.0.capture_frame(), closure.0.capture_frame());
        let mut other_numeric = NumericValues::default();
        let mut other_string_scratch = None;
        let mut other_bit_scratch = None;
        let other_ops = CallOps::new(
            &other,
            &mut other_numeric,
            &lists,
            &mut other_string_scratch,
            &mut other_bit_scratch,
        );
        assert!(!other_ops.belongs_to_execution(&closure.captures()));
        let retained = closure.captures().retain();
        drop(closure);
        assert_eq!(retained.0.values().len(), 1);
    }

    #[test]
    fn bit_array_callable_preserves_signature_target_capture_identity_and_domain() {
        let storage = CaptureStorage::default();
        let other = storage.for_execution();
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
        let signature = FunctionType::new(vec![ValueType::Int], ValueType::BitArray);
        let reference = ops.bit_array_reference(BitArrayFunctionId(2), signature.clone());
        assert_eq!(reference.target(), BitArrayFunctionId(2));
        assert!(reference.captures().0.values().is_empty());
        assert!(ops.belongs_to_execution(&reference.captures()));
        let closure = ops.bit_array_closure(
            BitArrayFunctionId(3),
            signature.clone(),
            vec![CallCapture::int(IntLocalId(1), 7)],
        );
        assert_eq!(closure.target(), BitArrayFunctionId(3));
        assert_eq!(closure.captures().int(IntLocalId(1)), Some(7));
        assert!(ops.belongs_to_execution(&closure.captures()));
        assert_eq!(closure.0, closure.clone().0);
        assert_ne!(
            closure.0,
            ops.bit_array_closure(
                BitArrayFunctionId(3),
                signature,
                vec![CallCapture::int(IntLocalId(1), 7)]
            )
            .0
        );
        let retagged_type = FunctionType::new(Vec::new(), ValueType::BitArray);
        let retagged = closure.clone().with_type(retagged_type.clone());
        assert_eq!(retagged.0.type_(), &retagged_type);
        assert_eq!(retagged.target(), closure.target());
        assert_eq!(retagged.0.capture_frame(), closure.0.capture_frame());
        let mut other_numeric = NumericValues::default();
        let mut other_string_scratch = None;
        let mut other_bit_scratch = None;
        let other_ops = CallOps::new(
            &other,
            &mut other_numeric,
            &lists,
            &mut other_string_scratch,
            &mut other_bit_scratch,
        );
        assert!(!other_ops.belongs_to_execution(&closure.captures()));
        let retained = closure.captures().retain();
        drop(closure);
        assert_eq!(retained.0.values().len(), 1);
    }

    #[test]
    fn utf_codepoint_callable_preserves_signature_target_capture_identity_and_domain() {
        let storage = CaptureStorage::default();
        let other = storage.for_execution();
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
        let signature = FunctionType::new(vec![ValueType::Int], ValueType::UtfCodepoint);
        let reference = ops.utf_codepoint_reference(UtfCodepointFunctionId(2), signature.clone());
        assert_eq!(reference.target(), UtfCodepointFunctionId(2));
        assert!(reference.captures().0.values().is_empty());
        assert!(ops.belongs_to_execution(&reference.captures()));
        let closure = ops.utf_codepoint_closure(
            UtfCodepointFunctionId(3),
            signature.clone(),
            vec![CallCapture::int(IntLocalId(1), 7)],
        );
        assert_eq!(closure.target(), UtfCodepointFunctionId(3));
        assert_eq!(closure.captures().int(IntLocalId(1)), Some(7));
        assert!(ops.belongs_to_execution(&closure.captures()));
        assert_eq!(closure.0, closure.clone().0);
        assert_ne!(
            closure.0,
            ops.utf_codepoint_closure(
                UtfCodepointFunctionId(3),
                signature,
                vec![CallCapture::int(IntLocalId(1), 7)]
            )
            .0
        );
        let retagged_type = FunctionType::new(Vec::new(), ValueType::UtfCodepoint);
        let retagged = closure.clone().with_type(retagged_type.clone());
        assert_eq!(retagged.0.type_(), &retagged_type);
        assert_eq!(retagged.target(), closure.target());
        assert_eq!(retagged.0.capture_frame(), closure.0.capture_frame());
        let mut other_numeric = NumericValues::default();
        let mut other_string_scratch = None;
        let mut other_bit_scratch = None;
        let other_ops = CallOps::new(
            &other,
            &mut other_numeric,
            &lists,
            &mut other_string_scratch,
            &mut other_bit_scratch,
        );
        assert!(!other_ops.belongs_to_execution(&closure.captures()));
        let retained = closure.captures().retain();
        drop(closure);
        assert_eq!(retained.0.values().len(), 1);
    }

    #[test]
    fn nil_callable_preserves_signature_target_capture_identity_and_domain() {
        let storage = CaptureStorage::default();
        let other = storage.for_execution();
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
        let signature = FunctionType::new(vec![ValueType::Int], ValueType::Nil);
        let reference = ops.nil_reference(NilFunctionId(2), signature.clone());
        assert_eq!(reference.target(), NilFunctionId(2));
        assert!(reference.captures().0.values().is_empty());
        assert!(ops.belongs_to_execution(&reference.captures()));
        let closure = ops.nil_closure(
            NilFunctionId(3),
            signature.clone(),
            vec![CallCapture::int(IntLocalId(1), 7)],
        );
        assert_eq!(closure.target(), NilFunctionId(3));
        assert_eq!(closure.captures().int(IntLocalId(1)), Some(7));
        assert!(ops.belongs_to_execution(&closure.captures()));
        assert_eq!(closure.0, closure.clone().0);
        assert_ne!(
            closure.0,
            ops.nil_closure(
                NilFunctionId(3),
                signature,
                vec![CallCapture::int(IntLocalId(1), 7)]
            )
            .0
        );
        let retagged_type = FunctionType::new(Vec::new(), ValueType::Nil);
        let retagged = closure.clone().with_type(retagged_type.clone());
        assert_eq!(retagged.0.type_(), &retagged_type);
        assert_eq!(retagged.target(), closure.target());
        assert_eq!(retagged.0.capture_frame(), closure.0.capture_frame());
        let mut other_numeric = NumericValues::default();
        let mut other_string_scratch = None;
        let mut other_bit_scratch = None;
        let other_ops = CallOps::new(
            &other,
            &mut other_numeric,
            &lists,
            &mut other_string_scratch,
            &mut other_bit_scratch,
        );
        assert!(!other_ops.belongs_to_execution(&closure.captures()));
        let retained = closure.captures().retain();
        drop(closure);
        assert_eq!(retained.0.values().len(), 1);
    }
}
