defmodule Day7 do
  def part1() do
    manifold =
      Klaus.Input.read!(7)
      |> Klaus.Grid.from_string()

    {_final_man, splits} =
      0..(manifold.height - 2)
      |> Enum.reduce({manifold, 0}, fn iteration, {man, acc} ->
        {new_man, splits} = advance_manifold(man, iteration)
        {new_man, acc + splits}
      end)

    splits
  end

  defp advance_manifold(manifold, iteration) do
    tachyons =
      Klaus.Grid.row_coords(manifold, iteration)
      |> Enum.filter(fn {_, val} -> val == "S" || val == "|" end)
      |> Enum.map(&elem(&1, 0))

    tachyons
    |> Enum.reduce({manifold, 0}, fn {x, y}, {man, splits} ->
      case Klaus.Grid.get(man, {x, y + 1}) do
        "." ->
          new_man = Klaus.Grid.put(man, {x, y + 1}, "|")
          {new_man, splits}

        # ASSUMPTION: I won't have to do bounds checking
        "^" ->
          new_man =
            man
            |> Klaus.Grid.put({x + 1, y + 1}, "|")
            |> Klaus.Grid.put({x - 1, y + 1}, "|")

          {new_man, splits + 1}

        "|" ->
          {man, splits}
      end
    end)
  end
end
