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
  end
end
