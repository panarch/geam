import gleam/erlang/process.{type Pid}
import gleam/otp/system.{type StatusInfo}

// The upstream actor handles GetStatus but has no public requester. This
// fixture-owned requester exercises that original handler without editing OTP.
@external(erlang, "otp_service_fixture_native", "get_status")
pub fn get_status(target: Pid) -> StatusInfo

// Delivers an untagged native message through the shared producer mailbox.
@external(erlang, "otp_service_fixture_native", "send_message")
pub fn send_message(target: Pid, message: message) -> Nil
