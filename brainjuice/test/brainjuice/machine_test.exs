defmodule Brainjuice.MachineTest do
  use ExUnit.Case

  alias Brainjuice.{Machine, Parser, Tape}

  describe "step" do
    test "+ increments the current cell and advances" do
      m = machine("-+", pc: 1)
      assert Machine.step(m) == %Machine{m | pc: 2, tape: %Tape{data: %{0 => 1}}}
    end

    test "- decrements the current cell and advances" do
      m = machine("+-", pc: 1, tape: %Tape{data: %{0 => 3}})
      assert Machine.step(m) == %Machine{m | pc: 2, tape: %Tape{data: %{0 => 2}}}
    end

    test "> moves the head right and advances" do
      m = machine("+>", pc: 1, tape: %Tape{head: 2})
      assert Machine.step(m) == %Machine{m | pc: 2, tape: %Tape{head: 3}}
    end

    test "< moves the head left and advances" do
      m = machine("+<", pc: 1, tape: %Tape{head: 2})
      assert Machine.step(m) == %Machine{m | pc: 2, tape: %Tape{head: 1}}
    end

    test "< at cell 0 raises" do
      assert_raise ArgumentError, fn -> Machine.step(machine("<")) end
    end

    test ". appends the current cell to the output as a byte" do
      m = machine("+.", pc: 1, tape: %Tape{data: %{0 => ?i}}, output: "H")
      assert Machine.step(m) == %Machine{m | pc: 2, output: "Hi"}
    end

    test ", writes the next input byte to the current cell and consumes it" do
      m = machine("+,", pc: 1, input: "ab")
      assert Machine.step(m) == %Machine{m | pc: 2, input: "b", tape: %Tape{data: %{0 => ?a}}}
    end

    test ", at EOF writes 0 and advances" do
      m = machine("+,", pc: 1, tape: %Tape{data: %{0 => 7}})
      assert Machine.step(m) == %Machine{m | pc: 2, tape: %Tape{data: %{0 => 0}}}
    end
  end

  describe "step jumps" do
    test "[ on a zero cell jumps past its matching ]" do
      m = machine("+[-]+", pc: 1)
      assert Machine.step(m) == %Machine{m | pc: 4}
    end

    test "[ on a non-zero cell enters the loop" do
      m = machine("+[-]+", pc: 1, tape: %Tape{data: %{0 => 2}})
      assert Machine.step(m) == %Machine{m | pc: 2}
    end

    test "] on a non-zero cell jumps back to just after its matching [" do
      m = machine("+[-]+", pc: 3, tape: %Tape{data: %{0 => 2}})
      assert Machine.step(m) == %Machine{m | pc: 2}
    end

    test "] on a zero cell leaves the loop" do
      m = machine("+[-]+", pc: 3)
      assert Machine.step(m) == %Machine{m | pc: 4}
    end
  end

  describe "halted?" do
    test "an empty program is halted before it starts" do
      assert Machine.halted?(machine(""))
    end

    test "is halted only once pc reaches the end of the ops" do
      refute Machine.halted?(machine("+-", pc: 1))
      assert Machine.halted?(machine("+-", pc: 2))
    end
  end

  describe "run" do
    test "runs a loop until its cell reaches zero" do
      m = machine("++[->+<]")
      assert Machine.run(m) == %Machine{m | pc: 8, tape: %Tape{data: %{0 => 0, 1 => 2}}}
    end

    test "runs nested loops" do
      m = machine("++[>+++[>+<-]<-]")
      assert Machine.run(m) == %Machine{m | pc: 16, tape: %Tape{data: %{0 => 0, 1 => 0, 2 => 6}}}
    end

    test "never enters a loop whose cell starts at zero" do
      m = machine("[+]")
      assert Machine.run(m) == %Machine{m | pc: 3}
    end

    test "echoes input to output until EOF" do
      m = machine(",[.,]", input: "hi")

      assert Machine.run(m) == %Machine{
               m
               | pc: 5,
                 input: "",
                 output: "hi",
                 tape: %Tape{data: %{0 => 0}}
             }
    end

    test "returns an empty program unchanged" do
      m = machine("")
      assert Machine.run(m) == m
    end
  end

  defp machine(source, fields \\ []),
    do: struct!(%Machine{program: Parser.parse(source)}, fields)
end
