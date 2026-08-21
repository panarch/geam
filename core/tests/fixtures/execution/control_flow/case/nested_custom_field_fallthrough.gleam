type Maybe(value) {
  Some(value)
  None
}

type Envelope {
  Envelope(value: Maybe(Int))
}

fn classify(envelope: Envelope) {
  case envelope {
    Envelope(value: Some(_)) -> 1
    _ -> 0
  }
}

pub fn main() {
  #(classify(Envelope(None)), classify(Envelope(Some(1))))
}
// @geam:expect Tuple([Int(0), Int(1)])
