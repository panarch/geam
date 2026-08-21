fn emit(value: Int) {
  echo value as "selected"
}

pub fn main() {
  emit(1)
}


// @geam:explain
// module main
// main int#0
//
// function int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     tail int#1 args=[%int#0]
//
// function int#1
//   entry b0 params=[%int#0:shape#0(Int)] captures=[]
//   block b0 params=[%int#0:shape#0(Int)]
//     %string#0:shape#1(String) = string.value "selected"
//     echo subject=%int#0 message=%string#0 site=main::emit@24..48 next=b1(%int#0)
//   block b1 params=[%int#0:shape#0(Int)]
//     return %int#0
