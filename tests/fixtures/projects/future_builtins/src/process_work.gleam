import geam/future
import gleam/erlang/process
import gleam/erlang/reference
import gleam/io
import gleam/list

type Payload {
  Payload(List(Int), fn(Int) -> Int, reference.Reference, process.Subject(Int))
}

pub fn from_exited_creator(seed: Int) -> future.Future(Int) {
  let delivery = process.new_subject()
  let creator =
    process.spawn_unlinked(fn() {
      let owner = process.self()
      let finish = process.new_subject()
      let payload =
        Payload(
          [1, 2],
          fn(value) { value * 2 },
          reference.new(),
          process.new_subject(),
        )
      let work =
        future.map(future.ready(seed), fn(value) {
          assert !process.is_alive(owner)
          assert process.self() != owner
          io.println("callback")
          let inbox = process.new_subject()
          process.spawn_unlinked(fn() { process.send(inbox, payload) })
          let Payload(values, compute, tag, inherited) =
            process.receive_forever(inbox)
          let Payload(_, _, original_tag, _) = payload
          assert tag == original_tag
          assert process.subject_owner(inherited) == Ok(owner)
          process.send(inherited, 100)
          process.sleep(10)
          compute(value) + list.length(values)
        })
      process.send(delivery, #(work, finish))
      process.receive_forever(finish)
    })
  let monitor = process.monitor(creator)
  let #(work, finish) = process.receive_forever(delivery)
  process.send(finish, Nil)
  let assert process.ProcessDown(_, _, process.Normal) =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  work
}

fn deferred(value: Int) -> future.Future(#(process.Pid, Int)) {
  future.map(future.ready(value), fn(value) {
    let me = process.self()
    let inbox = process.new_subject()
    process.spawn_unlinked(fn() { process.send(inbox, value) })
    let value = process.receive_forever(inbox)
    process.sleep(10)
    #(me, value)
  })
}

pub fn concurrent() -> future.Future(Int) {
  use results <- future.map(future.all([deferred(20), deferred(22)]))
  let assert [#(first, left), #(second, right)] = results
  assert first != second
  assert !process.is_alive(first)
  assert !process.is_alive(second)
  left + right
}

pub opaque type Service {
  Service(process.Pid, process.Subject(Query))
}

type Query {
  Status(process.Subject(Bool))
  AwaitCancellation(process.Subject(Bool))
  Stop
}

fn serve(
  requests: process.Subject(Query),
  monitor: process.Monitor,
  cancelled: Bool,
) {
  case process.receive_forever(requests) {
    Status(reply) -> {
      process.send(reply, cancelled)
      serve(requests, monitor, cancelled)
    }
    AwaitCancellation(reply) -> {
      case cancelled {
        True -> Nil
        False -> {
          let assert process.ProcessDown(_, _, process.Killed) =
            process.new_selector()
            |> process.select_specific_monitor(monitor, fn(down) { down })
            |> process.selector_receive_forever
          Nil
        }
      }
      process.send(reply, True)
      serve(requests, monitor, True)
    }
    Stop -> Nil
  }
}

pub fn cancellable() -> #(Service, future.Future(Int)) {
  let ready = process.new_subject()
  let service =
    process.spawn_unlinked(fn() {
      let callbacks = process.new_subject()
      let requests = process.new_subject()
      process.send(ready, #(callbacks, requests))
      let callback = process.receive_forever(callbacks)
      let monitor = process.monitor(callback)
      serve(requests, monitor, False)
    })
  let #(callbacks, requests) = process.receive_forever(ready)
  let work =
    future.map(future.ready(42), fn(value) {
      process.send(callbacks, process.self())
      process.sleep_forever()
      value
    })
  #(Service(service, requests), work)
}

pub fn status(service: Service) -> Bool {
  let Service(_, requests) = service
  process.call_forever(requests, Status)
}

pub fn await_cancellation(service: Service) -> Bool {
  let Service(_, requests) = service
  process.call_forever(requests, AwaitCancellation)
}

pub fn stop(service: Service) -> Nil {
  let Service(pid, requests) = service
  let monitor = process.monitor(pid)
  process.send(requests, Stop)
  let assert process.ProcessDown(_, _, process.Normal) =
    process.new_selector()
    |> process.select_specific_monitor(monitor, fn(down) { down })
    |> process.selector_receive_forever
  Nil
}
