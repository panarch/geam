pub type Marker {
  Marker(Int)
}

fn codepoint() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn empty() -> List(value) {
  []
}

fn nested_empty() -> List(List(value)) {
  [[]]
}

fn generic_identity(value: value) {
  value
}

fn never_value() -> value {
  panic
}

fn int_value() { 1 }
fn float_value() { 1.0 }
fn string_value() { "one" }
fn bit_array_value() { <<1>> }
fn utf_codepoint_value() { codepoint() }
fn custom_value() { Marker(1) }
fn bool_value() { True }
fn nil_value() { Nil }
fn tuple_value() { #(1) }
fn list_value() { [1] }
fn function_value() { int_value }

pub fn main() {
  let int = 1
  let float = 1.0
  let string = "one"
  let bit_array = <<1>>
  let utf_codepoint = codepoint()
  let custom = Marker(1)
  let bool = True
  let nil = Nil
  let tuple = #(1)
  let parameter_list = empty()
  let parameter_list_list = nested_empty()
  let int_list = [1]
  let string_list = ["one"]
  let bit_array_list = [<<1>>]
  let utf_codepoint_list = [utf_codepoint]
  let custom_list = [Marker(1)]
  let float_list = [1.0]
  let bool_list = [True]
  let nil_list = [Nil]
  let tuple_list = [#(1)]
  let list_list = [[1]]
  let function_list = [int_value]
  let generic_function_list = [generic_identity]
  let never_function_list = [never_value]
  let float_function_list = [float_value]
  let string_function_list = [string_value]
  let bit_array_function_list = [bit_array_value]
  let utf_codepoint_function_list = [utf_codepoint_value]
  let custom_function_list = [custom_value]
  let bool_function_list = [bool_value]
  let nil_function_list = [nil_value]
  let tuple_function_list = [tuple_value]
  let list_function_list = [list_value]
  let function_function_list = [function_value]
  let generic_function = generic_identity
  let never_function = never_value
  let int_function = int_value
  let float_function = float_value
  let string_function = string_value
  let bit_array_function = bit_array_value
  let utf_codepoint_function = utf_codepoint_value
  let custom_function = custom_value
  let bool_function = bool_value
  let nil_function = nil_value
  let tuple_function = tuple_value
  let list_function = list_value
  let function_function = function_value

  fn() {
    #(
      int,
      float,
      string,
      bit_array,
      utf_codepoint,
      custom,
      bool,
      nil,
      tuple,
      parameter_list,
      parameter_list_list,
      int_list,
      string_list,
      bit_array_list,
      utf_codepoint_list,
      custom_list,
      float_list,
      bool_list,
      nil_list,
      tuple_list,
      list_list,
      function_list,
      generic_function_list,
      never_function_list,
      float_function_list,
      string_function_list,
      bit_array_function_list,
      utf_codepoint_function_list,
      custom_function_list,
      bool_function_list,
      nil_function_list,
      tuple_function_list,
      list_function_list,
      function_function_list,
      generic_function,
      never_function,
      int_function,
      float_function,
      string_function,
      bit_array_function,
      utf_codepoint_function,
      custom_function,
      bool_function,
      nil_function,
      tuple_function,
      list_function,
      function_function,
    )
  }
}


// @geam:explain
// module main
// main function.tuple#0
//
// function never#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     source_stop kind=panic message=none
//
// function never#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     source_stop kind=panic message=none
//
// function int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#1(Int) = int.value 1
//     return %int#0
//
// function float#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %float#0:shape#7(Float) = float.value 1.0
//     return %float#0
//
// function string#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %string#0:shape#8(String) = string.value "one"
//     return %string#0
//
// function bit_array#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#1(Int) = int.value 1
//     %bit_array#0:shape#9(BitArray) = bit_array.value [int(%int#0, bits=8, big)]
//     return %bit_array#0
//
// function utf_codepoint#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#1(Int) = int.value 65
//     %bit_array#0:shape#9(BitArray) = bit_array.value [int(%int#0, bits=8, big)]
//     match %bit_array#0 pattern=<<utf_codepoint(binding#0, utf8)>> success=b1(binding#0) failure=b2(%bit_array#0)
//   block b1 params=[%utf_codepoint#0:shape#10(UtfCodepoint)]
//     return %utf_codepoint#0
//   block b2 params=[%bit_array#0:shape#9(BitArray)]
//     let_assert_panic subject=%bit_array#0 message=none
//
// function utf_codepoint#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     tail utf_codepoint#0 args=[]
//
// function custom#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#1(Int) = int.value 1
//     %custom#0:shape#11(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#0]
//     return %custom#0
//
// function bool#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %bool#0:shape#12(Bool) = bool.value True
//     return %bool#0
//
// function nil#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %nil#0:shape#13(Nil) = nil.value
//     return %nil#0
//
// function tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#1(Int) = int.value 1
//     %tuple#0:shape#14(#(Int)) = tuple.value elements=[%int#0]
//     return %tuple#0
//
// function tuple#1
//   entry b0 params=[] captures=[%int#0:shape#1(Int), %float#0:shape#7(Float), %string#0:shape#8(String), %bit_array#0:shape#9(BitArray), %utf_codepoint#0:shape#10(UtfCodepoint), %custom#0:shape#11(custom_type#0), %bool#0:shape#12(Bool), %nil#0:shape#13(Nil), %tuple#0:shape#14(#(Int)), %list.parameter#0:shape#16(list_type#0), %list.parameter_list#0:shape#19(list_type#2), %list.int#0:shape#20(list_type#3), %list.string#0:shape#21(list_type#4), %list.bit_array#0:shape#22(list_type#5), %list.utf_codepoint#0:shape#23(list_type#6), %list.custom#0:shape#24(list_type#7), %list.float#0:shape#25(list_type#8), %list.bool#0:shape#26(list_type#9), %list.nil#0:shape#27(list_type#10), %list.tuple#0:shape#28(list_type#11), %list.list#0:shape#29(list_type#12), %list.function#0:shape#30(list_type#13), %list.function#1:shape#32(list_type#14), %list.function#2:shape#35(list_type#15), %list.function#3:shape#37(list_type#16), %list.function#4:shape#39(list_type#17), %list.function#5:shape#41(list_type#18), %list.function#6:shape#43(list_type#19), %list.function#7:shape#46(list_type#20), %list.function#8:shape#48(list_type#21), %list.function#9:shape#50(list_type#22), %list.function#10:shape#52(list_type#23), %list.function#11:shape#54(list_type#24), %list.function#12:shape#56(list_type#25), %function.generic#0:shape#4(fn(param#4) -> param#4), %function.never#0:shape#6(fn() -> param#5), %function.int#0:shape#2(fn() -> Int), %function.float#0:shape#36(fn() -> Float), %function.string#0:shape#38(fn() -> String), %function.bit_array#0:shape#40(fn() -> BitArray), %function.utf_codepoint#0:shape#42(fn() -> UtfCodepoint), %function.custom#0:shape#45(fn() -> custom_type#0), %function.bool#0:shape#47(fn() -> Bool), %function.nil#0:shape#49(fn() -> Nil), %function.tuple#0:shape#51(fn() -> #(Int)), %function.list.int#0:shape#53(fn() -> list_type#3), %function.function#0:shape#55(fn() -> fn() -> Int)]
//   block b0 params=[%int#0:shape#1(Int), %float#0:shape#7(Float), %string#0:shape#8(String), %bit_array#0:shape#9(BitArray), %utf_codepoint#0:shape#10(UtfCodepoint), %custom#0:shape#11(custom_type#0), %bool#0:shape#12(Bool), %nil#0:shape#13(Nil), %tuple#0:shape#14(#(Int)), %list.parameter#0:shape#16(list_type#0), %list.parameter_list#0:shape#19(list_type#2), %list.int#0:shape#20(list_type#3), %list.string#0:shape#21(list_type#4), %list.bit_array#0:shape#22(list_type#5), %list.utf_codepoint#0:shape#23(list_type#6), %list.custom#0:shape#24(list_type#7), %list.float#0:shape#25(list_type#8), %list.bool#0:shape#26(list_type#9), %list.nil#0:shape#27(list_type#10), %list.tuple#0:shape#28(list_type#11), %list.list#0:shape#29(list_type#12), %list.function#0:shape#30(list_type#13), %list.function#1:shape#32(list_type#14), %list.function#2:shape#35(list_type#15), %list.function#3:shape#37(list_type#16), %list.function#4:shape#39(list_type#17), %list.function#5:shape#41(list_type#18), %list.function#6:shape#43(list_type#19), %list.function#7:shape#46(list_type#20), %list.function#8:shape#48(list_type#21), %list.function#9:shape#50(list_type#22), %list.function#10:shape#52(list_type#23), %list.function#11:shape#54(list_type#24), %list.function#12:shape#56(list_type#25), %function.generic#0:shape#4(fn(param#4) -> param#4), %function.never#0:shape#6(fn() -> param#5), %function.int#0:shape#2(fn() -> Int), %function.float#0:shape#36(fn() -> Float), %function.string#0:shape#38(fn() -> String), %function.bit_array#0:shape#40(fn() -> BitArray), %function.utf_codepoint#0:shape#42(fn() -> UtfCodepoint), %function.custom#0:shape#45(fn() -> custom_type#0), %function.bool#0:shape#47(fn() -> Bool), %function.nil#0:shape#49(fn() -> Nil), %function.tuple#0:shape#51(fn() -> #(Int)), %function.list.int#0:shape#53(fn() -> list_type#3), %function.function#0:shape#55(fn() -> fn() -> Int)]
//     %tuple#1:shape#62(#(Int, Float, String, BitArray, UtfCodepoint, custom_type#0, Bool, Nil, #(Int), list_type#0, list_type#2, list_type#3, list_type#4, list_type#5, list_type#6, list_type#7, list_type#8, list_type#9, list_type#10, list_type#11, list_type#12, list_type#13, list_type#14, list_type#15, list_type#16, list_type#17, list_type#18, list_type#19, list_type#20, list_type#21, list_type#22, list_type#23, list_type#24, list_type#25, fn(param#4) -> param#4, fn() -> param#5, fn() -> Int, fn() -> Float, fn() -> String, fn() -> BitArray, fn() -> UtfCodepoint, fn() -> custom_type#0, fn() -> Bool, fn() -> Nil, fn() -> #(Int), fn() -> list_type#3, fn() -> fn() -> Int)) = tuple.value elements=[%int#0, %float#0, %string#0, %bit_array#0, %utf_codepoint#0, %custom#0, %bool#0, %nil#0, %tuple#0, %list.parameter#0, %list.parameter_list#0, %list.int#0, %list.string#0, %list.bit_array#0, %list.utf_codepoint#0, %list.custom#0, %list.float#0, %list.bool#0, %list.nil#0, %list.tuple#0, %list.list#0, %list.function#0, %list.function#1, %list.function#2, %list.function#3, %list.function#4, %list.function#5, %list.function#6, %list.function#7, %list.function#8, %list.function#9, %list.function#10, %list.function#11, %list.function#12, %function.generic#0, %function.never#0, %function.int#0, %function.float#0, %function.string#0, %function.bit_array#0, %function.utf_codepoint#0, %function.custom#0, %function.bool#0, %function.nil#0, %function.tuple#0, %function.list.int#0, %function.function#0]
//     return %tuple#1
//
// function list.parameter#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#16(list_type#0) = list.parameter[type#0] empty
//     return %list.parameter#0
//
// function list.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#1(Int) = int.value 1
//     %list.int#0:shape#20(list_type#3) = list.int[type#3] value elements=[%int#0]
//     return %list.int#0
//
// function list.parameter_list#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %list.parameter#0:shape#18(list_type#1) = list.parameter[type#1] empty
//     %list.parameter_list#0:shape#19(list_type#2) = list.parameter_list[type#2] value elements=[%list.parameter#0]
//     return %list.parameter_list#0
//
// function function.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.int#0:shape#2(fn() -> Int) = function[Int] reference int#0
//     return %function.int#0
//
// function function.tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#1(Int) = int.value 1
//     %float#0:shape#7(Float) = float.value 1.0
//     %string#0:shape#8(String) = string.value "one"
//     %int#1:shape#1(Int) = int.value 1
//     %bit_array#0:shape#9(BitArray) = bit_array.value [int(%int#1, bits=8, big)]
//     %utf_codepoint#0:shape#10(UtfCodepoint) = utf_codepoint.call utf_codepoint#0 args=[]
//     %int#2:shape#1(Int) = int.value 1
//     %custom#0:shape#11(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#2]
//     %bool#0:shape#12(Bool) = bool.value True
//     %nil#0:shape#13(Nil) = nil.value
//     %int#3:shape#1(Int) = int.value 1
//     %tuple#0:shape#14(#(Int)) = tuple.value elements=[%int#3]
//     %list.parameter#0:shape#16(list_type#0) = list.parameter[type#0] call list.parameter#0 args=[]
//     %list.parameter_list#0:shape#19(list_type#2) = list.parameter_list[type#2] call list.parameter_list#0 args=[]
//     %int#4:shape#1(Int) = int.value 1
//     %list.int#0:shape#20(list_type#3) = list.int[type#3] value elements=[%int#4]
//     %string#1:shape#8(String) = string.value "one"
//     %list.string#0:shape#21(list_type#4) = list.string[type#4] value elements=[%string#1]
//     %int#5:shape#1(Int) = int.value 1
//     %bit_array#1:shape#9(BitArray) = bit_array.value [int(%int#5, bits=8, big)]
//     %list.bit_array#0:shape#22(list_type#5) = list.bit_array[type#5] value elements=[%bit_array#1]
//     %list.utf_codepoint#0:shape#23(list_type#6) = list.utf_codepoint[type#6] value elements=[%utf_codepoint#0]
//     %int#6:shape#1(Int) = int.value 1
//     %custom#1:shape#11(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#6]
//     %list.custom#0:shape#24(list_type#7) = list.custom[type#7] value elements=[%custom#1]
//     %float#1:shape#7(Float) = float.value 1.0
//     %list.float#0:shape#25(list_type#8) = list.float[type#8] value elements=[%float#1]
//     %bool#1:shape#12(Bool) = bool.value True
//     %list.bool#0:shape#26(list_type#9) = list.bool[type#9] value elements=[%bool#1]
//     %nil#1:shape#13(Nil) = nil.value
//     %list.nil#0:shape#27(list_type#10) = list.nil[type#10] value elements=[%nil#1]
//     %int#7:shape#1(Int) = int.value 1
//     %tuple#1:shape#14(#(Int)) = tuple.value elements=[%int#7]
//     %list.tuple#0:shape#28(list_type#11) = list.tuple[type#11] value elements=[%tuple#1]
//     %int#8:shape#1(Int) = int.value 1
//     %list.int#1:shape#20(list_type#3) = list.int[type#3] value elements=[%int#8]
//     %list.list#0:shape#29(list_type#12) = list.list[type#12] value elements=[%list.int#1]
//     %function.int#0:shape#2(fn() -> Int) = function[Int] reference int#0
//     %list.function#0:shape#30(list_type#13) = list.function[type#13] value elements=[%function.int#0]
//     %function.generic#0:shape#31(fn(param#2) -> param#2) = function[Generic] reference template#4 shapes=[shape#0]
//     %list.function#1:shape#32(list_type#14) = list.function[type#14] value elements=[%function.generic#0]
//     %function.never#0:shape#34(fn() -> param#3) = function[Never] reference never#0
//     %list.function#2:shape#35(list_type#15) = list.function[type#15] value elements=[%function.never#0]
//     %function.float#0:shape#36(fn() -> Float) = function[Float] reference float#0
//     %list.function#3:shape#37(list_type#16) = list.function[type#16] value elements=[%function.float#0]
//     %function.string#0:shape#38(fn() -> String) = function[String] reference string#0
//     %list.function#4:shape#39(list_type#17) = list.function[type#17] value elements=[%function.string#0]
//     %function.bit_array#0:shape#40(fn() -> BitArray) = function[BitArray] reference bit_array#0
//     %list.function#5:shape#41(list_type#18) = list.function[type#18] value elements=[%function.bit_array#0]
//     %function.utf_codepoint#0:shape#42(fn() -> UtfCodepoint) = function[UtfCodepoint] reference utf_codepoint#1
//     %list.function#6:shape#43(list_type#19) = list.function[type#19] value elements=[%function.utf_codepoint#0]
//     %function.custom#0:shape#45(fn() -> custom_type#0) = function[Custom] reference custom#0
//     %list.function#7:shape#46(list_type#20) = list.function[type#20] value elements=[%function.custom#0]
//     %function.bool#0:shape#47(fn() -> Bool) = function[Bool] reference bool#0
//     %list.function#8:shape#48(list_type#21) = list.function[type#21] value elements=[%function.bool#0]
//     %function.nil#0:shape#49(fn() -> Nil) = function[Nil] reference nil#0
//     %list.function#9:shape#50(list_type#22) = list.function[type#22] value elements=[%function.nil#0]
//     %function.tuple#0:shape#51(fn() -> #(Int)) = function[Tuple] reference tuple#0
//     %list.function#10:shape#52(list_type#23) = list.function[type#23] value elements=[%function.tuple#0]
//     %function.list.int#0:shape#53(fn() -> list_type#3) = function[List] reference list.int#0
//     %list.function#11:shape#54(list_type#24) = list.function[type#24] value elements=[%function.list.int#0]
//     %function.function#0:shape#55(fn() -> fn() -> Int) = function[Function] reference function.int#0
//     %list.function#12:shape#56(list_type#25) = list.function[type#25] value elements=[%function.function#0]
//     %function.generic#1:shape#4(fn(param#4) -> param#4) = function[Generic] reference template#4 shapes=[shape#3]
//     %function.never#1:shape#6(fn() -> param#5) = function[Never] reference never#1
//     %function.int#1:shape#2(fn() -> Int) = function[Int] reference int#0
//     %function.float#1:shape#36(fn() -> Float) = function[Float] reference float#0
//     %function.string#1:shape#38(fn() -> String) = function[String] reference string#0
//     %function.bit_array#1:shape#40(fn() -> BitArray) = function[BitArray] reference bit_array#0
//     %function.utf_codepoint#1:shape#42(fn() -> UtfCodepoint) = function[UtfCodepoint] reference utf_codepoint#1
//     %function.custom#1:shape#45(fn() -> custom_type#0) = function[Custom] reference custom#0
//     %function.bool#1:shape#47(fn() -> Bool) = function[Bool] reference bool#0
//     %function.nil#1:shape#49(fn() -> Nil) = function[Nil] reference nil#0
//     %function.tuple#1:shape#51(fn() -> #(Int)) = function[Tuple] reference tuple#0
//     %function.list.int#1:shape#53(fn() -> list_type#3) = function[List] reference list.int#0
//     %function.function#1:shape#55(fn() -> fn() -> Int) = function[Function] reference function.int#0
//     %function.tuple#2:shape#59(fn() -> #(Int, Float, String, BitArray, UtfCodepoint, custom_type#0, Bool, Nil, #(Int), list_type#0, list_type#2, list_type#3, list_type#4, list_type#5, list_type#6, list_type#7, list_type#8, list_type#9, list_type#10, list_type#11, list_type#12, list_type#13, list_type#14, list_type#15, list_type#16, list_type#17, list_type#18, list_type#19, list_type#20, list_type#21, list_type#22, list_type#23, list_type#24, list_type#25, fn(param#4) -> param#4, fn() -> param#5, fn() -> Int, fn() -> Float, fn() -> String, fn() -> BitArray, fn() -> UtfCodepoint, fn() -> custom_type#0, fn() -> Bool, fn() -> Nil, fn() -> #(Int), fn() -> list_type#3, fn() -> fn() -> Int)) = function[Tuple] closure target=tuple#1 captures=[%int#0<-%int#0, %float#0<-%float#0, %string#0<-%string#0, %bit_array#0<-%bit_array#0, %utf_codepoint#0<-%utf_codepoint#0, %custom#0<-%custom#0, %bool#0<-%bool#0, %nil#0<-%nil#0, %tuple#0<-%tuple#0, %list.parameter#0<-%list.parameter#0, %list.parameter_list#0<-%list.parameter_list#0, %list.int#0<-%list.int#0, %list.string#0<-%list.string#0, %list.bit_array#0<-%list.bit_array#0, %list.utf_codepoint#0<-%list.utf_codepoint#0, %list.custom#0<-%list.custom#0, %list.float#0<-%list.float#0, %list.bool#0<-%list.bool#0, %list.nil#0<-%list.nil#0, %list.tuple#0<-%list.tuple#0, %list.list#0<-%list.list#0, %list.function#0<-%list.function#0, %list.function#1<-%list.function#1, %list.function#2<-%list.function#2, %list.function#3<-%list.function#3, %list.function#4<-%list.function#4, %list.function#5<-%list.function#5, %list.function#6<-%list.function#6, %list.function#7<-%list.function#7, %list.function#8<-%list.function#8, %list.function#9<-%list.function#9, %list.function#10<-%list.function#10, %list.function#11<-%list.function#11, %list.function#12<-%list.function#12, %function.generic#0<-%function.generic#1, %function.never#0<-%function.never#1, %function.int#0<-%function.int#1, %function.float#0<-%function.float#1, %function.string#0<-%function.string#1, %function.bit_array#0<-%function.bit_array#1, %function.utf_codepoint#0<-%function.utf_codepoint#1, %function.custom#0<-%function.custom#1, %function.bool#0<-%function.bool#1, %function.nil#0<-%function.nil#1, %function.tuple#0<-%function.tuple#1, %function.list.int#0<-%function.list.int#1, %function.function#0<-%function.function#1]
//     return %function.tuple#2
