defmodule Brainjuice.Tape do
  alias Brainjuice.Tape

  defstruct data: %{}, head: 0

  def inc(%Tape{} = tape), do: write(tape, read(tape) + 1)
  def dec(%Tape{} = tape), do: write(tape, read(tape) - 1)

  def right(%Tape{} = tape), do: %Tape{tape | head: tape.head + 1}

  def left(%Tape{head: 0}), do: raise(ArgumentError, "tape head moved left of 0")
  def left(%Tape{} = tape), do: %Tape{tape | head: tape.head - 1}

  def read(%Tape{} = tape), do: Map.get(tape.data, tape.head, 0)

  def write(%Tape{} = tape, value) when is_integer(value) do
    data = Map.put(tape.data, tape.head, mod(value))
    %Tape{tape | data: data}
  end

  defp mod(num), do: Integer.mod(num, 256)
end
