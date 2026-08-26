import example_generic_box as box

fn increment(value: Int) -> Int {
  value + 1
}

pub fn main() {
  let original = box.new("alpha")
  let replaced = box.replace(original, 7)
  let mapped = box.map(replaced, increment)

  assert box.get(original) == "alpha"
  assert box.get(replaced) == 7
  assert box.get(mapped) == 8
  assert box.contains(original, "alpha")
  assert box.contains(replaced, 7)
  assert !box.contains(replaced, 8)
  assert original == box.new("alpha")
}
