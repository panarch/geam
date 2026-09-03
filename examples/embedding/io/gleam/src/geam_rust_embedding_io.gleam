import gleam/io

pub fn announce(name: String) -> String {
  let message = "Hello, " <> name <> "!"
  io.println(message)
  message
}
