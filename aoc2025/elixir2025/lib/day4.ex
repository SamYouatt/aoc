defmodule Day4 do
  def part1() do
    Klaus.Input.read!(4)
    |> Klaus.Grid.from_string()
    |> accessible_rolls()
    |> Enum.count()
  end

  def part2() do
    Klaus.Input.read!(4)
    |> Klaus.Grid.from_string()
    |> reduce_rolls(0)
  end

  defp accessible_rolls(grid) do
    grid
    |> Klaus.Grid.filter(fn coord, val ->
      paper_neighbours =
        grid
        |> Klaus.Grid.neighbours(coord, :all)
        |> Enum.count(&(&1 == "@"))

      val == "@" && paper_neighbours < 4
    end)
  end

  defp reduce_rolls(grid, removed) do
    accessible = accessible_rolls(grid)

    if length(accessible) == 0 do
      removed
    else
      updated_grid =
        accessible
        |> Enum.reduce(grid, fn {coord, _val}, acc_grid ->
          Klaus.Grid.put(acc_grid, coord, ".")
        end)

      reduce_rolls(updated_grid, removed + length(accessible))
    end
  end
end
