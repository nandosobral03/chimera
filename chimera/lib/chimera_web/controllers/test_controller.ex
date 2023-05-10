defmodule ChimeraWeb.TestController do
  use ChimeraWeb, :controller

  def index(conn, _params) do
    text conn, "Hello World"
  end


end
