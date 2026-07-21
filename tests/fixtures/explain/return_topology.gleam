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

// geam:explain
// module main
// main int#0
//
// function int#0
//   graph entry=b0
//   b0 instructions=4 tail int#1 args=4
//
// function int#1
//   graph entry=b0
//   b0 instructions=0 branch bool true=b1 false=b4
//   b1 instructions=0 switch int 1->b2 fallback=b3
//   b2 instructions=0 tail int#1 args=4
//   b3 instructions=0 return
//   b4 instructions=0 switch float 1.5->b5 fallback=b6
//   b5 instructions=1 return
//   b6 instructions=0 switch string "x"->b7 fallback=b8
//   b7 instructions=1 return
//   b8 instructions=1 return
