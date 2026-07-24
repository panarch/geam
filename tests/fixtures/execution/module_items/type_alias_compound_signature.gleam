pub type UserId =
  Int

pub type UserNames =
  #(UserId, List(String))

fn pair(id: UserId, names: List(String)) -> UserNames {
  #(id, names)
}

pub fn main() {
  pair(1, ["one"])
}

// @geam:expect Tuple([Int(1), List(String)([String("one")])])
