import gleam/io

pub fn main() {
  io.print("before")
  io.println("stdout line")
  echo Nil as "between"
  io.print_error("after")
  io.println_error("stderr line")
  panic as "stop"
}
