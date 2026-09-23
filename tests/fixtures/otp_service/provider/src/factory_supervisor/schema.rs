use crate::schema::{NativeSchema, Restart, StartError, StartResult};
use crate::{One, Three, Two};
use geam::gleam_erlang::{Atom, Name};
use geam::host::{
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostExternalSchema, HostExternalType, HostFunctionType, HostListType,
    HostTupleType, HostTypeIndex0, HostTypeIndexNext,
};
use geam::provider::BigInt;

pub(super) struct FlagsSchema;
pub(super) type Flags = HostExternalType<FlagsSchema>;
impl HostExternalSchema for FlagsSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/factory_supervisor";
    const NAME: &'static str = "ErlangStartFlags";
    const PARAMETER_COUNT: usize = 0;
}

impl NativeSchema for FlagsSchema {}

pub(super) struct TimeoutSchema;
pub(super) type Timeout = HostExternalType<TimeoutSchema>;
impl HostExternalSchema for TimeoutSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/factory_supervisor";
    const NAME: &'static str = "Timeout";
    const PARAMETER_COUNT: usize = 0;
}

impl NativeSchema for TimeoutSchema {}

pub(super) struct ChildSpecSchema;
pub(super) type ChildSpec = HostExternalType<ChildSpecSchema>;
impl HostExternalSchema for ChildSpecSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/factory_supervisor";
    const NAME: &'static str = "ErlangChildSpec";
    const PARAMETER_COUNT: usize = 0;
}

pub(super) struct HandleSchema;
pub(super) type Handle = HostExternalType<HandleSchema>;
impl HostExternalSchema for HandleSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/factory_supervisor";
    const NAME: &'static str = "SupervisorHandle";
    const PARAMETER_COUNT: usize = 0;
}

pub(super) struct MessageSchema;
pub(super) type Message<A, B> = HostExternalType<MessageSchema, Two<A, B>>;
impl HostExternalSchema for MessageSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/factory_supervisor";
    const NAME: &'static str = "Message";
    const PARAMETER_COUNT: usize = 2;
}

impl NativeSchema for MessageSchema {}

pub(super) struct SupervisorNameSchema;
pub(super) type SupervisorName<A, B> = HostCustomType<SupervisorNameSchema, Two<A, B>>;

pub(super) struct SupervisorNameLocal;

pub(super) struct SupervisorNameLocalField0;
impl HostCustomField for SupervisorNameLocalField0 {
    const LABEL: Option<&'static str> = None;
    type Type = Name<
        Message<
            HostCustomTypeArgument<HostTypeIndex0>,
            HostCustomTypeArgument<HostTypeIndexNext<HostTypeIndex0>>,
        >,
    >;
}

impl HostCustomConstructorDefinition for SupervisorNameLocal {
    const NAME: &'static str = "Local";
    type Fields = HostCustomFieldList<SupervisorNameLocalField0, HostCustomFieldListEnd>;
}

impl HostCustomSchema for SupervisorNameSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/factory_supervisor";
    const NAME: &'static str = "ErlangSupervisorName";
    const PARAMETER_COUNT: usize = 2;
    type Constructors =
        HostCustomConstructorList<SupervisorNameLocal, HostCustomConstructorListEnd>;
}

pub(super) struct StrategySchema;
pub(super) type Strategy = HostCustomType<StrategySchema>;

pub(super) struct StrategySimpleOneForOne;

impl HostCustomConstructorDefinition for StrategySimpleOneForOne {
    const NAME: &'static str = "SimpleOneForOne";
    type Fields = HostCustomFieldListEnd;
}

impl HostCustomSchema for StrategySchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/factory_supervisor";
    const NAME: &'static str = "Strategy";
    const PARAMETER_COUNT: usize = 0;
    type Constructors =
        HostCustomConstructorList<StrategySimpleOneForOne, HostCustomConstructorListEnd>;
}

pub(super) struct FlagSchema;
pub(super) type Flag<A> = HostCustomType<FlagSchema, One<A>>;

pub(super) struct FlagStrategy;

pub(super) struct FlagStrategyField0;
impl HostCustomField for FlagStrategyField0 {
    const LABEL: Option<&'static str> = None;
    type Type = Strategy;
}

impl HostCustomConstructorDefinition for FlagStrategy {
    const NAME: &'static str = "Strategy";
    type Fields = HostCustomFieldList<FlagStrategyField0, HostCustomFieldListEnd>;
}

pub(super) struct FlagIntensity;

pub(super) struct FlagIntensityField0;
impl HostCustomField for FlagIntensityField0 {
    const LABEL: Option<&'static str> = None;
    type Type = BigInt;
}

impl HostCustomConstructorDefinition for FlagIntensity {
    const NAME: &'static str = "Intensity";
    type Fields = HostCustomFieldList<FlagIntensityField0, HostCustomFieldListEnd>;
}

pub(super) struct FlagPeriod;

pub(super) struct FlagPeriodField0;
impl HostCustomField for FlagPeriodField0 {
    const LABEL: Option<&'static str> = None;
    type Type = BigInt;
}

impl HostCustomConstructorDefinition for FlagPeriod {
    const NAME: &'static str = "Period";
    type Fields = HostCustomFieldList<FlagPeriodField0, HostCustomFieldListEnd>;
}

impl HostCustomSchema for FlagSchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/factory_supervisor";
    const NAME: &'static str = "ErlangStartFlag";
    const PARAMETER_COUNT: usize = 1;
    type Constructors = HostCustomConstructorList<
        FlagStrategy,
        HostCustomConstructorList<
            FlagIntensity,
            HostCustomConstructorList<FlagPeriod, HostCustomConstructorListEnd>,
        >,
    >;
}

pub(super) struct PropertySchema;
pub(super) type Property<A, B> = HostCustomType<PropertySchema, Two<A, B>>;

pub(super) struct PropertyId;

pub(super) struct PropertyIdField0;
impl HostCustomField for PropertyIdField0 {
    const LABEL: Option<&'static str> = None;
    type Type = BigInt;
}

impl HostCustomConstructorDefinition for PropertyId {
    const NAME: &'static str = "Id";
    type Fields = HostCustomFieldList<PropertyIdField0, HostCustomFieldListEnd>;
}

pub(super) struct PropertyStart;

pub(super) struct PropertyStartField0;
impl HostCustomField for PropertyStartField0 {
    const LABEL: Option<&'static str> = None;
    type Type = Mfa<
        HostCustomTypeArgument<HostTypeIndex0>,
        HostCustomTypeArgument<HostTypeIndexNext<HostTypeIndex0>>,
    >;
}

impl HostCustomConstructorDefinition for PropertyStart {
    const NAME: &'static str = "Start";
    type Fields = HostCustomFieldList<PropertyStartField0, HostCustomFieldListEnd>;
}

pub(super) struct PropertyRestart;

pub(super) struct PropertyRestartField0;
impl HostCustomField for PropertyRestartField0 {
    const LABEL: Option<&'static str> = None;
    type Type = Restart;
}

impl HostCustomConstructorDefinition for PropertyRestart {
    const NAME: &'static str = "Restart";
    type Fields = HostCustomFieldList<PropertyRestartField0, HostCustomFieldListEnd>;
}

pub(super) struct PropertyType;

pub(super) struct PropertyTypeField0;
impl HostCustomField for PropertyTypeField0 {
    const LABEL: Option<&'static str> = None;
    type Type = Atom;
}

impl HostCustomConstructorDefinition for PropertyType {
    const NAME: &'static str = "Type";
    type Fields = HostCustomFieldList<PropertyTypeField0, HostCustomFieldListEnd>;
}

pub(super) struct PropertyShutdown;

pub(super) struct PropertyShutdownField0;
impl HostCustomField for PropertyShutdownField0 {
    const LABEL: Option<&'static str> = None;
    type Type = Timeout;
}

impl HostCustomConstructorDefinition for PropertyShutdown {
    const NAME: &'static str = "Shutdown";
    type Fields = HostCustomFieldList<PropertyShutdownField0, HostCustomFieldListEnd>;
}

impl HostCustomSchema for PropertySchema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/factory_supervisor";
    const NAME: &'static str = "ErlangChildSpecProperty";
    const PARAMETER_COUNT: usize = 2;
    type Constructors = HostCustomConstructorList<
        PropertyId,
        HostCustomConstructorList<
            PropertyStart,
            HostCustomConstructorList<
                PropertyRestart,
                HostCustomConstructorList<
                    PropertyType,
                    HostCustomConstructorList<PropertyShutdown, HostCustomConstructorListEnd>,
                >,
            >,
        >,
    >;
}

pub(super) struct Result2Schema;
pub(super) type Result2<A, B, C> = HostCustomType<Result2Schema, Three<A, B, C>>;

pub(super) struct Result2Ok;

pub(super) struct Result2OkField0;
impl HostCustomField for Result2OkField0 {
    const LABEL: Option<&'static str> = None;
    type Type = HostCustomTypeArgument<HostTypeIndex0>;
}

pub(super) struct Result2OkField1;
impl HostCustomField for Result2OkField1 {
    const LABEL: Option<&'static str> = None;
    type Type = HostCustomTypeArgument<HostTypeIndexNext<HostTypeIndex0>>;
}

impl HostCustomConstructorDefinition for Result2Ok {
    const NAME: &'static str = "Ok";
    type Fields = HostCustomFieldList<
        Result2OkField0,
        HostCustomFieldList<Result2OkField1, HostCustomFieldListEnd>,
    >;
}

pub(super) struct Result2Error;

pub(super) struct Result2ErrorField0;
impl HostCustomField for Result2ErrorField0 {
    const LABEL: Option<&'static str> = None;
    type Type = HostCustomTypeArgument<HostTypeIndexNext<HostTypeIndexNext<HostTypeIndex0>>>;
}

impl HostCustomConstructorDefinition for Result2Error {
    const NAME: &'static str = "Error";
    type Fields = HostCustomFieldList<Result2ErrorField0, HostCustomFieldListEnd>;
}

impl HostCustomSchema for Result2Schema {
    const PACKAGE: &'static str = "gleam_otp";
    const MODULE: &'static str = "gleam/otp/internal/result2";
    const NAME: &'static str = "Result2";
    const PARAMETER_COUNT: usize = 3;
    type Constructors = HostCustomConstructorList<
        Result2Ok,
        HostCustomConstructorList<Result2Error, HostCustomConstructorListEnd>,
    >;
}

pub(super) type Mfa<A, B> =
    HostTupleType<Three<Atom, Atom, HostListType<HostFunctionType<One<A>, StartResult<B>>>>>;
pub(super) type Start<A, B> =
    HostCustomConstructorAt<Property<A, B>, HostCustomIndexNext<HostCustomIndex0>, PropertyStart>;
pub(super) type Local<A, B> =
    HostCustomConstructorAt<SupervisorName<A, B>, HostCustomIndex0, SupervisorNameLocal>;
pub(super) type ChildResult<B> = Result2<geam::gleam_erlang::Pid, B, StartError>;
