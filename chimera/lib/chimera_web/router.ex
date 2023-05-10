defmodule ChimeraWeb.Router do
  use ChimeraWeb, :router

  pipeline :api do
    plug :accepts, ["json"]
  end

  scope "/api", ChimeraWeb do
    pipe_through :api
  end

  scope "/", ChimeraWeb do
    pipe_through :api
    get "/", TestController, :index
  end

end
