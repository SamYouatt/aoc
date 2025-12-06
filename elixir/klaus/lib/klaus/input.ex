defmodule Klaus.Input do
  @doc """
  Reads the given day input. Expects inputs in `/inputs` of the format `dayXX.txt`.
  """
  @spec read!(integer()) :: String.t()
  def read!(day) do
    day
    |> Integer.to_string()
    |> String.pad_leading(2, "0")
    |> then(&"inputs/day#{&1}.txt")
    |> File.read!()
    |> String.trim()
  end

  @doc """
  Reads the given day as input but doesn't perform any trimming
  """
  @spec read_raw!(integer()) :: String.t()
  def read_raw!(day) do
    day
    |> Integer.to_string()
    |> String.pad_leading(2, "0")
    |> then(&"inputs/day#{&1}.txt")
    |> File.read!()
  end

  @doc """
  Reads the given day input and maps a function over each line.
  Expects inputs in `/inputs` of the format `dayXX.txt`.
  """
  @spec parse_lines(integer(), (String.t() -> any())) :: list()
  def parse_lines(day, parser_fn) do
    day
    |> read!()
    |> String.split("\n", trim: true)
    |> Enum.map(parser_fn)
  end
end
