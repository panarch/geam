pub type Boxed(value) {
  Boxed(value)
}

fn tuple_block() -> #(value) {
  {
    let _ = panic as "generic tuple block failed"
    #(panic as "unreached tuple result")
  }
}

fn custom_block() -> Boxed(value) {
  {
    let _ = panic as "generic custom block failed"
    Boxed(panic as "unreached custom result")
  }
}

fn choose(selector: Bool) {
  case selector {
    True -> {
      let _ = tuple_block()
      Nil
    }
    False -> {
      let _ = custom_block()
      Nil
    }
  }
}

pub fn main() {
  choose(1 == 1)
}

// geam:expect-error
// geam::panic
//
//   x panic: generic tuple block failed
//    ,-[tests/fixtures/execution_errors/functions/generic_recursive_never_block_handoffs.gleam:7:13]
//  6 |   {
//  7 |     let _ = panic as "generic tuple block failed"
//    :             ^^^^^^^^^^^^^^^^^^|^^^^^^^^^^^^^^^^^^
//    :                               `-- panic in main.tuple_block
//  8 |     #(panic as "unreached tuple result")
//    `----
