pub type Empty

pub fn main() -> fn(Empty) -> Int {
  fn(_) { 42 }
}
// @geam:expect Function(fn(geam/main/Empty) -> Int)
