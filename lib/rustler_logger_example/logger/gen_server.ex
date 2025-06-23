defmodule RustlerLoggerExample.Logger.GenServer do
  use GenServer

  def start_link(args) do
    name = Keyword.get(args, :name, __MODULE__)
    GenServer.start_link(__MODULE__, args, name: name)
  end

  def init(_args) do
    {:ok, %{}}
  end
end
