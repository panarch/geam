pub type Contact {
  Email(name: String, address: String)
  Phone(name: String, number: String)
}

pub type Profile {
  Profile(person: Person)
}

pub type Person {
  Person(name: String, address: String)
}

fn shared_name(contact: Contact) {
  contact.name
}

fn inferred_detail(contact: Contact) {
  case contact {
    Email(..) -> contact.address
    Phone(..) -> contact.number
  }
}

fn guarded_name(contact: Contact) {
  case contact {
    _ if contact.name == "Lucy" -> contact.name
    _ -> "missing"
  }
}

pub fn main() {
  let email = Email(name: "Lucy", address: "lucy@example.com")
  let profile = Profile(person: Person(name: "Lucy", address: "lucy@example.com"))
  let captured = fn() { profile.person.name }
  #(
    shared_name(email),
    inferred_detail(email),
    guarded_name(email),
    profile.person.address,
    captured(),
  )
}

// geam:expect Tuple([String("Lucy"), String("lucy@example.com"), String("Lucy"), String("lucy@example.com"), String("Lucy")])
