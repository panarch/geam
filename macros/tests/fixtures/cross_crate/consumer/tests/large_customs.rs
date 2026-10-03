use geam_core::{
    HostComponentProfile, HostProfile, HostProviderComponent, HostProviderComponentRegistration,
};
use geam_macro_cross_crate_declarations::large_customs::Component;

struct Profile;

#[derive(Default)]
struct Stores {
    provider: <Component as HostProviderComponent>::Stores,
}

impl HostProfile for Profile {
    type RunState = ();
    type ExternalStores = Stores;
    type ExecutionState = ();
}

impl HostComponentProfile<Component> for Profile {
    fn component_stores(stores: &Stores) -> &<Component as HostProviderComponent>::Stores {
        &stores.provider
    }

    fn component_state(state: &mut ()) -> &mut () {
        state
    }
}

#[test]
fn complete_79_constructor_provider_registers_at_the_default_recursion_limit() {
    let modules = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    assert_eq!(modules.len(), 1);
    assert_eq!(modules[0].package(), "custom_repro");
    assert_eq!(modules[0].module(), "custom_repro");
    println!("Registered one module");
}

#[path = "../../../../../../tests/support/execution_host.rs"]
mod execution_fixture;

use geam_core::{
    HostProviderSet, HostedExecution, ModuleSource, PackageSource, Value,
    compile_typed_host_program, plan_host_program,
};

const DECLARATIONS: &str = include_str!("../../declarations/gleam/declarations.gleam");

fn execute(body: &str) -> Value {
    let modules = <Component as HostProviderComponentRegistration<Profile>>::providers().unwrap();
    let providers = HostProviderSet::from_providers(modules).unwrap();
    let source = format!("{DECLARATIONS}\npub fn main() {{\n{body}\n}}\n");
    let typed = compile_typed_host_program(
        "custom_repro",
        "custom_repro",
        [PackageSource::new(
            "custom_repro",
            Vec::<String>::new(),
            [ModuleSource::new(
                "custom_repro",
                "src/custom_repro.gleam",
                source,
            )],
        )],
        providers,
    )
    .unwrap();
    let mut execution =
        HostedExecution::try_from_module_plan(plan_host_program(typed).unwrap()).unwrap();
    execution_fixture::run(&mut execution, &mut (), &mut Vec::new()).unwrap()
}

#[test]
fn small_control_maps_every_constructor_in_source_order() {
    assert_eq!(
        execute(
            r#"
  assert small_values() == [Small0, Small1, Small2, Small3, Small4, Small5, Small6, Small7]
  assert small_index(Small0) == 0
  assert small_index(Small1) == 1
  assert small_index(Small2) == 2
  assert small_index(Small3) == 3
  assert small_index(Small4) == 4
  assert small_index(Small5) == 5
  assert small_index(Small6) == 6
  assert small_index(Small7) == 7
  True
"#
        ),
        Value::Bool(true)
    );
}

#[test]
fn all_79_constructors_round_trip_across_crates() {
    assert_eq!(
        execute(
            r#"
  assert reason() == Reason0
  assert reasons() == [Reason0, Reason1, Reason2, Reason3, Reason4, Reason5, Reason6, Reason7, Reason8, Reason9, Reason10, Reason11, Reason12, Reason13, Reason14, Reason15, Reason16, Reason17, Reason18, Reason19, Reason20, Reason21, Reason22, Reason23, Reason24, Reason25, Reason26, Reason27, Reason28, Reason29, Reason30, Reason31, Reason32, Reason33, Reason34, Reason35, Reason36, Reason37, Reason38, Reason39, Reason40, Reason41, Reason42, Reason43, Reason44, Reason45, Reason46, Reason47, Reason48, Reason49, Reason50, Reason51, Reason52, Reason53, Reason54, Reason55, Reason56, Reason57, Reason58, Reason59, Reason60, Reason61, Reason62, Reason63, Reason64, Reason65, Reason66, Reason67, Reason68, Reason69, Reason70, Reason71, Reason72, Reason73, Reason74, Reason75, Reason76, Reason77, Reason78]
  assert reason_index(Reason0) == 0
  assert reason_index(Reason1) == 1
  assert reason_index(Reason2) == 2
  assert reason_index(Reason3) == 3
  assert reason_index(Reason4) == 4
  assert reason_index(Reason5) == 5
  assert reason_index(Reason6) == 6
  assert reason_index(Reason7) == 7
  assert reason_index(Reason8) == 8
  assert reason_index(Reason9) == 9
  assert reason_index(Reason10) == 10
  assert reason_index(Reason11) == 11
  assert reason_index(Reason12) == 12
  assert reason_index(Reason13) == 13
  assert reason_index(Reason14) == 14
  assert reason_index(Reason15) == 15
  assert reason_index(Reason16) == 16
  assert reason_index(Reason17) == 17
  assert reason_index(Reason18) == 18
  assert reason_index(Reason19) == 19
  assert reason_index(Reason20) == 20
  assert reason_index(Reason21) == 21
  assert reason_index(Reason22) == 22
  assert reason_index(Reason23) == 23
  assert reason_index(Reason24) == 24
  assert reason_index(Reason25) == 25
  assert reason_index(Reason26) == 26
  assert reason_index(Reason27) == 27
  assert reason_index(Reason28) == 28
  assert reason_index(Reason29) == 29
  assert reason_index(Reason30) == 30
  assert reason_index(Reason31) == 31
  assert reason_index(Reason32) == 32
  assert reason_index(Reason33) == 33
  assert reason_index(Reason34) == 34
  assert reason_index(Reason35) == 35
  assert reason_index(Reason36) == 36
  assert reason_index(Reason37) == 37
  assert reason_index(Reason38) == 38
  assert reason_index(Reason39) == 39
  assert reason_index(Reason40) == 40
  assert reason_index(Reason41) == 41
  assert reason_index(Reason42) == 42
  assert reason_index(Reason43) == 43
  assert reason_index(Reason44) == 44
  assert reason_index(Reason45) == 45
  assert reason_index(Reason46) == 46
  assert reason_index(Reason47) == 47
  assert reason_index(Reason48) == 48
  assert reason_index(Reason49) == 49
  assert reason_index(Reason50) == 50
  assert reason_index(Reason51) == 51
  assert reason_index(Reason52) == 52
  assert reason_index(Reason53) == 53
  assert reason_index(Reason54) == 54
  assert reason_index(Reason55) == 55
  assert reason_index(Reason56) == 56
  assert reason_index(Reason57) == 57
  assert reason_index(Reason58) == 58
  assert reason_index(Reason59) == 59
  assert reason_index(Reason60) == 60
  assert reason_index(Reason61) == 61
  assert reason_index(Reason62) == 62
  assert reason_index(Reason63) == 63
  assert reason_index(Reason64) == 64
  assert reason_index(Reason65) == 65
  assert reason_index(Reason66) == 66
  assert reason_index(Reason67) == 67
  assert reason_index(Reason68) == 68
  assert reason_index(Reason69) == 69
  assert reason_index(Reason70) == 70
  assert reason_index(Reason71) == 71
  assert reason_index(Reason72) == 72
  assert reason_index(Reason73) == 73
  assert reason_index(Reason74) == 74
  assert reason_index(Reason75) == 75
  assert reason_index(Reason76) == 76
  assert reason_index(Reason77) == 77
  assert reason_index(Reason78) == 78
  assert reason_index(unwrap(wrap(Reason78))) == 78
  assert reason_index(unwrap(wrap_many(Reason39))) == 39
  assert unwrap(wrap(True)) == True
  True
"#
        ),
        Value::Bool(true)
    );
}

#[test]
fn all_160_payload_constructors_construct_and_decode_registered_fields() {
    assert_eq!(
        execute(
            r#"
  assert payloads() == [Item0([0]), Item1([1]), Item2([2]), Item3([3]), Item4([4]), Item5([5]), Item6([6]), Item7([7]), Item8([8]), Item9([9]), Item10([10]), Item11([11]), Item12([12]), Item13([13]), Item14([14]), Item15([15]), Item16([16]), Item17([17]), Item18([18]), Item19([19]), Item20([20]), Item21([21]), Item22([22]), Item23([23]), Item24([24]), Item25([25]), Item26([26]), Item27([27]), Item28([28]), Item29([29]), Item30([30]), Item31([31]), Item32([32]), Item33([33]), Item34([34]), Item35([35]), Item36([36]), Item37([37]), Item38([38]), Item39([39]), Item40([40]), Item41([41]), Item42([42]), Item43([43]), Item44([44]), Item45([45]), Item46([46]), Item47([47]), Item48([48]), Item49([49]), Item50([50]), Item51([51]), Item52([52]), Item53([53]), Item54([54]), Item55([55]), Item56([56]), Item57([57]), Item58([58]), Item59([59]), Item60([60]), Item61([61]), Item62([62]), Item63([63]), Item64([64]), Item65([65]), Item66([66]), Item67([67]), Item68([68]), Item69([69]), Item70([70]), Item71([71]), Item72([72]), Item73([73]), Item74([74]), Item75([75]), Item76([76]), Item77([77]), Item78([78]), Item79([79]), Item80([80]), Item81([81]), Item82([82]), Item83([83]), Item84([84]), Item85([85]), Item86([86]), Item87([87]), Item88([88]), Item89([89]), Item90([90]), Item91([91]), Item92([92]), Item93([93]), Item94([94]), Item95([95]), Item96([96]), Item97([97]), Item98([98]), Item99([99]), Item100([100]), Item101([101]), Item102([102]), Item103([103]), Item104([104]), Item105([105]), Item106([106]), Item107([107]), Item108([108]), Item109([109]), Item110([110]), Item111([111]), Item112([112]), Item113([113]), Item114([114]), Item115([115]), Item116([116]), Item117([117]), Item118([118]), Item119([119]), Item120([120]), Item121([121]), Item122([122]), Item123([123]), Item124([124]), Item125([125]), Item126([126]), Item127([127]), Item128([128]), Item129([129]), Item130([130]), Item131([131]), Item132([132]), Item133([133]), Item134([134]), Item135([135]), Item136([136]), Item137([137]), Item138([138]), Item139([139]), Item140([140]), Item141([141]), Item142([142]), Item143([143]), Item144([144]), Item145([145]), Item146([146]), Item147([147]), Item148([148]), Item149([149]), Item150([150]), Item151([151]), Item152([152]), Item153([153]), Item154([154]), Item155([155]), Item156([156]), Item157([157]), Item158([158]), Item159([159])]
  assert payload_number(Item0([0])) == 0
  assert payload_number(Item1([1])) == 1
  assert payload_number(Item2([2])) == 2
  assert payload_number(Item3([3])) == 3
  assert payload_number(Item4([4])) == 4
  assert payload_number(Item5([5])) == 5
  assert payload_number(Item6([6])) == 6
  assert payload_number(Item7([7])) == 7
  assert payload_number(Item8([8])) == 8
  assert payload_number(Item9([9])) == 9
  assert payload_number(Item10([10])) == 10
  assert payload_number(Item11([11])) == 11
  assert payload_number(Item12([12])) == 12
  assert payload_number(Item13([13])) == 13
  assert payload_number(Item14([14])) == 14
  assert payload_number(Item15([15])) == 15
  assert payload_number(Item16([16])) == 16
  assert payload_number(Item17([17])) == 17
  assert payload_number(Item18([18])) == 18
  assert payload_number(Item19([19])) == 19
  assert payload_number(Item20([20])) == 20
  assert payload_number(Item21([21])) == 21
  assert payload_number(Item22([22])) == 22
  assert payload_number(Item23([23])) == 23
  assert payload_number(Item24([24])) == 24
  assert payload_number(Item25([25])) == 25
  assert payload_number(Item26([26])) == 26
  assert payload_number(Item27([27])) == 27
  assert payload_number(Item28([28])) == 28
  assert payload_number(Item29([29])) == 29
  assert payload_number(Item30([30])) == 30
  assert payload_number(Item31([31])) == 31
  assert payload_number(Item32([32])) == 32
  assert payload_number(Item33([33])) == 33
  assert payload_number(Item34([34])) == 34
  assert payload_number(Item35([35])) == 35
  assert payload_number(Item36([36])) == 36
  assert payload_number(Item37([37])) == 37
  assert payload_number(Item38([38])) == 38
  assert payload_number(Item39([39])) == 39
  assert payload_number(Item40([40])) == 40
  assert payload_number(Item41([41])) == 41
  assert payload_number(Item42([42])) == 42
  assert payload_number(Item43([43])) == 43
  assert payload_number(Item44([44])) == 44
  assert payload_number(Item45([45])) == 45
  assert payload_number(Item46([46])) == 46
  assert payload_number(Item47([47])) == 47
  assert payload_number(Item48([48])) == 48
  assert payload_number(Item49([49])) == 49
  assert payload_number(Item50([50])) == 50
  assert payload_number(Item51([51])) == 51
  assert payload_number(Item52([52])) == 52
  assert payload_number(Item53([53])) == 53
  assert payload_number(Item54([54])) == 54
  assert payload_number(Item55([55])) == 55
  assert payload_number(Item56([56])) == 56
  assert payload_number(Item57([57])) == 57
  assert payload_number(Item58([58])) == 58
  assert payload_number(Item59([59])) == 59
  assert payload_number(Item60([60])) == 60
  assert payload_number(Item61([61])) == 61
  assert payload_number(Item62([62])) == 62
  assert payload_number(Item63([63])) == 63
  assert payload_number(Item64([64])) == 64
  assert payload_number(Item65([65])) == 65
  assert payload_number(Item66([66])) == 66
  assert payload_number(Item67([67])) == 67
  assert payload_number(Item68([68])) == 68
  assert payload_number(Item69([69])) == 69
  assert payload_number(Item70([70])) == 70
  assert payload_number(Item71([71])) == 71
  assert payload_number(Item72([72])) == 72
  assert payload_number(Item73([73])) == 73
  assert payload_number(Item74([74])) == 74
  assert payload_number(Item75([75])) == 75
  assert payload_number(Item76([76])) == 76
  assert payload_number(Item77([77])) == 77
  assert payload_number(Item78([78])) == 78
  assert payload_number(Item79([79])) == 79
  assert payload_number(Item80([80])) == 80
  assert payload_number(Item81([81])) == 81
  assert payload_number(Item82([82])) == 82
  assert payload_number(Item83([83])) == 83
  assert payload_number(Item84([84])) == 84
  assert payload_number(Item85([85])) == 85
  assert payload_number(Item86([86])) == 86
  assert payload_number(Item87([87])) == 87
  assert payload_number(Item88([88])) == 88
  assert payload_number(Item89([89])) == 89
  assert payload_number(Item90([90])) == 90
  assert payload_number(Item91([91])) == 91
  assert payload_number(Item92([92])) == 92
  assert payload_number(Item93([93])) == 93
  assert payload_number(Item94([94])) == 94
  assert payload_number(Item95([95])) == 95
  assert payload_number(Item96([96])) == 96
  assert payload_number(Item97([97])) == 97
  assert payload_number(Item98([98])) == 98
  assert payload_number(Item99([99])) == 99
  assert payload_number(Item100([100])) == 100
  assert payload_number(Item101([101])) == 101
  assert payload_number(Item102([102])) == 102
  assert payload_number(Item103([103])) == 103
  assert payload_number(Item104([104])) == 104
  assert payload_number(Item105([105])) == 105
  assert payload_number(Item106([106])) == 106
  assert payload_number(Item107([107])) == 107
  assert payload_number(Item108([108])) == 108
  assert payload_number(Item109([109])) == 109
  assert payload_number(Item110([110])) == 110
  assert payload_number(Item111([111])) == 111
  assert payload_number(Item112([112])) == 112
  assert payload_number(Item113([113])) == 113
  assert payload_number(Item114([114])) == 114
  assert payload_number(Item115([115])) == 115
  assert payload_number(Item116([116])) == 116
  assert payload_number(Item117([117])) == 117
  assert payload_number(Item118([118])) == 118
  assert payload_number(Item119([119])) == 119
  assert payload_number(Item120([120])) == 120
  assert payload_number(Item121([121])) == 121
  assert payload_number(Item122([122])) == 122
  assert payload_number(Item123([123])) == 123
  assert payload_number(Item124([124])) == 124
  assert payload_number(Item125([125])) == 125
  assert payload_number(Item126([126])) == 126
  assert payload_number(Item127([127])) == 127
  assert payload_number(Item128([128])) == 128
  assert payload_number(Item129([129])) == 129
  assert payload_number(Item130([130])) == 130
  assert payload_number(Item131([131])) == 131
  assert payload_number(Item132([132])) == 132
  assert payload_number(Item133([133])) == 133
  assert payload_number(Item134([134])) == 134
  assert payload_number(Item135([135])) == 135
  assert payload_number(Item136([136])) == 136
  assert payload_number(Item137([137])) == 137
  assert payload_number(Item138([138])) == 138
  assert payload_number(Item139([139])) == 139
  assert payload_number(Item140([140])) == 140
  assert payload_number(Item141([141])) == 141
  assert payload_number(Item142([142])) == 142
  assert payload_number(Item143([143])) == 143
  assert payload_number(Item144([144])) == 144
  assert payload_number(Item145([145])) == 145
  assert payload_number(Item146([146])) == 146
  assert payload_number(Item147([147])) == 147
  assert payload_number(Item148([148])) == 148
  assert payload_number(Item149([149])) == 149
  assert payload_number(Item150([150])) == 150
  assert payload_number(Item151([151])) == 151
  assert payload_number(Item152([152])) == 152
  assert payload_number(Item153([153])) == 153
  assert payload_number(Item154([154])) == 154
  assert payload_number(Item155([155])) == 155
  assert payload_number(Item156([156])) == 156
  assert payload_number(Item157([157])) == 157
  assert payload_number(Item158([158])) == 158
  assert payload_number(Item159([159])) == 159
  assert payload_number(Item159([])) == -1
  assert payload_number(unwrap(wrap(Item159([201])))) == 201
  assert payload_number(unwrap(wrap_many(Item80([99])))) == 99
  True
"#
        ),
        Value::Bool(true)
    );
}
