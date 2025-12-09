defmodule Day9 do
  def part1() do
    tiles =
      Klaus.Input.parse_lines(9, &parse_tile/1)

    areas = for a <- tiles, b <- tiles, a < b, do: area(a, b)

    Enum.max(areas)
  end

  defp area({x1, y1}, {x2, y2}), do: abs(x2 - x1 + 1) * abs(y2 - y1 + 1)

  defp parse_tile(line) do
    line
    |> String.split(",", parts: 2)
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
  end
end
