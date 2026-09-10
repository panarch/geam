pub opaque type Session {
  Session(total: Int, advance: fn(Int) -> Int)
}

pub fn start(initial: Int) -> Session {
  Session(initial, fn(total) { total + 2 })
}

pub fn next(session: Session) -> Session {
  let Session(total, advance) = session
  Session(advance(total), advance)
}

pub fn total(session: Session) -> Int {
  session.total
}
