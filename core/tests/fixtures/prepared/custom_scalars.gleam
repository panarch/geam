pub type Entry {
  Credit(Int)
  Debit(Int)
  Ignored
}

fn adjust(total: Int, entry: Entry) -> Int {
  case entry {
    Credit(amount) -> total + amount
    Debit(amount) -> total - amount
    Ignored -> total
  }
}

pub fn credit(amount: Int, total: Int) {
  adjust(total, Credit(amount))
}

pub fn debit(amount: Int, total: Int) {
  adjust(total, Debit(amount))
}

pub fn ignored(total: Int) {
  adjust(total, Ignored)
}

pub type Data {
  Data(left: Int, right: Int, enabled: Bool)
}

fn select(data: Data) -> Int {
  case data {
    Data(3, right, True) if right > 0 -> right * 2
    Data(left, 9, True) -> left - 1
    Data(_, right, _) -> right
  }
}

pub fn guarded(left: Int, right: Int, enabled: Bool) {
  select(Data(left, right, enabled))
}

fn retain(data: Data) -> Int {
  let Data(left, _, _) as alias = data
  left + data.right + alias.right
}

pub fn aliased(left: Int, right: Int) {
  retain(Data(left, right, True))
}

fn both(left: Data, right: Data) -> Int {
  case left, right {
    Data(a, _, True), Data(_, b, True) -> a + b
    _, _ -> left.right - right.left
  }
}

pub fn multiple(a: Int, b: Int, enabled: Bool) {
  both(Data(a, 7, enabled), Data(11, b, True))
}

fn is_enabled(data: Data) -> Bool {
  case data {
    Data(3, _, True) -> data.right > 0
    Data(_, _, enabled) -> enabled
  }
}

pub fn boolean(left: Int, right: Int, enabled: Bool) {
  is_enabled(Data(left, right, enabled))
}

fn getters(data: Data) -> Int {
  case data.enabled {
    True -> data.left / 3 + data.right % 4
    False -> data.left - data.right
  }
}

pub fn fields(left: Int, right: Int, enabled: Bool) {
  getters(Data(left, right, enabled))
}

fn repeat(data: Data, count: Int, total: Int) -> Int {
  case count > 0 {
    True -> repeat(data, count - 1, total + data.left)
    False -> total
  }
}

pub fn repeated(value: Int, count: Int) {
  repeat(Data(value, 0, True), count, 0)
}

fn asserted(data: Data) -> Int {
  let assert Data(3, 9, True) = data
  12
}

pub fn assertion(left: Int, right: Int, enabled: Bool) {
  asserted(Data(left, right, enabled))
}

fn stop(data: Data) -> Int {
  case data {
    Data(3, _, _) -> panic as "custom stop"
    Data(_, right, _) -> right
  }
}

pub fn panic_case(left: Int, right: Int) {
  stop(Data(left, right, True))
}

type Nested {
  Nested(Data)
}

fn nested_read(input: Nested) -> Int {
  case input {
    Nested(Data(left, _, _)) -> left
  }
}

pub fn nested(value: Int) {
  nested_read(Nested(Data(value, 0, True)))
}

pub fn main() {
  credit(11, 2)
  + debit(7, 20)
  + ignored(5)
  + guarded(3, 4, True)
  + aliased(2, 5)
  + multiple(4, 6, True)
  + fields(12, 7, True)
  + repeated(3, 4)
  + assertion(3, 9, True)
  + nested(8)
}
