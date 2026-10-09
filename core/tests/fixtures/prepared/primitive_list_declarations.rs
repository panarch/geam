use geam_core::embedding::{
    BigInt, BitArrayValue, CallableType, FunctionDeclaration, HostedModuleBindings,
    HostedModuleBuilder, List, PreparedHostedModule, StringValue,
};
use geam_core::{
    HostProviderSet, ModuleSource, PackageSource, StatelessHostProfile, compile_typed_host_program,
};

pub fn builder() -> HostedModuleBuilder<StatelessHostProfile> {
    let typed = compile_typed_host_program(
        "example",
        "example",
        [PackageSource::new(
            "example",
            Vec::<String>::new(),
            [ModuleSource::new(
                "example",
                "src/example.gleam",
                include_str!("primitive_list_calls.gleam"),
            )],
        )],
        HostProviderSet::<StatelessHostProfile>::new([]).unwrap(),
    )
    .unwrap();
    HostedModuleBuilder::new(typed).unwrap()
}

pub fn bindings() -> HostedModuleBindings<StatelessHostProfile> {
    let (mut bindings, _) = builder()
        .function(FunctionDeclaration::<
            (List<BigInt>, BigInt, BigInt, BigInt),
            BigInt,
        >::new("integers"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<f64>, f64, f64, f64), f64>::new(
            "floats",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<bool>, bool, bool, bool), bool>::new("booleans"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<StringValue>, StringValue, StringValue, StringValue),
            StringValue,
        >::new("strings"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (
                List<BitArrayValue>,
                BitArrayValue,
                BitArrayValue,
                BitArrayValue,
            ),
            BitArrayValue,
        >::new("bit_arrays"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<char>, char, char, char), char>::new("codepoints"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<()>, (), (), ()), ()>::new(
            "nils",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<BigInt>, BigInt),
            CallableType<(List<BigInt>,), BigInt>,
        >::new("make_integers"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<f64>, f64),
            CallableType<(List<f64>,), f64>,
        >::new("make_floats"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<bool>, bool),
            CallableType<(List<bool>,), bool>,
        >::new("make_booleans"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<StringValue>, StringValue),
            CallableType<(List<StringValue>,), StringValue>,
        >::new("make_strings"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<BitArrayValue>, BitArrayValue),
            CallableType<(List<BitArrayValue>,), BitArrayValue>,
        >::new("make_bit_arrays"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<char>, char),
            CallableType<(List<char>,), char>,
        >::new("make_codepoints"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<()>, ()),
            CallableType<(List<()>,), ()>,
        >::new("make_nils"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<f64>,), BigInt>::new(
            "count_floats",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<()>,), BigInt>::new(
            "count_nils",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(f64, f64), f64>::new("divide"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<f64>, f64), f64>::new(
            "canonical_floats",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<f64>, f64), f64>::new(
            "stopped_float",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<(BigInt, f64)>,), BigInt>::new(
            "unsupported",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(f64, f64), BigInt>::new(
            "float_comparisons",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<BigInt>, BigInt, List<BigInt>),
            BigInt,
        >::new("check_integers"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<f64>, f64, List<f64>), BigInt>::new("check_floats"))
        .unwrap();
    bindings
        .function(
            FunctionDeclaration::<(List<bool>, bool, List<bool>), BigInt>::new("check_booleans"),
        )
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<StringValue>, StringValue, List<StringValue>),
            BigInt,
        >::new("check_strings"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<BitArrayValue>, BitArrayValue, List<BitArrayValue>),
            BigInt,
        >::new("check_bit_arrays"))
        .unwrap();
    bindings
        .function(
            FunctionDeclaration::<(List<char>, char, List<char>), BigInt>::new("check_codepoints"),
        )
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<()>, (), List<()>), BigInt>::new("check_nils"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<BigInt>, BigInt, List<BigInt>),
            BigInt,
        >::new("through_integers"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<f64>, f64, List<f64>), f64>::new("through_floats"))
        .unwrap();
    bindings
        .function(
            FunctionDeclaration::<(List<bool>, bool, List<bool>), bool>::new("through_booleans"),
        )
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<StringValue>, StringValue, List<StringValue>),
            StringValue,
        >::new("through_strings"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<
            (List<BitArrayValue>, BitArrayValue, List<BitArrayValue>),
            BitArrayValue,
        >::new("through_bit_arrays"))
        .unwrap();
    bindings
        .function(
            FunctionDeclaration::<(List<char>, char, List<char>), char>::new("through_codepoints"),
        )
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(List<()>, (), List<()>), ()>::new(
            "through_nils",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(StringValue,), BigInt>::new(
            "call_prefix_length",
        ))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(StringValue, bool), StringValue>::new("call_string_slice"))
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(StringValue,), StringValue>::new(
            "call_string_slice_echo",
        ))
        .unwrap();
    bindings
        .function(
            FunctionDeclaration::<(BitArrayValue, bool), BitArrayValue>::new("call_bit_slice"),
        )
        .unwrap();
    bindings
        .function(FunctionDeclaration::<(BitArrayValue,), BigInt>::new(
            "call_unconnected_bit_checksum",
        ))
        .unwrap();
    bindings
}

pub fn prepare() -> PreparedHostedModule {
    bindings().prepare().unwrap()
}
