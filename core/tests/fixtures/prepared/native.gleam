pub type Tree(a) { Leaf(a) Branch(List(Tree(a))) }

@external(erlang, "native", "equal_native")
fn equal_native(value: a, target: b) -> Bool

@external(erlang, "native", "fold")
fn fold(callback: fn(Int) -> Int, initial: Int) -> Int

@external(erlang, "native", "keep_bits")
fn keep_bits(value: BitArray) -> BitArray

@external(erlang, "native", "success")
fn success(value: a) -> Result(a, String)

@external(erlang, "native", "failure")
fn failure(callback: fn() -> a) -> Result(a, String)

fn rebuild(result: Result(a, String)) -> Result(a, String) {
  case result {
    Ok(value) -> Ok(value)
    Error(reason) -> Error(reason)
  }
}

pub fn generic_results() {
  let assert Ok(5) = rebuild(success(5))
  let assert Error("caught") = rebuild(failure(fn() { 5 }))
  let assert Error("caught") = rebuild(failure(fn() { panic }))
  let assert Error("caught") = rebuild(failure(fn() -> Int { panic }))
  True
}

pub fn run() {
  let assert True = integer_comparisons()
  let source = Branch([Leaf(<<"one":utf8>>), Branch([Leaf(<<"two":utf8>>)])])
  let expected = Branch([Leaf("one"), Branch([Leaf("two")])])
  #(
    equal_native(source, expected),
    equal_native(#(<<"one":utf8>>, [<<"two":utf8>>]), #("one", ["two"])),
    fold(fn(value) { value + 1 }, 40),
  )
}

pub fn substring(value: String) {
  let assert "prefix:" <> rest = value
  let read = fn() { rest }
  #(equal_native(#(rest, [rest]), #(read(), [read()])), read())
}

pub fn bit_range(value: BitArray, start: Int, size: Int) {
  case value {
    <<selected:bits-size(size), _:bits>> if start == 0 -> keep_bits(selected)
    <<_:bits-size(start), selected:bits-size(size), _:bits>> -> {
      let read = fn() { selected }
      keep_bits(read())
    }
    _ -> <<>>
  }
}

pub fn bit_tail(value: BitArray) {
  let assert <<_:8, rest:bits>> = value
  keep_bits(rest)
}

fn compare(left, right) {
  #(left == right, left != right)
}

fn integer_comparisons() {
  let minimum = -9223372036854775808
  let maximum = 9223372036854775807
  let assert 9223372036854775808 = maximum + 1
  let assert -9223372036854775809 = minimum - 1
  let assert 9223372036854775808 = -9223372036854775808 / -1
  let assert 0 = minimum % -1
  let assert -2 = -7 / 3
  let assert -1 = -7 % 3
  let assert 0 = minimum / 0
  let assert 0 = maximum % 0
  let assert 85070591730234615865843651857942052864 = minimum * minimum
  let assert True = -9223372036854775809 < minimum
  let assert True = maximum < 9223372036854775808
  let wide = 340282366920938463463374607431768211456
  let negative = -340282366920938463463374607431768211456
  let assert #(True, False) = compare(wide, wide)
  let assert #(False, True) = compare(negative, wide)
  let assert #(False, True) = compare("left", "right")
  let assert True = negative < wide
  let assert True = negative <= negative
  let assert False = wide < negative
  let assert False = wide <= negative
  let assert True = wide > negative
  let assert True = wide >= wide
  let assert False = negative > wide
  let assert False = negative >= wide
  let assert True = wide == 340282366920938463463374607431768211456
  True
}
