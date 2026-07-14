fn codepoint(value: Int) -> UtfCodepoint {
  case <<value>> {
    <<value:utf8_codepoint>> -> value
    _ -> panic
  }
}

fn bits(value: UtfCodepoint) -> BitArray {
  <<value:utf8_codepoint>>
}

fn direct(value: Int) -> UtfCodepoint {
  codepoint(value)
}

fn choose_bool(value: Bool) -> UtfCodepoint {
  case value {
    True -> codepoint(3)
    False -> codepoint(4)
  }
}

fn choose_int(value: Int) -> UtfCodepoint {
  case value {
    1 -> codepoint(5)
    _ -> codepoint(6)
  }
}

fn choose_string(value: String) -> UtfCodepoint {
  case value {
    "hit" -> codepoint(7)
    _ -> codepoint(8)
  }
}

fn choose_float(value: Float) -> UtfCodepoint {
  case value {
    1.0 -> codepoint(9)
    _ -> codepoint(10)
  }
}

pub fn main() {
  let local = codepoint(1)
  let function = direct
  let pair = #(codepoint(11))
  let assert [from_list] = [codepoint(12)]
  let from_bool_case = case True {
    True -> codepoint(14)
    False -> codepoint(15)
  }
  let from_bool_case_fallback = case False {
    True -> codepoint(14)
    False -> codepoint(15)
  }
  let from_int_case = case 1 {
    1 -> codepoint(16)
    _ -> codepoint(17)
  }
  let from_int_case_fallback = case 0 {
    1 -> codepoint(16)
    _ -> codepoint(17)
  }
  let from_string_case = case "hit" {
    "hit" -> codepoint(18)
    _ -> codepoint(19)
  }
  let from_string_case_fallback = case "miss" {
    "hit" -> codepoint(18)
    _ -> codepoint(19)
  }
  let from_float_case = case 1.0 {
    1.0 -> codepoint(20)
    _ -> codepoint(21)
  }
  let from_float_case_fallback = case 0.0 {
    1.0 -> codepoint(20)
    _ -> codepoint(21)
  }
  let from_list_case = case [codepoint(22)] {
    [value] -> value
    _ -> panic
  }
  let from_block = {
    let ignored = 1
    codepoint(13)
  }

  #(
    bits(local),
    bits(direct(2)),
    bits(function(3)),
    bits(pair.0),
    bits(from_list),
    bits(choose_bool(True)),
    bits(choose_bool(False)),
    bits(choose_int(1)),
    bits(choose_int(0)),
    bits(choose_string("hit")),
    bits(choose_string("miss")),
    bits(choose_float(1.0)),
    bits(choose_float(0.0)),
    bits(from_bool_case),
    bits(from_bool_case_fallback),
    bits(from_int_case),
    bits(from_int_case_fallback),
    bits(from_string_case),
    bits(from_string_case_fallback),
    bits(from_float_case),
    bits(from_float_case_fallback),
    bits(from_list_case),
    bits(from_block),
  )
}

// geam:expect Tuple([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[2], bit_len=8), BitArray(bytes=[3], bit_len=8), BitArray(bytes=[11], bit_len=8), BitArray(bytes=[12], bit_len=8), BitArray(bytes=[3], bit_len=8), BitArray(bytes=[4], bit_len=8), BitArray(bytes=[5], bit_len=8), BitArray(bytes=[6], bit_len=8), BitArray(bytes=[7], bit_len=8), BitArray(bytes=[8], bit_len=8), BitArray(bytes=[9], bit_len=8), BitArray(bytes=[10], bit_len=8), BitArray(bytes=[14], bit_len=8), BitArray(bytes=[15], bit_len=8), BitArray(bytes=[16], bit_len=8), BitArray(bytes=[17], bit_len=8), BitArray(bytes=[18], bit_len=8), BitArray(bytes=[19], bit_len=8), BitArray(bytes=[20], bit_len=8), BitArray(bytes=[21], bit_len=8), BitArray(bytes=[22], bit_len=8), BitArray(bytes=[13], bit_len=8)])
