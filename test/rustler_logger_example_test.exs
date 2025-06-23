defmodule RustlerLoggerExampleTest do
  use ExUnit.Case
  doctest RustlerLoggerExample

  test "greets the world" do
    assert RustlerLoggerExample.hello() == :world
  end
end
