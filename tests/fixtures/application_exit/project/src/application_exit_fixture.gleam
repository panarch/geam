@external(erlang, "native", "exit")
fn exit(status: Int) -> Nil

@external(erlang, "native", "exit_async")
fn exit_async(status: Int) -> Nil

@external(erlang, "native", "calls")
fn calls() -> Int

pub fn stop(status: Int) {
  echo "before"
  exit(status)
  echo "after"
  Nil
}

pub fn stop_async(status: Int) {
  echo "before async"
  exit_async(status)
  echo "after async"
  Nil
}

pub fn normal() {
  echo "normal"
  calls()
}

pub fn ordinary_int() {
  7
}

pub fn ordinary_error() -> Result(Int, Int) {
  Error(7)
}

pub fn fail() -> Int {
  panic as "source failure"
}

pub fn main() {
  stop(7)
}
