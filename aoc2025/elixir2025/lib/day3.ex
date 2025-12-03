defmodule Day3 do
  def part1() do
    Klaus.Input.parse_lines(3, &String.graphemes/1)
    |> Enum.sum_by(fn bank ->
      bank |> largest_combo() |> String.to_integer()
    end)
  end

  def largest_combo(bank) do
    0..(length(bank) - 2)
    |> Enum.map(fn first ->
      Enum.at(bank, first) <> largest_after(bank, first)
    end)
    |> Enum.max()
  end

  def largest_after(batteries, index), do: Enum.max(Enum.drop(batteries, index + 1))
end
