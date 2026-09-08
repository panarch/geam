import geam/future
import standalone_future/native

pub fn main() {
  use _ <- future.map(native.timer())
  native.current()
}
