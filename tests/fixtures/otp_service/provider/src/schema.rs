use super::{One, Three};
use geam::gleam_erlang::service::types::ExitReason;
use geam::gleam_erlang::{Atom, Pid};
use geam::gleam_stdlib::provider_support::Dynamic;
use geam::gleam_stdlib::provider_support::GleamResult;
use geam::host::{
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostExternalSchema, HostExternalType, HostFunctionType, HostListType,
    HostTupleType, HostTypeIndex0, HostTypeListEnd,
};
use geam::provider::{BigInt, StringValue};

pub(super) trait NativeSchema: HostExternalSchema {}

pub(super) struct DebugStateSchema;
pub(super) type DebugState = HostExternalType<DebugStateSchema>;
impl NativeSchema for DebugStateSchema {}
impl HostExternalSchema for DebugStateSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/system";
    const NAME: &'static str = "DebugState";
    const PARAMETER_COUNT: usize = 0;
}

pub(super) struct DoNotLeakSchema;
pub(super) type DoNotLeak = HostExternalType<DoNotLeakSchema>;
impl NativeSchema for DoNotLeakSchema {}
impl HostExternalSchema for DoNotLeakSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/system";
    const NAME: &'static str = "DoNotLeak";
    const PARAMETER_COUNT: usize = 0;
}

pub(super) struct ModeSchema;
pub(super) type Mode = HostCustomType<ModeSchema>;
pub(super) struct ModeRunning;
impl HostCustomConstructorDefinition for ModeRunning {
    const NAME: &'static str = "Running";
    type Fields = HostCustomFieldListEnd;
}
pub(super) struct ModeSuspended;
impl HostCustomConstructorDefinition for ModeSuspended {
    const NAME: &'static str = "Suspended";
    type Fields = HostCustomFieldListEnd;
}
impl HostCustomSchema for ModeSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/system";
    const NAME: &'static str = "Mode";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<
        ModeRunning,
        HostCustomConstructorList<ModeSuspended, HostCustomConstructorListEnd>,
    >;
}

pub(super) struct DebugOptionSchema;
pub(super) type DebugOption = HostCustomType<DebugOptionSchema>;
pub(super) struct DebugOptionNoDebug;
impl HostCustomConstructorDefinition for DebugOptionNoDebug {
    const NAME: &'static str = "NoDebug";
    type Fields = HostCustomFieldListEnd;
}
impl HostCustomSchema for DebugOptionSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/system";
    const NAME: &'static str = "DebugOption";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<DebugOptionNoDebug, HostCustomConstructorListEnd>;
}

pub(super) struct StatusInfoSchema;
pub(super) type StatusInfo = HostCustomType<StatusInfoSchema>;
pub(super) struct StatusInfoStatusInfo;
pub(super) struct StatusInfoStatusInfoField0;
impl HostCustomField for StatusInfoStatusInfoField0 {
    const LABEL: Option<&'static str> = Some("module");
    type Type = Atom;
}
pub(super) struct StatusInfoStatusInfoField1;
impl HostCustomField for StatusInfoStatusInfoField1 {
    const LABEL: Option<&'static str> = Some("parent");
    type Type = Pid;
}
pub(super) struct StatusInfoStatusInfoField2;
impl HostCustomField for StatusInfoStatusInfoField2 {
    const LABEL: Option<&'static str> = Some("mode");
    type Type = Mode;
}
pub(super) struct StatusInfoStatusInfoField3;
impl HostCustomField for StatusInfoStatusInfoField3 {
    const LABEL: Option<&'static str> = Some("debug_state");
    type Type = DebugState;
}
pub(super) struct StatusInfoStatusInfoField4;
impl HostCustomField for StatusInfoStatusInfoField4 {
    const LABEL: Option<&'static str> = Some("state");
    type Type = Dynamic;
}
impl HostCustomConstructorDefinition for StatusInfoStatusInfo {
    const NAME: &'static str = "StatusInfo";
    type Fields = HostCustomFieldList<
        StatusInfoStatusInfoField0,
        HostCustomFieldList<
            StatusInfoStatusInfoField1,
            HostCustomFieldList<
                StatusInfoStatusInfoField2,
                HostCustomFieldList<
                    StatusInfoStatusInfoField3,
                    HostCustomFieldList<StatusInfoStatusInfoField4, HostCustomFieldListEnd>,
                >,
            >,
        >,
    >;
}
impl HostCustomSchema for StatusInfoSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/system";
    const NAME: &'static str = "StatusInfo";
    const PARAMETER_COUNT: usize = 0;
    type Constructors =
        HostCustomConstructorList<StatusInfoStatusInfo, HostCustomConstructorListEnd>;
}

pub(super) struct SystemMessageSchema;
pub(super) type SystemMessage = HostCustomType<SystemMessageSchema>;
pub(super) struct SystemMessageResume;
pub(super) struct SystemMessageResumeField0;
impl HostCustomField for SystemMessageResumeField0 {
    const LABEL: Option<&'static str> = None;
    type Type = HostFunctionType<HostTypeListEnd, ()>;
}
impl HostCustomConstructorDefinition for SystemMessageResume {
    const NAME: &'static str = "Resume";
    type Fields = HostCustomFieldList<SystemMessageResumeField0, HostCustomFieldListEnd>;
}
pub(super) struct SystemMessageSuspend;
pub(super) struct SystemMessageSuspendField0;
impl HostCustomField for SystemMessageSuspendField0 {
    const LABEL: Option<&'static str> = None;
    type Type = HostFunctionType<HostTypeListEnd, ()>;
}
impl HostCustomConstructorDefinition for SystemMessageSuspend {
    const NAME: &'static str = "Suspend";
    type Fields = HostCustomFieldList<SystemMessageSuspendField0, HostCustomFieldListEnd>;
}
pub(super) struct SystemMessageGetState;
pub(super) struct SystemMessageGetStateField0;
impl HostCustomField for SystemMessageGetStateField0 {
    const LABEL: Option<&'static str> = None;
    type Type = HostFunctionType<One<Dynamic>, ()>;
}
impl HostCustomConstructorDefinition for SystemMessageGetState {
    const NAME: &'static str = "GetState";
    type Fields = HostCustomFieldList<SystemMessageGetStateField0, HostCustomFieldListEnd>;
}
pub(super) struct SystemMessageGetStatus;
pub(super) struct SystemMessageGetStatusField0;
impl HostCustomField for SystemMessageGetStatusField0 {
    const LABEL: Option<&'static str> = None;
    type Type = HostFunctionType<One<StatusInfo>, ()>;
}
impl HostCustomConstructorDefinition for SystemMessageGetStatus {
    const NAME: &'static str = "GetStatus";
    type Fields = HostCustomFieldList<SystemMessageGetStatusField0, HostCustomFieldListEnd>;
}
impl HostCustomSchema for SystemMessageSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/system";
    const NAME: &'static str = "SystemMessage";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<
        SystemMessageResume,
        HostCustomConstructorList<
            SystemMessageSuspend,
            HostCustomConstructorList<
                SystemMessageGetState,
                HostCustomConstructorList<SystemMessageGetStatus, HostCustomConstructorListEnd>,
            >,
        >,
    >;
}

pub(super) struct MessageSchema;
pub(super) type Message<A> = HostCustomType<MessageSchema, One<A>>;
pub(super) type Unexpected<A> = HostCustomConstructorAt<
    Message<A>,
    HostCustomIndexNext<HostCustomIndexNext<HostCustomIndex0>>,
    MessageUnexpected,
>;
pub(super) struct MessageMessage;
pub(super) struct MessageMessageField0;
impl HostCustomField for MessageMessageField0 {
    const LABEL: Option<&'static str> = None;
    type Type = HostCustomTypeArgument<HostTypeIndex0>;
}
impl HostCustomConstructorDefinition for MessageMessage {
    const NAME: &'static str = "Message";
    type Fields = HostCustomFieldList<MessageMessageField0, HostCustomFieldListEnd>;
}
pub(super) struct MessageSystem;
pub(super) struct MessageSystemField0;
impl HostCustomField for MessageSystemField0 {
    const LABEL: Option<&'static str> = None;
    type Type = SystemMessage;
}
impl HostCustomConstructorDefinition for MessageSystem {
    const NAME: &'static str = "System";
    type Fields = HostCustomFieldList<MessageSystemField0, HostCustomFieldListEnd>;
}
pub(super) struct MessageUnexpected;
pub(super) struct MessageUnexpectedField0;
impl HostCustomField for MessageUnexpectedField0 {
    const LABEL: Option<&'static str> = None;
    type Type = Dynamic;
}
impl HostCustomConstructorDefinition for MessageUnexpected {
    const NAME: &'static str = "Unexpected";
    type Fields = HostCustomFieldList<MessageUnexpectedField0, HostCustomFieldListEnd>;
}
impl HostCustomSchema for MessageSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/actor";
    const NAME: &'static str = "Message";
    const PARAMETER_COUNT: usize = 1;
    type Constructors = HostCustomConstructorList<
        MessageMessage,
        HostCustomConstructorList<
            MessageSystem,
            HostCustomConstructorList<MessageUnexpected, HostCustomConstructorListEnd>,
        >,
    >;
}

pub(super) struct StartedSchema;
pub(super) type Started<A> = HostCustomType<StartedSchema, One<A>>;
pub(super) type StartedConstructor<A> =
    HostCustomConstructorAt<Started<A>, HostCustomIndex0, StartedStarted>;
pub(super) struct StartedStarted;
pub(super) struct StartedStartedField0;
impl HostCustomField for StartedStartedField0 {
    const LABEL: Option<&'static str> = Some("pid");
    type Type = Pid;
}
pub(super) struct StartedStartedField1;
impl HostCustomField for StartedStartedField1 {
    const LABEL: Option<&'static str> = Some("data");
    type Type = HostCustomTypeArgument<HostTypeIndex0>;
}
impl HostCustomConstructorDefinition for StartedStarted {
    const NAME: &'static str = "Started";
    type Fields = HostCustomFieldList<
        StartedStartedField0,
        HostCustomFieldList<StartedStartedField1, HostCustomFieldListEnd>,
    >;
}
impl HostCustomSchema for StartedSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/actor";
    const NAME: &'static str = "Started";
    const PARAMETER_COUNT: usize = 1;
    type Constructors = HostCustomConstructorList<StartedStarted, HostCustomConstructorListEnd>;
}

pub(super) struct StartErrorSchema;
pub(super) type StartError = HostCustomType<StartErrorSchema>;
pub(super) struct StartErrorInitTimeout;
impl HostCustomConstructorDefinition for StartErrorInitTimeout {
    const NAME: &'static str = "InitTimeout";
    type Fields = HostCustomFieldListEnd;
}
pub(super) struct StartErrorInitFailed;
pub(super) struct StartErrorInitFailedField0;
impl HostCustomField for StartErrorInitFailedField0 {
    const LABEL: Option<&'static str> = None;
    type Type = StringValue;
}
impl HostCustomConstructorDefinition for StartErrorInitFailed {
    const NAME: &'static str = "InitFailed";
    type Fields = HostCustomFieldList<StartErrorInitFailedField0, HostCustomFieldListEnd>;
}
pub(super) struct StartErrorInitExited;
pub(super) struct StartErrorInitExitedField0;
impl HostCustomField for StartErrorInitExitedField0 {
    const LABEL: Option<&'static str> = None;
    type Type = ExitReason;
}
impl HostCustomConstructorDefinition for StartErrorInitExited {
    const NAME: &'static str = "InitExited";
    type Fields = HostCustomFieldList<StartErrorInitExitedField0, HostCustomFieldListEnd>;
}
impl HostCustomSchema for StartErrorSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/actor";
    const NAME: &'static str = "StartError";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<
        StartErrorInitTimeout,
        HostCustomConstructorList<
            StartErrorInitFailed,
            HostCustomConstructorList<StartErrorInitExited, HostCustomConstructorListEnd>,
        >,
    >;
}

pub(super) struct RestartSchema;
pub(super) type Restart = HostCustomType<RestartSchema>;
pub(super) struct RestartPermanent;
impl HostCustomConstructorDefinition for RestartPermanent {
    const NAME: &'static str = "Permanent";
    type Fields = HostCustomFieldListEnd;
}
pub(super) struct RestartTransient;
impl HostCustomConstructorDefinition for RestartTransient {
    const NAME: &'static str = "Transient";
    type Fields = HostCustomFieldListEnd;
}
pub(super) struct RestartTemporary;
impl HostCustomConstructorDefinition for RestartTemporary {
    const NAME: &'static str = "Temporary";
    type Fields = HostCustomFieldListEnd;
}
impl HostCustomSchema for RestartSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/supervision";
    const NAME: &'static str = "Restart";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<
        RestartPermanent,
        HostCustomConstructorList<
            RestartTransient,
            HostCustomConstructorList<RestartTemporary, HostCustomConstructorListEnd>,
        >,
    >;
}

pub(super) struct StaticStrategySchema;
pub(super) type StaticStrategy = HostCustomType<StaticStrategySchema>;
pub(super) struct StaticStrategyOneForOne;
impl HostCustomConstructorDefinition for StaticStrategyOneForOne {
    const NAME: &'static str = "OneForOne";
    type Fields = HostCustomFieldListEnd;
}
pub(super) struct StaticStrategyOneForAll;
impl HostCustomConstructorDefinition for StaticStrategyOneForAll {
    const NAME: &'static str = "OneForAll";
    type Fields = HostCustomFieldListEnd;
}
pub(super) struct StaticStrategyRestForOne;
impl HostCustomConstructorDefinition for StaticStrategyRestForOne {
    const NAME: &'static str = "RestForOne";
    type Fields = HostCustomFieldListEnd;
}
impl HostCustomSchema for StaticStrategySchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/static_supervisor";
    const NAME: &'static str = "Strategy";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<
        StaticStrategyOneForOne,
        HostCustomConstructorList<
            StaticStrategyOneForAll,
            HostCustomConstructorList<StaticStrategyRestForOne, HostCustomConstructorListEnd>,
        >,
    >;
}

pub(super) struct StaticAutoShutdownSchema;
pub(super) type StaticAutoShutdown = HostCustomType<StaticAutoShutdownSchema>;
pub(super) struct StaticAutoShutdownNever;
impl HostCustomConstructorDefinition for StaticAutoShutdownNever {
    const NAME: &'static str = "Never";
    type Fields = HostCustomFieldListEnd;
}
pub(super) struct StaticAutoShutdownAnySignificant;
impl HostCustomConstructorDefinition for StaticAutoShutdownAnySignificant {
    const NAME: &'static str = "AnySignificant";
    type Fields = HostCustomFieldListEnd;
}
pub(super) struct StaticAutoShutdownAllSignificant;
impl HostCustomConstructorDefinition for StaticAutoShutdownAllSignificant {
    const NAME: &'static str = "AllSignificant";
    type Fields = HostCustomFieldListEnd;
}
impl HostCustomSchema for StaticAutoShutdownSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/static_supervisor";
    const NAME: &'static str = "AutoShutdown";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<
        StaticAutoShutdownNever,
        HostCustomConstructorList<
            StaticAutoShutdownAnySignificant,
            HostCustomConstructorList<
                StaticAutoShutdownAllSignificant,
                HostCustomConstructorListEnd,
            >,
        >,
    >;
}

pub(super) struct StaticFlagSchema;
pub(super) type StaticFlag<A> = HostCustomType<StaticFlagSchema, One<A>>;
pub(super) struct StaticFlagStrategy;
pub(super) struct StaticFlagStrategyField0;
impl HostCustomField for StaticFlagStrategyField0 {
    const LABEL: Option<&'static str> = None;
    type Type = StaticStrategy;
}
impl HostCustomConstructorDefinition for StaticFlagStrategy {
    const NAME: &'static str = "Strategy";
    type Fields = HostCustomFieldList<StaticFlagStrategyField0, HostCustomFieldListEnd>;
}
pub(super) struct StaticFlagIntensity;
pub(super) struct StaticFlagIntensityField0;
impl HostCustomField for StaticFlagIntensityField0 {
    const LABEL: Option<&'static str> = None;
    type Type = BigInt;
}
impl HostCustomConstructorDefinition for StaticFlagIntensity {
    const NAME: &'static str = "Intensity";
    type Fields = HostCustomFieldList<StaticFlagIntensityField0, HostCustomFieldListEnd>;
}
pub(super) struct StaticFlagPeriod;
pub(super) struct StaticFlagPeriodField0;
impl HostCustomField for StaticFlagPeriodField0 {
    const LABEL: Option<&'static str> = None;
    type Type = BigInt;
}
impl HostCustomConstructorDefinition for StaticFlagPeriod {
    const NAME: &'static str = "Period";
    type Fields = HostCustomFieldList<StaticFlagPeriodField0, HostCustomFieldListEnd>;
}
pub(super) struct StaticFlagAutoShutdown;
pub(super) struct StaticFlagAutoShutdownField0;
impl HostCustomField for StaticFlagAutoShutdownField0 {
    const LABEL: Option<&'static str> = None;
    type Type = StaticAutoShutdown;
}
impl HostCustomConstructorDefinition for StaticFlagAutoShutdown {
    const NAME: &'static str = "AutoShutdown";
    type Fields = HostCustomFieldList<StaticFlagAutoShutdownField0, HostCustomFieldListEnd>;
}
impl HostCustomSchema for StaticFlagSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/static_supervisor";
    const NAME: &'static str = "ErlangStartFlag";
    const PARAMETER_COUNT: usize = 1;
    type Constructors = HostCustomConstructorList<
        StaticFlagStrategy,
        HostCustomConstructorList<
            StaticFlagIntensity,
            HostCustomConstructorList<
                StaticFlagPeriod,
                HostCustomConstructorList<StaticFlagAutoShutdown, HostCustomConstructorListEnd>,
            >,
        >,
    >;
}

pub(super) struct StaticPropertySchema;
pub(super) type StaticProperty<A> = HostCustomType<StaticPropertySchema, One<A>>;
pub(super) struct StaticPropertyId;
pub(super) struct StaticPropertyIdField0;
impl HostCustomField for StaticPropertyIdField0 {
    const LABEL: Option<&'static str> = None;
    type Type = BigInt;
}
impl HostCustomConstructorDefinition for StaticPropertyId {
    const NAME: &'static str = "Id";
    type Fields = HostCustomFieldList<StaticPropertyIdField0, HostCustomFieldListEnd>;
}
pub(super) struct StaticPropertyStart;
pub(super) struct StaticPropertyStartField0;
impl HostCustomField for StaticPropertyStartField0 {
    const LABEL: Option<&'static str> = None;
    type Type = StaticMfa<HostCustomTypeArgument<HostTypeIndex0>>;
}
impl HostCustomConstructorDefinition for StaticPropertyStart {
    const NAME: &'static str = "Start";
    type Fields = HostCustomFieldList<StaticPropertyStartField0, HostCustomFieldListEnd>;
}
pub(super) struct StaticPropertyRestart;
pub(super) struct StaticPropertyRestartField0;
impl HostCustomField for StaticPropertyRestartField0 {
    const LABEL: Option<&'static str> = None;
    type Type = Restart;
}
impl HostCustomConstructorDefinition for StaticPropertyRestart {
    const NAME: &'static str = "Restart";
    type Fields = HostCustomFieldList<StaticPropertyRestartField0, HostCustomFieldListEnd>;
}
pub(super) struct StaticPropertySignificant;
pub(super) struct StaticPropertySignificantField0;
impl HostCustomField for StaticPropertySignificantField0 {
    const LABEL: Option<&'static str> = None;
    type Type = bool;
}
impl HostCustomConstructorDefinition for StaticPropertySignificant {
    const NAME: &'static str = "Significant";
    type Fields = HostCustomFieldList<StaticPropertySignificantField0, HostCustomFieldListEnd>;
}
pub(super) struct StaticPropertyType;
pub(super) struct StaticPropertyTypeField0;
impl HostCustomField for StaticPropertyTypeField0 {
    const LABEL: Option<&'static str> = None;
    type Type = Atom;
}
impl HostCustomConstructorDefinition for StaticPropertyType {
    const NAME: &'static str = "Type";
    type Fields = HostCustomFieldList<StaticPropertyTypeField0, HostCustomFieldListEnd>;
}
pub(super) struct StaticPropertyShutdown;
pub(super) struct StaticPropertyShutdownField0;
impl HostCustomField for StaticPropertyShutdownField0 {
    const LABEL: Option<&'static str> = None;
    type Type = StaticTimeout;
}
impl HostCustomConstructorDefinition for StaticPropertyShutdown {
    const NAME: &'static str = "Shutdown";
    type Fields = HostCustomFieldList<StaticPropertyShutdownField0, HostCustomFieldListEnd>;
}
impl HostCustomSchema for StaticPropertySchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/static_supervisor";
    const NAME: &'static str = "ErlangChildSpecProperty";
    const PARAMETER_COUNT: usize = 1;
    type Constructors = HostCustomConstructorList<
        StaticPropertyId,
        HostCustomConstructorList<
            StaticPropertyStart,
            HostCustomConstructorList<
                StaticPropertyRestart,
                HostCustomConstructorList<
                    StaticPropertySignificant,
                    HostCustomConstructorList<
                        StaticPropertyType,
                        HostCustomConstructorList<
                            StaticPropertyShutdown,
                            HostCustomConstructorListEnd,
                        >,
                    >,
                >,
            >,
        >,
    >;
}

pub(super) struct StaticFlagsSchema;
pub(super) type StaticFlags = HostExternalType<StaticFlagsSchema>;
impl HostExternalSchema for StaticFlagsSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/static_supervisor";
    const NAME: &'static str = "ErlangStartFlags";
    const PARAMETER_COUNT: usize = 0;
}

impl NativeSchema for StaticFlagsSchema {}
pub(super) struct StaticTimeoutSchema;
pub(super) type StaticTimeout = HostExternalType<StaticTimeoutSchema>;
impl HostExternalSchema for StaticTimeoutSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/static_supervisor";
    const NAME: &'static str = "Timeout";
    const PARAMETER_COUNT: usize = 0;
}

impl NativeSchema for StaticTimeoutSchema {}
pub(super) struct StaticChildSpecSchema;
pub(super) type StaticChildSpec = HostExternalType<StaticChildSpecSchema>;
impl HostExternalSchema for StaticChildSpecSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/static_supervisor";
    const NAME: &'static str = "ErlangChildSpec";
    const PARAMETER_COUNT: usize = 0;
}

pub(super) type StartResult<A> = GleamResult<Started<A>, StartError>;
pub(super) type StaticStart<A> = HostCustomConstructorAt<
    StaticProperty<A>,
    HostCustomIndexNext<HostCustomIndex0>,
    StaticPropertyStart,
>;

pub(super) type StaticMfa<A> = HostTupleType<
    Three<Atom, Atom, HostListType<HostFunctionType<HostTypeListEnd, StartResult<A>>>>,
>;
