pub type Marker {
  Marker(Int)
}

fn add_one(value: Int) { value + 1 }

const int_value = 1
const float_value = 1.5
const string_value = "one"
const bit_array_value = <<1>>
const custom_value = Marker(1)
const bool_value = True
const nil_value = Nil
const tuple_value = #(1)
const int_list = [1]
const string_list = ["one"]
const bit_array_list = [<<1>>]
const custom_list = [Marker(1)]
const float_list = [1.5]
const bool_list = [True]
const nil_list = [Nil]
const tuple_list = [#(1)]
const list_list = [[1]]
const function_list = [add_one]
const function_value = add_one

pub fn main() {
  #(
    int_value,
    float_value,
    string_value,
    bit_array_value,
    custom_value,
    bool_value,
    nil_value,
    tuple_value,
    int_list,
    string_list,
    bit_array_list,
    custom_list,
    float_list,
    bool_list,
    nil_list,
    tuple_list,
    list_list,
    function_list,
    function_value,
  )
}


// geam:explain
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
// function tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = constant.int#0
//     %float#0:shape#1(Float) = constant.float#0
//     %string#0:shape#2(String) = constant.string#0
//     %bit_array#0:shape#3(BitArray) = constant.bit_array#0
//     %custom#0:shape#11(custom_type#0) = constant.custom#0
//     %bool#0:shape#5(Bool) = constant.bool#0
//     %nil#0:shape#6(Nil) = constant.nil#0
//     %tuple#0:shape#7(#(Int)) = constant.tuple#0
//     %list.int#0:shape#8(list_type#0) = list.int[type#0] constant.list.int#0
//     %list.string#0:shape#9(list_type#1) = list.string[type#1] constant.list.string#0
//     %list.bit_array#0:shape#10(list_type#2) = list.bit_array[type#2] constant.list.bit_array#0
//     %list.custom#0:shape#12(list_type#3) = list.custom[type#3] constant.list.custom#0
//     %list.float#0:shape#13(list_type#4) = list.float[type#4] constant.list.float#0
//     %list.bool#0:shape#14(list_type#5) = list.bool[type#5] constant.list.bool#0
//     %list.nil#0:shape#15(list_type#6) = list.nil[type#6] constant.list.nil#0
//     %list.tuple#0:shape#16(list_type#7) = list.tuple[type#7] constant.list.tuple#0
//     %list.list#0:shape#17(list_type#8) = list.list[type#8] constant.list.list#0
//     %list.function#0:shape#19(list_type#9) = list.function[type#9] constant.list.function#0
//     %function.int#0:shape#18(fn(Int) -> Int) = function[Int] constant.function#0
//     %tuple#1:shape#20(#(Int, Float, String, BitArray, custom_type#0, Bool, Nil, #(Int), list_type#0, list_type#1, list_type#2, list_type#3, list_type#4, list_type#5, list_type#6, list_type#7, list_type#8, list_type#9, fn(Int) -> Int)) = tuple.value elements=[%int#0, %float#0, %string#0, %bit_array#0, %custom#0, %bool#0, %nil#0, %tuple#0, %list.int#0, %list.string#0, %list.bit_array#0, %list.custom#0, %list.float#0, %list.bool#0, %list.nil#0, %list.tuple#0, %list.list#0, %list.function#0, %function.int#0]
//     return %tuple#1
//
// constant.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     return %int#0
//
// constant.float#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %float#0:shape#1(Float) = float.value 1.5
//     return %float#0
//
// constant.string#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %string#0:shape#2(String) = string.value "one"
//     return %string#0
//
// constant.bit_array#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %bit_array#0:shape#3(BitArray) = bit_array.value [int(%int#0, bits=8, big)]
//     return %bit_array#0
//
// constant.custom#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %custom#0:shape#4(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#0]
//     return %custom#0
//
// constant.bool#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %bool#0:shape#5(Bool) = bool.value True
//     return %bool#0
//
// constant.nil#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %nil#0:shape#6(Nil) = nil.value
//     return %nil#0
//
// constant.tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %tuple#0:shape#7(#(Int)) = tuple.value elements=[%int#0]
//     return %tuple#0
//
// constant.list.int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %list.int#0:shape#8(list_type#0) = list.int[type#0] value elements=[%int#0]
//     return %list.int#0
//
// constant.list.string#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %string#0:shape#2(String) = string.value "one"
//     %list.string#0:shape#9(list_type#1) = list.string[type#1] value elements=[%string#0]
//     return %list.string#0
//
// constant.list.bit_array#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %bit_array#0:shape#3(BitArray) = bit_array.value [int(%int#0, bits=8, big)]
//     %list.bit_array#0:shape#10(list_type#2) = list.bit_array[type#2] value elements=[%bit_array#0]
//     return %list.bit_array#0
//
// constant.list.custom#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %custom#0:shape#4(custom_type#0) = custom.construct custom_type#0.constructor#0 fields=[%int#0]
//     %list.custom#0:shape#12(list_type#3) = list.custom[type#3] value elements=[%custom#0]
//     return %list.custom#0
//
// constant.list.float#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %float#0:shape#1(Float) = float.value 1.5
//     %list.float#0:shape#13(list_type#4) = list.float[type#4] value elements=[%float#0]
//     return %list.float#0
//
// constant.list.bool#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %bool#0:shape#5(Bool) = bool.value True
//     %list.bool#0:shape#14(list_type#5) = list.bool[type#5] value elements=[%bool#0]
//     return %list.bool#0
//
// constant.list.nil#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %nil#0:shape#6(Nil) = nil.value
//     %list.nil#0:shape#15(list_type#6) = list.nil[type#6] value elements=[%nil#0]
//     return %list.nil#0
//
// constant.list.tuple#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %tuple#0:shape#7(#(Int)) = tuple.value elements=[%int#0]
//     %list.tuple#0:shape#16(list_type#7) = list.tuple[type#7] value elements=[%tuple#0]
//     return %list.tuple#0
//
// constant.list.list#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %int#0:shape#0(Int) = int.value 1
//     %list.int#0:shape#8(list_type#0) = list.int[type#0] value elements=[%int#0]
//     %list.list#0:shape#17(list_type#8) = list.list[type#8] value elements=[%list.int#0]
//     return %list.list#0
//
// constant.list.function#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.int#0:shape#18(fn(Int) -> Int) = function[Int] reference int#0
//     %list.function#0:shape#19(list_type#9) = list.function[type#9] value elements=[%function.int#0]
//     return %list.function#0
//
// constant.function#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %function.int#0:shape#18(fn(Int) -> Int) = function[Int] reference int#0
//     return %function.int#0
