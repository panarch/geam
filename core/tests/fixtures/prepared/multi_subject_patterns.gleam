pub type Option(a) {
  Some(a)
  None
}

pub type Repeat {
  NoRepeat
  ManyRepeat
  Many1Repeat
}

pub fn label(repeat: Repeat, default: Option(String)) -> String {
  case repeat, default {
    ManyRepeat, _ -> "many"
    Many1Repeat, _ -> "many1"
    NoRepeat, None -> "required"
    NoRepeat, Some(value) -> value
  }
}

pub fn boolean_label(selected: Bool, default: Option(String)) -> String {
  case selected, default {
    True, _ -> "selected"
    False, None -> "required"
    False, Some(value) -> value
  }
}

pub type Configuration {
  Configuration(repeat: Repeat, default: Option(String))
}

fn record_label(config: Configuration) -> String {
  case config.repeat, config.default {
    ManyRepeat, _ -> "many"
    Many1Repeat, _ -> "many1"
    NoRepeat, None | NoRepeat, Some("Error(Nil)") -> "required"
    NoRepeat, Some(value) -> value
  }
}

fn captured(config: Configuration) -> fn() -> String {
  fn() {
    case config.repeat, config.default {
      ManyRepeat, _ -> "many"
      Many1Repeat, _ -> "many1"
      NoRepeat, None -> "required"
      NoRepeat, Some(value) -> value
    }
  }
}

fn guarded(repeat: Repeat, default: Option(String)) -> String {
  case repeat, default {
    ManyRepeat, _ -> "many"
    Many1Repeat, _ -> "many1"
    NoRepeat, Some(value) if value == "guarded" -> "selected"
    NoRepeat, None | NoRepeat, Some("Error(Nil)") -> "required"
    NoRepeat, Some(value) -> value
  }
}

fn choose(repeat: Repeat, default: Option(a), fallback: a) -> a {
  case repeat, default {
    ManyRepeat, _ -> fallback
    Many1Repeat, _ -> fallback
    NoRepeat, None -> fallback
    NoRepeat, Some(value) -> value
  }
}

fn single_subject(default: Option(String)) -> String {
  case default {
    None | Some("Error(Nil)") -> "required"
    Some(value) -> value
  }
}

pub fn main() -> String {
  let assert "many" = label(ManyRepeat, None)
  let assert "many1" = label(Many1Repeat, Some("value"))
  let assert "required" = label(NoRepeat, None)
  let assert "Error(Nil)" = label(NoRepeat, Some("Error(Nil)"))
  let assert "value" = label(NoRepeat, Some("value"))
  let assert "selected" = boolean_label(True, None)
  let assert "required" = boolean_label(False, None)
  let assert "value" = boolean_label(False, Some("value"))
  let assert "many" = record_label(Configuration(ManyRepeat, None))
  let assert "many1" = record_label(Configuration(Many1Repeat, Some("value")))
  let assert "required" = record_label(Configuration(NoRepeat, None))
  let assert "required" =
    record_label(Configuration(NoRepeat, Some("Error(Nil)")))
  let assert "value" = record_label(Configuration(NoRepeat, Some("value")))
  let read = captured(Configuration(NoRepeat, Some("captured")))
  let assert "captured" = read()
  let read = captured(Configuration(NoRepeat, None))
  let assert "required" = read()
  let assert "selected" = guarded(NoRepeat, Some("guarded"))
  let assert "value" = guarded(NoRepeat, Some("value"))
  let assert "required" = guarded(NoRepeat, None)
  let assert "required" = guarded(NoRepeat, Some("Error(Nil)"))
  let assert "required" = single_subject(None)
  let assert "required" = single_subject(Some("Error(Nil)"))
  let assert "value" = single_subject(Some("value"))
  let assert 7 = choose(NoRepeat, None, 7)
  let assert 42 = choose(NoRepeat, Some(42), 7)
  let assert False = choose(NoRepeat, None, False)
  let assert True = choose(NoRepeat, Some(True), False)
  let assert 1.5 = choose(NoRepeat, Some(1.5), 0.0)
  let assert #("tuple", 42) = choose(NoRepeat, Some(#("tuple", 42)), #("", 0))
  let assert [1, 2] = choose(NoRepeat, Some([1, 2]), [])
  let assert Some(42) = choose(NoRepeat, Some(Some(42)), None)
  let assert <<42>> = choose(NoRepeat, Some(<<42>>), <<>>)
  let add = choose(NoRepeat, Some(fn(n) { n + 1 }), fn(n) { n })
  let assert 43 = add(42)
  label(NoRepeat, Some("value"))
}
