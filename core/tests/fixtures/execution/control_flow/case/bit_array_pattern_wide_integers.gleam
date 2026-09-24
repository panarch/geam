pub fn main() {
  let input = <<
    0x92, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0, 0x13, 0x57, 0x9b, 0xdf, 0x24,
    0x68, 0xac, 0xe0, 0x80,
  >>
  #(
    case input {
      <<_:size(3), value:signed-big-size(64), _:bits>> -> value
      _ -> 0
    },
    case input {
      <<_:size(3), value:signed-little-size(64), _:bits>> -> value
      _ -> 0
    },
    case input {
      <<_:size(3), value:unsigned-big-size(65), _:bits>> -> value
      _ -> 0
    },
    case input {
      <<_:size(3), value:signed-big-size(65), _:bits>> -> value
      _ -> 0
    },
    case input {
      <<_:size(3), value:unsigned-little-size(65), _:bits>> -> value
      _ -> 0
    },
    case input {
      <<_:size(3), value:signed-little-size(65), _:bits>> -> value
      _ -> 0
    },
    case input {
      <<_:size(3), value:signed-big-size(128), _:bits>> -> value
      _ -> 0
    },
    case input {
      <<_:size(3), value:signed-little-size(128), _:bits>> -> value
      _ -> 0
    },
    case input {
      <<_:size(3), value:signed-big-size(129), _:bits>> -> value
      _ -> 0
    },
    case input {
      <<_:size(3), value:unsigned-little-size(129), _:bits>> -> value
      _ -> 0
    },
  )
}
// @geam:expect Tuple([Int(-7952596333999229056), Int(-9153593911804714351), Int(20988295479420645121), Int(-15905192667998458111), Int(27739894235614388881), Int(-9153593911804714351), Int(-146699509294804584544372181868356475132), Int(5853120896148130334324824293139260049), Int(-293399018589609169088744363736712950264), Int(5853120896148130334324824293139260049)])
