defmodule Day8 do
  def part1() do
    sorted_pairs =
      Klaus.Input.parse_lines(8, &parse_box/1)
      |> then(fn boxes ->
        for a <- boxes,
            b <- boxes,
            a < b,
            do: {a, b, distance(a, b)}
      end)
      |> Enum.sort_by(&elem(&1, 2))
      |> Enum.map(fn {a, b, _} -> {a, b} end)

    0..999
    |> Enum.reduce({[], sorted_pairs}, fn _, {circuits, [{a, b} | rem_pairs]} ->
      new_circuits =
        case {containing(circuits, a), containing(circuits, b)} do
          {nil, nil} ->
            [MapSet.new([a, b]) | circuits]

          {found_a, nil} ->
            circuits
            |> List.delete(found_a)
            |> then(fn rest -> [MapSet.put(found_a, b) | rest] end)

          {nil, found_b} ->
            circuits
            |> List.delete(found_b)
            |> then(fn rest -> [MapSet.put(found_b, a) | rest] end)

          {found_a, found_b} when found_a == found_b ->
            circuits

          {found_a, found_b} ->
            circuits
            |> List.delete(found_a)
            |> List.delete(found_b)
            |> then(fn rest -> [MapSet.union(found_a, found_b) | rest] end)
        end

      {new_circuits, rem_pairs}
    end)
    |> then(&elem(&1, 0))
    |> Enum.sort_by(&MapSet.size/1, :desc)
    |> Enum.map(&MapSet.size/1)
    |> Enum.take(3)
    |> Enum.product()
  end

  def part2() do
    boxes = Klaus.Input.parse_lines(8, &parse_box/1)

    sorted_pairs =
      boxes
      |> then(fn boxes ->
        for a <- boxes,
            b <- boxes,
            a < b,
            do: {a, b, distance(a, b)}
      end)
      |> Enum.sort_by(&elem(&1, 2))
      |> Enum.map(fn {a, b, _} -> {a, b} end)

    initial_sets = Enum.map(boxes, &MapSet.new([&1]))

    collapse_circuits(initial_sets, sorted_pairs)
  end

  defp collapse_circuits(circuits, [{a, b} | rem]) do
    new_circuits =
      case {containing(circuits, a), containing(circuits, b)} do
        {nil, nil} ->
          [MapSet.new([a, b]) | circuits]

        {found_a, nil} ->
          circuits
          |> List.delete(found_a)
          |> then(fn rest -> [MapSet.put(found_a, b) | rest] end)

        {nil, found_b} ->
          circuits
          |> List.delete(found_b)
          |> then(fn rest -> [MapSet.put(found_b, a) | rest] end)

        {found_a, found_b} when found_a == found_b ->
          circuits

        {found_a, found_b} ->
          circuits
          |> List.delete(found_a)
          |> List.delete(found_b)
          |> then(fn rest -> [MapSet.union(found_a, found_b) | rest] end)
      end

    if length(new_circuits) == 1 do
      elem(a, 0) * elem(b, 0)
    else
      collapse_circuits(new_circuits, rem)
    end
  end

  defp parse_box(line) do
    line
    |> String.split(",", parts: 3)
    |> Enum.map(&String.to_integer/1)
    |> List.to_tuple()
  end

  defp distance({x1, y1, z1}, {x2, y2, z2}),
    do: :math.sqrt(:math.pow(x2 - x1, 2) + :math.pow(y2 - y1, 2) + :math.pow(z2 - z1, 2))

  defp containing(circuits, box), do: Enum.find(circuits, nil, &MapSet.member?(&1, box))
end
