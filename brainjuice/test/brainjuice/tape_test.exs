defmodule Brainjuice.TapeTest do
  use ExUnit.Case

  alias Brainjuice.Tape

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

  describe "right" do
    test "right advances head by 1" do
      tape = Tape.right(%Tape{head: 3})
      assert tape.head == 4
    end
  end

  describe "left" do
    test "left rewinds head by 1" do
      tape = Tape.left(%Tape{head: 3})
      assert tape.head == 2
    end

    test "left at 0 raises" do
      assert_raise ArgumentError, fn -> Tape.left(%Tape{}) end
    end
  end

  describe "read" do
    test "reads the value at the current head" do
      tape = %Tape{data: %{0 => 2, 1 => 3, 2 => 4}, head: 1}
      assert Tape.read(tape) == 3
    end

    test "reads 0 when an unchanged cell" do
      tape = %Tape{data: %{0 => 2, 1 => 3}, head: 2}
      assert Tape.read(tape) == 0
    end
  end

  describe "write" do
    test "writes the value at the current head changing current" do
      tape =
        %Tape{data: %{0 => 2, 1 => 3, 2 => 4}, head: 1}
        |> Tape.write(9)

      assert tape.data == %{0 => 2, 1 => 9, 2 => 4}
    end

    test "writes the value at an unchanged head" do
      tape =
        %Tape{data: %{0 => 2}, head: 1}
        |> Tape.write(6)

      assert tape.data == %{0 => 2, 1 => 6}
    end

    test "wraps big inputs before writing" do
      tape =
        %Tape{data: %{0 => 2}, head: 0}
        |> Tape.write(278)

      assert tape.data == %{0 => 22}
    end

    test "wraps underflow inputs before writing" do
      tape =
        %Tape{data: %{0 => 2}, head: 0}
        |> Tape.write(-1)

      assert tape.data == %{0 => 255}
    end
  end
end
