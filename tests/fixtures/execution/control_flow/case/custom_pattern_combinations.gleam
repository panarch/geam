pub type Item {
  Empty
  One(Int)
  Pair(left: Int, right: Int)
}

fn sum(item: Item, fallback: Item) {
  case item, fallback {
    Pair(left: left, right: right) as pair, _ if left > 0 -> {
      let captured = fn() {
        case pair {
          Pair(left: inner_left, right: inner_right) -> inner_left + inner_right
          _ -> 0
        }
      }
      captured()
    }
    One(value), _ | _, One(value) if value > 0 -> value
    _, _ -> 0
  }
}

pub fn main() {
  let nested = case #(One(5), [One(6)]) {
    #(One(left), [One(right)]) -> left + right
    _ -> 0
  }

  #(sum(Pair(left: 1, right: 2), Empty), sum(Empty, One(4)), nested)
}

// @geam:expect Tuple([Int(3), Int(4), Int(11)])
