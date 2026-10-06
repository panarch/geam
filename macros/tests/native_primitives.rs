use geam_core::__prepared_support as data;
use geam_core::embedding::FunctionDeclaration;
use geam_core::execution::TokioHost;
use geam_core::provider::{BigInt, BitArrayValue, Call, StringValue, Value};
use geam_core::{
    HostComponentProfile, HostProfile, HostProviderComponent, HostProviderComponentRegistration,
    HostProviderSet,
};
use std::sync::atomic::{AtomicUsize, Ordering};

static PROGRAM: data::HostedModuleArtifact =
    include!("../../core/tests/fixtures/prepared/native_loop.rs");
static CALLS: [AtomicUsize; 9] = [const { AtomicUsize::new(0) }; 9];

#[geam_macros::provider(package = "application", modules = [native_loop], crate_path = geam_core)]
pub struct Component;

#[geam_macros::module(path = "native_loop", crate_path = geam_core)]
mod native_loop {
    use super::{BigInt, BitArrayValue, CALLS, Call, Ordering, StringValue, Value};

    #[geam_macros::function]
    fn observe(value: BigInt) -> BigInt {
        let index = CALLS[0].fetch_add(1, Ordering::Relaxed) + 1;
        value + index
    }

    #[geam_macros::function]
    fn observe_float(value: f64) -> f64 {
        assert_eq!(value.to_bits(), (-0.0_f64).to_bits());
        value + (CALLS[1].fetch_add(1, Ordering::Relaxed) + 1) as f64
    }

    #[geam_macros::function]
    fn observe_string(value: StringValue) -> StringValue {
        assert_eq!(value, StringValue::from("λ shared".repeat(4096)));
        if CALLS[2].fetch_add(1, Ordering::Relaxed).is_multiple_of(2) {
            "odd"
        } else {
            "even"
        }
        .into()
    }

    #[geam_macros::function]
    fn observe_bit_array(value: BitArrayValue) -> BitArrayValue {
        assert_eq!(value.bit_len(), 13);
        assert_eq!(value.bytes(), &[0xb7, 0xc8]);
        BitArrayValue::from_bytes(vec![(CALLS[3].fetch_add(1, Ordering::Relaxed) + 1) as u8])
    }

    #[geam_macros::function]
    fn observe_utf_codepoint(value: char) -> char {
        assert_eq!(value, '🦀');
        if CALLS[4].fetch_add(1, Ordering::Relaxed).is_multiple_of(2) {
            'β'
        } else {
            'λ'
        }
    }

    #[geam_macros::function]
    fn observe_bool(value: bool) -> bool {
        assert!(value);
        !CALLS[5].fetch_add(1, Ordering::Relaxed).is_multiple_of(2)
    }

    #[geam_macros::function]
    fn observe_nil(_value: ()) -> () {
        CALLS[6].fetch_add(1, Ordering::Relaxed);
    }

    #[geam_macros::function]
    fn float_to_bool(value: f64) -> bool {
        assert_eq!(value, 1.25);
        CALLS[7].fetch_add(1, Ordering::Relaxed).is_multiple_of(2)
    }

    #[geam_macros::function]
    fn keep<Item>(value: Value<Item>) -> Value<Item> {
        CALLS[8].fetch_add(1, Ordering::Relaxed);
        value
    }

    #[geam_macros::function]
    fn begin(#[geam_macros::call] _call: &Call<()>) -> () {}
}

struct Profile;

impl HostProfile for Profile {
    type RunState = ();
    type ExternalStores = <Component as HostProviderComponent>::Stores;
    type ExecutionState = ();
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Self::ExternalStores) -> &Self::ExternalStores {
        stores
    }
    fn component_state(state: &mut ()) -> &mut () {
        state
    }
}

#[test]
fn automatic_scalar_adapters_link_and_run_the_actual_prepared_loop() {
    let providers = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    let mut bindings = PROGRAM
        .load(HostProviderSet::from_providers(providers).unwrap())
        .unwrap();
    macro_rules! bind {
        ($name:expr, $input:ty, $output:ty) => {
            bindings
                .function(FunctionDeclaration::<(BigInt, $input), $output>::new($name))
                .unwrap()
        };
    }
    let integer = bind!("captured", BigInt, BigInt);
    let float = bind!("captured_float", f64, f64);
    let string = bind!("captured_string", StringValue, StringValue);
    let bits = bind!("captured_bit_array", BitArrayValue, BitArrayValue);
    let codepoint = bind!("captured_utf_codepoint", char, char);
    let boolean = bind!("captured_bool", bool, bool);
    let nil = bind!("captured_nil", (), ());
    let mixed = bind!("mixed", f64, bool);
    let literal = bindings
        .function(FunctionDeclaration::<(BigInt,), ()>::new("literal_nil"))
        .unwrap();
    let keep_float = bind!("retained_float", f64, f64);
    let keep_string = bind!("retained_string", StringValue, StringValue);
    let keep_bits = bind!("retained_bit_array", BitArrayValue, BitArrayValue);
    let mut module = bindings.seal();
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .unwrap();
    let host = TokioHost::new(runtime.handle().clone());
    macro_rules! call {
        ($index:expr, $function:expr, $arguments:expr, $count:expr) => {{
            CALLS[$index].store(0, Ordering::Relaxed);
            let returned = runtime
                .block_on(
                    module.with_execution(&host, &mut (), &mut Vec::new(), async |scope| {
                        scope.call($function, $arguments).await
                    }),
                )
                .unwrap()
                .try_into_value()
                .unwrap()
                .unwrap();
            assert_eq!(CALLS[$index].load(Ordering::Relaxed), $count);
            returned
        }};
    }
    let original_string: StringValue = "λ shared".repeat(4096).into();
    let raw_string = StringValue::from_bytes(vec![0xff; 4096]).slice(1..4095);
    let original_bits = BitArrayValue::try_from_parts(vec![0xb7, 0xc8], 13).unwrap();
    for count in [1usize, 3, 129] {
        assert_eq!(
            call!(0, &integer, (count.into(), 7.into()), count),
            BigInt::from(7 + count)
        );
        assert_eq!(call!(1, &float, (count.into(), -0.0), count), count as f64);
        assert_eq!(
            call!(2, &string, (count.into(), original_string.clone()), count),
            StringValue::from("odd")
        );
        assert_eq!(
            call!(3, &bits, (count.into(), original_bits.clone()), count),
            BitArrayValue::from_bytes(vec![count as u8])
        );
        assert_eq!(call!(4, &codepoint, (count.into(), '🦀'), count), 'β');
        assert!(!call!(5, &boolean, (count.into(), true), count));
        call!(6, &nil, (count.into(), ()), count);
        call!(6, &literal, (count.into(),), count);
        assert!(call!(7, &mixed, (count.into(), 1.25), count));
        let original = f64::from_bits(0x7ff8_0000_0000_0042);
        let returned = call!(8, &keep_float, (count.into(), original), count);
        assert_eq!(returned.to_bits(), original.to_bits());
        for value in [&original_string, &raw_string] {
            let returned = call!(8, &keep_string, (count.into(), value.clone()), count);
            assert_eq!(returned.as_bytes(), value.as_bytes());
            assert_eq!(returned.as_ptr(), value.as_ptr());
        }
        let returned = call!(8, &keep_bits, (count.into(), original_bits.clone()), count);
        assert_eq!(returned.bit_len(), 13);
        assert_eq!(returned.bytes().as_ptr(), original_bits.bytes().as_ptr());
    }
}
