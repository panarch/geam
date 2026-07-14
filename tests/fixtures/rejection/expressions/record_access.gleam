pub type Person {
  Person(name: String, age: Int)
}

pub fn main() {
  let person = Person(name: "Lucy", age: 30)
  person.name
}

// geam:reject unsupported expression: record access
