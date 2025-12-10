defmodule Day10 do
  use Dantzig.Polynomial.Operators
  alias Dantzig.{Problem, Constraint, Solution, Polynomial}

  def part1() do
    Klaus.Input.parse_lines(10, &parse_line/1)
    |> Enum.map(fn {goal, buttons, _} ->
      initial = List.duplicate(false, length(goal))
      visited = MapSet.new([initial])
      search([{initial, 0}], buttons, goal, visited)
    end)
    |> Enum.sum()
  end

  def part2() do
    Klaus.Input.parse_lines(10, &parse_line/1)
    |> Enum.map(fn {_, buttons, joltages} ->
      solve_ilp(buttons, joltages)
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

  # General solver approach is:
  # b0,b1,b2,b3 ... is the number of times we press each button
  # we are aiming to minimise b0 + b1 + b2 + ... which is the sum of all buttons pressed
  # out constraints are based on how the buttons affect the different joltages, and what the target joltage is for each position
  defp solve_ilp(buttons, joltages) do
    problem = Problem.new(direction: :minimize)

    {problem, button_vars} =
      buttons
      |> Enum.with_index()
      |> Enum.reduce({problem, []}, fn {_button, idx}, {prob, vars} ->
        # Create a variable which is the number of times the button is pressed, buttons must be pressed 0 or more times, never negative
        {prob, var} = Problem.new_variable(prob, "b#{idx}", min: 0, type: :integer)
        {prob, [var | vars]}
      end)

    button_vars = Enum.reverse(button_vars)

    problem =
      joltages
      |> Enum.with_index()
      |> Enum.reduce(problem, fn {target_joltage, joltage_idx}, prob ->
        left =
          buttons
          |> Enum.zip(button_vars)
          |> Enum.reduce(Polynomial.const(0), fn {affected_joltage, var}, acc ->
            if joltage_idx in affected_joltage do
              acc + var
            else
              acc
            end
          end)

        # Essentially, for a joltage indicator we would say something like: b4 + b5 = 5
        # In that example it means the number of button 4 pressed plus button 5 presses must achieve the goal of 5 joltage
        # Each button press increases the specified joltages by 1, so the count of whatever buttons affect that joltage is good enough
        constraint = Constraint.new_linear(left, :==, target_joltage)
        Problem.add_constraint(prob, constraint)
      end)

    # This is where we define out objective as b1 + b2 + b3 + ...
    objective =
      Enum.reduce(button_vars, Polynomial.const(0), fn var, acc ->
        acc + var
      end)

    # Not sure why we need both to call .minimize and specify direction: :minimize when we created the problem but its what the docs say
    problem = Problem.minimize(problem, objective)

    {:ok, solution} = Dantzig.solve(problem)

    button_vars
    |> Enum.map(&Solution.evaluate(solution, &1))
    |> Enum.map(&round/1)
    |> Enum.sum()
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

    parsed_joltages =
      joltage
      |> String.split(",")
      |> Enum.map(&String.to_integer/1)

    {parsed_goal, parsed_numbers, parsed_joltages}
  end
end
