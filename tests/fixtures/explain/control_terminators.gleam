fn join(flag: Bool) -> Int {
  let selected = case flag {
    True -> 1
    False -> 2
  }
  selected + 3
}

fn assert_one(values: List(Int)) -> Int {
  let assert [value] = values as "expected one value"
  let assert [_] = values
  value
}

fn stopped() -> Int {
  panic as "stopped"
}

fn stop() -> value {
  panic
}

fn never() -> Int {
  let function = stop
  let _ = function()
  1
}

fn direct_never() -> Int {
  let _ = stop()
  1
}

fn select_int(value: Int) {
  case value {
    0 -> 10
    1 -> 20
    _ -> 30
  }
}

fn select_float(value: Float) {
  case value {
    0.0 -> 10
    1.0 -> 20
    _ -> 30
  }
}

fn select_string(value: String) {
  case value {
    "zero" -> 10
    "one" -> 20
    _ -> 30
  }
}

pub fn main() {
  let _ = #(stopped, never, direct_never)
  join(True) + assert_one([1]) + select_int(0) + select_float(0.0) + select_string("zero")
}




// geam:run
// geam:explain
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
//     %function.int#0:shape#1(fn() -> Int) = function[Int] reference int#1
//     %function.int#1:shape#1(fn() -> Int) = function[Int] reference int#2
//     %function.int#2:shape#1(fn() -> Int) = function[Int] reference int#3
//     %tuple#0:shape#2(#(fn() -> Int, fn() -> Int, fn() -> Int)) = tuple.value elements=[%function.int#0, %function.int#1, %function.int#2]
//     %bool#0:shape#3(Bool) = bool.value True
//     %int#0:shape#0(Int) = int.call int#4 args=[%bool#0]
//     %int#1:shape#0(Int) = int.value 1
//     %list.int#0:shape#4(list_type#0) = list.int[type#0] value elements=[%int#1]
//     %int#2:shape#0(Int) = int.call int#5 args=[%list.int#0]
//     %int#3:shape#0(Int) = int.add %int#0 %int#2
//     %int#4:shape#0(Int) = int.value 0
//     %int#5:shape#0(Int) = int.call int#6 args=[%int#4]
//     %int#6:shape#0(Int) = int.add %int#3 %int#5
//     %float#0:shape#5(Float) = float.value 0.0
//     %int#7:shape#0(Int) = int.call int#7 args=[%float#0]
//     %int#8:shape#0(Int) = int.add %int#6 %int#7
//     %string#0:shape#6(String) = string.value "zero"
//     %int#9:shape#0(Int) = int.call int#8 args=[%string#0]
//     %int#10:shape#0(Int) = int.add %int#8 %int#9
//     return %int#10
//
// function int#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %string#0:shape#6(String) = string.value "stopped"
//     source_stop kind=panic message=%string#0
//
// function int#2
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.never#0:shape#8(fn() -> param#0) = function[Never] reference never#0
//     never_call %function.never#0 args=[]
//
// function int#3
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     never_call never#0 args=[]
//
// function int#4
//   entry b0 params=[%bool#0:shape#3(Bool)] captures=[]
//   block b0 params=[%bool#0:shape#3(Bool)]
//     branch %bool#0 true=b1() false=b3()
//   block b1 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     jump b2(%int#0)
//   block b2 params=[%int#0:shape#0(Int)]
//     %int#1:shape#0(Int) = int.value 3
//     %int#2:shape#0(Int) = int.add %int#0 %int#1
//     return %int#2
//   block b3 params=[]
//     %int#0:shape#0(Int) = int.value 2
//     jump b2(%int#0)
//
// function int#5
//   entry b0 params=[%list.int#0:shape#4(list_type#0)] captures=[]
//   block b0 params=[%list.int#0:shape#4(list_type#0)]
//     match %list.int#0 pattern=[binding#0] success=b1(binding#0, %list.int#0) failure=b4(%list.int#0)
//   block b1 params=[%int#0:shape#0(Int), %list.int#0:shape#4(list_type#0)]
//     match %list.int#0 pattern=[_] success=b2(%int#0) failure=b3(%list.int#0)
//   block b2 params=[%int#0:shape#0(Int)]
//     return %int#0
//   block b3 params=[%list.int#0:shape#4(list_type#0)]
//     let_assert_panic subject=%list.int#0 message=none
//   block b4 params=[%list.int#0:shape#4(list_type#0)]
//     %string#0:shape#6(String) = string.value "expected one value"
//     let_assert_panic subject=%list.int#0 message=%string#0
//
// function int#6
//   entry b0 params=[%int#0:shape#0(Int)] captures=[]
//   block b0 params=[%int#0:shape#0(Int)]
//     switch.int %int#0 clauses=[0->b1(), 1->b2()] fallback=b3()
//   block b1 params=[]
//     %int#0:shape#0(Int) = int.value 10
//     return %int#0
//   block b2 params=[]
//     %int#0:shape#0(Int) = int.value 20
//     return %int#0
//   block b3 params=[]
//     %int#0:shape#0(Int) = int.value 30
//     return %int#0
//
// function int#7
//   entry b0 params=[%float#0:shape#5(Float)] captures=[]
//   block b0 params=[%float#0:shape#5(Float)]
//     switch.float %float#0 clauses=[0.0->b1(), 1.0->b2()] fallback=b3()
//   block b1 params=[]
//     %int#0:shape#0(Int) = int.value 10
//     return %int#0
//   block b2 params=[]
//     %int#0:shape#0(Int) = int.value 20
//     return %int#0
//   block b3 params=[]
//     %int#0:shape#0(Int) = int.value 30
//     return %int#0
//
// function int#8
//   entry b0 params=[%string#0:shape#6(String)] captures=[]
//   block b0 params=[%string#0:shape#6(String)]
//     switch.string %string#0 clauses=["zero"->b1(), "one"->b2()] fallback=b3()
//   block b1 params=[]
//     %int#0:shape#0(Int) = int.value 10
//     return %int#0
//   block b2 params=[]
//     %int#0:shape#0(Int) = int.value 20
//     return %int#0
//   block b3 params=[]
//     %int#0:shape#0(Int) = int.value 30
//     return %int#0
