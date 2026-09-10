use crate::schema::{Monitor, Name, Pid, Port};
use geam_core::host::{
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostTypeIndex0, HostTypeList, HostTypeListEnd,
};
use geam_stdlib::provider_support::Dynamic;

pub(super) type Subject<A> = HostCustomType<SubjectSchema, HostTypeList<A, HostTypeListEnd>>;
pub(super) type OrdinarySubject<A> =
    HostCustomConstructorAt<Subject<A>, HostCustomIndex0, SubjectConstructor>;
pub(super) type NamedSubject<A> = HostCustomConstructorAt<
    Subject<A>,
    HostCustomIndexNext<HostCustomIndex0>,
    NamedSubjectConstructor,
>;
pub(super) type ExitReason = HostCustomType<ExitReasonSchema>;
pub(super) type NormalReason = HostCustomConstructorAt<ExitReason, HostCustomIndex0, Normal>;
pub(super) type KilledReason =
    HostCustomConstructorAt<ExitReason, HostCustomIndexNext<HostCustomIndex0>, Killed>;
pub(super) type AbnormalReason = HostCustomConstructorAt<
    ExitReason,
    HostCustomIndexNext<HostCustomIndexNext<HostCustomIndex0>>,
    Abnormal,
>;
pub(super) type Down = HostCustomType<DownSchema>;
pub(super) type ProcessFlag = HostCustomType<ProcessFlagSchema>;
pub(super) type KillFlag = HostCustomType<KillFlagSchema>;

pub(super) struct SubjectSchema;
pub(super) struct SubjectConstructor;
pub(super) struct NamedSubjectConstructor;
pub(super) struct OwnerField;
pub(super) struct TagField;
pub(super) struct NameField;

impl HostCustomField for OwnerField {
    const LABEL: Option<&'static str> = Some("owner");
    type Type = Pid;
}
impl HostCustomField for TagField {
    const LABEL: Option<&'static str> = Some("tag");
    type Type = Dynamic;
}
impl HostCustomField for NameField {
    const LABEL: Option<&'static str> = Some("name");
    type Type = Name<HostCustomTypeArgument<HostTypeIndex0>>;
}
impl HostCustomConstructorDefinition for SubjectConstructor {
    const NAME: &'static str = "Subject";
    type Fields =
        HostCustomFieldList<OwnerField, HostCustomFieldList<TagField, HostCustomFieldListEnd>>;
}
impl HostCustomConstructorDefinition for NamedSubjectConstructor {
    const NAME: &'static str = "NamedSubject";
    type Fields = HostCustomFieldList<NameField, HostCustomFieldListEnd>;
}
impl HostCustomSchema for SubjectSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/process";
    const NAME: &'static str = "Subject";
    const PARAMETER_COUNT: usize = 1;
    type Constructors = HostCustomConstructorList<
        SubjectConstructor,
        HostCustomConstructorList<NamedSubjectConstructor, HostCustomConstructorListEnd>,
    >;
}

pub(super) struct ExitReasonSchema;
pub(super) struct Normal;
pub(super) struct Killed;
pub(super) struct Abnormal;
pub(super) struct AbnormalReasonField;
impl HostCustomField for AbnormalReasonField {
    const LABEL: Option<&'static str> = Some("reason");
    type Type = Dynamic;
}
impl HostCustomConstructorDefinition for Normal {
    const NAME: &'static str = "Normal";
    type Fields = HostCustomFieldListEnd;
}
impl HostCustomConstructorDefinition for Killed {
    const NAME: &'static str = "Killed";
    type Fields = HostCustomFieldListEnd;
}
impl HostCustomConstructorDefinition for Abnormal {
    const NAME: &'static str = "Abnormal";
    type Fields = HostCustomFieldList<AbnormalReasonField, HostCustomFieldListEnd>;
}
impl HostCustomSchema for ExitReasonSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/process";
    const NAME: &'static str = "ExitReason";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<
        Normal,
        HostCustomConstructorList<
            Killed,
            HostCustomConstructorList<Abnormal, HostCustomConstructorListEnd>,
        >,
    >;
}

pub(super) struct DownSchema;
pub(super) struct ProcessDown;
pub(super) struct PortDown;
pub(super) struct MonitorField;
pub(super) struct PidField;
pub(super) struct PortField;
pub(super) struct ExitReasonField;
impl HostCustomField for MonitorField {
    const LABEL: Option<&'static str> = Some("monitor");
    type Type = Monitor;
}
impl HostCustomField for PidField {
    const LABEL: Option<&'static str> = Some("pid");
    type Type = Pid;
}
impl HostCustomField for PortField {
    const LABEL: Option<&'static str> = Some("port");
    type Type = Port;
}
impl HostCustomField for ExitReasonField {
    const LABEL: Option<&'static str> = Some("reason");
    type Type = ExitReason;
}
impl HostCustomConstructorDefinition for ProcessDown {
    const NAME: &'static str = "ProcessDown";
    type Fields = HostCustomFieldList<
        MonitorField,
        HostCustomFieldList<PidField, HostCustomFieldList<ExitReasonField, HostCustomFieldListEnd>>,
    >;
}
impl HostCustomConstructorDefinition for PortDown {
    const NAME: &'static str = "PortDown";
    type Fields = HostCustomFieldList<
        MonitorField,
        HostCustomFieldList<
            PortField,
            HostCustomFieldList<ExitReasonField, HostCustomFieldListEnd>,
        >,
    >;
}
impl HostCustomSchema for DownSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/process";
    const NAME: &'static str = "Down";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<
        ProcessDown,
        HostCustomConstructorList<PortDown, HostCustomConstructorListEnd>,
    >;
}

pub(super) struct ProcessFlagSchema;
pub(super) struct Process;
impl HostCustomConstructorDefinition for Process {
    const NAME: &'static str = "Process";
    type Fields = HostCustomFieldListEnd;
}
impl HostCustomSchema for ProcessFlagSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/process";
    const NAME: &'static str = "ProcessMonitorFlag";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<Process, HostCustomConstructorListEnd>;
}

pub(super) struct KillFlagSchema;
pub(super) struct Kill;
impl HostCustomConstructorDefinition for Kill {
    const NAME: &'static str = "Kill";
    type Fields = HostCustomFieldListEnd;
}
impl HostCustomSchema for KillFlagSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/process";
    const NAME: &'static str = "KillFlag";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<Kill, HostCustomConstructorListEnd>;
}
