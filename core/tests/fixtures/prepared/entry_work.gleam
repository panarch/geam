import fixture/work

pub fn main() {
  echo "main"
  work.map(work.ready(41), fn(value) {
    echo value + 1
    fn(value: Int) { value + 1 }
  })
}
