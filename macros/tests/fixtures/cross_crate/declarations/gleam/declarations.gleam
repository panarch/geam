pub type Reason {
  Reason0
  Reason1
  Reason2
  Reason3
  Reason4
  Reason5
  Reason6
  Reason7
  Reason8
  Reason9
  Reason10
  Reason11
  Reason12
  Reason13
  Reason14
  Reason15
  Reason16
  Reason17
  Reason18
  Reason19
  Reason20
  Reason21
  Reason22
  Reason23
  Reason24
  Reason25
  Reason26
  Reason27
  Reason28
  Reason29
  Reason30
  Reason31
  Reason32
  Reason33
  Reason34
  Reason35
  Reason36
  Reason37
  Reason38
  Reason39
  Reason40
  Reason41
  Reason42
  Reason43
  Reason44
  Reason45
  Reason46
  Reason47
  Reason48
  Reason49
  Reason50
  Reason51
  Reason52
  Reason53
  Reason54
  Reason55
  Reason56
  Reason57
  Reason58
  Reason59
  Reason60
  Reason61
  Reason62
  Reason63
  Reason64
  Reason65
  Reason66
  Reason67
  Reason68
  Reason69
  Reason70
  Reason71
  Reason72
  Reason73
  Reason74
  Reason75
  Reason76
  Reason77
  Reason78
}

pub type Small {
  Small0
  Small1
  Small2
  Small3
  Small4
  Small5
  Small6
  Small7
}

pub type Payload {
  Item0(List(Int))
  Item1(List(Int))
  Item2(List(Int))
  Item3(List(Int))
  Item4(List(Int))
  Item5(List(Int))
  Item6(List(Int))
  Item7(List(Int))
  Item8(List(Int))
  Item9(List(Int))
  Item10(List(Int))
  Item11(List(Int))
  Item12(List(Int))
  Item13(List(Int))
  Item14(List(Int))
  Item15(List(Int))
  Item16(List(Int))
  Item17(List(Int))
  Item18(List(Int))
  Item19(List(Int))
  Item20(List(Int))
  Item21(List(Int))
  Item22(List(Int))
  Item23(List(Int))
  Item24(List(Int))
  Item25(List(Int))
  Item26(List(Int))
  Item27(List(Int))
  Item28(List(Int))
  Item29(List(Int))
  Item30(List(Int))
  Item31(List(Int))
  Item32(List(Int))
  Item33(List(Int))
  Item34(List(Int))
  Item35(List(Int))
  Item36(List(Int))
  Item37(List(Int))
  Item38(List(Int))
  Item39(List(Int))
  Item40(List(Int))
  Item41(List(Int))
  Item42(List(Int))
  Item43(List(Int))
  Item44(List(Int))
  Item45(List(Int))
  Item46(List(Int))
  Item47(List(Int))
  Item48(List(Int))
  Item49(List(Int))
  Item50(List(Int))
  Item51(List(Int))
  Item52(List(Int))
  Item53(List(Int))
  Item54(List(Int))
  Item55(List(Int))
  Item56(List(Int))
  Item57(List(Int))
  Item58(List(Int))
  Item59(List(Int))
  Item60(List(Int))
  Item61(List(Int))
  Item62(List(Int))
  Item63(List(Int))
  Item64(List(Int))
  Item65(List(Int))
  Item66(List(Int))
  Item67(List(Int))
  Item68(List(Int))
  Item69(List(Int))
  Item70(List(Int))
  Item71(List(Int))
  Item72(List(Int))
  Item73(List(Int))
  Item74(List(Int))
  Item75(List(Int))
  Item76(List(Int))
  Item77(List(Int))
  Item78(List(Int))
  Item79(List(Int))
  Item80(List(Int))
  Item81(List(Int))
  Item82(List(Int))
  Item83(List(Int))
  Item84(List(Int))
  Item85(List(Int))
  Item86(List(Int))
  Item87(List(Int))
  Item88(List(Int))
  Item89(List(Int))
  Item90(List(Int))
  Item91(List(Int))
  Item92(List(Int))
  Item93(List(Int))
  Item94(List(Int))
  Item95(List(Int))
  Item96(List(Int))
  Item97(List(Int))
  Item98(List(Int))
  Item99(List(Int))
  Item100(List(Int))
  Item101(List(Int))
  Item102(List(Int))
  Item103(List(Int))
  Item104(List(Int))
  Item105(List(Int))
  Item106(List(Int))
  Item107(List(Int))
  Item108(List(Int))
  Item109(List(Int))
  Item110(List(Int))
  Item111(List(Int))
  Item112(List(Int))
  Item113(List(Int))
  Item114(List(Int))
  Item115(List(Int))
  Item116(List(Int))
  Item117(List(Int))
  Item118(List(Int))
  Item119(List(Int))
  Item120(List(Int))
  Item121(List(Int))
  Item122(List(Int))
  Item123(List(Int))
  Item124(List(Int))
  Item125(List(Int))
  Item126(List(Int))
  Item127(List(Int))
  Item128(List(Int))
  Item129(List(Int))
  Item130(List(Int))
  Item131(List(Int))
  Item132(List(Int))
  Item133(List(Int))
  Item134(List(Int))
  Item135(List(Int))
  Item136(List(Int))
  Item137(List(Int))
  Item138(List(Int))
  Item139(List(Int))
  Item140(List(Int))
  Item141(List(Int))
  Item142(List(Int))
  Item143(List(Int))
  Item144(List(Int))
  Item145(List(Int))
  Item146(List(Int))
  Item147(List(Int))
  Item148(List(Int))
  Item149(List(Int))
  Item150(List(Int))
  Item151(List(Int))
  Item152(List(Int))
  Item153(List(Int))
  Item154(List(Int))
  Item155(List(Int))
  Item156(List(Int))
  Item157(List(Int))
  Item158(List(Int))
  Item159(List(Int))
}

pub type Envelope(item) {
  Single(item)
  Many(List(item))
}

@external(erlang, "custom_repro", "reason")
fn reason() -> Reason

@external(erlang, "custom_repro", "reasons")
fn reasons() -> List(Reason)

@external(erlang, "custom_repro", "reason_index")
fn reason_index(value: Reason) -> Int

@external(erlang, "custom_repro", "small_values")
fn small_values() -> List(Small)

@external(erlang, "custom_repro", "small_index")
fn small_index(value: Small) -> Int

@external(erlang, "custom_repro", "payloads")
fn payloads() -> List(Payload)

@external(erlang, "custom_repro", "payload_number")
fn payload_number(value: Payload) -> Int

@external(erlang, "custom_repro", "wrap")
fn wrap(value: item) -> Envelope(item)

@external(erlang, "custom_repro", "wrap_many")
fn wrap_many(value: item) -> Envelope(item)

@external(erlang, "custom_repro", "unwrap")
fn unwrap(value: Envelope(item)) -> item
