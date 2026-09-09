import example_async_files as files
import geam/future.{type Future}

pub fn double(value: Int) -> Int {
  value * 2
}

pub fn greeting(path: String) -> Future(Result(String, String)) {
  use contents <- future.map(files.read(path))
  case contents {
    Ok(text) -> Ok("Hello " <> text)
    Error(error) -> Error(error)
  }
}
