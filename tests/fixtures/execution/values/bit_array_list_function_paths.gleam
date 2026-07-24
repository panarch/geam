fn values() {
  [<<1>>]
}

fn getter() {
  values
}

fn selector() {
  getter
}

fn first(function: fn() -> List(BitArray)) {
  let assert [value] = function()
  value
}

pub fn main() {
  let functions = [values]
  let assert [from_assert] = functions
  let from_case = case functions {
    [function] -> function
    _ -> values
  }

  #(
    first(values),
    first(getter()),
    first(selector()()),
    first(from_assert),
    first(from_case),
  )
}

// @geam:expect Tuple([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[1], bit_len=8), BitArray(bytes=[1], bit_len=8), BitArray(bytes=[1], bit_len=8), BitArray(bytes=[1], bit_len=8)])
