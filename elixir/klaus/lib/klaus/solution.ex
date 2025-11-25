defmodule Klaus.Solution do
  @type input :: String.t()
  @type answer :: String.t() | integer() | float()

  @callback part1(input) :: answer()
  @callback part2(input) :: answer()

  @optional_callbacks part2: 1
end
