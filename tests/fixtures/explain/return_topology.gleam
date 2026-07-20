fn choose(flag: Bool, number: Int, decimal: Float, text: String) -> Int {
  case flag {
    True -> case number {
      1 -> choose(flag, number, decimal, text)
      _ -> {
        let next = number
        next
      }
    }
    False -> case decimal {
      1.5 -> 20
      _ -> case text {
        "x" -> 30
        _ -> 40
      }
    }
  }
}

pub fn main() { choose(False, 0, 0.0, "") }
