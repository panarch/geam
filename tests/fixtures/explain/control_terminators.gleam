fn join(flag: Bool) -> Int {
  let selected = case flag {
    True -> 1
    False -> 2
  }
  selected + 3
}

fn assert_one(values: List(Int)) -> Int {
  let assert [value] = values
  value
}

fn stopped() -> Int {
  panic
}

fn stop() -> value {
  panic
}

fn never() -> Int {
  let _ = stop()
  1
}

pub fn main() {
  let _ = #(stopped, never)
  join(True) + assert_one([1])
}

// geam:explain
// module main
// main int#0
//
// function never#0
//   graph entry=b0
//   b0 instructions=0 source_stop
//
// function int#0
//   graph entry=b0
//   b0 instructions=9 return
//
// function int#1
//   graph entry=b0
//   b0 instructions=0 source_stop
//
// function int#2
//   graph entry=b0
//   b0 instructions=0 never_call
//
// function int#3
//   graph entry=b0
//   b0 instructions=0 branch bool true=b1 false=b3
//   b1 instructions=1 jump b2
//   b2 instructions=2 return
//   b3 instructions=1 jump b2
//
// function int#4
//   graph entry=b0
//   b0 instructions=0 match success=b1 failure=b2
//   b1 instructions=0 return
//   b2 instructions=0 let_assert_panic
