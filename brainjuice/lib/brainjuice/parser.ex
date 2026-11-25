defmodule Brainjuice.Parser do
  defmodule State do
    defstruct op_index: 0, position: 0, open: [], ops: [], positions: [], jumps: %{}
  end

  def parse(source) when is_binary(source), do: do_parse(source, %State{})

  defp do_parse(<<>>, %State{open: []} = state),
    do: %Brainjuice.Program{
      ops: state.ops |> Enum.reverse() |> List.to_tuple(),
      jumps: state.jumps,
      positions: state.positions |> Enum.reverse() |> List.to_tuple()
    }

  defp do_parse(<<>>, %State{open: [{_, position} | _]}),
    do: raise(ArgumentError, "no matching ] for [ found at position #{position}")

  defp do_parse(<<"+", rest::binary>>, state), do: do_parse(rest, push_op(state, :inc))
  defp do_parse(<<"-", rest::binary>>, state), do: do_parse(rest, push_op(state, :dec))

  defp do_parse(<<",", rest::binary>>, state),
    do: do_parse(rest, push_op(state, :input))

  defp do_parse(<<".", rest::binary>>, state),
    do: do_parse(rest, push_op(state, :output))

  defp do_parse(<<"<", rest::binary>>, state),
    do: do_parse(rest, push_op(state, :left))

  defp do_parse(<<">", rest::binary>>, state),
    do: do_parse(rest, push_op(state, :right))

  defp do_parse(<<"[", rest::binary>>, %State{} = state) do
    state = %State{state | open: [{state.op_index, state.position} | state.open]}
    do_parse(rest, push_op(state, :jump_if_zero))
  end

  defp do_parse(<<"]", _::binary>>, %State{open: [], position: position}),
    do: raise(ArgumentError, "no matching [ for ] found at position #{position}")

  defp do_parse(<<"]", rest::binary>>, %State{open: [{start, _} | open]} = state) do
    jumps = Map.merge(state.jumps, %{start => state.op_index, state.op_index => start})
    do_parse(rest, push_op(%State{state | open: open, jumps: jumps}, :jump_not_zero))
  end

  defp do_parse(<<_comment, rest::binary>>, %State{} = state),
    do: do_parse(rest, %State{state | position: state.position + 1})

  defp push_op(%State{} = state, op) do
    %State{
      state
      | op_index: state.op_index + 1,
        position: state.position + 1,
        ops: [op | state.ops],
        positions: [state.position | state.positions]
    }
  end
end
