defmodule Day01 do
  def part1() do
    {origins, _dial} =
      turns()
      |> Enum.reduce({0, 50}, fn move, {origins, current} ->
        new_pos = Integer.mod(current + move, 100)
        new_origins = origins + if new_pos == 0, do: 1, else: 0
        {new_origins, new_pos}
      end)

    origins
  end

  def part2() do
    0
  end

  defp turns() do
    Klaus.Input.read!(1)
    |> String.split("\n", trim: true)
    |> Enum.map(&parse_line/1)
  end

  defp parse_line("R" <> turns), do: String.to_integer(turns)
  defp parse_line("L" <> turns), do: -String.to_integer(turns)
end
