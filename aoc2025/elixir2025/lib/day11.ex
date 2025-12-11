defmodule Day11 do
  def part1() do
    graph =
      Klaus.Input.parse_lines(11, &parse_line/1)
      |> Enum.reduce(%{}, fn {node, edges}, graph ->
        Map.put(graph, node, edges)
      end)

    traverse(graph, "you", MapSet.new())
  end

  defp traverse(_graph, "out", _visited), do: 1

  defp traverse(graph, current, visited) do
    next = Map.get(graph, current)
    new_visited = MapSet.put(visited, current)

    next
    |> Enum.filter(fn n -> not MapSet.member?(visited, n) end)
    |> Enum.reduce(0, fn n, count ->
      count + traverse(graph, n, new_visited)
    end)
  end

  defp parse_line(line) do
    [node, edges] = String.split(line, ": ", trim: true)
    {node, String.split(edges, " ", trim: true)}
  end
end
