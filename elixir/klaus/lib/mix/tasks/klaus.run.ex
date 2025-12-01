defmodule Mix.Tasks.Klaus.Run do
  @moduledoc "Runs all days or a specific day. Use the -d {day} argument to run a specific day"
  @shortdoc "Runs and benchmarks days"
  use Mix.Task

  @impl Mix.Task
  def run(args) do
    Mix.Task.run("compile")
    # Parse args
    # If day specified:
    # - Parse day out and run that day
    # Else:
    # - Parse all days and run all of them
  end
end
