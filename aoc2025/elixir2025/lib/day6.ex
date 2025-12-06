defmodule Day6 do
  def part1() do
    Klaus.Input.parse_lines(6, &parse_line/1)
    |> Enum.zip_with(&Function.identity/1)
    |> Enum.map(&Enum.reverse/1)
    |> Enum.map(fn [operator | values] ->
      {Enum.map(values, &String.to_integer/1), operator}
    end)
    |> Enum.sum_by(&calc/1)
  end

  def part2() do
    {numbers, [operators]} =
      Klaus.Input.read_raw!(6)
      |> String.split("\n")
      # drop trailing \n
      |> Enum.drop(-1)
      |> Enum.split(-1)

    numbers
    |> Enum.map(&String.graphemes/1)
    |> Enum.zip_with(&Function.identity/1)
    |> Enum.chunk_by(fn col -> Enum.all?(col, &(&1 == " ")) end)
    |> Enum.reject(fn [x | _] -> Enum.all?(x, &(&1 == " ")) end)
    |> Enum.map(&parse_expression/1)
    |> Enum.zip(parse_line(operators))
    |> Enum.sum_by(&calc/1)
  end

  defp parse_expression(expression) do
    Enum.map(expression, fn col ->
      col
      |> Enum.reject(&(&1 == " "))
      |> Enum.join()
      |> String.to_integer()
    end)
  end

  defp calc({numbers, "+"}), do: Enum.sum(numbers)
  defp calc({numbers, "*"}), do: Enum.product(numbers)

  defp parse_line(line), do: String.split(line, ~r/\s+/, trim: true)
end
