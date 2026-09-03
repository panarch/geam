-module(example_text_pattern_ffi).

-export([compile/1, is_match/2, find_all/2, replace_all/3]).
-export_type(['Pattern'/0]).

-opaque 'Pattern'() :: re:mp().

-spec compile(binary()) ->
    {ok, 'Pattern'()} | {error, example_text_pattern:compile_error()}.
compile(Source) ->
    case re:compile(Source, [unicode, ucp]) of
        {ok, Pattern} ->
            {ok, Pattern};
        {error, {Reason, Offset}} ->
            Message = unicode:characters_to_binary(
                io_lib:format("~ts at byte ~B", [Reason, Offset])
            ),
            {error, {compile_error, Message}}
    end.

-spec is_match('Pattern'(), binary()) -> boolean().
is_match(Pattern, Text) ->
    case re:run(Text, Pattern, [{capture, none}]) of
        match -> true;
        nomatch -> false
    end.

-spec find_all('Pattern'(), binary()) -> [binary()].
find_all(Pattern, Text) ->
    case re:run(Text, Pattern, [global, {capture, first, binary}]) of
        {match, Matches} -> [Match || [Match] <- Matches];
        nomatch -> []
    end.

-spec replace_all('Pattern'(), binary(), binary()) -> binary().
replace_all(Pattern, Text, Replacement) ->
    re:replace(Text, Pattern, Replacement, [global, {return, binary}]).
