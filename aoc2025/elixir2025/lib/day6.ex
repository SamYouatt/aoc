defmodule Day6 do
  def part1() do
    Klaus.Input.parse_lines(6, &parse_line/1)
    |> Enum.zip_with(&Function.identity/1)
    |> Enum.map(&Enum.reverse/1)
    |> Enum.sum_by(&calculate_expression/1)
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

  defp parse_line(line), do: String.split(line, ~r/\s+/, trim: true)
end
