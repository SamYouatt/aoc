defmodule Klaus.Grid do
  defstruct [:data, :width, :height]

  @type coord :: {integer(), integer()}
  @type t :: %__MODULE__{
          data: %{coord() => any()},
          width: non_neg_integer(),
          height: non_neg_integer()
        }

  @spec from_nested_list([[any()]]) :: t()
  def from_nested_list([first_row | _] = rows) do
    data =
      for {row, y} <- Enum.with_index(rows),
          {val, x} <- Enum.with_index(row),
          into: %{} do
        {{x, y}, val}
      end

    %__MODULE__{
      data: data,
      width: length(first_row),
      height: length(rows)
    }
  end

  @spec from_string(String.t(), (String.t() -> any())) :: t()
  def from_string(string, mapping_fun \\ &Function.identity/1) do
    string
    |> String.trim()
    |> String.split("\n")
    |> Enum.map(fn line ->
      line |> String.graphemes() |> Enum.map(mapping_fun)
    end)
    |> from_nested_list()
  end

  @spec get(t(), coord(), any()) :: any()
  def get(%__MODULE__{data: data}, coord, default \\ nil) do
    Map.get(data, coord, default)
  end

  @spec put(t(), coord(), any()) :: t()
  def put(%__MODULE__{data: data} = grid, coord, value) do
    %{grid | data: Map.put(data, coord, value)}
  end

  @spec update(t(), coord(), any(), (any() -> any())) :: t()
  def update(%__MODULE__{data: data} = grid, coord, default, fun) do
    %{grid | data: Map.update(data, coord, default, fun)}
  end
end
