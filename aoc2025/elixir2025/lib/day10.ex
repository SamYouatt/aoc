defmodule Day10 do
  def part1() do
    Klaus.Input.parse_lines(10, &parse_line/1)
    |> Enum.map(fn {goal, buttons, _} ->
      initial = List.duplicate(false, length(goal))
      visited = MapSet.new([initial])
      search([{initial, 0}], buttons, goal, visited)
    end)
    |> Enum.sum()
  end

  defp search([{indicator, depth} | _rest], _buttons, goal, _visited) when indicator == goal do
    depth
  end

  defp search([], _buttons, _goal, _visited), do: raise("Something wrong we ran out of states")

  defp search([{indicator, depth} | rest], buttons, goal, visited) do
    next_states =
      buttons
      |> Enum.map(fn mask -> {flip(indicator, mask), depth + 1} end)
      |> Enum.reject(fn {indicators, _} -> MapSet.member?(visited, indicators) end)

    new_visited =
      Enum.reduce(next_states, visited, fn {state, _}, acc -> MapSet.put(acc, state) end)

    search(rest ++ next_states, buttons, goal, new_visited)
  end

  defp flip(indicators, mask) do
    Enum.reduce(mask, indicators, fn index, acc ->
      List.update_at(acc, index, &(!&1))
    end)
  end

  defp parse_line(line) do
    regex = ~r/\[([.#]+)\]\s+((?:\([0-9,]+\)\s*)+)\{([0-9,]+)\}/
    [_, goal, buttons, joltage] = Regex.run(regex, line)

    parsed_goal =
      goal
      |> String.graphemes()
      |> Enum.map(fn
        "#" -> true
        "." -> false
      end)

    parsed_numbers =
      Regex.scan(~r/\(([0-9,]+)\)/, buttons)
      |> Enum.map(fn [_, match] ->
        match
        |> String.split(",")
        |> Enum.map(&String.to_integer/1)
      end)

    {parsed_goal, parsed_numbers, joltage}
  end
end
