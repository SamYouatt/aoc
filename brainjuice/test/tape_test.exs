defmodule TapeTest do
  use ExUnit.Case

  describe "inc" do
    test "inc on init gives 1" do
      tape = Tape.inc(%Tape{})
      assert tape.data == %{0 => 1}
    end

    test "inc on x gives x + 1" do
      tape = Tape.inc(%Tape{data: %{0 => 2}})
      assert tape.data == %{0 => 3}
    end

    test "inc on 255 wraps" do
      tape = Tape.inc(%Tape{data: %{0 => 255}})
      assert tape.data == %{0 => 0}
    end

    test "inc changes only the cell under the head" do
      tape = Tape.inc(%Tape{head: 1, data: %{0 => 5}})
      assert tape.data == %{0 => 5, 1 => 1}
    end
  end

  describe "dec" do
    test "dec on init gives 255" do
      tape = Tape.dec(%Tape{})
      assert tape.data == %{0 => 255}
    end

    test "dec on 0 wraps to 255" do
      tape = Tape.dec(%Tape{data: %{0 => 0}})
      assert tape.data == %{0 => 255}
    end

    test "dec on x gives x - 1" do
      tape = Tape.dec(%Tape{data: %{0 => 2}})
      assert tape.data == %{0 => 1}
    end

    test "dec changes only the cell under the head" do
      tape = Tape.dec(%Tape{head: 1, data: %{0 => 5, 1 => 3}})
      assert tape.data == %{0 => 5, 1 => 2}
    end
  end
end
