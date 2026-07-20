pub type Boxed(value) {
  Boxed(value)
}

pub type Phantom(value) {
  Phantom
}

fn identity(value: value) {
  value
}

fn render(_value: Int) {
  "rendered"
}

fn nested_empty(flag: Bool) {
  case flag {
    True -> [[]]
    False -> []
  }
}

const make = Boxed
const render_function = render
const empty = []
const nested = [[]]
const phantom = Phantom

fn empty_nested() -> List(List(value)) {
  empty
}

fn constant_nested() -> List(List(value)) {
  nested
}

fn prepend_twice(value, values: List(value)) {
  [value, value, ..values]
}

fn prepend_empty(values: List(List(value))) {
  prepend_twice([], values)
}

pub fn main() {
  let local_constructor = Boxed
  let local_reference = identity
  let marker = 1
  let local_closure = fn(value) {
    let _ = marker
    value
  }

  #(
    local_constructor == local_constructor,
    Boxed == Boxed,
    local_reference == local_reference,
    identity == identity,
    local_closure == local_closure,
    fn(value) { value } == fn(value) { value },
    make == make,
    empty == [],
    phantom == Phantom,
    case Ok(1) {
      Ok(value) -> value
    },
    case Ok(1), 2 {
      Ok(left), right -> left + right
    },
    [Ok(1)] == [Ok(1)],
    nested_empty(True) == [[]],
    nested_empty(False) == [],
    local_reference(4),
    local_closure(5),
    make(6) == Boxed(6),
    render_function(0) == "rendered",
    empty_nested() == [],
    constant_nested() == [[]],
    prepend_empty(empty_nested()) == [[], []],
    prepend_empty([[1]]) == [[], [], [1]],
  )
}

// geam:expect Tuple([Bool(true), Bool(false), Bool(true), Bool(true), Bool(true), Bool(false), Bool(false), Bool(true), Bool(true), Int(1), Int(3), Bool(true), Bool(true), Bool(true), Int(4), Int(5), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true), Bool(true)])
