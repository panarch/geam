import fixture/work

fn identity(value) {
  value
}

pub fn make(seed: Int) -> work.Work(Int) {
  let original = work.ready(seed)
  let same = identity(original)
  let assert [same_list] = identity([same])
  let same_closure = identity(fn() { same_list })()
  let assert [same_closure_list] = identity(fn() { [same_closure] })()
  let offset = 2
  use value <- work.map(same_closure_list)
  value + offset
}

pub fn collect(seed: Int) -> work.Work(List(Int)) {
  let shared = make(seed)
  work.all([shared, shared])
}

pub fn keep(value: work.Work(Int)) -> work.Work(Int) {
  value
}

pub fn failure() -> work.Work(Int) {
  use _ <- work.map(work.ready(0))
  panic as "prepared work failed"
}
