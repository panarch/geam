#[geam::provider(package = "example_process_service", modules = [service])]
pub struct Component;

#[geam::module(
    path = "example_process_service",
    crate_path = geam,
    profile = geam::gleam_erlang::GleamErlangHostProfile,
    component = crate::Component,
)]
mod service {
    use geam::gleam_erlang::service::{self, ProcessCall};
    use geam::provider::{BigInt, Call, HostResult, Value};
    use std::time::Duration;

    #[geam::custom]
    enum RequestError {
        Unavailable,
        TimedOut,
        TargetExited,
        InvalidTimeout,
        InvalidReply,
    }

    #[geam::function(await, profile = Profile)]
    async fn exchange<Message, Reply>(
        #[geam::call] call: &mut Call<()>,
        name: service::Name<Message>,
        destination: service::Subject<Message>,
        message: Value<Message>,
        reply: service::Subject<Reply>,
        timeout_ms: BigInt,
    ) -> HostResult<Result<Value<Reply>, RequestError>> {
        let Ok(timeout_ms) = u64::try_from(timeout_ms) else {
            return Ok(Err(RequestError::InvalidTimeout));
        };
        let request = call
            .with_call(move |call| {
                let Some(target) = call.named(&name) else {
                    return Ok::<_, geam::HostCallError>(None);
                };
                let receive =
                    call.receive_subject(reply, Some(Duration::from_millis(timeout_ms)))?;
                call.send_subject(destination, message);
                Ok(Some((target, receive)))
            })
            .await??;
        let Some((target, receive)) = request else {
            return Ok(Err(RequestError::Unavailable));
        };
        let response = receive.wait_in(call).await?;
        call.with_call(move |call| match response {
            Some(response) => Ok(call
                .restore_native::<Value<Reply>>(&response)
                .ok_or(RequestError::InvalidReply)),
            None => Ok(Err(if call.is_alive(&target) {
                RequestError::TimedOut
            } else {
                RequestError::TargetExited
            })),
        })
        .await?
    }
}
