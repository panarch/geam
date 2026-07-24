fn utf8(bits: BitArray) -> UtfCodepoint {
  case bits {
    <<value:utf8_codepoint>> -> value
    _ -> panic
  }
}

fn utf32(bits: BitArray) -> UtfCodepoint {
  case bits {
    <<value:utf32_codepoint-big>> -> value
    _ -> panic
  }
}

fn utf16_big(bits: BitArray) -> UtfCodepoint {
  case bits {
    <<value:utf16_codepoint-big>> -> value
    _ -> panic
  }
}

fn utf16_little(bits: BitArray) -> UtfCodepoint {
  case bits {
    <<value:utf16_codepoint-little>> -> value
    _ -> panic
  }
}

fn utf32_little(bits: BitArray) -> UtfCodepoint {
  case bits {
    <<value:utf32_codepoint-little>> -> value
    _ -> panic
  }
}

pub fn main() {
  let ascii = utf8(<<"A":utf8>>)
  let won = utf8(<<"안":utf8>>)
  let smile = utf8(<<"😀":utf8>>)
  let null = utf32(<<0, 0, 0, 0>>)
  let before_surrogate = utf32(<<0, 0, 215, 255>>)
  let after_surrogate = utf32(<<0, 0, 224, 0>>)
  let maximum = utf32(<<0, 16, 255, 255>>)

  #(
    ascii,
    <<ascii:utf8_codepoint>>,
    <<won:utf8_codepoint>>,
    <<smile:utf8_codepoint>>,
    <<smile:utf16_codepoint-big>>,
    <<smile:utf16_codepoint-little>>,
    <<smile:utf32_codepoint-big>>,
    <<smile:utf32_codepoint-little>>,
    utf16_big(<<smile:utf16_codepoint-big>>),
    utf16_little(<<smile:utf16_codepoint-little>>),
    utf32(<<smile:utf32_codepoint-big>>),
    utf32_little(<<smile:utf32_codepoint-little>>),
    <<null:utf32_codepoint-big>>,
    <<before_surrogate:utf32_codepoint-big>>,
    <<after_surrogate:utf32_codepoint-big>>,
    <<maximum:utf32_codepoint-big>>,
    case <<255>> {
      <<_:utf8_codepoint>> -> 1
      _ -> 0
    },
    case <<216, 0>> {
      <<_:utf16_codepoint-big>> -> 1
      _ -> 0
    },
    case <<0>> {
      <<_:utf16_codepoint-big>> -> 1
      _ -> 0
    },
    case <<216, 0, 0, 65>> {
      <<_:utf16_codepoint-big>> -> 1
      _ -> 0
    },
    case <<0, 0, 0>> {
      <<_:utf32_codepoint-big>> -> 1
      _ -> 0
    },
    case <<0, 0, 216, 0>> {
      <<_:utf32_codepoint-big>> -> 1
      _ -> 0
    },
    case <<0, 17, 0, 0>> {
      <<_:utf32_codepoint-big>> -> 1
      _ -> 0
    },
    case <<226, 130>> {
      <<_:utf8_codepoint>> -> 1
      _ -> 0
    },
  )
}

// @geam:expect Tuple([UtfCodepoint('A'), BitArray(bytes=[65], bit_len=8), BitArray(bytes=[236, 149, 136], bit_len=24), BitArray(bytes=[240, 159, 152, 128], bit_len=32), BitArray(bytes=[216, 61, 222, 0], bit_len=32), BitArray(bytes=[61, 216, 0, 222], bit_len=32), BitArray(bytes=[0, 1, 246, 0], bit_len=32), BitArray(bytes=[0, 246, 1, 0], bit_len=32), UtfCodepoint('😀'), UtfCodepoint('😀'), UtfCodepoint('😀'), UtfCodepoint('😀'), BitArray(bytes=[0, 0, 0, 0], bit_len=32), BitArray(bytes=[0, 0, 215, 255], bit_len=32), BitArray(bytes=[0, 0, 224, 0], bit_len=32), BitArray(bytes=[0, 16, 255, 255], bit_len=32), Int(0), Int(0), Int(0), Int(0), Int(0), Int(0), Int(0), Int(0)])
