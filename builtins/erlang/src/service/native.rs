use crate::execution::ReferenceId;
use crate::{Component, GleamErlangHostProfile, Reference, ReferenceSchema, reference};
use geam_core::host::native::NativeCall;
use geam_core::host::{
    HostCallError, HostExternal, HostProvider, HostType, HostTypeIndex0, HostTypeList,
    HostTypeListEnd, HostTypeSequence,
};
use geam_core::provider::advanced::NativeValue;

/// Removes this process's monitor and flushes its queued DOWN message.
/// The registration's first native conversion target must be `Reference`.
pub fn demonitor<'call, Profile, Provider, Return, Rest>(
    call: &mut NativeCall<'call, Profile, Provider, Return, HostTypeList<Reference, Rest>>,
    monitor: HostExternal<'call, super::types::Monitor>,
) -> Result<(), HostCallError>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Rest: HostTypeSequence,
{
    call.call().require_execution_unit().and_then(|owner| {
        let reference = call
            .call()
            .external_payload_with::<Component<Profile>, super::types::MonitorSchema, HostTypeListEnd>(
                monitor,
            )
            .clone();
        let id = reference_id(call, &reference)?;
        Profile::erlang_execution(call.call().execution_state()).demonitor(owner.id(), id);
        Ok(())
    })
}

/// Cancels a timer and returns its remaining whole milliseconds.
/// `None` means it was delivered, cancelled, or its process destination ended.
/// The registration's first native conversion target must be `Reference`.
pub fn cancel_timer<'call, Profile, Provider, Return, Rest>(
    call: &mut NativeCall<'call, Profile, Provider, Return, HostTypeList<Reference, Rest>>,
    timer: HostExternal<'call, super::types::Timer>,
) -> Result<Option<u128>, HostCallError>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Rest: HostTypeSequence,
{
    let reference = call
        .call()
        .external_payload_with::<Component<Profile>, super::types::TimerSchema, HostTypeListEnd>(
            timer,
        )
        .clone();
    let id = reference_id(call, &reference)?;
    let now = call.call().clock().now();
    Ok(Profile::erlang_execution(call.call().execution_state()).cancel_timer(id, now))
}

fn reference_id<Profile, Provider, Return, Rest>(
    call: &mut NativeCall<'_, Profile, Provider, Return, HostTypeList<Reference, Rest>>,
    value: &NativeValue,
) -> Result<ReferenceId, HostCallError>
where
    Profile: GleamErlangHostProfile,
    Provider: HostProvider<Profile>,
    Return: HostType,
    Rest: HostTypeSequence,
{
    call.convert::<HostTypeIndex0>(value)
        .and_then(|reference| {
            match &*call
                .call()
                .external_payload_with::<Component<Profile>, ReferenceSchema, HostTypeListEnd>(
                    reference,
                ) {
                reference::Payload::Identity(id) => Some(*id),
                reference::Payload::View(_) => None,
            }
        })
        .ok_or_else(|| geam_core::HostFailure::new("operation requires a reference").into())
}
