import gleam/erlang/application
import gleam/erlang/atom
import gleam/erlang/charlist
import gleam/erlang/process

pub fn fail(which: Int, text: String) {
  case which {
    0 -> {
      let _ = charlist.from_string(text)
      Nil
    }
    1 -> {
      let _ = atom.create(text)
      Nil
    }
    _ -> {
      let _: process.Name(Int) = process.new_name(text)
      Nil
    }
  }
}

pub fn verify(text: String) {
  assert atom.get(text) == Error(Nil)
  assert application.priv_directory(text) == Error(Nil)
  assert charlist.to_string(charlist.from_string("é")) == "é"
  assert atom.to_string(atom.create("raw_text_still_valid"))
    == "raw_text_still_valid"
  Nil
}
