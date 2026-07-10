fn nil_value() { Nil }
fn get_nil() { nil_value }

pub fn main() {
  let local = Nil
  let provider = nil_value
  let function_provider = get_nil
  let projected_from_list = case [Nil] {
    [value] -> value
    _ -> Nil
  }

  assert local == Nil
  assert nil_value() == Nil
  assert provider() == Nil
  assert function_provider()() == Nil
  assert #(Nil).0 == Nil
  assert projected_from_list == Nil
  assert case True { True -> local False -> Nil } == Nil
  assert case False { True -> local False -> Nil } == Nil
  assert case 1 { 1 -> local _ -> Nil } == Nil
  assert case 0 { 1 -> local _ -> Nil } == Nil
  assert case "hit" { "hit" -> local _ -> Nil } == Nil
  assert case "miss" { "hit" -> local _ -> Nil } == Nil
  assert case 1.0 { 1.0 -> local _ -> Nil } == Nil
  assert case 0.0 { 1.0 -> local _ -> Nil } == Nil
  assert { let _ = 0 local } == Nil
  42
}

// geam:expect Int(42)
