defmodule Day9 do
  def part1() do
    Klaus.Input.parse_lines(9, &parse_tile/1)
    |> then(fn tiles ->
      for a <- tiles, b <- tiles, a < b, do: area(a, b)
    end)
    |> Enum.max()
  end

  def part2() do
    red_tiles = Klaus.Input.parse_lines(9, &parse_tile/1)

    green_edges =
      Enum.chunk_every(red_tiles, 2, 1, [hd(red_tiles)])
      |> Enum.map(fn [a, b] -> normalise(a, b) end)

    for a <- red_tiles, b <- red_tiles, a < b do
      {norm_a, norm_b} = normalise(a, b)
      {norm_a, norm_b, area(a, b)}
    end
    |> Enum.sort_by(fn {_, _, area} -> area end, :desc)
    |> Enum.find_value(fn {a, b, area} ->
      if !rect_intersected?(a, b, green_edges), do: area
    end)
  end

  defp rect_intersected?({x1, y1}, {x2, y2}, edges) do
    Enum.any?(edges, fn {{p, q}, {r, s}} ->
      p < x2 and q < y2 and r > x1 and s > y1
    end)
  end

  # Given any two opposite corners, work out the top right and bottom left of that rectangle
  defp normalise({x1, y1}, {x2, y2}), do: {{min(x1, x2), min(y1, y2)}, {max(x1, x2), max(y1, y2)}}

  defp area({x1, y1}, {x2, y2}), do: (abs(x2 - x1) + 1) * (abs(y2 - y1) + 1)

  defp parse_tile(line) do
    line
    |> String.split(",", parts: 2)
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
  end
end
