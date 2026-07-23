import operation

pub fn main() {
  let imported = operation.add_one
  let capture = fn() { imported }
  let returned = capture()

  #(
    operation.add_one == operation.add_one,
    imported == operation.add_one,
    imported(41),
    returned == operation.add_one,
    returned(41),
  )
}
// geam:expect Tuple([Bool(true), Bool(true), Int(42), Bool(true), Int(42)])
