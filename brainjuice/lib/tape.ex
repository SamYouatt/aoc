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

  def right(%Tape{} = tape), do: %Tape{tape | head: tape.head + 1}

  def left(%Tape{head: 0}), do: raise(ArgumentError, "tape head moved left of 0")
  def left(%Tape{} = tape), do: %Tape{tape | head: tape.head - 1}

  def read(%Tape{} = tape), do: Map.get(tape.data, tape.head, 0)

  defp mod(num), do: Integer.mod(num, 256)
end
