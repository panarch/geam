pub type Node(a) {
  Fragment(children: List(a))
  Element(namespace: String, children: List(a))
}

type PlainNode {
  PlainFragment(children: List(Int))
  PlainElement(namespace: String, children: List(Int))
}

type ReorderedNode(a) {
  EarlyElement(namespace: String, children: List(a))
  LateFragment(children: List(a))
}

type SingleNode(a) {
  SingleNode(namespace: String, children: List(a))
}

type SharedNode {
  SharedFragment(children: List(Int))
  SharedElement(children: List(Int), namespace: String)
}

type Details {
  Details(namespace: String)
}

type NestedNode {
  NestedFragment(details: Int)
  NestedElement(details: Details)
}

type TupleNode {
  TupleFragment(details: Int)
  TupleElement(details: #(String, Int))
}

type VNode(a) {
  VFragment(key: String, count: Int, children: List(a))
  VElement(key: String, count: Int, namespace: String, children: List(a))
}

fn check(node: Node(a)) -> Bool {
  case node {
    Element(..) if node.namespace == "html" -> True
    _ -> False
  }
}

fn check_or(node: Node(a), inline: Bool) -> Bool {
  case node {
    Element(..) if !inline || node.namespace != "" -> True
    _ -> False
  }
}

pub fn matches_namespace(
  namespace: String,
  inline: Bool,
  fragment: Bool,
) -> Bool {
  let node = case fragment {
    True -> Fragment([42])
    False -> Element(namespace, [42])
  }
  check_or(node, inline)
}

fn check_plain(node: PlainNode) -> Bool {
  case node {
    PlainElement(..) if node.namespace == "html" -> True
    _ -> False
  }
}

fn check_reordered(node: ReorderedNode(a)) -> Bool {
  case node {
    EarlyElement(..) if node.namespace == "html" -> True
    _ -> False
  }
}

fn check_body(node: Node(a)) -> Bool {
  case node {
    Element(..) -> node.namespace == "html"
    _ -> False
  }
}

fn check_binding(node: Node(a)) -> Bool {
  case node {
    Element(namespace: namespace, ..) if namespace == "html" -> True
    _ -> False
  }
}

fn check_single(node: SingleNode(a)) -> Bool {
  case node {
    SingleNode(..) if node.namespace == "html" -> True
    _ -> False
  }
}

fn check_shared(node: SharedNode) -> Bool {
  case node {
    _ if node.children == [] -> True
    _ -> False
  }
}

fn check_alias(pair: #(Node(a), Bool)) -> Bool {
  case pair {
    #(Element(..) as node, inline) if !inline || node.namespace != "" -> True
    _ -> False
  }
}

fn check_scopes(node: Node(a)) -> Bool {
  let first = case node {
    Element(..) if node.namespace == "html" -> True
    Element(..) if node.namespace == "svg" -> True
    Fragment(..) if node.children == [] -> True
    _ -> False
  }
  let node = Fragment([])
  first
  && case node {
    Fragment(..) if node.children == [] -> True
    _ -> False
  }
}

fn check_nested(node: NestedNode) -> Bool {
  case node {
    NestedElement(..) if node.details.namespace == "html" -> True
    _ -> False
  }
}

fn check_tuple(node: TupleNode) -> Bool {
  case node {
    TupleElement(..) if node.details.0 == "html" -> True
    _ -> False
  }
}

fn check_vnode(node: VNode(a), inline: Bool) -> Bool {
  case node {
    VElement(..) if !inline || node.namespace != "" -> True
    _ -> False
  }
}

pub fn main() -> Bool {
  let assert True = check(Element("html", [42]))
  let assert False = check(Element("svg", [42]))
  let assert False = check(Fragment([42]))
  let assert True = check_or(Element("", [42]), False)
  let assert True = check_or(Element("html", [42]), True)
  let assert False = check_or(Element("", [42]), True)
  let assert False = check_or(Fragment([42]), False)
  let assert False = check_or(Fragment([42]), True)
  let assert True = check_plain(PlainElement("html", [42]))
  let assert False = check_plain(PlainFragment([42]))
  let assert True = check_reordered(EarlyElement("html", [42]))
  let assert False = check_reordered(LateFragment([42]))
  let assert True = check_body(Element("html", [42]))
  let assert False = check_body(Fragment([42]))
  let assert True = check_binding(Element("html", [42]))
  let assert False = check_binding(Element("svg", [42]))
  let assert True = check_single(SingleNode("html", [42]))
  let assert False = check_single(SingleNode("svg", [42]))
  let assert True = check_shared(SharedElement([], "html"))
  let assert True = check_shared(SharedFragment([]))
  let assert False = check_shared(SharedFragment([42]))
  let assert True = check_alias(#(Element("html", [42]), True))
  let assert False = check_alias(#(Fragment([42]), False))
  let assert False = check_alias(#(Element("", [42]), True))
  let assert True = check_scopes(Element("html", [42]))
  let assert True = check_scopes(Element("svg", [42]))
  let assert True = check_scopes(Fragment([]))
  let assert False = check_scopes(Fragment([42]))
  let assert False = check_scopes(Element("", [42]))
  let assert True = check_nested(NestedElement(Details("html")))
  let assert False = check_nested(NestedElement(Details("svg")))
  let assert False = check_nested(NestedFragment(42))
  let assert True = check_tuple(TupleElement(#("html", 42)))
  let assert False = check_tuple(TupleElement(#("svg", 42)))
  let assert False = check_tuple(TupleFragment(42))
  let assert True = check_vnode(VElement("key", 1, "html", [42]), True)
  let assert True = check_vnode(VElement("key", 1, "", [42]), False)
  let assert False = check_vnode(VElement("key", 1, "", [42]), True)
  let assert False = check_vnode(VFragment("key", 1, [42]), False)
  True
}
