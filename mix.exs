defmodule RustlerLoggerExample.MixProject do
  use Mix.Project

  def project do
    [
      app: :rustler_logger_example,
      version: "0.1.0",
      elixir: "~> 1.17",
      start_permanent: Mix.env() == :prod,
      deps: deps()
    ]
  end

  # Run "mix help compile.app" to learn about applications.
  def application do
    [
      extra_applications: [:logger],
      mod: {RustlerLoggerExample.Application, []}
    ]
  end

  # Run "mix help deps" to learn about dependencies.
  defp deps do
    [
      {:rustler, "~> 0.36.2", runtime: false}
    ]
  end
end
