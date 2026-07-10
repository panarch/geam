fn ones() {
  [1]
}

fn twos() {
  [2]
}

fn get_ones() {
  ones
}

fn pair() {
  #(ones, twos)
}

fn first(function: fn() -> List(Int)) {
  case function() {
    [value, ..] -> value
    _ -> 0
  }
}

pub fn main() {
  let local = ones
  let bool_true = case True {
    True -> ones
    False -> twos
  }
  let bool_false = case False {
    True -> ones
    False -> twos
  }
  let int_hit = case 1 {
    1 -> ones
    _ -> twos
  }
  let int_fallback = case 0 {
    1 -> ones
    _ -> twos
  }
  let string_hit = case "hit" {
    "hit" -> ones
    _ -> twos
  }
  let string_fallback = case "miss" {
    "hit" -> ones
    _ -> twos
  }
  let float_hit = case 1.0 {
    1.0 -> ones
    _ -> twos
  }
  let float_fallback = case 0.0 {
    1.0 -> ones
    _ -> twos
  }
  let block = {
    let _ = 0
    ones
  }
  let direct_call = get_ones()
  let provider = get_ones
  let function_call = provider()
  let tuple_projection = pair().0
  let list_projection = case [ones] {
    [function] -> function
    _ -> twos
  }
  let captured = {
    let value = 1
    fn() { [value] }
  }

  first(local)
  + first(bool_true)
  + first(bool_false)
  + first(int_hit)
  + first(int_fallback)
  + first(string_hit)
  + first(string_fallback)
  + first(float_hit)
  + first(float_fallback)
  + first(block)
  + first(direct_call)
  + first(function_call)
  + first(tuple_projection)
  + first(list_projection)
  + first(captured)
}

// geam:expect Int(19)
