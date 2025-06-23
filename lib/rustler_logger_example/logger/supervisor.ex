defmodule RustlerLoggerExample.Logger.Supervisor do
  use Supervisor

  def start_link(args) do
    name = Keyword.get(args, :name, __MODULE__)
    Supervisor.start_link(__MODULE__, args, name: name)
  end

  def init(_args) do
    children = [
      {RustlerLoggerExample.Logger.GenServer, []}
    ]

    Supervisor.init(children, strategy: :one_for_one)
  end
end
