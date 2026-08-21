fn codepoint(value: Int) -> UtfCodepoint {
  case <<value>> {
    <<value:utf8_codepoint>> -> value
    _ -> panic
  }
}

fn bits(value: UtfCodepoint) -> BitArray {
  <<value:utf8_codepoint>>
}

fn identity(value: UtfCodepoint) -> UtfCodepoint {
  value
}

fn other(_: UtfCodepoint) -> UtfCodepoint {
  codepoint(99)
}

fn getter(_: Nil) -> fn(UtfCodepoint) -> UtfCodepoint {
  identity
}

fn selector() -> fn(Nil) -> fn(UtfCodepoint) -> UtfCodepoint {
  getter
}

fn apply(function: fn(UtfCodepoint) -> UtfCodepoint, value: UtfCodepoint) {
  function(value)
}

fn first(values: List(UtfCodepoint)) -> UtfCodepoint {
  let assert [value, ..] = values
  value
}

fn choose_bool(value: Bool) {
  case value {
    True -> identity
    False -> other
  }
}

fn choose_int(value: Int) {
  case value {
    1 -> identity
    _ -> other
  }
}

fn choose_string(value: String) {
  case value {
    "hit" -> identity
    _ -> other
  }
}

fn choose_float(value: Float) {
  case value {
    1.0 -> identity
    _ -> other
  }
}

pub fn main() {
  let local = identity
  let closure = fn(value) { value }
  let pair = #(identity)
  let assert [from_list] = [identity]
  let from_list_case = case [identity] {
    [function] -> function
    _ -> other
  }
  let from_direct_call = getter(Nil)
  let selected = selector()
  let from_function_call = selected(Nil)
  let from_function_subject = case identity {
    function -> function
  }
  let from_guarded_function_subject = case identity {
    function if True -> function
    _ -> other
  }
  let from_bool_case = case True {
    True -> identity
    False -> other
  }
  let from_bool_case_fallback = case False {
    True -> identity
    False -> other
  }
  let from_int_case = case 1 {
    1 -> identity
    _ -> other
  }
  let from_int_case_fallback = case 0 {
    1 -> identity
    _ -> other
  }
  let from_string_case = case "hit" {
    "hit" -> identity
    _ -> other
  }
  let from_string_case_fallback = case "miss" {
    "hit" -> identity
    _ -> other
  }
  let from_float_case = case 1.0 {
    1.0 -> identity
    _ -> other
  }
  let from_float_case_fallback = case 0.0 {
    1.0 -> identity
    _ -> other
  }
  let from_block = {
    let ignored = 1
    identity
  }
  let captured_function = identity
  let capturing_closure = fn(value) { captured_function(value) }

  #(
    bits(local(codepoint(1))),
    bits(closure(codepoint(2))),
    bits(pair.0(codepoint(3))),
    bits(from_list(codepoint(4))),
    bits(from_list_case(codepoint(24))),
    bits(from_direct_call(codepoint(5))),
    bits(from_function_call(codepoint(6))),
    bits(from_function_subject(codepoint(23))),
    bits(from_guarded_function_subject(codepoint(25))),
    bits(choose_bool(True)(codepoint(7))),
    bits(choose_bool(False)(codepoint(8))),
    bits(choose_int(1)(codepoint(9))),
    bits(choose_int(0)(codepoint(10))),
    bits(choose_string("hit")(codepoint(11))),
    bits(choose_string("miss")(codepoint(12))),
    bits(choose_float(1.0)(codepoint(13))),
    bits(choose_float(0.0)(codepoint(14))),
    bits(from_bool_case(codepoint(16))),
    bits(from_bool_case_fallback(codepoint(16))),
    bits(from_int_case(codepoint(17))),
    bits(from_int_case_fallback(codepoint(17))),
    bits(from_string_case(codepoint(18))),
    bits(from_string_case_fallback(codepoint(18))),
    bits(from_float_case(codepoint(19))),
    bits(from_float_case_fallback(codepoint(19))),
    bits(from_block(codepoint(15))),
    bits(apply(identity, codepoint(20))),
    bits(first([codepoint(21)])),
    bits(capturing_closure(codepoint(22))),
  )
}

// @geam:expect Tuple([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[2], bit_len=8), BitArray(bytes=[3], bit_len=8), BitArray(bytes=[4], bit_len=8), BitArray(bytes=[24], bit_len=8), BitArray(bytes=[5], bit_len=8), BitArray(bytes=[6], bit_len=8), BitArray(bytes=[23], bit_len=8), BitArray(bytes=[25], bit_len=8), BitArray(bytes=[7], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[9], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[11], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[13], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[16], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[17], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[18], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[19], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[15], bit_len=8), BitArray(bytes=[20], bit_len=8), BitArray(bytes=[21], bit_len=8), BitArray(bytes=[22], bit_len=8)])
