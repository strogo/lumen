-module(init).
-export([start/0]).
-import(erlang, [display/1]).

start() ->
  Module = init,
  %% Typo
  Function = child,
  Args = [],
  Options = [monitor],
  {ChildPid, ChildMonitorReference} = spawn_opt(Module, Function, Args, Options),
  receive
    {'DOWN', ChildMonitorReference, process, _, Info} ->
      display({child, exited, Info})
  after 10 ->
    display({child, alive, is_process_alive(ChildPid)})
  end.

child() ->
  display({in, child}).
