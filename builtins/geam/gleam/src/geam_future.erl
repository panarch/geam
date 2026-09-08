-module(geam_future).
-export([ready/1, map/2, flatten/1, all/1]).
-export_type(['Future'/1]).

-opaque 'Future'(_Value) :: term().

ready(_Value) ->
    erlang:error(<<"geam/future is currently implemented by Geam. An Erlang implementation is not currently available.">>).

map(_Value, _Callback) ->
    erlang:error(<<"geam/future is currently implemented by Geam. An Erlang implementation is not currently available.">>).

flatten(_Value) ->
    erlang:error(<<"geam/future is currently implemented by Geam. An Erlang implementation is not currently available.">>).

all(_Values) ->
    erlang:error(<<"geam/future is currently implemented by Geam. An Erlang implementation is not currently available.">>).
