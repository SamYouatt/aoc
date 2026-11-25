defmodule Brainjuice.Machine do
  alias Brainjuice.{Machine, Program, Tape}

  defstruct program: %Program{}, pc: 0, tape: %Tape{}, input: "", output: ""

  def new(program, input), do: %Machine{program: program, input: input}

  def run(%Machine{} = machine), do: if(halted?(machine), do: machine, else: run(step(machine)))

  def step(%Machine{} = machine) do
    op = elem(machine.program.ops, machine.pc)
    exec(machine, op)
  end

  def halted?(%Machine{} = machine), do: machine.pc >= tuple_size(machine.program.ops)

  defp exec(%Machine{} = machine, :inc),
    do: %Machine{machine | pc: machine.pc + 1, tape: Tape.inc(machine.tape)}

  defp exec(%Machine{} = machine, :dec),
    do: %Machine{machine | pc: machine.pc + 1, tape: Tape.dec(machine.tape)}

  defp exec(%Machine{} = machine, :left),
    do: %Machine{machine | pc: machine.pc + 1, tape: Tape.left(machine.tape)}

  defp exec(%Machine{} = machine, :right),
    do: %Machine{machine | pc: machine.pc + 1, tape: Tape.right(machine.tape)}

  defp exec(%Machine{input: <<byte, rest::binary>>} = machine, :input),
    do: %Machine{machine | pc: machine.pc + 1, tape: Tape.write(machine.tape, byte), input: rest}

  defp exec(%Machine{input: <<>>} = machine, :input),
    do: %Machine{machine | pc: machine.pc + 1, tape: Tape.write(machine.tape, 0)}

  defp exec(%Machine{} = machine, :output),
    do: %Machine{
      machine
      | pc: machine.pc + 1,
        output: machine.output <> <<Tape.read(machine.tape)>>
    }

  defp exec(%Machine{} = machine, :jump_if_zero) do
    pc =
      if Tape.read(machine.tape) == 0,
        do: machine.program.jumps[machine.pc] + 1,
        else: machine.pc + 1

    %Machine{machine | pc: pc}
  end

  defp exec(%Machine{} = machine, :jump_not_zero) do
    pc =
      if Tape.read(machine.tape) != 0,
        # different readings of the language say "jumps to the matching [" which would mean `jump` not `jump + 1`,
        # but the [ will always progress because it'll be non-zero. By advancing ourselves we save executing a step
        do: machine.program.jumps[machine.pc] + 1,
        else: machine.pc + 1

    %Machine{machine | pc: pc}
  end
end
