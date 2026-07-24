pub type Person {
  Person(name: String, age: Int)
}

pub fn main() {
  let person = Person(name: "Lucy", age: 30)
  Person(..person, age: 31)
}

// @geam:expect Custom(type=geam/main/Person, constructor=Person#0, fields=[name: String("Lucy"), age: Int(31)])
