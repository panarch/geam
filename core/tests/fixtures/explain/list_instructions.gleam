pub type Lists(item) {
  Lists(values: List(item))
}

pub type Item(item) {
  Item(value: item)
}

fn identity(values: List(item)) {
  values
}

fn identity_value(value: item) {
  value
}

fn operations(
  value: item,
  values: List(item),
  nested: List(List(item)),
  record: Lists(item),
  provider: fn() -> List(item),
) {
  let projected = case nested {
    [first] -> first
    _ -> []
  }
  let dropped = case values {
    [_, ..tail] -> tail
    _ -> []
  }
  let selected = case values {
    [first, ..] -> first
    _ -> value
  }
  let direct = identity_value(value)
  let scalar_provider = fn() { value }
  #(
    [value],
    [value, ..values],
    identity(values),
    scalar_provider(),
    #(values).0,
    record.values,
    projected,
    dropped,
    direct,
    provider(),
    #(value).0,
    Item(value).value,
    selected,
  )
}

pub type Marker {
  Marker(Int)
}

fn codepoint() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn add_one(value: Int) { value + 1 }

fn ints() { [1] }
fn strings() { ["one"] }
fn bit_arrays() { [<<1>>] }
fn codepoints() { [codepoint()] }
fn customs() { [Marker(1)] }
fn floats() { [1.0] }
fn bools() { [True] }
fn nils() { [Nil] }
fn tuples() { [#(1)] }
fn lists() { [[1]] }
fn functions() { [add_one] }
fn constructors() { [Marker] }

pub fn main() {
  #(
    operations(1, [2], [[3]], Lists([4]), ints),
    operations("one", ["two"], [["three"]], Lists(["four"]), strings),
    operations(<<1>>, [<<2>>], [[<<3>>]], Lists([<<4>>]), bit_arrays),
    operations(codepoint(), [codepoint()], [[codepoint()]], Lists([codepoint()]), codepoints),
    operations(Marker(1), [Marker(2)], [[Marker(3)]], Lists([Marker(4)]), customs),
    operations(1.0, [2.0], [[3.0]], Lists([4.0]), floats),
    operations(True, [False], [[True]], Lists([False]), bools),
    operations(Nil, [Nil], [[Nil]], Lists([Nil]), nils),
    operations(#(1), [#(2)], [[#(3)]], Lists([#(4)]), tuples),
    operations([1], [[2]], [[[3]]], Lists([[4]]), lists),
    operations(add_one, [add_one], [[add_one]], Lists([add_one]), functions),
    operations(Marker, [Marker], [[Marker]], Lists([Marker]), constructors),
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
//   entry b0 params=[%int#0:shape#0(Int)] captures=[]
//   block b0 params=[%int#0:shape#0(Int)]
//     return %int#0
//
// function int#2
//   entry b0 params=[] captures=[%int#0:shape#0(Int)]
//   block b0 params=[%int#0:shape#0(Int)]
//     return %int#0
//
// function float#0
//   entry b0 params=[%float#0:shape#32(Float)] captures=[]
//   block b0 params=[%float#0:shape#32(Float)]
//     return %float#0
//
// function float#1
//   entry b0 params=[] captures=[%float#0:shape#32(Float)]
//   block b0 params=[%float#0:shape#32(Float)]
//     return %float#0
//
// function string#0
//   entry b0 params=[%string#0:shape#6(String)] captures=[]
//   block b0 params=[%string#0:shape#6(String)]
//     return %string#0
//
// function string#1
//   entry b0 params=[] captures=[%string#0:shape#6(String)]
//   block b0 params=[%string#0:shape#6(String)]
//     return %string#0
//
// function bit_array#0
//   entry b0 params=[%bit_array#0:shape#12(BitArray)] captures=[]
//   block b0 params=[%bit_array#0:shape#12(BitArray)]
//     return %bit_array#0
//
// function bit_array#1
//   entry b0 params=[] captures=[%bit_array#0:shape#12(BitArray)]
//   block b0 params=[%bit_array#0:shape#12(BitArray)]
//     return %bit_array#0
//
// function utf_codepoint#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 65
//     %bit_array#0:shape#12(BitArray) = bit_array.value [int(%int#0, bits=8, big)]
//     match %bit_array#0 pattern=<<utf_codepoint(binding#0, utf8)>> success=b1(binding#0) failure=b2(%bit_array#0)
//   block b1 params=[%utf_codepoint#0:shape#18(UtfCodepoint)]
//     return %utf_codepoint#0
//   block b2 params=[%bit_array#0:shape#12(BitArray)]
//     let_assert_panic subject=%bit_array#0 message=none
//
// function utf_codepoint#1
//   entry b0 params=[%utf_codepoint#0:shape#18(UtfCodepoint)] captures=[]
//   block b0 params=[%utf_codepoint#0:shape#18(UtfCodepoint)]
//     return %utf_codepoint#0
//
// function utf_codepoint#2
//   entry b0 params=[] captures=[%utf_codepoint#0:shape#18(UtfCodepoint)]
//   block b0 params=[%utf_codepoint#0:shape#18(UtfCodepoint)]
//     return %utf_codepoint#0
//
// function custom#0
//   entry b0 params=[%custom#0:shape#28(custom_type#0)] captures=[]
//   block b0 params=[%custom#0:shape#28(custom_type#0)]
//     return %custom#0
//
// function custom#1
//   entry b0 params=[] captures=[%custom#0:shape#28(custom_type#0)]
//   block b0 params=[%custom#0:shape#28(custom_type#0)]
//     return %custom#0
//
// function bool#0
//   entry b0 params=[%bool#0:shape#38(Bool)] captures=[]
//   block b0 params=[%bool#0:shape#38(Bool)]
//     return %bool#0
//
// function bool#1
//   entry b0 params=[] captures=[%bool#0:shape#38(Bool)]
//   block b0 params=[%bool#0:shape#38(Bool)]
//     return %bool#0
//
// function nil#0
//   entry b0 params=[%nil#0:shape#44(Nil)] captures=[]
//   block b0 params=[%nil#0:shape#44(Nil)]
//     return %nil#0
//
// function nil#1
//   entry b0 params=[] captures=[%nil#0:shape#44(Nil)]
//   block b0 params=[%nil#0:shape#44(Nil)]
//     return %nil#0
//
// function tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %int#1:shape#0(Int) = int.value 2
//     %list.int#0:shape#1(list_type#0) = list.int[type#0] value elements=[%int#1]
//     %int#2:shape#0(Int) = int.value 3
//     %list.int#1:shape#1(list_type#0) = list.int[type#0] value elements=[%int#2]
//     %list.list#0:shape#2(list_type#9) = list.list[type#9] value elements=[%list.int#1]
//     %int#3:shape#0(Int) = int.value 4
//     %list.int#2:shape#1(list_type#0) = list.int[type#0] value elements=[%int#3]
//     %custom#0:shape#3(custom_type#1) = custom.construct custom_type#1.constructor#0 fields=[%list.int#2]
//     %function.list.int#0:shape#4(fn() -> list_type#0) = function[List] reference list.int#0
//     %tuple#0:shape#5(#(list_type#0, list_type#0, list_type#0, Int, list_type#0, list_type#0, list_type#0, list_type#0, Int, list_type#0, Int, Int, Int)) = tuple.call tuple#1 args=[%int#0, %list.int#0, %list.list#0, %custom#0, %function.list.int#0]
//     %string#0:shape#6(String) = string.value "one"
//     %string#1:shape#6(String) = string.value "two"
//     %list.string#0:shape#7(list_type#1) = list.string[type#1] value elements=[%string#1]
//     %string#2:shape#6(String) = string.value "three"
//     %list.string#1:shape#7(list_type#1) = list.string[type#1] value elements=[%string#2]
//     %list.list#1:shape#8(list_type#12) = list.list[type#12] value elements=[%list.string#1]
//     %string#3:shape#6(String) = string.value "four"
//     %list.string#2:shape#7(list_type#1) = list.string[type#1] value elements=[%string#3]
//     %custom#1:shape#9(custom_type#2) = custom.construct custom_type#2.constructor#0 fields=[%list.string#2]
//     %function.list.string#0:shape#10(fn() -> list_type#1) = function[List] reference list.string#0
//     %tuple#1:shape#11(#(list_type#1, list_type#1, list_type#1, String, list_type#1, list_type#1, list_type#1, list_type#1, String, list_type#1, String, String, String)) = tuple.call tuple#2 args=[%string#0, %list.string#0, %list.list#1, %custom#1, %function.list.string#0]
//     %int#4:shape#0(Int) = int.value 1
//     %bit_array#0:shape#12(BitArray) = bit_array.value [int(%int#4, bits=8, big)]
//     %int#5:shape#0(Int) = int.value 2
//     %bit_array#1:shape#12(BitArray) = bit_array.value [int(%int#5, bits=8, big)]
//     %list.bit_array#0:shape#13(list_type#2) = list.bit_array[type#2] value elements=[%bit_array#1]
//     %int#6:shape#0(Int) = int.value 3
//     %bit_array#2:shape#12(BitArray) = bit_array.value [int(%int#6, bits=8, big)]
//     %list.bit_array#1:shape#13(list_type#2) = list.bit_array[type#2] value elements=[%bit_array#2]
//     %list.list#2:shape#14(list_type#13) = list.list[type#13] value elements=[%list.bit_array#1]
//     %int#7:shape#0(Int) = int.value 4
//     %bit_array#3:shape#12(BitArray) = bit_array.value [int(%int#7, bits=8, big)]
//     %list.bit_array#2:shape#13(list_type#2) = list.bit_array[type#2] value elements=[%bit_array#3]
//     %custom#2:shape#15(custom_type#3) = custom.construct custom_type#3.constructor#0 fields=[%list.bit_array#2]
//     %function.list.bit_array#0:shape#16(fn() -> list_type#2) = function[List] reference list.bit_array#0
//     %tuple#2:shape#17(#(list_type#2, list_type#2, list_type#2, BitArray, list_type#2, list_type#2, list_type#2, list_type#2, BitArray, list_type#2, BitArray, BitArray, BitArray)) = tuple.call tuple#3 args=[%bit_array#0, %list.bit_array#0, %list.list#2, %custom#2, %function.list.bit_array#0]
//     %utf_codepoint#0:shape#18(UtfCodepoint) = utf_codepoint.call utf_codepoint#0 args=[]
//     %utf_codepoint#1:shape#18(UtfCodepoint) = utf_codepoint.call utf_codepoint#0 args=[]
//     %list.utf_codepoint#0:shape#19(list_type#3) = list.utf_codepoint[type#3] value elements=[%utf_codepoint#1]
//     %utf_codepoint#2:shape#18(UtfCodepoint) = utf_codepoint.call utf_codepoint#0 args=[]
//     %list.utf_codepoint#1:shape#19(list_type#3) = list.utf_codepoint[type#3] value elements=[%utf_codepoint#2]
//     %list.list#3:shape#20(list_type#14) = list.list[type#14] value elements=[%list.utf_codepoint#1]
//     %utf_codepoint#3:shape#18(UtfCodepoint) = utf_codepoint.call utf_codepoint#0 args=[]
//     %list.utf_codepoint#2:shape#19(list_type#3) = list.utf_codepoint[type#3] value elements=[%utf_codepoint#3]
//     %custom#3:shape#21(custom_type#4) = custom.construct custom_type#4.constructor#0 fields=[%list.utf_codepoint#2]
//     %function.list.utf_codepoint#0:shape#22(fn() -> list_type#3) = function[List] reference list.utf_codepoint#0
//     %tuple#3:shape#23(#(list_type#3, list_type#3, list_type#3, UtfCodepoint, list_type#3, list_type#3, list_type#3, list_type#3, UtfCodepoint, list_type#3, UtfCodepoint, UtfCodepoint, UtfCodepoint)) = tuple.call tuple#4 args=[%utf_codepoint#0, %list.utf_codepoint#0, %list.list#3, %custom#3, %function.list.utf_codepoint#0]
//     %int#8:shape#0(Int) = int.value 1
//     %custom#4:shape#24(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#8]
//     %int#9:shape#0(Int) = int.value 2
//     %custom#5:shape#24(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#9]
//     %list.custom#0:shape#25(list_type#4) = list.custom[type#4] value elements=[%custom#5]
//     %int#10:shape#0(Int) = int.value 3
//     %custom#6:shape#24(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#10]
//     %list.custom#1:shape#25(list_type#4) = list.custom[type#4] value elements=[%custom#6]
//     %list.list#4:shape#26(list_type#23) = list.list[type#15] value elements=[%list.custom#1]
//     %int#11:shape#0(Int) = int.value 4
//     %custom#7:shape#24(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#11]
//     %list.custom#2:shape#25(list_type#4) = list.custom[type#4] value elements=[%custom#7]
//     %custom#8:shape#27(custom_type#5) = custom.construct custom_type#5.constructor#0 fields=[%list.custom#2]
//     %function.list.custom#0:shape#30(fn() -> list_type#4) = function[List] reference list.custom#0
//     %tuple#4:shape#31(#(list_type#4, list_type#4, list_type#4, custom_type#0, list_type#4, list_type#4, list_type#4, list_type#4, custom_type#0, list_type#4, custom_type#0, custom_type#0, custom_type#0)) = tuple.call tuple#5 args=[%custom#4, %list.custom#0, %list.list#4, %custom#8, %function.list.custom#0]
//     %float#0:shape#32(Float) = float.value 1.0
//     %float#1:shape#32(Float) = float.value 2.0
//     %list.float#0:shape#33(list_type#5) = list.float[type#5] value elements=[%float#1]
//     %float#2:shape#32(Float) = float.value 3.0
//     %list.float#1:shape#33(list_type#5) = list.float[type#5] value elements=[%float#2]
//     %list.list#5:shape#34(list_type#16) = list.list[type#16] value elements=[%list.float#1]
//     %float#3:shape#32(Float) = float.value 4.0
//     %list.float#2:shape#33(list_type#5) = list.float[type#5] value elements=[%float#3]
//     %custom#9:shape#35(custom_type#6) = custom.construct custom_type#6.constructor#0 fields=[%list.float#2]
//     %function.list.float#0:shape#36(fn() -> list_type#5) = function[List] reference list.float#0
//     %tuple#5:shape#37(#(list_type#5, list_type#5, list_type#5, Float, list_type#5, list_type#5, list_type#5, list_type#5, Float, list_type#5, Float, Float, Float)) = tuple.call tuple#6 args=[%float#0, %list.float#0, %list.list#5, %custom#9, %function.list.float#0]
//     %bool#0:shape#38(Bool) = bool.value True
//     %bool#1:shape#38(Bool) = bool.value False
//     %list.bool#0:shape#39(list_type#6) = list.bool[type#6] value elements=[%bool#1]
//     %bool#2:shape#38(Bool) = bool.value True
//     %list.bool#1:shape#39(list_type#6) = list.bool[type#6] value elements=[%bool#2]
//     %list.list#6:shape#40(list_type#17) = list.list[type#17] value elements=[%list.bool#1]
//     %bool#3:shape#38(Bool) = bool.value False
//     %list.bool#2:shape#39(list_type#6) = list.bool[type#6] value elements=[%bool#3]
//     %custom#10:shape#41(custom_type#7) = custom.construct custom_type#7.constructor#0 fields=[%list.bool#2]
//     %function.list.bool#0:shape#42(fn() -> list_type#6) = function[List] reference list.bool#0
//     %tuple#6:shape#43(#(list_type#6, list_type#6, list_type#6, Bool, list_type#6, list_type#6, list_type#6, list_type#6, Bool, list_type#6, Bool, Bool, Bool)) = tuple.call tuple#7 args=[%bool#0, %list.bool#0, %list.list#6, %custom#10, %function.list.bool#0]
//     %nil#0:shape#44(Nil) = nil.value
//     %nil#1:shape#44(Nil) = nil.value
//     %list.nil#0:shape#45(list_type#7) = list.nil[type#7] value elements=[%nil#1]
//     %nil#2:shape#44(Nil) = nil.value
//     %list.nil#1:shape#45(list_type#7) = list.nil[type#7] value elements=[%nil#2]
//     %list.list#7:shape#46(list_type#18) = list.list[type#18] value elements=[%list.nil#1]
//     %nil#3:shape#44(Nil) = nil.value
//     %list.nil#2:shape#45(list_type#7) = list.nil[type#7] value elements=[%nil#3]
//     %custom#11:shape#47(custom_type#8) = custom.construct custom_type#8.constructor#0 fields=[%list.nil#2]
//     %function.list.nil#0:shape#48(fn() -> list_type#7) = function[List] reference list.nil#0
//     %tuple#7:shape#49(#(list_type#7, list_type#7, list_type#7, Nil, list_type#7, list_type#7, list_type#7, list_type#7, Nil, list_type#7, Nil, Nil, Nil)) = tuple.call tuple#8 args=[%nil#0, %list.nil#0, %list.list#7, %custom#11, %function.list.nil#0]
//     %int#12:shape#0(Int) = int.value 1
//     %tuple#8:shape#50(#(Int)) = tuple.value elements=[%int#12]
//     %int#13:shape#0(Int) = int.value 2
//     %tuple#9:shape#50(#(Int)) = tuple.value elements=[%int#13]
//     %list.tuple#0:shape#51(list_type#8) = list.tuple[type#8] value elements=[%tuple#9]
//     %int#14:shape#0(Int) = int.value 3
//     %tuple#10:shape#50(#(Int)) = tuple.value elements=[%int#14]
//     %list.tuple#1:shape#51(list_type#8) = list.tuple[type#8] value elements=[%tuple#10]
//     %list.list#8:shape#52(list_type#19) = list.list[type#19] value elements=[%list.tuple#1]
//     %int#15:shape#0(Int) = int.value 4
//     %tuple#11:shape#50(#(Int)) = tuple.value elements=[%int#15]
//     %list.tuple#2:shape#51(list_type#8) = list.tuple[type#8] value elements=[%tuple#11]
//     %custom#12:shape#53(custom_type#9) = custom.construct custom_type#9.constructor#0 fields=[%list.tuple#2]
//     %function.list.tuple#0:shape#54(fn() -> list_type#8) = function[List] reference list.tuple#0
//     %tuple#12:shape#55(#(list_type#8, list_type#8, list_type#8, #(Int), list_type#8, list_type#8, list_type#8, list_type#8, #(Int), list_type#8, #(Int), #(Int), #(Int))) = tuple.call tuple#9 args=[%tuple#8, %list.tuple#0, %list.list#8, %custom#12, %function.list.tuple#0]
//     %int#16:shape#0(Int) = int.value 1
//     %list.int#3:shape#1(list_type#0) = list.int[type#0] value elements=[%int#16]
//     %int#17:shape#0(Int) = int.value 2
//     %list.int#4:shape#1(list_type#0) = list.int[type#0] value elements=[%int#17]
//     %list.list#9:shape#2(list_type#9) = list.list[type#9] value elements=[%list.int#4]
//     %int#18:shape#0(Int) = int.value 3
//     %list.int#5:shape#1(list_type#0) = list.int[type#0] value elements=[%int#18]
//     %list.list#10:shape#2(list_type#9) = list.list[type#9] value elements=[%list.int#5]
//     %list.list#11:shape#56(list_type#20) = list.list[type#20] value elements=[%list.list#10]
//     %int#19:shape#0(Int) = int.value 4
//     %list.int#6:shape#1(list_type#0) = list.int[type#0] value elements=[%int#19]
//     %list.list#12:shape#2(list_type#9) = list.list[type#9] value elements=[%list.int#6]
//     %custom#13:shape#57(custom_type#10) = custom.construct custom_type#10.constructor#0 fields=[%list.list#12]
//     %function.list.list#0:shape#58(fn() -> list_type#9) = function[List] reference list.list#0
//     %tuple#13:shape#59(#(list_type#9, list_type#9, list_type#9, list_type#0, list_type#9, list_type#9, list_type#9, list_type#9, list_type#0, list_type#9, list_type#0, list_type#0, list_type#0)) = tuple.call tuple#10 args=[%list.int#3, %list.list#9, %list.list#11, %custom#13, %function.list.list#0]
//     %function.int#0:shape#60(fn(Int) -> Int) = function[Int] reference int#0
//     %function.int#1:shape#60(fn(Int) -> Int) = function[Int] reference int#0
//     %list.function#0:shape#61(list_type#10) = list.function[type#10] value elements=[%function.int#1]
//     %function.int#2:shape#60(fn(Int) -> Int) = function[Int] reference int#0
//     %list.function#1:shape#61(list_type#10) = list.function[type#10] value elements=[%function.int#2]
//     %list.list#13:shape#62(list_type#21) = list.list[type#21] value elements=[%list.function#1]
//     %function.int#3:shape#60(fn(Int) -> Int) = function[Int] reference int#0
//     %list.function#2:shape#61(list_type#10) = list.function[type#10] value elements=[%function.int#3]
//     %custom#14:shape#63(custom_type#11) = custom.construct custom_type#11.constructor#0 fields=[%list.function#2]
//     %function.list.function#0:shape#64(fn() -> list_type#10) = function[List] reference list.function#0
//     %tuple#14:shape#65(#(list_type#10, list_type#10, list_type#10, fn(Int) -> Int, list_type#10, list_type#10, list_type#10, list_type#10, fn(Int) -> Int, list_type#10, fn(Int) -> Int, fn(Int) -> Int, fn(Int) -> Int)) = tuple.call tuple#11 args=[%function.int#0, %list.function#0, %list.list#13, %custom#14, %function.list.function#0]
//     %function.custom#0:shape#66(fn(Int) -> custom_type#0) = function[Custom] constructor custom_type#0.constructor#0
//     %function.custom#1:shape#66(fn(Int) -> custom_type#0) = function[Custom] constructor custom_type#0.constructor#0
//     %list.function#3:shape#68(list_type#24) = list.function[type#11] value elements=[%function.custom#1]
//     %function.custom#2:shape#66(fn(Int) -> custom_type#0) = function[Custom] constructor custom_type#0.constructor#0
//     %list.function#4:shape#68(list_type#24) = list.function[type#11] value elements=[%function.custom#2]
//     %list.list#14:shape#69(list_type#25) = list.list[type#22] value elements=[%list.function#4]
//     %function.custom#3:shape#66(fn(Int) -> custom_type#0) = function[Custom] constructor custom_type#0.constructor#0
//     %list.function#5:shape#68(list_type#24) = list.function[type#11] value elements=[%function.custom#3]
//     %custom#15:shape#70(custom_type#12) = custom.construct custom_type#12.constructor#0 fields=[%list.function#5]
//     %function.list.function#1:shape#72(fn() -> list_type#11) = function[List] reference list.function#1
//     %tuple#15:shape#73(#(list_type#11, list_type#11, list_type#11, fn(Int) -> custom_type#0, list_type#11, list_type#11, list_type#11, list_type#11, fn(Int) -> custom_type#0, list_type#11, fn(Int) -> custom_type#0, fn(Int) -> custom_type#0, fn(Int) -> custom_type#0)) = tuple.call tuple#12 args=[%function.custom#0, %list.function#3, %list.list#14, %custom#15, %function.list.function#1]
//     %tuple#16:shape#74(#(#(list_type#0, list_type#0, list_type#0, Int, list_type#0, list_type#0, list_type#0, list_type#0, Int, list_type#0, Int, Int, Int), #(list_type#1, list_type#1, list_type#1, String, list_type#1, list_type#1, list_type#1, list_type#1, String, list_type#1, String, String, String), #(list_type#2, list_type#2, list_type#2, BitArray, list_type#2, list_type#2, list_type#2, list_type#2, BitArray, list_type#2, BitArray, BitArray, BitArray), #(list_type#3, list_type#3, list_type#3, UtfCodepoint, list_type#3, list_type#3, list_type#3, list_type#3, UtfCodepoint, list_type#3, UtfCodepoint, UtfCodepoint, UtfCodepoint), #(list_type#4, list_type#4, list_type#4, custom_type#0, list_type#4, list_type#4, list_type#4, list_type#4, custom_type#0, list_type#4, custom_type#0, custom_type#0, custom_type#0), #(list_type#5, list_type#5, list_type#5, Float, list_type#5, list_type#5, list_type#5, list_type#5, Float, list_type#5, Float, Float, Float), #(list_type#6, list_type#6, list_type#6, Bool, list_type#6, list_type#6, list_type#6, list_type#6, Bool, list_type#6, Bool, Bool, Bool), #(list_type#7, list_type#7, list_type#7, Nil, list_type#7, list_type#7, list_type#7, list_type#7, Nil, list_type#7, Nil, Nil, Nil), #(list_type#8, list_type#8, list_type#8, #(Int), list_type#8, list_type#8, list_type#8, list_type#8, #(Int), list_type#8, #(Int), #(Int), #(Int)), #(list_type#9, list_type#9, list_type#9, list_type#0, list_type#9, list_type#9, list_type#9, list_type#9, list_type#0, list_type#9, list_type#0, list_type#0, list_type#0), #(list_type#10, list_type#10, list_type#10, fn(Int) -> Int, list_type#10, list_type#10, list_type#10, list_type#10, fn(Int) -> Int, list_type#10, fn(Int) -> Int, fn(Int) -> Int, fn(Int) -> Int), #(list_type#11, list_type#11, list_type#11, fn(Int) -> custom_type#0, list_type#11, list_type#11, list_type#11, list_type#11, fn(Int) -> custom_type#0, list_type#11, fn(Int) -> custom_type#0, fn(Int) -> custom_type#0, fn(Int) -> custom_type#0))) = tuple.value elements=[%tuple#0, %tuple#1, %tuple#2, %tuple#3, %tuple#4, %tuple#5, %tuple#6, %tuple#7, %tuple#12, %tuple#13, %tuple#14, %tuple#15]
//     return %tuple#16
//
// function tuple#1
//   entry b0 params=[%int#0:shape#0(Int), %list.int#0:shape#1(list_type#0), %list.list#0:shape#2(list_type#9), %custom#0:shape#75(custom_type#1), %function.list.int#0:shape#4(fn() -> list_type#0)] captures=[]
//   block b0 params=[%int#0:shape#0(Int), %list.int#0:shape#1(list_type#0), %list.list#0:shape#2(list_type#9), %custom#0:shape#75(custom_type#1), %function.list.int#0:shape#4(fn() -> list_type#0)]
//     %bool#0:shape#38(Bool) = bool.list_length_equals %list.list#0 length=1
//     branch %bool#0 true=b1(%int#0, %list.int#0, %list.list#0, %custom#0, %function.list.int#0) false=b9(%int#0, %list.int#0, %custom#0, %function.list.int#0)
//   block b1 params=[%int#0:shape#0(Int), %list.int#0:shape#1(list_type#0), %list.list#0:shape#2(list_type#9), %custom#0:shape#75(custom_type#1), %function.list.int#0:shape#4(fn() -> list_type#0)]
//     %list.int#1:shape#1(list_type#0) = list.int[type#0] list_index %list.list#0 index=0
//     jump b2(%list.int#1, %int#0, %list.int#0, %custom#0, %function.list.int#0)
//   block b2 params=[%list.int#0:shape#1(list_type#0), %int#0:shape#0(Int), %list.int#1:shape#1(list_type#0), %custom#0:shape#75(custom_type#1), %function.list.int#0:shape#4(fn() -> list_type#0)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.int#1 length=1
//     branch %bool#0 true=b3(%int#0, %list.int#1, %custom#0, %function.list.int#0, %list.int#0) false=b8(%int#0, %list.int#1, %custom#0, %function.list.int#0, %list.int#0)
//   block b3 params=[%int#0:shape#0(Int), %list.int#0:shape#1(list_type#0), %custom#0:shape#75(custom_type#1), %function.list.int#0:shape#4(fn() -> list_type#0), %list.int#1:shape#1(list_type#0)]
//     %list.int#2:shape#1(list_type#0) = list.int[type#0] drop_first %list.int#0 count=1
//     jump b4(%list.int#2, %int#0, %list.int#0, %custom#0, %function.list.int#0, %list.int#1)
//   block b4 params=[%list.int#0:shape#1(list_type#0), %int#0:shape#0(Int), %list.int#1:shape#1(list_type#0), %custom#0:shape#75(custom_type#1), %function.list.int#0:shape#4(fn() -> list_type#0), %list.int#2:shape#1(list_type#0)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.int#1 length=1
//     branch %bool#0 true=b5(%int#0, %list.int#1, %custom#0, %function.list.int#0, %list.int#2, %list.int#0) false=b7(%int#0, %list.int#1, %custom#0, %function.list.int#0, %list.int#2, %list.int#0)
//   block b5 params=[%int#0:shape#0(Int), %list.int#0:shape#1(list_type#0), %custom#0:shape#75(custom_type#1), %function.list.int#0:shape#4(fn() -> list_type#0), %list.int#1:shape#1(list_type#0), %list.int#2:shape#1(list_type#0)]
//     %int#1:shape#0(Int) = int.list_index %list.int#0 index=0
//     jump b6(%int#1, %int#0, %list.int#0, %custom#0, %function.list.int#0, %list.int#1, %list.int#2)
//   block b6 params=[%int#0:shape#0(Int), %int#1:shape#0(Int), %list.int#0:shape#1(list_type#0), %custom#0:shape#75(custom_type#1), %function.list.int#0:shape#4(fn() -> list_type#0), %list.int#1:shape#1(list_type#0), %list.int#2:shape#1(list_type#0)]
//     %int#2:shape#0(Int) = int.call int#1 args=[%int#1]
//     %function.int#0:shape#76(fn() -> Int) = function[Int] closure target=int#2 captures=[%int#0<-%int#1]
//     %list.int#3:shape#1(list_type#0) = list.int[type#0] value elements=[%int#1]
//     %list.int#4:shape#1(list_type#0) = list.int[type#0] spread elements=[%int#1] tail=%list.int#0
//     %list.int#5:shape#1(list_type#0) = list.int[type#0] call list.int#1 args=[%list.int#0]
//     %int#3:shape#0(Int) = int.function_call %function.int#0 args=[]
//     %tuple#0:shape#77(#(list_type#0)) = tuple.value elements=[%list.int#0]
//     %list.int#6:shape#1(list_type#0) = list.int[type#0] tuple_index %tuple#0 index=0
//     %list.int#7:shape#1(list_type#0) = list.int[type#0] custom_field %custom#0 index=0
//     %list.int#8:shape#1(list_type#0) = list.int[type#0] function_call %function.list.int#0 args=[]
//     %tuple#1:shape#50(#(Int)) = tuple.value elements=[%int#1]
//     %int#4:shape#0(Int) = int.tuple_index %tuple#1 index=0
//     %custom#1:shape#78(custom_type#13) = custom.construct custom_type#13.constructor#0 fields=[%int#1]
//     %int#5:shape#0(Int) = int.custom_field %custom#1 index=0
//     %tuple#2:shape#5(#(list_type#0, list_type#0, list_type#0, Int, list_type#0, list_type#0, list_type#0, list_type#0, Int, list_type#0, Int, Int, Int)) = tuple.value elements=[%list.int#3, %list.int#4, %list.int#5, %int#3, %list.int#6, %list.int#7, %list.int#1, %list.int#2, %int#2, %list.int#8, %int#4, %int#5, %int#0]
//     return %tuple#2
//   block b7 params=[%int#0:shape#0(Int), %list.int#0:shape#1(list_type#0), %custom#0:shape#75(custom_type#1), %function.list.int#0:shape#4(fn() -> list_type#0), %list.int#1:shape#1(list_type#0), %list.int#2:shape#1(list_type#0)]
//     jump b6(%int#0, %int#0, %list.int#0, %custom#0, %function.list.int#0, %list.int#1, %list.int#2)
//   block b8 params=[%int#0:shape#0(Int), %list.int#0:shape#1(list_type#0), %custom#0:shape#75(custom_type#1), %function.list.int#0:shape#4(fn() -> list_type#0), %list.int#1:shape#1(list_type#0)]
//     %list.int#2:shape#1(list_type#0) = list.int[type#0] value elements=[]
//     jump b4(%list.int#2, %int#0, %list.int#0, %custom#0, %function.list.int#0, %list.int#1)
//   block b9 params=[%int#0:shape#0(Int), %list.int#0:shape#1(list_type#0), %custom#0:shape#75(custom_type#1), %function.list.int#0:shape#4(fn() -> list_type#0)]
//     %list.int#1:shape#1(list_type#0) = list.int[type#0] value elements=[]
//     jump b2(%list.int#1, %int#0, %list.int#0, %custom#0, %function.list.int#0)
//
// function tuple#2
//   entry b0 params=[%string#0:shape#6(String), %list.string#0:shape#7(list_type#1), %list.list#0:shape#8(list_type#12), %custom#0:shape#79(custom_type#2), %function.list.string#0:shape#10(fn() -> list_type#1)] captures=[]
//   block b0 params=[%string#0:shape#6(String), %list.string#0:shape#7(list_type#1), %list.list#0:shape#8(list_type#12), %custom#0:shape#79(custom_type#2), %function.list.string#0:shape#10(fn() -> list_type#1)]
//     %bool#0:shape#38(Bool) = bool.list_length_equals %list.list#0 length=1
//     branch %bool#0 true=b1(%string#0, %list.string#0, %list.list#0, %custom#0, %function.list.string#0) false=b9(%string#0, %list.string#0, %custom#0, %function.list.string#0)
//   block b1 params=[%string#0:shape#6(String), %list.string#0:shape#7(list_type#1), %list.list#0:shape#8(list_type#12), %custom#0:shape#79(custom_type#2), %function.list.string#0:shape#10(fn() -> list_type#1)]
//     %list.string#1:shape#7(list_type#1) = list.string[type#1] list_index %list.list#0 index=0
//     jump b2(%list.string#1, %string#0, %list.string#0, %custom#0, %function.list.string#0)
//   block b2 params=[%list.string#0:shape#7(list_type#1), %string#0:shape#6(String), %list.string#1:shape#7(list_type#1), %custom#0:shape#79(custom_type#2), %function.list.string#0:shape#10(fn() -> list_type#1)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.string#1 length=1
//     branch %bool#0 true=b3(%string#0, %list.string#1, %custom#0, %function.list.string#0, %list.string#0) false=b8(%string#0, %list.string#1, %custom#0, %function.list.string#0, %list.string#0)
//   block b3 params=[%string#0:shape#6(String), %list.string#0:shape#7(list_type#1), %custom#0:shape#79(custom_type#2), %function.list.string#0:shape#10(fn() -> list_type#1), %list.string#1:shape#7(list_type#1)]
//     %list.string#2:shape#7(list_type#1) = list.string[type#1] drop_first %list.string#0 count=1
//     jump b4(%list.string#2, %string#0, %list.string#0, %custom#0, %function.list.string#0, %list.string#1)
//   block b4 params=[%list.string#0:shape#7(list_type#1), %string#0:shape#6(String), %list.string#1:shape#7(list_type#1), %custom#0:shape#79(custom_type#2), %function.list.string#0:shape#10(fn() -> list_type#1), %list.string#2:shape#7(list_type#1)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.string#1 length=1
//     branch %bool#0 true=b5(%string#0, %list.string#1, %custom#0, %function.list.string#0, %list.string#2, %list.string#0) false=b7(%string#0, %list.string#1, %custom#0, %function.list.string#0, %list.string#2, %list.string#0)
//   block b5 params=[%string#0:shape#6(String), %list.string#0:shape#7(list_type#1), %custom#0:shape#79(custom_type#2), %function.list.string#0:shape#10(fn() -> list_type#1), %list.string#1:shape#7(list_type#1), %list.string#2:shape#7(list_type#1)]
//     %string#1:shape#6(String) = string.list_index %list.string#0 index=0
//     jump b6(%string#1, %string#0, %list.string#0, %custom#0, %function.list.string#0, %list.string#1, %list.string#2)
//   block b6 params=[%string#0:shape#6(String), %string#1:shape#6(String), %list.string#0:shape#7(list_type#1), %custom#0:shape#79(custom_type#2), %function.list.string#0:shape#10(fn() -> list_type#1), %list.string#1:shape#7(list_type#1), %list.string#2:shape#7(list_type#1)]
//     %string#2:shape#6(String) = string.call string#0 args=[%string#1]
//     %function.string#0:shape#80(fn() -> String) = function[String] closure target=string#1 captures=[%string#0<-%string#1]
//     %list.string#3:shape#7(list_type#1) = list.string[type#1] value elements=[%string#1]
//     %list.string#4:shape#7(list_type#1) = list.string[type#1] spread elements=[%string#1] tail=%list.string#0
//     %list.string#5:shape#7(list_type#1) = list.string[type#1] call list.string#1 args=[%list.string#0]
//     %string#3:shape#6(String) = string.function_call %function.string#0 args=[]
//     %tuple#0:shape#81(#(list_type#1)) = tuple.value elements=[%list.string#0]
//     %list.string#6:shape#7(list_type#1) = list.string[type#1] tuple_index %tuple#0 index=0
//     %list.string#7:shape#7(list_type#1) = list.string[type#1] custom_field %custom#0 index=0
//     %list.string#8:shape#7(list_type#1) = list.string[type#1] function_call %function.list.string#0 args=[]
//     %tuple#1:shape#82(#(String)) = tuple.value elements=[%string#1]
//     %string#4:shape#6(String) = string.tuple_index %tuple#1 index=0
//     %custom#1:shape#83(custom_type#14) = custom.construct custom_type#14.constructor#0 fields=[%string#1]
//     %string#5:shape#6(String) = string.custom_field %custom#1 index=0
//     %tuple#2:shape#11(#(list_type#1, list_type#1, list_type#1, String, list_type#1, list_type#1, list_type#1, list_type#1, String, list_type#1, String, String, String)) = tuple.value elements=[%list.string#3, %list.string#4, %list.string#5, %string#3, %list.string#6, %list.string#7, %list.string#1, %list.string#2, %string#2, %list.string#8, %string#4, %string#5, %string#0]
//     return %tuple#2
//   block b7 params=[%string#0:shape#6(String), %list.string#0:shape#7(list_type#1), %custom#0:shape#79(custom_type#2), %function.list.string#0:shape#10(fn() -> list_type#1), %list.string#1:shape#7(list_type#1), %list.string#2:shape#7(list_type#1)]
//     jump b6(%string#0, %string#0, %list.string#0, %custom#0, %function.list.string#0, %list.string#1, %list.string#2)
//   block b8 params=[%string#0:shape#6(String), %list.string#0:shape#7(list_type#1), %custom#0:shape#79(custom_type#2), %function.list.string#0:shape#10(fn() -> list_type#1), %list.string#1:shape#7(list_type#1)]
//     %list.string#2:shape#7(list_type#1) = list.string[type#1] value elements=[]
//     jump b4(%list.string#2, %string#0, %list.string#0, %custom#0, %function.list.string#0, %list.string#1)
//   block b9 params=[%string#0:shape#6(String), %list.string#0:shape#7(list_type#1), %custom#0:shape#79(custom_type#2), %function.list.string#0:shape#10(fn() -> list_type#1)]
//     %list.string#1:shape#7(list_type#1) = list.string[type#1] value elements=[]
//     jump b2(%list.string#1, %string#0, %list.string#0, %custom#0, %function.list.string#0)
//
// function tuple#3
//   entry b0 params=[%bit_array#0:shape#12(BitArray), %list.bit_array#0:shape#13(list_type#2), %list.list#0:shape#14(list_type#13), %custom#0:shape#84(custom_type#3), %function.list.bit_array#0:shape#16(fn() -> list_type#2)] captures=[]
//   block b0 params=[%bit_array#0:shape#12(BitArray), %list.bit_array#0:shape#13(list_type#2), %list.list#0:shape#14(list_type#13), %custom#0:shape#84(custom_type#3), %function.list.bit_array#0:shape#16(fn() -> list_type#2)]
//     %bool#0:shape#38(Bool) = bool.list_length_equals %list.list#0 length=1
//     branch %bool#0 true=b1(%bit_array#0, %list.bit_array#0, %list.list#0, %custom#0, %function.list.bit_array#0) false=b9(%bit_array#0, %list.bit_array#0, %custom#0, %function.list.bit_array#0)
//   block b1 params=[%bit_array#0:shape#12(BitArray), %list.bit_array#0:shape#13(list_type#2), %list.list#0:shape#14(list_type#13), %custom#0:shape#84(custom_type#3), %function.list.bit_array#0:shape#16(fn() -> list_type#2)]
//     %list.bit_array#1:shape#13(list_type#2) = list.bit_array[type#2] list_index %list.list#0 index=0
//     jump b2(%list.bit_array#1, %bit_array#0, %list.bit_array#0, %custom#0, %function.list.bit_array#0)
//   block b2 params=[%list.bit_array#0:shape#13(list_type#2), %bit_array#0:shape#12(BitArray), %list.bit_array#1:shape#13(list_type#2), %custom#0:shape#84(custom_type#3), %function.list.bit_array#0:shape#16(fn() -> list_type#2)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.bit_array#1 length=1
//     branch %bool#0 true=b3(%bit_array#0, %list.bit_array#1, %custom#0, %function.list.bit_array#0, %list.bit_array#0) false=b8(%bit_array#0, %list.bit_array#1, %custom#0, %function.list.bit_array#0, %list.bit_array#0)
//   block b3 params=[%bit_array#0:shape#12(BitArray), %list.bit_array#0:shape#13(list_type#2), %custom#0:shape#84(custom_type#3), %function.list.bit_array#0:shape#16(fn() -> list_type#2), %list.bit_array#1:shape#13(list_type#2)]
//     %list.bit_array#2:shape#13(list_type#2) = list.bit_array[type#2] drop_first %list.bit_array#0 count=1
//     jump b4(%list.bit_array#2, %bit_array#0, %list.bit_array#0, %custom#0, %function.list.bit_array#0, %list.bit_array#1)
//   block b4 params=[%list.bit_array#0:shape#13(list_type#2), %bit_array#0:shape#12(BitArray), %list.bit_array#1:shape#13(list_type#2), %custom#0:shape#84(custom_type#3), %function.list.bit_array#0:shape#16(fn() -> list_type#2), %list.bit_array#2:shape#13(list_type#2)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.bit_array#1 length=1
//     branch %bool#0 true=b5(%bit_array#0, %list.bit_array#1, %custom#0, %function.list.bit_array#0, %list.bit_array#2, %list.bit_array#0) false=b7(%bit_array#0, %list.bit_array#1, %custom#0, %function.list.bit_array#0, %list.bit_array#2, %list.bit_array#0)
//   block b5 params=[%bit_array#0:shape#12(BitArray), %list.bit_array#0:shape#13(list_type#2), %custom#0:shape#84(custom_type#3), %function.list.bit_array#0:shape#16(fn() -> list_type#2), %list.bit_array#1:shape#13(list_type#2), %list.bit_array#2:shape#13(list_type#2)]
//     %bit_array#1:shape#12(BitArray) = bit_array.list_index %list.bit_array#0 index=0
//     jump b6(%bit_array#1, %bit_array#0, %list.bit_array#0, %custom#0, %function.list.bit_array#0, %list.bit_array#1, %list.bit_array#2)
//   block b6 params=[%bit_array#0:shape#12(BitArray), %bit_array#1:shape#12(BitArray), %list.bit_array#0:shape#13(list_type#2), %custom#0:shape#84(custom_type#3), %function.list.bit_array#0:shape#16(fn() -> list_type#2), %list.bit_array#1:shape#13(list_type#2), %list.bit_array#2:shape#13(list_type#2)]
//     %bit_array#2:shape#12(BitArray) = bit_array.call bit_array#0 args=[%bit_array#1]
//     %function.bit_array#0:shape#85(fn() -> BitArray) = function[BitArray] closure target=bit_array#1 captures=[%bit_array#0<-%bit_array#1]
//     %list.bit_array#3:shape#13(list_type#2) = list.bit_array[type#2] value elements=[%bit_array#1]
//     %list.bit_array#4:shape#13(list_type#2) = list.bit_array[type#2] spread elements=[%bit_array#1] tail=%list.bit_array#0
//     %list.bit_array#5:shape#13(list_type#2) = list.bit_array[type#2] call list.bit_array#1 args=[%list.bit_array#0]
//     %bit_array#3:shape#12(BitArray) = bit_array.function_call %function.bit_array#0 args=[]
//     %tuple#0:shape#86(#(list_type#2)) = tuple.value elements=[%list.bit_array#0]
//     %list.bit_array#6:shape#13(list_type#2) = list.bit_array[type#2] tuple_index %tuple#0 index=0
//     %list.bit_array#7:shape#13(list_type#2) = list.bit_array[type#2] custom_field %custom#0 index=0
//     %list.bit_array#8:shape#13(list_type#2) = list.bit_array[type#2] function_call %function.list.bit_array#0 args=[]
//     %tuple#1:shape#87(#(BitArray)) = tuple.value elements=[%bit_array#1]
//     %bit_array#4:shape#12(BitArray) = bit_array.tuple_index %tuple#1 index=0
//     %custom#1:shape#88(custom_type#15) = custom.construct custom_type#15.constructor#0 fields=[%bit_array#1]
//     %bit_array#5:shape#12(BitArray) = bit_array.custom_field %custom#1 index=0
//     %tuple#2:shape#17(#(list_type#2, list_type#2, list_type#2, BitArray, list_type#2, list_type#2, list_type#2, list_type#2, BitArray, list_type#2, BitArray, BitArray, BitArray)) = tuple.value elements=[%list.bit_array#3, %list.bit_array#4, %list.bit_array#5, %bit_array#3, %list.bit_array#6, %list.bit_array#7, %list.bit_array#1, %list.bit_array#2, %bit_array#2, %list.bit_array#8, %bit_array#4, %bit_array#5, %bit_array#0]
//     return %tuple#2
//   block b7 params=[%bit_array#0:shape#12(BitArray), %list.bit_array#0:shape#13(list_type#2), %custom#0:shape#84(custom_type#3), %function.list.bit_array#0:shape#16(fn() -> list_type#2), %list.bit_array#1:shape#13(list_type#2), %list.bit_array#2:shape#13(list_type#2)]
//     jump b6(%bit_array#0, %bit_array#0, %list.bit_array#0, %custom#0, %function.list.bit_array#0, %list.bit_array#1, %list.bit_array#2)
//   block b8 params=[%bit_array#0:shape#12(BitArray), %list.bit_array#0:shape#13(list_type#2), %custom#0:shape#84(custom_type#3), %function.list.bit_array#0:shape#16(fn() -> list_type#2), %list.bit_array#1:shape#13(list_type#2)]
//     %list.bit_array#2:shape#13(list_type#2) = list.bit_array[type#2] value elements=[]
//     jump b4(%list.bit_array#2, %bit_array#0, %list.bit_array#0, %custom#0, %function.list.bit_array#0, %list.bit_array#1)
//   block b9 params=[%bit_array#0:shape#12(BitArray), %list.bit_array#0:shape#13(list_type#2), %custom#0:shape#84(custom_type#3), %function.list.bit_array#0:shape#16(fn() -> list_type#2)]
//     %list.bit_array#1:shape#13(list_type#2) = list.bit_array[type#2] value elements=[]
//     jump b2(%list.bit_array#1, %bit_array#0, %list.bit_array#0, %custom#0, %function.list.bit_array#0)
//
// function tuple#4
//   entry b0 params=[%utf_codepoint#0:shape#18(UtfCodepoint), %list.utf_codepoint#0:shape#19(list_type#3), %list.list#0:shape#20(list_type#14), %custom#0:shape#89(custom_type#4), %function.list.utf_codepoint#0:shape#22(fn() -> list_type#3)] captures=[]
//   block b0 params=[%utf_codepoint#0:shape#18(UtfCodepoint), %list.utf_codepoint#0:shape#19(list_type#3), %list.list#0:shape#20(list_type#14), %custom#0:shape#89(custom_type#4), %function.list.utf_codepoint#0:shape#22(fn() -> list_type#3)]
//     %bool#0:shape#38(Bool) = bool.list_length_equals %list.list#0 length=1
//     branch %bool#0 true=b1(%utf_codepoint#0, %list.utf_codepoint#0, %list.list#0, %custom#0, %function.list.utf_codepoint#0) false=b9(%utf_codepoint#0, %list.utf_codepoint#0, %custom#0, %function.list.utf_codepoint#0)
//   block b1 params=[%utf_codepoint#0:shape#18(UtfCodepoint), %list.utf_codepoint#0:shape#19(list_type#3), %list.list#0:shape#20(list_type#14), %custom#0:shape#89(custom_type#4), %function.list.utf_codepoint#0:shape#22(fn() -> list_type#3)]
//     %list.utf_codepoint#1:shape#19(list_type#3) = list.utf_codepoint[type#3] list_index %list.list#0 index=0
//     jump b2(%list.utf_codepoint#1, %utf_codepoint#0, %list.utf_codepoint#0, %custom#0, %function.list.utf_codepoint#0)
//   block b2 params=[%list.utf_codepoint#0:shape#19(list_type#3), %utf_codepoint#0:shape#18(UtfCodepoint), %list.utf_codepoint#1:shape#19(list_type#3), %custom#0:shape#89(custom_type#4), %function.list.utf_codepoint#0:shape#22(fn() -> list_type#3)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.utf_codepoint#1 length=1
//     branch %bool#0 true=b3(%utf_codepoint#0, %list.utf_codepoint#1, %custom#0, %function.list.utf_codepoint#0, %list.utf_codepoint#0) false=b8(%utf_codepoint#0, %list.utf_codepoint#1, %custom#0, %function.list.utf_codepoint#0, %list.utf_codepoint#0)
//   block b3 params=[%utf_codepoint#0:shape#18(UtfCodepoint), %list.utf_codepoint#0:shape#19(list_type#3), %custom#0:shape#89(custom_type#4), %function.list.utf_codepoint#0:shape#22(fn() -> list_type#3), %list.utf_codepoint#1:shape#19(list_type#3)]
//     %list.utf_codepoint#2:shape#19(list_type#3) = list.utf_codepoint[type#3] drop_first %list.utf_codepoint#0 count=1
//     jump b4(%list.utf_codepoint#2, %utf_codepoint#0, %list.utf_codepoint#0, %custom#0, %function.list.utf_codepoint#0, %list.utf_codepoint#1)
//   block b4 params=[%list.utf_codepoint#0:shape#19(list_type#3), %utf_codepoint#0:shape#18(UtfCodepoint), %list.utf_codepoint#1:shape#19(list_type#3), %custom#0:shape#89(custom_type#4), %function.list.utf_codepoint#0:shape#22(fn() -> list_type#3), %list.utf_codepoint#2:shape#19(list_type#3)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.utf_codepoint#1 length=1
//     branch %bool#0 true=b5(%utf_codepoint#0, %list.utf_codepoint#1, %custom#0, %function.list.utf_codepoint#0, %list.utf_codepoint#2, %list.utf_codepoint#0) false=b7(%utf_codepoint#0, %list.utf_codepoint#1, %custom#0, %function.list.utf_codepoint#0, %list.utf_codepoint#2, %list.utf_codepoint#0)
//   block b5 params=[%utf_codepoint#0:shape#18(UtfCodepoint), %list.utf_codepoint#0:shape#19(list_type#3), %custom#0:shape#89(custom_type#4), %function.list.utf_codepoint#0:shape#22(fn() -> list_type#3), %list.utf_codepoint#1:shape#19(list_type#3), %list.utf_codepoint#2:shape#19(list_type#3)]
//     %utf_codepoint#1:shape#18(UtfCodepoint) = utf_codepoint.list_index %list.utf_codepoint#0 index=0
//     jump b6(%utf_codepoint#1, %utf_codepoint#0, %list.utf_codepoint#0, %custom#0, %function.list.utf_codepoint#0, %list.utf_codepoint#1, %list.utf_codepoint#2)
//   block b6 params=[%utf_codepoint#0:shape#18(UtfCodepoint), %utf_codepoint#1:shape#18(UtfCodepoint), %list.utf_codepoint#0:shape#19(list_type#3), %custom#0:shape#89(custom_type#4), %function.list.utf_codepoint#0:shape#22(fn() -> list_type#3), %list.utf_codepoint#1:shape#19(list_type#3), %list.utf_codepoint#2:shape#19(list_type#3)]
//     %utf_codepoint#2:shape#18(UtfCodepoint) = utf_codepoint.call utf_codepoint#1 args=[%utf_codepoint#1]
//     %function.utf_codepoint#0:shape#90(fn() -> UtfCodepoint) = function[UtfCodepoint] closure target=utf_codepoint#2 captures=[%utf_codepoint#0<-%utf_codepoint#1]
//     %list.utf_codepoint#3:shape#19(list_type#3) = list.utf_codepoint[type#3] value elements=[%utf_codepoint#1]
//     %list.utf_codepoint#4:shape#19(list_type#3) = list.utf_codepoint[type#3] spread elements=[%utf_codepoint#1] tail=%list.utf_codepoint#0
//     %list.utf_codepoint#5:shape#19(list_type#3) = list.utf_codepoint[type#3] call list.utf_codepoint#1 args=[%list.utf_codepoint#0]
//     %utf_codepoint#3:shape#18(UtfCodepoint) = utf_codepoint.function_call %function.utf_codepoint#0 args=[]
//     %tuple#0:shape#91(#(list_type#3)) = tuple.value elements=[%list.utf_codepoint#0]
//     %list.utf_codepoint#6:shape#19(list_type#3) = list.utf_codepoint[type#3] tuple_index %tuple#0 index=0
//     %list.utf_codepoint#7:shape#19(list_type#3) = list.utf_codepoint[type#3] custom_field %custom#0 index=0
//     %list.utf_codepoint#8:shape#19(list_type#3) = list.utf_codepoint[type#3] function_call %function.list.utf_codepoint#0 args=[]
//     %tuple#1:shape#92(#(UtfCodepoint)) = tuple.value elements=[%utf_codepoint#1]
//     %utf_codepoint#4:shape#18(UtfCodepoint) = utf_codepoint.tuple_index %tuple#1 index=0
//     %custom#1:shape#93(custom_type#16) = custom.construct custom_type#16.constructor#0 fields=[%utf_codepoint#1]
//     %utf_codepoint#5:shape#18(UtfCodepoint) = utf_codepoint.custom_field %custom#1 index=0
//     %tuple#2:shape#23(#(list_type#3, list_type#3, list_type#3, UtfCodepoint, list_type#3, list_type#3, list_type#3, list_type#3, UtfCodepoint, list_type#3, UtfCodepoint, UtfCodepoint, UtfCodepoint)) = tuple.value elements=[%list.utf_codepoint#3, %list.utf_codepoint#4, %list.utf_codepoint#5, %utf_codepoint#3, %list.utf_codepoint#6, %list.utf_codepoint#7, %list.utf_codepoint#1, %list.utf_codepoint#2, %utf_codepoint#2, %list.utf_codepoint#8, %utf_codepoint#4, %utf_codepoint#5, %utf_codepoint#0]
//     return %tuple#2
//   block b7 params=[%utf_codepoint#0:shape#18(UtfCodepoint), %list.utf_codepoint#0:shape#19(list_type#3), %custom#0:shape#89(custom_type#4), %function.list.utf_codepoint#0:shape#22(fn() -> list_type#3), %list.utf_codepoint#1:shape#19(list_type#3), %list.utf_codepoint#2:shape#19(list_type#3)]
//     jump b6(%utf_codepoint#0, %utf_codepoint#0, %list.utf_codepoint#0, %custom#0, %function.list.utf_codepoint#0, %list.utf_codepoint#1, %list.utf_codepoint#2)
//   block b8 params=[%utf_codepoint#0:shape#18(UtfCodepoint), %list.utf_codepoint#0:shape#19(list_type#3), %custom#0:shape#89(custom_type#4), %function.list.utf_codepoint#0:shape#22(fn() -> list_type#3), %list.utf_codepoint#1:shape#19(list_type#3)]
//     %list.utf_codepoint#2:shape#19(list_type#3) = list.utf_codepoint[type#3] value elements=[]
//     jump b4(%list.utf_codepoint#2, %utf_codepoint#0, %list.utf_codepoint#0, %custom#0, %function.list.utf_codepoint#0, %list.utf_codepoint#1)
//   block b9 params=[%utf_codepoint#0:shape#18(UtfCodepoint), %list.utf_codepoint#0:shape#19(list_type#3), %custom#0:shape#89(custom_type#4), %function.list.utf_codepoint#0:shape#22(fn() -> list_type#3)]
//     %list.utf_codepoint#1:shape#19(list_type#3) = list.utf_codepoint[type#3] value elements=[]
//     jump b2(%list.utf_codepoint#1, %utf_codepoint#0, %list.utf_codepoint#0, %custom#0, %function.list.utf_codepoint#0)
//
// function tuple#5
//   entry b0 params=[%custom#0:shape#28(custom_type#0), %list.custom#0:shape#29(list_type#4), %list.list#0:shape#94(list_type#15), %custom#1:shape#95(custom_type#5), %function.list.custom#0:shape#30(fn() -> list_type#4)] captures=[]
//   block b0 params=[%custom#0:shape#28(custom_type#0), %list.custom#0:shape#29(list_type#4), %list.list#0:shape#94(list_type#15), %custom#1:shape#95(custom_type#5), %function.list.custom#0:shape#30(fn() -> list_type#4)]
//     %bool#0:shape#38(Bool) = bool.list_length_equals %list.list#0 length=1
//     branch %bool#0 true=b1(%custom#0, %list.custom#0, %list.list#0, %custom#1, %function.list.custom#0) false=b9(%custom#0, %list.custom#0, %custom#1, %function.list.custom#0)
//   block b1 params=[%custom#0:shape#28(custom_type#0), %list.custom#0:shape#29(list_type#4), %list.list#0:shape#94(list_type#15), %custom#1:shape#95(custom_type#5), %function.list.custom#0:shape#30(fn() -> list_type#4)]
//     %list.custom#1:shape#29(list_type#4) = list.custom[type#4] list_index %list.list#0 index=0
//     jump b2(%list.custom#1, %custom#0, %list.custom#0, %custom#1, %function.list.custom#0)
//   block b2 params=[%list.custom#0:shape#29(list_type#4), %custom#0:shape#28(custom_type#0), %list.custom#1:shape#29(list_type#4), %custom#1:shape#95(custom_type#5), %function.list.custom#0:shape#30(fn() -> list_type#4)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.custom#1 length=1
//     branch %bool#0 true=b3(%custom#0, %list.custom#1, %custom#1, %function.list.custom#0, %list.custom#0) false=b8(%custom#0, %list.custom#1, %custom#1, %function.list.custom#0, %list.custom#0)
//   block b3 params=[%custom#0:shape#28(custom_type#0), %list.custom#0:shape#29(list_type#4), %custom#1:shape#95(custom_type#5), %function.list.custom#0:shape#30(fn() -> list_type#4), %list.custom#1:shape#29(list_type#4)]
//     %list.custom#2:shape#29(list_type#4) = list.custom[type#4] drop_first %list.custom#0 count=1
//     jump b4(%list.custom#2, %custom#0, %list.custom#0, %custom#1, %function.list.custom#0, %list.custom#1)
//   block b4 params=[%list.custom#0:shape#29(list_type#4), %custom#0:shape#28(custom_type#0), %list.custom#1:shape#29(list_type#4), %custom#1:shape#95(custom_type#5), %function.list.custom#0:shape#30(fn() -> list_type#4), %list.custom#2:shape#29(list_type#4)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.custom#1 length=1
//     branch %bool#0 true=b5(%custom#0, %list.custom#1, %custom#1, %function.list.custom#0, %list.custom#2, %list.custom#0) false=b7(%custom#0, %list.custom#1, %custom#1, %function.list.custom#0, %list.custom#2, %list.custom#0)
//   block b5 params=[%custom#0:shape#28(custom_type#0), %list.custom#0:shape#29(list_type#4), %custom#1:shape#95(custom_type#5), %function.list.custom#0:shape#30(fn() -> list_type#4), %list.custom#1:shape#29(list_type#4), %list.custom#2:shape#29(list_type#4)]
//     %custom#2:shape#28(custom_type#0) = custom.list_index %list.custom#0 index=0
//     jump b6(%custom#2, %custom#0, %list.custom#0, %custom#1, %function.list.custom#0, %list.custom#1, %list.custom#2)
//   block b6 params=[%custom#0:shape#28(custom_type#0), %custom#1:shape#28(custom_type#0), %list.custom#0:shape#29(list_type#4), %custom#2:shape#95(custom_type#5), %function.list.custom#0:shape#30(fn() -> list_type#4), %list.custom#1:shape#29(list_type#4), %list.custom#2:shape#29(list_type#4)]
//     %custom#3:shape#28(custom_type#0) = custom.call custom#0 args=[%custom#1]
//     %function.custom#0:shape#96(fn() -> custom_type#0) = function[Custom] closure target=custom#1 captures=[%custom#0<-%custom#1]
//     %list.custom#3:shape#29(list_type#4) = list.custom[type#4] value elements=[%custom#1]
//     %list.custom#4:shape#29(list_type#4) = list.custom[type#4] spread elements=[%custom#1] tail=%list.custom#0
//     %list.custom#5:shape#29(list_type#4) = list.custom[type#4] call list.custom#1 args=[%list.custom#0]
//     %custom#4:shape#28(custom_type#0) = custom.function_call %function.custom#0 args=[]
//     %tuple#0:shape#97(#(list_type#4)) = tuple.value elements=[%list.custom#0]
//     %list.custom#6:shape#29(list_type#4) = list.custom[type#4] tuple_index %tuple#0 index=0
//     %list.custom#7:shape#29(list_type#4) = list.custom[type#4] custom_field %custom#2 index=0
//     %list.custom#8:shape#29(list_type#4) = list.custom[type#4] function_call %function.list.custom#0 args=[]
//     %tuple#1:shape#98(#(custom_type#0)) = tuple.value elements=[%custom#1]
//     %custom#5:shape#28(custom_type#0) = custom.tuple_index %tuple#1 index=0
//     %custom#6:shape#99(custom_type#17) = custom.construct custom_type#17.constructor#0 fields=[%custom#1]
//     %custom#7:shape#28(custom_type#0) = custom.custom_field %custom#6 index=0
//     %tuple#2:shape#31(#(list_type#4, list_type#4, list_type#4, custom_type#0, list_type#4, list_type#4, list_type#4, list_type#4, custom_type#0, list_type#4, custom_type#0, custom_type#0, custom_type#0)) = tuple.value elements=[%list.custom#3, %list.custom#4, %list.custom#5, %custom#4, %list.custom#6, %list.custom#7, %list.custom#1, %list.custom#2, %custom#3, %list.custom#8, %custom#5, %custom#7, %custom#0]
//     return %tuple#2
//   block b7 params=[%custom#0:shape#28(custom_type#0), %list.custom#0:shape#29(list_type#4), %custom#1:shape#95(custom_type#5), %function.list.custom#0:shape#30(fn() -> list_type#4), %list.custom#1:shape#29(list_type#4), %list.custom#2:shape#29(list_type#4)]
//     jump b6(%custom#0, %custom#0, %list.custom#0, %custom#1, %function.list.custom#0, %list.custom#1, %list.custom#2)
//   block b8 params=[%custom#0:shape#28(custom_type#0), %list.custom#0:shape#29(list_type#4), %custom#1:shape#95(custom_type#5), %function.list.custom#0:shape#30(fn() -> list_type#4), %list.custom#1:shape#29(list_type#4)]
//     %list.custom#2:shape#29(list_type#4) = list.custom[type#4] value elements=[]
//     jump b4(%list.custom#2, %custom#0, %list.custom#0, %custom#1, %function.list.custom#0, %list.custom#1)
//   block b9 params=[%custom#0:shape#28(custom_type#0), %list.custom#0:shape#29(list_type#4), %custom#1:shape#95(custom_type#5), %function.list.custom#0:shape#30(fn() -> list_type#4)]
//     %list.custom#1:shape#29(list_type#4) = list.custom[type#4] value elements=[]
//     jump b2(%list.custom#1, %custom#0, %list.custom#0, %custom#1, %function.list.custom#0)
//
// function tuple#6
//   entry b0 params=[%float#0:shape#32(Float), %list.float#0:shape#33(list_type#5), %list.list#0:shape#34(list_type#16), %custom#0:shape#100(custom_type#6), %function.list.float#0:shape#36(fn() -> list_type#5)] captures=[]
//   block b0 params=[%float#0:shape#32(Float), %list.float#0:shape#33(list_type#5), %list.list#0:shape#34(list_type#16), %custom#0:shape#100(custom_type#6), %function.list.float#0:shape#36(fn() -> list_type#5)]
//     %bool#0:shape#38(Bool) = bool.list_length_equals %list.list#0 length=1
//     branch %bool#0 true=b1(%float#0, %list.float#0, %list.list#0, %custom#0, %function.list.float#0) false=b9(%float#0, %list.float#0, %custom#0, %function.list.float#0)
//   block b1 params=[%float#0:shape#32(Float), %list.float#0:shape#33(list_type#5), %list.list#0:shape#34(list_type#16), %custom#0:shape#100(custom_type#6), %function.list.float#0:shape#36(fn() -> list_type#5)]
//     %list.float#1:shape#33(list_type#5) = list.float[type#5] list_index %list.list#0 index=0
//     jump b2(%list.float#1, %float#0, %list.float#0, %custom#0, %function.list.float#0)
//   block b2 params=[%list.float#0:shape#33(list_type#5), %float#0:shape#32(Float), %list.float#1:shape#33(list_type#5), %custom#0:shape#100(custom_type#6), %function.list.float#0:shape#36(fn() -> list_type#5)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.float#1 length=1
//     branch %bool#0 true=b3(%float#0, %list.float#1, %custom#0, %function.list.float#0, %list.float#0) false=b8(%float#0, %list.float#1, %custom#0, %function.list.float#0, %list.float#0)
//   block b3 params=[%float#0:shape#32(Float), %list.float#0:shape#33(list_type#5), %custom#0:shape#100(custom_type#6), %function.list.float#0:shape#36(fn() -> list_type#5), %list.float#1:shape#33(list_type#5)]
//     %list.float#2:shape#33(list_type#5) = list.float[type#5] drop_first %list.float#0 count=1
//     jump b4(%list.float#2, %float#0, %list.float#0, %custom#0, %function.list.float#0, %list.float#1)
//   block b4 params=[%list.float#0:shape#33(list_type#5), %float#0:shape#32(Float), %list.float#1:shape#33(list_type#5), %custom#0:shape#100(custom_type#6), %function.list.float#0:shape#36(fn() -> list_type#5), %list.float#2:shape#33(list_type#5)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.float#1 length=1
//     branch %bool#0 true=b5(%float#0, %list.float#1, %custom#0, %function.list.float#0, %list.float#2, %list.float#0) false=b7(%float#0, %list.float#1, %custom#0, %function.list.float#0, %list.float#2, %list.float#0)
//   block b5 params=[%float#0:shape#32(Float), %list.float#0:shape#33(list_type#5), %custom#0:shape#100(custom_type#6), %function.list.float#0:shape#36(fn() -> list_type#5), %list.float#1:shape#33(list_type#5), %list.float#2:shape#33(list_type#5)]
//     %float#1:shape#32(Float) = float.list_index %list.float#0 index=0
//     jump b6(%float#1, %float#0, %list.float#0, %custom#0, %function.list.float#0, %list.float#1, %list.float#2)
//   block b6 params=[%float#0:shape#32(Float), %float#1:shape#32(Float), %list.float#0:shape#33(list_type#5), %custom#0:shape#100(custom_type#6), %function.list.float#0:shape#36(fn() -> list_type#5), %list.float#1:shape#33(list_type#5), %list.float#2:shape#33(list_type#5)]
//     %float#2:shape#32(Float) = float.call float#0 args=[%float#1]
//     %function.float#0:shape#101(fn() -> Float) = function[Float] closure target=float#1 captures=[%float#0<-%float#1]
//     %list.float#3:shape#33(list_type#5) = list.float[type#5] value elements=[%float#1]
//     %list.float#4:shape#33(list_type#5) = list.float[type#5] spread elements=[%float#1] tail=%list.float#0
//     %list.float#5:shape#33(list_type#5) = list.float[type#5] call list.float#1 args=[%list.float#0]
//     %float#3:shape#32(Float) = float.function_call %function.float#0 args=[]
//     %tuple#0:shape#102(#(list_type#5)) = tuple.value elements=[%list.float#0]
//     %list.float#6:shape#33(list_type#5) = list.float[type#5] tuple_index %tuple#0 index=0
//     %list.float#7:shape#33(list_type#5) = list.float[type#5] custom_field %custom#0 index=0
//     %list.float#8:shape#33(list_type#5) = list.float[type#5] function_call %function.list.float#0 args=[]
//     %tuple#1:shape#103(#(Float)) = tuple.value elements=[%float#1]
//     %float#4:shape#32(Float) = float.tuple_index %tuple#1 index=0
//     %custom#1:shape#104(custom_type#18) = custom.construct custom_type#18.constructor#0 fields=[%float#1]
//     %float#5:shape#32(Float) = float.custom_field %custom#1 index=0
//     %tuple#2:shape#37(#(list_type#5, list_type#5, list_type#5, Float, list_type#5, list_type#5, list_type#5, list_type#5, Float, list_type#5, Float, Float, Float)) = tuple.value elements=[%list.float#3, %list.float#4, %list.float#5, %float#3, %list.float#6, %list.float#7, %list.float#1, %list.float#2, %float#2, %list.float#8, %float#4, %float#5, %float#0]
//     return %tuple#2
//   block b7 params=[%float#0:shape#32(Float), %list.float#0:shape#33(list_type#5), %custom#0:shape#100(custom_type#6), %function.list.float#0:shape#36(fn() -> list_type#5), %list.float#1:shape#33(list_type#5), %list.float#2:shape#33(list_type#5)]
//     jump b6(%float#0, %float#0, %list.float#0, %custom#0, %function.list.float#0, %list.float#1, %list.float#2)
//   block b8 params=[%float#0:shape#32(Float), %list.float#0:shape#33(list_type#5), %custom#0:shape#100(custom_type#6), %function.list.float#0:shape#36(fn() -> list_type#5), %list.float#1:shape#33(list_type#5)]
//     %list.float#2:shape#33(list_type#5) = list.float[type#5] value elements=[]
//     jump b4(%list.float#2, %float#0, %list.float#0, %custom#0, %function.list.float#0, %list.float#1)
//   block b9 params=[%float#0:shape#32(Float), %list.float#0:shape#33(list_type#5), %custom#0:shape#100(custom_type#6), %function.list.float#0:shape#36(fn() -> list_type#5)]
//     %list.float#1:shape#33(list_type#5) = list.float[type#5] value elements=[]
//     jump b2(%list.float#1, %float#0, %list.float#0, %custom#0, %function.list.float#0)
//
// function tuple#7
//   entry b0 params=[%bool#0:shape#38(Bool), %list.bool#0:shape#39(list_type#6), %list.list#0:shape#40(list_type#17), %custom#0:shape#105(custom_type#7), %function.list.bool#0:shape#42(fn() -> list_type#6)] captures=[]
//   block b0 params=[%bool#0:shape#38(Bool), %list.bool#0:shape#39(list_type#6), %list.list#0:shape#40(list_type#17), %custom#0:shape#105(custom_type#7), %function.list.bool#0:shape#42(fn() -> list_type#6)]
//     %bool#1:shape#38(Bool) = bool.list_length_equals %list.list#0 length=1
//     branch %bool#1 true=b1(%bool#0, %list.bool#0, %list.list#0, %custom#0, %function.list.bool#0) false=b9(%bool#0, %list.bool#0, %custom#0, %function.list.bool#0)
//   block b1 params=[%bool#0:shape#38(Bool), %list.bool#0:shape#39(list_type#6), %list.list#0:shape#40(list_type#17), %custom#0:shape#105(custom_type#7), %function.list.bool#0:shape#42(fn() -> list_type#6)]
//     %list.bool#1:shape#39(list_type#6) = list.bool[type#6] list_index %list.list#0 index=0
//     jump b2(%list.bool#1, %bool#0, %list.bool#0, %custom#0, %function.list.bool#0)
//   block b2 params=[%list.bool#0:shape#39(list_type#6), %bool#0:shape#38(Bool), %list.bool#1:shape#39(list_type#6), %custom#0:shape#105(custom_type#7), %function.list.bool#0:shape#42(fn() -> list_type#6)]
//     %bool#1:shape#38(Bool) = bool.list_length_at_least %list.bool#1 length=1
//     branch %bool#1 true=b3(%bool#0, %list.bool#1, %custom#0, %function.list.bool#0, %list.bool#0) false=b8(%bool#0, %list.bool#1, %custom#0, %function.list.bool#0, %list.bool#0)
//   block b3 params=[%bool#0:shape#38(Bool), %list.bool#0:shape#39(list_type#6), %custom#0:shape#105(custom_type#7), %function.list.bool#0:shape#42(fn() -> list_type#6), %list.bool#1:shape#39(list_type#6)]
//     %list.bool#2:shape#39(list_type#6) = list.bool[type#6] drop_first %list.bool#0 count=1
//     jump b4(%list.bool#2, %bool#0, %list.bool#0, %custom#0, %function.list.bool#0, %list.bool#1)
//   block b4 params=[%list.bool#0:shape#39(list_type#6), %bool#0:shape#38(Bool), %list.bool#1:shape#39(list_type#6), %custom#0:shape#105(custom_type#7), %function.list.bool#0:shape#42(fn() -> list_type#6), %list.bool#2:shape#39(list_type#6)]
//     %bool#1:shape#38(Bool) = bool.list_length_at_least %list.bool#1 length=1
//     branch %bool#1 true=b5(%bool#0, %list.bool#1, %custom#0, %function.list.bool#0, %list.bool#2, %list.bool#0) false=b7(%bool#0, %list.bool#1, %custom#0, %function.list.bool#0, %list.bool#2, %list.bool#0)
//   block b5 params=[%bool#0:shape#38(Bool), %list.bool#0:shape#39(list_type#6), %custom#0:shape#105(custom_type#7), %function.list.bool#0:shape#42(fn() -> list_type#6), %list.bool#1:shape#39(list_type#6), %list.bool#2:shape#39(list_type#6)]
//     %bool#1:shape#38(Bool) = bool.list_index %list.bool#0 index=0
//     jump b6(%bool#1, %bool#0, %list.bool#0, %custom#0, %function.list.bool#0, %list.bool#1, %list.bool#2)
//   block b6 params=[%bool#0:shape#38(Bool), %bool#1:shape#38(Bool), %list.bool#0:shape#39(list_type#6), %custom#0:shape#105(custom_type#7), %function.list.bool#0:shape#42(fn() -> list_type#6), %list.bool#1:shape#39(list_type#6), %list.bool#2:shape#39(list_type#6)]
//     %bool#2:shape#38(Bool) = bool.call bool#0 args=[%bool#1]
//     %function.bool#0:shape#106(fn() -> Bool) = function[Bool] closure target=bool#1 captures=[%bool#0<-%bool#1]
//     %list.bool#3:shape#39(list_type#6) = list.bool[type#6] value elements=[%bool#1]
//     %list.bool#4:shape#39(list_type#6) = list.bool[type#6] spread elements=[%bool#1] tail=%list.bool#0
//     %list.bool#5:shape#39(list_type#6) = list.bool[type#6] call list.bool#1 args=[%list.bool#0]
//     %bool#3:shape#38(Bool) = bool.function_call %function.bool#0 args=[]
//     %tuple#0:shape#107(#(list_type#6)) = tuple.value elements=[%list.bool#0]
//     %list.bool#6:shape#39(list_type#6) = list.bool[type#6] tuple_index %tuple#0 index=0
//     %list.bool#7:shape#39(list_type#6) = list.bool[type#6] custom_field %custom#0 index=0
//     %list.bool#8:shape#39(list_type#6) = list.bool[type#6] function_call %function.list.bool#0 args=[]
//     %tuple#1:shape#108(#(Bool)) = tuple.value elements=[%bool#1]
//     %bool#4:shape#38(Bool) = bool.tuple_index %tuple#1 index=0
//     %custom#1:shape#109(custom_type#19) = custom.construct custom_type#19.constructor#0 fields=[%bool#1]
//     %bool#5:shape#38(Bool) = bool.custom_field %custom#1 index=0
//     %tuple#2:shape#43(#(list_type#6, list_type#6, list_type#6, Bool, list_type#6, list_type#6, list_type#6, list_type#6, Bool, list_type#6, Bool, Bool, Bool)) = tuple.value elements=[%list.bool#3, %list.bool#4, %list.bool#5, %bool#3, %list.bool#6, %list.bool#7, %list.bool#1, %list.bool#2, %bool#2, %list.bool#8, %bool#4, %bool#5, %bool#0]
//     return %tuple#2
//   block b7 params=[%bool#0:shape#38(Bool), %list.bool#0:shape#39(list_type#6), %custom#0:shape#105(custom_type#7), %function.list.bool#0:shape#42(fn() -> list_type#6), %list.bool#1:shape#39(list_type#6), %list.bool#2:shape#39(list_type#6)]
//     jump b6(%bool#0, %bool#0, %list.bool#0, %custom#0, %function.list.bool#0, %list.bool#1, %list.bool#2)
//   block b8 params=[%bool#0:shape#38(Bool), %list.bool#0:shape#39(list_type#6), %custom#0:shape#105(custom_type#7), %function.list.bool#0:shape#42(fn() -> list_type#6), %list.bool#1:shape#39(list_type#6)]
//     %list.bool#2:shape#39(list_type#6) = list.bool[type#6] value elements=[]
//     jump b4(%list.bool#2, %bool#0, %list.bool#0, %custom#0, %function.list.bool#0, %list.bool#1)
//   block b9 params=[%bool#0:shape#38(Bool), %list.bool#0:shape#39(list_type#6), %custom#0:shape#105(custom_type#7), %function.list.bool#0:shape#42(fn() -> list_type#6)]
//     %list.bool#1:shape#39(list_type#6) = list.bool[type#6] value elements=[]
//     jump b2(%list.bool#1, %bool#0, %list.bool#0, %custom#0, %function.list.bool#0)
//
// function tuple#8
//   entry b0 params=[%nil#0:shape#44(Nil), %list.nil#0:shape#45(list_type#7), %list.list#0:shape#46(list_type#18), %custom#0:shape#110(custom_type#8), %function.list.nil#0:shape#48(fn() -> list_type#7)] captures=[]
//   block b0 params=[%nil#0:shape#44(Nil), %list.nil#0:shape#45(list_type#7), %list.list#0:shape#46(list_type#18), %custom#0:shape#110(custom_type#8), %function.list.nil#0:shape#48(fn() -> list_type#7)]
//     %bool#0:shape#38(Bool) = bool.list_length_equals %list.list#0 length=1
//     branch %bool#0 true=b1(%nil#0, %list.nil#0, %list.list#0, %custom#0, %function.list.nil#0) false=b9(%nil#0, %list.nil#0, %custom#0, %function.list.nil#0)
//   block b1 params=[%nil#0:shape#44(Nil), %list.nil#0:shape#45(list_type#7), %list.list#0:shape#46(list_type#18), %custom#0:shape#110(custom_type#8), %function.list.nil#0:shape#48(fn() -> list_type#7)]
//     %list.nil#1:shape#45(list_type#7) = list.nil[type#7] list_index %list.list#0 index=0
//     jump b2(%list.nil#1, %nil#0, %list.nil#0, %custom#0, %function.list.nil#0)
//   block b2 params=[%list.nil#0:shape#45(list_type#7), %nil#0:shape#44(Nil), %list.nil#1:shape#45(list_type#7), %custom#0:shape#110(custom_type#8), %function.list.nil#0:shape#48(fn() -> list_type#7)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.nil#1 length=1
//     branch %bool#0 true=b3(%nil#0, %list.nil#1, %custom#0, %function.list.nil#0, %list.nil#0) false=b8(%nil#0, %list.nil#1, %custom#0, %function.list.nil#0, %list.nil#0)
//   block b3 params=[%nil#0:shape#44(Nil), %list.nil#0:shape#45(list_type#7), %custom#0:shape#110(custom_type#8), %function.list.nil#0:shape#48(fn() -> list_type#7), %list.nil#1:shape#45(list_type#7)]
//     %list.nil#2:shape#45(list_type#7) = list.nil[type#7] drop_first %list.nil#0 count=1
//     jump b4(%list.nil#2, %nil#0, %list.nil#0, %custom#0, %function.list.nil#0, %list.nil#1)
//   block b4 params=[%list.nil#0:shape#45(list_type#7), %nil#0:shape#44(Nil), %list.nil#1:shape#45(list_type#7), %custom#0:shape#110(custom_type#8), %function.list.nil#0:shape#48(fn() -> list_type#7), %list.nil#2:shape#45(list_type#7)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.nil#1 length=1
//     branch %bool#0 true=b5(%nil#0, %list.nil#1, %custom#0, %function.list.nil#0, %list.nil#2, %list.nil#0) false=b7(%nil#0, %list.nil#1, %custom#0, %function.list.nil#0, %list.nil#2, %list.nil#0)
//   block b5 params=[%nil#0:shape#44(Nil), %list.nil#0:shape#45(list_type#7), %custom#0:shape#110(custom_type#8), %function.list.nil#0:shape#48(fn() -> list_type#7), %list.nil#1:shape#45(list_type#7), %list.nil#2:shape#45(list_type#7)]
//     %nil#1:shape#44(Nil) = nil.list_index %list.nil#0 index=0
//     jump b6(%nil#1, %nil#0, %list.nil#0, %custom#0, %function.list.nil#0, %list.nil#1, %list.nil#2)
//   block b6 params=[%nil#0:shape#44(Nil), %nil#1:shape#44(Nil), %list.nil#0:shape#45(list_type#7), %custom#0:shape#110(custom_type#8), %function.list.nil#0:shape#48(fn() -> list_type#7), %list.nil#1:shape#45(list_type#7), %list.nil#2:shape#45(list_type#7)]
//     %nil#2:shape#44(Nil) = nil.call nil#0 args=[%nil#1]
//     %function.nil#0:shape#111(fn() -> Nil) = function[Nil] closure target=nil#1 captures=[%nil#0<-%nil#1]
//     %list.nil#3:shape#45(list_type#7) = list.nil[type#7] value elements=[%nil#1]
//     %list.nil#4:shape#45(list_type#7) = list.nil[type#7] spread elements=[%nil#1] tail=%list.nil#0
//     %list.nil#5:shape#45(list_type#7) = list.nil[type#7] call list.nil#1 args=[%list.nil#0]
//     %nil#3:shape#44(Nil) = nil.function_call %function.nil#0 args=[]
//     %tuple#0:shape#112(#(list_type#7)) = tuple.value elements=[%list.nil#0]
//     %list.nil#6:shape#45(list_type#7) = list.nil[type#7] tuple_index %tuple#0 index=0
//     %list.nil#7:shape#45(list_type#7) = list.nil[type#7] custom_field %custom#0 index=0
//     %list.nil#8:shape#45(list_type#7) = list.nil[type#7] function_call %function.list.nil#0 args=[]
//     %tuple#1:shape#113(#(Nil)) = tuple.value elements=[%nil#1]
//     %nil#4:shape#44(Nil) = nil.tuple_index %tuple#1 index=0
//     %custom#1:shape#114(custom_type#20) = custom.construct custom_type#20.constructor#0 fields=[%nil#1]
//     %nil#5:shape#44(Nil) = nil.custom_field %custom#1 index=0
//     %tuple#2:shape#49(#(list_type#7, list_type#7, list_type#7, Nil, list_type#7, list_type#7, list_type#7, list_type#7, Nil, list_type#7, Nil, Nil, Nil)) = tuple.value elements=[%list.nil#3, %list.nil#4, %list.nil#5, %nil#3, %list.nil#6, %list.nil#7, %list.nil#1, %list.nil#2, %nil#2, %list.nil#8, %nil#4, %nil#5, %nil#0]
//     return %tuple#2
//   block b7 params=[%nil#0:shape#44(Nil), %list.nil#0:shape#45(list_type#7), %custom#0:shape#110(custom_type#8), %function.list.nil#0:shape#48(fn() -> list_type#7), %list.nil#1:shape#45(list_type#7), %list.nil#2:shape#45(list_type#7)]
//     jump b6(%nil#0, %nil#0, %list.nil#0, %custom#0, %function.list.nil#0, %list.nil#1, %list.nil#2)
//   block b8 params=[%nil#0:shape#44(Nil), %list.nil#0:shape#45(list_type#7), %custom#0:shape#110(custom_type#8), %function.list.nil#0:shape#48(fn() -> list_type#7), %list.nil#1:shape#45(list_type#7)]
//     %list.nil#2:shape#45(list_type#7) = list.nil[type#7] value elements=[]
//     jump b4(%list.nil#2, %nil#0, %list.nil#0, %custom#0, %function.list.nil#0, %list.nil#1)
//   block b9 params=[%nil#0:shape#44(Nil), %list.nil#0:shape#45(list_type#7), %custom#0:shape#110(custom_type#8), %function.list.nil#0:shape#48(fn() -> list_type#7)]
//     %list.nil#1:shape#45(list_type#7) = list.nil[type#7] value elements=[]
//     jump b2(%list.nil#1, %nil#0, %list.nil#0, %custom#0, %function.list.nil#0)
//
// function tuple#9
//   entry b0 params=[%tuple#0:shape#50(#(Int)), %list.tuple#0:shape#51(list_type#8), %list.list#0:shape#52(list_type#19), %custom#0:shape#115(custom_type#9), %function.list.tuple#0:shape#54(fn() -> list_type#8)] captures=[]
//   block b0 params=[%tuple#0:shape#50(#(Int)), %list.tuple#0:shape#51(list_type#8), %list.list#0:shape#52(list_type#19), %custom#0:shape#115(custom_type#9), %function.list.tuple#0:shape#54(fn() -> list_type#8)]
//     %bool#0:shape#38(Bool) = bool.list_length_equals %list.list#0 length=1
//     branch %bool#0 true=b1(%tuple#0, %list.tuple#0, %list.list#0, %custom#0, %function.list.tuple#0) false=b9(%tuple#0, %list.tuple#0, %custom#0, %function.list.tuple#0)
//   block b1 params=[%tuple#0:shape#50(#(Int)), %list.tuple#0:shape#51(list_type#8), %list.list#0:shape#52(list_type#19), %custom#0:shape#115(custom_type#9), %function.list.tuple#0:shape#54(fn() -> list_type#8)]
//     %list.tuple#1:shape#51(list_type#8) = list.tuple[type#8] list_index %list.list#0 index=0
//     jump b2(%list.tuple#1, %tuple#0, %list.tuple#0, %custom#0, %function.list.tuple#0)
//   block b2 params=[%list.tuple#0:shape#51(list_type#8), %tuple#0:shape#50(#(Int)), %list.tuple#1:shape#51(list_type#8), %custom#0:shape#115(custom_type#9), %function.list.tuple#0:shape#54(fn() -> list_type#8)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.tuple#1 length=1
//     branch %bool#0 true=b3(%tuple#0, %list.tuple#1, %custom#0, %function.list.tuple#0, %list.tuple#0) false=b8(%tuple#0, %list.tuple#1, %custom#0, %function.list.tuple#0, %list.tuple#0)
//   block b3 params=[%tuple#0:shape#50(#(Int)), %list.tuple#0:shape#51(list_type#8), %custom#0:shape#115(custom_type#9), %function.list.tuple#0:shape#54(fn() -> list_type#8), %list.tuple#1:shape#51(list_type#8)]
//     %list.tuple#2:shape#51(list_type#8) = list.tuple[type#8] drop_first %list.tuple#0 count=1
//     jump b4(%list.tuple#2, %tuple#0, %list.tuple#0, %custom#0, %function.list.tuple#0, %list.tuple#1)
//   block b4 params=[%list.tuple#0:shape#51(list_type#8), %tuple#0:shape#50(#(Int)), %list.tuple#1:shape#51(list_type#8), %custom#0:shape#115(custom_type#9), %function.list.tuple#0:shape#54(fn() -> list_type#8), %list.tuple#2:shape#51(list_type#8)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.tuple#1 length=1
//     branch %bool#0 true=b5(%tuple#0, %list.tuple#1, %custom#0, %function.list.tuple#0, %list.tuple#2, %list.tuple#0) false=b7(%tuple#0, %list.tuple#1, %custom#0, %function.list.tuple#0, %list.tuple#2, %list.tuple#0)
//   block b5 params=[%tuple#0:shape#50(#(Int)), %list.tuple#0:shape#51(list_type#8), %custom#0:shape#115(custom_type#9), %function.list.tuple#0:shape#54(fn() -> list_type#8), %list.tuple#1:shape#51(list_type#8), %list.tuple#2:shape#51(list_type#8)]
//     %tuple#1:shape#50(#(Int)) = tuple.list_index %list.tuple#0 index=0
//     jump b6(%tuple#1, %tuple#0, %list.tuple#0, %custom#0, %function.list.tuple#0, %list.tuple#1, %list.tuple#2)
//   block b6 params=[%tuple#0:shape#50(#(Int)), %tuple#1:shape#50(#(Int)), %list.tuple#0:shape#51(list_type#8), %custom#0:shape#115(custom_type#9), %function.list.tuple#0:shape#54(fn() -> list_type#8), %list.tuple#1:shape#51(list_type#8), %list.tuple#2:shape#51(list_type#8)]
//     %tuple#2:shape#50(#(Int)) = tuple.call tuple#13 args=[%tuple#1]
//     %function.tuple#0:shape#116(fn() -> #(Int)) = function[Tuple] closure target=tuple#14 captures=[%tuple#0<-%tuple#1]
//     %list.tuple#3:shape#51(list_type#8) = list.tuple[type#8] value elements=[%tuple#1]
//     %list.tuple#4:shape#51(list_type#8) = list.tuple[type#8] spread elements=[%tuple#1] tail=%list.tuple#0
//     %list.tuple#5:shape#51(list_type#8) = list.tuple[type#8] call list.tuple#1 args=[%list.tuple#0]
//     %tuple#3:shape#50(#(Int)) = tuple.function_call %function.tuple#0 args=[]
//     %tuple#4:shape#117(#(list_type#8)) = tuple.value elements=[%list.tuple#0]
//     %list.tuple#6:shape#51(list_type#8) = list.tuple[type#8] tuple_index %tuple#4 index=0
//     %list.tuple#7:shape#51(list_type#8) = list.tuple[type#8] custom_field %custom#0 index=0
//     %list.tuple#8:shape#51(list_type#8) = list.tuple[type#8] function_call %function.list.tuple#0 args=[]
//     %tuple#5:shape#118(#(#(Int))) = tuple.value elements=[%tuple#1]
//     %tuple#6:shape#50(#(Int)) = tuple.tuple_index %tuple#5 index=0
//     %custom#1:shape#119(custom_type#21) = custom.construct custom_type#21.constructor#0 fields=[%tuple#1]
//     %tuple#7:shape#50(#(Int)) = tuple.custom_field %custom#1 index=0
//     %tuple#8:shape#55(#(list_type#8, list_type#8, list_type#8, #(Int), list_type#8, list_type#8, list_type#8, list_type#8, #(Int), list_type#8, #(Int), #(Int), #(Int))) = tuple.value elements=[%list.tuple#3, %list.tuple#4, %list.tuple#5, %tuple#3, %list.tuple#6, %list.tuple#7, %list.tuple#1, %list.tuple#2, %tuple#2, %list.tuple#8, %tuple#6, %tuple#7, %tuple#0]
//     return %tuple#8
//   block b7 params=[%tuple#0:shape#50(#(Int)), %list.tuple#0:shape#51(list_type#8), %custom#0:shape#115(custom_type#9), %function.list.tuple#0:shape#54(fn() -> list_type#8), %list.tuple#1:shape#51(list_type#8), %list.tuple#2:shape#51(list_type#8)]
//     jump b6(%tuple#0, %tuple#0, %list.tuple#0, %custom#0, %function.list.tuple#0, %list.tuple#1, %list.tuple#2)
//   block b8 params=[%tuple#0:shape#50(#(Int)), %list.tuple#0:shape#51(list_type#8), %custom#0:shape#115(custom_type#9), %function.list.tuple#0:shape#54(fn() -> list_type#8), %list.tuple#1:shape#51(list_type#8)]
//     %list.tuple#2:shape#51(list_type#8) = list.tuple[type#8] value elements=[]
//     jump b4(%list.tuple#2, %tuple#0, %list.tuple#0, %custom#0, %function.list.tuple#0, %list.tuple#1)
//   block b9 params=[%tuple#0:shape#50(#(Int)), %list.tuple#0:shape#51(list_type#8), %custom#0:shape#115(custom_type#9), %function.list.tuple#0:shape#54(fn() -> list_type#8)]
//     %list.tuple#1:shape#51(list_type#8) = list.tuple[type#8] value elements=[]
//     jump b2(%list.tuple#1, %tuple#0, %list.tuple#0, %custom#0, %function.list.tuple#0)
//
// function tuple#10
//   entry b0 params=[%list.int#0:shape#1(list_type#0), %list.list#0:shape#2(list_type#9), %list.list#1:shape#56(list_type#20), %custom#0:shape#120(custom_type#10), %function.list.list#0:shape#58(fn() -> list_type#9)] captures=[]
//   block b0 params=[%list.int#0:shape#1(list_type#0), %list.list#0:shape#2(list_type#9), %list.list#1:shape#56(list_type#20), %custom#0:shape#120(custom_type#10), %function.list.list#0:shape#58(fn() -> list_type#9)]
//     %bool#0:shape#38(Bool) = bool.list_length_equals %list.list#1 length=1
//     branch %bool#0 true=b1(%list.int#0, %list.list#0, %list.list#1, %custom#0, %function.list.list#0) false=b9(%list.int#0, %list.list#0, %custom#0, %function.list.list#0)
//   block b1 params=[%list.int#0:shape#1(list_type#0), %list.list#0:shape#2(list_type#9), %list.list#1:shape#56(list_type#20), %custom#0:shape#120(custom_type#10), %function.list.list#0:shape#58(fn() -> list_type#9)]
//     %list.list#2:shape#2(list_type#9) = list.list[type#9] list_index %list.list#1 index=0
//     jump b2(%list.list#2, %list.int#0, %list.list#0, %custom#0, %function.list.list#0)
//   block b2 params=[%list.list#0:shape#2(list_type#9), %list.int#0:shape#1(list_type#0), %list.list#1:shape#2(list_type#9), %custom#0:shape#120(custom_type#10), %function.list.list#0:shape#58(fn() -> list_type#9)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.list#1 length=1
//     branch %bool#0 true=b3(%list.int#0, %list.list#1, %custom#0, %function.list.list#0, %list.list#0) false=b8(%list.int#0, %list.list#1, %custom#0, %function.list.list#0, %list.list#0)
//   block b3 params=[%list.int#0:shape#1(list_type#0), %list.list#0:shape#2(list_type#9), %custom#0:shape#120(custom_type#10), %function.list.list#0:shape#58(fn() -> list_type#9), %list.list#1:shape#2(list_type#9)]
//     %list.list#2:shape#2(list_type#9) = list.list[type#9] drop_first %list.list#0 count=1
//     jump b4(%list.list#2, %list.int#0, %list.list#0, %custom#0, %function.list.list#0, %list.list#1)
//   block b4 params=[%list.list#0:shape#2(list_type#9), %list.int#0:shape#1(list_type#0), %list.list#1:shape#2(list_type#9), %custom#0:shape#120(custom_type#10), %function.list.list#0:shape#58(fn() -> list_type#9), %list.list#2:shape#2(list_type#9)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.list#1 length=1
//     branch %bool#0 true=b5(%list.int#0, %list.list#1, %custom#0, %function.list.list#0, %list.list#2, %list.list#0) false=b7(%list.int#0, %list.list#1, %custom#0, %function.list.list#0, %list.list#2, %list.list#0)
//   block b5 params=[%list.int#0:shape#1(list_type#0), %list.list#0:shape#2(list_type#9), %custom#0:shape#120(custom_type#10), %function.list.list#0:shape#58(fn() -> list_type#9), %list.list#1:shape#2(list_type#9), %list.list#2:shape#2(list_type#9)]
//     %list.int#1:shape#1(list_type#0) = list.int[type#0] list_index %list.list#0 index=0
//     jump b6(%list.int#1, %list.int#0, %list.list#0, %custom#0, %function.list.list#0, %list.list#1, %list.list#2)
//   block b6 params=[%list.int#0:shape#1(list_type#0), %list.int#1:shape#1(list_type#0), %list.list#0:shape#2(list_type#9), %custom#0:shape#120(custom_type#10), %function.list.list#0:shape#58(fn() -> list_type#9), %list.list#1:shape#2(list_type#9), %list.list#2:shape#2(list_type#9)]
//     %list.int#2:shape#1(list_type#0) = list.int[type#0] call list.int#2 args=[%list.int#1]
//     %function.list.int#0:shape#4(fn() -> list_type#0) = function[List] closure target=list.int#3 captures=[%list.int#0<-%list.int#1]
//     %list.list#3:shape#2(list_type#9) = list.list[type#9] value elements=[%list.int#1]
//     %list.list#4:shape#2(list_type#9) = list.list[type#9] spread elements=[%list.int#1] tail=%list.list#0
//     %list.list#5:shape#2(list_type#9) = list.list[type#9] call list.list#1 args=[%list.list#0]
//     %list.int#3:shape#1(list_type#0) = list.int[type#0] function_call %function.list.int#0 args=[]
//     %tuple#0:shape#121(#(list_type#9)) = tuple.value elements=[%list.list#0]
//     %list.list#6:shape#2(list_type#9) = list.list[type#9] tuple_index %tuple#0 index=0
//     %list.list#7:shape#2(list_type#9) = list.list[type#9] custom_field %custom#0 index=0
//     %list.list#8:shape#2(list_type#9) = list.list[type#9] function_call %function.list.list#0 args=[]
//     %tuple#1:shape#77(#(list_type#0)) = tuple.value elements=[%list.int#1]
//     %list.int#4:shape#1(list_type#0) = list.int[type#0] tuple_index %tuple#1 index=0
//     %custom#1:shape#122(custom_type#22) = custom.construct custom_type#22.constructor#0 fields=[%list.int#1]
//     %list.int#5:shape#1(list_type#0) = list.int[type#0] custom_field %custom#1 index=0
//     %tuple#2:shape#59(#(list_type#9, list_type#9, list_type#9, list_type#0, list_type#9, list_type#9, list_type#9, list_type#9, list_type#0, list_type#9, list_type#0, list_type#0, list_type#0)) = tuple.value elements=[%list.list#3, %list.list#4, %list.list#5, %list.int#3, %list.list#6, %list.list#7, %list.list#1, %list.list#2, %list.int#2, %list.list#8, %list.int#4, %list.int#5, %list.int#0]
//     return %tuple#2
//   block b7 params=[%list.int#0:shape#1(list_type#0), %list.list#0:shape#2(list_type#9), %custom#0:shape#120(custom_type#10), %function.list.list#0:shape#58(fn() -> list_type#9), %list.list#1:shape#2(list_type#9), %list.list#2:shape#2(list_type#9)]
//     jump b6(%list.int#0, %list.int#0, %list.list#0, %custom#0, %function.list.list#0, %list.list#1, %list.list#2)
//   block b8 params=[%list.int#0:shape#1(list_type#0), %list.list#0:shape#2(list_type#9), %custom#0:shape#120(custom_type#10), %function.list.list#0:shape#58(fn() -> list_type#9), %list.list#1:shape#2(list_type#9)]
//     %list.list#2:shape#2(list_type#9) = list.list[type#9] value elements=[]
//     jump b4(%list.list#2, %list.int#0, %list.list#0, %custom#0, %function.list.list#0, %list.list#1)
//   block b9 params=[%list.int#0:shape#1(list_type#0), %list.list#0:shape#2(list_type#9), %custom#0:shape#120(custom_type#10), %function.list.list#0:shape#58(fn() -> list_type#9)]
//     %list.list#1:shape#2(list_type#9) = list.list[type#9] value elements=[]
//     jump b2(%list.list#1, %list.int#0, %list.list#0, %custom#0, %function.list.list#0)
//
// function tuple#11
//   entry b0 params=[%function.int#0:shape#60(fn(Int) -> Int), %list.function#0:shape#61(list_type#10), %list.list#0:shape#62(list_type#21), %custom#0:shape#123(custom_type#11), %function.list.function#0:shape#64(fn() -> list_type#10)] captures=[]
//   block b0 params=[%function.int#0:shape#60(fn(Int) -> Int), %list.function#0:shape#61(list_type#10), %list.list#0:shape#62(list_type#21), %custom#0:shape#123(custom_type#11), %function.list.function#0:shape#64(fn() -> list_type#10)]
//     %bool#0:shape#38(Bool) = bool.list_length_equals %list.list#0 length=1
//     branch %bool#0 true=b1(%function.int#0, %list.function#0, %list.list#0, %custom#0, %function.list.function#0) false=b9(%function.int#0, %list.function#0, %custom#0, %function.list.function#0)
//   block b1 params=[%function.int#0:shape#60(fn(Int) -> Int), %list.function#0:shape#61(list_type#10), %list.list#0:shape#62(list_type#21), %custom#0:shape#123(custom_type#11), %function.list.function#0:shape#64(fn() -> list_type#10)]
//     %list.function#1:shape#61(list_type#10) = list.function[type#10] list_index %list.list#0 index=0
//     jump b2(%list.function#1, %function.int#0, %list.function#0, %custom#0, %function.list.function#0)
//   block b2 params=[%list.function#0:shape#61(list_type#10), %function.int#0:shape#60(fn(Int) -> Int), %list.function#1:shape#61(list_type#10), %custom#0:shape#123(custom_type#11), %function.list.function#0:shape#64(fn() -> list_type#10)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.function#1 length=1
//     branch %bool#0 true=b3(%function.int#0, %list.function#1, %custom#0, %function.list.function#0, %list.function#0) false=b8(%function.int#0, %list.function#1, %custom#0, %function.list.function#0, %list.function#0)
//   block b3 params=[%function.int#0:shape#60(fn(Int) -> Int), %list.function#0:shape#61(list_type#10), %custom#0:shape#123(custom_type#11), %function.list.function#0:shape#64(fn() -> list_type#10), %list.function#1:shape#61(list_type#10)]
//     %list.function#2:shape#61(list_type#10) = list.function[type#10] drop_first %list.function#0 count=1
//     jump b4(%list.function#2, %function.int#0, %list.function#0, %custom#0, %function.list.function#0, %list.function#1)
//   block b4 params=[%list.function#0:shape#61(list_type#10), %function.int#0:shape#60(fn(Int) -> Int), %list.function#1:shape#61(list_type#10), %custom#0:shape#123(custom_type#11), %function.list.function#0:shape#64(fn() -> list_type#10), %list.function#2:shape#61(list_type#10)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.function#1 length=1
//     branch %bool#0 true=b5(%function.int#0, %list.function#1, %custom#0, %function.list.function#0, %list.function#2, %list.function#0) false=b7(%function.int#0, %list.function#1, %custom#0, %function.list.function#0, %list.function#2, %list.function#0)
//   block b5 params=[%function.int#0:shape#60(fn(Int) -> Int), %list.function#0:shape#61(list_type#10), %custom#0:shape#123(custom_type#11), %function.list.function#0:shape#64(fn() -> list_type#10), %list.function#1:shape#61(list_type#10), %list.function#2:shape#61(list_type#10)]
//     %function.int#1:shape#60(fn(Int) -> Int) = function[Int] list_index %list.function#0 index=0
//     jump b6(%function.int#1, %function.int#0, %list.function#0, %custom#0, %function.list.function#0, %list.function#1, %list.function#2)
//   block b6 params=[%function.int#0:shape#60(fn(Int) -> Int), %function.int#1:shape#60(fn(Int) -> Int), %list.function#0:shape#61(list_type#10), %custom#0:shape#123(custom_type#11), %function.list.function#0:shape#64(fn() -> list_type#10), %list.function#1:shape#61(list_type#10), %list.function#2:shape#61(list_type#10)]
//     %function.int#2:shape#60(fn(Int) -> Int) = function[Int] call function.int#0 args=[%function.int#1]
//     %function.function#0:shape#124(fn() -> fn(Int) -> Int) = function[Function] closure target=function.int#1 captures=[%function.int#0<-%function.int#1]
//     %list.function#3:shape#61(list_type#10) = list.function[type#10] value elements=[%function.int#1]
//     %list.function#4:shape#61(list_type#10) = list.function[type#10] spread elements=[%function.int#1] tail=%list.function#0
//     %list.function#5:shape#61(list_type#10) = list.function[type#10] call list.function#2 args=[%list.function#0]
//     %function.int#3:shape#60(fn(Int) -> Int) = function[Int] function_call %function.function#0 args=[]
//     %tuple#0:shape#125(#(list_type#10)) = tuple.value elements=[%list.function#0]
//     %list.function#6:shape#61(list_type#10) = list.function[type#10] tuple_index %tuple#0 index=0
//     %list.function#7:shape#61(list_type#10) = list.function[type#10] custom_field %custom#0 index=0
//     %list.function#8:shape#61(list_type#10) = list.function[type#10] function_call %function.list.function#0 args=[]
//     %tuple#1:shape#126(#(fn(Int) -> Int)) = tuple.value elements=[%function.int#1]
//     %function.int#4:shape#60(fn(Int) -> Int) = function[Int] tuple_index %tuple#1 index=0
//     %custom#1:shape#127(custom_type#23) = custom.construct custom_type#23.constructor#0 fields=[%function.int#1]
//     %function.int#5:shape#60(fn(Int) -> Int) = function[Int] custom_field %custom#1 index=0
//     %tuple#2:shape#65(#(list_type#10, list_type#10, list_type#10, fn(Int) -> Int, list_type#10, list_type#10, list_type#10, list_type#10, fn(Int) -> Int, list_type#10, fn(Int) -> Int, fn(Int) -> Int, fn(Int) -> Int)) = tuple.value elements=[%list.function#3, %list.function#4, %list.function#5, %function.int#3, %list.function#6, %list.function#7, %list.function#1, %list.function#2, %function.int#2, %list.function#8, %function.int#4, %function.int#5, %function.int#0]
//     return %tuple#2
//   block b7 params=[%function.int#0:shape#60(fn(Int) -> Int), %list.function#0:shape#61(list_type#10), %custom#0:shape#123(custom_type#11), %function.list.function#0:shape#64(fn() -> list_type#10), %list.function#1:shape#61(list_type#10), %list.function#2:shape#61(list_type#10)]
//     jump b6(%function.int#0, %function.int#0, %list.function#0, %custom#0, %function.list.function#0, %list.function#1, %list.function#2)
//   block b8 params=[%function.int#0:shape#60(fn(Int) -> Int), %list.function#0:shape#61(list_type#10), %custom#0:shape#123(custom_type#11), %function.list.function#0:shape#64(fn() -> list_type#10), %list.function#1:shape#61(list_type#10)]
//     %list.function#2:shape#61(list_type#10) = list.function[type#10] value elements=[]
//     jump b4(%list.function#2, %function.int#0, %list.function#0, %custom#0, %function.list.function#0, %list.function#1)
//   block b9 params=[%function.int#0:shape#60(fn(Int) -> Int), %list.function#0:shape#61(list_type#10), %custom#0:shape#123(custom_type#11), %function.list.function#0:shape#64(fn() -> list_type#10)]
//     %list.function#1:shape#61(list_type#10) = list.function[type#10] value elements=[]
//     jump b2(%list.function#1, %function.int#0, %list.function#0, %custom#0, %function.list.function#0)
//
// function tuple#12
//   entry b0 params=[%function.custom#0:shape#66(fn(Int) -> custom_type#0), %list.function#0:shape#71(list_type#11), %list.list#0:shape#128(list_type#22), %custom#0:shape#129(custom_type#12), %function.list.function#0:shape#72(fn() -> list_type#11)] captures=[]
//   block b0 params=[%function.custom#0:shape#66(fn(Int) -> custom_type#0), %list.function#0:shape#71(list_type#11), %list.list#0:shape#128(list_type#22), %custom#0:shape#129(custom_type#12), %function.list.function#0:shape#72(fn() -> list_type#11)]
//     %bool#0:shape#38(Bool) = bool.list_length_equals %list.list#0 length=1
//     branch %bool#0 true=b1(%function.custom#0, %list.function#0, %list.list#0, %custom#0, %function.list.function#0) false=b9(%function.custom#0, %list.function#0, %custom#0, %function.list.function#0)
//   block b1 params=[%function.custom#0:shape#66(fn(Int) -> custom_type#0), %list.function#0:shape#71(list_type#11), %list.list#0:shape#128(list_type#22), %custom#0:shape#129(custom_type#12), %function.list.function#0:shape#72(fn() -> list_type#11)]
//     %list.function#1:shape#71(list_type#11) = list.function[type#11] list_index %list.list#0 index=0
//     jump b2(%list.function#1, %function.custom#0, %list.function#0, %custom#0, %function.list.function#0)
//   block b2 params=[%list.function#0:shape#71(list_type#11), %function.custom#0:shape#66(fn(Int) -> custom_type#0), %list.function#1:shape#71(list_type#11), %custom#0:shape#129(custom_type#12), %function.list.function#0:shape#72(fn() -> list_type#11)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.function#1 length=1
//     branch %bool#0 true=b3(%function.custom#0, %list.function#1, %custom#0, %function.list.function#0, %list.function#0) false=b8(%function.custom#0, %list.function#1, %custom#0, %function.list.function#0, %list.function#0)
//   block b3 params=[%function.custom#0:shape#66(fn(Int) -> custom_type#0), %list.function#0:shape#71(list_type#11), %custom#0:shape#129(custom_type#12), %function.list.function#0:shape#72(fn() -> list_type#11), %list.function#1:shape#71(list_type#11)]
//     %list.function#2:shape#71(list_type#11) = list.function[type#11] drop_first %list.function#0 count=1
//     jump b4(%list.function#2, %function.custom#0, %list.function#0, %custom#0, %function.list.function#0, %list.function#1)
//   block b4 params=[%list.function#0:shape#71(list_type#11), %function.custom#0:shape#66(fn(Int) -> custom_type#0), %list.function#1:shape#71(list_type#11), %custom#0:shape#129(custom_type#12), %function.list.function#0:shape#72(fn() -> list_type#11), %list.function#2:shape#71(list_type#11)]
//     %bool#0:shape#38(Bool) = bool.list_length_at_least %list.function#1 length=1
//     branch %bool#0 true=b5(%function.custom#0, %list.function#1, %custom#0, %function.list.function#0, %list.function#2, %list.function#0) false=b7(%function.custom#0, %list.function#1, %custom#0, %function.list.function#0, %list.function#2, %list.function#0)
//   block b5 params=[%function.custom#0:shape#66(fn(Int) -> custom_type#0), %list.function#0:shape#71(list_type#11), %custom#0:shape#129(custom_type#12), %function.list.function#0:shape#72(fn() -> list_type#11), %list.function#1:shape#71(list_type#11), %list.function#2:shape#71(list_type#11)]
//     %function.custom#1:shape#66(fn(Int) -> custom_type#0) = function[Custom] list_index %list.function#0 index=0
//     jump b6(%function.custom#1, %function.custom#0, %list.function#0, %custom#0, %function.list.function#0, %list.function#1, %list.function#2)
//   block b6 params=[%function.custom#0:shape#66(fn(Int) -> custom_type#0), %function.custom#1:shape#66(fn(Int) -> custom_type#0), %list.function#0:shape#71(list_type#11), %custom#0:shape#129(custom_type#12), %function.list.function#0:shape#72(fn() -> list_type#11), %list.function#1:shape#71(list_type#11), %list.function#2:shape#71(list_type#11)]
//     %function.custom#2:shape#66(fn(Int) -> custom_type#0) = function[Custom] call function.custom#0 args=[%function.custom#1]
//     %function.function#0:shape#130(fn() -> fn(Int) -> custom_type#0) = function[Function] closure target=function.custom#1 captures=[%function.custom#0<-%function.custom#1]
//     %list.function#3:shape#71(list_type#11) = list.function[type#11] value elements=[%function.custom#1]
//     %list.function#4:shape#71(list_type#11) = list.function[type#11] spread elements=[%function.custom#1] tail=%list.function#0
//     %list.function#5:shape#71(list_type#11) = list.function[type#11] call list.function#3 args=[%list.function#0]
//     %function.custom#3:shape#66(fn(Int) -> custom_type#0) = function[Custom] function_call %function.function#0 args=[]
//     %tuple#0:shape#131(#(list_type#11)) = tuple.value elements=[%list.function#0]
//     %list.function#6:shape#71(list_type#11) = list.function[type#11] tuple_index %tuple#0 index=0
//     %list.function#7:shape#71(list_type#11) = list.function[type#11] custom_field %custom#0 index=0
//     %list.function#8:shape#71(list_type#11) = list.function[type#11] function_call %function.list.function#0 args=[]
//     %tuple#1:shape#132(#(fn(Int) -> custom_type#0)) = tuple.value elements=[%function.custom#1]
//     %function.custom#4:shape#66(fn(Int) -> custom_type#0) = function[Custom] tuple_index %tuple#1 index=0
//     %custom#1:shape#133(custom_type#24) = custom.construct custom_type#24.constructor#0 fields=[%function.custom#1]
//     %function.custom#5:shape#66(fn(Int) -> custom_type#0) = function[Custom] custom_field %custom#1 index=0
//     %tuple#2:shape#73(#(list_type#11, list_type#11, list_type#11, fn(Int) -> custom_type#0, list_type#11, list_type#11, list_type#11, list_type#11, fn(Int) -> custom_type#0, list_type#11, fn(Int) -> custom_type#0, fn(Int) -> custom_type#0, fn(Int) -> custom_type#0)) = tuple.value elements=[%list.function#3, %list.function#4, %list.function#5, %function.custom#3, %list.function#6, %list.function#7, %list.function#1, %list.function#2, %function.custom#2, %list.function#8, %function.custom#4, %function.custom#5, %function.custom#0]
//     return %tuple#2
//   block b7 params=[%function.custom#0:shape#66(fn(Int) -> custom_type#0), %list.function#0:shape#71(list_type#11), %custom#0:shape#129(custom_type#12), %function.list.function#0:shape#72(fn() -> list_type#11), %list.function#1:shape#71(list_type#11), %list.function#2:shape#71(list_type#11)]
//     jump b6(%function.custom#0, %function.custom#0, %list.function#0, %custom#0, %function.list.function#0, %list.function#1, %list.function#2)
//   block b8 params=[%function.custom#0:shape#66(fn(Int) -> custom_type#0), %list.function#0:shape#71(list_type#11), %custom#0:shape#129(custom_type#12), %function.list.function#0:shape#72(fn() -> list_type#11), %list.function#1:shape#71(list_type#11)]
//     %list.function#2:shape#71(list_type#11) = list.function[type#11] value elements=[]
//     jump b4(%list.function#2, %function.custom#0, %list.function#0, %custom#0, %function.list.function#0, %list.function#1)
//   block b9 params=[%function.custom#0:shape#66(fn(Int) -> custom_type#0), %list.function#0:shape#71(list_type#11), %custom#0:shape#129(custom_type#12), %function.list.function#0:shape#72(fn() -> list_type#11)]
//     %list.function#1:shape#71(list_type#11) = list.function[type#11] value elements=[]
//     jump b2(%list.function#1, %function.custom#0, %list.function#0, %custom#0, %function.list.function#0)
//
// function tuple#13
//   entry b0 params=[%tuple#0:shape#50(#(Int))] captures=[]
//   block b0 params=[%tuple#0:shape#50(#(Int))]
//     return %tuple#0
//
// function tuple#14
//   entry b0 params=[] captures=[%tuple#0:shape#50(#(Int))]
//   block b0 params=[%tuple#0:shape#50(#(Int))]
//     return %tuple#0
//
// function list.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %list.int#0:shape#1(list_type#0) = list.int[type#0] value elements=[%int#0]
//     return %list.int#0
//
// function list.int#1
//   entry b0 params=[%list.int#0:shape#1(list_type#0)] captures=[]
//   block b0 params=[%list.int#0:shape#1(list_type#0)]
//     return %list.int#0
//
// function list.int#2
//   entry b0 params=[%list.int#0:shape#1(list_type#0)] captures=[]
//   block b0 params=[%list.int#0:shape#1(list_type#0)]
//     return %list.int#0
//
// function list.int#3
//   entry b0 params=[] captures=[%list.int#0:shape#1(list_type#0)]
//   block b0 params=[%list.int#0:shape#1(list_type#0)]
//     return %list.int#0
//
// function list.string#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %string#0:shape#6(String) = string.value "one"
//     %list.string#0:shape#7(list_type#1) = list.string[type#1] value elements=[%string#0]
//     return %list.string#0
//
// function list.string#1
//   entry b0 params=[%list.string#0:shape#7(list_type#1)] captures=[]
//   block b0 params=[%list.string#0:shape#7(list_type#1)]
//     return %list.string#0
//
// function list.bit_array#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %bit_array#0:shape#12(BitArray) = bit_array.value [int(%int#0, bits=8, big)]
//     %list.bit_array#0:shape#13(list_type#2) = list.bit_array[type#2] value elements=[%bit_array#0]
//     return %list.bit_array#0
//
// function list.bit_array#1
//   entry b0 params=[%list.bit_array#0:shape#13(list_type#2)] captures=[]
//   block b0 params=[%list.bit_array#0:shape#13(list_type#2)]
//     return %list.bit_array#0
//
// function list.utf_codepoint#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %utf_codepoint#0:shape#18(UtfCodepoint) = utf_codepoint.call utf_codepoint#0 args=[]
//     %list.utf_codepoint#0:shape#19(list_type#3) = list.utf_codepoint[type#3] value elements=[%utf_codepoint#0]
//     return %list.utf_codepoint#0
//
// function list.utf_codepoint#1
//   entry b0 params=[%list.utf_codepoint#0:shape#19(list_type#3)] captures=[]
//   block b0 params=[%list.utf_codepoint#0:shape#19(list_type#3)]
//     return %list.utf_codepoint#0
//
// function list.custom#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %custom#0:shape#24(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#0]
//     %list.custom#0:shape#25(list_type#4) = list.custom[type#4] value elements=[%custom#0]
//     return %list.custom#0
//
// function list.custom#1
//   entry b0 params=[%list.custom#0:shape#29(list_type#4)] captures=[]
//   block b0 params=[%list.custom#0:shape#29(list_type#4)]
//     return %list.custom#0
//
// function list.float#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %float#0:shape#32(Float) = float.value 1.0
//     %list.float#0:shape#33(list_type#5) = list.float[type#5] value elements=[%float#0]
//     return %list.float#0
//
// function list.float#1
//   entry b0 params=[%list.float#0:shape#33(list_type#5)] captures=[]
//   block b0 params=[%list.float#0:shape#33(list_type#5)]
//     return %list.float#0
//
// function list.bool#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %bool#0:shape#38(Bool) = bool.value True
//     %list.bool#0:shape#39(list_type#6) = list.bool[type#6] value elements=[%bool#0]
//     return %list.bool#0
//
// function list.bool#1
//   entry b0 params=[%list.bool#0:shape#39(list_type#6)] captures=[]
//   block b0 params=[%list.bool#0:shape#39(list_type#6)]
//     return %list.bool#0
//
// function list.nil#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %nil#0:shape#44(Nil) = nil.value
//     %list.nil#0:shape#45(list_type#7) = list.nil[type#7] value elements=[%nil#0]
//     return %list.nil#0
//
// function list.nil#1
//   entry b0 params=[%list.nil#0:shape#45(list_type#7)] captures=[]
//   block b0 params=[%list.nil#0:shape#45(list_type#7)]
//     return %list.nil#0
//
// function list.tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %tuple#0:shape#50(#(Int)) = tuple.value elements=[%int#0]
//     %list.tuple#0:shape#51(list_type#8) = list.tuple[type#8] value elements=[%tuple#0]
//     return %list.tuple#0
//
// function list.tuple#1
//   entry b0 params=[%list.tuple#0:shape#51(list_type#8)] captures=[]
//   block b0 params=[%list.tuple#0:shape#51(list_type#8)]
//     return %list.tuple#0
//
// function list.list#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %list.int#0:shape#1(list_type#0) = list.int[type#0] value elements=[%int#0]
//     %list.list#0:shape#2(list_type#9) = list.list[type#9] value elements=[%list.int#0]
//     return %list.list#0
//
// function list.list#1
//   entry b0 params=[%list.list#0:shape#2(list_type#9)] captures=[]
//   block b0 params=[%list.list#0:shape#2(list_type#9)]
//     return %list.list#0
//
// function list.function#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.int#0:shape#60(fn(Int) -> Int) = function[Int] reference int#0
//     %list.function#0:shape#61(list_type#10) = list.function[type#10] value elements=[%function.int#0]
//     return %list.function#0
//
// function list.function#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.custom#0:shape#66(fn(Int) -> custom_type#0) = function[Custom] constructor custom_type#0.constructor#0
//     %list.function#0:shape#68(list_type#24) = list.function[type#11] value elements=[%function.custom#0]
//     return %list.function#0
//
// function list.function#2
//   entry b0 params=[%list.function#0:shape#61(list_type#10)] captures=[]
//   block b0 params=[%list.function#0:shape#61(list_type#10)]
//     return %list.function#0
//
// function list.function#3
//   entry b0 params=[%list.function#0:shape#71(list_type#11)] captures=[]
//   block b0 params=[%list.function#0:shape#71(list_type#11)]
//     return %list.function#0
//
// function function.int#0
//   entry b0 params=[%function.int#0:shape#60(fn(Int) -> Int)] captures=[]
//   block b0 params=[%function.int#0:shape#60(fn(Int) -> Int)]
//     return %function.int#0
//
// function function.int#1
//   entry b0 params=[] captures=[%function.int#0:shape#60(fn(Int) -> Int)]
//   block b0 params=[%function.int#0:shape#60(fn(Int) -> Int)]
//     return %function.int#0
//
// function function.custom#0
//   entry b0 params=[%function.custom#0:shape#66(fn(Int) -> custom_type#0)] captures=[]
//   block b0 params=[%function.custom#0:shape#66(fn(Int) -> custom_type#0)]
//     return %function.custom#0
//
// function function.custom#1
//   entry b0 params=[] captures=[%function.custom#0:shape#66(fn(Int) -> custom_type#0)]
//   block b0 params=[%function.custom#0:shape#66(fn(Int) -> custom_type#0)]
//     return %function.custom#0
