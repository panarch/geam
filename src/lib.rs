#![recursion_limit = "256"]

#[cfg(feature = "gleam-erlang")]
pub use geam_erlang as gleam_erlang;

#[cfg(feature = "gleam-json")]
pub mod gleam_json;
#[cfg(feature = "gleam-stdlib")]
pub mod gleam_stdlib;
#[cfg(feature = "gleam-time")]
pub mod gleam_time;
#[cfg(feature = "geam-builtin")]
pub use geam_builtin as builtin;
#[cfg(feature = "geam-builtin")]
pub use geam_builtin::FutureComponent;
#[cfg(feature = "provider")]
pub mod provider {
    pub mod advanced {
        pub use geam_core::provider::advanced::{
            DynamicKind, Equality, External, Hashing, Index0, Inspection, NativeKind, NativeMap,
            NativeMapEntry, NativeValue, Next, Retained, RetainedExternalPayload, StoredDynamic,
        };
    }

    pub use geam_core::provider::{
        BigInt, BitArrayValue, Call, Callback, Configuration, EcoString, ExternalPayload, Factory,
        Future, GleamError, GleamOk, GleamResult, HostFailure, HostResult, InitializationError,
        List, Stored, StringValue, Value,
    };
}

#[doc(hidden)]
#[cfg(feature = "provider")]
pub mod __macro_support {
    pub use geam_core::__macro_support::{
        Call, Callback, EcoString, Equality, ExternalPayload, Factory, Future, Hashing, HostCall,
        HostCallCompletion, HostCallContinuation, HostCallError, HostCallable, HostCallableSchema,
        HostCaptures, HostComponentProfile, HostConstruction, HostConstructions,
        HostCreatedFunction, HostCustom, HostCustomConstructorAt, HostCustomConstructorDefinition,
        HostCustomConstructorList, HostCustomConstructorListEnd, HostCustomField,
        HostCustomFieldList, HostCustomFieldListEnd, HostCustomIndex0, HostCustomIndexNext,
        HostCustomSchema, HostCustomType, HostCustomTypeArgument, HostExecutionError, HostExternal,
        HostExternalBinding, HostExternalEquality, HostExternalHashing, HostExternalInspection,
        HostExternalSchema, HostExternalStorage, HostExternalStore, HostExternalType,
        HostFunctionType, HostFutureType, HostList, HostListType, HostNeverContinuation,
        HostNominalCustomField, HostOpaqueFunctionType, HostOwnedCompletion, HostProfile,
        HostProvider, HostProviderComponent, HostProviderComponentInitialization,
        HostProviderComponentRegistration, HostProviderConfiguration,
        HostProviderInitializationError, HostProviderModule, HostRegistrationError, HostResult,
        HostReturns, HostStoredType, HostStoredValue, HostTuple, HostTupleType, HostType,
        HostTypeAt, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
        HostTypeParameter, HostTypeSequence, HostWorkProfile, HostWorkSchema, Index0, Inspection,
        List, MissingCallbackContext, MissingExternalInputContext, MissingExternalOutputContext,
        MissingListContext, MissingStoredContext, MissingValueContext, NativeValue, Next,
        NoCustomInput, ProviderActiveCall, ProviderCallPlaceholder, ProviderCallbackCodec,
        ProviderCallbackContext, ProviderCallbackListDecoder, ProviderConstruction,
        ProviderConstructionIndex0, ProviderConstructionIndexNext, ProviderConstructionList,
        ProviderConstructionRequirementAt, ProviderConstructionRequirements, ProviderConstructions,
        ProviderContextualValueForms, ProviderCustomDeclaration, ProviderCustomInputDeclaration,
        ProviderDynamicInput, ProviderDynamicValue, ProviderError, ProviderExecutionCall,
        ProviderExternalCodec, ProviderExternalDeclaration, ProviderExternalInputContext,
        ProviderExternalListDecoder, ProviderExternalOutput, ProviderExternalPayloadAccess,
        ProviderExternalReturn, ProviderExternalView, ProviderFactoryBinding,
        ProviderFactoryBindings, ProviderFactoryCodec, ProviderFuture, ProviderFutureCall,
        ProviderFutureCodec, ProviderFutureListDecoder, ProviderFutureValueContext,
        ProviderImmediateCaptures, ProviderInputValue, ProviderInvocationRequired,
        ProviderListContext, ProviderListCustomFields, ProviderListInputCodec,
        ProviderListInputValue, ProviderListItemDecoder, ProviderListItemValue,
        ProviderListTupleItems, ProviderMarkerListForms, ProviderModuleRegistration,
        ProviderNoConstructions, ProviderNoFactories, ProviderNone, ProviderOk, ProviderOption,
        ProviderOutputValue, ProviderOwnedCallbackContext, ProviderOwnedCallbackListDecoder,
        ProviderOwnedCaptures, ProviderOwnedExternal, ProviderOwnedExternalInputContext,
        ProviderOwnedExternalListDecoder, ProviderOwnedStoredInput, ProviderPackage,
        ProviderResult, ProviderRootOutputValue, ProviderRuntimeValueForms,
        ProviderScalarListDecoder, ProviderSharedCall, ProviderSome, ProviderStaticValueForms,
        ProviderStoredInput, ProviderStoredOutput, ProviderStoredOwner,
        ProviderTypedListItemDecoder, ProviderTypedValue, ProviderValue, ProviderValueContext,
        ProviderValueForms, Retained, RetainedExternalPayload, Stored, StoredDynamic, Value,
        component_initialization_error, external_payload_hash,
    };
}

#[doc(hidden)]
pub use geam_core::__prepared_support;
#[cfg(feature = "standalone")]
#[doc(hidden)]
#[path = "standalone.rs"]
pub mod __standalone_support;
pub use geam_core::List;
#[cfg(feature = "embedding")]
pub mod embedding;
pub use geam_core::{execution, frontend, host, plan, planner, runtime};
#[cfg(feature = "provider")]
pub use geam_macros::{callable, custom, external, function, module, provider};

pub use geam_core::frontend::{
    DeclaredTypedProgram, FrontendError, HostedTypedProgram, ModuleSource, PackageSource,
    ProjectError, TypedProgram, compile_declared_host_program, compile_declared_host_project,
    compile_typed_host_program, compile_typed_host_project, compile_typed_module,
    compile_typed_package_program, compile_typed_program, compile_typed_project,
};
pub use geam_core::host::{
    FallibleHostFunction, HostCall, HostCallCompletion, HostCallContinuation, HostCallError,
    HostCallable, HostCallableSchema, HostCaptures, HostCompletion, HostComponentProfile,
    HostConstruction, HostConstructions, HostCreatedFunction, HostCustom, HostCustomConstructor,
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomConstructorSchema, HostCustomField,
    HostCustomFieldList, HostCustomFieldListEnd, HostCustomFieldSchema, HostCustomIndex0,
    HostCustomIndexNext, HostCustomSchema, HostCustomType, HostCustomTypeArgument,
    HostCustomTypeSchema, HostDeclarations, HostDiverges, HostExecutionContext, HostExecutionError,
    HostExecutionService, HostExternal, HostExternalBinding, HostExternalEquality,
    HostExternalHashing, HostExternalInspection, HostExternalPayloadBuilder,
    HostExternalPayloadView, HostExternalSchema, HostExternalStorage, HostExternalStore,
    HostExternalType, HostExternalTypeSchema, HostFailure, HostFunction, HostFunctionDeclaration,
    HostFunctionSchema, HostFunctionType, HostFutureStore, HostList, HostListType, HostModule,
    HostModuleDeclaration, HostNeverContinuation, HostOwnedCallable, HostOwnedCompletion,
    HostProfile, HostProvider, HostProviderComponent, HostProviderComponentInitialization,
    HostProviderComponentRegistration, HostProviderConfiguration, HostProviderConfigurationValue,
    HostProviderInitializationError, HostProviderModule, HostProviderModuleDeclaration,
    HostProviderSet, HostRegistrationError, HostReturns, HostSchemaType, HostServiceProfile,
    HostStoredDynamic, HostStoredType, HostStoredValue, HostTuple, HostTupleType, HostType,
    HostTypeAt, HostTypeIndex0, HostTypeIndexNext, HostTypeList, HostTypeListEnd,
    HostTypeParameter, HostTypeSequence, HostValue, HostWorkProfile, HostWorkRepresentation,
    HostWorkSchema, HostWorkStorage, ResumableHostCallable, ScopedConstructingHostFunction,
    ScopedDivergingHostFunction, ScopedHostCallable, ScopedHostFunction, StatelessHostProfile,
};
pub use geam_core::plan::execution::{
    ExecutionPlan, ExecutionPlanExplanation, HostSpecializationError,
    HostSpecializationErrorReason, HostedEntry, HostedExecution, PreparedHostedEntry,
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
    BitArraySegmentPanicReason, BitArrayValue, BitArrayValueLengthError, CustomFieldValue,
    CustomValue, EchoLocation, EchoOutput, EchoSink, ExecutionError, ExternalValue,
    ExternalValueIdentity, FunctionValue, HostError, HostLocation, HostOrigin, InvariantError,
    ListValue, ListValueItemTypeMismatch, Panic, PanicDetails, PanicKind, PanicMessage, PanicValue,
    StringValue, Value, ValueInspection, run_main,
};
