use crate::schema::{Atom, Node, NodeSchema};
use crate::{Component, GleamErlangHostProfile};
use geam_core::host::{
    HostCall, HostCallCompletion, HostCallError, HostConstructions, HostCustomConstructorAt,
    HostCustomConstructorDefinition, HostCustomConstructorList, HostCustomConstructorListEnd,
    HostCustomFieldListEnd, HostCustomIndex0, HostCustomIndexNext, HostCustomSchema,
    HostCustomType, HostExternal, HostListType, HostProviderModule, HostRegistrationError,
    HostTypeIndex0, HostTypeList, HostTypeListEnd,
};
use geam_core::provider::advanced::NativeValue;
use geam_stdlib::provider_support::{GleamError, GleamResult};

struct ConnectErrorSchema;
struct FailedToConnect;
struct LocalNodeIsNotAlive;
type ConnectError = HostCustomType<ConnectErrorSchema>;
type NotAlive = HostCustomConstructorAt<
    ConnectError,
    HostCustomIndexNext<HostCustomIndex0>,
    LocalNodeIsNotAlive,
>;

impl HostCustomConstructorDefinition for FailedToConnect {
    const NAME: &'static str = "FailedToConnect";
    type Fields = HostCustomFieldListEnd;
}
impl HostCustomConstructorDefinition for LocalNodeIsNotAlive {
    const NAME: &'static str = "LocalNodeIsNotAlive";
    type Fields = HostCustomFieldListEnd;
}
impl HostCustomSchema for ConnectErrorSchema {
    const PACKAGE: &'static str = "gleam_erlang";
    const MODULE: &'static str = "gleam/erlang/node";
    const NAME: &'static str = "ConnectError";
    const PARAMETER_COUNT: usize = 0;
    type Constructors = HostCustomConstructorList<
        FailedToConnect,
        HostCustomConstructorList<LocalNodeIsNotAlive, HostCustomConstructorListEnd>,
    >;
}

pub(crate) fn host_provider<Profile: GleamErlangHostProfile>()
-> Result<HostProviderModule<Profile>, HostRegistrationError> {
    HostProviderModule::new("gleam_erlang", "gleam/erlang/node")
        .and_then(|module| module.with_external_type::<Component<Profile>, NodeSchema>())
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (), Node, _>("self", current::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (), HostListType<Node>, _>("visible", visible::<Profile>))
        .and_then(|module| module.with_scoped_function_and_constructions::<Component<Profile>, (Atom,), GleamResult<Node, ConnectError>, HostTypeList<ConnectError, HostTypeListEnd>, _>("connect", connect::<Profile>))
        .and_then(|module| module.with_scoped_function::<Component<Profile>, (Node,), Atom, _>("name", name::<Profile>))
}

fn current<'call, Profile: GleamErlangHostProfile>(
    mut call: HostCall<'call, Profile, Component<Profile>, Node>,
) -> Result<HostCallCompletion<'call, Node>, HostCallError> {
    let value = call.create_external(NativeValue::symbol("nonode@nohost"));
    Ok(call.return_value(value))
}

fn visible<'call, Profile: GleamErlangHostProfile>(
    call: HostCall<'call, Profile, Component<Profile>, HostListType<Node>>,
) -> Result<HostCallCompletion<'call, HostListType<Node>>, HostCallError> {
    Ok(call.return_list([]))
}

fn connect<'call, Profile: GleamErlangHostProfile>(
    mut call: HostCall<'call, Profile, Component<Profile>, GleamResult<Node, ConnectError>>,
    constructions: HostConstructions<'call, HostTypeList<ConnectError, HostTypeListEnd>>,
    _node: HostExternal<'call, Atom>,
) -> Result<HostCallCompletion<'call, GleamResult<Node, ConnectError>>, HostCallError> {
    let error = call.construct_custom::<NotAlive>(constructions.at::<HostTypeIndex0>(), ());
    Ok(call.return_custom::<GleamError<Node, ConnectError>>((error, ())))
}

fn name<'call, Profile: GleamErlangHostProfile>(
    mut call: HostCall<'call, Profile, Component<Profile>, Atom>,
    node: HostExternal<'call, Node>,
) -> Result<HostCallCompletion<'call, Atom>, HostCallError> {
    let value = call.external_payload(node).clone();
    let value = call.create_external(value);
    Ok(call.return_value(value))
}
