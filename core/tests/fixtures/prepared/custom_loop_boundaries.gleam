type Item {
  Item(Int)
}

fn walk(
  items: List(Item),
  total: Int,
  fail: Bool,
  apply: fn(Int, Item) -> Int,
) {
  case items {
    [] ->
      case fail {
        True -> panic
        False -> {
          echo total
          let assert [result] = [total]
          result
        }
      }
    [head, ..tail] -> walk(tail, apply(total + 1, head), fail, apply)
  }
}

fn add(total: Int, item: Item) {
  let Item(value) = item
  total + value
}

fn relay(total: Int, item: Item) {
  add(total, item)
}

pub fn integer(count: Int, fail: Bool, connected: Bool, initial: Int) {
  let apply = case connected {
    True -> add
    False -> relay
  }
  walk(items(count, []), initial, fail, apply)
}

fn any(items: List(Item), seen: Bool, apply: fn(Bool, Item) -> Bool) {
  case items {
    [] -> seen
    [head, ..tail] -> any(tail, apply(seen, head), apply)
  }
}

pub fn boolean(bias: Int) {
  any([Item(1)], False, fn(_seen, item) {
    let Item(value) = item
    value + bias > 0
  })
}

pub fn main() {
  integer(1, False, True, 3)
}

fn items(count: Int, result: List(Item)) {
  case count {
    0 -> result
    _ -> items(count - 1, [Item(2), ..result])
  }
}
