pub type Lists(value) {
  Lists(values: List(value))
}

pub type Nested(value) {
  Nested(values: List(List(value)))
}

pub type Boxed(value) {
  Boxed(value)
}

const empty = []
const nested = [[]]

fn identity(value: value) {
  value
}

fn empty_value() -> List(value) {
  []
}

fn empty_constant() -> List(value) {
  empty
}

fn empty_call() -> List(value) {
  empty_value()
}

fn empty_function_call(provider: fn() -> List(value)) -> List(value) {
  provider()
}

fn empty_tuple() -> List(value) {
  #([]).0
}

fn empty_field() -> List(value) {
  Lists([]).values
}

fn empty_index() -> List(value) {
  case [[]] {
    [first] -> first
    _ -> []
  }
}

fn nested_value() -> List(List(value)) {
  [[]]
}

fn nested_constant() -> List(List(value)) {
  nested
}

fn nested_spread(values: List(List(value))) -> List(List(value)) {
  [[], ..values]
}

fn nested_call() -> List(List(value)) {
  nested_value()
}

fn nested_function_call(provider: fn() -> List(List(value))) -> List(List(value)) {
  provider()
}

fn nested_tuple() -> List(List(value)) {
  #([[]]).0
}

fn nested_field() -> List(List(value)) {
  Nested([[]]).values
}

fn nested_index() -> List(List(value)) {
  case [[[]]] {
    [first] -> first
    _ -> []
  }
}

fn nested_drop() -> List(List(value)) {
  case [[], []] {
    [_, ..tail] -> tail
    _ -> []
  }
}

pub fn main() {
  #(
    identity,
    Boxed,
    empty_value,
    empty_constant,
    empty_call,
    empty_function_call,
    empty_tuple,
    empty_field,
    empty_index,
    nested_value,
    nested_constant,
    nested_spread,
    nested_call,
    nested_function_call,
    nested_tuple,
    nested_field,
    nested_index,
    nested_drop,
  )
}


// @geam:explain
// module main
// main tuple#0
//
// function tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.generic#0:shape#1(fn(param#0) -> param#0) = function[Generic] reference template#1 shapes=[shape#0]
//     %function.generic#1:shape#4(fn(param#1) -> custom_type#0) = function[Generic] closure target=custom_type#0.constructor#0 captures=[]
//     %function.list.parameter#0:shape#7(fn() -> list_type#0) = function[List] reference list.parameter#0
//     %function.list.parameter#1:shape#10(fn() -> list_type#1) = function[List] reference list.parameter#1
//     %function.list.parameter#2:shape#13(fn() -> list_type#2) = function[List] reference list.parameter#2
//     %function.list.parameter#3:shape#17(fn(fn() -> list_type#3) -> list_type#3) = function[List] reference list.parameter#3
//     %function.list.parameter#4:shape#20(fn() -> list_type#4) = function[List] reference list.parameter#4
//     %function.list.parameter#5:shape#23(fn() -> list_type#5) = function[List] reference list.parameter#5
//     %function.list.parameter#6:shape#26(fn() -> list_type#6) = function[List] reference list.parameter#6
//     %function.list.parameter_list#0:shape#30(fn() -> list_type#8) = function[List] reference list.parameter_list#0
//     %function.list.parameter_list#1:shape#34(fn() -> list_type#10) = function[List] reference list.parameter_list#1
//     %function.list.parameter_list#2:shape#38(fn(list_type#12) -> list_type#12) = function[List] reference list.parameter_list#2
//     %function.list.parameter_list#3:shape#42(fn() -> list_type#14) = function[List] reference list.parameter_list#3
//     %function.list.parameter_list#4:shape#47(fn(fn() -> list_type#16) -> list_type#16) = function[List] reference list.parameter_list#4
//     %function.list.parameter_list#5:shape#51(fn() -> list_type#18) = function[List] reference list.parameter_list#5
//     %function.list.parameter_list#6:shape#55(fn() -> list_type#20) = function[List] reference list.parameter_list#6
//     %function.list.parameter_list#7:shape#59(fn() -> list_type#22) = function[List] reference list.parameter_list#7
//     %function.list.parameter_list#8:shape#63(fn() -> list_type#24) = function[List] reference list.parameter_list#8
//     %tuple#0:shape#64(#(fn(param#0) -> param#0, fn(param#1) -> custom_type#0, fn() -> list_type#0, fn() -> list_type#1, fn() -> list_type#2, fn(fn() -> list_type#3) -> list_type#3, fn() -> list_type#4, fn() -> list_type#5, fn() -> list_type#6, fn() -> list_type#8, fn() -> list_type#10, fn(list_type#12) -> list_type#12, fn() -> list_type#14, fn(fn() -> list_type#16) -> list_type#16, fn() -> list_type#18, fn() -> list_type#20, fn() -> list_type#22, fn() -> list_type#24)) = tuple.value elements=[%function.generic#0, %function.generic#1, %function.list.parameter#0, %function.list.parameter#1, %function.list.parameter#2, %function.list.parameter#3, %function.list.parameter#4, %function.list.parameter#5, %function.list.parameter#6, %function.list.parameter_list#0, %function.list.parameter_list#1, %function.list.parameter_list#2, %function.list.parameter_list#3, %function.list.parameter_list#4, %function.list.parameter_list#5, %function.list.parameter_list#6, %function.list.parameter_list#7, %function.list.parameter_list#8]
//     return %tuple#0
//
// function list.parameter#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#6(list_type#0) = list.parameter[type#0] empty
//     return %list.parameter#0
//
// function list.parameter#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#9(list_type#1) = list.parameter[type#1] constant.list.parameter#0
//     return %list.parameter#0
//
// function list.parameter#2
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.parameter#7 args=[]
//
// function list.parameter#3
//   entry b0 params=[%function.list.parameter#0:shape#16(fn() -> list_type#3)] captures=[]
//   block b0 params=[%function.list.parameter#0:shape#16(fn() -> list_type#3)]
//     %list.parameter#0:shape#15(list_type#3) = list.parameter[type#3] function_call %function.list.parameter#0 args=[]
//     return %list.parameter#0
//
// function list.parameter#4
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#19(list_type#4) = list.parameter[type#4] empty
//     %tuple#0:shape#65(#(list_type#4)) = tuple.value elements=[%list.parameter#0]
//     %list.parameter#1:shape#19(list_type#4) = list.parameter[type#4] tuple_index %tuple#0 index=0
//     return %list.parameter#1
//
// function list.parameter#5
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#22(list_type#5) = list.parameter[type#5] empty
//     %custom#0:shape#66(custom_type#1) = custom.construct custom_type#1.constructor#0 fields=[%list.parameter#0]
//     %list.parameter#1:shape#22(list_type#5) = list.parameter[type#5] custom_field %custom#0 index=0
//     return %list.parameter#1
//
// function list.parameter#6
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#25(list_type#6) = list.parameter[type#6] empty
//     %list.parameter_list#0:shape#67(list_type#25) = list.parameter_list[type#25] value elements=[%list.parameter#0]
//     %bool#0:shape#68(Bool) = bool.list_length_equals %list.parameter_list#0 length=1
//     branch %bool#0 true=b1(%list.parameter_list#0) false=b2()
//   block b1 params=[%list.parameter_list#0:shape#67(list_type#25)]
//     %list.parameter#0:shape#25(list_type#6) = list.parameter[type#6] list_index %list.parameter_list#0 index=0
//     return %list.parameter#0
//   block b2 params=[]
//     %list.parameter#0:shape#25(list_type#6) = list.parameter[type#6] empty
//     return %list.parameter#0
//
// function list.parameter#7
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#12(list_type#2) = list.parameter[type#2] empty
//     return %list.parameter#0
//
// function list.parameter_list#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#28(list_type#7) = list.parameter[type#7] empty
//     %list.parameter_list#0:shape#29(list_type#8) = list.parameter_list[type#8] value elements=[%list.parameter#0]
//     return %list.parameter_list#0
//
// function list.parameter_list#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter_list#0:shape#33(list_type#10) = list.parameter_list[type#10] constant.list.parameter_list#0
//     return %list.parameter_list#0
//
// function list.parameter_list#2
//   entry b0 params=[%list.parameter_list#0:shape#37(list_type#12)] captures=[]
//   block b0 params=[%list.parameter_list#0:shape#37(list_type#12)]
//     %list.parameter#0:shape#36(list_type#11) = list.parameter[type#11] empty
//     %list.parameter_list#1:shape#37(list_type#12) = list.parameter_list[type#12] spread elements=[%list.parameter#0] tail=%list.parameter_list#0
//     return %list.parameter_list#1
//
// function list.parameter_list#3
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail list.parameter_list#9 args=[]
//
// function list.parameter_list#4
//   entry b0 params=[%function.list.parameter_list#0:shape#46(fn() -> list_type#16)] captures=[]
//   block b0 params=[%function.list.parameter_list#0:shape#46(fn() -> list_type#16)]
//     %list.parameter_list#0:shape#45(list_type#16) = list.parameter_list[type#16] function_call %function.list.parameter_list#0 args=[]
//     return %list.parameter_list#0
//
// function list.parameter_list#5
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#49(list_type#17) = list.parameter[type#17] empty
//     %list.parameter_list#0:shape#50(list_type#18) = list.parameter_list[type#18] value elements=[%list.parameter#0]
//     %tuple#0:shape#69(#(list_type#18)) = tuple.value elements=[%list.parameter_list#0]
//     %list.parameter_list#1:shape#50(list_type#18) = list.parameter_list[type#18] tuple_index %tuple#0 index=0
//     return %list.parameter_list#1
//
// function list.parameter_list#6
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#53(list_type#19) = list.parameter[type#19] empty
//     %list.parameter_list#0:shape#54(list_type#20) = list.parameter_list[type#20] value elements=[%list.parameter#0]
//     %custom#0:shape#70(custom_type#2) = custom.construct custom_type#2.constructor#0 fields=[%list.parameter_list#0]
//     %list.parameter_list#1:shape#54(list_type#20) = list.parameter_list[type#20] custom_field %custom#0 index=0
//     return %list.parameter_list#1
//
// function list.parameter_list#7
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#57(list_type#21) = list.parameter[type#21] empty
//     %list.parameter_list#0:shape#58(list_type#22) = list.parameter_list[type#22] value elements=[%list.parameter#0]
//     %list.list#0:shape#71(list_type#26) = list.list[type#26] value elements=[%list.parameter_list#0]
//     %bool#0:shape#68(Bool) = bool.list_length_equals %list.list#0 length=1
//     branch %bool#0 true=b1(%list.list#0) false=b2()
//   block b1 params=[%list.list#0:shape#71(list_type#26)]
//     %list.parameter_list#0:shape#58(list_type#22) = list.parameter_list[type#22] list_index %list.list#0 index=0
//     return %list.parameter_list#0
//   block b2 params=[]
//     %list.parameter_list#0:shape#58(list_type#22) = list.parameter_list[type#22] value elements=[]
//     return %list.parameter_list#0
//
// function list.parameter_list#8
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#61(list_type#23) = list.parameter[type#23] empty
//     %list.parameter#1:shape#61(list_type#23) = list.parameter[type#23] empty
//     %list.parameter_list#0:shape#62(list_type#24) = list.parameter_list[type#24] value elements=[%list.parameter#0, %list.parameter#1]
//     %bool#0:shape#68(Bool) = bool.list_length_at_least %list.parameter_list#0 length=1
//     branch %bool#0 true=b1(%list.parameter_list#0) false=b2()
//   block b1 params=[%list.parameter_list#0:shape#62(list_type#24)]
//     %list.parameter_list#1:shape#62(list_type#24) = list.parameter_list[type#24] drop_first %list.parameter_list#0 count=1
//     return %list.parameter_list#1
//   block b2 params=[]
//     %list.parameter_list#0:shape#62(list_type#24) = list.parameter_list[type#24] value elements=[]
//     return %list.parameter_list#0
//
// function list.parameter_list#9
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#40(list_type#13) = list.parameter[type#13] empty
//     %list.parameter_list#0:shape#41(list_type#14) = list.parameter_list[type#14] value elements=[%list.parameter#0]
//     return %list.parameter_list#0
//
// constant.list.parameter#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#9(list_type#1) = list.parameter[type#1] empty
//     return %list.parameter#0
//
// constant.list.parameter_list#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#32(list_type#9) = list.parameter[type#9] empty
//     %list.parameter_list#0:shape#33(list_type#10) = list.parameter_list[type#10] value elements=[%list.parameter#0]
//     return %list.parameter_list#0
