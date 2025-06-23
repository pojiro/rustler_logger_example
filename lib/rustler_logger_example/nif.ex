defmodule RustlerLoggerExample.Nif do
  use Rustler,
    otp_app: :rustler_logger_example,
    crate: :rustler_logger_example_nif,
    mode: :debug

  # When loading a NIF module, dummy clauses for all NIF function are required.
  # NIF dummies usually just error out when called when the NIF is not loaded, as that should never normally happen.
  def error(), do: :erlang.nif_error(:nif_not_loaded)

  def add(_a, _b), do: error()
end
