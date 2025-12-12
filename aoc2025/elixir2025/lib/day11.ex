defmodule Day11 do
  def part1() do
    Klaus.Input.parse_lines(11, &parse_line/1)
    |> Enum.into(%{})
    |> traverse("you", MapSet.new())
  end

  def part2() do
    Klaus.Input.parse_lines(11, &parse_line/1)
    |> Enum.into(%{})
    |> dag_traverse("svr", false, false, %{})
    |> then(&elem(&1, 0))
  end

  defp traverse(_graph, "out", _visited), do: 1

  defp traverse(graph, current, visited) do
    new_visited = MapSet.put(visited, current)

    Map.get(graph, current)
    |> Enum.filter(fn n -> not MapSet.member?(visited, n) end)
    |> Enum.reduce(0, fn n, count ->
      count + traverse(graph, n, new_visited)
    end)
  end

  defp dag_traverse(_graph, "out", seen_dac, seen_fft, cache) do
    result = if seen_dac and seen_fft, do: 1, else: 0
    {result, cache}
  end

  defp dag_traverse(graph, current, seen_dac, seen_fft, cache) do
    key = {current, seen_dac, seen_fft}

    case Map.get(cache, key) do
      nil ->
        new_seen_dac = seen_dac or current == "dac"
        new_seen_fft = seen_fft or current == "fft"

        {result, new_cache} =
          Map.get(graph, current)
          |> Enum.reduce({0, cache}, fn n, {count, acc_cache} ->
            {n_result, updated_cache} =
              dag_traverse(graph, n, new_seen_dac, new_seen_fft, acc_cache)

            {count + n_result, updated_cache}
          end)

        final_cache = Map.put(new_cache, key, result)
        {result, final_cache}

      cached_result ->
        {cached_result, cache}
    end
  end

  defp parse_line(line) do
    [node, edges] = String.split(line, ": ", trim: true)
    {node, String.split(edges, " ", trim: true)}
  end
end
