defmodule Brainjuice.ParserTest do
  use ExUnit.Case

  alias Brainjuice.Parser
  alias Brainjuice.Program

  describe "parse" do
    test "parses each command in order" do
      program = Parser.parse(">+<-,.")
      assert program.ops == {:right, :inc, :left, :dec, :input, :output}
      assert program.jumps == %{}
    end

    test "ignores comment characters" do
      program = Parser.parse("a+ b\n-c")
      assert program.ops == {:inc, :dec}
      assert program.jumps == %{}
    end

    test "parses an empty program to no commands" do
      assert Parser.parse("") == %Program{ops: {}, jumps: %{}, positions: {}}
    end

    test "parses a comment-only program to no commands" do
      assert Parser.parse("hello world\n") == %Program{ops: {}, jumps: %{}, positions: {}}
    end
  end

  describe "parse loops" do
    test "pairs a loop's brackets in both directions" do
      program = Parser.parse("+[-]+")
      assert program.ops == {:inc, :jump_if_zero, :dec, :jump_not_zero, :inc}
      assert program.jumps == %{1 => 3, 3 => 1}
    end

    test "pairs an empty loop" do
      program = Parser.parse("[]")
      assert program.ops == {:jump_if_zero, :jump_not_zero}
      assert program.jumps == %{0 => 1, 1 => 0}
    end

    test "pairs nested loops inner to inner and outer to outer" do
      program = Parser.parse("[[-]]")
      assert program.ops == {:jump_if_zero, :jump_if_zero, :dec, :jump_not_zero, :jump_not_zero}
      assert program.jumps == %{0 => 4, 4 => 0, 1 => 3, 3 => 1}
    end

    test "pairs sibling loops separately" do
      program = Parser.parse("[-][+]")

      assert program.ops ==
               {:jump_if_zero, :dec, :jump_not_zero, :jump_if_zero, :inc, :jump_not_zero}

      assert program.jumps == %{0 => 2, 2 => 0, 3 => 5, 5 => 3}
    end

    test "indexes jumps by op, not by character" do
      program = Parser.parse("a[ - ]")
      assert program.ops == {:jump_if_zero, :dec, :jump_not_zero}
      assert program.jumps == %{0 => 2, 2 => 0}
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

  describe "parse positions" do
    test "records the source offset of each op, counting comments" do
      assert Parser.parse("a+ b\n-c").positions == {1, 5}
    end

    test "records offsets for loops alongside their jumps" do
      assert Parser.parse("a[ - ]") == %Program{
               ops: {:jump_if_zero, :dec, :jump_not_zero},
               jumps: %{0 => 2, 2 => 0},
               positions: {1, 3, 5}
             }
    end

    test "counts offsets in bytes, not characters" do
      assert Parser.parse("é+").positions == {2}
    end
  end
end
