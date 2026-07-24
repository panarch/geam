fn identity(value: BitArray) -> BitArray {
  value
}

fn other(_: BitArray) -> BitArray {
  <<99>>
}

fn getter(_: Nil) -> fn(BitArray) -> BitArray {
  identity
}

fn selector() -> fn(Nil) -> fn(BitArray) -> BitArray {
  getter
}

fn apply(function: fn(BitArray) -> BitArray, value: BitArray) -> BitArray {
  function(value)
}

fn first(values: List(BitArray)) -> BitArray {
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
    local(<<1>>),
    closure(<<2>>),
    pair.0(<<3>>),
    from_list(<<4>>),
    from_list_case(<<24>>),
    from_direct_call(<<5>>),
    from_function_call(<<6>>),
    from_function_subject(<<23>>),
    choose_bool(True)(<<7>>),
    choose_bool(False)(<<8>>),
    choose_int(1)(<<9>>),
    choose_int(0)(<<10>>),
    choose_string("hit")(<<11>>),
    choose_string("miss")(<<12>>),
    choose_float(1.0)(<<13>>),
    choose_float(0.0)(<<14>>),
    from_bool_case(<<16>>),
    from_bool_case_fallback(<<16>>),
    from_int_case(<<17>>),
    from_int_case_fallback(<<17>>),
    from_string_case(<<18>>),
    from_string_case_fallback(<<18>>),
    from_float_case(<<19>>),
    from_float_case_fallback(<<19>>),
    from_block(<<15>>),
    apply(identity, <<20>>),
    first([<<21>>]),
    capturing_closure(<<22>>),
  )
}

// @geam:expect Tuple([BitArray(bytes=[1], bit_len=8), BitArray(bytes=[2], bit_len=8), BitArray(bytes=[3], bit_len=8), BitArray(bytes=[4], bit_len=8), BitArray(bytes=[24], bit_len=8), BitArray(bytes=[5], bit_len=8), BitArray(bytes=[6], bit_len=8), BitArray(bytes=[23], bit_len=8), BitArray(bytes=[7], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[9], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[11], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[13], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[16], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[17], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[18], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[19], bit_len=8), BitArray(bytes=[99], bit_len=8), BitArray(bytes=[15], bit_len=8), BitArray(bytes=[20], bit_len=8), BitArray(bytes=[21], bit_len=8), BitArray(bytes=[22], bit_len=8)])
