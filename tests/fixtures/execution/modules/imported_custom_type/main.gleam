import model

pub fn main() {
  let constructor: fn(Int, String) -> model.Boxed(Int) = model.Boxed
  let boxed = constructor(20, "initial")
  let mapped = model.map(boxed, fn(value) { value + 1 })
  let updated = model.Boxed(..mapped, value: 42)

  #(mapped, updated.value, updated.label)
}
// geam:expect Tuple([Custom(type=geam/model/Boxed(Int), constructor=Boxed#0, fields=[value: Int(21), label: String("initial")]), Int(42), String("initial")])
