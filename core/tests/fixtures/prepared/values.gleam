type Box(a) {
  Box(a)
}

fn identity(value) {
  value
}

fn add_one(value: Int) {
  value + 1
}

fn stop(_value: Int) -> a {
  panic as "prepared stop"
}

fn generic() {
  identity
}

fn never() {
  stop
}

fn empty() -> List(a) {
  []
}

fn nested_empty() -> List(List(a)) {
  [[]]
}

fn codepoint() {
  let assert <<value:utf8_codepoint>> = <<65>>
  value
}

fn capture(offset) {
  fn(value) { offset + value }
}

fn countdown(value) {
  case value {
    0 -> 42
    _ -> countdown(value - 1)
  }
}

const number = 42

const decimal = 1.5

const text = "text"

const bits = <<42>>

const boxed = Box(42)

const flag = True

const nil = Nil

const pair = #(42, True)

const numbers = [42]

const decimals = [1.5]

const texts = ["text"]

const arrays = [<<42>>]

const boxes = [Box(42)]

const flags = [True]

const nils = [Nil]

const pairs = [#(42, True)]

const nested = [[42]]

const callbacks = [add_one]

const callback = add_one

const no_items = []

const nested_no_items = [[]]

const no_codepoints: List(UtfCodepoint) = []

pub fn run() -> Int {
  let assert True = identity(number) == 42
  let assert True = identity(decimal) == 1.5
  let assert True = identity(text) == "text"
  let assert True = identity(bits) == <<42>>
  let assert True = identity(boxed) == Box(42)
  let assert True = identity(flag) == True
  let assert True = identity(nil) == Nil
  let assert True = identity(pair) == #(42, True)
  let assert True = identity(codepoint()) == codepoint()

  let assert True = identity(numbers) == [42]
  let assert True = identity(decimals) == [1.5]
  let assert True = identity(texts) == ["text"]
  let assert True = identity(arrays) == [<<42>>]
  let assert True = identity(boxes) == [Box(42)]
  let assert True = identity(flags) == [True]
  let assert True = identity(nils) == [Nil]
  let assert True = identity(pairs) == [#(42, True)]
  let assert True = identity(nested) == [[42]]
  let assert [saved] = identity(callbacks)
  let assert True = saved(41) == 42
  let assert True = identity([codepoint()]) == [codepoint()]
  let assert True = identity(no_codepoints) == []
  let assert True = identity(no_items) == empty()
  let assert True = identity(nested_no_items) == nested_empty()

  let assert True = identity(fn() { number })() == 42
  let assert True = identity(fn() { decimal })() == 1.5
  let assert True = identity(fn() { text })() == "text"
  let assert True = identity(fn() { bits })() == <<42>>
  let assert True = identity(fn() { boxed })() == Box(42)
  let assert True = identity(fn() { flag })() == True
  let assert True = identity(fn() { nil })() == Nil
  let assert True = identity(fn() { pair })() == #(42, True)
  let assert True = identity(fn() { codepoint() })() == codepoint()
  let assert True = identity(fn() { numbers })() == [42]
  let assert True = identity(fn() { decimals })() == [1.5]
  let assert True = identity(fn() { texts })() == ["text"]
  let assert True = identity(fn() { arrays })() == [<<42>>]
  let assert True = identity(fn() { boxes })() == [Box(42)]
  let assert True = identity(fn() { flags })() == [True]
  let assert True = identity(fn() { nils })() == [Nil]
  let assert True = identity(fn() { pairs })() == [#(42, True)]
  let assert True = identity(fn() { nested })() == [[42]]
  let assert True = identity(fn() { [codepoint()] })() == [codepoint()]
  let assert True = identity(fn() { no_items })() == empty()
  let assert True = identity(fn() { nested_no_items })() == nested_empty()
  let assert [saved] = identity(fn() { callbacks })()
  let assert True = saved(41) == 42
  let assert True = identity(fn() { callback })()(41) == 42
  let _ = #(generic(), never())
  let assert True = capture(20)(22) == 42
  let assert True = countdown(8) == 42
  let assert Box([_, ..tail]) as original = Box([0, 42])
  let assert True = tail == [42]
  let assert True = original == Box([0, 42])
  echo number
  42
}

pub fn fail() -> Int {
  never()(0)
}

pub fn assertion(value: Int) -> Int {
  let assert 42 = value
  value
}
