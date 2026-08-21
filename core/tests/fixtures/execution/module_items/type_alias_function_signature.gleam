pub type UserId =
  Int

fn identity(value: UserId) -> UserId {
  value
}

pub fn main() {
  identity(41)
}

// @geam:expect Int(41)
