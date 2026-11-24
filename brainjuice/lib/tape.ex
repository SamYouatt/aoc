defmodule Tape do
  defstruct data: %{}, head: 0

  def inc(%Tape{} = tape) do
    data = Map.update(tape.data, tape.head, 1, &mod(&1 + 1))
    %Tape{tape | data: data}
  end

  def dec(%Tape{} = tape) do
    data = Map.update(tape.data, tape.head, 255, &mod(&1 - 1))
    %Tape{tape | data: data}
  end

  defp mod(num), do: Integer.mod(num, 256)
end
