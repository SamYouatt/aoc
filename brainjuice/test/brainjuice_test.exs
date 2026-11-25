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
end
