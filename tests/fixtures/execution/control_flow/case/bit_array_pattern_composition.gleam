pub fn main() {
  let closure = case <<2, 9>> {
    <<1, value>> | <<2, value>> if value > 5 -> fn() { value }
    _ -> fn() { 0 }
  }
  let outer_size = 8
  let outer_size_closure = fn(bits) {
    case bits {
      <<value:size(outer_size)>> -> value
      _ -> 0
    }
  }
  let segment_size_closure = fn(bits) {
    case bits {
      <<size, value:size(size)>> -> value
      _ -> 0
    }
  }

  #(
    case <<1>> {
      <<1 as segment>> as whole -> #(segment, whole)
      _ -> #(0, <<>>)
    },
    case <<1>>, <<2>> {
      <<1>>, <<value>> -> value
      _, _ -> 0
    },
    case <<1>> {
      <<2 as value>> -> value
      _ -> 0
    },
    closure(),
    outer_size_closure(<<42>>),
    segment_size_closure(<<8, 43>>),
  )
}

// geam:expect Tuple([Tuple([Int(1), BitArray(bytes=[1], bit_len=8)]), Int(2), Int(0), Int(9), Int(42), Int(43)])
