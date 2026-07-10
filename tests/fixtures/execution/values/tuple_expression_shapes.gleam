fn one() { #(1) }
fn two() { #(2) }
fn get_one() { one }

pub fn main() {
  let local = #(1)
  let provider = one
  let function_provider = get_one
  let projected_from_list = case [#(1)] {
    [value] -> value
    _ -> #(0)
  }

  assert local == #(1)
  assert one() == #(1)
  assert provider() == #(1)
  assert function_provider()() == #(1)
  assert #(#(1)).0 == #(1)
  assert projected_from_list == #(1)
  assert case True { True -> local False -> #(2) } == #(1)
  assert case False { True -> local False -> #(2) } == #(2)
  assert case 1 { 1 -> local _ -> #(2) } == #(1)
  assert case 0 { 1 -> local _ -> #(2) } == #(2)
  assert case "hit" { "hit" -> local _ -> #(2) } == #(1)
  assert case "miss" { "hit" -> local _ -> #(2) } == #(2)
  assert case 1.0 { 1.0 -> local _ -> #(2) } == #(1)
  assert case 0.0 { 1.0 -> local _ -> #(2) } == #(2)
  assert { let _ = 0 local } == #(1)
  42
}

// geam:expect Int(42)
