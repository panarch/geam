import gleam/io

pub fn main() {
  assert io.print("stdout") == Nil
  assert io.print_error("stderr") == Nil
  assert io.println("stdout line") == Nil
  assert io.println_error("stderr line") == Nil

  assert io.print("") == Nil
  assert io.println("") == Nil
  assert io.print_error("embedded\n") == Nil
  assert io.println_error("embedded\n") == Nil

  Nil
}
// @geam:expect Nil
