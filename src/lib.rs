#![recursion_limit = "256"]

#[cfg(feature = "gleam-json")]
pub mod gleam_json;
#[cfg(feature = "gleam-stdlib")]
pub mod gleam_stdlib;
#[cfg(feature = "gleam-time")]
pub mod gleam_time;
#[cfg(feature = "geam-runtime-api")]
pub use geam_runtime_api as runtime_api;
#[cfg(feature = "geam-runtime-api")]
pub use geam_runtime_api::FutureComponent;
#[cfg(feature = "provider")]
pub mod provider {
    pub mod advanced {
        pub use geam_core::provider::advanced::{
            DynamicKind, Equality, External, Hashing, Index0, Inspection, Next, Retained,
            RetainedExternalPayload, StoredDynamic,
        };
    }

    pub use geam_core::provider::{
        BigInt, BitArrayValue, Call, Callback, Configuration, EcoString, ExternalPayload, Future,
        HostFailure, HostResult, InitializationError, List, Stored, Value,
    };
}

#[doc(hidden)]
#[cfg(feature = "provider")]
pub mod __macro_support {
    pub use geam_core::__macro_support::{
        AsyncHostCallError, AsyncHostComponentProfile, AsyncHostExternalBinding,
        AsyncHostExternalEquality, AsyncHostExternalHashing, AsyncHostExternalInspection,
        AsyncHostExternalStorage, AsyncHostExternalStore, AsyncHostProviderComponent, Call,
        Callback, EcoString, Equality, ExternalPayload, Hashing, HostCall, HostCallCompletion,
        HostCallError, HostCallable, HostComponentProfile, HostConstruction, HostConstructions,
        HostCustom, HostCustomConstructorAt, HostCustomConstructorDefinition,
        HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomField,
        HostCustomFieldList, HostCustomFieldListEnd, HostCustomIndex0, HostCustomIndexNext,
        HostCustomSchema, HostCustomType, HostExternal, HostExternalBinding, HostExternalEquality,
        HostExternalHashing, HostExternalInspection, HostExternalSchema, HostExternalStorage,
        HostExternalStore, HostExternalType, HostFunctionType, HostFutureCompletion,
        HostFutureError, HostFutureType, HostList, HostListType, HostOpaqueFunctionType,
        HostProfile, HostProvider, HostProviderComponent, HostProviderComponentInitialization,
        HostProviderComponentRegistration, HostProviderConfiguration,
        HostProviderInitializationError, HostProviderModule, HostRegistrationError, HostResult,
        HostStoredType, HostStoredValue, HostTuple, HostTupleType, HostType, HostTypeAt,
        HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd, HostTypeParameter,
        HostTypeSequence, HostWorkProfile, HostWorkSchema, Index0, Inspection, List,
        LocalRetainedContext, MissingCallbackContext, MissingExternalInputContext,
        MissingExternalOutputContext, MissingStoredContext, MissingValueContext, Next,
        NoCustomInput, ProviderActiveCall, ProviderAsyncExternalInputContext,
        ProviderAsyncStoredInput, ProviderCallPlaceholder, ProviderCallbackCodec,
        ProviderCallbackContext, ProviderConstruction, ProviderConstructionIndex0,
        ProviderConstructionIndexNext, ProviderConstructionList, ProviderConstructionRequirementAt,
        ProviderConstructionRequirements, ProviderConstructions, ProviderCustomDeclaration,
        ProviderCustomInputDeclaration, ProviderDynamicInput, ProviderDynamicValue, ProviderError,
        ProviderExternalCodec, ProviderExternalDeclaration, ProviderExternalInputContext,
        ProviderExternalItem, ProviderExternalListDecoder, ProviderExternalOutput,
        ProviderExternalOutputContext, ProviderExternalPayloadAccess, ProviderFuture,
        ProviderFutureCall, ProviderFutureCallbackContext, ProviderFutureValueContext,
        ProviderInputListContext, ProviderInputValue, ProviderListContext,
        ProviderListCustomFields, ProviderListInputCodec, ProviderListInputValue,
        ProviderListItemDecoder, ProviderListItemValue, ProviderModuleRegistration,
        ProviderNoConstructions, ProviderNone, ProviderOk, ProviderOption, ProviderOutputValue,
        ProviderPackage, ProviderResult, ProviderRootOutputValue, ProviderScalarListDecoder,
        ProviderSharedCall, ProviderSome, ProviderStoredInput, ProviderStoredOutput,
        ProviderStoredOwner, ProviderTransferActiveCall, ProviderTransferCallbackCodec,
        ProviderTransferCallbackContext, ProviderTransferDynamicInput,
        ProviderTransferDynamicValue, ProviderTransferEquality, ProviderTransferExternalCodec,
        ProviderTransferExternalInputContext, ProviderTransferExternalItem,
        ProviderTransferExternalListDecoder, ProviderTransferExternalOutput,
        ProviderTransferExternalPayloadAccess, ProviderTransferExternalView,
        ProviderTransferExternalViewListDecoder, ProviderTransferHashing,
        ProviderTransferInputListContext, ProviderTransferInputValue, ProviderTransferInspection,
        ProviderTransferListContext, ProviderTransferListCustomFields,
        ProviderTransferListInputCodec, ProviderTransferListInputValue,
        ProviderTransferListItemDecoder, ProviderTransferListItemValue,
        ProviderTransferListTupleItems, ProviderTransferOutputValue, ProviderTransferPayload,
        ProviderTransferRetained, ProviderTransferRetainedContext, ProviderTransferRootOutputValue,
        ProviderTransferStoredDynamic, ProviderTransferStoredInput, ProviderTransferStoredOutput,
        ProviderTransferValue, ProviderTransferValueContext, ProviderValue, ProviderValueContext,
        Retained, RetainedContext, RetainedExternalPayload, Stored, TransferHostCall,
        TransferHostProviderComponentRegistration, TransferHostProviderModule,
        TransferProviderModuleRegistration, Value, component_initialization_error,
        external_payload_hash,
    };
}

pub use geam_core::List;
#[cfg(feature = "embedding")]
pub mod embedding;
pub use geam_core::{frontend, host, plan, planner, runtime};
#[cfg(feature = "provider")]
pub use geam_macros::{custom, external, function, module, provider};

pub use geam_core::frontend::{
    FrontendError, HostedTypedProgram, ModuleSource, PackageSource, ProjectError,
    TransferHostedTypedProgram, TypedProgram, compile_typed_host_program,
    compile_typed_host_project, compile_typed_module, compile_typed_package_program,
    compile_typed_program, compile_typed_project, compile_typed_transfer_host_program,
    compile_typed_transfer_host_project,
};
pub use geam_core::host::{
    AsyncHostCallError, AsyncHostComponentProfile, AsyncHostExternalBinding,
    AsyncHostExternalEquality, AsyncHostExternalHashing, AsyncHostExternalInspection,
    AsyncHostExternalStorage, AsyncHostExternalStore, AsyncHostProviderComponent,
    FallibleHostFunction, HostCall, HostCallCompletion, HostCallError, HostCallable,
    HostComponentProfile, HostConstruction, HostConstructions, HostCustom, HostCustomConstructor,
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomConstructorSchema, HostCustomField,
    HostCustomFieldList, HostCustomFieldListEnd, HostCustomFieldSchema, HostCustomIndex0,
    HostCustomIndexNext, HostCustomSchema, HostCustomType, HostCustomTypeArgument,
    HostCustomTypeSchema, HostExternal, HostExternalBinding, HostExternalEquality,
    HostExternalHashing, HostExternalInspection, HostExternalPayloadBuilder,
    HostExternalPayloadView, HostExternalSchema, HostExternalStorage, HostExternalStore,
    HostExternalType, HostExternalTypeSchema, HostFailure, HostFunction, HostFunctionSchema,
    HostFunctionType, HostFutureStore, HostList, HostListType, HostModule, HostProfile,
    HostProvider, HostProviderComponent, HostProviderComponentInitialization,
    HostProviderComponentRegistration, HostProviderConfiguration, HostProviderConfigurationValue,
    HostProviderInitializationError, HostProviderModule, HostProviderSet, HostRegistrationError,
    HostSchemaType, HostStoredDynamic, HostStoredType, HostStoredValue, HostTuple, HostTupleType,
    HostType, HostTypeAt, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
    HostTypeParameter, HostTypeSequence, HostValue, HostWorkProfile, HostWorkRepresentation,
    HostWorkSchema, HostWorkStorage, ScopedConstructingHostFunction, ScopedDivergingHostFunction,
    ScopedHostFunction, StatelessHostProfile, TransferHostProviderComponentRegistration,
    TransferHostProviderModule, TransferHostProviderSet,
};
pub use geam_core::plan::execution::{
    ExecutionPlan, ExecutionPlanExplanation, HostSpecializationError,
    HostSpecializationErrorReason, HostedExecution,
};
pub use geam_core::plan::{
    BitArrayExpr, BitArrayLocalId, BoolExpr, BoolLocalId, CustomType, CustomTypeName, EchoSite,
    Expr, ExternalType, ExternalTypeDefinition, ExternalTypeName, FunctionTemplate,
    FunctionTemplateId, FunctionType, HostCallSite, HostFunctionTemplate, HostedModulePlan,
    HostedPlannedModule, IntExpr, IntLocalId, LocalId, ModuleId, ModulePlan, NilExpr, NilLocalId,
    PanicSite, Param, ParamBinding, PlannedModule, SourceContext, SourceSpan, Step, StringExpr,
    StringLocalId, ValueType,
};
pub use geam_core::planner::{
    ExternalTypeProviderLinkReason, HostProviderLinkReason, PlanError, RequiredHostFunction,
    plan_host_program, plan_module, plan_module_with_source, plan_program, required_host_functions,
};
pub use geam_core::runtime::{
    AsyncExecutionError, AsyncPanicValue, BitArraySegmentPanicReason, BitArrayValue,
    BitArrayValueLengthError, CustomFieldValue, CustomValue, EchoLocation, EchoOutput, EchoSink,
    ExecutionError, ExternalValue, ExternalValueIdentity, FunctionValue, HostError, HostLocation,
    HostOrigin, InvariantError, ListValue, ListValueItemTypeMismatch, Panic, PanicDetails,
    PanicKind, PanicMessage, Value, ValueInspection, run_main,
};
