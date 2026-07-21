pub type Payload {
  Payload(Int, String, List(Int))
}

pub fn main() {
  let assert True = True
  let assert False = False
  let assert Nil = Nil
  let assert #(
    1 as one,
    1.5,
    "ready",
    _,
    [first, ..],
    [second, ..rest],
    Payload(third, "pre" as prefix <> suffix, [fourth]),
  ) as whole = #(
    1,
    1.5,
    "ready",
    False,
    [2, 3],
    [4, 5],
    Payload(6, "prefix", [7]),
  )

  let outer = 8
  let assert <<
    -2 as signed:signed-size(12),
    unsigned:unsigned-little-size(12),
    16,
    dynamic:size(outer),
    _:unsigned-size(8),
    bound_size,
    bound_value:size(bound_size),
    1.5 as float_alias:float-size(16)-big,
    float_value:float-size(32)-little,
    _:float-size(64)-big,
    "A":utf8,
    _:utf16-big,
    "B":utf16-little,
    _:utf32-big,
    "C":utf32-little,
    codepoint:utf8_codepoint,
    _ as codepoint_alias:utf16_codepoint-little,
    _:utf32_codepoint-big,
    fixed:bits-size(8),
    _:bits-size(outer),
    _ as bits_alias:bits-size(outer + 0),
    _:bits-size(outer - 0),
    _:bits-size(outer * 1),
    _:bits-size(outer / 1),
    _:bits-size(outer % 5),
    remaining:bits,
  >> as all = <<
    -2:size(12),
    0x234:little-size(12),
    16,
    1,
    9,
    8,
    2,
    1.5:float-size(16)-big,
    1.5:float-size(32)-little,
    1.5:float-size(64)-big,
    "A":utf8,
    "A":utf16-big,
    "B":utf16-little,
    "A":utf32-big,
    "C":utf32-little,
    "D":utf8,
    "E":utf16-little,
    "F":utf32-big,
    1,
    2,
    3,
    5,
    6,
    7,
    8,
    4,
  >>

  #(
    one,
    first,
    second,
    rest,
    third,
    prefix,
    suffix,
    fourth,
    whole,
    signed,
    unsigned,
    dynamic,
    bound_value,
    float_alias,
    float_value,
    codepoint,
    codepoint_alias,
    fixed,
    bits_alias,
    remaining,
    all,
  )
}



// geam:run

// geam:explain
// module main
// main tuple#0
//
// function tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %bool#0:shape#0(Bool) = bool.value True
//     match %bool#0 pattern=True success=b1() failure=b10(%bool#0)
//   block b1 params=[]
//     %bool#0:shape#0(Bool) = bool.value False
//     match %bool#0 pattern=False success=b2() failure=b9(%bool#0)
//   block b2 params=[]
//     %nil#0:shape#1(Nil) = nil.value
//     match %nil#0 pattern=Nil success=b3() failure=b8(%nil#0)
//   block b3 params=[]
//     %int#0:shape#2(Int) = int.value 1
//     %float#0:shape#3(Float) = float.value 1.5
//     %string#0:shape#4(String) = string.value "ready"
//     %bool#0:shape#0(Bool) = bool.value False
//     %int#1:shape#2(Int) = int.value 2
//     %int#2:shape#2(Int) = int.value 3
//     %list.int#0:shape#5(list_type#0) = list.int[type#0] value elements=[%int#1, %int#2]
//     %int#3:shape#2(Int) = int.value 4
//     %int#4:shape#2(Int) = int.value 5
//     %list.int#1:shape#5(list_type#0) = list.int[type#0] value elements=[%int#3, %int#4]
//     %int#5:shape#2(Int) = int.value 6
//     %string#1:shape#4(String) = string.value "prefix"
//     %int#6:shape#2(Int) = int.value 7
//     %list.int#2:shape#5(list_type#0) = list.int[type#0] value elements=[%int#6]
//     %custom#0:shape#6(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#5, %string#1, %list.int#2]
//     %tuple#0:shape#7(#(Int, Float, String, Bool, list_type#0, list_type#0, custom_type#0)) = tuple.value elements=[%int#0, %float#0, %string#0, %bool#0, %list.int#0, %list.int#1, %custom#0]
//     match %tuple#0 pattern=alias(#(alias(1, binding#0), 1.5, "ready", _, [binding#1, .._], [binding#2, ..binding#3], custom_type#0.constructor#0(binding#4, string_prefix("pre", left=binding#5, right=binding#6), [binding#7])), binding#8) success=b4(binding#0, binding#1, binding#2, binding#3, binding#4, binding#5, binding#6, binding#7, binding#8) failure=b7(%tuple#0)
//   block b4 params=[%int#0:shape#2(Int), %int#1:shape#2(Int), %int#2:shape#2(Int), %list.int#0:shape#5(list_type#0), %int#3:shape#2(Int), %string#0:shape#4(String), %string#1:shape#4(String), %int#4:shape#2(Int), %tuple#0:shape#7(#(Int, Float, String, Bool, list_type#0, list_type#0, custom_type#0))]
//     %int#5:shape#2(Int) = int.value 8
//     %int#6:shape#2(Int) = int.value -2
//     %int#7:shape#2(Int) = int.value 564
//     %int#8:shape#2(Int) = int.value 16
//     %int#9:shape#2(Int) = int.value 1
//     %int#10:shape#2(Int) = int.value 9
//     %int#11:shape#2(Int) = int.value 8
//     %int#12:shape#2(Int) = int.value 2
//     %float#0:shape#3(Float) = float.value 1.5
//     %float#1:shape#3(Float) = float.value 1.5
//     %float#2:shape#3(Float) = float.value 1.5
//     %string#2:shape#4(String) = string.value "A"
//     %string#3:shape#4(String) = string.value "A"
//     %string#4:shape#4(String) = string.value "B"
//     %string#5:shape#4(String) = string.value "A"
//     %string#6:shape#4(String) = string.value "C"
//     %string#7:shape#4(String) = string.value "D"
//     %string#8:shape#4(String) = string.value "E"
//     %string#9:shape#4(String) = string.value "F"
//     %int#13:shape#2(Int) = int.value 1
//     %int#14:shape#2(Int) = int.value 2
//     %int#15:shape#2(Int) = int.value 3
//     %int#16:shape#2(Int) = int.value 5
//     %int#17:shape#2(Int) = int.value 6
//     %int#18:shape#2(Int) = int.value 7
//     %int#19:shape#2(Int) = int.value 8
//     %int#20:shape#2(Int) = int.value 4
//     %bit_array#0:shape#8(BitArray) = bit_array.value [int(%int#6, bits=12, big), int(%int#7, bits=12, little), int(%int#8, bits=8, big), int(%int#9, bits=8, big), int(%int#10, bits=8, big), int(%int#11, bits=8, big), int(%int#12, bits=8, big), float(%float#0, bits=16, big), float(%float#1, bits=32, little), float(%float#2, bits=64, big), string(%string#2, utf8), string(%string#3, utf16.big), string(%string#4, utf16.little), string(%string#5, utf32.big), string(%string#6, utf32.little), string(%string#7, utf8), string(%string#8, utf16.little), string(%string#9, utf32.big), int(%int#13, bits=8, big), int(%int#14, bits=8, big), int(%int#15, bits=8, big), int(%int#16, bits=8, big), int(%int#17, bits=8, big), int(%int#18, bits=8, big), int(%int#19, bits=8, big), int(%int#20, bits=8, big)]
//     match %bit_array#0 pattern=alias(<<int(alias(-2, binding#0), size=12*1, big, signed), int(binding#1, size=12*1, little, unsigned), int(16, size=8*1, big, unsigned), int(binding#2, size=%int#5*1, big, unsigned), int(_, size=8*1, big, unsigned), int(binding#3, size=8*1, big, unsigned), int(binding#4, size=binding#3*1, big, unsigned), float(alias(1.5, binding#5), size=16*1, big), float(binding#6, size=32*1, little), float(_, size=64*1, big), string("A", utf8), string(_, utf16.big), string("B", utf16.little), string(_, utf32.big), string("C", utf32.little), utf_codepoint(binding#7, utf8), utf_codepoint(alias(_, binding#8), utf16.little), utf_codepoint(_, utf32.big), bits(binding#9, size=8*1, unit=1), bits(_, size=%int#5*1, unit=1), bits(alias(_, binding#10), size=(%int#5 + 0)*1, unit=1), bits(_, size=(%int#5 - 0)*1, unit=1), bits(_, size=(%int#5 * 1)*1, unit=1), bits(_, size=(%int#5 / 1)*1, unit=1), bits(_, size=(%int#5 % 5)*1, unit=1), bits(binding#11, size=rest, unit=1)>>, binding#12) success=b5(binding#0, binding#1, binding#2, binding#4, binding#5, binding#6, binding#7, binding#8, binding#9, binding#10, binding#11, binding#12, %int#0, %int#1, %int#2, %list.int#0, %int#3, %string#0, %string#1, %int#4, %tuple#0) failure=b6(%bit_array#0)
//   block b5 params=[%int#0:shape#2(Int), %int#1:shape#2(Int), %int#2:shape#2(Int), %int#3:shape#2(Int), %float#0:shape#3(Float), %float#1:shape#3(Float), %utf_codepoint#0:shape#9(UtfCodepoint), %utf_codepoint#1:shape#9(UtfCodepoint), %bit_array#0:shape#8(BitArray), %bit_array#1:shape#8(BitArray), %bit_array#2:shape#8(BitArray), %bit_array#3:shape#8(BitArray), %int#4:shape#2(Int), %int#5:shape#2(Int), %int#6:shape#2(Int), %list.int#0:shape#5(list_type#0), %int#7:shape#2(Int), %string#0:shape#4(String), %string#1:shape#4(String), %int#8:shape#2(Int), %tuple#0:shape#7(#(Int, Float, String, Bool, list_type#0, list_type#0, custom_type#0))]
//     %tuple#1:shape#10(#(Int, Int, Int, list_type#0, Int, String, String, Int, #(Int, Float, String, Bool, list_type#0, list_type#0, custom_type#0), Int, Int, Int, Int, Float, Float, UtfCodepoint, UtfCodepoint, BitArray, BitArray, BitArray, BitArray)) = tuple.value elements=[%int#4, %int#5, %int#6, %list.int#0, %int#7, %string#0, %string#1, %int#8, %tuple#0, %int#0, %int#1, %int#2, %int#3, %float#0, %float#1, %utf_codepoint#0, %utf_codepoint#1, %bit_array#0, %bit_array#1, %bit_array#2, %bit_array#3]
//     return %tuple#1
//   block b6 params=[%bit_array#0:shape#8(BitArray)]
//     let_assert_panic subject=%bit_array#0 message=none
//   block b7 params=[%tuple#0:shape#7(#(Int, Float, String, Bool, list_type#0, list_type#0, custom_type#0))]
//     let_assert_panic subject=%tuple#0 message=none
//   block b8 params=[%nil#0:shape#1(Nil)]
//     let_assert_panic subject=%nil#0 message=none
//   block b9 params=[%bool#0:shape#0(Bool)]
//     let_assert_panic subject=%bool#0 message=none
//   block b10 params=[%bool#0:shape#0(Bool)]
//     let_assert_panic subject=%bool#0 message=none
