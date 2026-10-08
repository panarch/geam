pub type Direction {
  Before
  After
}

@external(erlang, "native", "join")
fn join(value: String, direction: Direction, index: Int) -> String

@external(erlang, "native", "later")
fn later(value: String) -> String

@external(erlang, "native", "stop")
fn stop(value: String) -> String

pub fn ordinary(value: String) -> String {
  let first = join(value, Before, 1)
  let second = join(first, After, 2)
  second
}

pub fn tail(value: String) -> String {
  join(value, Before, 3)
}

fn forward(value: String, direction: Direction) -> String {
  join(value, direction, 4)
}

pub fn nested_tail(value: String) -> String {
  let first = forward(value, Before)
  let second = join(first, After, 5)
  second
}

pub fn after_continuing(value: String) -> String {
  let first = join(value, Before, 6)
  let continued = later(first)
  let second = join(continued, After, 7)
  second
}

pub fn after_never(value: String) -> String {
  let first = join(value, Before, 8)
  let stopped = stop(first)
  join(stopped, After, 9)
}

pub fn source_caller(value: String) -> #(String, String) {
  echo value
  let result = ordinary(value)
  #(result <> "done", "caller")
}

pub fn source_string_caller(value: String) -> String {
  echo value
  ordinary(value) <> "done"
}
