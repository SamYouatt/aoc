defmodule BrainjuiceTest do
  use ExUnit.Case

  describe "interpret" do
    test "runs Hello World" do
      source = """
      ++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]
      >>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.
      """

      assert Brainjuice.interpret(source, "") == "Hello World!\n"
    end

    test "reads input until EOF and writes it back reversed" do
      assert Brainjuice.interpret(">,[>,]<[.<]", "hello") == "olleh"
    end
  end

  describe "tests.b from brainfuck.org" do
    test "reads newline as 10 and EOF as 0" do
      source = ">,>+++++++++,>+++++++++++[<++++++<++++++<+>>>-]<<.>.<<-.>.>.<<."
      assert Brainjuice.interpret(source, "\n") == "LB\nLB\n"
    end

    test "handles several obscure problems" do
      source =
        ~S|[]++++++++++[>>+>+>++++++[<<+<+++>>>-]<<<<-]"A*$";?@![#>>+<<]>[>>]<<<<[>++<[-]]>.>.|

      assert Brainjuice.interpret(source) == "H\n"
    end

    test "handles deeply nested brackets in rot13.b" do
      source = File.read!("test/fixtures/rot13.b")
      assert Brainjuice.interpret(source, "~mlk zyx") == "~zyx mlk"
    end

    @tag :slow
    test "reaches cell 30000" do
      source =
        "++++[>++++++<-]>[>+++++>+++++++<<-]>>++++<[[>[[>>+<<-]<]>>>-]>-[>+>+<<-]>]+++++[>+++++++<<++>-]>.<<."

      assert Brainjuice.interpret(source) == "#\n"
    end
  end
end
