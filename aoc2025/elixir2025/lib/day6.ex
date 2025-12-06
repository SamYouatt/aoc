defmodule Day6 do
  def part1() do
    Klaus.Input.parse_lines(6, &parse_line/1)
    |> Enum.zip_with(&Function.identity/1)
    |> Enum.map(&Enum.reverse/1)
    |> Enum.sum_by(&calculate_expression/1)
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
    |> Enum.reject(fn chunk ->
      hd(chunk) |> Enum.all?(&(&1 == " "))
    end)
    |> Enum.map(fn chunk ->
      chunk
      |> Enum.map(fn col ->
        col |> Enum.reject(&(&1 == " ")) |> Enum.join() |> String.to_integer()
      end)
    end)
    |> Enum.zip(parse_line(operators))
    |> Enum.sum_by(&calculate_ceph/1)
  end

  defp calculate_expression([operand | values]) do
    values
    |> Enum.map(&String.to_integer/1)
    |> then(
      case operand do
        "+" -> &Enum.sum/1
        "*" -> &Enum.product/1
      end
    )
  end

  defp calculate_ceph({numbers, "+"}), do: Enum.sum(numbers)
  defp calculate_ceph({numbers, "*"}), do: Enum.product(numbers)

  defp parse_line(line), do: String.split(line, ~r/\s+/, trim: true)
end
