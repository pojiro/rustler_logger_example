defmodule RustlerLoggerExample.Logger.GenServer do
  use GenServer

  require Logger

  def start_link(args) do
    name = Keyword.get(args, :name, __MODULE__)
    GenServer.start_link(__MODULE__, args, name: name)
  end

  def init(_args) do
    :ok = RustlerLoggerExample.Nif.init(self())
    {:ok, %{}}
  end

  def handle_info(message, state) do
    Logger.debug(message)
    {:noreply, state}
  end
end
