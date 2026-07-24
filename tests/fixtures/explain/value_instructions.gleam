pub type Everything {
  Everything(
    int: Int,
    float: Float,
    string: String,
    bits: BitArray,
    codepoint: UtfCodepoint,
    bool: Bool,
    nil: Nil,
    tuple: #(Int),
  )
}

fn codepoint() -> UtfCodepoint {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn int_value(value: Int) { value }
fn float_value(value: Float) { value }
fn string_value(value: String) { value }
fn bit_array_value(value: BitArray) { value }
fn codepoint_value(value: UtfCodepoint) { value }
fn custom_value(value: Everything) { value }
fn bool_value(value: Bool) { value }
fn nil_value(value: Nil) { value }
fn tuple_value(value: #(Int)) { value }

pub fn main() {
  let int_function = int_value
  let float_function = float_value
  let string_function = string_value
  let bit_array_function = bit_array_value
  let codepoint_function = codepoint_value
  let custom_function = custom_value
  let bool_function = bool_value
  let nil_function = nil_value
  let tuple_function = tuple_value
  let scalar = codepoint()
  let record = Everything(1, 1.5, "one", <<1>>, scalar, True, Nil, #(1))
  let tuple = #(1, 1.5, "one", <<1>>, scalar, record, True, Nil, #(1))
  let text = "prefix-tail"
  let assert "prefix-" <> suffix = text
  let sized = 4
  let one = 1
  let extracted = case text {
    "prefix-" <> rest -> rest
    _ -> ""
  }

  #(
    1 + 2 - 3 * 4 / 5 % 6,
    -one,
    1.0 +. 2.0 -. 3.0 *. 4.0 /. 5.0,
    "one" <> "two",
    suffix,
    extracted,
    !False,
    1 < 2,
    1 <= 2,
    2 > 1,
    2 >= 1,
    1.0 <. 2.0,
    1.0 <=. 2.0,
    2.0 >. 1.0,
    2.0 >=. 1.0,
    1 == 1,
    1 != 2,
    int_value(1),
    int_function(1),
    float_value(1.0),
    float_function(1.0),
    string_value("one"),
    string_function("one"),
    bit_array_value(<<1>>),
    bit_array_function(<<1>>),
    codepoint_value(scalar),
    codepoint_function(scalar),
    custom_value(record),
    custom_function(record),
    bool_value(True),
    bool_function(True),
    nil_value(Nil),
    nil_function(Nil),
    tuple_value(#(1)),
    tuple_function(#(1)),
    tuple.0,
    tuple.1,
    tuple.2,
    tuple.3,
    tuple.4,
    tuple.5,
    tuple.6,
    tuple.7,
    tuple.8,
    record.int,
    record.float,
    record.string,
    record.bits,
    record.codepoint,
    record,
    record.bool,
    record.nil,
    record.tuple,
    <<
      1:4-big,
      2:size(sized)-little,
      1.5:float-size(16)-big,
      1.5:float-size(sized * 8)-little,
      "one":utf8,
      "two":utf16-big,
      "three":utf32-little,
      scalar:utf8_codepoint,
      scalar:utf16_codepoint-little,
      scalar:utf32_codepoint-big,
      <<1>>:bits,
      <<2>>:bits-size(4),
      <<3>>:bits-size(sized),
    >>,
  )
}


// @geam:explain
// module main
// main tuple#0
//
// function int#0
//   entry b0 params=[%int#0:shape#1(Int)] captures=[]
//   block b0 params=[%int#0:shape#1(Int)]
//     return %int#0
//
// function float#0
//   entry b0 params=[%float#0:shape#3(Float)] captures=[]
//   block b0 params=[%float#0:shape#3(Float)]
//     return %float#0
//
// function string#0
//   entry b0 params=[%string#0:shape#5(String)] captures=[]
//   block b0 params=[%string#0:shape#5(String)]
//     return %string#0
//
// function bit_array#0
//   entry b0 params=[%bit_array#0:shape#7(BitArray)] captures=[]
//   block b0 params=[%bit_array#0:shape#7(BitArray)]
//     return %bit_array#0
//
// function utf_codepoint#0
//   entry b0 params=[%utf_codepoint#0:shape#9(UtfCodepoint)] captures=[]
//   block b0 params=[%utf_codepoint#0:shape#9(UtfCodepoint)]
//     return %utf_codepoint#0
//
// function utf_codepoint#1
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#1(Int) = int.value 65
//     %bit_array#0:shape#7(BitArray) = bit_array.value [int(%int#0, bits=8, big)]
//     match %bit_array#0 pattern=<<utf_codepoint(binding#0, utf8)>> success=b1(binding#0) failure=b2(%bit_array#0)
//   block b1 params=[%utf_codepoint#0:shape#9(UtfCodepoint)]
//     return %utf_codepoint#0
//   block b2 params=[%bit_array#0:shape#7(BitArray)]
//     let_assert_panic subject=%bit_array#0 message=none
//
// function custom#0
//   entry b0 params=[%custom#0:shape#0(custom_type#0)] captures=[]
//   block b0 params=[%custom#0:shape#0(custom_type#0)]
//     return %custom#0
//
// function bool#0
//   entry b0 params=[%bool#0:shape#12(Bool)] captures=[]
//   block b0 params=[%bool#0:shape#12(Bool)]
//     return %bool#0
//
// function nil#0
//   entry b0 params=[%nil#0:shape#14(Nil)] captures=[]
//   block b0 params=[%nil#0:shape#14(Nil)]
//     return %nil#0
//
// function tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.int#0:shape#2(fn(Int) -> Int) = function[Int] reference int#0
//     %function.float#0:shape#4(fn(Float) -> Float) = function[Float] reference float#0
//     %function.string#0:shape#6(fn(String) -> String) = function[String] reference string#0
//     %function.bit_array#0:shape#8(fn(BitArray) -> BitArray) = function[BitArray] reference bit_array#0
//     %function.utf_codepoint#0:shape#10(fn(UtfCodepoint) -> UtfCodepoint) = function[UtfCodepoint] reference utf_codepoint#0
//     %function.custom#0:shape#11(fn(custom_type#0) -> custom_type#0) = function[Custom] reference custom#0
//     %function.bool#0:shape#13(fn(Bool) -> Bool) = function[Bool] reference bool#0
//     %function.nil#0:shape#15(fn(Nil) -> Nil) = function[Nil] reference nil#0
//     %function.tuple#0:shape#17(fn(#(Int)) -> #(Int)) = function[Tuple] reference tuple#1
//     %utf_codepoint#0:shape#9(UtfCodepoint) = utf_codepoint.call utf_codepoint#1 args=[]
//     %int#0:shape#1(Int) = int.value 1
//     %float#0:shape#3(Float) = float.value 1.5
//     %string#0:shape#5(String) = string.value "one"
//     %int#1:shape#1(Int) = int.value 1
//     %bit_array#0:shape#7(BitArray) = bit_array.value [int(%int#1, bits=8, big)]
//     %bool#0:shape#12(Bool) = bool.value True
//     %nil#0:shape#14(Nil) = nil.value
//     %int#2:shape#1(Int) = int.value 1
//     %tuple#0:shape#16(#(Int)) = tuple.value elements=[%int#2]
//     %custom#0:shape#18(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#0, %float#0, %string#0, %bit_array#0, %utf_codepoint#0, %bool#0, %nil#0, %tuple#0]
//     %int#3:shape#1(Int) = int.value 1
//     %float#1:shape#3(Float) = float.value 1.5
//     %string#1:shape#5(String) = string.value "one"
//     %int#4:shape#1(Int) = int.value 1
//     %bit_array#1:shape#7(BitArray) = bit_array.value [int(%int#4, bits=8, big)]
//     %bool#1:shape#12(Bool) = bool.value True
//     %nil#1:shape#14(Nil) = nil.value
//     %int#5:shape#1(Int) = int.value 1
//     %tuple#1:shape#16(#(Int)) = tuple.value elements=[%int#5]
//     %tuple#2:shape#19(#(Int, Float, String, BitArray, UtfCodepoint, custom_type#0, Bool, Nil, #(Int))) = tuple.value elements=[%int#3, %float#1, %string#1, %bit_array#1, %utf_codepoint#0, %custom#0, %bool#1, %nil#1, %tuple#1]
//     %string#2:shape#5(String) = string.value "prefix-tail"
//     match %string#2 pattern=string_prefix("prefix-", left=_, right=binding#0) success=b1(binding#0, %function.int#0, %function.float#0, %function.string#0, %function.bit_array#0, %function.utf_codepoint#0, %function.custom#0, %function.bool#0, %function.nil#0, %function.tuple#0, %utf_codepoint#0, %custom#0, %tuple#2, %string#2) failure=b5(%string#2)
//   block b1 params=[%string#0:shape#5(String), %function.int#0:shape#2(fn(Int) -> Int), %function.float#0:shape#4(fn(Float) -> Float), %function.string#0:shape#6(fn(String) -> String), %function.bit_array#0:shape#8(fn(BitArray) -> BitArray), %function.utf_codepoint#0:shape#10(fn(UtfCodepoint) -> UtfCodepoint), %function.custom#0:shape#11(fn(custom_type#0) -> custom_type#0), %function.bool#0:shape#13(fn(Bool) -> Bool), %function.nil#0:shape#15(fn(Nil) -> Nil), %function.tuple#0:shape#17(fn(#(Int)) -> #(Int)), %utf_codepoint#0:shape#9(UtfCodepoint), %custom#0:shape#18(custom_type#0), %tuple#0:shape#19(#(Int, Float, String, BitArray, UtfCodepoint, custom_type#0, Bool, Nil, #(Int))), %string#1:shape#5(String)]
//     %int#0:shape#1(Int) = int.value 4
//     %int#1:shape#1(Int) = int.value 1
//     %bool#0:shape#12(Bool) = bool.string_starts_with %string#1 prefix="prefix-"
//     branch %bool#0 true=b2(%function.int#0, %function.float#0, %function.string#0, %function.bit_array#0, %function.utf_codepoint#0, %function.custom#0, %function.bool#0, %function.nil#0, %function.tuple#0, %utf_codepoint#0, %custom#0, %tuple#0, %string#1, %string#0, %int#0, %int#1) false=b4(%function.int#0, %function.float#0, %function.string#0, %function.bit_array#0, %function.utf_codepoint#0, %function.custom#0, %function.bool#0, %function.nil#0, %function.tuple#0, %utf_codepoint#0, %custom#0, %tuple#0, %string#0, %int#0, %int#1)
//   block b2 params=[%function.int#0:shape#2(fn(Int) -> Int), %function.float#0:shape#4(fn(Float) -> Float), %function.string#0:shape#6(fn(String) -> String), %function.bit_array#0:shape#8(fn(BitArray) -> BitArray), %function.utf_codepoint#0:shape#10(fn(UtfCodepoint) -> UtfCodepoint), %function.custom#0:shape#11(fn(custom_type#0) -> custom_type#0), %function.bool#0:shape#13(fn(Bool) -> Bool), %function.nil#0:shape#15(fn(Nil) -> Nil), %function.tuple#0:shape#17(fn(#(Int)) -> #(Int)), %utf_codepoint#0:shape#9(UtfCodepoint), %custom#0:shape#18(custom_type#0), %tuple#0:shape#19(#(Int, Float, String, BitArray, UtfCodepoint, custom_type#0, Bool, Nil, #(Int))), %string#0:shape#5(String), %string#1:shape#5(String), %int#0:shape#1(Int), %int#1:shape#1(Int)]
//     %string#2:shape#5(String) = string.drop_prefix %string#0 prefix="prefix-"
//     jump b3(%string#2, %function.int#0, %function.float#0, %function.string#0, %function.bit_array#0, %function.utf_codepoint#0, %function.custom#0, %function.bool#0, %function.nil#0, %function.tuple#0, %utf_codepoint#0, %custom#0, %tuple#0, %string#1, %int#0, %int#1)
//   block b3 params=[%string#0:shape#5(String), %function.int#0:shape#2(fn(Int) -> Int), %function.float#0:shape#4(fn(Float) -> Float), %function.string#0:shape#6(fn(String) -> String), %function.bit_array#0:shape#8(fn(BitArray) -> BitArray), %function.utf_codepoint#0:shape#10(fn(UtfCodepoint) -> UtfCodepoint), %function.custom#0:shape#11(fn(custom_type#0) -> custom_type#0), %function.bool#0:shape#13(fn(Bool) -> Bool), %function.nil#0:shape#15(fn(Nil) -> Nil), %function.tuple#0:shape#17(fn(#(Int)) -> #(Int)), %utf_codepoint#0:shape#9(UtfCodepoint), %custom#0:shape#18(custom_type#0), %tuple#0:shape#19(#(Int, Float, String, BitArray, UtfCodepoint, custom_type#0, Bool, Nil, #(Int))), %string#1:shape#5(String), %int#0:shape#1(Int), %int#1:shape#1(Int)]
//     %int#2:shape#1(Int) = int.value 1
//     %int#3:shape#1(Int) = int.value 2
//     %int#4:shape#1(Int) = int.add %int#2 %int#3
//     %int#5:shape#1(Int) = int.value 3
//     %int#6:shape#1(Int) = int.value 4
//     %int#7:shape#1(Int) = int.mult %int#5 %int#6
//     %int#8:shape#1(Int) = int.value 5
//     %int#9:shape#1(Int) = int.div %int#7 %int#8
//     %int#10:shape#1(Int) = int.value 6
//     %int#11:shape#1(Int) = int.remainder %int#9 %int#10
//     %int#12:shape#1(Int) = int.sub %int#4 %int#11
//     %int#13:shape#1(Int) = int.negate %int#1
//     %float#0:shape#3(Float) = float.value 1.0
//     %float#1:shape#3(Float) = float.value 2.0
//     %float#2:shape#3(Float) = float.add %float#0 %float#1
//     %float#3:shape#3(Float) = float.value 3.0
//     %float#4:shape#3(Float) = float.value 4.0
//     %float#5:shape#3(Float) = float.mult %float#3 %float#4
//     %float#6:shape#3(Float) = float.value 5.0
//     %float#7:shape#3(Float) = float.div %float#5 %float#6
//     %float#8:shape#3(Float) = float.sub %float#2 %float#7
//     %string#2:shape#5(String) = string.value "one"
//     %string#3:shape#5(String) = string.value "two"
//     %string#4:shape#5(String) = string.concatenate %string#2 %string#3
//     %bool#0:shape#12(Bool) = bool.value False
//     %bool#1:shape#12(Bool) = bool.not %bool#0
//     %int#14:shape#1(Int) = int.value 1
//     %int#15:shape#1(Int) = int.value 2
//     %bool#2:shape#12(Bool) = bool.lt_int %int#14 %int#15
//     %int#16:shape#1(Int) = int.value 1
//     %int#17:shape#1(Int) = int.value 2
//     %bool#3:shape#12(Bool) = bool.lte_int %int#16 %int#17
//     %int#18:shape#1(Int) = int.value 2
//     %int#19:shape#1(Int) = int.value 1
//     %bool#4:shape#12(Bool) = bool.gt_int %int#18 %int#19
//     %int#20:shape#1(Int) = int.value 2
//     %int#21:shape#1(Int) = int.value 1
//     %bool#5:shape#12(Bool) = bool.gte_int %int#20 %int#21
//     %float#9:shape#3(Float) = float.value 1.0
//     %float#10:shape#3(Float) = float.value 2.0
//     %bool#6:shape#12(Bool) = bool.lt_float %float#9 %float#10
//     %float#11:shape#3(Float) = float.value 1.0
//     %float#12:shape#3(Float) = float.value 2.0
//     %bool#7:shape#12(Bool) = bool.lte_float %float#11 %float#12
//     %float#13:shape#3(Float) = float.value 2.0
//     %float#14:shape#3(Float) = float.value 1.0
//     %bool#8:shape#12(Bool) = bool.gt_float %float#13 %float#14
//     %float#15:shape#3(Float) = float.value 2.0
//     %float#16:shape#3(Float) = float.value 1.0
//     %bool#9:shape#12(Bool) = bool.gte_float %float#15 %float#16
//     %int#22:shape#1(Int) = int.value 1
//     %int#23:shape#1(Int) = int.value 1
//     %bool#10:shape#12(Bool) = bool.equal %int#22 %int#23
//     %int#24:shape#1(Int) = int.value 1
//     %int#25:shape#1(Int) = int.value 2
//     %bool#11:shape#12(Bool) = bool.not_equal %int#24 %int#25
//     %int#26:shape#1(Int) = int.value 1
//     %int#27:shape#1(Int) = int.call int#0 args=[%int#26]
//     %int#28:shape#1(Int) = int.value 1
//     %int#29:shape#1(Int) = int.function_call %function.int#0 args=[%int#28]
//     %float#17:shape#3(Float) = float.value 1.0
//     %float#18:shape#3(Float) = float.call float#0 args=[%float#17]
//     %float#19:shape#3(Float) = float.value 1.0
//     %float#20:shape#3(Float) = float.function_call %function.float#0 args=[%float#19]
//     %string#5:shape#5(String) = string.value "one"
//     %string#6:shape#5(String) = string.call string#0 args=[%string#5]
//     %string#7:shape#5(String) = string.value "one"
//     %string#8:shape#5(String) = string.function_call %function.string#0 args=[%string#7]
//     %int#30:shape#1(Int) = int.value 1
//     %bit_array#0:shape#7(BitArray) = bit_array.value [int(%int#30, bits=8, big)]
//     %bit_array#1:shape#7(BitArray) = bit_array.call bit_array#0 args=[%bit_array#0]
//     %int#31:shape#1(Int) = int.value 1
//     %bit_array#2:shape#7(BitArray) = bit_array.value [int(%int#31, bits=8, big)]
//     %bit_array#3:shape#7(BitArray) = bit_array.function_call %function.bit_array#0 args=[%bit_array#2]
//     %utf_codepoint#1:shape#9(UtfCodepoint) = utf_codepoint.call utf_codepoint#0 args=[%utf_codepoint#0]
//     %utf_codepoint#2:shape#9(UtfCodepoint) = utf_codepoint.function_call %function.utf_codepoint#0 args=[%utf_codepoint#0]
//     %custom#1:shape#0(custom_type#0) = custom.call custom#0 args=[%custom#0]
//     %custom#2:shape#0(custom_type#0) = custom.function_call %function.custom#0 args=[%custom#0]
//     %bool#12:shape#12(Bool) = bool.value True
//     %bool#13:shape#12(Bool) = bool.call bool#0 args=[%bool#12]
//     %bool#14:shape#12(Bool) = bool.value True
//     %bool#15:shape#12(Bool) = bool.function_call %function.bool#0 args=[%bool#14]
//     %nil#0:shape#14(Nil) = nil.value
//     %nil#1:shape#14(Nil) = nil.call nil#0 args=[%nil#0]
//     %nil#2:shape#14(Nil) = nil.value
//     %nil#3:shape#14(Nil) = nil.function_call %function.nil#0 args=[%nil#2]
//     %int#32:shape#1(Int) = int.value 1
//     %tuple#1:shape#16(#(Int)) = tuple.value elements=[%int#32]
//     %tuple#2:shape#16(#(Int)) = tuple.call tuple#1 args=[%tuple#1]
//     %int#33:shape#1(Int) = int.value 1
//     %tuple#3:shape#16(#(Int)) = tuple.value elements=[%int#33]
//     %tuple#4:shape#16(#(Int)) = tuple.function_call %function.tuple#0 args=[%tuple#3]
//     %int#34:shape#1(Int) = int.tuple_index %tuple#0 index=0
//     %float#21:shape#3(Float) = float.tuple_index %tuple#0 index=1
//     %string#9:shape#5(String) = string.tuple_index %tuple#0 index=2
//     %bit_array#4:shape#7(BitArray) = bit_array.tuple_index %tuple#0 index=3
//     %utf_codepoint#3:shape#9(UtfCodepoint) = utf_codepoint.tuple_index %tuple#0 index=4
//     %custom#3:shape#18(custom_type#0) = custom.tuple_index %tuple#0 index=5
//     %bool#16:shape#12(Bool) = bool.tuple_index %tuple#0 index=6
//     %nil#4:shape#14(Nil) = nil.tuple_index %tuple#0 index=7
//     %tuple#5:shape#16(#(Int)) = tuple.tuple_index %tuple#0 index=8
//     %int#35:shape#1(Int) = int.custom_field %custom#0 index=0
//     %float#22:shape#3(Float) = float.custom_field %custom#0 index=1
//     %string#10:shape#5(String) = string.custom_field %custom#0 index=2
//     %bit_array#5:shape#7(BitArray) = bit_array.custom_field %custom#0 index=3
//     %utf_codepoint#4:shape#9(UtfCodepoint) = utf_codepoint.custom_field %custom#0 index=4
//     %bool#17:shape#12(Bool) = bool.custom_field %custom#0 index=5
//     %nil#5:shape#14(Nil) = nil.custom_field %custom#0 index=6
//     %tuple#6:shape#16(#(Int)) = tuple.custom_field %custom#0 index=7
//     %int#36:shape#1(Int) = int.value 1
//     %int#37:shape#1(Int) = int.value 2
//     %float#23:shape#3(Float) = float.value 1.5
//     %float#24:shape#3(Float) = float.value 1.5
//     %int#38:shape#1(Int) = int.value 8
//     %int#39:shape#1(Int) = int.mult %int#0 %int#38
//     %string#11:shape#5(String) = string.value "one"
//     %string#12:shape#5(String) = string.value "two"
//     %string#13:shape#5(String) = string.value "three"
//     %int#40:shape#1(Int) = int.value 1
//     %bit_array#6:shape#7(BitArray) = bit_array.value [int(%int#40, bits=8, big)]
//     %int#41:shape#1(Int) = int.value 2
//     %bit_array#7:shape#7(BitArray) = bit_array.value [int(%int#41, bits=8, big)]
//     %int#42:shape#1(Int) = int.value 3
//     %bit_array#8:shape#7(BitArray) = bit_array.value [int(%int#42, bits=8, big)]
//     %bit_array#9:shape#7(BitArray) = bit_array.value [int(%int#36, bits=4, big), int(%int#37, bits=%int#0*1, little), float(%float#23, bits=16, big), float(%float#24, bits=%int#39*1, little), string(%string#11, utf8), string(%string#12, utf16.big), string(%string#13, utf32.little), utf_codepoint(%utf_codepoint#0, utf8), utf_codepoint(%utf_codepoint#0, utf16.little), utf_codepoint(%utf_codepoint#0, utf32.big), bits(%bit_array#6), bits(%bit_array#7, bits=4), bits(%bit_array#8, bits=%int#0*1)]
//     %tuple#7:shape#20(#(Int, Int, Float, String, String, String, Bool, Bool, Bool, Bool, Bool, Bool, Bool, Bool, Bool, Bool, Bool, Int, Int, Float, Float, String, String, BitArray, BitArray, UtfCodepoint, UtfCodepoint, custom_type#0, custom_type#0, Bool, Bool, Nil, Nil, #(Int), #(Int), Int, Float, String, BitArray, UtfCodepoint, custom_type#0, Bool, Nil, #(Int), Int, Float, String, BitArray, UtfCodepoint, custom_type#0, Bool, Nil, #(Int), BitArray)) = tuple.value elements=[%int#12, %int#13, %float#8, %string#4, %string#1, %string#0, %bool#1, %bool#2, %bool#3, %bool#4, %bool#5, %bool#6, %bool#7, %bool#8, %bool#9, %bool#10, %bool#11, %int#27, %int#29, %float#18, %float#20, %string#6, %string#8, %bit_array#1, %bit_array#3, %utf_codepoint#1, %utf_codepoint#2, %custom#1, %custom#2, %bool#13, %bool#15, %nil#1, %nil#3, %tuple#2, %tuple#4, %int#34, %float#21, %string#9, %bit_array#4, %utf_codepoint#3, %custom#3, %bool#16, %nil#4, %tuple#5, %int#35, %float#22, %string#10, %bit_array#5, %utf_codepoint#4, %custom#0, %bool#17, %nil#5, %tuple#6, %bit_array#9]
//     return %tuple#7
//   block b4 params=[%function.int#0:shape#2(fn(Int) -> Int), %function.float#0:shape#4(fn(Float) -> Float), %function.string#0:shape#6(fn(String) -> String), %function.bit_array#0:shape#8(fn(BitArray) -> BitArray), %function.utf_codepoint#0:shape#10(fn(UtfCodepoint) -> UtfCodepoint), %function.custom#0:shape#11(fn(custom_type#0) -> custom_type#0), %function.bool#0:shape#13(fn(Bool) -> Bool), %function.nil#0:shape#15(fn(Nil) -> Nil), %function.tuple#0:shape#17(fn(#(Int)) -> #(Int)), %utf_codepoint#0:shape#9(UtfCodepoint), %custom#0:shape#18(custom_type#0), %tuple#0:shape#19(#(Int, Float, String, BitArray, UtfCodepoint, custom_type#0, Bool, Nil, #(Int))), %string#0:shape#5(String), %int#0:shape#1(Int), %int#1:shape#1(Int)]
//     %string#1:shape#5(String) = string.value ""
//     jump b3(%string#1, %function.int#0, %function.float#0, %function.string#0, %function.bit_array#0, %function.utf_codepoint#0, %function.custom#0, %function.bool#0, %function.nil#0, %function.tuple#0, %utf_codepoint#0, %custom#0, %tuple#0, %string#0, %int#0, %int#1)
//   block b5 params=[%string#0:shape#5(String)]
//     let_assert_panic subject=%string#0 message=none
//
// function tuple#1
//   entry b0 params=[%tuple#0:shape#16(#(Int))] captures=[]
//   block b0 params=[%tuple#0:shape#16(#(Int))]
//     return %tuple#0
