import gleam/io

pub fn main() {
  io.print("before")
  echo Nil as "between"
  io.print_error("after")
  panic as "stop"
}
