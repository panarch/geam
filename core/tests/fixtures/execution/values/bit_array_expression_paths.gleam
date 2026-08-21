fn direct(value: Int) -> BitArray {
  <<value>>
}

fn choose_bool(value: Bool) -> BitArray {
  case value {
    True -> <<3>>
    False -> <<4>>
  }
}

fn choose_int(value: Int) -> BitArray {
  case value {
    1 -> <<5>>
    _ -> <<6>>
  }
}

fn choose_string(value: String) -> BitArray {
  case value {
    "hit" -> <<7>>
    _ -> <<8>>
  }
}

fn choose_float(value: Float) -> BitArray {
  case value {
    1.0 -> <<9>>
    _ -> <<10>>
  }
}

pub fn main() {
  let local = <<1>>
  let function = direct
  let pair = #(<<11>>)
  let assert [from_list] = [<<12>>]
  let from_bool_case = case True {
    True -> <<14>>
    False -> <<15>>
  }
  let from_bool_case_fallback = case False {
    True -> <<14>>
    False -> <<15>>
  }
  let from_int_case = case 1 {
    1 -> <<16>>
    _ -> <<17>>
  }
  let from_int_case_fallback = case 0 {
    1 -> <<16>>
    _ -> <<17>>
  }
  let from_string_case = case "hit" {
    "hit" -> <<18>>
    _ -> <<19>>
  }
  let from_string_case_fallback = case "miss" {
    "hit" -> <<18>>
    _ -> <<19>>
  }
  let from_float_case = case 1.0 {
    1.0 -> <<20>>
    _ -> <<21>>
  }
  let from_float_case_fallback = case 0.0 {
    1.0 -> <<20>>
    _ -> <<21>>
  }
  let from_block = {
    let ignored = 1
    <<13>>
  }

  #(
    local,
    direct(2),
    function(3),
    pair.0,
    from_list,
    choose_bool(True),
    choose_bool(False),
    choose_int(1),
    choose_int(0),
    choose_string("hit"),
    choose_string("miss"),
    choose_float(1.0),
    choose_float(0.0),
    from_bool_case,
    from_bool_case_fallback,
    from_int_case,
    from_int_case_fallback,
    from_string_case,
    from_string_case_fallback,
    from_float_case,
    from_float_case_fallback,
    from_block,
  )
}

// @geam:expect Tuple([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[2], bit_len=8), BitArray(bytes=[3], bit_len=8), BitArray(bytes=[11], bit_len=8), BitArray(bytes=[12], bit_len=8), BitArray(bytes=[3], bit_len=8), BitArray(bytes=[4], bit_len=8), BitArray(bytes=[5], bit_len=8), BitArray(bytes=[6], bit_len=8), BitArray(bytes=[7], bit_len=8), BitArray(bytes=[8], bit_len=8), BitArray(bytes=[9], bit_len=8), BitArray(bytes=[10], bit_len=8), BitArray(bytes=[14], bit_len=8), BitArray(bytes=[15], bit_len=8), BitArray(bytes=[16], bit_len=8), BitArray(bytes=[17], bit_len=8), BitArray(bytes=[18], bit_len=8), BitArray(bytes=[19], bit_len=8), BitArray(bytes=[20], bit_len=8), BitArray(bytes=[21], bit_len=8), BitArray(bytes=[13], bit_len=8)])
