defmodule Day3 do
  def part1() do
    parse_input()
    |> Enum.sum_by(&max_sequence(&1, 2, []))
  end

  def part2() do
    parse_input()
    |> Enum.sum_by(&max_sequence(&1, 12, []))
  end

  def max_sequence(remaining, 1, picked) do
    [Enum.max(remaining) | picked] |> Enum.reverse() |> Enum.join() |> String.to_integer()
  end

  def max_sequence(remaining, unpicked, picked) do
    valid_span = Enum.drop(remaining, -(unpicked - 1))

    {battery, index} =
      remaining
      |> Enum.with_index()
      |> Enum.take(length(valid_span))
      |> Enum.max_by(fn {n, _i} -> n end)

    new_slice = Enum.drop(remaining, index + 1)

    max_sequence(new_slice, unpicked - 1, [battery | picked])
  end

  defp parse_input(), do: Klaus.Input.parse_lines(3, &String.graphemes/1)
end
