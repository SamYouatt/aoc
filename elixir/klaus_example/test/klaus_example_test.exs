defmodule KlausExampleTest do
  use ExUnit.Case
  doctest KlausExample

  test "greets the world" do
    assert KlausExample.hello() == :world
  end
end
