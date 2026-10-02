import clip
import clip/arg
import clip/arg_info.{
  ArgInfo, FlagInfo, Many1Repeat, ManyRepeat, NamedInfo, NoRepeat,
  PositionalInfo,
}
import clip/flag
import clip/help
import clip/opt
import gleam/option.{None, Some}
import gleam/string

pub type Selection {
  Selection(name: String, count: Int, enabled: Bool)
}

pub fn main() {
  let assert True = verify()
}

pub fn verify() -> Bool {
  commands()
  arguments()
  options()
  flags()
  subcommands()
  help_text()
  internal_wrappers()
  True
}

fn commands() -> Nil {
  let assert Ok(#(7, ["left"])) = clip.parse(clip.return(7), ["left"])
  let assert Ok(7) = clip.run(clip.return(7), ["unknown", "--anything"])
  let assert Error("stop") = clip.run(clip.fail("stop"), [])
  let build = {
    use name <- clip.parameter
    use count <- clip.parameter
    use enabled <- clip.parameter
    Selection(name, count, enabled)
  }
  let command =
    clip.command(build)
    |> clip.arg(arg.new("name"))
    |> clip.opt(opt.new("count") |> opt.short("c") |> opt.int)
    |> clip.flag(flag.new("enabled") |> flag.short("e"))
  let expected = Selection("한글", 3, True)
  let assert True =
    Ok(expected) == clip.run(command, ["--count", "3", "-e", "한글"])
  let assert True = Ok(expected) == clip.run(command, ["한글", "-c", "3", "-e"])
  let assert True =
    Ok(Selection("", 0, False)) == clip.run(command, ["", "--count", "0"])
  let assert Ok("name") =
    clip.run(clip.command1() |> clip.arg(arg.new("name")), ["name"])
  let assert Ok(#("name", 2)) =
    clip.run(
      clip.command2()
        |> clip.arg(arg.new("name"))
        |> clip.opt(opt.new("count") |> opt.int),
      ["--count", "2", "name"],
    )
  let assert Ok(#("name", 2, True)) =
    clip.run(
      clip.command3()
        |> clip.arg(arg.new("name"))
        |> clip.opt(opt.new("count") |> opt.int)
        |> clip.flag(flag.new("enabled")),
      ["--enabled", "name", "--count", "2"],
    )
  let assert Ok(#("name", 2, True, -1.5)) =
    clip.run(
      clip.command4()
        |> clip.arg(arg.new("name"))
        |> clip.opt(opt.new("count") |> opt.int)
        |> clip.flag(flag.new("enabled"))
        |> clip.arg(arg.new("ratio") |> arg.float),
      ["--enabled", "name", "-1.5", "--count", "2"],
    )
  let assert Ok(4) =
    clip.run(clip.apply(clip.return(fn(n) { n + 1 }), clip.return(3)), [])
  let assert Error("function first") =
    clip.run(clip.apply(clip.fail("function first"), clip.fail("value")), [])
  let assert Error("value") =
    clip.run(clip.apply(clip.return(fn(n) { n + 1 }), clip.fail("value")), [])
  let mapped =
    arg.new("trace")
    |> arg.map(fn(v) { v <> ":first" })
    |> arg.try_map(fn(v) { Ok(v <> ":second") })
  let assert Ok("start:first:second") =
    clip.run(clip.command1() |> clip.arg(mapped), ["start"])
  let failed =
    arg.new("trace")
    |> arg.try_map(fn(_) { Error("first failure") })
    |> arg.map(fn(_) { panic as "must not run after failure" })
  let assert Error("first failure") =
    clip.run(clip.command1() |> clip.arg(failed), ["start"])
  let assert Ok(#(1, ["--", "literal"])) =
    clip.parse(clip.return(1), ["--", "--", "literal"])
  Nil
}

fn arguments() -> Nil {
  let required = clip.command1() |> clip.arg(arg.new("file"))
  let assert Error("missing required arg: file") = clip.run(required, [])
  let assert Ok(#("one", ["two"])) = clip.parse(required, ["one", "two"])
  let assert Ok(#("-file", ["--help"])) =
    clip.parse(required, ["--", "-file", "--help"])
  let assert Ok(#("value", ["--unknown"])) =
    clip.parse(required, ["--unknown", "value"])
  let assert Ok("fallback") =
    clip.run(
      clip.command1() |> clip.arg(arg.new("file") |> arg.default("fallback")),
      [],
    )
  let optional = clip.command1() |> clip.arg(arg.new("file") |> arg.optional)
  let assert Ok(Error(Nil)) = clip.run(optional, [])
  let assert Ok(Ok("")) = clip.run(optional, [""])
  let numbers = clip.command1() |> clip.arg(arg.new("number") |> arg.int)
  let assert Ok(-12) = clip.run(numbers, ["-12"])
  let assert Error("Non-integer value provided for number") =
    clip.run(numbers, ["oops"])
  let assert Error("missing required arg: number") =
    clip.run(
      clip.command1()
        |> clip.arg(arg.new("number") |> arg.default("3") |> arg.int),
      [],
    )
  let assert Error("missing required arg: value") =
    clip.run(
      clip.command1()
        |> clip.arg(
          arg.new("value") |> arg.default("x") |> arg.map(string.uppercase),
        ),
      [],
    )
  let assert Error("Non-float value provided for ratio") =
    clip.run(clip.command1() |> clip.arg(arg.new("ratio") |> arg.float), [
      "oops",
    ])
  let many = clip.command1() |> clip.arg_many(arg.new("number") |> arg.int)
  let assert Ok(#([], [])) = clip.parse(many, [])
  let assert Ok(#([1, -2], ["oops", "3"])) =
    clip.parse(many, ["1", "-2", "oops", "3"])
  let one_or_more =
    clip.command1() |> clip.arg_many1(arg.new("number") |> arg.int)
  let assert Error("must provide at least one valid value for: number") =
    clip.run(one_or_more, [])
  let assert Error("must provide at least one valid value for: number") =
    clip.run(one_or_more, ["oops"])
  let assert Ok(#([1], ["oops"])) = clip.parse(one_or_more, ["1", "oops"])
  let assert Ok(["-file", "--other"]) =
    clip.run(clip.command1() |> clip.arg_many1(arg.new("file")), [
      "--",
      "-file",
      "--other",
    ])
  let mixed =
    clip.command3()
    |> clip.arg_many1(arg.new("file"))
    |> clip.opt(opt.new("pattern") |> opt.short("p"))
    |> clip.flag(flag.new("ignore") |> flag.short("i"))
  let assert Ok(#(["one", "two"], "space value", True)) =
    clip.run(mixed, ["one", "-p", "space value", "-i", "two"])
  let assert Error("missing required arg: --pattern, -p") =
    clip.run(mixed, ["one", "-p"])
  Nil
}

fn options() -> Nil {
  let named = opt.new("name") |> opt.short("n")
  let required = clip.command1() |> clip.opt(named)
  let assert Ok(#("first", ["--name", "second", "left"])) =
    clip.parse(required, ["--name", "first", "--name", "second", "left"])
  let assert Ok("-dash") = clip.run(required, ["-n", "-dash"])
  let assert Ok("") = clip.run(required, ["--name", ""])
  let assert Error("missing required arg: --name, -n") =
    clip.run(required, ["-n"])
  let assert Error("missing required arg: --name, -n") =
    clip.run(required, ["--", "--name", "value"])
  let assert Ok("fallback") =
    clip.run(clip.command1() |> clip.opt(named |> opt.default("fallback")), [])
  let assert Ok(#("fallback", ["--name", "x"])) =
    clip.parse(clip.command1() |> clip.opt(named |> opt.default("fallback")), [
      "--",
      "--name",
      "x",
    ])
  let optional = clip.command1() |> clip.opt(named |> opt.optional)
  let assert Ok(Error(Nil)) = clip.run(optional, [])
  let assert Ok(Ok("한글")) = clip.run(optional, ["-n", "한글"])
  let assert Ok(9) =
    clip.run(clip.command1() |> clip.opt(opt.new("count") |> opt.int), [
      "--count",
      "9",
    ])
  let assert Error("Non-integer value provided for count") =
    clip.run(clip.command1() |> clip.opt(opt.new("count") |> opt.int), [
      "--count",
      "x",
    ])
  let assert Ok(-1.25) =
    clip.run(clip.command1() |> clip.opt(opt.new("ratio") |> opt.float), [
      "--ratio",
      "-1.25",
    ])
  let assert Error("Non-float value provided for ratio") =
    clip.run(clip.command1() |> clip.opt(opt.new("ratio") |> opt.float), [
      "--ratio",
      "x",
    ])
  let mapped =
    named
    |> opt.map(string.uppercase)
    |> opt.try_map(fn(v) { Ok(v <> ":mapped") })
  let assert Ok("ABC:mapped") =
    clip.run(clip.command1() |> clip.opt(mapped), ["-n", "abc"])
  let failed =
    named
    |> opt.try_map(fn(_) { Error("option conversion") })
    |> opt.map(fn(_) { panic as "must not run after option failure" })
  let assert Error("option conversion") =
    clip.run(clip.command1() |> clip.opt(failed), ["-n", "x"])
  let assert Error("missing required arg: --name, -n") =
    clip.run(
      clip.command1()
        |> clip.opt(named |> opt.default("x") |> opt.map(string.uppercase)),
      [],
    )
  let assert Error("missing required arg: --name, -n") =
    clip.run(
      clip.command1() |> clip.opt(named |> opt.default("1") |> opt.int),
      [],
    )
  Nil
}

fn flags() -> Nil {
  let command =
    clip.command1() |> clip.flag(flag.new("quiet") |> flag.short("q"))
  let assert Ok(False) = clip.run(command, [])
  let assert Ok(#(True, ["left", "right"])) =
    clip.parse(command, ["left", "--quiet", "right"])
  let assert Ok(#(True, ["--quiet"])) = clip.parse(command, ["-q", "--quiet"])
  let assert Ok(#(False, ["--quiet"])) = clip.parse(command, ["--", "--quiet"])
  Nil
}

fn subcommands() -> Nil {
  let child =
    clip.command1()
    |> clip.arg(arg.new("value"))
    |> clip.help(help.custom(fn(_) { "child help" }))
  let parent =
    clip.subcommands([#("child", child), #("version", clip.return("1.0"))])
    |> clip.help(
      help.custom(fn(info) {
        let assert ["child", "version"] = info.subcommands
        "parent help"
      }),
    )
  let assert Ok("v") = clip.run(parent, ["child", "v"])
  let assert Ok("1.0") = clip.run(parent, ["version"])
  let assert Error("No subcommand provided") = clip.run(parent, [])
  let assert Error("No subcommand provided") = clip.run(parent, ["unknown"])
  let assert Error("parent help") = clip.run(parent, ["--help"])
  let assert Error("child help") = clip.run(parent, ["child", "-h"])
  let assert Ok("--help") = clip.run(parent, ["child", "--", "--help"])
  let assert Ok("default") =
    clip.run(
      clip.subcommands_with_default([#("child", child)], clip.return("default")),
      [],
    )
  let assert Ok(#("default", ["unknown"])) =
    clip.parse(clip.subcommands_with_default([], clip.return("default")), [
      "unknown",
    ])
  let assert Ok("nested") =
    clip.run(
      clip.subcommands([
        #("outer", clip.subcommands([#("inner", clip.return("nested"))])),
      ]),
      ["outer", "inner"],
    )
  Nil
}

fn help_text() -> Nil {
  let empty = arg_info.empty()
  let assert ArgInfo([], [], [], []) = empty
  let assert "find\n\n  Search files\n\nUsage:\n\n  find" =
    arg_info.help_text(empty, "find", "Search files")
  let file = arg.new("file") |> arg.help("Input file")
  let named =
    opt.new("limit")
    |> opt.short("l")
    |> opt.default("10")
    |> opt.help("Maximum")
  let quiet = flag.new("quiet") |> flag.short("q") |> flag.help("Quiet")
  let combined =
    arg_info.merge(
      opt.to_arg_info(named),
      arg_info.merge(arg.to_arg_info_many1(file), flag.to_arg_info(quiet)),
    )
  let expected =
    ArgInfo(
      named: [NamedInfo("limit", Some("l"), Some("\"10\""), Some("Maximum"))],
      positional: [
        PositionalInfo("file", None, Some("Input file"), Many1Repeat),
      ],
      flags: [FlagInfo("quiet", Some("q"), Some("Quiet"))],
      subcommands: [],
    )
  let assert True = combined == expected
  let assert True =
    arg_info.merge(combined, combined)
    == ArgInfo(
      named: [
        NamedInfo("limit", Some("l"), Some("\"10\""), Some("Maximum")),
        NamedInfo("limit", Some("l"), Some("\"10\""), Some("Maximum")),
      ],
      positional: [
        PositionalInfo("file", None, Some("Input file"), Many1Repeat),
        PositionalInfo("file", None, Some("Input file"), Many1Repeat),
      ],
      flags: [
        FlagInfo("quiet", Some("q"), Some("Quiet")),
        FlagInfo("quiet", Some("q"), Some("Quiet")),
      ],
      subcommands: [],
    )
  let assert "tool\n\n  description\n\nUsage:\n\n  tool [OPTIONS]\n\nOptions:\n\n  [--help,-h]\tPrint this help" =
    clip.run(clip.return(Nil) |> clip.help(help.simple("tool", "description")), [
      "-h",
    ])
    |> fn(result) {
      let assert Error(text) = result
      text
    }
  let captured =
    clip.command3()
    |> clip.opt(named)
    |> clip.arg_many1(file)
    |> clip.flag(quiet)
    |> clip.help(
      help.custom(fn(info) {
        let assert True =
          info
          == ArgInfo(..expected, flags: [
            FlagInfo("quiet", Some("q"), Some("Quiet")),
            FlagInfo("help", Some("h"), Some("Print this help")),
          ])
        "custom exact"
      }),
    )
  let assert Error("custom exact") = clip.run(captured, ["--help"])
  let assert Ok(#("10", ["--help"], False)) =
    clip.run(captured, ["--", "--help"])
  let cases = [
    #(PositionalInfo("item", None, None, NoRepeat), "ITEM", ""),
    #(PositionalInfo("item", Some("7"), None, NoRepeat), "[ITEM]", "Default: 7"),
    #(
      PositionalInfo("item", Some("7"), Some("Value"), NoRepeat),
      "[ITEM]",
      "Value (default: 7)",
    ),
    #(PositionalInfo("item", Some("Error(Nil)"), None, NoRepeat), "[ITEM]", ""),
    #(
      PositionalInfo("item", None, None, ManyRepeat),
      "[ITEM...]",
      "Zero or more",
    ),
    #(
      PositionalInfo("item", None, Some("Files"), ManyRepeat),
      "[ITEM...]",
      "Files (zero or more)",
    ),
    #(PositionalInfo("item", None, None, Many1Repeat), "ITEM...", "One or more"),
    #(
      PositionalInfo("item", None, Some("Files"), Many1Repeat),
      "ITEM...",
      "Files (one or more)",
    ),
  ]
  check_positional_help(cases)
  let assert "tool\n\n  description\n\nUsage:\n\n  tool <COMMAND>\n\nCommands:\n\n  search\n  count" =
    arg_info.help_text(
      ArgInfo(..empty, subcommands: ["search", "count"]),
      "tool",
      "description",
    )
  let assert "tool\n\n  description\n\nUsage:\n\n  tool [OPTIONS]\n\nOptions:\n\n  (--name NAME)\t" =
    arg_info.help_text(opt.to_arg_info(opt.new("name")), "tool", "description")
  let assert "tool\n\n  description\n\nUsage:\n\n  tool [OPTIONS]\n\nOptions:\n\n  [--name NAME]\tDefault: \"x\"" =
    arg_info.help_text(
      opt.to_arg_info(opt.new("name") |> opt.default("x")),
      "tool",
      "description",
    )
  let assert "tool\n\n  description\n\nUsage:\n\n  tool [OPTIONS]\n\nOptions:\n\n  [--name NAME]\tName (default: \"x\")" =
    arg_info.help_text(
      opt.to_arg_info(opt.new("name") |> opt.default("x") |> opt.help("Name")),
      "tool",
      "description",
    )
  let assert "tool\n\n  description\n\nUsage:\n\n  tool [OPTIONS]\n\nOptions:\n\n  [--name NAME]\t" =
    arg_info.help_text(
      opt.to_arg_info(opt.new("name") |> opt.optional),
      "tool",
      "description",
    )
  let assert "tool\n\n  description\n\nUsage:\n\n  tool [OPTIONS]\n\nOptions:\n\n  [--quiet]\t" =
    arg_info.help_text(
      flag.to_arg_info(flag.new("quiet")),
      "tool",
      "description",
    )
  Nil
}

fn check_positional_help(cases) {
  case cases {
    [] -> Nil
    [#(info, usage, description), ..rest] -> {
      let expected =
        "tool\n\n  description\n\nUsage:\n\n  tool "
        <> usage
        <> "\n\nArguments:\n\n  "
        <> usage
        <> "\t"
        <> description
      let assert True =
        expected
        == arg_info.help_text(
          ArgInfo(..arg_info.empty(), positional: [info]),
          "tool",
          "description",
        )
      check_positional_help(rest)
    }
  }
}

fn internal_wrappers() -> Nil {
  let plain = arg.new("file")
  let assert ArgInfo([], [PositionalInfo("file", None, None, NoRepeat)], [], []) =
    arg.to_arg_info(plain)
  let assert ArgInfo(
    [],
    [PositionalInfo("file", None, None, ManyRepeat)],
    [],
    [],
  ) = arg.to_arg_info_many(plain)
  let assert ArgInfo(
    [],
    [PositionalInfo("file", None, None, Many1Repeat)],
    [],
    [],
  ) = arg.to_arg_info_many1(plain)
  let assert Ok(#("x", ["y"])) = arg.run(plain, ["x", "y"])
  let info =
    arg_info.merge(
      opt.to_arg_info(opt.new("pattern")),
      flag.to_arg_info(flag.new("quiet")),
    )
  let assert Ok(#("file", ["--pattern", "text", "--quiet"])) =
    arg.run_with_info(plain, ["--pattern", "text", "--quiet", "file"], info)
  let assert Ok(#(["x", "y"], [])) = arg.run_many(plain, ["x", "y"])
  let assert Ok(#(["file"], ["--pattern", "text"])) =
    arg.run_many_with_info(plain, ["--pattern", "text", "file"], info)
  let assert Ok(#(["x"], [])) = arg.run_many1(plain, ["x"])
  let assert Error("must provide at least one valid value for: file") =
    arg.run_many1_with_info(plain, ["--pattern", "text"], info)
  let assert Ok(#("x", ["tail"])) =
    opt.run(opt.new("name"), ["--name", "x", "tail"])
  let assert Ok(#(True, ["tail"])) =
    flag.run(flag.new("quiet"), ["--quiet", "tail"])
  let assert "direct" =
    help.run(help.custom(fn(_) { "direct" }), arg_info.empty())
  Nil
}
