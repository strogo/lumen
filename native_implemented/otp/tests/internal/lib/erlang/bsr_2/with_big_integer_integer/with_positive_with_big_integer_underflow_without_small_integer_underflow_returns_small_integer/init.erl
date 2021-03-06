-module(init).
-export([start/0]).
-import(erlang, [display/1]).
-import(lumen, [is_big_integer/1, is_small_integer/1]).

start() ->
  Integer = 2#101100111000111100001111100000111111000000111111100000001111111100000000,
  true = is_big_integer(Integer),
  Shift = 71,
  true = (Shift > 0),
  Final = Integer bsr Shift,
  display(is_small_integer(Final)),
  display(Final).
