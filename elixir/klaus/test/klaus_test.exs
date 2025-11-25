defmodule KlausTest do
  use ExUnit.Case
  doctest Klaus

  test "greets the world" do
    assert Klaus.hello() == :world
  end
end
