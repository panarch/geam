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


// @geam:explain
// module main
// main int#0
//
// function int#0
//   entry b0 params=[] captures=[]
//   block b0 params=[]
//     %bool#0:shape#0(Bool) = bool.value False
//     %int#0:shape#1(Int) = int.value 0
//     %float#0:shape#2(Float) = float.value 0.0
//     %string#0:shape#3(String) = string.value ""
//     tail int#1 args=[%bool#0, %int#0, %float#0, %string#0]
//
// function int#1
//   entry b0 params=[%bool#0:shape#0(Bool), %int#0:shape#1(Int), %float#0:shape#2(Float), %string#0:shape#3(String)] captures=[]
//   block b0 params=[%bool#0:shape#0(Bool), %int#0:shape#1(Int), %float#0:shape#2(Float), %string#0:shape#3(String)]
//     branch %bool#0 true=b1(%bool#0, %int#0, %float#0, %string#0) false=b4(%float#0, %string#0)
//   block b1 params=[%bool#0:shape#0(Bool), %int#0:shape#1(Int), %float#0:shape#2(Float), %string#0:shape#3(String)]
//     switch.int %int#0 clauses=[1->b2(%bool#0, %int#0, %float#0, %string#0)] fallback=b3(%int#0)
//   block b2 params=[%bool#0:shape#0(Bool), %int#0:shape#1(Int), %float#0:shape#2(Float), %string#0:shape#3(String)]
//     tail int#1 args=[%bool#0, %int#0, %float#0, %string#0]
//   block b3 params=[%int#0:shape#1(Int)]
//     return %int#0
//   block b4 params=[%float#0:shape#2(Float), %string#0:shape#3(String)]
//     switch.float %float#0 clauses=[1.5->b5()] fallback=b6(%string#0)
//   block b5 params=[]
//     %int#0:shape#1(Int) = int.value 20
//     return %int#0
//   block b6 params=[%string#0:shape#3(String)]
//     switch.string %string#0 clauses=["x"->b7()] fallback=b8()
//   block b7 params=[]
//     %int#0:shape#1(Int) = int.value 30
//     return %int#0
//   block b8 params=[]
//     %int#0:shape#1(Int) = int.value 40
//     return %int#0
