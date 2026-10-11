use geam_core::__prepared_support as data;
use geam_core::embedding::{BigInt, CallError, FunctionDeclaration, ModuleBuilder};
use geam_core::{ExecutionError, PanicKind, PanicMessage, compile_typed_module};
use std::convert::Infallible;

#[cfg(feature = "tokio")]
#[path = "support/work_representation.rs"]
mod work_representation;

static CUSTOM: data::ModuleArtifact<Infallible> =
    include!("fixtures/prepared/generated/custom_scalars.rs");
const SOURCE: &str = include_str!("fixtures/prepared/custom_scalars.gleam");

macro_rules! functions {
    ($bindings:ident, $credit:expr) => {
        (
            $credit,
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                    "debit",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt,), BigInt>::new("ignored"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
                    "guarded",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                    "aliased",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
                    "multiple",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt, bool), bool>::new(
                    "boolean",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
                    "fields",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                    "repeated",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
                    "assertion",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                    "panic_case",
                ))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(BigInt,), BigInt>::new("nested"))
                .unwrap(),
            $bindings
                .function(FunctionDeclaration::<(), BigInt>::new("main"))
                .unwrap(),
        )
    };
}

#[test]
fn generated_custom_inputs_preserve_matches_guards_aliases_fields_and_big_values() {
    assert!(CUSTOM.program.compiled.callbacks.ints.is_empty());
    assert!(CUSTOM.program.compiled.callbacks.bools.is_empty());
    assert!(CUSTOM.program.compiled.ints.iter().all(|target| !matches!(
        target.implementation,
        data::compiled::CompiledImplementation::CustomLoop(_)
    )));
    assert!(CUSTOM.program.compiled.customs.is_empty());

    let mut canonical_panics = Vec::new();
    for prepared in [false, true] {
        let mut panics = Vec::new();
        let (module, handles) = if prepared {
            let mut bindings = CUSTOM.load().unwrap();
            let credit = bindings
                .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                    "credit",
                ))
                .unwrap();
            let handles = functions!(bindings, credit);
            (bindings.seal(), handles)
        } else {
            let typed =
                compile_typed_module("example", "src/custom_scalars.gleam", SOURCE).unwrap();
            let (mut bindings, credit) = ModuleBuilder::new(typed)
                .unwrap()
                .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                    "credit",
                ))
                .unwrap();
            let handles = functions!(bindings, credit);
            (bindings.seal(), handles)
        };
        let (
            credit,
            debit,
            ignored,
            guarded,
            aliased,
            multiple,
            boolean,
            fields,
            repeated,
            assertion,
            panic_case,
            nested,
            main,
        ) = handles;
        let big: BigInt = BigInt::from(1) << 180;
        for (amount, total) in [
            (3.into(), 7.into()),
            (i64::MAX.into(), 1.into()),
            (big.clone(), 7.into()),
            (3.into(), big.clone()),
        ] {
            assert_eq!(
                module
                    .call(&credit, (amount.clone(), total.clone()), &mut Vec::new())
                    .unwrap(),
                &total + &amount
            );
            assert_eq!(
                module
                    .call(&debit, (amount.clone(), total.clone()), &mut Vec::new())
                    .unwrap(),
                &total - &amount
            );
        }
        assert_eq!(
            module
                .call(&ignored, (big.clone(),), &mut Vec::new())
                .unwrap(),
            big
        );
        for (left, right, enabled, expected) in [
            (3, 4, true, 8),
            (3, -4, true, -4),
            (5, 9, true, 4),
            (5, 9, false, 9),
            (3, 4, false, 4),
        ] {
            assert_eq!(
                module
                    .call(
                        &guarded,
                        (left.into(), right.into(), enabled),
                        &mut Vec::new()
                    )
                    .unwrap(),
                BigInt::from(expected)
            );
        }
        assert_eq!(
            module
                .call(&guarded, (3.into(), big.clone(), true), &mut Vec::new())
                .unwrap(),
            &big * 2
        );
        assert_eq!(
            module
                .call(&guarded, (big.clone(), 9.into(), true), &mut Vec::new())
                .unwrap(),
            &big - 1
        );
        assert_eq!(
            module
                .call(&aliased, (big.clone(), 5.into()), &mut Vec::new())
                .unwrap(),
            &big + 10
        );
        for enabled in [false, true] {
            assert_eq!(
                module
                    .call(&multiple, (4.into(), 6.into(), enabled), &mut Vec::new())
                    .unwrap(),
                BigInt::from(if enabled { 10 } else { -4 })
            );
        }
        for (left, right, enabled, expected) in [
            (3, 2, true, true),
            (3, 0, true, false),
            (4, 0, true, true),
            (4, 1, false, false),
        ] {
            assert_eq!(
                module
                    .call(
                        &boolean,
                        (left.into(), right.into(), enabled),
                        &mut Vec::new()
                    )
                    .unwrap(),
                expected
            );
        }
        assert_eq!(
            module
                .call(&fields, (12.into(), 7.into(), true), &mut Vec::new())
                .unwrap(),
            BigInt::from(7)
        );
        assert_eq!(
            module
                .call(&fields, (12.into(), 7.into(), false), &mut Vec::new())
                .unwrap(),
            BigInt::from(5)
        );
        assert_eq!(
            module
                .call(&fields, (big.clone(), 7.into(), true), &mut Vec::new())
                .unwrap(),
            &big / 3 + 3
        );
        assert_eq!(
            module
                .call(&repeated, (3.into(), 4000.into()), &mut Vec::new())
                .unwrap(),
            BigInt::from(12000)
        );
        assert_eq!(
            module
                .call(&repeated, (big.clone(), 3.into()), &mut Vec::new())
                .unwrap(),
            &big * 3
        );
        assert_eq!(
            module
                .call(&assertion, (3.into(), 9.into(), true), &mut Vec::new())
                .unwrap(),
            BigInt::from(12)
        );
        for (left, right, enabled) in [
            (3.into(), 8.into(), true),
            (big.clone(), 9.into(), true),
            (3.into(), 9.into(), false),
        ] {
            let error = module
                .call(&assertion, (left, right, enabled), &mut Vec::new())
                .unwrap_err()
                .into_materialized();
            let CallError::Execution(ExecutionError::Panic(panic)) = error else {
                panic!("assertion error");
            };
            assert_eq!(panic.kind(), PanicKind::LetAssert);
            assert_eq!(panic.site().function(), "asserted");
            panics.push(panic);
        }
        let error = module
            .call(&panic_case, (3.into(), 7.into()), &mut Vec::new())
            .unwrap_err()
            .into_materialized();
        let CallError::Execution(ExecutionError::Panic(panic)) = error else {
            panic!("source panic");
        };
        assert_eq!(panic.site().function(), "stop");
        assert_eq!(
            panic.message(),
            &PanicMessage::Explicit("custom stop".into())
        );
        panics.push(panic);
        assert_eq!(
            module
                .call(&panic_case, (4.into(), 7.into()), &mut Vec::new())
                .unwrap(),
            BigInt::from(7)
        );
        assert_eq!(
            module
                .call(&nested, (big.clone(),), &mut Vec::new())
                .unwrap(),
            big
        );
        assert_eq!(
            module.call(&main, (), &mut Vec::new()).unwrap(),
            BigInt::from(100)
        );
        if prepared {
            // Compare complete source spans, messages and let-assert subjects.
            assert_eq!(panics, canonical_panics);
        } else {
            canonical_panics = panics;
        }
    }
}

#[cfg(feature = "tokio")]
#[test]
fn custom_generated_standalone_entry_has_the_same_source_free_execution_links() {
    use geam_core::HostProviderSet;
    use geam_core::execution::TokioHost;
    use geam_core::host::{HostComponentProfile, HostFutureStore, HostProfile, HostWorkProfile};
    use work_representation::WorkComponent;

    struct Profile;
    impl HostProfile for Profile {
        type RunState = ();
        type ExternalStores = HostFutureStore;
        type ExecutionState = ();
    }
    impl HostWorkProfile for Profile {
        type Work = WorkComponent;
    }
    impl HostComponentProfile<WorkComponent> for Profile {
        fn component_stores(stores: &HostFutureStore) -> &HostFutureStore {
            stores
        }
        fn component_state(state: &mut ()) -> &mut () {
            state
        }
    }

    static ENTRY: data::HostedEntryArtifact =
        include!("fixtures/prepared/generated/custom_scalars_entry.rs");
    assert!(ENTRY.program.compiled.callbacks.ints.is_empty());
    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut entry = ENTRY
        .load(HostProviderSet::<Profile>::new([]).unwrap())
        .unwrap();
    let mut echo = Vec::new();
    runtime
        .block_on(entry.run(&host, &mut (), &mut echo))
        .unwrap();
    assert!(echo.is_empty());
}
