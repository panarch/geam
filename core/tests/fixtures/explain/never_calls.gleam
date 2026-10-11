pub type Empty

fn stop() -> Empty {
  panic
}

pub fn main() -> Int {
  let _: Empty = stop()
  42
}
// @geam:explain
// module main
// main int#0
//
// function never#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     source_stop kind=panic message=none
//
// function int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     never_call never#0 args=[]
