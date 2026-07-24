pub type Holder {
  Holder(function: fn(Int) -> Int)
}

pub type Marker {
  Marker(Int)
}

fn add_one(value: Int) {
  value + 1
}

fn add_two(value: Int) {
  value + 2
}

fn direct() -> fn(Int) -> Int {
  add_one
}

fn functions() {
  [add_one]
}

fn pair(_left: Int, _right: String) {
  0
}

const stored = add_one

pub fn main() {
  let provider = fn() { add_two }
  let from_list = case functions() {
    [function] -> function
    _ -> add_one
  }

  #(
    stored,
    add_one,
    fn(value) { value },
    Marker,
    direct(),
    provider(),
    #(add_one).0,
    Holder(add_one).function,
    from_list,
    pair,
  )
}


// @geam:explain
// module main
// main tuple#0
//
// function int#0
//   entry b0 params=[%int#0:shape#0(Int)] captures=[]
//   block b0 params=[%int#0:shape#0(Int)]
//     %int#1:shape#0(Int) = int.value 1
//     %int#2:shape#0(Int) = int.add %int#0 %int#1
//     return %int#2
//
// function int#1
//   entry b0 params=[%int#0:shape#0(Int), %string#0:shape#11(String)] captures=[]
//   block b0 params=[%int#0:shape#0(Int), %string#0:shape#11(String)]
//     %int#1:shape#0(Int) = int.value 0
//     return %int#1
//
// function int#2
//   entry b0 params=[%int#0:shape#0(Int)] captures=[]
//   block b0 params=[%int#0:shape#0(Int)]
//     %int#1:shape#0(Int) = int.value 2
//     %int#2:shape#0(Int) = int.add %int#0 %int#1
//     return %int#2
//
// function tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.function#0:shape#3(fn() -> fn(Int) -> Int) = function[Function] closure target=function.int#0 captures=[]
//     %list.function#0:shape#4(list_type#0) = list.function[type#0] call list.function#0 args=[]
//     %bool#0:shape#5(Bool) = bool.list_length_equals %list.function#0 length=1
//     branch %bool#0 true=b1(%function.function#0, %list.function#0) false=b3(%function.function#0)
//   block b1 params=[%function.function#0:shape#3(fn() -> fn(Int) -> Int), %list.function#0:shape#4(list_type#0)]
//     %function.int#0:shape#1(fn(Int) -> Int) = function[Int] list_index %list.function#0 index=0
//     jump b2(%function.int#0, %function.function#0)
//   block b2 params=[%function.int#0:shape#1(fn(Int) -> Int), %function.function#0:shape#3(fn() -> fn(Int) -> Int)]
//     %function.int#1:shape#1(fn(Int) -> Int) = function[Int] constant.function#0
//     %function.int#2:shape#1(fn(Int) -> Int) = function[Int] reference int#0
//     %function.generic#0:shape#6(fn(param#0) -> param#0) = function[Generic] closure target=template#7 shapes=[shape#2] captures=[]
//     %function.custom#0:shape#8(fn(Int) -> custom_type#0) = function[Custom] constructor custom_type#0.constructor#0
//     %function.int#3:shape#1(fn(Int) -> Int) = function[Int] call function.int#1 args=[]
//     %function.int#4:shape#1(fn(Int) -> Int) = function[Int] function_call %function.function#0 args=[]
//     %function.int#5:shape#1(fn(Int) -> Int) = function[Int] reference int#0
//     %tuple#0:shape#9(#(fn(Int) -> Int)) = tuple.value elements=[%function.int#5]
//     %function.int#6:shape#1(fn(Int) -> Int) = function[Int] tuple_index %tuple#0 index=0
//     %function.int#7:shape#1(fn(Int) -> Int) = function[Int] reference int#0
//     %custom#0:shape#10(custom_type#1) = custom.construct custom_type#1.constructor#0 fields=[%function.int#7]
//     %function.int#8:shape#1(fn(Int) -> Int) = function[Int] custom_field %custom#0 index=0
//     %function.int#9:shape#12(fn(Int, String) -> Int) = function[Int] reference int#1
//     %tuple#1:shape#15(#(fn(Int) -> Int, fn(Int) -> Int, fn(param#0) -> param#0, fn(Int) -> custom_type#0, fn(Int) -> Int, fn(Int) -> Int, fn(Int) -> Int, fn(Int) -> Int, fn(Int) -> Int, fn(Int, String) -> Int)) = tuple.value elements=[%function.int#1, %function.int#2, %function.generic#0, %function.custom#0, %function.int#3, %function.int#4, %function.int#6, %function.int#8, %function.int#0, %function.int#9]
//     return %tuple#1
//   block b3 params=[%function.function#0:shape#3(fn() -> fn(Int) -> Int)]
//     %function.int#0:shape#1(fn(Int) -> Int) = function[Int] reference int#0
//     jump b2(%function.int#0, %function.function#0)
//
// function list.function#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.int#0:shape#1(fn(Int) -> Int) = function[Int] reference int#0
//     %list.function#0:shape#4(list_type#0) = list.function[type#0] value elements=[%function.int#0]
//     return %list.function#0
//
// function function.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.int#0:shape#1(fn(Int) -> Int) = function[Int] reference int#2
//     return %function.int#0
//
// function function.int#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.int#0:shape#1(fn(Int) -> Int) = function[Int] reference int#0
//     return %function.int#0
//
// constant.function#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.int#0:shape#1(fn(Int) -> Int) = function[Int] reference int#0
//     return %function.int#0
