defmodule Day5 do
  def part1() do
    {ranges, ids} = parse_input()
    Enum.count(ids, &within_ranges(&1, ranges))
  end

  def part2() do
    {ranges, _} = parse_input()

    ranges
    |> Enum.sort(fn {a, _}, {b, _} -> a <= b end)
    |> then(fn r ->
      {from, to} = hd(r)
      ranges_sum(r, from, to, 0)
    end)
  end

  defp within_ranges(id, ranges) do
    ranges
    |> Enum.any?(fn {from, to} -> id >= from && id <= to end)
  end

  defp ranges_sum([], from, to, sum), do: sum + (to + 1 - from)

  defp ranges_sum(ranges, from, to, sum) do
    [{new_from, new_to} | rest] = ranges

    cond do
      new_from > to -> ranges_sum(rest, new_from, new_to, sum + (to + 1 - from))
      new_to > to -> ranges_sum(rest, from, new_to, sum)
      true -> ranges_sum(rest, from, to, sum)
    end
  end

  defp parse_input() do
    Klaus.Input.read!(5)
    |> String.split("\n\n")
    |> then(fn [ranges, ids] ->
      parsed_ranges = parse_ranges(ranges)
      parsed_ids = parse_ids(ids)

      {parsed_ranges, parsed_ids}
    end)
  end

  defp parse_ranges(ranges) do
    ranges
    |> String.split("\n")
    |> Enum.map(fn range ->
      [from, to] = String.split(range, "-")
      {String.to_integer(from), String.to_integer(to)}
    end)
  end

  defp parse_ids(ids) do
    ids
    |> String.split("\n")
    |> Enum.map(&String.to_integer/1)
  end
end
