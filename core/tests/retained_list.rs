use geam_core::embedding::{FunctionDeclaration, Iter, List, ModuleBuilder};
use num_bigint::BigInt;

#[test]
fn retained_cursor_keeps_its_send_sync_contract_and_outlives_the_module() {
    fn require_send_sync<T: Send + Sync>() {}

    require_send_sync::<List<BigInt>>();
    require_send_sync::<Iter<'static, BigInt>>();

    let typed = geam_core::compile_typed_module(
        "library",
        "library.gleam",
        include_str!("fixtures/embedding/retained_list.gleam"),
    )
    .expect("typed source");
    let (bindings, keep) = ModuleBuilder::new(typed)
        .expect("plan")
        .function(FunctionDeclaration::<(List<BigInt>,), List<BigInt>>::new(
            "keep",
        ))
        .expect("entry");
    let module = bindings.seal();
    let expected: Vec<BigInt> = (0..1_000).map(BigInt::from).collect();
    let values = module
        .call(&keep, (expected.clone(),), &mut Vec::new())
        .expect("list");
    drop(module);

    let actual = std::thread::scope(|scope| {
        let iter = values.iter();
        scope
            .spawn(move || iter.collect::<Vec<_>>())
            .join()
            .expect("iterator transfer")
    });
    assert_eq!(actual, expected);
    assert_eq!(values.get(999), Some(999.into()));
}
