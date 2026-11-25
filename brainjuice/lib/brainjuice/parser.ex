defmodule Brainjuice.Parser do
  def parse(program) when is_binary(program) do
    do_parse(program, 0, [], [], %{})
  end

  defp do_parse(<<>>, _, [], ops, jumps), do: {ops |> Enum.reverse() |> List.to_tuple(), jumps}

  defp do_parse(<<>>, _, _, _, _), do: raise(ArgumentError, "no matching ] for [ found")

  defp do_parse(<<"+", rest::binary>>, i, open, ops, jumps),
    do: do_parse(rest, i + 1, open, [:inc | ops], jumps)

  defp do_parse(<<"-", rest::binary>>, i, open, ops, jumps),
    do: do_parse(rest, i + 1, open, [:dec | ops], jumps)

  defp do_parse(<<",", rest::binary>>, i, open, ops, jumps),
    do: do_parse(rest, i + 1, open, [:input | ops], jumps)

  defp do_parse(<<".", rest::binary>>, i, open, ops, jumps),
    do: do_parse(rest, i + 1, open, [:output | ops], jumps)

  defp do_parse(<<"<", rest::binary>>, i, open, ops, jumps),
    do: do_parse(rest, i + 1, open, [:left | ops], jumps)

  defp do_parse(<<">", rest::binary>>, i, open, ops, jumps),
    do: do_parse(rest, i + 1, open, [:right | ops], jumps)

  defp do_parse(<<"[", rest::binary>>, i, open, ops, jumps),
    do: do_parse(rest, i + 1, [i | open], [:jump_if_zero | ops], jumps)

  defp do_parse(<<"]", _::binary>>, _, [], _, _),
    do: raise(ArgumentError, "no matching [ for ] found")

  defp do_parse(<<"]", rest::binary>>, i, [start | open], ops, jumps) do
    jumps = Map.merge(jumps, %{start => i, i => start})
    do_parse(rest, i + 1, open, [:jump_not_zero | ops], jumps)
  end

  defp do_parse(<<_comment, rest::binary>>, i, open, ops, jumps),
    do: do_parse(rest, i, open, ops, jumps)
end
