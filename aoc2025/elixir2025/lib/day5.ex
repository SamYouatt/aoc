defmodule Day5 do
  def part1() do
    {ranges, ids} = parse_input()
    Enum.count(ids, &within_ranges(&1, ranges))
  end

  defp within_ranges(id, ranges) do
    ranges
    |> Enum.any?(fn {from, to} -> id >= from && id <= to end)
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
