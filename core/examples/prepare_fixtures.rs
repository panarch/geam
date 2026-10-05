use geam_core::embedding::{
    BigInt, BitArrayValue, FunctionDeclaration, HostedModuleBuilder, List, ModuleBuilder,
    StringValue,
};
use geam_core::{
    HostProviderSet, ModuleSource, PackageSource, PreparedHostedEntry, StatelessHostProfile,
};
use std::error::Error;
use std::path::Path;

#[path = "../tests/fixtures/prepared/list_provider.rs"]
mod list_provider;

#[path = "../tests/fixtures/prepared/native_provider.rs"]
mod native_provider;
#[path = "../tests/support/work_fixture.rs"]
mod work_fixture;
#[path = "../tests/fixtures/prepared/work_provider.rs"]
mod work_provider;

#[path = "../tests/fixtures/prepared/callable_declarations.rs"]
mod callable_declarations;

#[path = "../tests/fixtures/prepared/shared_provider.rs"]
mod shared_provider;

#[path = "../tests/fixtures/prepared/opaque_provider.rs"]
mod opaque_provider;

#[path = "../tests/fixtures/prepared/function_value_provider.rs"]
mod function_value_provider;

fn main() -> Result<(), Box<dyn Error>> {
    let arithmetic = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        include_str!("../tests/fixtures/prepared/arithmetic.gleam"),
    )?;
    let (arithmetic, _) =
        ModuleBuilder::new(arithmetic)?.function(FunctionDeclaration::<(), BigInt>::new("main"))?;

    let numeric_switch = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        include_str!("../tests/fixtures/prepared/numeric_switch.gleam"),
    )?;
    let (numeric_switch, _) = ModuleBuilder::new(numeric_switch)?
        .function(FunctionDeclaration::<(BigInt,), BigInt>::new("choose"))?;

    let numeric = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        include_str!("../tests/fixtures/prepared/numeric.gleam"),
    )?;
    let (mut numeric, _) = ModuleBuilder::new(numeric)?
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
            "arithmetic",
        ))?;
    numeric
        .function(FunctionDeclaration::<(BigInt, BigInt, bool, BigInt), BigInt>::new("shuffle"))?;
    numeric.function(FunctionDeclaration::<(BigInt, bool), bool>::new("choice"))?;
    numeric.function(FunctionDeclaration::<(BigInt,), BigInt>::new("switch"))?;
    numeric.function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
        "quotient",
    ))?;
    numeric.function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
        "operators",
    ))?;
    numeric.function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
        "divmod",
    ))?;
    numeric.function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
        "product",
    ))?;
    numeric.function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
        "captured",
    ))?;
    numeric.function(FunctionDeclaration::<
        (BigInt, BigInt, StringValue),
        (StringValue, BigInt, List<BigInt>, BigInt),
    >::new("caller"))?;
    numeric.function(FunctionDeclaration::<(), BigInt>::new("main"))?;
    numeric.function(FunctionDeclaration::<(BigInt, bool), BigInt>::new(
        "discarded",
    ))?;

    let hosted_numeric = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/example.gleam",
                include_str!("../tests/fixtures/prepared/numeric.gleam"),
            )],
        )],
        HostProviderSet::<work_provider::Profile>::new([])?,
    )?;
    let (mut hosted_numeric, _) =
        HostedModuleBuilder::new(hosted_numeric)?
            .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
                "arithmetic",
            ))?;
    hosted_numeric.function(FunctionDeclaration::<(BigInt, bool), bool>::new("choice"))?;
    hosted_numeric.function(FunctionDeclaration::<
        (BigInt, BigInt, StringValue),
        (StringValue, BigInt, List<BigInt>, BigInt),
    >::new("caller"))?;
    hosted_numeric.function(FunctionDeclaration::<(), BigInt>::new("main"))?;
    hosted_numeric.function(FunctionDeclaration::<(), BigInt>::new("running"))?;

    let numeric_entry = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/numeric_entry.gleam",
                include_str!("../tests/fixtures/prepared/numeric_entry.gleam"),
            )],
        )],
        HostProviderSet::<work_provider::Profile>::new([])?,
    )?;
    let numeric_entry =
        PreparedHostedEntry::try_from_module_plan(geam_core::plan_host_program(numeric_entry)?)?;

    let int_list = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        include_str!("../tests/fixtures/prepared/int_list.gleam"),
    )?;
    let (mut int_list, _) = ModuleBuilder::new(int_list)?
        .function(FunctionDeclaration::<(List<BigInt>, BigInt), BigInt>::new(
            "count",
        ))?;
    int_list.function(FunctionDeclaration::<(List<BigInt>, BigInt), BigInt>::new(
        "asserted",
    ))?;
    int_list.function(FunctionDeclaration::<
        (List<BigInt>, List<BigInt>, BigInt),
        BigInt,
    >::new("equal_walk"))?;
    int_list.function(FunctionDeclaration::<(List<BigInt>,), BigInt>::new(
        "prefix",
    ))?;
    int_list.function(FunctionDeclaration::<
        (List<BigInt>, List<BigInt>, bool),
        bool,
    >::new("same"))?;
    int_list.function(FunctionDeclaration::<
        (List<BigInt>, List<BigInt>, BigInt),
        BigInt,
    >::new("shuffle"))?;
    int_list.function(FunctionDeclaration::<
        (List<BigInt>, List<BigInt>, BigInt),
        BigInt,
    >::new("duplicate"))?;
    int_list.function(FunctionDeclaration::<(List<BigInt>, BigInt), BigInt>::new(
        "captured",
    ))?;
    int_list.function(FunctionDeclaration::<(List<BigInt>,), BigInt>::new("stop"))?;
    int_list.function(FunctionDeclaration::<(), BigInt>::new("main"))?;
    int_list.function(FunctionDeclaration::<(List<BigInt>,), BigInt>::new("late"))?;

    let hosted_int_list = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/example.gleam",
                include_str!("../tests/fixtures/prepared/int_list.gleam"),
            )],
        )],
        HostProviderSet::<work_provider::Profile>::new([])?,
    )?;
    let (mut hosted_int_list, _) =
        HostedModuleBuilder::new(hosted_int_list)?.function(FunctionDeclaration::<
            (List<BigInt>, BigInt, StringValue),
            (StringValue, BigInt, List<BigInt>, BigInt),
        >::new("caller"))?;
    hosted_int_list.function(FunctionDeclaration::<(), BigInt>::new("running"))?;

    let int_list_entry = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/example.gleam",
                include_str!("../tests/fixtures/prepared/int_list.gleam"),
            )],
        )],
        HostProviderSet::<work_provider::Profile>::new([])?,
    )?;
    let int_list_entry =
        PreparedHostedEntry::try_from_module_plan(geam_core::plan_host_program(int_list_entry)?)?;

    let bit_entry = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/bit_array_entry.gleam",
                include_str!("../tests/fixtures/prepared/bit_array_entry.gleam"),
            )],
        )],
        HostProviderSet::<work_provider::Profile>::new([])?,
    )?;
    let bit_entry =
        PreparedHostedEntry::try_from_module_plan(geam_core::plan_host_program(bit_entry)?)?;

    let construction_source = include_str!("../tests/fixtures/prepared/list_construction.gleam");
    let construction =
        geam_core::compile_typed_module("example", "src/example.gleam", construction_source)?;
    let (mut construction, _) = ModuleBuilder::new(construction)?
        .function(FunctionDeclaration::<(), List<BigInt>>::new("empty"))?;
    construction.function(FunctionDeclaration::<
        (BigInt, BigInt, List<BigInt>),
        List<BigInt>,
    >::new("prefix"))?;
    construction.function(FunctionDeclaration::<
        (bool, BigInt, BigInt, List<BigInt>, List<BigInt>),
        List<BigInt>,
    >::new("choose"))?;
    construction.function(FunctionDeclaration::<(List<BigInt>,), List<BigInt>>::new(
        "reverse",
    ))?;
    construction.function(FunctionDeclaration::<
        (List<BigInt>, List<BigInt>),
        List<BigInt>,
    >::new("selected_reverse"))?;
    construction
        .function(FunctionDeclaration::<(BigInt, List<BigInt>), List<BigInt>>::new("promoted"))?;
    construction.function(FunctionDeclaration::<
        (BigInt, List<BigInt>, bool),
        List<BigInt>,
    >::new("interpreted_tail"))?;
    construction.function(FunctionDeclaration::<(), List<BigInt>>::new("main"))?;
    construction.function(FunctionDeclaration::<(bool,), List<BigInt>>::new(
        "numeric_tail",
    ))?;
    let construction_entry = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/example.gleam",
                construction_source,
            )],
        )],
        HostProviderSet::<work_provider::Profile>::new([])?,
    )?;
    let construction_entry = PreparedHostedEntry::try_from_module_plan(
        geam_core::plan_host_program(construction_entry)?,
    )?;
    let construction_hosted = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/example.gleam",
                construction_source,
            )],
        )],
        HostProviderSet::<work_provider::Profile>::new([])?,
    )?;
    let (mut construction_hosted, _) =
        HostedModuleBuilder::new(construction_hosted)?.function(FunctionDeclaration::<
            (List<BigInt>, BigInt, StringValue),
            (StringValue, BigInt, List<BigInt>, List<BigInt>),
        >::new("caller"))?;
    construction_hosted.function(FunctionDeclaration::<(), List<BigInt>>::new("running"))?;
    construction_hosted.function(FunctionDeclaration::<(bool,), List<BigInt>>::new(
        "numeric_tail",
    ))?;
    let list_native = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/example.gleam",
                include_str!("../tests/fixtures/prepared/list_native.gleam"),
            )],
        )],
        list_provider::hosts(),
    )?;
    let (mut list_native, _) =
        HostedModuleBuilder::new(list_native)?.function(FunctionDeclaration::<
            (BigInt, List<BigInt>, bool),
            List<BigInt>,
        >::new("native_tail"))?;
    list_native.function(FunctionDeclaration::<
        (BigInt, List<BigInt>, StringValue, bool),
        (StringValue, BigInt, List<BigInt>, List<BigInt>),
    >::new("caller"))?;

    let values = geam_core::compile_typed_program(
        "example",
        [ModuleSource::new(
            "example",
            "src/example.gleam",
            include_str!("../tests/fixtures/prepared/values.gleam"),
        )],
    )?;
    let (mut values, _) = ModuleBuilder::from_program(values)?
        .function(FunctionDeclaration::<(), BigInt>::new("run"))?;
    values.function(FunctionDeclaration::<(), BigInt>::new("fail"))?;
    values.function(FunctionDeclaration::<(BigInt,), BigInt>::new("assertion"))?;

    let patterns = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        include_str!("../tests/fixtures/prepared/nested_patterns.gleam"),
    )?;
    let (patterns, _) = ModuleBuilder::new(patterns)?
        .function(FunctionDeclaration::<(), StringValue>::new("main"))?;

    let symbolic_patterns = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        include_str!("../tests/fixtures/prepared/symbolic_patterns.gleam"),
    )?;
    let (symbolic_patterns, _) = ModuleBuilder::new(symbolic_patterns)?
        .function(FunctionDeclaration::<(), ()>::new("main"))?;

    let multi_subject = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        include_str!("../tests/fixtures/prepared/multi_subject_patterns.gleam"),
    )?;
    let (multi_subject, _) = ModuleBuilder::new(multi_subject)?
        .function(FunctionDeclaration::<(), StringValue>::new("main"))?;

    let sparse = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        include_str!("../tests/fixtures/prepared/sparse_patterns.gleam"),
    )?;
    let (sparse, _) = ModuleBuilder::new(sparse)?
        .function(FunctionDeclaration::<(), StringValue>::new("main"))?;

    let bit_arrays = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        include_str!("../tests/fixtures/prepared/bit_array_patterns.gleam"),
    )?;
    let (mut bit_arrays, _) = ModuleBuilder::new(bit_arrays)?.function(FunctionDeclaration::<
        (BitArrayValue, BigInt),
        (BigInt, f64, BitArrayValue, BigInt),
    >::new("zero_fields"))?;
    bit_arrays.function(
        FunctionDeclaration::<(BitArrayValue, BigInt, BigInt), BigInt>::new("signed_little"),
    )?;
    bit_arrays.function(FunctionDeclaration::<(BitArrayValue,), BigInt>::new(
        "dependent_fields",
    ))?;
    bit_arrays.function(FunctionDeclaration::<(BitArrayValue,), BigInt>::new(
        "fixed_fields",
    ))?;
    bit_arrays.function(FunctionDeclaration::<(BitArrayValue,), BigInt>::new(
        "fixed_failure",
    ))?;

    let bit_loops = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        include_str!("../tests/fixtures/prepared/bit_array_loops.gleam"),
    )?;
    let (mut bit_loops, _) = ModuleBuilder::new(bit_loops)?
        .function(FunctionDeclaration::<(BitArrayValue, BigInt), BigInt>::new(
            "checksum",
        ))?;
    bit_loops.function(FunctionDeclaration::<
        (BitArrayValue, BigInt, BigInt),
        Result<BigInt, ()>,
    >::new("parse"))?;
    for name in ["wide", "aliases", "little"] {
        bit_loops.function(FunctionDeclaration::<(BitArrayValue, BigInt), BigInt>::new(
            name,
        ))?;
    }
    bit_loops.function(FunctionDeclaration::<(BitArrayValue,), BigInt>::new(
        "late_failure",
    ))?;
    bit_loops.function(FunctionDeclaration::<
        (BitArrayValue, BitArrayValue, BigInt),
        BigInt,
    >::new("paired"))?;
    bit_loops.function(FunctionDeclaration::<(BitArrayValue, bool), bool>::new(
        "toggle",
    ))?;

    let native = geam_core::compile_typed_host_program(
        "application",
        "main",
        [PackageSource::new(
            "application",
            Vec::<&str>::new(),
            [ModuleSource::new(
                "main",
                "src/main.gleam",
                include_str!("../tests/fixtures/prepared/native.gleam"),
            )],
        )],
        native_provider::hosts(),
    )?;
    let (mut native, _) = HostedModuleBuilder::new(native)?
        .function(FunctionDeclaration::<(), (bool, bool, BigInt)>::new("run"))?;
    native
        .function(FunctionDeclaration::<(StringValue,), (bool, StringValue)>::new("substring"))?;
    native.function(FunctionDeclaration::<
        (BitArrayValue, BigInt, BigInt),
        BitArrayValue,
    >::new("bit_range"))?;
    native.function(FunctionDeclaration::<(BitArrayValue,), BitArrayValue>::new(
        "bit_tail",
    ))?;
    native.function(FunctionDeclaration::<(), bool>::new("generic_results"))?;
    native.function(FunctionDeclaration::<(List<BigInt>, BigInt), BigInt>::new(
        "list_callback",
    ))?;

    let string_source = include_str!("../tests/fixtures/prepared/string_ranges.gleam");
    let string_typed =
        geam_core::compile_typed_module("example", "src/example.gleam", string_source)?;
    let (mut string_ranges, _) =
        ModuleBuilder::new(string_typed)?
            .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                "count",
            ))?;
    string_ranges.function(FunctionDeclaration::<(StringValue,), BigInt>::new("select"))?;
    string_ranges.function(FunctionDeclaration::<
        (StringValue, StringValue, bool, BigInt),
        BigInt,
    >::new("aliases"))?;
    string_ranges.function(FunctionDeclaration::<
        (StringValue, StringValue, BigInt),
        BigInt,
    >::new("alternate"))?;
    string_ranges
        .function(FunctionDeclaration::<(StringValue, StringValue, bool), bool>::new("same"))?;
    string_ranges.function(FunctionDeclaration::<(StringValue,), bool>::new(
        "empty_prefix",
    ))?;
    string_ranges.function(FunctionDeclaration::<(), BigInt>::new("literal_only"))?;
    string_ranges.function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
        "asserted",
    ))?;
    string_ranges.function(FunctionDeclaration::<
        (StringValue, BigInt),
        (StringValue, BigInt, List<BigInt>, bool),
    >::new("caller"))?;
    string_ranges.function(FunctionDeclaration::<(StringValue,), BigInt>::new(
        "unsupported",
    ))?;
    string_ranges.function(FunctionDeclaration::<(StringValue,), BigInt>::new("spin"))?;
    string_ranges.function(FunctionDeclaration::<(), BigInt>::new("main"))?;
    for name in ["assert_literal", "assert_prefix"] {
        string_ranges.function(FunctionDeclaration::<(StringValue,), BigInt>::new(name))?;
    }
    string_ranges.function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
        "assert_suffix",
    ))?;
    string_ranges.function(
        FunctionDeclaration::<(BitArrayValue, bool, bool), BigInt>::new("bits_with_boolean_guard"),
    )?;
    let string_hosted = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/example.gleam",
                string_source,
            )],
        )],
        HostProviderSet::<work_provider::Profile>::new([])?,
    )?;
    let (mut string_hosted, _) =
        HostedModuleBuilder::new(string_hosted)?
            .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                "count",
            ))?;
    string_hosted.function(FunctionDeclaration::<
        (StringValue, StringValue, bool, BigInt),
        BigInt,
    >::new("aliases"))?;
    string_hosted
        .function(FunctionDeclaration::<(StringValue, StringValue, bool), bool>::new("same"))?;
    string_hosted.function(FunctionDeclaration::<(StringValue,), bool>::new(
        "empty_prefix",
    ))?;
    string_hosted.function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
        "asserted",
    ))?;
    string_hosted.function(FunctionDeclaration::<
        (StringValue, BigInt),
        (StringValue, BigInt, List<BigInt>, bool),
    >::new("caller"))?;
    string_hosted.function(FunctionDeclaration::<(StringValue,), BigInt>::new(
        "running",
    ))?;
    let string_entry = PreparedHostedEntry::try_from_module_plan(geam_core::plan_host_program(
        geam_core::compile_typed_host_program(
            "example",
            "example",
            [PackageSource::new(
                "example",
                Vec::<String>::new(),
                [ModuleSource::new(
                    "example",
                    "src/example.gleam",
                    string_source,
                )],
            )],
            HostProviderSet::<work_provider::Profile>::new([])?,
        )?,
    )?)?;

    let checkpoint_source = include_str!("../tests/fixtures/prepared/string_checkpoints.gleam");
    let checkpoint_typed =
        geam_core::compile_typed_module("example", "src/example.gleam", checkpoint_source)?;
    let (checkpoint_assertion, _) =
        ModuleBuilder::new(checkpoint_typed)?
            .function(FunctionDeclaration::<(StringValue, BigInt), BigInt>::new(
                "after_step",
            ))?;
    let checkpoint_typed =
        geam_core::compile_typed_module("example", "src/example.gleam", checkpoint_source)?;
    let (mut checkpoint_stops, _) = ModuleBuilder::new(checkpoint_typed)?
        .function(FunctionDeclaration::<(StringValue,), BigInt>::new("stop"))?;
    checkpoint_stops.function(FunctionDeclaration::<(StringValue,), List<BigInt>>::new(
        "list_stop",
    ))?;
    let checkpoint_hosted = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/example.gleam",
                checkpoint_source,
            )],
        )],
        HostProviderSet::<StatelessHostProfile>::new([])?,
    )?;
    let (checkpoint_hosted, _) =
        HostedModuleBuilder::new(checkpoint_hosted)?
            .function(FunctionDeclaration::<(StringValue,), List<BigInt>>::new(
                "list_stop",
            ))?;
    let calls = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        include_str!("../tests/fixtures/prepared/function_calls.gleam"),
    )?;
    let (mut calls, _) = ModuleBuilder::new(calls)?
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
            "capture_chain",
        ))?;
    calls.function(FunctionDeclaration::<(bool, BigInt), BigInt>::new(
        "dynamic_target",
    ))?;
    calls.function(FunctionDeclaration::<(BigInt,), BigInt>::new("nested"))?;
    calls.function(FunctionDeclaration::<(BigInt,), bool>::new("mutual"))?;
    calls.function(FunctionDeclaration::<(BigInt, BigInt), bool>::new(
        "callable_captures",
    ))?;
    calls.function(FunctionDeclaration::<(BigInt, BigInt), bool>::new(
        "aliases",
    ))?;
    calls.function(FunctionDeclaration::<(BigInt,), BigInt>::new("canonical"))?;
    calls.function(FunctionDeclaration::<(BigInt,), BigInt>::new("big_return"))?;
    calls.function(FunctionDeclaration::<(BigInt,), BigInt>::new("failure"))?;
    calls.function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
        "producer_suffix_int",
    ))?;
    calls.function(FunctionDeclaration::<(BigInt, BigInt), bool>::new(
        "producer_suffix_bool",
    ))?;

    calls.function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
        "reuse_callback",
    ))?;
    calls
        .function(FunctionDeclaration::<(BigInt, BigInt, BigInt), BigInt>::new("repeated_roots"))?;
    calls.function(FunctionDeclaration::<(BigInt,), bool>::new(
        "canonical_bool",
    ))?;
    calls.function(FunctionDeclaration::<(bool, BigInt, BigInt), bool>::new(
        "bool_captures",
    ))?;

    let boolean_calls = geam_core::compile_typed_module(
        "example",
        "src/example.gleam",
        include_str!("../tests/fixtures/prepared/boolean_calls.gleam"),
    )?;
    let (boolean_calls, _) = ModuleBuilder::new(boolean_calls)?
        .function(FunctionDeclaration::<(bool,), bool>::new("flip"))?;

    let custom_source = include_str!("../tests/fixtures/prepared/custom_scalars.gleam");
    let custom =
        geam_core::compile_typed_module("example", "src/custom_scalars.gleam", custom_source)?;
    let (mut custom, _) = ModuleBuilder::new(custom)?
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
            "credit",
        ))?;
    custom.function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
        "debit",
    ))?;
    custom.function(FunctionDeclaration::<(BigInt,), BigInt>::new("ignored"))?;
    custom.function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
        "guarded",
    ))?;
    custom.function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
        "aliased",
    ))?;
    custom.function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
        "multiple",
    ))?;
    custom.function(FunctionDeclaration::<(BigInt, BigInt, bool), bool>::new(
        "boolean",
    ))?;
    custom.function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
        "fields",
    ))?;
    custom.function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
        "repeated",
    ))?;
    custom.function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
        "assertion",
    ))?;
    custom.function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
        "panic_case",
    ))?;
    custom.function(FunctionDeclaration::<(BigInt,), BigInt>::new("nested"))?;
    custom.function(FunctionDeclaration::<(), BigInt>::new("main"))?;
    let custom_entry = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/custom_scalars.gleam",
                custom_source,
            )],
        )],
        HostProviderSet::<work_provider::Profile>::new([])?,
    )?;
    let custom_entry =
        PreparedHostedEntry::try_from_module_plan(geam_core::plan_host_program(custom_entry)?)?;

    let custom_loop_source = include_str!("../tests/fixtures/prepared/custom_loop.gleam");
    let custom_loop =
        geam_core::compile_typed_module("example", "src/custom_loop.gleam", custom_loop_source)?;
    let (mut custom_loop, _) = ModuleBuilder::new(custom_loop)?
        .function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new("run"))?;
    custom_loop.function(FunctionDeclaration::<(BigInt, bool), BigInt>::new("chosen"))?;
    custom_loop.function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
        "captured",
    ))?;
    custom_loop.function(FunctionDeclaration::<(bool, BigInt), bool>::new("boolean"))?;
    custom_loop.function(FunctionDeclaration::<(BigInt,), BigInt>::new("assertion"))?;
    custom_loop.function(FunctionDeclaration::<(), BigInt>::new("empty"))?;
    custom_loop.function(FunctionDeclaration::<(BigInt,), BigInt>::new("panic_case"))?;
    custom_loop.function(FunctionDeclaration::<(BigInt,), BigInt>::new("unsupported"))?;
    custom_loop.function(FunctionDeclaration::<(BigInt, BigInt), BigInt>::new(
        "custom_capture",
    ))?;
    custom_loop
        .function(FunctionDeclaration::<(BigInt, BigInt, BigInt), BigInt>::new("repeated"))?;
    custom_loop.function(FunctionDeclaration::<(BigInt,), BigInt>::new(
        "caller_overflow",
    ))?;
    custom_loop.function(FunctionDeclaration::<(BigInt, BigInt, bool), BigInt>::new(
        "guarded",
    ))?;
    custom_loop.function(FunctionDeclaration::<(), BigInt>::new("main"))?;
    let custom_loop_entry = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/custom_loop.gleam",
                custom_loop_source,
            )],
        )],
        HostProviderSet::<work_provider::Profile>::new([])?,
    )?;
    let custom_loop_entry = PreparedHostedEntry::try_from_module_plan(
        geam_core::plan_host_program(custom_loop_entry)?,
    )?;

    let loop_boundaries = geam_core::compile_typed_module(
        "example",
        "src/custom_loop_boundaries.gleam",
        include_str!("../tests/fixtures/prepared/custom_loop_boundaries.gleam"),
    )?;
    let (mut loop_boundaries, _) = ModuleBuilder::new(loop_boundaries)?
        .function(FunctionDeclaration::<(BigInt, bool, bool, BigInt), BigInt>::new("integer"))?;
    loop_boundaries.function(FunctionDeclaration::<(BigInt,), bool>::new("boolean"))?;

    let hosted_boundaries = geam_core::compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/custom_loop_boundaries.gleam",
                include_str!("../tests/fixtures/prepared/custom_loop_boundaries.gleam"),
            )],
        )],
        HostProviderSet::<work_provider::Profile>::new([])?,
    )?;
    let (mut hosted_boundaries, _) = HostedModuleBuilder::new(hosted_boundaries)?
        .function(FunctionDeclaration::<(BigInt, bool, bool, BigInt), BigInt>::new("integer"))?;
    hosted_boundaries.function(FunctionDeclaration::<(BigInt,), bool>::new("boolean"))?;

    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/prepared");
    for (name, data) in [
        ("arithmetic.rs", arithmetic.prepare().emit_rust()),
        ("numeric_switch.rs", numeric_switch.prepare().emit_rust()),
        ("string_ranges.rs", string_ranges.prepare().emit_rust()),
        (
            "string_ranges_hosted.rs",
            string_hosted.prepare()?.emit_rust(),
        ),
        ("string_ranges_entry.rs", string_entry.emit_rust()),
        (
            "string_checkpoint_assertion.rs",
            checkpoint_assertion.prepare().emit_rust(),
        ),
        (
            "string_checkpoint_stops.rs",
            checkpoint_stops.prepare().emit_rust(),
        ),
        (
            "string_checkpoint_hosted_stop.rs",
            checkpoint_hosted.prepare()?.emit_rust(),
        ),
        ("numeric.rs", numeric.prepare().emit_rust()),
        ("function_calls.rs", calls.prepare().emit_rust()),
        ("boolean_calls.rs", boolean_calls.prepare().emit_rust()),
        ("numeric_hosted.rs", hosted_numeric.prepare()?.emit_rust()),
        ("numeric_entry.rs", numeric_entry.emit_rust()),
        ("bit_array_entry.rs", bit_entry.emit_rust()),
        ("int_list.rs", int_list.prepare().emit_rust()),
        ("int_list_hosted.rs", hosted_int_list.prepare()?.emit_rust()),
        ("int_list_entry.rs", int_list_entry.emit_rust()),
        ("custom_scalars.rs", custom.prepare().emit_rust()),
        ("custom_scalars_entry.rs", custom_entry.emit_rust()),
        ("custom_loop.rs", custom_loop.prepare().emit_rust()),
        ("custom_loop_entry.rs", custom_loop_entry.emit_rust()),
        (
            "custom_loop_boundaries_hosted.rs",
            hosted_boundaries.prepare()?.emit_rust(),
        ),
        (
            "custom_loop_boundaries.rs",
            loop_boundaries.prepare().emit_rust(),
        ),
        ("shared_custom.rs", shared_provider::prepare().emit_rust()),
        (
            "opaque_functions.rs",
            opaque_provider::prepare().emit_rust(),
        ),
        (
            "function_values.rs",
            function_value_provider::prepare().emit_rust(),
        ),
        ("callables.rs", callable_declarations::prepare().emit_rust()),
        (
            "callable_embedding.rs",
            callable_declarations::prepare_scoped().emit_rust(),
        ),
        (
            "callable_views.rs",
            callable_declarations::prepare_native_views().emit_rust(),
        ),
        ("list_construction.rs", construction.prepare().emit_rust()),
        ("list_construction_entry.rs", construction_entry.emit_rust()),
        (
            "list_construction_hosted.rs",
            construction_hosted.prepare()?.emit_rust(),
        ),
        ("list_native.rs", list_native.prepare()?.emit_rust()),
        ("values.rs", values.prepare().emit_rust()),
        ("nested_patterns.rs", patterns.prepare().emit_rust()),
        (
            "symbolic_patterns.rs",
            symbolic_patterns.prepare().emit_rust(),
        ),
        (
            "multi_subject_patterns.rs",
            multi_subject.prepare().emit_rust(),
        ),
        ("sparse_patterns.rs", sparse.prepare().emit_rust()),
        ("bit_array_patterns.rs", bit_arrays.prepare().emit_rust()),
        ("bit_array_loops.rs", bit_loops.prepare().emit_rust()),
        ("native.rs", native.prepare()?.emit_rust()),
        ("work.rs", work_provider::prepare().emit_rust()),
        (
            "entry.rs",
            work_provider::prepare_entry("entry").emit_rust(),
        ),
        (
            "entry_work.rs",
            work_provider::prepare_entry("entry_work").emit_rust(),
        ),
        (
            "entry_failure.rs",
            work_provider::prepare_entry("entry_failure").emit_rust(),
        ),
    ] {
        let destination = root.join(name);
        std::fs::write(&destination, format!("{data}\n"))?;
        println!("{}", destination.display());
    }
    Ok(())
}
