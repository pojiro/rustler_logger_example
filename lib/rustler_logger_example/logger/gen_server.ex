defmodule RustlerLoggerExample.Logger.GenServer do
  use GenServer

  require Logger

  def start_link(args) do
    name = Keyword.get(args, :name, __MODULE__)
    GenServer.start_link(__MODULE__, args, name: name)
  end

  def init(_args) do
    :ok = RustlerLoggerExample.Nif.logger_init(self())
    {:ok, %{}}
  end

  def handle_info({level, message}, state) do
    case level do
      :debug -> Logger.debug(message)
    end

    {:noreply, state}
  end
end
