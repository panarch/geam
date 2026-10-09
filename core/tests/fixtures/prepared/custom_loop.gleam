pub type Item {
  Add(Int)
  Subtract(Int)
  Skip
}

fn fold(items: List(Item), total: Int, apply: fn(Int, Item) -> Int) -> Int {
  case items {
    [] -> total
    [head, ..tail] -> fold(tail, apply(total, head), apply)
  }
}

fn adjust(total: Int, item: Item) -> Int {
  case item {
    Add(value) -> total + value
    Subtract(value) -> total - value
    Skip -> total
  }
}

fn double(total: Int, item: Item) -> Int {
  case item {
    Add(value) -> total + value * 2
    Subtract(value) -> total - value * 2
    Skip -> total
  }
}

pub fn run(seed: Int, value: Int) -> Int {
  fold([Add(value), Subtract(value), Add(7), Skip], seed, adjust)
}

pub fn chosen(seed: Int, doubled: Bool) -> Int {
  let apply = case doubled {
    True -> double
    False -> adjust
  }
  fold([Add(7), Subtract(2), Skip], seed, apply)
}

pub fn captured(seed: Int, bias: Int, enabled: Bool) -> Int {
  fold([Add(7), Subtract(2), Skip], seed, fn(total, item) {
    case enabled {
      True ->
        case item {
          Add(value) -> total + value + bias
          Subtract(value) -> total - value - bias
          Skip -> total + bias
        }
      False -> total
    }
  })
}

fn any(items: List(Item), seen: Bool, apply: fn(Bool, Item) -> Bool) -> Bool {
  case items {
    [] -> seen
    [head, ..tail] -> any(tail, apply(seen, head), apply)
  }
}

pub fn boolean(initial: Bool, value: Int) -> Bool {
  any([Skip, Add(value), Subtract(0)], initial, fn(seen, item) {
    case item {
      Add(n) ->
        case n > 0 {
          True -> True
          False -> seen
        }
      _ -> seen
    }
  })
}

fn asserted(total: Int, item: Item) -> Int {
  let assert Add(value) = item
  total + value
}

pub fn assertion(value: Int) -> Int {
  fold([Add(1), Subtract(value)], 0, asserted)
}

fn stop(total: Int, item: Item) -> Int {
  case item {
    Add(_) -> panic as "loop callback stop"
    _ -> total
  }
}

pub fn empty() -> Int {
  fold([], 19, stop)
}

pub fn panic_case(value: Int) -> Int {
  fold([Skip, Add(value)], 0, stop)
}

fn extra(value: Int) -> Int {
  value + 1
}

fn non_leaf(total: Int, item: Item) -> Int {
  case item {
    Add(value) -> total + extra(value)
    _ -> total
  }
}

pub fn unsupported(value: Int) -> Int {
  fold([Add(value), Skip], 0, non_leaf)
}

pub fn custom_capture(seed: Int, bias: Int) -> Int {
  let modifier = Add(bias)
  fold([Add(7), Subtract(2), Skip], seed, fn(total, item) {
    case modifier {
      Add(bias) ->
        case item {
          Add(value) -> total + value + bias
          Subtract(value) -> total - value
          Skip -> total
        }
      _ -> total
    }
  })
}

fn repeat_items(count: Int, value: Int, items: List(Item)) -> List(Item) {
  case count > 0 {
    True -> repeat_items(count - 1, value, [Add(value), ..items])
    False -> items
  }
}

pub fn repeated(count: Int, value: Int, seed: Int) -> Int {
  fold(repeat_items(count, value, []), seed, adjust)
}

fn fold_adjusted(
  items: List(Item),
  total: Int,
  apply: fn(Int, Item) -> Int,
) -> Int {
  case items {
    [] -> total
    [head, ..tail] -> fold_adjusted(tail, apply(total + 1, head), apply)
  }
}

pub fn caller_overflow(seed: Int) -> Int {
  fold_adjusted([Subtract(2), Add(3)], seed, adjust)
}

pub type Decision {
  Decision(Int, Bool)
}

fn fold_decisions(
  items: List(Decision),
  total: Int,
  apply: fn(Int, Decision) -> Int,
) -> Int {
  case items {
    [] -> total
    [head, ..tail] -> fold_decisions(tail, apply(total, head), apply)
  }
}

fn gated(total: Int, item: Decision) -> Int {
  case item {
    Decision(amount, True) as original if amount > 0 -> {
      let Decision(other, _) = original
      total + amount + other
    }
    _ -> total
  }
}

pub fn guarded(seed: Int, value: Int, enabled: Bool) -> Int {
  fold_decisions([Decision(value, enabled), Decision(3, True)], seed, gated)
}

pub fn main() -> Int {
  run(2, 9) + chosen(1, True) + captured(0, 3, True) + empty() + unsupported(2)
}

pub type Marker {
  Marker
}

fn fold_markers(
  items: List(Marker),
  total: Int,
  apply: fn(Int, Marker) -> Int,
) -> Int {
  case items {
    [] -> total
    [head, ..tail] -> fold_markers(tail, apply(total, head), apply)
  }
}

fn bump_marker(total: Int, _marker: Marker) -> Int {
  extra(total) + 2
}

pub fn markers(seed: Int) -> Int {
  fold_markers([Marker, Marker], seed, bump_marker)
}
