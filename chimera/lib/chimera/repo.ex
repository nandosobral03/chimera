defmodule Chimera.Repo do
  use Ecto.Repo,
    otp_app: :chimera,
    adapter: Ecto.Adapters.Postgres
end
