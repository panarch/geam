pub type Envelope {
  Wrapped(#(Bool, Result(#(String, String), Nil)))
  Empty
}

pub type Marker {
  Found
}

pub type NilBox {
  NilBox(field: Nil)
}
