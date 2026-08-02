import gleam/dynamic

pub fn main() {
  let bool = dynamic.bool(True)
  let string = dynamic.string("one")
  let float = dynamic.float(1.5)
  let int = dynamic.int(42)
  let bits = dynamic.bit_array(<<1, 2>>)
  let list = dynamic.list([int, string])
  let array = dynamic.array([int, string])
  let properties = dynamic.properties([#(string, int)])
  let nil = dynamic.nil()

  assert dynamic.classify(bool) == "Bool"
  assert dynamic.classify(string) == "String"
  assert dynamic.classify(float) == "Float"
  assert dynamic.classify(int) == "Int"
  assert dynamic.classify(bits) == "BitArray"
  assert dynamic.classify(list) == "List"
  assert dynamic.classify(array) == "Array"
  assert dynamic.classify(properties) == "Dict"
  assert dynamic.classify(nil) == "Nil"
  assert dynamic.int(42) == int
  assert list != array

  #(bool, string, float, int, bits, list, array, properties, nil)
}
// @geam:expect #(True, "one", 1.5, 42, <<1, 2>>, [42, "one"], #(42, "one"), dict.from_list([#("one", 42)]), Nil)
