import raw_string_values_fixture

pub fn main() {
  raw_string_values_fixture.main()
}

pub fn strip_prefix(value: String) -> String {
  raw_string_values_fixture.strip_prefix(value)
}

pub fn count_prefix(value: String, count: Int) -> Int {
  raw_string_values_fixture.count_prefix(value, count)
}

pub fn utf16_encode(text: String) -> BitArray {
  <<text:utf16>>
}

pub fn utf32_encode(text: String) -> BitArray {
  <<text:utf32>>
}
