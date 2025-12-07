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

  @doc "Set a complete row in the grid to the given list"
  @spec put_row(t(), integer(), [any()]) :: t()
  def put_row(%__MODULE__{data: data} = grid, y, row) do
    new_data =
      row
      |> Enum.with_index()
      |> Enum.reduce(data, fn {val, x}, acc ->
        Map.put(acc, {x, y}, val)
      end)

    %{grid | data: new_data}
  end

  @spec update(t(), coord(), any(), (any() -> any())) :: t()
  def update(%__MODULE__{data: data} = grid, coord, default, fun) do
    %{grid | data: Map.update(data, coord, default, fun)}
  end

  @spec map(t(), (any() -> any())) :: t()
  def map(%__MODULE__{data: data} = grid, mapping_fun) do
    new_data = Map.new(data, fn {coord, val} -> {coord, mapping_fun.(val)} end)
    %{grid | data: new_data}
  end

  @spec in_bounds?(t(), coord()) :: boolean()
  def in_bounds?(%__MODULE__{width: width, height: height}, {x, y}) do
    x >= 0 and x < width and y >= 0 and y < height
  end

  @doc "Get all the values in a single row at y"
  @spec row(t(), integer()) :: [any()]
  def row(%__MODULE__{data: data, width: width}, y) do
    for x <- 0..(width - 1), do: Map.get(data, {x, y})
  end

  @doc "Get all the values and coords in a single row at y"
  @spec row_coords(t(), integer()) :: [{coord(), any()}]
  def row_coords(%__MODULE__{data: data, width: width}, y) do
    for x <- 0..(width - 1), do: {{x, y}, Map.get(data, {x, y})}
  end

  @doc "Get all row as list of lists"
  @spec rows(t()) :: [[any()]]
  def rows(%__MODULE__{height: height} = grid) do
    for y <- 0..(height - 1), do: row(grid, y)
  end

  @spec filter(t(), (coord(), any() -> boolean())) :: [{coord(), any()}]
  def filter(%__MODULE__{data: data}, pred) do
    Enum.filter(data, fn {coord, val} -> pred.(coord, val) end)
  end

  @spec count(t(), (coord(), any() -> boolean())) :: non_neg_integer()
  def count(%__MODULE__{data: data}, pred) do
    Enum.count(data, fn {coord, val} -> pred.(coord, val) end)
  end

  @neighbours [{-1, 0}, {1, 0}, {0, -1}, {0, 1}]
  @all_neighbours for dy <- -1..1, dx <- -1..1, {dx, dy} != {0, 0}, do: {dx, dy}

  @doc "Get all the values of in bound neighbours. Default is direct neighbours. :any can be specified to include diagonals"
  @spec neighbours(t(), coord(), :direct | :all) :: [any()]
  def neighbours(grid, coord, mode \\ :direct)

  def neighbours(%__MODULE__{} = grid, {x, y}, :direct) do
    @neighbours
    |> Enum.map(fn {dx, dy} -> {x + dx, y + dy} end)
    |> Enum.filter(&in_bounds?(grid, &1))
    |> Enum.map(fn c -> get(grid, c) end)
  end

  def neighbours(%__MODULE__{} = grid, {x, y}, :all) do
    @all_neighbours
    |> Enum.map(fn {dx, dy} -> {x + dx, y + dy} end)
    |> Enum.filter(&in_bounds?(grid, &1))
    |> Enum.map(fn c -> get(grid, c) end)
  end

  @spec to_string(t(), (any() -> String.t())) :: String.t()
  def to_string(%__MODULE__{} = grid, formatter \\ &Kernel.to_string/1) do
    grid
    |> rows()
    |> Enum.map(fn row -> Enum.map_join(row, "", formatter) end)
    |> Enum.join("\n")
  end

  defimpl String.Chars do
    def to_string(grid), do: Klaus.Grid.to_string(grid)
  end
end
