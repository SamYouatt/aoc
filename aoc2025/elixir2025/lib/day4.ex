defmodule Day4 do
  def part1() do
    grid =
      Klaus.Input.read!(4)
      |> Klaus.Grid.from_string()

    Klaus.Grid.count(grid, fn coord, val ->
      val == "@" && count_paper_neighbours(grid, coord) < 4
    end)
  end

  defp count_paper_neighbours(grid, coord) do
    grid
    |> Klaus.Grid.neighbours(coord, :all)
    |> Enum.count(&(&1 == "@"))
  end
end
