defmodule RustlerLoggerExample.Application do
  use Application

  def start(_type, _args) do
    children = [
      {RustlerLoggerExample.Logger.Supervisor, []}
    ]

    Supervisor.start_link(children, strategy: :one_for_one)
  end
end
