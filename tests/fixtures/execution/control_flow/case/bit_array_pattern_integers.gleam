const base_pattern_size = 8
const pattern_size = base_pattern_size

pub fn main() {
  let outer_size = 12
  let negative_size = -1
  let zero_size = 0
  let huge_size = 184467440737095516160

  #(
    case <<-2:size(12)>> {
      <<value:signed-size(12)>> -> value
      _ -> 0
    },
    case <<-2:size(12)>> {
      <<value:unsigned-size(12)>> -> value
      _ -> 0
    },
    case <<0x234:little-size(12)>> {
      <<value:little-size(12)>> -> value
      _ -> 0
    },
    case <<0x234:size(12)>> {
      <<value:size(outer_size)>> -> value
      _ -> 0
    },
    case <<12, 0x234:size(12)>> {
      <<size, value:size(size)>> -> value
      _ -> 0
    },
    case <<>> {
      <<_:bits-size(negative_size)>> -> 1
      _ -> 0
    },
    case <<>> {
      <<_:bits-size(huge_size)>> -> 1
      _ -> 0
    },
    case <<1>> {
      <<_:bits-size(16)>> -> 1
      _ -> 0
    },
    case <<1>> {
      <<_:size(16)>> -> 1
      _ -> 0
    },
    case <<>> {
      <<value:size(zero_size)>> if value == 0 -> 1
      _ -> 0
    },
    case <<1, 2, 3, 4, 5>> {
      <<
        one:size(pattern_size),
        two:size(outer_size - 4),
        three:size(outer_size * 2 / 3),
        four:size(outer_size % 5 + 6),
        five:size({ outer_size - 4 }),
      >> -> one + two + three + four + five
      _ -> 0
    },
  )
}

// @geam:expect Tuple([Int(-2), Int(4094), Int(564), Int(564), Int(564), Int(0), Int(0), Int(0), Int(0), Int(0), Int(15)])
