use super::CaptureValue;
use crate::plan::FunctionType;

use crate::plan::execution::function::{
    BitArrayFunctionId, BoolFunctionId, CustomFunctionId, ExternalFunctionId, FloatFunctionId,
    FunctionFunctionId, GenericCallableId, IntFunctionId, NeverFunctionId, NilFunctionId,
    RuntimeListFunctionId, StringFunctionId, TupleFunctionId, UtfCodepointFunctionId,
};
#[cfg(test)]
use crate::plan::execution::function::{
    CoreRuntimeFunctionId, FunctionReturnFamily, RuntimeFunctionId,
};
use crate::plan::execution::graph::ParamLocal;
use crate::plan::execution::type_::CustomConstructorId;

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionValue {
    kind: FunctionValueKind,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum FunctionValueKind {
    Generic(GenericFunctionValue),
    Never(NeverFunctionValue),
    Int(IntFunctionValue),
    Float(FloatFunctionValue),
    String(StringFunctionValue),
    BitArray(BitArrayFunctionValue),
    UtfCodepoint(UtfCodepointFunctionValue),
    Custom(CustomFunctionValue),
    External(ExternalFunctionValue),
    Bool(BoolFunctionValue),
    Nil(NilFunctionValue),
    Tuple(TupleFunctionValue),
    List(ListFunctionValue),
    Function(FunctionFunctionValue),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct GenericFunctionValue {
    target: GenericCallableId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NeverFunctionValue {
    runtime_id: NeverFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct IntFunctionValue {
    runtime_id: IntFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FloatFunctionValue {
    runtime_id: FloatFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct StringFunctionValue {
    runtime_id: StringFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BitArrayFunctionValue {
    runtime_id: BitArrayFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct UtfCodepointFunctionValue {
    runtime_id: UtfCodepointFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct CustomFunctionValue {
    target: CustomFunctionValueTarget,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum CustomFunctionValueTarget {
    Function(CustomFunctionId),
    Constructor(CustomConstructorId),
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ExternalFunctionValue {
    runtime_id: ExternalFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct BoolFunctionValue {
    runtime_id: BoolFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct NilFunctionValue {
    runtime_id: NilFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct TupleFunctionValue {
    runtime_id: TupleFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct ListFunctionValue {
    runtime_id: RuntimeListFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct FunctionFunctionValue {
    runtime_id: FunctionFunctionId,
    params: Vec<ParamLocal>,
    captures: Vec<CaptureValue>,
    type_: FunctionType,
}

impl FunctionValue {
    pub(crate) fn from_kind(kind: FunctionValueKind) -> Self {
        Self { kind }
    }

    pub fn type_(&self) -> FunctionType {
        match &self.kind {
            FunctionValueKind::Generic(value) => value.type_(),
            FunctionValueKind::Never(value) => value.type_(),
            FunctionValueKind::Int(value) => value.type_(),
            FunctionValueKind::Float(value) => value.type_(),
            FunctionValueKind::String(value) => value.type_(),
            FunctionValueKind::BitArray(value) => value.type_(),
            FunctionValueKind::UtfCodepoint(value) => value.type_(),
            FunctionValueKind::Custom(value) => value.type_(),
            FunctionValueKind::External(value) => value.type_(),
            FunctionValueKind::Bool(value) => value.type_(),
            FunctionValueKind::Nil(value) => value.type_(),
            FunctionValueKind::Tuple(value) => value.type_(),
            FunctionValueKind::List(value) => value.type_(),
            FunctionValueKind::Function(value) => value.type_(),
        }
    }

    #[cfg(test)]
    pub(crate) fn new(
        runtime_id: RuntimeFunctionId,
        params: Vec<ParamLocal>,
        type_: FunctionType,
    ) -> Self {
        let kind = match runtime_id {
            RuntimeFunctionId::Core(runtime_id) => match runtime_id {
                CoreRuntimeFunctionId::Never(runtime_id) => FunctionValueKind::Never(
                    NeverFunctionValue::from_evaluated(runtime_id, params, Vec::new(), type_),
                ),
                CoreRuntimeFunctionId::Int(runtime_id) => FunctionValueKind::Int(
                    IntFunctionValue::new_with_captures(runtime_id, params, Vec::new(), type_),
                ),
                CoreRuntimeFunctionId::Float(runtime_id) => FunctionValueKind::Float(
                    FloatFunctionValue::new_with_captures(runtime_id, params, Vec::new(), type_),
                ),
                CoreRuntimeFunctionId::String(runtime_id) => FunctionValueKind::String(
                    StringFunctionValue::new_with_captures(runtime_id, params, Vec::new(), type_),
                ),
                CoreRuntimeFunctionId::BitArray(runtime_id) => FunctionValueKind::BitArray(
                    BitArrayFunctionValue::new_with_captures(runtime_id, params, Vec::new(), type_),
                ),
                CoreRuntimeFunctionId::UtfCodepoint(runtime_id) => {
                    FunctionValueKind::UtfCodepoint(UtfCodepointFunctionValue::new_with_captures(
                        runtime_id,
                        params,
                        Vec::new(),
                        type_,
                    ))
                }
                CoreRuntimeFunctionId::Custom(id) => {
                    FunctionValueKind::Custom(CustomFunctionValue::new_with_captures(
                        CustomFunctionValueTarget::Function(id),
                        params,
                        Vec::new(),
                        type_,
                    ))
                }
                CoreRuntimeFunctionId::Bool(runtime_id) => FunctionValueKind::Bool(
                    BoolFunctionValue::new_with_captures(runtime_id, params, Vec::new(), type_),
                ),
                CoreRuntimeFunctionId::Nil(runtime_id) => FunctionValueKind::Nil(
                    NilFunctionValue::new_with_captures(runtime_id, params, Vec::new(), type_),
                ),
                CoreRuntimeFunctionId::Tuple { id, return_type } => {
                    let _ = return_type;
                    FunctionValueKind::Tuple(TupleFunctionValue::from_evaluated(
                        id,
                        params,
                        Vec::new(),
                        type_,
                    ))
                }
                CoreRuntimeFunctionId::List(id) => FunctionValueKind::List(
                    ListFunctionValue::new_with_captures(id, params, Vec::new(), type_),
                ),
                CoreRuntimeFunctionId::Function { id, return_type } => {
                    let _ = return_type;
                    FunctionValueKind::Function(FunctionFunctionValue::from_evaluated(
                        id.runtime_id(),
                        params,
                        Vec::new(),
                        type_,
                    ))
                }
            },
            RuntimeFunctionId::External(runtime_id) => FunctionValueKind::External(
                ExternalFunctionValue::new_with_captures(runtime_id, params, Vec::new(), type_),
            ),
        };

        Self { kind }
    }

    #[cfg(test)]
    pub(crate) fn kind(&self) -> &FunctionValueKind {
        &self.kind
    }
}

impl FunctionValueKind {
    #[cfg(test)]
    pub(crate) fn family(&self) -> FunctionReturnFamily {
        match self {
            Self::Generic(_) => FunctionReturnFamily::Generic,
            Self::Never(_) => FunctionReturnFamily::Never,
            Self::Int(_) => FunctionReturnFamily::Int,
            Self::Float(_) => FunctionReturnFamily::Float,
            Self::String(_) => FunctionReturnFamily::String,
            Self::BitArray(_) => FunctionReturnFamily::BitArray,
            Self::UtfCodepoint(_) => FunctionReturnFamily::UtfCodepoint,
            Self::Custom(_) => FunctionReturnFamily::Custom,
            Self::External(_) => FunctionReturnFamily::External,
            Self::Bool(_) => FunctionReturnFamily::Bool,
            Self::Nil(_) => FunctionReturnFamily::Nil,
            Self::Tuple(_) => FunctionReturnFamily::Tuple,
            Self::List(_) => FunctionReturnFamily::List,
            Self::Function(_) => FunctionReturnFamily::Function,
        }
    }
}

impl GenericFunctionValue {
    pub(crate) fn from_evaluated(
        target: GenericCallableId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            target,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl NeverFunctionValue {
    pub(crate) fn from_evaluated(
        runtime_id: NeverFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl IntFunctionValue {
    #[cfg(test)]
    pub(crate) fn new(
        runtime_id: IntFunctionId,
        params: Vec<ParamLocal>,
        type_: FunctionType,
    ) -> Self {
        Self::new_with_captures(runtime_id, params, Vec::new(), type_)
    }

    pub(crate) fn new_with_captures(
        runtime_id: IntFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl FloatFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: FloatFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl StringFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: StringFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl BitArrayFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: BitArrayFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl UtfCodepointFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: UtfCodepointFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl CustomFunctionValue {
    pub(crate) fn new_with_captures(
        target: CustomFunctionValueTarget,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            target,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl ExternalFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: ExternalFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl BoolFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: BoolFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl NilFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: NilFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl TupleFunctionValue {
    pub(crate) fn from_evaluated(
        runtime_id: TupleFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl ListFunctionValue {
    pub(crate) fn new_with_captures(
        runtime_id: RuntimeListFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl FunctionFunctionValue {
    pub(crate) fn from_evaluated(
        runtime_id: FunctionFunctionId,
        params: Vec<ParamLocal>,
        captures: Vec<CaptureValue>,
        type_: FunctionType,
    ) -> Self {
        Self {
            runtime_id,
            params,
            captures,
            type_,
        }
    }

    pub(crate) fn type_(&self) -> FunctionType {
        self.type_.clone()
    }
}

impl From<GenericFunctionValue> for FunctionValue {
    fn from(value: GenericFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Generic(value),
        }
    }
}

impl From<NeverFunctionValue> for FunctionValue {
    fn from(value: NeverFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Never(value),
        }
    }
}

impl From<IntFunctionValue> for FunctionValue {
    fn from(value: IntFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Int(value),
        }
    }
}

impl From<FloatFunctionValue> for FunctionValue {
    fn from(value: FloatFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Float(value),
        }
    }
}

impl From<StringFunctionValue> for FunctionValue {
    fn from(value: StringFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::String(value),
        }
    }
}

impl From<BitArrayFunctionValue> for FunctionValue {
    fn from(value: BitArrayFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::BitArray(value),
        }
    }
}

impl From<UtfCodepointFunctionValue> for FunctionValue {
    fn from(value: UtfCodepointFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::UtfCodepoint(value),
        }
    }
}

impl From<CustomFunctionValue> for FunctionValue {
    fn from(value: CustomFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Custom(value),
        }
    }
}

impl From<ExternalFunctionValue> for FunctionValue {
    fn from(value: ExternalFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::External(value),
        }
    }
}

impl From<BoolFunctionValue> for FunctionValue {
    fn from(value: BoolFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Bool(value),
        }
    }
}

impl From<NilFunctionValue> for FunctionValue {
    fn from(value: NilFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Nil(value),
        }
    }
}

impl From<TupleFunctionValue> for FunctionValue {
    fn from(value: TupleFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Tuple(value),
        }
    }
}

impl From<ListFunctionValue> for FunctionValue {
    fn from(value: ListFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::List(value),
        }
    }
}

impl From<FunctionFunctionValue> for FunctionValue {
    fn from(value: FunctionFunctionValue) -> Self {
        Self {
            kind: FunctionValueKind::Function(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FunctionValue, FunctionValueKind, GenericFunctionValue};
    use crate::host::{
        ExternalTestProfile, ExternalTestRunState, HostCall, HostCallCompletion, HostCallError,
        HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType, HostProvider,
        HostProviderModule, HostProviderSet,
    };
    use crate::plan::execution::function::{FunctionReturnFamily, GenericCallableId};
    use crate::plan::execution::runtime::RuntimeExecutionPlan;
    use crate::plan::{
        CustomType, CustomTypeName, ExternalType, ExternalTypeName, FunctionType, TypeParameterId,
        ValueType,
    };
    use crate::{
        HostModule, HostedExecution, ModuleSource, PackageSource, compile_typed_host_program,
        plan_host_program,
    };
    use ecow::EcoString;

    struct ResourceSchema;

    struct ResourceProvider;

    type HostResource = HostExternalType<ResourceSchema>;

    impl HostExternalSchema for ResourceSchema {
        const PACKAGE: &'static str = "application";
        const MODULE: &'static str = "main";
        const NAME: &'static str = "Resource";
        const PARAMETER_COUNT: usize = 0;
    }

    impl HostExternalStorage<ResourceSchema> for ExternalTestProfile {
        type Payload = ();

        fn store(stores: &Self::ExternalStores) -> &HostExternalStore<Self::Payload> {
            &stores.units
        }

        fn source_equal(
            _: &crate::host::HostExternalEquality<'_>,
            _: &Self::Payload,
            _: &Self::Payload,
        ) -> bool {
            true
        }

        fn source_hash(_: &crate::host::HostExternalHashing<'_>, _: &Self::Payload) -> u64 {
            0
        }

        fn inspect(_: &crate::host::HostExternalInspection<'_>, _: &Self::Payload) -> EcoString {
            "Resource".into()
        }
    }

    impl HostProvider<ExternalTestProfile> for ResourceProvider {
        type State = ();

        fn project(state: &mut ExternalTestRunState) -> &mut Self::State {
            &mut state.provider
        }
    }

    fn external_main<'call>(
        mut call: HostCall<'call, ExternalTestProfile, ResourceProvider, HostResource>,
    ) -> Result<HostCallCompletion<'call, HostResource>, HostCallError> {
        let _ = call.state();
        let resource = call.create_external(());
        Ok(call.return_value(resource))
    }

    #[test]
    fn resource_fixture_source_hash_is_exact() {
        let retained_hash = |_: &crate::runtime::StoredRuntimeValue| 7;
        let hashing = crate::host::HostExternalHashing::new(&retained_hash);

        assert_eq!(
            <ExternalTestProfile as HostExternalStorage<ResourceSchema>>::source_hash(
                &hashing,
                &(),
            ),
            0,
        );
    }

    #[test]
    fn function_value_preserves_every_lowered_return_family() {
        let cases = [
            (
                "pub fn main() -> value { panic }",
                ValueType::Parameter(TypeParameterId(0)),
                FunctionReturnFamily::Never,
            ),
            (
                "pub fn main() -> Int { 1 }",
                ValueType::Int,
                FunctionReturnFamily::Int,
            ),
            (
                "pub fn main() -> Float { 1.0 }",
                ValueType::Float,
                FunctionReturnFamily::Float,
            ),
            (
                "pub fn main() -> String { \"one\" }",
                ValueType::String,
                FunctionReturnFamily::String,
            ),
            (
                "pub fn main() -> BitArray { <<1>> }",
                ValueType::BitArray,
                FunctionReturnFamily::BitArray,
            ),
            (
                "fn value() -> UtfCodepoint { let assert <<value:utf8_codepoint>> = <<65>> value } pub fn main() { value() }",
                ValueType::UtfCodepoint,
                FunctionReturnFamily::UtfCodepoint,
            ),
            (
                "pub type Boxed { Boxed(Int) } pub fn main() -> Boxed { Boxed(1) }",
                boxed_type(),
                FunctionReturnFamily::Custom,
            ),
            (
                "pub fn main() -> Bool { True }",
                ValueType::Bool,
                FunctionReturnFamily::Bool,
            ),
            (
                "pub fn main() -> Nil { Nil }",
                ValueType::Nil,
                FunctionReturnFamily::Nil,
            ),
            (
                "pub fn main() -> #(Int) { #(1) }",
                ValueType::Tuple(vec![ValueType::Int]),
                FunctionReturnFamily::Tuple,
            ),
            (
                "pub fn main() -> List(Int) { [] }",
                ValueType::List(Box::new(ValueType::Int)),
                FunctionReturnFamily::List,
            ),
            (
                "pub fn main() -> fn() -> Int { fn() { 1 } }",
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
                FunctionReturnFamily::Function,
            ),
        ];

        for (source, return_type, family) in cases {
            let plan = crate::runtime::plan_src(source);
            let value = FunctionValue::new(
                plan.main_runtime(),
                Vec::new(),
                FunctionType::new(Vec::new(), return_type.clone()),
            );

            assert_eq!(value.type_(), FunctionType::new(Vec::new(), return_type));
            assert_eq!(value.kind().family(), family);
        }
    }

    #[test]
    fn function_value_from_preserves_every_evaluated_return_family() {
        let cases = [
            (
                "pub fn main() -> value { panic }",
                ValueType::Parameter(TypeParameterId(0)),
            ),
            ("pub fn main() -> Int { 1 }", ValueType::Int),
            ("pub fn main() -> Float { 1.0 }", ValueType::Float),
            ("pub fn main() -> String { \"one\" }", ValueType::String),
            ("pub fn main() -> BitArray { <<1>> }", ValueType::BitArray),
            (
                "fn value() -> UtfCodepoint { let assert <<value:utf8_codepoint>> = <<65>> value } pub fn main() { value() }",
                ValueType::UtfCodepoint,
            ),
            (
                "pub type Boxed { Boxed(Int) } pub fn main() -> Boxed { Boxed(1) }",
                boxed_type(),
            ),
            ("pub fn main() -> Bool { True }", ValueType::Bool),
            ("pub fn main() -> Nil { Nil }", ValueType::Nil),
            (
                "pub fn main() -> #(Int) { #(1) }",
                ValueType::Tuple(vec![ValueType::Int]),
            ),
            (
                "pub fn main() -> List(Int) { [] }",
                ValueType::List(Box::new(ValueType::Int)),
            ),
            (
                "pub fn main() -> fn() -> Int { fn() { 1 } }",
                ValueType::Function(Box::new(FunctionType::new(Vec::new(), ValueType::Int))),
            ),
        ];

        for (source, return_type) in cases {
            let plan = crate::runtime::plan_src(source);
            let value = FunctionValue::new(
                plan.main_runtime(),
                Vec::new(),
                FunctionType::new(Vec::new(), return_type.clone()),
            );
            assert_eq!(clone_through_family(&value), value);
        }

        let generic = GenericFunctionValue::from_evaluated(
            GenericCallableId::Function {
                template: 0,
                substitution: Box::new([]),
            },
            Vec::new(),
            Vec::new(),
            FunctionType::new(
                vec![ValueType::Parameter(crate::plan::TypeParameterId(0))],
                ValueType::Parameter(crate::plan::TypeParameterId(0)),
            ),
        );
        let value = FunctionValue::from(generic.clone());
        assert_eq!(value.kind().family(), FunctionReturnFamily::Generic);
        assert_eq!(clone_through_family(&value), value);
        assert_eq!(FunctionValue::from(generic), value);
    }

    #[test]
    fn external_function_value_preserves_its_runtime_family_and_type() {
        let provider = HostProviderModule::<ExternalTestProfile>::new("application", "main")
            .expect("provider module should be valid")
            .with_external_type::<ResourceSchema>()
            .expect("external type should be valid")
            .with_scoped_function::<ResourceProvider, (), HostResource, _>(
                "resource",
                external_main,
            )
            .expect("external resource function should be valid");
        let source = r#"
@external(erlang, "host", "Resource")
pub type Resource

@external(erlang, "host", "resource")
fn resource() -> Resource

pub type Wrapped {
  Wrapped(value: Resource)
}

fn unwrap(value: Wrapped) -> Resource {
  value.value
}

fn resources() -> List(Resource) {
  [resource()]
}

fn resource_provider() -> fn() -> Resource {
  resource
}

fn resources_provider() -> fn() -> List(Resource) {
  resources
}

pub fn main() -> Resource {
  let selected_resource = #(resource).0
  let selected_resources = #(resources).0
  let selected_resource_provider = #(resource_provider).0
  let selected_resources_provider = #(resources_provider).0
  let assert True = selected_resource == resource
  let assert True = selected_resources == resources
  let assert True = selected_resource_provider == resource_provider
  let assert True = selected_resources_provider == resources_provider
  let first = unwrap(Wrapped(resource()))
  let assert True = first == resource()
  first
}
"#;
        let typed = compile_typed_host_program(
            "application",
            "main",
            [PackageSource::new(
                "application",
                Vec::<&str>::new(),
                [ModuleSource::new("main", "main.gleam", source)],
            )],
            HostProviderSet::with_providers(
                Vec::<HostModule<ExternalTestProfile>>::new(),
                [provider],
            )
            .expect("provider module should be unique"),
        )
        .expect("external main should compile");
        let plan = plan_host_program(typed).expect("external main should plan");
        let execution =
            HostedExecution::try_from_module_plan(plan).expect("external main should seal");
        let external_type = ExternalType::new(
            ExternalTypeName::new("application".into(), "main".into(), "Resource".into()),
            Vec::new(),
        );
        let function_type =
            FunctionType::new(Vec::new(), ValueType::External(external_type.clone()));
        let value = FunctionValue::new(
            RuntimeExecutionPlan::main_runtime(&execution),
            Vec::new(),
            function_type.clone(),
        );

        assert_eq!(value.type_(), function_type);
        assert_eq!(value.kind().family(), FunctionReturnFamily::External);
        assert_eq!(clone_through_family(&value), value);

        let returned = execution
            .run_main(&mut ExternalTestRunState::default(), &mut Vec::new())
            .expect("external main should execute");
        assert_eq!(returned.inspect().to_string(), "Resource");
        assert_eq!(returned.value_type(), ValueType::External(external_type));
    }

    fn clone_through_family(value: &FunctionValue) -> FunctionValue {
        match value.kind() {
            FunctionValueKind::Generic(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::Never(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::Int(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::Float(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::String(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::BitArray(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::UtfCodepoint(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::Custom(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::External(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::Bool(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::Nil(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::Tuple(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::List(value) => FunctionValue::from(value.clone()),
            FunctionValueKind::Function(value) => FunctionValue::from(value.clone()),
        }
    }

    fn boxed_type() -> ValueType {
        ValueType::Custom(CustomType::new(
            CustomTypeName::new("geam".into(), "main".into(), "Boxed".into()),
            Vec::new(),
        ))
    }
}
