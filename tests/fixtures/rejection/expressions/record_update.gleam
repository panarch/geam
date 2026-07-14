pub type Person {
  Person(name: String, age: Int)
}

pub fn main() {
  let person = Person(name: "Lucy", age: 30)
  Person(..person, age: 31)
}

// geam:reject unsupported expression: record update
