pub type Boxed(a) {
  Boxed(a, label: String)
}

pub type Generic(a) {
  Generic(value: a, label: String)
}

pub type Contact {
  Email(name: String, address: String)
  Phone(name: String, number: String)
}

pub type Person {
  Person(name: String, address: String)
}

const constant_person = Person(name: "Constant", address: "old@example.com")
const updated_constant_person = Person(..constant_person, address: "new@example.com")

fn count(counter: Int, value: Boxed(Int)) {
  #(counter + 1, value)
}

fn replace_boxed(counter: Int, value: Boxed(Int)) {
  let #(counter, value) = count(counter, value)
  #(counter, Boxed(..value, label: "two"))
}

fn update_email(contact: Contact) {
  case contact {
    Email(..) -> Email(..contact, name: contact.name <> "!")
    Phone(..) -> contact
  }
}

fn rename(person: Person) {
  Person(..person, name: person.name <> "!")
}

pub fn main() {
  let boxed = Boxed(1, label: "one")
  let #(counter, boxed) = replace_boxed(0, boxed)
  let generic = Generic(value: 1, label: "one")
  let generic = Generic(..generic, value: 1.5)
  let email = Email(name: "Lucy", address: "lucy@example.com")
  let email = update_email(email)
  let person = Person(name: "Lucy", address: "lucy@example.com")
  let captured = fn() { Person(..rename(person), address: "new@example.com") }
  #(counter, boxed, generic, email, captured(), updated_constant_person)
}

// geam:expect Tuple([Int(1), Custom(type=geam/main/Boxed(Int), constructor=Boxed#0, fields=[Int(1), label: String("two")]), Custom(type=geam/main/Generic(Float), constructor=Generic#0, fields=[value: Float(1.5), label: String("one")]), Custom(type=geam/main/Contact, constructor=Email#0, fields=[name: String("Lucy!"), address: String("lucy@example.com")]), Custom(type=geam/main/Person, constructor=Person#0, fields=[name: String("Lucy!"), address: String("new@example.com")]), Custom(type=geam/main/Person, constructor=Person#0, fields=[name: String("Constant"), address: String("new@example.com")])])
