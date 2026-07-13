pub fn main() {
  #(
    case <<"A">> {
      <<"A">> -> 1
      _ -> 0
    },
    case <<"안":utf8>> {
      <<"안":utf8>> -> 1
      _ -> 0
    },
    case <<"A":utf16-big>> {
      <<_:utf16-big>> -> 1
      _ -> 0
    },
    case <<"A":utf16-big>> {
      <<"A":utf16-big>> -> 1
      _ -> 0
    },
    case <<"A":utf16-little>> {
      <<"A":utf16-little>> -> 1
      _ -> 0
    },
    case <<"A":utf16-little>> {
      <<_:utf16-little>> -> 1
      _ -> 0
    },
    case <<"A":utf32-big>> {
      <<_:utf32-big>> -> 1
      _ -> 0
    },
    case <<"A":utf32-big>> {
      <<"A":utf32-big>> -> 1
      _ -> 0
    },
    case <<"A":utf32-little>> {
      <<"A":utf32-little>> -> 1
      _ -> 0
    },
    case <<"A":utf32-little>> {
      <<_:utf32-little>> -> 1
      _ -> 0
    },
    case <<"A":utf8>> {
      <<"B":utf8>> -> 1
      _ -> 0
    },
    case <<"A":utf8>> {
      <<"AB":utf8>> -> 1
      _ -> 0
    },
    case <<255>> {
      <<_:utf8>> -> 1
      _ -> 0
    },
  )
}

// geam:expect Tuple([Int(1), Int(1), Int(1), Int(1), Int(1), Int(1), Int(1), Int(1), Int(1), Int(1), Int(0), Int(0), Int(0)])
