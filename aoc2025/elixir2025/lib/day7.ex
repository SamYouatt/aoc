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

  def part2() do
    manifold =
      Klaus.Input.read!(7)
      |> Klaus.Grid.from_string(fn
        "S" -> 1
        "." -> 0
        _ -> "^"
      end)

    0..(manifold.height - 2)
    |> Enum.reduce(manifold, fn iteration, man ->
      advance_quantum(man, iteration)
    end)
    |> Klaus.Grid.row(manifold.height - 1)
    |> Enum.sum()
  end

  defp advance_manifold(manifold, iteration) do
    manifold
    |> Klaus.Grid.row_coords(iteration)
    |> Enum.filter(fn {_, val} -> val == "S" || val == "|" end)
    |> Enum.map(&elem(&1, 0))
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

  defp advance_quantum(manifold, iteration) do
    manifold
    |> Klaus.Grid.row_coords(iteration)
    |> Enum.filter(fn {_, val} -> is_integer(val) && val != 0 end)
    |> Enum.reduce(manifold, fn {{x, y}, val}, man ->
      case Klaus.Grid.get(man, {x, y + 1}) do
        0 ->
          Klaus.Grid.put(man, {x, y + 1}, val)

        # ASSUMPTION: I won't have to do bounds checking
        "^" ->
          man
          |> Klaus.Grid.update({x - 1, y + 1}, 0, fn x -> x + val end)
          |> Klaus.Grid.update({x + 1, y + 1}, 0, fn x -> x + val end)

        _ ->
          Klaus.Grid.update(man, {x, y + 1}, 0, fn x -> x + val end)
      end
    end)
  end
end
