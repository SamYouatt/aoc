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

    for a <- red_tiles, b <- red_tiles, a < b do
      {a, b, area(a, b)}
    end
    |> Enum.sort_by(fn {_, _, area} -> area end, :desc)
    |> Enum.find_value(fn {a, b, area} ->
      if rectangle_in_polygon(a, b, red_tiles) do
        area
      end
    end)
  end

  def rectangle_in_polygon({x1, y1}, {x2, y2}, polygon_vertices) do
    poly_edges = Enum.chunk_every(polygon_vertices, 2, 1, [hd(polygon_vertices)])
    corners = [{x1, y1}, {x1, y2}, {x2, y1}, {x2, y2}]

    rect_edges = [
      {{x1, y1}, {x2, y1}},
      {{x2, y1}, {x2, y2}},
      {{x2, y2}, {x1, y2}},
      {{x1, y2}, {x1, y1}}
    ]

    Enum.all?(corners, &point_inside(&1, poly_edges)) and
      Enum.all?(rect_edges, fn edge ->
        Enum.all?(poly_edges, fn [p1, p2] -> !lines_intersect(edge, {p1, p2}) end)
      end)
  end

  # This is a nasty implementation of the approach to counting boundaries as walking out the shape. If we cross odd boundaries we are inside, even we are outside.
  defp point_inside({x, y} = point, edges) do
    Enum.reduce_while(edges, false, fn [{x1, y1} = edge1, {x2, y2} = edge2], inside ->
      cond do
        # points on lines are considered inside
        point_on_line(point, edge1, edge2) -> {:halt, true}
        # if the edge is horizonal then skip it, ray casting only outwards to the right so this is wasted computation
        y1 > y == y2 > y or y1 == y2 -> {:cont, inside}
        # do the actual rightwards ray casting
        x < (x2 - x1) * (y - y1) / (y2 - y1) + x1 -> {:cont, not inside}
        true -> {:cont, inside}
      end
    end)
  end

  defp point_on_line({x, y} = point, {ex1, ey1} = edge1, {ex2, ey2} = edge2) do
    if cross_prod(edge1, edge2, point) != 0,
      do: false,
      else: x >= min(ex1, ex2) && x <= max(ex1, ex2) && y >= min(ey1, ey2) && y <= max(ey1, ey2)
  end

  defp lines_intersect({a1, a2}, {b1, b2}) do
    b_straddles_a = cross_prod(a1, a2, b1) * cross_prod(a1, a2, b2) < 0
    a_straddles_b = cross_prod(b1, b2, a1) * cross_prod(b1, b2, a2) < 0
    b_straddles_a and a_straddles_b
  end

  # Will return < 0 when x is below/left the line, and >0 when above/right, and 0 when on the line
  defp cross_prod({lx1, ly1}, {lx2, ly2}, {x, y}),
    do: (lx2 - lx1) * (y - ly1) - (ly2 - ly1) * (x - lx1)

  defp area({x1, y1}, {x2, y2}), do: (abs(x2 - x1) + 1) * (abs(y2 - y1) + 1)

  defp parse_tile(line) do
    line
    |> String.split(",", parts: 2)
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
  end
end
