use crate::schema::{Monitor, Name, Pid, Port};
use geam_core::host::{
    HostCustomConstructorAt, HostCustomConstructorDefinition, HostCustomConstructorList,
    HostCustomConstructorListEnd, HostCustomField, HostCustomFieldList, HostCustomFieldListEnd,
    HostCustomIndex0, HostCustomIndexNext, HostCustomSchema, HostCustomType,
    HostCustomTypeArgument, HostTypeIndex0, HostTypeList, HostTypeListEnd,
};
use geam_stdlib::provider_support::Dynamic;

pub type Subject<A> = HostCustomType<SubjectSchema, HostTypeList<A, HostTypeListEnd>>;
pub type OrdinarySubject<A> =
    HostCustomConstructorAt<Subject<A>, HostCustomIndex0, SubjectConstructor>;
pub type NamedSubject<A> = HostCustomConstructorAt<
    Subject<A>,
    HostCustomIndexNext<HostCustomIndex0>,
    NamedSubjectConstructor,
>;
pub type ExitReason = HostCustomType<ExitReasonSchema>;
pub type NormalReason = HostCustomConstructorAt<ExitReason, HostCustomIndex0, Normal>;
pub type KilledReason =
    HostCustomConstructorAt<ExitReason, HostCustomIndexNext<HostCustomIndex0>, Killed>;
pub type AbnormalReason = HostCustomConstructorAt<
    ExitReason,
    HostCustomIndexNext<HostCustomIndexNext<HostCustomIndex0>>,
    Abnormal,
>;
pub type Down = HostCustomType<DownSchema>;
pub type ProcessFlag = HostCustomType<ProcessFlagSchema>;
pub type KillFlag = HostCustomType<KillFlagSchema>;

pub struct SubjectSchema;
pub struct SubjectConstructor;
pub struct NamedSubjectConstructor;
pub struct OwnerField;
pub struct TagField;
pub struct NameField;

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
    const SHARED: bool = true;
    type Constructors = HostCustomConstructorList<
        SubjectConstructor,
        HostCustomConstructorList<NamedSubjectConstructor, HostCustomConstructorListEnd>,
    >;
}

pub struct ExitReasonSchema;
pub struct Normal;
pub struct Killed;
pub struct Abnormal;
pub struct AbnormalReasonField;
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

pub struct DownSchema;
pub struct ProcessDown;
pub struct PortDown;
pub struct MonitorField;
pub struct PidField;
pub struct PortField;
pub struct ExitReasonField;
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

pub struct ProcessFlagSchema;
pub struct Process;
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

pub struct KillFlagSchema;
pub struct Kill;
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
