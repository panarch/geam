use geam_core::__prepared_support as data;

#[path = "fixtures/prepared/compound_native_provider.rs"]
mod provider;

const GENERATED: data::HostedModuleArtifact =
    include!("fixtures/prepared/compound_native_calls.rs");
static ARTIFACT: data::HostedModuleArtifact = GENERATED;

#[cfg(feature = "tokio")]
mod bounded_native {
    use super::{ARTIFACT, GENERATED, data};
    use data::compiled::calls::{
        CallCustom, CallExecution, CallInputs, CallOps, CallProgress, CallStart, CallStorage,
        CallTuple, CustomNativeExecution, StringNativeExecution, TupleNativeExecution,
    };
    use data::compiled::{
        CallTarget, CompiledFunction, CompiledImplementation, FunctionCallsImplementation,
    };
    use geam_core::StringValue;
    use std::sync::Mutex;

    pub(super) static OBSERVATIONS: Mutex<Vec<(usize, usize, &'static str)>> =
        Mutex::new(Vec::new());
    struct Limited(Box<dyn CallExecution>);
    struct StringDelivery(Box<dyn StringNativeExecution>);
    struct CustomDelivery(Box<dyn CustomNativeExecution>);
    struct TupleDelivery(Box<dyn TupleNativeExecution>);

    fn advance(
        execution: Box<dyn CallExecution>,
        ops: &mut CallOps<'_>,
        budget: &mut usize,
    ) -> CallProgress {
        let offered = usize::from(!OBSERVATIONS.lock().unwrap().is_empty()).min(*budget);
        let mut remaining = offered;
        let progress = execution.advance(ops, &mut remaining);
        *budget -= offered - remaining;
        let boundary = match &progress {
            CallProgress::Yield(_) => "yield",
            CallProgress::Complete { .. } => "complete",
            CallProgress::StringNative(_) => "string native",
            CallProgress::CustomNative(_) => "custom native",
            CallProgress::TupleNative(_) => "tuple native",
            _ => "canonical",
        };
        OBSERVATIONS
            .lock()
            .unwrap()
            .push((offered, offered - remaining, boundary));
        match progress {
            CallProgress::Yield(next) => CallProgress::Yield(Box::new(Limited(next))),
            CallProgress::StringNative(mut request) => {
                request.execution = Box::new(StringDelivery(request.execution));
                CallProgress::StringNative(request)
            }
            CallProgress::CustomNative(mut request) => {
                request.execution = Box::new(CustomDelivery(request.execution));
                CallProgress::CustomNative(request)
            }
            CallProgress::TupleNative(mut request) => {
                request.execution = Box::new(TupleDelivery(request.execution));
                CallProgress::TupleNative(request)
            }
            CallProgress::Custom {
                function,
                site,
                arguments,
                resume,
            } => CallProgress::Custom {
                function,
                site,
                arguments,
                resume: Box::new(move |value| Box::new(Limited(resume(value)))),
            },
            CallProgress::Tuple {
                function,
                site,
                arguments,
                resume,
            } => CallProgress::Tuple {
                function,
                site,
                arguments,
                resume: Box::new(move |value| Box::new(Limited(resume(value)))),
            },
            progress => progress,
        }
    }
    impl CallExecution for Limited {
        fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
            self.0.restart(target, point, inputs)
        }
        fn retained_bytes(&self) -> usize {
            self.0.retained_bytes()
        }
        fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            advance(self.0, ops, budget)
        }
    }
    impl CallExecution for CustomDelivery {
        fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
            self.0.restart(target, point, inputs)
        }
        fn retained_bytes(&self) -> usize {
            self.0.retained_bytes()
        }
        fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            self.0.advance(ops, budget)
        }
    }
    impl CallExecution for StringDelivery {
        fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
            self.0.restart(target, point, inputs)
        }
        fn retained_bytes(&self) -> usize {
            self.0.retained_bytes()
        }
        fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            self.0.advance(ops, budget)
        }
    }
    impl StringNativeExecution for StringDelivery {
        fn resume_native(self: Box<Self>, value: StringValue) -> Box<dyn CallExecution> {
            Box::new(Limited(self.0.resume_native(value)))
        }
    }
    impl CustomNativeExecution for CustomDelivery {
        fn resume_native(self: Box<Self>, value: CallCustom) -> Box<dyn CallExecution> {
            Box::new(Limited(self.0.resume_native(value)))
        }
    }
    impl CallExecution for TupleDelivery {
        fn restart(&mut self, target: CallTarget, point: usize, inputs: CallInputs<'_>) -> bool {
            self.0.restart(target, point, inputs)
        }
        fn retained_bytes(&self) -> usize {
            self.0.retained_bytes()
        }
        fn advance(self: Box<Self>, ops: &mut CallOps<'_>, budget: &mut usize) -> CallProgress {
            self.0.advance(ops, budget)
        }
    }
    impl TupleNativeExecution for TupleDelivery {
        fn resume_native(self: Box<Self>, value: CallTuple) -> Box<dyn CallExecution> {
            Box::new(Limited(self.0.resume_native(value)))
        }
    }
    fn target(name: &str) -> CallTarget {
        let export = ARTIFACT
            .module
            .exports
            .iter()
            .find(|entry| entry.name.as_ref() == name)
            .unwrap();
        match export.signature.return_.as_ref() {
            data::type_::TypeMetadata::Custom(_) => {
                CallTarget::Custom(ARTIFACT.module.entries.customs[export.slot].function)
            }
            data::type_::TypeMetadata::String => {
                CallTarget::String(ARTIFACT.module.entries.strings[export.slot].function)
            }
            data::type_::TypeMetadata::Int => {
                CallTarget::Int(ARTIFACT.module.entries.ints[export.slot].function)
            }
            data::type_::TypeMetadata::Tuple(_) => {
                CallTarget::Tuple(ARTIFACT.module.entries.tuples[export.slot].function)
            }
            _ => panic!("the observed source entry returns String, Custom, Int or Tuple"),
        }
    }
    fn start(
        name: &str,
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        let target = target(name);
        let row = ARTIFACT
            .module
            .program
            .compiled
            .function_calls
            .iter()
            .find(|row| row.function == target)
            .unwrap();
        let body = observed_body(&row.implementation);
        (body.start)(point, inputs, storage)
            .map(|execution| Box::new(Limited(execution)) as Box<dyn CallExecution>)
    }
    fn walk(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start("walk", point, inputs, storage)
    }
    fn walk_pair(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start("walk_pair", point, inputs, storage)
    }
    fn walk_envelope(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start("walk_envelope", point, inputs, storage)
    }
    fn identity_tuple(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start("identity_tuple", point, inputs, storage)
    }
    fn all_fields(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start("all_fields", point, inputs, storage)
    }
    fn string_native_root(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start("string_native_root", point, inputs, storage)
    }
    fn string_native_after(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start("string_native_after", point, inputs, storage)
    }
    fn custom_native_root(
        point: usize,
        inputs: CallInputs<'_>,
        storage: &mut CallStorage,
    ) -> Option<Box<dyn CallExecution>> {
        start("custom_native_root", point, inputs, storage)
    }
    pub(super) fn artifact() -> &'static data::HostedModuleArtifact {
        let mut artifact = GENERATED;
        let replacements: [CallStart; 8] = [
            walk,
            walk_pair,
            walk_envelope,
            identity_tuple,
            all_fields,
            string_native_root,
            string_native_after,
            custom_native_root,
        ];
        let targets = [
            "walk",
            "walk_pair",
            "walk_envelope",
            "identity_tuple",
            "all_fields",
            "string_native_root",
            "string_native_after",
            "custom_native_root",
        ]
        .map(target);
        artifact.module.program.compiled.function_calls = ARTIFACT
            .module
            .program
            .compiled
            .function_calls
            .iter()
            .map(|row| {
                let body = observed_body(&row.implementation);
                CompiledFunction {
                    function: row.function,
                    implementation: CompiledImplementation::FunctionCalls(
                        Box::new(FunctionCallsImplementation {
                            root: body.root,
                            entry: body.entry,
                            checkpoints: body.checkpoints.clone(),
                            locals: body.locals.clone(),
                            calls: body.calls.clone(),
                            creations: body.creations.clone(),
                            returns: body.returns.clone(),
                            tails: body.tails.clone(),
                            start: targets
                                .iter()
                                .position(|target| *target == row.function)
                                .map_or(body.start, |slot| replacements[slot]),
                        })
                        .into(),
                    ),
                }
            })
            .collect();
        Box::leak(Box::new(artifact))
    }

    fn observed_body(implementation: &CompiledImplementation) -> &FunctionCallsImplementation {
        match implementation {
            CompiledImplementation::FunctionCalls(body) => body,
            _ => panic!("the observed fixture row must own generated calls"),
        }
    }

    #[test]
    #[should_panic(expected = "the observed source entry returns String, Custom, Int or Tuple")]
    fn observation_rejects_an_entry_outside_its_typed_families() {
        target("fieldless_control");
    }

    #[test]
    #[should_panic(expected = "the observed fixture row must own generated calls")]
    fn observation_rejects_a_numeric_kernel() {
        const NUMERIC: data::ModuleArtifact<data::profile::Plain> =
            include!("fixtures/prepared/numeric.rs");
        observed_body(&NUMERIC.program.compiled.ints[0].implementation);
    }
}

#[cfg(feature = "tokio")]
#[test]
fn generated_compound_loops_resume_at_zero_and_one_without_a_canonical_match() {
    use geam_core::embedding::{BigInt, FunctionDeclaration, StringValue};
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for continuing in [false, true] {
        for (name, native) in [
            ("walk", "custom native"),
            ("walk_pair", "tuple native"),
            ("walk_envelope", "custom native"),
        ] {
            let mut bindings = bounded_native::artifact()
                .load(provider::hosts(continuing))
                .unwrap();
            let function = bindings
                .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                    name,
                ))
                .unwrap();
            let mut module = bindings.seal();
            let mut audit = provider::Audit::default();
            bounded_native::OBSERVATIONS.lock().unwrap().clear();
            let result = runtime
                .block_on(module.with_execution(
                    &host,
                    &mut audit,
                    &mut Vec::new(),
                    async |scope| scope.call(&function, ("ab".into(), 5.into())).await,
                ))
                .unwrap()
                .try_into_value()
                .unwrap()
                .unwrap();
            assert_eq!(result, BigInt::from(7));
            assert_eq!(
                audit
                    .strings
                    .iter()
                    .map(|value| value.as_str().unwrap())
                    .collect::<Vec<_>>(),
                vec!["ab", "b", ""]
            );
            let observations = bounded_native::OBSERVATIONS.lock().unwrap();
            assert_eq!(observations[0], (0, 0, "yield"));
            assert!(
                observations
                    .iter()
                    .all(|(offered, consumed, _)| *offered <= 1 && consumed <= offered)
            );
            let asynchronous = continuing && name == "walk";
            assert_eq!(
                observations
                    .iter()
                    .filter(|(_, _, boundary)| *boundary == native)
                    .count(),
                if asynchronous { 0 } else { 3 },
                "{name}/{continuing}"
            );
            assert_eq!(
                observations
                    .iter()
                    .filter(|(_, _, boundary)| *boundary == "canonical")
                    .count(),
                if asynchronous { 3 } else { 0 },
                "{name}/{continuing}"
            );
            assert_eq!(observations.last().unwrap().2, "complete");
        }
    }
    // The same provider profile must carry String and compound requests. Both
    // a Native root and a caller continuation use zero/one generated grants.
    for name in ["string_native_root", "string_native_after"] {
        let mut bindings = bounded_native::artifact()
            .load(provider::hosts(false))
            .unwrap();
        let function = bindings
            .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
                name,
            ))
            .unwrap();
        let mut module = bindings.seal();
        let mut audit = provider::Audit::default();
        bounded_native::OBSERVATIONS.lock().unwrap().clear();
        let result = runtime
            .block_on(
                module.with_execution(&host, &mut audit, &mut Vec::new(), async |scope| {
                    scope.call(&function, ("ab".into(),)).await
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap();
        assert_eq!(result, StringValue::from("ab"));
        assert_eq!(audit.strings, [StringValue::from("ab")]);
        let observations = bounded_native::OBSERVATIONS.lock().unwrap();
        assert_eq!(observations[0], (0, 0, "yield"));
        assert_eq!(
            observations
                .iter()
                .filter(|(_, _, boundary)| *boundary == "string native")
                .count(),
            1
        );
        assert_eq!(
            observations
                .iter()
                .filter(|(_, _, boundary)| *boundary == "canonical")
                .count(),
            0
        );
        assert_eq!(observations.last().unwrap().2, "complete");
    }

    {
        use geam_core::embedding::CustomType;
        let mut bindings = bounded_native::artifact()
            .load(provider::hosts(false))
            .unwrap();
        let root = bindings
            .function(
                FunctionDeclaration::<(), CustomType<provider::MarkerType>>::new(
                    "custom_native_root",
                ),
            )
            .unwrap();
        let matches = bindings
            .function(FunctionDeclaration::<
                (CustomType<provider::MarkerType>,),
                bool,
            >::new("marker_matches"))
            .unwrap();
        let mut module = bindings.seal();
        let mut audit = provider::Audit::default();
        bounded_native::OBSERVATIONS.lock().unwrap().clear();
        let result = runtime
            .block_on(
                module.with_execution(&host, &mut audit, &mut Vec::new(), async |scope| {
                    let value = scope.call(&root, ()).await?;
                    scope.call(&matches, (value,)).await
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap();
        assert!(result);
        assert_eq!(audit.strings, [StringValue::from("marker")]);
        let observations = bounded_native::OBSERVATIONS.lock().unwrap();
        assert_eq!(observations[0], (0, 0, "yield"));
        assert_eq!(
            observations
                .iter()
                .filter(|(_, _, boundary)| *boundary == "custom native")
                .count(),
            1
        );
        assert_eq!(observations.last().unwrap().2, "complete");
    }

    for (name, calls) in [("identity_tuple", 1), ("all_fields", 2)] {
        let mut bindings = bounded_native::artifact()
            .load(provider::hosts(false))
            .unwrap();
        let function = bindings
            .function(FunctionDeclaration::<
                provider::Primitives,
                provider::Primitives,
            >::new(name))
            .unwrap();
        let mut module = bindings.seal();
        let expected = (
            17.into(),
            -0.0,
            false,
            (),
            '한',
            "text".into(),
            geam_core::BitArrayValue::from_bytes(vec![0xa5]),
        );
        bounded_native::OBSERVATIONS.lock().unwrap().clear();
        let result = runtime
            .block_on(module.with_execution(
                &host,
                &mut provider::Audit::default(),
                &mut Vec::new(),
                async |scope| scope.call(&function, expected.clone()).await,
            ))
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap();
        assert_eq!(result, expected);
        assert_eq!(result.1.to_bits(), (-0.0_f64).to_bits());
        let observations = bounded_native::OBSERVATIONS.lock().unwrap();
        assert_eq!(observations[0], (0, 0, "yield"));
        assert!(
            observations
                .iter()
                .all(|(offered, consumed, _)| *offered <= 1 && consumed <= offered)
        );
        assert_eq!(
            observations
                .iter()
                .filter(|(_, _, boundary)| *boundary == "tuple native")
                .count(),
            calls,
            "{name}"
        );
        assert_eq!(
            observations
                .iter()
                .filter(|(_, _, boundary)| *boundary == "canonical")
                .count(),
            0,
            "{name}"
        );
        assert_eq!(observations.last().unwrap().2, "complete");
    }
}

#[test]
fn compound_calls_match_the_maintained_public_preparation() {
    assert_eq!(
        provider::prepare().emit_rust().trim(),
        include_str!("fixtures/prepared/compound_native_calls.rs").trim()
    );
    let source = include_str!("fixtures/prepared/compound_native_calls.rs");
    assert!(source.contains("impl CustomNativeExecution for FunctionExecution"));
    assert!(source.contains("impl TupleNativeExecution for FunctionExecution"));
    assert!(source.contains(".matches_type(&data::type_::ValueType::Tuple"));
}

#[cfg(feature = "tokio")]
#[test]
fn fieldless_native_refinements_and_nil_projections_keep_their_public_results() {
    use geam_core::embedding::{BigInt, FunctionDeclaration, StringValue};
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        let (mut module, fieldless, empty_alias, nil_index, nil_field) = if prepared {
            let mut bindings = ARTIFACT.load(provider::hosts(false)).unwrap();
            let fieldless = bindings
                .function(FunctionDeclaration::<(), bool>::new("fieldless_control"))
                .unwrap();
            let empty_alias = bindings
                .function(FunctionDeclaration::<(StringValue,), bool>::new(
                    "empty_alias",
                ))
                .unwrap();
            let nil_index = bindings
                .function(FunctionDeclaration::<((BigInt, ()),), ()>::new("nil_index"))
                .unwrap();
            let nil_field = bindings
                .function(FunctionDeclaration::<(), ()>::new("nil_field_control"))
                .unwrap();
            (
                bindings.seal(),
                fieldless,
                empty_alias,
                nil_index,
                nil_field,
            )
        } else {
            let (mut bindings, fieldless) = provider::source(false)
                .function(FunctionDeclaration::<(), bool>::new("fieldless_control"))
                .unwrap();
            let empty_alias = bindings
                .function(FunctionDeclaration::<(StringValue,), bool>::new(
                    "empty_alias",
                ))
                .unwrap();
            let nil_index = bindings
                .function(FunctionDeclaration::<((BigInt, ()),), ()>::new("nil_index"))
                .unwrap();
            let nil_field = bindings
                .function(FunctionDeclaration::<(), ()>::new("nil_field_control"))
                .unwrap();
            (
                bindings.seal().unwrap(),
                fieldless,
                empty_alias,
                nil_index,
                nil_field,
            )
        };
        let mut audit = provider::Audit::default();
        let mut echo = Vec::new();
        let values = runtime
            .block_on(
                module.with_execution(&host, &mut audit, &mut echo, async |scope| {
                    Ok::<_, geam_core::embedding::CallError>((
                        scope.call(&fieldless, ()).await?,
                        scope.call(&empty_alias, ("!".into(),)).await?,
                        scope.call(&empty_alias, ("ab".into(),)).await?,
                        scope.call(&nil_index, ((42.into(), ()),)).await?,
                        scope.call(&nil_field, ()).await?,
                    ))
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap();
        assert_eq!(values, (true, true, false, (), ()));
        assert_eq!(audit.strings, ["marker", "!", "ab"].map(StringValue::from));
        assert!(echo.is_empty());
    }
}

#[cfg(feature = "tokio")]
#[test]
fn nested_compound_calls_keep_last_error_order_and_continuing_fallback() {
    use geam_core::embedding::{BigInt, FunctionDeclaration, StringValue};
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        for continuing in [false, true] {
            for name in ["walk", "walk_pair", "walk_envelope"] {
                let (mut module, function) = if prepared {
                    let mut bindings = ARTIFACT.load(provider::hosts(continuing)).unwrap();
                    let function = bindings
                        .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                            name,
                        ))
                        .unwrap();
                    (bindings.seal(), function)
                } else {
                    let (bindings, function) = provider::source(continuing)
                        .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                            name,
                        ))
                        .unwrap();
                    (bindings.seal().unwrap(), function)
                };
                let mut audit = provider::Audit::default();
                let mut echo = Vec::new();
                let result = runtime
                    .block_on(
                        module.with_execution(&host, &mut audit, &mut echo, async |scope| {
                            scope.call(&function, ("aéß".into(), 5.into())).await
                        }),
                    )
                    .unwrap()
                    .try_into_value()
                    .unwrap()
                    .unwrap();
                assert_eq!(result, BigInt::from(8), "{name}/{prepared}/{continuing}");
                assert_eq!(
                    audit
                        .strings
                        .iter()
                        .map(|value| value.as_str().unwrap())
                        .collect::<Vec<_>>(),
                    vec!["aéß", "éß", "ß", ""]
                );
                assert!(echo.is_empty());
            }
        }
    }
}

#[cfg(feature = "tokio")]
#[test]
fn literal_guard_fallthrough_alias_and_big_result_keep_the_source_behavior() {
    use geam_core::embedding::{BigInt, FunctionDeclaration};
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        let (mut module, function) = if prepared {
            let mut bindings = ARTIFACT.load(provider::hosts(false)).unwrap();
            let function = bindings
                .function(FunctionDeclaration::<(BigInt,), BigInt>::new("guarded"))
                .unwrap();
            (bindings.seal(), function)
        } else {
            let (bindings, function) = provider::source(false)
                .function(FunctionDeclaration::<(BigInt,), BigInt>::new("guarded"))
                .unwrap();
            (bindings.seal().unwrap(), function)
        };
        for (input, expected) in [(42, 7), (8, 16), (0, 0), (-1, -1), (-3, -3)] {
            let mut audit = provider::Audit::default();
            let result = runtime
                .block_on(module.with_execution(
                    &host,
                    &mut audit,
                    &mut Vec::new(),
                    async |scope| scope.call(&function, (input.into(),)).await,
                ))
                .unwrap()
                .try_into_value()
                .unwrap()
                .unwrap();
            assert_eq!(result, BigInt::from(expected));
            assert_eq!(audit.integers, vec![BigInt::from(input)]);
        }
        let wide: BigInt = BigInt::from(1_u8) << 130_usize;
        let mut audit = provider::Audit {
            integer_result: Some(wide.clone()),
            ..provider::Audit::default()
        };
        let result = runtime
            .block_on(
                module.with_execution(&host, &mut audit, &mut Vec::new(), async |scope| {
                    scope.call(&function, (5.into(),)).await
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap();
        assert_eq!(result, wide * 2);
        assert_eq!(
            audit.integers,
            vec![BigInt::from(5)],
            "a completed Native cannot be replayed at the Big Int match checkpoint"
        );
    }
}

#[cfg(feature = "tokio")]
#[test]
fn tuple_returns_and_custom_field_projections_preserve_all_seven_leaf_values() {
    use geam_core::embedding::{BitArrayValue, FunctionDeclaration};
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        for (name, added) in [
            ("identity_tuple", 0),
            ("all_fields", 0),
            ("custom_field_values", 2),
        ] {
            let (mut module, function) = if prepared {
                let mut bindings = ARTIFACT.load(provider::hosts(false)).unwrap();
                let function = bindings
                    .function(FunctionDeclaration::<
                        provider::Primitives,
                        provider::Primitives,
                    >::new(name))
                    .unwrap();
                (bindings.seal(), function)
            } else {
                let (bindings, function) = provider::source(false)
                    .function(FunctionDeclaration::<
                        provider::Primitives,
                        provider::Primitives,
                    >::new(name))
                    .unwrap();
                (bindings.seal().unwrap(), function)
            };
            let input: provider::Primitives = (
                17.into(),
                -0.0,
                true,
                (),
                '한',
                "text".into(),
                BitArrayValue::from_bytes(vec![0x12, 0x34]),
            );
            let mut audit = provider::Audit::default();
            let result = runtime
                .block_on(module.with_execution(
                    &host,
                    &mut audit,
                    &mut Vec::new(),
                    async |scope| scope.call(&function, input.clone()).await,
                ))
                .unwrap()
                .try_into_value()
                .unwrap()
                .unwrap();
            let mut expected = input;
            expected.0 += added;
            assert_eq!(result, expected);
            assert_eq!(result.1.to_bits(), (-0.0_f64).to_bits());
        }
    }
}

#[cfg(feature = "tokio")]
#[test]
fn native_tuple_float_assertions_accept_signed_zero_and_preserve_refutable_failure() {
    use geam_core::embedding::{BitArrayValue, CallError, FunctionDeclaration};
    use geam_core::execution::TokioHost;
    use geam_core::{ExecutionError, PanicKind};
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut source_results = Vec::new();
    let mut prepared_results = Vec::new();
    for prepared in [false, true] {
        let declaration =
            FunctionDeclaration::<provider::Primitives, bool>::new("assert_zero_float");
        let (mut module, function) = if prepared {
            let mut bindings = ARTIFACT.load(provider::hosts(false)).unwrap();
            let function = bindings.function(declaration).unwrap();
            (bindings.seal(), function)
        } else {
            let (bindings, function) = provider::source(false).function(declaration).unwrap();
            (bindings.seal().unwrap(), function)
        };
        for (fraction, expected) in [(0.0, true), (-0.0, true), (1.5, false), (f64::NAN, false)] {
            let input = (
                17.into(),
                fraction,
                true,
                (),
                '한',
                "text".into(),
                BitArrayValue::from_bytes(vec![7]),
            );
            let result = runtime
                .block_on(module.with_execution(
                    &host,
                    &mut provider::Audit::default(),
                    &mut Vec::new(),
                    async |scope| scope.call(&function, input).await,
                ))
                .unwrap()
                .try_into_value()
                .unwrap();
            if expected {
                assert_eq!(result, Ok(true));
            } else {
                assert!(
                    matches!(&result, Err(CallError::Execution(ExecutionError::Panic(panic)))
                    if panic.kind() == PanicKind::LetAssert)
                );
            }
            let observed = result.map_err(|error| format!("{error:?}"));
            if prepared {
                prepared_results.push(observed);
            } else {
                source_results.push(observed);
            }
        }
    }
    assert_eq!(prepared_results, source_results);
}

#[cfg(feature = "tokio")]
#[test]
fn native_errors_and_cancellation_preserve_the_completed_effect_prefix() {
    use geam_core::embedding::{BigInt, CallError, FunctionDeclaration, StringValue};
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut source_errors = Vec::new();
    let mut prepared_errors = Vec::new();
    for prepared in [false, true] {
        for continuing in [false, true] {
            for name in ["walk", "walk_pair", "walk_envelope"] {
                for (failure, cancelled, count) in [
                    (Some(1), None, 1),
                    (Some(2), None, 2),
                    (None, Some(1), 1),
                    (None, Some(2), 2),
                ] {
                    let (mut module, function) = if prepared {
                        let mut bindings = ARTIFACT.load(provider::hosts(continuing)).unwrap();
                        let function = bindings
                            .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                                name,
                            ))
                            .unwrap();
                        (bindings.seal(), function)
                    } else {
                        let (bindings, function) = provider::source(continuing)
                            .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                                name,
                            ))
                            .unwrap();
                        (bindings.seal().unwrap(), function)
                    };
                    let mut audit = provider::Audit {
                        fail_at: failure,
                        cancel_at: cancelled,
                        ..provider::Audit::default()
                    };
                    let result = runtime
                        .block_on(module.with_execution(
                            &host,
                            &mut audit,
                            &mut Vec::new(),
                            async |scope| scope.call(&function, ("abc".into(), 0.into())).await,
                        ))
                        .unwrap()
                        .try_into_value()
                        .unwrap();
                    if cancelled.is_some() {
                        assert_eq!(result, Err(CallError::Cancelled));
                    } else {
                        let Err(CallError::Execution(error)) = result else {
                            panic!("Native failure must keep its original error")
                        };
                        let error = error.to_string();
                        assert!(error.contains("compound native failure"));
                        if prepared {
                            prepared_errors.push(error);
                        } else {
                            source_errors.push(error);
                        }
                    }
                    assert_eq!(audit.strings.len(), count, "{name}/{prepared}/{continuing}");
                    assert_eq!(audit.strings[0].as_str().unwrap(), "abc");
                    if count == 2 {
                        assert_eq!(audit.strings[1].as_str().unwrap(), "bc");
                    }
                }
            }
        }
    }
    assert_eq!(
        prepared_errors, source_errors,
        "the original Native source origin must survive generated calls"
    );
}

#[cfg(feature = "tokio")]
#[test]
fn tuple_alias_escape_and_echo_handoff_do_not_replay_the_native_return() {
    use geam_core::embedding::{BigInt, FunctionDeclaration, StringValue};
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    for prepared in [false, true] {
        let (mut module, function) = if prepared {
            let mut bindings = ARTIFACT.load(provider::hosts(false)).unwrap();
            let function = bindings
                .function(FunctionDeclaration::<
                    (StringValue,),
                    (bool, (StringValue, StringValue)),
                >::new("aliased"))
                .unwrap();
            (bindings.seal(), function)
        } else {
            let (bindings, function) = provider::source(false)
                .function(FunctionDeclaration::<
                    (StringValue,),
                    (bool, (StringValue, StringValue)),
                >::new("aliased"))
                .unwrap();
            (bindings.seal().unwrap(), function)
        };
        let mut audit = provider::Audit::default();
        let result = runtime
            .block_on(
                module.with_execution(&host, &mut audit, &mut Vec::new(), async |scope| {
                    scope.call(&function, ("ab".into(),)).await
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap()
            .unwrap();
        assert_eq!(result, (true, ("a".into(), "b".into())));
        assert_eq!(audit.strings.len(), 1);
        for continuing in [false, true] {
            let (mut module, function) = if prepared {
                let mut bindings = ARTIFACT.load(provider::hosts(continuing)).unwrap();
                let function = bindings
                    .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                        "after_native",
                    ))
                    .unwrap();
                (bindings.seal(), function)
            } else {
                let (bindings, function) = provider::source(continuing)
                    .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                        "after_native",
                    ))
                    .unwrap();
                (bindings.seal().unwrap(), function)
            };
            let mut audit = provider::Audit::default();
            let mut echo = Vec::new();
            let result = runtime
                .block_on(
                    module.with_execution(&host, &mut audit, &mut echo, async |scope| {
                        scope.call(&function, ("ab".into(), 5.into())).await
                    }),
                )
                .unwrap()
                .try_into_value()
                .unwrap()
                .unwrap();
            assert_eq!(result, BigInt::from(6));
            assert_eq!(audit.strings.len(), 1);
            assert_eq!(echo.len(), 1);
            assert_eq!(echo[0].value().inspect().to_string(), "Ok(#(\"a\", \"b\"))");
        }
    }
}

#[cfg(feature = "tokio")]
#[test]
fn foreign_nominal_constructors_and_a_source_panic_keep_the_native_boundary() {
    use geam_core::embedding::{BigInt, CallError, FunctionDeclaration, StringValue};
    use geam_core::execution::TokioHost;
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut errors = Vec::new();
    for prepared in [false, true] {
        let (mut module, function) = if prepared {
            let mut bindings = ARTIFACT.load(provider::hosts(false)).unwrap();
            let function = bindings
                .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                    "walk_envelope",
                ))
                .unwrap();
            (bindings.seal(), function)
        } else {
            let (bindings, function) = provider::source(false)
                .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                    "walk_envelope",
                ))
                .unwrap();
            (bindings.seal().unwrap(), function)
        };
        for (input, expected) in [("", 5), ("!", -1), ("?", -2)] {
            let mut audit = provider::Audit::default();
            let result = runtime
                .block_on(module.with_execution(
                    &host,
                    &mut audit,
                    &mut Vec::new(),
                    async |scope| scope.call(&function, (input.into(), 5.into())).await,
                ))
                .unwrap()
                .try_into_value()
                .unwrap()
                .unwrap();
            assert_eq!(result, BigInt::from(expected));
            assert_eq!(audit.strings, vec![StringValue::from(input)]);
        }
        let (mut module, function) = if prepared {
            let mut bindings = ARTIFACT.load(provider::hosts(false)).unwrap();
            let function = bindings
                .function(FunctionDeclaration::<(StringValue,), BigInt>::new(
                    "after_native_panic",
                ))
                .unwrap();
            (bindings.seal(), function)
        } else {
            let (bindings, function) = provider::source(false)
                .function(FunctionDeclaration::<(StringValue,), BigInt>::new(
                    "after_native_panic",
                ))
                .unwrap();
            (bindings.seal().unwrap(), function)
        };
        let mut audit = provider::Audit::default();
        let result = runtime
            .block_on(
                module.with_execution(&host, &mut audit, &mut Vec::new(), async |scope| {
                    scope.call(&function, ("ab".into(),)).await
                }),
            )
            .unwrap()
            .try_into_value()
            .unwrap();
        let Err(CallError::Execution(error)) = result else {
            panic!("the source explicitly panics")
        };
        assert!(error.to_string().contains("after compound native"));
        errors.push(error.to_string());
        assert_eq!(audit.strings, vec![StringValue::from("ab")]);
    }
    assert_eq!(errors[0], errors[1]);
}

#[cfg(feature = "tokio")]
#[test]
fn rust_native_unwind_keeps_the_same_effect_prefix_and_closes_its_scope() {
    use geam_core::embedding::{BigInt, FunctionDeclaration, StringValue};
    use geam_core::execution::TokioHost;
    use std::panic::{AssertUnwindSafe, catch_unwind};
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    let mut messages = Vec::new();
    for prepared in [false, true] {
        for name in ["walk", "walk_pair", "walk_envelope"] {
            let (mut module, function) = if prepared {
                let mut bindings = ARTIFACT.load(provider::hosts(false)).unwrap();
                let function = bindings
                    .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                        name,
                    ))
                    .unwrap();
                (bindings.seal(), function)
            } else {
                let (bindings, function) = provider::source(false)
                    .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                        name,
                    ))
                    .unwrap();
                (bindings.seal().unwrap(), function)
            };
            let mut audit = provider::Audit {
                panic_at: Some(2),
                ..provider::Audit::default()
            };
            let panic = catch_unwind(AssertUnwindSafe(|| {
                runtime.block_on(module.with_execution(
                    &host,
                    &mut audit,
                    &mut Vec::new(),
                    async |scope| scope.call(&function, ("abc".into(), 0.into())).await,
                ))
            }))
            .unwrap_err();
            let message = *panic.downcast::<String>().unwrap();
            assert!(message.contains("compound native Rust panic"));
            messages.push(message);
            assert_eq!(
                audit.strings,
                vec![StringValue::from("abc"), StringValue::from("bc")]
            );
            assert!(!audit.unit.as_ref().unwrap().is_active());
        }
    }
    assert_eq!(&messages[..3], &messages[3..]);
}
