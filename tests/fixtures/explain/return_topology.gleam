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
//   entry steps=0
//   graph entry=b0
//   b0 tail int#1 args=4
//
// function int#1
//   entry steps=0
//   graph entry=b9
//   b0 tail int#1 args=4
//   b1 return
//   b2 steps count=1 next=b1
//   b3 switch int 1->b0 fallback=b2
//   b4 return
//   b5 return
//   b6 return
//   b7 switch string "x"->b5 fallback=b6
//   b8 switch float 1.5->b4 fallback=b7
//   b9 branch bool true=b3 false=b8
