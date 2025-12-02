defmodule Day2 do
  def part1() do
    Klaus.Input.read!(2)
    |> String.split(",")
    |> Enum.map(&parse_range/1)
    |> Enum.sum_by(&find_patterns/1)
  end

  defp parse_range(range) do
    [start, last] = String.split(range, "-", parts: 2)
    {start, last}
  end

  defp find_patterns({start, last}) do
    String.to_integer(start)..String.to_integer(last)
    |> Enum.reduce(0, fn x, acc ->
      as_text = Integer.to_string(x)
      len = String.length(as_text)
      mid = div(len, 2)

      first_half = String.slice(as_text, 0, mid)
      second_half = String.slice(as_text, mid..-1//1)

      if first_half == second_half, do: acc + x, else: acc
    end)
  end
end
