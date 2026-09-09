import example_async_files
import geam/future.{type Future}
import gleam/io

pub fn main() -> Future(Nil) {
  use result <- future.map(example_async_files.read("message.txt"))
  case result {
    Ok(text) -> io.print(text)
    Error(reason) -> io.println(reason)
  }
}
