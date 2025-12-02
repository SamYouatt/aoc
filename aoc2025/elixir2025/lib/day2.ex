defmodule Day2 do
  def part1() do
    Klaus.Input.read!(2)
    |> String.split(",")
    |> Enum.map(&parse_range/1)
    |> Enum.sum_by(&find_patterns/1)
  end

  def part2() do
    Klaus.Input.read!(2)
    |> String.split(",")
    |> Enum.map(&parse_range/1)
    |> Enum.sum_by(&find_complex_patterns/1)
  end

  defp parse_range(range) do
    [start, last] = String.split(range, "-", parts: 2)
    {start, last}
  end

  defp find_patterns({start, last}) do
    range(start, last)
    |> Enum.sum_by(fn x ->
      as_text = Integer.to_string(x)
      len = String.length(as_text)
      mid = div(len, 2)

      first_half = String.slice(as_text, 0, mid)
      second_half = String.slice(as_text, mid..-1//1)

      if first_half == second_half, do: x, else: 0
    end)
  end

  defp find_complex_patterns({start, last}) do
    range(start, last)
    |> Enum.sum_by(fn id ->
      as_text = Integer.to_string(id)
      len = String.length(as_text)
      mid = div(len, 2)

      has_pattern =
        Enum.any?(1..mid//1, fn chunk_size ->
          all_chunks_match(as_text, chunk_size)
        end)

      if has_pattern, do: id, else: 0
    end)
  end

  defp to_chunks(id, length) do
    id
    |> String.codepoints()
    |> Enum.chunk_every(length)
    |> Enum.map(&Enum.join/1)
  end

  defp range(start, last), do: String.to_integer(start)..String.to_integer(last)

  defp all_chunks_match(id, size) do
    chunks = to_chunks(id, size)
    length(chunks) > 1 && length(Enum.uniq(chunks)) == 1
  end
end
