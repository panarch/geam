import gleam/dynamic/decode
import gleam/erlang/atom
import gleam/erlang/process
import gleam/otp/factory_supervisor as factory
import otp_service_support as support

pub fn main() {
  let name = process.new_name("malformed_count_reply")
  let assert Ok(Nil) = process.register(process.self(), name)
  let supervisor: factory.Supervisor(Int, Nil) = factory.get_by_name(name)
  let caller =
    process.spawn_unlinked(fn() {
      let _ = factory.count_children(supervisor)
      Nil
    })
  let request =
    process.new_selector()
    |> process.select_record(atom.create("factory"), 4, fn(message) { message })
    |> process.selector_receive_forever
  let assert Ok(tag) =
    decode.run(request, {
      use tag <- decode.field(4, decode.dynamic)
      decode.success(tag)
    })
  support.send_message(caller, #(tag, "not a count"))
  let monitor = process.monitor(caller)
  let _ =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  let assert False = process.is_alive(caller)
  Nil
}
