defmodule Day8 do
  def part1() do
    {initial_circuits, pairs} = setup()

    0..999
    |> Enum.reduce({initial_circuits, pairs}, fn _, {circuits, [{a, b} | rem_pairs]} ->
      {merge_circuits(circuits, a, b), rem_pairs}
    end)
    |> then(&elem(&1, 0))
    |> Enum.sort_by(&MapSet.size/1, :desc)
    |> Enum.map(&MapSet.size/1)
    |> Enum.take(3)
    |> Enum.product()
  end

  def part2() do
    {initial_circuits, pairs} = setup()
    collapse_circuits(initial_circuits, pairs)
  end

  defp collapse_circuits(circuits, [{a, b} | rem]) do
    new_circuits = merge_circuits(circuits, a, b)

    if length(new_circuits) == 1,
      do: elem(a, 0) * elem(b, 0),
      else: collapse_circuits(new_circuits, rem)
  end

  defp merge_circuits(circuits, a, b) do
    found_a = containing(circuits, a)
    found_b = containing(circuits, b)

    if found_a == found_b do
      circuits
    else
      circuits
      |> List.delete(found_a)
      |> List.delete(found_b)
      |> then(fn rest -> [MapSet.union(found_a, found_b) | rest] end)
    end
  end

  defp setup do
    boxes = Klaus.Input.parse_lines(8, &parse_box/1)

    pairs =
      boxes
      |> then(fn boxes ->
        for a <- boxes,
            b <- boxes,
            a < b,
            do: {a, b, distance(a, b)}
      end)
      |> Enum.sort_by(&elem(&1, 2))
      |> Enum.map(fn {a, b, _} -> {a, b} end)

    initial_circuits = Enum.map(boxes, &MapSet.new([&1]))

    {initial_circuits, pairs}
  end

  defp distance({x1, y1, z1}, {x2, y2, z2}),
    do: :math.sqrt(:math.pow(x2 - x1, 2) + :math.pow(y2 - y1, 2) + :math.pow(z2 - z1, 2))

  defp containing(circuits, box), do: Enum.find(circuits, nil, &MapSet.member?(&1, box))

  defp parse_box(line) do
    line
    |> String.split(",", parts: 3)
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
  end
end
