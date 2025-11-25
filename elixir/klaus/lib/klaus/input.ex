defmodule Klaus.Input do
  @spec read!(integer()) :: String.t()
  def read!(day) do
    day
    |> Integer.to_string()
    |> String.pad_leading(2, "0")
    |> then(&"inputs/day#{&1}.txt")
    |> File.read!()
  end
end
