-module(geam_benchmarks_ffi).
-export([environment/1, monotonic_ns/0, consume/1]).

environment(Name) ->
    case os:getenv(binary_to_list(Name)) of
        false -> {error, <<"Missing environment variable: ", Name/binary>>};
        Value -> {ok, unicode:characters_to_binary(Value)}
    end.

monotonic_ns() -> erlang:monotonic_time(nanosecond).

consume(Value) ->
    erlang:put(geam_benchmarks_consumed, Value),
    Value.
