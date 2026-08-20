use geam::Value;

use super::{ExpectedSurface, assert_surface, run_fixture};

const SURFACE: ExpectedSurface = ExpectedSurface {
    values: &[
        "Eq",
        "Gt",
        "Lt",
        "break_tie",
        "compare",
        "lazy_break_tie",
        "negate",
        "reverse",
        "to_int",
    ],
    types: &[("Order", 0)],
    type_aliases: &[],
    constructors: &[("Order", "Eq", 0), ("Order", "Gt", 0), ("Order", "Lt", 0)],
    functions: r#"
break_tie: fn(in: Order, with: Order) -> Order
compare: fn(Order, with: Order) -> Order
lazy_break_tie: fn(in: Order, with: fn() -> Order) -> Order
negate: fn(Order) -> Order
reverse: fn(fn(a, a) -> Order) -> fn(a, a) -> Order
to_int: fn(Order) -> Int
"#,
};

#[test]
fn tracks_official_gleam_order_public_surface() {
    assert_surface("gleam_order", "gleam/order", &["gleam/order"], &SURFACE);
}

#[test]
fn runs_official_gleam_order_behavior() {
    let value = run_fixture("gleam_order", &["gleam/order"]);
    let Value::Custom(order) = value else {
        panic!("gleam/order fixture should return Order");
    };

    assert_eq!(order.constructor_name(), "Gt");
    assert_eq!(order.type_().type_name().package(), "gleam_stdlib");
    assert_eq!(order.type_().type_name().module(), "gleam/order");
    assert_eq!(order.type_().type_name().name(), "Order");
}
