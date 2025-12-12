defmodule Day12 do
  def part1() do
    {present_sizes, reqs} = parse_input()

    reqs
    |> Enum.count(fn {area, counts} ->
      min_required_space =
        Enum.zip_reduce(counts, present_sizes, 0, fn count, present, acc ->
          acc + count * present
        end)

      min_required_space <=
        area
    end)
  end

  defp parse_input() do
    {presents, [reqs]} =
      Klaus.Input.read!(12) |> String.split("\n\n", trim: true) |> Enum.split(-1)

    present_sizes = presents |> Enum.map(&String.count(&1, "#"))

    reqs = reqs |> String.split("\n", trim: true) |> Enum.map(&parse_requirement/1)

    {present_sizes, reqs}
  end

  def parse_requirement(req) do
    [size, counts] = String.split(req, ": ")

    area = size |> String.split("x") |> Enum.map(&String.to_integer/1) |> Enum.product()
    counts = counts |> String.split(" ", trim: true) |> Enum.map(&String.to_integer/1)

    {area, counts}
  end
end
