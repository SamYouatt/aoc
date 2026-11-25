defmodule Brainjuice.ParserTest do
  use ExUnit.Case

  alias Brainjuice.Parser

  describe "parse" do
    test "parses each command in order" do
      assert Parser.parse(">+<-,.") == {{:right, :inc, :left, :dec, :input, :output}, %{}}
    end

    test "ignores comment characters" do
      assert Parser.parse("a+ b\n-c") == {{:inc, :dec}, %{}}
    end

    test "parses an empty program to no commands" do
      assert Parser.parse("") == {{}, %{}}
    end

    test "parses a comment-only program to no commands" do
      assert Parser.parse("hello world\n") == {{}, %{}}
    end
  end

  describe "parse loops" do
    test "pairs a loop's brackets in both directions" do
      assert Parser.parse("+[-]+") ==
               {{:inc, :jump_if_zero, :dec, :jump_not_zero, :inc}, %{1 => 3, 3 => 1}}
    end

    test "pairs an empty loop" do
      assert Parser.parse("[]") == {{:jump_if_zero, :jump_not_zero}, %{0 => 1, 1 => 0}}
    end

    test "pairs nested loops inner to inner and outer to outer" do
      assert Parser.parse("[[-]]") ==
               {{:jump_if_zero, :jump_if_zero, :dec, :jump_not_zero, :jump_not_zero},
                %{0 => 4, 4 => 0, 1 => 3, 3 => 1}}
    end

    test "pairs sibling loops separately" do
      assert Parser.parse("[-][+]") ==
               {{:jump_if_zero, :dec, :jump_not_zero, :jump_if_zero, :inc, :jump_not_zero},
                %{0 => 2, 2 => 0, 3 => 5, 5 => 3}}
    end

    test "indexes jumps by op, not by character" do
      assert Parser.parse("a[ - ]") == {{:jump_if_zero, :dec, :jump_not_zero}, %{0 => 2, 2 => 0}}
    end

    test "raises on an unmatched [" do
      assert_raise ArgumentError, fn -> Parser.parse("[") end
    end

    test "raises on an unmatched ]" do
      assert_raise ArgumentError, fn -> Parser.parse("]") end
    end

    test "raises on brackets in the wrong order" do
      assert_raise ArgumentError, fn -> Parser.parse("][") end
    end
  end
end
