defmodule Brainjuice do
  alias Brainjuice.{Parser, Machine}

  def interpret(source, input \\ "") do
    source
    |> Parser.parse()
    |> Machine.new(input)
    |> Machine.run()
    |> Map.fetch!(:output)
  end
end
