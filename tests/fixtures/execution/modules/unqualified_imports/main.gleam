import model.{type Boxed, Boxed, answer, double}

fn value(boxed: Boxed) {
  case boxed {
    Boxed(value) -> value
  }
}

pub fn main() {
  value(Boxed(double(answer)))
}
// geam:expect Int(42)
